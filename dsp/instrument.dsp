declare name        "leapotron";
declare version     "1.0";
declare author      "Pierre Lulé";
declare license     "BSD";

import("stdfaust.lib");

note = hslider("note", 60, 0, 127, 0.01) : si.smoo;
vol = hslider("volume [unit:dB]", -25, -96, 0, 0.01) : ba.db2linear : si.smoo;
cutoff_note = hslider("cutoff_note", 0, -20, 50, 0.01) : si.smoo;
q = hslider("res", 1, 1, 30, 0.01);
saw_res = hslider("saw_res", 1, 0, 40, 0.1);
detune = hslider("detune", 0, 0, 0.1, 0.001) : si.smoo;
supersaw = hslider("supersaw", 0, 0, 1.0, 0.01) : si.smoo;

cutoff = ba.midikey2hz(note + cutoff_note);
freq = ba.midikey2hz(note);

// Volume of the main saw and supersaws
supersaw_volume = supersaw / 3;
saw_volume = 1 - supersaw_volume * 2;

saw1 = os.sawtooth(freq);
saw2 = os.sawtooth(freq * (1 + detune * 2)) * supersaw;
saw3 = os.sawtooth(freq * (1 + detune * 2)) * supersaw;

process = saw1, saw2, saw3 :> fi.resonlp(cutoff, q, 1) * vol;
