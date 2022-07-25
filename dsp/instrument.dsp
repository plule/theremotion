declare name        "leapotron";
declare version     "1.0";
declare author      "Pierre Lulé";
declare license     "BSD";

import("stdfaust.lib");

note = hslider("note", 60, 0, 127, 0.01) : si.smoo;
vol = hslider("volume [unit:dB]", -25, -96, 0, 0.01) : ba.db2linear : si.smoo;
cutoff_note = hslider("cutoff_note", 0, -20, 20, 0.01) : si.smoo;

cutoff = ba.midikey2hz(note + cutoff_note);
freq = ba.midikey2hz(note);
q = hslider("res", 1, 1, 30, 0.01);

process = sy.dubDub(freq,cutoff,q,1) * vol;
