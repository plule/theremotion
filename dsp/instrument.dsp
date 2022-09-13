declare name        "theremotion";
declare version     "1.0";
declare author      "Pierre Lulé";
declare license     "BSD";

import("stdfaust.lib");

// Main voice controls
leadVolume = hslider("[0]vol", 0.0, 0, 1, 0.001) : si.smoo;
chordGroup(x) = vgroup("[3]chord", x);
vol1 = chordGroup(hslider("[1]vol1", 1.0, 0, 1, 0.001)) : si.smoo;
vol2 = chordGroup(hslider("[3]vol2", 0.0, 0, 1, 0.001)) : si.smoo;
vol3 = chordGroup(hslider("[5]vol3", 0.0, 0, 1, 0.001)) : si.smoo;
vol4 = chordGroup(hslider("[7]vol4", 0.0, 0, 1, 0.001)) : si.smoo;
note1 = chordGroup(hslider("[2]note1", 60, 0, 127, 0.001)) : si.smoo;
note2 = chordGroup(hslider("[4]note2", 60, 0, 127, 0.001)) : si.smoo;
note3 = chordGroup(hslider("[6]note3", 60, 0, 127, 0.001)) : si.smoo;
note4 = chordGroup(hslider("[8]note4", 60, 0, 127, 0.001)) : si.smoo;

// Main voice filter
filterGroup(x) = hgroup("[10]filter", x);


// Global
pitchBend = hslider("[3]pitchBend", 0, -1, 1, 0.001) : si.smoo;

// Lead oscillator
supersaw(note) = osc(note) + amount * (osc(note + detune) + osc(note - detune))
with {
    amount = hslider("[0]amount", 0, 0, 1.0, 0.001) : si.smoo;
    detune = hslider("[1]detune", 0.001, 0.001, 0.02, 0.001) : si.smoo;
    osc(note) = os.sawtooth(note : ba.midikey2hz);
};

leadVoiceFilter(note) = ve.moog_vcf_2b(res, cutoffFreq)
with {
    res = hslider("[1]res", 0, 0, 0.99, 0.001) : si.smoo;
    cutoffNote = hslider("[0]cutoffNote", 0, -20, 50, 0.001) : si.smoo;
    cutoffFreq = (note + cutoffNote) : ba.midikey2hz;
};

leadVoice(note, volume) = hgroup("[1]supersaw", supersaw(note))
    : hgroup("[2]filter", leadVoiceFilter(note))
    : _ * volume / 2
with {
    f = note : ba.midikey2hz;
    saw_osc(detune) = os.sawtooth(f * (1 + detune));
    supersaw_osc = saw_osc(detune) + saw_osc(-detune);
};

lead(pitchBend) = leadVolume * (
    (note1 + pitchBend, vol1 : leadVoice) +
    (note2 + pitchBend, vol2 : leadVoice) +
    (note3 + pitchBend, vol3 : leadVoice) +
    (note4 + pitchBend, vol4 : leadVoice)
);

feedback(signal)= signal * 0.005;

// Guitar
elecGuitar(stringLength,pluckPosition,mute,gain,trigger) =
    (pm.elecGuitarModel(stringLength,pluckPosition,mute) : co.compressor_mono(20,-10,0,0.1)) ~
    (_  : ef.gate_mono(-20, 0.0001, 0.1, 0.02)) * 0.005 + pm.pluckString(stringLength,1,1,1,gain,trigger);

guitar(pitchBend) = elecGuitar(length,0.5,mute,strength,gate)
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
process = hgroup("[2]drone", drone)
    + (pitchBend : vgroup("[0]lead", lead))
    + (pitchBend : hgroup("[1]pluck", guitar))
    : ef.echo(1.0, 0.3, 0.3) <: _, _;
