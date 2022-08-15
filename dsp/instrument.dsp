declare name        "leapotron";
declare version     "1.0";
declare author      "Pierre Lulé";
declare license     "BSD";

import("stdfaust.lib");

// Inputs
note = hslider("note", 60, 0, 127, 0.001) : si.smoo;
vol = hslider("volume", 0.0, 0, 1, 0.001) : si.smoo;
sub_volume = hslider("sub_volume", 0.5, 0, 1, 0.001) : si.smoo;
cutoff_note = hslider("cutoff_note", 0, -20, 50, 0.001) : si.smoo;
res = hslider("res", 0, 0, 0.99, 0.001) : si.smoo;
detune = hslider("detune", 0.001, 0.001, 0.02, 0.001) : si.smoo;
supersaw = hslider("supersaw", 0, 0, 1.0, 0.001) : si.smoo;
pluck_position = hslider("pluck_position", 0.5, 0, 1, 0.001) : si.smoo;
pluck = button("pluck");
drone_volume = hslider("drone_volume", 0, 0, 1, 0.001) : si.smoo;
drone_note = hslider("drone_note", 60, 0, 127, 0.001) : si.smoo;

// Lead oscillator
lead = saw_osc(0) + supersaw_osc * supersaw
with {
    f = note : ba.midikey2hz;
    saw_osc(detune) = os.sawtooth(f * (1 + detune));
    supersaw_osc = saw_osc(detune) + saw_osc(-detune);
};

// Sub oscillator
sub = os.square(f) * sub_volume
with {
    f = note - 12 : ba.midikey2hz;
};

// Guitar
guitar = pm.guitar(pm.f2l(f), pluck_position, 1.0, pluck)
with {
    f = note - 12 : ba.midikey2hz;
};

// Drone
drone = drone_osc(drone_note) * drone_volume
with {
    osc(note) = os.triangle(ba.midikey2hz(note)) / 5;
    drone_osc(note) = osc(note) + osc(note+12.10) + osc(note-12.11) + osc(note + 7.12);
};


// Mix
process = lead + sub : ve.moog_vcf_2b(res, cutoff_freq) * vol : _ + drone : ef.echo(1.0, 0.3, 0.3) : _ + guitar <: _, _
with {
    cutoff_freq = ba.midikey2hz(note + cutoff_note);
};
