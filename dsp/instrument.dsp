declare name        "leapotron";
declare version     "1.0";
declare author      "Pierre Lulé";
declare license     "BSD";

import("stdfaust.lib");

// Main voice controls
leadGroup(x) = vgroup("[0]lead", x);
vol = leadGroup(hslider("[0]volume", 0.0, 0, 1, 0.001)) : si.smoo;
note = leadGroup(hslider("[1]note", 60, 0, 127, 0.001)) : si.smoo;
sub_volume = leadGroup(hslider("[2]sub", 0.5, 0, 1, 0.001)) : si.smoo;

// Main voice supersaw controls
ssawGroup(x) = leadGroup(hgroup("[3]supersaw", x));
supersaw = ssawGroup(hslider("[0]volume", 0, 0, 1.0, 0.001)) : si.smoo;
detune = ssawGroup(hslider("[1]detune", 0.001, 0.001, 0.02, 0.001)) : si.smoo;

// Main voice filter
filterGroup(x) = leadGroup(hgroup("[4]filter", x));
cutoff_note = filterGroup(hslider("[0]cutoff_note", 0, -20, 50, 0.001)) : si.smoo;
res = filterGroup(hslider("[1]res", 0, 0, 0.99, 0.001)) : si.smoo;

// Pluck voice
pluckGroup(x) = hgroup("[1]pluck", x);
pluck = pluckGroup(button("[0]gate"));
pluck_gain = pluckGroup(hslider("[1]gain", 0.8, 0, 1, 0.001));
pluck_damping = pluckGroup(hslider("[2]damping", 0.0, 0, 1, 0.001));

// Drone voice
droneGroup(x) = hgroup("[2]drone", x);
drone_volume = droneGroup(hslider("[0]volume", 0, 0, 1, 0.001)) : si.smoo;
drone_note = droneGroup(hslider("[1]note", 60, 0, 127, 0.001)) : si.smoo;

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
guitar = pm.ks(len, pluck_damping, pulse)
with {
    len = note : ba.midikey2hz : pm.f2l;
    pulse = pluck : pm.impulseExcitation * pluck_gain;
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
