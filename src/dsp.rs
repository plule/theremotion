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
	fButton0: F32,
	fVec0: [F32;2],
	iVec1: [i32;2],
	fHslider0: F32,
	fRec2: [F32;2],
	fSampleRate: i32,
	fConst0: F32,
	fConst1: F32,
	fConst2: F32,
	fRec1: [F32;2],
	fHslider1: F32,
	fRec3: [F32;2],
	fConst3: F32,
	IOTA0: i32,
	fConst4: F32,
	fConst5: F32,
	fRec33: [F32;2],
	fRec37: [F32;2],
	fConst6: F32,
	fConst7: F32,
	fRec42: [F32;2],
	iVec2: [i32;2],
	iConst8: i32,
	iRec43: [i32;2],
	fConst9: F32,
	fRec40: [F32;2],
	fRec39: [F32;2],
	fRec44: [F32;4],
	fRec45: [F32;2048],
	fVec3: [F32;2],
	fConst10: F32,
	fConst11: F32,
	iRec46: [i32;2],
	iRec48: [i32;2],
	fRec47: [F32;3],
	fVec4: [F32;3],
	fRec38: [F32;2048],
	fRec29: [F32;2],
	fRec25: [F32;2],
	fRec21: [F32;2048],
	fRec23: [F32;2],
	fHslider2: F32,
	fRec19: [F32;4],
	fRec14: [F32;2],
	fRec10: [F32;2048],
	fRec8: [F32;2],
	fConst12: F32,
	fRec7: [F32;2],
	fRec6: [F32;2],
	fRec4: [F32;2],
	fRec0: [F32;2],
	fHslider3: F32,
	fRec50: [F32;2],
	fRec80: [F32;2],
	fRec84: [F32;2],
	fRec89: [F32;2],
	iVec5: [i32;2],
	iRec90: [i32;2],
	fRec87: [F32;2],
	fRec86: [F32;2],
	fRec91: [F32;4],
	fRec92: [F32;2048],
	fVec6: [F32;2],
	fButton1: F32,
	fVec7: [F32;2],
	iRec93: [i32;2],
	fRec94: [F32;3],
	fVec8: [F32;3],
	fRec85: [F32;2048],
	fRec76: [F32;2],
	fRec72: [F32;2],
	fRec68: [F32;2048],
	fRec70: [F32;2],
	fRec66: [F32;4],
	fRec61: [F32;2],
	fRec57: [F32;2048],
	fRec55: [F32;2],
	fRec54: [F32;2],
	fRec53: [F32;2],
	fRec51: [F32;2],
	fRec49: [F32;2],
	fButton2: F32,
	fVec9: [F32;2],
	fHslider4: F32,
	fRec97: [F32;2],
	fRec96: [F32;2],
	fRec127: [F32;2],
	fRec131: [F32;2],
	fRec136: [F32;2],
	iVec10: [i32;2],
	iRec137: [i32;2],
	fRec134: [F32;2],
	fRec133: [F32;2],
	fRec138: [F32;4],
	fRec139: [F32;2048],
	fVec11: [F32;2],
	iRec140: [i32;2],
	fRec141: [F32;3],
	fVec12: [F32;3],
	fRec132: [F32;2048],
	fRec123: [F32;2],
	fRec119: [F32;2],
	fRec115: [F32;2048],
	fRec117: [F32;2],
	fRec113: [F32;4],
	fRec108: [F32;2],
	fRec104: [F32;2048],
	fRec102: [F32;2],
	fRec101: [F32;2],
	fRec100: [F32;2],
	fRec98: [F32;2],
	fRec95: [F32;2],
	fButton3: F32,
	fVec13: [F32;2],
	fHslider5: F32,
	fRec144: [F32;2],
	fRec143: [F32;2],
	fRec174: [F32;2],
	fRec178: [F32;2],
	fRec183: [F32;2],
	iVec14: [i32;2],
	iRec184: [i32;2],
	fRec181: [F32;2],
	fRec180: [F32;2],
	fRec185: [F32;4],
	fRec186: [F32;2048],
	fVec15: [F32;2],
	iRec187: [i32;2],
	fRec188: [F32;3],
	fVec16: [F32;3],
	fRec179: [F32;2048],
	fRec170: [F32;2],
	fRec166: [F32;2],
	fRec162: [F32;2048],
	fRec164: [F32;2],
	fRec160: [F32;4],
	fRec155: [F32;2],
	fRec151: [F32;2048],
	fRec149: [F32;2],
	fRec148: [F32;2],
	fRec147: [F32;2],
	fRec145: [F32;2],
	fRec142: [F32;2],
	fButton4: F32,
	fVec17: [F32;2],
	fHslider6: F32,
	fRec191: [F32;2],
	fRec190: [F32;2],
	fRec221: [F32;2],
	fRec225: [F32;2],
	fRec230: [F32;2],
	iVec18: [i32;2],
	iRec231: [i32;2],
	fRec228: [F32;2],
	fRec227: [F32;2],
	fRec232: [F32;4],
	fRec233: [F32;2048],
	fVec19: [F32;2],
	iRec234: [i32;2],
	fRec235: [F32;3],
	fVec20: [F32;3],
	fRec226: [F32;2048],
	fRec217: [F32;2],
	fRec213: [F32;2],
	fRec209: [F32;2048],
	fRec211: [F32;2],
	fRec207: [F32;4],
	fRec202: [F32;2],
	fRec198: [F32;2048],
	fRec196: [F32;2],
	fRec195: [F32;2],
	fRec194: [F32;2],
	fRec192: [F32;2],
	fRec189: [F32;2],
	fHslider7: F32,
	fRec236: [F32;2],
	fHslider8: F32,
	fRec237: [F32;2],
	fHslider9: F32,
	fRec239: [F32;2],
	fHslider10: F32,
	fConst13: F32,
	fRec238: [F32;2],
	fConst14: F32,
	fRec244: [F32;2],
	fConst15: F32,
	fRec242: [F32;2],
	fHslider11: F32,
	fRec245: [F32;2],
	fRec241: [F32;3],
	fRec240: [F32;3],
	fHslider12: F32,
	fRec246: [F32;2],
	fRec251: [F32;2],
	fRec249: [F32;2],
	fHslider13: F32,
	fRec252: [F32;2],
	fRec248: [F32;3],
	fRec247: [F32;3],
	fHslider14: F32,
	fRec253: [F32;2],
	fRec258: [F32;2],
	fRec256: [F32;2],
	fHslider15: F32,
	fRec259: [F32;2],
	fRec255: [F32;3],
	fRec254: [F32;3],
	fHslider16: F32,
	fRec260: [F32;2],
	fRec265: [F32;2],
	fRec263: [F32;2],
	fHslider17: F32,
	fRec266: [F32;2],
	fRec262: [F32;3],
	fRec261: [F32;3],
	fHslider18: F32,
	fRec267: [F32;2],
	fHslider19: F32,
	fRec268: [F32;2],
	fHslider20: F32,
	fRec269: [F32;2],
	fRec271: [F32;2],
	fVec21: [F32;2],
	fVec22: [F32;4096],
	fConst16: F32,
	fConst17: F32,
	fRec270: [F32;2],
	fRec273: [F32;2],
	fVec23: [F32;2],
	fVec24: [F32;4096],
	fRec272: [F32;2],
	fRec275: [F32;2],
	fVec25: [F32;2],
	fVec26: [F32;4096],
	fRec274: [F32;2],
	fRec277: [F32;2],
	fVec27: [F32;2],
	fVec28: [F32;4096],
	fRec276: [F32;2],
	fHslider21: F32,
	fRec278: [F32;2],
	fHslider22: F32,
	fRec279: [F32;2],
	fConst18: F32,
	fHslider23: F32,
	fRec280: [F32;2],
	fConst19: F32,
	fHslider24: F32,
	fRec282: [F32;2],
	fHslider25: F32,
	fRec281: [F32;2097152],
	fHslider26: F32,
	fRec283: [F32;2],
}

impl FaustDsp for mydsp {
	type T = F32;
		
	fn new() -> mydsp { 
		mydsp {
			fButton0: 0.0,
			fVec0: [0.0;2],
			iVec1: [0;2],
			fHslider0: 0.0,
			fRec2: [0.0;2],
			fSampleRate: 0,
			fConst0: 0.0,
			fConst1: 0.0,
			fConst2: 0.0,
			fRec1: [0.0;2],
			fHslider1: 0.0,
			fRec3: [0.0;2],
			fConst3: 0.0,
			IOTA0: 0,
			fConst4: 0.0,
			fConst5: 0.0,
			fRec33: [0.0;2],
			fRec37: [0.0;2],
			fConst6: 0.0,
			fConst7: 0.0,
			fRec42: [0.0;2],
			iVec2: [0;2],
			iConst8: 0,
			iRec43: [0;2],
			fConst9: 0.0,
			fRec40: [0.0;2],
			fRec39: [0.0;2],
			fRec44: [0.0;4],
			fRec45: [0.0;2048],
			fVec3: [0.0;2],
			fConst10: 0.0,
			fConst11: 0.0,
			iRec46: [0;2],
			iRec48: [0;2],
			fRec47: [0.0;3],
			fVec4: [0.0;3],
			fRec38: [0.0;2048],
			fRec29: [0.0;2],
			fRec25: [0.0;2],
			fRec21: [0.0;2048],
			fRec23: [0.0;2],
			fHslider2: 0.0,
			fRec19: [0.0;4],
			fRec14: [0.0;2],
			fRec10: [0.0;2048],
			fRec8: [0.0;2],
			fConst12: 0.0,
			fRec7: [0.0;2],
			fRec6: [0.0;2],
			fRec4: [0.0;2],
			fRec0: [0.0;2],
			fHslider3: 0.0,
			fRec50: [0.0;2],
			fRec80: [0.0;2],
			fRec84: [0.0;2],
			fRec89: [0.0;2],
			iVec5: [0;2],
			iRec90: [0;2],
			fRec87: [0.0;2],
			fRec86: [0.0;2],
			fRec91: [0.0;4],
			fRec92: [0.0;2048],
			fVec6: [0.0;2],
			fButton1: 0.0,
			fVec7: [0.0;2],
			iRec93: [0;2],
			fRec94: [0.0;3],
			fVec8: [0.0;3],
			fRec85: [0.0;2048],
			fRec76: [0.0;2],
			fRec72: [0.0;2],
			fRec68: [0.0;2048],
			fRec70: [0.0;2],
			fRec66: [0.0;4],
			fRec61: [0.0;2],
			fRec57: [0.0;2048],
			fRec55: [0.0;2],
			fRec54: [0.0;2],
			fRec53: [0.0;2],
			fRec51: [0.0;2],
			fRec49: [0.0;2],
			fButton2: 0.0,
			fVec9: [0.0;2],
			fHslider4: 0.0,
			fRec97: [0.0;2],
			fRec96: [0.0;2],
			fRec127: [0.0;2],
			fRec131: [0.0;2],
			fRec136: [0.0;2],
			iVec10: [0;2],
			iRec137: [0;2],
			fRec134: [0.0;2],
			fRec133: [0.0;2],
			fRec138: [0.0;4],
			fRec139: [0.0;2048],
			fVec11: [0.0;2],
			iRec140: [0;2],
			fRec141: [0.0;3],
			fVec12: [0.0;3],
			fRec132: [0.0;2048],
			fRec123: [0.0;2],
			fRec119: [0.0;2],
			fRec115: [0.0;2048],
			fRec117: [0.0;2],
			fRec113: [0.0;4],
			fRec108: [0.0;2],
			fRec104: [0.0;2048],
			fRec102: [0.0;2],
			fRec101: [0.0;2],
			fRec100: [0.0;2],
			fRec98: [0.0;2],
			fRec95: [0.0;2],
			fButton3: 0.0,
			fVec13: [0.0;2],
			fHslider5: 0.0,
			fRec144: [0.0;2],
			fRec143: [0.0;2],
			fRec174: [0.0;2],
			fRec178: [0.0;2],
			fRec183: [0.0;2],
			iVec14: [0;2],
			iRec184: [0;2],
			fRec181: [0.0;2],
			fRec180: [0.0;2],
			fRec185: [0.0;4],
			fRec186: [0.0;2048],
			fVec15: [0.0;2],
			iRec187: [0;2],
			fRec188: [0.0;3],
			fVec16: [0.0;3],
			fRec179: [0.0;2048],
			fRec170: [0.0;2],
			fRec166: [0.0;2],
			fRec162: [0.0;2048],
			fRec164: [0.0;2],
			fRec160: [0.0;4],
			fRec155: [0.0;2],
			fRec151: [0.0;2048],
			fRec149: [0.0;2],
			fRec148: [0.0;2],
			fRec147: [0.0;2],
			fRec145: [0.0;2],
			fRec142: [0.0;2],
			fButton4: 0.0,
			fVec17: [0.0;2],
			fHslider6: 0.0,
			fRec191: [0.0;2],
			fRec190: [0.0;2],
			fRec221: [0.0;2],
			fRec225: [0.0;2],
			fRec230: [0.0;2],
			iVec18: [0;2],
			iRec231: [0;2],
			fRec228: [0.0;2],
			fRec227: [0.0;2],
			fRec232: [0.0;4],
			fRec233: [0.0;2048],
			fVec19: [0.0;2],
			iRec234: [0;2],
			fRec235: [0.0;3],
			fVec20: [0.0;3],
			fRec226: [0.0;2048],
			fRec217: [0.0;2],
			fRec213: [0.0;2],
			fRec209: [0.0;2048],
			fRec211: [0.0;2],
			fRec207: [0.0;4],
			fRec202: [0.0;2],
			fRec198: [0.0;2048],
			fRec196: [0.0;2],
			fRec195: [0.0;2],
			fRec194: [0.0;2],
			fRec192: [0.0;2],
			fRec189: [0.0;2],
			fHslider7: 0.0,
			fRec236: [0.0;2],
			fHslider8: 0.0,
			fRec237: [0.0;2],
			fHslider9: 0.0,
			fRec239: [0.0;2],
			fHslider10: 0.0,
			fConst13: 0.0,
			fRec238: [0.0;2],
			fConst14: 0.0,
			fRec244: [0.0;2],
			fConst15: 0.0,
			fRec242: [0.0;2],
			fHslider11: 0.0,
			fRec245: [0.0;2],
			fRec241: [0.0;3],
			fRec240: [0.0;3],
			fHslider12: 0.0,
			fRec246: [0.0;2],
			fRec251: [0.0;2],
			fRec249: [0.0;2],
			fHslider13: 0.0,
			fRec252: [0.0;2],
			fRec248: [0.0;3],
			fRec247: [0.0;3],
			fHslider14: 0.0,
			fRec253: [0.0;2],
			fRec258: [0.0;2],
			fRec256: [0.0;2],
			fHslider15: 0.0,
			fRec259: [0.0;2],
			fRec255: [0.0;3],
			fRec254: [0.0;3],
			fHslider16: 0.0,
			fRec260: [0.0;2],
			fRec265: [0.0;2],
			fRec263: [0.0;2],
			fHslider17: 0.0,
			fRec266: [0.0;2],
			fRec262: [0.0;3],
			fRec261: [0.0;3],
			fHslider18: 0.0,
			fRec267: [0.0;2],
			fHslider19: 0.0,
			fRec268: [0.0;2],
			fHslider20: 0.0,
			fRec269: [0.0;2],
			fRec271: [0.0;2],
			fVec21: [0.0;2],
			fVec22: [0.0;4096],
			fConst16: 0.0,
			fConst17: 0.0,
			fRec270: [0.0;2],
			fRec273: [0.0;2],
			fVec23: [0.0;2],
			fVec24: [0.0;4096],
			fRec272: [0.0;2],
			fRec275: [0.0;2],
			fVec25: [0.0;2],
			fVec26: [0.0;4096],
			fRec274: [0.0;2],
			fRec277: [0.0;2],
			fVec27: [0.0;2],
			fVec28: [0.0;4096],
			fRec276: [0.0;2],
			fHslider21: 0.0,
			fRec278: [0.0;2],
			fHslider22: 0.0,
			fRec279: [0.0;2],
			fConst18: 0.0,
			fHslider23: 0.0,
			fRec280: [0.0;2],
			fConst19: 0.0,
			fHslider24: 0.0,
			fRec282: [0.0;2],
			fHslider25: 0.0,
			fRec281: [0.0;2097152],
			fHslider26: 0.0,
			fRec283: [0.0;2],
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
		self.fButton0 = 0.0;
		self.fHslider0 = 8e+01;
		self.fHslider1 = 0.0;
		self.fHslider2 = 1.0;
		self.fHslider3 = 8e+01;
		self.fButton1 = 0.0;
		self.fButton2 = 0.0;
		self.fHslider4 = 8e+01;
		self.fButton3 = 0.0;
		self.fHslider5 = 8e+01;
		self.fButton4 = 0.0;
		self.fHslider6 = 8e+01;
		self.fHslider7 = 1.0;
		self.fHslider8 = 0.0;
		self.fHslider9 = 0.0;
		self.fHslider10 = 6e+01;
		self.fHslider11 = 0.0;
		self.fHslider12 = 6e+01;
		self.fHslider13 = 0.0;
		self.fHslider14 = 6e+01;
		self.fHslider15 = 0.0;
		self.fHslider16 = 6e+01;
		self.fHslider17 = 0.0;
		self.fHslider18 = 1.0;
		self.fHslider19 = 0.0;
		self.fHslider20 = 6e+01;
		self.fHslider21 = 1.0;
		self.fHslider22 = 0.0;
		self.fHslider23 = 1.0;
		self.fHslider24 = 0.3;
		self.fHslider25 = 0.3;
		self.fHslider26 = 1.0;
	}
	fn instance_clear(&mut self) {
		for l0 in 0..2 {
			self.fVec0[(l0) as usize] = 0.0;
		}
		for l1 in 0..2 {
			self.iVec1[(l1) as usize] = 0;
		}
		for l2 in 0..2 {
			self.fRec2[(l2) as usize] = 0.0;
		}
		for l3 in 0..2 {
			self.fRec1[(l3) as usize] = 0.0;
		}
		for l4 in 0..2 {
			self.fRec3[(l4) as usize] = 0.0;
		}
		self.IOTA0 = 0;
		for l5 in 0..2 {
			self.fRec33[(l5) as usize] = 0.0;
		}
		for l6 in 0..2 {
			self.fRec37[(l6) as usize] = 0.0;
		}
		for l7 in 0..2 {
			self.fRec42[(l7) as usize] = 0.0;
		}
		for l8 in 0..2 {
			self.iVec2[(l8) as usize] = 0;
		}
		for l9 in 0..2 {
			self.iRec43[(l9) as usize] = 0;
		}
		for l10 in 0..2 {
			self.fRec40[(l10) as usize] = 0.0;
		}
		for l11 in 0..2 {
			self.fRec39[(l11) as usize] = 0.0;
		}
		for l12 in 0..4 {
			self.fRec44[(l12) as usize] = 0.0;
		}
		for l13 in 0..2048 {
			self.fRec45[(l13) as usize] = 0.0;
		}
		for l14 in 0..2 {
			self.fVec3[(l14) as usize] = 0.0;
		}
		for l15 in 0..2 {
			self.iRec46[(l15) as usize] = 0;
		}
		for l16 in 0..2 {
			self.iRec48[(l16) as usize] = 0;
		}
		for l17 in 0..3 {
			self.fRec47[(l17) as usize] = 0.0;
		}
		for l18 in 0..3 {
			self.fVec4[(l18) as usize] = 0.0;
		}
		for l19 in 0..2048 {
			self.fRec38[(l19) as usize] = 0.0;
		}
		for l20 in 0..2 {
			self.fRec29[(l20) as usize] = 0.0;
		}
		for l21 in 0..2 {
			self.fRec25[(l21) as usize] = 0.0;
		}
		for l22 in 0..2048 {
			self.fRec21[(l22) as usize] = 0.0;
		}
		for l23 in 0..2 {
			self.fRec23[(l23) as usize] = 0.0;
		}
		for l24 in 0..4 {
			self.fRec19[(l24) as usize] = 0.0;
		}
		for l25 in 0..2 {
			self.fRec14[(l25) as usize] = 0.0;
		}
		for l26 in 0..2048 {
			self.fRec10[(l26) as usize] = 0.0;
		}
		for l27 in 0..2 {
			self.fRec8[(l27) as usize] = 0.0;
		}
		for l28 in 0..2 {
			self.fRec7[(l28) as usize] = 0.0;
		}
		for l29 in 0..2 {
			self.fRec6[(l29) as usize] = 0.0;
		}
		for l30 in 0..2 {
			self.fRec4[(l30) as usize] = 0.0;
		}
		for l31 in 0..2 {
			self.fRec0[(l31) as usize] = 0.0;
		}
		for l32 in 0..2 {
			self.fRec50[(l32) as usize] = 0.0;
		}
		for l33 in 0..2 {
			self.fRec80[(l33) as usize] = 0.0;
		}
		for l34 in 0..2 {
			self.fRec84[(l34) as usize] = 0.0;
		}
		for l35 in 0..2 {
			self.fRec89[(l35) as usize] = 0.0;
		}
		for l36 in 0..2 {
			self.iVec5[(l36) as usize] = 0;
		}
		for l37 in 0..2 {
			self.iRec90[(l37) as usize] = 0;
		}
		for l38 in 0..2 {
			self.fRec87[(l38) as usize] = 0.0;
		}
		for l39 in 0..2 {
			self.fRec86[(l39) as usize] = 0.0;
		}
		for l40 in 0..4 {
			self.fRec91[(l40) as usize] = 0.0;
		}
		for l41 in 0..2048 {
			self.fRec92[(l41) as usize] = 0.0;
		}
		for l42 in 0..2 {
			self.fVec6[(l42) as usize] = 0.0;
		}
		for l43 in 0..2 {
			self.fVec7[(l43) as usize] = 0.0;
		}
		for l44 in 0..2 {
			self.iRec93[(l44) as usize] = 0;
		}
		for l45 in 0..3 {
			self.fRec94[(l45) as usize] = 0.0;
		}
		for l46 in 0..3 {
			self.fVec8[(l46) as usize] = 0.0;
		}
		for l47 in 0..2048 {
			self.fRec85[(l47) as usize] = 0.0;
		}
		for l48 in 0..2 {
			self.fRec76[(l48) as usize] = 0.0;
		}
		for l49 in 0..2 {
			self.fRec72[(l49) as usize] = 0.0;
		}
		for l50 in 0..2048 {
			self.fRec68[(l50) as usize] = 0.0;
		}
		for l51 in 0..2 {
			self.fRec70[(l51) as usize] = 0.0;
		}
		for l52 in 0..4 {
			self.fRec66[(l52) as usize] = 0.0;
		}
		for l53 in 0..2 {
			self.fRec61[(l53) as usize] = 0.0;
		}
		for l54 in 0..2048 {
			self.fRec57[(l54) as usize] = 0.0;
		}
		for l55 in 0..2 {
			self.fRec55[(l55) as usize] = 0.0;
		}
		for l56 in 0..2 {
			self.fRec54[(l56) as usize] = 0.0;
		}
		for l57 in 0..2 {
			self.fRec53[(l57) as usize] = 0.0;
		}
		for l58 in 0..2 {
			self.fRec51[(l58) as usize] = 0.0;
		}
		for l59 in 0..2 {
			self.fRec49[(l59) as usize] = 0.0;
		}
		for l60 in 0..2 {
			self.fVec9[(l60) as usize] = 0.0;
		}
		for l61 in 0..2 {
			self.fRec97[(l61) as usize] = 0.0;
		}
		for l62 in 0..2 {
			self.fRec96[(l62) as usize] = 0.0;
		}
		for l63 in 0..2 {
			self.fRec127[(l63) as usize] = 0.0;
		}
		for l64 in 0..2 {
			self.fRec131[(l64) as usize] = 0.0;
		}
		for l65 in 0..2 {
			self.fRec136[(l65) as usize] = 0.0;
		}
		for l66 in 0..2 {
			self.iVec10[(l66) as usize] = 0;
		}
		for l67 in 0..2 {
			self.iRec137[(l67) as usize] = 0;
		}
		for l68 in 0..2 {
			self.fRec134[(l68) as usize] = 0.0;
		}
		for l69 in 0..2 {
			self.fRec133[(l69) as usize] = 0.0;
		}
		for l70 in 0..4 {
			self.fRec138[(l70) as usize] = 0.0;
		}
		for l71 in 0..2048 {
			self.fRec139[(l71) as usize] = 0.0;
		}
		for l72 in 0..2 {
			self.fVec11[(l72) as usize] = 0.0;
		}
		for l73 in 0..2 {
			self.iRec140[(l73) as usize] = 0;
		}
		for l74 in 0..3 {
			self.fRec141[(l74) as usize] = 0.0;
		}
		for l75 in 0..3 {
			self.fVec12[(l75) as usize] = 0.0;
		}
		for l76 in 0..2048 {
			self.fRec132[(l76) as usize] = 0.0;
		}
		for l77 in 0..2 {
			self.fRec123[(l77) as usize] = 0.0;
		}
		for l78 in 0..2 {
			self.fRec119[(l78) as usize] = 0.0;
		}
		for l79 in 0..2048 {
			self.fRec115[(l79) as usize] = 0.0;
		}
		for l80 in 0..2 {
			self.fRec117[(l80) as usize] = 0.0;
		}
		for l81 in 0..4 {
			self.fRec113[(l81) as usize] = 0.0;
		}
		for l82 in 0..2 {
			self.fRec108[(l82) as usize] = 0.0;
		}
		for l83 in 0..2048 {
			self.fRec104[(l83) as usize] = 0.0;
		}
		for l84 in 0..2 {
			self.fRec102[(l84) as usize] = 0.0;
		}
		for l85 in 0..2 {
			self.fRec101[(l85) as usize] = 0.0;
		}
		for l86 in 0..2 {
			self.fRec100[(l86) as usize] = 0.0;
		}
		for l87 in 0..2 {
			self.fRec98[(l87) as usize] = 0.0;
		}
		for l88 in 0..2 {
			self.fRec95[(l88) as usize] = 0.0;
		}
		for l89 in 0..2 {
			self.fVec13[(l89) as usize] = 0.0;
		}
		for l90 in 0..2 {
			self.fRec144[(l90) as usize] = 0.0;
		}
		for l91 in 0..2 {
			self.fRec143[(l91) as usize] = 0.0;
		}
		for l92 in 0..2 {
			self.fRec174[(l92) as usize] = 0.0;
		}
		for l93 in 0..2 {
			self.fRec178[(l93) as usize] = 0.0;
		}
		for l94 in 0..2 {
			self.fRec183[(l94) as usize] = 0.0;
		}
		for l95 in 0..2 {
			self.iVec14[(l95) as usize] = 0;
		}
		for l96 in 0..2 {
			self.iRec184[(l96) as usize] = 0;
		}
		for l97 in 0..2 {
			self.fRec181[(l97) as usize] = 0.0;
		}
		for l98 in 0..2 {
			self.fRec180[(l98) as usize] = 0.0;
		}
		for l99 in 0..4 {
			self.fRec185[(l99) as usize] = 0.0;
		}
		for l100 in 0..2048 {
			self.fRec186[(l100) as usize] = 0.0;
		}
		for l101 in 0..2 {
			self.fVec15[(l101) as usize] = 0.0;
		}
		for l102 in 0..2 {
			self.iRec187[(l102) as usize] = 0;
		}
		for l103 in 0..3 {
			self.fRec188[(l103) as usize] = 0.0;
		}
		for l104 in 0..3 {
			self.fVec16[(l104) as usize] = 0.0;
		}
		for l105 in 0..2048 {
			self.fRec179[(l105) as usize] = 0.0;
		}
		for l106 in 0..2 {
			self.fRec170[(l106) as usize] = 0.0;
		}
		for l107 in 0..2 {
			self.fRec166[(l107) as usize] = 0.0;
		}
		for l108 in 0..2048 {
			self.fRec162[(l108) as usize] = 0.0;
		}
		for l109 in 0..2 {
			self.fRec164[(l109) as usize] = 0.0;
		}
		for l110 in 0..4 {
			self.fRec160[(l110) as usize] = 0.0;
		}
		for l111 in 0..2 {
			self.fRec155[(l111) as usize] = 0.0;
		}
		for l112 in 0..2048 {
			self.fRec151[(l112) as usize] = 0.0;
		}
		for l113 in 0..2 {
			self.fRec149[(l113) as usize] = 0.0;
		}
		for l114 in 0..2 {
			self.fRec148[(l114) as usize] = 0.0;
		}
		for l115 in 0..2 {
			self.fRec147[(l115) as usize] = 0.0;
		}
		for l116 in 0..2 {
			self.fRec145[(l116) as usize] = 0.0;
		}
		for l117 in 0..2 {
			self.fRec142[(l117) as usize] = 0.0;
		}
		for l118 in 0..2 {
			self.fVec17[(l118) as usize] = 0.0;
		}
		for l119 in 0..2 {
			self.fRec191[(l119) as usize] = 0.0;
		}
		for l120 in 0..2 {
			self.fRec190[(l120) as usize] = 0.0;
		}
		for l121 in 0..2 {
			self.fRec221[(l121) as usize] = 0.0;
		}
		for l122 in 0..2 {
			self.fRec225[(l122) as usize] = 0.0;
		}
		for l123 in 0..2 {
			self.fRec230[(l123) as usize] = 0.0;
		}
		for l124 in 0..2 {
			self.iVec18[(l124) as usize] = 0;
		}
		for l125 in 0..2 {
			self.iRec231[(l125) as usize] = 0;
		}
		for l126 in 0..2 {
			self.fRec228[(l126) as usize] = 0.0;
		}
		for l127 in 0..2 {
			self.fRec227[(l127) as usize] = 0.0;
		}
		for l128 in 0..4 {
			self.fRec232[(l128) as usize] = 0.0;
		}
		for l129 in 0..2048 {
			self.fRec233[(l129) as usize] = 0.0;
		}
		for l130 in 0..2 {
			self.fVec19[(l130) as usize] = 0.0;
		}
		for l131 in 0..2 {
			self.iRec234[(l131) as usize] = 0;
		}
		for l132 in 0..3 {
			self.fRec235[(l132) as usize] = 0.0;
		}
		for l133 in 0..3 {
			self.fVec20[(l133) as usize] = 0.0;
		}
		for l134 in 0..2048 {
			self.fRec226[(l134) as usize] = 0.0;
		}
		for l135 in 0..2 {
			self.fRec217[(l135) as usize] = 0.0;
		}
		for l136 in 0..2 {
			self.fRec213[(l136) as usize] = 0.0;
		}
		for l137 in 0..2048 {
			self.fRec209[(l137) as usize] = 0.0;
		}
		for l138 in 0..2 {
			self.fRec211[(l138) as usize] = 0.0;
		}
		for l139 in 0..4 {
			self.fRec207[(l139) as usize] = 0.0;
		}
		for l140 in 0..2 {
			self.fRec202[(l140) as usize] = 0.0;
		}
		for l141 in 0..2048 {
			self.fRec198[(l141) as usize] = 0.0;
		}
		for l142 in 0..2 {
			self.fRec196[(l142) as usize] = 0.0;
		}
		for l143 in 0..2 {
			self.fRec195[(l143) as usize] = 0.0;
		}
		for l144 in 0..2 {
			self.fRec194[(l144) as usize] = 0.0;
		}
		for l145 in 0..2 {
			self.fRec192[(l145) as usize] = 0.0;
		}
		for l146 in 0..2 {
			self.fRec189[(l146) as usize] = 0.0;
		}
		for l147 in 0..2 {
			self.fRec236[(l147) as usize] = 0.0;
		}
		for l148 in 0..2 {
			self.fRec237[(l148) as usize] = 0.0;
		}
		for l149 in 0..2 {
			self.fRec239[(l149) as usize] = 0.0;
		}
		for l150 in 0..2 {
			self.fRec238[(l150) as usize] = 0.0;
		}
		for l151 in 0..2 {
			self.fRec244[(l151) as usize] = 0.0;
		}
		for l152 in 0..2 {
			self.fRec242[(l152) as usize] = 0.0;
		}
		for l153 in 0..2 {
			self.fRec245[(l153) as usize] = 0.0;
		}
		for l154 in 0..3 {
			self.fRec241[(l154) as usize] = 0.0;
		}
		for l155 in 0..3 {
			self.fRec240[(l155) as usize] = 0.0;
		}
		for l156 in 0..2 {
			self.fRec246[(l156) as usize] = 0.0;
		}
		for l157 in 0..2 {
			self.fRec251[(l157) as usize] = 0.0;
		}
		for l158 in 0..2 {
			self.fRec249[(l158) as usize] = 0.0;
		}
		for l159 in 0..2 {
			self.fRec252[(l159) as usize] = 0.0;
		}
		for l160 in 0..3 {
			self.fRec248[(l160) as usize] = 0.0;
		}
		for l161 in 0..3 {
			self.fRec247[(l161) as usize] = 0.0;
		}
		for l162 in 0..2 {
			self.fRec253[(l162) as usize] = 0.0;
		}
		for l163 in 0..2 {
			self.fRec258[(l163) as usize] = 0.0;
		}
		for l164 in 0..2 {
			self.fRec256[(l164) as usize] = 0.0;
		}
		for l165 in 0..2 {
			self.fRec259[(l165) as usize] = 0.0;
		}
		for l166 in 0..3 {
			self.fRec255[(l166) as usize] = 0.0;
		}
		for l167 in 0..3 {
			self.fRec254[(l167) as usize] = 0.0;
		}
		for l168 in 0..2 {
			self.fRec260[(l168) as usize] = 0.0;
		}
		for l169 in 0..2 {
			self.fRec265[(l169) as usize] = 0.0;
		}
		for l170 in 0..2 {
			self.fRec263[(l170) as usize] = 0.0;
		}
		for l171 in 0..2 {
			self.fRec266[(l171) as usize] = 0.0;
		}
		for l172 in 0..3 {
			self.fRec262[(l172) as usize] = 0.0;
		}
		for l173 in 0..3 {
			self.fRec261[(l173) as usize] = 0.0;
		}
		for l174 in 0..2 {
			self.fRec267[(l174) as usize] = 0.0;
		}
		for l175 in 0..2 {
			self.fRec268[(l175) as usize] = 0.0;
		}
		for l176 in 0..2 {
			self.fRec269[(l176) as usize] = 0.0;
		}
		for l177 in 0..2 {
			self.fRec271[(l177) as usize] = 0.0;
		}
		for l178 in 0..2 {
			self.fVec21[(l178) as usize] = 0.0;
		}
		for l179 in 0..4096 {
			self.fVec22[(l179) as usize] = 0.0;
		}
		for l180 in 0..2 {
			self.fRec270[(l180) as usize] = 0.0;
		}
		for l181 in 0..2 {
			self.fRec273[(l181) as usize] = 0.0;
		}
		for l182 in 0..2 {
			self.fVec23[(l182) as usize] = 0.0;
		}
		for l183 in 0..4096 {
			self.fVec24[(l183) as usize] = 0.0;
		}
		for l184 in 0..2 {
			self.fRec272[(l184) as usize] = 0.0;
		}
		for l185 in 0..2 {
			self.fRec275[(l185) as usize] = 0.0;
		}
		for l186 in 0..2 {
			self.fVec25[(l186) as usize] = 0.0;
		}
		for l187 in 0..4096 {
			self.fVec26[(l187) as usize] = 0.0;
		}
		for l188 in 0..2 {
			self.fRec274[(l188) as usize] = 0.0;
		}
		for l189 in 0..2 {
			self.fRec277[(l189) as usize] = 0.0;
		}
		for l190 in 0..2 {
			self.fVec27[(l190) as usize] = 0.0;
		}
		for l191 in 0..4096 {
			self.fVec28[(l191) as usize] = 0.0;
		}
		for l192 in 0..2 {
			self.fRec276[(l192) as usize] = 0.0;
		}
		for l193 in 0..2 {
			self.fRec278[(l193) as usize] = 0.0;
		}
		for l194 in 0..2 {
			self.fRec279[(l194) as usize] = 0.0;
		}
		for l195 in 0..2 {
			self.fRec280[(l195) as usize] = 0.0;
		}
		for l196 in 0..2 {
			self.fRec282[(l196) as usize] = 0.0;
		}
		for l197 in 0..2097152 {
			self.fRec281[(l197) as usize] = 0.0;
		}
		for l198 in 0..2 {
			self.fRec283[(l198) as usize] = 0.0;
		}
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		self.fConst0 = F32::min(1.92e+05, F32::max(1.0, ((self.fSampleRate) as F32)));
		self.fConst1 = 44.1 / self.fConst0;
		self.fConst2 = 1.0 - self.fConst1;
		self.fConst3 = 2764.6016 / self.fConst0;
		self.fConst4 = 0.00882353 * self.fConst0;
		self.fConst5 = 0.00073529413 * self.fConst0;
		self.fConst6 = F32::exp(0.0 - 1e+04 / self.fConst0);
		self.fConst7 = 1.0 - self.fConst6;
		self.iConst8 = ((0.1 * self.fConst0) as i32);
		self.fConst9 = F32::exp(0.0 - 5e+01 / self.fConst0);
		self.fConst10 = 6911.504 / self.fConst0;
		self.fConst11 = 0.002 * self.fConst0;
		self.fConst12 = F32::exp(0.0 - 1e+01 / self.fConst0);
		self.fConst13 = 19404.0 / self.fConst0;
		self.fConst14 = 3.1415927 / self.fConst0;
		self.fConst15 = 1.0 / self.fConst0;
		self.fConst16 = 0.5 * self.fConst0;
		self.fConst17 = 0.25 * self.fConst0;
		self.fConst18 = 352.0 / self.fConst0;
		self.fConst19 = 1e+01 * self.fConst0;
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
		ui_interface.declare(None, "2", "");
		ui_interface.open_horizontal_box("fx");
		ui_interface.declare(None, "0", "");
		ui_interface.open_vertical_box("echo");
		ui_interface.declare(Some(ParamIndex(25)), "0", "");
		ui_interface.declare(Some(ParamIndex(25)), "scale", "log");
		ui_interface.add_horizontal_slider("duration", ParamIndex(25), 0.3, 0.01, 1e+01, 0.001);
		ui_interface.declare(Some(ParamIndex(26)), "0", "");
		ui_interface.add_horizontal_slider("mix", ParamIndex(26), 1.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(27)), "1", "");
		ui_interface.add_horizontal_slider("feedback", ParamIndex(27), 0.3, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.close_box();
		ui_interface.declare(None, "3", "");
		ui_interface.open_vertical_box("mix");
		ui_interface.declare(Some(ParamIndex(28)), "0", "");
		ui_interface.add_horizontal_slider("master", ParamIndex(28), 1.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(29)), "1", "");
		ui_interface.add_horizontal_slider("drone", ParamIndex(29), 1.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(30)), "2", "");
		ui_interface.add_horizontal_slider("lead", ParamIndex(30), 1.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(31)), "3", "");
		ui_interface.add_horizontal_slider("pluck", ParamIndex(31), 1.0, 0.0, 1.0, 0.001);
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
			15 => Some(self.fHslider0),
			22 => Some(self.fHslider1),
			9 => Some(self.fHslider10),
			10 => Some(self.fHslider11),
			7 => Some(self.fHslider12),
			8 => Some(self.fHslider13),
			5 => Some(self.fHslider14),
			6 => Some(self.fHslider15),
			3 => Some(self.fHslider16),
			4 => Some(self.fHslider17),
			30 => Some(self.fHslider18),
			0 => Some(self.fHslider19),
			13 => Some(self.fHslider2),
			24 => Some(self.fHslider20),
			29 => Some(self.fHslider21),
			23 => Some(self.fHslider22),
			26 => Some(self.fHslider23),
			25 => Some(self.fHslider24),
			27 => Some(self.fHslider25),
			28 => Some(self.fHslider26),
			12 => Some(self.fHslider3),
			17 => Some(self.fHslider4),
			19 => Some(self.fHslider5),
			21 => Some(self.fHslider6),
			31 => Some(self.fHslider7),
			2 => Some(self.fHslider8),
			1 => Some(self.fHslider9),
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
			15 => { self.fHslider0 = value }
			22 => { self.fHslider1 = value }
			9 => { self.fHslider10 = value }
			10 => { self.fHslider11 = value }
			7 => { self.fHslider12 = value }
			8 => { self.fHslider13 = value }
			5 => { self.fHslider14 = value }
			6 => { self.fHslider15 = value }
			3 => { self.fHslider16 = value }
			4 => { self.fHslider17 = value }
			30 => { self.fHslider18 = value }
			0 => { self.fHslider19 = value }
			13 => { self.fHslider2 = value }
			24 => { self.fHslider20 = value }
			29 => { self.fHslider21 = value }
			23 => { self.fHslider22 = value }
			26 => { self.fHslider23 = value }
			25 => { self.fHslider24 = value }
			27 => { self.fHslider25 = value }
			28 => { self.fHslider26 = value }
			12 => { self.fHslider3 = value }
			17 => { self.fHslider4 = value }
			19 => { self.fHslider5 = value }
			21 => { self.fHslider6 = value }
			31 => { self.fHslider7 = value }
			2 => { self.fHslider8 = value }
			1 => { self.fHslider9 = value }
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
		let mut fSlow0: F32 = self.fButton0;
		let mut iSlow1: i32 = ((1.0 - fSlow0) as i32);
		let mut fSlow2: F32 = self.fHslider0;
		let mut fSlow3: F32 = self.fConst1 * self.fHslider1;
		let mut fSlow4: F32 = self.fHslider2;
		let mut fSlow5: F32 = self.fConst1 * self.fHslider3;
		let mut fSlow6: F32 = self.fButton1;
		let mut fSlow7: F32 = self.fButton2;
		let mut iSlow8: i32 = ((1.0 - fSlow7) as i32);
		let mut fSlow9: F32 = self.fHslider4;
		let mut fSlow10: F32 = self.fButton3;
		let mut iSlow11: i32 = ((1.0 - fSlow10) as i32);
		let mut fSlow12: F32 = self.fHslider5;
		let mut fSlow13: F32 = self.fButton4;
		let mut iSlow14: i32 = ((1.0 - fSlow13) as i32);
		let mut fSlow15: F32 = self.fHslider6;
		let mut fSlow16: F32 = self.fConst1 * self.fHslider7;
		let mut fSlow17: F32 = self.fConst1 * self.fHslider8;
		let mut fSlow18: F32 = self.fConst1 * self.fHslider9;
		let mut fSlow19: F32 = self.fHslider10;
		let mut fSlow20: F32 = self.fConst13 * F32::powf(2.0, 0.083333336 * (fSlow19 + -69.0));
		let mut fSlow21: F32 = self.fConst1 * self.fHslider11;
		let mut fSlow22: F32 = self.fHslider12;
		let mut fSlow23: F32 = self.fConst13 * F32::powf(2.0, 0.083333336 * (fSlow22 + -69.0));
		let mut fSlow24: F32 = self.fConst1 * self.fHslider13;
		let mut fSlow25: F32 = self.fHslider14;
		let mut fSlow26: F32 = self.fConst13 * F32::powf(2.0, 0.083333336 * (fSlow25 + -69.0));
		let mut fSlow27: F32 = self.fConst1 * self.fHslider15;
		let mut fSlow28: F32 = self.fHslider16;
		let mut fSlow29: F32 = self.fConst13 * F32::powf(2.0, 0.083333336 * (fSlow28 + -69.0));
		let mut fSlow30: F32 = self.fConst1 * self.fHslider17;
		let mut fSlow31: F32 = self.fConst1 * self.fHslider18;
		let mut fSlow32: F32 = self.fConst1 * self.fHslider19;
		let mut fSlow33: F32 = self.fConst1 * self.fHslider20;
		let mut fSlow34: F32 = self.fConst1 * self.fHslider21;
		let mut fSlow35: F32 = self.fConst1 * self.fHslider22;
		let mut fSlow36: F32 = self.fConst1 * self.fHslider23;
		let mut fSlow37: F32 = self.fConst1 * self.fHslider24;
		let mut fSlow38: F32 = self.fHslider25;
		let mut fSlow39: F32 = self.fConst1 * self.fHslider26;
		let zipped_iterators = outputs0.zip(outputs1);
		for (output0, output1) in zipped_iterators {
			self.fVec0[0] = fSlow0;
			self.iVec1[0] = 1;
			self.fRec2[0] = if (iSlow1 as i32 != 0) { fSlow2 } else { self.fRec2[1] };
			self.fRec1[0] = self.fConst2 * self.fRec1[1] + self.fConst1 * self.fRec2[0];
			self.fRec3[0] = fSlow3 + self.fConst2 * self.fRec3[1];
			let mut fTemp0: F32 = F32::powf(2.0, 0.083333336 * (self.fRec3[0] + self.fRec1[0] + -69.0));
			let mut fTemp1: F32 = 1.0 / F32::tan(self.fConst3 * fTemp0);
			let mut fRec18: F32 = -1.0 * 0.9973053 * (0.9 * self.fRec19[2] + 0.05 * (self.fRec19[1] + self.fRec19[3]));
			let mut fTemp2: F32 = self.fConst5 * (0.77272725 / fTemp0 + -0.11);
			let mut fTemp3: F32 = fTemp2 + -1.499995;
			let mut iTemp4: i32 = ((fTemp3) as i32);
			let mut iTemp5: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp4, 4))) as F32))) as i32);
			let mut iTemp6: i32 = i32::wrapping_add(iTemp5, 1);
			let mut fTemp7: F32 = F32::floor(fTemp3);
			let mut fTemp8: F32 = fTemp2 + (-3.0 - fTemp7);
			let mut fTemp9: F32 = fTemp2 + (-2.0 - fTemp7);
			let mut fTemp10: F32 = fTemp2 + (-1.0 - fTemp7);
			let mut fTemp11: F32 = fTemp10 * fTemp9;
			let mut fTemp12: F32 = fTemp11 * fTemp8;
			let mut fTemp13: F32 = fTemp2 + (-4.0 - fTemp7);
			let mut fTemp14: F32 = 0.0 - fTemp13;
			let mut iTemp15: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp4, 3))) as F32))) as i32);
			let mut iTemp16: i32 = i32::wrapping_add(iTemp15, 1);
			let mut fTemp17: F32 = 0.0 - 0.5 * fTemp13;
			let mut fTemp18: F32 = 0.0 - fTemp8;
			let mut iTemp19: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp4, 2))) as F32))) as i32);
			let mut iTemp20: i32 = i32::wrapping_add(iTemp19, 1);
			let mut fTemp21: F32 = 0.0 - 0.33333334 * fTemp13;
			let mut fTemp22: F32 = 0.0 - 0.5 * fTemp8;
			let mut fTemp23: F32 = 0.0 - fTemp9;
			let mut iTemp24: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp4, 1))) as F32))) as i32);
			let mut iTemp25: i32 = i32::wrapping_add(iTemp24, 1);
			let mut fTemp26: F32 = fTemp2 - fTemp7;
			let mut fTemp27: F32 = 0.0 - 0.25 * fTemp13;
			let mut fTemp28: F32 = 0.0 - 0.33333334 * fTemp8;
			let mut fTemp29: F32 = 0.0 - 0.5 * fTemp9;
			let mut fTemp30: F32 = 0.0 - fTemp10;
			let mut iTemp31: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, iTemp4)) as F32))) as i32);
			let mut iTemp32: i32 = i32::wrapping_add(iTemp31, 1);
			self.fRec33[0] = self.fRec10[((i32::wrapping_sub(self.IOTA0, iTemp32)) & 2047) as usize] * fTemp30 * fTemp29 * fTemp28 * fTemp27 + fTemp26 * (self.fRec10[((i32::wrapping_sub(self.IOTA0, iTemp25)) & 2047) as usize] * fTemp23 * fTemp22 * fTemp21 + 0.5 * fTemp10 * self.fRec10[((i32::wrapping_sub(self.IOTA0, iTemp20)) & 2047) as usize] * fTemp18 * fTemp17 + 0.16666667 * fTemp11 * self.fRec10[((i32::wrapping_sub(self.IOTA0, iTemp16)) & 2047) as usize] * fTemp14 + 0.041666668 * fTemp12 * self.fRec10[((i32::wrapping_sub(self.IOTA0, iTemp6)) & 2047) as usize]);
			self.fRec37[0] = 0.05 * self.fRec37[1] + 0.95 * self.fRec33[1];
			let mut fRec34: F32 = self.fRec37[0];
			self.fRec42[0] = self.fConst6 * self.fRec42[1] + self.fConst7 * F32::abs(self.fRec4[1]);
			let mut fRec41: F32 = self.fRec42[0];
			let mut iTemp33: i32 = ((fRec41 > 0.1) as i32);
			self.iVec2[0] = iTemp33;
			self.iRec43[0] = std::cmp::max(i32::wrapping_mul(self.iConst8, ((iTemp33 < self.iVec2[1]) as i32)), i32::wrapping_add(self.iRec43[1], -1));
			let mut fTemp34: F32 = F32::abs(F32::max(((iTemp33) as F32), ((((self.iRec43[0] > 0) as i32)) as F32)));
			let mut fTemp35: F32 = if (((self.fRec39[1] > fTemp34) as i32) as i32 != 0) { self.fConst9 } else { self.fConst6 };
			self.fRec40[0] = self.fRec40[1] * fTemp35 + fTemp34 * (1.0 - fTemp35);
			self.fRec39[0] = self.fRec40[0];
			let mut fTemp36: F32 = 0.005 * self.fRec39[0] * self.fRec4[1];
			self.fRec44[0] = self.fRec8[1];
			self.fRec45[(self.IOTA0 & 2047) as usize] = -1.0 * 0.9973053 * (0.9 * self.fRec44[2] + 0.05 * (self.fRec44[1] + self.fRec44[3]));
			let mut fTemp37: F32 = fTemp11 * fTemp14;
			let mut fTemp38: F32 = fTemp10 * fTemp18 * fTemp17;
			let mut fTemp39: F32 = fTemp23 * fTemp22 * fTemp21;
			let mut fTemp40: F32 = fTemp30 * fTemp29 * fTemp28 * fTemp27;
			self.fVec3[0] = fTemp40 * self.fRec45[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp31, 2))) & 2047) as usize] + fTemp26 * (fTemp39 * self.fRec45[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp24, 2))) & 2047) as usize] + 0.5 * fTemp38 * self.fRec45[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp19, 2))) & 2047) as usize] + 0.16666667 * fTemp37 * self.fRec45[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp15, 2))) & 2047) as usize] + 0.041666668 * fTemp12 * self.fRec45[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp5, 2))) & 2047) as usize]);
			let mut fTemp41: F32 = F32::tan(self.fConst10 * fTemp0);
			let mut fTemp42: F32 = 1.0 / fTemp41;
			let mut fTemp43: F32 = (fTemp42 + 1.4142135) / fTemp41 + 1.0;
			self.iRec46[0] = i32::wrapping_add(i32::wrapping_mul(i32::wrapping_add(self.iRec46[1], ((self.iRec46[1] > 0) as i32)), ((fSlow0 <= self.fVec0[1]) as i32)), ((fSlow0 > self.fVec0[1]) as i32));
			let mut fTemp44: F32 = ((self.iRec46[0]) as F32) / F32::max(1.0, self.fConst11 * mydsp_faustpower2_f(1.0 - 0.22 * fTemp0));
			self.iRec48[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, self.iRec48[1]), 12345);
			let mut fTemp45: F32 = 4.656613e-10 * ((self.iRec48[0]) as F32);
			self.fRec47[0] = fTemp45 - (self.fRec47[2] * ((fTemp42 + -1.4142135) / fTemp41 + 1.0) + 2.0 * self.fRec47[1] * (1.0 - 1.0 / mydsp_faustpower2_f(fTemp41))) / fTemp43;
			let mut fTemp46: F32 = 0.5 * ((self.fRec47[2] + self.fRec47[0] + 2.0 * self.fRec47[1]) * F32::max(0.0, F32::min(fTemp44, 2.0 - fTemp44)) / fTemp43);
			let mut fTemp47: F32 = fTemp46 + self.fVec3[1] + fTemp36;
			self.fVec4[0] = fTemp47;
			self.fRec38[(self.IOTA0 & 2047) as usize] = 0.05 * self.fRec38[((i32::wrapping_sub(self.IOTA0, 1)) & 2047) as usize] + 0.95 * self.fVec4[2];
			let mut fRec35: F32 = fTemp40 * self.fRec38[((i32::wrapping_sub(self.IOTA0, iTemp31)) & 2047) as usize] + fTemp26 * (fTemp39 * self.fRec38[((i32::wrapping_sub(self.IOTA0, iTemp24)) & 2047) as usize] + 0.5 * fTemp38 * self.fRec38[((i32::wrapping_sub(self.IOTA0, iTemp19)) & 2047) as usize] + 0.16666667 * fTemp37 * self.fRec38[((i32::wrapping_sub(self.IOTA0, iTemp15)) & 2047) as usize] + 0.041666668 * fTemp12 * self.fRec38[((i32::wrapping_sub(self.IOTA0, iTemp5)) & 2047) as usize]);
			let mut fRec36: F32 = self.fVec4[1] + self.fRec29[1];
			self.fRec29[0] = fRec34;
			let mut fRec30: F32 = self.fRec29[1];
			let mut fRec31: F32 = fRec35;
			let mut fRec32: F32 = fRec36;
			self.fRec25[0] = fRec30;
			let mut fRec26: F32 = fTemp36 + fTemp46 + self.fRec25[1];
			let mut fRec27: F32 = fRec31;
			let mut fRec28: F32 = fRec32;
			self.fRec21[(self.IOTA0 & 2047) as usize] = fRec26;
			let mut fRec22: F32 = fTemp40 * self.fRec21[((i32::wrapping_sub(self.IOTA0, iTemp32)) & 2047) as usize] + fTemp26 * (fTemp39 * self.fRec21[((i32::wrapping_sub(self.IOTA0, iTemp25)) & 2047) as usize] + 0.5 * fTemp38 * self.fRec21[((i32::wrapping_sub(self.IOTA0, iTemp20)) & 2047) as usize] + 0.16666667 * fTemp37 * self.fRec21[((i32::wrapping_sub(self.IOTA0, iTemp16)) & 2047) as usize] + 0.041666668 * fTemp12 * self.fRec21[((i32::wrapping_sub(self.IOTA0, iTemp6)) & 2047) as usize]);
			self.fRec23[0] = fRec27;
			let mut fRec24: F32 = fRec28;
			self.fRec19[0] = fSlow4 * self.fRec23[1];
			let mut fRec20: F32 = fRec24;
			self.fRec14[0] = fRec18;
			let mut fRec15: F32 = fSlow4 * self.fRec14[1];
			let mut fRec16: F32 = self.fRec19[0];
			let mut fRec17: F32 = fRec20;
			self.fRec10[(self.IOTA0 & 2047) as usize] = fRec15;
			let mut fRec11: F32 = fRec22;
			let mut fRec12: F32 = fRec16;
			let mut fRec13: F32 = fRec17;
			self.fRec8[0] = fRec11;
			let mut fRec9: F32 = fRec13;
			let mut fTemp48: F32 = F32::abs(fRec9);
			let mut fTemp49: F32 = if (((self.fRec6[1] > fTemp48) as i32) as i32 != 0) { self.fConst12 } else { 0.0 };
			self.fRec7[0] = self.fRec7[1] * fTemp49 + fTemp48 * (1.0 - fTemp49);
			self.fRec6[0] = self.fRec7[0];
			let mut fRec5: F32 = 0.0 - 0.95 * F32::max(2e+01 * F32::log10(self.fRec6[0]) + 1e+01, 0.0);
			self.fRec4[0] = fRec9 * F32::powf(1e+01, 0.05 * fRec5);
			self.fRec0[0] = 0.0 - (self.fRec0[1] * (1.0 - fTemp1) - (self.fRec4[0] + self.fRec4[1])) / (fTemp1 + 1.0);
			self.fRec50[0] = fSlow5 + self.fConst2 * self.fRec50[1];
			let mut fTemp50: F32 = F32::powf(2.0, 0.083333336 * (self.fRec50[0] + self.fRec3[0] + -69.0));
			let mut fTemp51: F32 = 1.0 / F32::tan(self.fConst3 * fTemp50);
			let mut fRec65: F32 = -1.0 * 0.9973053 * (0.9 * self.fRec66[2] + 0.05 * (self.fRec66[1] + self.fRec66[3]));
			let mut fTemp52: F32 = self.fConst5 * (0.77272725 / fTemp50 + -0.11);
			let mut fTemp53: F32 = fTemp52 + -1.499995;
			let mut iTemp54: i32 = ((fTemp53) as i32);
			let mut iTemp55: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp54, 4))) as F32))) as i32);
			let mut iTemp56: i32 = i32::wrapping_add(iTemp55, 1);
			let mut fTemp57: F32 = F32::floor(fTemp53);
			let mut fTemp58: F32 = fTemp52 + (-3.0 - fTemp57);
			let mut fTemp59: F32 = fTemp52 + (-2.0 - fTemp57);
			let mut fTemp60: F32 = fTemp52 + (-1.0 - fTemp57);
			let mut fTemp61: F32 = fTemp60 * fTemp59;
			let mut fTemp62: F32 = fTemp61 * fTemp58;
			let mut fTemp63: F32 = fTemp52 + (-4.0 - fTemp57);
			let mut fTemp64: F32 = 0.0 - fTemp63;
			let mut iTemp65: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp54, 3))) as F32))) as i32);
			let mut iTemp66: i32 = i32::wrapping_add(iTemp65, 1);
			let mut fTemp67: F32 = 0.0 - 0.5 * fTemp63;
			let mut fTemp68: F32 = 0.0 - fTemp58;
			let mut iTemp69: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp54, 2))) as F32))) as i32);
			let mut iTemp70: i32 = i32::wrapping_add(iTemp69, 1);
			let mut fTemp71: F32 = 0.0 - 0.33333334 * fTemp63;
			let mut fTemp72: F32 = 0.0 - 0.5 * fTemp58;
			let mut fTemp73: F32 = 0.0 - fTemp59;
			let mut iTemp74: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp54, 1))) as F32))) as i32);
			let mut iTemp75: i32 = i32::wrapping_add(iTemp74, 1);
			let mut fTemp76: F32 = fTemp52 - fTemp57;
			let mut fTemp77: F32 = 0.0 - 0.25 * fTemp63;
			let mut fTemp78: F32 = 0.0 - 0.33333334 * fTemp58;
			let mut fTemp79: F32 = 0.0 - 0.5 * fTemp59;
			let mut fTemp80: F32 = 0.0 - fTemp60;
			let mut iTemp81: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, iTemp54)) as F32))) as i32);
			let mut iTemp82: i32 = i32::wrapping_add(iTemp81, 1);
			self.fRec80[0] = self.fRec57[((i32::wrapping_sub(self.IOTA0, iTemp82)) & 2047) as usize] * fTemp80 * fTemp79 * fTemp78 * fTemp77 + fTemp76 * (self.fRec57[((i32::wrapping_sub(self.IOTA0, iTemp75)) & 2047) as usize] * fTemp73 * fTemp72 * fTemp71 + 0.5 * fTemp60 * self.fRec57[((i32::wrapping_sub(self.IOTA0, iTemp70)) & 2047) as usize] * fTemp68 * fTemp67 + 0.16666667 * fTemp61 * self.fRec57[((i32::wrapping_sub(self.IOTA0, iTemp66)) & 2047) as usize] * fTemp64 + 0.041666668 * fTemp62 * self.fRec57[((i32::wrapping_sub(self.IOTA0, iTemp56)) & 2047) as usize]);
			self.fRec84[0] = 0.05 * self.fRec84[1] + 0.95 * self.fRec80[1];
			let mut fRec81: F32 = self.fRec84[0];
			self.fRec89[0] = self.fConst6 * self.fRec89[1] + self.fConst7 * F32::abs(self.fRec51[1]);
			let mut fRec88: F32 = self.fRec89[0];
			let mut iTemp83: i32 = ((fRec88 > 0.1) as i32);
			self.iVec5[0] = iTemp83;
			self.iRec90[0] = std::cmp::max(i32::wrapping_mul(self.iConst8, ((iTemp83 < self.iVec5[1]) as i32)), i32::wrapping_add(self.iRec90[1], -1));
			let mut fTemp84: F32 = F32::abs(F32::max(((iTemp83) as F32), ((((self.iRec90[0] > 0) as i32)) as F32)));
			let mut fTemp85: F32 = if (((self.fRec86[1] > fTemp84) as i32) as i32 != 0) { self.fConst9 } else { self.fConst6 };
			self.fRec87[0] = self.fRec87[1] * fTemp85 + fTemp84 * (1.0 - fTemp85);
			self.fRec86[0] = self.fRec87[0];
			let mut fTemp86: F32 = 0.005 * self.fRec86[0] * self.fRec51[1];
			self.fRec91[0] = self.fRec55[1];
			self.fRec92[(self.IOTA0 & 2047) as usize] = -1.0 * 0.9973053 * (0.9 * self.fRec91[2] + 0.05 * (self.fRec91[1] + self.fRec91[3]));
			let mut fTemp87: F32 = fTemp61 * fTemp64;
			let mut fTemp88: F32 = fTemp60 * fTemp68 * fTemp67;
			let mut fTemp89: F32 = fTemp73 * fTemp72 * fTemp71;
			let mut fTemp90: F32 = fTemp80 * fTemp79 * fTemp78 * fTemp77;
			self.fVec6[0] = fTemp90 * self.fRec92[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp81, 2))) & 2047) as usize] + fTemp76 * (fTemp89 * self.fRec92[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp74, 2))) & 2047) as usize] + 0.5 * fTemp88 * self.fRec92[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp69, 2))) & 2047) as usize] + 0.16666667 * fTemp87 * self.fRec92[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp65, 2))) & 2047) as usize] + 0.041666668 * fTemp62 * self.fRec92[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp55, 2))) & 2047) as usize]);
			let mut fTemp91: F32 = F32::tan(self.fConst10 * fTemp50);
			let mut fTemp92: F32 = 1.0 / fTemp91;
			let mut fTemp93: F32 = (fTemp92 + 1.4142135) / fTemp91 + 1.0;
			self.fVec7[0] = fSlow6;
			self.iRec93[0] = i32::wrapping_add(i32::wrapping_mul(i32::wrapping_add(self.iRec93[1], ((self.iRec93[1] > 0) as i32)), ((fSlow6 <= self.fVec7[1]) as i32)), ((fSlow6 > self.fVec7[1]) as i32));
			let mut fTemp94: F32 = ((self.iRec93[0]) as F32) / F32::max(1.0, self.fConst11 * mydsp_faustpower2_f(1.0 - 0.22 * fTemp50));
			self.fRec94[0] = fTemp45 - (self.fRec94[2] * ((fTemp92 + -1.4142135) / fTemp91 + 1.0) + 2.0 * self.fRec94[1] * (1.0 - 1.0 / mydsp_faustpower2_f(fTemp91))) / fTemp93;
			let mut fTemp95: F32 = 0.5 * ((self.fRec94[2] + self.fRec94[0] + 2.0 * self.fRec94[1]) * F32::max(0.0, F32::min(fTemp94, 2.0 - fTemp94)) / fTemp93);
			let mut fTemp96: F32 = fTemp95 + self.fVec6[1] + fTemp86;
			self.fVec8[0] = fTemp96;
			self.fRec85[(self.IOTA0 & 2047) as usize] = 0.05 * self.fRec85[((i32::wrapping_sub(self.IOTA0, 1)) & 2047) as usize] + 0.95 * self.fVec8[2];
			let mut fRec82: F32 = fTemp90 * self.fRec85[((i32::wrapping_sub(self.IOTA0, iTemp81)) & 2047) as usize] + fTemp76 * (fTemp89 * self.fRec85[((i32::wrapping_sub(self.IOTA0, iTemp74)) & 2047) as usize] + 0.5 * fTemp88 * self.fRec85[((i32::wrapping_sub(self.IOTA0, iTemp69)) & 2047) as usize] + 0.16666667 * fTemp87 * self.fRec85[((i32::wrapping_sub(self.IOTA0, iTemp65)) & 2047) as usize] + 0.041666668 * fTemp62 * self.fRec85[((i32::wrapping_sub(self.IOTA0, iTemp55)) & 2047) as usize]);
			let mut fRec83: F32 = self.fVec8[1] + self.fRec76[1];
			self.fRec76[0] = fRec81;
			let mut fRec77: F32 = self.fRec76[1];
			let mut fRec78: F32 = fRec82;
			let mut fRec79: F32 = fRec83;
			self.fRec72[0] = fRec77;
			let mut fRec73: F32 = fTemp86 + fTemp95 + self.fRec72[1];
			let mut fRec74: F32 = fRec78;
			let mut fRec75: F32 = fRec79;
			self.fRec68[(self.IOTA0 & 2047) as usize] = fRec73;
			let mut fRec69: F32 = fTemp90 * self.fRec68[((i32::wrapping_sub(self.IOTA0, iTemp82)) & 2047) as usize] + fTemp76 * (fTemp89 * self.fRec68[((i32::wrapping_sub(self.IOTA0, iTemp75)) & 2047) as usize] + 0.5 * fTemp88 * self.fRec68[((i32::wrapping_sub(self.IOTA0, iTemp70)) & 2047) as usize] + 0.16666667 * fTemp87 * self.fRec68[((i32::wrapping_sub(self.IOTA0, iTemp66)) & 2047) as usize] + 0.041666668 * fTemp62 * self.fRec68[((i32::wrapping_sub(self.IOTA0, iTemp56)) & 2047) as usize]);
			self.fRec70[0] = fRec74;
			let mut fRec71: F32 = fRec75;
			self.fRec66[0] = fSlow4 * self.fRec70[1];
			let mut fRec67: F32 = fRec71;
			self.fRec61[0] = fRec65;
			let mut fRec62: F32 = fSlow4 * self.fRec61[1];
			let mut fRec63: F32 = self.fRec66[0];
			let mut fRec64: F32 = fRec67;
			self.fRec57[(self.IOTA0 & 2047) as usize] = fRec62;
			let mut fRec58: F32 = fRec69;
			let mut fRec59: F32 = fRec63;
			let mut fRec60: F32 = fRec64;
			self.fRec55[0] = fRec58;
			let mut fRec56: F32 = fRec60;
			let mut fTemp97: F32 = F32::abs(fRec56);
			let mut fTemp98: F32 = if (((self.fRec53[1] > fTemp97) as i32) as i32 != 0) { self.fConst12 } else { 0.0 };
			self.fRec54[0] = self.fRec54[1] * fTemp98 + fTemp97 * (1.0 - fTemp98);
			self.fRec53[0] = self.fRec54[0];
			let mut fRec52: F32 = 0.0 - 0.95 * F32::max(2e+01 * F32::log10(self.fRec53[0]) + 1e+01, 0.0);
			self.fRec51[0] = fRec56 * F32::powf(1e+01, 0.05 * fRec52);
			self.fRec49[0] = 0.0 - (self.fRec49[1] * (1.0 - fTemp51) - (self.fRec51[0] + self.fRec51[1])) / (fTemp51 + 1.0);
			self.fVec9[0] = fSlow7;
			self.fRec97[0] = if (iSlow8 as i32 != 0) { fSlow9 } else { self.fRec97[1] };
			self.fRec96[0] = self.fConst2 * self.fRec96[1] + self.fConst1 * self.fRec97[0];
			let mut fTemp99: F32 = F32::powf(2.0, 0.083333336 * (self.fRec3[0] + self.fRec96[0] + -69.0));
			let mut fTemp100: F32 = 1.0 / F32::tan(self.fConst3 * fTemp99);
			let mut fRec112: F32 = -1.0 * 0.9973053 * (0.9 * self.fRec113[2] + 0.05 * (self.fRec113[1] + self.fRec113[3]));
			let mut fTemp101: F32 = self.fConst5 * (0.77272725 / fTemp99 + -0.11);
			let mut fTemp102: F32 = fTemp101 + -1.499995;
			let mut iTemp103: i32 = ((fTemp102) as i32);
			let mut iTemp104: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp103, 4))) as F32))) as i32);
			let mut iTemp105: i32 = i32::wrapping_add(iTemp104, 1);
			let mut fTemp106: F32 = F32::floor(fTemp102);
			let mut fTemp107: F32 = fTemp101 + (-3.0 - fTemp106);
			let mut fTemp108: F32 = fTemp101 + (-2.0 - fTemp106);
			let mut fTemp109: F32 = fTemp101 + (-1.0 - fTemp106);
			let mut fTemp110: F32 = fTemp109 * fTemp108;
			let mut fTemp111: F32 = fTemp110 * fTemp107;
			let mut fTemp112: F32 = fTemp101 + (-4.0 - fTemp106);
			let mut fTemp113: F32 = 0.0 - fTemp112;
			let mut iTemp114: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp103, 3))) as F32))) as i32);
			let mut iTemp115: i32 = i32::wrapping_add(iTemp114, 1);
			let mut fTemp116: F32 = 0.0 - 0.5 * fTemp112;
			let mut fTemp117: F32 = 0.0 - fTemp107;
			let mut iTemp118: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp103, 2))) as F32))) as i32);
			let mut iTemp119: i32 = i32::wrapping_add(iTemp118, 1);
			let mut fTemp120: F32 = 0.0 - 0.33333334 * fTemp112;
			let mut fTemp121: F32 = 0.0 - 0.5 * fTemp107;
			let mut fTemp122: F32 = 0.0 - fTemp108;
			let mut iTemp123: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp103, 1))) as F32))) as i32);
			let mut iTemp124: i32 = i32::wrapping_add(iTemp123, 1);
			let mut fTemp125: F32 = fTemp101 - fTemp106;
			let mut fTemp126: F32 = 0.0 - 0.25 * fTemp112;
			let mut fTemp127: F32 = 0.0 - 0.33333334 * fTemp107;
			let mut fTemp128: F32 = 0.0 - 0.5 * fTemp108;
			let mut fTemp129: F32 = 0.0 - fTemp109;
			let mut iTemp130: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, iTemp103)) as F32))) as i32);
			let mut iTemp131: i32 = i32::wrapping_add(iTemp130, 1);
			self.fRec127[0] = self.fRec104[((i32::wrapping_sub(self.IOTA0, iTemp131)) & 2047) as usize] * fTemp129 * fTemp128 * fTemp127 * fTemp126 + fTemp125 * (self.fRec104[((i32::wrapping_sub(self.IOTA0, iTemp124)) & 2047) as usize] * fTemp122 * fTemp121 * fTemp120 + 0.5 * fTemp109 * self.fRec104[((i32::wrapping_sub(self.IOTA0, iTemp119)) & 2047) as usize] * fTemp117 * fTemp116 + 0.16666667 * fTemp110 * self.fRec104[((i32::wrapping_sub(self.IOTA0, iTemp115)) & 2047) as usize] * fTemp113 + 0.041666668 * fTemp111 * self.fRec104[((i32::wrapping_sub(self.IOTA0, iTemp105)) & 2047) as usize]);
			self.fRec131[0] = 0.05 * self.fRec131[1] + 0.95 * self.fRec127[1];
			let mut fRec128: F32 = self.fRec131[0];
			self.fRec136[0] = self.fConst6 * self.fRec136[1] + self.fConst7 * F32::abs(self.fRec98[1]);
			let mut fRec135: F32 = self.fRec136[0];
			let mut iTemp132: i32 = ((fRec135 > 0.1) as i32);
			self.iVec10[0] = iTemp132;
			self.iRec137[0] = std::cmp::max(i32::wrapping_mul(self.iConst8, ((iTemp132 < self.iVec10[1]) as i32)), i32::wrapping_add(self.iRec137[1], -1));
			let mut fTemp133: F32 = F32::abs(F32::max(((iTemp132) as F32), ((((self.iRec137[0] > 0) as i32)) as F32)));
			let mut fTemp134: F32 = if (((self.fRec133[1] > fTemp133) as i32) as i32 != 0) { self.fConst9 } else { self.fConst6 };
			self.fRec134[0] = self.fRec134[1] * fTemp134 + fTemp133 * (1.0 - fTemp134);
			self.fRec133[0] = self.fRec134[0];
			let mut fTemp135: F32 = 0.005 * self.fRec133[0] * self.fRec98[1];
			self.fRec138[0] = self.fRec102[1];
			self.fRec139[(self.IOTA0 & 2047) as usize] = -1.0 * 0.9973053 * (0.9 * self.fRec138[2] + 0.05 * (self.fRec138[1] + self.fRec138[3]));
			let mut fTemp136: F32 = fTemp110 * fTemp113;
			let mut fTemp137: F32 = fTemp109 * fTemp117 * fTemp116;
			let mut fTemp138: F32 = fTemp122 * fTemp121 * fTemp120;
			let mut fTemp139: F32 = fTemp129 * fTemp128 * fTemp127 * fTemp126;
			self.fVec11[0] = fTemp139 * self.fRec139[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp130, 2))) & 2047) as usize] + fTemp125 * (fTemp138 * self.fRec139[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp123, 2))) & 2047) as usize] + 0.5 * fTemp137 * self.fRec139[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp118, 2))) & 2047) as usize] + 0.16666667 * fTemp136 * self.fRec139[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp114, 2))) & 2047) as usize] + 0.041666668 * fTemp111 * self.fRec139[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp104, 2))) & 2047) as usize]);
			let mut fTemp140: F32 = F32::tan(self.fConst10 * fTemp99);
			let mut fTemp141: F32 = 1.0 / fTemp140;
			let mut fTemp142: F32 = (fTemp141 + 1.4142135) / fTemp140 + 1.0;
			self.iRec140[0] = i32::wrapping_add(i32::wrapping_mul(i32::wrapping_add(self.iRec140[1], ((self.iRec140[1] > 0) as i32)), ((fSlow7 <= self.fVec9[1]) as i32)), ((fSlow7 > self.fVec9[1]) as i32));
			let mut fTemp143: F32 = ((self.iRec140[0]) as F32) / F32::max(1.0, self.fConst11 * mydsp_faustpower2_f(1.0 - 0.22 * fTemp99));
			self.fRec141[0] = fTemp45 - (self.fRec141[2] * ((fTemp141 + -1.4142135) / fTemp140 + 1.0) + 2.0 * self.fRec141[1] * (1.0 - 1.0 / mydsp_faustpower2_f(fTemp140))) / fTemp142;
			let mut fTemp144: F32 = 0.5 * ((self.fRec141[2] + self.fRec141[0] + 2.0 * self.fRec141[1]) * F32::max(0.0, F32::min(fTemp143, 2.0 - fTemp143)) / fTemp142);
			let mut fTemp145: F32 = fTemp144 + self.fVec11[1] + fTemp135;
			self.fVec12[0] = fTemp145;
			self.fRec132[(self.IOTA0 & 2047) as usize] = 0.05 * self.fRec132[((i32::wrapping_sub(self.IOTA0, 1)) & 2047) as usize] + 0.95 * self.fVec12[2];
			let mut fRec129: F32 = fTemp139 * self.fRec132[((i32::wrapping_sub(self.IOTA0, iTemp130)) & 2047) as usize] + fTemp125 * (fTemp138 * self.fRec132[((i32::wrapping_sub(self.IOTA0, iTemp123)) & 2047) as usize] + 0.5 * fTemp137 * self.fRec132[((i32::wrapping_sub(self.IOTA0, iTemp118)) & 2047) as usize] + 0.16666667 * fTemp136 * self.fRec132[((i32::wrapping_sub(self.IOTA0, iTemp114)) & 2047) as usize] + 0.041666668 * fTemp111 * self.fRec132[((i32::wrapping_sub(self.IOTA0, iTemp104)) & 2047) as usize]);
			let mut fRec130: F32 = self.fVec12[1] + self.fRec123[1];
			self.fRec123[0] = fRec128;
			let mut fRec124: F32 = self.fRec123[1];
			let mut fRec125: F32 = fRec129;
			let mut fRec126: F32 = fRec130;
			self.fRec119[0] = fRec124;
			let mut fRec120: F32 = fTemp135 + fTemp144 + self.fRec119[1];
			let mut fRec121: F32 = fRec125;
			let mut fRec122: F32 = fRec126;
			self.fRec115[(self.IOTA0 & 2047) as usize] = fRec120;
			let mut fRec116: F32 = fTemp139 * self.fRec115[((i32::wrapping_sub(self.IOTA0, iTemp131)) & 2047) as usize] + fTemp125 * (fTemp138 * self.fRec115[((i32::wrapping_sub(self.IOTA0, iTemp124)) & 2047) as usize] + 0.5 * fTemp137 * self.fRec115[((i32::wrapping_sub(self.IOTA0, iTemp119)) & 2047) as usize] + 0.16666667 * fTemp136 * self.fRec115[((i32::wrapping_sub(self.IOTA0, iTemp115)) & 2047) as usize] + 0.041666668 * fTemp111 * self.fRec115[((i32::wrapping_sub(self.IOTA0, iTemp105)) & 2047) as usize]);
			self.fRec117[0] = fRec121;
			let mut fRec118: F32 = fRec122;
			self.fRec113[0] = fSlow4 * self.fRec117[1];
			let mut fRec114: F32 = fRec118;
			self.fRec108[0] = fRec112;
			let mut fRec109: F32 = fSlow4 * self.fRec108[1];
			let mut fRec110: F32 = self.fRec113[0];
			let mut fRec111: F32 = fRec114;
			self.fRec104[(self.IOTA0 & 2047) as usize] = fRec109;
			let mut fRec105: F32 = fRec116;
			let mut fRec106: F32 = fRec110;
			let mut fRec107: F32 = fRec111;
			self.fRec102[0] = fRec105;
			let mut fRec103: F32 = fRec107;
			let mut fTemp146: F32 = F32::abs(fRec103);
			let mut fTemp147: F32 = if (((self.fRec100[1] > fTemp146) as i32) as i32 != 0) { self.fConst12 } else { 0.0 };
			self.fRec101[0] = self.fRec101[1] * fTemp147 + fTemp146 * (1.0 - fTemp147);
			self.fRec100[0] = self.fRec101[0];
			let mut fRec99: F32 = 0.0 - 0.95 * F32::max(2e+01 * F32::log10(self.fRec100[0]) + 1e+01, 0.0);
			self.fRec98[0] = fRec103 * F32::powf(1e+01, 0.05 * fRec99);
			self.fRec95[0] = 0.0 - (self.fRec95[1] * (1.0 - fTemp100) - (self.fRec98[0] + self.fRec98[1])) / (fTemp100 + 1.0);
			self.fVec13[0] = fSlow10;
			self.fRec144[0] = if (iSlow11 as i32 != 0) { fSlow12 } else { self.fRec144[1] };
			self.fRec143[0] = self.fConst2 * self.fRec143[1] + self.fConst1 * self.fRec144[0];
			let mut fTemp148: F32 = F32::powf(2.0, 0.083333336 * (self.fRec3[0] + self.fRec143[0] + -69.0));
			let mut fTemp149: F32 = 1.0 / F32::tan(self.fConst3 * fTemp148);
			let mut fRec159: F32 = -1.0 * 0.9973053 * (0.9 * self.fRec160[2] + 0.05 * (self.fRec160[1] + self.fRec160[3]));
			let mut fTemp150: F32 = self.fConst5 * (0.77272725 / fTemp148 + -0.11);
			let mut fTemp151: F32 = fTemp150 + -1.499995;
			let mut iTemp152: i32 = ((fTemp151) as i32);
			let mut iTemp153: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp152, 4))) as F32))) as i32);
			let mut iTemp154: i32 = i32::wrapping_add(iTemp153, 1);
			let mut fTemp155: F32 = F32::floor(fTemp151);
			let mut fTemp156: F32 = fTemp150 + (-3.0 - fTemp155);
			let mut fTemp157: F32 = fTemp150 + (-2.0 - fTemp155);
			let mut fTemp158: F32 = fTemp150 + (-1.0 - fTemp155);
			let mut fTemp159: F32 = fTemp158 * fTemp157;
			let mut fTemp160: F32 = fTemp159 * fTemp156;
			let mut fTemp161: F32 = fTemp150 + (-4.0 - fTemp155);
			let mut fTemp162: F32 = 0.0 - fTemp161;
			let mut iTemp163: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp152, 3))) as F32))) as i32);
			let mut iTemp164: i32 = i32::wrapping_add(iTemp163, 1);
			let mut fTemp165: F32 = 0.0 - 0.5 * fTemp161;
			let mut fTemp166: F32 = 0.0 - fTemp156;
			let mut iTemp167: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp152, 2))) as F32))) as i32);
			let mut iTemp168: i32 = i32::wrapping_add(iTemp167, 1);
			let mut fTemp169: F32 = 0.0 - 0.33333334 * fTemp161;
			let mut fTemp170: F32 = 0.0 - 0.5 * fTemp156;
			let mut fTemp171: F32 = 0.0 - fTemp157;
			let mut iTemp172: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp152, 1))) as F32))) as i32);
			let mut iTemp173: i32 = i32::wrapping_add(iTemp172, 1);
			let mut fTemp174: F32 = fTemp150 - fTemp155;
			let mut fTemp175: F32 = 0.0 - 0.25 * fTemp161;
			let mut fTemp176: F32 = 0.0 - 0.33333334 * fTemp156;
			let mut fTemp177: F32 = 0.0 - 0.5 * fTemp157;
			let mut fTemp178: F32 = 0.0 - fTemp158;
			let mut iTemp179: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, iTemp152)) as F32))) as i32);
			let mut iTemp180: i32 = i32::wrapping_add(iTemp179, 1);
			self.fRec174[0] = self.fRec151[((i32::wrapping_sub(self.IOTA0, iTemp180)) & 2047) as usize] * fTemp178 * fTemp177 * fTemp176 * fTemp175 + fTemp174 * (self.fRec151[((i32::wrapping_sub(self.IOTA0, iTemp173)) & 2047) as usize] * fTemp171 * fTemp170 * fTemp169 + 0.5 * fTemp158 * self.fRec151[((i32::wrapping_sub(self.IOTA0, iTemp168)) & 2047) as usize] * fTemp166 * fTemp165 + 0.16666667 * fTemp159 * self.fRec151[((i32::wrapping_sub(self.IOTA0, iTemp164)) & 2047) as usize] * fTemp162 + 0.041666668 * fTemp160 * self.fRec151[((i32::wrapping_sub(self.IOTA0, iTemp154)) & 2047) as usize]);
			self.fRec178[0] = 0.05 * self.fRec178[1] + 0.95 * self.fRec174[1];
			let mut fRec175: F32 = self.fRec178[0];
			self.fRec183[0] = self.fConst6 * self.fRec183[1] + self.fConst7 * F32::abs(self.fRec145[1]);
			let mut fRec182: F32 = self.fRec183[0];
			let mut iTemp181: i32 = ((fRec182 > 0.1) as i32);
			self.iVec14[0] = iTemp181;
			self.iRec184[0] = std::cmp::max(i32::wrapping_mul(self.iConst8, ((iTemp181 < self.iVec14[1]) as i32)), i32::wrapping_add(self.iRec184[1], -1));
			let mut fTemp182: F32 = F32::abs(F32::max(((iTemp181) as F32), ((((self.iRec184[0] > 0) as i32)) as F32)));
			let mut fTemp183: F32 = if (((self.fRec180[1] > fTemp182) as i32) as i32 != 0) { self.fConst9 } else { self.fConst6 };
			self.fRec181[0] = self.fRec181[1] * fTemp183 + fTemp182 * (1.0 - fTemp183);
			self.fRec180[0] = self.fRec181[0];
			let mut fTemp184: F32 = 0.005 * self.fRec180[0] * self.fRec145[1];
			self.fRec185[0] = self.fRec149[1];
			self.fRec186[(self.IOTA0 & 2047) as usize] = -1.0 * 0.9973053 * (0.9 * self.fRec185[2] + 0.05 * (self.fRec185[1] + self.fRec185[3]));
			let mut fTemp185: F32 = fTemp159 * fTemp162;
			let mut fTemp186: F32 = fTemp158 * fTemp166 * fTemp165;
			let mut fTemp187: F32 = fTemp171 * fTemp170 * fTemp169;
			let mut fTemp188: F32 = fTemp178 * fTemp177 * fTemp176 * fTemp175;
			self.fVec15[0] = fTemp188 * self.fRec186[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp179, 2))) & 2047) as usize] + fTemp174 * (fTemp187 * self.fRec186[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp172, 2))) & 2047) as usize] + 0.5 * fTemp186 * self.fRec186[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp167, 2))) & 2047) as usize] + 0.16666667 * fTemp185 * self.fRec186[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp163, 2))) & 2047) as usize] + 0.041666668 * fTemp160 * self.fRec186[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp153, 2))) & 2047) as usize]);
			let mut fTemp189: F32 = F32::tan(self.fConst10 * fTemp148);
			let mut fTemp190: F32 = 1.0 / fTemp189;
			let mut fTemp191: F32 = (fTemp190 + 1.4142135) / fTemp189 + 1.0;
			self.iRec187[0] = i32::wrapping_add(i32::wrapping_mul(i32::wrapping_add(self.iRec187[1], ((self.iRec187[1] > 0) as i32)), ((fSlow10 <= self.fVec13[1]) as i32)), ((fSlow10 > self.fVec13[1]) as i32));
			let mut fTemp192: F32 = ((self.iRec187[0]) as F32) / F32::max(1.0, self.fConst11 * mydsp_faustpower2_f(1.0 - 0.22 * fTemp148));
			self.fRec188[0] = fTemp45 - (self.fRec188[2] * ((fTemp190 + -1.4142135) / fTemp189 + 1.0) + 2.0 * self.fRec188[1] * (1.0 - 1.0 / mydsp_faustpower2_f(fTemp189))) / fTemp191;
			let mut fTemp193: F32 = 0.5 * ((self.fRec188[2] + self.fRec188[0] + 2.0 * self.fRec188[1]) * F32::max(0.0, F32::min(fTemp192, 2.0 - fTemp192)) / fTemp191);
			let mut fTemp194: F32 = fTemp193 + self.fVec15[1] + fTemp184;
			self.fVec16[0] = fTemp194;
			self.fRec179[(self.IOTA0 & 2047) as usize] = 0.05 * self.fRec179[((i32::wrapping_sub(self.IOTA0, 1)) & 2047) as usize] + 0.95 * self.fVec16[2];
			let mut fRec176: F32 = fTemp188 * self.fRec179[((i32::wrapping_sub(self.IOTA0, iTemp179)) & 2047) as usize] + fTemp174 * (fTemp187 * self.fRec179[((i32::wrapping_sub(self.IOTA0, iTemp172)) & 2047) as usize] + 0.5 * fTemp186 * self.fRec179[((i32::wrapping_sub(self.IOTA0, iTemp167)) & 2047) as usize] + 0.16666667 * fTemp185 * self.fRec179[((i32::wrapping_sub(self.IOTA0, iTemp163)) & 2047) as usize] + 0.041666668 * fTemp160 * self.fRec179[((i32::wrapping_sub(self.IOTA0, iTemp153)) & 2047) as usize]);
			let mut fRec177: F32 = self.fVec16[1] + self.fRec170[1];
			self.fRec170[0] = fRec175;
			let mut fRec171: F32 = self.fRec170[1];
			let mut fRec172: F32 = fRec176;
			let mut fRec173: F32 = fRec177;
			self.fRec166[0] = fRec171;
			let mut fRec167: F32 = fTemp184 + fTemp193 + self.fRec166[1];
			let mut fRec168: F32 = fRec172;
			let mut fRec169: F32 = fRec173;
			self.fRec162[(self.IOTA0 & 2047) as usize] = fRec167;
			let mut fRec163: F32 = fTemp188 * self.fRec162[((i32::wrapping_sub(self.IOTA0, iTemp180)) & 2047) as usize] + fTemp174 * (fTemp187 * self.fRec162[((i32::wrapping_sub(self.IOTA0, iTemp173)) & 2047) as usize] + 0.5 * fTemp186 * self.fRec162[((i32::wrapping_sub(self.IOTA0, iTemp168)) & 2047) as usize] + 0.16666667 * fTemp185 * self.fRec162[((i32::wrapping_sub(self.IOTA0, iTemp164)) & 2047) as usize] + 0.041666668 * fTemp160 * self.fRec162[((i32::wrapping_sub(self.IOTA0, iTemp154)) & 2047) as usize]);
			self.fRec164[0] = fRec168;
			let mut fRec165: F32 = fRec169;
			self.fRec160[0] = fSlow4 * self.fRec164[1];
			let mut fRec161: F32 = fRec165;
			self.fRec155[0] = fRec159;
			let mut fRec156: F32 = fSlow4 * self.fRec155[1];
			let mut fRec157: F32 = self.fRec160[0];
			let mut fRec158: F32 = fRec161;
			self.fRec151[(self.IOTA0 & 2047) as usize] = fRec156;
			let mut fRec152: F32 = fRec163;
			let mut fRec153: F32 = fRec157;
			let mut fRec154: F32 = fRec158;
			self.fRec149[0] = fRec152;
			let mut fRec150: F32 = fRec154;
			let mut fTemp195: F32 = F32::abs(fRec150);
			let mut fTemp196: F32 = if (((self.fRec147[1] > fTemp195) as i32) as i32 != 0) { self.fConst12 } else { 0.0 };
			self.fRec148[0] = self.fRec148[1] * fTemp196 + fTemp195 * (1.0 - fTemp196);
			self.fRec147[0] = self.fRec148[0];
			let mut fRec146: F32 = 0.0 - 0.95 * F32::max(2e+01 * F32::log10(self.fRec147[0]) + 1e+01, 0.0);
			self.fRec145[0] = fRec150 * F32::powf(1e+01, 0.05 * fRec146);
			self.fRec142[0] = 0.0 - (self.fRec142[1] * (1.0 - fTemp149) - (self.fRec145[0] + self.fRec145[1])) / (fTemp149 + 1.0);
			self.fVec17[0] = fSlow13;
			self.fRec191[0] = if (iSlow14 as i32 != 0) { fSlow15 } else { self.fRec191[1] };
			self.fRec190[0] = self.fConst2 * self.fRec190[1] + self.fConst1 * self.fRec191[0];
			let mut fTemp197: F32 = F32::powf(2.0, 0.083333336 * (self.fRec3[0] + self.fRec190[0] + -69.0));
			let mut fTemp198: F32 = 1.0 / F32::tan(self.fConst3 * fTemp197);
			let mut fRec206: F32 = -1.0 * 0.9973053 * (0.9 * self.fRec207[2] + 0.05 * (self.fRec207[1] + self.fRec207[3]));
			let mut fTemp199: F32 = self.fConst5 * (0.77272725 / fTemp197 + -0.11);
			let mut fTemp200: F32 = fTemp199 + -1.499995;
			let mut iTemp201: i32 = ((fTemp200) as i32);
			let mut iTemp202: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp201, 4))) as F32))) as i32);
			let mut iTemp203: i32 = i32::wrapping_add(iTemp202, 1);
			let mut fTemp204: F32 = F32::floor(fTemp200);
			let mut fTemp205: F32 = fTemp199 + (-3.0 - fTemp204);
			let mut fTemp206: F32 = fTemp199 + (-2.0 - fTemp204);
			let mut fTemp207: F32 = fTemp199 + (-1.0 - fTemp204);
			let mut fTemp208: F32 = fTemp207 * fTemp206;
			let mut fTemp209: F32 = fTemp208 * fTemp205;
			let mut fTemp210: F32 = fTemp199 + (-4.0 - fTemp204);
			let mut fTemp211: F32 = 0.0 - fTemp210;
			let mut iTemp212: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp201, 3))) as F32))) as i32);
			let mut iTemp213: i32 = i32::wrapping_add(iTemp212, 1);
			let mut fTemp214: F32 = 0.0 - 0.5 * fTemp210;
			let mut fTemp215: F32 = 0.0 - fTemp205;
			let mut iTemp216: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp201, 2))) as F32))) as i32);
			let mut iTemp217: i32 = i32::wrapping_add(iTemp216, 1);
			let mut fTemp218: F32 = 0.0 - 0.33333334 * fTemp210;
			let mut fTemp219: F32 = 0.0 - 0.5 * fTemp205;
			let mut fTemp220: F32 = 0.0 - fTemp206;
			let mut iTemp221: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp201, 1))) as F32))) as i32);
			let mut iTemp222: i32 = i32::wrapping_add(iTemp221, 1);
			let mut fTemp223: F32 = fTemp199 - fTemp204;
			let mut fTemp224: F32 = 0.0 - 0.25 * fTemp210;
			let mut fTemp225: F32 = 0.0 - 0.33333334 * fTemp205;
			let mut fTemp226: F32 = 0.0 - 0.5 * fTemp206;
			let mut fTemp227: F32 = 0.0 - fTemp207;
			let mut iTemp228: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, iTemp201)) as F32))) as i32);
			let mut iTemp229: i32 = i32::wrapping_add(iTemp228, 1);
			self.fRec221[0] = self.fRec198[((i32::wrapping_sub(self.IOTA0, iTemp229)) & 2047) as usize] * fTemp227 * fTemp226 * fTemp225 * fTemp224 + fTemp223 * (self.fRec198[((i32::wrapping_sub(self.IOTA0, iTemp222)) & 2047) as usize] * fTemp220 * fTemp219 * fTemp218 + 0.5 * fTemp207 * self.fRec198[((i32::wrapping_sub(self.IOTA0, iTemp217)) & 2047) as usize] * fTemp215 * fTemp214 + 0.16666667 * fTemp208 * self.fRec198[((i32::wrapping_sub(self.IOTA0, iTemp213)) & 2047) as usize] * fTemp211 + 0.041666668 * fTemp209 * self.fRec198[((i32::wrapping_sub(self.IOTA0, iTemp203)) & 2047) as usize]);
			self.fRec225[0] = 0.05 * self.fRec225[1] + 0.95 * self.fRec221[1];
			let mut fRec222: F32 = self.fRec225[0];
			self.fRec230[0] = self.fConst6 * self.fRec230[1] + self.fConst7 * F32::abs(self.fRec192[1]);
			let mut fRec229: F32 = self.fRec230[0];
			let mut iTemp230: i32 = ((fRec229 > 0.1) as i32);
			self.iVec18[0] = iTemp230;
			self.iRec231[0] = std::cmp::max(i32::wrapping_mul(self.iConst8, ((iTemp230 < self.iVec18[1]) as i32)), i32::wrapping_add(self.iRec231[1], -1));
			let mut fTemp231: F32 = F32::abs(F32::max(((iTemp230) as F32), ((((self.iRec231[0] > 0) as i32)) as F32)));
			let mut fTemp232: F32 = if (((self.fRec227[1] > fTemp231) as i32) as i32 != 0) { self.fConst9 } else { self.fConst6 };
			self.fRec228[0] = self.fRec228[1] * fTemp232 + fTemp231 * (1.0 - fTemp232);
			self.fRec227[0] = self.fRec228[0];
			let mut fTemp233: F32 = 0.005 * self.fRec227[0] * self.fRec192[1];
			self.fRec232[0] = self.fRec196[1];
			self.fRec233[(self.IOTA0 & 2047) as usize] = -1.0 * 0.9973053 * (0.9 * self.fRec232[2] + 0.05 * (self.fRec232[1] + self.fRec232[3]));
			let mut fTemp234: F32 = fTemp208 * fTemp211;
			let mut fTemp235: F32 = fTemp207 * fTemp215 * fTemp214;
			let mut fTemp236: F32 = fTemp220 * fTemp219 * fTemp218;
			let mut fTemp237: F32 = fTemp227 * fTemp226 * fTemp225 * fTemp224;
			self.fVec19[0] = fTemp237 * self.fRec233[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp228, 2))) & 2047) as usize] + fTemp223 * (fTemp236 * self.fRec233[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp221, 2))) & 2047) as usize] + 0.5 * fTemp235 * self.fRec233[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp216, 2))) & 2047) as usize] + 0.16666667 * fTemp234 * self.fRec233[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp212, 2))) & 2047) as usize] + 0.041666668 * fTemp209 * self.fRec233[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp202, 2))) & 2047) as usize]);
			let mut fTemp238: F32 = F32::tan(self.fConst10 * fTemp197);
			let mut fTemp239: F32 = 1.0 / fTemp238;
			let mut fTemp240: F32 = (fTemp239 + 1.4142135) / fTemp238 + 1.0;
			self.iRec234[0] = i32::wrapping_add(i32::wrapping_mul(i32::wrapping_add(self.iRec234[1], ((self.iRec234[1] > 0) as i32)), ((fSlow13 <= self.fVec17[1]) as i32)), ((fSlow13 > self.fVec17[1]) as i32));
			let mut fTemp241: F32 = ((self.iRec234[0]) as F32) / F32::max(1.0, self.fConst11 * mydsp_faustpower2_f(1.0 - 0.22 * fTemp197));
			self.fRec235[0] = fTemp45 - (self.fRec235[2] * ((fTemp239 + -1.4142135) / fTemp238 + 1.0) + 2.0 * self.fRec235[1] * (1.0 - 1.0 / mydsp_faustpower2_f(fTemp238))) / fTemp240;
			let mut fTemp242: F32 = 0.5 * ((self.fRec235[2] + self.fRec235[0] + 2.0 * self.fRec235[1]) * F32::max(0.0, F32::min(fTemp241, 2.0 - fTemp241)) / fTemp240);
			let mut fTemp243: F32 = fTemp242 + self.fVec19[1] + fTemp233;
			self.fVec20[0] = fTemp243;
			self.fRec226[(self.IOTA0 & 2047) as usize] = 0.05 * self.fRec226[((i32::wrapping_sub(self.IOTA0, 1)) & 2047) as usize] + 0.95 * self.fVec20[2];
			let mut fRec223: F32 = fTemp237 * self.fRec226[((i32::wrapping_sub(self.IOTA0, iTemp228)) & 2047) as usize] + fTemp223 * (fTemp236 * self.fRec226[((i32::wrapping_sub(self.IOTA0, iTemp221)) & 2047) as usize] + 0.5 * fTemp235 * self.fRec226[((i32::wrapping_sub(self.IOTA0, iTemp216)) & 2047) as usize] + 0.16666667 * fTemp234 * self.fRec226[((i32::wrapping_sub(self.IOTA0, iTemp212)) & 2047) as usize] + 0.041666668 * fTemp209 * self.fRec226[((i32::wrapping_sub(self.IOTA0, iTemp202)) & 2047) as usize]);
			let mut fRec224: F32 = self.fVec20[1] + self.fRec217[1];
			self.fRec217[0] = fRec222;
			let mut fRec218: F32 = self.fRec217[1];
			let mut fRec219: F32 = fRec223;
			let mut fRec220: F32 = fRec224;
			self.fRec213[0] = fRec218;
			let mut fRec214: F32 = fTemp233 + fTemp242 + self.fRec213[1];
			let mut fRec215: F32 = fRec219;
			let mut fRec216: F32 = fRec220;
			self.fRec209[(self.IOTA0 & 2047) as usize] = fRec214;
			let mut fRec210: F32 = fTemp237 * self.fRec209[((i32::wrapping_sub(self.IOTA0, iTemp229)) & 2047) as usize] + fTemp223 * (fTemp236 * self.fRec209[((i32::wrapping_sub(self.IOTA0, iTemp222)) & 2047) as usize] + 0.5 * fTemp235 * self.fRec209[((i32::wrapping_sub(self.IOTA0, iTemp217)) & 2047) as usize] + 0.16666667 * fTemp234 * self.fRec209[((i32::wrapping_sub(self.IOTA0, iTemp213)) & 2047) as usize] + 0.041666668 * fTemp209 * self.fRec209[((i32::wrapping_sub(self.IOTA0, iTemp203)) & 2047) as usize]);
			self.fRec211[0] = fRec215;
			let mut fRec212: F32 = fRec216;
			self.fRec207[0] = fSlow4 * self.fRec211[1];
			let mut fRec208: F32 = fRec212;
			self.fRec202[0] = fRec206;
			let mut fRec203: F32 = fSlow4 * self.fRec202[1];
			let mut fRec204: F32 = self.fRec207[0];
			let mut fRec205: F32 = fRec208;
			self.fRec198[(self.IOTA0 & 2047) as usize] = fRec203;
			let mut fRec199: F32 = fRec210;
			let mut fRec200: F32 = fRec204;
			let mut fRec201: F32 = fRec205;
			self.fRec196[0] = fRec199;
			let mut fRec197: F32 = fRec201;
			let mut fTemp244: F32 = F32::abs(fRec197);
			let mut fTemp245: F32 = if (((self.fRec194[1] > fTemp244) as i32) as i32 != 0) { self.fConst12 } else { 0.0 };
			self.fRec195[0] = self.fRec195[1] * fTemp245 + fTemp244 * (1.0 - fTemp245);
			self.fRec194[0] = self.fRec195[0];
			let mut fRec193: F32 = 0.0 - 0.95 * F32::max(2e+01 * F32::log10(self.fRec194[0]) + 1e+01, 0.0);
			self.fRec192[0] = fRec197 * F32::powf(1e+01, 0.05 * fRec193);
			self.fRec189[0] = 0.0 - (self.fRec189[1] * (1.0 - fTemp198) - (self.fRec192[0] + self.fRec192[1])) / (fTemp198 + 1.0);
			self.fRec236[0] = fSlow16 + self.fConst2 * self.fRec236[1];
			let mut fTemp246: F32 = self.fRec236[0] * (self.fRec189[0] + self.fRec142[0] + self.fRec95[0] + self.fRec49[0] + self.fRec0[0]);
			self.fRec237[0] = fSlow17 + self.fConst2 * self.fRec237[1];
			let mut fTemp247: F32 = F32::min(1.4141995, 1.4142135 * self.fRec237[0]);
			let mut fTemp248: F32 = 1.4142135 * fTemp247;
			let mut fTemp249: F32 = 1.0 - fTemp248;
			self.fRec239[0] = fSlow18 + self.fConst2 * self.fRec239[1];
			let mut fTemp250: F32 = self.fRec239[0] + -69.0;
			self.fRec238[0] = self.fConst2 * self.fRec238[1] + self.fConst13 * F32::powf(2.0, 0.083333336 * (fSlow19 + fTemp250));
			let mut fTemp251: F32 = F32::tan(self.fConst14 * F32::max(2e+01, F32::min(1e+04, self.fRec238[0])));
			let mut fTemp252: F32 = 1.0 / fTemp251;
			let mut fTemp253: F32 = 2.0 - fTemp248;
			let mut fTemp254: F32 = mydsp_faustpower2_f(fTemp247);
			let mut fTemp255: F32 = fTemp254 + (fTemp253 + fTemp252) / fTemp251 + fTemp249;
			let mut fTemp256: F32 = 1.0 / mydsp_faustpower2_f(fTemp251);
			let mut fTemp257: F32 = fTemp248 + 2.0;
			let mut fTemp258: F32 = fTemp248 + fTemp254;
			let mut fTemp259: F32 = fTemp258 + (fTemp257 + fTemp252) / fTemp251 + 1.0;
			self.fRec244[0] = fSlow20 + self.fConst2 * self.fRec244[1];
			let mut fTemp260: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec244[0]));
			let mut fTemp261: F32 = self.fRec242[1] + self.fConst15 * fTemp260;
			let mut fTemp262: F32 = fTemp261 + -1.0;
			let mut iTemp263: i32 = ((fTemp262 < 0.0) as i32);
			self.fRec242[0] = if (iTemp263 as i32 != 0) { fTemp261 } else { fTemp262 };
			let mut fThen15: F32 = fTemp261 + (1.0 - self.fConst0 / fTemp260) * fTemp262;
			let mut fRec243: F32 = if (iTemp263 as i32 != 0) { fTemp261 } else { fThen15 };
			self.fRec245[0] = fSlow21 + self.fConst2 * self.fRec245[1];
			self.fRec241[0] = self.fRec245[0] * (2.0 * fRec243 + -1.0) - (self.fRec241[2] * (fTemp258 + (fTemp252 - fTemp257) / fTemp251 + 1.0) + 2.0 * self.fRec241[1] * (fTemp258 + (1.0 - fTemp256))) / fTemp259;
			self.fRec240[0] = (self.fRec241[2] + self.fRec241[0] + 2.0 * self.fRec241[1]) / fTemp259 - (self.fRec240[2] * (fTemp254 + (fTemp252 - fTemp253) / fTemp251 + fTemp249) + 2.0 * self.fRec240[1] * (fTemp254 + (1.0 - (fTemp248 + fTemp256)))) / fTemp255;
			self.fRec246[0] = self.fConst2 * self.fRec246[1] + self.fConst13 * F32::powf(2.0, 0.083333336 * (fSlow22 + fTemp250));
			let mut fTemp264: F32 = F32::tan(self.fConst14 * F32::max(2e+01, F32::min(1e+04, self.fRec246[0])));
			let mut fTemp265: F32 = 1.0 / fTemp264;
			let mut fTemp266: F32 = fTemp254 + (fTemp253 + fTemp265) / fTemp264 + fTemp249;
			let mut fTemp267: F32 = 1.0 / mydsp_faustpower2_f(fTemp264);
			let mut fTemp268: F32 = fTemp258 + (fTemp257 + fTemp265) / fTemp264 + 1.0;
			self.fRec251[0] = fSlow23 + self.fConst2 * self.fRec251[1];
			let mut fTemp269: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec251[0]));
			let mut fTemp270: F32 = self.fConst15 * fTemp269;
			let mut fTemp271: F32 = self.fRec249[1] + fTemp270;
			let mut fTemp272: F32 = fTemp271 + -1.0;
			let mut iTemp273: i32 = ((fTemp272 < 0.0) as i32);
			self.fRec249[0] = if (iTemp273 as i32 != 0) { fTemp271 } else { fTemp272 };
			let mut fThen17: F32 = self.fRec249[1] + fTemp270 + (1.0 - self.fConst0 / fTemp269) * fTemp272;
			let mut fRec250: F32 = if (iTemp273 as i32 != 0) { fTemp271 } else { fThen17 };
			self.fRec252[0] = fSlow24 + self.fConst2 * self.fRec252[1];
			self.fRec248[0] = self.fRec252[0] * (2.0 * fRec250 + -1.0) - (self.fRec248[2] * (fTemp258 + (fTemp265 - fTemp257) / fTemp264 + 1.0) + 2.0 * self.fRec248[1] * (fTemp258 + (1.0 - fTemp267))) / fTemp268;
			self.fRec247[0] = (self.fRec248[2] + self.fRec248[0] + 2.0 * self.fRec248[1]) / fTemp268 - (self.fRec247[2] * (fTemp254 + (fTemp265 - fTemp253) / fTemp264 + fTemp249) + 2.0 * self.fRec247[1] * (fTemp254 + (1.0 - (fTemp248 + fTemp267)))) / fTemp266;
			self.fRec253[0] = self.fConst2 * self.fRec253[1] + self.fConst13 * F32::powf(2.0, 0.083333336 * (fSlow25 + fTemp250));
			let mut fTemp274: F32 = F32::tan(self.fConst14 * F32::max(2e+01, F32::min(1e+04, self.fRec253[0])));
			let mut fTemp275: F32 = 1.0 / fTemp274;
			let mut fTemp276: F32 = fTemp254 + (fTemp253 + fTemp275) / fTemp274 + fTemp249;
			let mut fTemp277: F32 = 1.0 / mydsp_faustpower2_f(fTemp274);
			let mut fTemp278: F32 = fTemp258 + (fTemp257 + fTemp275) / fTemp274 + 1.0;
			self.fRec258[0] = fSlow26 + self.fConst2 * self.fRec258[1];
			let mut fTemp279: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec258[0]));
			let mut fTemp280: F32 = self.fConst15 * fTemp279;
			let mut fTemp281: F32 = self.fRec256[1] + fTemp280;
			let mut fTemp282: F32 = fTemp281 + -1.0;
			let mut iTemp283: i32 = ((fTemp282 < 0.0) as i32);
			self.fRec256[0] = if (iTemp283 as i32 != 0) { fTemp281 } else { fTemp282 };
			let mut fThen19: F32 = self.fRec256[1] + fTemp280 + (1.0 - self.fConst0 / fTemp279) * fTemp282;
			let mut fRec257: F32 = if (iTemp283 as i32 != 0) { fTemp281 } else { fThen19 };
			self.fRec259[0] = fSlow27 + self.fConst2 * self.fRec259[1];
			self.fRec255[0] = self.fRec259[0] * (2.0 * fRec257 + -1.0) - (self.fRec255[2] * (fTemp258 + (fTemp275 - fTemp257) / fTemp274 + 1.0) + 2.0 * self.fRec255[1] * (fTemp258 + (1.0 - fTemp277))) / fTemp278;
			self.fRec254[0] = (self.fRec255[2] + self.fRec255[0] + 2.0 * self.fRec255[1]) / fTemp278 - (self.fRec254[2] * (fTemp254 + (fTemp275 - fTemp253) / fTemp274 + fTemp249) + 2.0 * self.fRec254[1] * (fTemp254 + (1.0 - (fTemp248 + fTemp277)))) / fTemp276;
			self.fRec260[0] = self.fConst2 * self.fRec260[1] + self.fConst13 * F32::powf(2.0, 0.083333336 * (fSlow28 + fTemp250));
			let mut fTemp284: F32 = F32::tan(self.fConst14 * F32::max(2e+01, F32::min(1e+04, self.fRec260[0])));
			let mut fTemp285: F32 = 1.0 / fTemp284;
			let mut fTemp286: F32 = fTemp254 + (fTemp253 + fTemp285) / fTemp284 + fTemp249;
			let mut fTemp287: F32 = 1.0 / mydsp_faustpower2_f(fTemp284);
			let mut fTemp288: F32 = fTemp258 + (fTemp285 + fTemp257) / fTemp284 + 1.0;
			self.fRec265[0] = fSlow29 + self.fConst2 * self.fRec265[1];
			let mut fTemp289: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec265[0]));
			let mut fTemp290: F32 = self.fRec263[1] + self.fConst15 * fTemp289;
			let mut fTemp291: F32 = fTemp290 + -1.0;
			let mut iTemp292: i32 = ((fTemp291 < 0.0) as i32);
			self.fRec263[0] = if (iTemp292 as i32 != 0) { fTemp290 } else { fTemp291 };
			let mut fThen21: F32 = fTemp290 + (1.0 - self.fConst0 / fTemp289) * fTemp291;
			let mut fRec264: F32 = if (iTemp292 as i32 != 0) { fTemp290 } else { fThen21 };
			self.fRec266[0] = fSlow30 + self.fConst2 * self.fRec266[1];
			self.fRec262[0] = self.fRec266[0] * (2.0 * fRec264 + -1.0) - (self.fRec262[2] * (fTemp258 + (1.0 - (fTemp257 - fTemp285) / fTemp284)) + 2.0 * self.fRec262[1] * (fTemp258 + (1.0 - fTemp287))) / fTemp288;
			self.fRec261[0] = (self.fRec262[2] + self.fRec262[0] + 2.0 * self.fRec262[1]) / fTemp288 - (self.fRec261[2] * (fTemp254 + (fTemp285 - fTemp253) / fTemp284 + fTemp249) + 2.0 * self.fRec261[1] * (fTemp254 + (1.0 - (fTemp248 + fTemp287)))) / fTemp286;
			self.fRec267[0] = fSlow31 + self.fConst2 * self.fRec267[1];
			self.fRec268[0] = fSlow32 + self.fConst2 * self.fRec268[1];
			let mut fTemp293: F32 = self.fRec268[0] * self.fRec267[0] * ((self.fRec261[2] + self.fRec261[0] + 2.0 * self.fRec261[1]) / fTemp286 + (self.fRec254[2] + self.fRec254[0] + 2.0 * self.fRec254[1]) / fTemp276 + (self.fRec247[2] + self.fRec247[0] + 2.0 * self.fRec247[1]) / fTemp266 + (self.fRec240[2] + self.fRec240[0] + 2.0 * self.fRec240[1]) / fTemp255);
			self.fRec269[0] = fSlow33 + self.fConst2 * self.fRec269[1];
			let mut fTemp294: F32 = F32::powf(2.0, 0.083333336 * (self.fRec269[0] + -61.88));
			let mut fTemp295: F32 = F32::max(4.4e+02 * fTemp294, 23.44895);
			let mut fTemp296: F32 = F32::max(2e+01, F32::abs(fTemp295));
			let mut fTemp297: F32 = self.fRec271[1] + self.fConst15 * fTemp296;
			self.fRec271[0] = fTemp297 - F32::floor(fTemp297);
			let mut fTemp298: F32 = mydsp_faustpower2_f(2.0 * self.fRec271[0] + -1.0);
			self.fVec21[0] = fTemp298;
			let mut fTemp299: F32 = ((self.iVec1[1]) as F32);
			let mut fTemp300: F32 = fTemp299 * (fTemp298 - self.fVec21[1]) / fTemp296;
			self.fVec22[(self.IOTA0 & 4095) as usize] = fTemp300;
			let mut fTemp301: F32 = F32::max(0.0, F32::min(2047.0, self.fConst16 / fTemp295));
			let mut iTemp302: i32 = ((fTemp301) as i32);
			let mut fTemp303: F32 = F32::floor(fTemp301);
			self.fRec270[0] = 0.999 * self.fRec270[1] - self.fConst17 * (self.fVec22[((i32::wrapping_sub(self.IOTA0, iTemp302)) & 4095) as usize] * (fTemp303 + (1.0 - fTemp301)) - fTemp300 + (fTemp301 - fTemp303) * self.fVec22[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp302, 1))) & 4095) as usize]);
			let mut fTemp304: F32 = F32::powf(2.0, 0.083333336 * (self.fRec269[0] + -81.11));
			let mut fTemp305: F32 = F32::max(4.4e+02 * fTemp304, 23.44895);
			let mut fTemp306: F32 = F32::max(2e+01, F32::abs(fTemp305));
			let mut fTemp307: F32 = self.fRec273[1] + self.fConst15 * fTemp306;
			self.fRec273[0] = fTemp307 - F32::floor(fTemp307);
			let mut fTemp308: F32 = mydsp_faustpower2_f(2.0 * self.fRec273[0] + -1.0);
			self.fVec23[0] = fTemp308;
			let mut fTemp309: F32 = fTemp299 * (fTemp308 - self.fVec23[1]) / fTemp306;
			self.fVec24[(self.IOTA0 & 4095) as usize] = fTemp309;
			let mut fTemp310: F32 = F32::max(0.0, F32::min(2047.0, self.fConst16 / fTemp305));
			let mut iTemp311: i32 = ((fTemp310) as i32);
			let mut fTemp312: F32 = F32::floor(fTemp310);
			self.fRec272[0] = 0.999 * self.fRec272[1] - self.fConst17 * (self.fVec24[((i32::wrapping_sub(self.IOTA0, iTemp311)) & 4095) as usize] * (fTemp312 + (1.0 - fTemp310)) - fTemp309 + (fTemp310 - fTemp312) * self.fVec24[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp311, 1))) & 4095) as usize]);
			let mut fTemp313: F32 = F32::powf(2.0, 0.083333336 * (self.fRec269[0] + -56.9));
			let mut fTemp314: F32 = F32::max(4.4e+02 * fTemp313, 23.44895);
			let mut fTemp315: F32 = F32::max(2e+01, F32::abs(fTemp314));
			let mut fTemp316: F32 = self.fRec275[1] + self.fConst15 * fTemp315;
			self.fRec275[0] = fTemp316 - F32::floor(fTemp316);
			let mut fTemp317: F32 = mydsp_faustpower2_f(2.0 * self.fRec275[0] + -1.0);
			self.fVec25[0] = fTemp317;
			let mut fTemp318: F32 = fTemp299 * (fTemp317 - self.fVec25[1]) / fTemp315;
			self.fVec26[(self.IOTA0 & 4095) as usize] = fTemp318;
			let mut fTemp319: F32 = F32::max(0.0, F32::min(2047.0, self.fConst16 / fTemp314));
			let mut iTemp320: i32 = ((fTemp319) as i32);
			let mut fTemp321: F32 = F32::floor(fTemp319);
			self.fRec274[0] = 0.999 * self.fRec274[1] - self.fConst17 * (self.fVec26[((i32::wrapping_sub(self.IOTA0, iTemp320)) & 4095) as usize] * (fTemp321 + (1.0 - fTemp319)) - fTemp318 + (fTemp319 - fTemp321) * self.fVec26[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp320, 1))) & 4095) as usize]);
			let mut fTemp322: F32 = F32::powf(2.0, 0.083333336 * (self.fRec269[0] + -69.0));
			let mut fTemp323: F32 = F32::max(4.4e+02 * fTemp322, 23.44895);
			let mut fTemp324: F32 = F32::max(2e+01, F32::abs(fTemp323));
			let mut fTemp325: F32 = self.fRec277[1] + self.fConst15 * fTemp324;
			self.fRec277[0] = fTemp325 - F32::floor(fTemp325);
			let mut fTemp326: F32 = mydsp_faustpower2_f(2.0 * self.fRec277[0] + -1.0);
			self.fVec27[0] = fTemp326;
			let mut fTemp327: F32 = fTemp299 * (fTemp326 - self.fVec27[1]) / fTemp324;
			self.fVec28[(self.IOTA0 & 4095) as usize] = fTemp327;
			let mut fTemp328: F32 = F32::max(0.0, F32::min(2047.0, self.fConst16 / fTemp323));
			let mut iTemp329: i32 = ((fTemp328) as i32);
			let mut fTemp330: F32 = F32::floor(fTemp328);
			self.fRec276[0] = 0.999 * self.fRec276[1] - self.fConst17 * (self.fVec28[((i32::wrapping_sub(self.IOTA0, iTemp329)) & 4095) as usize] * (fTemp330 + (1.0 - fTemp328)) - fTemp327 + (fTemp328 - fTemp330) * self.fVec28[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp329, 1))) & 4095) as usize]);
			self.fRec278[0] = fSlow34 + self.fConst2 * self.fRec278[1];
			self.fRec279[0] = fSlow35 + self.fConst2 * self.fRec279[1];
			let mut fTemp331: F32 = self.fConst18 * self.fRec279[0] * self.fRec278[0] * (self.fRec276[0] * fTemp322 + self.fRec274[0] * fTemp313 + self.fRec272[0] * fTemp304 + self.fRec270[0] * fTemp294);
			self.fRec280[0] = fSlow36 + self.fConst2 * self.fRec280[1];
			self.fRec282[0] = fSlow37 + self.fConst2 * self.fRec282[1];
			self.fRec281[(self.IOTA0 & 2097151) as usize] = fTemp246 + fTemp293 + fSlow38 * self.fRec281[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(((F32::min(self.fConst19, F32::max(0.0, self.fConst0 * self.fRec282[0]))) as i32), 1))) & 2097151) as usize] + fTemp331;
			self.fRec283[0] = fSlow39 + self.fConst2 * self.fRec283[1];
			let mut fTemp332: F32 = self.fRec283[0] * (self.fRec281[(self.IOTA0 & 2097151) as usize] * self.fRec280[0] + (1.0 - self.fRec280[0]) * (fTemp331 + fTemp293 + fTemp246));
			*output0 = fTemp332;
			*output1 = fTemp332;
			self.fVec0[1] = self.fVec0[0];
			self.iVec1[1] = self.iVec1[0];
			self.fRec2[1] = self.fRec2[0];
			self.fRec1[1] = self.fRec1[0];
			self.fRec3[1] = self.fRec3[0];
			self.IOTA0 = i32::wrapping_add(self.IOTA0, 1);
			self.fRec33[1] = self.fRec33[0];
			self.fRec37[1] = self.fRec37[0];
			self.fRec42[1] = self.fRec42[0];
			self.iVec2[1] = self.iVec2[0];
			self.iRec43[1] = self.iRec43[0];
			self.fRec40[1] = self.fRec40[0];
			self.fRec39[1] = self.fRec39[0];
			for j0 in (1..=3).rev() {
				self.fRec44[(j0) as usize] = self.fRec44[(i32::wrapping_sub(j0, 1)) as usize];
			}
			self.fVec3[1] = self.fVec3[0];
			self.iRec46[1] = self.iRec46[0];
			self.iRec48[1] = self.iRec48[0];
			self.fRec47[2] = self.fRec47[1];
			self.fRec47[1] = self.fRec47[0];
			self.fVec4[2] = self.fVec4[1];
			self.fVec4[1] = self.fVec4[0];
			self.fRec29[1] = self.fRec29[0];
			self.fRec25[1] = self.fRec25[0];
			self.fRec23[1] = self.fRec23[0];
			for j1 in (1..=3).rev() {
				self.fRec19[(j1) as usize] = self.fRec19[(i32::wrapping_sub(j1, 1)) as usize];
			}
			self.fRec14[1] = self.fRec14[0];
			self.fRec8[1] = self.fRec8[0];
			self.fRec7[1] = self.fRec7[0];
			self.fRec6[1] = self.fRec6[0];
			self.fRec4[1] = self.fRec4[0];
			self.fRec0[1] = self.fRec0[0];
			self.fRec50[1] = self.fRec50[0];
			self.fRec80[1] = self.fRec80[0];
			self.fRec84[1] = self.fRec84[0];
			self.fRec89[1] = self.fRec89[0];
			self.iVec5[1] = self.iVec5[0];
			self.iRec90[1] = self.iRec90[0];
			self.fRec87[1] = self.fRec87[0];
			self.fRec86[1] = self.fRec86[0];
			for j2 in (1..=3).rev() {
				self.fRec91[(j2) as usize] = self.fRec91[(i32::wrapping_sub(j2, 1)) as usize];
			}
			self.fVec6[1] = self.fVec6[0];
			self.fVec7[1] = self.fVec7[0];
			self.iRec93[1] = self.iRec93[0];
			self.fRec94[2] = self.fRec94[1];
			self.fRec94[1] = self.fRec94[0];
			self.fVec8[2] = self.fVec8[1];
			self.fVec8[1] = self.fVec8[0];
			self.fRec76[1] = self.fRec76[0];
			self.fRec72[1] = self.fRec72[0];
			self.fRec70[1] = self.fRec70[0];
			for j3 in (1..=3).rev() {
				self.fRec66[(j3) as usize] = self.fRec66[(i32::wrapping_sub(j3, 1)) as usize];
			}
			self.fRec61[1] = self.fRec61[0];
			self.fRec55[1] = self.fRec55[0];
			self.fRec54[1] = self.fRec54[0];
			self.fRec53[1] = self.fRec53[0];
			self.fRec51[1] = self.fRec51[0];
			self.fRec49[1] = self.fRec49[0];
			self.fVec9[1] = self.fVec9[0];
			self.fRec97[1] = self.fRec97[0];
			self.fRec96[1] = self.fRec96[0];
			self.fRec127[1] = self.fRec127[0];
			self.fRec131[1] = self.fRec131[0];
			self.fRec136[1] = self.fRec136[0];
			self.iVec10[1] = self.iVec10[0];
			self.iRec137[1] = self.iRec137[0];
			self.fRec134[1] = self.fRec134[0];
			self.fRec133[1] = self.fRec133[0];
			for j4 in (1..=3).rev() {
				self.fRec138[(j4) as usize] = self.fRec138[(i32::wrapping_sub(j4, 1)) as usize];
			}
			self.fVec11[1] = self.fVec11[0];
			self.iRec140[1] = self.iRec140[0];
			self.fRec141[2] = self.fRec141[1];
			self.fRec141[1] = self.fRec141[0];
			self.fVec12[2] = self.fVec12[1];
			self.fVec12[1] = self.fVec12[0];
			self.fRec123[1] = self.fRec123[0];
			self.fRec119[1] = self.fRec119[0];
			self.fRec117[1] = self.fRec117[0];
			for j5 in (1..=3).rev() {
				self.fRec113[(j5) as usize] = self.fRec113[(i32::wrapping_sub(j5, 1)) as usize];
			}
			self.fRec108[1] = self.fRec108[0];
			self.fRec102[1] = self.fRec102[0];
			self.fRec101[1] = self.fRec101[0];
			self.fRec100[1] = self.fRec100[0];
			self.fRec98[1] = self.fRec98[0];
			self.fRec95[1] = self.fRec95[0];
			self.fVec13[1] = self.fVec13[0];
			self.fRec144[1] = self.fRec144[0];
			self.fRec143[1] = self.fRec143[0];
			self.fRec174[1] = self.fRec174[0];
			self.fRec178[1] = self.fRec178[0];
			self.fRec183[1] = self.fRec183[0];
			self.iVec14[1] = self.iVec14[0];
			self.iRec184[1] = self.iRec184[0];
			self.fRec181[1] = self.fRec181[0];
			self.fRec180[1] = self.fRec180[0];
			for j6 in (1..=3).rev() {
				self.fRec185[(j6) as usize] = self.fRec185[(i32::wrapping_sub(j6, 1)) as usize];
			}
			self.fVec15[1] = self.fVec15[0];
			self.iRec187[1] = self.iRec187[0];
			self.fRec188[2] = self.fRec188[1];
			self.fRec188[1] = self.fRec188[0];
			self.fVec16[2] = self.fVec16[1];
			self.fVec16[1] = self.fVec16[0];
			self.fRec170[1] = self.fRec170[0];
			self.fRec166[1] = self.fRec166[0];
			self.fRec164[1] = self.fRec164[0];
			for j7 in (1..=3).rev() {
				self.fRec160[(j7) as usize] = self.fRec160[(i32::wrapping_sub(j7, 1)) as usize];
			}
			self.fRec155[1] = self.fRec155[0];
			self.fRec149[1] = self.fRec149[0];
			self.fRec148[1] = self.fRec148[0];
			self.fRec147[1] = self.fRec147[0];
			self.fRec145[1] = self.fRec145[0];
			self.fRec142[1] = self.fRec142[0];
			self.fVec17[1] = self.fVec17[0];
			self.fRec191[1] = self.fRec191[0];
			self.fRec190[1] = self.fRec190[0];
			self.fRec221[1] = self.fRec221[0];
			self.fRec225[1] = self.fRec225[0];
			self.fRec230[1] = self.fRec230[0];
			self.iVec18[1] = self.iVec18[0];
			self.iRec231[1] = self.iRec231[0];
			self.fRec228[1] = self.fRec228[0];
			self.fRec227[1] = self.fRec227[0];
			for j8 in (1..=3).rev() {
				self.fRec232[(j8) as usize] = self.fRec232[(i32::wrapping_sub(j8, 1)) as usize];
			}
			self.fVec19[1] = self.fVec19[0];
			self.iRec234[1] = self.iRec234[0];
			self.fRec235[2] = self.fRec235[1];
			self.fRec235[1] = self.fRec235[0];
			self.fVec20[2] = self.fVec20[1];
			self.fVec20[1] = self.fVec20[0];
			self.fRec217[1] = self.fRec217[0];
			self.fRec213[1] = self.fRec213[0];
			self.fRec211[1] = self.fRec211[0];
			for j9 in (1..=3).rev() {
				self.fRec207[(j9) as usize] = self.fRec207[(i32::wrapping_sub(j9, 1)) as usize];
			}
			self.fRec202[1] = self.fRec202[0];
			self.fRec196[1] = self.fRec196[0];
			self.fRec195[1] = self.fRec195[0];
			self.fRec194[1] = self.fRec194[0];
			self.fRec192[1] = self.fRec192[0];
			self.fRec189[1] = self.fRec189[0];
			self.fRec236[1] = self.fRec236[0];
			self.fRec237[1] = self.fRec237[0];
			self.fRec239[1] = self.fRec239[0];
			self.fRec238[1] = self.fRec238[0];
			self.fRec244[1] = self.fRec244[0];
			self.fRec242[1] = self.fRec242[0];
			self.fRec245[1] = self.fRec245[0];
			self.fRec241[2] = self.fRec241[1];
			self.fRec241[1] = self.fRec241[0];
			self.fRec240[2] = self.fRec240[1];
			self.fRec240[1] = self.fRec240[0];
			self.fRec246[1] = self.fRec246[0];
			self.fRec251[1] = self.fRec251[0];
			self.fRec249[1] = self.fRec249[0];
			self.fRec252[1] = self.fRec252[0];
			self.fRec248[2] = self.fRec248[1];
			self.fRec248[1] = self.fRec248[0];
			self.fRec247[2] = self.fRec247[1];
			self.fRec247[1] = self.fRec247[0];
			self.fRec253[1] = self.fRec253[0];
			self.fRec258[1] = self.fRec258[0];
			self.fRec256[1] = self.fRec256[0];
			self.fRec259[1] = self.fRec259[0];
			self.fRec255[2] = self.fRec255[1];
			self.fRec255[1] = self.fRec255[0];
			self.fRec254[2] = self.fRec254[1];
			self.fRec254[1] = self.fRec254[0];
			self.fRec260[1] = self.fRec260[0];
			self.fRec265[1] = self.fRec265[0];
			self.fRec263[1] = self.fRec263[0];
			self.fRec266[1] = self.fRec266[0];
			self.fRec262[2] = self.fRec262[1];
			self.fRec262[1] = self.fRec262[0];
			self.fRec261[2] = self.fRec261[1];
			self.fRec261[1] = self.fRec261[0];
			self.fRec267[1] = self.fRec267[0];
			self.fRec268[1] = self.fRec268[0];
			self.fRec269[1] = self.fRec269[0];
			self.fRec271[1] = self.fRec271[0];
			self.fVec21[1] = self.fVec21[0];
			self.fRec270[1] = self.fRec270[0];
			self.fRec273[1] = self.fRec273[0];
			self.fVec23[1] = self.fVec23[0];
			self.fRec272[1] = self.fRec272[0];
			self.fRec275[1] = self.fRec275[0];
			self.fVec25[1] = self.fVec25[0];
			self.fRec274[1] = self.fRec274[0];
			self.fRec277[1] = self.fRec277[0];
			self.fVec27[1] = self.fVec27[0];
			self.fRec276[1] = self.fRec276[0];
			self.fRec278[1] = self.fRec278[0];
			self.fRec279[1] = self.fRec279[0];
			self.fRec280[1] = self.fRec280[0];
			self.fRec282[1] = self.fRec282[0];
			self.fRec283[1] = self.fRec283[0];
		}
	}

}


}
pub use dsp::mydsp as Instrument;
