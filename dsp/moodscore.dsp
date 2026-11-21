// Moodscore DSP: six drone voices.
//
// Six drone voices, each two detuned sawtooths with slow analog-style pitch
// drift. Voice 1 is the foundation and also carries a sub sine an octave
// below. Each voice's two oscillators open slightly left and right of the
// voice's position, so width comes from the oscillators themselves, never
// from delaying one channel (which would pull the image to the other side).
// The voices are summed, run through a stereo Moog-style ladder low-pass and
// a stereo reverb. The room can be moved near or far: its size, how dark its
// tail is, the gap before it answers (predelay), how loud it answers (wet),
// how loud the direct sound is and how much of its top the air takes away.
//
// The note voices (the synth in moodscore-synth) arrive as four inputs: their
// dry sound (left, right) and what they send to the reverb (left, right).
// They share the drones' room, mono bass, master gain and soft clip, so
// notes and drones sound in the same space.
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
// Pan position, -1 (left) .. 1 (right). The two voices that sound at rest
// sit in the centre; the rest come in mirrored pairs, so the image stays
// balanced as voices enter.
pan_pos(v)    = ba.take(v, (0, 0, -0.4, 0.4, -0.65, 0.65));
// How far each voice's two oscillators open either side of its position.
width = 0.25;

pitch(v)  = hslider("drone_%v/pitch [unit:Hz] [scale:log]", init_pitch(v), 20, 880, 0.01) : si.smoo;
detune(v) = hslider("drone_%v/detune [unit:cents]", 7, 0, 50, 0.01) : si.smoo;
drift(v)  = hslider("drone_%v/drift", 0.3, 0, 1, 0.001) : si.smoo;
level(v)  = hslider("drone_%v/level", init_level(v), 0, 1, 0.001) : si.smoo;

sub       = hslider("drone_1/sub", 0.5, 0, 1, 0.001) : si.smoo;

cutoff    = hslider("filter/cutoff [unit:Hz] [scale:log]", 900, 40, 12000, 0.1) : si.smoo;
reso      = hslider("filter/resonance", 0.3, 0, 0.95, 0.001) : si.smoo;

rev_mix   = hslider("space/reverb_mix", 0.35, 0, 1, 0.001) : si.smoo;
// The room. At their defaults the room is exactly the one Moodscore always
// had: an 8-second tail darkening above 6 kHz, no predelay, the direct
// sound untouched.
// si.smoo starts from 0, and a room of size 0 (or a tail darkening above
// 0 Hz) is no room at all: smooth the distance from the default instead,
// so each starts exactly where it should.
from_default(d) = -(d) : si.smoo : +(d);
size      = hslider("space/size [unit:s]", 8, 1, 20, 0.01) : from_default(8);
damping   = hslider("space/damping [unit:Hz] [scale:log]", 6000, 1500, 12000, 1) : from_default(6000);
predelay  = hslider("space/predelay [unit:ms]", 0, 0, 80, 0.01) : si.smoo;
direct    = hslider("space/direct", 1, 0, 1, 0.001) : from_default(1);
wet_gain  = hslider("space/wet", 1, 0, 1.5, 0.001) : from_default(1);
air       = hslider("space/air", 0, 0, 1, 0.001) : si.smoo;

master    = hslider("master/gain", 0.5, 0, 1, 0.001) : si.smoo;

// ------------------------------------------------------------------ voices

// Slow, smoothed random walk: the "analog" wander of an unstable oscillator.
// Output is in cents, scaled by the voice's drift. Each oscillator gets its
// own noise stream, so voices never wander in lockstep.
noise(k) = no.noises(3 * N, k);
wander(v, k) = noise(3 * (v - 1) + k) : fi.lowpass(1, 0.15) : *(4500 * drift(v));

cents2ratio(c) = pow(2, c / 1200);

saw_a(v) = os.sawtooth(pitch(v) * cents2ratio(wander(v, 0) + detune(v))) * 0.5;
saw_b(v) = os.sawtooth(pitch(v) * cents2ratio(wander(v, 1) - detune(v))) * 0.5;

// os.oscs is table-free: table-based oscillators (os.osc) lock in compute().
sub_osc = os.oscs(pitch(1) * 0.5 * cents2ratio(wander(1, 2))) * sub;

// A voice as a stereo pair: one oscillator a little left of the voice's
// position, the other a little right.
saws_st(v) = (saw_a(v) * level(v) : pan(pan_pos(v) - width)),
             (saw_b(v) * level(v) : pan(pan_pos(v) + width)) :> _, _;

voice(1) = saws_st(1), (sub_osc * level(1) : pan(0)) :> _, _;
voice(v) = saws_st(v);

// Equal-power pan.
// Scaled so the centre is unity gain (a centred voice is as loud as before panning).
pan(p) = _ <: *(sqrt(2) * cos((p + 1) * ma.PI / 4)), *(sqrt(2) * sin((p + 1) * ma.PI / 4));

// Gentle normalisation so six voices together grow bigger without slamming
// into the clipper: unity up to a combined level of 1.5, then scaled back.
total_level = sqrt(sum(i, N, level(i + 1) ^ 2));
norm = 1 / max(1, total_level / 1.5);

bus = par(i, N, voice(i + 1)) :> *(norm), *(norm);

// ------------------------------------------------------------------- tone

filter = ve.moog_vcf_2bn(reso, cutoff), ve.moog_vcf_2bn(reso, cutoff);

// Headroom before the reverb (kept from the original single-voice design).
spread = *(0.9), *(0.9);

// ------------------------------------------------------------------- space

// The tail: low frequencies decay in three quarters of `size`, the highs
// above `damping` faster. The predelay is interpolated, so it can glide.
pre = de.fdelay(8192, predelay * ma.SR * 0.001);
wet = (pre, pre) : re.zita_rev1_stereo(0, 200, damping, size * 0.75, size, 48000)
    : *(wet_gain), *(wet_gain);

// The air between the listener and a far source swallows the highs of the
// direct sound: two gentle poles, from none (air 0, an exact pass-through)
// to a darkening around 2 kHz (air 1).
air_pole = air * exp(-2 * ma.PI * 3000 / ma.SR);
absorb = fi.pole(air_pole) : *(1 - air_pole) : fi.pole(air_pole) : *(1 - air_pole);
near = absorb : *(direct);

// Drones (dl, dr) keep their dry/wet balance; notes add their dry sound and
// their send. The reverb is linear, so the drones sound exactly as before.
// Distance acts on everything: the direct sound of drones and notes alike
// goes through the air and the direct level.
room(dl, dr, nl, nr, sl, sr) =
    ((dl * (1 - rev_mix) + nl : near), (dr * (1 - rev_mix) + nr : near)),
    ((dl * rev_mix + sl, dr * rev_mix + sr) : wet)
    :> _, _;

// -------------------------------------------------------------- mono bass

// Below about 150 Hz the ear barely localises, but level differences there
// still tip a drone's weight to one side: the reverb, summed with the dry
// signal, leaves the bass a couple of dB louder on the right. Standard mix
// practice: keep the lows mono. Linkwitz-Riley crossover, so the bands sum
// flat; above it, the stereo image is untouched.
bass_mono_fc = 150;
mono_bass = _, _ <: (fi.highpassLR4(bass_mono_fc), fi.highpassLR4(bass_mono_fc)),
                    ((+ : *(0.5) : fi.lowpassLR4(bass_mono_fc)) <: _, _)
          :> _, _;

// ------------------------------------------------------------------- out

safety = ma.tanh;   // soft clip: nothing leaves at more than full scale

drones = bus : filter : spread;

process = (drones, _, _, _, _) : room : mono_bass : (*(master) : safety), (*(master) : safety);
