declare name        "osc";
declare version     "1.0";
declare author      "Grame";
declare license     "BSD";
declare copyright   "(c)GRAME 2009";

//-----------------------------------------------
//      Sinusoidal Oscillator
//-----------------------------------------------

import("stdfaust.lib");

vol = hslider("volume [unit:dB]", -25, -96, 0, 0.1) : ba.db2linear : si.smoo;
freq = hslider("freq [unit:Hz]", 440, 20, 24000, 1);
cutoff = hslider("cutoff [unit:Hz]", 440, 20, 24000, 1) : si.smoo;
q = hslider("res", 1, 1, 30, 0.01);

process = sy.dubDub(freq,cutoff,q,1) * vol;

