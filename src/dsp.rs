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
	fRec24: [F32;2],
	fVec9: [F32;2],
	fVec10: [F32;4096],
	fHslider7: F32,
	fRec25: [F32;2],
	fConst8: F32,
	fHslider8: F32,
	fRec26: [F32;2],
	fRec13: [F32;3],
	fRec12: [F32;3],
	fRec0: [F32;65536],
	fHslider9: F32,
	fRec34: [F32;2048],
	fConst9: F32,
	fConst10: F32,
	fHslider10: F32,
	fButton0: F32,
	fVec11: [F32;2],
	fVec12: [F32;2048],
	fRec30: [F32;2],
	fRec27: [F32;3],
	fRec28: [F32;3],
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
			fRec24: [0.0;2],
			fVec9: [0.0;2],
			fVec10: [0.0;4096],
			fHslider7: 0.0,
			fRec25: [0.0;2],
			fConst8: 0.0,
			fHslider8: 0.0,
			fRec26: [0.0;2],
			fRec13: [0.0;3],
			fRec12: [0.0;3],
			fRec0: [0.0;65536],
			fHslider9: 0.0,
			fRec34: [0.0;2048],
			fConst9: 0.0,
			fConst10: 0.0,
			fHslider10: 0.0,
			fButton0: 0.0,
			fVec11: [0.0;2],
			fVec12: [0.0;2048],
			fRec30: [0.0;2],
			fRec27: [0.0;3],
			fRec28: [0.0;3],
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
		self.fHslider6 = 0.5;
		self.fHslider7 = 0.0;
		self.fHslider8 = 0.0;
		self.fHslider9 = 0.0;
		self.fHslider10 = 0.800000012;
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
		for l28 in 0..2 {
			self.fVec9[(l28) as usize] = 0.0;
		}
		for l29 in 0..4096 {
			self.fVec10[(l29) as usize] = 0.0;
		}
		for l30 in 0..2 {
			self.fRec25[(l30) as usize] = 0.0;
		}
		for l31 in 0..2 {
			self.fRec26[(l31) as usize] = 0.0;
		}
		for l32 in 0..3 {
			self.fRec13[(l32) as usize] = 0.0;
		}
		for l33 in 0..3 {
			self.fRec12[(l33) as usize] = 0.0;
		}
		for l34 in 0..65536 {
			self.fRec0[(l34) as usize] = 0.0;
		}
		for l35 in 0..2048 {
			self.fRec34[(l35) as usize] = 0.0;
		}
		for l36 in 0..2 {
			self.fVec11[(l36) as usize] = 0.0;
		}
		for l37 in 0..2048 {
			self.fVec12[(l37) as usize] = 0.0;
		}
		for l38 in 0..2 {
			self.fRec30[(l38) as usize] = 0.0;
		}
		for l39 in 0..3 {
			self.fRec27[(l39) as usize] = 0.0;
		}
		for l40 in 0..3 {
			self.fRec28[(l40) as usize] = 0.0;
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
		ui_interface.add_horizontal_slider("volume", ParamIndex(0), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(1)), "1", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(1), 60.0, 0.0, 127.0, 0.001);
		ui_interface.declare(Some(ParamIndex(2)), "2", "");
		ui_interface.add_horizontal_slider("sub", ParamIndex(2), 0.5, 0.0, 1.0, 0.001);
		ui_interface.declare(None, "3", "");
		ui_interface.open_horizontal_box("supersaw");
		ui_interface.declare(Some(ParamIndex(3)), "0", "");
		ui_interface.add_horizontal_slider("volume", ParamIndex(3), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(4)), "1", "");
		ui_interface.add_horizontal_slider("detune", ParamIndex(4), 0.001, 0.001, 0.02, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "4", "");
		ui_interface.open_horizontal_box("filter");
		ui_interface.declare(Some(ParamIndex(5)), "0", "");
		ui_interface.add_horizontal_slider("cutoff_note", ParamIndex(5), 0.0, -20.0, 50.0, 0.001);
		ui_interface.declare(Some(ParamIndex(6)), "1", "");
		ui_interface.add_horizontal_slider("res", ParamIndex(6), 0.0, 0.0, 0.98999999999999999, 0.001);
		ui_interface.close_box();
		ui_interface.close_box();
		ui_interface.declare(None, "1", "");
		ui_interface.open_horizontal_box("pluck");
		ui_interface.declare(Some(ParamIndex(7)), "0", "");
		ui_interface.add_button("gate", ParamIndex(7));
		ui_interface.declare(Some(ParamIndex(8)), "1", "");
		ui_interface.add_horizontal_slider("gain", ParamIndex(8), 0.80000000000000004, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(9)), "2", "");
		ui_interface.add_horizontal_slider("damping", ParamIndex(9), 0.0, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "2", "");
		ui_interface.open_horizontal_box("drone");
		ui_interface.declare(Some(ParamIndex(10)), "0", "");
		ui_interface.add_horizontal_slider("volume", ParamIndex(10), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(11)), "1", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(11), 60.0, 0.0, 127.0, 0.001);
		ui_interface.close_box();
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			7 => Some(self.fButton0),
			10 => Some(self.fHslider0),
			11 => Some(self.fHslider1),
			8 => Some(self.fHslider10),
			0 => Some(self.fHslider2),
			1 => Some(self.fHslider3),
			3 => Some(self.fHslider4),
			4 => Some(self.fHslider5),
			2 => Some(self.fHslider6),
			6 => Some(self.fHslider7),
			5 => Some(self.fHslider8),
			9 => Some(self.fHslider9),
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			7 => { self.fButton0 = value }
			10 => { self.fHslider0 = value }
			11 => { self.fHslider1 = value }
			8 => { self.fHslider10 = value }
			0 => { self.fHslider2 = value }
			1 => { self.fHslider3 = value }
			3 => { self.fHslider4 = value }
			4 => { self.fHslider5 = value }
			2 => { self.fHslider6 = value }
			6 => { self.fHslider7 = value }
			5 => { self.fHslider8 = value }
			9 => { self.fHslider9 = value }
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
		let mut fSlow9: F32 = 0.5 * (0.200000003 * (1.0 - ((self.fHslider9) as F32)) + 0.800000012);
		let mut fSlow10: F32 = ((self.fHslider10) as F32);
		let mut fSlow11: F32 = ((self.fButton0) as F32);
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
			let mut fTemp30: F32 = F32::max(0.0, F32::min(2047.0, self.fConst6 / fTemp29));
			let mut fTemp31: F32 = F32::floor(fTemp30);
			let mut fTemp32: F32 = F32::max(20.0, F32::abs(fTemp29));
			let mut fTemp33: F32 = self.fRec10[1] + self.fConst5 * fTemp32;
			self.fRec10[0] = fTemp33 - F32::floor(fTemp33);
			let mut fTemp34: F32 = mydsp_faustpower2_f(2.0 * self.fRec10[0] + -1.0);
			self.fVec7[0] = fTemp34;
			let mut fTemp35: F32 = (fTemp0 * (fTemp34 - self.fVec7[1])) / fTemp32;
			self.fVec8[(self.IOTA0 & 4095) as usize] = fTemp35;
			let mut iTemp36: i32 = ((fTemp30) as i32);
			self.fRec9[0] = 0.999000013 * self.fRec9[1] - self.fConst4 * ((fTemp30 - fTemp31) * self.fVec8[((self.IOTA0 - (iTemp36 + 1)) & 4095) as usize] - fTemp35 - self.fVec8[((self.IOTA0 - iTemp36) & 4095) as usize] * (fTemp31 + 1.0 - fTemp30));
			self.fRec11[0] = fSlow2 + self.fConst3 * self.fRec11[1];
			self.fRec16[0] = fSlow3 + self.fConst3 * self.fRec16[1];
			let mut fTemp37: F32 = F32::powf(2.0, 0.0833333358 * (self.fRec16[0] + -69.0));
			let mut fTemp38: F32 = F32::max(1.1920929e-07, F32::abs(440.0 * fTemp37));
			let mut fTemp39: F32 = self.fRec14[1] + self.fConst5 * fTemp38;
			let mut fTemp40: F32 = fTemp39 + -1.0;
			let mut iTemp41: i32 = ((fTemp40 < 0.0) as i32);
			self.fRec14[0] = if (iTemp41 as i32 != 0) { fTemp39 } else { fTemp40 };
			let mut fThen1: F32 = fTemp39 + fTemp40 * (1.0 - self.fConst0 / fTemp38);
			let mut fRec15: F32 = if (iTemp41 as i32 != 0) { fTemp39 } else { fThen1 };
			self.fRec17[0] = fSlow4 + self.fConst3 * self.fRec17[1];
			self.fRec20[0] = fSlow5 + self.fConst3 * self.fRec20[1];
			let mut fTemp42: F32 = F32::max(1.1920929e-07, F32::abs(440.0 * fTemp37 * (self.fRec20[0] + 1.0)));
			let mut fTemp43: F32 = self.fRec18[1] + self.fConst5 * fTemp42;
			let mut fTemp44: F32 = fTemp43 + -1.0;
			let mut iTemp45: i32 = ((fTemp44 < 0.0) as i32);
			self.fRec18[0] = if (iTemp45 as i32 != 0) { fTemp43 } else { fTemp44 };
			let mut fThen3: F32 = fTemp43 + fTemp44 * (1.0 - self.fConst0 / fTemp42);
			let mut fRec19: F32 = if (iTemp45 as i32 != 0) { fTemp43 } else { fThen3 };
			let mut fTemp46: F32 = F32::max(1.1920929e-07, F32::abs(440.0 * fTemp37 * (1.0 - self.fRec20[0])));
			let mut fTemp47: F32 = self.fRec21[1] + self.fConst5 * fTemp46;
			let mut fTemp48: F32 = fTemp47 + -1.0;
			let mut iTemp49: i32 = ((fTemp48 < 0.0) as i32);
			self.fRec21[0] = if (iTemp49 as i32 != 0) { fTemp47 } else { fTemp48 };
			let mut fThen5: F32 = fTemp47 + fTemp48 * (1.0 - self.fConst0 / fTemp46);
			let mut fRec22: F32 = if (iTemp49 as i32 != 0) { fTemp47 } else { fThen5 };
			self.fRec23[0] = fSlow6 + self.fConst3 * self.fRec23[1];
			let mut fTemp50: F32 = F32::max(440.0 * F32::powf(2.0, 0.0833333358 * (self.fRec16[0] + -81.0)), 23.4489498);
			let mut fTemp51: F32 = F32::max(20.0, F32::abs(fTemp50));
			let mut fTemp52: F32 = self.fRec24[1] + self.fConst5 * fTemp51;
			self.fRec24[0] = fTemp52 - F32::floor(fTemp52);
			let mut fTemp53: F32 = mydsp_faustpower2_f(2.0 * self.fRec24[0] + -1.0);
			self.fVec9[0] = fTemp53;
			let mut fTemp54: F32 = (fTemp0 * (fTemp53 - self.fVec9[1])) / fTemp51;
			self.fVec10[(self.IOTA0 & 4095) as usize] = fTemp54;
			let mut fTemp55: F32 = F32::max(0.0, F32::min(2047.0, self.fConst6 / fTemp50));
			let mut iTemp56: i32 = ((fTemp55) as i32);
			let mut fTemp57: F32 = F32::floor(fTemp55);
			self.fRec25[0] = fSlow7 + self.fConst3 * self.fRec25[1];
			let mut fTemp58: F32 = F32::min(1.41419947, 1.41421354 * self.fRec25[0]);
			let mut fTemp59: F32 = fTemp58 * (fTemp58 + 1.41421354);
			self.fRec26[0] = fSlow8 + self.fConst3 * self.fRec26[1];
			let mut fTemp60: F32 = F32::tan(self.fConst8 * F32::max(20.0, F32::min(10000.0, 440.0 * F32::powf(2.0, 0.0833333358 * (self.fRec16[0] + self.fRec26[0] + -69.0)))));
			let mut fTemp61: F32 = 1.0 / fTemp60;
			let mut fTemp62: F32 = 1.41421354 * fTemp58;
			let mut fTemp63: F32 = fTemp62 + 2.0;
			let mut fTemp64: F32 = 1.0 - 1.0 / mydsp_faustpower2_f(fTemp60);
			let mut fTemp65: F32 = fTemp59 + (fTemp61 + fTemp63) / fTemp60 + 1.0;
			self.fRec13[0] = 2.0 * fRec15 + self.fRec17[0] * (0.0 - 2.0 * (1.0 - (fRec19 + fRec22))) + self.fRec23[0] * (0.0 - self.fConst4 * (self.fVec10[((self.IOTA0 - iTemp56) & 4095) as usize] * (fTemp57 + 1.0 - fTemp55) - fTemp54 + (fTemp55 - fTemp57) * self.fVec10[((self.IOTA0 - (iTemp56 + 1)) & 4095) as usize])) + -1.0 - (self.fRec13[2] * (fTemp59 + (fTemp61 - fTemp63) / fTemp60 + 1.0) + 2.0 * self.fRec13[1] * (fTemp59 + fTemp64)) / fTemp65;
			let mut fTemp66: F32 = fTemp58 * (fTemp58 + -1.41421354);
			let mut fTemp67: F32 = 2.0 - fTemp62;
			let mut fTemp68: F32 = fTemp66 + (fTemp67 + fTemp61) / fTemp60 + 1.0;
			self.fRec12[0] = (self.fRec13[2] + self.fRec13[0] + 2.0 * self.fRec13[1]) / fTemp65 - (self.fRec12[2] * (fTemp66 + (fTemp61 - fTemp67) / fTemp60 + 1.0) + 2.0 * self.fRec12[1] * (fTemp66 + fTemp64)) / fTemp68;
			self.fRec0[(self.IOTA0 & 65535) as usize] = self.fConst1 * self.fRec1[0] * (self.fRec2[0] * fTemp1 + self.fRec5[0] * fTemp10 + self.fRec7[0] * fTemp19 + self.fRec9[0] * fTemp28) + 0.300000012 * self.fRec0[((self.IOTA0 - self.iConst7) & 65535) as usize] + (self.fRec11[0] * (self.fRec12[2] + self.fRec12[0] + 2.0 * self.fRec12[1])) / fTemp68;
			let mut fTemp69: F32 = fSlow9 * (self.fRec28[1] + self.fRec28[2]);
			self.fRec34[(self.IOTA0 & 2047) as usize] = fTemp69;
			let mut fTemp70: F32 = self.fConst10 * (0.772727251 / fTemp37 + -0.0500000007);
			let mut fTemp71: F32 = fTemp70 + -1.49999499;
			let mut iTemp72: i32 = ((fTemp71) as i32);
			let mut iTemp73: i32 = ((F32::min(self.fConst9, ((std::cmp::max(0, iTemp72)) as F32))) as i32) + 1;
			let mut fTemp74: F32 = F32::floor(fTemp71);
			let mut fTemp75: F32 = fTemp70 + -1.0 - fTemp74;
			let mut fTemp76: F32 = 0.0 - fTemp75;
			let mut fTemp77: F32 = fTemp70 + -2.0 - fTemp74;
			let mut fTemp78: F32 = 0.0 - 0.5 * fTemp77;
			let mut fTemp79: F32 = fTemp70 + -3.0 - fTemp74;
			let mut fTemp80: F32 = 0.0 - 0.333333343 * fTemp79;
			let mut fTemp81: F32 = fTemp70 + -4.0 - fTemp74;
			let mut fTemp82: F32 = 0.0 - 0.25 * fTemp81;
			let mut fTemp83: F32 = fTemp70 - fTemp74;
			let mut iTemp84: i32 = ((F32::min(self.fConst9, ((std::cmp::max(0, iTemp72 + 1)) as F32))) as i32) + 1;
			let mut fTemp85: F32 = 0.0 - fTemp77;
			let mut fTemp86: F32 = 0.0 - 0.5 * fTemp79;
			let mut fTemp87: F32 = 0.0 - 0.333333343 * fTemp81;
			let mut iTemp88: i32 = ((F32::min(self.fConst9, ((std::cmp::max(0, iTemp72 + 2)) as F32))) as i32) + 1;
			let mut fTemp89: F32 = 0.0 - fTemp79;
			let mut fTemp90: F32 = 0.0 - 0.5 * fTemp81;
			let mut fTemp91: F32 = fTemp75 * fTemp77;
			let mut iTemp92: i32 = ((F32::min(self.fConst9, ((std::cmp::max(0, iTemp72 + 3)) as F32))) as i32) + 1;
			let mut fTemp93: F32 = 0.0 - fTemp81;
			let mut fTemp94: F32 = fTemp91 * fTemp79;
			let mut iTemp95: i32 = ((F32::min(self.fConst9, ((std::cmp::max(0, iTemp72 + 4)) as F32))) as i32) + 1;
			let mut fRec35: F32 = self.fRec34[((self.IOTA0 - iTemp73) & 2047) as usize] * fTemp76 * fTemp78 * fTemp80 * fTemp82 + fTemp83 * (self.fRec34[((self.IOTA0 - iTemp84) & 2047) as usize] * fTemp85 * fTemp86 * fTemp87 + 0.5 * fTemp75 * self.fRec34[((self.IOTA0 - iTemp88) & 2047) as usize] * fTemp89 * fTemp90 + 0.166666672 * fTemp91 * self.fRec34[((self.IOTA0 - iTemp92) & 2047) as usize] * fTemp93 + 0.0416666679 * fTemp94 * self.fRec34[((self.IOTA0 - iTemp95) & 2047) as usize]);
			self.fVec11[0] = fSlow11;
			let mut fTemp96: F32 = fSlow11 - self.fVec11[1];
			let mut fTemp97: F32 = fSlow10 * fTemp96 * ((((fTemp96 > 0.0) as i32)) as F32);
			let mut fTemp98: F32 = self.fRec27[2] + fTemp97;
			self.fVec12[(self.IOTA0 & 2047) as usize] = fTemp98;
			let mut fTemp99: F32 = fTemp76 * fTemp78 * fTemp80 * fTemp82 * self.fVec12[((self.IOTA0 - iTemp73) & 2047) as usize];
			let mut fTemp100: F32 = fTemp83 * (fTemp85 * fTemp86 * fTemp87 * self.fVec12[((self.IOTA0 - iTemp84) & 2047) as usize] + 0.5 * fTemp75 * fTemp89 * fTemp90 * self.fVec12[((self.IOTA0 - iTemp88) & 2047) as usize] + 0.166666672 * fTemp91 * fTemp93 * self.fVec12[((self.IOTA0 - iTemp92) & 2047) as usize] + 0.0416666679 * fTemp94 * self.fVec12[((self.IOTA0 - iTemp95) & 2047) as usize]);
			let mut fRec36: F32 = fTemp99 + fTemp100;
			let mut fRec37: F32 = fTemp99 + fTemp69 + fTemp100;
			self.fRec30[0] = fRec35;
			let mut fRec31: F32 = fTemp97 + self.fRec30[1];
			let mut fRec32: F32 = fRec36;
			let mut fRec33: F32 = fRec37;
			self.fRec27[0] = fRec31;
			self.fRec28[0] = fRec32;
			let mut fRec29: F32 = fRec33;
			let mut fTemp101: F32 = self.fRec0[(self.IOTA0 & 65535) as usize] + fRec29;
			*output0 = ((fTemp101) as F32);
			*output1 = ((fTemp101) as F32);
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
			self.fVec9[1] = self.fVec9[0];
			self.fRec25[1] = self.fRec25[0];
			self.fRec26[1] = self.fRec26[0];
			self.fRec13[2] = self.fRec13[1];
			self.fRec13[1] = self.fRec13[0];
			self.fRec12[2] = self.fRec12[1];
			self.fRec12[1] = self.fRec12[0];
			self.fVec11[1] = self.fVec11[0];
			self.fRec30[1] = self.fRec30[0];
			self.fRec27[2] = self.fRec27[1];
			self.fRec27[1] = self.fRec27[0];
			self.fRec28[2] = self.fRec28[1];
			self.fRec28[1] = self.fRec28[0];
		}
	}

}


}
pub use dsp::mydsp as Instrument;
