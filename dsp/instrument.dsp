declare name        "leapotron";
declare version     "1.0";
declare author      "Pierre Lulé";
declare license     "BSD";

import("stdfaust.lib");

// Main voice controls
leadGroup(x) = vgroup("[0]lead", x);
lead_volume = leadGroup(hslider("[0]vol", 0.0, 0, 1, 0.001)) : si.smoo;
chordGroup(x) = leadGroup(vgroup("[1]chord", x));
vol1 = chordGroup(hslider("[1]vol1", 1.0, 0, 1, 0.001)) : si.smoo;
vol2 = chordGroup(hslider("[3]vol2", 0.0, 0, 1, 0.001)) : si.smoo;
vol3 = chordGroup(hslider("[5]vol3", 0.0, 0, 1, 0.001)) : si.smoo;
vol4 = chordGroup(hslider("[7]vol4", 0.0, 0, 1, 0.001)) : si.smoo;
note1 = chordGroup(hslider("[2]note1", 60, 0, 127, 0.001)) : si.smoo;
note2 = chordGroup(hslider("[4]note2", 60, 0, 127, 0.001)) : si.smoo;
note3 = chordGroup(hslider("[6]note3", 60, 0, 127, 0.001)) : si.smoo;
note4 = chordGroup(hslider("[8]note4", 60, 0, 127, 0.001)) : si.smoo;

// Main voice supersaw controls
ssawGroup(x) = leadGroup(hgroup("[9]supersaw", x));
supersaw = ssawGroup(hslider("[0]volume", 0, 0, 1.0, 0.001)) : si.smoo;
detune = ssawGroup(hslider("[1]detune", 0.001, 0.001, 0.02, 0.001)) : si.smoo;

// Main voice filter
filterGroup(x) = leadGroup(hgroup("[10]filter", x));
cutoff_note = filterGroup(hslider("[0]cutoff_note", 0, -20, 50, 0.001)) : si.smoo;
res = filterGroup(hslider("[1]res", 0, 0, 0.99, 0.001)) : si.smoo;

// Pluck voice


// Drone voice
droneGroup(x) = hgroup("[2]drone", x);
drone_volume = droneGroup(hslider("[0]volume", 0, 0, 1, 0.001)) : si.smoo;
drone_note = droneGroup(hslider("[1]note", 60, 0, 127, 0.001)) : si.smoo;

// Global
pitch_bend = hslider("[3]pitchBend", 0, -1, 1, 0.001);

// Lead oscillator
lead_i(n, v) = saw_osc(0) + supersaw_osc * supersaw : ve.moog_vcf_2b(res, cutoff_freq) : _ * v / 2
with {
    f = n + pitch_bend : ba.midikey2hz;
    saw_osc(detune) = os.sawtooth(f * (1 + detune));
    supersaw_osc = saw_osc(detune) + saw_osc(-detune);
    cutoff_freq = ba.midikey2hz(n + cutoff_note);
};
lead = lead_i(note1, vol1) + lead_i(note2, vol2) + lead_i(note3, vol3) + lead_i(note4, vol4) : _ * lead_volume;

// Guitar
guitar = sy.combString(f, pluck_release, pluck) : co.compressor_mono(ratio,thresh,att,rel) * pluck_gain : fi.lowpass(1, f * 2) : ve.crybaby(pluck_wah)
with {
    pluckGroup(x) = hgroup("[1]pluck", x);
    pluck = pluckGroup(button("[0]gate"));
    pluck_note = pluckGroup(hslider("[1]note", 80, 0, 127, 0.001)) : si.smoo;
    pluck_wah = pluckGroup(hslider("[1]wah", 0.5, 0.25, 0.75, 0.001)) : si.smoo;
    pluck_gain = pluckGroup(hslider("[2]gain", 100, 0, 100, 0.001));
    pluck_release = pluckGroup(hslider("[3]release", 10, 0, 10, 0.001));

    f = pluck_note + pitch_bend : ba.midikey2hz;
    osc(f) = os.square(f) + os.square(f*2.01);
    env = en.ar(0.01, pluck_release, pluck);

    coGroup(x) = pluckGroup(vgroup("[1]pluck", x));
    ratio = coGroup(hslider("[0]co/ratio", 20, 1, 20, 0.001));
    thresh = coGroup(hslider("[1]co/thresh", -60, -100, 20, 0.001));
    att = coGroup(hslider("[2]co/att", 0, 0, 0.1, 0.001));
    rel = coGroup(hslider("[3]co/rel", 0.1, 0.1, 0.1, 0.001));
};

// Drone
drone = drone_osc(drone_note) * drone_volume
with {
    osc(note) = os.triangle(ba.midikey2hz(note)) / 5;
    drone_osc(note) = osc(note) + osc(note+12.10) + osc(note-12.11) + osc(note + 7.12);
};

// Mix
process = lead + drone + guitar : ef.echo(1.0, 0.3, 0.3) <: _, _;
