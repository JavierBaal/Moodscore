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
    tools == {"start", "release", "morph", "gesture", "get_state"},
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
