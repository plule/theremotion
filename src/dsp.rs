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
	fRec4: [F32;2],
	fHslider2: F32,
	fRec5: [F32;2],
	fHslider3: F32,
	fHslider4: F32,
	IOTA0: i32,
	fConst4: F32,
	fConst5: F32,
	fRec32: [F32;2],
	fRec36: [F32;2],
	fHslider5: F32,
	iRec39: [i32;2],
	fConst6: F32,
	fRec38: [F32;3],
	fButton0: F32,
	fVec1: [F32;2],
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
	fVec5: [F32;2],
	fRec3: [F32;2],
	fHslider6: F32,
	fRec52: [F32;2],
	fRec51: [F32;2],
	fConst13: F32,
	fConst14: F32,
	fRec53: [F32;2],
	fRec54: [F32;2],
	fRec2: [F32;3],
	fHslider7: F32,
	fRec55: [F32;2],
	fHslider8: F32,
	fRec56: [F32;2],
	fHslider9: F32,
	fRec59: [F32;2],
	fConst15: F32,
	fConst16: F32,
	fHslider10: F32,
	fRec62: [F32;2],
	fRec60: [F32;2],
	fHslider11: F32,
	fRec63: [F32;2],
	fConst17: F32,
	fHslider12: F32,
	fRec65: [F32;2],
	fRec64: [F32;2],
	fRec58: [F32;3],
	fRec57: [F32;3],
	fHslider13: F32,
	fRec68: [F32;2],
	fHslider14: F32,
	fRec71: [F32;2],
	fRec69: [F32;2],
	fRec72: [F32;2],
	fRec67: [F32;3],
	fRec66: [F32;3],
	fHslider15: F32,
	fRec75: [F32;2],
	fHslider16: F32,
	fRec78: [F32;2],
	fRec76: [F32;2],
	fRec79: [F32;2],
	fRec74: [F32;3],
	fRec73: [F32;3],
	fHslider17: F32,
	fRec82: [F32;2],
	fHslider18: F32,
	fRec85: [F32;2],
	fRec83: [F32;2],
	fRec86: [F32;2],
	fRec81: [F32;3],
	fRec80: [F32;3],
	iConst18: i32,
	fConst19: F32,
	fHslider19: F32,
	fRec87: [F32;2],
	fHslider20: F32,
	fRec88: [F32;2],
	fConst20: F32,
	fHslider21: F32,
	fRec91: [F32;2],
	fRec90: [F32;2],
	fVec6: [F32;2],
	fVec7: [F32;4096],
	fConst21: F32,
	fRec89: [F32;2],
	fRec93: [F32;2],
	fVec8: [F32;2],
	fVec9: [F32;4096],
	fRec92: [F32;2],
	fRec95: [F32;2],
	fVec10: [F32;2],
	fVec11: [F32;4096],
	fRec94: [F32;2],
	fRec97: [F32;2],
	fVec12: [F32;2],
	fVec13: [F32;4096],
	fRec96: [F32;2],
	fRec0: [F32;65536],
	fHslider22: F32,
	fRec98: [F32;2],
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
			fRec4: [0.0;2],
			fHslider2: 0.0,
			fRec5: [0.0;2],
			fHslider3: 0.0,
			fHslider4: 0.0,
			IOTA0: 0,
			fConst4: 0.0,
			fConst5: 0.0,
			fRec32: [0.0;2],
			fRec36: [0.0;2],
			fHslider5: 0.0,
			iRec39: [0;2],
			fConst6: 0.0,
			fRec38: [0.0;3],
			fButton0: 0.0,
			fVec1: [0.0;2],
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
			fVec5: [0.0;2],
			fRec3: [0.0;2],
			fHslider6: 0.0,
			fRec52: [0.0;2],
			fRec51: [0.0;2],
			fConst13: 0.0,
			fConst14: 0.0,
			fRec53: [0.0;2],
			fRec54: [0.0;2],
			fRec2: [0.0;3],
			fHslider7: 0.0,
			fRec55: [0.0;2],
			fHslider8: 0.0,
			fRec56: [0.0;2],
			fHslider9: 0.0,
			fRec59: [0.0;2],
			fConst15: 0.0,
			fConst16: 0.0,
			fHslider10: 0.0,
			fRec62: [0.0;2],
			fRec60: [0.0;2],
			fHslider11: 0.0,
			fRec63: [0.0;2],
			fConst17: 0.0,
			fHslider12: 0.0,
			fRec65: [0.0;2],
			fRec64: [0.0;2],
			fRec58: [0.0;3],
			fRec57: [0.0;3],
			fHslider13: 0.0,
			fRec68: [0.0;2],
			fHslider14: 0.0,
			fRec71: [0.0;2],
			fRec69: [0.0;2],
			fRec72: [0.0;2],
			fRec67: [0.0;3],
			fRec66: [0.0;3],
			fHslider15: 0.0,
			fRec75: [0.0;2],
			fHslider16: 0.0,
			fRec78: [0.0;2],
			fRec76: [0.0;2],
			fRec79: [0.0;2],
			fRec74: [0.0;3],
			fRec73: [0.0;3],
			fHslider17: 0.0,
			fRec82: [0.0;2],
			fHslider18: 0.0,
			fRec85: [0.0;2],
			fRec83: [0.0;2],
			fRec86: [0.0;2],
			fRec81: [0.0;3],
			fRec80: [0.0;3],
			iConst18: 0,
			fConst19: 0.0,
			fHslider19: 0.0,
			fRec87: [0.0;2],
			fHslider20: 0.0,
			fRec88: [0.0;2],
			fConst20: 0.0,
			fHslider21: 0.0,
			fRec91: [0.0;2],
			fRec90: [0.0;2],
			fVec6: [0.0;2],
			fVec7: [0.0;4096],
			fConst21: 0.0,
			fRec89: [0.0;2],
			fRec93: [0.0;2],
			fVec8: [0.0;2],
			fVec9: [0.0;4096],
			fRec92: [0.0;2],
			fRec95: [0.0;2],
			fVec10: [0.0;2],
			fVec11: [0.0;4096],
			fRec94: [0.0;2],
			fRec97: [0.0;2],
			fVec12: [0.0;2],
			fVec13: [0.0;4096],
			fRec96: [0.0;2],
			fRec0: [0.0;65536],
			fHslider22: 0.0,
			fRec98: [0.0;2],
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
		m.declare("filters.lib/lowpass0_highpass1", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
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
		self.fHslider1 = 80.0;
		self.fHslider2 = 0.0;
		self.fHslider3 = 1.0;
		self.fHslider4 = 1.0;
		self.fHslider5 = 0.5;
		self.fButton0 = 0.0;
		self.fHslider6 = 0.5;
		self.fHslider7 = 0.0;
		self.fHslider8 = 1.0;
		self.fHslider9 = 0.0;
		self.fHslider10 = 60.0;
		self.fHslider11 = 0.0;
		self.fHslider12 = 0.0;
		self.fHslider13 = 0.0;
		self.fHslider14 = 60.0;
		self.fHslider15 = 0.0;
		self.fHslider16 = 60.0;
		self.fHslider17 = 0.0;
		self.fHslider18 = 60.0;
		self.fHslider19 = 0.0;
		self.fHslider20 = 1.0;
		self.fHslider21 = 60.0;
		self.fHslider22 = 1.0;
	}
	fn instance_clear(&mut self) {
		for l0 in 0..2 {
			self.iVec0[(l0) as usize] = 0;
		}
		for l1 in 0..2 {
			self.fRec1[(l1) as usize] = 0.0;
		}
		for l2 in 0..2 {
			self.fRec4[(l2) as usize] = 0.0;
		}
		for l3 in 0..2 {
			self.fRec5[(l3) as usize] = 0.0;
		}
		self.IOTA0 = 0;
		for l4 in 0..2 {
			self.fRec32[(l4) as usize] = 0.0;
		}
		for l5 in 0..2 {
			self.fRec36[(l5) as usize] = 0.0;
		}
		for l6 in 0..2 {
			self.iRec39[(l6) as usize] = 0;
		}
		for l7 in 0..3 {
			self.fRec38[(l7) as usize] = 0.0;
		}
		for l8 in 0..2 {
			self.fVec1[(l8) as usize] = 0.0;
		}
		for l9 in 0..2 {
			self.iRec40[(l9) as usize] = 0;
		}
		for l10 in 0..4 {
			self.fRec41[(l10) as usize] = 0.0;
		}
		for l11 in 0..2048 {
			self.fRec42[(l11) as usize] = 0.0;
		}
		for l12 in 0..2 {
			self.fVec2[(l12) as usize] = 0.0;
		}
		for l13 in 0..2 {
			self.fRec46[(l13) as usize] = 0.0;
		}
		for l14 in 0..2 {
			self.iVec3[(l14) as usize] = 0;
		}
		for l15 in 0..2 {
			self.iRec47[(l15) as usize] = 0;
		}
		for l16 in 0..2 {
			self.fRec44[(l16) as usize] = 0.0;
		}
		for l17 in 0..2 {
			self.fRec43[(l17) as usize] = 0.0;
		}
		for l18 in 0..3 {
			self.fVec4[(l18) as usize] = 0.0;
		}
		for l19 in 0..2048 {
			self.fRec37[(l19) as usize] = 0.0;
		}
		for l20 in 0..2 {
			self.fRec28[(l20) as usize] = 0.0;
		}
		for l21 in 0..2 {
			self.fRec24[(l21) as usize] = 0.0;
		}
		for l22 in 0..2048 {
			self.fRec20[(l22) as usize] = 0.0;
		}
		for l23 in 0..2 {
			self.fRec22[(l23) as usize] = 0.0;
		}
		for l24 in 0..4 {
			self.fRec18[(l24) as usize] = 0.0;
		}
		for l25 in 0..2 {
			self.fRec13[(l25) as usize] = 0.0;
		}
		for l26 in 0..2048 {
			self.fRec9[(l26) as usize] = 0.0;
		}
		for l27 in 0..2 {
			self.fRec7[(l27) as usize] = 0.0;
		}
		for l28 in 0..2 {
			self.fRec50[(l28) as usize] = 0.0;
		}
		for l29 in 0..2 {
			self.fRec49[(l29) as usize] = 0.0;
		}
		for l30 in 0..2 {
			self.fRec6[(l30) as usize] = 0.0;
		}
		for l31 in 0..2 {
			self.fVec5[(l31) as usize] = 0.0;
		}
		for l32 in 0..2 {
			self.fRec3[(l32) as usize] = 0.0;
		}
		for l33 in 0..2 {
			self.fRec52[(l33) as usize] = 0.0;
		}
		for l34 in 0..2 {
			self.fRec51[(l34) as usize] = 0.0;
		}
		for l35 in 0..2 {
			self.fRec53[(l35) as usize] = 0.0;
		}
		for l36 in 0..2 {
			self.fRec54[(l36) as usize] = 0.0;
		}
		for l37 in 0..3 {
			self.fRec2[(l37) as usize] = 0.0;
		}
		for l38 in 0..2 {
			self.fRec55[(l38) as usize] = 0.0;
		}
		for l39 in 0..2 {
			self.fRec56[(l39) as usize] = 0.0;
		}
		for l40 in 0..2 {
			self.fRec59[(l40) as usize] = 0.0;
		}
		for l41 in 0..2 {
			self.fRec62[(l41) as usize] = 0.0;
		}
		for l42 in 0..2 {
			self.fRec60[(l42) as usize] = 0.0;
		}
		for l43 in 0..2 {
			self.fRec63[(l43) as usize] = 0.0;
		}
		for l44 in 0..2 {
			self.fRec65[(l44) as usize] = 0.0;
		}
		for l45 in 0..2 {
			self.fRec64[(l45) as usize] = 0.0;
		}
		for l46 in 0..3 {
			self.fRec58[(l46) as usize] = 0.0;
		}
		for l47 in 0..3 {
			self.fRec57[(l47) as usize] = 0.0;
		}
		for l48 in 0..2 {
			self.fRec68[(l48) as usize] = 0.0;
		}
		for l49 in 0..2 {
			self.fRec71[(l49) as usize] = 0.0;
		}
		for l50 in 0..2 {
			self.fRec69[(l50) as usize] = 0.0;
		}
		for l51 in 0..2 {
			self.fRec72[(l51) as usize] = 0.0;
		}
		for l52 in 0..3 {
			self.fRec67[(l52) as usize] = 0.0;
		}
		for l53 in 0..3 {
			self.fRec66[(l53) as usize] = 0.0;
		}
		for l54 in 0..2 {
			self.fRec75[(l54) as usize] = 0.0;
		}
		for l55 in 0..2 {
			self.fRec78[(l55) as usize] = 0.0;
		}
		for l56 in 0..2 {
			self.fRec76[(l56) as usize] = 0.0;
		}
		for l57 in 0..2 {
			self.fRec79[(l57) as usize] = 0.0;
		}
		for l58 in 0..3 {
			self.fRec74[(l58) as usize] = 0.0;
		}
		for l59 in 0..3 {
			self.fRec73[(l59) as usize] = 0.0;
		}
		for l60 in 0..2 {
			self.fRec82[(l60) as usize] = 0.0;
		}
		for l61 in 0..2 {
			self.fRec85[(l61) as usize] = 0.0;
		}
		for l62 in 0..2 {
			self.fRec83[(l62) as usize] = 0.0;
		}
		for l63 in 0..2 {
			self.fRec86[(l63) as usize] = 0.0;
		}
		for l64 in 0..3 {
			self.fRec81[(l64) as usize] = 0.0;
		}
		for l65 in 0..3 {
			self.fRec80[(l65) as usize] = 0.0;
		}
		for l66 in 0..2 {
			self.fRec87[(l66) as usize] = 0.0;
		}
		for l67 in 0..2 {
			self.fRec88[(l67) as usize] = 0.0;
		}
		for l68 in 0..2 {
			self.fRec91[(l68) as usize] = 0.0;
		}
		for l69 in 0..2 {
			self.fRec90[(l69) as usize] = 0.0;
		}
		for l70 in 0..2 {
			self.fVec6[(l70) as usize] = 0.0;
		}
		for l71 in 0..4096 {
			self.fVec7[(l71) as usize] = 0.0;
		}
		for l72 in 0..2 {
			self.fRec89[(l72) as usize] = 0.0;
		}
		for l73 in 0..2 {
			self.fRec93[(l73) as usize] = 0.0;
		}
		for l74 in 0..2 {
			self.fVec8[(l74) as usize] = 0.0;
		}
		for l75 in 0..4096 {
			self.fVec9[(l75) as usize] = 0.0;
		}
		for l76 in 0..2 {
			self.fRec92[(l76) as usize] = 0.0;
		}
		for l77 in 0..2 {
			self.fRec95[(l77) as usize] = 0.0;
		}
		for l78 in 0..2 {
			self.fVec10[(l78) as usize] = 0.0;
		}
		for l79 in 0..4096 {
			self.fVec11[(l79) as usize] = 0.0;
		}
		for l80 in 0..2 {
			self.fRec94[(l80) as usize] = 0.0;
		}
		for l81 in 0..2 {
			self.fRec97[(l81) as usize] = 0.0;
		}
		for l82 in 0..2 {
			self.fVec12[(l82) as usize] = 0.0;
		}
		for l83 in 0..4096 {
			self.fVec13[(l83) as usize] = 0.0;
		}
		for l84 in 0..2 {
			self.fRec96[(l84) as usize] = 0.0;
		}
		for l85 in 0..65536 {
			self.fRec0[(l85) as usize] = 0.0;
		}
		for l86 in 0..2 {
			self.fRec98[(l86) as usize] = 0.0;
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
		self.fConst13 = 1413.71667 / self.fConst0;
		self.fConst14 = 2827.43335 / self.fConst0;
		self.fConst15 = 1.0 / self.fConst0;
		self.fConst16 = 19404.0 / self.fConst0;
		self.fConst17 = 3.14159274 / self.fConst0;
		self.iConst18 = ((F32::min(self.fConst0, F32::max(0.0, 0.300000012 * self.fConst0))) as i32) + 1;
		self.fConst19 = 352.0 / self.fConst0;
		self.fConst20 = 0.25 * self.fConst0;
		self.fConst21 = 0.5 * self.fConst0;
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
		ui_interface.declare(Some(ParamIndex(13)), "1", "");
		ui_interface.add_horizontal_slider("wah", ParamIndex(13), 0.5, 0.25, 0.75, 0.001);
		ui_interface.declare(Some(ParamIndex(14)), "2", "");
		ui_interface.add_horizontal_slider("gain", ParamIndex(14), 1.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(15)), "3", "");
		ui_interface.add_horizontal_slider("mute", ParamIndex(15), 1.0, 0.90000000000000002, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(16)), "4", "");
		ui_interface.add_horizontal_slider("strength", ParamIndex(16), 0.5, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(17)), "5", "");
		ui_interface.add_horizontal_slider("pitchBend", ParamIndex(17), 0.0, -1.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "2", "");
		ui_interface.open_horizontal_box("drone");
		ui_interface.declare(Some(ParamIndex(18)), "0", "");
		ui_interface.add_horizontal_slider("volume", ParamIndex(18), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(19)), "1", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(19), 60.0, 0.0, 127.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "3", "");
		ui_interface.open_vertical_box("mix");
		ui_interface.declare(Some(ParamIndex(20)), "0", "");
		ui_interface.add_horizontal_slider("master", ParamIndex(20), 1.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(21)), "1", "");
		ui_interface.add_horizontal_slider("drone", ParamIndex(21), 1.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(22)), "2", "");
		ui_interface.add_horizontal_slider("lead", ParamIndex(22), 1.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(23)), "3", "");
		ui_interface.add_horizontal_slider("pluck", ParamIndex(23), 1.0, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			11 => Some(self.fButton0),
			23 => Some(self.fHslider0),
			12 => Some(self.fHslider1),
			3 => Some(self.fHslider10),
			2 => Some(self.fHslider11),
			1 => Some(self.fHslider12),
			6 => Some(self.fHslider13),
			5 => Some(self.fHslider14),
			8 => Some(self.fHslider15),
			7 => Some(self.fHslider16),
			10 => Some(self.fHslider17),
			9 => Some(self.fHslider18),
			18 => Some(self.fHslider19),
			17 => Some(self.fHslider2),
			21 => Some(self.fHslider20),
			19 => Some(self.fHslider21),
			20 => Some(self.fHslider22),
			14 => Some(self.fHslider3),
			15 => Some(self.fHslider4),
			16 => Some(self.fHslider5),
			13 => Some(self.fHslider6),
			0 => Some(self.fHslider7),
			22 => Some(self.fHslider8),
			4 => Some(self.fHslider9),
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			11 => { self.fButton0 = value }
			23 => { self.fHslider0 = value }
			12 => { self.fHslider1 = value }
			3 => { self.fHslider10 = value }
			2 => { self.fHslider11 = value }
			1 => { self.fHslider12 = value }
			6 => { self.fHslider13 = value }
			5 => { self.fHslider14 = value }
			8 => { self.fHslider15 = value }
			7 => { self.fHslider16 = value }
			10 => { self.fHslider17 = value }
			9 => { self.fHslider18 = value }
			18 => { self.fHslider19 = value }
			17 => { self.fHslider2 = value }
			21 => { self.fHslider20 = value }
			19 => { self.fHslider21 = value }
			20 => { self.fHslider22 = value }
			14 => { self.fHslider3 = value }
			15 => { self.fHslider4 = value }
			16 => { self.fHslider5 = value }
			13 => { self.fHslider6 = value }
			0 => { self.fHslider7 = value }
			22 => { self.fHslider8 = value }
			4 => { self.fHslider9 = value }
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
		let mut fSlow4: F32 = ((self.fHslider4) as F32);
		let mut fSlow5: F32 = ((self.fHslider5) as F32);
		let mut fSlow6: F32 = ((self.fButton0) as F32);
		let mut fSlow7: F32 = self.fConst1 * ((self.fHslider6) as F32);
		let mut fSlow8: F32 = self.fConst1 * ((self.fHslider7) as F32);
		let mut fSlow9: F32 = self.fConst1 * ((self.fHslider8) as F32);
		let mut fSlow10: F32 = self.fConst1 * ((self.fHslider9) as F32);
		let mut fSlow11: F32 = ((self.fHslider10) as F32);
		let mut fSlow12: F32 = self.fConst16 * F32::powf(2.0, 0.0833333358 * (fSlow11 + -69.0));
		let mut fSlow13: F32 = self.fConst1 * ((self.fHslider11) as F32);
		let mut fSlow14: F32 = self.fConst1 * ((self.fHslider12) as F32);
		let mut fSlow15: F32 = self.fConst1 * ((self.fHslider13) as F32);
		let mut fSlow16: F32 = ((self.fHslider14) as F32);
		let mut fSlow17: F32 = self.fConst16 * F32::powf(2.0, 0.0833333358 * (fSlow16 + -69.0));
		let mut fSlow18: F32 = self.fConst1 * ((self.fHslider15) as F32);
		let mut fSlow19: F32 = ((self.fHslider16) as F32);
		let mut fSlow20: F32 = self.fConst16 * F32::powf(2.0, 0.0833333358 * (fSlow19 + -69.0));
		let mut fSlow21: F32 = self.fConst1 * ((self.fHslider17) as F32);
		let mut fSlow22: F32 = ((self.fHslider18) as F32);
		let mut fSlow23: F32 = self.fConst16 * F32::powf(2.0, 0.0833333358 * (fSlow22 + -69.0));
		let mut fSlow24: F32 = self.fConst1 * ((self.fHslider19) as F32);
		let mut fSlow25: F32 = self.fConst1 * ((self.fHslider20) as F32);
		let mut fSlow26: F32 = self.fConst1 * ((self.fHslider21) as F32);
		let mut fSlow27: F32 = self.fConst1 * ((self.fHslider22) as F32);
		let zipped_iterators = outputs0.zip(outputs1);
		for (output0, output1) in zipped_iterators {
			self.iVec0[0] = 1;
			self.fRec1[0] = fSlow0 + self.fConst2 * self.fRec1[1];
			self.fRec4[0] = fSlow1 + self.fConst2 * self.fRec4[1];
			self.fRec5[0] = fSlow2 + self.fConst2 * self.fRec5[1];
			let mut fTemp0: F32 = F32::powf(2.0, 0.0833333358 * (self.fRec4[0] + self.fRec5[0] + -69.0));
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
			let mut fTemp34: F32 = F32::tan(self.fConst6 * fTemp0);
			let mut fTemp35: F32 = 1.0 / fTemp34;
			let mut fTemp36: F32 = (fTemp35 + 1.41421354) / fTemp34 + 1.0;
			self.fRec38[0] = 4.65661287e-10 * ((self.iRec39[0]) as F32) - (self.fRec38[2] * ((fTemp35 + -1.41421354) / fTemp34 + 1.0) + 2.0 * self.fRec38[1] * (1.0 - 1.0 / mydsp_faustpower2_f(fTemp34))) / fTemp36;
			self.fVec1[0] = fSlow6;
			self.iRec40[0] = (self.iRec40[1] + ((self.iRec40[1] > 0) as i32)) * ((fSlow6 <= self.fVec1[1]) as i32) + ((fSlow6 > self.fVec1[1]) as i32);
			let mut fTemp37: F32 = ((self.iRec40[0]) as F32) / F32::max(1.0, self.fConst7 * mydsp_faustpower2_f(1.0 - 0.219999999 * fTemp0));
			let mut fTemp38: F32 = fSlow5 * ((self.fRec38[2] + self.fRec38[0] + 2.0 * self.fRec38[1]) * F32::max(0.0, F32::min(fTemp37, 2.0 - fTemp37))) / fTemp36;
			self.fRec41[0] = self.fRec7[1];
			self.fRec42[(self.IOTA0 & 2047) as usize] = -1.0 * 0.997305274 * (0.899999976 * self.fRec41[2] + 0.0500000007 * (self.fRec41[1] + self.fRec41[3]));
			let mut fTemp39: F32 = fTemp19 * fTemp20 * fTemp21;
			let mut fTemp40: F32 = fTemp8 * fTemp24 * fTemp25;
			let mut fTemp41: F32 = fTemp26 * fTemp29;
			self.fVec2[0] = fTemp33 * self.fRec42[((self.IOTA0 - (iTemp5 + 2)) & 2047) as usize] + fTemp16 * (fTemp39 * self.fRec42[((self.IOTA0 - (iTemp17 + 2)) & 2047) as usize] + 0.5 * fTemp40 * self.fRec42[((self.IOTA0 - (iTemp22 + 2)) & 2047) as usize] + 0.166666672 * fTemp41 * self.fRec42[((self.IOTA0 - (iTemp27 + 2)) & 2047) as usize] + 0.0416666679 * fTemp30 * self.fRec42[((self.IOTA0 - (iTemp31 + 2)) & 2047) as usize]);
			self.fRec46[0] = self.fConst8 * self.fRec46[1] + self.fConst9 * F32::abs(self.fRec6[1]);
			let mut fRec45: F32 = self.fRec46[0];
			let mut iTemp42: i32 = ((fRec45 > 0.100000001) as i32);
			self.iVec3[0] = iTemp42;
			self.iRec47[0] = std::cmp::max(((self.iConst10 * ((iTemp42 < self.iVec3[1]) as i32)) as i32), ((self.iRec47[1] + -1) as i32));
			let mut fTemp43: F32 = F32::abs(F32::max(((iTemp42) as F32), ((((self.iRec47[0] > 0) as i32)) as F32)));
			let mut fTemp44: F32 = if (((self.fRec43[1] > fTemp43) as i32) as i32 != 0) { self.fConst11 } else { self.fConst8 };
			self.fRec44[0] = self.fRec44[1] * fTemp44 + fTemp43 * (1.0 - fTemp44);
			self.fRec43[0] = self.fRec44[0];
			let mut fTemp45: F32 = 0.00499999989 * self.fRec43[0] * self.fRec6[1];
			let mut fTemp46: F32 = fTemp38 + self.fVec2[1] + fTemp45;
			self.fVec4[0] = fTemp46;
			self.fRec37[(self.IOTA0 & 2047) as usize] = 0.0500000007 * self.fRec37[((self.IOTA0 - 1) & 2047) as usize] + 0.949999988 * self.fVec4[2];
			let mut fRec34: F32 = fTemp33 * self.fRec37[((self.IOTA0 - iTemp5) & 2047) as usize] + fTemp16 * (fTemp39 * self.fRec37[((self.IOTA0 - iTemp17) & 2047) as usize] + 0.5 * fTemp40 * self.fRec37[((self.IOTA0 - iTemp22) & 2047) as usize] + 0.166666672 * fTemp41 * self.fRec37[((self.IOTA0 - iTemp27) & 2047) as usize] + 0.0416666679 * fTemp30 * self.fRec37[((self.IOTA0 - iTemp31) & 2047) as usize]);
			let mut fRec35: F32 = self.fVec4[1] + self.fRec28[1];
			self.fRec28[0] = fRec33;
			let mut fRec29: F32 = self.fRec28[1];
			let mut fRec30: F32 = fRec34;
			let mut fRec31: F32 = fRec35;
			self.fRec24[0] = fRec29;
			let mut fRec25: F32 = fTemp45 + fTemp38 + self.fRec24[1];
			let mut fRec26: F32 = fRec30;
			let mut fRec27: F32 = fRec31;
			self.fRec20[(self.IOTA0 & 2047) as usize] = fRec25;
			let mut fRec21: F32 = fTemp33 * self.fRec20[((self.IOTA0 - iTemp6) & 2047) as usize] + fTemp16 * (fTemp39 * self.fRec20[((self.IOTA0 - iTemp18) & 2047) as usize] + 0.5 * fTemp40 * self.fRec20[((self.IOTA0 - iTemp23) & 2047) as usize] + 0.166666672 * fTemp41 * self.fRec20[((self.IOTA0 - iTemp28) & 2047) as usize] + 0.0416666679 * fTemp30 * self.fRec20[((self.IOTA0 - iTemp32) & 2047) as usize]);
			self.fRec22[0] = fRec26;
			let mut fRec23: F32 = fRec27;
			self.fRec18[0] = fSlow4 * self.fRec22[1];
			let mut fRec19: F32 = fRec23;
			self.fRec13[0] = fRec17;
			let mut fRec14: F32 = fSlow4 * self.fRec13[1];
			let mut fRec15: F32 = self.fRec18[0];
			let mut fRec16: F32 = fRec19;
			self.fRec9[(self.IOTA0 & 2047) as usize] = fRec14;
			let mut fRec10: F32 = fRec21;
			let mut fRec11: F32 = fRec15;
			let mut fRec12: F32 = fRec16;
			self.fRec7[0] = fRec10;
			let mut fRec8: F32 = fRec12;
			let mut fTemp47: F32 = F32::abs(fRec8);
			let mut fTemp48: F32 = if (((self.fRec49[1] > fTemp47) as i32) as i32 != 0) { self.fConst12 } else { 0.0 };
			self.fRec50[0] = self.fRec50[1] * fTemp48 + fTemp47 * (1.0 - fTemp48);
			self.fRec49[0] = self.fRec50[0];
			let mut fRec48: F32 = 0.0 - 0.949999988 * F32::max(20.0 * F32::log10(self.fRec49[0]) + 10.0, 0.0);
			self.fRec6[0] = fRec8 * F32::powf(10.0, 0.0500000007 * fRec48);
			let mut fTemp49: F32 = fSlow3 * self.fRec6[0];
			self.fVec5[0] = fTemp49;
			self.fRec3[0] = 0.0 - (self.fRec3[1] * (1.0 - fTemp1) - (fTemp49 + self.fVec5[1])) / (fTemp1 + 1.0);
			self.fRec52[0] = fSlow7 + self.fConst2 * self.fRec52[1];
			self.fRec51[0] = 0.999000013 * self.fRec51[1] + 9.99999975e-05 * F32::powf(4.0, self.fRec52[0]);
			let mut fTemp50: F32 = F32::powf(2.0, 2.29999995 * self.fRec52[0]);
			let mut fTemp51: F32 = 1.0 - self.fConst13 * fTemp50 / F32::powf(2.0, 2.0 * (1.0 - self.fRec52[0]) + 1.0);
			self.fRec53[0] = 0.999000013 * self.fRec53[1] - 0.00200000009 * fTemp51 * F32::cos(self.fConst14 * fTemp50);
			self.fRec54[0] = 0.999000013 * self.fRec54[1] + 0.00100000005 * mydsp_faustpower2_f(fTemp51);
			self.fRec2[0] = self.fRec3[0] * self.fRec51[0] - (self.fRec53[0] * self.fRec2[1] + self.fRec54[0] * self.fRec2[2]);
			self.fRec55[0] = fSlow8 + self.fConst2 * self.fRec55[1];
			self.fRec56[0] = fSlow9 + self.fConst2 * self.fRec56[1];
			self.fRec59[0] = fSlow10 + self.fConst2 * self.fRec59[1];
			self.fRec62[0] = fSlow12 + self.fConst2 * self.fRec62[1];
			let mut fTemp52: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec62[0]));
			let mut fTemp53: F32 = self.fRec60[1] + self.fConst15 * fTemp52;
			let mut fTemp54: F32 = fTemp53 + -1.0;
			let mut iTemp55: i32 = ((fTemp54 < 0.0) as i32);
			self.fRec60[0] = if (iTemp55 as i32 != 0) { fTemp53 } else { fTemp54 };
			let mut fThen3: F32 = fTemp53 + (1.0 - self.fConst0 / fTemp52) * fTemp54;
			let mut fRec61: F32 = if (iTemp55 as i32 != 0) { fTemp53 } else { fThen3 };
			self.fRec63[0] = fSlow13 + self.fConst2 * self.fRec63[1];
			let mut fTemp56: F32 = F32::min(1.41419947, 1.41421354 * self.fRec63[0]);
			let mut fTemp57: F32 = fTemp56 * (fTemp56 + 1.41421354);
			self.fRec65[0] = fSlow14 + self.fConst2 * self.fRec65[1];
			let mut fTemp58: F32 = self.fRec65[0] + -69.0;
			self.fRec64[0] = self.fConst2 * self.fRec64[1] + self.fConst16 * F32::powf(2.0, 0.0833333358 * (fSlow11 + fTemp58));
			let mut fTemp59: F32 = F32::tan(self.fConst17 * F32::max(20.0, F32::min(10000.0, self.fRec64[0])));
			let mut fTemp60: F32 = 1.0 / fTemp59;
			let mut fTemp61: F32 = 1.41421354 * fTemp56;
			let mut fTemp62: F32 = fTemp61 + 2.0;
			let mut fTemp63: F32 = 1.0 - 1.0 / mydsp_faustpower2_f(fTemp59);
			let mut fTemp64: F32 = fTemp57 + (fTemp60 + fTemp62) / fTemp59 + 1.0;
			self.fRec58[0] = self.fRec59[0] * (2.0 * fRec61 + -1.0) - (self.fRec58[2] * (fTemp57 + (fTemp60 - fTemp62) / fTemp59 + 1.0) + 2.0 * self.fRec58[1] * (fTemp57 + fTemp63)) / fTemp64;
			let mut fTemp65: F32 = fTemp56 * (fTemp56 + -1.41421354);
			let mut fTemp66: F32 = 2.0 - fTemp61;
			let mut fTemp67: F32 = fTemp65 + (fTemp66 + fTemp60) / fTemp59 + 1.0;
			self.fRec57[0] = (self.fRec58[2] + self.fRec58[0] + 2.0 * self.fRec58[1]) / fTemp64 - (self.fRec57[2] * (fTemp65 + (fTemp60 - fTemp66) / fTemp59 + 1.0) + 2.0 * self.fRec57[1] * (fTemp65 + fTemp63)) / fTemp67;
			self.fRec68[0] = fSlow15 + self.fConst2 * self.fRec68[1];
			self.fRec71[0] = fSlow17 + self.fConst2 * self.fRec71[1];
			let mut fTemp68: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec71[0]));
			let mut fTemp69: F32 = self.fRec69[1] + self.fConst15 * fTemp68;
			let mut fTemp70: F32 = fTemp69 + -1.0;
			let mut iTemp71: i32 = ((fTemp70 < 0.0) as i32);
			self.fRec69[0] = if (iTemp71 as i32 != 0) { fTemp69 } else { fTemp70 };
			let mut fThen5: F32 = fTemp69 + (1.0 - self.fConst0 / fTemp68) * fTemp70;
			let mut fRec70: F32 = if (iTemp71 as i32 != 0) { fTemp69 } else { fThen5 };
			self.fRec72[0] = self.fConst2 * self.fRec72[1] + self.fConst16 * F32::powf(2.0, 0.0833333358 * (fSlow16 + fTemp58));
			let mut fTemp72: F32 = F32::tan(self.fConst17 * F32::max(20.0, F32::min(10000.0, self.fRec72[0])));
			let mut fTemp73: F32 = 1.0 / fTemp72;
			let mut fTemp74: F32 = 1.0 - 1.0 / mydsp_faustpower2_f(fTemp72);
			let mut fTemp75: F32 = fTemp57 + (fTemp62 + fTemp73) / fTemp72 + 1.0;
			self.fRec67[0] = self.fRec68[0] * (2.0 * fRec70 + -1.0) - (self.fRec67[2] * (fTemp57 + (fTemp73 - fTemp62) / fTemp72 + 1.0) + 2.0 * self.fRec67[1] * (fTemp57 + fTemp74)) / fTemp75;
			let mut fTemp76: F32 = fTemp65 + (fTemp66 + fTemp73) / fTemp72 + 1.0;
			self.fRec66[0] = (self.fRec67[2] + self.fRec67[0] + 2.0 * self.fRec67[1]) / fTemp75 - (self.fRec66[2] * (fTemp65 + (fTemp73 - fTemp66) / fTemp72 + 1.0) + 2.0 * self.fRec66[1] * (fTemp65 + fTemp74)) / fTemp76;
			self.fRec75[0] = fSlow18 + self.fConst2 * self.fRec75[1];
			self.fRec78[0] = fSlow20 + self.fConst2 * self.fRec78[1];
			let mut fTemp77: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec78[0]));
			let mut fTemp78: F32 = self.fConst15 * fTemp77;
			let mut fTemp79: F32 = self.fRec76[1] + fTemp78;
			let mut fTemp80: F32 = fTemp79 + -1.0;
			let mut iTemp81: i32 = ((fTemp80 < 0.0) as i32);
			self.fRec76[0] = if (iTemp81 as i32 != 0) { fTemp79 } else { fTemp80 };
			let mut fThen7: F32 = self.fRec76[1] + fTemp78 + (1.0 - self.fConst0 / fTemp77) * fTemp80;
			let mut fRec77: F32 = if (iTemp81 as i32 != 0) { fTemp79 } else { fThen7 };
			self.fRec79[0] = self.fConst2 * self.fRec79[1] + self.fConst16 * F32::powf(2.0, 0.0833333358 * (fSlow19 + fTemp58));
			let mut fTemp82: F32 = F32::tan(self.fConst17 * F32::max(20.0, F32::min(10000.0, self.fRec79[0])));
			let mut fTemp83: F32 = 1.0 / fTemp82;
			let mut fTemp84: F32 = 1.0 - 1.0 / mydsp_faustpower2_f(fTemp82);
			let mut fTemp85: F32 = fTemp57 + (fTemp62 + fTemp83) / fTemp82 + 1.0;
			self.fRec74[0] = self.fRec75[0] * (2.0 * fRec77 + -1.0) - (self.fRec74[2] * (fTemp57 + 1.0 - (fTemp62 - fTemp83) / fTemp82) + 2.0 * self.fRec74[1] * (fTemp57 + fTemp84)) / fTemp85;
			let mut fTemp86: F32 = fTemp65 + (fTemp66 + fTemp83) / fTemp82 + 1.0;
			self.fRec73[0] = (self.fRec74[2] + self.fRec74[0] + 2.0 * self.fRec74[1]) / fTemp85 - (self.fRec73[2] * (fTemp65 + (fTemp83 - fTemp66) / fTemp82 + 1.0) + 2.0 * self.fRec73[1] * (fTemp65 + fTemp84)) / fTemp86;
			self.fRec82[0] = fSlow21 + self.fConst2 * self.fRec82[1];
			self.fRec85[0] = fSlow23 + self.fConst2 * self.fRec85[1];
			let mut fTemp87: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec85[0]));
			let mut fTemp88: F32 = self.fRec83[1] + self.fConst15 * fTemp87;
			let mut fTemp89: F32 = fTemp88 + -1.0;
			let mut iTemp90: i32 = ((fTemp89 < 0.0) as i32);
			self.fRec83[0] = if (iTemp90 as i32 != 0) { fTemp88 } else { fTemp89 };
			let mut fThen9: F32 = fTemp88 + (1.0 - self.fConst0 / fTemp87) * fTemp89;
			let mut fRec84: F32 = if (iTemp90 as i32 != 0) { fTemp88 } else { fThen9 };
			self.fRec86[0] = self.fConst2 * self.fRec86[1] + self.fConst16 * F32::powf(2.0, 0.0833333358 * (fSlow22 + fTemp58));
			let mut fTemp91: F32 = F32::tan(self.fConst17 * F32::max(20.0, F32::min(10000.0, self.fRec86[0])));
			let mut fTemp92: F32 = 1.0 / fTemp91;
			let mut fTemp93: F32 = 1.0 - 1.0 / mydsp_faustpower2_f(fTemp91);
			let mut fTemp94: F32 = fTemp57 + (fTemp62 + fTemp92) / fTemp91 + 1.0;
			self.fRec81[0] = self.fRec82[0] * (2.0 * fRec84 + -1.0) - (self.fRec81[2] * (fTemp57 + (fTemp92 - fTemp62) / fTemp91 + 1.0) + 2.0 * self.fRec81[1] * (fTemp57 + fTemp93)) / fTemp94;
			let mut fTemp95: F32 = fTemp65 + (fTemp66 + fTemp92) / fTemp91 + 1.0;
			self.fRec80[0] = (self.fRec81[2] + self.fRec81[0] + 2.0 * self.fRec81[1]) / fTemp94 - (self.fRec80[2] * (fTemp65 + 1.0 - (fTemp66 - fTemp92) / fTemp91) + 2.0 * self.fRec80[1] * (fTemp65 + fTemp93)) / fTemp95;
			self.fRec87[0] = fSlow24 + self.fConst2 * self.fRec87[1];
			self.fRec88[0] = fSlow25 + self.fConst2 * self.fRec88[1];
			let mut fTemp96: F32 = ((self.iVec0[1]) as F32);
			self.fRec91[0] = fSlow26 + self.fConst2 * self.fRec91[1];
			let mut fTemp97: F32 = F32::powf(2.0, 0.0833333358 * (self.fRec91[0] + -69.0));
			let mut fTemp98: F32 = F32::max(440.0 * fTemp97, 23.4489498);
			let mut fTemp99: F32 = F32::max(20.0, F32::abs(fTemp98));
			let mut fTemp100: F32 = self.fRec90[1] + self.fConst15 * fTemp99;
			self.fRec90[0] = fTemp100 - F32::floor(fTemp100);
			let mut fTemp101: F32 = mydsp_faustpower2_f(2.0 * self.fRec90[0] + -1.0);
			self.fVec6[0] = fTemp101;
			let mut fTemp102: F32 = (fTemp96 * (fTemp101 - self.fVec6[1])) / fTemp99;
			self.fVec7[(self.IOTA0 & 4095) as usize] = fTemp102;
			let mut fTemp103: F32 = F32::max(0.0, F32::min(2047.0, self.fConst21 / fTemp98));
			let mut iTemp104: i32 = ((fTemp103) as i32);
			let mut fTemp105: F32 = F32::floor(fTemp103);
			self.fRec89[0] = 0.999000013 * self.fRec89[1] + self.fConst20 * (fTemp102 - self.fVec7[((self.IOTA0 - iTemp104) & 4095) as usize] * (fTemp105 + 1.0 - fTemp103) - (fTemp103 - fTemp105) * self.fVec7[((self.IOTA0 - (iTemp104 + 1)) & 4095) as usize]);
			let mut fTemp106: F32 = F32::powf(2.0, 0.0833333358 * (self.fRec91[0] + -56.9000015));
			let mut fTemp107: F32 = F32::max(440.0 * fTemp106, 23.4489498);
			let mut fTemp108: F32 = F32::max(0.0, F32::min(2047.0, self.fConst21 / fTemp107));
			let mut fTemp109: F32 = F32::floor(fTemp108);
			let mut fTemp110: F32 = F32::max(20.0, F32::abs(fTemp107));
			let mut fTemp111: F32 = self.fRec93[1] + self.fConst15 * fTemp110;
			self.fRec93[0] = fTemp111 - F32::floor(fTemp111);
			let mut fTemp112: F32 = mydsp_faustpower2_f(2.0 * self.fRec93[0] + -1.0);
			self.fVec8[0] = fTemp112;
			let mut fTemp113: F32 = (fTemp96 * (fTemp112 - self.fVec8[1])) / fTemp110;
			self.fVec9[(self.IOTA0 & 4095) as usize] = fTemp113;
			let mut iTemp114: i32 = ((fTemp108) as i32);
			self.fRec92[0] = 0.999000013 * self.fRec92[1] - self.fConst20 * ((fTemp108 - fTemp109) * self.fVec9[((self.IOTA0 - (iTemp114 + 1)) & 4095) as usize] - fTemp113 - self.fVec9[((self.IOTA0 - iTemp114) & 4095) as usize] * (fTemp109 + 1.0 - fTemp108));
			let mut fTemp115: F32 = F32::powf(2.0, 0.0833333358 * (self.fRec91[0] + -81.1100006));
			let mut fTemp116: F32 = F32::max(440.0 * fTemp115, 23.4489498);
			let mut fTemp117: F32 = F32::max(0.0, F32::min(2047.0, self.fConst21 / fTemp116));
			let mut fTemp118: F32 = F32::floor(fTemp117);
			let mut fTemp119: F32 = F32::max(20.0, F32::abs(fTemp116));
			let mut fTemp120: F32 = self.fRec95[1] + self.fConst15 * fTemp119;
			self.fRec95[0] = fTemp120 - F32::floor(fTemp120);
			let mut fTemp121: F32 = mydsp_faustpower2_f(2.0 * self.fRec95[0] + -1.0);
			self.fVec10[0] = fTemp121;
			let mut fTemp122: F32 = (fTemp96 * (fTemp121 - self.fVec10[1])) / fTemp119;
			self.fVec11[(self.IOTA0 & 4095) as usize] = fTemp122;
			let mut iTemp123: i32 = ((fTemp117) as i32);
			self.fRec94[0] = 0.999000013 * self.fRec94[1] - self.fConst20 * ((fTemp117 - fTemp118) * self.fVec11[((self.IOTA0 - (iTemp123 + 1)) & 4095) as usize] - fTemp122 - self.fVec11[((self.IOTA0 - iTemp123) & 4095) as usize] * (fTemp118 + 1.0 - fTemp117));
			let mut fTemp124: F32 = F32::powf(2.0, 0.0833333358 * (self.fRec91[0] + -61.8800011));
			let mut fTemp125: F32 = F32::max(440.0 * fTemp124, 23.4489498);
			let mut fTemp126: F32 = F32::max(0.0, F32::min(2047.0, self.fConst21 / fTemp125));
			let mut fTemp127: F32 = F32::floor(fTemp126);
			let mut fTemp128: F32 = F32::max(20.0, F32::abs(fTemp125));
			let mut fTemp129: F32 = self.fRec97[1] + self.fConst15 * fTemp128;
			self.fRec97[0] = fTemp129 - F32::floor(fTemp129);
			let mut fTemp130: F32 = mydsp_faustpower2_f(2.0 * self.fRec97[0] + -1.0);
			self.fVec12[0] = fTemp130;
			let mut fTemp131: F32 = (fTemp96 * (fTemp130 - self.fVec12[1])) / fTemp128;
			self.fVec13[(self.IOTA0 & 4095) as usize] = fTemp131;
			let mut iTemp132: i32 = ((fTemp126) as i32);
			self.fRec96[0] = 0.999000013 * self.fRec96[1] - self.fConst20 * ((fTemp126 - fTemp127) * self.fVec13[((self.IOTA0 - (iTemp132 + 1)) & 4095) as usize] - fTemp131 - self.fVec13[((self.IOTA0 - iTemp132) & 4095) as usize] * (fTemp127 + 1.0 - fTemp126));
			self.fRec0[(self.IOTA0 & 65535) as usize] = self.fRec1[0] * (self.fRec2[0] - self.fRec2[1]) + self.fRec55[0] * self.fRec56[0] * ((self.fRec57[2] + self.fRec57[0] + 2.0 * self.fRec57[1]) / fTemp67 + (self.fRec66[2] + self.fRec66[0] + 2.0 * self.fRec66[1]) / fTemp76 + (self.fRec73[2] + self.fRec73[0] + 2.0 * self.fRec73[1]) / fTemp86 + (self.fRec80[2] + self.fRec80[0] + 2.0 * self.fRec80[1]) / fTemp95) + 0.300000012 * self.fRec0[((self.IOTA0 - self.iConst18) & 65535) as usize] + self.fConst19 * self.fRec87[0] * self.fRec88[0] * (self.fRec89[0] * fTemp97 + self.fRec92[0] * fTemp106 + self.fRec94[0] * fTemp115 + self.fRec96[0] * fTemp124);
			self.fRec98[0] = fSlow27 + self.fConst2 * self.fRec98[1];
			let mut fTemp133: F32 = self.fRec0[(self.IOTA0 & 65535) as usize] * self.fRec98[0];
			*output0 = ((fTemp133) as F32);
			*output1 = ((fTemp133) as F32);
			self.iVec0[1] = self.iVec0[0];
			self.fRec1[1] = self.fRec1[0];
			self.fRec4[1] = self.fRec4[0];
			self.fRec5[1] = self.fRec5[0];
			self.IOTA0 = self.IOTA0 + 1;
			self.fRec32[1] = self.fRec32[0];
			self.fRec36[1] = self.fRec36[0];
			self.iRec39[1] = self.iRec39[0];
			self.fRec38[2] = self.fRec38[1];
			self.fRec38[1] = self.fRec38[0];
			self.fVec1[1] = self.fVec1[0];
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
			self.fVec5[1] = self.fVec5[0];
			self.fRec3[1] = self.fRec3[0];
			self.fRec52[1] = self.fRec52[0];
			self.fRec51[1] = self.fRec51[0];
			self.fRec53[1] = self.fRec53[0];
			self.fRec54[1] = self.fRec54[0];
			self.fRec2[2] = self.fRec2[1];
			self.fRec2[1] = self.fRec2[0];
			self.fRec55[1] = self.fRec55[0];
			self.fRec56[1] = self.fRec56[0];
			self.fRec59[1] = self.fRec59[0];
			self.fRec62[1] = self.fRec62[0];
			self.fRec60[1] = self.fRec60[0];
			self.fRec63[1] = self.fRec63[0];
			self.fRec65[1] = self.fRec65[0];
			self.fRec64[1] = self.fRec64[0];
			self.fRec58[2] = self.fRec58[1];
			self.fRec58[1] = self.fRec58[0];
			self.fRec57[2] = self.fRec57[1];
			self.fRec57[1] = self.fRec57[0];
			self.fRec68[1] = self.fRec68[0];
			self.fRec71[1] = self.fRec71[0];
			self.fRec69[1] = self.fRec69[0];
			self.fRec72[1] = self.fRec72[0];
			self.fRec67[2] = self.fRec67[1];
			self.fRec67[1] = self.fRec67[0];
			self.fRec66[2] = self.fRec66[1];
			self.fRec66[1] = self.fRec66[0];
			self.fRec75[1] = self.fRec75[0];
			self.fRec78[1] = self.fRec78[0];
			self.fRec76[1] = self.fRec76[0];
			self.fRec79[1] = self.fRec79[0];
			self.fRec74[2] = self.fRec74[1];
			self.fRec74[1] = self.fRec74[0];
			self.fRec73[2] = self.fRec73[1];
			self.fRec73[1] = self.fRec73[0];
			self.fRec82[1] = self.fRec82[0];
			self.fRec85[1] = self.fRec85[0];
			self.fRec83[1] = self.fRec83[0];
			self.fRec86[1] = self.fRec86[0];
			self.fRec81[2] = self.fRec81[1];
			self.fRec81[1] = self.fRec81[0];
			self.fRec80[2] = self.fRec80[1];
			self.fRec80[1] = self.fRec80[0];
			self.fRec87[1] = self.fRec87[0];
			self.fRec88[1] = self.fRec88[0];
			self.fRec91[1] = self.fRec91[0];
			self.fRec90[1] = self.fRec90[0];
			self.fVec6[1] = self.fVec6[0];
			self.fRec89[1] = self.fRec89[0];
			self.fRec93[1] = self.fRec93[0];
			self.fVec8[1] = self.fVec8[0];
			self.fRec92[1] = self.fRec92[0];
			self.fRec95[1] = self.fRec95[0];
			self.fVec10[1] = self.fVec10[0];
			self.fRec94[1] = self.fRec94[0];
			self.fRec97[1] = self.fRec97[0];
			self.fVec12[1] = self.fVec12[0];
			self.fRec96[1] = self.fRec96[0];
			self.fRec98[1] = self.fRec98[0];
		}
	}

}


}
pub use dsp::mydsp as Instrument;
