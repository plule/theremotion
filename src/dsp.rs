mod dsp {
    #![allow(unused_parens)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(non_upper_case_globals)]

use faust_types::*;


fn mydsp_faustpower2_f(value: F32) -> F32 {
	return value * value;
}

#[cfg_attr(feature = "default-boxed", derive(default_boxed::DefaultBoxed))]
#[derive(Debug,Clone)]
pub struct mydsp {
	iVec0: [i32;2],
	fSampleRate: i32,
	fConst0: F32,
	fConst1: F32,
	fConst2: F32,
	fHslider0: F32,
	fRec0: [F32;2],
	fHslider1: F32,
	fRec2: [F32;2],
	fConst3: F32,
	fConst4: F32,
	fRec4: [F32;2],
	fVec1: [F32;2],
	IOTA0: i32,
	fVec2: [F32;4096],
	fConst5: F32,
	fRec3: [F32;2],
	fRec6: [F32;2],
	fVec3: [F32;2],
	fVec4: [F32;4096],
	fRec5: [F32;2],
	fRec8: [F32;2],
	fVec5: [F32;2],
	fVec6: [F32;4096],
	fRec7: [F32;2],
	fRec10: [F32;2],
	fVec7: [F32;2],
	fVec8: [F32;4096],
	fRec9: [F32;2],
	fHslider2: F32,
	fRec11: [F32;2],
	fHslider3: F32,
	fRec12: [F32;2],
	fConst6: F32,
	iConst7: i32,
	fHslider4: F32,
	fRec13: [F32;2],
	fHslider5: F32,
	fRec15: [F32;2],
	fHslider6: F32,
	fConst8: F32,
	fRec14: [F32;2],
	fConst9: F32,
	fRec20: [F32;2],
	fRec18: [F32;2],
	fHslider7: F32,
	fRec21: [F32;2],
	fRec17: [F32;3],
	fRec16: [F32;3],
	fHslider8: F32,
	fRec22: [F32;2],
	fRec27: [F32;2],
	fRec25: [F32;2],
	fHslider9: F32,
	fRec28: [F32;2],
	fRec24: [F32;3],
	fRec23: [F32;3],
	fHslider10: F32,
	fRec29: [F32;2],
	fRec34: [F32;2],
	fRec32: [F32;2],
	fHslider11: F32,
	fRec35: [F32;2],
	fRec31: [F32;3],
	fRec30: [F32;3],
	fHslider12: F32,
	fRec36: [F32;2],
	fRec41: [F32;2],
	fRec39: [F32;2],
	fHslider13: F32,
	fRec42: [F32;2],
	fRec38: [F32;3],
	fRec37: [F32;3],
	fHslider14: F32,
	fRec43: [F32;2],
	fHslider15: F32,
	fRec44: [F32;2],
	fButton0: F32,
	fVec9: [F32;2],
	fHslider16: F32,
	fRec47: [F32;2],
	fRec46: [F32;2],
	fHslider17: F32,
	fRec48: [F32;2],
	fConst10: F32,
	fConst11: F32,
	fConst12: F32,
	fRec78: [F32;2],
	fRec82: [F32;2],
	fConst13: F32,
	fConst14: F32,
	fRec87: [F32;2],
	iVec10: [i32;2],
	iConst15: i32,
	iRec88: [i32;2],
	fConst16: F32,
	fRec85: [F32;2],
	fRec84: [F32;2],
	fRec89: [F32;4],
	fRec90: [F32;2048],
	fVec11: [F32;2],
	fConst17: F32,
	fConst18: F32,
	iRec91: [i32;2],
	iRec93: [i32;2],
	fRec92: [F32;3],
	fVec12: [F32;3],
	fRec83: [F32;2048],
	fRec74: [F32;2],
	fRec70: [F32;2],
	fRec66: [F32;2048],
	fRec68: [F32;2],
	fHslider18: F32,
	fRec64: [F32;4],
	fRec59: [F32;2],
	fRec55: [F32;2048],
	fRec53: [F32;2],
	fConst19: F32,
	fRec52: [F32;2],
	fRec51: [F32;2],
	fRec49: [F32;2],
	fRec45: [F32;2],
	fHslider19: F32,
	fRec95: [F32;2],
	fRec125: [F32;2],
	fRec129: [F32;2],
	fButton1: F32,
	fVec13: [F32;2],
	iRec131: [i32;2],
	fRec132: [F32;3],
	fRec133: [F32;4],
	fRec134: [F32;2048],
	fVec14: [F32;2],
	fRec138: [F32;2],
	iVec15: [i32;2],
	iRec139: [i32;2],
	fRec136: [F32;2],
	fRec135: [F32;2],
	fVec16: [F32;3],
	fRec130: [F32;2048],
	fRec121: [F32;2],
	fRec117: [F32;2],
	fRec113: [F32;2048],
	fRec115: [F32;2],
	fRec111: [F32;4],
	fRec106: [F32;2],
	fRec102: [F32;2048],
	fRec100: [F32;2],
	fRec99: [F32;2],
	fRec98: [F32;2],
	fRec96: [F32;2],
	fRec94: [F32;2],
	fButton2: F32,
	fVec17: [F32;2],
	fHslider20: F32,
	fRec142: [F32;2],
	fRec141: [F32;2],
	fRec172: [F32;2],
	fRec176: [F32;2],
	fRec181: [F32;2],
	iVec18: [i32;2],
	iRec182: [i32;2],
	fRec179: [F32;2],
	fRec178: [F32;2],
	fRec183: [F32;4],
	fRec184: [F32;2048],
	fVec19: [F32;2],
	iRec185: [i32;2],
	fRec186: [F32;3],
	fVec20: [F32;3],
	fRec177: [F32;2048],
	fRec168: [F32;2],
	fRec164: [F32;2],
	fRec160: [F32;2048],
	fRec162: [F32;2],
	fRec158: [F32;4],
	fRec153: [F32;2],
	fRec149: [F32;2048],
	fRec147: [F32;2],
	fRec146: [F32;2],
	fRec145: [F32;2],
	fRec143: [F32;2],
	fRec140: [F32;2],
	fButton3: F32,
	fVec21: [F32;2],
	fHslider21: F32,
	fRec189: [F32;2],
	fRec188: [F32;2],
	fRec219: [F32;2],
	fRec223: [F32;2],
	fRec228: [F32;2],
	iVec22: [i32;2],
	iRec229: [i32;2],
	fRec226: [F32;2],
	fRec225: [F32;2],
	fRec230: [F32;4],
	fRec231: [F32;2048],
	fVec23: [F32;2],
	iRec232: [i32;2],
	fRec233: [F32;3],
	fVec24: [F32;3],
	fRec224: [F32;2048],
	fRec215: [F32;2],
	fRec211: [F32;2],
	fRec207: [F32;2048],
	fRec209: [F32;2],
	fRec205: [F32;4],
	fRec200: [F32;2],
	fRec196: [F32;2048],
	fRec194: [F32;2],
	fRec193: [F32;2],
	fRec192: [F32;2],
	fRec190: [F32;2],
	fRec187: [F32;2],
	fButton4: F32,
	fVec25: [F32;2],
	fHslider22: F32,
	fRec236: [F32;2],
	fRec235: [F32;2],
	fRec266: [F32;2],
	fRec270: [F32;2],
	fRec275: [F32;2],
	iVec26: [i32;2],
	iRec276: [i32;2],
	fRec273: [F32;2],
	fRec272: [F32;2],
	fRec277: [F32;4],
	fRec278: [F32;2048],
	fVec27: [F32;2],
	iRec279: [i32;2],
	fRec280: [F32;3],
	fVec28: [F32;3],
	fRec271: [F32;2048],
	fRec262: [F32;2],
	fRec258: [F32;2],
	fRec254: [F32;2048],
	fRec256: [F32;2],
	fRec252: [F32;4],
	fRec247: [F32;2],
	fRec243: [F32;2048],
	fRec241: [F32;2],
	fRec240: [F32;2],
	fRec239: [F32;2],
	fRec237: [F32;2],
	fRec234: [F32;2],
	fHslider23: F32,
	fRec281: [F32;2],
	fRec1: [F32;262144],
}

impl FaustDsp for mydsp {
	type T = F32;
		
	fn new() -> mydsp { 
		mydsp {
			iVec0: [0;2],
			fSampleRate: 0,
			fConst0: 0.0,
			fConst1: 0.0,
			fConst2: 0.0,
			fHslider0: 0.0,
			fRec0: [0.0;2],
			fHslider1: 0.0,
			fRec2: [0.0;2],
			fConst3: 0.0,
			fConst4: 0.0,
			fRec4: [0.0;2],
			fVec1: [0.0;2],
			IOTA0: 0,
			fVec2: [0.0;4096],
			fConst5: 0.0,
			fRec3: [0.0;2],
			fRec6: [0.0;2],
			fVec3: [0.0;2],
			fVec4: [0.0;4096],
			fRec5: [0.0;2],
			fRec8: [0.0;2],
			fVec5: [0.0;2],
			fVec6: [0.0;4096],
			fRec7: [0.0;2],
			fRec10: [0.0;2],
			fVec7: [0.0;2],
			fVec8: [0.0;4096],
			fRec9: [0.0;2],
			fHslider2: 0.0,
			fRec11: [0.0;2],
			fHslider3: 0.0,
			fRec12: [0.0;2],
			fConst6: 0.0,
			iConst7: 0,
			fHslider4: 0.0,
			fRec13: [0.0;2],
			fHslider5: 0.0,
			fRec15: [0.0;2],
			fHslider6: 0.0,
			fConst8: 0.0,
			fRec14: [0.0;2],
			fConst9: 0.0,
			fRec20: [0.0;2],
			fRec18: [0.0;2],
			fHslider7: 0.0,
			fRec21: [0.0;2],
			fRec17: [0.0;3],
			fRec16: [0.0;3],
			fHslider8: 0.0,
			fRec22: [0.0;2],
			fRec27: [0.0;2],
			fRec25: [0.0;2],
			fHslider9: 0.0,
			fRec28: [0.0;2],
			fRec24: [0.0;3],
			fRec23: [0.0;3],
			fHslider10: 0.0,
			fRec29: [0.0;2],
			fRec34: [0.0;2],
			fRec32: [0.0;2],
			fHslider11: 0.0,
			fRec35: [0.0;2],
			fRec31: [0.0;3],
			fRec30: [0.0;3],
			fHslider12: 0.0,
			fRec36: [0.0;2],
			fRec41: [0.0;2],
			fRec39: [0.0;2],
			fHslider13: 0.0,
			fRec42: [0.0;2],
			fRec38: [0.0;3],
			fRec37: [0.0;3],
			fHslider14: 0.0,
			fRec43: [0.0;2],
			fHslider15: 0.0,
			fRec44: [0.0;2],
			fButton0: 0.0,
			fVec9: [0.0;2],
			fHslider16: 0.0,
			fRec47: [0.0;2],
			fRec46: [0.0;2],
			fHslider17: 0.0,
			fRec48: [0.0;2],
			fConst10: 0.0,
			fConst11: 0.0,
			fConst12: 0.0,
			fRec78: [0.0;2],
			fRec82: [0.0;2],
			fConst13: 0.0,
			fConst14: 0.0,
			fRec87: [0.0;2],
			iVec10: [0;2],
			iConst15: 0,
			iRec88: [0;2],
			fConst16: 0.0,
			fRec85: [0.0;2],
			fRec84: [0.0;2],
			fRec89: [0.0;4],
			fRec90: [0.0;2048],
			fVec11: [0.0;2],
			fConst17: 0.0,
			fConst18: 0.0,
			iRec91: [0;2],
			iRec93: [0;2],
			fRec92: [0.0;3],
			fVec12: [0.0;3],
			fRec83: [0.0;2048],
			fRec74: [0.0;2],
			fRec70: [0.0;2],
			fRec66: [0.0;2048],
			fRec68: [0.0;2],
			fHslider18: 0.0,
			fRec64: [0.0;4],
			fRec59: [0.0;2],
			fRec55: [0.0;2048],
			fRec53: [0.0;2],
			fConst19: 0.0,
			fRec52: [0.0;2],
			fRec51: [0.0;2],
			fRec49: [0.0;2],
			fRec45: [0.0;2],
			fHslider19: 0.0,
			fRec95: [0.0;2],
			fRec125: [0.0;2],
			fRec129: [0.0;2],
			fButton1: 0.0,
			fVec13: [0.0;2],
			iRec131: [0;2],
			fRec132: [0.0;3],
			fRec133: [0.0;4],
			fRec134: [0.0;2048],
			fVec14: [0.0;2],
			fRec138: [0.0;2],
			iVec15: [0;2],
			iRec139: [0;2],
			fRec136: [0.0;2],
			fRec135: [0.0;2],
			fVec16: [0.0;3],
			fRec130: [0.0;2048],
			fRec121: [0.0;2],
			fRec117: [0.0;2],
			fRec113: [0.0;2048],
			fRec115: [0.0;2],
			fRec111: [0.0;4],
			fRec106: [0.0;2],
			fRec102: [0.0;2048],
			fRec100: [0.0;2],
			fRec99: [0.0;2],
			fRec98: [0.0;2],
			fRec96: [0.0;2],
			fRec94: [0.0;2],
			fButton2: 0.0,
			fVec17: [0.0;2],
			fHslider20: 0.0,
			fRec142: [0.0;2],
			fRec141: [0.0;2],
			fRec172: [0.0;2],
			fRec176: [0.0;2],
			fRec181: [0.0;2],
			iVec18: [0;2],
			iRec182: [0;2],
			fRec179: [0.0;2],
			fRec178: [0.0;2],
			fRec183: [0.0;4],
			fRec184: [0.0;2048],
			fVec19: [0.0;2],
			iRec185: [0;2],
			fRec186: [0.0;3],
			fVec20: [0.0;3],
			fRec177: [0.0;2048],
			fRec168: [0.0;2],
			fRec164: [0.0;2],
			fRec160: [0.0;2048],
			fRec162: [0.0;2],
			fRec158: [0.0;4],
			fRec153: [0.0;2],
			fRec149: [0.0;2048],
			fRec147: [0.0;2],
			fRec146: [0.0;2],
			fRec145: [0.0;2],
			fRec143: [0.0;2],
			fRec140: [0.0;2],
			fButton3: 0.0,
			fVec21: [0.0;2],
			fHslider21: 0.0,
			fRec189: [0.0;2],
			fRec188: [0.0;2],
			fRec219: [0.0;2],
			fRec223: [0.0;2],
			fRec228: [0.0;2],
			iVec22: [0;2],
			iRec229: [0;2],
			fRec226: [0.0;2],
			fRec225: [0.0;2],
			fRec230: [0.0;4],
			fRec231: [0.0;2048],
			fVec23: [0.0;2],
			iRec232: [0;2],
			fRec233: [0.0;3],
			fVec24: [0.0;3],
			fRec224: [0.0;2048],
			fRec215: [0.0;2],
			fRec211: [0.0;2],
			fRec207: [0.0;2048],
			fRec209: [0.0;2],
			fRec205: [0.0;4],
			fRec200: [0.0;2],
			fRec196: [0.0;2048],
			fRec194: [0.0;2],
			fRec193: [0.0;2],
			fRec192: [0.0;2],
			fRec190: [0.0;2],
			fRec187: [0.0;2],
			fButton4: 0.0,
			fVec25: [0.0;2],
			fHslider22: 0.0,
			fRec236: [0.0;2],
			fRec235: [0.0;2],
			fRec266: [0.0;2],
			fRec270: [0.0;2],
			fRec275: [0.0;2],
			iVec26: [0;2],
			iRec276: [0;2],
			fRec273: [0.0;2],
			fRec272: [0.0;2],
			fRec277: [0.0;4],
			fRec278: [0.0;2048],
			fVec27: [0.0;2],
			iRec279: [0;2],
			fRec280: [0.0;3],
			fVec28: [0.0;3],
			fRec271: [0.0;2048],
			fRec262: [0.0;2],
			fRec258: [0.0;2],
			fRec254: [0.0;2048],
			fRec256: [0.0;2],
			fRec252: [0.0;4],
			fRec247: [0.0;2],
			fRec243: [0.0;2048],
			fRec241: [0.0;2],
			fRec240: [0.0;2],
			fRec239: [0.0;2],
			fRec237: [0.0;2],
			fRec234: [0.0;2],
			fHslider23: 0.0,
			fRec281: [0.0;2],
			fRec1: [0.0;262144],
		}
	}
	fn metadata(&self, m: &mut dyn Meta) { 
		m.declare("analyzers.lib/name", "Faust Analyzer Library");
		m.declare("analyzers.lib/version", "0.1");
		m.declare("author", "Pierre Lulé");
		m.declare("basics.lib/name", "Faust Basic Element Library");
		m.declare("basics.lib/version", "0.1");
		m.declare("compressors.lib/compression_gain_mono:author", "Julius O. Smith III");
		m.declare("compressors.lib/compression_gain_mono:copyright", "Copyright (C) 2014-2020 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("compressors.lib/compression_gain_mono:license", "MIT-style STK-4.3 license");
		m.declare("compressors.lib/compressor_lad_mono:author", "Julius O. Smith III");
		m.declare("compressors.lib/compressor_lad_mono:copyright", "Copyright (C) 2014-2020 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("compressors.lib/compressor_lad_mono:license", "MIT-style STK-4.3 license");
		m.declare("compressors.lib/compressor_mono:author", "Julius O. Smith III");
		m.declare("compressors.lib/compressor_mono:copyright", "Copyright (C) 2014-2020 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("compressors.lib/compressor_mono:license", "MIT-style STK-4.3 license");
		m.declare("compressors.lib/name", "Faust Compressor Effect Library");
		m.declare("compressors.lib/version", "0.1");
		m.declare("delays.lib/name", "Faust Delay Library");
		m.declare("delays.lib/version", "0.1");
		m.declare("envelopes.lib/ar:author", "Yann Orlarey, Stéphane Letz");
		m.declare("envelopes.lib/author", "GRAME");
		m.declare("envelopes.lib/copyright", "GRAME");
		m.declare("envelopes.lib/license", "LGPL with exception");
		m.declare("envelopes.lib/name", "Faust Envelope Library");
		m.declare("envelopes.lib/version", "0.1");
		m.declare("filename", "instrument.dsp");
		m.declare("filters.lib/fir:author", "Julius O. Smith III");
		m.declare("filters.lib/fir:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/fir:license", "MIT-style STK-4.3 license");
		m.declare("filters.lib/iir:author", "Julius O. Smith III");
		m.declare("filters.lib/iir:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/iir:license", "MIT-style STK-4.3 license");
		m.declare("filters.lib/lowpass0_highpass1", "MIT-style STK-4.3 license");
		m.declare("filters.lib/lowpass0_highpass1:author", "Julius O. Smith III");
		m.declare("filters.lib/lowpass:author", "Julius O. Smith III");
		m.declare("filters.lib/lowpass:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/lowpass:license", "MIT-style STK-4.3 license");
		m.declare("filters.lib/name", "Faust Filters Library");
		m.declare("filters.lib/pole:author", "Julius O. Smith III");
		m.declare("filters.lib/pole:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/pole:license", "MIT-style STK-4.3 license");
		m.declare("filters.lib/tf1:author", "Julius O. Smith III");
		m.declare("filters.lib/tf1:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf1:license", "MIT-style STK-4.3 license");
		m.declare("filters.lib/tf1s:author", "Julius O. Smith III");
		m.declare("filters.lib/tf1s:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf1s:license", "MIT-style STK-4.3 license");
		m.declare("filters.lib/tf2:author", "Julius O. Smith III");
		m.declare("filters.lib/tf2:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf2:license", "MIT-style STK-4.3 license");
		m.declare("filters.lib/tf2s:author", "Julius O. Smith III");
		m.declare("filters.lib/tf2s:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf2s:license", "MIT-style STK-4.3 license");
		m.declare("filters.lib/version", "0.3");
		m.declare("license", "BSD");
		m.declare("maths.lib/author", "GRAME");
		m.declare("maths.lib/copyright", "GRAME");
		m.declare("maths.lib/license", "LGPL with exception");
		m.declare("maths.lib/name", "Faust Math Library");
		m.declare("maths.lib/version", "2.3");
		m.declare("misceffects.lib/name", "Misc Effects Library");
		m.declare("misceffects.lib/version", "2.0");
		m.declare("name", "theremotion");
		m.declare("noises.lib/name", "Faust Noise Generator Library");
		m.declare("noises.lib/version", "0.0");
		m.declare("oscillators.lib/name", "Faust Oscillator Library");
		m.declare("oscillators.lib/version", "0.1");
		m.declare("platform.lib/name", "Generic Platform Library");
		m.declare("platform.lib/version", "0.1");
		m.declare("routes.lib/name", "Faust Signal Routing Library");
		m.declare("routes.lib/version", "0.2");
		m.declare("signals.lib/name", "Faust Signal Routing Library");
		m.declare("signals.lib/version", "0.0");
		m.declare("vaeffects.lib/moog_vcf_2b:author", "Julius O. Smith III");
		m.declare("vaeffects.lib/moog_vcf_2b:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("vaeffects.lib/moog_vcf_2b:license", "MIT-style STK-4.3 license");
		m.declare("vaeffects.lib/name", "Faust Virtual Analog Filter Effect Library");
		m.declare("vaeffects.lib/version", "0.0");
		m.declare("version", "1.0");
	}

	fn get_sample_rate(&self) -> i32 {
		return self.fSampleRate;
	}
	fn get_num_inputs(&self) -> i32 {
		return 0;
	}
	fn get_num_outputs(&self) -> i32 {
		return 2;
	}
	
	fn class_init(sample_rate: i32) {
	}
	fn instance_reset_params(&mut self) {
		self.fHslider0 = 1.0;
		self.fHslider1 = 6e+01;
		self.fHslider2 = 1.0;
		self.fHslider3 = 0.0;
		self.fHslider4 = 0.0;
		self.fHslider5 = 0.0;
		self.fHslider6 = 6e+01;
		self.fHslider7 = 0.0;
		self.fHslider8 = 6e+01;
		self.fHslider9 = 0.0;
		self.fHslider10 = 6e+01;
		self.fHslider11 = 0.0;
		self.fHslider12 = 6e+01;
		self.fHslider13 = 0.0;
		self.fHslider14 = 1.0;
		self.fHslider15 = 0.0;
		self.fButton0 = 0.0;
		self.fHslider16 = 8e+01;
		self.fHslider17 = 0.0;
		self.fHslider18 = 1.0;
		self.fHslider19 = 8e+01;
		self.fButton1 = 0.0;
		self.fButton2 = 0.0;
		self.fHslider20 = 8e+01;
		self.fButton3 = 0.0;
		self.fHslider21 = 8e+01;
		self.fButton4 = 0.0;
		self.fHslider22 = 8e+01;
		self.fHslider23 = 1.0;
	}
	fn instance_clear(&mut self) {
		for l0 in 0..2 {
			self.iVec0[(l0) as usize] = 0;
		}
		for l1 in 0..2 {
			self.fRec0[(l1) as usize] = 0.0;
		}
		for l2 in 0..2 {
			self.fRec2[(l2) as usize] = 0.0;
		}
		for l3 in 0..2 {
			self.fRec4[(l3) as usize] = 0.0;
		}
		for l4 in 0..2 {
			self.fVec1[(l4) as usize] = 0.0;
		}
		self.IOTA0 = 0;
		for l5 in 0..4096 {
			self.fVec2[(l5) as usize] = 0.0;
		}
		for l6 in 0..2 {
			self.fRec3[(l6) as usize] = 0.0;
		}
		for l7 in 0..2 {
			self.fRec6[(l7) as usize] = 0.0;
		}
		for l8 in 0..2 {
			self.fVec3[(l8) as usize] = 0.0;
		}
		for l9 in 0..4096 {
			self.fVec4[(l9) as usize] = 0.0;
		}
		for l10 in 0..2 {
			self.fRec5[(l10) as usize] = 0.0;
		}
		for l11 in 0..2 {
			self.fRec8[(l11) as usize] = 0.0;
		}
		for l12 in 0..2 {
			self.fVec5[(l12) as usize] = 0.0;
		}
		for l13 in 0..4096 {
			self.fVec6[(l13) as usize] = 0.0;
		}
		for l14 in 0..2 {
			self.fRec7[(l14) as usize] = 0.0;
		}
		for l15 in 0..2 {
			self.fRec10[(l15) as usize] = 0.0;
		}
		for l16 in 0..2 {
			self.fVec7[(l16) as usize] = 0.0;
		}
		for l17 in 0..4096 {
			self.fVec8[(l17) as usize] = 0.0;
		}
		for l18 in 0..2 {
			self.fRec9[(l18) as usize] = 0.0;
		}
		for l19 in 0..2 {
			self.fRec11[(l19) as usize] = 0.0;
		}
		for l20 in 0..2 {
			self.fRec12[(l20) as usize] = 0.0;
		}
		for l21 in 0..2 {
			self.fRec13[(l21) as usize] = 0.0;
		}
		for l22 in 0..2 {
			self.fRec15[(l22) as usize] = 0.0;
		}
		for l23 in 0..2 {
			self.fRec14[(l23) as usize] = 0.0;
		}
		for l24 in 0..2 {
			self.fRec20[(l24) as usize] = 0.0;
		}
		for l25 in 0..2 {
			self.fRec18[(l25) as usize] = 0.0;
		}
		for l26 in 0..2 {
			self.fRec21[(l26) as usize] = 0.0;
		}
		for l27 in 0..3 {
			self.fRec17[(l27) as usize] = 0.0;
		}
		for l28 in 0..3 {
			self.fRec16[(l28) as usize] = 0.0;
		}
		for l29 in 0..2 {
			self.fRec22[(l29) as usize] = 0.0;
		}
		for l30 in 0..2 {
			self.fRec27[(l30) as usize] = 0.0;
		}
		for l31 in 0..2 {
			self.fRec25[(l31) as usize] = 0.0;
		}
		for l32 in 0..2 {
			self.fRec28[(l32) as usize] = 0.0;
		}
		for l33 in 0..3 {
			self.fRec24[(l33) as usize] = 0.0;
		}
		for l34 in 0..3 {
			self.fRec23[(l34) as usize] = 0.0;
		}
		for l35 in 0..2 {
			self.fRec29[(l35) as usize] = 0.0;
		}
		for l36 in 0..2 {
			self.fRec34[(l36) as usize] = 0.0;
		}
		for l37 in 0..2 {
			self.fRec32[(l37) as usize] = 0.0;
		}
		for l38 in 0..2 {
			self.fRec35[(l38) as usize] = 0.0;
		}
		for l39 in 0..3 {
			self.fRec31[(l39) as usize] = 0.0;
		}
		for l40 in 0..3 {
			self.fRec30[(l40) as usize] = 0.0;
		}
		for l41 in 0..2 {
			self.fRec36[(l41) as usize] = 0.0;
		}
		for l42 in 0..2 {
			self.fRec41[(l42) as usize] = 0.0;
		}
		for l43 in 0..2 {
			self.fRec39[(l43) as usize] = 0.0;
		}
		for l44 in 0..2 {
			self.fRec42[(l44) as usize] = 0.0;
		}
		for l45 in 0..3 {
			self.fRec38[(l45) as usize] = 0.0;
		}
		for l46 in 0..3 {
			self.fRec37[(l46) as usize] = 0.0;
		}
		for l47 in 0..2 {
			self.fRec43[(l47) as usize] = 0.0;
		}
		for l48 in 0..2 {
			self.fRec44[(l48) as usize] = 0.0;
		}
		for l49 in 0..2 {
			self.fVec9[(l49) as usize] = 0.0;
		}
		for l50 in 0..2 {
			self.fRec47[(l50) as usize] = 0.0;
		}
		for l51 in 0..2 {
			self.fRec46[(l51) as usize] = 0.0;
		}
		for l52 in 0..2 {
			self.fRec48[(l52) as usize] = 0.0;
		}
		for l53 in 0..2 {
			self.fRec78[(l53) as usize] = 0.0;
		}
		for l54 in 0..2 {
			self.fRec82[(l54) as usize] = 0.0;
		}
		for l55 in 0..2 {
			self.fRec87[(l55) as usize] = 0.0;
		}
		for l56 in 0..2 {
			self.iVec10[(l56) as usize] = 0;
		}
		for l57 in 0..2 {
			self.iRec88[(l57) as usize] = 0;
		}
		for l58 in 0..2 {
			self.fRec85[(l58) as usize] = 0.0;
		}
		for l59 in 0..2 {
			self.fRec84[(l59) as usize] = 0.0;
		}
		for l60 in 0..4 {
			self.fRec89[(l60) as usize] = 0.0;
		}
		for l61 in 0..2048 {
			self.fRec90[(l61) as usize] = 0.0;
		}
		for l62 in 0..2 {
			self.fVec11[(l62) as usize] = 0.0;
		}
		for l63 in 0..2 {
			self.iRec91[(l63) as usize] = 0;
		}
		for l64 in 0..2 {
			self.iRec93[(l64) as usize] = 0;
		}
		for l65 in 0..3 {
			self.fRec92[(l65) as usize] = 0.0;
		}
		for l66 in 0..3 {
			self.fVec12[(l66) as usize] = 0.0;
		}
		for l67 in 0..2048 {
			self.fRec83[(l67) as usize] = 0.0;
		}
		for l68 in 0..2 {
			self.fRec74[(l68) as usize] = 0.0;
		}
		for l69 in 0..2 {
			self.fRec70[(l69) as usize] = 0.0;
		}
		for l70 in 0..2048 {
			self.fRec66[(l70) as usize] = 0.0;
		}
		for l71 in 0..2 {
			self.fRec68[(l71) as usize] = 0.0;
		}
		for l72 in 0..4 {
			self.fRec64[(l72) as usize] = 0.0;
		}
		for l73 in 0..2 {
			self.fRec59[(l73) as usize] = 0.0;
		}
		for l74 in 0..2048 {
			self.fRec55[(l74) as usize] = 0.0;
		}
		for l75 in 0..2 {
			self.fRec53[(l75) as usize] = 0.0;
		}
		for l76 in 0..2 {
			self.fRec52[(l76) as usize] = 0.0;
		}
		for l77 in 0..2 {
			self.fRec51[(l77) as usize] = 0.0;
		}
		for l78 in 0..2 {
			self.fRec49[(l78) as usize] = 0.0;
		}
		for l79 in 0..2 {
			self.fRec45[(l79) as usize] = 0.0;
		}
		for l80 in 0..2 {
			self.fRec95[(l80) as usize] = 0.0;
		}
		for l81 in 0..2 {
			self.fRec125[(l81) as usize] = 0.0;
		}
		for l82 in 0..2 {
			self.fRec129[(l82) as usize] = 0.0;
		}
		for l83 in 0..2 {
			self.fVec13[(l83) as usize] = 0.0;
		}
		for l84 in 0..2 {
			self.iRec131[(l84) as usize] = 0;
		}
		for l85 in 0..3 {
			self.fRec132[(l85) as usize] = 0.0;
		}
		for l86 in 0..4 {
			self.fRec133[(l86) as usize] = 0.0;
		}
		for l87 in 0..2048 {
			self.fRec134[(l87) as usize] = 0.0;
		}
		for l88 in 0..2 {
			self.fVec14[(l88) as usize] = 0.0;
		}
		for l89 in 0..2 {
			self.fRec138[(l89) as usize] = 0.0;
		}
		for l90 in 0..2 {
			self.iVec15[(l90) as usize] = 0;
		}
		for l91 in 0..2 {
			self.iRec139[(l91) as usize] = 0;
		}
		for l92 in 0..2 {
			self.fRec136[(l92) as usize] = 0.0;
		}
		for l93 in 0..2 {
			self.fRec135[(l93) as usize] = 0.0;
		}
		for l94 in 0..3 {
			self.fVec16[(l94) as usize] = 0.0;
		}
		for l95 in 0..2048 {
			self.fRec130[(l95) as usize] = 0.0;
		}
		for l96 in 0..2 {
			self.fRec121[(l96) as usize] = 0.0;
		}
		for l97 in 0..2 {
			self.fRec117[(l97) as usize] = 0.0;
		}
		for l98 in 0..2048 {
			self.fRec113[(l98) as usize] = 0.0;
		}
		for l99 in 0..2 {
			self.fRec115[(l99) as usize] = 0.0;
		}
		for l100 in 0..4 {
			self.fRec111[(l100) as usize] = 0.0;
		}
		for l101 in 0..2 {
			self.fRec106[(l101) as usize] = 0.0;
		}
		for l102 in 0..2048 {
			self.fRec102[(l102) as usize] = 0.0;
		}
		for l103 in 0..2 {
			self.fRec100[(l103) as usize] = 0.0;
		}
		for l104 in 0..2 {
			self.fRec99[(l104) as usize] = 0.0;
		}
		for l105 in 0..2 {
			self.fRec98[(l105) as usize] = 0.0;
		}
		for l106 in 0..2 {
			self.fRec96[(l106) as usize] = 0.0;
		}
		for l107 in 0..2 {
			self.fRec94[(l107) as usize] = 0.0;
		}
		for l108 in 0..2 {
			self.fVec17[(l108) as usize] = 0.0;
		}
		for l109 in 0..2 {
			self.fRec142[(l109) as usize] = 0.0;
		}
		for l110 in 0..2 {
			self.fRec141[(l110) as usize] = 0.0;
		}
		for l111 in 0..2 {
			self.fRec172[(l111) as usize] = 0.0;
		}
		for l112 in 0..2 {
			self.fRec176[(l112) as usize] = 0.0;
		}
		for l113 in 0..2 {
			self.fRec181[(l113) as usize] = 0.0;
		}
		for l114 in 0..2 {
			self.iVec18[(l114) as usize] = 0;
		}
		for l115 in 0..2 {
			self.iRec182[(l115) as usize] = 0;
		}
		for l116 in 0..2 {
			self.fRec179[(l116) as usize] = 0.0;
		}
		for l117 in 0..2 {
			self.fRec178[(l117) as usize] = 0.0;
		}
		for l118 in 0..4 {
			self.fRec183[(l118) as usize] = 0.0;
		}
		for l119 in 0..2048 {
			self.fRec184[(l119) as usize] = 0.0;
		}
		for l120 in 0..2 {
			self.fVec19[(l120) as usize] = 0.0;
		}
		for l121 in 0..2 {
			self.iRec185[(l121) as usize] = 0;
		}
		for l122 in 0..3 {
			self.fRec186[(l122) as usize] = 0.0;
		}
		for l123 in 0..3 {
			self.fVec20[(l123) as usize] = 0.0;
		}
		for l124 in 0..2048 {
			self.fRec177[(l124) as usize] = 0.0;
		}
		for l125 in 0..2 {
			self.fRec168[(l125) as usize] = 0.0;
		}
		for l126 in 0..2 {
			self.fRec164[(l126) as usize] = 0.0;
		}
		for l127 in 0..2048 {
			self.fRec160[(l127) as usize] = 0.0;
		}
		for l128 in 0..2 {
			self.fRec162[(l128) as usize] = 0.0;
		}
		for l129 in 0..4 {
			self.fRec158[(l129) as usize] = 0.0;
		}
		for l130 in 0..2 {
			self.fRec153[(l130) as usize] = 0.0;
		}
		for l131 in 0..2048 {
			self.fRec149[(l131) as usize] = 0.0;
		}
		for l132 in 0..2 {
			self.fRec147[(l132) as usize] = 0.0;
		}
		for l133 in 0..2 {
			self.fRec146[(l133) as usize] = 0.0;
		}
		for l134 in 0..2 {
			self.fRec145[(l134) as usize] = 0.0;
		}
		for l135 in 0..2 {
			self.fRec143[(l135) as usize] = 0.0;
		}
		for l136 in 0..2 {
			self.fRec140[(l136) as usize] = 0.0;
		}
		for l137 in 0..2 {
			self.fVec21[(l137) as usize] = 0.0;
		}
		for l138 in 0..2 {
			self.fRec189[(l138) as usize] = 0.0;
		}
		for l139 in 0..2 {
			self.fRec188[(l139) as usize] = 0.0;
		}
		for l140 in 0..2 {
			self.fRec219[(l140) as usize] = 0.0;
		}
		for l141 in 0..2 {
			self.fRec223[(l141) as usize] = 0.0;
		}
		for l142 in 0..2 {
			self.fRec228[(l142) as usize] = 0.0;
		}
		for l143 in 0..2 {
			self.iVec22[(l143) as usize] = 0;
		}
		for l144 in 0..2 {
			self.iRec229[(l144) as usize] = 0;
		}
		for l145 in 0..2 {
			self.fRec226[(l145) as usize] = 0.0;
		}
		for l146 in 0..2 {
			self.fRec225[(l146) as usize] = 0.0;
		}
		for l147 in 0..4 {
			self.fRec230[(l147) as usize] = 0.0;
		}
		for l148 in 0..2048 {
			self.fRec231[(l148) as usize] = 0.0;
		}
		for l149 in 0..2 {
			self.fVec23[(l149) as usize] = 0.0;
		}
		for l150 in 0..2 {
			self.iRec232[(l150) as usize] = 0;
		}
		for l151 in 0..3 {
			self.fRec233[(l151) as usize] = 0.0;
		}
		for l152 in 0..3 {
			self.fVec24[(l152) as usize] = 0.0;
		}
		for l153 in 0..2048 {
			self.fRec224[(l153) as usize] = 0.0;
		}
		for l154 in 0..2 {
			self.fRec215[(l154) as usize] = 0.0;
		}
		for l155 in 0..2 {
			self.fRec211[(l155) as usize] = 0.0;
		}
		for l156 in 0..2048 {
			self.fRec207[(l156) as usize] = 0.0;
		}
		for l157 in 0..2 {
			self.fRec209[(l157) as usize] = 0.0;
		}
		for l158 in 0..4 {
			self.fRec205[(l158) as usize] = 0.0;
		}
		for l159 in 0..2 {
			self.fRec200[(l159) as usize] = 0.0;
		}
		for l160 in 0..2048 {
			self.fRec196[(l160) as usize] = 0.0;
		}
		for l161 in 0..2 {
			self.fRec194[(l161) as usize] = 0.0;
		}
		for l162 in 0..2 {
			self.fRec193[(l162) as usize] = 0.0;
		}
		for l163 in 0..2 {
			self.fRec192[(l163) as usize] = 0.0;
		}
		for l164 in 0..2 {
			self.fRec190[(l164) as usize] = 0.0;
		}
		for l165 in 0..2 {
			self.fRec187[(l165) as usize] = 0.0;
		}
		for l166 in 0..2 {
			self.fVec25[(l166) as usize] = 0.0;
		}
		for l167 in 0..2 {
			self.fRec236[(l167) as usize] = 0.0;
		}
		for l168 in 0..2 {
			self.fRec235[(l168) as usize] = 0.0;
		}
		for l169 in 0..2 {
			self.fRec266[(l169) as usize] = 0.0;
		}
		for l170 in 0..2 {
			self.fRec270[(l170) as usize] = 0.0;
		}
		for l171 in 0..2 {
			self.fRec275[(l171) as usize] = 0.0;
		}
		for l172 in 0..2 {
			self.iVec26[(l172) as usize] = 0;
		}
		for l173 in 0..2 {
			self.iRec276[(l173) as usize] = 0;
		}
		for l174 in 0..2 {
			self.fRec273[(l174) as usize] = 0.0;
		}
		for l175 in 0..2 {
			self.fRec272[(l175) as usize] = 0.0;
		}
		for l176 in 0..4 {
			self.fRec277[(l176) as usize] = 0.0;
		}
		for l177 in 0..2048 {
			self.fRec278[(l177) as usize] = 0.0;
		}
		for l178 in 0..2 {
			self.fVec27[(l178) as usize] = 0.0;
		}
		for l179 in 0..2 {
			self.iRec279[(l179) as usize] = 0;
		}
		for l180 in 0..3 {
			self.fRec280[(l180) as usize] = 0.0;
		}
		for l181 in 0..3 {
			self.fVec28[(l181) as usize] = 0.0;
		}
		for l182 in 0..2048 {
			self.fRec271[(l182) as usize] = 0.0;
		}
		for l183 in 0..2 {
			self.fRec262[(l183) as usize] = 0.0;
		}
		for l184 in 0..2 {
			self.fRec258[(l184) as usize] = 0.0;
		}
		for l185 in 0..2048 {
			self.fRec254[(l185) as usize] = 0.0;
		}
		for l186 in 0..2 {
			self.fRec256[(l186) as usize] = 0.0;
		}
		for l187 in 0..4 {
			self.fRec252[(l187) as usize] = 0.0;
		}
		for l188 in 0..2 {
			self.fRec247[(l188) as usize] = 0.0;
		}
		for l189 in 0..2048 {
			self.fRec243[(l189) as usize] = 0.0;
		}
		for l190 in 0..2 {
			self.fRec241[(l190) as usize] = 0.0;
		}
		for l191 in 0..2 {
			self.fRec240[(l191) as usize] = 0.0;
		}
		for l192 in 0..2 {
			self.fRec239[(l192) as usize] = 0.0;
		}
		for l193 in 0..2 {
			self.fRec237[(l193) as usize] = 0.0;
		}
		for l194 in 0..2 {
			self.fRec234[(l194) as usize] = 0.0;
		}
		for l195 in 0..2 {
			self.fRec281[(l195) as usize] = 0.0;
		}
		for l196 in 0..262144 {
			self.fRec1[(l196) as usize] = 0.0;
		}
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		self.fConst0 = F32::min(1.92e+05, F32::max(1.0, ((self.fSampleRate) as F32)));
		self.fConst1 = 44.1 / self.fConst0;
		self.fConst2 = 1.0 - self.fConst1;
		self.fConst3 = 0.5 * self.fConst0;
		self.fConst4 = 1.0 / self.fConst0;
		self.fConst5 = 0.25 * self.fConst0;
		self.fConst6 = 352.0 / self.fConst0;
		self.iConst7 = i32::wrapping_add(((F32::min(1e+01 * self.fConst0, F32::max(0.0, self.fConst0))) as i32), 1);
		self.fConst8 = 19404.0 / self.fConst0;
		self.fConst9 = 3.1415927 / self.fConst0;
		self.fConst10 = 2764.6016 / self.fConst0;
		self.fConst11 = 0.00882353 * self.fConst0;
		self.fConst12 = 0.00073529413 * self.fConst0;
		self.fConst13 = F32::exp(0.0 - 1e+04 / self.fConst0);
		self.fConst14 = 1.0 - self.fConst13;
		self.iConst15 = ((0.1 * self.fConst0) as i32);
		self.fConst16 = F32::exp(0.0 - 5e+01 / self.fConst0);
		self.fConst17 = 6911.504 / self.fConst0;
		self.fConst18 = 0.002 * self.fConst0;
		self.fConst19 = F32::exp(0.0 - 1e+01 / self.fConst0);
	}
	fn instance_init(&mut self, sample_rate: i32) {
		self.instance_constants(sample_rate);
		self.instance_reset_params();
		self.instance_clear();
	}
	fn init(&mut self, sample_rate: i32) {
		mydsp::class_init(sample_rate);
		self.instance_init(sample_rate);
	}
	
	fn build_user_interface(&self, ui_interface: &mut dyn UI<Self::T>) {
		Self::build_user_interface_static(ui_interface);
	}
	
	fn build_user_interface_static(ui_interface: &mut dyn UI<Self::T>) {
		ui_interface.open_vertical_box("theremotion");
		ui_interface.declare(None, "0", "");
		ui_interface.open_vertical_box("lead");
		ui_interface.declare(Some(ParamIndex(0)), "0", "");
		ui_interface.add_horizontal_slider("volume", ParamIndex(0), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(1)), "1", "");
		ui_interface.add_horizontal_slider("cutoffNote", ParamIndex(1), 0.0, -2e+01, 5e+01, 0.001);
		ui_interface.declare(Some(ParamIndex(2)), "2", "");
		ui_interface.add_horizontal_slider("res", ParamIndex(2), 0.0, 0.0, 0.99, 0.001);
		ui_interface.declare(None, "3", "");
		ui_interface.open_vertical_box("0");
		ui_interface.declare(Some(ParamIndex(3)), "0", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(3), 6e+01, 0.0, 127.0, 0.001);
		ui_interface.declare(Some(ParamIndex(4)), "1", "");
		ui_interface.add_horizontal_slider("volume", ParamIndex(4), 0.0, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "3", "");
		ui_interface.open_vertical_box("1");
		ui_interface.declare(Some(ParamIndex(5)), "0", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(5), 6e+01, 0.0, 127.0, 0.001);
		ui_interface.declare(Some(ParamIndex(6)), "1", "");
		ui_interface.add_horizontal_slider("volume", ParamIndex(6), 0.0, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "3", "");
		ui_interface.open_vertical_box("2");
		ui_interface.declare(Some(ParamIndex(7)), "0", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(7), 6e+01, 0.0, 127.0, 0.001);
		ui_interface.declare(Some(ParamIndex(8)), "1", "");
		ui_interface.add_horizontal_slider("volume", ParamIndex(8), 0.0, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "3", "");
		ui_interface.open_vertical_box("3");
		ui_interface.declare(Some(ParamIndex(9)), "0", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(9), 6e+01, 0.0, 127.0, 0.001);
		ui_interface.declare(Some(ParamIndex(10)), "1", "");
		ui_interface.add_horizontal_slider("volume", ParamIndex(10), 0.0, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.close_box();
		ui_interface.declare(None, "1", "");
		ui_interface.open_horizontal_box("pluck");
		ui_interface.declare(Some(ParamIndex(11)), "0", "");
		ui_interface.add_button("gate", ParamIndex(11));
		ui_interface.declare(Some(ParamIndex(12)), "1", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(12), 8e+01, 0.0, 127.0, 0.001);
		ui_interface.declare(Some(ParamIndex(13)), "2", "");
		ui_interface.add_horizontal_slider("mute", ParamIndex(13), 1.0, 0.9, 1.0, 0.001);
		ui_interface.declare(None, "3", "");
		ui_interface.open_vertical_box("0");
		ui_interface.declare(Some(ParamIndex(14)), "0", "");
		ui_interface.add_button("gate", ParamIndex(14));
		ui_interface.declare(Some(ParamIndex(15)), "1", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(15), 8e+01, 0.0, 127.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "3", "");
		ui_interface.open_vertical_box("1");
		ui_interface.declare(Some(ParamIndex(16)), "0", "");
		ui_interface.add_button("gate", ParamIndex(16));
		ui_interface.declare(Some(ParamIndex(17)), "1", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(17), 8e+01, 0.0, 127.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "3", "");
		ui_interface.open_vertical_box("2");
		ui_interface.declare(Some(ParamIndex(18)), "0", "");
		ui_interface.add_button("gate", ParamIndex(18));
		ui_interface.declare(Some(ParamIndex(19)), "1", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(19), 8e+01, 0.0, 127.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "3", "");
		ui_interface.open_vertical_box("3");
		ui_interface.declare(Some(ParamIndex(20)), "0", "");
		ui_interface.add_button("gate", ParamIndex(20));
		ui_interface.declare(Some(ParamIndex(21)), "1", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(21), 8e+01, 0.0, 127.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(Some(ParamIndex(22)), "3", "");
		ui_interface.add_horizontal_slider("pitchBend", ParamIndex(22), 0.0, -1.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "2", "");
		ui_interface.open_horizontal_box("drone");
		ui_interface.declare(Some(ParamIndex(23)), "0", "");
		ui_interface.add_horizontal_slider("volume", ParamIndex(23), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(24)), "1", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(24), 6e+01, 0.0, 127.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "3", "");
		ui_interface.open_vertical_box("mix");
		ui_interface.declare(Some(ParamIndex(25)), "0", "");
		ui_interface.add_horizontal_slider("master", ParamIndex(25), 1.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(26)), "1", "");
		ui_interface.add_horizontal_slider("drone", ParamIndex(26), 1.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(27)), "2", "");
		ui_interface.add_horizontal_slider("lead", ParamIndex(27), 1.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(28)), "3", "");
		ui_interface.add_horizontal_slider("pluck", ParamIndex(28), 1.0, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			14 => Some(self.fButton0),
			11 => Some(self.fButton1),
			16 => Some(self.fButton2),
			18 => Some(self.fButton3),
			20 => Some(self.fButton4),
			25 => Some(self.fHslider0),
			24 => Some(self.fHslider1),
			5 => Some(self.fHslider10),
			6 => Some(self.fHslider11),
			3 => Some(self.fHslider12),
			4 => Some(self.fHslider13),
			27 => Some(self.fHslider14),
			0 => Some(self.fHslider15),
			15 => Some(self.fHslider16),
			22 => Some(self.fHslider17),
			13 => Some(self.fHslider18),
			12 => Some(self.fHslider19),
			26 => Some(self.fHslider2),
			17 => Some(self.fHslider20),
			19 => Some(self.fHslider21),
			21 => Some(self.fHslider22),
			28 => Some(self.fHslider23),
			23 => Some(self.fHslider3),
			2 => Some(self.fHslider4),
			1 => Some(self.fHslider5),
			9 => Some(self.fHslider6),
			10 => Some(self.fHslider7),
			7 => Some(self.fHslider8),
			8 => Some(self.fHslider9),
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			14 => { self.fButton0 = value }
			11 => { self.fButton1 = value }
			16 => { self.fButton2 = value }
			18 => { self.fButton3 = value }
			20 => { self.fButton4 = value }
			25 => { self.fHslider0 = value }
			24 => { self.fHslider1 = value }
			5 => { self.fHslider10 = value }
			6 => { self.fHslider11 = value }
			3 => { self.fHslider12 = value }
			4 => { self.fHslider13 = value }
			27 => { self.fHslider14 = value }
			0 => { self.fHslider15 = value }
			15 => { self.fHslider16 = value }
			22 => { self.fHslider17 = value }
			13 => { self.fHslider18 = value }
			12 => { self.fHslider19 = value }
			26 => { self.fHslider2 = value }
			17 => { self.fHslider20 = value }
			19 => { self.fHslider21 = value }
			21 => { self.fHslider22 = value }
			28 => { self.fHslider23 = value }
			23 => { self.fHslider3 = value }
			2 => { self.fHslider4 = value }
			1 => { self.fHslider5 = value }
			9 => { self.fHslider6 = value }
			10 => { self.fHslider7 = value }
			7 => { self.fHslider8 = value }
			8 => { self.fHslider9 = value }
			_ => {}
		}
	}
	
	fn compute(&mut self, count: i32, inputs: &[&[Self::T]], outputs: &mut[&mut[Self::T]]) {
		let (outputs0, outputs1) = if let [outputs0, outputs1, ..] = outputs {
			let outputs0 = outputs0[..count as usize].iter_mut();
			let outputs1 = outputs1[..count as usize].iter_mut();
			(outputs0, outputs1)
		} else {
			panic!("wrong number of outputs");
		};
		let mut fSlow0: F32 = self.fConst1 * self.fHslider0;
		let mut fSlow1: F32 = self.fConst1 * self.fHslider1;
		let mut fSlow2: F32 = self.fConst1 * self.fHslider2;
		let mut fSlow3: F32 = self.fConst1 * self.fHslider3;
		let mut fSlow4: F32 = self.fConst1 * self.fHslider4;
		let mut fSlow5: F32 = self.fConst1 * self.fHslider5;
		let mut fSlow6: F32 = self.fHslider6;
		let mut fSlow7: F32 = self.fConst8 * F32::powf(2.0, 0.083333336 * (fSlow6 + -69.0));
		let mut fSlow8: F32 = self.fConst1 * self.fHslider7;
		let mut fSlow9: F32 = self.fHslider8;
		let mut fSlow10: F32 = self.fConst8 * F32::powf(2.0, 0.083333336 * (fSlow9 + -69.0));
		let mut fSlow11: F32 = self.fConst1 * self.fHslider9;
		let mut fSlow12: F32 = self.fHslider10;
		let mut fSlow13: F32 = self.fConst8 * F32::powf(2.0, 0.083333336 * (fSlow12 + -69.0));
		let mut fSlow14: F32 = self.fConst1 * self.fHslider11;
		let mut fSlow15: F32 = self.fHslider12;
		let mut fSlow16: F32 = self.fConst8 * F32::powf(2.0, 0.083333336 * (fSlow15 + -69.0));
		let mut fSlow17: F32 = self.fConst1 * self.fHslider13;
		let mut fSlow18: F32 = self.fConst1 * self.fHslider14;
		let mut fSlow19: F32 = self.fConst1 * self.fHslider15;
		let mut fSlow20: F32 = self.fButton0;
		let mut iSlow21: i32 = ((1.0 - fSlow20) as i32);
		let mut fSlow22: F32 = self.fHslider16;
		let mut fSlow23: F32 = self.fConst1 * self.fHslider17;
		let mut fSlow24: F32 = self.fHslider18;
		let mut fSlow25: F32 = self.fConst1 * self.fHslider19;
		let mut fSlow26: F32 = self.fButton1;
		let mut fSlow27: F32 = self.fButton2;
		let mut iSlow28: i32 = ((1.0 - fSlow27) as i32);
		let mut fSlow29: F32 = self.fHslider20;
		let mut fSlow30: F32 = self.fButton3;
		let mut iSlow31: i32 = ((1.0 - fSlow30) as i32);
		let mut fSlow32: F32 = self.fHslider21;
		let mut fSlow33: F32 = self.fButton4;
		let mut iSlow34: i32 = ((1.0 - fSlow33) as i32);
		let mut fSlow35: F32 = self.fHslider22;
		let mut fSlow36: F32 = self.fConst1 * self.fHslider23;
		let zipped_iterators = outputs0.zip(outputs1);
		for (output0, output1) in zipped_iterators {
			self.iVec0[0] = 1;
			self.fRec0[0] = fSlow0 + self.fConst2 * self.fRec0[1];
			self.fRec2[0] = fSlow1 + self.fConst2 * self.fRec2[1];
			let mut fTemp0: F32 = F32::powf(2.0, 0.083333336 * (self.fRec2[0] + -61.88));
			let mut fTemp1: F32 = F32::max(4.4e+02 * fTemp0, 23.44895);
			let mut fTemp2: F32 = F32::max(0.0, F32::min(2047.0, self.fConst3 / fTemp1));
			let mut fTemp3: F32 = F32::floor(fTemp2);
			let mut fTemp4: F32 = F32::max(2e+01, F32::abs(fTemp1));
			let mut fTemp5: F32 = self.fRec4[1] + self.fConst4 * fTemp4;
			self.fRec4[0] = fTemp5 - F32::floor(fTemp5);
			let mut fTemp6: F32 = mydsp_faustpower2_f(2.0 * self.fRec4[0] + -1.0);
			self.fVec1[0] = fTemp6;
			let mut fTemp7: F32 = ((self.iVec0[1]) as F32);
			let mut fTemp8: F32 = fTemp7 * (fTemp6 - self.fVec1[1]) / fTemp4;
			self.fVec2[(self.IOTA0 & 4095) as usize] = fTemp8;
			let mut iTemp9: i32 = ((fTemp2) as i32);
			self.fRec3[0] = 0.999 * self.fRec3[1] - self.fConst5 * ((fTemp2 - fTemp3) * self.fVec2[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp9, 1))) & 4095) as usize] - (fTemp8 - self.fVec2[((i32::wrapping_sub(self.IOTA0, iTemp9)) & 4095) as usize] * (fTemp3 + (1.0 - fTemp2))));
			let mut fTemp10: F32 = F32::powf(2.0, 0.083333336 * (self.fRec2[0] + -81.11));
			let mut fTemp11: F32 = F32::max(4.4e+02 * fTemp10, 23.44895);
			let mut fTemp12: F32 = F32::max(2e+01, F32::abs(fTemp11));
			let mut fTemp13: F32 = self.fRec6[1] + self.fConst4 * fTemp12;
			self.fRec6[0] = fTemp13 - F32::floor(fTemp13);
			let mut fTemp14: F32 = mydsp_faustpower2_f(2.0 * self.fRec6[0] + -1.0);
			self.fVec3[0] = fTemp14;
			let mut fTemp15: F32 = fTemp7 * (fTemp14 - self.fVec3[1]) / fTemp12;
			self.fVec4[(self.IOTA0 & 4095) as usize] = fTemp15;
			let mut fTemp16: F32 = F32::max(0.0, F32::min(2047.0, self.fConst3 / fTemp11));
			let mut iTemp17: i32 = ((fTemp16) as i32);
			let mut fTemp18: F32 = F32::floor(fTemp16);
			self.fRec5[0] = 0.999 * self.fRec5[1] - self.fConst5 * (self.fVec4[((i32::wrapping_sub(self.IOTA0, iTemp17)) & 4095) as usize] * (fTemp18 + (1.0 - fTemp16)) - fTemp15 + (fTemp16 - fTemp18) * self.fVec4[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp17, 1))) & 4095) as usize]);
			let mut fTemp19: F32 = F32::powf(2.0, 0.083333336 * (self.fRec2[0] + -56.9));
			let mut fTemp20: F32 = F32::max(4.4e+02 * fTemp19, 23.44895);
			let mut fTemp21: F32 = F32::max(0.0, F32::min(2047.0, self.fConst3 / fTemp20));
			let mut fTemp22: F32 = F32::floor(fTemp21);
			let mut fTemp23: F32 = F32::max(2e+01, F32::abs(fTemp20));
			let mut fTemp24: F32 = self.fRec8[1] + self.fConst4 * fTemp23;
			self.fRec8[0] = fTemp24 - F32::floor(fTemp24);
			let mut fTemp25: F32 = mydsp_faustpower2_f(2.0 * self.fRec8[0] + -1.0);
			self.fVec5[0] = fTemp25;
			let mut fTemp26: F32 = fTemp7 * (fTemp25 - self.fVec5[1]) / fTemp23;
			self.fVec6[(self.IOTA0 & 4095) as usize] = fTemp26;
			let mut iTemp27: i32 = ((fTemp21) as i32);
			self.fRec7[0] = 0.999 * self.fRec7[1] - self.fConst5 * ((fTemp21 - fTemp22) * self.fVec6[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp27, 1))) & 4095) as usize] - (fTemp26 - self.fVec6[((i32::wrapping_sub(self.IOTA0, iTemp27)) & 4095) as usize] * (fTemp22 + (1.0 - fTemp21))));
			let mut fTemp28: F32 = F32::powf(2.0, 0.083333336 * (self.fRec2[0] + -69.0));
			let mut fTemp29: F32 = F32::max(4.4e+02 * fTemp28, 23.44895);
			let mut fTemp30: F32 = F32::max(0.0, F32::min(2047.0, self.fConst3 / fTemp29));
			let mut fTemp31: F32 = F32::floor(fTemp30);
			let mut fTemp32: F32 = F32::max(2e+01, F32::abs(fTemp29));
			let mut fTemp33: F32 = self.fRec10[1] + self.fConst4 * fTemp32;
			self.fRec10[0] = fTemp33 - F32::floor(fTemp33);
			let mut fTemp34: F32 = mydsp_faustpower2_f(2.0 * self.fRec10[0] + -1.0);
			self.fVec7[0] = fTemp34;
			let mut fTemp35: F32 = fTemp7 * (fTemp34 - self.fVec7[1]) / fTemp32;
			self.fVec8[(self.IOTA0 & 4095) as usize] = fTemp35;
			let mut iTemp36: i32 = ((fTemp30) as i32);
			self.fRec9[0] = 0.999 * self.fRec9[1] - self.fConst5 * ((fTemp30 - fTemp31) * self.fVec8[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp36, 1))) & 4095) as usize] - (fTemp35 - self.fVec8[((i32::wrapping_sub(self.IOTA0, iTemp36)) & 4095) as usize] * (fTemp31 + (1.0 - fTemp30))));
			self.fRec11[0] = fSlow2 + self.fConst2 * self.fRec11[1];
			self.fRec12[0] = fSlow3 + self.fConst2 * self.fRec12[1];
			self.fRec13[0] = fSlow4 + self.fConst2 * self.fRec13[1];
			let mut fTemp37: F32 = F32::min(1.4141995, 1.4142135 * self.fRec13[0]);
			let mut fTemp38: F32 = 1.4142135 * fTemp37;
			let mut fTemp39: F32 = 1.0 - fTemp38;
			self.fRec15[0] = fSlow5 + self.fConst2 * self.fRec15[1];
			let mut fTemp40: F32 = self.fRec15[0] + -69.0;
			self.fRec14[0] = self.fConst2 * self.fRec14[1] + self.fConst8 * F32::powf(2.0, 0.083333336 * (fSlow6 + fTemp40));
			let mut fTemp41: F32 = F32::tan(self.fConst9 * F32::max(2e+01, F32::min(1e+04, self.fRec14[0])));
			let mut fTemp42: F32 = 1.0 / fTemp41;
			let mut fTemp43: F32 = 2.0 - fTemp38;
			let mut fTemp44: F32 = mydsp_faustpower2_f(fTemp37);
			let mut fTemp45: F32 = fTemp44 + (fTemp43 + fTemp42) / fTemp41 + fTemp39;
			let mut fTemp46: F32 = 1.0 / mydsp_faustpower2_f(fTemp41);
			let mut fTemp47: F32 = fTemp38 + 2.0;
			let mut fTemp48: F32 = fTemp38 + fTemp44;
			let mut fTemp49: F32 = fTemp48 + (fTemp47 + fTemp42) / fTemp41 + 1.0;
			self.fRec20[0] = fSlow7 + self.fConst2 * self.fRec20[1];
			let mut fTemp50: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec20[0]));
			let mut fTemp51: F32 = self.fRec18[1] + self.fConst4 * fTemp50;
			let mut fTemp52: F32 = fTemp51 + -1.0;
			let mut iTemp53: i32 = ((fTemp52 < 0.0) as i32);
			self.fRec18[0] = if (iTemp53 as i32 != 0) { fTemp51 } else { fTemp52 };
			let mut fThen1: F32 = fTemp51 + (1.0 - self.fConst0 / fTemp50) * fTemp52;
			let mut fRec19: F32 = if (iTemp53 as i32 != 0) { fTemp51 } else { fThen1 };
			self.fRec21[0] = fSlow8 + self.fConst2 * self.fRec21[1];
			self.fRec17[0] = self.fRec21[0] * (2.0 * fRec19 + -1.0) - (self.fRec17[2] * (fTemp48 + (fTemp42 - fTemp47) / fTemp41 + 1.0) + 2.0 * self.fRec17[1] * (fTemp48 + (1.0 - fTemp46))) / fTemp49;
			self.fRec16[0] = (self.fRec17[2] + self.fRec17[0] + 2.0 * self.fRec17[1]) / fTemp49 - (self.fRec16[2] * (fTemp44 + (fTemp42 - fTemp43) / fTemp41 + fTemp39) + 2.0 * self.fRec16[1] * (fTemp44 + (1.0 - (fTemp38 + fTemp46)))) / fTemp45;
			self.fRec22[0] = self.fConst2 * self.fRec22[1] + self.fConst8 * F32::powf(2.0, 0.083333336 * (fSlow9 + fTemp40));
			let mut fTemp54: F32 = F32::tan(self.fConst9 * F32::max(2e+01, F32::min(1e+04, self.fRec22[0])));
			let mut fTemp55: F32 = 1.0 / fTemp54;
			let mut fTemp56: F32 = fTemp44 + (fTemp43 + fTemp55) / fTemp54 + fTemp39;
			let mut fTemp57: F32 = 1.0 / mydsp_faustpower2_f(fTemp54);
			let mut fTemp58: F32 = fTemp48 + (fTemp47 + fTemp55) / fTemp54 + 1.0;
			self.fRec27[0] = fSlow10 + self.fConst2 * self.fRec27[1];
			let mut fTemp59: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec27[0]));
			let mut fTemp60: F32 = self.fConst4 * fTemp59;
			let mut fTemp61: F32 = self.fRec25[1] + fTemp60;
			let mut fTemp62: F32 = fTemp61 + -1.0;
			let mut iTemp63: i32 = ((fTemp62 < 0.0) as i32);
			self.fRec25[0] = if (iTemp63 as i32 != 0) { fTemp61 } else { fTemp62 };
			let mut fThen3: F32 = fTemp60 + self.fRec25[1] + (1.0 - self.fConst0 / fTemp59) * fTemp62;
			let mut fRec26: F32 = if (iTemp63 as i32 != 0) { fTemp61 } else { fThen3 };
			self.fRec28[0] = fSlow11 + self.fConst2 * self.fRec28[1];
			self.fRec24[0] = self.fRec28[0] * (2.0 * fRec26 + -1.0) - (self.fRec24[2] * (fTemp48 + (fTemp55 - fTemp47) / fTemp54 + 1.0) + 2.0 * self.fRec24[1] * (fTemp48 + (1.0 - fTemp57))) / fTemp58;
			self.fRec23[0] = (self.fRec24[2] + self.fRec24[0] + 2.0 * self.fRec24[1]) / fTemp58 - (self.fRec23[2] * (fTemp44 + (fTemp55 - fTemp43) / fTemp54 + fTemp39) + 2.0 * self.fRec23[1] * (fTemp44 + (1.0 - (fTemp38 + fTemp57)))) / fTemp56;
			self.fRec29[0] = self.fConst2 * self.fRec29[1] + self.fConst8 * F32::powf(2.0, 0.083333336 * (fSlow12 + fTemp40));
			let mut fTemp64: F32 = F32::tan(self.fConst9 * F32::max(2e+01, F32::min(1e+04, self.fRec29[0])));
			let mut fTemp65: F32 = 1.0 / fTemp64;
			let mut fTemp66: F32 = fTemp44 + (fTemp43 + fTemp65) / fTemp64 + fTemp39;
			let mut fTemp67: F32 = 1.0 / mydsp_faustpower2_f(fTemp64);
			let mut fTemp68: F32 = fTemp48 + (fTemp47 + fTemp65) / fTemp64 + 1.0;
			self.fRec34[0] = fSlow13 + self.fConst2 * self.fRec34[1];
			let mut fTemp69: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec34[0]));
			let mut fTemp70: F32 = self.fRec32[1] + self.fConst4 * fTemp69;
			let mut fTemp71: F32 = fTemp70 + -1.0;
			let mut iTemp72: i32 = ((fTemp71 < 0.0) as i32);
			self.fRec32[0] = if (iTemp72 as i32 != 0) { fTemp70 } else { fTemp71 };
			let mut fThen5: F32 = fTemp70 + (1.0 - self.fConst0 / fTemp69) * fTemp71;
			let mut fRec33: F32 = if (iTemp72 as i32 != 0) { fTemp70 } else { fThen5 };
			self.fRec35[0] = fSlow14 + self.fConst2 * self.fRec35[1];
			self.fRec31[0] = self.fRec35[0] * (2.0 * fRec33 + -1.0) - (self.fRec31[2] * (fTemp48 + (fTemp65 - fTemp47) / fTemp64 + 1.0) + 2.0 * self.fRec31[1] * (fTemp48 + (1.0 - fTemp67))) / fTemp68;
			self.fRec30[0] = (self.fRec31[2] + self.fRec31[0] + 2.0 * self.fRec31[1]) / fTemp68 - (self.fRec30[2] * (fTemp44 + (fTemp65 - fTemp43) / fTemp64 + fTemp39) + 2.0 * self.fRec30[1] * (fTemp44 + (1.0 - (fTemp38 + fTemp67)))) / fTemp66;
			self.fRec36[0] = self.fConst2 * self.fRec36[1] + self.fConst8 * F32::powf(2.0, 0.083333336 * (fSlow15 + fTemp40));
			let mut fTemp73: F32 = F32::tan(self.fConst9 * F32::max(2e+01, F32::min(1e+04, self.fRec36[0])));
			let mut fTemp74: F32 = 1.0 / fTemp73;
			let mut fTemp75: F32 = fTemp44 + (fTemp43 + fTemp74) / fTemp73 + fTemp39;
			let mut fTemp76: F32 = 1.0 / mydsp_faustpower2_f(fTemp73);
			let mut fTemp77: F32 = fTemp48 + (fTemp74 + fTemp47) / fTemp73 + 1.0;
			self.fRec41[0] = fSlow16 + self.fConst2 * self.fRec41[1];
			let mut fTemp78: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec41[0]));
			let mut fTemp79: F32 = self.fRec39[1] + self.fConst4 * fTemp78;
			let mut fTemp80: F32 = fTemp79 + -1.0;
			let mut iTemp81: i32 = ((fTemp80 < 0.0) as i32);
			self.fRec39[0] = if (iTemp81 as i32 != 0) { fTemp79 } else { fTemp80 };
			let mut fThen7: F32 = fTemp79 + (1.0 - self.fConst0 / fTemp78) * fTemp80;
			let mut fRec40: F32 = if (iTemp81 as i32 != 0) { fTemp79 } else { fThen7 };
			self.fRec42[0] = fSlow17 + self.fConst2 * self.fRec42[1];
			self.fRec38[0] = self.fRec42[0] * (2.0 * fRec40 + -1.0) - (self.fRec38[2] * (fTemp48 + (1.0 - (fTemp47 - fTemp74) / fTemp73)) + 2.0 * self.fRec38[1] * (fTemp48 + (1.0 - fTemp76))) / fTemp77;
			self.fRec37[0] = (self.fRec38[2] + self.fRec38[0] + 2.0 * self.fRec38[1]) / fTemp77 - (self.fRec37[2] * (fTemp44 + (fTemp74 - fTemp43) / fTemp73 + fTemp39) + 2.0 * self.fRec37[1] * (fTemp44 + (1.0 - (fTemp38 + fTemp76)))) / fTemp75;
			self.fRec43[0] = fSlow18 + self.fConst2 * self.fRec43[1];
			self.fRec44[0] = fSlow19 + self.fConst2 * self.fRec44[1];
			self.fVec9[0] = fSlow20;
			self.fRec47[0] = if (iSlow21 as i32 != 0) { fSlow22 } else { self.fRec47[1] };
			self.fRec46[0] = self.fConst2 * self.fRec46[1] + self.fConst1 * self.fRec47[0];
			self.fRec48[0] = fSlow23 + self.fConst2 * self.fRec48[1];
			let mut fTemp82: F32 = F32::powf(2.0, 0.083333336 * (self.fRec48[0] + self.fRec46[0] + -69.0));
			let mut fTemp83: F32 = 1.0 / F32::tan(self.fConst10 * fTemp82);
			let mut fRec63: F32 = -1.0 * 0.9973053 * (0.9 * self.fRec64[2] + 0.05 * (self.fRec64[1] + self.fRec64[3]));
			let mut fTemp84: F32 = self.fConst12 * (0.77272725 / fTemp82 + -0.11);
			let mut fTemp85: F32 = fTemp84 + -1.499995;
			let mut iTemp86: i32 = ((fTemp85) as i32);
			let mut iTemp87: i32 = ((F32::min(self.fConst11, ((std::cmp::max(0, i32::wrapping_add(iTemp86, 4))) as F32))) as i32);
			let mut iTemp88: i32 = i32::wrapping_add(iTemp87, 1);
			let mut fTemp89: F32 = F32::floor(fTemp85);
			let mut fTemp90: F32 = fTemp84 + (-3.0 - fTemp89);
			let mut fTemp91: F32 = fTemp84 + (-2.0 - fTemp89);
			let mut fTemp92: F32 = fTemp84 + (-1.0 - fTemp89);
			let mut fTemp93: F32 = fTemp92 * fTemp91;
			let mut fTemp94: F32 = fTemp93 * fTemp90;
			let mut fTemp95: F32 = fTemp84 + (-4.0 - fTemp89);
			let mut fTemp96: F32 = 0.0 - fTemp95;
			let mut iTemp97: i32 = ((F32::min(self.fConst11, ((std::cmp::max(0, i32::wrapping_add(iTemp86, 3))) as F32))) as i32);
			let mut iTemp98: i32 = i32::wrapping_add(iTemp97, 1);
			let mut fTemp99: F32 = 0.0 - 0.5 * fTemp95;
			let mut fTemp100: F32 = 0.0 - fTemp90;
			let mut iTemp101: i32 = ((F32::min(self.fConst11, ((std::cmp::max(0, i32::wrapping_add(iTemp86, 2))) as F32))) as i32);
			let mut iTemp102: i32 = i32::wrapping_add(iTemp101, 1);
			let mut fTemp103: F32 = 0.0 - 0.33333334 * fTemp95;
			let mut fTemp104: F32 = 0.0 - 0.5 * fTemp90;
			let mut fTemp105: F32 = 0.0 - fTemp91;
			let mut iTemp106: i32 = ((F32::min(self.fConst11, ((std::cmp::max(0, i32::wrapping_add(iTemp86, 1))) as F32))) as i32);
			let mut iTemp107: i32 = i32::wrapping_add(iTemp106, 1);
			let mut fTemp108: F32 = fTemp84 - fTemp89;
			let mut fTemp109: F32 = 0.0 - 0.25 * fTemp95;
			let mut fTemp110: F32 = 0.0 - 0.33333334 * fTemp90;
			let mut fTemp111: F32 = 0.0 - 0.5 * fTemp91;
			let mut fTemp112: F32 = 0.0 - fTemp92;
			let mut iTemp113: i32 = ((F32::min(self.fConst11, ((std::cmp::max(0, iTemp86)) as F32))) as i32);
			let mut iTemp114: i32 = i32::wrapping_add(iTemp113, 1);
			self.fRec78[0] = self.fRec55[((i32::wrapping_sub(self.IOTA0, iTemp114)) & 2047) as usize] * fTemp112 * fTemp111 * fTemp110 * fTemp109 + fTemp108 * (self.fRec55[((i32::wrapping_sub(self.IOTA0, iTemp107)) & 2047) as usize] * fTemp105 * fTemp104 * fTemp103 + 0.5 * fTemp92 * self.fRec55[((i32::wrapping_sub(self.IOTA0, iTemp102)) & 2047) as usize] * fTemp100 * fTemp99 + 0.16666667 * fTemp93 * self.fRec55[((i32::wrapping_sub(self.IOTA0, iTemp98)) & 2047) as usize] * fTemp96 + 0.041666668 * fTemp94 * self.fRec55[((i32::wrapping_sub(self.IOTA0, iTemp88)) & 2047) as usize]);
			self.fRec82[0] = 0.05 * self.fRec82[1] + 0.95 * self.fRec78[1];
			let mut fRec79: F32 = self.fRec82[0];
			self.fRec87[0] = self.fConst13 * self.fRec87[1] + self.fConst14 * F32::abs(self.fRec49[1]);
			let mut fRec86: F32 = self.fRec87[0];
			let mut iTemp115: i32 = ((fRec86 > 0.1) as i32);
			self.iVec10[0] = iTemp115;
			self.iRec88[0] = std::cmp::max(i32::wrapping_mul(self.iConst15, ((iTemp115 < self.iVec10[1]) as i32)), i32::wrapping_add(self.iRec88[1], -1));
			let mut fTemp116: F32 = F32::abs(F32::max(((iTemp115) as F32), ((((self.iRec88[0] > 0) as i32)) as F32)));
			let mut fTemp117: F32 = if (((self.fRec84[1] > fTemp116) as i32) as i32 != 0) { self.fConst16 } else { self.fConst13 };
			self.fRec85[0] = self.fRec85[1] * fTemp117 + fTemp116 * (1.0 - fTemp117);
			self.fRec84[0] = self.fRec85[0];
			let mut fTemp118: F32 = 0.005 * self.fRec84[0] * self.fRec49[1];
			self.fRec89[0] = self.fRec53[1];
			self.fRec90[(self.IOTA0 & 2047) as usize] = -1.0 * 0.9973053 * (0.9 * self.fRec89[2] + 0.05 * (self.fRec89[1] + self.fRec89[3]));
			let mut fTemp119: F32 = fTemp93 * fTemp96;
			let mut fTemp120: F32 = fTemp92 * fTemp100 * fTemp99;
			let mut fTemp121: F32 = fTemp105 * fTemp104 * fTemp103;
			let mut fTemp122: F32 = fTemp112 * fTemp111 * fTemp110 * fTemp109;
			self.fVec11[0] = fTemp122 * self.fRec90[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp113, 2))) & 2047) as usize] + fTemp108 * (fTemp121 * self.fRec90[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp106, 2))) & 2047) as usize] + 0.5 * fTemp120 * self.fRec90[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp101, 2))) & 2047) as usize] + 0.16666667 * fTemp119 * self.fRec90[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp97, 2))) & 2047) as usize] + 0.041666668 * fTemp94 * self.fRec90[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp87, 2))) & 2047) as usize]);
			let mut fTemp123: F32 = F32::tan(self.fConst17 * fTemp82);
			let mut fTemp124: F32 = 1.0 / fTemp123;
			let mut fTemp125: F32 = (fTemp124 + 1.4142135) / fTemp123 + 1.0;
			self.iRec91[0] = i32::wrapping_add(i32::wrapping_mul(i32::wrapping_add(self.iRec91[1], ((self.iRec91[1] > 0) as i32)), ((fSlow20 <= self.fVec9[1]) as i32)), ((fSlow20 > self.fVec9[1]) as i32));
			let mut fTemp126: F32 = ((self.iRec91[0]) as F32) / F32::max(1.0, self.fConst18 * mydsp_faustpower2_f(1.0 - 0.22 * fTemp82));
			self.iRec93[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, self.iRec93[1]), 12345);
			let mut fTemp127: F32 = 4.656613e-10 * ((self.iRec93[0]) as F32);
			self.fRec92[0] = fTemp127 - (self.fRec92[2] * ((fTemp124 + -1.4142135) / fTemp123 + 1.0) + 2.0 * self.fRec92[1] * (1.0 - 1.0 / mydsp_faustpower2_f(fTemp123))) / fTemp125;
			let mut fTemp128: F32 = 0.5 * ((self.fRec92[2] + self.fRec92[0] + 2.0 * self.fRec92[1]) * F32::max(0.0, F32::min(fTemp126, 2.0 - fTemp126)) / fTemp125);
			let mut fTemp129: F32 = fTemp128 + self.fVec11[1] + fTemp118;
			self.fVec12[0] = fTemp129;
			self.fRec83[(self.IOTA0 & 2047) as usize] = 0.05 * self.fRec83[((i32::wrapping_sub(self.IOTA0, 1)) & 2047) as usize] + 0.95 * self.fVec12[2];
			let mut fRec80: F32 = fTemp122 * self.fRec83[((i32::wrapping_sub(self.IOTA0, iTemp113)) & 2047) as usize] + fTemp108 * (fTemp121 * self.fRec83[((i32::wrapping_sub(self.IOTA0, iTemp106)) & 2047) as usize] + 0.5 * fTemp120 * self.fRec83[((i32::wrapping_sub(self.IOTA0, iTemp101)) & 2047) as usize] + 0.16666667 * fTemp119 * self.fRec83[((i32::wrapping_sub(self.IOTA0, iTemp97)) & 2047) as usize] + 0.041666668 * fTemp94 * self.fRec83[((i32::wrapping_sub(self.IOTA0, iTemp87)) & 2047) as usize]);
			let mut fRec81: F32 = self.fVec12[1] + self.fRec74[1];
			self.fRec74[0] = fRec79;
			let mut fRec75: F32 = self.fRec74[1];
			let mut fRec76: F32 = fRec80;
			let mut fRec77: F32 = fRec81;
			self.fRec70[0] = fRec75;
			let mut fRec71: F32 = fTemp118 + fTemp128 + self.fRec70[1];
			let mut fRec72: F32 = fRec76;
			let mut fRec73: F32 = fRec77;
			self.fRec66[(self.IOTA0 & 2047) as usize] = fRec71;
			let mut fRec67: F32 = fTemp122 * self.fRec66[((i32::wrapping_sub(self.IOTA0, iTemp114)) & 2047) as usize] + fTemp108 * (fTemp121 * self.fRec66[((i32::wrapping_sub(self.IOTA0, iTemp107)) & 2047) as usize] + 0.5 * fTemp120 * self.fRec66[((i32::wrapping_sub(self.IOTA0, iTemp102)) & 2047) as usize] + 0.16666667 * fTemp119 * self.fRec66[((i32::wrapping_sub(self.IOTA0, iTemp98)) & 2047) as usize] + 0.041666668 * fTemp94 * self.fRec66[((i32::wrapping_sub(self.IOTA0, iTemp88)) & 2047) as usize]);
			self.fRec68[0] = fRec72;
			let mut fRec69: F32 = fRec73;
			self.fRec64[0] = fSlow24 * self.fRec68[1];
			let mut fRec65: F32 = fRec69;
			self.fRec59[0] = fRec63;
			let mut fRec60: F32 = fSlow24 * self.fRec59[1];
			let mut fRec61: F32 = self.fRec64[0];
			let mut fRec62: F32 = fRec65;
			self.fRec55[(self.IOTA0 & 2047) as usize] = fRec60;
			let mut fRec56: F32 = fRec67;
			let mut fRec57: F32 = fRec61;
			let mut fRec58: F32 = fRec62;
			self.fRec53[0] = fRec56;
			let mut fRec54: F32 = fRec58;
			let mut fTemp130: F32 = F32::abs(fRec54);
			let mut fTemp131: F32 = if (((self.fRec51[1] > fTemp130) as i32) as i32 != 0) { self.fConst19 } else { 0.0 };
			self.fRec52[0] = self.fRec52[1] * fTemp131 + fTemp130 * (1.0 - fTemp131);
			self.fRec51[0] = self.fRec52[0];
			let mut fRec50: F32 = 0.0 - 0.95 * F32::max(2e+01 * F32::log10(self.fRec51[0]) + 1e+01, 0.0);
			self.fRec49[0] = fRec54 * F32::powf(1e+01, 0.05 * fRec50);
			self.fRec45[0] = 0.0 - (self.fRec45[1] * (1.0 - fTemp83) - (self.fRec49[0] + self.fRec49[1])) / (fTemp83 + 1.0);
			self.fRec95[0] = fSlow25 + self.fConst2 * self.fRec95[1];
			let mut fTemp132: F32 = F32::powf(2.0, 0.083333336 * (self.fRec95[0] + self.fRec48[0] + -69.0));
			let mut fTemp133: F32 = 1.0 / F32::tan(self.fConst10 * fTemp132);
			let mut fRec110: F32 = -1.0 * 0.9973053 * (0.9 * self.fRec111[2] + 0.05 * (self.fRec111[1] + self.fRec111[3]));
			let mut fTemp134: F32 = self.fConst12 * (0.77272725 / fTemp132 + -0.11);
			let mut fTemp135: F32 = fTemp134 + -1.499995;
			let mut iTemp136: i32 = ((fTemp135) as i32);
			let mut iTemp137: i32 = ((F32::min(self.fConst11, ((std::cmp::max(0, i32::wrapping_add(iTemp136, 4))) as F32))) as i32);
			let mut iTemp138: i32 = i32::wrapping_add(iTemp137, 1);
			let mut fTemp139: F32 = F32::floor(fTemp135);
			let mut fTemp140: F32 = fTemp134 + (-3.0 - fTemp139);
			let mut fTemp141: F32 = fTemp134 + (-2.0 - fTemp139);
			let mut fTemp142: F32 = fTemp134 + (-1.0 - fTemp139);
			let mut fTemp143: F32 = fTemp142 * fTemp141;
			let mut fTemp144: F32 = fTemp143 * fTemp140;
			let mut fTemp145: F32 = fTemp134 + (-4.0 - fTemp139);
			let mut fTemp146: F32 = 0.0 - fTemp145;
			let mut iTemp147: i32 = ((F32::min(self.fConst11, ((std::cmp::max(0, i32::wrapping_add(iTemp136, 3))) as F32))) as i32);
			let mut iTemp148: i32 = i32::wrapping_add(iTemp147, 1);
			let mut fTemp149: F32 = 0.0 - 0.5 * fTemp145;
			let mut fTemp150: F32 = 0.0 - fTemp140;
			let mut iTemp151: i32 = ((F32::min(self.fConst11, ((std::cmp::max(0, i32::wrapping_add(iTemp136, 2))) as F32))) as i32);
			let mut iTemp152: i32 = i32::wrapping_add(iTemp151, 1);
			let mut fTemp153: F32 = 0.0 - 0.33333334 * fTemp145;
			let mut fTemp154: F32 = 0.0 - 0.5 * fTemp140;
			let mut fTemp155: F32 = 0.0 - fTemp141;
			let mut iTemp156: i32 = ((F32::min(self.fConst11, ((std::cmp::max(0, i32::wrapping_add(iTemp136, 1))) as F32))) as i32);
			let mut iTemp157: i32 = i32::wrapping_add(iTemp156, 1);
			let mut fTemp158: F32 = fTemp134 - fTemp139;
			let mut fTemp159: F32 = 0.0 - 0.25 * fTemp145;
			let mut fTemp160: F32 = 0.0 - 0.33333334 * fTemp140;
			let mut fTemp161: F32 = 0.0 - 0.5 * fTemp141;
			let mut fTemp162: F32 = 0.0 - fTemp142;
			let mut iTemp163: i32 = ((F32::min(self.fConst11, ((std::cmp::max(0, iTemp136)) as F32))) as i32);
			let mut iTemp164: i32 = i32::wrapping_add(iTemp163, 1);
			self.fRec125[0] = self.fRec102[((i32::wrapping_sub(self.IOTA0, iTemp164)) & 2047) as usize] * fTemp162 * fTemp161 * fTemp160 * fTemp159 + fTemp158 * (self.fRec102[((i32::wrapping_sub(self.IOTA0, iTemp157)) & 2047) as usize] * fTemp155 * fTemp154 * fTemp153 + 0.5 * fTemp142 * self.fRec102[((i32::wrapping_sub(self.IOTA0, iTemp152)) & 2047) as usize] * fTemp150 * fTemp149 + 0.16666667 * fTemp143 * self.fRec102[((i32::wrapping_sub(self.IOTA0, iTemp148)) & 2047) as usize] * fTemp146 + 0.041666668 * fTemp144 * self.fRec102[((i32::wrapping_sub(self.IOTA0, iTemp138)) & 2047) as usize]);
			self.fRec129[0] = 0.05 * self.fRec129[1] + 0.95 * self.fRec125[1];
			let mut fRec126: F32 = self.fRec129[0];
			let mut fTemp165: F32 = F32::tan(self.fConst17 * fTemp132);
			let mut fTemp166: F32 = 1.0 / fTemp165;
			let mut fTemp167: F32 = (fTemp166 + 1.4142135) / fTemp165 + 1.0;
			self.fVec13[0] = fSlow26;
			self.iRec131[0] = i32::wrapping_add(i32::wrapping_mul(i32::wrapping_add(self.iRec131[1], ((self.iRec131[1] > 0) as i32)), ((fSlow26 <= self.fVec13[1]) as i32)), ((fSlow26 > self.fVec13[1]) as i32));
			let mut fTemp168: F32 = ((self.iRec131[0]) as F32) / F32::max(1.0, self.fConst18 * mydsp_faustpower2_f(1.0 - 0.22 * fTemp132));
			self.fRec132[0] = fTemp127 - (self.fRec132[2] * ((fTemp166 + -1.4142135) / fTemp165 + 1.0) + 2.0 * self.fRec132[1] * (1.0 - 1.0 / mydsp_faustpower2_f(fTemp165))) / fTemp167;
			let mut fTemp169: F32 = 0.5 * ((self.fRec132[2] + self.fRec132[0] + 2.0 * self.fRec132[1]) * F32::max(0.0, F32::min(fTemp168, 2.0 - fTemp168)) / fTemp167);
			self.fRec133[0] = self.fRec100[1];
			self.fRec134[(self.IOTA0 & 2047) as usize] = -1.0 * 0.9973053 * (0.9 * self.fRec133[2] + 0.05 * (self.fRec133[1] + self.fRec133[3]));
			let mut fTemp170: F32 = fTemp143 * fTemp146;
			let mut fTemp171: F32 = fTemp142 * fTemp150 * fTemp149;
			let mut fTemp172: F32 = fTemp155 * fTemp154 * fTemp153;
			let mut fTemp173: F32 = fTemp162 * fTemp161 * fTemp160 * fTemp159;
			self.fVec14[0] = fTemp173 * self.fRec134[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp163, 2))) & 2047) as usize] + fTemp158 * (fTemp172 * self.fRec134[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp156, 2))) & 2047) as usize] + 0.5 * fTemp171 * self.fRec134[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp151, 2))) & 2047) as usize] + 0.16666667 * fTemp170 * self.fRec134[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp147, 2))) & 2047) as usize] + 0.041666668 * fTemp144 * self.fRec134[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp137, 2))) & 2047) as usize]);
			self.fRec138[0] = self.fConst13 * self.fRec138[1] + self.fConst14 * F32::abs(self.fRec96[1]);
			let mut fRec137: F32 = self.fRec138[0];
			let mut iTemp174: i32 = ((fRec137 > 0.1) as i32);
			self.iVec15[0] = iTemp174;
			self.iRec139[0] = std::cmp::max(i32::wrapping_mul(self.iConst15, ((iTemp174 < self.iVec15[1]) as i32)), i32::wrapping_add(self.iRec139[1], -1));
			let mut fTemp175: F32 = F32::abs(F32::max(((iTemp174) as F32), ((((self.iRec139[0] > 0) as i32)) as F32)));
			let mut fTemp176: F32 = if (((self.fRec135[1] > fTemp175) as i32) as i32 != 0) { self.fConst16 } else { self.fConst13 };
			self.fRec136[0] = self.fRec136[1] * fTemp176 + fTemp175 * (1.0 - fTemp176);
			self.fRec135[0] = self.fRec136[0];
			let mut fTemp177: F32 = 0.005 * self.fRec135[0] * self.fRec96[1];
			let mut fTemp178: F32 = fTemp177 + self.fVec14[1] + fTemp169;
			self.fVec16[0] = fTemp178;
			self.fRec130[(self.IOTA0 & 2047) as usize] = 0.05 * self.fRec130[((i32::wrapping_sub(self.IOTA0, 1)) & 2047) as usize] + 0.95 * self.fVec16[2];
			let mut fRec127: F32 = fTemp173 * self.fRec130[((i32::wrapping_sub(self.IOTA0, iTemp163)) & 2047) as usize] + fTemp158 * (fTemp172 * self.fRec130[((i32::wrapping_sub(self.IOTA0, iTemp156)) & 2047) as usize] + 0.5 * fTemp171 * self.fRec130[((i32::wrapping_sub(self.IOTA0, iTemp151)) & 2047) as usize] + 0.16666667 * fTemp170 * self.fRec130[((i32::wrapping_sub(self.IOTA0, iTemp147)) & 2047) as usize] + 0.041666668 * fTemp144 * self.fRec130[((i32::wrapping_sub(self.IOTA0, iTemp137)) & 2047) as usize]);
			let mut fRec128: F32 = self.fVec16[1] + self.fRec121[1];
			self.fRec121[0] = fRec126;
			let mut fRec122: F32 = self.fRec121[1];
			let mut fRec123: F32 = fRec127;
			let mut fRec124: F32 = fRec128;
			self.fRec117[0] = fRec122;
			let mut fRec118: F32 = fTemp177 + fTemp169 + self.fRec117[1];
			let mut fRec119: F32 = fRec123;
			let mut fRec120: F32 = fRec124;
			self.fRec113[(self.IOTA0 & 2047) as usize] = fRec118;
			let mut fRec114: F32 = fTemp173 * self.fRec113[((i32::wrapping_sub(self.IOTA0, iTemp164)) & 2047) as usize] + fTemp158 * (fTemp172 * self.fRec113[((i32::wrapping_sub(self.IOTA0, iTemp157)) & 2047) as usize] + 0.5 * fTemp171 * self.fRec113[((i32::wrapping_sub(self.IOTA0, iTemp152)) & 2047) as usize] + 0.16666667 * fTemp170 * self.fRec113[((i32::wrapping_sub(self.IOTA0, iTemp148)) & 2047) as usize] + 0.041666668 * fTemp144 * self.fRec113[((i32::wrapping_sub(self.IOTA0, iTemp138)) & 2047) as usize]);
			self.fRec115[0] = fRec119;
			let mut fRec116: F32 = fRec120;
			self.fRec111[0] = fSlow24 * self.fRec115[1];
			let mut fRec112: F32 = fRec116;
			self.fRec106[0] = fRec110;
			let mut fRec107: F32 = fSlow24 * self.fRec106[1];
			let mut fRec108: F32 = self.fRec111[0];
			let mut fRec109: F32 = fRec112;
			self.fRec102[(self.IOTA0 & 2047) as usize] = fRec107;
			let mut fRec103: F32 = fRec114;
			let mut fRec104: F32 = fRec108;
			let mut fRec105: F32 = fRec109;
			self.fRec100[0] = fRec103;
			let mut fRec101: F32 = fRec105;
			let mut fTemp179: F32 = F32::abs(fRec101);
			let mut fTemp180: F32 = if (((self.fRec98[1] > fTemp179) as i32) as i32 != 0) { self.fConst19 } else { 0.0 };
			self.fRec99[0] = self.fRec99[1] * fTemp180 + fTemp179 * (1.0 - fTemp180);
			self.fRec98[0] = self.fRec99[0];
			let mut fRec97: F32 = 0.0 - 0.95 * F32::max(2e+01 * F32::log10(self.fRec98[0]) + 1e+01, 0.0);
			self.fRec96[0] = fRec101 * F32::powf(1e+01, 0.05 * fRec97);
			self.fRec94[0] = 0.0 - (self.fRec94[1] * (1.0 - fTemp133) - (self.fRec96[0] + self.fRec96[1])) / (fTemp133 + 1.0);
			self.fVec17[0] = fSlow27;
			self.fRec142[0] = if (iSlow28 as i32 != 0) { fSlow29 } else { self.fRec142[1] };
			self.fRec141[0] = self.fConst2 * self.fRec141[1] + self.fConst1 * self.fRec142[0];
			let mut fTemp181: F32 = F32::powf(2.0, 0.083333336 * (self.fRec48[0] + self.fRec141[0] + -69.0));
			let mut fTemp182: F32 = 1.0 / F32::tan(self.fConst10 * fTemp181);
			let mut fRec157: F32 = -1.0 * 0.9973053 * (0.9 * self.fRec158[2] + 0.05 * (self.fRec158[1] + self.fRec158[3]));
			let mut fTemp183: F32 = self.fConst12 * (0.77272725 / fTemp181 + -0.11);
			let mut fTemp184: F32 = fTemp183 + -1.499995;
			let mut iTemp185: i32 = ((fTemp184) as i32);
			let mut iTemp186: i32 = ((F32::min(self.fConst11, ((std::cmp::max(0, i32::wrapping_add(iTemp185, 4))) as F32))) as i32);
			let mut iTemp187: i32 = i32::wrapping_add(iTemp186, 1);
			let mut fTemp188: F32 = F32::floor(fTemp184);
			let mut fTemp189: F32 = fTemp183 + (-3.0 - fTemp188);
			let mut fTemp190: F32 = fTemp183 + (-2.0 - fTemp188);
			let mut fTemp191: F32 = fTemp183 + (-1.0 - fTemp188);
			let mut fTemp192: F32 = fTemp191 * fTemp190;
			let mut fTemp193: F32 = fTemp192 * fTemp189;
			let mut fTemp194: F32 = fTemp183 + (-4.0 - fTemp188);
			let mut fTemp195: F32 = 0.0 - fTemp194;
			let mut iTemp196: i32 = ((F32::min(self.fConst11, ((std::cmp::max(0, i32::wrapping_add(iTemp185, 3))) as F32))) as i32);
			let mut iTemp197: i32 = i32::wrapping_add(iTemp196, 1);
			let mut fTemp198: F32 = 0.0 - 0.5 * fTemp194;
			let mut fTemp199: F32 = 0.0 - fTemp189;
			let mut iTemp200: i32 = ((F32::min(self.fConst11, ((std::cmp::max(0, i32::wrapping_add(iTemp185, 2))) as F32))) as i32);
			let mut iTemp201: i32 = i32::wrapping_add(iTemp200, 1);
			let mut fTemp202: F32 = 0.0 - 0.33333334 * fTemp194;
			let mut fTemp203: F32 = 0.0 - 0.5 * fTemp189;
			let mut fTemp204: F32 = 0.0 - fTemp190;
			let mut iTemp205: i32 = ((F32::min(self.fConst11, ((std::cmp::max(0, i32::wrapping_add(iTemp185, 1))) as F32))) as i32);
			let mut iTemp206: i32 = i32::wrapping_add(iTemp205, 1);
			let mut fTemp207: F32 = fTemp183 - fTemp188;
			let mut fTemp208: F32 = 0.0 - 0.25 * fTemp194;
			let mut fTemp209: F32 = 0.0 - 0.33333334 * fTemp189;
			let mut fTemp210: F32 = 0.0 - 0.5 * fTemp190;
			let mut fTemp211: F32 = 0.0 - fTemp191;
			let mut iTemp212: i32 = ((F32::min(self.fConst11, ((std::cmp::max(0, iTemp185)) as F32))) as i32);
			let mut iTemp213: i32 = i32::wrapping_add(iTemp212, 1);
			self.fRec172[0] = self.fRec149[((i32::wrapping_sub(self.IOTA0, iTemp213)) & 2047) as usize] * fTemp211 * fTemp210 * fTemp209 * fTemp208 + fTemp207 * (self.fRec149[((i32::wrapping_sub(self.IOTA0, iTemp206)) & 2047) as usize] * fTemp204 * fTemp203 * fTemp202 + 0.5 * fTemp191 * self.fRec149[((i32::wrapping_sub(self.IOTA0, iTemp201)) & 2047) as usize] * fTemp199 * fTemp198 + 0.16666667 * fTemp192 * self.fRec149[((i32::wrapping_sub(self.IOTA0, iTemp197)) & 2047) as usize] * fTemp195 + 0.041666668 * fTemp193 * self.fRec149[((i32::wrapping_sub(self.IOTA0, iTemp187)) & 2047) as usize]);
			self.fRec176[0] = 0.05 * self.fRec176[1] + 0.95 * self.fRec172[1];
			let mut fRec173: F32 = self.fRec176[0];
			self.fRec181[0] = self.fConst13 * self.fRec181[1] + self.fConst14 * F32::abs(self.fRec143[1]);
			let mut fRec180: F32 = self.fRec181[0];
			let mut iTemp214: i32 = ((fRec180 > 0.1) as i32);
			self.iVec18[0] = iTemp214;
			self.iRec182[0] = std::cmp::max(i32::wrapping_mul(self.iConst15, ((iTemp214 < self.iVec18[1]) as i32)), i32::wrapping_add(self.iRec182[1], -1));
			let mut fTemp215: F32 = F32::abs(F32::max(((iTemp214) as F32), ((((self.iRec182[0] > 0) as i32)) as F32)));
			let mut fTemp216: F32 = if (((self.fRec178[1] > fTemp215) as i32) as i32 != 0) { self.fConst16 } else { self.fConst13 };
			self.fRec179[0] = self.fRec179[1] * fTemp216 + fTemp215 * (1.0 - fTemp216);
			self.fRec178[0] = self.fRec179[0];
			let mut fTemp217: F32 = 0.005 * self.fRec178[0] * self.fRec143[1];
			self.fRec183[0] = self.fRec147[1];
			self.fRec184[(self.IOTA0 & 2047) as usize] = -1.0 * 0.9973053 * (0.9 * self.fRec183[2] + 0.05 * (self.fRec183[1] + self.fRec183[3]));
			let mut fTemp218: F32 = fTemp192 * fTemp195;
			let mut fTemp219: F32 = fTemp191 * fTemp199 * fTemp198;
			let mut fTemp220: F32 = fTemp204 * fTemp203 * fTemp202;
			let mut fTemp221: F32 = fTemp211 * fTemp210 * fTemp209 * fTemp208;
			self.fVec19[0] = fTemp221 * self.fRec184[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp212, 2))) & 2047) as usize] + fTemp207 * (fTemp220 * self.fRec184[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp205, 2))) & 2047) as usize] + 0.5 * fTemp219 * self.fRec184[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp200, 2))) & 2047) as usize] + 0.16666667 * fTemp218 * self.fRec184[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp196, 2))) & 2047) as usize] + 0.041666668 * fTemp193 * self.fRec184[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp186, 2))) & 2047) as usize]);
			let mut fTemp222: F32 = F32::tan(self.fConst17 * fTemp181);
			let mut fTemp223: F32 = 1.0 / fTemp222;
			let mut fTemp224: F32 = (fTemp223 + 1.4142135) / fTemp222 + 1.0;
			self.iRec185[0] = i32::wrapping_add(i32::wrapping_mul(i32::wrapping_add(self.iRec185[1], ((self.iRec185[1] > 0) as i32)), ((fSlow27 <= self.fVec17[1]) as i32)), ((fSlow27 > self.fVec17[1]) as i32));
			let mut fTemp225: F32 = ((self.iRec185[0]) as F32) / F32::max(1.0, self.fConst18 * mydsp_faustpower2_f(1.0 - 0.22 * fTemp181));
			self.fRec186[0] = fTemp127 - (self.fRec186[2] * ((fTemp223 + -1.4142135) / fTemp222 + 1.0) + 2.0 * self.fRec186[1] * (1.0 - 1.0 / mydsp_faustpower2_f(fTemp222))) / fTemp224;
			let mut fTemp226: F32 = 0.5 * ((self.fRec186[2] + self.fRec186[0] + 2.0 * self.fRec186[1]) * F32::max(0.0, F32::min(fTemp225, 2.0 - fTemp225)) / fTemp224);
			let mut fTemp227: F32 = fTemp226 + self.fVec19[1] + fTemp217;
			self.fVec20[0] = fTemp227;
			self.fRec177[(self.IOTA0 & 2047) as usize] = 0.05 * self.fRec177[((i32::wrapping_sub(self.IOTA0, 1)) & 2047) as usize] + 0.95 * self.fVec20[2];
			let mut fRec174: F32 = fTemp221 * self.fRec177[((i32::wrapping_sub(self.IOTA0, iTemp212)) & 2047) as usize] + fTemp207 * (fTemp220 * self.fRec177[((i32::wrapping_sub(self.IOTA0, iTemp205)) & 2047) as usize] + 0.5 * fTemp219 * self.fRec177[((i32::wrapping_sub(self.IOTA0, iTemp200)) & 2047) as usize] + 0.16666667 * fTemp218 * self.fRec177[((i32::wrapping_sub(self.IOTA0, iTemp196)) & 2047) as usize] + 0.041666668 * fTemp193 * self.fRec177[((i32::wrapping_sub(self.IOTA0, iTemp186)) & 2047) as usize]);
			let mut fRec175: F32 = self.fVec20[1] + self.fRec168[1];
			self.fRec168[0] = fRec173;
			let mut fRec169: F32 = self.fRec168[1];
			let mut fRec170: F32 = fRec174;
			let mut fRec171: F32 = fRec175;
			self.fRec164[0] = fRec169;
			let mut fRec165: F32 = fTemp217 + fTemp226 + self.fRec164[1];
			let mut fRec166: F32 = fRec170;
			let mut fRec167: F32 = fRec171;
			self.fRec160[(self.IOTA0 & 2047) as usize] = fRec165;
			let mut fRec161: F32 = fTemp221 * self.fRec160[((i32::wrapping_sub(self.IOTA0, iTemp213)) & 2047) as usize] + fTemp207 * (fTemp220 * self.fRec160[((i32::wrapping_sub(self.IOTA0, iTemp206)) & 2047) as usize] + 0.5 * fTemp219 * self.fRec160[((i32::wrapping_sub(self.IOTA0, iTemp201)) & 2047) as usize] + 0.16666667 * fTemp218 * self.fRec160[((i32::wrapping_sub(self.IOTA0, iTemp197)) & 2047) as usize] + 0.041666668 * fTemp193 * self.fRec160[((i32::wrapping_sub(self.IOTA0, iTemp187)) & 2047) as usize]);
			self.fRec162[0] = fRec166;
			let mut fRec163: F32 = fRec167;
			self.fRec158[0] = fSlow24 * self.fRec162[1];
			let mut fRec159: F32 = fRec163;
			self.fRec153[0] = fRec157;
			let mut fRec154: F32 = fSlow24 * self.fRec153[1];
			let mut fRec155: F32 = self.fRec158[0];
			let mut fRec156: F32 = fRec159;
			self.fRec149[(self.IOTA0 & 2047) as usize] = fRec154;
			let mut fRec150: F32 = fRec161;
			let mut fRec151: F32 = fRec155;
			let mut fRec152: F32 = fRec156;
			self.fRec147[0] = fRec150;
			let mut fRec148: F32 = fRec152;
			let mut fTemp228: F32 = F32::abs(fRec148);
			let mut fTemp229: F32 = if (((self.fRec145[1] > fTemp228) as i32) as i32 != 0) { self.fConst19 } else { 0.0 };
			self.fRec146[0] = self.fRec146[1] * fTemp229 + fTemp228 * (1.0 - fTemp229);
			self.fRec145[0] = self.fRec146[0];
			let mut fRec144: F32 = 0.0 - 0.95 * F32::max(2e+01 * F32::log10(self.fRec145[0]) + 1e+01, 0.0);
			self.fRec143[0] = fRec148 * F32::powf(1e+01, 0.05 * fRec144);
			self.fRec140[0] = 0.0 - (self.fRec140[1] * (1.0 - fTemp182) - (self.fRec143[0] + self.fRec143[1])) / (fTemp182 + 1.0);
			self.fVec21[0] = fSlow30;
			self.fRec189[0] = if (iSlow31 as i32 != 0) { fSlow32 } else { self.fRec189[1] };
			self.fRec188[0] = self.fConst2 * self.fRec188[1] + self.fConst1 * self.fRec189[0];
			let mut fTemp230: F32 = F32::powf(2.0, 0.083333336 * (self.fRec48[0] + self.fRec188[0] + -69.0));
			let mut fTemp231: F32 = 1.0 / F32::tan(self.fConst10 * fTemp230);
			let mut fRec204: F32 = -1.0 * 0.9973053 * (0.9 * self.fRec205[2] + 0.05 * (self.fRec205[1] + self.fRec205[3]));
			let mut fTemp232: F32 = self.fConst12 * (0.77272725 / fTemp230 + -0.11);
			let mut fTemp233: F32 = fTemp232 + -1.499995;
			let mut iTemp234: i32 = ((fTemp233) as i32);
			let mut iTemp235: i32 = ((F32::min(self.fConst11, ((std::cmp::max(0, i32::wrapping_add(iTemp234, 4))) as F32))) as i32);
			let mut iTemp236: i32 = i32::wrapping_add(iTemp235, 1);
			let mut fTemp237: F32 = F32::floor(fTemp233);
			let mut fTemp238: F32 = fTemp232 + (-3.0 - fTemp237);
			let mut fTemp239: F32 = fTemp232 + (-2.0 - fTemp237);
			let mut fTemp240: F32 = fTemp232 + (-1.0 - fTemp237);
			let mut fTemp241: F32 = fTemp240 * fTemp239;
			let mut fTemp242: F32 = fTemp241 * fTemp238;
			let mut fTemp243: F32 = fTemp232 + (-4.0 - fTemp237);
			let mut fTemp244: F32 = 0.0 - fTemp243;
			let mut iTemp245: i32 = ((F32::min(self.fConst11, ((std::cmp::max(0, i32::wrapping_add(iTemp234, 3))) as F32))) as i32);
			let mut iTemp246: i32 = i32::wrapping_add(iTemp245, 1);
			let mut fTemp247: F32 = 0.0 - 0.5 * fTemp243;
			let mut fTemp248: F32 = 0.0 - fTemp238;
			let mut iTemp249: i32 = ((F32::min(self.fConst11, ((std::cmp::max(0, i32::wrapping_add(iTemp234, 2))) as F32))) as i32);
			let mut iTemp250: i32 = i32::wrapping_add(iTemp249, 1);
			let mut fTemp251: F32 = 0.0 - 0.33333334 * fTemp243;
			let mut fTemp252: F32 = 0.0 - 0.5 * fTemp238;
			let mut fTemp253: F32 = 0.0 - fTemp239;
			let mut iTemp254: i32 = ((F32::min(self.fConst11, ((std::cmp::max(0, i32::wrapping_add(iTemp234, 1))) as F32))) as i32);
			let mut iTemp255: i32 = i32::wrapping_add(iTemp254, 1);
			let mut fTemp256: F32 = fTemp232 - fTemp237;
			let mut fTemp257: F32 = 0.0 - 0.25 * fTemp243;
			let mut fTemp258: F32 = 0.0 - 0.33333334 * fTemp238;
			let mut fTemp259: F32 = 0.0 - 0.5 * fTemp239;
			let mut fTemp260: F32 = 0.0 - fTemp240;
			let mut iTemp261: i32 = ((F32::min(self.fConst11, ((std::cmp::max(0, iTemp234)) as F32))) as i32);
			let mut iTemp262: i32 = i32::wrapping_add(iTemp261, 1);
			self.fRec219[0] = self.fRec196[((i32::wrapping_sub(self.IOTA0, iTemp262)) & 2047) as usize] * fTemp260 * fTemp259 * fTemp258 * fTemp257 + fTemp256 * (self.fRec196[((i32::wrapping_sub(self.IOTA0, iTemp255)) & 2047) as usize] * fTemp253 * fTemp252 * fTemp251 + 0.5 * fTemp240 * self.fRec196[((i32::wrapping_sub(self.IOTA0, iTemp250)) & 2047) as usize] * fTemp248 * fTemp247 + 0.16666667 * fTemp241 * self.fRec196[((i32::wrapping_sub(self.IOTA0, iTemp246)) & 2047) as usize] * fTemp244 + 0.041666668 * fTemp242 * self.fRec196[((i32::wrapping_sub(self.IOTA0, iTemp236)) & 2047) as usize]);
			self.fRec223[0] = 0.05 * self.fRec223[1] + 0.95 * self.fRec219[1];
			let mut fRec220: F32 = self.fRec223[0];
			self.fRec228[0] = self.fConst13 * self.fRec228[1] + self.fConst14 * F32::abs(self.fRec190[1]);
			let mut fRec227: F32 = self.fRec228[0];
			let mut iTemp263: i32 = ((fRec227 > 0.1) as i32);
			self.iVec22[0] = iTemp263;
			self.iRec229[0] = std::cmp::max(i32::wrapping_mul(self.iConst15, ((iTemp263 < self.iVec22[1]) as i32)), i32::wrapping_add(self.iRec229[1], -1));
			let mut fTemp264: F32 = F32::abs(F32::max(((iTemp263) as F32), ((((self.iRec229[0] > 0) as i32)) as F32)));
			let mut fTemp265: F32 = if (((self.fRec225[1] > fTemp264) as i32) as i32 != 0) { self.fConst16 } else { self.fConst13 };
			self.fRec226[0] = self.fRec226[1] * fTemp265 + fTemp264 * (1.0 - fTemp265);
			self.fRec225[0] = self.fRec226[0];
			let mut fTemp266: F32 = 0.005 * self.fRec225[0] * self.fRec190[1];
			self.fRec230[0] = self.fRec194[1];
			self.fRec231[(self.IOTA0 & 2047) as usize] = -1.0 * 0.9973053 * (0.9 * self.fRec230[2] + 0.05 * (self.fRec230[1] + self.fRec230[3]));
			let mut fTemp267: F32 = fTemp241 * fTemp244;
			let mut fTemp268: F32 = fTemp240 * fTemp248 * fTemp247;
			let mut fTemp269: F32 = fTemp253 * fTemp252 * fTemp251;
			let mut fTemp270: F32 = fTemp260 * fTemp259 * fTemp258 * fTemp257;
			self.fVec23[0] = fTemp270 * self.fRec231[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp261, 2))) & 2047) as usize] + fTemp256 * (fTemp269 * self.fRec231[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp254, 2))) & 2047) as usize] + 0.5 * fTemp268 * self.fRec231[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp249, 2))) & 2047) as usize] + 0.16666667 * fTemp267 * self.fRec231[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp245, 2))) & 2047) as usize] + 0.041666668 * fTemp242 * self.fRec231[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp235, 2))) & 2047) as usize]);
			let mut fTemp271: F32 = F32::tan(self.fConst17 * fTemp230);
			let mut fTemp272: F32 = 1.0 / fTemp271;
			let mut fTemp273: F32 = (fTemp272 + 1.4142135) / fTemp271 + 1.0;
			self.iRec232[0] = i32::wrapping_add(i32::wrapping_mul(i32::wrapping_add(self.iRec232[1], ((self.iRec232[1] > 0) as i32)), ((fSlow30 <= self.fVec21[1]) as i32)), ((fSlow30 > self.fVec21[1]) as i32));
			let mut fTemp274: F32 = ((self.iRec232[0]) as F32) / F32::max(1.0, self.fConst18 * mydsp_faustpower2_f(1.0 - 0.22 * fTemp230));
			self.fRec233[0] = fTemp127 - (self.fRec233[2] * ((fTemp272 + -1.4142135) / fTemp271 + 1.0) + 2.0 * self.fRec233[1] * (1.0 - 1.0 / mydsp_faustpower2_f(fTemp271))) / fTemp273;
			let mut fTemp275: F32 = 0.5 * ((self.fRec233[2] + self.fRec233[0] + 2.0 * self.fRec233[1]) * F32::max(0.0, F32::min(fTemp274, 2.0 - fTemp274)) / fTemp273);
			let mut fTemp276: F32 = fTemp275 + self.fVec23[1] + fTemp266;
			self.fVec24[0] = fTemp276;
			self.fRec224[(self.IOTA0 & 2047) as usize] = 0.05 * self.fRec224[((i32::wrapping_sub(self.IOTA0, 1)) & 2047) as usize] + 0.95 * self.fVec24[2];
			let mut fRec221: F32 = fTemp270 * self.fRec224[((i32::wrapping_sub(self.IOTA0, iTemp261)) & 2047) as usize] + fTemp256 * (fTemp269 * self.fRec224[((i32::wrapping_sub(self.IOTA0, iTemp254)) & 2047) as usize] + 0.5 * fTemp268 * self.fRec224[((i32::wrapping_sub(self.IOTA0, iTemp249)) & 2047) as usize] + 0.16666667 * fTemp267 * self.fRec224[((i32::wrapping_sub(self.IOTA0, iTemp245)) & 2047) as usize] + 0.041666668 * fTemp242 * self.fRec224[((i32::wrapping_sub(self.IOTA0, iTemp235)) & 2047) as usize]);
			let mut fRec222: F32 = self.fVec24[1] + self.fRec215[1];
			self.fRec215[0] = fRec220;
			let mut fRec216: F32 = self.fRec215[1];
			let mut fRec217: F32 = fRec221;
			let mut fRec218: F32 = fRec222;
			self.fRec211[0] = fRec216;
			let mut fRec212: F32 = fTemp266 + fTemp275 + self.fRec211[1];
			let mut fRec213: F32 = fRec217;
			let mut fRec214: F32 = fRec218;
			self.fRec207[(self.IOTA0 & 2047) as usize] = fRec212;
			let mut fRec208: F32 = fTemp270 * self.fRec207[((i32::wrapping_sub(self.IOTA0, iTemp262)) & 2047) as usize] + fTemp256 * (fTemp269 * self.fRec207[((i32::wrapping_sub(self.IOTA0, iTemp255)) & 2047) as usize] + 0.5 * fTemp268 * self.fRec207[((i32::wrapping_sub(self.IOTA0, iTemp250)) & 2047) as usize] + 0.16666667 * fTemp267 * self.fRec207[((i32::wrapping_sub(self.IOTA0, iTemp246)) & 2047) as usize] + 0.041666668 * fTemp242 * self.fRec207[((i32::wrapping_sub(self.IOTA0, iTemp236)) & 2047) as usize]);
			self.fRec209[0] = fRec213;
			let mut fRec210: F32 = fRec214;
			self.fRec205[0] = fSlow24 * self.fRec209[1];
			let mut fRec206: F32 = fRec210;
			self.fRec200[0] = fRec204;
			let mut fRec201: F32 = fSlow24 * self.fRec200[1];
			let mut fRec202: F32 = self.fRec205[0];
			let mut fRec203: F32 = fRec206;
			self.fRec196[(self.IOTA0 & 2047) as usize] = fRec201;
			let mut fRec197: F32 = fRec208;
			let mut fRec198: F32 = fRec202;
			let mut fRec199: F32 = fRec203;
			self.fRec194[0] = fRec197;
			let mut fRec195: F32 = fRec199;
			let mut fTemp277: F32 = F32::abs(fRec195);
			let mut fTemp278: F32 = if (((self.fRec192[1] > fTemp277) as i32) as i32 != 0) { self.fConst19 } else { 0.0 };
			self.fRec193[0] = self.fRec193[1] * fTemp278 + fTemp277 * (1.0 - fTemp278);
			self.fRec192[0] = self.fRec193[0];
			let mut fRec191: F32 = 0.0 - 0.95 * F32::max(2e+01 * F32::log10(self.fRec192[0]) + 1e+01, 0.0);
			self.fRec190[0] = fRec195 * F32::powf(1e+01, 0.05 * fRec191);
			self.fRec187[0] = 0.0 - (self.fRec187[1] * (1.0 - fTemp231) - (self.fRec190[0] + self.fRec190[1])) / (fTemp231 + 1.0);
			self.fVec25[0] = fSlow33;
			self.fRec236[0] = if (iSlow34 as i32 != 0) { fSlow35 } else { self.fRec236[1] };
			self.fRec235[0] = self.fConst2 * self.fRec235[1] + self.fConst1 * self.fRec236[0];
			let mut fTemp279: F32 = F32::powf(2.0, 0.083333336 * (self.fRec48[0] + self.fRec235[0] + -69.0));
			let mut fTemp280: F32 = 1.0 / F32::tan(self.fConst10 * fTemp279);
			let mut fRec251: F32 = -1.0 * 0.9973053 * (0.9 * self.fRec252[2] + 0.05 * (self.fRec252[1] + self.fRec252[3]));
			let mut fTemp281: F32 = self.fConst12 * (0.77272725 / fTemp279 + -0.11);
			let mut fTemp282: F32 = fTemp281 + -1.499995;
			let mut iTemp283: i32 = ((fTemp282) as i32);
			let mut iTemp284: i32 = ((F32::min(self.fConst11, ((std::cmp::max(0, i32::wrapping_add(iTemp283, 4))) as F32))) as i32);
			let mut iTemp285: i32 = i32::wrapping_add(iTemp284, 1);
			let mut fTemp286: F32 = F32::floor(fTemp282);
			let mut fTemp287: F32 = fTemp281 + (-3.0 - fTemp286);
			let mut fTemp288: F32 = fTemp281 + (-2.0 - fTemp286);
			let mut fTemp289: F32 = fTemp281 + (-1.0 - fTemp286);
			let mut fTemp290: F32 = fTemp289 * fTemp288;
			let mut fTemp291: F32 = fTemp290 * fTemp287;
			let mut fTemp292: F32 = fTemp281 + (-4.0 - fTemp286);
			let mut fTemp293: F32 = 0.0 - fTemp292;
			let mut iTemp294: i32 = ((F32::min(self.fConst11, ((std::cmp::max(0, i32::wrapping_add(iTemp283, 3))) as F32))) as i32);
			let mut iTemp295: i32 = i32::wrapping_add(iTemp294, 1);
			let mut fTemp296: F32 = 0.0 - 0.5 * fTemp292;
			let mut fTemp297: F32 = 0.0 - fTemp287;
			let mut iTemp298: i32 = ((F32::min(self.fConst11, ((std::cmp::max(0, i32::wrapping_add(iTemp283, 2))) as F32))) as i32);
			let mut iTemp299: i32 = i32::wrapping_add(iTemp298, 1);
			let mut fTemp300: F32 = 0.0 - 0.33333334 * fTemp292;
			let mut fTemp301: F32 = 0.0 - 0.5 * fTemp287;
			let mut fTemp302: F32 = 0.0 - fTemp288;
			let mut iTemp303: i32 = ((F32::min(self.fConst11, ((std::cmp::max(0, i32::wrapping_add(iTemp283, 1))) as F32))) as i32);
			let mut iTemp304: i32 = i32::wrapping_add(iTemp303, 1);
			let mut fTemp305: F32 = fTemp281 - fTemp286;
			let mut fTemp306: F32 = 0.0 - 0.25 * fTemp292;
			let mut fTemp307: F32 = 0.0 - 0.33333334 * fTemp287;
			let mut fTemp308: F32 = 0.0 - 0.5 * fTemp288;
			let mut fTemp309: F32 = 0.0 - fTemp289;
			let mut iTemp310: i32 = ((F32::min(self.fConst11, ((std::cmp::max(0, iTemp283)) as F32))) as i32);
			let mut iTemp311: i32 = i32::wrapping_add(iTemp310, 1);
			self.fRec266[0] = self.fRec243[((i32::wrapping_sub(self.IOTA0, iTemp311)) & 2047) as usize] * fTemp309 * fTemp308 * fTemp307 * fTemp306 + fTemp305 * (self.fRec243[((i32::wrapping_sub(self.IOTA0, iTemp304)) & 2047) as usize] * fTemp302 * fTemp301 * fTemp300 + 0.5 * fTemp289 * self.fRec243[((i32::wrapping_sub(self.IOTA0, iTemp299)) & 2047) as usize] * fTemp297 * fTemp296 + 0.16666667 * fTemp290 * self.fRec243[((i32::wrapping_sub(self.IOTA0, iTemp295)) & 2047) as usize] * fTemp293 + 0.041666668 * fTemp291 * self.fRec243[((i32::wrapping_sub(self.IOTA0, iTemp285)) & 2047) as usize]);
			self.fRec270[0] = 0.05 * self.fRec270[1] + 0.95 * self.fRec266[1];
			let mut fRec267: F32 = self.fRec270[0];
			self.fRec275[0] = self.fConst13 * self.fRec275[1] + self.fConst14 * F32::abs(self.fRec237[1]);
			let mut fRec274: F32 = self.fRec275[0];
			let mut iTemp312: i32 = ((fRec274 > 0.1) as i32);
			self.iVec26[0] = iTemp312;
			self.iRec276[0] = std::cmp::max(i32::wrapping_mul(self.iConst15, ((iTemp312 < self.iVec26[1]) as i32)), i32::wrapping_add(self.iRec276[1], -1));
			let mut fTemp313: F32 = F32::abs(F32::max(((iTemp312) as F32), ((((self.iRec276[0] > 0) as i32)) as F32)));
			let mut fTemp314: F32 = if (((self.fRec272[1] > fTemp313) as i32) as i32 != 0) { self.fConst16 } else { self.fConst13 };
			self.fRec273[0] = self.fRec273[1] * fTemp314 + fTemp313 * (1.0 - fTemp314);
			self.fRec272[0] = self.fRec273[0];
			let mut fTemp315: F32 = 0.005 * self.fRec272[0] * self.fRec237[1];
			self.fRec277[0] = self.fRec241[1];
			self.fRec278[(self.IOTA0 & 2047) as usize] = -1.0 * 0.9973053 * (0.9 * self.fRec277[2] + 0.05 * (self.fRec277[1] + self.fRec277[3]));
			let mut fTemp316: F32 = fTemp290 * fTemp293;
			let mut fTemp317: F32 = fTemp289 * fTemp297 * fTemp296;
			let mut fTemp318: F32 = fTemp302 * fTemp301 * fTemp300;
			let mut fTemp319: F32 = fTemp309 * fTemp308 * fTemp307 * fTemp306;
			self.fVec27[0] = fTemp319 * self.fRec278[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp310, 2))) & 2047) as usize] + fTemp305 * (fTemp318 * self.fRec278[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp303, 2))) & 2047) as usize] + 0.5 * fTemp317 * self.fRec278[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp298, 2))) & 2047) as usize] + 0.16666667 * fTemp316 * self.fRec278[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp294, 2))) & 2047) as usize] + 0.041666668 * fTemp291 * self.fRec278[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp284, 2))) & 2047) as usize]);
			let mut fTemp320: F32 = F32::tan(self.fConst17 * fTemp279);
			let mut fTemp321: F32 = 1.0 / fTemp320;
			let mut fTemp322: F32 = (fTemp321 + 1.4142135) / fTemp320 + 1.0;
			self.iRec279[0] = i32::wrapping_add(i32::wrapping_mul(i32::wrapping_add(self.iRec279[1], ((self.iRec279[1] > 0) as i32)), ((fSlow33 <= self.fVec25[1]) as i32)), ((fSlow33 > self.fVec25[1]) as i32));
			let mut fTemp323: F32 = ((self.iRec279[0]) as F32) / F32::max(1.0, self.fConst18 * mydsp_faustpower2_f(1.0 - 0.22 * fTemp279));
			self.fRec280[0] = fTemp127 - (self.fRec280[2] * ((fTemp321 + -1.4142135) / fTemp320 + 1.0) + 2.0 * self.fRec280[1] * (1.0 - 1.0 / mydsp_faustpower2_f(fTemp320))) / fTemp322;
			let mut fTemp324: F32 = 0.5 * ((self.fRec280[2] + self.fRec280[0] + 2.0 * self.fRec280[1]) * F32::max(0.0, F32::min(fTemp323, 2.0 - fTemp323)) / fTemp322);
			let mut fTemp325: F32 = fTemp324 + self.fVec27[1] + fTemp315;
			self.fVec28[0] = fTemp325;
			self.fRec271[(self.IOTA0 & 2047) as usize] = 0.05 * self.fRec271[((i32::wrapping_sub(self.IOTA0, 1)) & 2047) as usize] + 0.95 * self.fVec28[2];
			let mut fRec268: F32 = fTemp319 * self.fRec271[((i32::wrapping_sub(self.IOTA0, iTemp310)) & 2047) as usize] + fTemp305 * (fTemp318 * self.fRec271[((i32::wrapping_sub(self.IOTA0, iTemp303)) & 2047) as usize] + 0.5 * fTemp317 * self.fRec271[((i32::wrapping_sub(self.IOTA0, iTemp298)) & 2047) as usize] + 0.16666667 * fTemp316 * self.fRec271[((i32::wrapping_sub(self.IOTA0, iTemp294)) & 2047) as usize] + 0.041666668 * fTemp291 * self.fRec271[((i32::wrapping_sub(self.IOTA0, iTemp284)) & 2047) as usize]);
			let mut fRec269: F32 = self.fVec28[1] + self.fRec262[1];
			self.fRec262[0] = fRec267;
			let mut fRec263: F32 = self.fRec262[1];
			let mut fRec264: F32 = fRec268;
			let mut fRec265: F32 = fRec269;
			self.fRec258[0] = fRec263;
			let mut fRec259: F32 = fTemp315 + fTemp324 + self.fRec258[1];
			let mut fRec260: F32 = fRec264;
			let mut fRec261: F32 = fRec265;
			self.fRec254[(self.IOTA0 & 2047) as usize] = fRec259;
			let mut fRec255: F32 = fTemp319 * self.fRec254[((i32::wrapping_sub(self.IOTA0, iTemp311)) & 2047) as usize] + fTemp305 * (fTemp318 * self.fRec254[((i32::wrapping_sub(self.IOTA0, iTemp304)) & 2047) as usize] + 0.5 * fTemp317 * self.fRec254[((i32::wrapping_sub(self.IOTA0, iTemp299)) & 2047) as usize] + 0.16666667 * fTemp316 * self.fRec254[((i32::wrapping_sub(self.IOTA0, iTemp295)) & 2047) as usize] + 0.041666668 * fTemp291 * self.fRec254[((i32::wrapping_sub(self.IOTA0, iTemp285)) & 2047) as usize]);
			self.fRec256[0] = fRec260;
			let mut fRec257: F32 = fRec261;
			self.fRec252[0] = fSlow24 * self.fRec256[1];
			let mut fRec253: F32 = fRec257;
			self.fRec247[0] = fRec251;
			let mut fRec248: F32 = fSlow24 * self.fRec247[1];
			let mut fRec249: F32 = self.fRec252[0];
			let mut fRec250: F32 = fRec253;
			self.fRec243[(self.IOTA0 & 2047) as usize] = fRec248;
			let mut fRec244: F32 = fRec255;
			let mut fRec245: F32 = fRec249;
			let mut fRec246: F32 = fRec250;
			self.fRec241[0] = fRec244;
			let mut fRec242: F32 = fRec246;
			let mut fTemp326: F32 = F32::abs(fRec242);
			let mut fTemp327: F32 = if (((self.fRec239[1] > fTemp326) as i32) as i32 != 0) { self.fConst19 } else { 0.0 };
			self.fRec240[0] = self.fRec240[1] * fTemp327 + fTemp326 * (1.0 - fTemp327);
			self.fRec239[0] = self.fRec240[0];
			let mut fRec238: F32 = 0.0 - 0.95 * F32::max(2e+01 * F32::log10(self.fRec239[0]) + 1e+01, 0.0);
			self.fRec237[0] = fRec242 * F32::powf(1e+01, 0.05 * fRec238);
			self.fRec234[0] = 0.0 - (self.fRec234[1] * (1.0 - fTemp280) - (self.fRec237[0] + self.fRec237[1])) / (fTemp280 + 1.0);
			self.fRec281[0] = fSlow36 + self.fConst2 * self.fRec281[1];
			self.fRec1[(self.IOTA0 & 262143) as usize] = self.fRec281[0] * (self.fRec234[0] + self.fRec187[0] + self.fRec140[0] + self.fRec94[0] + self.fRec45[0]) + self.fRec44[0] * self.fRec43[0] * ((self.fRec37[2] + self.fRec37[0] + 2.0 * self.fRec37[1]) / fTemp75 + (self.fRec30[2] + self.fRec30[0] + 2.0 * self.fRec30[1]) / fTemp66 + (self.fRec23[2] + self.fRec23[0] + 2.0 * self.fRec23[1]) / fTemp56 + (self.fRec16[2] + self.fRec16[0] + 2.0 * self.fRec16[1]) / fTemp45) + 0.7 * self.fRec1[((i32::wrapping_sub(self.IOTA0, self.iConst7)) & 262143) as usize] + self.fConst6 * self.fRec12[0] * self.fRec11[0] * (self.fRec9[0] * fTemp28 + self.fRec7[0] * fTemp19 + self.fRec5[0] * fTemp10 + self.fRec3[0] * fTemp0);
			let mut fTemp328: F32 = self.fRec1[(self.IOTA0 & 262143) as usize] * self.fRec0[0];
			*output0 = fTemp328;
			*output1 = fTemp328;
			self.iVec0[1] = self.iVec0[0];
			self.fRec0[1] = self.fRec0[0];
			self.fRec2[1] = self.fRec2[0];
			self.fRec4[1] = self.fRec4[0];
			self.fVec1[1] = self.fVec1[0];
			self.IOTA0 = i32::wrapping_add(self.IOTA0, 1);
			self.fRec3[1] = self.fRec3[0];
			self.fRec6[1] = self.fRec6[0];
			self.fVec3[1] = self.fVec3[0];
			self.fRec5[1] = self.fRec5[0];
			self.fRec8[1] = self.fRec8[0];
			self.fVec5[1] = self.fVec5[0];
			self.fRec7[1] = self.fRec7[0];
			self.fRec10[1] = self.fRec10[0];
			self.fVec7[1] = self.fVec7[0];
			self.fRec9[1] = self.fRec9[0];
			self.fRec11[1] = self.fRec11[0];
			self.fRec12[1] = self.fRec12[0];
			self.fRec13[1] = self.fRec13[0];
			self.fRec15[1] = self.fRec15[0];
			self.fRec14[1] = self.fRec14[0];
			self.fRec20[1] = self.fRec20[0];
			self.fRec18[1] = self.fRec18[0];
			self.fRec21[1] = self.fRec21[0];
			self.fRec17[2] = self.fRec17[1];
			self.fRec17[1] = self.fRec17[0];
			self.fRec16[2] = self.fRec16[1];
			self.fRec16[1] = self.fRec16[0];
			self.fRec22[1] = self.fRec22[0];
			self.fRec27[1] = self.fRec27[0];
			self.fRec25[1] = self.fRec25[0];
			self.fRec28[1] = self.fRec28[0];
			self.fRec24[2] = self.fRec24[1];
			self.fRec24[1] = self.fRec24[0];
			self.fRec23[2] = self.fRec23[1];
			self.fRec23[1] = self.fRec23[0];
			self.fRec29[1] = self.fRec29[0];
			self.fRec34[1] = self.fRec34[0];
			self.fRec32[1] = self.fRec32[0];
			self.fRec35[1] = self.fRec35[0];
			self.fRec31[2] = self.fRec31[1];
			self.fRec31[1] = self.fRec31[0];
			self.fRec30[2] = self.fRec30[1];
			self.fRec30[1] = self.fRec30[0];
			self.fRec36[1] = self.fRec36[0];
			self.fRec41[1] = self.fRec41[0];
			self.fRec39[1] = self.fRec39[0];
			self.fRec42[1] = self.fRec42[0];
			self.fRec38[2] = self.fRec38[1];
			self.fRec38[1] = self.fRec38[0];
			self.fRec37[2] = self.fRec37[1];
			self.fRec37[1] = self.fRec37[0];
			self.fRec43[1] = self.fRec43[0];
			self.fRec44[1] = self.fRec44[0];
			self.fVec9[1] = self.fVec9[0];
			self.fRec47[1] = self.fRec47[0];
			self.fRec46[1] = self.fRec46[0];
			self.fRec48[1] = self.fRec48[0];
			self.fRec78[1] = self.fRec78[0];
			self.fRec82[1] = self.fRec82[0];
			self.fRec87[1] = self.fRec87[0];
			self.iVec10[1] = self.iVec10[0];
			self.iRec88[1] = self.iRec88[0];
			self.fRec85[1] = self.fRec85[0];
			self.fRec84[1] = self.fRec84[0];
			for j0 in (1..=3).rev() {
				self.fRec89[(j0) as usize] = self.fRec89[(i32::wrapping_sub(j0, 1)) as usize];
			}
			self.fVec11[1] = self.fVec11[0];
			self.iRec91[1] = self.iRec91[0];
			self.iRec93[1] = self.iRec93[0];
			self.fRec92[2] = self.fRec92[1];
			self.fRec92[1] = self.fRec92[0];
			self.fVec12[2] = self.fVec12[1];
			self.fVec12[1] = self.fVec12[0];
			self.fRec74[1] = self.fRec74[0];
			self.fRec70[1] = self.fRec70[0];
			self.fRec68[1] = self.fRec68[0];
			for j1 in (1..=3).rev() {
				self.fRec64[(j1) as usize] = self.fRec64[(i32::wrapping_sub(j1, 1)) as usize];
			}
			self.fRec59[1] = self.fRec59[0];
			self.fRec53[1] = self.fRec53[0];
			self.fRec52[1] = self.fRec52[0];
			self.fRec51[1] = self.fRec51[0];
			self.fRec49[1] = self.fRec49[0];
			self.fRec45[1] = self.fRec45[0];
			self.fRec95[1] = self.fRec95[0];
			self.fRec125[1] = self.fRec125[0];
			self.fRec129[1] = self.fRec129[0];
			self.fVec13[1] = self.fVec13[0];
			self.iRec131[1] = self.iRec131[0];
			self.fRec132[2] = self.fRec132[1];
			self.fRec132[1] = self.fRec132[0];
			for j2 in (1..=3).rev() {
				self.fRec133[(j2) as usize] = self.fRec133[(i32::wrapping_sub(j2, 1)) as usize];
			}
			self.fVec14[1] = self.fVec14[0];
			self.fRec138[1] = self.fRec138[0];
			self.iVec15[1] = self.iVec15[0];
			self.iRec139[1] = self.iRec139[0];
			self.fRec136[1] = self.fRec136[0];
			self.fRec135[1] = self.fRec135[0];
			self.fVec16[2] = self.fVec16[1];
			self.fVec16[1] = self.fVec16[0];
			self.fRec121[1] = self.fRec121[0];
			self.fRec117[1] = self.fRec117[0];
			self.fRec115[1] = self.fRec115[0];
			for j3 in (1..=3).rev() {
				self.fRec111[(j3) as usize] = self.fRec111[(i32::wrapping_sub(j3, 1)) as usize];
			}
			self.fRec106[1] = self.fRec106[0];
			self.fRec100[1] = self.fRec100[0];
			self.fRec99[1] = self.fRec99[0];
			self.fRec98[1] = self.fRec98[0];
			self.fRec96[1] = self.fRec96[0];
			self.fRec94[1] = self.fRec94[0];
			self.fVec17[1] = self.fVec17[0];
			self.fRec142[1] = self.fRec142[0];
			self.fRec141[1] = self.fRec141[0];
			self.fRec172[1] = self.fRec172[0];
			self.fRec176[1] = self.fRec176[0];
			self.fRec181[1] = self.fRec181[0];
			self.iVec18[1] = self.iVec18[0];
			self.iRec182[1] = self.iRec182[0];
			self.fRec179[1] = self.fRec179[0];
			self.fRec178[1] = self.fRec178[0];
			for j4 in (1..=3).rev() {
				self.fRec183[(j4) as usize] = self.fRec183[(i32::wrapping_sub(j4, 1)) as usize];
			}
			self.fVec19[1] = self.fVec19[0];
			self.iRec185[1] = self.iRec185[0];
			self.fRec186[2] = self.fRec186[1];
			self.fRec186[1] = self.fRec186[0];
			self.fVec20[2] = self.fVec20[1];
			self.fVec20[1] = self.fVec20[0];
			self.fRec168[1] = self.fRec168[0];
			self.fRec164[1] = self.fRec164[0];
			self.fRec162[1] = self.fRec162[0];
			for j5 in (1..=3).rev() {
				self.fRec158[(j5) as usize] = self.fRec158[(i32::wrapping_sub(j5, 1)) as usize];
			}
			self.fRec153[1] = self.fRec153[0];
			self.fRec147[1] = self.fRec147[0];
			self.fRec146[1] = self.fRec146[0];
			self.fRec145[1] = self.fRec145[0];
			self.fRec143[1] = self.fRec143[0];
			self.fRec140[1] = self.fRec140[0];
			self.fVec21[1] = self.fVec21[0];
			self.fRec189[1] = self.fRec189[0];
			self.fRec188[1] = self.fRec188[0];
			self.fRec219[1] = self.fRec219[0];
			self.fRec223[1] = self.fRec223[0];
			self.fRec228[1] = self.fRec228[0];
			self.iVec22[1] = self.iVec22[0];
			self.iRec229[1] = self.iRec229[0];
			self.fRec226[1] = self.fRec226[0];
			self.fRec225[1] = self.fRec225[0];
			for j6 in (1..=3).rev() {
				self.fRec230[(j6) as usize] = self.fRec230[(i32::wrapping_sub(j6, 1)) as usize];
			}
			self.fVec23[1] = self.fVec23[0];
			self.iRec232[1] = self.iRec232[0];
			self.fRec233[2] = self.fRec233[1];
			self.fRec233[1] = self.fRec233[0];
			self.fVec24[2] = self.fVec24[1];
			self.fVec24[1] = self.fVec24[0];
			self.fRec215[1] = self.fRec215[0];
			self.fRec211[1] = self.fRec211[0];
			self.fRec209[1] = self.fRec209[0];
			for j7 in (1..=3).rev() {
				self.fRec205[(j7) as usize] = self.fRec205[(i32::wrapping_sub(j7, 1)) as usize];
			}
			self.fRec200[1] = self.fRec200[0];
			self.fRec194[1] = self.fRec194[0];
			self.fRec193[1] = self.fRec193[0];
			self.fRec192[1] = self.fRec192[0];
			self.fRec190[1] = self.fRec190[0];
			self.fRec187[1] = self.fRec187[0];
			self.fVec25[1] = self.fVec25[0];
			self.fRec236[1] = self.fRec236[0];
			self.fRec235[1] = self.fRec235[0];
			self.fRec266[1] = self.fRec266[0];
			self.fRec270[1] = self.fRec270[0];
			self.fRec275[1] = self.fRec275[0];
			self.iVec26[1] = self.iVec26[0];
			self.iRec276[1] = self.iRec276[0];
			self.fRec273[1] = self.fRec273[0];
			self.fRec272[1] = self.fRec272[0];
			for j8 in (1..=3).rev() {
				self.fRec277[(j8) as usize] = self.fRec277[(i32::wrapping_sub(j8, 1)) as usize];
			}
			self.fVec27[1] = self.fVec27[0];
			self.iRec279[1] = self.iRec279[0];
			self.fRec280[2] = self.fRec280[1];
			self.fRec280[1] = self.fRec280[0];
			self.fVec28[2] = self.fVec28[1];
			self.fVec28[1] = self.fVec28[0];
			self.fRec262[1] = self.fRec262[0];
			self.fRec258[1] = self.fRec258[0];
			self.fRec256[1] = self.fRec256[0];
			for j9 in (1..=3).rev() {
				self.fRec252[(j9) as usize] = self.fRec252[(i32::wrapping_sub(j9, 1)) as usize];
			}
			self.fRec247[1] = self.fRec247[0];
			self.fRec241[1] = self.fRec241[0];
			self.fRec240[1] = self.fRec240[0];
			self.fRec239[1] = self.fRec239[0];
			self.fRec237[1] = self.fRec237[0];
			self.fRec234[1] = self.fRec234[0];
			self.fRec281[1] = self.fRec281[0];
		}
	}

}


}
pub use dsp::mydsp as Instrument;
