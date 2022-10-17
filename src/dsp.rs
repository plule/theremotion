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
#[derive(Debug,Clone)]
pub struct mydsp {
	fSampleRate: i32,
	fConst0: F32,
	fConst1: F32,
	fHslider0: F32,
	fConst2: F32,
	iVec0: [i32;2],
	fRec1: [F32;2],
	fConst3: F32,
	fHslider1: F32,
	fRec3: [F32;2],
	fButton0: F32,
	fVec1: [F32;2],
	fHslider2: F32,
	fRec5: [F32;2],
	fRec4: [F32;2],
	fHslider3: F32,
	IOTA0: i32,
	fConst4: F32,
	fConst5: F32,
	fRec32: [F32;2],
	fRec36: [F32;2],
	iRec39: [i32;2],
	fConst6: F32,
	fRec38: [F32;3],
	iRec40: [i32;2],
	fConst7: F32,
	fRec41: [F32;4],
	fRec42: [F32;2048],
	fVec2: [F32;2],
	fConst8: F32,
	fConst9: F32,
	fRec46: [F32;2],
	iVec3: [i32;2],
	iConst10: i32,
	iRec47: [i32;2],
	fConst11: F32,
	fRec44: [F32;2],
	fRec43: [F32;2],
	fVec4: [F32;3],
	fRec37: [F32;2048],
	fRec28: [F32;2],
	fRec24: [F32;2],
	fRec20: [F32;2048],
	fRec22: [F32;2],
	fRec18: [F32;4],
	fRec13: [F32;2],
	fRec9: [F32;2048],
	fRec7: [F32;2],
	fConst12: F32,
	fRec50: [F32;2],
	fRec49: [F32;2],
	fRec6: [F32;2],
	fRec2: [F32;2],
	fButton1: F32,
	fVec5: [F32;2],
	fHslider4: F32,
	fRec53: [F32;2],
	fRec52: [F32;2],
	fRec80: [F32;2],
	fRec84: [F32;2],
	fRec86: [F32;3],
	iRec87: [i32;2],
	fRec88: [F32;4],
	fRec89: [F32;2048],
	fVec6: [F32;2],
	fRec93: [F32;2],
	iVec7: [i32;2],
	iRec94: [i32;2],
	fRec91: [F32;2],
	fRec90: [F32;2],
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
	fRec97: [F32;2],
	fRec96: [F32;2],
	fRec54: [F32;2],
	fRec51: [F32;2],
	fButton2: F32,
	fVec9: [F32;2],
	fHslider5: F32,
	fRec100: [F32;2],
	fRec99: [F32;2],
	fRec127: [F32;2],
	fRec131: [F32;2],
	fRec133: [F32;3],
	iRec134: [i32;2],
	fRec135: [F32;4],
	fRec136: [F32;2048],
	fVec10: [F32;2],
	fRec140: [F32;2],
	iVec11: [i32;2],
	iRec141: [i32;2],
	fRec138: [F32;2],
	fRec137: [F32;2],
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
	fRec144: [F32;2],
	fRec143: [F32;2],
	fRec101: [F32;2],
	fRec98: [F32;2],
	fHslider6: F32,
	fRec146: [F32;2],
	fRec173: [F32;2],
	fRec177: [F32;2],
	fRec179: [F32;3],
	fButton3: F32,
	fVec13: [F32;2],
	iRec180: [i32;2],
	fRec181: [F32;4],
	fRec182: [F32;2048],
	fVec14: [F32;2],
	fRec186: [F32;2],
	iVec15: [i32;2],
	iRec187: [i32;2],
	fRec184: [F32;2],
	fRec183: [F32;2],
	fVec16: [F32;3],
	fRec178: [F32;2048],
	fRec169: [F32;2],
	fRec165: [F32;2],
	fRec161: [F32;2048],
	fRec163: [F32;2],
	fRec159: [F32;4],
	fRec154: [F32;2],
	fRec150: [F32;2048],
	fRec148: [F32;2],
	fRec190: [F32;2],
	fRec189: [F32;2],
	fRec147: [F32;2],
	fRec145: [F32;2],
	fButton4: F32,
	fVec17: [F32;2],
	fHslider7: F32,
	fRec193: [F32;2],
	fRec192: [F32;2],
	fRec220: [F32;2],
	fRec224: [F32;2],
	fRec226: [F32;3],
	iRec227: [i32;2],
	fRec228: [F32;4],
	fRec229: [F32;2048],
	fVec18: [F32;2],
	fRec233: [F32;2],
	iVec19: [i32;2],
	iRec234: [i32;2],
	fRec231: [F32;2],
	fRec230: [F32;2],
	fVec20: [F32;3],
	fRec225: [F32;2048],
	fRec216: [F32;2],
	fRec212: [F32;2],
	fRec208: [F32;2048],
	fRec210: [F32;2],
	fRec206: [F32;4],
	fRec201: [F32;2],
	fRec197: [F32;2048],
	fRec195: [F32;2],
	fRec237: [F32;2],
	fRec236: [F32;2],
	fRec194: [F32;2],
	fRec191: [F32;2],
	fHslider8: F32,
	fRec238: [F32;2],
	fHslider9: F32,
	fRec239: [F32;2],
	fHslider10: F32,
	fRec242: [F32;2],
	fConst13: F32,
	fConst14: F32,
	fHslider11: F32,
	fRec245: [F32;2],
	fRec243: [F32;2],
	fHslider12: F32,
	fRec246: [F32;2],
	fConst15: F32,
	fHslider13: F32,
	fRec248: [F32;2],
	fRec247: [F32;2],
	fRec241: [F32;3],
	fRec240: [F32;3],
	fHslider14: F32,
	fRec251: [F32;2],
	fHslider15: F32,
	fRec254: [F32;2],
	fRec252: [F32;2],
	fRec255: [F32;2],
	fRec250: [F32;3],
	fRec249: [F32;3],
	fHslider16: F32,
	fRec258: [F32;2],
	fHslider17: F32,
	fRec261: [F32;2],
	fRec259: [F32;2],
	fRec262: [F32;2],
	fRec257: [F32;3],
	fRec256: [F32;3],
	fHslider18: F32,
	fRec265: [F32;2],
	fHslider19: F32,
	fRec268: [F32;2],
	fRec266: [F32;2],
	fRec269: [F32;2],
	fRec264: [F32;3],
	fRec263: [F32;3],
	iConst16: i32,
	fConst17: F32,
	fHslider20: F32,
	fRec270: [F32;2],
	fHslider21: F32,
	fRec271: [F32;2],
	fConst18: F32,
	fHslider22: F32,
	fRec274: [F32;2],
	fRec273: [F32;2],
	fVec21: [F32;2],
	fVec22: [F32;4096],
	fConst19: F32,
	fRec272: [F32;2],
	fRec276: [F32;2],
	fVec23: [F32;2],
	fVec24: [F32;4096],
	fRec275: [F32;2],
	fRec278: [F32;2],
	fVec25: [F32;2],
	fVec26: [F32;4096],
	fRec277: [F32;2],
	fRec280: [F32;2],
	fVec27: [F32;2],
	fVec28: [F32;4096],
	fRec279: [F32;2],
	fRec0: [F32;65536],
	fHslider23: F32,
	fRec281: [F32;2],
}

impl FaustDsp for mydsp {
	type T = F32;
		
	fn new() -> mydsp { 
		mydsp {
			fSampleRate: 0,
			fConst0: 0.0,
			fConst1: 0.0,
			fHslider0: 0.0,
			fConst2: 0.0,
			iVec0: [0;2],
			fRec1: [0.0;2],
			fConst3: 0.0,
			fHslider1: 0.0,
			fRec3: [0.0;2],
			fButton0: 0.0,
			fVec1: [0.0;2],
			fHslider2: 0.0,
			fRec5: [0.0;2],
			fRec4: [0.0;2],
			fHslider3: 0.0,
			IOTA0: 0,
			fConst4: 0.0,
			fConst5: 0.0,
			fRec32: [0.0;2],
			fRec36: [0.0;2],
			iRec39: [0;2],
			fConst6: 0.0,
			fRec38: [0.0;3],
			iRec40: [0;2],
			fConst7: 0.0,
			fRec41: [0.0;4],
			fRec42: [0.0;2048],
			fVec2: [0.0;2],
			fConst8: 0.0,
			fConst9: 0.0,
			fRec46: [0.0;2],
			iVec3: [0;2],
			iConst10: 0,
			iRec47: [0;2],
			fConst11: 0.0,
			fRec44: [0.0;2],
			fRec43: [0.0;2],
			fVec4: [0.0;3],
			fRec37: [0.0;2048],
			fRec28: [0.0;2],
			fRec24: [0.0;2],
			fRec20: [0.0;2048],
			fRec22: [0.0;2],
			fRec18: [0.0;4],
			fRec13: [0.0;2],
			fRec9: [0.0;2048],
			fRec7: [0.0;2],
			fConst12: 0.0,
			fRec50: [0.0;2],
			fRec49: [0.0;2],
			fRec6: [0.0;2],
			fRec2: [0.0;2],
			fButton1: 0.0,
			fVec5: [0.0;2],
			fHslider4: 0.0,
			fRec53: [0.0;2],
			fRec52: [0.0;2],
			fRec80: [0.0;2],
			fRec84: [0.0;2],
			fRec86: [0.0;3],
			iRec87: [0;2],
			fRec88: [0.0;4],
			fRec89: [0.0;2048],
			fVec6: [0.0;2],
			fRec93: [0.0;2],
			iVec7: [0;2],
			iRec94: [0;2],
			fRec91: [0.0;2],
			fRec90: [0.0;2],
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
			fRec97: [0.0;2],
			fRec96: [0.0;2],
			fRec54: [0.0;2],
			fRec51: [0.0;2],
			fButton2: 0.0,
			fVec9: [0.0;2],
			fHslider5: 0.0,
			fRec100: [0.0;2],
			fRec99: [0.0;2],
			fRec127: [0.0;2],
			fRec131: [0.0;2],
			fRec133: [0.0;3],
			iRec134: [0;2],
			fRec135: [0.0;4],
			fRec136: [0.0;2048],
			fVec10: [0.0;2],
			fRec140: [0.0;2],
			iVec11: [0;2],
			iRec141: [0;2],
			fRec138: [0.0;2],
			fRec137: [0.0;2],
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
			fRec144: [0.0;2],
			fRec143: [0.0;2],
			fRec101: [0.0;2],
			fRec98: [0.0;2],
			fHslider6: 0.0,
			fRec146: [0.0;2],
			fRec173: [0.0;2],
			fRec177: [0.0;2],
			fRec179: [0.0;3],
			fButton3: 0.0,
			fVec13: [0.0;2],
			iRec180: [0;2],
			fRec181: [0.0;4],
			fRec182: [0.0;2048],
			fVec14: [0.0;2],
			fRec186: [0.0;2],
			iVec15: [0;2],
			iRec187: [0;2],
			fRec184: [0.0;2],
			fRec183: [0.0;2],
			fVec16: [0.0;3],
			fRec178: [0.0;2048],
			fRec169: [0.0;2],
			fRec165: [0.0;2],
			fRec161: [0.0;2048],
			fRec163: [0.0;2],
			fRec159: [0.0;4],
			fRec154: [0.0;2],
			fRec150: [0.0;2048],
			fRec148: [0.0;2],
			fRec190: [0.0;2],
			fRec189: [0.0;2],
			fRec147: [0.0;2],
			fRec145: [0.0;2],
			fButton4: 0.0,
			fVec17: [0.0;2],
			fHslider7: 0.0,
			fRec193: [0.0;2],
			fRec192: [0.0;2],
			fRec220: [0.0;2],
			fRec224: [0.0;2],
			fRec226: [0.0;3],
			iRec227: [0;2],
			fRec228: [0.0;4],
			fRec229: [0.0;2048],
			fVec18: [0.0;2],
			fRec233: [0.0;2],
			iVec19: [0;2],
			iRec234: [0;2],
			fRec231: [0.0;2],
			fRec230: [0.0;2],
			fVec20: [0.0;3],
			fRec225: [0.0;2048],
			fRec216: [0.0;2],
			fRec212: [0.0;2],
			fRec208: [0.0;2048],
			fRec210: [0.0;2],
			fRec206: [0.0;4],
			fRec201: [0.0;2],
			fRec197: [0.0;2048],
			fRec195: [0.0;2],
			fRec237: [0.0;2],
			fRec236: [0.0;2],
			fRec194: [0.0;2],
			fRec191: [0.0;2],
			fHslider8: 0.0,
			fRec238: [0.0;2],
			fHslider9: 0.0,
			fRec239: [0.0;2],
			fHslider10: 0.0,
			fRec242: [0.0;2],
			fConst13: 0.0,
			fConst14: 0.0,
			fHslider11: 0.0,
			fRec245: [0.0;2],
			fRec243: [0.0;2],
			fHslider12: 0.0,
			fRec246: [0.0;2],
			fConst15: 0.0,
			fHslider13: 0.0,
			fRec248: [0.0;2],
			fRec247: [0.0;2],
			fRec241: [0.0;3],
			fRec240: [0.0;3],
			fHslider14: 0.0,
			fRec251: [0.0;2],
			fHslider15: 0.0,
			fRec254: [0.0;2],
			fRec252: [0.0;2],
			fRec255: [0.0;2],
			fRec250: [0.0;3],
			fRec249: [0.0;3],
			fHslider16: 0.0,
			fRec258: [0.0;2],
			fHslider17: 0.0,
			fRec261: [0.0;2],
			fRec259: [0.0;2],
			fRec262: [0.0;2],
			fRec257: [0.0;3],
			fRec256: [0.0;3],
			fHslider18: 0.0,
			fRec265: [0.0;2],
			fHslider19: 0.0,
			fRec268: [0.0;2],
			fRec266: [0.0;2],
			fRec269: [0.0;2],
			fRec264: [0.0;3],
			fRec263: [0.0;3],
			iConst16: 0,
			fConst17: 0.0,
			fHslider20: 0.0,
			fRec270: [0.0;2],
			fHslider21: 0.0,
			fRec271: [0.0;2],
			fConst18: 0.0,
			fHslider22: 0.0,
			fRec274: [0.0;2],
			fRec273: [0.0;2],
			fVec21: [0.0;2],
			fVec22: [0.0;4096],
			fConst19: 0.0,
			fRec272: [0.0;2],
			fRec276: [0.0;2],
			fVec23: [0.0;2],
			fVec24: [0.0;4096],
			fRec275: [0.0;2],
			fRec278: [0.0;2],
			fVec25: [0.0;2],
			fVec26: [0.0;4096],
			fRec277: [0.0;2],
			fRec280: [0.0;2],
			fVec27: [0.0;2],
			fVec28: [0.0;4096],
			fRec279: [0.0;2],
			fRec0: [0.0;65536],
			fHslider23: 0.0,
			fRec281: [0.0;2],
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
		self.fHslider1 = 0.0;
		self.fButton0 = 0.0;
		self.fHslider2 = 80.0;
		self.fHslider3 = 1.0;
		self.fButton1 = 0.0;
		self.fHslider4 = 80.0;
		self.fButton2 = 0.0;
		self.fHslider5 = 80.0;
		self.fHslider6 = 80.0;
		self.fButton3 = 0.0;
		self.fButton4 = 0.0;
		self.fHslider7 = 80.0;
		self.fHslider8 = 0.0;
		self.fHslider9 = 1.0;
		self.fHslider10 = 0.0;
		self.fHslider11 = 60.0;
		self.fHslider12 = 0.0;
		self.fHslider13 = 0.0;
		self.fHslider14 = 0.0;
		self.fHslider15 = 60.0;
		self.fHslider16 = 0.0;
		self.fHslider17 = 60.0;
		self.fHslider18 = 0.0;
		self.fHslider19 = 60.0;
		self.fHslider20 = 0.0;
		self.fHslider21 = 1.0;
		self.fHslider22 = 60.0;
		self.fHslider23 = 1.0;
	}
	fn instance_clear(&mut self) {
		for l0 in 0..2 {
			self.iVec0[(l0) as usize] = 0;
		}
		for l1 in 0..2 {
			self.fRec1[(l1) as usize] = 0.0;
		}
		for l2 in 0..2 {
			self.fRec3[(l2) as usize] = 0.0;
		}
		for l3 in 0..2 {
			self.fVec1[(l3) as usize] = 0.0;
		}
		for l4 in 0..2 {
			self.fRec5[(l4) as usize] = 0.0;
		}
		for l5 in 0..2 {
			self.fRec4[(l5) as usize] = 0.0;
		}
		self.IOTA0 = 0;
		for l6 in 0..2 {
			self.fRec32[(l6) as usize] = 0.0;
		}
		for l7 in 0..2 {
			self.fRec36[(l7) as usize] = 0.0;
		}
		for l8 in 0..2 {
			self.iRec39[(l8) as usize] = 0;
		}
		for l9 in 0..3 {
			self.fRec38[(l9) as usize] = 0.0;
		}
		for l10 in 0..2 {
			self.iRec40[(l10) as usize] = 0;
		}
		for l11 in 0..4 {
			self.fRec41[(l11) as usize] = 0.0;
		}
		for l12 in 0..2048 {
			self.fRec42[(l12) as usize] = 0.0;
		}
		for l13 in 0..2 {
			self.fVec2[(l13) as usize] = 0.0;
		}
		for l14 in 0..2 {
			self.fRec46[(l14) as usize] = 0.0;
		}
		for l15 in 0..2 {
			self.iVec3[(l15) as usize] = 0;
		}
		for l16 in 0..2 {
			self.iRec47[(l16) as usize] = 0;
		}
		for l17 in 0..2 {
			self.fRec44[(l17) as usize] = 0.0;
		}
		for l18 in 0..2 {
			self.fRec43[(l18) as usize] = 0.0;
		}
		for l19 in 0..3 {
			self.fVec4[(l19) as usize] = 0.0;
		}
		for l20 in 0..2048 {
			self.fRec37[(l20) as usize] = 0.0;
		}
		for l21 in 0..2 {
			self.fRec28[(l21) as usize] = 0.0;
		}
		for l22 in 0..2 {
			self.fRec24[(l22) as usize] = 0.0;
		}
		for l23 in 0..2048 {
			self.fRec20[(l23) as usize] = 0.0;
		}
		for l24 in 0..2 {
			self.fRec22[(l24) as usize] = 0.0;
		}
		for l25 in 0..4 {
			self.fRec18[(l25) as usize] = 0.0;
		}
		for l26 in 0..2 {
			self.fRec13[(l26) as usize] = 0.0;
		}
		for l27 in 0..2048 {
			self.fRec9[(l27) as usize] = 0.0;
		}
		for l28 in 0..2 {
			self.fRec7[(l28) as usize] = 0.0;
		}
		for l29 in 0..2 {
			self.fRec50[(l29) as usize] = 0.0;
		}
		for l30 in 0..2 {
			self.fRec49[(l30) as usize] = 0.0;
		}
		for l31 in 0..2 {
			self.fRec6[(l31) as usize] = 0.0;
		}
		for l32 in 0..2 {
			self.fRec2[(l32) as usize] = 0.0;
		}
		for l33 in 0..2 {
			self.fVec5[(l33) as usize] = 0.0;
		}
		for l34 in 0..2 {
			self.fRec53[(l34) as usize] = 0.0;
		}
		for l35 in 0..2 {
			self.fRec52[(l35) as usize] = 0.0;
		}
		for l36 in 0..2 {
			self.fRec80[(l36) as usize] = 0.0;
		}
		for l37 in 0..2 {
			self.fRec84[(l37) as usize] = 0.0;
		}
		for l38 in 0..3 {
			self.fRec86[(l38) as usize] = 0.0;
		}
		for l39 in 0..2 {
			self.iRec87[(l39) as usize] = 0;
		}
		for l40 in 0..4 {
			self.fRec88[(l40) as usize] = 0.0;
		}
		for l41 in 0..2048 {
			self.fRec89[(l41) as usize] = 0.0;
		}
		for l42 in 0..2 {
			self.fVec6[(l42) as usize] = 0.0;
		}
		for l43 in 0..2 {
			self.fRec93[(l43) as usize] = 0.0;
		}
		for l44 in 0..2 {
			self.iVec7[(l44) as usize] = 0;
		}
		for l45 in 0..2 {
			self.iRec94[(l45) as usize] = 0;
		}
		for l46 in 0..2 {
			self.fRec91[(l46) as usize] = 0.0;
		}
		for l47 in 0..2 {
			self.fRec90[(l47) as usize] = 0.0;
		}
		for l48 in 0..3 {
			self.fVec8[(l48) as usize] = 0.0;
		}
		for l49 in 0..2048 {
			self.fRec85[(l49) as usize] = 0.0;
		}
		for l50 in 0..2 {
			self.fRec76[(l50) as usize] = 0.0;
		}
		for l51 in 0..2 {
			self.fRec72[(l51) as usize] = 0.0;
		}
		for l52 in 0..2048 {
			self.fRec68[(l52) as usize] = 0.0;
		}
		for l53 in 0..2 {
			self.fRec70[(l53) as usize] = 0.0;
		}
		for l54 in 0..4 {
			self.fRec66[(l54) as usize] = 0.0;
		}
		for l55 in 0..2 {
			self.fRec61[(l55) as usize] = 0.0;
		}
		for l56 in 0..2048 {
			self.fRec57[(l56) as usize] = 0.0;
		}
		for l57 in 0..2 {
			self.fRec55[(l57) as usize] = 0.0;
		}
		for l58 in 0..2 {
			self.fRec97[(l58) as usize] = 0.0;
		}
		for l59 in 0..2 {
			self.fRec96[(l59) as usize] = 0.0;
		}
		for l60 in 0..2 {
			self.fRec54[(l60) as usize] = 0.0;
		}
		for l61 in 0..2 {
			self.fRec51[(l61) as usize] = 0.0;
		}
		for l62 in 0..2 {
			self.fVec9[(l62) as usize] = 0.0;
		}
		for l63 in 0..2 {
			self.fRec100[(l63) as usize] = 0.0;
		}
		for l64 in 0..2 {
			self.fRec99[(l64) as usize] = 0.0;
		}
		for l65 in 0..2 {
			self.fRec127[(l65) as usize] = 0.0;
		}
		for l66 in 0..2 {
			self.fRec131[(l66) as usize] = 0.0;
		}
		for l67 in 0..3 {
			self.fRec133[(l67) as usize] = 0.0;
		}
		for l68 in 0..2 {
			self.iRec134[(l68) as usize] = 0;
		}
		for l69 in 0..4 {
			self.fRec135[(l69) as usize] = 0.0;
		}
		for l70 in 0..2048 {
			self.fRec136[(l70) as usize] = 0.0;
		}
		for l71 in 0..2 {
			self.fVec10[(l71) as usize] = 0.0;
		}
		for l72 in 0..2 {
			self.fRec140[(l72) as usize] = 0.0;
		}
		for l73 in 0..2 {
			self.iVec11[(l73) as usize] = 0;
		}
		for l74 in 0..2 {
			self.iRec141[(l74) as usize] = 0;
		}
		for l75 in 0..2 {
			self.fRec138[(l75) as usize] = 0.0;
		}
		for l76 in 0..2 {
			self.fRec137[(l76) as usize] = 0.0;
		}
		for l77 in 0..3 {
			self.fVec12[(l77) as usize] = 0.0;
		}
		for l78 in 0..2048 {
			self.fRec132[(l78) as usize] = 0.0;
		}
		for l79 in 0..2 {
			self.fRec123[(l79) as usize] = 0.0;
		}
		for l80 in 0..2 {
			self.fRec119[(l80) as usize] = 0.0;
		}
		for l81 in 0..2048 {
			self.fRec115[(l81) as usize] = 0.0;
		}
		for l82 in 0..2 {
			self.fRec117[(l82) as usize] = 0.0;
		}
		for l83 in 0..4 {
			self.fRec113[(l83) as usize] = 0.0;
		}
		for l84 in 0..2 {
			self.fRec108[(l84) as usize] = 0.0;
		}
		for l85 in 0..2048 {
			self.fRec104[(l85) as usize] = 0.0;
		}
		for l86 in 0..2 {
			self.fRec102[(l86) as usize] = 0.0;
		}
		for l87 in 0..2 {
			self.fRec144[(l87) as usize] = 0.0;
		}
		for l88 in 0..2 {
			self.fRec143[(l88) as usize] = 0.0;
		}
		for l89 in 0..2 {
			self.fRec101[(l89) as usize] = 0.0;
		}
		for l90 in 0..2 {
			self.fRec98[(l90) as usize] = 0.0;
		}
		for l91 in 0..2 {
			self.fRec146[(l91) as usize] = 0.0;
		}
		for l92 in 0..2 {
			self.fRec173[(l92) as usize] = 0.0;
		}
		for l93 in 0..2 {
			self.fRec177[(l93) as usize] = 0.0;
		}
		for l94 in 0..3 {
			self.fRec179[(l94) as usize] = 0.0;
		}
		for l95 in 0..2 {
			self.fVec13[(l95) as usize] = 0.0;
		}
		for l96 in 0..2 {
			self.iRec180[(l96) as usize] = 0;
		}
		for l97 in 0..4 {
			self.fRec181[(l97) as usize] = 0.0;
		}
		for l98 in 0..2048 {
			self.fRec182[(l98) as usize] = 0.0;
		}
		for l99 in 0..2 {
			self.fVec14[(l99) as usize] = 0.0;
		}
		for l100 in 0..2 {
			self.fRec186[(l100) as usize] = 0.0;
		}
		for l101 in 0..2 {
			self.iVec15[(l101) as usize] = 0;
		}
		for l102 in 0..2 {
			self.iRec187[(l102) as usize] = 0;
		}
		for l103 in 0..2 {
			self.fRec184[(l103) as usize] = 0.0;
		}
		for l104 in 0..2 {
			self.fRec183[(l104) as usize] = 0.0;
		}
		for l105 in 0..3 {
			self.fVec16[(l105) as usize] = 0.0;
		}
		for l106 in 0..2048 {
			self.fRec178[(l106) as usize] = 0.0;
		}
		for l107 in 0..2 {
			self.fRec169[(l107) as usize] = 0.0;
		}
		for l108 in 0..2 {
			self.fRec165[(l108) as usize] = 0.0;
		}
		for l109 in 0..2048 {
			self.fRec161[(l109) as usize] = 0.0;
		}
		for l110 in 0..2 {
			self.fRec163[(l110) as usize] = 0.0;
		}
		for l111 in 0..4 {
			self.fRec159[(l111) as usize] = 0.0;
		}
		for l112 in 0..2 {
			self.fRec154[(l112) as usize] = 0.0;
		}
		for l113 in 0..2048 {
			self.fRec150[(l113) as usize] = 0.0;
		}
		for l114 in 0..2 {
			self.fRec148[(l114) as usize] = 0.0;
		}
		for l115 in 0..2 {
			self.fRec190[(l115) as usize] = 0.0;
		}
		for l116 in 0..2 {
			self.fRec189[(l116) as usize] = 0.0;
		}
		for l117 in 0..2 {
			self.fRec147[(l117) as usize] = 0.0;
		}
		for l118 in 0..2 {
			self.fRec145[(l118) as usize] = 0.0;
		}
		for l119 in 0..2 {
			self.fVec17[(l119) as usize] = 0.0;
		}
		for l120 in 0..2 {
			self.fRec193[(l120) as usize] = 0.0;
		}
		for l121 in 0..2 {
			self.fRec192[(l121) as usize] = 0.0;
		}
		for l122 in 0..2 {
			self.fRec220[(l122) as usize] = 0.0;
		}
		for l123 in 0..2 {
			self.fRec224[(l123) as usize] = 0.0;
		}
		for l124 in 0..3 {
			self.fRec226[(l124) as usize] = 0.0;
		}
		for l125 in 0..2 {
			self.iRec227[(l125) as usize] = 0;
		}
		for l126 in 0..4 {
			self.fRec228[(l126) as usize] = 0.0;
		}
		for l127 in 0..2048 {
			self.fRec229[(l127) as usize] = 0.0;
		}
		for l128 in 0..2 {
			self.fVec18[(l128) as usize] = 0.0;
		}
		for l129 in 0..2 {
			self.fRec233[(l129) as usize] = 0.0;
		}
		for l130 in 0..2 {
			self.iVec19[(l130) as usize] = 0;
		}
		for l131 in 0..2 {
			self.iRec234[(l131) as usize] = 0;
		}
		for l132 in 0..2 {
			self.fRec231[(l132) as usize] = 0.0;
		}
		for l133 in 0..2 {
			self.fRec230[(l133) as usize] = 0.0;
		}
		for l134 in 0..3 {
			self.fVec20[(l134) as usize] = 0.0;
		}
		for l135 in 0..2048 {
			self.fRec225[(l135) as usize] = 0.0;
		}
		for l136 in 0..2 {
			self.fRec216[(l136) as usize] = 0.0;
		}
		for l137 in 0..2 {
			self.fRec212[(l137) as usize] = 0.0;
		}
		for l138 in 0..2048 {
			self.fRec208[(l138) as usize] = 0.0;
		}
		for l139 in 0..2 {
			self.fRec210[(l139) as usize] = 0.0;
		}
		for l140 in 0..4 {
			self.fRec206[(l140) as usize] = 0.0;
		}
		for l141 in 0..2 {
			self.fRec201[(l141) as usize] = 0.0;
		}
		for l142 in 0..2048 {
			self.fRec197[(l142) as usize] = 0.0;
		}
		for l143 in 0..2 {
			self.fRec195[(l143) as usize] = 0.0;
		}
		for l144 in 0..2 {
			self.fRec237[(l144) as usize] = 0.0;
		}
		for l145 in 0..2 {
			self.fRec236[(l145) as usize] = 0.0;
		}
		for l146 in 0..2 {
			self.fRec194[(l146) as usize] = 0.0;
		}
		for l147 in 0..2 {
			self.fRec191[(l147) as usize] = 0.0;
		}
		for l148 in 0..2 {
			self.fRec238[(l148) as usize] = 0.0;
		}
		for l149 in 0..2 {
			self.fRec239[(l149) as usize] = 0.0;
		}
		for l150 in 0..2 {
			self.fRec242[(l150) as usize] = 0.0;
		}
		for l151 in 0..2 {
			self.fRec245[(l151) as usize] = 0.0;
		}
		for l152 in 0..2 {
			self.fRec243[(l152) as usize] = 0.0;
		}
		for l153 in 0..2 {
			self.fRec246[(l153) as usize] = 0.0;
		}
		for l154 in 0..2 {
			self.fRec248[(l154) as usize] = 0.0;
		}
		for l155 in 0..2 {
			self.fRec247[(l155) as usize] = 0.0;
		}
		for l156 in 0..3 {
			self.fRec241[(l156) as usize] = 0.0;
		}
		for l157 in 0..3 {
			self.fRec240[(l157) as usize] = 0.0;
		}
		for l158 in 0..2 {
			self.fRec251[(l158) as usize] = 0.0;
		}
		for l159 in 0..2 {
			self.fRec254[(l159) as usize] = 0.0;
		}
		for l160 in 0..2 {
			self.fRec252[(l160) as usize] = 0.0;
		}
		for l161 in 0..2 {
			self.fRec255[(l161) as usize] = 0.0;
		}
		for l162 in 0..3 {
			self.fRec250[(l162) as usize] = 0.0;
		}
		for l163 in 0..3 {
			self.fRec249[(l163) as usize] = 0.0;
		}
		for l164 in 0..2 {
			self.fRec258[(l164) as usize] = 0.0;
		}
		for l165 in 0..2 {
			self.fRec261[(l165) as usize] = 0.0;
		}
		for l166 in 0..2 {
			self.fRec259[(l166) as usize] = 0.0;
		}
		for l167 in 0..2 {
			self.fRec262[(l167) as usize] = 0.0;
		}
		for l168 in 0..3 {
			self.fRec257[(l168) as usize] = 0.0;
		}
		for l169 in 0..3 {
			self.fRec256[(l169) as usize] = 0.0;
		}
		for l170 in 0..2 {
			self.fRec265[(l170) as usize] = 0.0;
		}
		for l171 in 0..2 {
			self.fRec268[(l171) as usize] = 0.0;
		}
		for l172 in 0..2 {
			self.fRec266[(l172) as usize] = 0.0;
		}
		for l173 in 0..2 {
			self.fRec269[(l173) as usize] = 0.0;
		}
		for l174 in 0..3 {
			self.fRec264[(l174) as usize] = 0.0;
		}
		for l175 in 0..3 {
			self.fRec263[(l175) as usize] = 0.0;
		}
		for l176 in 0..2 {
			self.fRec270[(l176) as usize] = 0.0;
		}
		for l177 in 0..2 {
			self.fRec271[(l177) as usize] = 0.0;
		}
		for l178 in 0..2 {
			self.fRec274[(l178) as usize] = 0.0;
		}
		for l179 in 0..2 {
			self.fRec273[(l179) as usize] = 0.0;
		}
		for l180 in 0..2 {
			self.fVec21[(l180) as usize] = 0.0;
		}
		for l181 in 0..4096 {
			self.fVec22[(l181) as usize] = 0.0;
		}
		for l182 in 0..2 {
			self.fRec272[(l182) as usize] = 0.0;
		}
		for l183 in 0..2 {
			self.fRec276[(l183) as usize] = 0.0;
		}
		for l184 in 0..2 {
			self.fVec23[(l184) as usize] = 0.0;
		}
		for l185 in 0..4096 {
			self.fVec24[(l185) as usize] = 0.0;
		}
		for l186 in 0..2 {
			self.fRec275[(l186) as usize] = 0.0;
		}
		for l187 in 0..2 {
			self.fRec278[(l187) as usize] = 0.0;
		}
		for l188 in 0..2 {
			self.fVec25[(l188) as usize] = 0.0;
		}
		for l189 in 0..4096 {
			self.fVec26[(l189) as usize] = 0.0;
		}
		for l190 in 0..2 {
			self.fRec277[(l190) as usize] = 0.0;
		}
		for l191 in 0..2 {
			self.fRec280[(l191) as usize] = 0.0;
		}
		for l192 in 0..2 {
			self.fVec27[(l192) as usize] = 0.0;
		}
		for l193 in 0..4096 {
			self.fVec28[(l193) as usize] = 0.0;
		}
		for l194 in 0..2 {
			self.fRec279[(l194) as usize] = 0.0;
		}
		for l195 in 0..65536 {
			self.fRec0[(l195) as usize] = 0.0;
		}
		for l196 in 0..2 {
			self.fRec281[(l196) as usize] = 0.0;
		}
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		self.fConst0 = F32::min(192000.0, F32::max(1.0, ((self.fSampleRate) as F32)));
		self.fConst1 = 44.0999985 / self.fConst0;
		self.fConst2 = 1.0 - self.fConst1;
		self.fConst3 = 2764.60156 / self.fConst0;
		self.fConst4 = 0.00882352982 * self.fConst0;
		self.fConst5 = 0.000735294132 * self.fConst0;
		self.fConst6 = 6911.50391 / self.fConst0;
		self.fConst7 = 0.00200000009 * self.fConst0;
		self.fConst8 = F32::exp(0.0 - 10000.0 / self.fConst0);
		self.fConst9 = 1.0 - self.fConst8;
		self.iConst10 = ((0.100000001 * self.fConst0) as i32);
		self.fConst11 = F32::exp(0.0 - 50.0 / self.fConst0);
		self.fConst12 = F32::exp(0.0 - 10.0 / self.fConst0);
		self.fConst13 = 1.0 / self.fConst0;
		self.fConst14 = 19404.0 / self.fConst0;
		self.fConst15 = 3.14159274 / self.fConst0;
		self.iConst16 = ((F32::min(self.fConst0, F32::max(0.0, 0.300000012 * self.fConst0))) as i32) + 1;
		self.fConst17 = 352.0 / self.fConst0;
		self.fConst18 = 0.25 * self.fConst0;
		self.fConst19 = 0.5 * self.fConst0;
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
		ui_interface.add_horizontal_slider("cutoffNote", ParamIndex(1), 0.0, -20.0, 50.0, 0.001);
		ui_interface.declare(Some(ParamIndex(2)), "2", "");
		ui_interface.add_horizontal_slider("res", ParamIndex(2), 0.0, 0.0, 0.98999999999999999, 0.001);
		ui_interface.declare(None, "3", "");
		ui_interface.open_vertical_box("0");
		ui_interface.declare(Some(ParamIndex(3)), "0", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(3), 60.0, 0.0, 127.0, 0.001);
		ui_interface.declare(Some(ParamIndex(4)), "1", "");
		ui_interface.add_horizontal_slider("volume", ParamIndex(4), 0.0, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "3", "");
		ui_interface.open_vertical_box("1");
		ui_interface.declare(Some(ParamIndex(5)), "0", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(5), 60.0, 0.0, 127.0, 0.001);
		ui_interface.declare(Some(ParamIndex(6)), "1", "");
		ui_interface.add_horizontal_slider("volume", ParamIndex(6), 0.0, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "3", "");
		ui_interface.open_vertical_box("2");
		ui_interface.declare(Some(ParamIndex(7)), "0", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(7), 60.0, 0.0, 127.0, 0.001);
		ui_interface.declare(Some(ParamIndex(8)), "1", "");
		ui_interface.add_horizontal_slider("volume", ParamIndex(8), 0.0, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "3", "");
		ui_interface.open_vertical_box("3");
		ui_interface.declare(Some(ParamIndex(9)), "0", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(9), 60.0, 0.0, 127.0, 0.001);
		ui_interface.declare(Some(ParamIndex(10)), "1", "");
		ui_interface.add_horizontal_slider("volume", ParamIndex(10), 0.0, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.close_box();
		ui_interface.declare(None, "1", "");
		ui_interface.open_horizontal_box("pluck");
		ui_interface.declare(Some(ParamIndex(11)), "0", "");
		ui_interface.add_button("gate", ParamIndex(11));
		ui_interface.declare(Some(ParamIndex(12)), "1", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(12), 80.0, 0.0, 127.0, 0.001);
		ui_interface.declare(Some(ParamIndex(13)), "2", "");
		ui_interface.add_horizontal_slider("mute", ParamIndex(13), 1.0, 0.90000000000000002, 1.0, 0.001);
		ui_interface.declare(None, "3", "");
		ui_interface.open_vertical_box("0");
		ui_interface.declare(Some(ParamIndex(14)), "0", "");
		ui_interface.add_button("gate", ParamIndex(14));
		ui_interface.declare(Some(ParamIndex(15)), "1", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(15), 80.0, 0.0, 127.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "3", "");
		ui_interface.open_vertical_box("1");
		ui_interface.declare(Some(ParamIndex(16)), "0", "");
		ui_interface.add_button("gate", ParamIndex(16));
		ui_interface.declare(Some(ParamIndex(17)), "1", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(17), 80.0, 0.0, 127.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "3", "");
		ui_interface.open_vertical_box("2");
		ui_interface.declare(Some(ParamIndex(18)), "0", "");
		ui_interface.add_button("gate", ParamIndex(18));
		ui_interface.declare(Some(ParamIndex(19)), "1", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(19), 80.0, 0.0, 127.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "3", "");
		ui_interface.open_vertical_box("3");
		ui_interface.declare(Some(ParamIndex(20)), "0", "");
		ui_interface.add_button("gate", ParamIndex(20));
		ui_interface.declare(Some(ParamIndex(21)), "1", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(21), 80.0, 0.0, 127.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(Some(ParamIndex(22)), "3", "");
		ui_interface.add_horizontal_slider("pitchBend", ParamIndex(22), 0.0, -1.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "2", "");
		ui_interface.open_horizontal_box("drone");
		ui_interface.declare(Some(ParamIndex(23)), "0", "");
		ui_interface.add_horizontal_slider("volume", ParamIndex(23), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(24)), "1", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(24), 60.0, 0.0, 127.0, 0.001);
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
			20 => Some(self.fButton0),
			18 => Some(self.fButton1),
			16 => Some(self.fButton2),
			11 => Some(self.fButton3),
			14 => Some(self.fButton4),
			28 => Some(self.fHslider0),
			22 => Some(self.fHslider1),
			4 => Some(self.fHslider10),
			3 => Some(self.fHslider11),
			2 => Some(self.fHslider12),
			1 => Some(self.fHslider13),
			6 => Some(self.fHslider14),
			5 => Some(self.fHslider15),
			8 => Some(self.fHslider16),
			7 => Some(self.fHslider17),
			10 => Some(self.fHslider18),
			9 => Some(self.fHslider19),
			21 => Some(self.fHslider2),
			23 => Some(self.fHslider20),
			26 => Some(self.fHslider21),
			24 => Some(self.fHslider22),
			25 => Some(self.fHslider23),
			13 => Some(self.fHslider3),
			19 => Some(self.fHslider4),
			17 => Some(self.fHslider5),
			12 => Some(self.fHslider6),
			15 => Some(self.fHslider7),
			0 => Some(self.fHslider8),
			27 => Some(self.fHslider9),
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			20 => { self.fButton0 = value }
			18 => { self.fButton1 = value }
			16 => { self.fButton2 = value }
			11 => { self.fButton3 = value }
			14 => { self.fButton4 = value }
			28 => { self.fHslider0 = value }
			22 => { self.fHslider1 = value }
			4 => { self.fHslider10 = value }
			3 => { self.fHslider11 = value }
			2 => { self.fHslider12 = value }
			1 => { self.fHslider13 = value }
			6 => { self.fHslider14 = value }
			5 => { self.fHslider15 = value }
			8 => { self.fHslider16 = value }
			7 => { self.fHslider17 = value }
			10 => { self.fHslider18 = value }
			9 => { self.fHslider19 = value }
			21 => { self.fHslider2 = value }
			23 => { self.fHslider20 = value }
			26 => { self.fHslider21 = value }
			24 => { self.fHslider22 = value }
			25 => { self.fHslider23 = value }
			13 => { self.fHslider3 = value }
			19 => { self.fHslider4 = value }
			17 => { self.fHslider5 = value }
			12 => { self.fHslider6 = value }
			15 => { self.fHslider7 = value }
			0 => { self.fHslider8 = value }
			27 => { self.fHslider9 = value }
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
		let mut fSlow0: F32 = self.fConst1 * ((self.fHslider0) as F32);
		let mut fSlow1: F32 = self.fConst1 * ((self.fHslider1) as F32);
		let mut fSlow2: F32 = ((self.fButton0) as F32);
		let mut iSlow3: i32 = ((1.0 - fSlow2) as i32);
		let mut fSlow4: F32 = ((self.fHslider2) as F32);
		let mut fSlow5: F32 = ((self.fHslider3) as F32);
		let mut fSlow6: F32 = ((self.fButton1) as F32);
		let mut iSlow7: i32 = ((1.0 - fSlow6) as i32);
		let mut fSlow8: F32 = ((self.fHslider4) as F32);
		let mut fSlow9: F32 = ((self.fButton2) as F32);
		let mut iSlow10: i32 = ((1.0 - fSlow9) as i32);
		let mut fSlow11: F32 = ((self.fHslider5) as F32);
		let mut fSlow12: F32 = self.fConst1 * ((self.fHslider6) as F32);
		let mut fSlow13: F32 = ((self.fButton3) as F32);
		let mut fSlow14: F32 = ((self.fButton4) as F32);
		let mut iSlow15: i32 = ((1.0 - fSlow14) as i32);
		let mut fSlow16: F32 = ((self.fHslider7) as F32);
		let mut fSlow17: F32 = self.fConst1 * ((self.fHslider8) as F32);
		let mut fSlow18: F32 = self.fConst1 * ((self.fHslider9) as F32);
		let mut fSlow19: F32 = self.fConst1 * ((self.fHslider10) as F32);
		let mut fSlow20: F32 = ((self.fHslider11) as F32);
		let mut fSlow21: F32 = self.fConst14 * F32::powf(2.0, 0.0833333358 * (fSlow20 + -69.0));
		let mut fSlow22: F32 = self.fConst1 * ((self.fHslider12) as F32);
		let mut fSlow23: F32 = self.fConst1 * ((self.fHslider13) as F32);
		let mut fSlow24: F32 = self.fConst1 * ((self.fHslider14) as F32);
		let mut fSlow25: F32 = ((self.fHslider15) as F32);
		let mut fSlow26: F32 = self.fConst14 * F32::powf(2.0, 0.0833333358 * (fSlow25 + -69.0));
		let mut fSlow27: F32 = self.fConst1 * ((self.fHslider16) as F32);
		let mut fSlow28: F32 = ((self.fHslider17) as F32);
		let mut fSlow29: F32 = self.fConst14 * F32::powf(2.0, 0.0833333358 * (fSlow28 + -69.0));
		let mut fSlow30: F32 = self.fConst1 * ((self.fHslider18) as F32);
		let mut fSlow31: F32 = ((self.fHslider19) as F32);
		let mut fSlow32: F32 = self.fConst14 * F32::powf(2.0, 0.0833333358 * (fSlow31 + -69.0));
		let mut fSlow33: F32 = self.fConst1 * ((self.fHslider20) as F32);
		let mut fSlow34: F32 = self.fConst1 * ((self.fHslider21) as F32);
		let mut fSlow35: F32 = self.fConst1 * ((self.fHslider22) as F32);
		let mut fSlow36: F32 = self.fConst1 * ((self.fHslider23) as F32);
		let zipped_iterators = outputs0.zip(outputs1);
		for (output0, output1) in zipped_iterators {
			self.iVec0[0] = 1;
			self.fRec1[0] = fSlow0 + self.fConst2 * self.fRec1[1];
			self.fRec3[0] = fSlow1 + self.fConst2 * self.fRec3[1];
			self.fVec1[0] = fSlow2;
			self.fRec5[0] = if (iSlow3 as i32 != 0) { fSlow4 } else { self.fRec5[1] };
			self.fRec4[0] = self.fConst2 * self.fRec4[1] + self.fConst1 * self.fRec5[0];
			let mut fTemp0: F32 = F32::powf(2.0, 0.0833333358 * (self.fRec3[0] + self.fRec4[0] + -69.0));
			let mut fTemp1: F32 = 1.0 / F32::tan(self.fConst3 * fTemp0);
			let mut fRec17: F32 = -1.0 * 0.997305274 * (0.899999976 * self.fRec18[2] + 0.0500000007 * (self.fRec18[1] + self.fRec18[3]));
			let mut fTemp2: F32 = self.fConst5 * (0.772727251 / fTemp0 + -0.109999999);
			let mut fTemp3: F32 = fTemp2 + -1.49999499;
			let mut iTemp4: i32 = ((fTemp3) as i32);
			let mut iTemp5: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, iTemp4)) as F32))) as i32);
			let mut iTemp6: i32 = iTemp5 + 1;
			let mut fTemp7: F32 = F32::floor(fTemp3);
			let mut fTemp8: F32 = fTemp2 + -1.0 - fTemp7;
			let mut fTemp9: F32 = 0.0 - fTemp8;
			let mut fTemp10: F32 = fTemp2 + -2.0 - fTemp7;
			let mut fTemp11: F32 = 0.0 - 0.5 * fTemp10;
			let mut fTemp12: F32 = fTemp2 + -3.0 - fTemp7;
			let mut fTemp13: F32 = 0.0 - 0.333333343 * fTemp12;
			let mut fTemp14: F32 = fTemp2 + -4.0 - fTemp7;
			let mut fTemp15: F32 = 0.0 - 0.25 * fTemp14;
			let mut fTemp16: F32 = fTemp2 - fTemp7;
			let mut iTemp17: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, iTemp4 + 1)) as F32))) as i32);
			let mut iTemp18: i32 = iTemp17 + 1;
			let mut fTemp19: F32 = 0.0 - fTemp10;
			let mut fTemp20: F32 = 0.0 - 0.5 * fTemp12;
			let mut fTemp21: F32 = 0.0 - 0.333333343 * fTemp14;
			let mut iTemp22: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, iTemp4 + 2)) as F32))) as i32);
			let mut iTemp23: i32 = iTemp22 + 1;
			let mut fTemp24: F32 = 0.0 - fTemp12;
			let mut fTemp25: F32 = 0.0 - 0.5 * fTemp14;
			let mut fTemp26: F32 = fTemp8 * fTemp10;
			let mut iTemp27: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, iTemp4 + 3)) as F32))) as i32);
			let mut iTemp28: i32 = iTemp27 + 1;
			let mut fTemp29: F32 = 0.0 - fTemp14;
			let mut fTemp30: F32 = fTemp26 * fTemp12;
			let mut iTemp31: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, iTemp4 + 4)) as F32))) as i32);
			let mut iTemp32: i32 = iTemp31 + 1;
			self.fRec32[0] = self.fRec9[((self.IOTA0 - iTemp6) & 2047) as usize] * fTemp9 * fTemp11 * fTemp13 * fTemp15 + fTemp16 * (self.fRec9[((self.IOTA0 - iTemp18) & 2047) as usize] * fTemp19 * fTemp20 * fTemp21 + 0.5 * fTemp8 * self.fRec9[((self.IOTA0 - iTemp23) & 2047) as usize] * fTemp24 * fTemp25 + 0.166666672 * fTemp26 * self.fRec9[((self.IOTA0 - iTemp28) & 2047) as usize] * fTemp29 + 0.0416666679 * fTemp30 * self.fRec9[((self.IOTA0 - iTemp32) & 2047) as usize]);
			self.fRec36[0] = 0.0500000007 * self.fRec36[1] + 0.949999988 * self.fRec32[1];
			let mut fRec33: F32 = self.fRec36[0];
			let mut fTemp33: F32 = fTemp9 * fTemp11 * fTemp13 * fTemp15;
			self.iRec39[0] = 1103515245 * self.iRec39[1] + 12345;
			let mut fTemp34: F32 = 4.65661287e-10 * ((self.iRec39[0]) as F32);
			let mut fTemp35: F32 = F32::tan(self.fConst6 * fTemp0);
			let mut fTemp36: F32 = 1.0 / fTemp35;
			let mut fTemp37: F32 = (fTemp36 + 1.41421354) / fTemp35 + 1.0;
			self.fRec38[0] = fTemp34 - (self.fRec38[2] * ((fTemp36 + -1.41421354) / fTemp35 + 1.0) + 2.0 * self.fRec38[1] * (1.0 - 1.0 / mydsp_faustpower2_f(fTemp35))) / fTemp37;
			self.iRec40[0] = (self.iRec40[1] + ((self.iRec40[1] > 0) as i32)) * ((fSlow2 <= self.fVec1[1]) as i32) + ((fSlow2 > self.fVec1[1]) as i32);
			let mut fTemp38: F32 = ((self.iRec40[0]) as F32) / F32::max(1.0, self.fConst7 * mydsp_faustpower2_f(1.0 - 0.219999999 * fTemp0));
			let mut fTemp39: F32 = 0.5 * ((self.fRec38[2] + self.fRec38[0] + 2.0 * self.fRec38[1]) * F32::max(0.0, F32::min(fTemp38, 2.0 - fTemp38))) / fTemp37;
			self.fRec41[0] = self.fRec7[1];
			self.fRec42[(self.IOTA0 & 2047) as usize] = -1.0 * 0.997305274 * (0.899999976 * self.fRec41[2] + 0.0500000007 * (self.fRec41[1] + self.fRec41[3]));
			let mut fTemp40: F32 = fTemp19 * fTemp20 * fTemp21;
			let mut fTemp41: F32 = fTemp8 * fTemp24 * fTemp25;
			let mut fTemp42: F32 = fTemp26 * fTemp29;
			self.fVec2[0] = fTemp33 * self.fRec42[((self.IOTA0 - (iTemp5 + 2)) & 2047) as usize] + fTemp16 * (fTemp40 * self.fRec42[((self.IOTA0 - (iTemp17 + 2)) & 2047) as usize] + 0.5 * fTemp41 * self.fRec42[((self.IOTA0 - (iTemp22 + 2)) & 2047) as usize] + 0.166666672 * fTemp42 * self.fRec42[((self.IOTA0 - (iTemp27 + 2)) & 2047) as usize] + 0.0416666679 * fTemp30 * self.fRec42[((self.IOTA0 - (iTemp31 + 2)) & 2047) as usize]);
			self.fRec46[0] = self.fConst8 * self.fRec46[1] + self.fConst9 * F32::abs(self.fRec6[1]);
			let mut fRec45: F32 = self.fRec46[0];
			let mut iTemp43: i32 = ((fRec45 > 0.100000001) as i32);
			self.iVec3[0] = iTemp43;
			self.iRec47[0] = std::cmp::max(((self.iConst10 * ((iTemp43 < self.iVec3[1]) as i32)) as i32), ((self.iRec47[1] + -1) as i32));
			let mut fTemp44: F32 = F32::abs(F32::max(((iTemp43) as F32), ((((self.iRec47[0] > 0) as i32)) as F32)));
			let mut fTemp45: F32 = if (((self.fRec43[1] > fTemp44) as i32) as i32 != 0) { self.fConst11 } else { self.fConst8 };
			self.fRec44[0] = self.fRec44[1] * fTemp45 + fTemp44 * (1.0 - fTemp45);
			self.fRec43[0] = self.fRec44[0];
			let mut fTemp46: F32 = 0.00499999989 * self.fRec43[0] * self.fRec6[1];
			let mut fTemp47: F32 = fTemp39 + self.fVec2[1] + fTemp46;
			self.fVec4[0] = fTemp47;
			self.fRec37[(self.IOTA0 & 2047) as usize] = 0.0500000007 * self.fRec37[((self.IOTA0 - 1) & 2047) as usize] + 0.949999988 * self.fVec4[2];
			let mut fRec34: F32 = fTemp33 * self.fRec37[((self.IOTA0 - iTemp5) & 2047) as usize] + fTemp16 * (fTemp40 * self.fRec37[((self.IOTA0 - iTemp17) & 2047) as usize] + 0.5 * fTemp41 * self.fRec37[((self.IOTA0 - iTemp22) & 2047) as usize] + 0.166666672 * fTemp42 * self.fRec37[((self.IOTA0 - iTemp27) & 2047) as usize] + 0.0416666679 * fTemp30 * self.fRec37[((self.IOTA0 - iTemp31) & 2047) as usize]);
			let mut fRec35: F32 = self.fVec4[1] + self.fRec28[1];
			self.fRec28[0] = fRec33;
			let mut fRec29: F32 = self.fRec28[1];
			let mut fRec30: F32 = fRec34;
			let mut fRec31: F32 = fRec35;
			self.fRec24[0] = fRec29;
			let mut fRec25: F32 = fTemp46 + fTemp39 + self.fRec24[1];
			let mut fRec26: F32 = fRec30;
			let mut fRec27: F32 = fRec31;
			self.fRec20[(self.IOTA0 & 2047) as usize] = fRec25;
			let mut fRec21: F32 = fTemp33 * self.fRec20[((self.IOTA0 - iTemp6) & 2047) as usize] + fTemp16 * (fTemp40 * self.fRec20[((self.IOTA0 - iTemp18) & 2047) as usize] + 0.5 * fTemp41 * self.fRec20[((self.IOTA0 - iTemp23) & 2047) as usize] + 0.166666672 * fTemp42 * self.fRec20[((self.IOTA0 - iTemp28) & 2047) as usize] + 0.0416666679 * fTemp30 * self.fRec20[((self.IOTA0 - iTemp32) & 2047) as usize]);
			self.fRec22[0] = fRec26;
			let mut fRec23: F32 = fRec27;
			self.fRec18[0] = fSlow5 * self.fRec22[1];
			let mut fRec19: F32 = fRec23;
			self.fRec13[0] = fRec17;
			let mut fRec14: F32 = fSlow5 * self.fRec13[1];
			let mut fRec15: F32 = self.fRec18[0];
			let mut fRec16: F32 = fRec19;
			self.fRec9[(self.IOTA0 & 2047) as usize] = fRec14;
			let mut fRec10: F32 = fRec21;
			let mut fRec11: F32 = fRec15;
			let mut fRec12: F32 = fRec16;
			self.fRec7[0] = fRec10;
			let mut fRec8: F32 = fRec12;
			let mut fTemp48: F32 = F32::abs(fRec8);
			let mut fTemp49: F32 = if (((self.fRec49[1] > fTemp48) as i32) as i32 != 0) { self.fConst12 } else { 0.0 };
			self.fRec50[0] = self.fRec50[1] * fTemp49 + fTemp48 * (1.0 - fTemp49);
			self.fRec49[0] = self.fRec50[0];
			let mut fRec48: F32 = 0.0 - 0.949999988 * F32::max(20.0 * F32::log10(self.fRec49[0]) + 10.0, 0.0);
			self.fRec6[0] = fRec8 * F32::powf(10.0, 0.0500000007 * fRec48);
			self.fRec2[0] = 0.0 - (self.fRec2[1] * (1.0 - fTemp1) - (self.fRec6[0] + self.fRec6[1])) / (fTemp1 + 1.0);
			self.fVec5[0] = fSlow6;
			self.fRec53[0] = if (iSlow7 as i32 != 0) { fSlow8 } else { self.fRec53[1] };
			self.fRec52[0] = self.fConst2 * self.fRec52[1] + self.fConst1 * self.fRec53[0];
			let mut fTemp50: F32 = F32::powf(2.0, 0.0833333358 * (self.fRec3[0] + self.fRec52[0] + -69.0));
			let mut fTemp51: F32 = 1.0 / F32::tan(self.fConst3 * fTemp50);
			let mut fRec65: F32 = -1.0 * 0.997305274 * (0.899999976 * self.fRec66[2] + 0.0500000007 * (self.fRec66[1] + self.fRec66[3]));
			let mut fTemp52: F32 = self.fConst5 * (0.772727251 / fTemp50 + -0.109999999);
			let mut fTemp53: F32 = fTemp52 + -1.49999499;
			let mut iTemp54: i32 = ((fTemp53) as i32);
			let mut iTemp55: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, iTemp54)) as F32))) as i32);
			let mut iTemp56: i32 = iTemp55 + 1;
			let mut fTemp57: F32 = F32::floor(fTemp53);
			let mut fTemp58: F32 = fTemp52 + -1.0 - fTemp57;
			let mut fTemp59: F32 = 0.0 - fTemp58;
			let mut fTemp60: F32 = fTemp52 + -2.0 - fTemp57;
			let mut fTemp61: F32 = 0.0 - 0.5 * fTemp60;
			let mut fTemp62: F32 = fTemp52 + -3.0 - fTemp57;
			let mut fTemp63: F32 = 0.0 - 0.333333343 * fTemp62;
			let mut fTemp64: F32 = fTemp52 + -4.0 - fTemp57;
			let mut fTemp65: F32 = 0.0 - 0.25 * fTemp64;
			let mut fTemp66: F32 = fTemp52 - fTemp57;
			let mut iTemp67: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, iTemp54 + 1)) as F32))) as i32);
			let mut iTemp68: i32 = iTemp67 + 1;
			let mut fTemp69: F32 = 0.0 - fTemp60;
			let mut fTemp70: F32 = 0.0 - 0.5 * fTemp62;
			let mut fTemp71: F32 = 0.0 - 0.333333343 * fTemp64;
			let mut iTemp72: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, iTemp54 + 2)) as F32))) as i32);
			let mut iTemp73: i32 = iTemp72 + 1;
			let mut fTemp74: F32 = 0.0 - fTemp62;
			let mut fTemp75: F32 = 0.0 - 0.5 * fTemp64;
			let mut fTemp76: F32 = fTemp58 * fTemp60;
			let mut iTemp77: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, iTemp54 + 3)) as F32))) as i32);
			let mut iTemp78: i32 = iTemp77 + 1;
			let mut fTemp79: F32 = 0.0 - fTemp64;
			let mut fTemp80: F32 = fTemp76 * fTemp62;
			let mut iTemp81: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, iTemp54 + 4)) as F32))) as i32);
			let mut iTemp82: i32 = iTemp81 + 1;
			self.fRec80[0] = self.fRec57[((self.IOTA0 - iTemp56) & 2047) as usize] * fTemp59 * fTemp61 * fTemp63 * fTemp65 + fTemp66 * (self.fRec57[((self.IOTA0 - iTemp68) & 2047) as usize] * fTemp69 * fTemp70 * fTemp71 + 0.5 * fTemp58 * self.fRec57[((self.IOTA0 - iTemp73) & 2047) as usize] * fTemp74 * fTemp75 + 0.166666672 * fTemp76 * self.fRec57[((self.IOTA0 - iTemp78) & 2047) as usize] * fTemp79 + 0.0416666679 * fTemp80 * self.fRec57[((self.IOTA0 - iTemp82) & 2047) as usize]);
			self.fRec84[0] = 0.0500000007 * self.fRec84[1] + 0.949999988 * self.fRec80[1];
			let mut fRec81: F32 = self.fRec84[0];
			let mut fTemp83: F32 = fTemp59 * fTemp61 * fTemp63 * fTemp65;
			let mut fTemp84: F32 = F32::tan(self.fConst6 * fTemp50);
			let mut fTemp85: F32 = 1.0 / fTemp84;
			let mut fTemp86: F32 = (fTemp85 + 1.41421354) / fTemp84 + 1.0;
			self.fRec86[0] = fTemp34 - (self.fRec86[2] * ((fTemp85 + -1.41421354) / fTemp84 + 1.0) + 2.0 * self.fRec86[1] * (1.0 - 1.0 / mydsp_faustpower2_f(fTemp84))) / fTemp86;
			self.iRec87[0] = (self.iRec87[1] + ((self.iRec87[1] > 0) as i32)) * ((fSlow6 <= self.fVec5[1]) as i32) + ((fSlow6 > self.fVec5[1]) as i32);
			let mut fTemp87: F32 = ((self.iRec87[0]) as F32) / F32::max(1.0, self.fConst7 * mydsp_faustpower2_f(1.0 - 0.219999999 * fTemp50));
			let mut fTemp88: F32 = 0.5 * ((self.fRec86[2] + self.fRec86[0] + 2.0 * self.fRec86[1]) * F32::max(0.0, F32::min(fTemp87, 2.0 - fTemp87))) / fTemp86;
			self.fRec88[0] = self.fRec55[1];
			self.fRec89[(self.IOTA0 & 2047) as usize] = -1.0 * 0.997305274 * (0.899999976 * self.fRec88[2] + 0.0500000007 * (self.fRec88[1] + self.fRec88[3]));
			let mut fTemp89: F32 = fTemp69 * fTemp70 * fTemp71;
			let mut fTemp90: F32 = fTemp58 * fTemp74 * fTemp75;
			let mut fTemp91: F32 = fTemp76 * fTemp79;
			self.fVec6[0] = fTemp83 * self.fRec89[((self.IOTA0 - (iTemp55 + 2)) & 2047) as usize] + fTemp66 * (fTemp89 * self.fRec89[((self.IOTA0 - (iTemp67 + 2)) & 2047) as usize] + 0.5 * fTemp90 * self.fRec89[((self.IOTA0 - (iTemp72 + 2)) & 2047) as usize] + 0.166666672 * fTemp91 * self.fRec89[((self.IOTA0 - (iTemp77 + 2)) & 2047) as usize] + 0.0416666679 * fTemp80 * self.fRec89[((self.IOTA0 - (iTemp81 + 2)) & 2047) as usize]);
			self.fRec93[0] = self.fConst8 * self.fRec93[1] + self.fConst9 * F32::abs(self.fRec54[1]);
			let mut fRec92: F32 = self.fRec93[0];
			let mut iTemp92: i32 = ((fRec92 > 0.100000001) as i32);
			self.iVec7[0] = iTemp92;
			self.iRec94[0] = std::cmp::max(((self.iConst10 * ((iTemp92 < self.iVec7[1]) as i32)) as i32), ((self.iRec94[1] + -1) as i32));
			let mut fTemp93: F32 = F32::abs(F32::max(((iTemp92) as F32), ((((self.iRec94[0] > 0) as i32)) as F32)));
			let mut fTemp94: F32 = if (((self.fRec90[1] > fTemp93) as i32) as i32 != 0) { self.fConst11 } else { self.fConst8 };
			self.fRec91[0] = self.fRec91[1] * fTemp94 + fTemp93 * (1.0 - fTemp94);
			self.fRec90[0] = self.fRec91[0];
			let mut fTemp95: F32 = 0.00499999989 * self.fRec90[0] * self.fRec54[1];
			let mut fTemp96: F32 = fTemp88 + self.fVec6[1] + fTemp95;
			self.fVec8[0] = fTemp96;
			self.fRec85[(self.IOTA0 & 2047) as usize] = 0.0500000007 * self.fRec85[((self.IOTA0 - 1) & 2047) as usize] + 0.949999988 * self.fVec8[2];
			let mut fRec82: F32 = fTemp83 * self.fRec85[((self.IOTA0 - iTemp55) & 2047) as usize] + fTemp66 * (fTemp89 * self.fRec85[((self.IOTA0 - iTemp67) & 2047) as usize] + 0.5 * fTemp90 * self.fRec85[((self.IOTA0 - iTemp72) & 2047) as usize] + 0.166666672 * fTemp91 * self.fRec85[((self.IOTA0 - iTemp77) & 2047) as usize] + 0.0416666679 * fTemp80 * self.fRec85[((self.IOTA0 - iTemp81) & 2047) as usize]);
			let mut fRec83: F32 = self.fVec8[1] + self.fRec76[1];
			self.fRec76[0] = fRec81;
			let mut fRec77: F32 = self.fRec76[1];
			let mut fRec78: F32 = fRec82;
			let mut fRec79: F32 = fRec83;
			self.fRec72[0] = fRec77;
			let mut fRec73: F32 = fTemp95 + fTemp88 + self.fRec72[1];
			let mut fRec74: F32 = fRec78;
			let mut fRec75: F32 = fRec79;
			self.fRec68[(self.IOTA0 & 2047) as usize] = fRec73;
			let mut fRec69: F32 = fTemp83 * self.fRec68[((self.IOTA0 - iTemp56) & 2047) as usize] + fTemp66 * (fTemp89 * self.fRec68[((self.IOTA0 - iTemp68) & 2047) as usize] + 0.5 * fTemp90 * self.fRec68[((self.IOTA0 - iTemp73) & 2047) as usize] + 0.166666672 * fTemp91 * self.fRec68[((self.IOTA0 - iTemp78) & 2047) as usize] + 0.0416666679 * fTemp80 * self.fRec68[((self.IOTA0 - iTemp82) & 2047) as usize]);
			self.fRec70[0] = fRec74;
			let mut fRec71: F32 = fRec75;
			self.fRec66[0] = fSlow5 * self.fRec70[1];
			let mut fRec67: F32 = fRec71;
			self.fRec61[0] = fRec65;
			let mut fRec62: F32 = fSlow5 * self.fRec61[1];
			let mut fRec63: F32 = self.fRec66[0];
			let mut fRec64: F32 = fRec67;
			self.fRec57[(self.IOTA0 & 2047) as usize] = fRec62;
			let mut fRec58: F32 = fRec69;
			let mut fRec59: F32 = fRec63;
			let mut fRec60: F32 = fRec64;
			self.fRec55[0] = fRec58;
			let mut fRec56: F32 = fRec60;
			let mut fTemp97: F32 = F32::abs(fRec56);
			let mut fTemp98: F32 = if (((self.fRec96[1] > fTemp97) as i32) as i32 != 0) { self.fConst12 } else { 0.0 };
			self.fRec97[0] = self.fRec97[1] * fTemp98 + fTemp97 * (1.0 - fTemp98);
			self.fRec96[0] = self.fRec97[0];
			let mut fRec95: F32 = 0.0 - 0.949999988 * F32::max(20.0 * F32::log10(self.fRec96[0]) + 10.0, 0.0);
			self.fRec54[0] = fRec56 * F32::powf(10.0, 0.0500000007 * fRec95);
			self.fRec51[0] = 0.0 - (self.fRec51[1] * (1.0 - fTemp51) - (self.fRec54[0] + self.fRec54[1])) / (fTemp51 + 1.0);
			self.fVec9[0] = fSlow9;
			self.fRec100[0] = if (iSlow10 as i32 != 0) { fSlow11 } else { self.fRec100[1] };
			self.fRec99[0] = self.fConst2 * self.fRec99[1] + self.fConst1 * self.fRec100[0];
			let mut fTemp99: F32 = F32::powf(2.0, 0.0833333358 * (self.fRec3[0] + self.fRec99[0] + -69.0));
			let mut fTemp100: F32 = 1.0 / F32::tan(self.fConst3 * fTemp99);
			let mut fRec112: F32 = -1.0 * 0.997305274 * (0.899999976 * self.fRec113[2] + 0.0500000007 * (self.fRec113[1] + self.fRec113[3]));
			let mut fTemp101: F32 = self.fConst5 * (0.772727251 / fTemp99 + -0.109999999);
			let mut fTemp102: F32 = fTemp101 + -1.49999499;
			let mut iTemp103: i32 = ((fTemp102) as i32);
			let mut iTemp104: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, iTemp103)) as F32))) as i32);
			let mut iTemp105: i32 = iTemp104 + 1;
			let mut fTemp106: F32 = F32::floor(fTemp102);
			let mut fTemp107: F32 = fTemp101 + -1.0 - fTemp106;
			let mut fTemp108: F32 = 0.0 - fTemp107;
			let mut fTemp109: F32 = fTemp101 + -2.0 - fTemp106;
			let mut fTemp110: F32 = 0.0 - 0.5 * fTemp109;
			let mut fTemp111: F32 = fTemp101 + -3.0 - fTemp106;
			let mut fTemp112: F32 = 0.0 - 0.333333343 * fTemp111;
			let mut fTemp113: F32 = fTemp101 + -4.0 - fTemp106;
			let mut fTemp114: F32 = 0.0 - 0.25 * fTemp113;
			let mut fTemp115: F32 = fTemp101 - fTemp106;
			let mut iTemp116: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, iTemp103 + 1)) as F32))) as i32);
			let mut iTemp117: i32 = iTemp116 + 1;
			let mut fTemp118: F32 = 0.0 - fTemp109;
			let mut fTemp119: F32 = 0.0 - 0.5 * fTemp111;
			let mut fTemp120: F32 = 0.0 - 0.333333343 * fTemp113;
			let mut iTemp121: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, iTemp103 + 2)) as F32))) as i32);
			let mut iTemp122: i32 = iTemp121 + 1;
			let mut fTemp123: F32 = 0.0 - fTemp111;
			let mut fTemp124: F32 = 0.0 - 0.5 * fTemp113;
			let mut fTemp125: F32 = fTemp107 * fTemp109;
			let mut iTemp126: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, iTemp103 + 3)) as F32))) as i32);
			let mut iTemp127: i32 = iTemp126 + 1;
			let mut fTemp128: F32 = 0.0 - fTemp113;
			let mut fTemp129: F32 = fTemp125 * fTemp111;
			let mut iTemp130: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, iTemp103 + 4)) as F32))) as i32);
			let mut iTemp131: i32 = iTemp130 + 1;
			self.fRec127[0] = self.fRec104[((self.IOTA0 - iTemp105) & 2047) as usize] * fTemp108 * fTemp110 * fTemp112 * fTemp114 + fTemp115 * (self.fRec104[((self.IOTA0 - iTemp117) & 2047) as usize] * fTemp118 * fTemp119 * fTemp120 + 0.5 * fTemp107 * self.fRec104[((self.IOTA0 - iTemp122) & 2047) as usize] * fTemp123 * fTemp124 + 0.166666672 * fTemp125 * self.fRec104[((self.IOTA0 - iTemp127) & 2047) as usize] * fTemp128 + 0.0416666679 * fTemp129 * self.fRec104[((self.IOTA0 - iTemp131) & 2047) as usize]);
			self.fRec131[0] = 0.0500000007 * self.fRec131[1] + 0.949999988 * self.fRec127[1];
			let mut fRec128: F32 = self.fRec131[0];
			let mut fTemp132: F32 = fTemp108 * fTemp110 * fTemp112 * fTemp114;
			let mut fTemp133: F32 = F32::tan(self.fConst6 * fTemp99);
			let mut fTemp134: F32 = 1.0 / fTemp133;
			let mut fTemp135: F32 = (fTemp134 + 1.41421354) / fTemp133 + 1.0;
			self.fRec133[0] = fTemp34 - (self.fRec133[2] * ((fTemp134 + -1.41421354) / fTemp133 + 1.0) + 2.0 * self.fRec133[1] * (1.0 - 1.0 / mydsp_faustpower2_f(fTemp133))) / fTemp135;
			self.iRec134[0] = (self.iRec134[1] + ((self.iRec134[1] > 0) as i32)) * ((fSlow9 <= self.fVec9[1]) as i32) + ((fSlow9 > self.fVec9[1]) as i32);
			let mut fTemp136: F32 = ((self.iRec134[0]) as F32) / F32::max(1.0, self.fConst7 * mydsp_faustpower2_f(1.0 - 0.219999999 * fTemp99));
			let mut fTemp137: F32 = 0.5 * ((self.fRec133[2] + self.fRec133[0] + 2.0 * self.fRec133[1]) * F32::max(0.0, F32::min(fTemp136, 2.0 - fTemp136))) / fTemp135;
			self.fRec135[0] = self.fRec102[1];
			self.fRec136[(self.IOTA0 & 2047) as usize] = -1.0 * 0.997305274 * (0.899999976 * self.fRec135[2] + 0.0500000007 * (self.fRec135[1] + self.fRec135[3]));
			let mut fTemp138: F32 = fTemp118 * fTemp119 * fTemp120;
			let mut fTemp139: F32 = fTemp107 * fTemp123 * fTemp124;
			let mut fTemp140: F32 = fTemp125 * fTemp128;
			self.fVec10[0] = fTemp132 * self.fRec136[((self.IOTA0 - (iTemp104 + 2)) & 2047) as usize] + fTemp115 * (fTemp138 * self.fRec136[((self.IOTA0 - (iTemp116 + 2)) & 2047) as usize] + 0.5 * fTemp139 * self.fRec136[((self.IOTA0 - (iTemp121 + 2)) & 2047) as usize] + 0.166666672 * fTemp140 * self.fRec136[((self.IOTA0 - (iTemp126 + 2)) & 2047) as usize] + 0.0416666679 * fTemp129 * self.fRec136[((self.IOTA0 - (iTemp130 + 2)) & 2047) as usize]);
			self.fRec140[0] = self.fConst8 * self.fRec140[1] + self.fConst9 * F32::abs(self.fRec101[1]);
			let mut fRec139: F32 = self.fRec140[0];
			let mut iTemp141: i32 = ((fRec139 > 0.100000001) as i32);
			self.iVec11[0] = iTemp141;
			self.iRec141[0] = std::cmp::max(((self.iConst10 * ((iTemp141 < self.iVec11[1]) as i32)) as i32), ((self.iRec141[1] + -1) as i32));
			let mut fTemp142: F32 = F32::abs(F32::max(((iTemp141) as F32), ((((self.iRec141[0] > 0) as i32)) as F32)));
			let mut fTemp143: F32 = if (((self.fRec137[1] > fTemp142) as i32) as i32 != 0) { self.fConst11 } else { self.fConst8 };
			self.fRec138[0] = self.fRec138[1] * fTemp143 + fTemp142 * (1.0 - fTemp143);
			self.fRec137[0] = self.fRec138[0];
			let mut fTemp144: F32 = 0.00499999989 * self.fRec137[0] * self.fRec101[1];
			let mut fTemp145: F32 = fTemp137 + self.fVec10[1] + fTemp144;
			self.fVec12[0] = fTemp145;
			self.fRec132[(self.IOTA0 & 2047) as usize] = 0.0500000007 * self.fRec132[((self.IOTA0 - 1) & 2047) as usize] + 0.949999988 * self.fVec12[2];
			let mut fRec129: F32 = fTemp132 * self.fRec132[((self.IOTA0 - iTemp104) & 2047) as usize] + fTemp115 * (fTemp138 * self.fRec132[((self.IOTA0 - iTemp116) & 2047) as usize] + 0.5 * fTemp139 * self.fRec132[((self.IOTA0 - iTemp121) & 2047) as usize] + 0.166666672 * fTemp140 * self.fRec132[((self.IOTA0 - iTemp126) & 2047) as usize] + 0.0416666679 * fTemp129 * self.fRec132[((self.IOTA0 - iTemp130) & 2047) as usize]);
			let mut fRec130: F32 = self.fVec12[1] + self.fRec123[1];
			self.fRec123[0] = fRec128;
			let mut fRec124: F32 = self.fRec123[1];
			let mut fRec125: F32 = fRec129;
			let mut fRec126: F32 = fRec130;
			self.fRec119[0] = fRec124;
			let mut fRec120: F32 = fTemp144 + fTemp137 + self.fRec119[1];
			let mut fRec121: F32 = fRec125;
			let mut fRec122: F32 = fRec126;
			self.fRec115[(self.IOTA0 & 2047) as usize] = fRec120;
			let mut fRec116: F32 = fTemp132 * self.fRec115[((self.IOTA0 - iTemp105) & 2047) as usize] + fTemp115 * (fTemp138 * self.fRec115[((self.IOTA0 - iTemp117) & 2047) as usize] + 0.5 * fTemp139 * self.fRec115[((self.IOTA0 - iTemp122) & 2047) as usize] + 0.166666672 * fTemp140 * self.fRec115[((self.IOTA0 - iTemp127) & 2047) as usize] + 0.0416666679 * fTemp129 * self.fRec115[((self.IOTA0 - iTemp131) & 2047) as usize]);
			self.fRec117[0] = fRec121;
			let mut fRec118: F32 = fRec122;
			self.fRec113[0] = fSlow5 * self.fRec117[1];
			let mut fRec114: F32 = fRec118;
			self.fRec108[0] = fRec112;
			let mut fRec109: F32 = fSlow5 * self.fRec108[1];
			let mut fRec110: F32 = self.fRec113[0];
			let mut fRec111: F32 = fRec114;
			self.fRec104[(self.IOTA0 & 2047) as usize] = fRec109;
			let mut fRec105: F32 = fRec116;
			let mut fRec106: F32 = fRec110;
			let mut fRec107: F32 = fRec111;
			self.fRec102[0] = fRec105;
			let mut fRec103: F32 = fRec107;
			let mut fTemp146: F32 = F32::abs(fRec103);
			let mut fTemp147: F32 = if (((self.fRec143[1] > fTemp146) as i32) as i32 != 0) { self.fConst12 } else { 0.0 };
			self.fRec144[0] = self.fRec144[1] * fTemp147 + fTemp146 * (1.0 - fTemp147);
			self.fRec143[0] = self.fRec144[0];
			let mut fRec142: F32 = 0.0 - 0.949999988 * F32::max(20.0 * F32::log10(self.fRec143[0]) + 10.0, 0.0);
			self.fRec101[0] = fRec103 * F32::powf(10.0, 0.0500000007 * fRec142);
			self.fRec98[0] = 0.0 - (self.fRec98[1] * (1.0 - fTemp100) - (self.fRec101[0] + self.fRec101[1])) / (fTemp100 + 1.0);
			self.fRec146[0] = fSlow12 + self.fConst2 * self.fRec146[1];
			let mut fTemp148: F32 = F32::powf(2.0, 0.0833333358 * (self.fRec146[0] + self.fRec3[0] + -69.0));
			let mut fTemp149: F32 = 1.0 / F32::tan(self.fConst3 * fTemp148);
			let mut fRec158: F32 = -1.0 * 0.997305274 * (0.899999976 * self.fRec159[2] + 0.0500000007 * (self.fRec159[1] + self.fRec159[3]));
			let mut fTemp150: F32 = self.fConst5 * (0.772727251 / fTemp148 + -0.109999999);
			let mut fTemp151: F32 = fTemp150 + -1.49999499;
			let mut iTemp152: i32 = ((fTemp151) as i32);
			let mut iTemp153: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, iTemp152)) as F32))) as i32);
			let mut iTemp154: i32 = iTemp153 + 1;
			let mut fTemp155: F32 = F32::floor(fTemp151);
			let mut fTemp156: F32 = fTemp150 + -1.0 - fTemp155;
			let mut fTemp157: F32 = 0.0 - fTemp156;
			let mut fTemp158: F32 = fTemp150 + -2.0 - fTemp155;
			let mut fTemp159: F32 = 0.0 - 0.5 * fTemp158;
			let mut fTemp160: F32 = fTemp150 + -3.0 - fTemp155;
			let mut fTemp161: F32 = 0.0 - 0.333333343 * fTemp160;
			let mut fTemp162: F32 = fTemp150 + -4.0 - fTemp155;
			let mut fTemp163: F32 = 0.0 - 0.25 * fTemp162;
			let mut fTemp164: F32 = fTemp150 - fTemp155;
			let mut iTemp165: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, iTemp152 + 1)) as F32))) as i32);
			let mut iTemp166: i32 = iTemp165 + 1;
			let mut fTemp167: F32 = 0.0 - fTemp158;
			let mut fTemp168: F32 = 0.0 - 0.5 * fTemp160;
			let mut fTemp169: F32 = 0.0 - 0.333333343 * fTemp162;
			let mut iTemp170: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, iTemp152 + 2)) as F32))) as i32);
			let mut iTemp171: i32 = iTemp170 + 1;
			let mut fTemp172: F32 = 0.0 - fTemp160;
			let mut fTemp173: F32 = 0.0 - 0.5 * fTemp162;
			let mut fTemp174: F32 = fTemp156 * fTemp158;
			let mut iTemp175: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, iTemp152 + 3)) as F32))) as i32);
			let mut iTemp176: i32 = iTemp175 + 1;
			let mut fTemp177: F32 = 0.0 - fTemp162;
			let mut fTemp178: F32 = fTemp174 * fTemp160;
			let mut iTemp179: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, iTemp152 + 4)) as F32))) as i32);
			let mut iTemp180: i32 = iTemp179 + 1;
			self.fRec173[0] = self.fRec150[((self.IOTA0 - iTemp154) & 2047) as usize] * fTemp157 * fTemp159 * fTemp161 * fTemp163 + fTemp164 * (self.fRec150[((self.IOTA0 - iTemp166) & 2047) as usize] * fTemp167 * fTemp168 * fTemp169 + 0.5 * fTemp156 * self.fRec150[((self.IOTA0 - iTemp171) & 2047) as usize] * fTemp172 * fTemp173 + 0.166666672 * fTemp174 * self.fRec150[((self.IOTA0 - iTemp176) & 2047) as usize] * fTemp177 + 0.0416666679 * fTemp178 * self.fRec150[((self.IOTA0 - iTemp180) & 2047) as usize]);
			self.fRec177[0] = 0.0500000007 * self.fRec177[1] + 0.949999988 * self.fRec173[1];
			let mut fRec174: F32 = self.fRec177[0];
			let mut fTemp181: F32 = fTemp157 * fTemp159 * fTemp161 * fTemp163;
			let mut fTemp182: F32 = F32::tan(self.fConst6 * fTemp148);
			let mut fTemp183: F32 = 1.0 / fTemp182;
			let mut fTemp184: F32 = (fTemp183 + 1.41421354) / fTemp182 + 1.0;
			self.fRec179[0] = fTemp34 - (self.fRec179[2] * ((fTemp183 + -1.41421354) / fTemp182 + 1.0) + 2.0 * self.fRec179[1] * (1.0 - 1.0 / mydsp_faustpower2_f(fTemp182))) / fTemp184;
			self.fVec13[0] = fSlow13;
			self.iRec180[0] = (self.iRec180[1] + ((self.iRec180[1] > 0) as i32)) * ((fSlow13 <= self.fVec13[1]) as i32) + ((fSlow13 > self.fVec13[1]) as i32);
			let mut fTemp185: F32 = ((self.iRec180[0]) as F32) / F32::max(1.0, self.fConst7 * mydsp_faustpower2_f(1.0 - 0.219999999 * fTemp148));
			let mut fTemp186: F32 = 0.5 * ((self.fRec179[2] + self.fRec179[0] + 2.0 * self.fRec179[1]) * F32::max(0.0, F32::min(fTemp185, 2.0 - fTemp185))) / fTemp184;
			self.fRec181[0] = self.fRec148[1];
			self.fRec182[(self.IOTA0 & 2047) as usize] = -1.0 * 0.997305274 * (0.899999976 * self.fRec181[2] + 0.0500000007 * (self.fRec181[1] + self.fRec181[3]));
			let mut fTemp187: F32 = fTemp167 * fTemp168 * fTemp169;
			let mut fTemp188: F32 = fTemp156 * fTemp172 * fTemp173;
			let mut fTemp189: F32 = fTemp174 * fTemp177;
			self.fVec14[0] = fTemp181 * self.fRec182[((self.IOTA0 - (iTemp153 + 2)) & 2047) as usize] + fTemp164 * (fTemp187 * self.fRec182[((self.IOTA0 - (iTemp165 + 2)) & 2047) as usize] + 0.5 * fTemp188 * self.fRec182[((self.IOTA0 - (iTemp170 + 2)) & 2047) as usize] + 0.166666672 * fTemp189 * self.fRec182[((self.IOTA0 - (iTemp175 + 2)) & 2047) as usize] + 0.0416666679 * fTemp178 * self.fRec182[((self.IOTA0 - (iTemp179 + 2)) & 2047) as usize]);
			self.fRec186[0] = self.fConst8 * self.fRec186[1] + self.fConst9 * F32::abs(self.fRec147[1]);
			let mut fRec185: F32 = self.fRec186[0];
			let mut iTemp190: i32 = ((fRec185 > 0.100000001) as i32);
			self.iVec15[0] = iTemp190;
			self.iRec187[0] = std::cmp::max(((self.iConst10 * ((iTemp190 < self.iVec15[1]) as i32)) as i32), ((self.iRec187[1] + -1) as i32));
			let mut fTemp191: F32 = F32::abs(F32::max(((iTemp190) as F32), ((((self.iRec187[0] > 0) as i32)) as F32)));
			let mut fTemp192: F32 = if (((self.fRec183[1] > fTemp191) as i32) as i32 != 0) { self.fConst11 } else { self.fConst8 };
			self.fRec184[0] = self.fRec184[1] * fTemp192 + fTemp191 * (1.0 - fTemp192);
			self.fRec183[0] = self.fRec184[0];
			let mut fTemp193: F32 = 0.00499999989 * self.fRec183[0] * self.fRec147[1];
			let mut fTemp194: F32 = fTemp186 + self.fVec14[1] + fTemp193;
			self.fVec16[0] = fTemp194;
			self.fRec178[(self.IOTA0 & 2047) as usize] = 0.0500000007 * self.fRec178[((self.IOTA0 - 1) & 2047) as usize] + 0.949999988 * self.fVec16[2];
			let mut fRec175: F32 = fTemp181 * self.fRec178[((self.IOTA0 - iTemp153) & 2047) as usize] + fTemp164 * (fTemp187 * self.fRec178[((self.IOTA0 - iTemp165) & 2047) as usize] + 0.5 * fTemp188 * self.fRec178[((self.IOTA0 - iTemp170) & 2047) as usize] + 0.166666672 * fTemp189 * self.fRec178[((self.IOTA0 - iTemp175) & 2047) as usize] + 0.0416666679 * fTemp178 * self.fRec178[((self.IOTA0 - iTemp179) & 2047) as usize]);
			let mut fRec176: F32 = self.fVec16[1] + self.fRec169[1];
			self.fRec169[0] = fRec174;
			let mut fRec170: F32 = self.fRec169[1];
			let mut fRec171: F32 = fRec175;
			let mut fRec172: F32 = fRec176;
			self.fRec165[0] = fRec170;
			let mut fRec166: F32 = fTemp186 + fTemp193 + self.fRec165[1];
			let mut fRec167: F32 = fRec171;
			let mut fRec168: F32 = fRec172;
			self.fRec161[(self.IOTA0 & 2047) as usize] = fRec166;
			let mut fRec162: F32 = fTemp181 * self.fRec161[((self.IOTA0 - iTemp154) & 2047) as usize] + fTemp164 * (fTemp187 * self.fRec161[((self.IOTA0 - iTemp166) & 2047) as usize] + 0.5 * fTemp188 * self.fRec161[((self.IOTA0 - iTemp171) & 2047) as usize] + 0.166666672 * fTemp189 * self.fRec161[((self.IOTA0 - iTemp176) & 2047) as usize] + 0.0416666679 * fTemp178 * self.fRec161[((self.IOTA0 - iTemp180) & 2047) as usize]);
			self.fRec163[0] = fRec167;
			let mut fRec164: F32 = fRec168;
			self.fRec159[0] = fSlow5 * self.fRec163[1];
			let mut fRec160: F32 = fRec164;
			self.fRec154[0] = fRec158;
			let mut fRec155: F32 = fSlow5 * self.fRec154[1];
			let mut fRec156: F32 = self.fRec159[0];
			let mut fRec157: F32 = fRec160;
			self.fRec150[(self.IOTA0 & 2047) as usize] = fRec155;
			let mut fRec151: F32 = fRec162;
			let mut fRec152: F32 = fRec156;
			let mut fRec153: F32 = fRec157;
			self.fRec148[0] = fRec151;
			let mut fRec149: F32 = fRec153;
			let mut fTemp195: F32 = F32::abs(fRec149);
			let mut fTemp196: F32 = if (((self.fRec189[1] > fTemp195) as i32) as i32 != 0) { self.fConst12 } else { 0.0 };
			self.fRec190[0] = self.fRec190[1] * fTemp196 + fTemp195 * (1.0 - fTemp196);
			self.fRec189[0] = self.fRec190[0];
			let mut fRec188: F32 = 0.0 - 0.949999988 * F32::max(20.0 * F32::log10(self.fRec189[0]) + 10.0, 0.0);
			self.fRec147[0] = fRec149 * F32::powf(10.0, 0.0500000007 * fRec188);
			self.fRec145[0] = 0.0 - (self.fRec145[1] * (1.0 - fTemp149) - (self.fRec147[0] + self.fRec147[1])) / (fTemp149 + 1.0);
			self.fVec17[0] = fSlow14;
			self.fRec193[0] = if (iSlow15 as i32 != 0) { fSlow16 } else { self.fRec193[1] };
			self.fRec192[0] = self.fConst2 * self.fRec192[1] + self.fConst1 * self.fRec193[0];
			let mut fTemp197: F32 = F32::powf(2.0, 0.0833333358 * (self.fRec3[0] + self.fRec192[0] + -69.0));
			let mut fTemp198: F32 = 1.0 / F32::tan(self.fConst3 * fTemp197);
			let mut fRec205: F32 = -1.0 * 0.997305274 * (0.899999976 * self.fRec206[2] + 0.0500000007 * (self.fRec206[1] + self.fRec206[3]));
			let mut fTemp199: F32 = self.fConst5 * (0.772727251 / fTemp197 + -0.109999999);
			let mut fTemp200: F32 = fTemp199 + -1.49999499;
			let mut iTemp201: i32 = ((fTemp200) as i32);
			let mut iTemp202: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, iTemp201)) as F32))) as i32);
			let mut iTemp203: i32 = iTemp202 + 1;
			let mut fTemp204: F32 = F32::floor(fTemp200);
			let mut fTemp205: F32 = fTemp199 + -1.0 - fTemp204;
			let mut fTemp206: F32 = 0.0 - fTemp205;
			let mut fTemp207: F32 = fTemp199 + -2.0 - fTemp204;
			let mut fTemp208: F32 = 0.0 - 0.5 * fTemp207;
			let mut fTemp209: F32 = fTemp199 + -3.0 - fTemp204;
			let mut fTemp210: F32 = 0.0 - 0.333333343 * fTemp209;
			let mut fTemp211: F32 = fTemp199 + -4.0 - fTemp204;
			let mut fTemp212: F32 = 0.0 - 0.25 * fTemp211;
			let mut fTemp213: F32 = fTemp199 - fTemp204;
			let mut iTemp214: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, iTemp201 + 1)) as F32))) as i32);
			let mut iTemp215: i32 = iTemp214 + 1;
			let mut fTemp216: F32 = 0.0 - fTemp207;
			let mut fTemp217: F32 = 0.0 - 0.5 * fTemp209;
			let mut fTemp218: F32 = 0.0 - 0.333333343 * fTemp211;
			let mut iTemp219: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, iTemp201 + 2)) as F32))) as i32);
			let mut iTemp220: i32 = iTemp219 + 1;
			let mut fTemp221: F32 = 0.0 - fTemp209;
			let mut fTemp222: F32 = 0.0 - 0.5 * fTemp211;
			let mut fTemp223: F32 = fTemp205 * fTemp207;
			let mut iTemp224: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, iTemp201 + 3)) as F32))) as i32);
			let mut iTemp225: i32 = iTemp224 + 1;
			let mut fTemp226: F32 = 0.0 - fTemp211;
			let mut fTemp227: F32 = fTemp223 * fTemp209;
			let mut iTemp228: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, iTemp201 + 4)) as F32))) as i32);
			let mut iTemp229: i32 = iTemp228 + 1;
			self.fRec220[0] = self.fRec197[((self.IOTA0 - iTemp203) & 2047) as usize] * fTemp206 * fTemp208 * fTemp210 * fTemp212 + fTemp213 * (self.fRec197[((self.IOTA0 - iTemp215) & 2047) as usize] * fTemp216 * fTemp217 * fTemp218 + 0.5 * fTemp205 * self.fRec197[((self.IOTA0 - iTemp220) & 2047) as usize] * fTemp221 * fTemp222 + 0.166666672 * fTemp223 * self.fRec197[((self.IOTA0 - iTemp225) & 2047) as usize] * fTemp226 + 0.0416666679 * fTemp227 * self.fRec197[((self.IOTA0 - iTemp229) & 2047) as usize]);
			self.fRec224[0] = 0.0500000007 * self.fRec224[1] + 0.949999988 * self.fRec220[1];
			let mut fRec221: F32 = self.fRec224[0];
			let mut fTemp230: F32 = fTemp206 * fTemp208 * fTemp210 * fTemp212;
			let mut fTemp231: F32 = F32::tan(self.fConst6 * fTemp197);
			let mut fTemp232: F32 = 1.0 / fTemp231;
			let mut fTemp233: F32 = (fTemp232 + 1.41421354) / fTemp231 + 1.0;
			self.fRec226[0] = fTemp34 - (self.fRec226[2] * ((fTemp232 + -1.41421354) / fTemp231 + 1.0) + 2.0 * self.fRec226[1] * (1.0 - 1.0 / mydsp_faustpower2_f(fTemp231))) / fTemp233;
			self.iRec227[0] = (self.iRec227[1] + ((self.iRec227[1] > 0) as i32)) * ((fSlow14 <= self.fVec17[1]) as i32) + ((fSlow14 > self.fVec17[1]) as i32);
			let mut fTemp234: F32 = ((self.iRec227[0]) as F32) / F32::max(1.0, self.fConst7 * mydsp_faustpower2_f(1.0 - 0.219999999 * fTemp197));
			let mut fTemp235: F32 = 0.5 * ((self.fRec226[2] + self.fRec226[0] + 2.0 * self.fRec226[1]) * F32::max(0.0, F32::min(fTemp234, 2.0 - fTemp234))) / fTemp233;
			self.fRec228[0] = self.fRec195[1];
			self.fRec229[(self.IOTA0 & 2047) as usize] = -1.0 * 0.997305274 * (0.899999976 * self.fRec228[2] + 0.0500000007 * (self.fRec228[1] + self.fRec228[3]));
			let mut fTemp236: F32 = fTemp216 * fTemp217 * fTemp218;
			let mut fTemp237: F32 = fTemp205 * fTemp221 * fTemp222;
			let mut fTemp238: F32 = fTemp223 * fTemp226;
			self.fVec18[0] = fTemp230 * self.fRec229[((self.IOTA0 - (iTemp202 + 2)) & 2047) as usize] + fTemp213 * (fTemp236 * self.fRec229[((self.IOTA0 - (iTemp214 + 2)) & 2047) as usize] + 0.5 * fTemp237 * self.fRec229[((self.IOTA0 - (iTemp219 + 2)) & 2047) as usize] + 0.166666672 * fTemp238 * self.fRec229[((self.IOTA0 - (iTemp224 + 2)) & 2047) as usize] + 0.0416666679 * fTemp227 * self.fRec229[((self.IOTA0 - (iTemp228 + 2)) & 2047) as usize]);
			self.fRec233[0] = self.fConst8 * self.fRec233[1] + self.fConst9 * F32::abs(self.fRec194[1]);
			let mut fRec232: F32 = self.fRec233[0];
			let mut iTemp239: i32 = ((fRec232 > 0.100000001) as i32);
			self.iVec19[0] = iTemp239;
			self.iRec234[0] = std::cmp::max(((self.iConst10 * ((iTemp239 < self.iVec19[1]) as i32)) as i32), ((self.iRec234[1] + -1) as i32));
			let mut fTemp240: F32 = F32::abs(F32::max(((iTemp239) as F32), ((((self.iRec234[0] > 0) as i32)) as F32)));
			let mut fTemp241: F32 = if (((self.fRec230[1] > fTemp240) as i32) as i32 != 0) { self.fConst11 } else { self.fConst8 };
			self.fRec231[0] = self.fRec231[1] * fTemp241 + fTemp240 * (1.0 - fTemp241);
			self.fRec230[0] = self.fRec231[0];
			let mut fTemp242: F32 = 0.00499999989 * self.fRec230[0] * self.fRec194[1];
			let mut fTemp243: F32 = fTemp235 + self.fVec18[1] + fTemp242;
			self.fVec20[0] = fTemp243;
			self.fRec225[(self.IOTA0 & 2047) as usize] = 0.0500000007 * self.fRec225[((self.IOTA0 - 1) & 2047) as usize] + 0.949999988 * self.fVec20[2];
			let mut fRec222: F32 = fTemp230 * self.fRec225[((self.IOTA0 - iTemp202) & 2047) as usize] + fTemp213 * (fTemp236 * self.fRec225[((self.IOTA0 - iTemp214) & 2047) as usize] + 0.5 * fTemp237 * self.fRec225[((self.IOTA0 - iTemp219) & 2047) as usize] + 0.166666672 * fTemp238 * self.fRec225[((self.IOTA0 - iTemp224) & 2047) as usize] + 0.0416666679 * fTemp227 * self.fRec225[((self.IOTA0 - iTemp228) & 2047) as usize]);
			let mut fRec223: F32 = self.fVec20[1] + self.fRec216[1];
			self.fRec216[0] = fRec221;
			let mut fRec217: F32 = self.fRec216[1];
			let mut fRec218: F32 = fRec222;
			let mut fRec219: F32 = fRec223;
			self.fRec212[0] = fRec217;
			let mut fRec213: F32 = fTemp242 + fTemp235 + self.fRec212[1];
			let mut fRec214: F32 = fRec218;
			let mut fRec215: F32 = fRec219;
			self.fRec208[(self.IOTA0 & 2047) as usize] = fRec213;
			let mut fRec209: F32 = fTemp230 * self.fRec208[((self.IOTA0 - iTemp203) & 2047) as usize] + fTemp213 * (fTemp236 * self.fRec208[((self.IOTA0 - iTemp215) & 2047) as usize] + 0.5 * fTemp237 * self.fRec208[((self.IOTA0 - iTemp220) & 2047) as usize] + 0.166666672 * fTemp238 * self.fRec208[((self.IOTA0 - iTemp225) & 2047) as usize] + 0.0416666679 * fTemp227 * self.fRec208[((self.IOTA0 - iTemp229) & 2047) as usize]);
			self.fRec210[0] = fRec214;
			let mut fRec211: F32 = fRec215;
			self.fRec206[0] = fSlow5 * self.fRec210[1];
			let mut fRec207: F32 = fRec211;
			self.fRec201[0] = fRec205;
			let mut fRec202: F32 = fSlow5 * self.fRec201[1];
			let mut fRec203: F32 = self.fRec206[0];
			let mut fRec204: F32 = fRec207;
			self.fRec197[(self.IOTA0 & 2047) as usize] = fRec202;
			let mut fRec198: F32 = fRec209;
			let mut fRec199: F32 = fRec203;
			let mut fRec200: F32 = fRec204;
			self.fRec195[0] = fRec198;
			let mut fRec196: F32 = fRec200;
			let mut fTemp244: F32 = F32::abs(fRec196);
			let mut fTemp245: F32 = if (((self.fRec236[1] > fTemp244) as i32) as i32 != 0) { self.fConst12 } else { 0.0 };
			self.fRec237[0] = self.fRec237[1] * fTemp245 + fTemp244 * (1.0 - fTemp245);
			self.fRec236[0] = self.fRec237[0];
			let mut fRec235: F32 = 0.0 - 0.949999988 * F32::max(20.0 * F32::log10(self.fRec236[0]) + 10.0, 0.0);
			self.fRec194[0] = fRec196 * F32::powf(10.0, 0.0500000007 * fRec235);
			self.fRec191[0] = 0.0 - (self.fRec191[1] * (1.0 - fTemp198) - (self.fRec194[0] + self.fRec194[1])) / (fTemp198 + 1.0);
			self.fRec238[0] = fSlow17 + self.fConst2 * self.fRec238[1];
			self.fRec239[0] = fSlow18 + self.fConst2 * self.fRec239[1];
			self.fRec242[0] = fSlow19 + self.fConst2 * self.fRec242[1];
			self.fRec245[0] = fSlow21 + self.fConst2 * self.fRec245[1];
			let mut fTemp246: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec245[0]));
			let mut fTemp247: F32 = self.fConst13 * fTemp246;
			let mut fTemp248: F32 = self.fRec243[1] + fTemp247;
			let mut fTemp249: F32 = fTemp248 + -1.0;
			let mut iTemp250: i32 = ((fTemp249 < 0.0) as i32);
			self.fRec243[0] = if (iTemp250 as i32 != 0) { fTemp248 } else { fTemp249 };
			let mut fThen15: F32 = fTemp247 + self.fRec243[1] + (1.0 - self.fConst0 / fTemp246) * fTemp249;
			let mut fRec244: F32 = if (iTemp250 as i32 != 0) { fTemp248 } else { fThen15 };
			self.fRec246[0] = fSlow22 + self.fConst2 * self.fRec246[1];
			let mut fTemp251: F32 = F32::min(1.41419947, 1.41421354 * self.fRec246[0]);
			let mut fTemp252: F32 = fTemp251 * (fTemp251 + 1.41421354);
			self.fRec248[0] = fSlow23 + self.fConst2 * self.fRec248[1];
			let mut fTemp253: F32 = self.fRec248[0] + -69.0;
			self.fRec247[0] = self.fConst2 * self.fRec247[1] + self.fConst14 * F32::powf(2.0, 0.0833333358 * (fSlow20 + fTemp253));
			let mut fTemp254: F32 = F32::tan(self.fConst15 * F32::max(20.0, F32::min(10000.0, self.fRec247[0])));
			let mut fTemp255: F32 = 1.0 / fTemp254;
			let mut fTemp256: F32 = 1.41421354 * fTemp251;
			let mut fTemp257: F32 = fTemp256 + 2.0;
			let mut fTemp258: F32 = 1.0 - 1.0 / mydsp_faustpower2_f(fTemp254);
			let mut fTemp259: F32 = fTemp252 + (fTemp255 + fTemp257) / fTemp254 + 1.0;
			self.fRec241[0] = self.fRec242[0] * (2.0 * fRec244 + -1.0) - (self.fRec241[2] * (fTemp252 + (fTemp255 - fTemp257) / fTemp254 + 1.0) + 2.0 * self.fRec241[1] * (fTemp252 + fTemp258)) / fTemp259;
			let mut fTemp260: F32 = fTemp251 * (fTemp251 + -1.41421354);
			let mut fTemp261: F32 = 2.0 - fTemp256;
			let mut fTemp262: F32 = fTemp260 + (fTemp261 + fTemp255) / fTemp254 + 1.0;
			self.fRec240[0] = (self.fRec241[2] + self.fRec241[0] + 2.0 * self.fRec241[1]) / fTemp259 - (self.fRec240[2] * (fTemp260 + (fTemp255 - fTemp261) / fTemp254 + 1.0) + 2.0 * self.fRec240[1] * (fTemp260 + fTemp258)) / fTemp262;
			self.fRec251[0] = fSlow24 + self.fConst2 * self.fRec251[1];
			self.fRec254[0] = fSlow26 + self.fConst2 * self.fRec254[1];
			let mut fTemp263: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec254[0]));
			let mut fTemp264: F32 = self.fConst13 * fTemp263;
			let mut fTemp265: F32 = self.fRec252[1] + fTemp264;
			let mut fTemp266: F32 = fTemp265 + -1.0;
			let mut iTemp267: i32 = ((fTemp266 < 0.0) as i32);
			self.fRec252[0] = if (iTemp267 as i32 != 0) { fTemp265 } else { fTemp266 };
			let mut fThen17: F32 = self.fRec252[1] + fTemp264 + (1.0 - self.fConst0 / fTemp263) * fTemp266;
			let mut fRec253: F32 = if (iTemp267 as i32 != 0) { fTemp265 } else { fThen17 };
			self.fRec255[0] = self.fConst2 * self.fRec255[1] + self.fConst14 * F32::powf(2.0, 0.0833333358 * (fSlow25 + fTemp253));
			let mut fTemp268: F32 = F32::tan(self.fConst15 * F32::max(20.0, F32::min(10000.0, self.fRec255[0])));
			let mut fTemp269: F32 = 1.0 / fTemp268;
			let mut fTemp270: F32 = 1.0 - 1.0 / mydsp_faustpower2_f(fTemp268);
			let mut fTemp271: F32 = fTemp252 + (fTemp257 + fTemp269) / fTemp268 + 1.0;
			self.fRec250[0] = self.fRec251[0] * (2.0 * fRec253 + -1.0) - (self.fRec250[2] * (fTemp252 + (fTemp269 - fTemp257) / fTemp268 + 1.0) + 2.0 * self.fRec250[1] * (fTemp252 + fTemp270)) / fTemp271;
			let mut fTemp272: F32 = fTemp260 + (fTemp261 + fTemp269) / fTemp268 + 1.0;
			self.fRec249[0] = (self.fRec250[2] + self.fRec250[0] + 2.0 * self.fRec250[1]) / fTemp271 - (self.fRec249[2] * (fTemp260 + (fTemp269 - fTemp261) / fTemp268 + 1.0) + 2.0 * self.fRec249[1] * (fTemp260 + fTemp270)) / fTemp272;
			self.fRec258[0] = fSlow27 + self.fConst2 * self.fRec258[1];
			self.fRec261[0] = fSlow29 + self.fConst2 * self.fRec261[1];
			let mut fTemp273: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec261[0]));
			let mut fTemp274: F32 = self.fConst13 * fTemp273;
			let mut fTemp275: F32 = self.fRec259[1] + fTemp274;
			let mut fTemp276: F32 = fTemp275 + -1.0;
			let mut iTemp277: i32 = ((fTemp276 < 0.0) as i32);
			self.fRec259[0] = if (iTemp277 as i32 != 0) { fTemp275 } else { fTemp276 };
			let mut fThen19: F32 = self.fRec259[1] + fTemp274 + (1.0 - self.fConst0 / fTemp273) * fTemp276;
			let mut fRec260: F32 = if (iTemp277 as i32 != 0) { fTemp275 } else { fThen19 };
			self.fRec262[0] = self.fConst2 * self.fRec262[1] + self.fConst14 * F32::powf(2.0, 0.0833333358 * (fSlow28 + fTemp253));
			let mut fTemp278: F32 = F32::tan(self.fConst15 * F32::max(20.0, F32::min(10000.0, self.fRec262[0])));
			let mut fTemp279: F32 = 1.0 / fTemp278;
			let mut fTemp280: F32 = 1.0 - 1.0 / mydsp_faustpower2_f(fTemp278);
			let mut fTemp281: F32 = fTemp252 + (fTemp257 + fTemp279) / fTemp278 + 1.0;
			self.fRec257[0] = self.fRec258[0] * (2.0 * fRec260 + -1.0) - (self.fRec257[2] * (fTemp252 + (fTemp279 - fTemp257) / fTemp278 + 1.0) + 2.0 * self.fRec257[1] * (fTemp252 + fTemp280)) / fTemp281;
			let mut fTemp282: F32 = fTemp260 + (fTemp261 + fTemp279) / fTemp278 + 1.0;
			self.fRec256[0] = (self.fRec257[2] + self.fRec257[0] + 2.0 * self.fRec257[1]) / fTemp281 - (self.fRec256[2] * (fTemp260 + (fTemp279 - fTemp261) / fTemp278 + 1.0) + 2.0 * self.fRec256[1] * (fTemp260 + fTemp280)) / fTemp282;
			self.fRec265[0] = fSlow30 + self.fConst2 * self.fRec265[1];
			self.fRec268[0] = fSlow32 + self.fConst2 * self.fRec268[1];
			let mut fTemp283: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec268[0]));
			let mut fTemp284: F32 = self.fRec266[1] + self.fConst13 * fTemp283;
			let mut fTemp285: F32 = fTemp284 + -1.0;
			let mut iTemp286: i32 = ((fTemp285 < 0.0) as i32);
			self.fRec266[0] = if (iTemp286 as i32 != 0) { fTemp284 } else { fTemp285 };
			let mut fThen21: F32 = fTemp284 + (1.0 - self.fConst0 / fTemp283) * fTemp285;
			let mut fRec267: F32 = if (iTemp286 as i32 != 0) { fTemp284 } else { fThen21 };
			self.fRec269[0] = self.fConst2 * self.fRec269[1] + self.fConst14 * F32::powf(2.0, 0.0833333358 * (fSlow31 + fTemp253));
			let mut fTemp287: F32 = F32::tan(self.fConst15 * F32::max(20.0, F32::min(10000.0, self.fRec269[0])));
			let mut fTemp288: F32 = 1.0 / fTemp287;
			let mut fTemp289: F32 = 1.0 - 1.0 / mydsp_faustpower2_f(fTemp287);
			let mut fTemp290: F32 = fTemp252 + (fTemp257 + fTemp288) / fTemp287 + 1.0;
			self.fRec264[0] = self.fRec265[0] * (2.0 * fRec267 + -1.0) - (self.fRec264[2] * (fTemp252 + (fTemp288 - fTemp257) / fTemp287 + 1.0) + 2.0 * self.fRec264[1] * (fTemp252 + fTemp289)) / fTemp290;
			let mut fTemp291: F32 = fTemp260 + (fTemp261 + fTemp288) / fTemp287 + 1.0;
			self.fRec263[0] = (self.fRec264[2] + self.fRec264[0] + 2.0 * self.fRec264[1]) / fTemp290 - (self.fRec263[2] * (fTemp260 + (fTemp288 - fTemp261) / fTemp287 + 1.0) + 2.0 * self.fRec263[1] * (fTemp260 + fTemp289)) / fTemp291;
			self.fRec270[0] = fSlow33 + self.fConst2 * self.fRec270[1];
			self.fRec271[0] = fSlow34 + self.fConst2 * self.fRec271[1];
			let mut fTemp292: F32 = ((self.iVec0[1]) as F32);
			self.fRec274[0] = fSlow35 + self.fConst2 * self.fRec274[1];
			let mut fTemp293: F32 = F32::powf(2.0, 0.0833333358 * (self.fRec274[0] + -69.0));
			let mut fTemp294: F32 = F32::max(440.0 * fTemp293, 23.4489498);
			let mut fTemp295: F32 = F32::max(20.0, F32::abs(fTemp294));
			let mut fTemp296: F32 = self.fRec273[1] + self.fConst13 * fTemp295;
			self.fRec273[0] = fTemp296 - F32::floor(fTemp296);
			let mut fTemp297: F32 = mydsp_faustpower2_f(2.0 * self.fRec273[0] + -1.0);
			self.fVec21[0] = fTemp297;
			let mut fTemp298: F32 = (fTemp292 * (fTemp297 - self.fVec21[1])) / fTemp295;
			self.fVec22[(self.IOTA0 & 4095) as usize] = fTemp298;
			let mut fTemp299: F32 = F32::max(0.0, F32::min(2047.0, self.fConst19 / fTemp294));
			let mut iTemp300: i32 = ((fTemp299) as i32);
			let mut fTemp301: F32 = F32::floor(fTemp299);
			self.fRec272[0] = 0.999000013 * self.fRec272[1] + self.fConst18 * (fTemp298 - self.fVec22[((self.IOTA0 - iTemp300) & 4095) as usize] * (fTemp301 + 1.0 - fTemp299) - (fTemp299 - fTemp301) * self.fVec22[((self.IOTA0 - (iTemp300 + 1)) & 4095) as usize]);
			let mut fTemp302: F32 = F32::powf(2.0, 0.0833333358 * (self.fRec274[0] + -56.9000015));
			let mut fTemp303: F32 = F32::max(440.0 * fTemp302, 23.4489498);
			let mut fTemp304: F32 = F32::max(0.0, F32::min(2047.0, self.fConst19 / fTemp303));
			let mut fTemp305: F32 = F32::floor(fTemp304);
			let mut fTemp306: F32 = F32::max(20.0, F32::abs(fTemp303));
			let mut fTemp307: F32 = self.fRec276[1] + self.fConst13 * fTemp306;
			self.fRec276[0] = fTemp307 - F32::floor(fTemp307);
			let mut fTemp308: F32 = mydsp_faustpower2_f(2.0 * self.fRec276[0] + -1.0);
			self.fVec23[0] = fTemp308;
			let mut fTemp309: F32 = (fTemp292 * (fTemp308 - self.fVec23[1])) / fTemp306;
			self.fVec24[(self.IOTA0 & 4095) as usize] = fTemp309;
			let mut iTemp310: i32 = ((fTemp304) as i32);
			self.fRec275[0] = 0.999000013 * self.fRec275[1] - self.fConst18 * ((fTemp304 - fTemp305) * self.fVec24[((self.IOTA0 - (iTemp310 + 1)) & 4095) as usize] - fTemp309 - self.fVec24[((self.IOTA0 - iTemp310) & 4095) as usize] * (fTemp305 + 1.0 - fTemp304));
			let mut fTemp311: F32 = F32::powf(2.0, 0.0833333358 * (self.fRec274[0] + -81.1100006));
			let mut fTemp312: F32 = F32::max(440.0 * fTemp311, 23.4489498);
			let mut fTemp313: F32 = F32::max(20.0, F32::abs(fTemp312));
			let mut fTemp314: F32 = self.fRec278[1] + self.fConst13 * fTemp313;
			self.fRec278[0] = fTemp314 - F32::floor(fTemp314);
			let mut fTemp315: F32 = mydsp_faustpower2_f(2.0 * self.fRec278[0] + -1.0);
			self.fVec25[0] = fTemp315;
			let mut fTemp316: F32 = (fTemp292 * (fTemp315 - self.fVec25[1])) / fTemp313;
			self.fVec26[(self.IOTA0 & 4095) as usize] = fTemp316;
			let mut fTemp317: F32 = F32::max(0.0, F32::min(2047.0, self.fConst19 / fTemp312));
			let mut iTemp318: i32 = ((fTemp317) as i32);
			let mut fTemp319: F32 = F32::floor(fTemp317);
			self.fRec277[0] = 0.999000013 * self.fRec277[1] - self.fConst18 * (self.fVec26[((self.IOTA0 - iTemp318) & 4095) as usize] * (fTemp319 + 1.0 - fTemp317) - fTemp316 + (fTemp317 - fTemp319) * self.fVec26[((self.IOTA0 - (iTemp318 + 1)) & 4095) as usize]);
			let mut fTemp320: F32 = F32::powf(2.0, 0.0833333358 * (self.fRec274[0] + -61.8800011));
			let mut fTemp321: F32 = F32::max(440.0 * fTemp320, 23.4489498);
			let mut fTemp322: F32 = F32::max(20.0, F32::abs(fTemp321));
			let mut fTemp323: F32 = self.fRec280[1] + self.fConst13 * fTemp322;
			self.fRec280[0] = fTemp323 - F32::floor(fTemp323);
			let mut fTemp324: F32 = mydsp_faustpower2_f(2.0 * self.fRec280[0] + -1.0);
			self.fVec27[0] = fTemp324;
			let mut fTemp325: F32 = (fTemp292 * (fTemp324 - self.fVec27[1])) / fTemp322;
			self.fVec28[(self.IOTA0 & 4095) as usize] = fTemp325;
			let mut fTemp326: F32 = F32::max(0.0, F32::min(2047.0, self.fConst19 / fTemp321));
			let mut iTemp327: i32 = ((fTemp326) as i32);
			let mut fTemp328: F32 = F32::floor(fTemp326);
			self.fRec279[0] = 0.999000013 * self.fRec279[1] - self.fConst18 * (self.fVec28[((self.IOTA0 - iTemp327) & 4095) as usize] * (fTemp328 + 1.0 - fTemp326) - fTemp325 + (fTemp326 - fTemp328) * self.fVec28[((self.IOTA0 - (iTemp327 + 1)) & 4095) as usize]);
			self.fRec0[(self.IOTA0 & 65535) as usize] = self.fRec1[0] * (self.fRec2[0] + self.fRec51[0] + self.fRec98[0] + self.fRec145[0] + self.fRec191[0]) + self.fRec238[0] * self.fRec239[0] * ((self.fRec240[2] + self.fRec240[0] + 2.0 * self.fRec240[1]) / fTemp262 + (self.fRec249[2] + self.fRec249[0] + 2.0 * self.fRec249[1]) / fTemp272 + (self.fRec256[2] + self.fRec256[0] + 2.0 * self.fRec256[1]) / fTemp282 + (self.fRec263[2] + self.fRec263[0] + 2.0 * self.fRec263[1]) / fTemp291) + 0.300000012 * self.fRec0[((self.IOTA0 - self.iConst16) & 65535) as usize] + self.fConst17 * self.fRec270[0] * self.fRec271[0] * (self.fRec272[0] * fTemp293 + self.fRec275[0] * fTemp302 + self.fRec277[0] * fTemp311 + self.fRec279[0] * fTemp320);
			self.fRec281[0] = fSlow36 + self.fConst2 * self.fRec281[1];
			let mut fTemp329: F32 = self.fRec0[(self.IOTA0 & 65535) as usize] * self.fRec281[0];
			*output0 = ((fTemp329) as F32);
			*output1 = ((fTemp329) as F32);
			self.iVec0[1] = self.iVec0[0];
			self.fRec1[1] = self.fRec1[0];
			self.fRec3[1] = self.fRec3[0];
			self.fVec1[1] = self.fVec1[0];
			self.fRec5[1] = self.fRec5[0];
			self.fRec4[1] = self.fRec4[0];
			self.IOTA0 = self.IOTA0 + 1;
			self.fRec32[1] = self.fRec32[0];
			self.fRec36[1] = self.fRec36[0];
			self.iRec39[1] = self.iRec39[0];
			self.fRec38[2] = self.fRec38[1];
			self.fRec38[1] = self.fRec38[0];
			self.iRec40[1] = self.iRec40[0];
			for j0 in (1..=3).rev() {
				self.fRec41[(j0) as usize] = self.fRec41[(j0 - 1) as usize];
			}
			self.fVec2[1] = self.fVec2[0];
			self.fRec46[1] = self.fRec46[0];
			self.iVec3[1] = self.iVec3[0];
			self.iRec47[1] = self.iRec47[0];
			self.fRec44[1] = self.fRec44[0];
			self.fRec43[1] = self.fRec43[0];
			self.fVec4[2] = self.fVec4[1];
			self.fVec4[1] = self.fVec4[0];
			self.fRec28[1] = self.fRec28[0];
			self.fRec24[1] = self.fRec24[0];
			self.fRec22[1] = self.fRec22[0];
			for j1 in (1..=3).rev() {
				self.fRec18[(j1) as usize] = self.fRec18[(j1 - 1) as usize];
			}
			self.fRec13[1] = self.fRec13[0];
			self.fRec7[1] = self.fRec7[0];
			self.fRec50[1] = self.fRec50[0];
			self.fRec49[1] = self.fRec49[0];
			self.fRec6[1] = self.fRec6[0];
			self.fRec2[1] = self.fRec2[0];
			self.fVec5[1] = self.fVec5[0];
			self.fRec53[1] = self.fRec53[0];
			self.fRec52[1] = self.fRec52[0];
			self.fRec80[1] = self.fRec80[0];
			self.fRec84[1] = self.fRec84[0];
			self.fRec86[2] = self.fRec86[1];
			self.fRec86[1] = self.fRec86[0];
			self.iRec87[1] = self.iRec87[0];
			for j2 in (1..=3).rev() {
				self.fRec88[(j2) as usize] = self.fRec88[(j2 - 1) as usize];
			}
			self.fVec6[1] = self.fVec6[0];
			self.fRec93[1] = self.fRec93[0];
			self.iVec7[1] = self.iVec7[0];
			self.iRec94[1] = self.iRec94[0];
			self.fRec91[1] = self.fRec91[0];
			self.fRec90[1] = self.fRec90[0];
			self.fVec8[2] = self.fVec8[1];
			self.fVec8[1] = self.fVec8[0];
			self.fRec76[1] = self.fRec76[0];
			self.fRec72[1] = self.fRec72[0];
			self.fRec70[1] = self.fRec70[0];
			for j3 in (1..=3).rev() {
				self.fRec66[(j3) as usize] = self.fRec66[(j3 - 1) as usize];
			}
			self.fRec61[1] = self.fRec61[0];
			self.fRec55[1] = self.fRec55[0];
			self.fRec97[1] = self.fRec97[0];
			self.fRec96[1] = self.fRec96[0];
			self.fRec54[1] = self.fRec54[0];
			self.fRec51[1] = self.fRec51[0];
			self.fVec9[1] = self.fVec9[0];
			self.fRec100[1] = self.fRec100[0];
			self.fRec99[1] = self.fRec99[0];
			self.fRec127[1] = self.fRec127[0];
			self.fRec131[1] = self.fRec131[0];
			self.fRec133[2] = self.fRec133[1];
			self.fRec133[1] = self.fRec133[0];
			self.iRec134[1] = self.iRec134[0];
			for j4 in (1..=3).rev() {
				self.fRec135[(j4) as usize] = self.fRec135[(j4 - 1) as usize];
			}
			self.fVec10[1] = self.fVec10[0];
			self.fRec140[1] = self.fRec140[0];
			self.iVec11[1] = self.iVec11[0];
			self.iRec141[1] = self.iRec141[0];
			self.fRec138[1] = self.fRec138[0];
			self.fRec137[1] = self.fRec137[0];
			self.fVec12[2] = self.fVec12[1];
			self.fVec12[1] = self.fVec12[0];
			self.fRec123[1] = self.fRec123[0];
			self.fRec119[1] = self.fRec119[0];
			self.fRec117[1] = self.fRec117[0];
			for j5 in (1..=3).rev() {
				self.fRec113[(j5) as usize] = self.fRec113[(j5 - 1) as usize];
			}
			self.fRec108[1] = self.fRec108[0];
			self.fRec102[1] = self.fRec102[0];
			self.fRec144[1] = self.fRec144[0];
			self.fRec143[1] = self.fRec143[0];
			self.fRec101[1] = self.fRec101[0];
			self.fRec98[1] = self.fRec98[0];
			self.fRec146[1] = self.fRec146[0];
			self.fRec173[1] = self.fRec173[0];
			self.fRec177[1] = self.fRec177[0];
			self.fRec179[2] = self.fRec179[1];
			self.fRec179[1] = self.fRec179[0];
			self.fVec13[1] = self.fVec13[0];
			self.iRec180[1] = self.iRec180[0];
			for j6 in (1..=3).rev() {
				self.fRec181[(j6) as usize] = self.fRec181[(j6 - 1) as usize];
			}
			self.fVec14[1] = self.fVec14[0];
			self.fRec186[1] = self.fRec186[0];
			self.iVec15[1] = self.iVec15[0];
			self.iRec187[1] = self.iRec187[0];
			self.fRec184[1] = self.fRec184[0];
			self.fRec183[1] = self.fRec183[0];
			self.fVec16[2] = self.fVec16[1];
			self.fVec16[1] = self.fVec16[0];
			self.fRec169[1] = self.fRec169[0];
			self.fRec165[1] = self.fRec165[0];
			self.fRec163[1] = self.fRec163[0];
			for j7 in (1..=3).rev() {
				self.fRec159[(j7) as usize] = self.fRec159[(j7 - 1) as usize];
			}
			self.fRec154[1] = self.fRec154[0];
			self.fRec148[1] = self.fRec148[0];
			self.fRec190[1] = self.fRec190[0];
			self.fRec189[1] = self.fRec189[0];
			self.fRec147[1] = self.fRec147[0];
			self.fRec145[1] = self.fRec145[0];
			self.fVec17[1] = self.fVec17[0];
			self.fRec193[1] = self.fRec193[0];
			self.fRec192[1] = self.fRec192[0];
			self.fRec220[1] = self.fRec220[0];
			self.fRec224[1] = self.fRec224[0];
			self.fRec226[2] = self.fRec226[1];
			self.fRec226[1] = self.fRec226[0];
			self.iRec227[1] = self.iRec227[0];
			for j8 in (1..=3).rev() {
				self.fRec228[(j8) as usize] = self.fRec228[(j8 - 1) as usize];
			}
			self.fVec18[1] = self.fVec18[0];
			self.fRec233[1] = self.fRec233[0];
			self.iVec19[1] = self.iVec19[0];
			self.iRec234[1] = self.iRec234[0];
			self.fRec231[1] = self.fRec231[0];
			self.fRec230[1] = self.fRec230[0];
			self.fVec20[2] = self.fVec20[1];
			self.fVec20[1] = self.fVec20[0];
			self.fRec216[1] = self.fRec216[0];
			self.fRec212[1] = self.fRec212[0];
			self.fRec210[1] = self.fRec210[0];
			for j9 in (1..=3).rev() {
				self.fRec206[(j9) as usize] = self.fRec206[(j9 - 1) as usize];
			}
			self.fRec201[1] = self.fRec201[0];
			self.fRec195[1] = self.fRec195[0];
			self.fRec237[1] = self.fRec237[0];
			self.fRec236[1] = self.fRec236[0];
			self.fRec194[1] = self.fRec194[0];
			self.fRec191[1] = self.fRec191[0];
			self.fRec238[1] = self.fRec238[0];
			self.fRec239[1] = self.fRec239[0];
			self.fRec242[1] = self.fRec242[0];
			self.fRec245[1] = self.fRec245[0];
			self.fRec243[1] = self.fRec243[0];
			self.fRec246[1] = self.fRec246[0];
			self.fRec248[1] = self.fRec248[0];
			self.fRec247[1] = self.fRec247[0];
			self.fRec241[2] = self.fRec241[1];
			self.fRec241[1] = self.fRec241[0];
			self.fRec240[2] = self.fRec240[1];
			self.fRec240[1] = self.fRec240[0];
			self.fRec251[1] = self.fRec251[0];
			self.fRec254[1] = self.fRec254[0];
			self.fRec252[1] = self.fRec252[0];
			self.fRec255[1] = self.fRec255[0];
			self.fRec250[2] = self.fRec250[1];
			self.fRec250[1] = self.fRec250[0];
			self.fRec249[2] = self.fRec249[1];
			self.fRec249[1] = self.fRec249[0];
			self.fRec258[1] = self.fRec258[0];
			self.fRec261[1] = self.fRec261[0];
			self.fRec259[1] = self.fRec259[0];
			self.fRec262[1] = self.fRec262[0];
			self.fRec257[2] = self.fRec257[1];
			self.fRec257[1] = self.fRec257[0];
			self.fRec256[2] = self.fRec256[1];
			self.fRec256[1] = self.fRec256[0];
			self.fRec265[1] = self.fRec265[0];
			self.fRec268[1] = self.fRec268[0];
			self.fRec266[1] = self.fRec266[0];
			self.fRec269[1] = self.fRec269[0];
			self.fRec264[2] = self.fRec264[1];
			self.fRec264[1] = self.fRec264[0];
			self.fRec263[2] = self.fRec263[1];
			self.fRec263[1] = self.fRec263[0];
			self.fRec270[1] = self.fRec270[0];
			self.fRec271[1] = self.fRec271[0];
			self.fRec274[1] = self.fRec274[0];
			self.fRec273[1] = self.fRec273[0];
			self.fVec21[1] = self.fVec21[0];
			self.fRec272[1] = self.fRec272[0];
			self.fRec276[1] = self.fRec276[0];
			self.fVec23[1] = self.fVec23[0];
			self.fRec275[1] = self.fRec275[0];
			self.fRec278[1] = self.fRec278[0];
			self.fVec25[1] = self.fVec25[0];
			self.fRec277[1] = self.fRec277[0];
			self.fRec280[1] = self.fRec280[0];
			self.fVec27[1] = self.fVec27[0];
			self.fRec279[1] = self.fRec279[0];
			self.fRec281[1] = self.fRec281[0];
		}
	}

}


}
pub use dsp::mydsp as Instrument;
