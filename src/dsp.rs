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
	fConst3: F32,
	fHslider2: F32,
	fRec7: [F32;2],
	fHslider3: F32,
	fRec8: [F32;2],
	fRec5: [F32;2],
	fHslider4: F32,
	fRec9: [F32;2],
	fHslider5: F32,
	fRec12: [F32;2],
	fRec10: [F32;2],
	fRec13: [F32;2],
	fHslider6: F32,
	fRec15: [F32;2],
	fConst4: F32,
	fHslider7: F32,
	fRec16: [F32;2],
	fRec4: [F32;3],
	fRec3: [F32;3],
	fHslider8: F32,
	fRec17: [F32;2],
	fHslider9: F32,
	fRec22: [F32;2],
	fRec20: [F32;2],
	fRec23: [F32;2],
	fRec25: [F32;2],
	fRec19: [F32;3],
	fRec18: [F32;3],
	fHslider10: F32,
	fRec27: [F32;2],
	fHslider11: F32,
	fRec32: [F32;2],
	fRec30: [F32;2],
	fRec33: [F32;2],
	fRec35: [F32;2],
	fRec29: [F32;3],
	fRec28: [F32;3],
	fHslider12: F32,
	fRec37: [F32;2],
	fHslider13: F32,
	fRec42: [F32;2],
	fRec40: [F32;2],
	fRec43: [F32;2],
	fRec45: [F32;2],
	fRec39: [F32;3],
	fRec38: [F32;3],
	fConst5: F32,
	fHslider14: F32,
	fRec47: [F32;2],
	fConst6: F32,
	fConst7: F32,
	fHslider15: F32,
	fRec49: [F32;2],
	fRec50: [F32;2],
	fVec1: [F32;2],
	IOTA0: i32,
	fVec2: [F32;4096],
	fRec48: [F32;2],
	fRec52: [F32;2],
	fVec3: [F32;2],
	fVec4: [F32;4096],
	fRec51: [F32;2],
	fRec54: [F32;2],
	fVec5: [F32;2],
	fVec6: [F32;4096],
	fRec53: [F32;2],
	fRec56: [F32;2],
	fVec7: [F32;2],
	fVec8: [F32;4096],
	fRec55: [F32;2],
	fConst8: F32,
	fHslider16: F32,
	fRec59: [F32;2],
	fHslider17: F32,
	fHslider18: F32,
	fConst9: F32,
	fConst10: F32,
	fRec85: [F32;2],
	fRec89: [F32;2],
	fRec91: [F32;4],
	fRec92: [F32;2048],
	fVec9: [F32;2],
	fHslider19: F32,
	iRec94: [i32;2],
	fConst11: F32,
	fRec93: [F32;3],
	fButton0: F32,
	fVec10: [F32;2],
	iRec95: [i32;2],
	fConst12: F32,
	fVec11: [F32;3],
	fRec90: [F32;2048],
	fRec81: [F32;2],
	fRec77: [F32;2],
	fRec73: [F32;2048],
	fRec75: [F32;2],
	fRec71: [F32;4],
	fRec66: [F32;2],
	fRec62: [F32;2048],
	fRec60: [F32;2],
	fConst13: F32,
	fRec98: [F32;2],
	fRec97: [F32;2],
	fVec12: [F32;2],
	fRec58: [F32;2],
	fHslider20: F32,
	fRec100: [F32;2],
	fRec99: [F32;2],
	fConst14: F32,
	fConst15: F32,
	fRec101: [F32;2],
	fRec102: [F32;2],
	fRec57: [F32;3],
	iConst16: i32,
	fRec0: [F32;65536],
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
			fConst3: 0.0,
			fHslider2: 0.0,
			fRec7: [0.0;2],
			fHslider3: 0.0,
			fRec8: [0.0;2],
			fRec5: [0.0;2],
			fHslider4: 0.0,
			fRec9: [0.0;2],
			fHslider5: 0.0,
			fRec12: [0.0;2],
			fRec10: [0.0;2],
			fRec13: [0.0;2],
			fHslider6: 0.0,
			fRec15: [0.0;2],
			fConst4: 0.0,
			fHslider7: 0.0,
			fRec16: [0.0;2],
			fRec4: [0.0;3],
			fRec3: [0.0;3],
			fHslider8: 0.0,
			fRec17: [0.0;2],
			fHslider9: 0.0,
			fRec22: [0.0;2],
			fRec20: [0.0;2],
			fRec23: [0.0;2],
			fRec25: [0.0;2],
			fRec19: [0.0;3],
			fRec18: [0.0;3],
			fHslider10: 0.0,
			fRec27: [0.0;2],
			fHslider11: 0.0,
			fRec32: [0.0;2],
			fRec30: [0.0;2],
			fRec33: [0.0;2],
			fRec35: [0.0;2],
			fRec29: [0.0;3],
			fRec28: [0.0;3],
			fHslider12: 0.0,
			fRec37: [0.0;2],
			fHslider13: 0.0,
			fRec42: [0.0;2],
			fRec40: [0.0;2],
			fRec43: [0.0;2],
			fRec45: [0.0;2],
			fRec39: [0.0;3],
			fRec38: [0.0;3],
			fConst5: 0.0,
			fHslider14: 0.0,
			fRec47: [0.0;2],
			fConst6: 0.0,
			fConst7: 0.0,
			fHslider15: 0.0,
			fRec49: [0.0;2],
			fRec50: [0.0;2],
			fVec1: [0.0;2],
			IOTA0: 0,
			fVec2: [0.0;4096],
			fRec48: [0.0;2],
			fRec52: [0.0;2],
			fVec3: [0.0;2],
			fVec4: [0.0;4096],
			fRec51: [0.0;2],
			fRec54: [0.0;2],
			fVec5: [0.0;2],
			fVec6: [0.0;4096],
			fRec53: [0.0;2],
			fRec56: [0.0;2],
			fVec7: [0.0;2],
			fVec8: [0.0;4096],
			fRec55: [0.0;2],
			fConst8: 0.0,
			fHslider16: 0.0,
			fRec59: [0.0;2],
			fHslider17: 0.0,
			fHslider18: 0.0,
			fConst9: 0.0,
			fConst10: 0.0,
			fRec85: [0.0;2],
			fRec89: [0.0;2],
			fRec91: [0.0;4],
			fRec92: [0.0;2048],
			fVec9: [0.0;2],
			fHslider19: 0.0,
			iRec94: [0;2],
			fConst11: 0.0,
			fRec93: [0.0;3],
			fButton0: 0.0,
			fVec10: [0.0;2],
			iRec95: [0;2],
			fConst12: 0.0,
			fVec11: [0.0;3],
			fRec90: [0.0;2048],
			fRec81: [0.0;2],
			fRec77: [0.0;2],
			fRec73: [0.0;2048],
			fRec75: [0.0;2],
			fRec71: [0.0;4],
			fRec66: [0.0;2],
			fRec62: [0.0;2048],
			fRec60: [0.0;2],
			fConst13: 0.0,
			fRec98: [0.0;2],
			fRec97: [0.0;2],
			fVec12: [0.0;2],
			fRec58: [0.0;2],
			fHslider20: 0.0,
			fRec100: [0.0;2],
			fRec99: [0.0;2],
			fConst14: 0.0,
			fConst15: 0.0,
			fRec101: [0.0;2],
			fRec102: [0.0;2],
			fRec57: [0.0;3],
			iConst16: 0,
			fRec0: [0.0;65536],
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
		self.fHslider2 = 60.0;
		self.fHslider3 = 0.0;
		self.fHslider4 = 0.0;
		self.fHslider5 = 0.00100000005;
		self.fHslider6 = 0.0;
		self.fHslider7 = 0.0;
		self.fHslider8 = 0.0;
		self.fHslider9 = 60.0;
		self.fHslider10 = 0.0;
		self.fHslider11 = 60.0;
		self.fHslider12 = 0.0;
		self.fHslider13 = 60.0;
		self.fHslider14 = 0.0;
		self.fHslider15 = 60.0;
		self.fHslider16 = 80.0;
		self.fHslider17 = 100.0;
		self.fHslider18 = 1.0;
		self.fHslider19 = 1.0;
		self.fButton0 = 0.0;
		self.fHslider20 = 0.5;
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
			self.fRec7[(l3) as usize] = 0.0;
		}
		for l4 in 0..2 {
			self.fRec8[(l4) as usize] = 0.0;
		}
		for l5 in 0..2 {
			self.fRec5[(l5) as usize] = 0.0;
		}
		for l6 in 0..2 {
			self.fRec9[(l6) as usize] = 0.0;
		}
		for l7 in 0..2 {
			self.fRec12[(l7) as usize] = 0.0;
		}
		for l8 in 0..2 {
			self.fRec10[(l8) as usize] = 0.0;
		}
		for l9 in 0..2 {
			self.fRec13[(l9) as usize] = 0.0;
		}
		for l10 in 0..2 {
			self.fRec15[(l10) as usize] = 0.0;
		}
		for l11 in 0..2 {
			self.fRec16[(l11) as usize] = 0.0;
		}
		for l12 in 0..3 {
			self.fRec4[(l12) as usize] = 0.0;
		}
		for l13 in 0..3 {
			self.fRec3[(l13) as usize] = 0.0;
		}
		for l14 in 0..2 {
			self.fRec17[(l14) as usize] = 0.0;
		}
		for l15 in 0..2 {
			self.fRec22[(l15) as usize] = 0.0;
		}
		for l16 in 0..2 {
			self.fRec20[(l16) as usize] = 0.0;
		}
		for l17 in 0..2 {
			self.fRec23[(l17) as usize] = 0.0;
		}
		for l18 in 0..2 {
			self.fRec25[(l18) as usize] = 0.0;
		}
		for l19 in 0..3 {
			self.fRec19[(l19) as usize] = 0.0;
		}
		for l20 in 0..3 {
			self.fRec18[(l20) as usize] = 0.0;
		}
		for l21 in 0..2 {
			self.fRec27[(l21) as usize] = 0.0;
		}
		for l22 in 0..2 {
			self.fRec32[(l22) as usize] = 0.0;
		}
		for l23 in 0..2 {
			self.fRec30[(l23) as usize] = 0.0;
		}
		for l24 in 0..2 {
			self.fRec33[(l24) as usize] = 0.0;
		}
		for l25 in 0..2 {
			self.fRec35[(l25) as usize] = 0.0;
		}
		for l26 in 0..3 {
			self.fRec29[(l26) as usize] = 0.0;
		}
		for l27 in 0..3 {
			self.fRec28[(l27) as usize] = 0.0;
		}
		for l28 in 0..2 {
			self.fRec37[(l28) as usize] = 0.0;
		}
		for l29 in 0..2 {
			self.fRec42[(l29) as usize] = 0.0;
		}
		for l30 in 0..2 {
			self.fRec40[(l30) as usize] = 0.0;
		}
		for l31 in 0..2 {
			self.fRec43[(l31) as usize] = 0.0;
		}
		for l32 in 0..2 {
			self.fRec45[(l32) as usize] = 0.0;
		}
		for l33 in 0..3 {
			self.fRec39[(l33) as usize] = 0.0;
		}
		for l34 in 0..3 {
			self.fRec38[(l34) as usize] = 0.0;
		}
		for l35 in 0..2 {
			self.fRec47[(l35) as usize] = 0.0;
		}
		for l36 in 0..2 {
			self.fRec49[(l36) as usize] = 0.0;
		}
		for l37 in 0..2 {
			self.fRec50[(l37) as usize] = 0.0;
		}
		for l38 in 0..2 {
			self.fVec1[(l38) as usize] = 0.0;
		}
		self.IOTA0 = 0;
		for l39 in 0..4096 {
			self.fVec2[(l39) as usize] = 0.0;
		}
		for l40 in 0..2 {
			self.fRec48[(l40) as usize] = 0.0;
		}
		for l41 in 0..2 {
			self.fRec52[(l41) as usize] = 0.0;
		}
		for l42 in 0..2 {
			self.fVec3[(l42) as usize] = 0.0;
		}
		for l43 in 0..4096 {
			self.fVec4[(l43) as usize] = 0.0;
		}
		for l44 in 0..2 {
			self.fRec51[(l44) as usize] = 0.0;
		}
		for l45 in 0..2 {
			self.fRec54[(l45) as usize] = 0.0;
		}
		for l46 in 0..2 {
			self.fVec5[(l46) as usize] = 0.0;
		}
		for l47 in 0..4096 {
			self.fVec6[(l47) as usize] = 0.0;
		}
		for l48 in 0..2 {
			self.fRec53[(l48) as usize] = 0.0;
		}
		for l49 in 0..2 {
			self.fRec56[(l49) as usize] = 0.0;
		}
		for l50 in 0..2 {
			self.fVec7[(l50) as usize] = 0.0;
		}
		for l51 in 0..4096 {
			self.fVec8[(l51) as usize] = 0.0;
		}
		for l52 in 0..2 {
			self.fRec55[(l52) as usize] = 0.0;
		}
		for l53 in 0..2 {
			self.fRec59[(l53) as usize] = 0.0;
		}
		for l54 in 0..2 {
			self.fRec85[(l54) as usize] = 0.0;
		}
		for l55 in 0..2 {
			self.fRec89[(l55) as usize] = 0.0;
		}
		for l56 in 0..4 {
			self.fRec91[(l56) as usize] = 0.0;
		}
		for l57 in 0..2048 {
			self.fRec92[(l57) as usize] = 0.0;
		}
		for l58 in 0..2 {
			self.fVec9[(l58) as usize] = 0.0;
		}
		for l59 in 0..2 {
			self.iRec94[(l59) as usize] = 0;
		}
		for l60 in 0..3 {
			self.fRec93[(l60) as usize] = 0.0;
		}
		for l61 in 0..2 {
			self.fVec10[(l61) as usize] = 0.0;
		}
		for l62 in 0..2 {
			self.iRec95[(l62) as usize] = 0;
		}
		for l63 in 0..3 {
			self.fVec11[(l63) as usize] = 0.0;
		}
		for l64 in 0..2048 {
			self.fRec90[(l64) as usize] = 0.0;
		}
		for l65 in 0..2 {
			self.fRec81[(l65) as usize] = 0.0;
		}
		for l66 in 0..2 {
			self.fRec77[(l66) as usize] = 0.0;
		}
		for l67 in 0..2048 {
			self.fRec73[(l67) as usize] = 0.0;
		}
		for l68 in 0..2 {
			self.fRec75[(l68) as usize] = 0.0;
		}
		for l69 in 0..4 {
			self.fRec71[(l69) as usize] = 0.0;
		}
		for l70 in 0..2 {
			self.fRec66[(l70) as usize] = 0.0;
		}
		for l71 in 0..2048 {
			self.fRec62[(l71) as usize] = 0.0;
		}
		for l72 in 0..2 {
			self.fRec60[(l72) as usize] = 0.0;
		}
		for l73 in 0..2 {
			self.fRec98[(l73) as usize] = 0.0;
		}
		for l74 in 0..2 {
			self.fRec97[(l74) as usize] = 0.0;
		}
		for l75 in 0..2 {
			self.fVec12[(l75) as usize] = 0.0;
		}
		for l76 in 0..2 {
			self.fRec58[(l76) as usize] = 0.0;
		}
		for l77 in 0..2 {
			self.fRec100[(l77) as usize] = 0.0;
		}
		for l78 in 0..2 {
			self.fRec99[(l78) as usize] = 0.0;
		}
		for l79 in 0..2 {
			self.fRec101[(l79) as usize] = 0.0;
		}
		for l80 in 0..2 {
			self.fRec102[(l80) as usize] = 0.0;
		}
		for l81 in 0..3 {
			self.fRec57[(l81) as usize] = 0.0;
		}
		for l82 in 0..65536 {
			self.fRec0[(l82) as usize] = 0.0;
		}
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		self.fConst0 = F32::min(192000.0, F32::max(1.0, ((self.fSampleRate) as F32)));
		self.fConst1 = 44.0999985 / self.fConst0;
		self.fConst2 = 1.0 - self.fConst1;
		self.fConst3 = 1.0 / self.fConst0;
		self.fConst4 = 3.14159274 / self.fConst0;
		self.fConst5 = 352.0 / self.fConst0;
		self.fConst6 = 0.25 * self.fConst0;
		self.fConst7 = 0.5 * self.fConst0;
		self.fConst8 = 2764.60156 / self.fConst0;
		self.fConst9 = 0.00882352982 * self.fConst0;
		self.fConst10 = 0.000735294132 * self.fConst0;
		self.fConst11 = 6911.50391 / self.fConst0;
		self.fConst12 = 0.00200000009 * self.fConst0;
		self.fConst13 = F32::exp(0.0 - 10.0 / self.fConst0);
		self.fConst14 = 1413.71667 / self.fConst0;
		self.fConst15 = 2827.43335 / self.fConst0;
		self.iConst16 = ((F32::min(self.fConst0, F32::max(0.0, 0.300000012 * self.fConst0))) as i32) + 1;
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
		ui_interface.add_horizontal_slider("vol", ParamIndex(0), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(None, "1", "");
		ui_interface.open_horizontal_box("supersaw");
		ui_interface.declare(Some(ParamIndex(1)), "0", "");
		ui_interface.add_horizontal_slider("amount", ParamIndex(1), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(2)), "1", "");
		ui_interface.add_horizontal_slider("detune", ParamIndex(2), 0.001, 0.001, 0.02, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "2", "");
		ui_interface.open_horizontal_box("filter");
		ui_interface.declare(Some(ParamIndex(3)), "0", "");
		ui_interface.add_horizontal_slider("cutoffNote", ParamIndex(3), 0.0, -20.0, 50.0, 0.001);
		ui_interface.declare(Some(ParamIndex(4)), "1", "");
		ui_interface.add_horizontal_slider("res", ParamIndex(4), 0.0, 0.0, 0.98999999999999999, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "3", "");
		ui_interface.open_vertical_box("chord");
		ui_interface.declare(Some(ParamIndex(5)), "1", "");
		ui_interface.add_horizontal_slider("vol1", ParamIndex(5), 1.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(6)), "2", "");
		ui_interface.add_horizontal_slider("note1", ParamIndex(6), 60.0, 0.0, 127.0, 0.001);
		ui_interface.declare(Some(ParamIndex(7)), "3", "");
		ui_interface.add_horizontal_slider("vol2", ParamIndex(7), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(8)), "4", "");
		ui_interface.add_horizontal_slider("note2", ParamIndex(8), 60.0, 0.0, 127.0, 0.001);
		ui_interface.declare(Some(ParamIndex(9)), "5", "");
		ui_interface.add_horizontal_slider("vol3", ParamIndex(9), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(10)), "6", "");
		ui_interface.add_horizontal_slider("note3", ParamIndex(10), 60.0, 0.0, 127.0, 0.001);
		ui_interface.declare(Some(ParamIndex(11)), "7", "");
		ui_interface.add_horizontal_slider("vol4", ParamIndex(11), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(12)), "8", "");
		ui_interface.add_horizontal_slider("note4", ParamIndex(12), 60.0, 0.0, 127.0, 0.001);
		ui_interface.close_box();
		ui_interface.close_box();
		ui_interface.declare(None, "1", "");
		ui_interface.open_horizontal_box("pluck");
		ui_interface.declare(Some(ParamIndex(13)), "0", "");
		ui_interface.add_button("gate", ParamIndex(13));
		ui_interface.declare(Some(ParamIndex(14)), "1", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(14), 80.0, 0.0, 127.0, 0.001);
		ui_interface.declare(Some(ParamIndex(15)), "1", "");
		ui_interface.add_horizontal_slider("wah", ParamIndex(15), 0.5, 0.25, 0.75, 0.001);
		ui_interface.declare(Some(ParamIndex(16)), "2", "");
		ui_interface.add_horizontal_slider("gain", ParamIndex(16), 100.0, 0.0, 100.0, 0.001);
		ui_interface.declare(Some(ParamIndex(17)), "3", "");
		ui_interface.add_horizontal_slider("mute", ParamIndex(17), 1.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(18)), "4", "");
		ui_interface.add_horizontal_slider("strength", ParamIndex(18), 1.0, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "2", "");
		ui_interface.open_horizontal_box("drone");
		ui_interface.declare(Some(ParamIndex(19)), "0", "");
		ui_interface.add_horizontal_slider("volume", ParamIndex(19), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(20)), "1", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(20), 60.0, 0.0, 127.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(Some(ParamIndex(21)), "3", "");
		ui_interface.add_horizontal_slider("pitchBend", ParamIndex(21), 0.0, -1.0, 1.0, 0.001);
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			13 => Some(self.fButton0),
			0 => Some(self.fHslider0),
			5 => Some(self.fHslider1),
			9 => Some(self.fHslider10),
			10 => Some(self.fHslider11),
			11 => Some(self.fHslider12),
			12 => Some(self.fHslider13),
			19 => Some(self.fHslider14),
			20 => Some(self.fHslider15),
			14 => Some(self.fHslider16),
			16 => Some(self.fHslider17),
			17 => Some(self.fHslider18),
			18 => Some(self.fHslider19),
			6 => Some(self.fHslider2),
			15 => Some(self.fHslider20),
			21 => Some(self.fHslider3),
			1 => Some(self.fHslider4),
			2 => Some(self.fHslider5),
			4 => Some(self.fHslider6),
			3 => Some(self.fHslider7),
			7 => Some(self.fHslider8),
			8 => Some(self.fHslider9),
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			13 => { self.fButton0 = value }
			0 => { self.fHslider0 = value }
			5 => { self.fHslider1 = value }
			9 => { self.fHslider10 = value }
			10 => { self.fHslider11 = value }
			11 => { self.fHslider12 = value }
			12 => { self.fHslider13 = value }
			19 => { self.fHslider14 = value }
			20 => { self.fHslider15 = value }
			14 => { self.fHslider16 = value }
			16 => { self.fHslider17 = value }
			17 => { self.fHslider18 = value }
			18 => { self.fHslider19 = value }
			6 => { self.fHslider2 = value }
			15 => { self.fHslider20 = value }
			21 => { self.fHslider3 = value }
			1 => { self.fHslider4 = value }
			2 => { self.fHslider5 = value }
			4 => { self.fHslider6 = value }
			3 => { self.fHslider7 = value }
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
		let mut fSlow0: F32 = self.fConst1 * ((self.fHslider0) as F32);
		let mut fSlow1: F32 = self.fConst1 * ((self.fHslider1) as F32);
		let mut fSlow2: F32 = self.fConst1 * ((self.fHslider2) as F32);
		let mut fSlow3: F32 = self.fConst1 * ((self.fHslider3) as F32);
		let mut fSlow4: F32 = self.fConst1 * ((self.fHslider4) as F32);
		let mut fSlow5: F32 = self.fConst1 * ((self.fHslider5) as F32);
		let mut fSlow6: F32 = self.fConst1 * ((self.fHslider6) as F32);
		let mut fSlow7: F32 = self.fConst1 * ((self.fHslider7) as F32);
		let mut fSlow8: F32 = self.fConst1 * ((self.fHslider8) as F32);
		let mut fSlow9: F32 = self.fConst1 * ((self.fHslider9) as F32);
		let mut fSlow10: F32 = self.fConst1 * ((self.fHslider10) as F32);
		let mut fSlow11: F32 = self.fConst1 * ((self.fHslider11) as F32);
		let mut fSlow12: F32 = self.fConst1 * ((self.fHslider12) as F32);
		let mut fSlow13: F32 = self.fConst1 * ((self.fHslider13) as F32);
		let mut fSlow14: F32 = self.fConst1 * ((self.fHslider14) as F32);
		let mut fSlow15: F32 = self.fConst1 * ((self.fHslider15) as F32);
		let mut fSlow16: F32 = self.fConst1 * ((self.fHslider16) as F32);
		let mut fSlow17: F32 = ((self.fHslider17) as F32);
		let mut fSlow18: F32 = ((self.fHslider18) as F32);
		let mut fSlow19: F32 = ((self.fHslider19) as F32);
		let mut fSlow20: F32 = ((self.fButton0) as F32);
		let mut fSlow21: F32 = self.fConst1 * ((self.fHslider20) as F32);
		let zipped_iterators = outputs0.zip(outputs1);
		for (output0, output1) in zipped_iterators {
			self.iVec0[0] = 1;
			self.fRec1[0] = fSlow0 + self.fConst2 * self.fRec1[1];
			self.fRec2[0] = fSlow1 + self.fConst2 * self.fRec2[1];
			self.fRec7[0] = fSlow2 + self.fConst2 * self.fRec7[1];
			self.fRec8[0] = fSlow3 + self.fConst2 * self.fRec8[1];
			let mut fTemp0: F32 = self.fRec7[0] + self.fRec8[0];
			let mut fTemp1: F32 = F32::max(1.1920929e-07, F32::abs(440.0 * F32::powf(2.0, 0.0833333358 * (fTemp0 + -69.0))));
			let mut fTemp2: F32 = self.fConst3 * fTemp1;
			let mut fTemp3: F32 = self.fRec5[1] + fTemp2;
			let mut fTemp4: F32 = fTemp3 + -1.0;
			let mut iTemp5: i32 = ((fTemp4 < 0.0) as i32);
			self.fRec5[0] = if (iTemp5 as i32 != 0) { fTemp3 } else { fTemp4 };
			let mut fThen1: F32 = self.fRec5[1] + fTemp2 + fTemp4 * (1.0 - self.fConst0 / fTemp1);
			let mut fRec6: F32 = if (iTemp5 as i32 != 0) { fTemp3 } else { fThen1 };
			self.fRec9[0] = fSlow4 + self.fConst2 * self.fRec9[1];
			self.fRec12[0] = fSlow5 + self.fConst2 * self.fRec12[1];
			let mut fTemp6: F32 = F32::max(1.1920929e-07, F32::abs(440.0 * F32::powf(2.0, 0.0833333358 * (fTemp0 + self.fRec12[0] + -69.0))));
			let mut fTemp7: F32 = self.fConst3 * fTemp6;
			let mut fTemp8: F32 = self.fRec10[1] + fTemp7;
			let mut fTemp9: F32 = fTemp8 + -1.0;
			let mut iTemp10: i32 = ((fTemp9 < 0.0) as i32);
			self.fRec10[0] = if (iTemp10 as i32 != 0) { fTemp8 } else { fTemp9 };
			let mut fThen3: F32 = self.fRec10[1] + fTemp7 + fTemp9 * (1.0 - self.fConst0 / fTemp6);
			let mut fRec11: F32 = if (iTemp10 as i32 != 0) { fTemp8 } else { fThen3 };
			let mut fTemp11: F32 = -69.0 - self.fRec12[0];
			let mut fTemp12: F32 = F32::max(1.1920929e-07, F32::abs(440.0 * F32::powf(2.0, 0.0833333358 * (fTemp0 + fTemp11))));
			let mut fTemp13: F32 = self.fRec13[1] + self.fConst3 * fTemp12;
			let mut fTemp14: F32 = fTemp13 + -1.0;
			let mut iTemp15: i32 = ((fTemp14 < 0.0) as i32);
			self.fRec13[0] = if (iTemp15 as i32 != 0) { fTemp13 } else { fTemp14 };
			let mut fThen5: F32 = fTemp13 + fTemp14 * (1.0 - self.fConst0 / fTemp12);
			let mut fRec14: F32 = if (iTemp15 as i32 != 0) { fTemp13 } else { fThen5 };
			self.fRec15[0] = fSlow6 + self.fConst2 * self.fRec15[1];
			let mut fTemp16: F32 = F32::min(1.41419947, 1.41421354 * self.fRec15[0]);
			let mut fTemp17: F32 = fTemp16 * (fTemp16 + 1.41421354);
			self.fRec16[0] = fSlow7 + self.fConst2 * self.fRec16[1];
			let mut fTemp18: F32 = F32::tan(self.fConst4 * F32::max(20.0, F32::min(10000.0, 440.0 * F32::powf(2.0, 0.0833333358 * (fTemp0 + self.fRec16[0] + -69.0)))));
			let mut fTemp19: F32 = 1.0 / fTemp18;
			let mut fTemp20: F32 = 1.41421354 * fTemp16;
			let mut fTemp21: F32 = fTemp20 + 2.0;
			let mut fTemp22: F32 = 1.0 - 1.0 / mydsp_faustpower2_f(fTemp18);
			let mut fTemp23: F32 = fTemp17 + (fTemp19 + fTemp21) / fTemp18 + 1.0;
			self.fRec4[0] = 2.0 * fRec6 + self.fRec9[0] * (0.0 - 2.0 * (1.0 - (fRec11 + fRec14))) + -1.0 - (self.fRec4[2] * (fTemp17 + (fTemp19 - fTemp21) / fTemp18 + 1.0) + 2.0 * self.fRec4[1] * (fTemp17 + fTemp22)) / fTemp23;
			let mut fTemp24: F32 = fTemp16 * (fTemp16 + -1.41421354);
			let mut fTemp25: F32 = 2.0 - fTemp20;
			let mut fTemp26: F32 = fTemp24 + (fTemp25 + fTemp19) / fTemp18 + 1.0;
			self.fRec3[0] = (self.fRec4[2] + self.fRec4[0] + 2.0 * self.fRec4[1]) / fTemp23 - (self.fRec3[2] * (fTemp24 + (fTemp19 - fTemp25) / fTemp18 + 1.0) + 2.0 * self.fRec3[1] * (fTemp24 + fTemp22)) / fTemp26;
			self.fRec17[0] = fSlow8 + self.fConst2 * self.fRec17[1];
			self.fRec22[0] = fSlow9 + self.fConst2 * self.fRec22[1];
			let mut fTemp27: F32 = self.fRec8[0] + self.fRec22[0];
			let mut fTemp28: F32 = F32::max(1.1920929e-07, F32::abs(440.0 * F32::powf(2.0, 0.0833333358 * (fTemp27 + -69.0))));
			let mut fTemp29: F32 = self.fRec20[1] + self.fConst3 * fTemp28;
			let mut fTemp30: F32 = fTemp29 + -1.0;
			let mut iTemp31: i32 = ((fTemp30 < 0.0) as i32);
			self.fRec20[0] = if (iTemp31 as i32 != 0) { fTemp29 } else { fTemp30 };
			let mut fThen7: F32 = fTemp29 + fTemp30 * (1.0 - self.fConst0 / fTemp28);
			let mut fRec21: F32 = if (iTemp31 as i32 != 0) { fTemp29 } else { fThen7 };
			let mut fTemp32: F32 = self.fRec8[0] + self.fRec12[0];
			let mut fTemp33: F32 = F32::max(1.1920929e-07, F32::abs(440.0 * F32::powf(2.0, 0.0833333358 * (self.fRec22[0] + fTemp32 + -69.0))));
			let mut fTemp34: F32 = self.fConst3 * fTemp33;
			let mut fTemp35: F32 = self.fRec23[1] + fTemp34;
			let mut fTemp36: F32 = fTemp35 + -1.0;
			let mut iTemp37: i32 = ((fTemp36 < 0.0) as i32);
			self.fRec23[0] = if (iTemp37 as i32 != 0) { fTemp35 } else { fTemp36 };
			let mut fThen9: F32 = fTemp34 + self.fRec23[1] + fTemp36 * (1.0 - self.fConst0 / fTemp33);
			let mut fRec24: F32 = if (iTemp37 as i32 != 0) { fTemp35 } else { fThen9 };
			let mut fTemp38: F32 = F32::max(1.1920929e-07, F32::abs(440.0 * F32::powf(2.0, 0.0833333358 * (fTemp27 + fTemp11))));
			let mut fTemp39: F32 = self.fConst3 * fTemp38;
			let mut fTemp40: F32 = self.fRec25[1] + fTemp39;
			let mut fTemp41: F32 = fTemp40 + -1.0;
			let mut iTemp42: i32 = ((fTemp41 < 0.0) as i32);
			self.fRec25[0] = if (iTemp42 as i32 != 0) { fTemp40 } else { fTemp41 };
			let mut fThen11: F32 = fTemp39 + self.fRec25[1] + fTemp41 * (1.0 - self.fConst0 / fTemp38);
			let mut fRec26: F32 = if (iTemp42 as i32 != 0) { fTemp40 } else { fThen11 };
			let mut fTemp43: F32 = self.fRec8[0] + self.fRec16[0];
			let mut fTemp44: F32 = F32::tan(self.fConst4 * F32::max(20.0, F32::min(10000.0, 440.0 * F32::powf(2.0, 0.0833333358 * (self.fRec22[0] + fTemp43 + -69.0)))));
			let mut fTemp45: F32 = 1.0 / fTemp44;
			let mut fTemp46: F32 = 1.0 - 1.0 / mydsp_faustpower2_f(fTemp44);
			let mut fTemp47: F32 = fTemp17 + (fTemp21 + fTemp45) / fTemp44 + 1.0;
			self.fRec19[0] = 2.0 * fRec21 + self.fRec9[0] * (0.0 - 2.0 * (1.0 - (fRec24 + fRec26))) + -1.0 - (self.fRec19[2] * (fTemp17 + 1.0 - (fTemp21 - fTemp45) / fTemp44) + 2.0 * self.fRec19[1] * (fTemp17 + fTemp46)) / fTemp47;
			let mut fTemp48: F32 = fTemp24 + (fTemp25 + fTemp45) / fTemp44 + 1.0;
			self.fRec18[0] = (self.fRec19[2] + self.fRec19[0] + 2.0 * self.fRec19[1]) / fTemp47 - (self.fRec18[2] * (fTemp24 + (fTemp45 - fTemp25) / fTemp44 + 1.0) + 2.0 * self.fRec18[1] * (fTemp24 + fTemp46)) / fTemp48;
			self.fRec27[0] = fSlow10 + self.fConst2 * self.fRec27[1];
			self.fRec32[0] = fSlow11 + self.fConst2 * self.fRec32[1];
			let mut fTemp49: F32 = self.fRec8[0] + self.fRec32[0];
			let mut fTemp50: F32 = F32::max(1.1920929e-07, F32::abs(440.0 * F32::powf(2.0, 0.0833333358 * (fTemp49 + -69.0))));
			let mut fTemp51: F32 = self.fRec30[1] + self.fConst3 * fTemp50;
			let mut fTemp52: F32 = fTemp51 + -1.0;
			let mut iTemp53: i32 = ((fTemp52 < 0.0) as i32);
			self.fRec30[0] = if (iTemp53 as i32 != 0) { fTemp51 } else { fTemp52 };
			let mut fThen13: F32 = fTemp51 + fTemp52 * (1.0 - self.fConst0 / fTemp50);
			let mut fRec31: F32 = if (iTemp53 as i32 != 0) { fTemp51 } else { fThen13 };
			let mut fTemp54: F32 = F32::max(1.1920929e-07, F32::abs(440.0 * F32::powf(2.0, 0.0833333358 * (self.fRec32[0] + fTemp32 + -69.0))));
			let mut fTemp55: F32 = self.fRec33[1] + self.fConst3 * fTemp54;
			let mut fTemp56: F32 = fTemp55 + -1.0;
			let mut iTemp57: i32 = ((fTemp56 < 0.0) as i32);
			self.fRec33[0] = if (iTemp57 as i32 != 0) { fTemp55 } else { fTemp56 };
			let mut fThen15: F32 = fTemp55 + fTemp56 * (1.0 - self.fConst0 / fTemp54);
			let mut fRec34: F32 = if (iTemp57 as i32 != 0) { fTemp55 } else { fThen15 };
			let mut fTemp58: F32 = F32::max(1.1920929e-07, F32::abs(440.0 * F32::powf(2.0, 0.0833333358 * (fTemp49 + fTemp11))));
			let mut fTemp59: F32 = self.fRec35[1] + self.fConst3 * fTemp58;
			let mut fTemp60: F32 = fTemp59 + -1.0;
			let mut iTemp61: i32 = ((fTemp60 < 0.0) as i32);
			self.fRec35[0] = if (iTemp61 as i32 != 0) { fTemp59 } else { fTemp60 };
			let mut fThen17: F32 = fTemp59 + fTemp60 * (1.0 - self.fConst0 / fTemp58);
			let mut fRec36: F32 = if (iTemp61 as i32 != 0) { fTemp59 } else { fThen17 };
			let mut fTemp62: F32 = F32::tan(self.fConst4 * F32::max(20.0, F32::min(10000.0, 440.0 * F32::powf(2.0, 0.0833333358 * (self.fRec32[0] + fTemp43 + -69.0)))));
			let mut fTemp63: F32 = 1.0 / fTemp62;
			let mut fTemp64: F32 = 1.0 - 1.0 / mydsp_faustpower2_f(fTemp62);
			let mut fTemp65: F32 = fTemp17 + (fTemp21 + fTemp63) / fTemp62 + 1.0;
			self.fRec29[0] = 2.0 * fRec31 + self.fRec9[0] * (0.0 - 2.0 * (1.0 - (fRec34 + fRec36))) + -1.0 - (self.fRec29[2] * (fTemp17 + 1.0 - (fTemp21 - fTemp63) / fTemp62) + 2.0 * self.fRec29[1] * (fTemp17 + fTemp64)) / fTemp65;
			let mut fTemp66: F32 = fTemp24 + (fTemp25 + fTemp63) / fTemp62 + 1.0;
			self.fRec28[0] = (self.fRec29[2] + self.fRec29[0] + 2.0 * self.fRec29[1]) / fTemp65 - (self.fRec28[2] * (fTemp24 + (fTemp63 - fTemp25) / fTemp62 + 1.0) + 2.0 * self.fRec28[1] * (fTemp24 + fTemp64)) / fTemp66;
			self.fRec37[0] = fSlow12 + self.fConst2 * self.fRec37[1];
			self.fRec42[0] = fSlow13 + self.fConst2 * self.fRec42[1];
			let mut fTemp67: F32 = self.fRec8[0] + self.fRec42[0];
			let mut fTemp68: F32 = F32::max(1.1920929e-07, F32::abs(440.0 * F32::powf(2.0, 0.0833333358 * (fTemp67 + -69.0))));
			let mut fTemp69: F32 = self.fConst3 * fTemp68;
			let mut fTemp70: F32 = self.fRec40[1] + fTemp69;
			let mut fTemp71: F32 = fTemp70 + -1.0;
			let mut iTemp72: i32 = ((fTemp71 < 0.0) as i32);
			self.fRec40[0] = if (iTemp72 as i32 != 0) { fTemp70 } else { fTemp71 };
			let mut fThen19: F32 = fTemp69 + self.fRec40[1] + fTemp71 * (1.0 - self.fConst0 / fTemp68);
			let mut fRec41: F32 = if (iTemp72 as i32 != 0) { fTemp70 } else { fThen19 };
			let mut fTemp73: F32 = F32::max(1.1920929e-07, F32::abs(440.0 * F32::powf(2.0, 0.0833333358 * (self.fRec42[0] + fTemp32 + -69.0))));
			let mut fTemp74: F32 = self.fRec43[1] + self.fConst3 * fTemp73;
			let mut fTemp75: F32 = fTemp74 + -1.0;
			let mut iTemp76: i32 = ((fTemp75 < 0.0) as i32);
			self.fRec43[0] = if (iTemp76 as i32 != 0) { fTemp74 } else { fTemp75 };
			let mut fThen21: F32 = fTemp74 + fTemp75 * (1.0 - self.fConst0 / fTemp73);
			let mut fRec44: F32 = if (iTemp76 as i32 != 0) { fTemp74 } else { fThen21 };
			let mut fTemp77: F32 = F32::max(1.1920929e-07, F32::abs(440.0 * F32::powf(2.0, 0.0833333358 * (fTemp67 + fTemp11))));
			let mut fTemp78: F32 = self.fConst3 * fTemp77;
			let mut fTemp79: F32 = self.fRec45[1] + fTemp78;
			let mut fTemp80: F32 = fTemp79 + -1.0;
			let mut iTemp81: i32 = ((fTemp80 < 0.0) as i32);
			self.fRec45[0] = if (iTemp81 as i32 != 0) { fTemp79 } else { fTemp80 };
			let mut fThen23: F32 = fTemp78 + self.fRec45[1] + fTemp80 * (1.0 - self.fConst0 / fTemp77);
			let mut fRec46: F32 = if (iTemp81 as i32 != 0) { fTemp79 } else { fThen23 };
			let mut fTemp82: F32 = F32::tan(self.fConst4 * F32::max(20.0, F32::min(10000.0, 440.0 * F32::powf(2.0, 0.0833333358 * (self.fRec42[0] + fTemp43 + -69.0)))));
			let mut fTemp83: F32 = 1.0 / fTemp82;
			let mut fTemp84: F32 = 1.0 - 1.0 / mydsp_faustpower2_f(fTemp82);
			let mut fTemp85: F32 = fTemp17 + (fTemp21 + fTemp83) / fTemp82 + 1.0;
			self.fRec39[0] = 2.0 * fRec41 + self.fRec9[0] * (0.0 - 2.0 * (1.0 - (fRec44 + fRec46))) + -1.0 - (self.fRec39[2] * (fTemp17 + 1.0 - (fTemp21 - fTemp83) / fTemp82) + 2.0 * self.fRec39[1] * (fTemp17 + fTemp84)) / fTemp85;
			let mut fTemp86: F32 = fTemp24 + (fTemp25 + fTemp83) / fTemp82 + 1.0;
			self.fRec38[0] = (self.fRec39[2] + self.fRec39[0] + 2.0 * self.fRec39[1]) / fTemp85 - (self.fRec38[2] * (fTemp24 + (fTemp83 - fTemp25) / fTemp82 + 1.0) + 2.0 * self.fRec38[1] * (fTemp24 + fTemp84)) / fTemp86;
			self.fRec47[0] = fSlow14 + self.fConst2 * self.fRec47[1];
			self.fRec49[0] = fSlow15 + self.fConst2 * self.fRec49[1];
			let mut fTemp87: F32 = F32::powf(2.0, 0.0833333358 * (self.fRec49[0] + -69.0));
			let mut fTemp88: F32 = F32::max(440.0 * fTemp87, 23.4489498);
			let mut fTemp89: F32 = F32::max(0.0, F32::min(2047.0, self.fConst7 / fTemp88));
			let mut fTemp90: F32 = F32::floor(fTemp89);
			let mut fTemp91: F32 = ((self.iVec0[1]) as F32);
			let mut fTemp92: F32 = F32::max(20.0, F32::abs(fTemp88));
			let mut fTemp93: F32 = self.fRec50[1] + self.fConst3 * fTemp92;
			self.fRec50[0] = fTemp93 - F32::floor(fTemp93);
			let mut fTemp94: F32 = mydsp_faustpower2_f(2.0 * self.fRec50[0] + -1.0);
			self.fVec1[0] = fTemp94;
			let mut fTemp95: F32 = (fTemp91 * (fTemp94 - self.fVec1[1])) / fTemp92;
			self.fVec2[(self.IOTA0 & 4095) as usize] = fTemp95;
			let mut iTemp96: i32 = ((fTemp89) as i32);
			self.fRec48[0] = 0.999000013 * self.fRec48[1] - self.fConst6 * ((fTemp89 - fTemp90) * self.fVec2[((self.IOTA0 - (iTemp96 + 1)) & 4095) as usize] - fTemp95 - self.fVec2[((self.IOTA0 - iTemp96) & 4095) as usize] * (fTemp90 + 1.0 - fTemp89));
			let mut fTemp97: F32 = F32::powf(2.0, 0.0833333358 * (self.fRec49[0] + -56.9000015));
			let mut fTemp98: F32 = F32::max(440.0 * fTemp97, 23.4489498);
			let mut fTemp99: F32 = F32::max(20.0, F32::abs(fTemp98));
			let mut fTemp100: F32 = self.fRec52[1] + self.fConst3 * fTemp99;
			self.fRec52[0] = fTemp100 - F32::floor(fTemp100);
			let mut fTemp101: F32 = mydsp_faustpower2_f(2.0 * self.fRec52[0] + -1.0);
			self.fVec3[0] = fTemp101;
			let mut fTemp102: F32 = (fTemp91 * (fTemp101 - self.fVec3[1])) / fTemp99;
			self.fVec4[(self.IOTA0 & 4095) as usize] = fTemp102;
			let mut fTemp103: F32 = F32::max(0.0, F32::min(2047.0, self.fConst7 / fTemp98));
			let mut iTemp104: i32 = ((fTemp103) as i32);
			let mut fTemp105: F32 = F32::floor(fTemp103);
			self.fRec51[0] = 0.999000013 * self.fRec51[1] + self.fConst6 * (fTemp102 - self.fVec4[((self.IOTA0 - iTemp104) & 4095) as usize] * (fTemp105 + 1.0 - fTemp103) - (fTemp103 - fTemp105) * self.fVec4[((self.IOTA0 - (iTemp104 + 1)) & 4095) as usize]);
			let mut fTemp106: F32 = F32::powf(2.0, 0.0833333358 * (self.fRec49[0] + -81.1100006));
			let mut fTemp107: F32 = F32::max(440.0 * fTemp106, 23.4489498);
			let mut fTemp108: F32 = F32::max(20.0, F32::abs(fTemp107));
			let mut fTemp109: F32 = self.fRec54[1] + self.fConst3 * fTemp108;
			self.fRec54[0] = fTemp109 - F32::floor(fTemp109);
			let mut fTemp110: F32 = mydsp_faustpower2_f(2.0 * self.fRec54[0] + -1.0);
			self.fVec5[0] = fTemp110;
			let mut fTemp111: F32 = (fTemp91 * (fTemp110 - self.fVec5[1])) / fTemp108;
			self.fVec6[(self.IOTA0 & 4095) as usize] = fTemp111;
			let mut fTemp112: F32 = F32::max(0.0, F32::min(2047.0, self.fConst7 / fTemp107));
			let mut iTemp113: i32 = ((fTemp112) as i32);
			let mut fTemp114: F32 = F32::floor(fTemp112);
			self.fRec53[0] = 0.999000013 * self.fRec53[1] + self.fConst6 * (fTemp111 - self.fVec6[((self.IOTA0 - iTemp113) & 4095) as usize] * (fTemp114 + 1.0 - fTemp112) - (fTemp112 - fTemp114) * self.fVec6[((self.IOTA0 - (iTemp113 + 1)) & 4095) as usize]);
			let mut fTemp115: F32 = F32::powf(2.0, 0.0833333358 * (self.fRec49[0] + -61.8800011));
			let mut fTemp116: F32 = F32::max(440.0 * fTemp115, 23.4489498);
			let mut fTemp117: F32 = F32::max(20.0, F32::abs(fTemp116));
			let mut fTemp118: F32 = self.fRec56[1] + self.fConst3 * fTemp117;
			self.fRec56[0] = fTemp118 - F32::floor(fTemp118);
			let mut fTemp119: F32 = mydsp_faustpower2_f(2.0 * self.fRec56[0] + -1.0);
			self.fVec7[0] = fTemp119;
			let mut fTemp120: F32 = (fTemp91 * (fTemp119 - self.fVec7[1])) / fTemp117;
			self.fVec8[(self.IOTA0 & 4095) as usize] = fTemp120;
			let mut fTemp121: F32 = F32::max(0.0, F32::min(2047.0, self.fConst7 / fTemp116));
			let mut iTemp122: i32 = ((fTemp121) as i32);
			let mut fTemp123: F32 = F32::floor(fTemp121);
			self.fRec55[0] = 0.999000013 * self.fRec55[1] + self.fConst6 * (fTemp120 - self.fVec8[((self.IOTA0 - iTemp122) & 4095) as usize] * (fTemp123 + 1.0 - fTemp121) - (fTemp121 - fTemp123) * self.fVec8[((self.IOTA0 - (iTemp122 + 1)) & 4095) as usize]);
			self.fRec59[0] = fSlow16 + self.fConst2 * self.fRec59[1];
			let mut fTemp124: F32 = F32::powf(2.0, 0.0833333358 * (self.fRec8[0] + self.fRec59[0] + -69.0));
			let mut fTemp125: F32 = 1.0 / F32::tan(self.fConst8 * fTemp124);
			let mut fRec70: F32 = -1.0 * 0.997305274 * (0.899999976 * self.fRec71[2] + 0.0500000007 * (self.fRec71[1] + self.fRec71[3]));
			let mut fTemp126: F32 = self.fConst10 * (0.772727251 / fTemp124 + -0.109999999);
			let mut fTemp127: F32 = fTemp126 + -1.49999499;
			let mut iTemp128: i32 = ((fTemp127) as i32);
			let mut iTemp129: i32 = ((F32::min(self.fConst9, ((std::cmp::max(0, iTemp128)) as F32))) as i32);
			let mut iTemp130: i32 = iTemp129 + 1;
			let mut fTemp131: F32 = F32::floor(fTemp127);
			let mut fTemp132: F32 = fTemp126 + -1.0 - fTemp131;
			let mut fTemp133: F32 = 0.0 - fTemp132;
			let mut fTemp134: F32 = fTemp126 + -2.0 - fTemp131;
			let mut fTemp135: F32 = 0.0 - 0.5 * fTemp134;
			let mut fTemp136: F32 = fTemp126 + -3.0 - fTemp131;
			let mut fTemp137: F32 = 0.0 - 0.333333343 * fTemp136;
			let mut fTemp138: F32 = fTemp126 + -4.0 - fTemp131;
			let mut fTemp139: F32 = 0.0 - 0.25 * fTemp138;
			let mut fTemp140: F32 = fTemp126 - fTemp131;
			let mut iTemp141: i32 = ((F32::min(self.fConst9, ((std::cmp::max(0, iTemp128 + 1)) as F32))) as i32);
			let mut iTemp142: i32 = iTemp141 + 1;
			let mut fTemp143: F32 = 0.0 - fTemp134;
			let mut fTemp144: F32 = 0.0 - 0.5 * fTemp136;
			let mut fTemp145: F32 = 0.0 - 0.333333343 * fTemp138;
			let mut iTemp146: i32 = ((F32::min(self.fConst9, ((std::cmp::max(0, iTemp128 + 2)) as F32))) as i32);
			let mut iTemp147: i32 = iTemp146 + 1;
			let mut fTemp148: F32 = 0.0 - fTemp136;
			let mut fTemp149: F32 = 0.0 - 0.5 * fTemp138;
			let mut fTemp150: F32 = fTemp132 * fTemp134;
			let mut iTemp151: i32 = ((F32::min(self.fConst9, ((std::cmp::max(0, iTemp128 + 3)) as F32))) as i32);
			let mut iTemp152: i32 = iTemp151 + 1;
			let mut fTemp153: F32 = 0.0 - fTemp138;
			let mut fTemp154: F32 = fTemp150 * fTemp136;
			let mut iTemp155: i32 = ((F32::min(self.fConst9, ((std::cmp::max(0, iTemp128 + 4)) as F32))) as i32);
			let mut iTemp156: i32 = iTemp155 + 1;
			self.fRec85[0] = self.fRec62[((self.IOTA0 - iTemp130) & 2047) as usize] * fTemp133 * fTemp135 * fTemp137 * fTemp139 + fTemp140 * (self.fRec62[((self.IOTA0 - iTemp142) & 2047) as usize] * fTemp143 * fTemp144 * fTemp145 + 0.5 * fTemp132 * self.fRec62[((self.IOTA0 - iTemp147) & 2047) as usize] * fTemp148 * fTemp149 + 0.166666672 * fTemp150 * self.fRec62[((self.IOTA0 - iTemp152) & 2047) as usize] * fTemp153 + 0.0416666679 * fTemp154 * self.fRec62[((self.IOTA0 - iTemp156) & 2047) as usize]);
			self.fRec89[0] = 0.0500000007 * self.fRec89[1] + 0.949999988 * self.fRec85[1];
			let mut fRec86: F32 = self.fRec89[0];
			let mut fTemp157: F32 = fTemp133 * fTemp135 * fTemp137 * fTemp139;
			self.fRec91[0] = self.fRec60[1];
			self.fRec92[(self.IOTA0 & 2047) as usize] = -1.0 * 0.997305274 * (0.899999976 * self.fRec91[2] + 0.0500000007 * (self.fRec91[1] + self.fRec91[3]));
			let mut fTemp158: F32 = fTemp143 * fTemp144 * fTemp145;
			let mut fTemp159: F32 = fTemp132 * fTemp148 * fTemp149;
			let mut fTemp160: F32 = fTemp150 * fTemp153;
			self.fVec9[0] = fTemp157 * self.fRec92[((self.IOTA0 - (iTemp129 + 2)) & 2047) as usize] + fTemp140 * (fTemp158 * self.fRec92[((self.IOTA0 - (iTemp141 + 2)) & 2047) as usize] + 0.5 * fTemp159 * self.fRec92[((self.IOTA0 - (iTemp146 + 2)) & 2047) as usize] + 0.166666672 * fTemp160 * self.fRec92[((self.IOTA0 - (iTemp151 + 2)) & 2047) as usize] + 0.0416666679 * fTemp154 * self.fRec92[((self.IOTA0 - (iTemp155 + 2)) & 2047) as usize]);
			self.iRec94[0] = 1103515245 * self.iRec94[1] + 12345;
			let mut fTemp161: F32 = F32::tan(self.fConst11 * fTemp124);
			let mut fTemp162: F32 = 1.0 / fTemp161;
			let mut fTemp163: F32 = (fTemp162 + 1.41421354) / fTemp161 + 1.0;
			self.fRec93[0] = 4.65661287e-10 * ((self.iRec94[0]) as F32) - (self.fRec93[2] * ((fTemp162 + -1.41421354) / fTemp161 + 1.0) + 2.0 * self.fRec93[1] * (1.0 - 1.0 / mydsp_faustpower2_f(fTemp161))) / fTemp163;
			self.fVec10[0] = fSlow20;
			self.iRec95[0] = (self.iRec95[1] + ((self.iRec95[1] > 0) as i32)) * ((fSlow20 <= self.fVec10[1]) as i32) + ((fSlow20 > self.fVec10[1]) as i32);
			let mut fTemp164: F32 = ((self.iRec95[0]) as F32) / F32::max(1.0, self.fConst12 * mydsp_faustpower2_f(1.0 - 0.219999999 * fTemp124));
			let mut fTemp165: F32 = fSlow19 * ((self.fRec93[2] + self.fRec93[0] + 2.0 * self.fRec93[1]) * F32::max(0.0, F32::min(fTemp164, 2.0 - fTemp164))) / fTemp163;
			let mut fTemp166: F32 = self.fVec9[1] + fTemp165;
			self.fVec11[0] = fTemp166;
			self.fRec90[(self.IOTA0 & 2047) as usize] = 0.0500000007 * self.fRec90[((self.IOTA0 - 1) & 2047) as usize] + 0.949999988 * self.fVec11[2];
			let mut fRec87: F32 = fTemp157 * self.fRec90[((self.IOTA0 - iTemp129) & 2047) as usize] + fTemp140 * (fTemp158 * self.fRec90[((self.IOTA0 - iTemp141) & 2047) as usize] + 0.5 * fTemp159 * self.fRec90[((self.IOTA0 - iTemp146) & 2047) as usize] + 0.166666672 * fTemp160 * self.fRec90[((self.IOTA0 - iTemp151) & 2047) as usize] + 0.0416666679 * fTemp154 * self.fRec90[((self.IOTA0 - iTemp155) & 2047) as usize]);
			let mut fRec88: F32 = self.fVec11[1] + self.fRec81[1];
			self.fRec81[0] = fRec86;
			let mut fRec82: F32 = self.fRec81[1];
			let mut fRec83: F32 = fRec87;
			let mut fRec84: F32 = fRec88;
			self.fRec77[0] = fRec82;
			let mut fRec78: F32 = fTemp165 + self.fRec77[1];
			let mut fRec79: F32 = fRec83;
			let mut fRec80: F32 = fRec84;
			self.fRec73[(self.IOTA0 & 2047) as usize] = fRec78;
			let mut fRec74: F32 = fTemp157 * self.fRec73[((self.IOTA0 - iTemp130) & 2047) as usize] + fTemp140 * (fTemp158 * self.fRec73[((self.IOTA0 - iTemp142) & 2047) as usize] + 0.5 * fTemp159 * self.fRec73[((self.IOTA0 - iTemp147) & 2047) as usize] + 0.166666672 * fTemp160 * self.fRec73[((self.IOTA0 - iTemp152) & 2047) as usize] + 0.0416666679 * fTemp154 * self.fRec73[((self.IOTA0 - iTemp156) & 2047) as usize]);
			self.fRec75[0] = fRec79;
			let mut fRec76: F32 = fRec80;
			self.fRec71[0] = fSlow18 * self.fRec75[1];
			let mut fRec72: F32 = fRec76;
			self.fRec66[0] = fRec70;
			let mut fRec67: F32 = fSlow18 * self.fRec66[1];
			let mut fRec68: F32 = self.fRec71[0];
			let mut fRec69: F32 = fRec72;
			self.fRec62[(self.IOTA0 & 2047) as usize] = fRec67;
			let mut fRec63: F32 = fRec74;
			let mut fRec64: F32 = fRec68;
			let mut fRec65: F32 = fRec69;
			self.fRec60[0] = fRec63;
			let mut fRec61: F32 = fRec65;
			let mut fTemp167: F32 = F32::abs(fRec61);
			let mut fTemp168: F32 = if (((self.fRec97[1] > fTemp167) as i32) as i32 != 0) { self.fConst13 } else { 0.0 };
			self.fRec98[0] = self.fRec98[1] * fTemp168 + fTemp167 * (1.0 - fTemp168);
			self.fRec97[0] = self.fRec98[0];
			let mut fRec96: F32 = 0.0 - 0.949999988 * F32::max(20.0 * F32::log10(self.fRec97[0]) + 60.0, 0.0);
			let mut fTemp169: F32 = fSlow17 * fRec61 * F32::powf(10.0, 0.0500000007 * fRec96);
			self.fVec12[0] = fTemp169;
			self.fRec58[0] = 0.0 - (self.fRec58[1] * (1.0 - fTemp125) - (fTemp169 + self.fVec12[1])) / (fTemp125 + 1.0);
			self.fRec100[0] = fSlow21 + self.fConst2 * self.fRec100[1];
			self.fRec99[0] = 0.999000013 * self.fRec99[1] + 9.99999975e-05 * F32::powf(4.0, self.fRec100[0]);
			let mut fTemp170: F32 = F32::powf(2.0, 2.29999995 * self.fRec100[0]);
			let mut fTemp171: F32 = 1.0 - self.fConst14 * fTemp170 / F32::powf(2.0, 2.0 * (1.0 - self.fRec100[0]) + 1.0);
			self.fRec101[0] = 0.999000013 * self.fRec101[1] - 0.00200000009 * fTemp171 * F32::cos(self.fConst15 * fTemp170);
			self.fRec102[0] = 0.999000013 * self.fRec102[1] + 0.00100000005 * mydsp_faustpower2_f(fTemp171);
			self.fRec57[0] = self.fRec58[0] * self.fRec99[0] - (self.fRec101[0] * self.fRec57[1] + self.fRec102[0] * self.fRec57[2]);
			self.fRec0[(self.IOTA0 & 65535) as usize] = (0.5 * self.fRec1[0] * ((self.fRec2[0] * (self.fRec3[2] + self.fRec3[0] + 2.0 * self.fRec3[1])) / fTemp26 + (self.fRec17[0] * (self.fRec18[2] + self.fRec18[0] + 2.0 * self.fRec18[1])) / fTemp48 + (self.fRec27[0] * (self.fRec28[2] + self.fRec28[0] + 2.0 * self.fRec28[1])) / fTemp66 + (self.fRec37[0] * (self.fRec38[2] + self.fRec38[0] + 2.0 * self.fRec38[1])) / fTemp86) + self.fConst5 * self.fRec47[0] * (self.fRec48[0] * fTemp87 + self.fRec51[0] * fTemp97 + self.fRec53[0] * fTemp106 + self.fRec55[0] * fTemp115) + self.fRec57[0] + 0.300000012 * self.fRec0[((self.IOTA0 - self.iConst16) & 65535) as usize]) - self.fRec57[1];
			let mut fTemp172: F32 = self.fRec0[(self.IOTA0 & 65535) as usize];
			*output0 = ((fTemp172) as F32);
			*output1 = ((fTemp172) as F32);
			self.iVec0[1] = self.iVec0[0];
			self.fRec1[1] = self.fRec1[0];
			self.fRec2[1] = self.fRec2[0];
			self.fRec7[1] = self.fRec7[0];
			self.fRec8[1] = self.fRec8[0];
			self.fRec5[1] = self.fRec5[0];
			self.fRec9[1] = self.fRec9[0];
			self.fRec12[1] = self.fRec12[0];
			self.fRec10[1] = self.fRec10[0];
			self.fRec13[1] = self.fRec13[0];
			self.fRec15[1] = self.fRec15[0];
			self.fRec16[1] = self.fRec16[0];
			self.fRec4[2] = self.fRec4[1];
			self.fRec4[1] = self.fRec4[0];
			self.fRec3[2] = self.fRec3[1];
			self.fRec3[1] = self.fRec3[0];
			self.fRec17[1] = self.fRec17[0];
			self.fRec22[1] = self.fRec22[0];
			self.fRec20[1] = self.fRec20[0];
			self.fRec23[1] = self.fRec23[0];
			self.fRec25[1] = self.fRec25[0];
			self.fRec19[2] = self.fRec19[1];
			self.fRec19[1] = self.fRec19[0];
			self.fRec18[2] = self.fRec18[1];
			self.fRec18[1] = self.fRec18[0];
			self.fRec27[1] = self.fRec27[0];
			self.fRec32[1] = self.fRec32[0];
			self.fRec30[1] = self.fRec30[0];
			self.fRec33[1] = self.fRec33[0];
			self.fRec35[1] = self.fRec35[0];
			self.fRec29[2] = self.fRec29[1];
			self.fRec29[1] = self.fRec29[0];
			self.fRec28[2] = self.fRec28[1];
			self.fRec28[1] = self.fRec28[0];
			self.fRec37[1] = self.fRec37[0];
			self.fRec42[1] = self.fRec42[0];
			self.fRec40[1] = self.fRec40[0];
			self.fRec43[1] = self.fRec43[0];
			self.fRec45[1] = self.fRec45[0];
			self.fRec39[2] = self.fRec39[1];
			self.fRec39[1] = self.fRec39[0];
			self.fRec38[2] = self.fRec38[1];
			self.fRec38[1] = self.fRec38[0];
			self.fRec47[1] = self.fRec47[0];
			self.fRec49[1] = self.fRec49[0];
			self.fRec50[1] = self.fRec50[0];
			self.fVec1[1] = self.fVec1[0];
			self.IOTA0 = self.IOTA0 + 1;
			self.fRec48[1] = self.fRec48[0];
			self.fRec52[1] = self.fRec52[0];
			self.fVec3[1] = self.fVec3[0];
			self.fRec51[1] = self.fRec51[0];
			self.fRec54[1] = self.fRec54[0];
			self.fVec5[1] = self.fVec5[0];
			self.fRec53[1] = self.fRec53[0];
			self.fRec56[1] = self.fRec56[0];
			self.fVec7[1] = self.fVec7[0];
			self.fRec55[1] = self.fRec55[0];
			self.fRec59[1] = self.fRec59[0];
			self.fRec85[1] = self.fRec85[0];
			self.fRec89[1] = self.fRec89[0];
			for j0 in (1..=3).rev() {
				self.fRec91[(j0) as usize] = self.fRec91[(j0 - 1) as usize];
			}
			self.fVec9[1] = self.fVec9[0];
			self.iRec94[1] = self.iRec94[0];
			self.fRec93[2] = self.fRec93[1];
			self.fRec93[1] = self.fRec93[0];
			self.fVec10[1] = self.fVec10[0];
			self.iRec95[1] = self.iRec95[0];
			self.fVec11[2] = self.fVec11[1];
			self.fVec11[1] = self.fVec11[0];
			self.fRec81[1] = self.fRec81[0];
			self.fRec77[1] = self.fRec77[0];
			self.fRec75[1] = self.fRec75[0];
			for j1 in (1..=3).rev() {
				self.fRec71[(j1) as usize] = self.fRec71[(j1 - 1) as usize];
			}
			self.fRec66[1] = self.fRec66[0];
			self.fRec60[1] = self.fRec60[0];
			self.fRec98[1] = self.fRec98[0];
			self.fRec97[1] = self.fRec97[0];
			self.fVec12[1] = self.fVec12[0];
			self.fRec58[1] = self.fRec58[0];
			self.fRec100[1] = self.fRec100[0];
			self.fRec99[1] = self.fRec99[0];
			self.fRec101[1] = self.fRec101[0];
			self.fRec102[1] = self.fRec102[0];
			self.fRec57[2] = self.fRec57[1];
			self.fRec57[1] = self.fRec57[0];
		}
	}

}


}
pub use dsp::mydsp as Instrument;
