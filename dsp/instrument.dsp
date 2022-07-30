declare name        "leapotron";
declare version     "1.0";
declare author      "Pierre Lulé";
declare license     "BSD";

import("stdfaust.lib");

note = hslider("note", 60, 0, 127, 0.01) : si.smoo;
vol = hslider("volume", 0.8, 0, 1, 0.01) : si.smoo;
sub_volume = hslider("sub_volume", 0.1, 0, 1, 0.01) : si.smoo;
cutoff_note = hslider("cutoff_note", 0, -20, 50, 0.01) : si.smoo;
res = hslider("res", 0, 0, 0.99, 0.01);
saw_res = hslider("saw_res", 1, 0, 40, 0.1);
detune = hslider("detune", 0.001, 0.001, 0.02, 0.001) : si.smoo;
supersaw = hslider("supersaw", 0, 0, 1.0, 0.01) : si.smoo;

cutoff = ba.midikey2hz(note + cutoff_note);
freq = ba.midikey2hz(note);

// Volume of the main saw and supersaws
supersaw_volume = supersaw / 3;
saw_volume = 1 - supersaw_volume * 2;

saw1 = os.sawtooth(freq);
saw2 = os.sawtooth(freq * (1 + detune)) * supersaw;
saw3 = os.sawtooth(freq * (1 - detune)) * supersaw;

lead = (saw1 + saw2 + saw3);

// Sub
sub_freq = ba.midikey2hz(note - 12);
sub = os.oscsin(sub_freq) * sub_volume;

n = lead + sub : ve.moog_vcf_2b(res, cutoff) : _ * vol : ef.echo(1.0, 0.3, 0.3);
process = n,n;