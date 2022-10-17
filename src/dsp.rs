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
	fHslider1: F32,
	fRec2: [F32;2],
	fHslider2: F32,
	fRec5: [F32;2],
	fConst3: F32,
	fConst4: F32,
	fHslider3: F32,
	fRec8: [F32;2],
	fRec6: [F32;2],
	fHslider4: F32,
	fRec9: [F32;2],
	fConst5: F32,
	fHslider5: F32,
	fRec11: [F32;2],
	fRec10: [F32;2],
	fRec4: [F32;3],
	fRec3: [F32;3],
	fHslider6: F32,
	fRec14: [F32;2],
	fHslider7: F32,
	fRec17: [F32;2],
	fRec15: [F32;2],
	fRec18: [F32;2],
	fRec13: [F32;3],
	fRec12: [F32;3],
	fHslider8: F32,
	fRec21: [F32;2],
	fHslider9: F32,
	fRec24: [F32;2],
	fRec22: [F32;2],
	fRec25: [F32;2],
	fRec20: [F32;3],
	fRec19: [F32;3],
	fHslider10: F32,
	fRec28: [F32;2],
	fHslider11: F32,
	fRec31: [F32;2],
	fRec29: [F32;2],
	fRec32: [F32;2],
	fRec27: [F32;3],
	fRec26: [F32;3],
	fConst6: F32,
	fHslider12: F32,
	fRec33: [F32;2],
	fHslider13: F32,
	fRec34: [F32;2],
	fConst7: F32,
	fHslider14: F32,
	fRec37: [F32;2],
	fRec36: [F32;2],
	fVec1: [F32;2],
	IOTA0: i32,
	fVec2: [F32;4096],
	fConst8: F32,
	fRec35: [F32;2],
	fRec39: [F32;2],
	fVec3: [F32;2],
	fVec4: [F32;4096],
	fRec38: [F32;2],
	fRec41: [F32;2],
	fVec5: [F32;2],
	fVec6: [F32;4096],
	fRec40: [F32;2],
	fRec43: [F32;2],
	fVec7: [F32;2],
	fVec8: [F32;4096],
	fRec42: [F32;2],
	fConst9: F32,
	fHslider15: F32,
	fRec45: [F32;2],
	fHslider16: F32,
	fRec46: [F32;2],
	fHslider17: F32,
	fConst10: F32,
	fConst11: F32,
	fRec73: [F32;2],
	fRec77: [F32;2],
	fHslider18: F32,
	iRec80: [i32;2],
	fConst12: F32,
	fRec79: [F32;3],
	fButton0: F32,
	fVec9: [F32;2],
	iRec81: [i32;2],
	fConst13: F32,
	fRec82: [F32;4],
	fRec83: [F32;2048],
	fVec10: [F32;2],
	fConst14: F32,
	fConst15: F32,
	fRec87: [F32;2],
	iVec11: [i32;2],
	iConst16: i32,
	iRec88: [i32;2],
	fConst17: F32,
	fRec85: [F32;2],
	fRec84: [F32;2],
	fVec12: [F32;3],
	fRec78: [F32;2048],
	fRec69: [F32;2],
	fRec65: [F32;2],
	fRec61: [F32;2048],
	fRec63: [F32;2],
	fRec59: [F32;4],
	fRec54: [F32;2],
	fRec50: [F32;2048],
	fRec48: [F32;2],
	fConst18: F32,
	fRec91: [F32;2],
	fRec90: [F32;2],
	fRec47: [F32;2],
	fRec44: [F32;2],
	fHslider19: F32,
	fRec92: [F32;2],
	iConst19: i32,
	fRec0: [F32;65536],
	fHslider20: F32,
	fRec93: [F32;2],
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
			fHslider1: 0.0,
			fRec2: [0.0;2],
			fHslider2: 0.0,
			fRec5: [0.0;2],
			fConst3: 0.0,
			fConst4: 0.0,
			fHslider3: 0.0,
			fRec8: [0.0;2],
			fRec6: [0.0;2],
			fHslider4: 0.0,
			fRec9: [0.0;2],
			fConst5: 0.0,
			fHslider5: 0.0,
			fRec11: [0.0;2],
			fRec10: [0.0;2],
			fRec4: [0.0;3],
			fRec3: [0.0;3],
			fHslider6: 0.0,
			fRec14: [0.0;2],
			fHslider7: 0.0,
			fRec17: [0.0;2],
			fRec15: [0.0;2],
			fRec18: [0.0;2],
			fRec13: [0.0;3],
			fRec12: [0.0;3],
			fHslider8: 0.0,
			fRec21: [0.0;2],
			fHslider9: 0.0,
			fRec24: [0.0;2],
			fRec22: [0.0;2],
			fRec25: [0.0;2],
			fRec20: [0.0;3],
			fRec19: [0.0;3],
			fHslider10: 0.0,
			fRec28: [0.0;2],
			fHslider11: 0.0,
			fRec31: [0.0;2],
			fRec29: [0.0;2],
			fRec32: [0.0;2],
			fRec27: [0.0;3],
			fRec26: [0.0;3],
			fConst6: 0.0,
			fHslider12: 0.0,
			fRec33: [0.0;2],
			fHslider13: 0.0,
			fRec34: [0.0;2],
			fConst7: 0.0,
			fHslider14: 0.0,
			fRec37: [0.0;2],
			fRec36: [0.0;2],
			fVec1: [0.0;2],
			IOTA0: 0,
			fVec2: [0.0;4096],
			fConst8: 0.0,
			fRec35: [0.0;2],
			fRec39: [0.0;2],
			fVec3: [0.0;2],
			fVec4: [0.0;4096],
			fRec38: [0.0;2],
			fRec41: [0.0;2],
			fVec5: [0.0;2],
			fVec6: [0.0;4096],
			fRec40: [0.0;2],
			fRec43: [0.0;2],
			fVec7: [0.0;2],
			fVec8: [0.0;4096],
			fRec42: [0.0;2],
			fConst9: 0.0,
			fHslider15: 0.0,
			fRec45: [0.0;2],
			fHslider16: 0.0,
			fRec46: [0.0;2],
			fHslider17: 0.0,
			fConst10: 0.0,
			fConst11: 0.0,
			fRec73: [0.0;2],
			fRec77: [0.0;2],
			fHslider18: 0.0,
			iRec80: [0;2],
			fConst12: 0.0,
			fRec79: [0.0;3],
			fButton0: 0.0,
			fVec9: [0.0;2],
			iRec81: [0;2],
			fConst13: 0.0,
			fRec82: [0.0;4],
			fRec83: [0.0;2048],
			fVec10: [0.0;2],
			fConst14: 0.0,
			fConst15: 0.0,
			fRec87: [0.0;2],
			iVec11: [0;2],
			iConst16: 0,
			iRec88: [0;2],
			fConst17: 0.0,
			fRec85: [0.0;2],
			fRec84: [0.0;2],
			fVec12: [0.0;3],
			fRec78: [0.0;2048],
			fRec69: [0.0;2],
			fRec65: [0.0;2],
			fRec61: [0.0;2048],
			fRec63: [0.0;2],
			fRec59: [0.0;4],
			fRec54: [0.0;2],
			fRec50: [0.0;2048],
			fRec48: [0.0;2],
			fConst18: 0.0,
			fRec91: [0.0;2],
			fRec90: [0.0;2],
			fRec47: [0.0;2],
			fRec44: [0.0;2],
			fHslider19: 0.0,
			fRec92: [0.0;2],
			iConst19: 0,
			fRec0: [0.0;65536],
			fHslider20: 0.0,
			fRec93: [0.0;2],
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
		self.fHslider0 = 0.0;
		self.fHslider1 = 1.0;
		self.fHslider2 = 0.0;
		self.fHslider3 = 60.0;
		self.fHslider4 = 0.0;
		self.fHslider5 = 0.0;
		self.fHslider6 = 0.0;
		self.fHslider7 = 60.0;
		self.fHslider8 = 0.0;
		self.fHslider9 = 60.0;
		self.fHslider10 = 0.0;
		self.fHslider11 = 60.0;
		self.fHslider12 = 0.0;
		self.fHslider13 = 1.0;
		self.fHslider14 = 60.0;
		self.fHslider15 = 80.0;
		self.fHslider16 = 0.0;
		self.fHslider17 = 1.0;
		self.fHslider18 = 0.5;
		self.fButton0 = 0.0;
		self.fHslider19 = 1.0;
		self.fHslider20 = 1.0;
	}
	fn instance_clear(&mut self) {
		for l0 in 0..2 {
			self.iVec0[(l0) as usize] = 0;
		}
		for l1 in 0..2 {
			self.fRec1[(l1) as usize] = 0.0;
		}
		for l2 in 0..2 {
			self.fRec2[(l2) as usize] = 0.0;
		}
		for l3 in 0..2 {
			self.fRec5[(l3) as usize] = 0.0;
		}
		for l4 in 0..2 {
			self.fRec8[(l4) as usize] = 0.0;
		}
		for l5 in 0..2 {
			self.fRec6[(l5) as usize] = 0.0;
		}
		for l6 in 0..2 {
			self.fRec9[(l6) as usize] = 0.0;
		}
		for l7 in 0..2 {
			self.fRec11[(l7) as usize] = 0.0;
		}
		for l8 in 0..2 {
			self.fRec10[(l8) as usize] = 0.0;
		}
		for l9 in 0..3 {
			self.fRec4[(l9) as usize] = 0.0;
		}
		for l10 in 0..3 {
			self.fRec3[(l10) as usize] = 0.0;
		}
		for l11 in 0..2 {
			self.fRec14[(l11) as usize] = 0.0;
		}
		for l12 in 0..2 {
			self.fRec17[(l12) as usize] = 0.0;
		}
		for l13 in 0..2 {
			self.fRec15[(l13) as usize] = 0.0;
		}
		for l14 in 0..2 {
			self.fRec18[(l14) as usize] = 0.0;
		}
		for l15 in 0..3 {
			self.fRec13[(l15) as usize] = 0.0;
		}
		for l16 in 0..3 {
			self.fRec12[(l16) as usize] = 0.0;
		}
		for l17 in 0..2 {
			self.fRec21[(l17) as usize] = 0.0;
		}
		for l18 in 0..2 {
			self.fRec24[(l18) as usize] = 0.0;
		}
		for l19 in 0..2 {
			self.fRec22[(l19) as usize] = 0.0;
		}
		for l20 in 0..2 {
			self.fRec25[(l20) as usize] = 0.0;
		}
		for l21 in 0..3 {
			self.fRec20[(l21) as usize] = 0.0;
		}
		for l22 in 0..3 {
			self.fRec19[(l22) as usize] = 0.0;
		}
		for l23 in 0..2 {
			self.fRec28[(l23) as usize] = 0.0;
		}
		for l24 in 0..2 {
			self.fRec31[(l24) as usize] = 0.0;
		}
		for l25 in 0..2 {
			self.fRec29[(l25) as usize] = 0.0;
		}
		for l26 in 0..2 {
			self.fRec32[(l26) as usize] = 0.0;
		}
		for l27 in 0..3 {
			self.fRec27[(l27) as usize] = 0.0;
		}
		for l28 in 0..3 {
			self.fRec26[(l28) as usize] = 0.0;
		}
		for l29 in 0..2 {
			self.fRec33[(l29) as usize] = 0.0;
		}
		for l30 in 0..2 {
			self.fRec34[(l30) as usize] = 0.0;
		}
		for l31 in 0..2 {
			self.fRec37[(l31) as usize] = 0.0;
		}
		for l32 in 0..2 {
			self.fRec36[(l32) as usize] = 0.0;
		}
		for l33 in 0..2 {
			self.fVec1[(l33) as usize] = 0.0;
		}
		self.IOTA0 = 0;
		for l34 in 0..4096 {
			self.fVec2[(l34) as usize] = 0.0;
		}
		for l35 in 0..2 {
			self.fRec35[(l35) as usize] = 0.0;
		}
		for l36 in 0..2 {
			self.fRec39[(l36) as usize] = 0.0;
		}
		for l37 in 0..2 {
			self.fVec3[(l37) as usize] = 0.0;
		}
		for l38 in 0..4096 {
			self.fVec4[(l38) as usize] = 0.0;
		}
		for l39 in 0..2 {
			self.fRec38[(l39) as usize] = 0.0;
		}
		for l40 in 0..2 {
			self.fRec41[(l40) as usize] = 0.0;
		}
		for l41 in 0..2 {
			self.fVec5[(l41) as usize] = 0.0;
		}
		for l42 in 0..4096 {
			self.fVec6[(l42) as usize] = 0.0;
		}
		for l43 in 0..2 {
			self.fRec40[(l43) as usize] = 0.0;
		}
		for l44 in 0..2 {
			self.fRec43[(l44) as usize] = 0.0;
		}
		for l45 in 0..2 {
			self.fVec7[(l45) as usize] = 0.0;
		}
		for l46 in 0..4096 {
			self.fVec8[(l46) as usize] = 0.0;
		}
		for l47 in 0..2 {
			self.fRec42[(l47) as usize] = 0.0;
		}
		for l48 in 0..2 {
			self.fRec45[(l48) as usize] = 0.0;
		}
		for l49 in 0..2 {
			self.fRec46[(l49) as usize] = 0.0;
		}
		for l50 in 0..2 {
			self.fRec73[(l50) as usize] = 0.0;
		}
		for l51 in 0..2 {
			self.fRec77[(l51) as usize] = 0.0;
		}
		for l52 in 0..2 {
			self.iRec80[(l52) as usize] = 0;
		}
		for l53 in 0..3 {
			self.fRec79[(l53) as usize] = 0.0;
		}
		for l54 in 0..2 {
			self.fVec9[(l54) as usize] = 0.0;
		}
		for l55 in 0..2 {
			self.iRec81[(l55) as usize] = 0;
		}
		for l56 in 0..4 {
			self.fRec82[(l56) as usize] = 0.0;
		}
		for l57 in 0..2048 {
			self.fRec83[(l57) as usize] = 0.0;
		}
		for l58 in 0..2 {
			self.fVec10[(l58) as usize] = 0.0;
		}
		for l59 in 0..2 {
			self.fRec87[(l59) as usize] = 0.0;
		}
		for l60 in 0..2 {
			self.iVec11[(l60) as usize] = 0;
		}
		for l61 in 0..2 {
			self.iRec88[(l61) as usize] = 0;
		}
		for l62 in 0..2 {
			self.fRec85[(l62) as usize] = 0.0;
		}
		for l63 in 0..2 {
			self.fRec84[(l63) as usize] = 0.0;
		}
		for l64 in 0..3 {
			self.fVec12[(l64) as usize] = 0.0;
		}
		for l65 in 0..2048 {
			self.fRec78[(l65) as usize] = 0.0;
		}
		for l66 in 0..2 {
			self.fRec69[(l66) as usize] = 0.0;
		}
		for l67 in 0..2 {
			self.fRec65[(l67) as usize] = 0.0;
		}
		for l68 in 0..2048 {
			self.fRec61[(l68) as usize] = 0.0;
		}
		for l69 in 0..2 {
			self.fRec63[(l69) as usize] = 0.0;
		}
		for l70 in 0..4 {
			self.fRec59[(l70) as usize] = 0.0;
		}
		for l71 in 0..2 {
			self.fRec54[(l71) as usize] = 0.0;
		}
		for l72 in 0..2048 {
			self.fRec50[(l72) as usize] = 0.0;
		}
		for l73 in 0..2 {
			self.fRec48[(l73) as usize] = 0.0;
		}
		for l74 in 0..2 {
			self.fRec91[(l74) as usize] = 0.0;
		}
		for l75 in 0..2 {
			self.fRec90[(l75) as usize] = 0.0;
		}
		for l76 in 0..2 {
			self.fRec47[(l76) as usize] = 0.0;
		}
		for l77 in 0..2 {
			self.fRec44[(l77) as usize] = 0.0;
		}
		for l78 in 0..2 {
			self.fRec92[(l78) as usize] = 0.0;
		}
		for l79 in 0..65536 {
			self.fRec0[(l79) as usize] = 0.0;
		}
		for l80 in 0..2 {
			self.fRec93[(l80) as usize] = 0.0;
		}
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		self.fConst0 = F32::min(192000.0, F32::max(1.0, ((self.fSampleRate) as F32)));
		self.fConst1 = 44.0999985 / self.fConst0;
		self.fConst2 = 1.0 - self.fConst1;
		self.fConst3 = 1.0 / self.fConst0;
		self.fConst4 = 19404.0 / self.fConst0;
		self.fConst5 = 3.14159274 / self.fConst0;
		self.fConst6 = 352.0 / self.fConst0;
		self.fConst7 = 0.25 * self.fConst0;
		self.fConst8 = 0.5 * self.fConst0;
		self.fConst9 = 2764.60156 / self.fConst0;
		self.fConst10 = 0.00882352982 * self.fConst0;
		self.fConst11 = 0.000735294132 * self.fConst0;
		self.fConst12 = 6911.50391 / self.fConst0;
		self.fConst13 = 0.00200000009 * self.fConst0;
		self.fConst14 = F32::exp(0.0 - 10000.0 / self.fConst0);
		self.fConst15 = 1.0 - self.fConst14;
		self.iConst16 = ((0.100000001 * self.fConst0) as i32);
		self.fConst17 = F32::exp(0.0 - 50.0 / self.fConst0);
		self.fConst18 = F32::exp(0.0 - 10.0 / self.fConst0);
		self.iConst19 = ((F32::min(self.fConst0, F32::max(0.0, 0.300000012 * self.fConst0))) as i32) + 1;
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
		ui_interface.declare(Some(ParamIndex(14)), "3", "");
		ui_interface.add_horizontal_slider("strength", ParamIndex(14), 0.5, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(15)), "4", "");
		ui_interface.add_horizontal_slider("pitchBend", ParamIndex(15), 0.0, -1.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "2", "");
		ui_interface.open_horizontal_box("drone");
		ui_interface.declare(Some(ParamIndex(16)), "0", "");
		ui_interface.add_horizontal_slider("volume", ParamIndex(16), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(17)), "1", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(17), 60.0, 0.0, 127.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "3", "");
		ui_interface.open_vertical_box("mix");
		ui_interface.declare(Some(ParamIndex(18)), "0", "");
		ui_interface.add_horizontal_slider("master", ParamIndex(18), 1.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(19)), "1", "");
		ui_interface.add_horizontal_slider("drone", ParamIndex(19), 1.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(20)), "2", "");
		ui_interface.add_horizontal_slider("lead", ParamIndex(20), 1.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(21)), "3", "");
		ui_interface.add_horizontal_slider("pluck", ParamIndex(21), 1.0, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			11 => Some(self.fButton0),
			0 => Some(self.fHslider0),
			20 => Some(self.fHslider1),
			10 => Some(self.fHslider10),
			9 => Some(self.fHslider11),
			16 => Some(self.fHslider12),
			19 => Some(self.fHslider13),
			17 => Some(self.fHslider14),
			12 => Some(self.fHslider15),
			15 => Some(self.fHslider16),
			13 => Some(self.fHslider17),
			14 => Some(self.fHslider18),
			21 => Some(self.fHslider19),
			4 => Some(self.fHslider2),
			18 => Some(self.fHslider20),
			3 => Some(self.fHslider3),
			2 => Some(self.fHslider4),
			1 => Some(self.fHslider5),
			6 => Some(self.fHslider6),
			5 => Some(self.fHslider7),
			8 => Some(self.fHslider8),
			7 => Some(self.fHslider9),
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			11 => { self.fButton0 = value }
			0 => { self.fHslider0 = value }
			20 => { self.fHslider1 = value }
			10 => { self.fHslider10 = value }
			9 => { self.fHslider11 = value }
			16 => { self.fHslider12 = value }
			19 => { self.fHslider13 = value }
			17 => { self.fHslider14 = value }
			12 => { self.fHslider15 = value }
			15 => { self.fHslider16 = value }
			13 => { self.fHslider17 = value }
			14 => { self.fHslider18 = value }
			21 => { self.fHslider19 = value }
			4 => { self.fHslider2 = value }
			18 => { self.fHslider20 = value }
			3 => { self.fHslider3 = value }
			2 => { self.fHslider4 = value }
			1 => { self.fHslider5 = value }
			6 => { self.fHslider6 = value }
			5 => { self.fHslider7 = value }
			8 => { self.fHslider8 = value }
			7 => { self.fHslider9 = value }
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
		let mut fSlow2: F32 = self.fConst1 * ((self.fHslider2) as F32);
		let mut fSlow3: F32 = ((self.fHslider3) as F32);
		let mut fSlow4: F32 = self.fConst4 * F32::powf(2.0, 0.0833333358 * (fSlow3 + -69.0));
		let mut fSlow5: F32 = self.fConst1 * ((self.fHslider4) as F32);
		let mut fSlow6: F32 = self.fConst1 * ((self.fHslider5) as F32);
		let mut fSlow7: F32 = self.fConst1 * ((self.fHslider6) as F32);
		let mut fSlow8: F32 = ((self.fHslider7) as F32);
		let mut fSlow9: F32 = self.fConst4 * F32::powf(2.0, 0.0833333358 * (fSlow8 + -69.0));
		let mut fSlow10: F32 = self.fConst1 * ((self.fHslider8) as F32);
		let mut fSlow11: F32 = ((self.fHslider9) as F32);
		let mut fSlow12: F32 = self.fConst4 * F32::powf(2.0, 0.0833333358 * (fSlow11 + -69.0));
		let mut fSlow13: F32 = self.fConst1 * ((self.fHslider10) as F32);
		let mut fSlow14: F32 = ((self.fHslider11) as F32);
		let mut fSlow15: F32 = self.fConst4 * F32::powf(2.0, 0.0833333358 * (fSlow14 + -69.0));
		let mut fSlow16: F32 = self.fConst1 * ((self.fHslider12) as F32);
		let mut fSlow17: F32 = self.fConst1 * ((self.fHslider13) as F32);
		let mut fSlow18: F32 = self.fConst1 * ((self.fHslider14) as F32);
		let mut fSlow19: F32 = self.fConst1 * ((self.fHslider15) as F32);
		let mut fSlow20: F32 = self.fConst1 * ((self.fHslider16) as F32);
		let mut fSlow21: F32 = ((self.fHslider17) as F32);
		let mut fSlow22: F32 = ((self.fHslider18) as F32);
		let mut fSlow23: F32 = ((self.fButton0) as F32);
		let mut fSlow24: F32 = self.fConst1 * ((self.fHslider19) as F32);
		let mut fSlow25: F32 = self.fConst1 * ((self.fHslider20) as F32);
		let zipped_iterators = outputs0.zip(outputs1);
		for (output0, output1) in zipped_iterators {
			self.iVec0[0] = 1;
			self.fRec1[0] = fSlow0 + self.fConst2 * self.fRec1[1];
			self.fRec2[0] = fSlow1 + self.fConst2 * self.fRec2[1];
			self.fRec5[0] = fSlow2 + self.fConst2 * self.fRec5[1];
			self.fRec8[0] = fSlow4 + self.fConst2 * self.fRec8[1];
			let mut fTemp0: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec8[0]));
			let mut fTemp1: F32 = self.fRec6[1] + self.fConst3 * fTemp0;
			let mut fTemp2: F32 = fTemp1 + -1.0;
			let mut iTemp3: i32 = ((fTemp2 < 0.0) as i32);
			self.fRec6[0] = if (iTemp3 as i32 != 0) { fTemp1 } else { fTemp2 };
			let mut fThen1: F32 = fTemp1 + (1.0 - self.fConst0 / fTemp0) * fTemp2;
			let mut fRec7: F32 = if (iTemp3 as i32 != 0) { fTemp1 } else { fThen1 };
			self.fRec9[0] = fSlow5 + self.fConst2 * self.fRec9[1];
			let mut fTemp4: F32 = F32::min(1.41419947, 1.41421354 * self.fRec9[0]);
			let mut fTemp5: F32 = fTemp4 * (fTemp4 + 1.41421354);
			let mut fTemp6: F32 = 1.41421354 * fTemp4;
			let mut fTemp7: F32 = fTemp6 + 2.0;
			self.fRec11[0] = fSlow6 + self.fConst2 * self.fRec11[1];
			let mut fTemp8: F32 = self.fRec11[0] + -69.0;
			self.fRec10[0] = self.fConst2 * self.fRec10[1] + self.fConst4 * F32::powf(2.0, 0.0833333358 * (fSlow3 + fTemp8));
			let mut fTemp9: F32 = F32::tan(self.fConst5 * F32::max(20.0, F32::min(10000.0, self.fRec10[0])));
			let mut fTemp10: F32 = 1.0 / fTemp9;
			let mut fTemp11: F32 = 1.0 - 1.0 / mydsp_faustpower2_f(fTemp9);
			let mut fTemp12: F32 = fTemp5 + (fTemp10 + fTemp7) / fTemp9 + 1.0;
			self.fRec4[0] = self.fRec5[0] * (2.0 * fRec7 + -1.0) - (self.fRec4[2] * (fTemp5 + 1.0 - (fTemp7 - fTemp10) / fTemp9) + 2.0 * self.fRec4[1] * (fTemp5 + fTemp11)) / fTemp12;
			let mut fTemp13: F32 = fTemp4 * (fTemp4 + -1.41421354);
			let mut fTemp14: F32 = 2.0 - fTemp6;
			let mut fTemp15: F32 = fTemp13 + (fTemp14 + fTemp10) / fTemp9 + 1.0;
			self.fRec3[0] = (self.fRec4[2] + self.fRec4[0] + 2.0 * self.fRec4[1]) / fTemp12 - (self.fRec3[2] * (fTemp13 + (fTemp10 - fTemp14) / fTemp9 + 1.0) + 2.0 * self.fRec3[1] * (fTemp13 + fTemp11)) / fTemp15;
			self.fRec14[0] = fSlow7 + self.fConst2 * self.fRec14[1];
			self.fRec17[0] = fSlow9 + self.fConst2 * self.fRec17[1];
			let mut fTemp16: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec17[0]));
			let mut fTemp17: F32 = self.fRec15[1] + self.fConst3 * fTemp16;
			let mut fTemp18: F32 = fTemp17 + -1.0;
			let mut iTemp19: i32 = ((fTemp18 < 0.0) as i32);
			self.fRec15[0] = if (iTemp19 as i32 != 0) { fTemp17 } else { fTemp18 };
			let mut fThen3: F32 = fTemp17 + (1.0 - self.fConst0 / fTemp16) * fTemp18;
			let mut fRec16: F32 = if (iTemp19 as i32 != 0) { fTemp17 } else { fThen3 };
			self.fRec18[0] = self.fConst2 * self.fRec18[1] + self.fConst4 * F32::powf(2.0, 0.0833333358 * (fSlow8 + fTemp8));
			let mut fTemp20: F32 = F32::tan(self.fConst5 * F32::max(20.0, F32::min(10000.0, self.fRec18[0])));
			let mut fTemp21: F32 = 1.0 / fTemp20;
			let mut fTemp22: F32 = 1.0 - 1.0 / mydsp_faustpower2_f(fTemp20);
			let mut fTemp23: F32 = fTemp5 + (fTemp7 + fTemp21) / fTemp20 + 1.0;
			self.fRec13[0] = self.fRec14[0] * (2.0 * fRec16 + -1.0) - (self.fRec13[2] * (fTemp5 + (fTemp21 - fTemp7) / fTemp20 + 1.0) + 2.0 * self.fRec13[1] * (fTemp5 + fTemp22)) / fTemp23;
			let mut fTemp24: F32 = fTemp13 + (fTemp14 + fTemp21) / fTemp20 + 1.0;
			self.fRec12[0] = (self.fRec13[2] + self.fRec13[0] + 2.0 * self.fRec13[1]) / fTemp23 - (self.fRec12[2] * (fTemp13 + (fTemp21 - fTemp14) / fTemp20 + 1.0) + 2.0 * self.fRec12[1] * (fTemp13 + fTemp22)) / fTemp24;
			self.fRec21[0] = fSlow10 + self.fConst2 * self.fRec21[1];
			self.fRec24[0] = fSlow12 + self.fConst2 * self.fRec24[1];
			let mut fTemp25: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec24[0]));
			let mut fTemp26: F32 = self.fRec22[1] + self.fConst3 * fTemp25;
			let mut fTemp27: F32 = fTemp26 + -1.0;
			let mut iTemp28: i32 = ((fTemp27 < 0.0) as i32);
			self.fRec22[0] = if (iTemp28 as i32 != 0) { fTemp26 } else { fTemp27 };
			let mut fThen5: F32 = fTemp26 + (1.0 - self.fConst0 / fTemp25) * fTemp27;
			let mut fRec23: F32 = if (iTemp28 as i32 != 0) { fTemp26 } else { fThen5 };
			self.fRec25[0] = self.fConst2 * self.fRec25[1] + self.fConst4 * F32::powf(2.0, 0.0833333358 * (fSlow11 + fTemp8));
			let mut fTemp29: F32 = F32::tan(self.fConst5 * F32::max(20.0, F32::min(10000.0, self.fRec25[0])));
			let mut fTemp30: F32 = 1.0 / fTemp29;
			let mut fTemp31: F32 = 1.0 - 1.0 / mydsp_faustpower2_f(fTemp29);
			let mut fTemp32: F32 = fTemp5 + (fTemp7 + fTemp30) / fTemp29 + 1.0;
			self.fRec20[0] = self.fRec21[0] * (2.0 * fRec23 + -1.0) - (self.fRec20[2] * (fTemp5 + (fTemp30 - fTemp7) / fTemp29 + 1.0) + 2.0 * self.fRec20[1] * (fTemp5 + fTemp31)) / fTemp32;
			let mut fTemp33: F32 = fTemp13 + (fTemp14 + fTemp30) / fTemp29 + 1.0;
			self.fRec19[0] = (self.fRec20[2] + self.fRec20[0] + 2.0 * self.fRec20[1]) / fTemp32 - (self.fRec19[2] * (fTemp13 + (fTemp30 - fTemp14) / fTemp29 + 1.0) + 2.0 * self.fRec19[1] * (fTemp13 + fTemp31)) / fTemp33;
			self.fRec28[0] = fSlow13 + self.fConst2 * self.fRec28[1];
			self.fRec31[0] = fSlow15 + self.fConst2 * self.fRec31[1];
			let mut fTemp34: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec31[0]));
			let mut fTemp35: F32 = self.fConst3 * fTemp34;
			let mut fTemp36: F32 = self.fRec29[1] + fTemp35;
			let mut fTemp37: F32 = fTemp36 + -1.0;
			let mut iTemp38: i32 = ((fTemp37 < 0.0) as i32);
			self.fRec29[0] = if (iTemp38 as i32 != 0) { fTemp36 } else { fTemp37 };
			let mut fThen7: F32 = fTemp35 + self.fRec29[1] + (1.0 - self.fConst0 / fTemp34) * fTemp37;
			let mut fRec30: F32 = if (iTemp38 as i32 != 0) { fTemp36 } else { fThen7 };
			self.fRec32[0] = self.fConst2 * self.fRec32[1] + self.fConst4 * F32::powf(2.0, 0.0833333358 * (fSlow14 + fTemp8));
			let mut fTemp39: F32 = F32::tan(self.fConst5 * F32::max(20.0, F32::min(10000.0, self.fRec32[0])));
			let mut fTemp40: F32 = 1.0 / fTemp39;
			let mut fTemp41: F32 = 1.0 - 1.0 / mydsp_faustpower2_f(fTemp39);
			let mut fTemp42: F32 = fTemp5 + (fTemp7 + fTemp40) / fTemp39 + 1.0;
			self.fRec27[0] = self.fRec28[0] * (2.0 * fRec30 + -1.0) - (self.fRec27[2] * (fTemp5 + (fTemp40 - fTemp7) / fTemp39 + 1.0) + 2.0 * self.fRec27[1] * (fTemp5 + fTemp41)) / fTemp42;
			let mut fTemp43: F32 = fTemp13 + (fTemp14 + fTemp40) / fTemp39 + 1.0;
			self.fRec26[0] = (self.fRec27[2] + self.fRec27[0] + 2.0 * self.fRec27[1]) / fTemp42 - (self.fRec26[2] * (fTemp13 + (fTemp40 - fTemp14) / fTemp39 + 1.0) + 2.0 * self.fRec26[1] * (fTemp13 + fTemp41)) / fTemp43;
			self.fRec33[0] = fSlow16 + self.fConst2 * self.fRec33[1];
			self.fRec34[0] = fSlow17 + self.fConst2 * self.fRec34[1];
			let mut fTemp44: F32 = ((self.iVec0[1]) as F32);
			self.fRec37[0] = fSlow18 + self.fConst2 * self.fRec37[1];
			let mut fTemp45: F32 = F32::powf(2.0, 0.0833333358 * (self.fRec37[0] + -69.0));
			let mut fTemp46: F32 = F32::max(440.0 * fTemp45, 23.4489498);
			let mut fTemp47: F32 = F32::max(20.0, F32::abs(fTemp46));
			let mut fTemp48: F32 = self.fRec36[1] + self.fConst3 * fTemp47;
			self.fRec36[0] = fTemp48 - F32::floor(fTemp48);
			let mut fTemp49: F32 = mydsp_faustpower2_f(2.0 * self.fRec36[0] + -1.0);
			self.fVec1[0] = fTemp49;
			let mut fTemp50: F32 = (fTemp44 * (fTemp49 - self.fVec1[1])) / fTemp47;
			self.fVec2[(self.IOTA0 & 4095) as usize] = fTemp50;
			let mut fTemp51: F32 = F32::max(0.0, F32::min(2047.0, self.fConst8 / fTemp46));
			let mut iTemp52: i32 = ((fTemp51) as i32);
			let mut fTemp53: F32 = F32::floor(fTemp51);
			self.fRec35[0] = 0.999000013 * self.fRec35[1] - self.fConst7 * (self.fVec2[((self.IOTA0 - iTemp52) & 4095) as usize] * (fTemp53 + 1.0 - fTemp51) - fTemp50 + (fTemp51 - fTemp53) * self.fVec2[((self.IOTA0 - (iTemp52 + 1)) & 4095) as usize]);
			let mut fTemp54: F32 = F32::powf(2.0, 0.0833333358 * (self.fRec37[0] + -56.9000015));
			let mut fTemp55: F32 = F32::max(440.0 * fTemp54, 23.4489498);
			let mut fTemp56: F32 = F32::max(20.0, F32::abs(fTemp55));
			let mut fTemp57: F32 = self.fRec39[1] + self.fConst3 * fTemp56;
			self.fRec39[0] = fTemp57 - F32::floor(fTemp57);
			let mut fTemp58: F32 = mydsp_faustpower2_f(2.0 * self.fRec39[0] + -1.0);
			self.fVec3[0] = fTemp58;
			let mut fTemp59: F32 = (fTemp44 * (fTemp58 - self.fVec3[1])) / fTemp56;
			self.fVec4[(self.IOTA0 & 4095) as usize] = fTemp59;
			let mut fTemp60: F32 = F32::max(0.0, F32::min(2047.0, self.fConst8 / fTemp55));
			let mut iTemp61: i32 = ((fTemp60) as i32);
			let mut fTemp62: F32 = F32::floor(fTemp60);
			self.fRec38[0] = 0.999000013 * self.fRec38[1] - self.fConst7 * (self.fVec4[((self.IOTA0 - iTemp61) & 4095) as usize] * (fTemp62 + 1.0 - fTemp60) - fTemp59 + (fTemp60 - fTemp62) * self.fVec4[((self.IOTA0 - (iTemp61 + 1)) & 4095) as usize]);
			let mut fTemp63: F32 = F32::powf(2.0, 0.0833333358 * (self.fRec37[0] + -81.1100006));
			let mut fTemp64: F32 = F32::max(440.0 * fTemp63, 23.4489498);
			let mut fTemp65: F32 = F32::max(20.0, F32::abs(fTemp64));
			let mut fTemp66: F32 = self.fRec41[1] + self.fConst3 * fTemp65;
			self.fRec41[0] = fTemp66 - F32::floor(fTemp66);
			let mut fTemp67: F32 = mydsp_faustpower2_f(2.0 * self.fRec41[0] + -1.0);
			self.fVec5[0] = fTemp67;
			let mut fTemp68: F32 = (fTemp44 * (fTemp67 - self.fVec5[1])) / fTemp65;
			self.fVec6[(self.IOTA0 & 4095) as usize] = fTemp68;
			let mut fTemp69: F32 = F32::max(0.0, F32::min(2047.0, self.fConst8 / fTemp64));
			let mut iTemp70: i32 = ((fTemp69) as i32);
			let mut fTemp71: F32 = F32::floor(fTemp69);
			self.fRec40[0] = 0.999000013 * self.fRec40[1] - self.fConst7 * (self.fVec6[((self.IOTA0 - iTemp70) & 4095) as usize] * (fTemp71 + 1.0 - fTemp69) - fTemp68 + (fTemp69 - fTemp71) * self.fVec6[((self.IOTA0 - (iTemp70 + 1)) & 4095) as usize]);
			let mut fTemp72: F32 = F32::powf(2.0, 0.0833333358 * (self.fRec37[0] + -61.8800011));
			let mut fTemp73: F32 = F32::max(440.0 * fTemp72, 23.4489498);
			let mut fTemp74: F32 = F32::max(20.0, F32::abs(fTemp73));
			let mut fTemp75: F32 = self.fRec43[1] + self.fConst3 * fTemp74;
			self.fRec43[0] = fTemp75 - F32::floor(fTemp75);
			let mut fTemp76: F32 = mydsp_faustpower2_f(2.0 * self.fRec43[0] + -1.0);
			self.fVec7[0] = fTemp76;
			let mut fTemp77: F32 = (fTemp44 * (fTemp76 - self.fVec7[1])) / fTemp74;
			self.fVec8[(self.IOTA0 & 4095) as usize] = fTemp77;
			let mut fTemp78: F32 = F32::max(0.0, F32::min(2047.0, self.fConst8 / fTemp73));
			let mut iTemp79: i32 = ((fTemp78) as i32);
			let mut fTemp80: F32 = F32::floor(fTemp78);
			self.fRec42[0] = 0.999000013 * self.fRec42[1] - self.fConst7 * (self.fVec8[((self.IOTA0 - iTemp79) & 4095) as usize] * (fTemp80 + 1.0 - fTemp78) - fTemp77 + (fTemp78 - fTemp80) * self.fVec8[((self.IOTA0 - (iTemp79 + 1)) & 4095) as usize]);
			self.fRec45[0] = fSlow19 + self.fConst2 * self.fRec45[1];
			self.fRec46[0] = fSlow20 + self.fConst2 * self.fRec46[1];
			let mut fTemp81: F32 = F32::powf(2.0, 0.0833333358 * (self.fRec45[0] + self.fRec46[0] + -69.0));
			let mut fTemp82: F32 = 1.0 / F32::tan(self.fConst9 * fTemp81);
			let mut fRec58: F32 = -1.0 * 0.997305274 * (0.899999976 * self.fRec59[2] + 0.0500000007 * (self.fRec59[1] + self.fRec59[3]));
			let mut fTemp83: F32 = self.fConst11 * (0.772727251 / fTemp81 + -0.109999999);
			let mut fTemp84: F32 = fTemp83 + -1.49999499;
			let mut iTemp85: i32 = ((fTemp84) as i32);
			let mut iTemp86: i32 = ((F32::min(self.fConst10, ((std::cmp::max(0, iTemp85)) as F32))) as i32);
			let mut iTemp87: i32 = iTemp86 + 1;
			let mut fTemp88: F32 = F32::floor(fTemp84);
			let mut fTemp89: F32 = fTemp83 + -1.0 - fTemp88;
			let mut fTemp90: F32 = 0.0 - fTemp89;
			let mut fTemp91: F32 = fTemp83 + -2.0 - fTemp88;
			let mut fTemp92: F32 = 0.0 - 0.5 * fTemp91;
			let mut fTemp93: F32 = fTemp83 + -3.0 - fTemp88;
			let mut fTemp94: F32 = 0.0 - 0.333333343 * fTemp93;
			let mut fTemp95: F32 = fTemp83 + -4.0 - fTemp88;
			let mut fTemp96: F32 = 0.0 - 0.25 * fTemp95;
			let mut fTemp97: F32 = fTemp83 - fTemp88;
			let mut iTemp98: i32 = ((F32::min(self.fConst10, ((std::cmp::max(0, iTemp85 + 1)) as F32))) as i32);
			let mut iTemp99: i32 = iTemp98 + 1;
			let mut fTemp100: F32 = 0.0 - fTemp91;
			let mut fTemp101: F32 = 0.0 - 0.5 * fTemp93;
			let mut fTemp102: F32 = 0.0 - 0.333333343 * fTemp95;
			let mut iTemp103: i32 = ((F32::min(self.fConst10, ((std::cmp::max(0, iTemp85 + 2)) as F32))) as i32);
			let mut iTemp104: i32 = iTemp103 + 1;
			let mut fTemp105: F32 = 0.0 - fTemp93;
			let mut fTemp106: F32 = 0.0 - 0.5 * fTemp95;
			let mut fTemp107: F32 = fTemp89 * fTemp91;
			let mut iTemp108: i32 = ((F32::min(self.fConst10, ((std::cmp::max(0, iTemp85 + 3)) as F32))) as i32);
			let mut iTemp109: i32 = iTemp108 + 1;
			let mut fTemp110: F32 = 0.0 - fTemp95;
			let mut fTemp111: F32 = fTemp107 * fTemp93;
			let mut iTemp112: i32 = ((F32::min(self.fConst10, ((std::cmp::max(0, iTemp85 + 4)) as F32))) as i32);
			let mut iTemp113: i32 = iTemp112 + 1;
			self.fRec73[0] = self.fRec50[((self.IOTA0 - iTemp87) & 2047) as usize] * fTemp90 * fTemp92 * fTemp94 * fTemp96 + fTemp97 * (self.fRec50[((self.IOTA0 - iTemp99) & 2047) as usize] * fTemp100 * fTemp101 * fTemp102 + 0.5 * fTemp89 * self.fRec50[((self.IOTA0 - iTemp104) & 2047) as usize] * fTemp105 * fTemp106 + 0.166666672 * fTemp107 * self.fRec50[((self.IOTA0 - iTemp109) & 2047) as usize] * fTemp110 + 0.0416666679 * fTemp111 * self.fRec50[((self.IOTA0 - iTemp113) & 2047) as usize]);
			self.fRec77[0] = 0.0500000007 * self.fRec77[1] + 0.949999988 * self.fRec73[1];
			let mut fRec74: F32 = self.fRec77[0];
			let mut fTemp114: F32 = fTemp90 * fTemp92 * fTemp94 * fTemp96;
			self.iRec80[0] = 1103515245 * self.iRec80[1] + 12345;
			let mut fTemp115: F32 = F32::tan(self.fConst12 * fTemp81);
			let mut fTemp116: F32 = 1.0 / fTemp115;
			let mut fTemp117: F32 = (fTemp116 + 1.41421354) / fTemp115 + 1.0;
			self.fRec79[0] = 4.65661287e-10 * ((self.iRec80[0]) as F32) - (self.fRec79[2] * ((fTemp116 + -1.41421354) / fTemp115 + 1.0) + 2.0 * self.fRec79[1] * (1.0 - 1.0 / mydsp_faustpower2_f(fTemp115))) / fTemp117;
			self.fVec9[0] = fSlow23;
			self.iRec81[0] = (self.iRec81[1] + ((self.iRec81[1] > 0) as i32)) * ((fSlow23 <= self.fVec9[1]) as i32) + ((fSlow23 > self.fVec9[1]) as i32);
			let mut fTemp118: F32 = ((self.iRec81[0]) as F32) / F32::max(1.0, self.fConst13 * mydsp_faustpower2_f(1.0 - 0.219999999 * fTemp81));
			let mut fTemp119: F32 = fSlow22 * ((self.fRec79[2] + self.fRec79[0] + 2.0 * self.fRec79[1]) * F32::max(0.0, F32::min(fTemp118, 2.0 - fTemp118))) / fTemp117;
			self.fRec82[0] = self.fRec48[1];
			self.fRec83[(self.IOTA0 & 2047) as usize] = -1.0 * 0.997305274 * (0.899999976 * self.fRec82[2] + 0.0500000007 * (self.fRec82[1] + self.fRec82[3]));
			let mut fTemp120: F32 = fTemp100 * fTemp101 * fTemp102;
			let mut fTemp121: F32 = fTemp89 * fTemp105 * fTemp106;
			let mut fTemp122: F32 = fTemp107 * fTemp110;
			self.fVec10[0] = fTemp114 * self.fRec83[((self.IOTA0 - (iTemp86 + 2)) & 2047) as usize] + fTemp97 * (fTemp120 * self.fRec83[((self.IOTA0 - (iTemp98 + 2)) & 2047) as usize] + 0.5 * fTemp121 * self.fRec83[((self.IOTA0 - (iTemp103 + 2)) & 2047) as usize] + 0.166666672 * fTemp122 * self.fRec83[((self.IOTA0 - (iTemp108 + 2)) & 2047) as usize] + 0.0416666679 * fTemp111 * self.fRec83[((self.IOTA0 - (iTemp112 + 2)) & 2047) as usize]);
			self.fRec87[0] = self.fConst14 * self.fRec87[1] + self.fConst15 * F32::abs(self.fRec47[1]);
			let mut fRec86: F32 = self.fRec87[0];
			let mut iTemp123: i32 = ((fRec86 > 0.100000001) as i32);
			self.iVec11[0] = iTemp123;
			self.iRec88[0] = std::cmp::max(((self.iConst16 * ((iTemp123 < self.iVec11[1]) as i32)) as i32), ((self.iRec88[1] + -1) as i32));
			let mut fTemp124: F32 = F32::abs(F32::max(((iTemp123) as F32), ((((self.iRec88[0] > 0) as i32)) as F32)));
			let mut fTemp125: F32 = if (((self.fRec84[1] > fTemp124) as i32) as i32 != 0) { self.fConst17 } else { self.fConst14 };
			self.fRec85[0] = self.fRec85[1] * fTemp125 + fTemp124 * (1.0 - fTemp125);
			self.fRec84[0] = self.fRec85[0];
			let mut fTemp126: F32 = 0.00499999989 * self.fRec84[0] * self.fRec47[1];
			let mut fTemp127: F32 = fTemp119 + self.fVec10[1] + fTemp126;
			self.fVec12[0] = fTemp127;
			self.fRec78[(self.IOTA0 & 2047) as usize] = 0.0500000007 * self.fRec78[((self.IOTA0 - 1) & 2047) as usize] + 0.949999988 * self.fVec12[2];
			let mut fRec75: F32 = fTemp114 * self.fRec78[((self.IOTA0 - iTemp86) & 2047) as usize] + fTemp97 * (fTemp120 * self.fRec78[((self.IOTA0 - iTemp98) & 2047) as usize] + 0.5 * fTemp121 * self.fRec78[((self.IOTA0 - iTemp103) & 2047) as usize] + 0.166666672 * fTemp122 * self.fRec78[((self.IOTA0 - iTemp108) & 2047) as usize] + 0.0416666679 * fTemp111 * self.fRec78[((self.IOTA0 - iTemp112) & 2047) as usize]);
			let mut fRec76: F32 = self.fVec12[1] + self.fRec69[1];
			self.fRec69[0] = fRec74;
			let mut fRec70: F32 = self.fRec69[1];
			let mut fRec71: F32 = fRec75;
			let mut fRec72: F32 = fRec76;
			self.fRec65[0] = fRec70;
			let mut fRec66: F32 = fTemp126 + fTemp119 + self.fRec65[1];
			let mut fRec67: F32 = fRec71;
			let mut fRec68: F32 = fRec72;
			self.fRec61[(self.IOTA0 & 2047) as usize] = fRec66;
			let mut fRec62: F32 = fTemp114 * self.fRec61[((self.IOTA0 - iTemp87) & 2047) as usize] + fTemp97 * (fTemp120 * self.fRec61[((self.IOTA0 - iTemp99) & 2047) as usize] + 0.5 * fTemp121 * self.fRec61[((self.IOTA0 - iTemp104) & 2047) as usize] + 0.166666672 * fTemp122 * self.fRec61[((self.IOTA0 - iTemp109) & 2047) as usize] + 0.0416666679 * fTemp111 * self.fRec61[((self.IOTA0 - iTemp113) & 2047) as usize]);
			self.fRec63[0] = fRec67;
			let mut fRec64: F32 = fRec68;
			self.fRec59[0] = fSlow21 * self.fRec63[1];
			let mut fRec60: F32 = fRec64;
			self.fRec54[0] = fRec58;
			let mut fRec55: F32 = fSlow21 * self.fRec54[1];
			let mut fRec56: F32 = self.fRec59[0];
			let mut fRec57: F32 = fRec60;
			self.fRec50[(self.IOTA0 & 2047) as usize] = fRec55;
			let mut fRec51: F32 = fRec62;
			let mut fRec52: F32 = fRec56;
			let mut fRec53: F32 = fRec57;
			self.fRec48[0] = fRec51;
			let mut fRec49: F32 = fRec53;
			let mut fTemp128: F32 = F32::abs(fRec49);
			let mut fTemp129: F32 = if (((self.fRec90[1] > fTemp128) as i32) as i32 != 0) { self.fConst18 } else { 0.0 };
			self.fRec91[0] = self.fRec91[1] * fTemp129 + fTemp128 * (1.0 - fTemp129);
			self.fRec90[0] = self.fRec91[0];
			let mut fRec89: F32 = 0.0 - 0.949999988 * F32::max(20.0 * F32::log10(self.fRec90[0]) + 10.0, 0.0);
			self.fRec47[0] = fRec49 * F32::powf(10.0, 0.0500000007 * fRec89);
			self.fRec44[0] = 0.0 - (self.fRec44[1] * (1.0 - fTemp82) - (self.fRec47[0] + self.fRec47[1])) / (fTemp82 + 1.0);
			self.fRec92[0] = fSlow24 + self.fConst2 * self.fRec92[1];
			self.fRec0[(self.IOTA0 & 65535) as usize] = self.fRec1[0] * self.fRec2[0] * ((self.fRec3[2] + self.fRec3[0] + 2.0 * self.fRec3[1]) / fTemp15 + (self.fRec12[2] + self.fRec12[0] + 2.0 * self.fRec12[1]) / fTemp24 + (self.fRec19[2] + self.fRec19[0] + 2.0 * self.fRec19[1]) / fTemp33 + (self.fRec26[2] + self.fRec26[0] + 2.0 * self.fRec26[1]) / fTemp43) + self.fConst6 * self.fRec33[0] * self.fRec34[0] * (self.fRec35[0] * fTemp45 + self.fRec38[0] * fTemp54 + self.fRec40[0] * fTemp63 + self.fRec42[0] * fTemp72) + self.fRec44[0] * self.fRec92[0] + 0.300000012 * self.fRec0[((self.IOTA0 - self.iConst19) & 65535) as usize];
			self.fRec93[0] = fSlow25 + self.fConst2 * self.fRec93[1];
			let mut fTemp130: F32 = self.fRec0[(self.IOTA0 & 65535) as usize] * self.fRec93[0];
			*output0 = ((fTemp130) as F32);
			*output1 = ((fTemp130) as F32);
			self.iVec0[1] = self.iVec0[0];
			self.fRec1[1] = self.fRec1[0];
			self.fRec2[1] = self.fRec2[0];
			self.fRec5[1] = self.fRec5[0];
			self.fRec8[1] = self.fRec8[0];
			self.fRec6[1] = self.fRec6[0];
			self.fRec9[1] = self.fRec9[0];
			self.fRec11[1] = self.fRec11[0];
			self.fRec10[1] = self.fRec10[0];
			self.fRec4[2] = self.fRec4[1];
			self.fRec4[1] = self.fRec4[0];
			self.fRec3[2] = self.fRec3[1];
			self.fRec3[1] = self.fRec3[0];
			self.fRec14[1] = self.fRec14[0];
			self.fRec17[1] = self.fRec17[0];
			self.fRec15[1] = self.fRec15[0];
			self.fRec18[1] = self.fRec18[0];
			self.fRec13[2] = self.fRec13[1];
			self.fRec13[1] = self.fRec13[0];
			self.fRec12[2] = self.fRec12[1];
			self.fRec12[1] = self.fRec12[0];
			self.fRec21[1] = self.fRec21[0];
			self.fRec24[1] = self.fRec24[0];
			self.fRec22[1] = self.fRec22[0];
			self.fRec25[1] = self.fRec25[0];
			self.fRec20[2] = self.fRec20[1];
			self.fRec20[1] = self.fRec20[0];
			self.fRec19[2] = self.fRec19[1];
			self.fRec19[1] = self.fRec19[0];
			self.fRec28[1] = self.fRec28[0];
			self.fRec31[1] = self.fRec31[0];
			self.fRec29[1] = self.fRec29[0];
			self.fRec32[1] = self.fRec32[0];
			self.fRec27[2] = self.fRec27[1];
			self.fRec27[1] = self.fRec27[0];
			self.fRec26[2] = self.fRec26[1];
			self.fRec26[1] = self.fRec26[0];
			self.fRec33[1] = self.fRec33[0];
			self.fRec34[1] = self.fRec34[0];
			self.fRec37[1] = self.fRec37[0];
			self.fRec36[1] = self.fRec36[0];
			self.fVec1[1] = self.fVec1[0];
			self.IOTA0 = self.IOTA0 + 1;
			self.fRec35[1] = self.fRec35[0];
			self.fRec39[1] = self.fRec39[0];
			self.fVec3[1] = self.fVec3[0];
			self.fRec38[1] = self.fRec38[0];
			self.fRec41[1] = self.fRec41[0];
			self.fVec5[1] = self.fVec5[0];
			self.fRec40[1] = self.fRec40[0];
			self.fRec43[1] = self.fRec43[0];
			self.fVec7[1] = self.fVec7[0];
			self.fRec42[1] = self.fRec42[0];
			self.fRec45[1] = self.fRec45[0];
			self.fRec46[1] = self.fRec46[0];
			self.fRec73[1] = self.fRec73[0];
			self.fRec77[1] = self.fRec77[0];
			self.iRec80[1] = self.iRec80[0];
			self.fRec79[2] = self.fRec79[1];
			self.fRec79[1] = self.fRec79[0];
			self.fVec9[1] = self.fVec9[0];
			self.iRec81[1] = self.iRec81[0];
			for j0 in (1..=3).rev() {
				self.fRec82[(j0) as usize] = self.fRec82[(j0 - 1) as usize];
			}
			self.fVec10[1] = self.fVec10[0];
			self.fRec87[1] = self.fRec87[0];
			self.iVec11[1] = self.iVec11[0];
			self.iRec88[1] = self.iRec88[0];
			self.fRec85[1] = self.fRec85[0];
			self.fRec84[1] = self.fRec84[0];
			self.fVec12[2] = self.fVec12[1];
			self.fVec12[1] = self.fVec12[0];
			self.fRec69[1] = self.fRec69[0];
			self.fRec65[1] = self.fRec65[0];
			self.fRec63[1] = self.fRec63[0];
			for j1 in (1..=3).rev() {
				self.fRec59[(j1) as usize] = self.fRec59[(j1 - 1) as usize];
			}
			self.fRec54[1] = self.fRec54[0];
			self.fRec48[1] = self.fRec48[0];
			self.fRec91[1] = self.fRec91[0];
			self.fRec90[1] = self.fRec90[0];
			self.fRec47[1] = self.fRec47[0];
			self.fRec44[1] = self.fRec44[0];
			self.fRec92[1] = self.fRec92[0];
			self.fRec93[1] = self.fRec93[0];
		}
	}

}


}
pub use dsp::mydsp as Instrument;
