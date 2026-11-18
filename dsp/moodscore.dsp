// Moodscore DSP: Phase 0 drone voice.
//
// One drone voice: two detuned sawtooths plus a sub sine, slow analog-style
// pitch drift, a Moog-style ladder low-pass, and a stereo reverb.
//
// This file only exposes raw synthesis parameters. It knows nothing about
// moods, gestures or MCP; that lives in the director.
//
// Parameter values arriving here are already smoothed by the engine, but
// every control is also lightly smoothed (si.smoo) as a second line of
// defence against zipper noise.

declare name "moodscore";
declare version "0.0.1";
declare license "MIT OR Apache-2.0";

import("stdfaust.lib");

// ---------------------------------------------------------------- controls

pitch    = hslider("drone_1/pitch [unit:Hz]", 55, 20, 440, 0.01) : si.smoo;
detune   = hslider("drone_1/detune [unit:cents]", 7, 0, 50, 0.01) : si.smoo;
sub      = hslider("drone_1/sub", 0.5, 0, 1, 0.001) : si.smoo;
drift    = hslider("drone_1/drift", 0.3, 0, 1, 0.001) : si.smoo;
level    = hslider("drone_1/level", 0.7, 0, 1, 0.001) : si.smoo;

cutoff   = hslider("filter/cutoff [unit:Hz] [scale:log]", 900, 40, 12000, 0.1) : si.smoo;
reso     = hslider("filter/resonance", 0.3, 0, 0.95, 0.001) : si.smoo;

rev_mix  = hslider("space/reverb_mix", 0.35, 0, 1, 0.001) : si.smoo;

master   = hslider("master/gain", 0.5, 0, 1, 0.001) : si.smoo;

// ------------------------------------------------------------------ voice

// Slow, smoothed random walk: the "analog" wander of an unstable oscillator.
// Output is in cents, scaled by `drift` (0..1 maps to 0..15 cents).
wander(i) = no.noises(3, i) : fi.lowpass(1, 0.15) : *(4500 * drift);

cents2ratio(c) = pow(2, c / 1200);

osc_a = os.sawtooth(pitch * cents2ratio(wander(0) + detune));
osc_b = os.sawtooth(pitch * cents2ratio(wander(1) - detune));
// os.oscs is table-free: table-based oscillators (os.osc) lock in compute().
osc_s = os.oscs(pitch * 0.5 * cents2ratio(wander(2))) * sub;

voice = (osc_a + osc_b) * 0.5 + osc_s
      : ve.moog_vcf_2bn(reso, cutoff)
      : *(level);

// ------------------------------------------------------------------- space

// Slight stereo spread before the reverb so the drone is not dead centre.
spread(x) = x * 0.9, x * 0.9 : de.delay(64, 23), _;

wet = re.zita_rev1_stereo(0, 200, 6000, 6, 8, 48000);
space = _, _ <: (_, _), wet : ro.interleave(2, 2)
      : (_ * (1 - rev_mix) + _ * rev_mix), (_ * (1 - rev_mix) + _ * rev_mix);

// ------------------------------------------------------------------- out

safety = ma.tanh;   // soft clip: nothing leaves at more than full scale

process = voice : spread : space : (*(master) : safety), (*(master) : safety);
