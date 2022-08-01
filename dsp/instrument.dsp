declare name        "leapotron";
declare version     "1.0";
declare author      "Pierre Lulé";
declare license     "BSD";

import("stdfaust.lib");

// Inputs
note_s = hslider("note", 60, 0, 127, 0.01) : si.smoo;
raw_note = hslider("raw_note", 60, 0, 127, 0.01);
vol = hslider("volume", 0.0, 0, 1, 0.01) : si.smoo;
sub_volume = hslider("sub_volume", 0.1, 0, 1, 0.01) : si.smoo;
cutoff_note = hslider("cutoff_note", 0, -20, 50, 0.01) : si.smoo;
res = hslider("res", 0, 0, 0.99, 0.01) : si.smoo;
detune = hslider("detune", 0.001, 0.001, 0.02, 0.001) : si.smoo;
supersaw = hslider("supersaw", 0, 0, 1.0, 0.01) : si.smoo;
autotune_strength = hslider("autotune_strength", 0, 0, 7, 1);

// hack, raw note and autotune strength not yet in dsp, prevent optim
note = attach(attach(note_s, autotune_strength), raw_note);

// Instrument frequencies
cutoff_freq = ba.midikey2hz(note + cutoff_note);
note_freq = ba.midikey2hz(note);
sub_freq = ba.midikey2hz(note - 12);

// Lead oscillator
saw_osc(detune) = os.sawtooth(note_freq * (1 + detune));
supersaw_osc = saw_osc(detune) + saw_osc(-detune);
lead = saw_osc(0) + supersaw_osc * supersaw;

// Sub oscillator
sub = os.oscsin(sub_freq) * sub_volume;

// Mix
n = lead + sub : ve.moog_vcf_2b(res, cutoff_freq) * vol : ef.echo(1.0, 0.3, 0.3);
process = n,n;