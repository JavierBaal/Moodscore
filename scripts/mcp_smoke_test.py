#!/usr/bin/env python3
"""End-to-end MCP smoke test: talks JSON-RPC to `moodscore serve --no-audio`.

Usage: python3 scripts/mcp_smoke_test.py [path/to/moodscore]
Exits non-zero on any failure.
"""

import json
import subprocess
import sys
import time

BIN = sys.argv[1] if len(sys.argv) > 1 else "target/release/moodscore"

proc = subprocess.Popen(
    [BIN, "serve", "--no-audio"],
    stdin=subprocess.PIPE,
    stdout=subprocess.PIPE,
    stderr=subprocess.PIPE,
    text=True,
)
next_id = 0


def send(method, params=None, notify=False):
    global next_id
    msg = {"jsonrpc": "2.0", "method": method}
    if params is not None:
        msg["params"] = params
    if not notify:
        next_id += 1
        msg["id"] = next_id
    proc.stdin.write(json.dumps(msg) + "\n")
    proc.stdin.flush()
    if notify:
        return None
    while True:
        line = proc.stdout.readline()
        if not line:
            sys.exit("server closed stdout:\n" + proc.stderr.read())
        reply = json.loads(line)
        if reply.get("id") == next_id:
            return reply


def call(tool, args):
    reply = send("tools/call", {"name": tool, "arguments": args})
    result = reply["result"]
    text = result["content"][0]["text"]
    return result.get("isError", False), text


def check(cond, what):
    print(("ok   " if cond else "FAIL ") + what)
    if not cond:
        proc.kill()
        sys.exit(1)


init = send(
    "initialize",
    {
        "protocolVersion": "2025-06-18",
        "capabilities": {},
        "clientInfo": {"name": "smoke-test", "version": "0"},
    },
)
check(init["result"]["serverInfo"]["name"] == "moodscore", "initialize")
check("drone" in init["result"].get("instructions", ""), "server instructions present")
send("notifications/initialized", notify=True)

tools = {t["name"] for t in send("tools/list")["result"]["tools"]}
check(
    tools == {"start", "release", "morph", "gesture", "set_pace", "get_state"},
    f"tools/list -> {sorted(tools)}",
)

time.sleep(1.0)
err, text = call("get_state", {})
state = json.loads(text)
check(not err and state["macros"][0]["name"] == "darkness", "get_state")
check(
    not state["output"]["playing"] and state["output"]["peak_dbfs"] <= -100,
    f"starts silent ({state['output']['peak_dbfs']} dBFS peak)",
)

err, text = call("start", {"over": 0.5})
check(not err and json.loads(text)["output"]["playing"], "start")

err, text = call("morph", {"darkness": 0.9, "over": 1.5, "curve": "ease_in_out"})
check(not err and json.loads(text)["macros"][0]["target"] == 0.9, "morph darkness -> 0.9")

time.sleep(2.0)
state = json.loads(call("get_state", {})[1])
dark = state["macros"][0]
cutoff = next(p for p in state["params"] if p["path"] == "filter/cutoff")
check(abs(dark["value"] - 0.9) < 1e-3, f"darkness arrived ({dark['value']})")
check(cutoff["value"] < 250, f"filter closed ({cutoff['value']} Hz)")
check(state["output"]["rms_dbfs"] > -60, f"engine is sounding ({state['output']['rms_dbfs']} dBFS)")
check(state["output"]["gain"] == 1.0, "faded in fully")

err, text = call("gesture", {"param": "drone_1/pitch", "to": 41.2, "over": 0.5})
check(not err, "gesture drone_1/pitch")

err, text = call("set_pace", {"over": 40, "curve": "linear"})
pace = json.loads(text)["pace"] if not err else None
check(pace == {"over": 40.0, "curve": "linear"}, f"set_pace -> {pace}")
err, text = call("morph", {"instability": 0.8})
state = json.loads(text)
inst = next(m for m in state["macros"] if m["name"] == "instability")
check(
    not err and inst["target"] == 0.8 and 39 < inst["seconds_remaining"] <= 40,
    f"morph without 'over' follows the pace ({inst['seconds_remaining']} s left)",
)
err, text = call("morph", {"instability": 0.2, "over": 0.5})
inst = next(m for m in json.loads(text)["macros"] if m["name"] == "instability")
check(not err and inst["seconds_remaining"] <= 0.5, "explicit 'over' overrides the pace")
err, text = call("set_pace", {})
check(err, f"empty set_pace rejected: {text}")

full = json.loads(call("get_state", {})[1])
check(
    sum(1 for p in full["params"] if p["path"].endswith("/level") and p["path"].startswith("drone_")) == 6,
    "six drone voices",
)
err, text = call("gesture", {"param": "drone_3/level", "to": 0.5, "over": 2})
brief = json.loads(text)
check(
    not err and "params" not in brief and any(p["path"] == "drone_3/level" for p in brief["moving"]),
    "action tools return a brief report with the moving parameters",
)

err, text = call("release", {"over": 1, "start_in": 30})
out = json.loads(text)["output"]
check(
    not err and out["playing"] and out["scheduled_fade"]["to"] == 0.0
    and 29 < out["scheduled_fade"]["starts_in"] <= 30,
    f"release can wait ({out.get('scheduled_fade')})",
)
err, text = call("start", {"over": 0.5})
check(not err and "scheduled_fade" not in json.loads(text)["output"], "a new transport call cancels the scheduled one")
err, text = call("release", {"over": 0.5, "start_in": 0.5})
time.sleep(1.5)
out = json.loads(call("get_state", {})[1])["output"]
check(out["gain"] == 0.0 and "scheduled_fade" not in out, "a delayed release happens on time")
call("start", {"over": 0.2})
time.sleep(0.5)

err, text = call("release", {"over": 0.5})
check(not err and not json.loads(text)["output"]["playing"], "release")
time.sleep(1.0)
state = json.loads(call("get_state", {})[1])
check(
    state["output"]["gain"] == 0.0 and state["output"]["peak_dbfs"] <= -100,
    f"released to silence ({state['output']['peak_dbfs']} dBFS peak)",
)
err, text = call("start", {"over": -1})
check(err, f"bad fade time rejected: {text}")

err, text = call("morph", {"darkness": 3})
check(err and "outside" in text, f"out-of-range rejected: {text}")
err, text = call("gesture", {"param": "nope", "to": 1})
check(err and "unknown parameter" in text, f"unknown parameter rejected: {text}")

proc.stdin.close()
proc.wait(timeout=10)
check(proc.returncode == 0, "clean shutdown when the client disconnects")
print("MCP smoke test passed")
