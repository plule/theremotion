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
	fConst2: F32,
	fHslider0: F32,
	fConst3: F32,
	iVec0: [i32;2],
	fRec1: [F32;2],
	fConst4: F32,
	fConst5: F32,
	fHslider1: F32,
	fRec4: [F32;2],
	fRec3: [F32;2],
	fVec1: [F32;2],
	IOTA0: i32,
	fVec2: [F32;4096],
	fConst6: F32,
	fRec2: [F32;2],
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
	iConst7: i32,
	fHslider2: F32,
	fRec11: [F32;2],
	fHslider3: F32,
	fRec12: [F32;2],
	fHslider4: F32,
	fRec17: [F32;2],
	fRec15: [F32;2],
	fHslider5: F32,
	fRec18: [F32;2],
	fHslider6: F32,
	fRec21: [F32;2],
	fRec19: [F32;2],
	fRec22: [F32;2],
	fHslider7: F32,
	fRec24: [F32;2],
	fConst8: F32,
	fHslider8: F32,
	fRec25: [F32;2],
	fRec14: [F32;3],
	fRec13: [F32;3],
	fHslider9: F32,
	fRec26: [F32;2],
	fHslider10: F32,
	fRec31: [F32;2],
	fRec29: [F32;2],
	fRec32: [F32;2],
	fRec34: [F32;2],
	fRec28: [F32;3],
	fRec27: [F32;3],
	fHslider11: F32,
	fRec36: [F32;2],
	fHslider12: F32,
	fRec41: [F32;2],
	fRec39: [F32;2],
	fRec42: [F32;2],
	fRec44: [F32;2],
	fRec38: [F32;3],
	fRec37: [F32;3],
	fHslider13: F32,
	fRec46: [F32;2],
	fHslider14: F32,
	fRec51: [F32;2],
	fRec49: [F32;2],
	fRec52: [F32;2],
	fRec54: [F32;2],
	fRec48: [F32;3],
	fRec47: [F32;3],
	fRec0: [F32;65536],
	fHslider15: F32,
	fHslider16: F32,
	fRec57: [F32;2],
	fHslider17: F32,
	fRec61: [F32;2],
	fRec60: [F32;2],
	iRec64: [i32;2],
	fConst9: F32,
	fButton0: F32,
	fVec9: [F32;2],
	iRec65: [i32;2],
	fHslider18: F32,
	fVec10: [F32;2048],
	fConst10: F32,
	fRec62: [F32;2],
	fRec63: [F32;2],
	fConst11: F32,
	fConst12: F32,
	fRec66: [F32;2],
	fRec67: [F32;2],
	fRec59: [F32;3],
	fRec58: [F32;3],
	fRec56: [F32;3],
}

impl FaustDsp for mydsp {
	type T = F32;
		
	fn new() -> mydsp { 
		mydsp {
			fSampleRate: 0,
			fConst0: 0.0,
			fConst1: 0.0,
			fConst2: 0.0,
			fHslider0: 0.0,
			fConst3: 0.0,
			iVec0: [0;2],
			fRec1: [0.0;2],
			fConst4: 0.0,
			fConst5: 0.0,
			fHslider1: 0.0,
			fRec4: [0.0;2],
			fRec3: [0.0;2],
			fVec1: [0.0;2],
			IOTA0: 0,
			fVec2: [0.0;4096],
			fConst6: 0.0,
			fRec2: [0.0;2],
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
			iConst7: 0,
			fHslider2: 0.0,
			fRec11: [0.0;2],
			fHslider3: 0.0,
			fRec12: [0.0;2],
			fHslider4: 0.0,
			fRec17: [0.0;2],
			fRec15: [0.0;2],
			fHslider5: 0.0,
			fRec18: [0.0;2],
			fHslider6: 0.0,
			fRec21: [0.0;2],
			fRec19: [0.0;2],
			fRec22: [0.0;2],
			fHslider7: 0.0,
			fRec24: [0.0;2],
			fConst8: 0.0,
			fHslider8: 0.0,
			fRec25: [0.0;2],
			fRec14: [0.0;3],
			fRec13: [0.0;3],
			fHslider9: 0.0,
			fRec26: [0.0;2],
			fHslider10: 0.0,
			fRec31: [0.0;2],
			fRec29: [0.0;2],
			fRec32: [0.0;2],
			fRec34: [0.0;2],
			fRec28: [0.0;3],
			fRec27: [0.0;3],
			fHslider11: 0.0,
			fRec36: [0.0;2],
			fHslider12: 0.0,
			fRec41: [0.0;2],
			fRec39: [0.0;2],
			fRec42: [0.0;2],
			fRec44: [0.0;2],
			fRec38: [0.0;3],
			fRec37: [0.0;3],
			fHslider13: 0.0,
			fRec46: [0.0;2],
			fHslider14: 0.0,
			fRec51: [0.0;2],
			fRec49: [0.0;2],
			fRec52: [0.0;2],
			fRec54: [0.0;2],
			fRec48: [0.0;3],
			fRec47: [0.0;3],
			fRec0: [0.0;65536],
			fHslider15: 0.0,
			fHslider16: 0.0,
			fRec57: [0.0;2],
			fHslider17: 0.0,
			fRec61: [0.0;2],
			fRec60: [0.0;2],
			iRec64: [0;2],
			fConst9: 0.0,
			fButton0: 0.0,
			fVec9: [0.0;2],
			iRec65: [0;2],
			fHslider18: 0.0,
			fVec10: [0.0;2048],
			fConst10: 0.0,
			fRec62: [0.0;2],
			fRec63: [0.0;2],
			fConst11: 0.0,
			fConst12: 0.0,
			fRec66: [0.0;2],
			fRec67: [0.0;2],
			fRec59: [0.0;3],
			fRec58: [0.0;3],
			fRec56: [0.0;3],
		}
	}
	fn metadata(&self, m: &mut dyn Meta) { 
		m.declare("author", "Pierre Lulé");
		m.declare("basics.lib/name", "Faust Basic Element Library");
		m.declare("basics.lib/version", "0.1");
		m.declare("delays.lib/name", "Faust Delay Library");
		m.declare("delays.lib/version", "0.1");
		m.declare("envelopes.lib/ar:author", "Yann Orlarey, Stéphane Letz");
		m.declare("envelopes.lib/author", "GRAME");
		m.declare("envelopes.lib/copyright", "GRAME");
		m.declare("envelopes.lib/license", "LGPL with exception");
		m.declare("envelopes.lib/name", "Faust Envelope Library");
		m.declare("envelopes.lib/version", "0.1");
		m.declare("filename", "instrument.dsp");
		m.declare("filters.lib/fb_fcomb:author", "Julius O. Smith III");
		m.declare("filters.lib/fb_fcomb:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/fb_fcomb:license", "MIT-style STK-4.3 license");
		m.declare("filters.lib/fir:author", "Julius O. Smith III");
		m.declare("filters.lib/fir:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/fir:license", "MIT-style STK-4.3 license");
		m.declare("filters.lib/iir:author", "Julius O. Smith III");
		m.declare("filters.lib/iir:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/iir:license", "MIT-style STK-4.3 license");
		m.declare("filters.lib/lowpass0_highpass1", "MIT-style STK-4.3 license");
		m.declare("filters.lib/name", "Faust Filters Library");
		m.declare("filters.lib/pole:author", "Julius O. Smith III");
		m.declare("filters.lib/pole:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/pole:license", "MIT-style STK-4.3 license");
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
		m.declare("name", "leapotron");
		m.declare("noises.lib/name", "Faust Noise Generator Library");
		m.declare("noises.lib/version", "0.0");
		m.declare("oscillators.lib/name", "Faust Oscillator Library");
		m.declare("oscillators.lib/version", "0.1");
		m.declare("platform.lib/name", "Generic Platform Library");
		m.declare("platform.lib/version", "0.1");
		m.declare("signals.lib/name", "Faust Signal Routing Library");
		m.declare("signals.lib/version", "0.0");
		m.declare("synths.lib/name", "Faust Synthesizer Library");
		m.declare("synths.lib/version", "0.0");
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
		self.fHslider1 = 60.0;
		self.fHslider2 = 0.0;
		self.fHslider3 = 1.0;
		self.fHslider4 = 60.0;
		self.fHslider5 = 0.0;
		self.fHslider6 = 0.00100000005;
		self.fHslider7 = 0.0;
		self.fHslider8 = 0.0;
		self.fHslider9 = 0.0;
		self.fHslider10 = 60.0;
		self.fHslider11 = 0.0;
		self.fHslider12 = 60.0;
		self.fHslider13 = 0.0;
		self.fHslider14 = 60.0;
		self.fHslider15 = 1.0;
		self.fHslider16 = 60.0;
		self.fHslider17 = 0.5;
		self.fButton0 = 0.0;
		self.fHslider18 = 10.0;
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
			self.fRec3[(l3) as usize] = 0.0;
		}
		for l4 in 0..2 {
			self.fVec1[(l4) as usize] = 0.0;
		}
		self.IOTA0 = 0;
		for l5 in 0..4096 {
			self.fVec2[(l5) as usize] = 0.0;
		}
		for l6 in 0..2 {
			self.fRec2[(l6) as usize] = 0.0;
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
			self.fRec17[(l21) as usize] = 0.0;
		}
		for l22 in 0..2 {
			self.fRec15[(l22) as usize] = 0.0;
		}
		for l23 in 0..2 {
			self.fRec18[(l23) as usize] = 0.0;
		}
		for l24 in 0..2 {
			self.fRec21[(l24) as usize] = 0.0;
		}
		for l25 in 0..2 {
			self.fRec19[(l25) as usize] = 0.0;
		}
		for l26 in 0..2 {
			self.fRec22[(l26) as usize] = 0.0;
		}
		for l27 in 0..2 {
			self.fRec24[(l27) as usize] = 0.0;
		}
		for l28 in 0..2 {
			self.fRec25[(l28) as usize] = 0.0;
		}
		for l29 in 0..3 {
			self.fRec14[(l29) as usize] = 0.0;
		}
		for l30 in 0..3 {
			self.fRec13[(l30) as usize] = 0.0;
		}
		for l31 in 0..2 {
			self.fRec26[(l31) as usize] = 0.0;
		}
		for l32 in 0..2 {
			self.fRec31[(l32) as usize] = 0.0;
		}
		for l33 in 0..2 {
			self.fRec29[(l33) as usize] = 0.0;
		}
		for l34 in 0..2 {
			self.fRec32[(l34) as usize] = 0.0;
		}
		for l35 in 0..2 {
			self.fRec34[(l35) as usize] = 0.0;
		}
		for l36 in 0..3 {
			self.fRec28[(l36) as usize] = 0.0;
		}
		for l37 in 0..3 {
			self.fRec27[(l37) as usize] = 0.0;
		}
		for l38 in 0..2 {
			self.fRec36[(l38) as usize] = 0.0;
		}
		for l39 in 0..2 {
			self.fRec41[(l39) as usize] = 0.0;
		}
		for l40 in 0..2 {
			self.fRec39[(l40) as usize] = 0.0;
		}
		for l41 in 0..2 {
			self.fRec42[(l41) as usize] = 0.0;
		}
		for l42 in 0..2 {
			self.fRec44[(l42) as usize] = 0.0;
		}
		for l43 in 0..3 {
			self.fRec38[(l43) as usize] = 0.0;
		}
		for l44 in 0..3 {
			self.fRec37[(l44) as usize] = 0.0;
		}
		for l45 in 0..2 {
			self.fRec46[(l45) as usize] = 0.0;
		}
		for l46 in 0..2 {
			self.fRec51[(l46) as usize] = 0.0;
		}
		for l47 in 0..2 {
			self.fRec49[(l47) as usize] = 0.0;
		}
		for l48 in 0..2 {
			self.fRec52[(l48) as usize] = 0.0;
		}
		for l49 in 0..2 {
			self.fRec54[(l49) as usize] = 0.0;
		}
		for l50 in 0..3 {
			self.fRec48[(l50) as usize] = 0.0;
		}
		for l51 in 0..3 {
			self.fRec47[(l51) as usize] = 0.0;
		}
		for l52 in 0..65536 {
			self.fRec0[(l52) as usize] = 0.0;
		}
		for l53 in 0..2 {
			self.fRec57[(l53) as usize] = 0.0;
		}
		for l54 in 0..2 {
			self.fRec61[(l54) as usize] = 0.0;
		}
		for l55 in 0..2 {
			self.fRec60[(l55) as usize] = 0.0;
		}
		for l56 in 0..2 {
			self.iRec64[(l56) as usize] = 0;
		}
		for l57 in 0..2 {
			self.fVec9[(l57) as usize] = 0.0;
		}
		for l58 in 0..2 {
			self.iRec65[(l58) as usize] = 0;
		}
		for l59 in 0..2048 {
			self.fVec10[(l59) as usize] = 0.0;
		}
		for l60 in 0..2 {
			self.fRec62[(l60) as usize] = 0.0;
		}
		for l61 in 0..2 {
			self.fRec63[(l61) as usize] = 0.0;
		}
		for l62 in 0..2 {
			self.fRec66[(l62) as usize] = 0.0;
		}
		for l63 in 0..2 {
			self.fRec67[(l63) as usize] = 0.0;
		}
		for l64 in 0..3 {
			self.fRec59[(l64) as usize] = 0.0;
		}
		for l65 in 0..3 {
			self.fRec58[(l65) as usize] = 0.0;
		}
		for l66 in 0..3 {
			self.fRec56[(l66) as usize] = 0.0;
		}
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		self.fConst0 = F32::min(192000.0, F32::max(1.0, ((self.fSampleRate) as F32)));
		self.fConst1 = 352.0 / self.fConst0;
		self.fConst2 = 44.0999985 / self.fConst0;
		self.fConst3 = 1.0 - self.fConst2;
		self.fConst4 = 0.25 * self.fConst0;
		self.fConst5 = 1.0 / self.fConst0;
		self.fConst6 = 0.5 * self.fConst0;
		self.iConst7 = ((F32::min(self.fConst0, F32::max(0.0, 0.300000012 * self.fConst0))) as i32) + 1;
		self.fConst8 = 3.14159274 / self.fConst0;
		self.fConst9 = 1.0 / F32::max(1.0, 0.00100000005 * self.fConst0);
		self.fConst10 = 0.0022727272 * self.fConst0;
		self.fConst11 = 1413.71667 / self.fConst0;
		self.fConst12 = 2827.43335 / self.fConst0;
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
		ui_interface.open_vertical_box("leapotron");
		ui_interface.declare(None, "0", "");
		ui_interface.open_vertical_box("lead");
		ui_interface.declare(Some(ParamIndex(0)), "0", "");
		ui_interface.add_horizontal_slider("vol", ParamIndex(0), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(None, "10", "");
		ui_interface.open_horizontal_box("filter");
		ui_interface.declare(Some(ParamIndex(1)), "0", "");
		ui_interface.add_horizontal_slider("cutoff_note", ParamIndex(1), 0.0, -20.0, 50.0, 0.001);
		ui_interface.declare(Some(ParamIndex(2)), "1", "");
		ui_interface.add_horizontal_slider("res", ParamIndex(2), 0.0, 0.0, 0.98999999999999999, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "1", "");
		ui_interface.open_vertical_box("chord");
		ui_interface.declare(Some(ParamIndex(3)), "1", "");
		ui_interface.add_horizontal_slider("vol1", ParamIndex(3), 1.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(4)), "2", "");
		ui_interface.add_horizontal_slider("note1", ParamIndex(4), 60.0, 0.0, 127.0, 0.001);
		ui_interface.declare(Some(ParamIndex(5)), "3", "");
		ui_interface.add_horizontal_slider("vol2", ParamIndex(5), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(6)), "4", "");
		ui_interface.add_horizontal_slider("note2", ParamIndex(6), 60.0, 0.0, 127.0, 0.001);
		ui_interface.declare(Some(ParamIndex(7)), "5", "");
		ui_interface.add_horizontal_slider("vol3", ParamIndex(7), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(8)), "6", "");
		ui_interface.add_horizontal_slider("note3", ParamIndex(8), 60.0, 0.0, 127.0, 0.001);
		ui_interface.declare(Some(ParamIndex(9)), "7", "");
		ui_interface.add_horizontal_slider("vol4", ParamIndex(9), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(10)), "8", "");
		ui_interface.add_horizontal_slider("note4", ParamIndex(10), 60.0, 0.0, 127.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "9", "");
		ui_interface.open_horizontal_box("supersaw");
		ui_interface.declare(Some(ParamIndex(11)), "0", "");
		ui_interface.add_horizontal_slider("volume", ParamIndex(11), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(12)), "1", "");
		ui_interface.add_horizontal_slider("detune", ParamIndex(12), 0.001, 0.001, 0.02, 0.001);
		ui_interface.close_box();
		ui_interface.close_box();
		ui_interface.declare(None, "1", "");
		ui_interface.open_horizontal_box("pluck");
		ui_interface.declare(Some(ParamIndex(13)), "0", "");
		ui_interface.add_button("gate", ParamIndex(13));
		ui_interface.declare(Some(ParamIndex(14)), "1", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(14), 60.0, 0.0, 127.0, 0.001);
		ui_interface.declare(Some(ParamIndex(15)), "1", "");
		ui_interface.add_horizontal_slider("wah", ParamIndex(15), 0.5, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(16)), "2", "");
		ui_interface.add_horizontal_slider("gain", ParamIndex(16), 1.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(17)), "3", "");
		ui_interface.add_horizontal_slider("release", ParamIndex(17), 10.0, 0.0, 10.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "2", "");
		ui_interface.open_horizontal_box("drone");
		ui_interface.declare(Some(ParamIndex(18)), "0", "");
		ui_interface.add_horizontal_slider("volume", ParamIndex(18), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(19)), "1", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(19), 60.0, 0.0, 127.0, 0.001);
		ui_interface.close_box();
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			13 => Some(self.fButton0),
			18 => Some(self.fHslider0),
			19 => Some(self.fHslider1),
			6 => Some(self.fHslider10),
			7 => Some(self.fHslider11),
			8 => Some(self.fHslider12),
			9 => Some(self.fHslider13),
			10 => Some(self.fHslider14),
			16 => Some(self.fHslider15),
			14 => Some(self.fHslider16),
			15 => Some(self.fHslider17),
			17 => Some(self.fHslider18),
			0 => Some(self.fHslider2),
			3 => Some(self.fHslider3),
			4 => Some(self.fHslider4),
			11 => Some(self.fHslider5),
			12 => Some(self.fHslider6),
			2 => Some(self.fHslider7),
			1 => Some(self.fHslider8),
			5 => Some(self.fHslider9),
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			13 => { self.fButton0 = value }
			18 => { self.fHslider0 = value }
			19 => { self.fHslider1 = value }
			6 => { self.fHslider10 = value }
			7 => { self.fHslider11 = value }
			8 => { self.fHslider12 = value }
			9 => { self.fHslider13 = value }
			10 => { self.fHslider14 = value }
			16 => { self.fHslider15 = value }
			14 => { self.fHslider16 = value }
			15 => { self.fHslider17 = value }
			17 => { self.fHslider18 = value }
			0 => { self.fHslider2 = value }
			3 => { self.fHslider3 = value }
			4 => { self.fHslider4 = value }
			11 => { self.fHslider5 = value }
			12 => { self.fHslider6 = value }
			2 => { self.fHslider7 = value }
			1 => { self.fHslider8 = value }
			5 => { self.fHslider9 = value }
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
		let mut fSlow0: F32 = self.fConst2 * ((self.fHslider0) as F32);
		let mut fSlow1: F32 = self.fConst2 * ((self.fHslider1) as F32);
		let mut fSlow2: F32 = self.fConst2 * ((self.fHslider2) as F32);
		let mut fSlow3: F32 = self.fConst2 * ((self.fHslider3) as F32);
		let mut fSlow4: F32 = self.fConst2 * ((self.fHslider4) as F32);
		let mut fSlow5: F32 = self.fConst2 * ((self.fHslider5) as F32);
		let mut fSlow6: F32 = self.fConst2 * ((self.fHslider6) as F32);
		let mut fSlow7: F32 = self.fConst2 * ((self.fHslider7) as F32);
		let mut fSlow8: F32 = self.fConst2 * ((self.fHslider8) as F32);
		let mut fSlow9: F32 = self.fConst2 * ((self.fHslider9) as F32);
		let mut fSlow10: F32 = self.fConst2 * ((self.fHslider10) as F32);
		let mut fSlow11: F32 = self.fConst2 * ((self.fHslider11) as F32);
		let mut fSlow12: F32 = self.fConst2 * ((self.fHslider12) as F32);
		let mut fSlow13: F32 = self.fConst2 * ((self.fHslider13) as F32);
		let mut fSlow14: F32 = self.fConst2 * ((self.fHslider14) as F32);
		let mut fSlow15: F32 = 2.0 * ((self.fHslider15) as F32);
		let mut fSlow16: F32 = self.fConst2 * ((self.fHslider16) as F32);
		let mut fSlow17: F32 = self.fConst2 * ((self.fHslider17) as F32);
		let mut fSlow18: F32 = ((self.fButton0) as F32);
		let mut fSlow19: F32 = 0.00100000005 * ((self.fHslider18) as F32);
		let mut iSlow20: i32 = ((F32::abs(fSlow19) < 1.1920929e-07) as i32);
		let mut fThen25: F32 = F32::exp(0.0 - self.fConst5 / if (iSlow20 as i32 != 0) { 1.0 } else { fSlow19 });
		let mut fSlow21: F32 = 0.0 - if (iSlow20 as i32 != 0) { 0.0 } else { fThen25 };
		let zipped_iterators = outputs0.zip(outputs1);
		for (output0, output1) in zipped_iterators {
			self.iVec0[0] = 1;
			self.fRec1[0] = fSlow0 + self.fConst3 * self.fRec1[1];
			let mut fTemp0: F32 = ((self.iVec0[1]) as F32);
			self.fRec4[0] = fSlow1 + self.fConst3 * self.fRec4[1];
			let mut fTemp1: F32 = F32::powf(2.0, 0.0833333358 * (self.fRec4[0] + -69.0));
			let mut fTemp2: F32 = F32::max(440.0 * fTemp1, 23.4489498);
			let mut fTemp3: F32 = F32::max(20.0, F32::abs(fTemp2));
			let mut fTemp4: F32 = self.fRec3[1] + self.fConst5 * fTemp3;
			self.fRec3[0] = fTemp4 - F32::floor(fTemp4);
			let mut fTemp5: F32 = mydsp_faustpower2_f(2.0 * self.fRec3[0] + -1.0);
			self.fVec1[0] = fTemp5;
			let mut fTemp6: F32 = (fTemp0 * (fTemp5 - self.fVec1[1])) / fTemp3;
			self.fVec2[(self.IOTA0 & 4095) as usize] = fTemp6;
			let mut fTemp7: F32 = F32::max(0.0, F32::min(2047.0, self.fConst6 / fTemp2));
			let mut iTemp8: i32 = ((fTemp7) as i32);
			let mut fTemp9: F32 = F32::floor(fTemp7);
			self.fRec2[0] = 0.999000013 * self.fRec2[1] - self.fConst4 * (self.fVec2[((self.IOTA0 - iTemp8) & 4095) as usize] * (fTemp9 + 1.0 - fTemp7) - fTemp6 + (fTemp7 - fTemp9) * self.fVec2[((self.IOTA0 - (iTemp8 + 1)) & 4095) as usize]);
			let mut fTemp10: F32 = F32::powf(2.0, 0.0833333358 * (self.fRec4[0] + -56.9000015));
			let mut fTemp11: F32 = F32::max(440.0 * fTemp10, 23.4489498);
			let mut fTemp12: F32 = F32::max(20.0, F32::abs(fTemp11));
			let mut fTemp13: F32 = self.fRec6[1] + self.fConst5 * fTemp12;
			self.fRec6[0] = fTemp13 - F32::floor(fTemp13);
			let mut fTemp14: F32 = mydsp_faustpower2_f(2.0 * self.fRec6[0] + -1.0);
			self.fVec3[0] = fTemp14;
			let mut fTemp15: F32 = (fTemp0 * (fTemp14 - self.fVec3[1])) / fTemp12;
			self.fVec4[(self.IOTA0 & 4095) as usize] = fTemp15;
			let mut fTemp16: F32 = F32::max(0.0, F32::min(2047.0, self.fConst6 / fTemp11));
			let mut iTemp17: i32 = ((fTemp16) as i32);
			let mut fTemp18: F32 = F32::floor(fTemp16);
			self.fRec5[0] = 0.999000013 * self.fRec5[1] + self.fConst4 * (fTemp15 - self.fVec4[((self.IOTA0 - iTemp17) & 4095) as usize] * (fTemp18 + 1.0 - fTemp16) - (fTemp16 - fTemp18) * self.fVec4[((self.IOTA0 - (iTemp17 + 1)) & 4095) as usize]);
			let mut fTemp19: F32 = F32::powf(2.0, 0.0833333358 * (self.fRec4[0] + -81.1100006));
			let mut fTemp20: F32 = F32::max(440.0 * fTemp19, 23.4489498);
			let mut fTemp21: F32 = F32::max(20.0, F32::abs(fTemp20));
			let mut fTemp22: F32 = self.fRec8[1] + self.fConst5 * fTemp21;
			self.fRec8[0] = fTemp22 - F32::floor(fTemp22);
			let mut fTemp23: F32 = mydsp_faustpower2_f(2.0 * self.fRec8[0] + -1.0);
			self.fVec5[0] = fTemp23;
			let mut fTemp24: F32 = (fTemp0 * (fTemp23 - self.fVec5[1])) / fTemp21;
			self.fVec6[(self.IOTA0 & 4095) as usize] = fTemp24;
			let mut fTemp25: F32 = F32::max(0.0, F32::min(2047.0, self.fConst6 / fTemp20));
			let mut iTemp26: i32 = ((fTemp25) as i32);
			let mut fTemp27: F32 = F32::floor(fTemp25);
			self.fRec7[0] = 0.999000013 * self.fRec7[1] - self.fConst4 * (self.fVec6[((self.IOTA0 - iTemp26) & 4095) as usize] * (fTemp27 + 1.0 - fTemp25) - fTemp24 + (fTemp25 - fTemp27) * self.fVec6[((self.IOTA0 - (iTemp26 + 1)) & 4095) as usize]);
			let mut fTemp28: F32 = F32::powf(2.0, 0.0833333358 * (self.fRec4[0] + -61.8800011));
			let mut fTemp29: F32 = F32::max(440.0 * fTemp28, 23.4489498);
			let mut fTemp30: F32 = F32::max(20.0, F32::abs(fTemp29));
			let mut fTemp31: F32 = self.fRec10[1] + self.fConst5 * fTemp30;
			self.fRec10[0] = fTemp31 - F32::floor(fTemp31);
			let mut fTemp32: F32 = mydsp_faustpower2_f(2.0 * self.fRec10[0] + -1.0);
			self.fVec7[0] = fTemp32;
			let mut fTemp33: F32 = (fTemp0 * (fTemp32 - self.fVec7[1])) / fTemp30;
			self.fVec8[(self.IOTA0 & 4095) as usize] = fTemp33;
			let mut fTemp34: F32 = F32::max(0.0, F32::min(2047.0, self.fConst6 / fTemp29));
			let mut iTemp35: i32 = ((fTemp34) as i32);
			let mut fTemp36: F32 = F32::floor(fTemp34);
			self.fRec9[0] = 0.999000013 * self.fRec9[1] + self.fConst4 * (fTemp33 - self.fVec8[((self.IOTA0 - iTemp35) & 4095) as usize] * (fTemp36 + 1.0 - fTemp34) - (fTemp34 - fTemp36) * self.fVec8[((self.IOTA0 - (iTemp35 + 1)) & 4095) as usize]);
			self.fRec11[0] = fSlow2 + self.fConst3 * self.fRec11[1];
			self.fRec12[0] = fSlow3 + self.fConst3 * self.fRec12[1];
			self.fRec17[0] = fSlow4 + self.fConst3 * self.fRec17[1];
			let mut fTemp37: F32 = F32::powf(2.0, 0.0833333358 * (self.fRec17[0] + -69.0));
			let mut fTemp38: F32 = F32::max(1.1920929e-07, F32::abs(440.0 * fTemp37));
			let mut fTemp39: F32 = self.fConst5 * fTemp38;
			let mut fTemp40: F32 = self.fRec15[1] + fTemp39;
			let mut fTemp41: F32 = fTemp40 + -1.0;
			let mut iTemp42: i32 = ((fTemp41 < 0.0) as i32);
			self.fRec15[0] = if (iTemp42 as i32 != 0) { fTemp40 } else { fTemp41 };
			let mut fThen1: F32 = fTemp39 + self.fRec15[1] + fTemp41 * (1.0 - self.fConst0 / fTemp38);
			let mut fRec16: F32 = if (iTemp42 as i32 != 0) { fTemp40 } else { fThen1 };
			self.fRec18[0] = fSlow5 + self.fConst3 * self.fRec18[1];
			self.fRec21[0] = fSlow6 + self.fConst3 * self.fRec21[1];
			let mut fTemp43: F32 = self.fRec21[0] + 1.0;
			let mut fTemp44: F32 = F32::max(1.1920929e-07, F32::abs(440.0 * fTemp37 * fTemp43));
			let mut fTemp45: F32 = self.fRec19[1] + self.fConst5 * fTemp44;
			let mut fTemp46: F32 = fTemp45 + -1.0;
			let mut iTemp47: i32 = ((fTemp46 < 0.0) as i32);
			self.fRec19[0] = if (iTemp47 as i32 != 0) { fTemp45 } else { fTemp46 };
			let mut fThen3: F32 = fTemp45 + fTemp46 * (1.0 - self.fConst0 / fTemp44);
			let mut fRec20: F32 = if (iTemp47 as i32 != 0) { fTemp45 } else { fThen3 };
			let mut fTemp48: F32 = 1.0 - self.fRec21[0];
			let mut fTemp49: F32 = F32::max(1.1920929e-07, F32::abs(440.0 * fTemp37 * fTemp48));
			let mut fTemp50: F32 = self.fRec22[1] + self.fConst5 * fTemp49;
			let mut fTemp51: F32 = fTemp50 + -1.0;
			let mut iTemp52: i32 = ((fTemp51 < 0.0) as i32);
			self.fRec22[0] = if (iTemp52 as i32 != 0) { fTemp50 } else { fTemp51 };
			let mut fThen5: F32 = fTemp50 + fTemp51 * (1.0 - self.fConst0 / fTemp49);
			let mut fRec23: F32 = if (iTemp52 as i32 != 0) { fTemp50 } else { fThen5 };
			self.fRec24[0] = fSlow7 + self.fConst3 * self.fRec24[1];
			let mut fTemp53: F32 = F32::min(1.41419947, 1.41421354 * self.fRec24[0]);
			let mut fTemp54: F32 = fTemp53 * (fTemp53 + 1.41421354);
			self.fRec25[0] = fSlow8 + self.fConst3 * self.fRec25[1];
			let mut fTemp55: F32 = F32::tan(self.fConst8 * F32::max(20.0, F32::min(10000.0, 440.0 * F32::powf(2.0, 0.0833333358 * (self.fRec17[0] + self.fRec25[0] + -69.0)))));
			let mut fTemp56: F32 = 1.0 / fTemp55;
			let mut fTemp57: F32 = 1.41421354 * fTemp53;
			let mut fTemp58: F32 = fTemp57 + 2.0;
			let mut fTemp59: F32 = 1.0 - 1.0 / mydsp_faustpower2_f(fTemp55);
			let mut fTemp60: F32 = fTemp54 + (fTemp56 + fTemp58) / fTemp55 + 1.0;
			self.fRec14[0] = 2.0 * fRec16 + self.fRec18[0] * (0.0 - 2.0 * (1.0 - (fRec20 + fRec23))) + -1.0 - (self.fRec14[2] * (fTemp54 + (fTemp56 - fTemp58) / fTemp55 + 1.0) + 2.0 * self.fRec14[1] * (fTemp54 + fTemp59)) / fTemp60;
			let mut fTemp61: F32 = fTemp53 * (fTemp53 + -1.41421354);
			let mut fTemp62: F32 = 2.0 - fTemp57;
			let mut fTemp63: F32 = fTemp61 + (fTemp62 + fTemp56) / fTemp55 + 1.0;
			self.fRec13[0] = (self.fRec14[2] + self.fRec14[0] + 2.0 * self.fRec14[1]) / fTemp60 - (self.fRec13[2] * (fTemp61 + (fTemp56 - fTemp62) / fTemp55 + 1.0) + 2.0 * self.fRec13[1] * (fTemp61 + fTemp59)) / fTemp63;
			self.fRec26[0] = fSlow9 + self.fConst3 * self.fRec26[1];
			self.fRec31[0] = fSlow10 + self.fConst3 * self.fRec31[1];
			let mut fTemp64: F32 = F32::powf(2.0, 0.0833333358 * (self.fRec31[0] + -69.0));
			let mut fTemp65: F32 = F32::max(1.1920929e-07, F32::abs(440.0 * fTemp64));
			let mut fTemp66: F32 = self.fConst5 * fTemp65;
			let mut fTemp67: F32 = self.fRec29[1] + fTemp66;
			let mut fTemp68: F32 = fTemp67 + -1.0;
			let mut iTemp69: i32 = ((fTemp68 < 0.0) as i32);
			self.fRec29[0] = if (iTemp69 as i32 != 0) { fTemp67 } else { fTemp68 };
			let mut fThen7: F32 = fTemp66 + self.fRec29[1] + fTemp68 * (1.0 - self.fConst0 / fTemp65);
			let mut fRec30: F32 = if (iTemp69 as i32 != 0) { fTemp67 } else { fThen7 };
			let mut fTemp70: F32 = F32::max(1.1920929e-07, F32::abs(440.0 * fTemp43 * fTemp64));
			let mut fTemp71: F32 = self.fConst5 * fTemp70;
			let mut fTemp72: F32 = self.fRec32[1] + fTemp71;
			let mut fTemp73: F32 = fTemp72 + -1.0;
			let mut iTemp74: i32 = ((fTemp73 < 0.0) as i32);
			self.fRec32[0] = if (iTemp74 as i32 != 0) { fTemp72 } else { fTemp73 };
			let mut fThen9: F32 = fTemp71 + self.fRec32[1] + fTemp73 * (1.0 - self.fConst0 / fTemp70);
			let mut fRec33: F32 = if (iTemp74 as i32 != 0) { fTemp72 } else { fThen9 };
			let mut fTemp75: F32 = F32::max(1.1920929e-07, F32::abs(440.0 * fTemp48 * fTemp64));
			let mut fTemp76: F32 = self.fRec34[1] + self.fConst5 * fTemp75;
			let mut fTemp77: F32 = fTemp76 + -1.0;
			let mut iTemp78: i32 = ((fTemp77 < 0.0) as i32);
			self.fRec34[0] = if (iTemp78 as i32 != 0) { fTemp76 } else { fTemp77 };
			let mut fThen11: F32 = fTemp76 + fTemp77 * (1.0 - self.fConst0 / fTemp75);
			let mut fRec35: F32 = if (iTemp78 as i32 != 0) { fTemp76 } else { fThen11 };
			let mut fTemp79: F32 = F32::tan(self.fConst8 * F32::max(20.0, F32::min(10000.0, 440.0 * F32::powf(2.0, 0.0833333358 * (self.fRec25[0] + self.fRec31[0] + -69.0)))));
			let mut fTemp80: F32 = 1.0 / fTemp79;
			let mut fTemp81: F32 = 1.0 - 1.0 / mydsp_faustpower2_f(fTemp79);
			let mut fTemp82: F32 = fTemp54 + (fTemp58 + fTemp80) / fTemp79 + 1.0;
			self.fRec28[0] = 2.0 * fRec30 + self.fRec18[0] * (0.0 - 2.0 * (1.0 - (fRec33 + fRec35))) + -1.0 - (self.fRec28[2] * (fTemp54 + (fTemp80 - fTemp58) / fTemp79 + 1.0) + 2.0 * self.fRec28[1] * (fTemp54 + fTemp81)) / fTemp82;
			let mut fTemp83: F32 = fTemp61 + (fTemp62 + fTemp80) / fTemp79 + 1.0;
			self.fRec27[0] = (self.fRec28[2] + self.fRec28[0] + 2.0 * self.fRec28[1]) / fTemp82 - (self.fRec27[2] * (fTemp61 + (fTemp80 - fTemp62) / fTemp79 + 1.0) + 2.0 * self.fRec27[1] * (fTemp61 + fTemp81)) / fTemp83;
			self.fRec36[0] = fSlow11 + self.fConst3 * self.fRec36[1];
			self.fRec41[0] = fSlow12 + self.fConst3 * self.fRec41[1];
			let mut fTemp84: F32 = F32::powf(2.0, 0.0833333358 * (self.fRec41[0] + -69.0));
			let mut fTemp85: F32 = F32::max(1.1920929e-07, F32::abs(440.0 * fTemp84));
			let mut fTemp86: F32 = self.fConst5 * fTemp85;
			let mut fTemp87: F32 = self.fRec39[1] + fTemp86;
			let mut fTemp88: F32 = fTemp87 + -1.0;
			let mut iTemp89: i32 = ((fTemp88 < 0.0) as i32);
			self.fRec39[0] = if (iTemp89 as i32 != 0) { fTemp87 } else { fTemp88 };
			let mut fThen13: F32 = self.fRec39[1] + fTemp86 + fTemp88 * (1.0 - self.fConst0 / fTemp85);
			let mut fRec40: F32 = if (iTemp89 as i32 != 0) { fTemp87 } else { fThen13 };
			let mut fTemp90: F32 = F32::max(1.1920929e-07, F32::abs(440.0 * fTemp43 * fTemp84));
			let mut fTemp91: F32 = self.fRec42[1] + self.fConst5 * fTemp90;
			let mut fTemp92: F32 = fTemp91 + -1.0;
			let mut iTemp93: i32 = ((fTemp92 < 0.0) as i32);
			self.fRec42[0] = if (iTemp93 as i32 != 0) { fTemp91 } else { fTemp92 };
			let mut fThen15: F32 = fTemp91 + fTemp92 * (1.0 - self.fConst0 / fTemp90);
			let mut fRec43: F32 = if (iTemp93 as i32 != 0) { fTemp91 } else { fThen15 };
			let mut fTemp94: F32 = F32::max(1.1920929e-07, F32::abs(440.0 * fTemp48 * fTemp84));
			let mut fTemp95: F32 = self.fConst5 * fTemp94;
			let mut fTemp96: F32 = self.fRec44[1] + fTemp95;
			let mut fTemp97: F32 = fTemp96 + -1.0;
			let mut iTemp98: i32 = ((fTemp97 < 0.0) as i32);
			self.fRec44[0] = if (iTemp98 as i32 != 0) { fTemp96 } else { fTemp97 };
			let mut fThen17: F32 = fTemp95 + self.fRec44[1] + fTemp97 * (1.0 - self.fConst0 / fTemp94);
			let mut fRec45: F32 = if (iTemp98 as i32 != 0) { fTemp96 } else { fThen17 };
			let mut fTemp99: F32 = F32::tan(self.fConst8 * F32::max(20.0, F32::min(10000.0, 440.0 * F32::powf(2.0, 0.0833333358 * (self.fRec25[0] + self.fRec41[0] + -69.0)))));
			let mut fTemp100: F32 = 1.0 / fTemp99;
			let mut fTemp101: F32 = 1.0 - 1.0 / mydsp_faustpower2_f(fTemp99);
			let mut fTemp102: F32 = fTemp54 + (fTemp58 + fTemp100) / fTemp99 + 1.0;
			self.fRec38[0] = 2.0 * fRec40 + self.fRec18[0] * (0.0 - 2.0 * (1.0 - (fRec43 + fRec45))) + -1.0 - (self.fRec38[2] * (fTemp54 + (fTemp100 - fTemp58) / fTemp99 + 1.0) + 2.0 * self.fRec38[1] * (fTemp54 + fTemp101)) / fTemp102;
			let mut fTemp103: F32 = fTemp61 + (fTemp62 + fTemp100) / fTemp99 + 1.0;
			self.fRec37[0] = (self.fRec38[2] + self.fRec38[0] + 2.0 * self.fRec38[1]) / fTemp102 - (self.fRec37[2] * (fTemp61 + (fTemp100 - fTemp62) / fTemp99 + 1.0) + 2.0 * self.fRec37[1] * (fTemp61 + fTemp101)) / fTemp103;
			self.fRec46[0] = fSlow13 + self.fConst3 * self.fRec46[1];
			self.fRec51[0] = fSlow14 + self.fConst3 * self.fRec51[1];
			let mut fTemp104: F32 = F32::powf(2.0, 0.0833333358 * (self.fRec51[0] + -69.0));
			let mut fTemp105: F32 = F32::max(1.1920929e-07, F32::abs(440.0 * fTemp104));
			let mut fTemp106: F32 = self.fRec49[1] + self.fConst5 * fTemp105;
			let mut fTemp107: F32 = fTemp106 + -1.0;
			let mut iTemp108: i32 = ((fTemp107 < 0.0) as i32);
			self.fRec49[0] = if (iTemp108 as i32 != 0) { fTemp106 } else { fTemp107 };
			let mut fThen19: F32 = fTemp106 + fTemp107 * (1.0 - self.fConst0 / fTemp105);
			let mut fRec50: F32 = if (iTemp108 as i32 != 0) { fTemp106 } else { fThen19 };
			let mut fTemp109: F32 = F32::max(1.1920929e-07, F32::abs(440.0 * fTemp43 * fTemp104));
			let mut fTemp110: F32 = self.fConst5 * fTemp109;
			let mut fTemp111: F32 = self.fRec52[1] + fTemp110;
			let mut fTemp112: F32 = fTemp111 + -1.0;
			let mut iTemp113: i32 = ((fTemp112 < 0.0) as i32);
			self.fRec52[0] = if (iTemp113 as i32 != 0) { fTemp111 } else { fTemp112 };
			let mut fThen21: F32 = fTemp110 + self.fRec52[1] + fTemp112 * (1.0 - self.fConst0 / fTemp109);
			let mut fRec53: F32 = if (iTemp113 as i32 != 0) { fTemp111 } else { fThen21 };
			let mut fTemp114: F32 = F32::max(1.1920929e-07, F32::abs(440.0 * fTemp48 * fTemp104));
			let mut fTemp115: F32 = self.fRec54[1] + self.fConst5 * fTemp114;
			let mut fTemp116: F32 = fTemp115 + -1.0;
			let mut iTemp117: i32 = ((fTemp116 < 0.0) as i32);
			self.fRec54[0] = if (iTemp117 as i32 != 0) { fTemp115 } else { fTemp116 };
			let mut fThen23: F32 = fTemp115 + fTemp116 * (1.0 - self.fConst0 / fTemp114);
			let mut fRec55: F32 = if (iTemp117 as i32 != 0) { fTemp115 } else { fThen23 };
			let mut fTemp118: F32 = F32::tan(self.fConst8 * F32::max(20.0, F32::min(10000.0, 440.0 * F32::powf(2.0, 0.0833333358 * (self.fRec25[0] + self.fRec51[0] + -69.0)))));
			let mut fTemp119: F32 = 1.0 / fTemp118;
			let mut fTemp120: F32 = 1.0 - 1.0 / mydsp_faustpower2_f(fTemp118);
			let mut fTemp121: F32 = fTemp54 + (fTemp58 + fTemp119) / fTemp118 + 1.0;
			self.fRec48[0] = 2.0 * fRec50 + self.fRec18[0] * (0.0 - 2.0 * (1.0 - (fRec53 + fRec55))) + -1.0 - (self.fRec48[2] * (fTemp54 + (fTemp119 - fTemp58) / fTemp118 + 1.0) + 2.0 * self.fRec48[1] * (fTemp54 + fTemp120)) / fTemp121;
			let mut fTemp122: F32 = fTemp61 + (fTemp62 + fTemp119) / fTemp118 + 1.0;
			self.fRec47[0] = (self.fRec48[2] + self.fRec48[0] + 2.0 * self.fRec48[1]) / fTemp121 - (self.fRec47[2] * (fTemp61 + (fTemp119 - fTemp62) / fTemp118 + 1.0) + 2.0 * self.fRec47[1] * (fTemp61 + fTemp120)) / fTemp122;
			self.fRec0[(self.IOTA0 & 65535) as usize] = self.fConst1 * self.fRec1[0] * (self.fRec2[0] * fTemp1 + self.fRec5[0] * fTemp10 + self.fRec7[0] * fTemp19 + self.fRec9[0] * fTemp28) + 0.300000012 * self.fRec0[((self.IOTA0 - self.iConst7) & 65535) as usize] + 0.5 * self.fRec11[0] * ((self.fRec12[0] * (self.fRec13[2] + self.fRec13[0] + 2.0 * self.fRec13[1])) / fTemp63 + (self.fRec26[0] * (self.fRec27[2] + self.fRec27[0] + 2.0 * self.fRec27[1])) / fTemp83 + (self.fRec36[0] * (self.fRec37[2] + self.fRec37[0] + 2.0 * self.fRec37[1])) / fTemp103 + (self.fRec46[0] * (self.fRec47[2] + self.fRec47[0] + 2.0 * self.fRec47[1])) / fTemp122);
			self.fRec57[0] = fSlow16 + self.fConst3 * self.fRec57[1];
			let mut fTemp123: F32 = F32::powf(2.0, 0.0833333358 * (self.fRec57[0] + -69.0));
			let mut fTemp124: F32 = F32::tan(self.fConst8 * F32::max(20.0, F32::min(10000.0, 1760.0 * fTemp123)));
			let mut fTemp125: F32 = 1.0 / fTemp124;
			let mut fTemp126: F32 = (fTemp125 + -2.0) / fTemp124 + 1.0;
			let mut fTemp127: F32 = 1.0 - 1.0 / mydsp_faustpower2_f(fTemp124);
			self.fRec61[0] = fSlow17 + self.fConst3 * self.fRec61[1];
			self.fRec60[0] = 0.999000013 * self.fRec60[1] + 9.99999975e-05 * F32::powf(4.0, self.fRec61[0]);
			self.iRec64[0] = 1103515245 * self.iRec64[1] + 12345;
			self.fVec9[0] = fSlow18;
			self.iRec65[0] = (self.iRec65[1] + ((self.iRec65[1] > 0) as i32)) * ((fSlow18 <= self.fVec9[1]) as i32) + ((fSlow18 > self.fVec9[1]) as i32);
			let mut fTemp128: F32 = self.fConst9 * ((self.iRec65[0]) as F32);
			let mut fTemp129: F32 = 4.65661287e-10 * ((self.iRec64[0]) as F32) * F32::max(0.0, F32::min(fTemp128, 2.0 - fTemp128)) - self.fRec62[1] * fSlow21;
			self.fVec10[(self.IOTA0 & 2047) as usize] = fTemp129;
			let mut fTemp130: F32 = self.fConst10 / fTemp123;
			let mut fTemp131: F32 = fTemp130 + -1.0;
			let mut iTemp132: i32 = ((fTemp131) as i32);
			let mut fTemp133: F32 = F32::floor(fTemp131);
			self.fRec62[0] = self.fVec10[((self.IOTA0 - std::cmp::min(1025, std::cmp::max(0, iTemp132))) & 2047) as usize] * (fTemp133 + 2.0 - fTemp130) + (fTemp130 + -1.0 - fTemp133) * self.fVec10[((self.IOTA0 - std::cmp::min(1025, std::cmp::max(0, iTemp132 + 1))) & 2047) as usize];
			self.fRec63[0] = fTemp129;
			let mut fTemp134: F32 = F32::powf(2.0, 2.29999995 * self.fRec61[0]);
			let mut fTemp135: F32 = 1.0 - self.fConst11 * fTemp134 / F32::powf(2.0, 2.0 * (1.0 - self.fRec61[0]) + 1.0);
			self.fRec66[0] = 0.999000013 * self.fRec66[1] - 0.00200000009 * fTemp135 * F32::cos(self.fConst12 * fTemp134);
			self.fRec67[0] = 0.999000013 * self.fRec67[1] + 0.00100000005 * mydsp_faustpower2_f(fTemp135);
			self.fRec59[0] = self.fRec60[0] * self.fRec63[1] - (self.fRec66[0] * self.fRec59[1] + self.fRec67[0] * self.fRec59[2]);
			let mut fTemp136: F32 = (fTemp125 + 2.0) / fTemp124 + 1.0;
			self.fRec58[0] = self.fRec59[0] - (self.fRec59[1] + (self.fRec58[2] * fTemp126 + 2.0 * self.fRec58[1] * fTemp127) / fTemp136);
			self.fRec56[0] = 0.0 - ((fTemp126 * self.fRec56[2] + 2.0 * fTemp127 * self.fRec56[1]) - (self.fRec58[2] + self.fRec58[0] + 2.0 * self.fRec58[1])) / fTemp136;
			let mut fTemp137: F32 = self.fRec0[(self.IOTA0 & 65535) as usize] + fSlow15 * (self.fRec56[2] + self.fRec56[0] + 2.0 * self.fRec56[1]) / fTemp136;
			*output0 = ((fTemp137) as F32);
			*output1 = ((fTemp137) as F32);
			self.iVec0[1] = self.iVec0[0];
			self.fRec1[1] = self.fRec1[0];
			self.fRec4[1] = self.fRec4[0];
			self.fRec3[1] = self.fRec3[0];
			self.fVec1[1] = self.fVec1[0];
			self.IOTA0 = self.IOTA0 + 1;
			self.fRec2[1] = self.fRec2[0];
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
			self.fRec17[1] = self.fRec17[0];
			self.fRec15[1] = self.fRec15[0];
			self.fRec18[1] = self.fRec18[0];
			self.fRec21[1] = self.fRec21[0];
			self.fRec19[1] = self.fRec19[0];
			self.fRec22[1] = self.fRec22[0];
			self.fRec24[1] = self.fRec24[0];
			self.fRec25[1] = self.fRec25[0];
			self.fRec14[2] = self.fRec14[1];
			self.fRec14[1] = self.fRec14[0];
			self.fRec13[2] = self.fRec13[1];
			self.fRec13[1] = self.fRec13[0];
			self.fRec26[1] = self.fRec26[0];
			self.fRec31[1] = self.fRec31[0];
			self.fRec29[1] = self.fRec29[0];
			self.fRec32[1] = self.fRec32[0];
			self.fRec34[1] = self.fRec34[0];
			self.fRec28[2] = self.fRec28[1];
			self.fRec28[1] = self.fRec28[0];
			self.fRec27[2] = self.fRec27[1];
			self.fRec27[1] = self.fRec27[0];
			self.fRec36[1] = self.fRec36[0];
			self.fRec41[1] = self.fRec41[0];
			self.fRec39[1] = self.fRec39[0];
			self.fRec42[1] = self.fRec42[0];
			self.fRec44[1] = self.fRec44[0];
			self.fRec38[2] = self.fRec38[1];
			self.fRec38[1] = self.fRec38[0];
			self.fRec37[2] = self.fRec37[1];
			self.fRec37[1] = self.fRec37[0];
			self.fRec46[1] = self.fRec46[0];
			self.fRec51[1] = self.fRec51[0];
			self.fRec49[1] = self.fRec49[0];
			self.fRec52[1] = self.fRec52[0];
			self.fRec54[1] = self.fRec54[0];
			self.fRec48[2] = self.fRec48[1];
			self.fRec48[1] = self.fRec48[0];
			self.fRec47[2] = self.fRec47[1];
			self.fRec47[1] = self.fRec47[0];
			self.fRec57[1] = self.fRec57[0];
			self.fRec61[1] = self.fRec61[0];
			self.fRec60[1] = self.fRec60[0];
			self.iRec64[1] = self.iRec64[0];
			self.fVec9[1] = self.fVec9[0];
			self.iRec65[1] = self.iRec65[0];
			self.fRec62[1] = self.fRec62[0];
			self.fRec63[1] = self.fRec63[0];
			self.fRec66[1] = self.fRec66[0];
			self.fRec67[1] = self.fRec67[0];
			self.fRec59[2] = self.fRec59[1];
			self.fRec59[1] = self.fRec59[0];
			self.fRec58[2] = self.fRec58[1];
			self.fRec58[1] = self.fRec58[0];
			self.fRec56[2] = self.fRec56[1];
			self.fRec56[1] = self.fRec56[0];
		}
	}

}


}
pub use dsp::mydsp as Instrument;
