declare name        "theremotion";
declare version     "1.0";
declare author      "Pierre Lulé";
declare license     "BSD";

import("stdfaust.lib");

// Lead oscillator
lead(res, cutoffNote) = os.sawtooth(f) * v : ve.moog_vcf_2b(res, cutoffFreq)
with {    
    v = hslider("[1]volume", 0.0, 0, 1, 0.001) : si.smoo;

    note = hslider("[0]note", 60, 0, 127, 0.001);
    f = note : ba.midikey2hz : si.smoo;
    cutoffFreq = note + cutoffNote : ba.midikey2hz : si.smoo;
};

leadChord = (res, cutoffNote) <: par(i, 4, vgroup("[3]%i", lead)) :> _ * v
with {
    v = hslider("[0]volume", 0.0, 0, 1, 0.001) : si.smoo;
    cutoffNote = hslider("[1]cutoffNote", 0, -20, 50, 0.001) : si.smoo;
    res = hslider("[2]res", 0, 0, 0.99, 0.001) : si.smoo;
};

feedback(signal)= signal * 0.005;

// Guitar
elecGuitar(stringLength,pluckPosition,mute,gain,trigger) =
    (pm.elecGuitarModel(stringLength,pluckPosition,mute) : co.compressor_mono(20,-10,0,0.1)) ~
    (_  : ef.gate_mono(-20, 0.0001, 0.1, 0.02)) * 0.005 + pm.pluckString(stringLength,1,1,1,gain,trigger);

guitar = elecGuitar(length,0.5,mute,strength,gate)
        : _ * gain
        : fi.lowpass(1, f * 2)
        : ve.crybaby(wah)
with {
    gate = button("[0]gate");
    note = hslider("[1]note", 80, 0, 127, 0.001) : si.smoo;
    wah = hslider("[1]wah", 0.5, 0.25, 0.75, 0.001) : si.smoo;
    gain = hslider("[2]gain", 1, 0, 1, 0.001);
    mute = hslider("[3]mute", 1, 0.90, 1, 0.001);
    strength = hslider("[4]strength", 0.5, 0, 1, 0.001);
    pitchBend = hslider("[5]pitchBend", 0, -1, 1, 0.001) : si.smoo;

    f = note + pitchBend : ba.midikey2hz;
    length = f : pm.f2l;
};

// Drone
drone = (osc(note) + osc(note+12.10) + osc(note-12.11) + osc(note + 7.12)) * volume
with {
    volume = hslider("[0]volume", 0, 0, 1, 0.001) : si.smoo;
    note = hslider("[1]note", 60, 0, 127, 0.001) : si.smoo;
    osc(note) = os.triangle(ba.midikey2hz(note)) / 5;
};

// Mix
process = hgroup("[2]drone", drone) * drone_volume
    + vgroup("[0]lead", leadChord) * lead_volume
    + hgroup("[1]pluck", guitar) * pluck_volume
    : ef.echo(1.0, 0.3, 0.3)
    : _ * master_volume
    <: _, _
with {
    mixGroup(x) = vgroup("[3]mix", x);
    master_volume = mixGroup(hslider("[0]master", 1, 0, 1, 0.001)) : si.smoo;
    drone_volume = mixGroup(hslider("[1]drone", 1, 0, 1, 0.001)) : si.smoo;
    lead_volume = mixGroup(hslider("[2]lead", 1, 0, 1, 0.001)) : si.smoo;
    pluck_volume = mixGroup(hslider("[3]pluck", 1, 0, 1, 0.001)) : si.smoo;
};