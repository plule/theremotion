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
	fRec16: [F32;2],
	fRec14: [F32;2],
	fHslider4: F32,
	fRec17: [F32;2],
	fHslider5: F32,
	fRec20: [F32;2],
	fRec18: [F32;2],
	fRec21: [F32;2],
	fHslider6: F32,
	fRec23: [F32;2],
	fConst8: F32,
	fHslider7: F32,
	fRec24: [F32;2],
	fRec13: [F32;3],
	fRec12: [F32;3],
	fHslider8: F32,
	fRec25: [F32;2],
	fHslider9: F32,
	fRec30: [F32;2],
	fRec28: [F32;2],
	fRec31: [F32;2],
	fRec33: [F32;2],
	fRec27: [F32;3],
	fRec26: [F32;3],
	fHslider10: F32,
	fRec35: [F32;2],
	fHslider11: F32,
	fRec40: [F32;2],
	fRec38: [F32;2],
	fRec41: [F32;2],
	fRec43: [F32;2],
	fRec37: [F32;3],
	fRec36: [F32;3],
	fHslider12: F32,
	fRec45: [F32;2],
	fHslider13: F32,
	fRec50: [F32;2],
	fRec48: [F32;2],
	fRec51: [F32;2],
	fRec53: [F32;2],
	fRec47: [F32;3],
	fRec46: [F32;3],
	fRec0: [F32;65536],
	fHslider14: F32,
	fRec62: [F32;2048],
	fConst9: F32,
	fConst10: F32,
	fHslider15: F32,
	fRec66: [F32;2],
	fHslider16: F32,
	fButton0: F32,
	fVec9: [F32;2],
	fVec10: [F32;2048],
	fRec58: [F32;2],
	fRec55: [F32;3],
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
			fRec16: [0.0;2],
			fRec14: [0.0;2],
			fHslider4: 0.0,
			fRec17: [0.0;2],
			fHslider5: 0.0,
			fRec20: [0.0;2],
			fRec18: [0.0;2],
			fRec21: [0.0;2],
			fHslider6: 0.0,
			fRec23: [0.0;2],
			fConst8: 0.0,
			fHslider7: 0.0,
			fRec24: [0.0;2],
			fRec13: [0.0;3],
			fRec12: [0.0;3],
			fHslider8: 0.0,
			fRec25: [0.0;2],
			fHslider9: 0.0,
			fRec30: [0.0;2],
			fRec28: [0.0;2],
			fRec31: [0.0;2],
			fRec33: [0.0;2],
			fRec27: [0.0;3],
			fRec26: [0.0;3],
			fHslider10: 0.0,
			fRec35: [0.0;2],
			fHslider11: 0.0,
			fRec40: [0.0;2],
			fRec38: [0.0;2],
			fRec41: [0.0;2],
			fRec43: [0.0;2],
			fRec37: [0.0;3],
			fRec36: [0.0;3],
			fHslider12: 0.0,
			fRec45: [0.0;2],
			fHslider13: 0.0,
			fRec50: [0.0;2],
			fRec48: [0.0;2],
			fRec51: [0.0;2],
			fRec53: [0.0;2],
			fRec47: [0.0;3],
			fRec46: [0.0;3],
			fRec0: [0.0;65536],
			fHslider14: 0.0,
			fRec62: [0.0;2048],
			fConst9: 0.0,
			fConst10: 0.0,
			fHslider15: 0.0,
			fRec66: [0.0;2],
			fHslider16: 0.0,
			fButton0: 0.0,
			fVec9: [0.0;2],
			fVec10: [0.0;2048],
			fRec58: [0.0;2],
			fRec55: [0.0;3],
			fRec56: [0.0;3],
		}
	}
	fn metadata(&self, m: &mut dyn Meta) { 
		m.declare("author", "Pierre Lulé");
		m.declare("basics.lib/name", "Faust Basic Element Library");
		m.declare("basics.lib/version", "0.1");
		m.declare("delays.lib/name", "Faust Delay Library");
		m.declare("delays.lib/version", "0.1");
		m.declare("filename", "instrument.dsp");
		m.declare("filters.lib/fir:author", "Julius O. Smith III");
		m.declare("filters.lib/fir:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/fir:license", "MIT-style STK-4.3 license");
		m.declare("filters.lib/iir:author", "Julius O. Smith III");
		m.declare("filters.lib/iir:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/iir:license", "MIT-style STK-4.3 license");
		m.declare("filters.lib/lowpass0_highpass1", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
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
		self.fHslider1 = 60.0;
		self.fHslider2 = 0.0;
		self.fHslider3 = 60.0;
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
		self.fHslider16 = 0.800000012;
		self.fButton0 = 0.0;
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
			self.fRec16[(l20) as usize] = 0.0;
		}
		for l21 in 0..2 {
			self.fRec14[(l21) as usize] = 0.0;
		}
		for l22 in 0..2 {
			self.fRec17[(l22) as usize] = 0.0;
		}
		for l23 in 0..2 {
			self.fRec20[(l23) as usize] = 0.0;
		}
		for l24 in 0..2 {
			self.fRec18[(l24) as usize] = 0.0;
		}
		for l25 in 0..2 {
			self.fRec21[(l25) as usize] = 0.0;
		}
		for l26 in 0..2 {
			self.fRec23[(l26) as usize] = 0.0;
		}
		for l27 in 0..2 {
			self.fRec24[(l27) as usize] = 0.0;
		}
		for l28 in 0..3 {
			self.fRec13[(l28) as usize] = 0.0;
		}
		for l29 in 0..3 {
			self.fRec12[(l29) as usize] = 0.0;
		}
		for l30 in 0..2 {
			self.fRec25[(l30) as usize] = 0.0;
		}
		for l31 in 0..2 {
			self.fRec30[(l31) as usize] = 0.0;
		}
		for l32 in 0..2 {
			self.fRec28[(l32) as usize] = 0.0;
		}
		for l33 in 0..2 {
			self.fRec31[(l33) as usize] = 0.0;
		}
		for l34 in 0..2 {
			self.fRec33[(l34) as usize] = 0.0;
		}
		for l35 in 0..3 {
			self.fRec27[(l35) as usize] = 0.0;
		}
		for l36 in 0..3 {
			self.fRec26[(l36) as usize] = 0.0;
		}
		for l37 in 0..2 {
			self.fRec35[(l37) as usize] = 0.0;
		}
		for l38 in 0..2 {
			self.fRec40[(l38) as usize] = 0.0;
		}
		for l39 in 0..2 {
			self.fRec38[(l39) as usize] = 0.0;
		}
		for l40 in 0..2 {
			self.fRec41[(l40) as usize] = 0.0;
		}
		for l41 in 0..2 {
			self.fRec43[(l41) as usize] = 0.0;
		}
		for l42 in 0..3 {
			self.fRec37[(l42) as usize] = 0.0;
		}
		for l43 in 0..3 {
			self.fRec36[(l43) as usize] = 0.0;
		}
		for l44 in 0..2 {
			self.fRec45[(l44) as usize] = 0.0;
		}
		for l45 in 0..2 {
			self.fRec50[(l45) as usize] = 0.0;
		}
		for l46 in 0..2 {
			self.fRec48[(l46) as usize] = 0.0;
		}
		for l47 in 0..2 {
			self.fRec51[(l47) as usize] = 0.0;
		}
		for l48 in 0..2 {
			self.fRec53[(l48) as usize] = 0.0;
		}
		for l49 in 0..3 {
			self.fRec47[(l49) as usize] = 0.0;
		}
		for l50 in 0..3 {
			self.fRec46[(l50) as usize] = 0.0;
		}
		for l51 in 0..65536 {
			self.fRec0[(l51) as usize] = 0.0;
		}
		for l52 in 0..2048 {
			self.fRec62[(l52) as usize] = 0.0;
		}
		for l53 in 0..2 {
			self.fRec66[(l53) as usize] = 0.0;
		}
		for l54 in 0..2 {
			self.fVec9[(l54) as usize] = 0.0;
		}
		for l55 in 0..2048 {
			self.fVec10[(l55) as usize] = 0.0;
		}
		for l56 in 0..2 {
			self.fRec58[(l56) as usize] = 0.0;
		}
		for l57 in 0..3 {
			self.fRec55[(l57) as usize] = 0.0;
		}
		for l58 in 0..3 {
			self.fRec56[(l58) as usize] = 0.0;
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
		self.fConst9 = 0.00882352982 * self.fConst0;
		self.fConst10 = 0.00147058826 * self.fConst0;
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
		ui_interface.add_horizontal_slider("vol1", ParamIndex(0), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(1)), "1", "");
		ui_interface.add_horizontal_slider("note1", ParamIndex(1), 60.0, 0.0, 127.0, 0.001);
		ui_interface.declare(Some(ParamIndex(2)), "2", "");
		ui_interface.add_horizontal_slider("vol2", ParamIndex(2), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(3)), "3", "");
		ui_interface.add_horizontal_slider("note2", ParamIndex(3), 60.0, 0.0, 127.0, 0.001);
		ui_interface.declare(Some(ParamIndex(4)), "4", "");
		ui_interface.add_horizontal_slider("vol3", ParamIndex(4), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(5)), "5", "");
		ui_interface.add_horizontal_slider("note3", ParamIndex(5), 60.0, 0.0, 127.0, 0.001);
		ui_interface.declare(Some(ParamIndex(6)), "6", "");
		ui_interface.add_horizontal_slider("vol4", ParamIndex(6), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(7)), "7", "");
		ui_interface.add_horizontal_slider("note4", ParamIndex(7), 60.0, 0.0, 127.0, 0.001);
		ui_interface.declare(None, "8", "");
		ui_interface.open_horizontal_box("supersaw");
		ui_interface.declare(Some(ParamIndex(8)), "0", "");
		ui_interface.add_horizontal_slider("volume", ParamIndex(8), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(9)), "1", "");
		ui_interface.add_horizontal_slider("detune", ParamIndex(9), 0.001, 0.001, 0.02, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "9", "");
		ui_interface.open_horizontal_box("filter");
		ui_interface.declare(Some(ParamIndex(10)), "0", "");
		ui_interface.add_horizontal_slider("cutoff_note", ParamIndex(10), 0.0, -20.0, 50.0, 0.001);
		ui_interface.declare(Some(ParamIndex(11)), "1", "");
		ui_interface.add_horizontal_slider("res", ParamIndex(11), 0.0, 0.0, 0.98999999999999999, 0.001);
		ui_interface.close_box();
		ui_interface.close_box();
		ui_interface.declare(None, "1", "");
		ui_interface.open_horizontal_box("pluck");
		ui_interface.declare(Some(ParamIndex(12)), "0", "");
		ui_interface.add_button("gate", ParamIndex(12));
		ui_interface.declare(Some(ParamIndex(13)), "1", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(13), 60.0, 0.0, 127.0, 0.001);
		ui_interface.declare(Some(ParamIndex(14)), "2", "");
		ui_interface.add_horizontal_slider("gain", ParamIndex(14), 0.80000000000000004, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(15)), "3", "");
		ui_interface.add_horizontal_slider("damping", ParamIndex(15), 0.0, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "2", "");
		ui_interface.open_horizontal_box("drone");
		ui_interface.declare(Some(ParamIndex(16)), "0", "");
		ui_interface.add_horizontal_slider("volume", ParamIndex(16), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(17)), "1", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(17), 60.0, 0.0, 127.0, 0.001);
		ui_interface.close_box();
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			12 => Some(self.fButton0),
			16 => Some(self.fHslider0),
			17 => Some(self.fHslider1),
			4 => Some(self.fHslider10),
			5 => Some(self.fHslider11),
			6 => Some(self.fHslider12),
			7 => Some(self.fHslider13),
			15 => Some(self.fHslider14),
			13 => Some(self.fHslider15),
			14 => Some(self.fHslider16),
			0 => Some(self.fHslider2),
			1 => Some(self.fHslider3),
			8 => Some(self.fHslider4),
			9 => Some(self.fHslider5),
			11 => Some(self.fHslider6),
			10 => Some(self.fHslider7),
			2 => Some(self.fHslider8),
			3 => Some(self.fHslider9),
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			12 => { self.fButton0 = value }
			16 => { self.fHslider0 = value }
			17 => { self.fHslider1 = value }
			4 => { self.fHslider10 = value }
			5 => { self.fHslider11 = value }
			6 => { self.fHslider12 = value }
			7 => { self.fHslider13 = value }
			15 => { self.fHslider14 = value }
			13 => { self.fHslider15 = value }
			14 => { self.fHslider16 = value }
			0 => { self.fHslider2 = value }
			1 => { self.fHslider3 = value }
			8 => { self.fHslider4 = value }
			9 => { self.fHslider5 = value }
			11 => { self.fHslider6 = value }
			10 => { self.fHslider7 = value }
			2 => { self.fHslider8 = value }
			3 => { self.fHslider9 = value }
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
		let mut fSlow14: F32 = 0.5 * (0.200000003 * (1.0 - ((self.fHslider14) as F32)) + 0.800000012);
		let mut fSlow15: F32 = self.fConst2 * ((self.fHslider15) as F32);
		let mut fSlow16: F32 = ((self.fHslider16) as F32);
		let mut fSlow17: F32 = ((self.fButton0) as F32);
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
			let mut fTemp21: F32 = F32::max(0.0, F32::min(2047.0, self.fConst6 / fTemp20));
			let mut fTemp22: F32 = F32::floor(fTemp21);
			let mut fTemp23: F32 = F32::max(20.0, F32::abs(fTemp20));
			let mut fTemp24: F32 = self.fRec8[1] + self.fConst5 * fTemp23;
			self.fRec8[0] = fTemp24 - F32::floor(fTemp24);
			let mut fTemp25: F32 = mydsp_faustpower2_f(2.0 * self.fRec8[0] + -1.0);
			self.fVec5[0] = fTemp25;
			let mut fTemp26: F32 = (fTemp0 * (fTemp25 - self.fVec5[1])) / fTemp23;
			self.fVec6[(self.IOTA0 & 4095) as usize] = fTemp26;
			let mut iTemp27: i32 = ((fTemp21) as i32);
			self.fRec7[0] = 0.999000013 * self.fRec7[1] - self.fConst4 * ((fTemp21 - fTemp22) * self.fVec6[((self.IOTA0 - (iTemp27 + 1)) & 4095) as usize] - fTemp26 - self.fVec6[((self.IOTA0 - iTemp27) & 4095) as usize] * (fTemp22 + 1.0 - fTemp21));
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
			self.fRec9[0] = 0.999000013 * self.fRec9[1] - self.fConst4 * (self.fVec8[((self.IOTA0 - iTemp35) & 4095) as usize] * (fTemp36 + 1.0 - fTemp34) - fTemp33 + (fTemp34 - fTemp36) * self.fVec8[((self.IOTA0 - (iTemp35 + 1)) & 4095) as usize]);
			self.fRec11[0] = fSlow2 + self.fConst3 * self.fRec11[1];
			self.fRec16[0] = fSlow3 + self.fConst3 * self.fRec16[1];
			let mut fTemp37: F32 = F32::powf(2.0, 0.0833333358 * (self.fRec16[0] + -69.0));
			let mut fTemp38: F32 = F32::max(1.1920929e-07, F32::abs(440.0 * fTemp37));
			let mut fTemp39: F32 = self.fConst5 * fTemp38;
			let mut fTemp40: F32 = self.fRec14[1] + fTemp39;
			let mut fTemp41: F32 = fTemp40 + -1.0;
			let mut iTemp42: i32 = ((fTemp41 < 0.0) as i32);
			self.fRec14[0] = if (iTemp42 as i32 != 0) { fTemp40 } else { fTemp41 };
			let mut fThen1: F32 = self.fRec14[1] + fTemp39 + fTemp41 * (1.0 - self.fConst0 / fTemp38);
			let mut fRec15: F32 = if (iTemp42 as i32 != 0) { fTemp40 } else { fThen1 };
			self.fRec17[0] = fSlow4 + self.fConst3 * self.fRec17[1];
			self.fRec20[0] = fSlow5 + self.fConst3 * self.fRec20[1];
			let mut fTemp43: F32 = self.fRec20[0] + 1.0;
			let mut fTemp44: F32 = F32::max(1.1920929e-07, F32::abs(440.0 * fTemp37 * fTemp43));
			let mut fTemp45: F32 = self.fConst5 * fTemp44;
			let mut fTemp46: F32 = self.fRec18[1] + fTemp45;
			let mut fTemp47: F32 = fTemp46 + -1.0;
			let mut iTemp48: i32 = ((fTemp47 < 0.0) as i32);
			self.fRec18[0] = if (iTemp48 as i32 != 0) { fTemp46 } else { fTemp47 };
			let mut fThen3: F32 = fTemp45 + self.fRec18[1] + fTemp47 * (1.0 - self.fConst0 / fTemp44);
			let mut fRec19: F32 = if (iTemp48 as i32 != 0) { fTemp46 } else { fThen3 };
			let mut fTemp49: F32 = 1.0 - self.fRec20[0];
			let mut fTemp50: F32 = F32::max(1.1920929e-07, F32::abs(440.0 * fTemp37 * fTemp49));
			let mut fTemp51: F32 = self.fRec21[1] + self.fConst5 * fTemp50;
			let mut fTemp52: F32 = fTemp51 + -1.0;
			let mut iTemp53: i32 = ((fTemp52 < 0.0) as i32);
			self.fRec21[0] = if (iTemp53 as i32 != 0) { fTemp51 } else { fTemp52 };
			let mut fThen5: F32 = fTemp51 + fTemp52 * (1.0 - self.fConst0 / fTemp50);
			let mut fRec22: F32 = if (iTemp53 as i32 != 0) { fTemp51 } else { fThen5 };
			self.fRec23[0] = fSlow6 + self.fConst3 * self.fRec23[1];
			let mut fTemp54: F32 = F32::min(1.41419947, 1.41421354 * self.fRec23[0]);
			let mut fTemp55: F32 = fTemp54 * (fTemp54 + 1.41421354);
			let mut fTemp56: F32 = 1.41421354 * fTemp54;
			let mut fTemp57: F32 = fTemp56 + 2.0;
			self.fRec24[0] = fSlow7 + self.fConst3 * self.fRec24[1];
			let mut fTemp58: F32 = F32::tan(self.fConst8 * F32::max(20.0, F32::min(10000.0, 440.0 * F32::powf(2.0, 0.0833333358 * (self.fRec16[0] + self.fRec24[0] + -69.0)))));
			let mut fTemp59: F32 = 1.0 / fTemp58;
			let mut fTemp60: F32 = 1.0 - 1.0 / mydsp_faustpower2_f(fTemp58);
			let mut fTemp61: F32 = fTemp55 + (fTemp59 + fTemp57) / fTemp58 + 1.0;
			self.fRec13[0] = 2.0 * fRec15 + self.fRec17[0] * (0.0 - 2.0 * (1.0 - (fRec19 + fRec22))) + -1.0 - (self.fRec13[2] * (fTemp55 + 1.0 - (fTemp57 - fTemp59) / fTemp58) + 2.0 * self.fRec13[1] * (fTemp55 + fTemp60)) / fTemp61;
			let mut fTemp62: F32 = fTemp54 * (fTemp54 + -1.41421354);
			let mut fTemp63: F32 = 2.0 - fTemp56;
			let mut fTemp64: F32 = fTemp62 + (fTemp63 + fTemp59) / fTemp58 + 1.0;
			self.fRec12[0] = (self.fRec13[2] + self.fRec13[0] + 2.0 * self.fRec13[1]) / fTemp61 - (self.fRec12[2] * (fTemp62 + (fTemp59 - fTemp63) / fTemp58 + 1.0) + 2.0 * self.fRec12[1] * (fTemp62 + fTemp60)) / fTemp64;
			self.fRec25[0] = fSlow8 + self.fConst3 * self.fRec25[1];
			self.fRec30[0] = fSlow9 + self.fConst3 * self.fRec30[1];
			let mut fTemp65: F32 = F32::powf(2.0, 0.0833333358 * (self.fRec30[0] + -69.0));
			let mut fTemp66: F32 = F32::max(1.1920929e-07, F32::abs(440.0 * fTemp65));
			let mut fTemp67: F32 = self.fConst5 * fTemp66;
			let mut fTemp68: F32 = self.fRec28[1] + fTemp67;
			let mut fTemp69: F32 = fTemp68 + -1.0;
			let mut iTemp70: i32 = ((fTemp69 < 0.0) as i32);
			self.fRec28[0] = if (iTemp70 as i32 != 0) { fTemp68 } else { fTemp69 };
			let mut fThen7: F32 = self.fRec28[1] + fTemp67 + fTemp69 * (1.0 - self.fConst0 / fTemp66);
			let mut fRec29: F32 = if (iTemp70 as i32 != 0) { fTemp68 } else { fThen7 };
			let mut fTemp71: F32 = F32::max(1.1920929e-07, F32::abs(440.0 * fTemp43 * fTemp65));
			let mut fTemp72: F32 = self.fConst5 * fTemp71;
			let mut fTemp73: F32 = self.fRec31[1] + fTemp72;
			let mut fTemp74: F32 = fTemp73 + -1.0;
			let mut iTemp75: i32 = ((fTemp74 < 0.0) as i32);
			self.fRec31[0] = if (iTemp75 as i32 != 0) { fTemp73 } else { fTemp74 };
			let mut fThen9: F32 = fTemp72 + self.fRec31[1] + fTemp74 * (1.0 - self.fConst0 / fTemp71);
			let mut fRec32: F32 = if (iTemp75 as i32 != 0) { fTemp73 } else { fThen9 };
			let mut fTemp76: F32 = F32::max(1.1920929e-07, F32::abs(440.0 * fTemp49 * fTemp65));
			let mut fTemp77: F32 = self.fConst5 * fTemp76;
			let mut fTemp78: F32 = self.fRec33[1] + fTemp77;
			let mut fTemp79: F32 = fTemp78 + -1.0;
			let mut iTemp80: i32 = ((fTemp79 < 0.0) as i32);
			self.fRec33[0] = if (iTemp80 as i32 != 0) { fTemp78 } else { fTemp79 };
			let mut fThen11: F32 = fTemp77 + self.fRec33[1] + fTemp79 * (1.0 - self.fConst0 / fTemp76);
			let mut fRec34: F32 = if (iTemp80 as i32 != 0) { fTemp78 } else { fThen11 };
			let mut fTemp81: F32 = F32::tan(self.fConst8 * F32::max(20.0, F32::min(10000.0, 440.0 * F32::powf(2.0, 0.0833333358 * (self.fRec24[0] + self.fRec30[0] + -69.0)))));
			let mut fTemp82: F32 = 1.0 / fTemp81;
			let mut fTemp83: F32 = 1.0 - 1.0 / mydsp_faustpower2_f(fTemp81);
			let mut fTemp84: F32 = fTemp55 + (fTemp57 + fTemp82) / fTemp81 + 1.0;
			self.fRec27[0] = 2.0 * fRec29 + self.fRec17[0] * (0.0 - 2.0 * (1.0 - (fRec32 + fRec34))) + -1.0 - (self.fRec27[2] * (fTemp55 + (fTemp82 - fTemp57) / fTemp81 + 1.0) + 2.0 * self.fRec27[1] * (fTemp55 + fTemp83)) / fTemp84;
			let mut fTemp85: F32 = fTemp62 + (fTemp63 + fTemp82) / fTemp81 + 1.0;
			self.fRec26[0] = (self.fRec27[2] + self.fRec27[0] + 2.0 * self.fRec27[1]) / fTemp84 - (self.fRec26[2] * (fTemp62 + (fTemp82 - fTemp63) / fTemp81 + 1.0) + 2.0 * self.fRec26[1] * (fTemp62 + fTemp83)) / fTemp85;
			self.fRec35[0] = fSlow10 + self.fConst3 * self.fRec35[1];
			self.fRec40[0] = fSlow11 + self.fConst3 * self.fRec40[1];
			let mut fTemp86: F32 = F32::powf(2.0, 0.0833333358 * (self.fRec40[0] + -69.0));
			let mut fTemp87: F32 = F32::max(1.1920929e-07, F32::abs(440.0 * fTemp86));
			let mut fTemp88: F32 = self.fRec38[1] + self.fConst5 * fTemp87;
			let mut fTemp89: F32 = fTemp88 + -1.0;
			let mut iTemp90: i32 = ((fTemp89 < 0.0) as i32);
			self.fRec38[0] = if (iTemp90 as i32 != 0) { fTemp88 } else { fTemp89 };
			let mut fThen13: F32 = fTemp88 + fTemp89 * (1.0 - self.fConst0 / fTemp87);
			let mut fRec39: F32 = if (iTemp90 as i32 != 0) { fTemp88 } else { fThen13 };
			let mut fTemp91: F32 = F32::max(1.1920929e-07, F32::abs(440.0 * fTemp43 * fTemp86));
			let mut fTemp92: F32 = self.fConst5 * fTemp91;
			let mut fTemp93: F32 = self.fRec41[1] + fTemp92;
			let mut fTemp94: F32 = fTemp93 + -1.0;
			let mut iTemp95: i32 = ((fTemp94 < 0.0) as i32);
			self.fRec41[0] = if (iTemp95 as i32 != 0) { fTemp93 } else { fTemp94 };
			let mut fThen15: F32 = self.fRec41[1] + fTemp92 + fTemp94 * (1.0 - self.fConst0 / fTemp91);
			let mut fRec42: F32 = if (iTemp95 as i32 != 0) { fTemp93 } else { fThen15 };
			let mut fTemp96: F32 = F32::max(1.1920929e-07, F32::abs(440.0 * fTemp49 * fTemp86));
			let mut fTemp97: F32 = self.fConst5 * fTemp96;
			let mut fTemp98: F32 = self.fRec43[1] + fTemp97;
			let mut fTemp99: F32 = fTemp98 + -1.0;
			let mut iTemp100: i32 = ((fTemp99 < 0.0) as i32);
			self.fRec43[0] = if (iTemp100 as i32 != 0) { fTemp98 } else { fTemp99 };
			let mut fThen17: F32 = self.fRec43[1] + fTemp97 + fTemp99 * (1.0 - self.fConst0 / fTemp96);
			let mut fRec44: F32 = if (iTemp100 as i32 != 0) { fTemp98 } else { fThen17 };
			let mut fTemp101: F32 = F32::tan(self.fConst8 * F32::max(20.0, F32::min(10000.0, 440.0 * F32::powf(2.0, 0.0833333358 * (self.fRec24[0] + self.fRec40[0] + -69.0)))));
			let mut fTemp102: F32 = 1.0 / fTemp101;
			let mut fTemp103: F32 = 1.0 - 1.0 / mydsp_faustpower2_f(fTemp101);
			let mut fTemp104: F32 = fTemp55 + (fTemp57 + fTemp102) / fTemp101 + 1.0;
			self.fRec37[0] = 2.0 * fRec39 + self.fRec17[0] * (0.0 - 2.0 * (1.0 - (fRec42 + fRec44))) + -1.0 - (self.fRec37[2] * (fTemp55 + (fTemp102 - fTemp57) / fTemp101 + 1.0) + 2.0 * self.fRec37[1] * (fTemp55 + fTemp103)) / fTemp104;
			let mut fTemp105: F32 = fTemp62 + (fTemp63 + fTemp102) / fTemp101 + 1.0;
			self.fRec36[0] = (self.fRec37[2] + self.fRec37[0] + 2.0 * self.fRec37[1]) / fTemp104 - (self.fRec36[2] * (fTemp62 + (fTemp102 - fTemp63) / fTemp101 + 1.0) + 2.0 * self.fRec36[1] * (fTemp62 + fTemp103)) / fTemp105;
			self.fRec45[0] = fSlow12 + self.fConst3 * self.fRec45[1];
			self.fRec50[0] = fSlow13 + self.fConst3 * self.fRec50[1];
			let mut fTemp106: F32 = F32::powf(2.0, 0.0833333358 * (self.fRec50[0] + -69.0));
			let mut fTemp107: F32 = F32::max(1.1920929e-07, F32::abs(440.0 * fTemp106));
			let mut fTemp108: F32 = self.fRec48[1] + self.fConst5 * fTemp107;
			let mut fTemp109: F32 = fTemp108 + -1.0;
			let mut iTemp110: i32 = ((fTemp109 < 0.0) as i32);
			self.fRec48[0] = if (iTemp110 as i32 != 0) { fTemp108 } else { fTemp109 };
			let mut fThen19: F32 = fTemp108 + fTemp109 * (1.0 - self.fConst0 / fTemp107);
			let mut fRec49: F32 = if (iTemp110 as i32 != 0) { fTemp108 } else { fThen19 };
			let mut fTemp111: F32 = F32::max(1.1920929e-07, F32::abs(440.0 * fTemp43 * fTemp106));
			let mut fTemp112: F32 = self.fConst5 * fTemp111;
			let mut fTemp113: F32 = self.fRec51[1] + fTemp112;
			let mut fTemp114: F32 = fTemp113 + -1.0;
			let mut iTemp115: i32 = ((fTemp114 < 0.0) as i32);
			self.fRec51[0] = if (iTemp115 as i32 != 0) { fTemp113 } else { fTemp114 };
			let mut fThen21: F32 = fTemp112 + self.fRec51[1] + fTemp114 * (1.0 - self.fConst0 / fTemp111);
			let mut fRec52: F32 = if (iTemp115 as i32 != 0) { fTemp113 } else { fThen21 };
			let mut fTemp116: F32 = F32::max(1.1920929e-07, F32::abs(440.0 * fTemp49 * fTemp106));
			let mut fTemp117: F32 = self.fConst5 * fTemp116;
			let mut fTemp118: F32 = self.fRec53[1] + fTemp117;
			let mut fTemp119: F32 = fTemp118 + -1.0;
			let mut iTemp120: i32 = ((fTemp119 < 0.0) as i32);
			self.fRec53[0] = if (iTemp120 as i32 != 0) { fTemp118 } else { fTemp119 };
			let mut fThen23: F32 = fTemp117 + self.fRec53[1] + fTemp119 * (1.0 - self.fConst0 / fTemp116);
			let mut fRec54: F32 = if (iTemp120 as i32 != 0) { fTemp118 } else { fThen23 };
			let mut fTemp121: F32 = F32::tan(self.fConst8 * F32::max(20.0, F32::min(10000.0, 440.0 * F32::powf(2.0, 0.0833333358 * (self.fRec24[0] + self.fRec50[0] + -69.0)))));
			let mut fTemp122: F32 = 1.0 / fTemp121;
			let mut fTemp123: F32 = 1.0 - 1.0 / mydsp_faustpower2_f(fTemp121);
			let mut fTemp124: F32 = fTemp55 + (fTemp57 + fTemp122) / fTemp121 + 1.0;
			self.fRec47[0] = 2.0 * fRec49 + self.fRec17[0] * (0.0 - 2.0 * (1.0 - (fRec52 + fRec54))) + -1.0 - (self.fRec47[2] * (fTemp55 + (fTemp122 - fTemp57) / fTemp121 + 1.0) + 2.0 * self.fRec47[1] * (fTemp55 + fTemp123)) / fTemp124;
			let mut fTemp125: F32 = fTemp62 + (fTemp63 + fTemp122) / fTemp121 + 1.0;
			self.fRec46[0] = (self.fRec47[2] + self.fRec47[0] + 2.0 * self.fRec47[1]) / fTemp124 - (self.fRec46[2] * (fTemp62 + (fTemp122 - fTemp63) / fTemp121 + 1.0) + 2.0 * self.fRec46[1] * (fTemp62 + fTemp123)) / fTemp125;
			self.fRec0[(self.IOTA0 & 65535) as usize] = self.fConst1 * self.fRec1[0] * (self.fRec2[0] * fTemp1 + self.fRec5[0] * fTemp10 + self.fRec7[0] * fTemp19 + self.fRec9[0] * fTemp28) + 0.300000012 * self.fRec0[((self.IOTA0 - self.iConst7) & 65535) as usize] + 0.5 * ((self.fRec11[0] * (self.fRec12[2] + self.fRec12[0] + 2.0 * self.fRec12[1])) / fTemp64 + (self.fRec25[0] * (self.fRec26[2] + self.fRec26[0] + 2.0 * self.fRec26[1])) / fTemp85 + (self.fRec35[0] * (self.fRec36[2] + self.fRec36[0] + 2.0 * self.fRec36[1])) / fTemp105 + (self.fRec45[0] * (self.fRec46[2] + self.fRec46[0] + 2.0 * self.fRec46[1])) / fTemp125);
			let mut fTemp126: F32 = fSlow14 * (self.fRec56[1] + self.fRec56[2]);
			self.fRec62[(self.IOTA0 & 2047) as usize] = fTemp126;
			self.fRec66[0] = fSlow15 + self.fConst3 * self.fRec66[1];
			let mut fTemp127: F32 = self.fConst10 * (0.772727251 / F32::powf(2.0, 0.0833333358 * (self.fRec66[0] + -69.0)) + -0.0500000007);
			let mut fTemp128: F32 = fTemp127 + -1.49999499;
			let mut iTemp129: i32 = ((fTemp128) as i32);
			let mut iTemp130: i32 = ((F32::min(self.fConst9, ((std::cmp::max(0, iTemp129)) as F32))) as i32) + 1;
			let mut fTemp131: F32 = F32::floor(fTemp128);
			let mut fTemp132: F32 = fTemp127 + -1.0 - fTemp131;
			let mut fTemp133: F32 = 0.0 - fTemp132;
			let mut fTemp134: F32 = fTemp127 + -2.0 - fTemp131;
			let mut fTemp135: F32 = 0.0 - 0.5 * fTemp134;
			let mut fTemp136: F32 = fTemp127 + -3.0 - fTemp131;
			let mut fTemp137: F32 = 0.0 - 0.333333343 * fTemp136;
			let mut fTemp138: F32 = fTemp127 + -4.0 - fTemp131;
			let mut fTemp139: F32 = 0.0 - 0.25 * fTemp138;
			let mut fTemp140: F32 = fTemp127 - fTemp131;
			let mut iTemp141: i32 = ((F32::min(self.fConst9, ((std::cmp::max(0, iTemp129 + 1)) as F32))) as i32) + 1;
			let mut fTemp142: F32 = 0.0 - fTemp134;
			let mut fTemp143: F32 = 0.0 - 0.5 * fTemp136;
			let mut fTemp144: F32 = 0.0 - 0.333333343 * fTemp138;
			let mut iTemp145: i32 = ((F32::min(self.fConst9, ((std::cmp::max(0, iTemp129 + 2)) as F32))) as i32) + 1;
			let mut fTemp146: F32 = 0.0 - fTemp136;
			let mut fTemp147: F32 = 0.0 - 0.5 * fTemp138;
			let mut fTemp148: F32 = fTemp132 * fTemp134;
			let mut iTemp149: i32 = ((F32::min(self.fConst9, ((std::cmp::max(0, iTemp129 + 3)) as F32))) as i32) + 1;
			let mut fTemp150: F32 = 0.0 - fTemp138;
			let mut fTemp151: F32 = fTemp148 * fTemp136;
			let mut iTemp152: i32 = ((F32::min(self.fConst9, ((std::cmp::max(0, iTemp129 + 4)) as F32))) as i32) + 1;
			let mut fRec63: F32 = self.fRec62[((self.IOTA0 - iTemp130) & 2047) as usize] * fTemp133 * fTemp135 * fTemp137 * fTemp139 + fTemp140 * (self.fRec62[((self.IOTA0 - iTemp141) & 2047) as usize] * fTemp142 * fTemp143 * fTemp144 + 0.5 * fTemp132 * self.fRec62[((self.IOTA0 - iTemp145) & 2047) as usize] * fTemp146 * fTemp147 + 0.166666672 * fTemp148 * self.fRec62[((self.IOTA0 - iTemp149) & 2047) as usize] * fTemp150 + 0.0416666679 * fTemp151 * self.fRec62[((self.IOTA0 - iTemp152) & 2047) as usize]);
			self.fVec9[0] = fSlow17;
			let mut fTemp153: F32 = fSlow17 - self.fVec9[1];
			let mut fTemp154: F32 = fSlow16 * fTemp153 * ((((fTemp153 > 0.0) as i32)) as F32);
			let mut fTemp155: F32 = self.fRec55[2] + fTemp154;
			self.fVec10[(self.IOTA0 & 2047) as usize] = fTemp155;
			let mut fTemp156: F32 = fTemp133 * fTemp135 * fTemp137 * fTemp139 * self.fVec10[((self.IOTA0 - iTemp130) & 2047) as usize];
			let mut fTemp157: F32 = fTemp140 * (fTemp142 * fTemp143 * fTemp144 * self.fVec10[((self.IOTA0 - iTemp141) & 2047) as usize] + 0.5 * fTemp132 * fTemp146 * fTemp147 * self.fVec10[((self.IOTA0 - iTemp145) & 2047) as usize] + 0.166666672 * fTemp148 * fTemp150 * self.fVec10[((self.IOTA0 - iTemp149) & 2047) as usize] + 0.0416666679 * fTemp151 * self.fVec10[((self.IOTA0 - iTemp152) & 2047) as usize]);
			let mut fRec64: F32 = fTemp156 + fTemp157;
			let mut fRec65: F32 = fTemp157 + fTemp126 + fTemp156;
			self.fRec58[0] = fRec63;
			let mut fRec59: F32 = fTemp154 + self.fRec58[1];
			let mut fRec60: F32 = fRec64;
			let mut fRec61: F32 = fRec65;
			self.fRec55[0] = fRec59;
			self.fRec56[0] = fRec60;
			let mut fRec57: F32 = fRec61;
			let mut fTemp158: F32 = self.fRec0[(self.IOTA0 & 65535) as usize] + fRec57;
			*output0 = ((fTemp158) as F32);
			*output1 = ((fTemp158) as F32);
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
			self.fRec16[1] = self.fRec16[0];
			self.fRec14[1] = self.fRec14[0];
			self.fRec17[1] = self.fRec17[0];
			self.fRec20[1] = self.fRec20[0];
			self.fRec18[1] = self.fRec18[0];
			self.fRec21[1] = self.fRec21[0];
			self.fRec23[1] = self.fRec23[0];
			self.fRec24[1] = self.fRec24[0];
			self.fRec13[2] = self.fRec13[1];
			self.fRec13[1] = self.fRec13[0];
			self.fRec12[2] = self.fRec12[1];
			self.fRec12[1] = self.fRec12[0];
			self.fRec25[1] = self.fRec25[0];
			self.fRec30[1] = self.fRec30[0];
			self.fRec28[1] = self.fRec28[0];
			self.fRec31[1] = self.fRec31[0];
			self.fRec33[1] = self.fRec33[0];
			self.fRec27[2] = self.fRec27[1];
			self.fRec27[1] = self.fRec27[0];
			self.fRec26[2] = self.fRec26[1];
			self.fRec26[1] = self.fRec26[0];
			self.fRec35[1] = self.fRec35[0];
			self.fRec40[1] = self.fRec40[0];
			self.fRec38[1] = self.fRec38[0];
			self.fRec41[1] = self.fRec41[0];
			self.fRec43[1] = self.fRec43[0];
			self.fRec37[2] = self.fRec37[1];
			self.fRec37[1] = self.fRec37[0];
			self.fRec36[2] = self.fRec36[1];
			self.fRec36[1] = self.fRec36[0];
			self.fRec45[1] = self.fRec45[0];
			self.fRec50[1] = self.fRec50[0];
			self.fRec48[1] = self.fRec48[0];
			self.fRec51[1] = self.fRec51[0];
			self.fRec53[1] = self.fRec53[0];
			self.fRec47[2] = self.fRec47[1];
			self.fRec47[1] = self.fRec47[0];
			self.fRec46[2] = self.fRec46[1];
			self.fRec46[1] = self.fRec46[0];
			self.fRec66[1] = self.fRec66[0];
			self.fVec9[1] = self.fVec9[0];
			self.fRec58[1] = self.fRec58[0];
			self.fRec55[2] = self.fRec55[1];
			self.fRec55[1] = self.fRec55[0];
			self.fRec56[2] = self.fRec56[1];
			self.fRec56[1] = self.fRec56[0];
		}
	}

}


}
pub use dsp::mydsp as Instrument;
