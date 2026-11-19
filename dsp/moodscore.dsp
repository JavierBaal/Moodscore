// Moodscore DSP: six drone voices.
//
// Six drone voices, each two detuned sawtooths with slow analog-style pitch
// drift. Voice 1 is the foundation and also carries a sub sine an octave
// below. The voices are panned across the stereo field, summed, run through
// a stereo Moog-style ladder low-pass and a stereo reverb.
//
// At rest only voice 1 (A1) and a quiet voice 2 (the fifth, E2) sound; the
// others wait at level 0 on the notes of an A minor chord, ready to enter.
//
// This file only exposes raw synthesis parameters. It knows nothing about
// moods, gestures, chords or MCP; that lives in the director.
//
// Parameter values arriving here are already smoothed by the engine, but
// every control is also lightly smoothed (si.smoo) as a second line of
// defence against zipper noise.

declare name "moodscore";
declare version "0.1.0";
declare license "MIT OR Apache-2.0";

import("stdfaust.lib");

N = 6;

// ------------------------------------------------------------- per voice

// Defaults, by voice: A1, E2, A2, C3, E3, A3 (an open A minor chord).
init_pitch(v) = ba.take(v, (55, 82.41, 110, 130.81, 164.81, 220));
init_level(v) = ba.take(v, (0.7, 0.3, 0, 0, 0, 0));
// Pan position, -1 (left) .. 1 (right). Voice 1 stays in the centre.
pan_pos(v)    = ba.take(v, (0, -0.35, 0.35, -0.7, 0.7, 0));

pitch(v)  = hslider("drone_%v/pitch [unit:Hz] [scale:log]", init_pitch(v), 20, 880, 0.01) : si.smoo;
detune(v) = hslider("drone_%v/detune [unit:cents]", 7, 0, 50, 0.01) : si.smoo;
drift(v)  = hslider("drone_%v/drift", 0.3, 0, 1, 0.001) : si.smoo;
level(v)  = hslider("drone_%v/level", init_level(v), 0, 1, 0.001) : si.smoo;

sub       = hslider("drone_1/sub", 0.5, 0, 1, 0.001) : si.smoo;

cutoff    = hslider("filter/cutoff [unit:Hz] [scale:log]", 900, 40, 12000, 0.1) : si.smoo;
reso      = hslider("filter/resonance", 0.3, 0, 0.95, 0.001) : si.smoo;

rev_mix   = hslider("space/reverb_mix", 0.35, 0, 1, 0.001) : si.smoo;

master    = hslider("master/gain", 0.5, 0, 1, 0.001) : si.smoo;

// ------------------------------------------------------------------ voices

// Slow, smoothed random walk: the "analog" wander of an unstable oscillator.
// Output is in cents, scaled by the voice's drift. Each oscillator gets its
// own noise stream, so voices never wander in lockstep.
noise(k) = no.noises(3 * N, k);
wander(v, k) = noise(3 * (v - 1) + k) : fi.lowpass(1, 0.15) : *(4500 * drift(v));

cents2ratio(c) = pow(2, c / 1200);

saws(v) = (os.sawtooth(pitch(v) * cents2ratio(wander(v, 0) + detune(v)))
         + os.sawtooth(pitch(v) * cents2ratio(wander(v, 1) - detune(v)))) * 0.5;

// os.oscs is table-free: table-based oscillators (os.osc) lock in compute().
sub_osc = os.oscs(pitch(1) * 0.5 * cents2ratio(wander(1, 2))) * sub;

voice(1) = (saws(1) + sub_osc) * level(1);
voice(v) = saws(v) * level(v);

// Equal-power pan.
// Scaled so the centre is unity gain (a centred voice is as loud as before panning).
pan(p) = _ <: *(sqrt(2) * cos((p + 1) * ma.PI / 4)), *(sqrt(2) * sin((p + 1) * ma.PI / 4));

// Gentle normalisation so six voices together grow bigger without slamming
// into the clipper: unity up to a combined level of 1.5, then scaled back.
total_level = sqrt(sum(i, N, level(i + 1) ^ 2));
norm = 1 / max(1, total_level / 1.5);

bus = par(i, N, voice(i + 1) : pan(pan_pos(i + 1))) :> *(norm), *(norm);

// ------------------------------------------------------------------- tone

filter = ve.moog_vcf_2bn(reso, cutoff), ve.moog_vcf_2bn(reso, cutoff);

// Slight extra width so the centre voices are not dead centre.
spread = *(0.9), (*(0.9) : de.delay(64, 23));

// ------------------------------------------------------------------- space

wet = re.zita_rev1_stereo(0, 200, 6000, 6, 8, 48000);
space = _, _ <: (_, _), wet : ro.interleave(2, 2)
      : (_ * (1 - rev_mix) + _ * rev_mix), (_ * (1 - rev_mix) + _ * rev_mix);

// ------------------------------------------------------------------- out

safety = ma.tanh;   // soft clip: nothing leaves at more than full scale

process = bus : filter : spread : space : (*(master) : safety), (*(master) : safety);
