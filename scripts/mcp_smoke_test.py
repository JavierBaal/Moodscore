#!/usr/bin/env python3
"""End-to-end MCP smoke test: talks JSON-RPC to `moodscore serve --no-audio`.

Usage: python3 scripts/mcp_smoke_test.py [path/to/moodscore]
Exits non-zero on any failure.
"""

import json
import os
import subprocess
import sys
import tempfile
import time

BIN = sys.argv[1] if len(sys.argv) > 1 else "target/release/moodscore"

PRESETS = tempfile.mkdtemp(prefix="moodscore-presets-")
proc = subprocess.Popen(
    [BIN, "serve", "--no-audio"],
    env={**os.environ, "MOODSCORE_PRESETS": PRESETS},
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

tool_list = send("tools/list")["result"]["tools"]
tools = {t["name"] for t in tool_list}
check(
    tools == {"start", "release", "morph", "gesture", "set_pace", "harmony", "tempo", "get_state",
             "patch", "play_notes", "list_presets", "save_preset"},
    f"tools/list -> {sorted(tools)}",
)

for t in tool_list:
    curve = t["inputSchema"].get("properties", {}).get("curve")
    if curve is not None:
        check(
            "$ref" not in json.dumps(curve) and "anyOf" not in json.dumps(curve),
            f"{t['name']}: curve is a plain string in the schema ({curve.get('type')})",
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

err, text = call("morph", {"darkness": 0.5, "over": 1, "curve": "Ease-Out"})
check(not err, "curve names are lenient ('Ease-Out')")
err, text = call("morph", {"darkness": 0.5, "curve": "wobbly"})
check(err and "ease_in_out" in text, f"unknown curve rejected: {text[:60]}...")
check(
    all(p["owner"] == "manual" for p in json.loads(call("morph", {"darkness": 0.6, "over": 2})[1])["moving"]),
    "brief report groups macro-driven parameters under their macro",
)

call("gesture", {"param": "space/reverb_mix", "to": 0.7, "over": 0})
time.sleep(0.3)
call("morph", {"darkness": 0.1, "over": 0})
time.sleep(0.3)
reverb = next(p for p in json.loads(call("get_state", {})[1])["params"] if p["path"] == "space/reverb_mix")
check(
    reverb["owner"] == "manual" and abs(reverb["value"] - 0.7) < 1e-3 and reverb["held_from"] == ["darkness"],
    f"a morph leaves a hand-set parameter alone ({reverb['owner']}, {reverb['value']})",
)
call("morph", {"darkness": 0.1, "over": 0, "reclaim": True})
time.sleep(0.3)
reverb = next(p for p in json.loads(call("get_state", {})[1])["params"] if p["path"] == "space/reverb_mix")
check(reverb["owner"] == "macro:darkness" and "held_from" not in reverb, "reclaim: true takes it back")

import math

def pcs():
    st = json.loads(call("get_state", {})[1])
    hz = [next(p["value"] for p in st["params"] if p["path"] == f"drone_{v}/pitch") for v in range(1, 7)]
    return [round(69 + 12 * math.log2(h / 440)) % 12 for h in hz], st

# drone_1/pitch was tuned by hand above: without reclaim the bass would stay put
err, text = call("harmony", {
    "key": "A", "mode": "aeolian", "glide": 0.3, "reclaim": True,
    "progression": [{"chord": "i", "seconds": 1.5}, {"chord": "VI", "seconds": 1.5}],
})
h = json.loads(text).get("harmony", {}) if not err else {}
check(not err and h.get("chord", {}).get("name") == "Am" and h.get("next", {}).get("name") == "F",
      f"harmony starts on Am, F next ({text[:80] if err else h.get('position')})")
time.sleep(2.2)
p, st = pcs()
check(p[0] == 5 and sorted(set(p)) == [0, 5, 9], f"voices play F major after the change ({p})")
check(all(q["owner"] == "harmony" for q in st["params"] if q["path"].endswith("/pitch")), "voice pitches owned by harmony")
time.sleep(1.5)
p, _ = pcs()
check(p[0] == 9 and sorted(set(p)) == [0, 4, 9], f"loops back to A minor ({p})")

err, text = call("harmony", {"key": "A", "progression": [{"chord": "Vi", "seconds": 4}]})
check(err and "upper case" in text, f"bad chord explained: {text[:70]}")
err, text = call("harmony", {"key": "A", "mode": "spooky", "progression": [{"chord": "i", "seconds": 4}]})
check(err and "lydian" in text, "bad mode explained")
err, text = call("harmony", {"stop": True})
check(not err and "harmony" not in json.loads(text), "harmony stops")

# Musical clock: 240 BPM 4/4 makes a bar last 1 s; chords in bars follow it.
err, text = call("tempo", {"bpm": 240, "meter": "4/4", "downbeat": True})
t = json.loads(text).get("tempo", {}) if not err else {}
check(not err and t.get("bpm") == 240 and t.get("meter") == "4/4" and t.get("seconds_per_bar") == 1.0,
      f"tempo set ({text[:80] if err else t})")
err, text = call("harmony", {
    "key": "C", "mode": "ionian", "glide": 0.2, "start_on": "bar",
    "progression": [{"chord": "I", "bars": 1}, {"chord": "IV", "beats": 4}],
})
h = json.loads(text).get("harmony", {}) if not err else {}
check(not err and 0 < h.get("starts_in", 0) <= 1.0, f"plan waits for the downbeat ({text[:80] if err else h})")
time.sleep(2.6)  # downbeat (<1 s), C for a bar, then F
p, st = pcs()
check(p[0] == 5 and sorted(set(p)) == [0, 5, 9], f"chords in bars follow the tempo ({p})")
err, text = call("tempo", {"bpm": 60, "over_bars": 2})
t = json.loads(text).get("tempo", {}) if not err else {}
check(not err and t.get("ramping_to") == 60 and t.get("ramp_seconds_remaining", 0) > 2,
      f"ritardando ramps ({text[:80] if err else t})")
err, text = call("tempo", {"bpm": 5})
check(err and "between" in text, "bad bpm explained")
err, text = call("harmony", {"key": "C", "progression": [{"chord": "I", "seconds": 2, "bars": 1}]})
check(err and "only one" in text, "two lengths explained")
call("harmony", {"stop": True})

# Note parts: presets, patches, phrases.
err, text = call("list_presets", {})
names = [p["name"] for p in json.loads(text)["presets"]] if not err else []
check(not err and {"pluck", "lead"} <= set(names), f"factory presets listed ({names})")
err, text = call("patch", {"part": 1, "preset": "pluck"})
r = json.loads(text) if not err else {}
check(not err and r.get("preset") == "pluck" and "4 voices" in r.get("description", "") and not r.get("modified"),
      f"pluck loaded ({text[:120]})")
err, text = call("patch", {"part": 1, "set": {"filter/cutoff": 900, "mod2/source": "lfo1", "mod2/dest": "voice/pan", "mod2/amount": 0.4}})
r = json.loads(text) if not err else {}
check(not err and r.get("modified") and r["params"].get("filter/cutoff") == 900 and r["params"].get("mod2/source") == "lfo1",
      f"patch set changes values ({text[:120]})")
err, text = call("patch", {"set": {"filter/type": "notch"}})
check(err and "low_pass" in text, f"bad choice explained: {text[:70]}")
err, text = call("patch", {"part": 3})
check(err and "1 or 2" in text, "bad part explained")
st = json.loads(call("get_state", {})[1])
check(not any(p["path"].startswith("note_") for p in st["params"]) and len(st["notes"]) == 2,
      "get_state summarises the note parts instead of listing their parameters")
err, text = call("play_notes", {"part": 1, "steps": [{"notes": "A3", "beats": 1}, {"notes": "C4 E4", "beats": 1}, {"notes": "rest"}]})
r = json.loads(text) if not err else {}
check(not err and r.get("phrase", {}).get("seconds", 0) > 0.5, f"phrase scheduled ({text[:100] if err else r.get('phrase')})")
time.sleep(0.8)
st = json.loads(call("get_state", {})[1])
check(st["notes"][0]["sounding"] >= 1, f"notes sound ({st['notes'][0]})")
err, text = call("play_notes", {"steps": [{"notes": "H3"}]})
check(err and "not a note" in text, f"bad note explained: {text[:60]}")
err, text = call("play_notes", {"steps": [{"notes": "A"}]})
check(err and "octave" in text, "missing octave explained")
err, text = call("save_preset", {"part": 1, "name": "Dark Pluck", "description": "A darker pluck."})
check(not err and json.loads(text)["saved"] == "dark_pluck", f"preset saved ({text[:80]})")
err, text = call("save_preset", {"name": "pluck", "description": "x"})
check(err and "built-in" in text, "built-in names protected")
err, text = call("patch", {"part": 2, "preset": "dark_pluck"})
r = json.loads(text) if not err else {}
check(not err and r.get("params", {}).get("filter/cutoff") == 900 and r.get("part") == "note_2", "saved preset loads on part 2")
err, text = call("play_notes", {"stop": True})
check(not err, "play_notes stop")

# Bars by number: layers that start together, phrases in short form.
call("tempo", {"bpm": 120, "meter": "4/4"})
bar = json.loads(call("get_state", {})[1])["tempo"]["bar"]
err, text = call("harmony", {"key": "D", "mode": "ionian", "at_bar": bar + 2, "glide": 0.2,
                             "progression": [{"chord": "I", "beats": 2}, {"chord": "I/C#", "beats": 2}]})
h = json.loads(text).get("harmony", {}) if not err else {}
check(not err and h.get("starts_in", 0) > 0, f"harmony waits for bar {bar + 2} ({text[:80] if err else h})")
err, text = call("play_notes", {"part": 1, "at_bar": bar + 2, "phrase": "D4:0.5 F#4 A4:1 | C#4+E4+A4:2@0.6 r:1"})
r = json.loads(text) if not err else {}
check(not err and abs(r["phrase"]["starts_in"] - h.get("starts_in", -9)) < 0.05,
      f"phrase and harmony start on the same bar ({text[:100] if err else r['phrase']})")
err, text = call("play_notes", {"part": 1, "at_bar": max(1, bar - 1), "phrase": "D4"})
check(err and "has passed" in text, f"a passed bar is refused: {text[:60]}")
err, text = call("play_notes", {"part": 2, "phrase": " ".join(["B3:0.25", "D4", "G4", "D4"] * 75)})
st = json.loads(call("get_state", {})[1])
check(not err and st["notes"][1].get("waiting", 0) > 400, f"long phrases are fine ({text[:80] if err else st['notes'][1]})")
err, text = call("play_notes", {"part": 2, "stop": True})
st = json.loads(call("get_state", {})[1])
check(not err and st["notes"][1].get("waiting", 0) == 0, "stop cancels what is waiting")
err, text = call("play_notes", {"phrase": "A3:x"})
check(err and "token 1" in text, f"bad token explained: {text[:60]}")
err, text = call("release", {"at_bar": bar + 6, "over": 2})
f = json.loads(text)["output"].get("scheduled_fade") if not err else None
check(not err and f and f["starts_in"] > 4, f"release waits for its bar ({f})")
call("start", {"over": 0.5})
err, text = call("harmony", {"key": "D", "progression": [{"chord": "V/V", "beats": 2}]})
check(err and "secondary" in text, f"V/V explained: {text[:80]}")
call("harmony", {"stop": True})

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
