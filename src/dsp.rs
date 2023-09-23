mod dsp {
    /* ------------------------------------------------------------
author: "Pierre Lulé"
license: "BSD"
name: "theremotion"
version: "1.0"
Code generated with Faust 2.68.1 (https://faust.grame.fr)
Compilation options: -a C:\Users\Pierre\AppData\Local\Temp\.tmpLyRcxs -lang rust -ct 1 -es 1 -mcd 16 -single -ftz 0 -vec -lv 0 -vs 32
------------------------------------------------------------ */
#![allow(clippy::all)]
#![allow(unused_parens)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(non_upper_case_globals)]

use faust_types::*;


static mut imydspSIG0Wave0: [i32;1302] = [2,3,5,7,11,13,17,19,23,29,31,37,41,43,47,53,59,61,67,71,73,79,83,89,97,101,103,107,109,113,127,131,137,139,149,151,157,163,167,173,179,181,191,193,197,199,211,223,227,229,233,239,241,251,257,263,269,271,277,281,283,293,307,311,313,317,331,337,347,349,353,359,367,373,379,383,389,397,401,409,419,421,431,433,439,443,449,457,461,463,467,479,487,491,499,503,509,521,523,541,547,557,563,569,571,577,587,593,599,601,607,613,617,619,631,641,643,647,653,659,661,673,677,683,691,701,709,719,727,733,739,743,751,757,761,769,773,787,797,809,811,821,823,827,829,839,853,857,859,863,877,881,883,887,907,911,919,929,937,941,947,953,967,971,977,983,991,997,1009,1013,1019,1021,1031,1033,1039,1049,1051,1061,1063,1069,1087,1091,1093,1097,1103,1109,1117,1123,1129,1151,1153,1163,1171,1181,1187,1193,1201,1213,1217,1223,1229,1231,1237,1249,1259,1277,1279,1283,1289,1291,1297,1301,1303,1307,1319,1321,1327,1361,1367,1373,1381,1399,1409,1423,1427,1429,1433,1439,1447,1451,1453,1459,1471,1481,1483,1487,1489,1493,1499,1511,1523,1531,1543,1549,1553,1559,1567,1571,1579,1583,1597,1601,1607,1609,1613,1619,1621,1627,1637,1657,1663,1667,1669,1693,1697,1699,1709,1721,1723,1733,1741,1747,1753,1759,1777,1783,1787,1789,1801,1811,1823,1831,1847,1861,1867,1871,1873,1877,1879,1889,1901,1907,1913,1931,1933,1949,1951,1973,1979,1987,1993,1997,1999,2003,2011,2017,2027,2029,2039,2053,2063,2069,2081,2083,2087,2089,2099,2111,2113,2129,2131,2137,2141,2143,2153,2161,2179,2203,2207,2213,2221,2237,2239,2243,2251,2267,2269,2273,2281,2287,2293,2297,2309,2311,2333,2339,2341,2347,2351,2357,2371,2377,2381,2383,2389,2393,2399,2411,2417,2423,2437,2441,2447,2459,2467,2473,2477,2503,2521,2531,2539,2543,2549,2551,2557,2579,2591,2593,2609,2617,2621,2633,2647,2657,2659,2663,2671,2677,2683,2687,2689,2693,2699,2707,2711,2713,2719,2729,2731,2741,2749,2753,2767,2777,2789,2791,2797,2801,2803,2819,2833,2837,2843,2851,2857,2861,2879,2887,2897,2903,2909,2917,2927,2939,2953,2957,2963,2969,2971,2999,3001,3011,3019,3023,3037,3041,3049,3061,3067,3079,3083,3089,3109,3119,3121,3137,3163,3167,3169,3181,3187,3191,3203,3209,3217,3221,3229,3251,3253,3257,3259,3271,3299,3301,3307,3313,3319,3323,3329,3331,3343,3347,3359,3361,3371,3373,3389,3391,3407,3413,3433,3449,3457,3461,3463,3467,3469,3491,3499,3511,3517,3527,3529,3533,3539,3541,3547,3557,3559,3571,3581,3583,3593,3607,3613,3617,3623,3631,3637,3643,3659,3671,3673,3677,3691,3697,3701,3709,3719,3727,3733,3739,3761,3767,3769,3779,3793,3797,3803,3821,3823,3833,3847,3851,3853,3863,3877,3881,3889,3907,3911,3917,3919,3923,3929,3931,3943,3947,3967,3989,4001,4003,4007,4013,4019,4021,4027,4049,4051,4057,4073,4079,4091,4093,4099,4111,4127,4129,4133,4139,4153,4157,4159,4177,4201,4211,4217,4219,4229,4231,4241,4243,4253,4259,4261,4271,4273,4283,4289,4297,4327,4337,4339,4349,4357,4363,4373,4391,4397,4409,4421,4423,4441,4447,4451,4457,4463,4481,4483,4493,4507,4513,4517,4519,4523,4547,4549,4561,4567,4583,4591,4597,4603,4621,4637,4639,4643,4649,4651,4657,4663,4673,4679,4691,4703,4721,4723,4729,4733,4751,4759,4783,4787,4789,4793,4799,4801,4813,4817,4831,4861,4871,4877,4889,4903,4909,4919,4931,4933,4937,4943,4951,4957,4967,4969,4973,4987,4993,4999,5003,5009,5011,5021,5023,5039,5051,5059,5077,5081,5087,5099,5101,5107,5113,5119,5147,5153,5167,5171,5179,5189,5197,5209,5227,5231,5233,5237,5261,5273,5279,5281,5297,5303,5309,5323,5333,5347,5351,5381,5387,5393,5399,5407,5413,5417,5419,5431,5437,5441,5443,5449,5471,5477,5479,5483,5501,5503,5507,5519,5521,5527,5531,5557,5563,5569,5573,5581,5591,5623,5639,5641,5647,5651,5653,5657,5659,5669,5683,5689,5693,5701,5711,5717,5737,5741,5743,5749,5779,5783,5791,5801,5807,5813,5821,5827,5839,5843,5849,5851,5857,5861,5867,5869,5879,5881,5897,5903,5923,5927,5939,5953,5981,5987,6007,6011,6029,6037,6043,6047,6053,6067,6073,6079,6089,6091,6101,6113,6121,6131,6133,6143,6151,6163,6173,6197,6199,6203,6211,6217,6221,6229,6247,6257,6263,6269,6271,6277,6287,6299,6301,6311,6317,6323,6329,6337,6343,6353,6359,6361,6367,6373,6379,6389,6397,6421,6427,6449,6451,6469,6473,6481,6491,6521,6529,6547,6551,6553,6563,6569,6571,6577,6581,6599,6607,6619,6637,6653,6659,6661,6673,6679,6689,6691,6701,6703,6709,6719,6733,6737,6761,6763,6779,6781,6791,6793,6803,6823,6827,6829,6833,6841,6857,6863,6869,6871,6883,6899,6907,6911,6917,6947,6949,6959,6961,6967,6971,6977,6983,6991,6997,7001,7013,7019,7027,7039,7043,7057,7069,7079,7103,7109,7121,7127,7129,7151,7159,7177,7187,7193,7207,7211,7213,7219,7229,7237,7243,7247,7253,7283,7297,7307,7309,7321,7331,7333,7349,7351,7369,7393,7411,7417,7433,7451,7457,7459,7477,7481,7487,7489,7499,7507,7517,7523,7529,7537,7541,7547,7549,7559,7561,7573,7577,7583,7589,7591,7603,7607,7621,7639,7643,7649,7669,7673,7681,7687,7691,7699,7703,7717,7723,7727,7741,7753,7757,7759,7789,7793,7817,7823,7829,7841,7853,7867,7873,7877,7879,7883,7901,7907,7919,7927,7933,7937,7949,7951,7963,7993,8009,8011,8017,8039,8053,8059,8069,8081,8087,8089,8093,8101,8111,8117,8123,8147,8161,8167,8171,8179,8191,8209,8219,8221,8231,8233,8237,8243,8263,8269,8273,8287,8291,8293,8297,8311,8317,8329,8353,8363,8369,8377,8387,8389,8419,8423,8429,8431,8443,8447,8461,8467,8501,8513,8521,8527,8537,8539,8543,8563,8573,8581,8597,8599,8609,8623,8627,8629,8641,8647,8663,8669,8677,8681,8689,8693,8699,8707,8713,8719,8731,8737,8741,8747,8753,8761,8779,8783,8803,8807,8819,8821,8831,8837,8839,8849,8861,8863,8867,8887,8893,8923,8929,8933,8941,8951,8963,8969,8971,8999,9001,9007,9011,9013,9029,9041,9043,9049,9059,9067,9091,9103,9109,9127,9133,9137,9151,9157,9161,9173,9181,9187,9199,9203,9209,9221,9227,9239,9241,9257,9277,9281,9283,9293,9311,9319,9323,9337,9341,9343,9349,9371,9377,9391,9397,9403,9413,9419,9421,9431,9433,9437,9439,9461,9463,9467,9473,9479,9491,9497,9511,9521,9533,9539,9547,9551,9587,9601,9613,9619,9623,9629,9631,9643,9649,9661,9677,9679,9689,9697,9719,9721,9733,9739,9743,9749,9767,9769,9781,9787,9791,9803,9811,9817,9829,9833,9839,9851,9857,9859,9871,9883,9887,9901,9907,9923,9929,9931,9941,9949,9967,9973,10007,10009,10037,10039,10061,10067,10069,10079,10091,10093,10099,10103,10111,10133,10139,10141,10151,10159,10163,10169,10177,10181,10193,10211,10223,10243,10247,10253,10259,10267,10271,10273,10289,10301,10303,10313,10321,10331,10333,10337,10343,10357,10369,10391,10399,10427,10429,10433,10453,10457,10459,10463,10477,10487,10499,10501,10513,10529,10531,10559,10567,10589,10597,10601,10607,10613,10627,10631,10639,10651,10657,10663,10667];

#[derive(Debug,Clone)]
pub struct mydspSIG0 {
	imydspSIG0Wave0_idx: i32,
}

impl mydspSIG0 {
	
	fn get_num_inputsmydspSIG0(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsmydspSIG0(&self) -> i32 {
		return 1;
	}
	
	fn instance_initmydspSIG0(&mut self, sample_rate: i32) {
		self.imydspSIG0Wave0_idx = 0;
	}
	
	fn fillmydspSIG0(&mut self, count: i32, table: &mut[i32]) {
		for i1 in 0..count {
			table[i1 as usize] = unsafe { imydspSIG0Wave0[self.imydspSIG0Wave0_idx as usize] };
			self.imydspSIG0Wave0_idx = (i32::wrapping_add(1, self.imydspSIG0Wave0_idx)) % 1302;
		}
	}

}


pub fn newmydspSIG0() -> mydspSIG0 { 
	mydspSIG0 {
		imydspSIG0Wave0_idx: 0,
	}
}

#[derive(Debug,Clone)]
pub struct mydspSIG1 {
	iRec26: [i32;2],
}

impl mydspSIG1 {
	
	fn get_num_inputsmydspSIG1(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsmydspSIG1(&self) -> i32 {
		return 1;
	}
	
	fn instance_initmydspSIG1(&mut self, sample_rate: i32) {
		for l0 in 0..2 {
			self.iRec26[l0 as usize] = 0;
		}
	}
	
	fn fillmydspSIG1(&mut self, count: i32, table: &mut[F32]) {
		for i2 in 0..count {
			self.iRec26[0] = i32::wrapping_add(self.iRec26[1], 1);
			table[i2 as usize] = 4.4e+02 * F32::powf(2.0, 0.083333336 * (0.062042013 * (i32::wrapping_add(self.iRec26[0], -1)) as F32 + -69.0));
			self.iRec26[1] = self.iRec26[0];
		}
	}

}


pub fn newmydspSIG1() -> mydspSIG1 { 
	mydspSIG1 {
		iRec26: [0;2],
	}
}
static mut itbl0mydspSIG0: [i32;1302] = [0;1302];
static mut ftbl1mydspSIG1: [F32;2048] = [0.0;2048];
fn mydsp_faustpower2_f(value: F32) -> F32 {
	return value * value;
}

#[cfg_attr(feature = "default-boxed", derive(default_boxed::DefaultBoxed))]
#[repr(C)]
#[derive(Debug,Clone)]
pub struct mydsp {
	fSampleRate: i32,
	fConst0: F32,
	fConst1: F32,
	fConst2: F32,
	fHslider0: F32,
	fRec0_perm: [F32;4],
	fHslider1: F32,
	iVec0_perm: [i32;4],
	fRec9_perm: [F32;4],
	fRec13_perm: [F32;4],
	fRec17_perm: [F32;4],
	fHslider2: F32,
	fConst3: F32,
	fRec19_perm: [F32;4],
	fRec20_perm: [F32;4],
	fHslider3: F32,
	fRec21_perm: [F32;4],
	fHslider4: F32,
	fRec22_perm: [F32;4],
	fHslider5: F32,
	fRec23_perm: [F32;4],
	fHslider6: F32,
	fRec24_perm: [F32;4],
	fHslider7: F32,
	fRec25_perm: [F32;4],
	fConst4: F32,
	fRec28_perm: [F32;4],
	fConst5: F32,
	fYec0_perm: [F32;4],
	fYec1: [F32;4096],
	fYec1_idx: i32,
	fYec1_idx_save: i32,
	fConst6: F32,
	fRec27_perm: [F32;4],
	fHslider8: F32,
	fRec29_perm: [F32;4],
	fRec31_perm: [F32;4],
	fYec2_perm: [F32;4],
	fYec3: [F32;4096],
	fYec3_idx: i32,
	fYec3_idx_save: i32,
	fRec30_perm: [F32;4],
	fRec32_perm: [F32;4],
	fRec34_perm: [F32;4],
	fYec4_perm: [F32;4],
	fYec5: [F32;4096],
	fYec5_idx: i32,
	fYec5_idx_save: i32,
	fRec33_perm: [F32;4],
	fHslider9: F32,
	fRec35_perm: [F32;4],
	fHslider10: F32,
	fRec36_perm: [F32;4],
	fRec38_perm: [F32;4],
	fYec6_perm: [F32;4],
	fYec7: [F32;4096],
	fYec7_idx: i32,
	fYec7_idx_save: i32,
	fRec37_perm: [F32;4],
	fRec39_perm: [F32;4],
	fRec41_perm: [F32;4],
	fYec8_perm: [F32;4],
	fYec9: [F32;4096],
	fYec9_idx: i32,
	fYec9_idx_save: i32,
	fRec40_perm: [F32;4],
	fRec42_perm: [F32;4],
	fRec44_perm: [F32;4],
	fYec10_perm: [F32;4],
	fYec11: [F32;4096],
	fYec11_idx: i32,
	fYec11_idx_save: i32,
	fRec43_perm: [F32;4],
	fHslider11: F32,
	fRec45_perm: [F32;4],
	fHslider12: F32,
	fRec46_perm: [F32;4],
	fRec48_perm: [F32;4],
	fYec12_perm: [F32;4],
	fYec13: [F32;4096],
	fYec13_idx: i32,
	fYec13_idx_save: i32,
	fRec47_perm: [F32;4],
	fRec49_perm: [F32;4],
	fRec51_perm: [F32;4],
	fYec14_perm: [F32;4],
	fYec15: [F32;4096],
	fYec15_idx: i32,
	fYec15_idx_save: i32,
	fRec50_perm: [F32;4],
	fRec52_perm: [F32;4],
	fRec54_perm: [F32;4],
	fYec16_perm: [F32;4],
	fYec17: [F32;4096],
	fYec17_idx: i32,
	fYec17_idx_save: i32,
	fRec53_perm: [F32;4],
	fHslider13: F32,
	fRec55_perm: [F32;4],
	fHslider14: F32,
	fRec56_perm: [F32;4],
	fRec58_perm: [F32;4],
	fYec18_perm: [F32;4],
	fYec19: [F32;4096],
	fYec19_idx: i32,
	fYec19_idx_save: i32,
	fRec57_perm: [F32;4],
	fRec59_perm: [F32;4],
	fRec61_perm: [F32;4],
	fYec20_perm: [F32;4],
	fYec21: [F32;4096],
	fYec21_idx: i32,
	fYec21_idx_save: i32,
	fRec60_perm: [F32;4],
	fRec62_perm: [F32;4],
	fRec64_perm: [F32;4],
	fYec22_perm: [F32;4],
	fYec23: [F32;4096],
	fYec23_idx: i32,
	fYec23_idx_save: i32,
	fRec63_perm: [F32;4],
	fHslider15: F32,
	fRec65_perm: [F32;4],
	fHslider16: F32,
	fRec66_perm: [F32;4],
	fHslider17: F32,
	fRec69_perm: [F32;4],
	fHslider18: F32,
	fRec73_perm: [F32;4],
	fHslider19: F32,
	fRec72_perm: [F32;4],
	fRec70_perm: [F32;4],
	fHslider20: F32,
	fRec74_perm: [F32;4],
	fHslider21: F32,
	fRec76_perm: [F32;4],
	fRec75_perm: [F32;4],
	fConst7: F32,
	fRec68_perm: [F32;4],
	fRec67_perm: [F32;4],
	fHslider22: F32,
	fRec79_perm: [F32;4],
	fHslider23: F32,
	fRec82_perm: [F32;4],
	fRec80_perm: [F32;4],
	fRec83_perm: [F32;4],
	fRec78_perm: [F32;4],
	fRec77_perm: [F32;4],
	fHslider24: F32,
	fRec86_perm: [F32;4],
	fHslider25: F32,
	fRec89_perm: [F32;4],
	fRec87_perm: [F32;4],
	fRec90_perm: [F32;4],
	fRec85_perm: [F32;4],
	fRec84_perm: [F32;4],
	fHslider26: F32,
	fRec93_perm: [F32;4],
	fHslider27: F32,
	fRec96_perm: [F32;4],
	fRec94_perm: [F32;4],
	fRec97_perm: [F32;4],
	fRec92_perm: [F32;4],
	fRec91_perm: [F32;4],
	fHslider28: F32,
	fRec98_perm: [F32;4],
	fHslider29: F32,
	fRec131_perm: [F32;4],
	fConst8: F32,
	fConst9: F32,
	fRec127_perm: [F32;4],
	fRec132_perm: [F32;4],
	iRec135_perm: [i32;4],
	fConst10: F32,
	fRec134_perm: [F32;4],
	fButton0: F32,
	fVec1_perm: [F32;4],
	iRec136_perm: [i32;4],
	fRec137_perm: [F32;4],
	fRec138: [F32;2048],
	fRec138_idx: i32,
	fRec138_idx_save: i32,
	fConst11: F32,
	fConst12: F32,
	fRec140_perm: [F32;4],
	iYec24_perm: [i32;4],
	iConst13: i32,
	iRec141_perm: [i32;4],
	fConst14: F32,
	fRec139_perm: [F32;4],
	fYec25_perm: [F32;4],
	fConst15: F32,
	fYec26_perm: [F32;4],
	fRec133: [F32;2048],
	fRec133_idx: i32,
	fRec133_idx_save: i32,
	fRec123_perm: [F32;4],
	fRec119_perm: [F32;4],
	fRec115: [F32;2048],
	fRec115_idx: i32,
	fRec115_idx_save: i32,
	fRec117_perm: [F32;4],
	fHslider30: F32,
	fRec113_perm: [F32;4],
	fRec108_perm: [F32;4],
	fRec104: [F32;2048],
	fRec104_idx: i32,
	fRec104_idx_save: i32,
	fRec102_perm: [F32;4],
	fConst16: F32,
	fRec143_perm: [F32;4],
	fRec101_perm: [F32;4],
	fRec144_perm: [F32;4],
	fRec100_perm: [F32;4],
	fRec99_perm: [F32;4],
	fHslider31: F32,
	fRec177_perm: [F32;4],
	fRec173_perm: [F32;4],
	fRec178_perm: [F32;4],
	fRec180_perm: [F32;4],
	fButton1: F32,
	fVec2_perm: [F32;4],
	iRec181_perm: [i32;4],
	fRec182_perm: [F32;4],
	fRec183: [F32;2048],
	fRec183_idx: i32,
	fRec183_idx_save: i32,
	fRec185_perm: [F32;4],
	iYec27_perm: [i32;4],
	iRec186_perm: [i32;4],
	fRec184_perm: [F32;4],
	fYec28_perm: [F32;4],
	fYec29_perm: [F32;4],
	fRec179: [F32;2048],
	fRec179_idx: i32,
	fRec179_idx_save: i32,
	fRec169_perm: [F32;4],
	fRec165_perm: [F32;4],
	fRec161: [F32;2048],
	fRec161_idx: i32,
	fRec161_idx_save: i32,
	fRec163_perm: [F32;4],
	fRec159_perm: [F32;4],
	fRec154_perm: [F32;4],
	fRec150: [F32;2048],
	fRec150_idx: i32,
	fRec150_idx_save: i32,
	fRec148_perm: [F32;4],
	fRec188_perm: [F32;4],
	fRec147_perm: [F32;4],
	fRec189_perm: [F32;4],
	fRec146_perm: [F32;4],
	fRec145_perm: [F32;4],
	fHslider32: F32,
	fRec222_perm: [F32;4],
	fRec218_perm: [F32;4],
	fRec223_perm: [F32;4],
	fRec225_perm: [F32;4],
	fButton2: F32,
	fVec3_perm: [F32;4],
	iRec226_perm: [i32;4],
	fRec227_perm: [F32;4],
	fRec228: [F32;2048],
	fRec228_idx: i32,
	fRec228_idx_save: i32,
	fRec230_perm: [F32;4],
	iYec30_perm: [i32;4],
	iRec231_perm: [i32;4],
	fRec229_perm: [F32;4],
	fYec31_perm: [F32;4],
	fYec32_perm: [F32;4],
	fRec224: [F32;2048],
	fRec224_idx: i32,
	fRec224_idx_save: i32,
	fRec214_perm: [F32;4],
	fRec210_perm: [F32;4],
	fRec206: [F32;2048],
	fRec206_idx: i32,
	fRec206_idx_save: i32,
	fRec208_perm: [F32;4],
	fRec204_perm: [F32;4],
	fRec199_perm: [F32;4],
	fRec195: [F32;2048],
	fRec195_idx: i32,
	fRec195_idx_save: i32,
	fRec193_perm: [F32;4],
	fRec233_perm: [F32;4],
	fRec192_perm: [F32;4],
	fRec234_perm: [F32;4],
	fRec191_perm: [F32;4],
	fRec190_perm: [F32;4],
	fHslider33: F32,
	fRec267_perm: [F32;4],
	fRec263_perm: [F32;4],
	fRec268_perm: [F32;4],
	fRec270_perm: [F32;4],
	fButton3: F32,
	fVec4_perm: [F32;4],
	iRec271_perm: [i32;4],
	fRec272_perm: [F32;4],
	fRec273: [F32;2048],
	fRec273_idx: i32,
	fRec273_idx_save: i32,
	fRec275_perm: [F32;4],
	iYec33_perm: [i32;4],
	iRec276_perm: [i32;4],
	fRec274_perm: [F32;4],
	fYec34_perm: [F32;4],
	fYec35_perm: [F32;4],
	fRec269: [F32;2048],
	fRec269_idx: i32,
	fRec269_idx_save: i32,
	fRec259_perm: [F32;4],
	fRec255_perm: [F32;4],
	fRec251: [F32;2048],
	fRec251_idx: i32,
	fRec251_idx_save: i32,
	fRec253_perm: [F32;4],
	fRec249_perm: [F32;4],
	fRec244_perm: [F32;4],
	fRec240: [F32;2048],
	fRec240_idx: i32,
	fRec240_idx_save: i32,
	fRec238_perm: [F32;4],
	fRec278_perm: [F32;4],
	fRec237_perm: [F32;4],
	fRec279_perm: [F32;4],
	fRec236_perm: [F32;4],
	fRec235_perm: [F32;4],
	fHslider34: F32,
	fRec312_perm: [F32;4],
	fRec308_perm: [F32;4],
	fRec313_perm: [F32;4],
	fRec315_perm: [F32;4],
	fButton4: F32,
	fVec5_perm: [F32;4],
	iRec316_perm: [i32;4],
	fRec317_perm: [F32;4],
	fRec318: [F32;2048],
	fRec318_idx: i32,
	fRec318_idx_save: i32,
	fRec320_perm: [F32;4],
	iYec36_perm: [i32;4],
	iRec321_perm: [i32;4],
	fRec319_perm: [F32;4],
	fYec37_perm: [F32;4],
	fYec38_perm: [F32;4],
	fRec314: [F32;2048],
	fRec314_idx: i32,
	fRec314_idx_save: i32,
	fRec304_perm: [F32;4],
	fRec300_perm: [F32;4],
	fRec296: [F32;2048],
	fRec296_idx: i32,
	fRec296_idx_save: i32,
	fRec298_perm: [F32;4],
	fRec294_perm: [F32;4],
	fRec289_perm: [F32;4],
	fRec285: [F32;2048],
	fRec285_idx: i32,
	fRec285_idx_save: i32,
	fRec283_perm: [F32;4],
	fRec323_perm: [F32;4],
	fRec282_perm: [F32;4],
	fRec324_perm: [F32;4],
	fRec281_perm: [F32;4],
	fRec280_perm: [F32;4],
	fHslider35: F32,
	fRec326_perm: [F32;4],
	fConst17: F32,
	fConst18: F32,
	fHslider36: F32,
	fRec325: [F32;2097152],
	fRec325_idx: i32,
	fRec325_idx_save: i32,
	fRec335_perm: [F32;4],
	fRec342_perm: [F32;4],
	fRec346_perm: [F32;4],
	fRec350_perm: [F32;4],
	fRec358_perm: [F32;4],
	fRec362_perm: [F32;4],
	fRec369_perm: [F32;4],
	fYec39: [F32;16384],
	fYec39_idx: i32,
	fYec39_idx_save: i32,
	fYec40_perm: [F32;4],
	fRec368_perm: [F32;4],
	fRec366_perm: [F32;4],
	fRec371_perm: [F32;4],
	fYec41: [F32;16384],
	fYec41_idx: i32,
	fYec41_idx_save: i32,
	fYec42_perm: [F32;4],
	fRec370_perm: [F32;4],
	fRec367_perm: [F32;4],
	fRec372_perm: [F32;4],
	fYec43: [F32;16384],
	fYec43_idx: i32,
	fYec43_idx_save: i32,
	fYec44_perm: [F32;4],
	fRec365_perm: [F32;4],
	fRec363_perm: [F32;4],
	fRec374_perm: [F32;4],
	fYec45: [F32;16384],
	fYec45_idx: i32,
	fYec45_idx_save: i32,
	fYec46_perm: [F32;4],
	fRec373_perm: [F32;4],
	fRec364_perm: [F32;4],
	fYec47: [F32;16384],
	fYec47_idx: i32,
	fYec47_idx_save: i32,
	fYec48_perm: [F32;4],
	fRec361_perm: [F32;4],
	fRec359_perm: [F32;4],
	fRec376_perm: [F32;4],
	fYec49: [F32;16384],
	fYec49_idx: i32,
	fYec49_idx_save: i32,
	fYec50_perm: [F32;4],
	fRec375_perm: [F32;4],
	fRec360_perm: [F32;4],
	fYec51: [F32;16384],
	fYec51_idx: i32,
	fYec51_idx_save: i32,
	fYec52_perm: [F32;4],
	fRec357_perm: [F32;4],
	fRec355_perm: [F32;4],
	fRec378_perm: [F32;4],
	fYec53: [F32;16384],
	fYec53_idx: i32,
	fYec53_idx_save: i32,
	fYec54_perm: [F32;4],
	fRec377_perm: [F32;4],
	fRec356_perm: [F32;4],
	fRec379_perm: [F32;4],
	fYec55: [F32;16384],
	fYec55_idx: i32,
	fYec55_idx_save: i32,
	fYec56_perm: [F32;4],
	fRec354_perm: [F32;4],
	fRec352_perm: [F32;4],
	fRec381_perm: [F32;4],
	fYec57: [F32;16384],
	fYec57_idx: i32,
	fYec57_idx_save: i32,
	fYec58_perm: [F32;4],
	fRec380_perm: [F32;4],
	fRec353_perm: [F32;4],
	fRec382_perm: [F32;4],
	fYec59: [F32;1024],
	fYec59_idx: i32,
	fYec59_idx_save: i32,
	fHslider37: F32,
	fYec60: [F32;16384],
	fYec60_idx: i32,
	fYec60_idx_save: i32,
	fYec61_perm: [F32;4],
	fRec351_perm: [F32;4],
	fRec384_perm: [F32;4],
	fYec62: [F32;1024],
	fYec62_idx: i32,
	fYec62_idx_save: i32,
	fYec63: [F32;16384],
	fYec63_idx: i32,
	fYec63_idx_save: i32,
	fYec64_perm: [F32;4],
	fRec383_perm: [F32;4],
	fYec65: [F32;16384],
	fYec65_idx: i32,
	fYec65_idx_save: i32,
	fYec66_perm: [F32;4],
	fRec349_perm: [F32;4],
	fRec347_perm: [F32;4],
	fRec386_perm: [F32;4],
	fYec67: [F32;16384],
	fYec67_idx: i32,
	fYec67_idx_save: i32,
	fYec68_perm: [F32;4],
	fRec385_perm: [F32;4],
	fRec348_perm: [F32;4],
	fYec69: [F32;16384],
	fYec69_idx: i32,
	fYec69_idx_save: i32,
	fYec70_perm: [F32;4],
	fRec345_perm: [F32;4],
	fRec343_perm: [F32;4],
	fYec71: [F32;16384],
	fYec71_idx: i32,
	fYec71_idx_save: i32,
	fYec72_perm: [F32;4],
	fRec387_perm: [F32;4],
	fRec344_perm: [F32;4],
	fYec73: [F32;16384],
	fYec73_idx: i32,
	fYec73_idx_save: i32,
	fYec74_perm: [F32;4],
	fRec341_perm: [F32;4],
	fRec339_perm: [F32;4],
	fRec389_perm: [F32;4],
	fYec75: [F32;16384],
	fYec75_idx: i32,
	fYec75_idx_save: i32,
	fYec76_perm: [F32;4],
	fRec388_perm: [F32;4],
	fRec340_perm: [F32;4],
	fYec77: [F32;16384],
	fYec77_idx: i32,
	fYec77_idx_save: i32,
	fYec78_perm: [F32;4],
	fRec338_perm: [F32;4],
	fRec336_perm: [F32;4],
	fYec79: [F32;16384],
	fYec79_idx: i32,
	fYec79_idx_save: i32,
	fYec80_perm: [F32;4],
	fRec390_perm: [F32;4],
	fRec337_perm: [F32;4],
	fYec81: [F32;16384],
	fYec81_idx: i32,
	fYec81_idx_save: i32,
	fYec82_perm: [F32;4],
	fRec334_perm: [F32;4],
	fRec332_perm: [F32;4],
	fRec392_perm: [F32;4],
	fYec83: [F32;16384],
	fYec83_idx: i32,
	fYec83_idx_save: i32,
	fYec84_perm: [F32;4],
	fRec391_perm: [F32;4],
	fRec333_perm: [F32;4],
	fRec393_perm: [F32;4],
	fYec85: [F32;16384],
	fYec85_idx: i32,
	fYec85_idx_save: i32,
	fYec86: [F32;16384],
	fYec86_idx: i32,
	fYec86_idx_save: i32,
	fYec87_perm: [F32;4],
	fRec331_perm: [F32;4],
	fConst20: F32,
	fConst21: F32,
	fConst23: F32,
	fConst24: F32,
	fRec330_perm: [F32;4],
	fConst26: F32,
	fConst27: F32,
	fConst28: F32,
	fConst29: F32,
	fRec329_perm: [F32;4],
	fConst30: F32,
	fConst31: F32,
	fConst32: F32,
	fRec328_perm: [F32;4],
	fConst35: F32,
	fConst36: F32,
	fConst38: F32,
	fConst39: F32,
	fRec327_perm: [F32;4],
	fRec399_perm: [F32;4],
	fRec398_perm: [F32;4],
	fRec397_perm: [F32;4],
	fConst41: F32,
	fConst42: F32,
	fYec88_perm: [F32;4],
	fConst43: F32,
	fConst44: F32,
	fRec396_perm: [F32;4],
	fConst45: F32,
	fConst46: F32,
	fRec395_perm: [F32;4],
	fConst47: F32,
	fConst48: F32,
	fConst49: F32,
	fRec394_perm: [F32;4],
	fConst50: F32,
	fRec402_perm: [F32;4],
	fRec401_perm: [F32;4],
	fRec400_perm: [F32;4],
	fHslider38: F32,
	fHslider39: F32,
	fYec89: [F32;1024],
	fYec89_idx: i32,
	fYec89_idx_save: i32,
	fRec18_perm: [F32;4],
	fRec409_perm: [F32;4],
	fYec90: [F32;16384],
	fYec90_idx: i32,
	fYec90_idx_save: i32,
	fYec91: [F32;16384],
	fYec91_idx: i32,
	fYec91_idx_save: i32,
	fYec92_perm: [F32;4],
	fRec408_perm: [F32;4],
	fRec407_perm: [F32;4],
	fRec406_perm: [F32;4],
	fRec405_perm: [F32;4],
	fRec404_perm: [F32;4],
	fRec415_perm: [F32;4],
	fRec414_perm: [F32;4],
	fRec413_perm: [F32;4],
	fYec93_perm: [F32;4],
	fRec412_perm: [F32;4],
	fRec411_perm: [F32;4],
	fRec410_perm: [F32;4],
	fRec418_perm: [F32;4],
	fRec417_perm: [F32;4],
	fRec416_perm: [F32;4],
	fYec94: [F32;1024],
	fYec94_idx: i32,
	fYec94_idx_save: i32,
	fRec403_perm: [F32;4],
	fHslider40: F32,
	fYec95: [F32;16384],
	fYec95_idx: i32,
	fYec95_idx_save: i32,
	fYec96_perm: [F32;4],
	fRec16_perm: [F32;4],
	fRec14_perm: [F32;4],
	fRec420_perm: [F32;4],
	fYec97: [F32;16384],
	fYec97_idx: i32,
	fYec97_idx_save: i32,
	fYec98_perm: [F32;4],
	fRec419_perm: [F32;4],
	fRec15_perm: [F32;4],
	fYec99: [F32;16384],
	fYec99_idx: i32,
	fYec99_idx_save: i32,
	fYec100_perm: [F32;4],
	fRec12_perm: [F32;4],
	fRec10_perm: [F32;4],
	fYec101: [F32;16384],
	fYec101_idx: i32,
	fYec101_idx_save: i32,
	fYec102_perm: [F32;4],
	fRec421_perm: [F32;4],
	fRec11_perm: [F32;4],
	fYec103: [F32;16384],
	fYec103_idx: i32,
	fYec103_idx_save: i32,
	fYec104_perm: [F32;4],
	fRec8_perm: [F32;4],
	fRec6_perm: [F32;4],
	fRec423_perm: [F32;4],
	fYec105: [F32;16384],
	fYec105_idx: i32,
	fYec105_idx_save: i32,
	fYec106_perm: [F32;4],
	fRec422_perm: [F32;4],
	fRec7_perm: [F32;4],
	fRec424_perm: [F32;4],
	fYec107: [F32;16384],
	fYec107_idx: i32,
	fYec107_idx_save: i32,
	fYec108_perm: [F32;4],
	fRec5_perm: [F32;4],
	fRec3_perm: [F32;4],
	fYec109: [F32;16384],
	fYec109_idx: i32,
	fYec109_idx_save: i32,
	fYec110_perm: [F32;4],
	fRec425_perm: [F32;4],
	fRec4_perm: [F32;4],
	fRec1_perm: [F32;4],
	fRec2_perm: [F32;4],
	fHslider41: F32,
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
			fRec0_perm: [0.0;4],
			fHslider1: 0.0,
			iVec0_perm: [0;4],
			fRec9_perm: [0.0;4],
			fRec13_perm: [0.0;4],
			fRec17_perm: [0.0;4],
			fHslider2: 0.0,
			fConst3: 0.0,
			fRec19_perm: [0.0;4],
			fRec20_perm: [0.0;4],
			fHslider3: 0.0,
			fRec21_perm: [0.0;4],
			fHslider4: 0.0,
			fRec22_perm: [0.0;4],
			fHslider5: 0.0,
			fRec23_perm: [0.0;4],
			fHslider6: 0.0,
			fRec24_perm: [0.0;4],
			fHslider7: 0.0,
			fRec25_perm: [0.0;4],
			fConst4: 0.0,
			fRec28_perm: [0.0;4],
			fConst5: 0.0,
			fYec0_perm: [0.0;4],
			fYec1: [0.0;4096],
			fYec1_idx: 0,
			fYec1_idx_save: 0,
			fConst6: 0.0,
			fRec27_perm: [0.0;4],
			fHslider8: 0.0,
			fRec29_perm: [0.0;4],
			fRec31_perm: [0.0;4],
			fYec2_perm: [0.0;4],
			fYec3: [0.0;4096],
			fYec3_idx: 0,
			fYec3_idx_save: 0,
			fRec30_perm: [0.0;4],
			fRec32_perm: [0.0;4],
			fRec34_perm: [0.0;4],
			fYec4_perm: [0.0;4],
			fYec5: [0.0;4096],
			fYec5_idx: 0,
			fYec5_idx_save: 0,
			fRec33_perm: [0.0;4],
			fHslider9: 0.0,
			fRec35_perm: [0.0;4],
			fHslider10: 0.0,
			fRec36_perm: [0.0;4],
			fRec38_perm: [0.0;4],
			fYec6_perm: [0.0;4],
			fYec7: [0.0;4096],
			fYec7_idx: 0,
			fYec7_idx_save: 0,
			fRec37_perm: [0.0;4],
			fRec39_perm: [0.0;4],
			fRec41_perm: [0.0;4],
			fYec8_perm: [0.0;4],
			fYec9: [0.0;4096],
			fYec9_idx: 0,
			fYec9_idx_save: 0,
			fRec40_perm: [0.0;4],
			fRec42_perm: [0.0;4],
			fRec44_perm: [0.0;4],
			fYec10_perm: [0.0;4],
			fYec11: [0.0;4096],
			fYec11_idx: 0,
			fYec11_idx_save: 0,
			fRec43_perm: [0.0;4],
			fHslider11: 0.0,
			fRec45_perm: [0.0;4],
			fHslider12: 0.0,
			fRec46_perm: [0.0;4],
			fRec48_perm: [0.0;4],
			fYec12_perm: [0.0;4],
			fYec13: [0.0;4096],
			fYec13_idx: 0,
			fYec13_idx_save: 0,
			fRec47_perm: [0.0;4],
			fRec49_perm: [0.0;4],
			fRec51_perm: [0.0;4],
			fYec14_perm: [0.0;4],
			fYec15: [0.0;4096],
			fYec15_idx: 0,
			fYec15_idx_save: 0,
			fRec50_perm: [0.0;4],
			fRec52_perm: [0.0;4],
			fRec54_perm: [0.0;4],
			fYec16_perm: [0.0;4],
			fYec17: [0.0;4096],
			fYec17_idx: 0,
			fYec17_idx_save: 0,
			fRec53_perm: [0.0;4],
			fHslider13: 0.0,
			fRec55_perm: [0.0;4],
			fHslider14: 0.0,
			fRec56_perm: [0.0;4],
			fRec58_perm: [0.0;4],
			fYec18_perm: [0.0;4],
			fYec19: [0.0;4096],
			fYec19_idx: 0,
			fYec19_idx_save: 0,
			fRec57_perm: [0.0;4],
			fRec59_perm: [0.0;4],
			fRec61_perm: [0.0;4],
			fYec20_perm: [0.0;4],
			fYec21: [0.0;4096],
			fYec21_idx: 0,
			fYec21_idx_save: 0,
			fRec60_perm: [0.0;4],
			fRec62_perm: [0.0;4],
			fRec64_perm: [0.0;4],
			fYec22_perm: [0.0;4],
			fYec23: [0.0;4096],
			fYec23_idx: 0,
			fYec23_idx_save: 0,
			fRec63_perm: [0.0;4],
			fHslider15: 0.0,
			fRec65_perm: [0.0;4],
			fHslider16: 0.0,
			fRec66_perm: [0.0;4],
			fHslider17: 0.0,
			fRec69_perm: [0.0;4],
			fHslider18: 0.0,
			fRec73_perm: [0.0;4],
			fHslider19: 0.0,
			fRec72_perm: [0.0;4],
			fRec70_perm: [0.0;4],
			fHslider20: 0.0,
			fRec74_perm: [0.0;4],
			fHslider21: 0.0,
			fRec76_perm: [0.0;4],
			fRec75_perm: [0.0;4],
			fConst7: 0.0,
			fRec68_perm: [0.0;4],
			fRec67_perm: [0.0;4],
			fHslider22: 0.0,
			fRec79_perm: [0.0;4],
			fHslider23: 0.0,
			fRec82_perm: [0.0;4],
			fRec80_perm: [0.0;4],
			fRec83_perm: [0.0;4],
			fRec78_perm: [0.0;4],
			fRec77_perm: [0.0;4],
			fHslider24: 0.0,
			fRec86_perm: [0.0;4],
			fHslider25: 0.0,
			fRec89_perm: [0.0;4],
			fRec87_perm: [0.0;4],
			fRec90_perm: [0.0;4],
			fRec85_perm: [0.0;4],
			fRec84_perm: [0.0;4],
			fHslider26: 0.0,
			fRec93_perm: [0.0;4],
			fHslider27: 0.0,
			fRec96_perm: [0.0;4],
			fRec94_perm: [0.0;4],
			fRec97_perm: [0.0;4],
			fRec92_perm: [0.0;4],
			fRec91_perm: [0.0;4],
			fHslider28: 0.0,
			fRec98_perm: [0.0;4],
			fHslider29: 0.0,
			fRec131_perm: [0.0;4],
			fConst8: 0.0,
			fConst9: 0.0,
			fRec127_perm: [0.0;4],
			fRec132_perm: [0.0;4],
			iRec135_perm: [0;4],
			fConst10: 0.0,
			fRec134_perm: [0.0;4],
			fButton0: 0.0,
			fVec1_perm: [0.0;4],
			iRec136_perm: [0;4],
			fRec137_perm: [0.0;4],
			fRec138: [0.0;2048],
			fRec138_idx: 0,
			fRec138_idx_save: 0,
			fConst11: 0.0,
			fConst12: 0.0,
			fRec140_perm: [0.0;4],
			iYec24_perm: [0;4],
			iConst13: 0,
			iRec141_perm: [0;4],
			fConst14: 0.0,
			fRec139_perm: [0.0;4],
			fYec25_perm: [0.0;4],
			fConst15: 0.0,
			fYec26_perm: [0.0;4],
			fRec133: [0.0;2048],
			fRec133_idx: 0,
			fRec133_idx_save: 0,
			fRec123_perm: [0.0;4],
			fRec119_perm: [0.0;4],
			fRec115: [0.0;2048],
			fRec115_idx: 0,
			fRec115_idx_save: 0,
			fRec117_perm: [0.0;4],
			fHslider30: 0.0,
			fRec113_perm: [0.0;4],
			fRec108_perm: [0.0;4],
			fRec104: [0.0;2048],
			fRec104_idx: 0,
			fRec104_idx_save: 0,
			fRec102_perm: [0.0;4],
			fConst16: 0.0,
			fRec143_perm: [0.0;4],
			fRec101_perm: [0.0;4],
			fRec144_perm: [0.0;4],
			fRec100_perm: [0.0;4],
			fRec99_perm: [0.0;4],
			fHslider31: 0.0,
			fRec177_perm: [0.0;4],
			fRec173_perm: [0.0;4],
			fRec178_perm: [0.0;4],
			fRec180_perm: [0.0;4],
			fButton1: 0.0,
			fVec2_perm: [0.0;4],
			iRec181_perm: [0;4],
			fRec182_perm: [0.0;4],
			fRec183: [0.0;2048],
			fRec183_idx: 0,
			fRec183_idx_save: 0,
			fRec185_perm: [0.0;4],
			iYec27_perm: [0;4],
			iRec186_perm: [0;4],
			fRec184_perm: [0.0;4],
			fYec28_perm: [0.0;4],
			fYec29_perm: [0.0;4],
			fRec179: [0.0;2048],
			fRec179_idx: 0,
			fRec179_idx_save: 0,
			fRec169_perm: [0.0;4],
			fRec165_perm: [0.0;4],
			fRec161: [0.0;2048],
			fRec161_idx: 0,
			fRec161_idx_save: 0,
			fRec163_perm: [0.0;4],
			fRec159_perm: [0.0;4],
			fRec154_perm: [0.0;4],
			fRec150: [0.0;2048],
			fRec150_idx: 0,
			fRec150_idx_save: 0,
			fRec148_perm: [0.0;4],
			fRec188_perm: [0.0;4],
			fRec147_perm: [0.0;4],
			fRec189_perm: [0.0;4],
			fRec146_perm: [0.0;4],
			fRec145_perm: [0.0;4],
			fHslider32: 0.0,
			fRec222_perm: [0.0;4],
			fRec218_perm: [0.0;4],
			fRec223_perm: [0.0;4],
			fRec225_perm: [0.0;4],
			fButton2: 0.0,
			fVec3_perm: [0.0;4],
			iRec226_perm: [0;4],
			fRec227_perm: [0.0;4],
			fRec228: [0.0;2048],
			fRec228_idx: 0,
			fRec228_idx_save: 0,
			fRec230_perm: [0.0;4],
			iYec30_perm: [0;4],
			iRec231_perm: [0;4],
			fRec229_perm: [0.0;4],
			fYec31_perm: [0.0;4],
			fYec32_perm: [0.0;4],
			fRec224: [0.0;2048],
			fRec224_idx: 0,
			fRec224_idx_save: 0,
			fRec214_perm: [0.0;4],
			fRec210_perm: [0.0;4],
			fRec206: [0.0;2048],
			fRec206_idx: 0,
			fRec206_idx_save: 0,
			fRec208_perm: [0.0;4],
			fRec204_perm: [0.0;4],
			fRec199_perm: [0.0;4],
			fRec195: [0.0;2048],
			fRec195_idx: 0,
			fRec195_idx_save: 0,
			fRec193_perm: [0.0;4],
			fRec233_perm: [0.0;4],
			fRec192_perm: [0.0;4],
			fRec234_perm: [0.0;4],
			fRec191_perm: [0.0;4],
			fRec190_perm: [0.0;4],
			fHslider33: 0.0,
			fRec267_perm: [0.0;4],
			fRec263_perm: [0.0;4],
			fRec268_perm: [0.0;4],
			fRec270_perm: [0.0;4],
			fButton3: 0.0,
			fVec4_perm: [0.0;4],
			iRec271_perm: [0;4],
			fRec272_perm: [0.0;4],
			fRec273: [0.0;2048],
			fRec273_idx: 0,
			fRec273_idx_save: 0,
			fRec275_perm: [0.0;4],
			iYec33_perm: [0;4],
			iRec276_perm: [0;4],
			fRec274_perm: [0.0;4],
			fYec34_perm: [0.0;4],
			fYec35_perm: [0.0;4],
			fRec269: [0.0;2048],
			fRec269_idx: 0,
			fRec269_idx_save: 0,
			fRec259_perm: [0.0;4],
			fRec255_perm: [0.0;4],
			fRec251: [0.0;2048],
			fRec251_idx: 0,
			fRec251_idx_save: 0,
			fRec253_perm: [0.0;4],
			fRec249_perm: [0.0;4],
			fRec244_perm: [0.0;4],
			fRec240: [0.0;2048],
			fRec240_idx: 0,
			fRec240_idx_save: 0,
			fRec238_perm: [0.0;4],
			fRec278_perm: [0.0;4],
			fRec237_perm: [0.0;4],
			fRec279_perm: [0.0;4],
			fRec236_perm: [0.0;4],
			fRec235_perm: [0.0;4],
			fHslider34: 0.0,
			fRec312_perm: [0.0;4],
			fRec308_perm: [0.0;4],
			fRec313_perm: [0.0;4],
			fRec315_perm: [0.0;4],
			fButton4: 0.0,
			fVec5_perm: [0.0;4],
			iRec316_perm: [0;4],
			fRec317_perm: [0.0;4],
			fRec318: [0.0;2048],
			fRec318_idx: 0,
			fRec318_idx_save: 0,
			fRec320_perm: [0.0;4],
			iYec36_perm: [0;4],
			iRec321_perm: [0;4],
			fRec319_perm: [0.0;4],
			fYec37_perm: [0.0;4],
			fYec38_perm: [0.0;4],
			fRec314: [0.0;2048],
			fRec314_idx: 0,
			fRec314_idx_save: 0,
			fRec304_perm: [0.0;4],
			fRec300_perm: [0.0;4],
			fRec296: [0.0;2048],
			fRec296_idx: 0,
			fRec296_idx_save: 0,
			fRec298_perm: [0.0;4],
			fRec294_perm: [0.0;4],
			fRec289_perm: [0.0;4],
			fRec285: [0.0;2048],
			fRec285_idx: 0,
			fRec285_idx_save: 0,
			fRec283_perm: [0.0;4],
			fRec323_perm: [0.0;4],
			fRec282_perm: [0.0;4],
			fRec324_perm: [0.0;4],
			fRec281_perm: [0.0;4],
			fRec280_perm: [0.0;4],
			fHslider35: 0.0,
			fRec326_perm: [0.0;4],
			fConst17: 0.0,
			fConst18: 0.0,
			fHslider36: 0.0,
			fRec325: [0.0;2097152],
			fRec325_idx: 0,
			fRec325_idx_save: 0,
			fRec335_perm: [0.0;4],
			fRec342_perm: [0.0;4],
			fRec346_perm: [0.0;4],
			fRec350_perm: [0.0;4],
			fRec358_perm: [0.0;4],
			fRec362_perm: [0.0;4],
			fRec369_perm: [0.0;4],
			fYec39: [0.0;16384],
			fYec39_idx: 0,
			fYec39_idx_save: 0,
			fYec40_perm: [0.0;4],
			fRec368_perm: [0.0;4],
			fRec366_perm: [0.0;4],
			fRec371_perm: [0.0;4],
			fYec41: [0.0;16384],
			fYec41_idx: 0,
			fYec41_idx_save: 0,
			fYec42_perm: [0.0;4],
			fRec370_perm: [0.0;4],
			fRec367_perm: [0.0;4],
			fRec372_perm: [0.0;4],
			fYec43: [0.0;16384],
			fYec43_idx: 0,
			fYec43_idx_save: 0,
			fYec44_perm: [0.0;4],
			fRec365_perm: [0.0;4],
			fRec363_perm: [0.0;4],
			fRec374_perm: [0.0;4],
			fYec45: [0.0;16384],
			fYec45_idx: 0,
			fYec45_idx_save: 0,
			fYec46_perm: [0.0;4],
			fRec373_perm: [0.0;4],
			fRec364_perm: [0.0;4],
			fYec47: [0.0;16384],
			fYec47_idx: 0,
			fYec47_idx_save: 0,
			fYec48_perm: [0.0;4],
			fRec361_perm: [0.0;4],
			fRec359_perm: [0.0;4],
			fRec376_perm: [0.0;4],
			fYec49: [0.0;16384],
			fYec49_idx: 0,
			fYec49_idx_save: 0,
			fYec50_perm: [0.0;4],
			fRec375_perm: [0.0;4],
			fRec360_perm: [0.0;4],
			fYec51: [0.0;16384],
			fYec51_idx: 0,
			fYec51_idx_save: 0,
			fYec52_perm: [0.0;4],
			fRec357_perm: [0.0;4],
			fRec355_perm: [0.0;4],
			fRec378_perm: [0.0;4],
			fYec53: [0.0;16384],
			fYec53_idx: 0,
			fYec53_idx_save: 0,
			fYec54_perm: [0.0;4],
			fRec377_perm: [0.0;4],
			fRec356_perm: [0.0;4],
			fRec379_perm: [0.0;4],
			fYec55: [0.0;16384],
			fYec55_idx: 0,
			fYec55_idx_save: 0,
			fYec56_perm: [0.0;4],
			fRec354_perm: [0.0;4],
			fRec352_perm: [0.0;4],
			fRec381_perm: [0.0;4],
			fYec57: [0.0;16384],
			fYec57_idx: 0,
			fYec57_idx_save: 0,
			fYec58_perm: [0.0;4],
			fRec380_perm: [0.0;4],
			fRec353_perm: [0.0;4],
			fRec382_perm: [0.0;4],
			fYec59: [0.0;1024],
			fYec59_idx: 0,
			fYec59_idx_save: 0,
			fHslider37: 0.0,
			fYec60: [0.0;16384],
			fYec60_idx: 0,
			fYec60_idx_save: 0,
			fYec61_perm: [0.0;4],
			fRec351_perm: [0.0;4],
			fRec384_perm: [0.0;4],
			fYec62: [0.0;1024],
			fYec62_idx: 0,
			fYec62_idx_save: 0,
			fYec63: [0.0;16384],
			fYec63_idx: 0,
			fYec63_idx_save: 0,
			fYec64_perm: [0.0;4],
			fRec383_perm: [0.0;4],
			fYec65: [0.0;16384],
			fYec65_idx: 0,
			fYec65_idx_save: 0,
			fYec66_perm: [0.0;4],
			fRec349_perm: [0.0;4],
			fRec347_perm: [0.0;4],
			fRec386_perm: [0.0;4],
			fYec67: [0.0;16384],
			fYec67_idx: 0,
			fYec67_idx_save: 0,
			fYec68_perm: [0.0;4],
			fRec385_perm: [0.0;4],
			fRec348_perm: [0.0;4],
			fYec69: [0.0;16384],
			fYec69_idx: 0,
			fYec69_idx_save: 0,
			fYec70_perm: [0.0;4],
			fRec345_perm: [0.0;4],
			fRec343_perm: [0.0;4],
			fYec71: [0.0;16384],
			fYec71_idx: 0,
			fYec71_idx_save: 0,
			fYec72_perm: [0.0;4],
			fRec387_perm: [0.0;4],
			fRec344_perm: [0.0;4],
			fYec73: [0.0;16384],
			fYec73_idx: 0,
			fYec73_idx_save: 0,
			fYec74_perm: [0.0;4],
			fRec341_perm: [0.0;4],
			fRec339_perm: [0.0;4],
			fRec389_perm: [0.0;4],
			fYec75: [0.0;16384],
			fYec75_idx: 0,
			fYec75_idx_save: 0,
			fYec76_perm: [0.0;4],
			fRec388_perm: [0.0;4],
			fRec340_perm: [0.0;4],
			fYec77: [0.0;16384],
			fYec77_idx: 0,
			fYec77_idx_save: 0,
			fYec78_perm: [0.0;4],
			fRec338_perm: [0.0;4],
			fRec336_perm: [0.0;4],
			fYec79: [0.0;16384],
			fYec79_idx: 0,
			fYec79_idx_save: 0,
			fYec80_perm: [0.0;4],
			fRec390_perm: [0.0;4],
			fRec337_perm: [0.0;4],
			fYec81: [0.0;16384],
			fYec81_idx: 0,
			fYec81_idx_save: 0,
			fYec82_perm: [0.0;4],
			fRec334_perm: [0.0;4],
			fRec332_perm: [0.0;4],
			fRec392_perm: [0.0;4],
			fYec83: [0.0;16384],
			fYec83_idx: 0,
			fYec83_idx_save: 0,
			fYec84_perm: [0.0;4],
			fRec391_perm: [0.0;4],
			fRec333_perm: [0.0;4],
			fRec393_perm: [0.0;4],
			fYec85: [0.0;16384],
			fYec85_idx: 0,
			fYec85_idx_save: 0,
			fYec86: [0.0;16384],
			fYec86_idx: 0,
			fYec86_idx_save: 0,
			fYec87_perm: [0.0;4],
			fRec331_perm: [0.0;4],
			fConst20: 0.0,
			fConst21: 0.0,
			fConst23: 0.0,
			fConst24: 0.0,
			fRec330_perm: [0.0;4],
			fConst26: 0.0,
			fConst27: 0.0,
			fConst28: 0.0,
			fConst29: 0.0,
			fRec329_perm: [0.0;4],
			fConst30: 0.0,
			fConst31: 0.0,
			fConst32: 0.0,
			fRec328_perm: [0.0;4],
			fConst35: 0.0,
			fConst36: 0.0,
			fConst38: 0.0,
			fConst39: 0.0,
			fRec327_perm: [0.0;4],
			fRec399_perm: [0.0;4],
			fRec398_perm: [0.0;4],
			fRec397_perm: [0.0;4],
			fConst41: 0.0,
			fConst42: 0.0,
			fYec88_perm: [0.0;4],
			fConst43: 0.0,
			fConst44: 0.0,
			fRec396_perm: [0.0;4],
			fConst45: 0.0,
			fConst46: 0.0,
			fRec395_perm: [0.0;4],
			fConst47: 0.0,
			fConst48: 0.0,
			fConst49: 0.0,
			fRec394_perm: [0.0;4],
			fConst50: 0.0,
			fRec402_perm: [0.0;4],
			fRec401_perm: [0.0;4],
			fRec400_perm: [0.0;4],
			fHslider38: 0.0,
			fHslider39: 0.0,
			fYec89: [0.0;1024],
			fYec89_idx: 0,
			fYec89_idx_save: 0,
			fRec18_perm: [0.0;4],
			fRec409_perm: [0.0;4],
			fYec90: [0.0;16384],
			fYec90_idx: 0,
			fYec90_idx_save: 0,
			fYec91: [0.0;16384],
			fYec91_idx: 0,
			fYec91_idx_save: 0,
			fYec92_perm: [0.0;4],
			fRec408_perm: [0.0;4],
			fRec407_perm: [0.0;4],
			fRec406_perm: [0.0;4],
			fRec405_perm: [0.0;4],
			fRec404_perm: [0.0;4],
			fRec415_perm: [0.0;4],
			fRec414_perm: [0.0;4],
			fRec413_perm: [0.0;4],
			fYec93_perm: [0.0;4],
			fRec412_perm: [0.0;4],
			fRec411_perm: [0.0;4],
			fRec410_perm: [0.0;4],
			fRec418_perm: [0.0;4],
			fRec417_perm: [0.0;4],
			fRec416_perm: [0.0;4],
			fYec94: [0.0;1024],
			fYec94_idx: 0,
			fYec94_idx_save: 0,
			fRec403_perm: [0.0;4],
			fHslider40: 0.0,
			fYec95: [0.0;16384],
			fYec95_idx: 0,
			fYec95_idx_save: 0,
			fYec96_perm: [0.0;4],
			fRec16_perm: [0.0;4],
			fRec14_perm: [0.0;4],
			fRec420_perm: [0.0;4],
			fYec97: [0.0;16384],
			fYec97_idx: 0,
			fYec97_idx_save: 0,
			fYec98_perm: [0.0;4],
			fRec419_perm: [0.0;4],
			fRec15_perm: [0.0;4],
			fYec99: [0.0;16384],
			fYec99_idx: 0,
			fYec99_idx_save: 0,
			fYec100_perm: [0.0;4],
			fRec12_perm: [0.0;4],
			fRec10_perm: [0.0;4],
			fYec101: [0.0;16384],
			fYec101_idx: 0,
			fYec101_idx_save: 0,
			fYec102_perm: [0.0;4],
			fRec421_perm: [0.0;4],
			fRec11_perm: [0.0;4],
			fYec103: [0.0;16384],
			fYec103_idx: 0,
			fYec103_idx_save: 0,
			fYec104_perm: [0.0;4],
			fRec8_perm: [0.0;4],
			fRec6_perm: [0.0;4],
			fRec423_perm: [0.0;4],
			fYec105: [0.0;16384],
			fYec105_idx: 0,
			fYec105_idx_save: 0,
			fYec106_perm: [0.0;4],
			fRec422_perm: [0.0;4],
			fRec7_perm: [0.0;4],
			fRec424_perm: [0.0;4],
			fYec107: [0.0;16384],
			fYec107_idx: 0,
			fYec107_idx_save: 0,
			fYec108_perm: [0.0;4],
			fRec5_perm: [0.0;4],
			fRec3_perm: [0.0;4],
			fYec109: [0.0;16384],
			fYec109_idx: 0,
			fYec109_idx_save: 0,
			fYec110_perm: [0.0;4],
			fRec425_perm: [0.0;4],
			fRec4_perm: [0.0;4],
			fRec1_perm: [0.0;4],
			fRec2_perm: [0.0;4],
			fHslider41: 0.0,
		}
	}
	fn metadata(&self, m: &mut dyn Meta) { 
		m.declare("analyzers.lib/amp_follower_ar:author", "Jonatan Liljedahl, revised by Romain Michon");
		m.declare("analyzers.lib/name", "Faust Analyzer Library");
		m.declare("analyzers.lib/version", "1.2.0");
		m.declare("author", "Pierre Lulé");
		m.declare("basics.lib/name", "Faust Basic Element Library");
		m.declare("basics.lib/tabulate:author", "Stephane Letz");
		m.declare("basics.lib/tabulateNd", "Copyright (C) 2023 Bart Brouns <bart@magnetophon.nl>");
		m.declare("basics.lib/version", "1.11.1");
		m.declare("compile_options", r"-a C:\Users\Pierre\AppData\Local\Temp\.tmpLyRcxs -lang rust -ct 1 -es 1 -mcd 16 -single -ftz 0 -vec -lv 0 -vs 32");
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
		m.declare("compressors.lib/version", "1.5.0");
		m.declare("delays.lib/fdelay1a:author", "Julius O. Smith III");
		m.declare("delays.lib/fdelay4:author", "Julius O. Smith III");
		m.declare("delays.lib/fdelayltv:author", "Julius O. Smith III");
		m.declare("delays.lib/name", "Faust Delay Library");
		m.declare("delays.lib/version", "1.1.0");
		m.declare("envelopes.lib/ar:author", "Yann Orlarey, Stéphane Letz");
		m.declare("envelopes.lib/author", "GRAME");
		m.declare("envelopes.lib/copyright", "GRAME");
		m.declare("envelopes.lib/license", "LGPL with exception");
		m.declare("envelopes.lib/name", "Faust Envelope Library");
		m.declare("envelopes.lib/version", "1.2.0");
		m.declare("filename", "instrument.dsp");
		m.declare("filters.lib/filterbank:author", "Julius O. Smith III");
		m.declare("filters.lib/filterbank:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/filterbank:license", "MIT-style STK-4.3 license");
		m.declare("filters.lib/fir:author", "Julius O. Smith III");
		m.declare("filters.lib/fir:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/fir:license", "MIT-style STK-4.3 license");
		m.declare("filters.lib/highpass:author", "Julius O. Smith III");
		m.declare("filters.lib/highpass:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/highpass_plus_lowpass:author", "Julius O. Smith III");
		m.declare("filters.lib/highpass_plus_lowpass:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/highpass_plus_lowpass:license", "MIT-style STK-4.3 license");
		m.declare("filters.lib/iir:author", "Julius O. Smith III");
		m.declare("filters.lib/iir:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/iir:license", "MIT-style STK-4.3 license");
		m.declare("filters.lib/lowpass0_highpass1", "MIT-style STK-4.3 license");
		m.declare("filters.lib/lowpass0_highpass1:author", "Julius O. Smith III");
		m.declare("filters.lib/lowpass:author", "Julius O. Smith III");
		m.declare("filters.lib/lowpass:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/lowpass:license", "MIT-style STK-4.3 license");
		m.declare("filters.lib/name", "Faust Filters Library");
		m.declare("filters.lib/nlf2:author", "Julius O. Smith III");
		m.declare("filters.lib/nlf2:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/nlf2:license", "MIT-style STK-4.3 license");
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
		m.declare("filters.lib/version", "1.3.0");
		m.declare("license", "BSD");
		m.declare("maths.lib/author", "GRAME");
		m.declare("maths.lib/copyright", "GRAME");
		m.declare("maths.lib/license", "LGPL with exception");
		m.declare("maths.lib/name", "Faust Math Library");
		m.declare("maths.lib/version", "2.6.0");
		m.declare("misceffects.lib/echo:author", "Romain Michon");
		m.declare("misceffects.lib/name", "Misc Effects Library");
		m.declare("misceffects.lib/version", "2.1.0");
		m.declare("name", "theremotion");
		m.declare("noises.lib/name", "Faust Noise Generator Library");
		m.declare("noises.lib/version", "1.4.0");
		m.declare("oscillators.lib/lf_sawpos:author", "Bart Brouns, revised by Stéphane Letz");
		m.declare("oscillators.lib/lf_sawpos:licence", "STK-4.3");
		m.declare("oscillators.lib/name", "Faust Oscillator Library");
		m.declare("oscillators.lib/saw2ptr:author", "Julius O. Smith III");
		m.declare("oscillators.lib/saw2ptr:license", "STK-4.3");
		m.declare("oscillators.lib/sawN:author", "Julius O. Smith III");
		m.declare("oscillators.lib/sawN:license", "STK-4.3");
		m.declare("oscillators.lib/version", "1.4.0");
		m.declare("physmodels.lib/name", "Faust Physical Models Library");
		m.declare("physmodels.lib/version", "1.1.0");
		m.declare("platform.lib/name", "Generic Platform Library");
		m.declare("platform.lib/version", "1.3.0");
		m.declare("reverbs.lib/jpverb:author", "Julian Parker, bug fixes and minor interface changes by Till Bovermann");
		m.declare("reverbs.lib/jpverb:license", "GPL2+");
		m.declare("reverbs.lib/name", "Faust Reverb Library");
		m.declare("reverbs.lib/version", "1.2.0");
		m.declare("routes.lib/name", "Faust Signal Routing Library");
		m.declare("routes.lib/version", "1.2.0");
		m.declare("signals.lib/name", "Faust Signal Routing Library");
		m.declare("signals.lib/onePoleSwitching:author", "Jonatan Liljedahl, revised by Dario Sanfilippo");
		m.declare("signals.lib/onePoleSwitching:licence", "STK-4.3");
		m.declare("signals.lib/version", "1.3.0");
		m.declare("vaeffects.lib/moog_vcf_2b:author", "Julius O. Smith III");
		m.declare("vaeffects.lib/moog_vcf_2b:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("vaeffects.lib/moog_vcf_2b:license", "MIT-style STK-4.3 license");
		m.declare("vaeffects.lib/name", "Faust Virtual Analog Filter Effect Library");
		m.declare("vaeffects.lib/version", "1.2.0");
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
		let mut sig0: mydspSIG0 = newmydspSIG0();
		sig0.instance_initmydspSIG0(sample_rate);
		sig0.fillmydspSIG0(1302, unsafe { &mut itbl0mydspSIG0 });
		let mut sig1: mydspSIG1 = newmydspSIG1();
		sig1.instance_initmydspSIG1(sample_rate);
		sig1.fillmydspSIG1(2048, unsafe { &mut ftbl1mydspSIG1 });
	}
	fn instance_reset_params(&mut self) {
		self.fHslider0 = 1.0;
		self.fHslider1 = 5.0;
		self.fHslider2 = 0.6;
		self.fHslider3 = 1.0;
		self.fHslider4 = 1.0;
		self.fHslider5 = 0.0;
		self.fHslider6 = 0.0;
		self.fHslider7 = 6e+01;
		self.fHslider8 = 0.1;
		self.fHslider9 = 0.0;
		self.fHslider10 = 6e+01;
		self.fHslider11 = 0.0;
		self.fHslider12 = 6e+01;
		self.fHslider13 = 0.0;
		self.fHslider14 = 6e+01;
		self.fHslider15 = 0.0;
		self.fHslider16 = 1.0;
		self.fHslider17 = 0.0;
		self.fHslider18 = 0.0;
		self.fHslider19 = 6e+01;
		self.fHslider20 = 0.0;
		self.fHslider21 = 0.0;
		self.fHslider22 = 0.0;
		self.fHslider23 = 6e+01;
		self.fHslider24 = 0.0;
		self.fHslider25 = 6e+01;
		self.fHslider26 = 0.0;
		self.fHslider27 = 6e+01;
		self.fHslider28 = 1.0;
		self.fHslider29 = 8e+01;
		self.fButton0 = 0.0;
		self.fHslider30 = 1.0;
		self.fHslider31 = 8e+01;
		self.fButton1 = 0.0;
		self.fHslider32 = 8e+01;
		self.fButton2 = 0.0;
		self.fHslider33 = 8e+01;
		self.fButton3 = 0.0;
		self.fHslider34 = 8e+01;
		self.fButton4 = 0.0;
		self.fHslider35 = 0.3;
		self.fHslider36 = 0.3;
		self.fHslider37 = 0.98;
		self.fHslider38 = 0.88;
		self.fHslider39 = 3.5;
		self.fHslider40 = 0.75;
		self.fHslider41 = 0.11;
	}
	fn instance_clear(&mut self) {
		self.fRec0_perm.fill(0.0);
		self.iVec0_perm.fill(0);
		self.fRec9_perm.fill(0.0);
		self.fRec13_perm.fill(0.0);
		self.fRec17_perm.fill(0.0);
		self.fRec19_perm.fill(0.0);
		self.fRec20_perm.fill(0.0);
		self.fRec21_perm.fill(0.0);
		self.fRec22_perm.fill(0.0);
		self.fRec23_perm.fill(0.0);
		self.fRec24_perm.fill(0.0);
		self.fRec25_perm.fill(0.0);
		self.fRec28_perm.fill(0.0);
		self.fYec0_perm.fill(0.0);
		self.fYec1.fill(0.0);
		self.fYec1_idx = 0;
		self.fYec1_idx_save = 0;
		self.fRec27_perm.fill(0.0);
		self.fRec29_perm.fill(0.0);
		self.fRec31_perm.fill(0.0);
		self.fYec2_perm.fill(0.0);
		self.fYec3.fill(0.0);
		self.fYec3_idx = 0;
		self.fYec3_idx_save = 0;
		self.fRec30_perm.fill(0.0);
		self.fRec32_perm.fill(0.0);
		self.fRec34_perm.fill(0.0);
		self.fYec4_perm.fill(0.0);
		self.fYec5.fill(0.0);
		self.fYec5_idx = 0;
		self.fYec5_idx_save = 0;
		self.fRec33_perm.fill(0.0);
		self.fRec35_perm.fill(0.0);
		self.fRec36_perm.fill(0.0);
		self.fRec38_perm.fill(0.0);
		self.fYec6_perm.fill(0.0);
		self.fYec7.fill(0.0);
		self.fYec7_idx = 0;
		self.fYec7_idx_save = 0;
		self.fRec37_perm.fill(0.0);
		self.fRec39_perm.fill(0.0);
		self.fRec41_perm.fill(0.0);
		self.fYec8_perm.fill(0.0);
		self.fYec9.fill(0.0);
		self.fYec9_idx = 0;
		self.fYec9_idx_save = 0;
		self.fRec40_perm.fill(0.0);
		self.fRec42_perm.fill(0.0);
		self.fRec44_perm.fill(0.0);
		self.fYec10_perm.fill(0.0);
		self.fYec11.fill(0.0);
		self.fYec11_idx = 0;
		self.fYec11_idx_save = 0;
		self.fRec43_perm.fill(0.0);
		self.fRec45_perm.fill(0.0);
		self.fRec46_perm.fill(0.0);
		self.fRec48_perm.fill(0.0);
		self.fYec12_perm.fill(0.0);
		self.fYec13.fill(0.0);
		self.fYec13_idx = 0;
		self.fYec13_idx_save = 0;
		self.fRec47_perm.fill(0.0);
		self.fRec49_perm.fill(0.0);
		self.fRec51_perm.fill(0.0);
		self.fYec14_perm.fill(0.0);
		self.fYec15.fill(0.0);
		self.fYec15_idx = 0;
		self.fYec15_idx_save = 0;
		self.fRec50_perm.fill(0.0);
		self.fRec52_perm.fill(0.0);
		self.fRec54_perm.fill(0.0);
		self.fYec16_perm.fill(0.0);
		self.fYec17.fill(0.0);
		self.fYec17_idx = 0;
		self.fYec17_idx_save = 0;
		self.fRec53_perm.fill(0.0);
		self.fRec55_perm.fill(0.0);
		self.fRec56_perm.fill(0.0);
		self.fRec58_perm.fill(0.0);
		self.fYec18_perm.fill(0.0);
		self.fYec19.fill(0.0);
		self.fYec19_idx = 0;
		self.fYec19_idx_save = 0;
		self.fRec57_perm.fill(0.0);
		self.fRec59_perm.fill(0.0);
		self.fRec61_perm.fill(0.0);
		self.fYec20_perm.fill(0.0);
		self.fYec21.fill(0.0);
		self.fYec21_idx = 0;
		self.fYec21_idx_save = 0;
		self.fRec60_perm.fill(0.0);
		self.fRec62_perm.fill(0.0);
		self.fRec64_perm.fill(0.0);
		self.fYec22_perm.fill(0.0);
		self.fYec23.fill(0.0);
		self.fYec23_idx = 0;
		self.fYec23_idx_save = 0;
		self.fRec63_perm.fill(0.0);
		self.fRec65_perm.fill(0.0);
		self.fRec66_perm.fill(0.0);
		self.fRec69_perm.fill(0.0);
		self.fRec73_perm.fill(0.0);
		self.fRec72_perm.fill(0.0);
		self.fRec70_perm.fill(0.0);
		self.fRec74_perm.fill(0.0);
		self.fRec76_perm.fill(0.0);
		self.fRec75_perm.fill(0.0);
		self.fRec68_perm.fill(0.0);
		self.fRec67_perm.fill(0.0);
		self.fRec79_perm.fill(0.0);
		self.fRec82_perm.fill(0.0);
		self.fRec80_perm.fill(0.0);
		self.fRec83_perm.fill(0.0);
		self.fRec78_perm.fill(0.0);
		self.fRec77_perm.fill(0.0);
		self.fRec86_perm.fill(0.0);
		self.fRec89_perm.fill(0.0);
		self.fRec87_perm.fill(0.0);
		self.fRec90_perm.fill(0.0);
		self.fRec85_perm.fill(0.0);
		self.fRec84_perm.fill(0.0);
		self.fRec93_perm.fill(0.0);
		self.fRec96_perm.fill(0.0);
		self.fRec94_perm.fill(0.0);
		self.fRec97_perm.fill(0.0);
		self.fRec92_perm.fill(0.0);
		self.fRec91_perm.fill(0.0);
		self.fRec98_perm.fill(0.0);
		self.fRec131_perm.fill(0.0);
		self.fRec127_perm.fill(0.0);
		self.fRec132_perm.fill(0.0);
		self.iRec135_perm.fill(0);
		self.fRec134_perm.fill(0.0);
		self.fVec1_perm.fill(0.0);
		self.iRec136_perm.fill(0);
		self.fRec137_perm.fill(0.0);
		self.fRec138.fill(0.0);
		self.fRec138_idx = 0;
		self.fRec138_idx_save = 0;
		self.fRec140_perm.fill(0.0);
		self.iYec24_perm.fill(0);
		self.iRec141_perm.fill(0);
		self.fRec139_perm.fill(0.0);
		self.fYec25_perm.fill(0.0);
		self.fYec26_perm.fill(0.0);
		self.fRec133.fill(0.0);
		self.fRec133_idx = 0;
		self.fRec133_idx_save = 0;
		self.fRec123_perm.fill(0.0);
		self.fRec119_perm.fill(0.0);
		self.fRec115.fill(0.0);
		self.fRec115_idx = 0;
		self.fRec115_idx_save = 0;
		self.fRec117_perm.fill(0.0);
		self.fRec113_perm.fill(0.0);
		self.fRec108_perm.fill(0.0);
		self.fRec104.fill(0.0);
		self.fRec104_idx = 0;
		self.fRec104_idx_save = 0;
		self.fRec102_perm.fill(0.0);
		self.fRec143_perm.fill(0.0);
		self.fRec101_perm.fill(0.0);
		self.fRec144_perm.fill(0.0);
		self.fRec100_perm.fill(0.0);
		self.fRec99_perm.fill(0.0);
		self.fRec177_perm.fill(0.0);
		self.fRec173_perm.fill(0.0);
		self.fRec178_perm.fill(0.0);
		self.fRec180_perm.fill(0.0);
		self.fVec2_perm.fill(0.0);
		self.iRec181_perm.fill(0);
		self.fRec182_perm.fill(0.0);
		self.fRec183.fill(0.0);
		self.fRec183_idx = 0;
		self.fRec183_idx_save = 0;
		self.fRec185_perm.fill(0.0);
		self.iYec27_perm.fill(0);
		self.iRec186_perm.fill(0);
		self.fRec184_perm.fill(0.0);
		self.fYec28_perm.fill(0.0);
		self.fYec29_perm.fill(0.0);
		self.fRec179.fill(0.0);
		self.fRec179_idx = 0;
		self.fRec179_idx_save = 0;
		self.fRec169_perm.fill(0.0);
		self.fRec165_perm.fill(0.0);
		self.fRec161.fill(0.0);
		self.fRec161_idx = 0;
		self.fRec161_idx_save = 0;
		self.fRec163_perm.fill(0.0);
		self.fRec159_perm.fill(0.0);
		self.fRec154_perm.fill(0.0);
		self.fRec150.fill(0.0);
		self.fRec150_idx = 0;
		self.fRec150_idx_save = 0;
		self.fRec148_perm.fill(0.0);
		self.fRec188_perm.fill(0.0);
		self.fRec147_perm.fill(0.0);
		self.fRec189_perm.fill(0.0);
		self.fRec146_perm.fill(0.0);
		self.fRec145_perm.fill(0.0);
		self.fRec222_perm.fill(0.0);
		self.fRec218_perm.fill(0.0);
		self.fRec223_perm.fill(0.0);
		self.fRec225_perm.fill(0.0);
		self.fVec3_perm.fill(0.0);
		self.iRec226_perm.fill(0);
		self.fRec227_perm.fill(0.0);
		self.fRec228.fill(0.0);
		self.fRec228_idx = 0;
		self.fRec228_idx_save = 0;
		self.fRec230_perm.fill(0.0);
		self.iYec30_perm.fill(0);
		self.iRec231_perm.fill(0);
		self.fRec229_perm.fill(0.0);
		self.fYec31_perm.fill(0.0);
		self.fYec32_perm.fill(0.0);
		self.fRec224.fill(0.0);
		self.fRec224_idx = 0;
		self.fRec224_idx_save = 0;
		self.fRec214_perm.fill(0.0);
		self.fRec210_perm.fill(0.0);
		self.fRec206.fill(0.0);
		self.fRec206_idx = 0;
		self.fRec206_idx_save = 0;
		self.fRec208_perm.fill(0.0);
		self.fRec204_perm.fill(0.0);
		self.fRec199_perm.fill(0.0);
		self.fRec195.fill(0.0);
		self.fRec195_idx = 0;
		self.fRec195_idx_save = 0;
		self.fRec193_perm.fill(0.0);
		self.fRec233_perm.fill(0.0);
		self.fRec192_perm.fill(0.0);
		self.fRec234_perm.fill(0.0);
		self.fRec191_perm.fill(0.0);
		self.fRec190_perm.fill(0.0);
		self.fRec267_perm.fill(0.0);
		self.fRec263_perm.fill(0.0);
		self.fRec268_perm.fill(0.0);
		self.fRec270_perm.fill(0.0);
		self.fVec4_perm.fill(0.0);
		self.iRec271_perm.fill(0);
		self.fRec272_perm.fill(0.0);
		self.fRec273.fill(0.0);
		self.fRec273_idx = 0;
		self.fRec273_idx_save = 0;
		self.fRec275_perm.fill(0.0);
		self.iYec33_perm.fill(0);
		self.iRec276_perm.fill(0);
		self.fRec274_perm.fill(0.0);
		self.fYec34_perm.fill(0.0);
		self.fYec35_perm.fill(0.0);
		self.fRec269.fill(0.0);
		self.fRec269_idx = 0;
		self.fRec269_idx_save = 0;
		self.fRec259_perm.fill(0.0);
		self.fRec255_perm.fill(0.0);
		self.fRec251.fill(0.0);
		self.fRec251_idx = 0;
		self.fRec251_idx_save = 0;
		self.fRec253_perm.fill(0.0);
		self.fRec249_perm.fill(0.0);
		self.fRec244_perm.fill(0.0);
		self.fRec240.fill(0.0);
		self.fRec240_idx = 0;
		self.fRec240_idx_save = 0;
		self.fRec238_perm.fill(0.0);
		self.fRec278_perm.fill(0.0);
		self.fRec237_perm.fill(0.0);
		self.fRec279_perm.fill(0.0);
		self.fRec236_perm.fill(0.0);
		self.fRec235_perm.fill(0.0);
		self.fRec312_perm.fill(0.0);
		self.fRec308_perm.fill(0.0);
		self.fRec313_perm.fill(0.0);
		self.fRec315_perm.fill(0.0);
		self.fVec5_perm.fill(0.0);
		self.iRec316_perm.fill(0);
		self.fRec317_perm.fill(0.0);
		self.fRec318.fill(0.0);
		self.fRec318_idx = 0;
		self.fRec318_idx_save = 0;
		self.fRec320_perm.fill(0.0);
		self.iYec36_perm.fill(0);
		self.iRec321_perm.fill(0);
		self.fRec319_perm.fill(0.0);
		self.fYec37_perm.fill(0.0);
		self.fYec38_perm.fill(0.0);
		self.fRec314.fill(0.0);
		self.fRec314_idx = 0;
		self.fRec314_idx_save = 0;
		self.fRec304_perm.fill(0.0);
		self.fRec300_perm.fill(0.0);
		self.fRec296.fill(0.0);
		self.fRec296_idx = 0;
		self.fRec296_idx_save = 0;
		self.fRec298_perm.fill(0.0);
		self.fRec294_perm.fill(0.0);
		self.fRec289_perm.fill(0.0);
		self.fRec285.fill(0.0);
		self.fRec285_idx = 0;
		self.fRec285_idx_save = 0;
		self.fRec283_perm.fill(0.0);
		self.fRec323_perm.fill(0.0);
		self.fRec282_perm.fill(0.0);
		self.fRec324_perm.fill(0.0);
		self.fRec281_perm.fill(0.0);
		self.fRec280_perm.fill(0.0);
		self.fRec326_perm.fill(0.0);
		self.fRec325.fill(0.0);
		self.fRec325_idx = 0;
		self.fRec325_idx_save = 0;
		self.fRec335_perm.fill(0.0);
		self.fRec342_perm.fill(0.0);
		self.fRec346_perm.fill(0.0);
		self.fRec350_perm.fill(0.0);
		self.fRec358_perm.fill(0.0);
		self.fRec362_perm.fill(0.0);
		self.fRec369_perm.fill(0.0);
		self.fYec39.fill(0.0);
		self.fYec39_idx = 0;
		self.fYec39_idx_save = 0;
		self.fYec40_perm.fill(0.0);
		self.fRec368_perm.fill(0.0);
		self.fRec366_perm.fill(0.0);
		self.fRec371_perm.fill(0.0);
		self.fYec41.fill(0.0);
		self.fYec41_idx = 0;
		self.fYec41_idx_save = 0;
		self.fYec42_perm.fill(0.0);
		self.fRec370_perm.fill(0.0);
		self.fRec367_perm.fill(0.0);
		self.fRec372_perm.fill(0.0);
		self.fYec43.fill(0.0);
		self.fYec43_idx = 0;
		self.fYec43_idx_save = 0;
		self.fYec44_perm.fill(0.0);
		self.fRec365_perm.fill(0.0);
		self.fRec363_perm.fill(0.0);
		self.fRec374_perm.fill(0.0);
		self.fYec45.fill(0.0);
		self.fYec45_idx = 0;
		self.fYec45_idx_save = 0;
		self.fYec46_perm.fill(0.0);
		self.fRec373_perm.fill(0.0);
		self.fRec364_perm.fill(0.0);
		self.fYec47.fill(0.0);
		self.fYec47_idx = 0;
		self.fYec47_idx_save = 0;
		self.fYec48_perm.fill(0.0);
		self.fRec361_perm.fill(0.0);
		self.fRec359_perm.fill(0.0);
		self.fRec376_perm.fill(0.0);
		self.fYec49.fill(0.0);
		self.fYec49_idx = 0;
		self.fYec49_idx_save = 0;
		self.fYec50_perm.fill(0.0);
		self.fRec375_perm.fill(0.0);
		self.fRec360_perm.fill(0.0);
		self.fYec51.fill(0.0);
		self.fYec51_idx = 0;
		self.fYec51_idx_save = 0;
		self.fYec52_perm.fill(0.0);
		self.fRec357_perm.fill(0.0);
		self.fRec355_perm.fill(0.0);
		self.fRec378_perm.fill(0.0);
		self.fYec53.fill(0.0);
		self.fYec53_idx = 0;
		self.fYec53_idx_save = 0;
		self.fYec54_perm.fill(0.0);
		self.fRec377_perm.fill(0.0);
		self.fRec356_perm.fill(0.0);
		self.fRec379_perm.fill(0.0);
		self.fYec55.fill(0.0);
		self.fYec55_idx = 0;
		self.fYec55_idx_save = 0;
		self.fYec56_perm.fill(0.0);
		self.fRec354_perm.fill(0.0);
		self.fRec352_perm.fill(0.0);
		self.fRec381_perm.fill(0.0);
		self.fYec57.fill(0.0);
		self.fYec57_idx = 0;
		self.fYec57_idx_save = 0;
		self.fYec58_perm.fill(0.0);
		self.fRec380_perm.fill(0.0);
		self.fRec353_perm.fill(0.0);
		self.fRec382_perm.fill(0.0);
		self.fYec59.fill(0.0);
		self.fYec59_idx = 0;
		self.fYec59_idx_save = 0;
		self.fYec60.fill(0.0);
		self.fYec60_idx = 0;
		self.fYec60_idx_save = 0;
		self.fYec61_perm.fill(0.0);
		self.fRec351_perm.fill(0.0);
		self.fRec384_perm.fill(0.0);
		self.fYec62.fill(0.0);
		self.fYec62_idx = 0;
		self.fYec62_idx_save = 0;
		self.fYec63.fill(0.0);
		self.fYec63_idx = 0;
		self.fYec63_idx_save = 0;
		self.fYec64_perm.fill(0.0);
		self.fRec383_perm.fill(0.0);
		self.fYec65.fill(0.0);
		self.fYec65_idx = 0;
		self.fYec65_idx_save = 0;
		self.fYec66_perm.fill(0.0);
		self.fRec349_perm.fill(0.0);
		self.fRec347_perm.fill(0.0);
		self.fRec386_perm.fill(0.0);
		self.fYec67.fill(0.0);
		self.fYec67_idx = 0;
		self.fYec67_idx_save = 0;
		self.fYec68_perm.fill(0.0);
		self.fRec385_perm.fill(0.0);
		self.fRec348_perm.fill(0.0);
		self.fYec69.fill(0.0);
		self.fYec69_idx = 0;
		self.fYec69_idx_save = 0;
		self.fYec70_perm.fill(0.0);
		self.fRec345_perm.fill(0.0);
		self.fRec343_perm.fill(0.0);
		self.fYec71.fill(0.0);
		self.fYec71_idx = 0;
		self.fYec71_idx_save = 0;
		self.fYec72_perm.fill(0.0);
		self.fRec387_perm.fill(0.0);
		self.fRec344_perm.fill(0.0);
		self.fYec73.fill(0.0);
		self.fYec73_idx = 0;
		self.fYec73_idx_save = 0;
		self.fYec74_perm.fill(0.0);
		self.fRec341_perm.fill(0.0);
		self.fRec339_perm.fill(0.0);
		self.fRec389_perm.fill(0.0);
		self.fYec75.fill(0.0);
		self.fYec75_idx = 0;
		self.fYec75_idx_save = 0;
		self.fYec76_perm.fill(0.0);
		self.fRec388_perm.fill(0.0);
		self.fRec340_perm.fill(0.0);
		self.fYec77.fill(0.0);
		self.fYec77_idx = 0;
		self.fYec77_idx_save = 0;
		self.fYec78_perm.fill(0.0);
		self.fRec338_perm.fill(0.0);
		self.fRec336_perm.fill(0.0);
		self.fYec79.fill(0.0);
		self.fYec79_idx = 0;
		self.fYec79_idx_save = 0;
		self.fYec80_perm.fill(0.0);
		self.fRec390_perm.fill(0.0);
		self.fRec337_perm.fill(0.0);
		self.fYec81.fill(0.0);
		self.fYec81_idx = 0;
		self.fYec81_idx_save = 0;
		self.fYec82_perm.fill(0.0);
		self.fRec334_perm.fill(0.0);
		self.fRec332_perm.fill(0.0);
		self.fRec392_perm.fill(0.0);
		self.fYec83.fill(0.0);
		self.fYec83_idx = 0;
		self.fYec83_idx_save = 0;
		self.fYec84_perm.fill(0.0);
		self.fRec391_perm.fill(0.0);
		self.fRec333_perm.fill(0.0);
		self.fRec393_perm.fill(0.0);
		self.fYec85.fill(0.0);
		self.fYec85_idx = 0;
		self.fYec85_idx_save = 0;
		self.fYec86.fill(0.0);
		self.fYec86_idx = 0;
		self.fYec86_idx_save = 0;
		self.fYec87_perm.fill(0.0);
		self.fRec331_perm.fill(0.0);
		self.fRec330_perm.fill(0.0);
		self.fRec329_perm.fill(0.0);
		self.fRec328_perm.fill(0.0);
		self.fRec327_perm.fill(0.0);
		self.fRec399_perm.fill(0.0);
		self.fRec398_perm.fill(0.0);
		self.fRec397_perm.fill(0.0);
		self.fYec88_perm.fill(0.0);
		self.fRec396_perm.fill(0.0);
		self.fRec395_perm.fill(0.0);
		self.fRec394_perm.fill(0.0);
		self.fRec402_perm.fill(0.0);
		self.fRec401_perm.fill(0.0);
		self.fRec400_perm.fill(0.0);
		self.fYec89.fill(0.0);
		self.fYec89_idx = 0;
		self.fYec89_idx_save = 0;
		self.fRec18_perm.fill(0.0);
		self.fRec409_perm.fill(0.0);
		self.fYec90.fill(0.0);
		self.fYec90_idx = 0;
		self.fYec90_idx_save = 0;
		self.fYec91.fill(0.0);
		self.fYec91_idx = 0;
		self.fYec91_idx_save = 0;
		self.fYec92_perm.fill(0.0);
		self.fRec408_perm.fill(0.0);
		self.fRec407_perm.fill(0.0);
		self.fRec406_perm.fill(0.0);
		self.fRec405_perm.fill(0.0);
		self.fRec404_perm.fill(0.0);
		self.fRec415_perm.fill(0.0);
		self.fRec414_perm.fill(0.0);
		self.fRec413_perm.fill(0.0);
		self.fYec93_perm.fill(0.0);
		self.fRec412_perm.fill(0.0);
		self.fRec411_perm.fill(0.0);
		self.fRec410_perm.fill(0.0);
		self.fRec418_perm.fill(0.0);
		self.fRec417_perm.fill(0.0);
		self.fRec416_perm.fill(0.0);
		self.fYec94.fill(0.0);
		self.fYec94_idx = 0;
		self.fYec94_idx_save = 0;
		self.fRec403_perm.fill(0.0);
		self.fYec95.fill(0.0);
		self.fYec95_idx = 0;
		self.fYec95_idx_save = 0;
		self.fYec96_perm.fill(0.0);
		self.fRec16_perm.fill(0.0);
		self.fRec14_perm.fill(0.0);
		self.fRec420_perm.fill(0.0);
		self.fYec97.fill(0.0);
		self.fYec97_idx = 0;
		self.fYec97_idx_save = 0;
		self.fYec98_perm.fill(0.0);
		self.fRec419_perm.fill(0.0);
		self.fRec15_perm.fill(0.0);
		self.fYec99.fill(0.0);
		self.fYec99_idx = 0;
		self.fYec99_idx_save = 0;
		self.fYec100_perm.fill(0.0);
		self.fRec12_perm.fill(0.0);
		self.fRec10_perm.fill(0.0);
		self.fYec101.fill(0.0);
		self.fYec101_idx = 0;
		self.fYec101_idx_save = 0;
		self.fYec102_perm.fill(0.0);
		self.fRec421_perm.fill(0.0);
		self.fRec11_perm.fill(0.0);
		self.fYec103.fill(0.0);
		self.fYec103_idx = 0;
		self.fYec103_idx_save = 0;
		self.fYec104_perm.fill(0.0);
		self.fRec8_perm.fill(0.0);
		self.fRec6_perm.fill(0.0);
		self.fRec423_perm.fill(0.0);
		self.fYec105.fill(0.0);
		self.fYec105_idx = 0;
		self.fYec105_idx_save = 0;
		self.fYec106_perm.fill(0.0);
		self.fRec422_perm.fill(0.0);
		self.fRec7_perm.fill(0.0);
		self.fRec424_perm.fill(0.0);
		self.fYec107.fill(0.0);
		self.fYec107_idx = 0;
		self.fYec107_idx_save = 0;
		self.fYec108_perm.fill(0.0);
		self.fRec5_perm.fill(0.0);
		self.fRec3_perm.fill(0.0);
		self.fYec109.fill(0.0);
		self.fYec109_idx = 0;
		self.fYec109_idx_save = 0;
		self.fYec110_perm.fill(0.0);
		self.fRec425_perm.fill(0.0);
		self.fRec4_perm.fill(0.0);
		self.fRec1_perm.fill(0.0);
		self.fRec2_perm.fill(0.0);
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		self.fConst0 = F32::min(1.92e+05, F32::max(1.0, (self.fSampleRate) as F32));
		self.fConst1 = 44.1 / self.fConst0;
		self.fConst2 = 1.0 - self.fConst1;
		self.fConst3 = 6.2831855 / self.fConst0;
		self.fConst4 = 1.0 / self.fConst0;
		self.fConst5 = 0.5 * self.fConst0;
		self.fConst6 = 0.25 * self.fConst0;
		self.fConst7 = 3.1415927 / self.fConst0;
		self.fConst8 = 0.00882353 * self.fConst0;
		self.fConst9 = 0.00073529413 * self.fConst0;
		self.fConst10 = 15.707963 / self.fConst0;
		self.fConst11 = F32::exp(0.0 - 1e+04 / self.fConst0);
		self.fConst12 = 1.0 - self.fConst11;
		self.iConst13 = (0.1 * self.fConst0) as i32;
		self.fConst14 = F32::exp(0.0 - 5e+01 / self.fConst0);
		self.fConst15 = 0.002 * self.fConst0;
		self.fConst16 = F32::exp(0.0 - 1e+01 / self.fConst0);
		self.fConst17 = 1.3333334 / self.fConst0;
		self.fConst18 = 1e+01 * self.fConst0;
		let mut fConst19: F32 = F32::tan(25132.742 / self.fConst0);
		self.fConst20 = 1.0 / fConst19;
		self.fConst21 = 1.0 - self.fConst20;
		let mut fConst22: F32 = self.fConst20 + 1.0;
		self.fConst23 = 1.0 / fConst22;
		self.fConst24 = 0.0 - 1.0 / (fConst19 * fConst22);
		let mut fConst25: F32 = mydsp_faustpower2_f(fConst19);
		self.fConst26 = 1.0 / fConst25;
		self.fConst27 = 2.0 * (1.0 - self.fConst26);
		self.fConst28 = (self.fConst20 + -1.618034) / fConst19 + 1.0;
		self.fConst29 = 1.0 / ((self.fConst20 + 1.618034) / fConst19 + 1.0);
		self.fConst30 = (self.fConst20 + -0.618034) / fConst19 + 1.0;
		self.fConst31 = 1.0 / ((self.fConst20 + 0.618034) / fConst19 + 1.0);
		self.fConst32 = 0.0 - 2.0 / fConst25;
		let mut fConst33: F32 = F32::tan(1382.3008 / self.fConst0);
		let mut fConst34: F32 = mydsp_faustpower2_f(fConst33);
		self.fConst35 = 1.0 / fConst34;
		self.fConst36 = 2.0 * (1.0 - self.fConst35);
		let mut fConst37: F32 = 1.0 / fConst33;
		self.fConst38 = (fConst37 + -1.618034) / fConst33 + 1.0;
		self.fConst39 = 1.0 / ((fConst37 + 1.618034) / fConst33 + 1.0);
		let mut fConst40: F32 = fConst37 + 1.0;
		self.fConst41 = 1.0 - fConst37;
		self.fConst42 = self.fConst41 / fConst40;
		self.fConst43 = 1.0 / (fConst33 * fConst40);
		self.fConst44 = 0.0 - self.fConst43;
		self.fConst45 = (fConst37 + -1.618034) / fConst33 + 1.0;
		self.fConst46 = 1.0 / ((fConst37 + 1.618034) / fConst33 + 1.0);
		self.fConst47 = (fConst37 + -0.618034) / fConst33 + 1.0;
		self.fConst48 = 1.0 / ((fConst37 + 0.618034) / fConst33 + 1.0);
		self.fConst49 = 0.0 - 2.0 / fConst34;
		self.fConst50 = 1.0 / fConst40;
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
		ui_interface.declare(None, "3", "");
		ui_interface.open_vertical_box("0");
		ui_interface.declare(Some(ParamIndex(1)), "0", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(1), 6e+01, 0.0, 127.0, 0.001);
		ui_interface.declare(Some(ParamIndex(2)), "1", "");
		ui_interface.add_horizontal_slider("volume", ParamIndex(2), 0.0, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "3", "");
		ui_interface.open_vertical_box("1");
		ui_interface.declare(Some(ParamIndex(3)), "0", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(3), 6e+01, 0.0, 127.0, 0.001);
		ui_interface.declare(Some(ParamIndex(4)), "1", "");
		ui_interface.add_horizontal_slider("volume", ParamIndex(4), 0.0, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "3", "");
		ui_interface.open_vertical_box("2");
		ui_interface.declare(Some(ParamIndex(5)), "0", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(5), 6e+01, 0.0, 127.0, 0.001);
		ui_interface.declare(Some(ParamIndex(6)), "1", "");
		ui_interface.add_horizontal_slider("volume", ParamIndex(6), 0.0, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "3", "");
		ui_interface.open_vertical_box("3");
		ui_interface.declare(Some(ParamIndex(7)), "0", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(7), 6e+01, 0.0, 127.0, 0.001);
		ui_interface.declare(Some(ParamIndex(8)), "1", "");
		ui_interface.add_horizontal_slider("volume", ParamIndex(8), 0.0, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.close_box();
		ui_interface.declare(None, "1", "");
		ui_interface.open_horizontal_box("pluck");
		ui_interface.declare(Some(ParamIndex(9)), "2", "");
		ui_interface.add_horizontal_slider("mute", ParamIndex(9), 1.0, 0.9, 1.0, 0.001);
		ui_interface.declare(None, "3", "");
		ui_interface.open_vertical_box("0");
		ui_interface.declare(Some(ParamIndex(10)), "0", "");
		ui_interface.add_button("gate", ParamIndex(10));
		ui_interface.declare(Some(ParamIndex(11)), "1", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(11), 8e+01, 0.0, 127.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "3", "");
		ui_interface.open_vertical_box("1");
		ui_interface.declare(Some(ParamIndex(12)), "0", "");
		ui_interface.add_button("gate", ParamIndex(12));
		ui_interface.declare(Some(ParamIndex(13)), "1", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(13), 8e+01, 0.0, 127.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "3", "");
		ui_interface.open_vertical_box("2");
		ui_interface.declare(Some(ParamIndex(14)), "0", "");
		ui_interface.add_button("gate", ParamIndex(14));
		ui_interface.declare(Some(ParamIndex(15)), "1", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(15), 8e+01, 0.0, 127.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "3", "");
		ui_interface.open_vertical_box("3");
		ui_interface.declare(Some(ParamIndex(16)), "0", "");
		ui_interface.add_button("gate", ParamIndex(16));
		ui_interface.declare(Some(ParamIndex(17)), "1", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(17), 8e+01, 0.0, 127.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "3", "");
		ui_interface.open_vertical_box("4");
		ui_interface.declare(Some(ParamIndex(18)), "0", "");
		ui_interface.add_button("gate", ParamIndex(18));
		ui_interface.declare(Some(ParamIndex(19)), "1", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(19), 8e+01, 0.0, 127.0, 0.001);
		ui_interface.close_box();
		ui_interface.close_box();
		ui_interface.declare(None, "2", "");
		ui_interface.open_horizontal_box("drone");
		ui_interface.declare(Some(ParamIndex(20)), "0", "");
		ui_interface.add_horizontal_slider("detune", ParamIndex(20), 0.1, 0.0, 0.3, 0.001);
		ui_interface.declare(None, "1", "");
		ui_interface.open_vertical_box("0");
		ui_interface.declare(Some(ParamIndex(21)), "0", "");
		ui_interface.add_horizontal_slider("volume", ParamIndex(21), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(22)), "1", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(22), 6e+01, 0.0, 127.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "1", "");
		ui_interface.open_vertical_box("1");
		ui_interface.declare(Some(ParamIndex(23)), "0", "");
		ui_interface.add_horizontal_slider("volume", ParamIndex(23), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(24)), "1", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(24), 6e+01, 0.0, 127.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "1", "");
		ui_interface.open_vertical_box("2");
		ui_interface.declare(Some(ParamIndex(25)), "0", "");
		ui_interface.add_horizontal_slider("volume", ParamIndex(25), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(26)), "1", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(26), 6e+01, 0.0, 127.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "1", "");
		ui_interface.open_vertical_box("3");
		ui_interface.declare(Some(ParamIndex(27)), "0", "");
		ui_interface.add_horizontal_slider("volume", ParamIndex(27), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(28)), "1", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(28), 6e+01, 0.0, 127.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(Some(ParamIndex(29)), "1", "");
		ui_interface.add_horizontal_slider("trumpet", ParamIndex(29), 0.0, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "2", "");
		ui_interface.open_horizontal_box("fx");
		ui_interface.declare(None, "0", "");
		ui_interface.open_vertical_box("echo");
		ui_interface.declare(Some(ParamIndex(30)), "0", "");
		ui_interface.declare(Some(ParamIndex(30)), "scale", "log");
		ui_interface.add_horizontal_slider("duration", ParamIndex(30), 0.3, 0.01, 3.0, 0.001);
		ui_interface.declare(Some(ParamIndex(31)), "0", "");
		ui_interface.add_horizontal_slider("mix", ParamIndex(31), 1.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(32)), "1", "");
		ui_interface.add_horizontal_slider("feedback", ParamIndex(32), 0.3, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "1", "");
		ui_interface.open_vertical_box("reverb");
		ui_interface.declare(Some(ParamIndex(33)), "0", "");
		ui_interface.add_horizontal_slider("mix", ParamIndex(33), 0.11, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(34)), "1", "");
		ui_interface.add_horizontal_slider("time", ParamIndex(34), 3.5, 0.1, 6e+01, 0.001);
		ui_interface.declare(Some(ParamIndex(35)), "2", "");
		ui_interface.add_horizontal_slider("damp", ParamIndex(35), 0.88, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(36)), "3", "");
		ui_interface.add_horizontal_slider("size", ParamIndex(36), 5.0, 0.5, 5.0, 0.001);
		ui_interface.declare(Some(ParamIndex(37)), "4", "");
		ui_interface.add_horizontal_slider("early_diff", ParamIndex(37), 0.75, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(38)), "5", "");
		ui_interface.add_horizontal_slider("mod_depth", ParamIndex(38), 0.98, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(39)), "6", "");
		ui_interface.add_horizontal_slider("mod_freq", ParamIndex(39), 0.6, 0.0, 1e+01, 0.001);
		ui_interface.close_box();
		ui_interface.close_box();
		ui_interface.declare(None, "3", "");
		ui_interface.open_vertical_box("mix");
		ui_interface.declare(Some(ParamIndex(40)), "0", "");
		ui_interface.add_horizontal_slider("master", ParamIndex(40), 1.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(41)), "1", "");
		ui_interface.add_horizontal_slider("drone", ParamIndex(41), 1.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(42)), "2", "");
		ui_interface.add_horizontal_slider("lead", ParamIndex(42), 1.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(43)), "3", "");
		ui_interface.add_horizontal_slider("pluck", ParamIndex(43), 1.0, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "4", "");
		ui_interface.open_vertical_box("filter");
		ui_interface.declare(Some(ParamIndex(44)), "1", "");
		ui_interface.add_horizontal_slider("cutoffNote", ParamIndex(44), 0.0, -2e+01, 5e+01, 0.001);
		ui_interface.declare(Some(ParamIndex(45)), "2", "");
		ui_interface.add_horizontal_slider("res", ParamIndex(45), 0.0, 0.0, 0.99, 0.001);
		ui_interface.close_box();
		ui_interface.declare(Some(ParamIndex(46)), "5", "");
		ui_interface.add_horizontal_slider("pitchBend", ParamIndex(46), 0.0, -1.0, 1.0, 0.001);
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			10 => Some(self.fButton0),
			12 => Some(self.fButton1),
			14 => Some(self.fButton2),
			16 => Some(self.fButton3),
			18 => Some(self.fButton4),
			40 => Some(self.fHslider0),
			36 => Some(self.fHslider1),
			24 => Some(self.fHslider10),
			25 => Some(self.fHslider11),
			26 => Some(self.fHslider12),
			27 => Some(self.fHslider13),
			28 => Some(self.fHslider14),
			0 => Some(self.fHslider15),
			42 => Some(self.fHslider16),
			2 => Some(self.fHslider17),
			46 => Some(self.fHslider18),
			1 => Some(self.fHslider19),
			39 => Some(self.fHslider2),
			45 => Some(self.fHslider20),
			44 => Some(self.fHslider21),
			4 => Some(self.fHslider22),
			3 => Some(self.fHslider23),
			6 => Some(self.fHslider24),
			5 => Some(self.fHslider25),
			8 => Some(self.fHslider26),
			7 => Some(self.fHslider27),
			43 => Some(self.fHslider28),
			11 => Some(self.fHslider29),
			31 => Some(self.fHslider3),
			9 => Some(self.fHslider30),
			13 => Some(self.fHslider31),
			15 => Some(self.fHslider32),
			17 => Some(self.fHslider33),
			19 => Some(self.fHslider34),
			30 => Some(self.fHslider35),
			32 => Some(self.fHslider36),
			38 => Some(self.fHslider37),
			35 => Some(self.fHslider38),
			34 => Some(self.fHslider39),
			41 => Some(self.fHslider4),
			37 => Some(self.fHslider40),
			33 => Some(self.fHslider41),
			29 => Some(self.fHslider5),
			21 => Some(self.fHslider6),
			22 => Some(self.fHslider7),
			20 => Some(self.fHslider8),
			23 => Some(self.fHslider9),
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			10 => { self.fButton0 = value }
			12 => { self.fButton1 = value }
			14 => { self.fButton2 = value }
			16 => { self.fButton3 = value }
			18 => { self.fButton4 = value }
			40 => { self.fHslider0 = value }
			36 => { self.fHslider1 = value }
			24 => { self.fHslider10 = value }
			25 => { self.fHslider11 = value }
			26 => { self.fHslider12 = value }
			27 => { self.fHslider13 = value }
			28 => { self.fHslider14 = value }
			0 => { self.fHslider15 = value }
			42 => { self.fHslider16 = value }
			2 => { self.fHslider17 = value }
			46 => { self.fHslider18 = value }
			1 => { self.fHslider19 = value }
			39 => { self.fHslider2 = value }
			45 => { self.fHslider20 = value }
			44 => { self.fHslider21 = value }
			4 => { self.fHslider22 = value }
			3 => { self.fHslider23 = value }
			6 => { self.fHslider24 = value }
			5 => { self.fHslider25 = value }
			8 => { self.fHslider26 = value }
			7 => { self.fHslider27 = value }
			43 => { self.fHslider28 = value }
			11 => { self.fHslider29 = value }
			31 => { self.fHslider3 = value }
			9 => { self.fHslider30 = value }
			13 => { self.fHslider31 = value }
			15 => { self.fHslider32 = value }
			17 => { self.fHslider33 = value }
			19 => { self.fHslider34 = value }
			30 => { self.fHslider35 = value }
			32 => { self.fHslider36 = value }
			38 => { self.fHslider37 = value }
			35 => { self.fHslider38 = value }
			34 => { self.fHslider39 = value }
			41 => { self.fHslider4 = value }
			37 => { self.fHslider40 = value }
			33 => { self.fHslider41 = value }
			29 => { self.fHslider5 = value }
			21 => { self.fHslider6 = value }
			22 => { self.fHslider7 = value }
			20 => { self.fHslider8 = value }
			23 => { self.fHslider9 = value }
			_ => {}
		}
	}
	
	#[allow(non_snake_case)]
	#[allow(unused_mut)]
	fn compute(&mut self, count: i32, inputs: &[&[Self::T]], outputs: &mut[&mut[Self::T]]) {
		const vsize: i32 = 32;
		let (outputs0, outputs1) = if let [outputs0, outputs1, ..] = outputs {
			let outputs0 = outputs0[..count as usize].chunks_mut(vsize as usize);
			let outputs1 = outputs1[..count as usize].chunks_mut(vsize as usize);
			(outputs0, outputs1)
		} else {
			panic!("wrong number of outputs");
		};
		let mut fSlow0: F32 = self.fConst1 * self.fHslider0;
		let mut fRec0_tmp: [F32;36] = [0.0;36];
		let mut fSlow1: F32 = self.fHslider1;
		let mut iSlow2: i32 = unsafe { itbl0mydspSIG0[((115.0 * fSlow1) as i32) as usize] };
		let mut fSlow3: F32 = 0.0001 * (iSlow2) as F32;
		let mut iVec0_tmp: [i32;36] = [0;36];
		let mut iZec0: [i32;32] = [0;32];
		let mut fRec9_tmp: [F32;36] = [0.0;36];
		let mut iSlow4: i32 = unsafe { itbl0mydspSIG0[((215.0 * fSlow1) as i32) as usize] };
		let mut fSlow5: F32 = 0.0001 * (iSlow4) as F32;
		let mut fRec13_tmp: [F32;36] = [0.0;36];
		let mut iSlow6: i32 = unsafe { itbl0mydspSIG0[((55.0 * fSlow1) as i32) as usize] };
		let mut fSlow7: F32 = 0.0001 * (iSlow6) as F32;
		let mut fRec17_tmp: [F32;36] = [0.0;36];
		let mut fSlow8: F32 = self.fConst3 * self.fHslider2;
		let mut fSlow9: F32 = F32::cos(fSlow8);
		let mut fSlow10: F32 = F32::sin(fSlow8);
		let mut fRec19_tmp: [F32;36] = [0.0;36];
		let mut fRec20_tmp: [F32;36] = [0.0;36];
		let mut fSlow11: F32 = self.fConst1 * self.fHslider3;
		let mut fRec21_tmp: [F32;36] = [0.0;36];
		let mut fSlow12: F32 = self.fConst1 * self.fHslider4;
		let mut fRec22_tmp: [F32;36] = [0.0;36];
		let mut fSlow13: F32 = self.fConst1 * self.fHslider5;
		let mut fRec23_tmp: [F32;36] = [0.0;36];
		let mut fSlow14: F32 = self.fConst1 * self.fHslider6;
		let mut fRec24_tmp: [F32;36] = [0.0;36];
		let mut fSlow15: F32 = self.fHslider7;
		let mut fSlow16: F32 = self.fConst1 * unsafe { ftbl1mydspSIG1[(std::cmp::max(0, std::cmp::min((16.11811 * fSlow15 + 0.5) as i32, 2047))) as usize] };
		let mut fRec25_tmp: [F32;36] = [0.0;36];
		let mut fZec1: [F32;32] = [0.0;32];
		let mut fZec2: [F32;32] = [0.0;32];
		let mut fZec3: [F32;32] = [0.0;32];
		let mut fRec28_tmp: [F32;36] = [0.0;36];
		let mut fZec4: [F32;32] = [0.0;32];
		let mut fZec5: [F32;32] = [0.0;32];
		let mut fYec0_tmp: [F32;36] = [0.0;36];
		let mut fZec6: [F32;32] = [0.0;32];
		let mut iZec7: [i32;32] = [0;32];
		let mut fRec27_tmp: [F32;36] = [0.0;36];
		let mut fSlow17: F32 = self.fHslider8;
		let mut fSlow18: F32 = self.fConst1 * unsafe { ftbl1mydspSIG1[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow17 + fSlow15) + 0.5) as i32, 2047))) as usize] };
		let mut fRec29_tmp: [F32;36] = [0.0;36];
		let mut fZec8: [F32;32] = [0.0;32];
		let mut fZec9: [F32;32] = [0.0;32];
		let mut fZec10: [F32;32] = [0.0;32];
		let mut fRec31_tmp: [F32;36] = [0.0;36];
		let mut fYec2_tmp: [F32;36] = [0.0;36];
		let mut fZec11: [F32;32] = [0.0;32];
		let mut iZec12: [i32;32] = [0;32];
		let mut fZec13: [F32;32] = [0.0;32];
		let mut fRec30_tmp: [F32;36] = [0.0;36];
		let mut fSlow19: F32 = self.fConst1 * unsafe { ftbl1mydspSIG1[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow15 - fSlow17) + 0.5) as i32, 2047))) as usize] };
		let mut fRec32_tmp: [F32;36] = [0.0;36];
		let mut fZec14: [F32;32] = [0.0;32];
		let mut fZec15: [F32;32] = [0.0;32];
		let mut fZec16: [F32;32] = [0.0;32];
		let mut fRec34_tmp: [F32;36] = [0.0;36];
		let mut fYec4_tmp: [F32;36] = [0.0;36];
		let mut fZec17: [F32;32] = [0.0;32];
		let mut iZec18: [i32;32] = [0;32];
		let mut fZec19: [F32;32] = [0.0;32];
		let mut fRec33_tmp: [F32;36] = [0.0;36];
		let mut fSlow20: F32 = self.fConst1 * self.fHslider9;
		let mut fRec35_tmp: [F32;36] = [0.0;36];
		let mut fSlow21: F32 = self.fHslider10;
		let mut fSlow22: F32 = self.fConst1 * unsafe { ftbl1mydspSIG1[(std::cmp::max(0, std::cmp::min((16.11811 * fSlow21 + 0.5) as i32, 2047))) as usize] };
		let mut fRec36_tmp: [F32;36] = [0.0;36];
		let mut fZec20: [F32;32] = [0.0;32];
		let mut fZec21: [F32;32] = [0.0;32];
		let mut fZec22: [F32;32] = [0.0;32];
		let mut fRec38_tmp: [F32;36] = [0.0;36];
		let mut fYec6_tmp: [F32;36] = [0.0;36];
		let mut fZec23: [F32;32] = [0.0;32];
		let mut iZec24: [i32;32] = [0;32];
		let mut fZec25: [F32;32] = [0.0;32];
		let mut fRec37_tmp: [F32;36] = [0.0;36];
		let mut fSlow23: F32 = self.fConst1 * unsafe { ftbl1mydspSIG1[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow17 + fSlow21) + 0.5) as i32, 2047))) as usize] };
		let mut fRec39_tmp: [F32;36] = [0.0;36];
		let mut fZec26: [F32;32] = [0.0;32];
		let mut fZec27: [F32;32] = [0.0;32];
		let mut fZec28: [F32;32] = [0.0;32];
		let mut fRec41_tmp: [F32;36] = [0.0;36];
		let mut fZec29: [F32;32] = [0.0;32];
		let mut fZec30: [F32;32] = [0.0;32];
		let mut fYec8_tmp: [F32;36] = [0.0;36];
		let mut iZec31: [i32;32] = [0;32];
		let mut fRec40_tmp: [F32;36] = [0.0;36];
		let mut fSlow24: F32 = self.fConst1 * unsafe { ftbl1mydspSIG1[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow21 - fSlow17) + 0.5) as i32, 2047))) as usize] };
		let mut fRec42_tmp: [F32;36] = [0.0;36];
		let mut fZec32: [F32;32] = [0.0;32];
		let mut fZec33: [F32;32] = [0.0;32];
		let mut fZec34: [F32;32] = [0.0;32];
		let mut fRec44_tmp: [F32;36] = [0.0;36];
		let mut fYec10_tmp: [F32;36] = [0.0;36];
		let mut fZec35: [F32;32] = [0.0;32];
		let mut iZec36: [i32;32] = [0;32];
		let mut fZec37: [F32;32] = [0.0;32];
		let mut fRec43_tmp: [F32;36] = [0.0;36];
		let mut fSlow25: F32 = self.fConst1 * self.fHslider11;
		let mut fRec45_tmp: [F32;36] = [0.0;36];
		let mut fSlow26: F32 = self.fHslider12;
		let mut fSlow27: F32 = self.fConst1 * unsafe { ftbl1mydspSIG1[(std::cmp::max(0, std::cmp::min((16.11811 * fSlow26 + 0.5) as i32, 2047))) as usize] };
		let mut fRec46_tmp: [F32;36] = [0.0;36];
		let mut fZec38: [F32;32] = [0.0;32];
		let mut fZec39: [F32;32] = [0.0;32];
		let mut fZec40: [F32;32] = [0.0;32];
		let mut fRec48_tmp: [F32;36] = [0.0;36];
		let mut fYec12_tmp: [F32;36] = [0.0;36];
		let mut fZec41: [F32;32] = [0.0;32];
		let mut iZec42: [i32;32] = [0;32];
		let mut fZec43: [F32;32] = [0.0;32];
		let mut fRec47_tmp: [F32;36] = [0.0;36];
		let mut fSlow28: F32 = self.fConst1 * unsafe { ftbl1mydspSIG1[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow17 + fSlow26) + 0.5) as i32, 2047))) as usize] };
		let mut fRec49_tmp: [F32;36] = [0.0;36];
		let mut fZec44: [F32;32] = [0.0;32];
		let mut fZec45: [F32;32] = [0.0;32];
		let mut fZec46: [F32;32] = [0.0;32];
		let mut fRec51_tmp: [F32;36] = [0.0;36];
		let mut fZec47: [F32;32] = [0.0;32];
		let mut fZec48: [F32;32] = [0.0;32];
		let mut fYec14_tmp: [F32;36] = [0.0;36];
		let mut iZec49: [i32;32] = [0;32];
		let mut fRec50_tmp: [F32;36] = [0.0;36];
		let mut fSlow29: F32 = self.fConst1 * unsafe { ftbl1mydspSIG1[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow26 - fSlow17) + 0.5) as i32, 2047))) as usize] };
		let mut fRec52_tmp: [F32;36] = [0.0;36];
		let mut fZec50: [F32;32] = [0.0;32];
		let mut fZec51: [F32;32] = [0.0;32];
		let mut fZec52: [F32;32] = [0.0;32];
		let mut fRec54_tmp: [F32;36] = [0.0;36];
		let mut fYec16_tmp: [F32;36] = [0.0;36];
		let mut fZec53: [F32;32] = [0.0;32];
		let mut iZec54: [i32;32] = [0;32];
		let mut fZec55: [F32;32] = [0.0;32];
		let mut fRec53_tmp: [F32;36] = [0.0;36];
		let mut fSlow30: F32 = self.fConst1 * self.fHslider13;
		let mut fRec55_tmp: [F32;36] = [0.0;36];
		let mut fSlow31: F32 = self.fHslider14;
		let mut fSlow32: F32 = self.fConst1 * unsafe { ftbl1mydspSIG1[(std::cmp::max(0, std::cmp::min((16.11811 * fSlow31 + 0.5) as i32, 2047))) as usize] };
		let mut fRec56_tmp: [F32;36] = [0.0;36];
		let mut fZec56: [F32;32] = [0.0;32];
		let mut fZec57: [F32;32] = [0.0;32];
		let mut fZec58: [F32;32] = [0.0;32];
		let mut fRec58_tmp: [F32;36] = [0.0;36];
		let mut fYec18_tmp: [F32;36] = [0.0;36];
		let mut fZec59: [F32;32] = [0.0;32];
		let mut iZec60: [i32;32] = [0;32];
		let mut fZec61: [F32;32] = [0.0;32];
		let mut fRec57_tmp: [F32;36] = [0.0;36];
		let mut fSlow33: F32 = self.fConst1 * unsafe { ftbl1mydspSIG1[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow17 + fSlow31) + 0.5) as i32, 2047))) as usize] };
		let mut fRec59_tmp: [F32;36] = [0.0;36];
		let mut fZec62: [F32;32] = [0.0;32];
		let mut fZec63: [F32;32] = [0.0;32];
		let mut fZec64: [F32;32] = [0.0;32];
		let mut fRec61_tmp: [F32;36] = [0.0;36];
		let mut fZec65: [F32;32] = [0.0;32];
		let mut fZec66: [F32;32] = [0.0;32];
		let mut fYec20_tmp: [F32;36] = [0.0;36];
		let mut iZec67: [i32;32] = [0;32];
		let mut fRec60_tmp: [F32;36] = [0.0;36];
		let mut fSlow34: F32 = self.fConst1 * unsafe { ftbl1mydspSIG1[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow31 - fSlow17) + 0.5) as i32, 2047))) as usize] };
		let mut fRec62_tmp: [F32;36] = [0.0;36];
		let mut fZec68: [F32;32] = [0.0;32];
		let mut fZec69: [F32;32] = [0.0;32];
		let mut fZec70: [F32;32] = [0.0;32];
		let mut fRec64_tmp: [F32;36] = [0.0;36];
		let mut fYec22_tmp: [F32;36] = [0.0;36];
		let mut fZec71: [F32;32] = [0.0;32];
		let mut iZec72: [i32;32] = [0;32];
		let mut fZec73: [F32;32] = [0.0;32];
		let mut fRec63_tmp: [F32;36] = [0.0;36];
		let mut fSlow35: F32 = self.fConst1 * self.fHslider15;
		let mut fRec65_tmp: [F32;36] = [0.0;36];
		let mut fSlow36: F32 = self.fConst1 * self.fHslider16;
		let mut fRec66_tmp: [F32;36] = [0.0;36];
		let mut fSlow37: F32 = self.fConst1 * self.fHslider17;
		let mut fRec69_tmp: [F32;36] = [0.0;36];
		let mut fSlow38: F32 = self.fConst1 * self.fHslider18;
		let mut fRec73_tmp: [F32;36] = [0.0;36];
		let mut fSlow39: F32 = self.fHslider19;
		let mut fRec72_tmp: [F32;36] = [0.0;36];
		let mut fZec74: [F32;32] = [0.0;32];
		let mut fZec75: [F32;32] = [0.0;32];
		let mut fZec76: [F32;32] = [0.0;32];
		let mut iZec77: [i32;32] = [0;32];
		let mut fRec70_tmp: [F32;36] = [0.0;36];
		let mut fRec71: [F32;32] = [0.0;32];
		let mut fSlow40: F32 = self.fConst1 * self.fHslider20;
		let mut fRec74_tmp: [F32;36] = [0.0;36];
		let mut fSlow41: F32 = self.fConst1 * self.fHslider21;
		let mut fRec76_tmp: [F32;36] = [0.0;36];
		let mut fZec78: [F32;32] = [0.0;32];
		let mut fRec75_tmp: [F32;36] = [0.0;36];
		let mut fZec79: [F32;32] = [0.0;32];
		let mut fZec80: [F32;32] = [0.0;32];
		let mut fZec81: [F32;32] = [0.0;32];
		let mut fZec82: [F32;32] = [0.0;32];
		let mut fZec83: [F32;32] = [0.0;32];
		let mut fZec84: [F32;32] = [0.0;32];
		let mut fZec85: [F32;32] = [0.0;32];
		let mut fZec86: [F32;32] = [0.0;32];
		let mut fZec87: [F32;32] = [0.0;32];
		let mut fRec68_tmp: [F32;36] = [0.0;36];
		let mut fZec88: [F32;32] = [0.0;32];
		let mut fZec89: [F32;32] = [0.0;32];
		let mut fZec90: [F32;32] = [0.0;32];
		let mut fRec67_tmp: [F32;36] = [0.0;36];
		let mut fSlow42: F32 = self.fConst1 * self.fHslider22;
		let mut fRec79_tmp: [F32;36] = [0.0;36];
		let mut fSlow43: F32 = self.fHslider23;
		let mut fRec82_tmp: [F32;36] = [0.0;36];
		let mut fZec91: [F32;32] = [0.0;32];
		let mut fZec92: [F32;32] = [0.0;32];
		let mut fZec93: [F32;32] = [0.0;32];
		let mut fZec94: [F32;32] = [0.0;32];
		let mut iZec95: [i32;32] = [0;32];
		let mut fRec80_tmp: [F32;36] = [0.0;36];
		let mut fRec81: [F32;32] = [0.0;32];
		let mut fRec83_tmp: [F32;36] = [0.0;36];
		let mut fZec96: [F32;32] = [0.0;32];
		let mut fZec97: [F32;32] = [0.0;32];
		let mut fZec98: [F32;32] = [0.0;32];
		let mut fZec99: [F32;32] = [0.0;32];
		let mut fRec78_tmp: [F32;36] = [0.0;36];
		let mut fZec100: [F32;32] = [0.0;32];
		let mut fRec77_tmp: [F32;36] = [0.0;36];
		let mut fSlow44: F32 = self.fConst1 * self.fHslider24;
		let mut fRec86_tmp: [F32;36] = [0.0;36];
		let mut fSlow45: F32 = self.fHslider25;
		let mut fRec89_tmp: [F32;36] = [0.0;36];
		let mut fZec101: [F32;32] = [0.0;32];
		let mut fZec102: [F32;32] = [0.0;32];
		let mut fZec103: [F32;32] = [0.0;32];
		let mut iZec104: [i32;32] = [0;32];
		let mut fRec87_tmp: [F32;36] = [0.0;36];
		let mut fRec88: [F32;32] = [0.0;32];
		let mut fRec90_tmp: [F32;36] = [0.0;36];
		let mut fZec105: [F32;32] = [0.0;32];
		let mut fZec106: [F32;32] = [0.0;32];
		let mut fZec107: [F32;32] = [0.0;32];
		let mut fZec108: [F32;32] = [0.0;32];
		let mut fRec85_tmp: [F32;36] = [0.0;36];
		let mut fZec109: [F32;32] = [0.0;32];
		let mut fRec84_tmp: [F32;36] = [0.0;36];
		let mut fSlow46: F32 = self.fConst1 * self.fHslider26;
		let mut fRec93_tmp: [F32;36] = [0.0;36];
		let mut fSlow47: F32 = self.fHslider27;
		let mut fRec96_tmp: [F32;36] = [0.0;36];
		let mut fZec110: [F32;32] = [0.0;32];
		let mut fZec111: [F32;32] = [0.0;32];
		let mut fZec112: [F32;32] = [0.0;32];
		let mut iZec113: [i32;32] = [0;32];
		let mut fRec94_tmp: [F32;36] = [0.0;36];
		let mut fRec95: [F32;32] = [0.0;32];
		let mut fRec97_tmp: [F32;36] = [0.0;36];
		let mut fZec114: [F32;32] = [0.0;32];
		let mut fZec115: [F32;32] = [0.0;32];
		let mut fZec116: [F32;32] = [0.0;32];
		let mut fZec117: [F32;32] = [0.0;32];
		let mut fRec92_tmp: [F32;36] = [0.0;36];
		let mut fZec118: [F32;32] = [0.0;32];
		let mut fRec91_tmp: [F32;36] = [0.0;36];
		let mut fSlow48: F32 = self.fConst1 * self.fHslider28;
		let mut fRec98_tmp: [F32;36] = [0.0;36];
		let mut fRec112: [F32;32] = [0.0;32];
		let mut fSlow49: F32 = self.fHslider29;
		let mut fRec131_tmp: [F32;36] = [0.0;36];
		let mut fZec119: [F32;32] = [0.0;32];
		let mut fZec120: [F32;32] = [0.0;32];
		let mut iZec121: [i32;32] = [0;32];
		let mut iZec122: [i32;32] = [0;32];
		let mut iZec123: [i32;32] = [0;32];
		let mut fZec124: [F32;32] = [0.0;32];
		let mut fZec125: [F32;32] = [0.0;32];
		let mut fZec126: [F32;32] = [0.0;32];
		let mut fZec127: [F32;32] = [0.0;32];
		let mut fZec128: [F32;32] = [0.0;32];
		let mut fZec129: [F32;32] = [0.0;32];
		let mut fZec130: [F32;32] = [0.0;32];
		let mut fZec131: [F32;32] = [0.0;32];
		let mut iZec132: [i32;32] = [0;32];
		let mut iZec133: [i32;32] = [0;32];
		let mut fZec134: [F32;32] = [0.0;32];
		let mut fZec135: [F32;32] = [0.0;32];
		let mut iZec136: [i32;32] = [0;32];
		let mut iZec137: [i32;32] = [0;32];
		let mut fZec138: [F32;32] = [0.0;32];
		let mut fZec139: [F32;32] = [0.0;32];
		let mut fZec140: [F32;32] = [0.0;32];
		let mut iZec141: [i32;32] = [0;32];
		let mut iZec142: [i32;32] = [0;32];
		let mut fZec143: [F32;32] = [0.0;32];
		let mut fZec144: [F32;32] = [0.0;32];
		let mut fZec145: [F32;32] = [0.0;32];
		let mut fZec146: [F32;32] = [0.0;32];
		let mut fZec147: [F32;32] = [0.0;32];
		let mut iZec148: [i32;32] = [0;32];
		let mut iZec149: [i32;32] = [0;32];
		let mut fRec127_tmp: [F32;36] = [0.0;36];
		let mut fRec132_tmp: [F32;36] = [0.0;36];
		let mut fRec128: [F32;32] = [0.0;32];
		let mut iRec135_tmp: [i32;36] = [0;36];
		let mut fZec150: [F32;32] = [0.0;32];
		let mut fZec151: [F32;32] = [0.0;32];
		let mut fZec152: [F32;32] = [0.0;32];
		let mut fZec153: [F32;32] = [0.0;32];
		let mut fRec134_tmp: [F32;36] = [0.0;36];
		let mut fSlow50: F32 = self.fButton0;
		let mut fVec1_tmp: [F32;36] = [0.0;36];
		let mut iRec136_tmp: [i32;36] = [0;36];
		let mut fRec137_tmp: [F32;36] = [0.0;36];
		let mut fRec140_tmp: [F32;36] = [0.0;36];
		let mut iYec24_tmp: [i32;36] = [0;36];
		let mut iRec141_tmp: [i32;36] = [0;36];
		let mut fZec154: [F32;32] = [0.0;32];
		let mut fZec155: [F32;32] = [0.0;32];
		let mut fRec139_tmp: [F32;36] = [0.0;36];
		let mut fZec156: [F32;32] = [0.0;32];
		let mut fZec157: [F32;32] = [0.0;32];
		let mut fZec158: [F32;32] = [0.0;32];
		let mut fZec159: [F32;32] = [0.0;32];
		let mut fZec160: [F32;32] = [0.0;32];
		let mut fYec25_tmp: [F32;36] = [0.0;36];
		let mut fZec161: [F32;32] = [0.0;32];
		let mut fZec162: [F32;32] = [0.0;32];
		let mut fYec26_tmp: [F32;36] = [0.0;36];
		let mut fRec129: [F32;32] = [0.0;32];
		let mut fRec130: [F32;32] = [0.0;32];
		let mut fRec123_tmp: [F32;36] = [0.0;36];
		let mut fRec124: [F32;32] = [0.0;32];
		let mut fRec125: [F32;32] = [0.0;32];
		let mut fRec126: [F32;32] = [0.0;32];
		let mut fRec119_tmp: [F32;36] = [0.0;36];
		let mut fRec120: [F32;32] = [0.0;32];
		let mut fRec121: [F32;32] = [0.0;32];
		let mut fRec122: [F32;32] = [0.0;32];
		let mut fRec116: [F32;32] = [0.0;32];
		let mut fRec117_tmp: [F32;36] = [0.0;36];
		let mut fRec118: [F32;32] = [0.0;32];
		let mut fSlow51: F32 = self.fHslider30;
		let mut fRec113_tmp: [F32;36] = [0.0;36];
		let mut fRec114: [F32;32] = [0.0;32];
		let mut fRec108_tmp: [F32;36] = [0.0;36];
		let mut fRec109: [F32;32] = [0.0;32];
		let mut fRec110: [F32;32] = [0.0;32];
		let mut fRec111: [F32;32] = [0.0;32];
		let mut fRec105: [F32;32] = [0.0;32];
		let mut fRec106: [F32;32] = [0.0;32];
		let mut fRec107: [F32;32] = [0.0;32];
		let mut fRec102_tmp: [F32;36] = [0.0;36];
		let mut fRec103: [F32;32] = [0.0;32];
		let mut fZec163: [F32;32] = [0.0;32];
		let mut fZec164: [F32;32] = [0.0;32];
		let mut fRec143_tmp: [F32;36] = [0.0;36];
		let mut fRec142: [F32;32] = [0.0;32];
		let mut fRec101_tmp: [F32;36] = [0.0;36];
		let mut fRec144_tmp: [F32;36] = [0.0;36];
		let mut fZec165: [F32;32] = [0.0;32];
		let mut fZec166: [F32;32] = [0.0;32];
		let mut fZec167: [F32;32] = [0.0;32];
		let mut fZec168: [F32;32] = [0.0;32];
		let mut fRec100_tmp: [F32;36] = [0.0;36];
		let mut fZec169: [F32;32] = [0.0;32];
		let mut fRec99_tmp: [F32;36] = [0.0;36];
		let mut fRec158: [F32;32] = [0.0;32];
		let mut fSlow52: F32 = self.fHslider31;
		let mut fRec177_tmp: [F32;36] = [0.0;36];
		let mut fZec170: [F32;32] = [0.0;32];
		let mut fZec171: [F32;32] = [0.0;32];
		let mut iZec172: [i32;32] = [0;32];
		let mut iZec173: [i32;32] = [0;32];
		let mut iZec174: [i32;32] = [0;32];
		let mut fZec175: [F32;32] = [0.0;32];
		let mut fZec176: [F32;32] = [0.0;32];
		let mut fZec177: [F32;32] = [0.0;32];
		let mut fZec178: [F32;32] = [0.0;32];
		let mut fZec179: [F32;32] = [0.0;32];
		let mut fZec180: [F32;32] = [0.0;32];
		let mut fZec181: [F32;32] = [0.0;32];
		let mut fZec182: [F32;32] = [0.0;32];
		let mut iZec183: [i32;32] = [0;32];
		let mut iZec184: [i32;32] = [0;32];
		let mut fZec185: [F32;32] = [0.0;32];
		let mut fZec186: [F32;32] = [0.0;32];
		let mut iZec187: [i32;32] = [0;32];
		let mut iZec188: [i32;32] = [0;32];
		let mut fZec189: [F32;32] = [0.0;32];
		let mut fZec190: [F32;32] = [0.0;32];
		let mut fZec191: [F32;32] = [0.0;32];
		let mut iZec192: [i32;32] = [0;32];
		let mut iZec193: [i32;32] = [0;32];
		let mut fZec194: [F32;32] = [0.0;32];
		let mut fZec195: [F32;32] = [0.0;32];
		let mut fZec196: [F32;32] = [0.0;32];
		let mut fZec197: [F32;32] = [0.0;32];
		let mut fZec198: [F32;32] = [0.0;32];
		let mut iZec199: [i32;32] = [0;32];
		let mut iZec200: [i32;32] = [0;32];
		let mut fRec173_tmp: [F32;36] = [0.0;36];
		let mut fRec178_tmp: [F32;36] = [0.0;36];
		let mut fRec174: [F32;32] = [0.0;32];
		let mut fZec201: [F32;32] = [0.0;32];
		let mut fZec202: [F32;32] = [0.0;32];
		let mut fZec203: [F32;32] = [0.0;32];
		let mut fRec180_tmp: [F32;36] = [0.0;36];
		let mut fSlow53: F32 = self.fButton1;
		let mut fVec2_tmp: [F32;36] = [0.0;36];
		let mut iRec181_tmp: [i32;36] = [0;36];
		let mut fRec182_tmp: [F32;36] = [0.0;36];
		let mut fRec185_tmp: [F32;36] = [0.0;36];
		let mut iYec27_tmp: [i32;36] = [0;36];
		let mut iRec186_tmp: [i32;36] = [0;36];
		let mut fZec204: [F32;32] = [0.0;32];
		let mut fZec205: [F32;32] = [0.0;32];
		let mut fRec184_tmp: [F32;36] = [0.0;36];
		let mut fZec206: [F32;32] = [0.0;32];
		let mut fZec207: [F32;32] = [0.0;32];
		let mut fZec208: [F32;32] = [0.0;32];
		let mut fZec209: [F32;32] = [0.0;32];
		let mut fZec210: [F32;32] = [0.0;32];
		let mut fYec28_tmp: [F32;36] = [0.0;36];
		let mut fZec211: [F32;32] = [0.0;32];
		let mut fZec212: [F32;32] = [0.0;32];
		let mut fYec29_tmp: [F32;36] = [0.0;36];
		let mut fRec175: [F32;32] = [0.0;32];
		let mut fRec176: [F32;32] = [0.0;32];
		let mut fRec169_tmp: [F32;36] = [0.0;36];
		let mut fRec170: [F32;32] = [0.0;32];
		let mut fRec171: [F32;32] = [0.0;32];
		let mut fRec172: [F32;32] = [0.0;32];
		let mut fRec165_tmp: [F32;36] = [0.0;36];
		let mut fRec166: [F32;32] = [0.0;32];
		let mut fRec167: [F32;32] = [0.0;32];
		let mut fRec168: [F32;32] = [0.0;32];
		let mut fRec162: [F32;32] = [0.0;32];
		let mut fRec163_tmp: [F32;36] = [0.0;36];
		let mut fRec164: [F32;32] = [0.0;32];
		let mut fRec159_tmp: [F32;36] = [0.0;36];
		let mut fRec160: [F32;32] = [0.0;32];
		let mut fRec154_tmp: [F32;36] = [0.0;36];
		let mut fRec155: [F32;32] = [0.0;32];
		let mut fRec156: [F32;32] = [0.0;32];
		let mut fRec157: [F32;32] = [0.0;32];
		let mut fRec151: [F32;32] = [0.0;32];
		let mut fRec152: [F32;32] = [0.0;32];
		let mut fRec153: [F32;32] = [0.0;32];
		let mut fRec148_tmp: [F32;36] = [0.0;36];
		let mut fRec149: [F32;32] = [0.0;32];
		let mut fZec213: [F32;32] = [0.0;32];
		let mut fZec214: [F32;32] = [0.0;32];
		let mut fRec188_tmp: [F32;36] = [0.0;36];
		let mut fRec187: [F32;32] = [0.0;32];
		let mut fRec147_tmp: [F32;36] = [0.0;36];
		let mut fRec189_tmp: [F32;36] = [0.0;36];
		let mut fZec215: [F32;32] = [0.0;32];
		let mut fZec216: [F32;32] = [0.0;32];
		let mut fZec217: [F32;32] = [0.0;32];
		let mut fZec218: [F32;32] = [0.0;32];
		let mut fRec146_tmp: [F32;36] = [0.0;36];
		let mut fZec219: [F32;32] = [0.0;32];
		let mut fRec145_tmp: [F32;36] = [0.0;36];
		let mut fRec203: [F32;32] = [0.0;32];
		let mut fSlow54: F32 = self.fHslider32;
		let mut fRec222_tmp: [F32;36] = [0.0;36];
		let mut fZec220: [F32;32] = [0.0;32];
		let mut fZec221: [F32;32] = [0.0;32];
		let mut iZec222: [i32;32] = [0;32];
		let mut iZec223: [i32;32] = [0;32];
		let mut iZec224: [i32;32] = [0;32];
		let mut fZec225: [F32;32] = [0.0;32];
		let mut fZec226: [F32;32] = [0.0;32];
		let mut fZec227: [F32;32] = [0.0;32];
		let mut fZec228: [F32;32] = [0.0;32];
		let mut fZec229: [F32;32] = [0.0;32];
		let mut fZec230: [F32;32] = [0.0;32];
		let mut fZec231: [F32;32] = [0.0;32];
		let mut fZec232: [F32;32] = [0.0;32];
		let mut iZec233: [i32;32] = [0;32];
		let mut iZec234: [i32;32] = [0;32];
		let mut fZec235: [F32;32] = [0.0;32];
		let mut fZec236: [F32;32] = [0.0;32];
		let mut iZec237: [i32;32] = [0;32];
		let mut iZec238: [i32;32] = [0;32];
		let mut fZec239: [F32;32] = [0.0;32];
		let mut fZec240: [F32;32] = [0.0;32];
		let mut fZec241: [F32;32] = [0.0;32];
		let mut iZec242: [i32;32] = [0;32];
		let mut iZec243: [i32;32] = [0;32];
		let mut fZec244: [F32;32] = [0.0;32];
		let mut fZec245: [F32;32] = [0.0;32];
		let mut fZec246: [F32;32] = [0.0;32];
		let mut fZec247: [F32;32] = [0.0;32];
		let mut fZec248: [F32;32] = [0.0;32];
		let mut iZec249: [i32;32] = [0;32];
		let mut iZec250: [i32;32] = [0;32];
		let mut fRec218_tmp: [F32;36] = [0.0;36];
		let mut fRec223_tmp: [F32;36] = [0.0;36];
		let mut fRec219: [F32;32] = [0.0;32];
		let mut fZec251: [F32;32] = [0.0;32];
		let mut fZec252: [F32;32] = [0.0;32];
		let mut fZec253: [F32;32] = [0.0;32];
		let mut fRec225_tmp: [F32;36] = [0.0;36];
		let mut fSlow55: F32 = self.fButton2;
		let mut fVec3_tmp: [F32;36] = [0.0;36];
		let mut iRec226_tmp: [i32;36] = [0;36];
		let mut fRec227_tmp: [F32;36] = [0.0;36];
		let mut fRec230_tmp: [F32;36] = [0.0;36];
		let mut iYec30_tmp: [i32;36] = [0;36];
		let mut iRec231_tmp: [i32;36] = [0;36];
		let mut fZec254: [F32;32] = [0.0;32];
		let mut fZec255: [F32;32] = [0.0;32];
		let mut fRec229_tmp: [F32;36] = [0.0;36];
		let mut fZec256: [F32;32] = [0.0;32];
		let mut fZec257: [F32;32] = [0.0;32];
		let mut fZec258: [F32;32] = [0.0;32];
		let mut fZec259: [F32;32] = [0.0;32];
		let mut fZec260: [F32;32] = [0.0;32];
		let mut fYec31_tmp: [F32;36] = [0.0;36];
		let mut fZec261: [F32;32] = [0.0;32];
		let mut fZec262: [F32;32] = [0.0;32];
		let mut fYec32_tmp: [F32;36] = [0.0;36];
		let mut fRec220: [F32;32] = [0.0;32];
		let mut fRec221: [F32;32] = [0.0;32];
		let mut fRec214_tmp: [F32;36] = [0.0;36];
		let mut fRec215: [F32;32] = [0.0;32];
		let mut fRec216: [F32;32] = [0.0;32];
		let mut fRec217: [F32;32] = [0.0;32];
		let mut fRec210_tmp: [F32;36] = [0.0;36];
		let mut fRec211: [F32;32] = [0.0;32];
		let mut fRec212: [F32;32] = [0.0;32];
		let mut fRec213: [F32;32] = [0.0;32];
		let mut fRec207: [F32;32] = [0.0;32];
		let mut fRec208_tmp: [F32;36] = [0.0;36];
		let mut fRec209: [F32;32] = [0.0;32];
		let mut fRec204_tmp: [F32;36] = [0.0;36];
		let mut fRec205: [F32;32] = [0.0;32];
		let mut fRec199_tmp: [F32;36] = [0.0;36];
		let mut fRec200: [F32;32] = [0.0;32];
		let mut fRec201: [F32;32] = [0.0;32];
		let mut fRec202: [F32;32] = [0.0;32];
		let mut fRec196: [F32;32] = [0.0;32];
		let mut fRec197: [F32;32] = [0.0;32];
		let mut fRec198: [F32;32] = [0.0;32];
		let mut fRec193_tmp: [F32;36] = [0.0;36];
		let mut fRec194: [F32;32] = [0.0;32];
		let mut fZec263: [F32;32] = [0.0;32];
		let mut fZec264: [F32;32] = [0.0;32];
		let mut fRec233_tmp: [F32;36] = [0.0;36];
		let mut fRec232: [F32;32] = [0.0;32];
		let mut fRec192_tmp: [F32;36] = [0.0;36];
		let mut fRec234_tmp: [F32;36] = [0.0;36];
		let mut fZec265: [F32;32] = [0.0;32];
		let mut fZec266: [F32;32] = [0.0;32];
		let mut fZec267: [F32;32] = [0.0;32];
		let mut fZec268: [F32;32] = [0.0;32];
		let mut fRec191_tmp: [F32;36] = [0.0;36];
		let mut fZec269: [F32;32] = [0.0;32];
		let mut fRec190_tmp: [F32;36] = [0.0;36];
		let mut fRec248: [F32;32] = [0.0;32];
		let mut fSlow56: F32 = self.fHslider33;
		let mut fRec267_tmp: [F32;36] = [0.0;36];
		let mut fZec270: [F32;32] = [0.0;32];
		let mut fZec271: [F32;32] = [0.0;32];
		let mut iZec272: [i32;32] = [0;32];
		let mut iZec273: [i32;32] = [0;32];
		let mut iZec274: [i32;32] = [0;32];
		let mut fZec275: [F32;32] = [0.0;32];
		let mut fZec276: [F32;32] = [0.0;32];
		let mut fZec277: [F32;32] = [0.0;32];
		let mut fZec278: [F32;32] = [0.0;32];
		let mut fZec279: [F32;32] = [0.0;32];
		let mut fZec280: [F32;32] = [0.0;32];
		let mut fZec281: [F32;32] = [0.0;32];
		let mut fZec282: [F32;32] = [0.0;32];
		let mut iZec283: [i32;32] = [0;32];
		let mut iZec284: [i32;32] = [0;32];
		let mut fZec285: [F32;32] = [0.0;32];
		let mut fZec286: [F32;32] = [0.0;32];
		let mut iZec287: [i32;32] = [0;32];
		let mut iZec288: [i32;32] = [0;32];
		let mut fZec289: [F32;32] = [0.0;32];
		let mut fZec290: [F32;32] = [0.0;32];
		let mut fZec291: [F32;32] = [0.0;32];
		let mut iZec292: [i32;32] = [0;32];
		let mut iZec293: [i32;32] = [0;32];
		let mut fZec294: [F32;32] = [0.0;32];
		let mut fZec295: [F32;32] = [0.0;32];
		let mut fZec296: [F32;32] = [0.0;32];
		let mut fZec297: [F32;32] = [0.0;32];
		let mut fZec298: [F32;32] = [0.0;32];
		let mut iZec299: [i32;32] = [0;32];
		let mut iZec300: [i32;32] = [0;32];
		let mut fRec263_tmp: [F32;36] = [0.0;36];
		let mut fRec268_tmp: [F32;36] = [0.0;36];
		let mut fRec264: [F32;32] = [0.0;32];
		let mut fZec301: [F32;32] = [0.0;32];
		let mut fZec302: [F32;32] = [0.0;32];
		let mut fZec303: [F32;32] = [0.0;32];
		let mut fRec270_tmp: [F32;36] = [0.0;36];
		let mut fSlow57: F32 = self.fButton3;
		let mut fVec4_tmp: [F32;36] = [0.0;36];
		let mut iRec271_tmp: [i32;36] = [0;36];
		let mut fRec272_tmp: [F32;36] = [0.0;36];
		let mut fRec275_tmp: [F32;36] = [0.0;36];
		let mut iYec33_tmp: [i32;36] = [0;36];
		let mut iRec276_tmp: [i32;36] = [0;36];
		let mut fZec304: [F32;32] = [0.0;32];
		let mut fZec305: [F32;32] = [0.0;32];
		let mut fRec274_tmp: [F32;36] = [0.0;36];
		let mut fZec306: [F32;32] = [0.0;32];
		let mut fZec307: [F32;32] = [0.0;32];
		let mut fZec308: [F32;32] = [0.0;32];
		let mut fZec309: [F32;32] = [0.0;32];
		let mut fZec310: [F32;32] = [0.0;32];
		let mut fYec34_tmp: [F32;36] = [0.0;36];
		let mut fZec311: [F32;32] = [0.0;32];
		let mut fZec312: [F32;32] = [0.0;32];
		let mut fYec35_tmp: [F32;36] = [0.0;36];
		let mut fRec265: [F32;32] = [0.0;32];
		let mut fRec266: [F32;32] = [0.0;32];
		let mut fRec259_tmp: [F32;36] = [0.0;36];
		let mut fRec260: [F32;32] = [0.0;32];
		let mut fRec261: [F32;32] = [0.0;32];
		let mut fRec262: [F32;32] = [0.0;32];
		let mut fRec255_tmp: [F32;36] = [0.0;36];
		let mut fRec256: [F32;32] = [0.0;32];
		let mut fRec257: [F32;32] = [0.0;32];
		let mut fRec258: [F32;32] = [0.0;32];
		let mut fRec252: [F32;32] = [0.0;32];
		let mut fRec253_tmp: [F32;36] = [0.0;36];
		let mut fRec254: [F32;32] = [0.0;32];
		let mut fRec249_tmp: [F32;36] = [0.0;36];
		let mut fRec250: [F32;32] = [0.0;32];
		let mut fRec244_tmp: [F32;36] = [0.0;36];
		let mut fRec245: [F32;32] = [0.0;32];
		let mut fRec246: [F32;32] = [0.0;32];
		let mut fRec247: [F32;32] = [0.0;32];
		let mut fRec241: [F32;32] = [0.0;32];
		let mut fRec242: [F32;32] = [0.0;32];
		let mut fRec243: [F32;32] = [0.0;32];
		let mut fRec238_tmp: [F32;36] = [0.0;36];
		let mut fRec239: [F32;32] = [0.0;32];
		let mut fZec313: [F32;32] = [0.0;32];
		let mut fZec314: [F32;32] = [0.0;32];
		let mut fRec278_tmp: [F32;36] = [0.0;36];
		let mut fRec277: [F32;32] = [0.0;32];
		let mut fRec237_tmp: [F32;36] = [0.0;36];
		let mut fRec279_tmp: [F32;36] = [0.0;36];
		let mut fZec315: [F32;32] = [0.0;32];
		let mut fZec316: [F32;32] = [0.0;32];
		let mut fZec317: [F32;32] = [0.0;32];
		let mut fZec318: [F32;32] = [0.0;32];
		let mut fRec236_tmp: [F32;36] = [0.0;36];
		let mut fZec319: [F32;32] = [0.0;32];
		let mut fRec235_tmp: [F32;36] = [0.0;36];
		let mut fRec293: [F32;32] = [0.0;32];
		let mut fSlow58: F32 = self.fHslider34;
		let mut fRec312_tmp: [F32;36] = [0.0;36];
		let mut fZec320: [F32;32] = [0.0;32];
		let mut fZec321: [F32;32] = [0.0;32];
		let mut iZec322: [i32;32] = [0;32];
		let mut iZec323: [i32;32] = [0;32];
		let mut iZec324: [i32;32] = [0;32];
		let mut fZec325: [F32;32] = [0.0;32];
		let mut fZec326: [F32;32] = [0.0;32];
		let mut fZec327: [F32;32] = [0.0;32];
		let mut fZec328: [F32;32] = [0.0;32];
		let mut fZec329: [F32;32] = [0.0;32];
		let mut fZec330: [F32;32] = [0.0;32];
		let mut fZec331: [F32;32] = [0.0;32];
		let mut fZec332: [F32;32] = [0.0;32];
		let mut iZec333: [i32;32] = [0;32];
		let mut iZec334: [i32;32] = [0;32];
		let mut fZec335: [F32;32] = [0.0;32];
		let mut fZec336: [F32;32] = [0.0;32];
		let mut iZec337: [i32;32] = [0;32];
		let mut iZec338: [i32;32] = [0;32];
		let mut fZec339: [F32;32] = [0.0;32];
		let mut fZec340: [F32;32] = [0.0;32];
		let mut fZec341: [F32;32] = [0.0;32];
		let mut iZec342: [i32;32] = [0;32];
		let mut iZec343: [i32;32] = [0;32];
		let mut fZec344: [F32;32] = [0.0;32];
		let mut fZec345: [F32;32] = [0.0;32];
		let mut fZec346: [F32;32] = [0.0;32];
		let mut fZec347: [F32;32] = [0.0;32];
		let mut fZec348: [F32;32] = [0.0;32];
		let mut iZec349: [i32;32] = [0;32];
		let mut iZec350: [i32;32] = [0;32];
		let mut fRec308_tmp: [F32;36] = [0.0;36];
		let mut fRec313_tmp: [F32;36] = [0.0;36];
		let mut fRec309: [F32;32] = [0.0;32];
		let mut fZec351: [F32;32] = [0.0;32];
		let mut fZec352: [F32;32] = [0.0;32];
		let mut fZec353: [F32;32] = [0.0;32];
		let mut fRec315_tmp: [F32;36] = [0.0;36];
		let mut fSlow59: F32 = self.fButton4;
		let mut fVec5_tmp: [F32;36] = [0.0;36];
		let mut iRec316_tmp: [i32;36] = [0;36];
		let mut fRec317_tmp: [F32;36] = [0.0;36];
		let mut fRec320_tmp: [F32;36] = [0.0;36];
		let mut iYec36_tmp: [i32;36] = [0;36];
		let mut iRec321_tmp: [i32;36] = [0;36];
		let mut fZec354: [F32;32] = [0.0;32];
		let mut fZec355: [F32;32] = [0.0;32];
		let mut fRec319_tmp: [F32;36] = [0.0;36];
		let mut fZec356: [F32;32] = [0.0;32];
		let mut fZec357: [F32;32] = [0.0;32];
		let mut fZec358: [F32;32] = [0.0;32];
		let mut fZec359: [F32;32] = [0.0;32];
		let mut fZec360: [F32;32] = [0.0;32];
		let mut fYec37_tmp: [F32;36] = [0.0;36];
		let mut fZec361: [F32;32] = [0.0;32];
		let mut fZec362: [F32;32] = [0.0;32];
		let mut fYec38_tmp: [F32;36] = [0.0;36];
		let mut fRec310: [F32;32] = [0.0;32];
		let mut fRec311: [F32;32] = [0.0;32];
		let mut fRec304_tmp: [F32;36] = [0.0;36];
		let mut fRec305: [F32;32] = [0.0;32];
		let mut fRec306: [F32;32] = [0.0;32];
		let mut fRec307: [F32;32] = [0.0;32];
		let mut fRec300_tmp: [F32;36] = [0.0;36];
		let mut fRec301: [F32;32] = [0.0;32];
		let mut fRec302: [F32;32] = [0.0;32];
		let mut fRec303: [F32;32] = [0.0;32];
		let mut fRec297: [F32;32] = [0.0;32];
		let mut fRec298_tmp: [F32;36] = [0.0;36];
		let mut fRec299: [F32;32] = [0.0;32];
		let mut fRec294_tmp: [F32;36] = [0.0;36];
		let mut fRec295: [F32;32] = [0.0;32];
		let mut fRec289_tmp: [F32;36] = [0.0;36];
		let mut fRec290: [F32;32] = [0.0;32];
		let mut fRec291: [F32;32] = [0.0;32];
		let mut fRec292: [F32;32] = [0.0;32];
		let mut fRec286: [F32;32] = [0.0;32];
		let mut fRec287: [F32;32] = [0.0;32];
		let mut fRec288: [F32;32] = [0.0;32];
		let mut fRec283_tmp: [F32;36] = [0.0;36];
		let mut fRec284: [F32;32] = [0.0;32];
		let mut fZec363: [F32;32] = [0.0;32];
		let mut fZec364: [F32;32] = [0.0;32];
		let mut fRec323_tmp: [F32;36] = [0.0;36];
		let mut fRec322: [F32;32] = [0.0;32];
		let mut fRec282_tmp: [F32;36] = [0.0;36];
		let mut fRec324_tmp: [F32;36] = [0.0;36];
		let mut fZec365: [F32;32] = [0.0;32];
		let mut fZec366: [F32;32] = [0.0;32];
		let mut fZec367: [F32;32] = [0.0;32];
		let mut fZec368: [F32;32] = [0.0;32];
		let mut fRec281_tmp: [F32;36] = [0.0;36];
		let mut fZec369: [F32;32] = [0.0;32];
		let mut fRec280_tmp: [F32;36] = [0.0;36];
		let mut fSlow60: F32 = self.fConst1 * self.fHslider35;
		let mut fRec326_tmp: [F32;36] = [0.0;36];
		let mut fZec370: [F32;32] = [0.0;32];
		let mut fZec371: [F32;32] = [0.0;32];
		let mut fSlow61: F32 = self.fHslider36;
		let mut fZec372: [F32;32] = [0.0;32];
		let mut fZec373: [F32;32] = [0.0;32];
		let mut iSlow62: i32 = unsafe { itbl0mydspSIG0[((245.0 * fSlow1) as i32) as usize] };
		let mut fSlow63: F32 = 0.0001 * (iSlow62) as F32;
		let mut fRec335_tmp: [F32;36] = [0.0;36];
		let mut iSlow64: i32 = unsafe { itbl0mydspSIG0[((185.0 * fSlow1) as i32) as usize] };
		let mut fSlow65: F32 = 0.0001 * (iSlow64) as F32;
		let mut fRec342_tmp: [F32;36] = [0.0;36];
		let mut iSlow66: i32 = unsafe { itbl0mydspSIG0[((155.0 * fSlow1) as i32) as usize] };
		let mut fSlow67: F32 = 0.0001 * (iSlow66) as F32;
		let mut fRec346_tmp: [F32;36] = [0.0;36];
		let mut iSlow68: i32 = unsafe { itbl0mydspSIG0[((125.0 * fSlow1) as i32) as usize] };
		let mut fSlow69: F32 = 0.0001 * (iSlow68) as F32;
		let mut fRec350_tmp: [F32;36] = [0.0;36];
		let mut iSlow70: i32 = unsafe { itbl0mydspSIG0[((1e+02 * fSlow1) as i32) as usize] };
		let mut fSlow71: F32 = 0.0001 * (iSlow70) as F32;
		let mut fRec358_tmp: [F32;36] = [0.0;36];
		let mut iSlow72: i32 = unsafe { itbl0mydspSIG0[((7e+01 * fSlow1) as i32) as usize] };
		let mut fSlow73: F32 = 0.0001 * (iSlow72) as F32;
		let mut fRec362_tmp: [F32;36] = [0.0;36];
		let mut iSlow74: i32 = unsafe { itbl0mydspSIG0[((1e+01 * fSlow1) as i32) as usize] };
		let mut fSlow75: F32 = 0.0001 * (iSlow74) as F32;
		let mut fRec369_tmp: [F32;36] = [0.0;36];
		let mut fZec374: [F32;32] = [0.0;32];
		let mut fZec375: [F32;32] = [0.0;32];
		let mut fZec376: [F32;32] = [0.0;32];
		let mut fZec377: [F32;32] = [0.0;32];
		let mut fZec378: [F32;32] = [0.0;32];
		let mut fZec379: [F32;32] = [0.0;32];
		let mut fYec40_tmp: [F32;36] = [0.0;36];
		let mut fRec368_tmp: [F32;36] = [0.0;36];
		let mut fRec366_tmp: [F32;36] = [0.0;36];
		let mut iSlow76: i32 = unsafe { itbl0mydspSIG0[((1.1e+02 * fSlow1) as i32) as usize] };
		let mut fSlow77: F32 = 0.0001 * (iSlow76) as F32;
		let mut fRec371_tmp: [F32;36] = [0.0;36];
		let mut fZec380: [F32;32] = [0.0;32];
		let mut fZec381: [F32;32] = [0.0;32];
		let mut fZec382: [F32;32] = [0.0;32];
		let mut fZec383: [F32;32] = [0.0;32];
		let mut fYec42_tmp: [F32;36] = [0.0;36];
		let mut fRec370_tmp: [F32;36] = [0.0;36];
		let mut fRec367_tmp: [F32;36] = [0.0;36];
		let mut iSlow78: i32 = unsafe { itbl0mydspSIG0[((4e+01 * fSlow1) as i32) as usize] };
		let mut fSlow79: F32 = 0.0001 * (iSlow78) as F32;
		let mut fRec372_tmp: [F32;36] = [0.0;36];
		let mut fZec384: [F32;32] = [0.0;32];
		let mut fZec385: [F32;32] = [0.0;32];
		let mut fZec386: [F32;32] = [0.0;32];
		let mut fZec387: [F32;32] = [0.0;32];
		let mut fZec388: [F32;32] = [0.0;32];
		let mut fZec389: [F32;32] = [0.0;32];
		let mut fYec44_tmp: [F32;36] = [0.0;36];
		let mut fRec365_tmp: [F32;36] = [0.0;36];
		let mut fRec363_tmp: [F32;36] = [0.0;36];
		let mut iSlow80: i32 = unsafe { itbl0mydspSIG0[((1.4e+02 * fSlow1) as i32) as usize] };
		let mut fSlow81: F32 = 0.0001 * (iSlow80) as F32;
		let mut fRec374_tmp: [F32;36] = [0.0;36];
		let mut fZec390: [F32;32] = [0.0;32];
		let mut fZec391: [F32;32] = [0.0;32];
		let mut fZec392: [F32;32] = [0.0;32];
		let mut fZec393: [F32;32] = [0.0;32];
		let mut fYec46_tmp: [F32;36] = [0.0;36];
		let mut fRec373_tmp: [F32;36] = [0.0;36];
		let mut fRec364_tmp: [F32;36] = [0.0;36];
		let mut fZec394: [F32;32] = [0.0;32];
		let mut fZec395: [F32;32] = [0.0;32];
		let mut fZec396: [F32;32] = [0.0;32];
		let mut fZec397: [F32;32] = [0.0;32];
		let mut fZec398: [F32;32] = [0.0;32];
		let mut fZec399: [F32;32] = [0.0;32];
		let mut fZec400: [F32;32] = [0.0;32];
		let mut fZec401: [F32;32] = [0.0;32];
		let mut fYec48_tmp: [F32;36] = [0.0;36];
		let mut fRec361_tmp: [F32;36] = [0.0;36];
		let mut fRec359_tmp: [F32;36] = [0.0;36];
		let mut iSlow82: i32 = unsafe { itbl0mydspSIG0[((1.7e+02 * fSlow1) as i32) as usize] };
		let mut fSlow83: F32 = 0.0001 * (iSlow82) as F32;
		let mut fRec376_tmp: [F32;36] = [0.0;36];
		let mut fZec402: [F32;32] = [0.0;32];
		let mut fZec403: [F32;32] = [0.0;32];
		let mut fZec404: [F32;32] = [0.0;32];
		let mut fZec405: [F32;32] = [0.0;32];
		let mut fYec50_tmp: [F32;36] = [0.0;36];
		let mut fRec375_tmp: [F32;36] = [0.0;36];
		let mut fRec360_tmp: [F32;36] = [0.0;36];
		let mut fZec406: [F32;32] = [0.0;32];
		let mut fZec407: [F32;32] = [0.0;32];
		let mut fZec408: [F32;32] = [0.0;32];
		let mut fZec409: [F32;32] = [0.0;32];
		let mut fZec410: [F32;32] = [0.0;32];
		let mut fZec411: [F32;32] = [0.0;32];
		let mut fZec412: [F32;32] = [0.0;32];
		let mut fZec413: [F32;32] = [0.0;32];
		let mut fYec52_tmp: [F32;36] = [0.0;36];
		let mut fRec357_tmp: [F32;36] = [0.0;36];
		let mut fRec355_tmp: [F32;36] = [0.0;36];
		let mut iSlow84: i32 = unsafe { itbl0mydspSIG0[((2e+02 * fSlow1) as i32) as usize] };
		let mut fSlow85: F32 = 0.0001 * (iSlow84) as F32;
		let mut fRec378_tmp: [F32;36] = [0.0;36];
		let mut fZec414: [F32;32] = [0.0;32];
		let mut fZec415: [F32;32] = [0.0;32];
		let mut fZec416: [F32;32] = [0.0;32];
		let mut fZec417: [F32;32] = [0.0;32];
		let mut fYec54_tmp: [F32;36] = [0.0;36];
		let mut fRec377_tmp: [F32;36] = [0.0;36];
		let mut fRec356_tmp: [F32;36] = [0.0;36];
		let mut iSlow86: i32 = unsafe { itbl0mydspSIG0[((1.3e+02 * fSlow1) as i32) as usize] };
		let mut fSlow87: F32 = 0.0001 * (iSlow86) as F32;
		let mut fRec379_tmp: [F32;36] = [0.0;36];
		let mut fZec418: [F32;32] = [0.0;32];
		let mut fZec419: [F32;32] = [0.0;32];
		let mut fZec420: [F32;32] = [0.0;32];
		let mut fZec421: [F32;32] = [0.0;32];
		let mut fZec422: [F32;32] = [0.0;32];
		let mut fZec423: [F32;32] = [0.0;32];
		let mut fYec56_tmp: [F32;36] = [0.0;36];
		let mut fRec354_tmp: [F32;36] = [0.0;36];
		let mut fRec352_tmp: [F32;36] = [0.0;36];
		let mut iSlow88: i32 = unsafe { itbl0mydspSIG0[((2.3e+02 * fSlow1) as i32) as usize] };
		let mut fSlow89: F32 = 0.0001 * (iSlow88) as F32;
		let mut fRec381_tmp: [F32;36] = [0.0;36];
		let mut fZec424: [F32;32] = [0.0;32];
		let mut fZec425: [F32;32] = [0.0;32];
		let mut fZec426: [F32;32] = [0.0;32];
		let mut fZec427: [F32;32] = [0.0;32];
		let mut fYec58_tmp: [F32;36] = [0.0;36];
		let mut fRec380_tmp: [F32;36] = [0.0;36];
		let mut fRec353_tmp: [F32;36] = [0.0;36];
		let mut iSlow90: i32 = unsafe { itbl0mydspSIG0[((54.0 * fSlow1) as i32) as usize] };
		let mut fSlow91: F32 = 0.005 * (iSlow90) as F32;
		let mut fRec382_tmp: [F32;36] = [0.0;36];
		let mut fZec428: [F32;32] = [0.0;32];
		let mut fZec429: [F32;32] = [0.0;32];
		let mut fSlow92: F32 = 5e+01 * self.fHslider37;
		let mut fZec430: [F32;32] = [0.0;32];
		let mut fZec431: [F32;32] = [0.0;32];
		let mut iZec432: [i32;32] = [0;32];
		let mut iZec433: [i32;32] = [0;32];
		let mut fZec434: [F32;32] = [0.0;32];
		let mut fZec435: [F32;32] = [0.0;32];
		let mut fZec436: [F32;32] = [0.0;32];
		let mut fZec437: [F32;32] = [0.0;32];
		let mut fZec438: [F32;32] = [0.0;32];
		let mut fZec439: [F32;32] = [0.0;32];
		let mut fZec440: [F32;32] = [0.0;32];
		let mut fZec441: [F32;32] = [0.0;32];
		let mut iZec442: [i32;32] = [0;32];
		let mut fZec443: [F32;32] = [0.0;32];
		let mut fZec444: [F32;32] = [0.0;32];
		let mut iZec445: [i32;32] = [0;32];
		let mut fZec446: [F32;32] = [0.0;32];
		let mut fZec447: [F32;32] = [0.0;32];
		let mut fZec448: [F32;32] = [0.0;32];
		let mut iZec449: [i32;32] = [0;32];
		let mut fZec450: [F32;32] = [0.0;32];
		let mut fZec451: [F32;32] = [0.0;32];
		let mut fZec452: [F32;32] = [0.0;32];
		let mut fZec453: [F32;32] = [0.0;32];
		let mut fZec454: [F32;32] = [0.0;32];
		let mut iZec455: [i32;32] = [0;32];
		let mut fYec61_tmp: [F32;36] = [0.0;36];
		let mut fRec351_tmp: [F32;36] = [0.0;36];
		let mut iSlow93: i32 = unsafe { itbl0mydspSIG0[((204.0 * fSlow1) as i32) as usize] };
		let mut fSlow94: F32 = 0.005 * (iSlow93) as F32;
		let mut fRec384_tmp: [F32;36] = [0.0;36];
		let mut fZec456: [F32;32] = [0.0;32];
		let mut fZec457: [F32;32] = [0.0;32];
		let mut fSlow95: F32 = 0.0 - fSlow92;
		let mut fZec458: [F32;32] = [0.0;32];
		let mut fZec459: [F32;32] = [0.0;32];
		let mut iZec460: [i32;32] = [0;32];
		let mut fZec461: [F32;32] = [0.0;32];
		let mut fZec462: [F32;32] = [0.0;32];
		let mut fZec463: [F32;32] = [0.0;32];
		let mut fZec464: [F32;32] = [0.0;32];
		let mut fZec465: [F32;32] = [0.0;32];
		let mut fZec466: [F32;32] = [0.0;32];
		let mut fYec64_tmp: [F32;36] = [0.0;36];
		let mut fRec383_tmp: [F32;36] = [0.0;36];
		let mut fZec467: [F32;32] = [0.0;32];
		let mut fZec468: [F32;32] = [0.0;32];
		let mut fZec469: [F32;32] = [0.0;32];
		let mut fZec470: [F32;32] = [0.0;32];
		let mut fZec471: [F32;32] = [0.0;32];
		let mut fZec472: [F32;32] = [0.0;32];
		let mut fYec66_tmp: [F32;36] = [0.0;36];
		let mut fRec349_tmp: [F32;36] = [0.0;36];
		let mut fRec347_tmp: [F32;36] = [0.0;36];
		let mut iSlow96: i32 = unsafe { itbl0mydspSIG0[((25.0 * fSlow1) as i32) as usize] };
		let mut fSlow97: F32 = 0.0001 * (iSlow96) as F32;
		let mut fRec386_tmp: [F32;36] = [0.0;36];
		let mut fZec473: [F32;32] = [0.0;32];
		let mut fZec474: [F32;32] = [0.0;32];
		let mut fZec475: [F32;32] = [0.0;32];
		let mut fZec476: [F32;32] = [0.0;32];
		let mut fYec68_tmp: [F32;36] = [0.0;36];
		let mut fRec385_tmp: [F32;36] = [0.0;36];
		let mut fRec348_tmp: [F32;36] = [0.0;36];
		let mut fZec477: [F32;32] = [0.0;32];
		let mut fZec478: [F32;32] = [0.0;32];
		let mut fZec479: [F32;32] = [0.0;32];
		let mut fZec480: [F32;32] = [0.0;32];
		let mut fZec481: [F32;32] = [0.0;32];
		let mut fZec482: [F32;32] = [0.0;32];
		let mut fZec483: [F32;32] = [0.0;32];
		let mut fZec484: [F32;32] = [0.0;32];
		let mut fYec70_tmp: [F32;36] = [0.0;36];
		let mut fRec345_tmp: [F32;36] = [0.0;36];
		let mut fRec343_tmp: [F32;36] = [0.0;36];
		let mut fZec485: [F32;32] = [0.0;32];
		let mut fZec486: [F32;32] = [0.0;32];
		let mut fZec487: [F32;32] = [0.0;32];
		let mut fZec488: [F32;32] = [0.0;32];
		let mut iZec489: [i32;32] = [0;32];
		let mut fYec72_tmp: [F32;36] = [0.0;36];
		let mut fRec387_tmp: [F32;36] = [0.0;36];
		let mut fRec344_tmp: [F32;36] = [0.0;36];
		let mut fZec490: [F32;32] = [0.0;32];
		let mut fZec491: [F32;32] = [0.0;32];
		let mut fZec492: [F32;32] = [0.0;32];
		let mut fZec493: [F32;32] = [0.0;32];
		let mut fZec494: [F32;32] = [0.0;32];
		let mut fZec495: [F32;32] = [0.0;32];
		let mut fZec496: [F32;32] = [0.0;32];
		let mut fZec497: [F32;32] = [0.0;32];
		let mut fYec74_tmp: [F32;36] = [0.0;36];
		let mut fRec341_tmp: [F32;36] = [0.0;36];
		let mut fRec339_tmp: [F32;36] = [0.0;36];
		let mut iSlow98: i32 = unsafe { itbl0mydspSIG0[((85.0 * fSlow1) as i32) as usize] };
		let mut fSlow99: F32 = 0.0001 * (iSlow98) as F32;
		let mut fRec389_tmp: [F32;36] = [0.0;36];
		let mut fZec498: [F32;32] = [0.0;32];
		let mut fZec499: [F32;32] = [0.0;32];
		let mut fZec500: [F32;32] = [0.0;32];
		let mut fZec501: [F32;32] = [0.0;32];
		let mut iZec502: [i32;32] = [0;32];
		let mut fYec76_tmp: [F32;36] = [0.0;36];
		let mut fRec388_tmp: [F32;36] = [0.0;36];
		let mut fRec340_tmp: [F32;36] = [0.0;36];
		let mut fZec503: [F32;32] = [0.0;32];
		let mut fZec504: [F32;32] = [0.0;32];
		let mut fZec505: [F32;32] = [0.0;32];
		let mut fZec506: [F32;32] = [0.0;32];
		let mut fZec507: [F32;32] = [0.0;32];
		let mut fZec508: [F32;32] = [0.0;32];
		let mut fZec509: [F32;32] = [0.0;32];
		let mut fZec510: [F32;32] = [0.0;32];
		let mut iZec511: [i32;32] = [0;32];
		let mut fYec78_tmp: [F32;36] = [0.0;36];
		let mut fRec338_tmp: [F32;36] = [0.0;36];
		let mut fRec336_tmp: [F32;36] = [0.0;36];
		let mut fZec512: [F32;32] = [0.0;32];
		let mut fZec513: [F32;32] = [0.0;32];
		let mut fZec514: [F32;32] = [0.0;32];
		let mut fZec515: [F32;32] = [0.0;32];
		let mut iZec516: [i32;32] = [0;32];
		let mut fYec80_tmp: [F32;36] = [0.0;36];
		let mut fRec390_tmp: [F32;36] = [0.0;36];
		let mut fRec337_tmp: [F32;36] = [0.0;36];
		let mut fZec517: [F32;32] = [0.0;32];
		let mut fZec518: [F32;32] = [0.0;32];
		let mut fZec519: [F32;32] = [0.0;32];
		let mut fZec520: [F32;32] = [0.0;32];
		let mut fZec521: [F32;32] = [0.0;32];
		let mut fZec522: [F32;32] = [0.0;32];
		let mut fZec523: [F32;32] = [0.0;32];
		let mut fZec524: [F32;32] = [0.0;32];
		let mut fYec82_tmp: [F32;36] = [0.0;36];
		let mut fRec334_tmp: [F32;36] = [0.0;36];
		let mut fRec332_tmp: [F32;36] = [0.0;36];
		let mut iSlow100: i32 = unsafe { itbl0mydspSIG0[((145.0 * fSlow1) as i32) as usize] };
		let mut fSlow101: F32 = 0.0001 * (iSlow100) as F32;
		let mut fRec392_tmp: [F32;36] = [0.0;36];
		let mut fZec525: [F32;32] = [0.0;32];
		let mut fZec526: [F32;32] = [0.0;32];
		let mut fZec527: [F32;32] = [0.0;32];
		let mut fZec528: [F32;32] = [0.0;32];
		let mut iZec529: [i32;32] = [0;32];
		let mut fYec84_tmp: [F32;36] = [0.0;36];
		let mut fRec391_tmp: [F32;36] = [0.0;36];
		let mut fRec333_tmp: [F32;36] = [0.0;36];
		let mut iSlow102: i32 = unsafe { itbl0mydspSIG0[((134.0 * fSlow1) as i32) as usize] };
		let mut fSlow103: F32 = 0.005 * (iSlow102) as F32;
		let mut fRec393_tmp: [F32;36] = [0.0;36];
		let mut fZec530: [F32;32] = [0.0;32];
		let mut fZec531: [F32;32] = [0.0;32];
		let mut fZec532: [F32;32] = [0.0;32];
		let mut fZec533: [F32;32] = [0.0;32];
		let mut iZec534: [i32;32] = [0;32];
		let mut iZec535: [i32;32] = [0;32];
		let mut fZec536: [F32;32] = [0.0;32];
		let mut fZec537: [F32;32] = [0.0;32];
		let mut fZec538: [F32;32] = [0.0;32];
		let mut fZec539: [F32;32] = [0.0;32];
		let mut fZec540: [F32;32] = [0.0;32];
		let mut fZec541: [F32;32] = [0.0;32];
		let mut fZec542: [F32;32] = [0.0;32];
		let mut fZec543: [F32;32] = [0.0;32];
		let mut iZec544: [i32;32] = [0;32];
		let mut fZec545: [F32;32] = [0.0;32];
		let mut fZec546: [F32;32] = [0.0;32];
		let mut iZec547: [i32;32] = [0;32];
		let mut fZec548: [F32;32] = [0.0;32];
		let mut fZec549: [F32;32] = [0.0;32];
		let mut fZec550: [F32;32] = [0.0;32];
		let mut iZec551: [i32;32] = [0;32];
		let mut fZec552: [F32;32] = [0.0;32];
		let mut fZec553: [F32;32] = [0.0;32];
		let mut fZec554: [F32;32] = [0.0;32];
		let mut fZec555: [F32;32] = [0.0;32];
		let mut fZec556: [F32;32] = [0.0;32];
		let mut iZec557: [i32;32] = [0;32];
		let mut fYec87_tmp: [F32;36] = [0.0;36];
		let mut fRec331_tmp: [F32;36] = [0.0;36];
		let mut fRec330_tmp: [F32;36] = [0.0;36];
		let mut fRec329_tmp: [F32;36] = [0.0;36];
		let mut fRec328_tmp: [F32;36] = [0.0;36];
		let mut fZec558: [F32;32] = [0.0;32];
		let mut fRec327_tmp: [F32;36] = [0.0;36];
		let mut fRec399_tmp: [F32;36] = [0.0;36];
		let mut fRec398_tmp: [F32;36] = [0.0;36];
		let mut fRec397_tmp: [F32;36] = [0.0;36];
		let mut fYec88_tmp: [F32;36] = [0.0;36];
		let mut fRec396_tmp: [F32;36] = [0.0;36];
		let mut fRec395_tmp: [F32;36] = [0.0;36];
		let mut fRec394_tmp: [F32;36] = [0.0;36];
		let mut fRec402_tmp: [F32;36] = [0.0;36];
		let mut fRec401_tmp: [F32;36] = [0.0;36];
		let mut fRec400_tmp: [F32;36] = [0.0;36];
		let mut fSlow104: F32 = self.fHslider38;
		let mut fSlow105: F32 = F32::powf(1e+01, 0.0 - 0.51 * ((1.25 * fSlow1 + -0.25) / self.fHslider39));
		let mut fZec559: [F32;32] = [0.0;32];
		let mut fZec560: [F32;32] = [0.0;32];
		let mut fSlow106: F32 = 1.0 - fSlow104;
		let mut fRec18_tmp: [F32;36] = [0.0;36];
		let mut iSlow107: i32 = unsafe { itbl0mydspSIG0[((34.0 * fSlow1) as i32) as usize] };
		let mut fSlow108: F32 = 0.005 * (iSlow107) as F32;
		let mut fRec409_tmp: [F32;36] = [0.0;36];
		let mut fZec561: [F32;32] = [0.0;32];
		let mut fZec562: [F32;32] = [0.0;32];
		let mut fZec563: [F32;32] = [0.0;32];
		let mut fZec564: [F32;32] = [0.0;32];
		let mut iZec565: [i32;32] = [0;32];
		let mut fZec566: [F32;32] = [0.0;32];
		let mut fZec567: [F32;32] = [0.0;32];
		let mut fZec568: [F32;32] = [0.0;32];
		let mut fZec569: [F32;32] = [0.0;32];
		let mut fZec570: [F32;32] = [0.0;32];
		let mut fZec571: [F32;32] = [0.0;32];
		let mut fYec92_tmp: [F32;36] = [0.0;36];
		let mut fRec408_tmp: [F32;36] = [0.0;36];
		let mut fRec407_tmp: [F32;36] = [0.0;36];
		let mut fRec406_tmp: [F32;36] = [0.0;36];
		let mut fRec405_tmp: [F32;36] = [0.0;36];
		let mut fZec572: [F32;32] = [0.0;32];
		let mut fRec404_tmp: [F32;36] = [0.0;36];
		let mut fRec415_tmp: [F32;36] = [0.0;36];
		let mut fRec414_tmp: [F32;36] = [0.0;36];
		let mut fRec413_tmp: [F32;36] = [0.0;36];
		let mut fYec93_tmp: [F32;36] = [0.0;36];
		let mut fRec412_tmp: [F32;36] = [0.0;36];
		let mut fRec411_tmp: [F32;36] = [0.0;36];
		let mut fRec410_tmp: [F32;36] = [0.0;36];
		let mut fRec418_tmp: [F32;36] = [0.0;36];
		let mut fRec417_tmp: [F32;36] = [0.0;36];
		let mut fRec416_tmp: [F32;36] = [0.0;36];
		let mut fZec573: [F32;32] = [0.0;32];
		let mut fRec403_tmp: [F32;36] = [0.0;36];
		let mut fSlow109: F32 = self.fHslider40;
		let mut fSlow110: F32 = F32::sin(fSlow109);
		let mut fSlow111: F32 = F32::cos(fSlow109);
		let mut fZec574: [F32;32] = [0.0;32];
		let mut fZec575: [F32;32] = [0.0;32];
		let mut fYec96_tmp: [F32;36] = [0.0;36];
		let mut fRec16_tmp: [F32;36] = [0.0;36];
		let mut fRec14_tmp: [F32;36] = [0.0;36];
		let mut iSlow112: i32 = unsafe { itbl0mydspSIG0[((2.4e+02 * fSlow1) as i32) as usize] };
		let mut fSlow113: F32 = 0.0001 * (iSlow112) as F32;
		let mut fRec420_tmp: [F32;36] = [0.0;36];
		let mut fZec576: [F32;32] = [0.0;32];
		let mut fZec577: [F32;32] = [0.0;32];
		let mut fZec578: [F32;32] = [0.0;32];
		let mut fZec579: [F32;32] = [0.0;32];
		let mut fYec98_tmp: [F32;36] = [0.0;36];
		let mut fRec419_tmp: [F32;36] = [0.0;36];
		let mut fRec15_tmp: [F32;36] = [0.0;36];
		let mut fZec580: [F32;32] = [0.0;32];
		let mut fZec581: [F32;32] = [0.0;32];
		let mut fZec582: [F32;32] = [0.0;32];
		let mut fZec583: [F32;32] = [0.0;32];
		let mut fYec100_tmp: [F32;36] = [0.0;36];
		let mut fRec12_tmp: [F32;36] = [0.0;36];
		let mut fRec10_tmp: [F32;36] = [0.0;36];
		let mut fYec102_tmp: [F32;36] = [0.0;36];
		let mut fRec421_tmp: [F32;36] = [0.0;36];
		let mut fRec11_tmp: [F32;36] = [0.0;36];
		let mut fZec584: [F32;32] = [0.0;32];
		let mut fZec585: [F32;32] = [0.0;32];
		let mut fZec586: [F32;32] = [0.0;32];
		let mut fZec587: [F32;32] = [0.0;32];
		let mut fYec104_tmp: [F32;36] = [0.0;36];
		let mut fRec8_tmp: [F32;36] = [0.0;36];
		let mut fRec6_tmp: [F32;36] = [0.0;36];
		let mut iSlow114: i32 = unsafe { itbl0mydspSIG0[((1.9e+02 * fSlow1) as i32) as usize] };
		let mut fSlow115: F32 = 0.0001 * (iSlow114) as F32;
		let mut fRec423_tmp: [F32;36] = [0.0;36];
		let mut fZec588: [F32;32] = [0.0;32];
		let mut fZec589: [F32;32] = [0.0;32];
		let mut fZec590: [F32;32] = [0.0;32];
		let mut fZec591: [F32;32] = [0.0;32];
		let mut fYec106_tmp: [F32;36] = [0.0;36];
		let mut fRec422_tmp: [F32;36] = [0.0;36];
		let mut fRec7_tmp: [F32;36] = [0.0;36];
		let mut iSlow116: i32 = unsafe { itbl0mydspSIG0[((175.0 * fSlow1) as i32) as usize] };
		let mut fSlow117: F32 = 0.0001 * (iSlow116) as F32;
		let mut fRec424_tmp: [F32;36] = [0.0;36];
		let mut fZec592: [F32;32] = [0.0;32];
		let mut fZec593: [F32;32] = [0.0;32];
		let mut fZec594: [F32;32] = [0.0;32];
		let mut fZec595: [F32;32] = [0.0;32];
		let mut fZec596: [F32;32] = [0.0;32];
		let mut fZec597: [F32;32] = [0.0;32];
		let mut fYec108_tmp: [F32;36] = [0.0;36];
		let mut fRec5_tmp: [F32;36] = [0.0;36];
		let mut fRec3_tmp: [F32;36] = [0.0;36];
		let mut fYec110_tmp: [F32;36] = [0.0;36];
		let mut fRec425_tmp: [F32;36] = [0.0;36];
		let mut fRec4_tmp: [F32;36] = [0.0;36];
		let mut fRec1_tmp: [F32;36] = [0.0;36];
		let mut fRec2_tmp: [F32;36] = [0.0;36];
		let mut fSlow118: F32 = self.fHslider41;
		let mut fSlow119: F32 = 1.0 - fSlow118;
		let mut fZec598: [F32;32] = [0.0;32];
		/* Main loop */
		let zipped_iterators = outputs0.zip(outputs1);
		for (output0, output1) in zipped_iterators {
			/* Vectorizable loop 0 */
			/* Pre code */
			for j2 in 0..4 {
				iVec0_tmp[j2 as usize] = self.iVec0_perm[j2 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iVec0_tmp[(i32::wrapping_add(4, i)) as usize] = 1;
			}
			/* Post code */
			for j3 in 0..4 {
				self.iVec0_perm[j3 as usize] = iVec0_tmp[(i32::wrapping_add(vsize, j3)) as usize];
			}
			/* Recursive loop 1 */
			/* Pre code */
			for j130 in 0..4 {
				fRec73_tmp[j130 as usize] = self.fRec73_perm[j130 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec73_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow38 + self.fConst2 * fRec73_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
			}
			/* Post code */
			for j131 in 0..4 {
				self.fRec73_perm[j131 as usize] = fRec73_tmp[(i32::wrapping_add(vsize, j131)) as usize];
			}
			/* Recursive loop 2 */
			/* Pre code */
			for j184 in 0..4 {
				fRec131_tmp[j184 as usize] = self.fRec131_perm[j184 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec131_tmp[(i32::wrapping_add(4, i)) as usize] = self.fConst1 * unsafe { ftbl1mydspSIG1[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow49 + fRec73_tmp[(i32::wrapping_add(4, i)) as usize]) + 0.5) as i32, 2047))) as usize] } + self.fConst2 * fRec131_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
			}
			/* Post code */
			for j185 in 0..4 {
				self.fRec131_perm[j185 as usize] = fRec131_tmp[(i32::wrapping_add(vsize, j185)) as usize];
			}
			/* Recursive loop 3 */
			/* Pre code */
			for j378 in 0..4 {
				fRec312_tmp[j378 as usize] = self.fRec312_perm[j378 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec312_tmp[(i32::wrapping_add(4, i)) as usize] = self.fConst1 * unsafe { ftbl1mydspSIG1[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow58 + fRec73_tmp[(i32::wrapping_add(4, i)) as usize]) + 0.5) as i32, 2047))) as usize] } + self.fConst2 * fRec312_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
			}
			/* Post code */
			for j379 in 0..4 {
				self.fRec312_perm[j379 as usize] = fRec312_tmp[(i32::wrapping_add(vsize, j379)) as usize];
			}
			/* Recursive loop 4 */
			/* Pre code */
			for j234 in 0..4 {
				fRec177_tmp[j234 as usize] = self.fRec177_perm[j234 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec177_tmp[(i32::wrapping_add(4, i)) as usize] = self.fConst1 * unsafe { ftbl1mydspSIG1[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow52 + fRec73_tmp[(i32::wrapping_add(4, i)) as usize]) + 0.5) as i32, 2047))) as usize] } + self.fConst2 * fRec177_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
			}
			/* Post code */
			for j235 in 0..4 {
				self.fRec177_perm[j235 as usize] = fRec177_tmp[(i32::wrapping_add(vsize, j235)) as usize];
			}
			/* Recursive loop 5 */
			/* Pre code */
			for j282 in 0..4 {
				fRec222_tmp[j282 as usize] = self.fRec222_perm[j282 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec222_tmp[(i32::wrapping_add(4, i)) as usize] = self.fConst1 * unsafe { ftbl1mydspSIG1[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow54 + fRec73_tmp[(i32::wrapping_add(4, i)) as usize]) + 0.5) as i32, 2047))) as usize] } + self.fConst2 * fRec222_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
			}
			/* Post code */
			for j283 in 0..4 {
				self.fRec222_perm[j283 as usize] = fRec222_tmp[(i32::wrapping_add(vsize, j283)) as usize];
			}
			/* Recursive loop 6 */
			/* Pre code */
			for j330 in 0..4 {
				fRec267_tmp[j330 as usize] = self.fRec267_perm[j330 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec267_tmp[(i32::wrapping_add(4, i)) as usize] = self.fConst1 * unsafe { ftbl1mydspSIG1[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow56 + fRec73_tmp[(i32::wrapping_add(4, i)) as usize]) + 0.5) as i32, 2047))) as usize] } + self.fConst2 * fRec267_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
			}
			/* Post code */
			for j331 in 0..4 {
				self.fRec267_perm[j331 as usize] = fRec267_tmp[(i32::wrapping_add(vsize, j331)) as usize];
			}
			/* Vectorizable loop 7 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec119[i as usize] = self.fConst9 * (3.4e+02 / fRec131_tmp[(i32::wrapping_add(4, i)) as usize] + -0.11);
			}
			/* Vectorizable loop 8 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec320[i as usize] = self.fConst9 * (3.4e+02 / fRec312_tmp[(i32::wrapping_add(4, i)) as usize] + -0.11);
			}
			/* Vectorizable loop 9 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec170[i as usize] = self.fConst9 * (3.4e+02 / fRec177_tmp[(i32::wrapping_add(4, i)) as usize] + -0.11);
			}
			/* Vectorizable loop 10 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec220[i as usize] = self.fConst9 * (3.4e+02 / fRec222_tmp[(i32::wrapping_add(4, i)) as usize] + -0.11);
			}
			/* Vectorizable loop 11 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec270[i as usize] = self.fConst9 * (3.4e+02 / fRec267_tmp[(i32::wrapping_add(4, i)) as usize] + -0.11);
			}
			/* Recursive loop 12 */
			/* Pre code */
			for j22 in 0..4 {
				fRec25_tmp[j22 as usize] = self.fRec25_perm[j22 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec25_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow16 + self.fConst2 * fRec25_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
			}
			/* Post code */
			for j23 in 0..4 {
				self.fRec25_perm[j23 as usize] = fRec25_tmp[(i32::wrapping_add(vsize, j23)) as usize];
			}
			/* Vectorizable loop 13 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec120[i as usize] = fZec119[i as usize] + -1.499995;
			}
			/* Vectorizable loop 14 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec150[i as usize] = F32::tan(self.fConst10 * fRec131_tmp[(i32::wrapping_add(4, i)) as usize]);
			}
			/* Vectorizable loop 15 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec321[i as usize] = fZec320[i as usize] + -1.499995;
			}
			/* Vectorizable loop 16 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec301[i as usize] = F32::tan(self.fConst10 * fRec267_tmp[(i32::wrapping_add(4, i)) as usize]);
			}
			/* Vectorizable loop 17 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec351[i as usize] = F32::tan(self.fConst10 * fRec312_tmp[(i32::wrapping_add(4, i)) as usize]);
			}
			/* Recursive loop 18 */
			/* Pre code */
			for j38 in 0..4 {
				fRec32_tmp[j38 as usize] = self.fRec32_perm[j38 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec32_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow19 + self.fConst2 * fRec32_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
			}
			/* Post code */
			for j39 in 0..4 {
				self.fRec32_perm[j39 as usize] = fRec32_tmp[(i32::wrapping_add(vsize, j39)) as usize];
			}
			/* Recursive loop 19 */
			/* Pre code */
			for j30 in 0..4 {
				fRec29_tmp[j30 as usize] = self.fRec29_perm[j30 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec29_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow18 + self.fConst2 * fRec29_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
			}
			/* Post code */
			for j31 in 0..4 {
				self.fRec29_perm[j31 as usize] = fRec29_tmp[(i32::wrapping_add(vsize, j31)) as usize];
			}
			/* Recursive loop 20 */
			/* Pre code */
			for j56 in 0..4 {
				fRec39_tmp[j56 as usize] = self.fRec39_perm[j56 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec39_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow23 + self.fConst2 * fRec39_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
			}
			/* Post code */
			for j57 in 0..4 {
				self.fRec39_perm[j57 as usize] = fRec39_tmp[(i32::wrapping_add(vsize, j57)) as usize];
			}
			/* Recursive loop 21 */
			/* Pre code */
			for j48 in 0..4 {
				fRec36_tmp[j48 as usize] = self.fRec36_perm[j48 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec36_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow22 + self.fConst2 * fRec36_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
			}
			/* Post code */
			for j49 in 0..4 {
				self.fRec36_perm[j49 as usize] = fRec36_tmp[(i32::wrapping_add(vsize, j49)) as usize];
			}
			/* Recursive loop 22 */
			/* Pre code */
			for j100 in 0..4 {
				fRec56_tmp[j100 as usize] = self.fRec56_perm[j100 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec56_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow32 + self.fConst2 * fRec56_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
			}
			/* Post code */
			for j101 in 0..4 {
				self.fRec56_perm[j101 as usize] = fRec56_tmp[(i32::wrapping_add(vsize, j101)) as usize];
			}
			/* Recursive loop 23 */
			/* Pre code */
			for j116 in 0..4 {
				fRec62_tmp[j116 as usize] = self.fRec62_perm[j116 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec62_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow34 + self.fConst2 * fRec62_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
			}
			/* Post code */
			for j117 in 0..4 {
				self.fRec62_perm[j117 as usize] = fRec62_tmp[(i32::wrapping_add(vsize, j117)) as usize];
			}
			/* Recursive loop 24 */
			/* Pre code */
			for j108 in 0..4 {
				fRec59_tmp[j108 as usize] = self.fRec59_perm[j108 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec59_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow33 + self.fConst2 * fRec59_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
			}
			/* Post code */
			for j109 in 0..4 {
				self.fRec59_perm[j109 as usize] = fRec59_tmp[(i32::wrapping_add(vsize, j109)) as usize];
			}
			/* Recursive loop 25 */
			/* Pre code */
			for j90 in 0..4 {
				fRec52_tmp[j90 as usize] = self.fRec52_perm[j90 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec52_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow29 + self.fConst2 * fRec52_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
			}
			/* Post code */
			for j91 in 0..4 {
				self.fRec52_perm[j91 as usize] = fRec52_tmp[(i32::wrapping_add(vsize, j91)) as usize];
			}
			/* Recursive loop 26 */
			/* Pre code */
			for j138 in 0..4 {
				fRec76_tmp[j138 as usize] = self.fRec76_perm[j138 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec76_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow41 + self.fConst2 * fRec76_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
			}
			/* Post code */
			for j139 in 0..4 {
				self.fRec76_perm[j139 as usize] = fRec76_tmp[(i32::wrapping_add(vsize, j139)) as usize];
			}
			/* Recursive loop 27 */
			/* Pre code */
			for j64 in 0..4 {
				fRec42_tmp[j64 as usize] = self.fRec42_perm[j64 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec42_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow24 + self.fConst2 * fRec42_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
			}
			/* Post code */
			for j65 in 0..4 {
				self.fRec42_perm[j65 as usize] = fRec42_tmp[(i32::wrapping_add(vsize, j65)) as usize];
			}
			/* Recursive loop 28 */
			/* Pre code */
			for j82 in 0..4 {
				fRec49_tmp[j82 as usize] = self.fRec49_perm[j82 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec49_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow28 + self.fConst2 * fRec49_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
			}
			/* Post code */
			for j83 in 0..4 {
				self.fRec49_perm[j83 as usize] = fRec49_tmp[(i32::wrapping_add(vsize, j83)) as usize];
			}
			/* Recursive loop 29 */
			/* Pre code */
			for j74 in 0..4 {
				fRec46_tmp[j74 as usize] = self.fRec46_perm[j74 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec46_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow27 + self.fConst2 * fRec46_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
			}
			/* Post code */
			for j75 in 0..4 {
				self.fRec46_perm[j75 as usize] = fRec46_tmp[(i32::wrapping_add(vsize, j75)) as usize];
			}
			/* Vectorizable loop 30 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec171[i as usize] = fZec170[i as usize] + -1.499995;
			}
			/* Vectorizable loop 31 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec201[i as usize] = F32::tan(self.fConst10 * fRec177_tmp[(i32::wrapping_add(4, i)) as usize]);
			}
			/* Vectorizable loop 32 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec221[i as usize] = fZec220[i as usize] + -1.499995;
			}
			/* Vectorizable loop 33 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec251[i as usize] = F32::tan(self.fConst10 * fRec222_tmp[(i32::wrapping_add(4, i)) as usize]);
			}
			/* Vectorizable loop 34 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec271[i as usize] = fZec270[i as usize] + -1.499995;
			}
			/* Vectorizable loop 35 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec1[i as usize] = F32::max(fRec25_tmp[(i32::wrapping_add(4, i)) as usize], 23.44895);
			}
			/* Vectorizable loop 36 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec124[i as usize] = F32::floor(fZec120[i as usize]);
			}
			/* Vectorizable loop 37 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec151[i as usize] = 1.0 / fZec150[i as usize];
			}
			/* Vectorizable loop 38 */
			/* Pre code */
			for j194 in 0..4 {
				fVec1_tmp[j194 as usize] = self.fVec1_perm[j194 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fVec1_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow50;
			}
			/* Post code */
			for j195 in 0..4 {
				self.fVec1_perm[j195 as usize] = fVec1_tmp[(i32::wrapping_add(vsize, j195)) as usize];
			}
			/* Recursive loop 39 */
			/* Pre code */
			for j190 in 0..4 {
				iRec135_tmp[j190 as usize] = self.iRec135_perm[j190 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iRec135_tmp[(i32::wrapping_add(4, i)) as usize] = i32::wrapping_add(i32::wrapping_mul(1103515245, iRec135_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]), 12345);
			}
			/* Post code */
			for j191 in 0..4 {
				self.iRec135_perm[j191 as usize] = iRec135_tmp[(i32::wrapping_add(vsize, j191)) as usize];
			}
			/* Vectorizable loop 40 */
			/* Pre code */
			for j338 in 0..4 {
				fVec4_tmp[j338 as usize] = self.fVec4_perm[j338 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fVec4_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow57;
			}
			/* Post code */
			for j339 in 0..4 {
				self.fVec4_perm[j339 as usize] = fVec4_tmp[(i32::wrapping_add(vsize, j339)) as usize];
			}
			/* Vectorizable loop 41 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec325[i as usize] = F32::floor(fZec321[i as usize]);
			}
			/* Vectorizable loop 42 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec302[i as usize] = 1.0 / fZec301[i as usize];
			}
			/* Vectorizable loop 43 */
			/* Pre code */
			for j386 in 0..4 {
				fVec5_tmp[j386 as usize] = self.fVec5_perm[j386 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fVec5_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow59;
			}
			/* Post code */
			for j387 in 0..4 {
				self.fVec5_perm[j387 as usize] = fVec5_tmp[(i32::wrapping_add(vsize, j387)) as usize];
			}
			/* Vectorizable loop 44 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec352[i as usize] = 1.0 / fZec351[i as usize];
			}
			/* Vectorizable loop 45 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec8[i as usize] = F32::max(fRec29_tmp[(i32::wrapping_add(4, i)) as usize], 23.44895);
			}
			/* Vectorizable loop 46 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec20[i as usize] = F32::max(fRec36_tmp[(i32::wrapping_add(4, i)) as usize], 23.44895);
			}
			/* Vectorizable loop 47 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec14[i as usize] = F32::max(fRec32_tmp[(i32::wrapping_add(4, i)) as usize], 23.44895);
			}
			/* Vectorizable loop 48 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec50[i as usize] = F32::max(fRec52_tmp[(i32::wrapping_add(4, i)) as usize], 23.44895);
			}
			/* Vectorizable loop 49 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec56[i as usize] = F32::max(fRec56_tmp[(i32::wrapping_add(4, i)) as usize], 23.44895);
			}
			/* Vectorizable loop 50 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec62[i as usize] = F32::max(fRec59_tmp[(i32::wrapping_add(4, i)) as usize], 23.44895);
			}
			/* Vectorizable loop 51 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec68[i as usize] = F32::max(fRec62_tmp[(i32::wrapping_add(4, i)) as usize], 23.44895);
			}
			/* Recursive loop 52 */
			/* Pre code */
			for j136 in 0..4 {
				fRec74_tmp[j136 as usize] = self.fRec74_perm[j136 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec74_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow40 + self.fConst2 * fRec74_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
			}
			/* Post code */
			for j137 in 0..4 {
				self.fRec74_perm[j137 as usize] = fRec74_tmp[(i32::wrapping_add(vsize, j137)) as usize];
			}
			/* Vectorizable loop 53 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec78[i as usize] = fRec73_tmp[(i32::wrapping_add(4, i)) as usize] + fRec76_tmp[(i32::wrapping_add(4, i)) as usize];
			}
			/* Vectorizable loop 54 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec44[i as usize] = F32::max(fRec49_tmp[(i32::wrapping_add(4, i)) as usize], 23.44895);
			}
			/* Vectorizable loop 55 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec32[i as usize] = F32::max(fRec42_tmp[(i32::wrapping_add(4, i)) as usize], 23.44895);
			}
			/* Vectorizable loop 56 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec26[i as usize] = F32::max(fRec39_tmp[(i32::wrapping_add(4, i)) as usize], 23.44895);
			}
			/* Vectorizable loop 57 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec38[i as usize] = F32::max(fRec46_tmp[(i32::wrapping_add(4, i)) as usize], 23.44895);
			}
			/* Vectorizable loop 58 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec175[i as usize] = F32::floor(fZec171[i as usize]);
			}
			/* Vectorizable loop 59 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec202[i as usize] = 1.0 / fZec201[i as usize];
			}
			/* Vectorizable loop 60 */
			/* Pre code */
			for j242 in 0..4 {
				fVec2_tmp[j242 as usize] = self.fVec2_perm[j242 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fVec2_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow53;
			}
			/* Post code */
			for j243 in 0..4 {
				self.fVec2_perm[j243 as usize] = fVec2_tmp[(i32::wrapping_add(vsize, j243)) as usize];
			}
			/* Vectorizable loop 61 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec225[i as usize] = F32::floor(fZec221[i as usize]);
			}
			/* Vectorizable loop 62 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec275[i as usize] = F32::floor(fZec271[i as usize]);
			}
			/* Vectorizable loop 63 */
			/* Pre code */
			for j290 in 0..4 {
				fVec3_tmp[j290 as usize] = self.fVec3_perm[j290 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fVec3_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow55;
			}
			/* Post code */
			for j291 in 0..4 {
				self.fVec3_perm[j291 as usize] = fVec3_tmp[(i32::wrapping_add(vsize, j291)) as usize];
			}
			/* Vectorizable loop 64 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec252[i as usize] = 1.0 / fZec251[i as usize];
			}
			/* Vectorizable loop 65 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec0[i as usize] = i32::wrapping_sub(1, iVec0_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]);
			}
			/* Vectorizable loop 66 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec2[i as usize] = F32::max(2e+01, F32::abs(fZec1[i as usize]));
			}
			/* Vectorizable loop 67 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec126[i as usize] = fZec119[i as usize] + (-2.0 - fZec124[i as usize]);
			}
			/* Vectorizable loop 68 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec125[i as usize] = fZec119[i as usize] + (-3.0 - fZec124[i as usize]);
			}
			/* Vectorizable loop 69 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec130[i as usize] = fZec119[i as usize] + (-4.0 - fZec124[i as usize]);
			}
			/* Vectorizable loop 70 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec127[i as usize] = fZec119[i as usize] + (-1.0 - fZec124[i as usize]);
			}
			/* Vectorizable loop 71 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec121[i as usize] = (fZec120[i as usize]) as i32;
			}
			/* Vectorizable loop 72 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec153[i as usize] = 4.656613e-10 * (iRec135_tmp[(i32::wrapping_add(4, i)) as usize]) as F32;
			}
			/* Recursive loop 73 */
			/* Pre code */
			for j196 in 0..4 {
				iRec136_tmp[j196 as usize] = self.iRec136_perm[j196 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iRec136_tmp[(i32::wrapping_add(4, i)) as usize] = i32::wrapping_add(i32::wrapping_mul(i32::wrapping_add(iRec136_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize], (iRec136_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] > 0) as i32), (fSlow50 <= fVec1_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) as i32), (fSlow50 > fVec1_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) as i32);
			}
			/* Post code */
			for j197 in 0..4 {
				self.iRec136_perm[j197 as usize] = iRec136_tmp[(i32::wrapping_add(vsize, j197)) as usize];
			}
			/* Vectorizable loop 74 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec152[i as usize] = (fZec151[i as usize] + 1.4142135) / fZec150[i as usize] + 1.0;
			}
			/* Recursive loop 75 */
			/* Pre code */
			for j372 in 0..4 {
				fRec279_tmp[j372 as usize] = self.fRec279_perm[j372 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec279_tmp[(i32::wrapping_add(4, i)) as usize] = self.fConst1 * unsafe { ftbl1mydspSIG1[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow56 + fRec76_tmp[(i32::wrapping_add(4, i)) as usize]) + 0.5) as i32, 2047))) as usize] } + self.fConst2 * fRec279_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
			}
			/* Post code */
			for j373 in 0..4 {
				self.fRec279_perm[j373 as usize] = fRec279_tmp[(i32::wrapping_add(vsize, j373)) as usize];
			}
			/* Vectorizable loop 76 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec322[i as usize] = (fZec321[i as usize]) as i32;
			}
			/* Recursive loop 77 */
			/* Pre code */
			for j340 in 0..4 {
				iRec271_tmp[j340 as usize] = self.iRec271_perm[j340 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iRec271_tmp[(i32::wrapping_add(4, i)) as usize] = i32::wrapping_add(i32::wrapping_mul(i32::wrapping_add(iRec271_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize], (iRec271_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] > 0) as i32), (fSlow57 <= fVec4_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) as i32), (fSlow57 > fVec4_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) as i32);
			}
			/* Post code */
			for j341 in 0..4 {
				self.iRec271_perm[j341 as usize] = iRec271_tmp[(i32::wrapping_add(vsize, j341)) as usize];
			}
			/* Vectorizable loop 78 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec326[i as usize] = fZec320[i as usize] + (-3.0 - fZec325[i as usize]);
			}
			/* Vectorizable loop 79 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec303[i as usize] = (fZec302[i as usize] + 1.4142135) / fZec301[i as usize] + 1.0;
			}
			/* Vectorizable loop 80 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec328[i as usize] = fZec320[i as usize] + (-1.0 - fZec325[i as usize]);
			}
			/* Vectorizable loop 81 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec327[i as usize] = fZec320[i as usize] + (-2.0 - fZec325[i as usize]);
			}
			/* Vectorizable loop 82 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec331[i as usize] = fZec320[i as usize] + (-4.0 - fZec325[i as usize]);
			}
			/* Vectorizable loop 83 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec353[i as usize] = (fZec352[i as usize] + 1.4142135) / fZec351[i as usize] + 1.0;
			}
			/* Recursive loop 84 */
			/* Pre code */
			for j420 in 0..4 {
				fRec324_tmp[j420 as usize] = self.fRec324_perm[j420 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec324_tmp[(i32::wrapping_add(4, i)) as usize] = self.fConst1 * unsafe { ftbl1mydspSIG1[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow58 + fRec76_tmp[(i32::wrapping_add(4, i)) as usize]) + 0.5) as i32, 2047))) as usize] } + self.fConst2 * fRec324_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
			}
			/* Post code */
			for j421 in 0..4 {
				self.fRec324_perm[j421 as usize] = fRec324_tmp[(i32::wrapping_add(vsize, j421)) as usize];
			}
			/* Recursive loop 85 */
			/* Pre code */
			for j388 in 0..4 {
				iRec316_tmp[j388 as usize] = self.iRec316_perm[j388 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iRec316_tmp[(i32::wrapping_add(4, i)) as usize] = i32::wrapping_add(i32::wrapping_mul(i32::wrapping_add(iRec316_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize], (iRec316_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] > 0) as i32), (fSlow59 <= fVec5_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) as i32), (fSlow59 > fVec5_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) as i32);
			}
			/* Post code */
			for j389 in 0..4 {
				self.iRec316_perm[j389 as usize] = iRec316_tmp[(i32::wrapping_add(vsize, j389)) as usize];
			}
			/* Vectorizable loop 86 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec9[i as usize] = F32::max(2e+01, F32::abs(fZec8[i as usize]));
			}
			/* Vectorizable loop 87 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec21[i as usize] = F32::max(2e+01, F32::abs(fZec20[i as usize]));
			}
			/* Vectorizable loop 88 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec15[i as usize] = F32::max(2e+01, F32::abs(fZec14[i as usize]));
			}
			/* Vectorizable loop 89 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec51[i as usize] = F32::max(2e+01, F32::abs(fZec50[i as usize]));
			}
			/* Vectorizable loop 90 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec63[i as usize] = F32::max(2e+01, F32::abs(fZec62[i as usize]));
			}
			/* Vectorizable loop 91 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec57[i as usize] = F32::max(2e+01, F32::abs(fZec56[i as usize]));
			}
			/* Recursive loop 92 */
			/* Pre code */
			for j140 in 0..4 {
				fRec75_tmp[j140 as usize] = self.fRec75_perm[j140 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec75_tmp[(i32::wrapping_add(4, i)) as usize] = self.fConst1 * unsafe { ftbl1mydspSIG1[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow39 + fZec78[i as usize]) + 0.5) as i32, 2047))) as usize] } + self.fConst2 * fRec75_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
			}
			/* Post code */
			for j141 in 0..4 {
				self.fRec75_perm[j141 as usize] = fRec75_tmp[(i32::wrapping_add(vsize, j141)) as usize];
			}
			/* Vectorizable loop 93 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec69[i as usize] = F32::max(2e+01, F32::abs(fZec68[i as usize]));
			}
			/* Vectorizable loop 94 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec80[i as usize] = F32::min(1.4141995, 1.4142135 * fRec74_tmp[(i32::wrapping_add(4, i)) as usize]);
			}
			/* Recursive loop 95 */
			/* Pre code */
			for j152 in 0..4 {
				fRec83_tmp[j152 as usize] = self.fRec83_perm[j152 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec83_tmp[(i32::wrapping_add(4, i)) as usize] = self.fConst1 * unsafe { ftbl1mydspSIG1[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow43 + fZec78[i as usize]) + 0.5) as i32, 2047))) as usize] } + self.fConst2 * fRec83_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
			}
			/* Post code */
			for j153 in 0..4 {
				self.fRec83_perm[j153 as usize] = fRec83_tmp[(i32::wrapping_add(vsize, j153)) as usize];
			}
			/* Recursive loop 96 */
			/* Pre code */
			for j176 in 0..4 {
				fRec97_tmp[j176 as usize] = self.fRec97_perm[j176 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec97_tmp[(i32::wrapping_add(4, i)) as usize] = self.fConst1 * unsafe { ftbl1mydspSIG1[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow47 + fZec78[i as usize]) + 0.5) as i32, 2047))) as usize] } + self.fConst2 * fRec97_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
			}
			/* Post code */
			for j177 in 0..4 {
				self.fRec97_perm[j177 as usize] = fRec97_tmp[(i32::wrapping_add(vsize, j177)) as usize];
			}
			/* Recursive loop 97 */
			/* Pre code */
			for j148 in 0..4 {
				fRec82_tmp[j148 as usize] = self.fRec82_perm[j148 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec82_tmp[(i32::wrapping_add(4, i)) as usize] = self.fConst1 * unsafe { ftbl1mydspSIG1[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow43 + fRec73_tmp[(i32::wrapping_add(4, i)) as usize]) + 0.5) as i32, 2047))) as usize] } + self.fConst2 * fRec82_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
			}
			/* Post code */
			for j149 in 0..4 {
				self.fRec82_perm[j149 as usize] = fRec82_tmp[(i32::wrapping_add(vsize, j149)) as usize];
			}
			/* Recursive loop 98 */
			/* Pre code */
			for j164 in 0..4 {
				fRec90_tmp[j164 as usize] = self.fRec90_perm[j164 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec90_tmp[(i32::wrapping_add(4, i)) as usize] = self.fConst1 * unsafe { ftbl1mydspSIG1[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow45 + fZec78[i as usize]) + 0.5) as i32, 2047))) as usize] } + self.fConst2 * fRec90_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
			}
			/* Post code */
			for j165 in 0..4 {
				self.fRec90_perm[j165 as usize] = fRec90_tmp[(i32::wrapping_add(vsize, j165)) as usize];
			}
			/* Vectorizable loop 99 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec39[i as usize] = F32::max(2e+01, F32::abs(fZec38[i as usize]));
			}
			/* Vectorizable loop 100 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec33[i as usize] = F32::max(2e+01, F32::abs(fZec32[i as usize]));
			}
			/* Vectorizable loop 101 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec27[i as usize] = F32::max(2e+01, F32::abs(fZec26[i as usize]));
			}
			/* Vectorizable loop 102 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec45[i as usize] = F32::max(2e+01, F32::abs(fZec44[i as usize]));
			}
			/* Vectorizable loop 103 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec176[i as usize] = fZec170[i as usize] + (-3.0 - fZec175[i as usize]);
			}
			/* Vectorizable loop 104 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec177[i as usize] = fZec170[i as usize] + (-2.0 - fZec175[i as usize]);
			}
			/* Recursive loop 105 */
			/* Pre code */
			for j228 in 0..4 {
				fRec144_tmp[j228 as usize] = self.fRec144_perm[j228 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec144_tmp[(i32::wrapping_add(4, i)) as usize] = self.fConst1 * unsafe { ftbl1mydspSIG1[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow49 + fRec76_tmp[(i32::wrapping_add(4, i)) as usize]) + 0.5) as i32, 2047))) as usize] } + self.fConst2 * fRec144_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
			}
			/* Post code */
			for j229 in 0..4 {
				self.fRec144_perm[j229 as usize] = fRec144_tmp[(i32::wrapping_add(vsize, j229)) as usize];
			}
			/* Vectorizable loop 106 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec178[i as usize] = fZec170[i as usize] + (-1.0 - fZec175[i as usize]);
			}
			/* Vectorizable loop 107 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec172[i as usize] = (fZec171[i as usize]) as i32;
			}
			/* Vectorizable loop 108 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec181[i as usize] = fZec170[i as usize] + (-4.0 - fZec175[i as usize]);
			}
			/* Recursive loop 109 */
			/* Pre code */
			for j276 in 0..4 {
				fRec189_tmp[j276 as usize] = self.fRec189_perm[j276 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec189_tmp[(i32::wrapping_add(4, i)) as usize] = self.fConst1 * unsafe { ftbl1mydspSIG1[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow52 + fRec76_tmp[(i32::wrapping_add(4, i)) as usize]) + 0.5) as i32, 2047))) as usize] } + self.fConst2 * fRec189_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
			}
			/* Post code */
			for j277 in 0..4 {
				self.fRec189_perm[j277 as usize] = fRec189_tmp[(i32::wrapping_add(vsize, j277)) as usize];
			}
			/* Vectorizable loop 110 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec222[i as usize] = (fZec221[i as usize]) as i32;
			}
			/* Vectorizable loop 111 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec226[i as usize] = fZec220[i as usize] + (-3.0 - fZec225[i as usize]);
			}
			/* Recursive loop 112 */
			/* Pre code */
			for j244 in 0..4 {
				iRec181_tmp[j244 as usize] = self.iRec181_perm[j244 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iRec181_tmp[(i32::wrapping_add(4, i)) as usize] = i32::wrapping_add(i32::wrapping_mul(i32::wrapping_add(iRec181_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize], (iRec181_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] > 0) as i32), (fSlow53 <= fVec2_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) as i32), (fSlow53 > fVec2_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) as i32);
			}
			/* Post code */
			for j245 in 0..4 {
				self.iRec181_perm[j245 as usize] = iRec181_tmp[(i32::wrapping_add(vsize, j245)) as usize];
			}
			/* Vectorizable loop 113 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec203[i as usize] = (fZec202[i as usize] + 1.4142135) / fZec201[i as usize] + 1.0;
			}
			/* Vectorizable loop 114 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec228[i as usize] = fZec220[i as usize] + (-1.0 - fZec225[i as usize]);
			}
			/* Vectorizable loop 115 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec227[i as usize] = fZec220[i as usize] + (-2.0 - fZec225[i as usize]);
			}
			/* Vectorizable loop 116 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec231[i as usize] = fZec220[i as usize] + (-4.0 - fZec225[i as usize]);
			}
			/* Vectorizable loop 117 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec272[i as usize] = (fZec271[i as usize]) as i32;
			}
			/* Vectorizable loop 118 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec276[i as usize] = fZec270[i as usize] + (-3.0 - fZec275[i as usize]);
			}
			/* Vectorizable loop 119 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec253[i as usize] = (fZec252[i as usize] + 1.4142135) / fZec251[i as usize] + 1.0;
			}
			/* Recursive loop 120 */
			/* Pre code */
			for j324 in 0..4 {
				fRec234_tmp[j324 as usize] = self.fRec234_perm[j324 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec234_tmp[(i32::wrapping_add(4, i)) as usize] = self.fConst1 * unsafe { ftbl1mydspSIG1[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow54 + fRec76_tmp[(i32::wrapping_add(4, i)) as usize]) + 0.5) as i32, 2047))) as usize] } + self.fConst2 * fRec234_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
			}
			/* Post code */
			for j325 in 0..4 {
				self.fRec234_perm[j325 as usize] = fRec234_tmp[(i32::wrapping_add(vsize, j325)) as usize];
			}
			/* Recursive loop 121 */
			/* Pre code */
			for j292 in 0..4 {
				iRec226_tmp[j292 as usize] = self.iRec226_perm[j292 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iRec226_tmp[(i32::wrapping_add(4, i)) as usize] = i32::wrapping_add(i32::wrapping_mul(i32::wrapping_add(iRec226_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize], (iRec226_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] > 0) as i32), (fSlow55 <= fVec3_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) as i32), (fSlow55 > fVec3_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) as i32);
			}
			/* Post code */
			for j293 in 0..4 {
				self.iRec226_perm[j293 as usize] = iRec226_tmp[(i32::wrapping_add(vsize, j293)) as usize];
			}
			/* Vectorizable loop 122 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec278[i as usize] = fZec270[i as usize] + (-1.0 - fZec275[i as usize]);
			}
			/* Vectorizable loop 123 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec281[i as usize] = fZec270[i as usize] + (-4.0 - fZec275[i as usize]);
			}
			/* Vectorizable loop 124 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec277[i as usize] = fZec270[i as usize] + (-2.0 - fZec275[i as usize]);
			}
			/* Recursive loop 125 */
			/* Pre code */
			for j24 in 0..4 {
				fRec28_tmp[j24 as usize] = self.fRec28_perm[j24 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec3[i as usize] = if iZec0[i as usize] != 0 {0.0} else {fRec28_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + self.fConst4 * fZec2[i as usize]};
				fRec28_tmp[(i32::wrapping_add(4, i)) as usize] = fZec3[i as usize] - F32::floor(fZec3[i as usize]);
			}
			/* Post code */
			for j25 in 0..4 {
				self.fRec28_perm[j25 as usize] = fRec28_tmp[(i32::wrapping_add(vsize, j25)) as usize];
			}
			/* Vectorizable loop 126 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec132[i as usize] = (F32::min(self.fConst8, (std::cmp::max(0, i32::wrapping_add(iZec121[i as usize], 3))) as F32)) as i32;
			}
			/* Vectorizable loop 127 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec135[i as usize] = 0.0 - fZec125[i as usize];
			}
			/* Vectorizable loop 128 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec136[i as usize] = (F32::min(self.fConst8, (std::cmp::max(0, i32::wrapping_add(iZec121[i as usize], 2))) as F32)) as i32;
			}
			/* Vectorizable loop 129 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec114[i as usize] = F32::tan(self.fConst7 * F32::max(2e+01, F32::min(1e+04, fRec97_tmp[(i32::wrapping_add(4, i)) as usize])));
			}
			/* Vectorizable loop 130 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec122[i as usize] = (F32::min(self.fConst8, (std::cmp::max(0, i32::wrapping_add(iZec121[i as usize], 4))) as F32)) as i32;
			}
			/* Vectorizable loop 131 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec131[i as usize] = 0.0 - fZec130[i as usize];
			}
			/* Vectorizable loop 132 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec134[i as usize] = 0.0 - 0.5 * fZec130[i as usize];
			}
			/* Vectorizable loop 133 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec138[i as usize] = 0.0 - 0.33333334 * fZec130[i as usize];
			}
			/* Vectorizable loop 134 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec139[i as usize] = 0.0 - 0.5 * fZec125[i as usize];
			}
			/* Vectorizable loop 135 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec128[i as usize] = fZec127[i as usize] * fZec126[i as usize];
			}
			/* Vectorizable loop 136 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec146[i as usize] = 0.0 - 0.5 * fZec126[i as usize];
			}
			/* Vectorizable loop 137 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec147[i as usize] = 0.0 - fZec127[i as usize];
			}
			/* Recursive loop 138 */
			/* Pre code */
			for j192 in 0..4 {
				fRec134_tmp[j192 as usize] = self.fRec134_perm[j192 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec134_tmp[(i32::wrapping_add(4, i)) as usize] = fZec153[i as usize] - (fRec134_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] * ((fZec151[i as usize] + -1.4142135) / fZec150[i as usize] + 1.0) + 2.0 * fRec134_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] * (1.0 - 1.0 / mydsp_faustpower2_f(fZec150[i as usize]))) / fZec152[i as usize];
			}
			/* Post code */
			for j193 in 0..4 {
				self.fRec134_perm[j193 as usize] = fRec134_tmp[(i32::wrapping_add(vsize, j193)) as usize];
			}
			/* Vectorizable loop 139 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec144[i as usize] = 0.0 - 0.25 * fZec130[i as usize];
			}
			/* Vectorizable loop 140 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec145[i as usize] = 0.0 - 0.33333334 * fZec125[i as usize];
			}
			/* Vectorizable loop 141 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec141[i as usize] = (F32::min(self.fConst8, (std::cmp::max(0, i32::wrapping_add(iZec121[i as usize], 1))) as F32)) as i32;
			}
			/* Vectorizable loop 142 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec148[i as usize] = (F32::min(self.fConst8, (std::cmp::max(0, iZec121[i as usize])) as F32)) as i32;
			}
			/* Vectorizable loop 143 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec140[i as usize] = 0.0 - fZec126[i as usize];
			}
			/* Vectorizable loop 144 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec161[i as usize] = (iRec136_tmp[(i32::wrapping_add(4, i)) as usize]) as F32 / F32::max(1.0, self.fConst15 * mydsp_faustpower2_f(1.0 - 0.0005 * fRec131_tmp[(i32::wrapping_add(4, i)) as usize]));
			}
			/* Vectorizable loop 145 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec315[i as usize] = F32::tan(self.fConst7 * F32::max(2e+01, F32::min(1e+04, fRec279_tmp[(i32::wrapping_add(4, i)) as usize])));
			}
			/* Vectorizable loop 146 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec299[i as usize] = (F32::min(self.fConst8, (std::cmp::max(0, iZec272[i as usize])) as F32)) as i32;
			}
			/* Vectorizable loop 147 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec297[i as usize] = 0.0 - 0.5 * fZec277[i as usize];
			}
			/* Vectorizable loop 148 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec292[i as usize] = (F32::min(self.fConst8, (std::cmp::max(0, i32::wrapping_add(iZec272[i as usize], 1))) as F32)) as i32;
			}
			/* Vectorizable loop 149 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec295[i as usize] = 0.0 - 0.25 * fZec281[i as usize];
			}
			/* Vectorizable loop 150 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec311[i as usize] = (iRec271_tmp[(i32::wrapping_add(4, i)) as usize]) as F32 / F32::max(1.0, self.fConst15 * mydsp_faustpower2_f(1.0 - 0.0005 * fRec267_tmp[(i32::wrapping_add(4, i)) as usize]));
			}
			/* Vectorizable loop 151 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec296[i as usize] = 0.0 - 0.33333334 * fZec276[i as usize];
			}
			/* Vectorizable loop 152 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec323[i as usize] = (F32::min(self.fConst8, (std::cmp::max(0, i32::wrapping_add(iZec322[i as usize], 4))) as F32)) as i32;
			}
			/* Recursive loop 153 */
			/* Pre code */
			for j336 in 0..4 {
				fRec270_tmp[j336 as usize] = self.fRec270_perm[j336 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec270_tmp[(i32::wrapping_add(4, i)) as usize] = fZec153[i as usize] - (fRec270_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] * ((fZec302[i as usize] + -1.4142135) / fZec301[i as usize] + 1.0) + 2.0 * fRec270_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] * (1.0 - 1.0 / mydsp_faustpower2_f(fZec301[i as usize]))) / fZec303[i as usize];
			}
			/* Post code */
			for j337 in 0..4 {
				self.fRec270_perm[j337 as usize] = fRec270_tmp[(i32::wrapping_add(vsize, j337)) as usize];
			}
			/* Vectorizable loop 154 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec329[i as usize] = fZec328[i as usize] * fZec327[i as usize];
			}
			/* Vectorizable loop 155 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec298[i as usize] = 0.0 - fZec278[i as usize];
			}
			/* Vectorizable loop 156 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec333[i as usize] = (F32::min(self.fConst8, (std::cmp::max(0, i32::wrapping_add(iZec322[i as usize], 3))) as F32)) as i32;
			}
			/* Vectorizable loop 157 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec335[i as usize] = 0.0 - 0.5 * fZec331[i as usize];
			}
			/* Vectorizable loop 158 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec336[i as usize] = 0.0 - fZec326[i as usize];
			}
			/* Vectorizable loop 159 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec332[i as usize] = 0.0 - fZec331[i as usize];
			}
			/* Vectorizable loop 160 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec337[i as usize] = (F32::min(self.fConst8, (std::cmp::max(0, i32::wrapping_add(iZec322[i as usize], 2))) as F32)) as i32;
			}
			/* Vectorizable loop 161 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec339[i as usize] = 0.0 - 0.33333334 * fZec331[i as usize];
			}
			/* Vectorizable loop 162 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec340[i as usize] = 0.0 - 0.5 * fZec326[i as usize];
			}
			/* Vectorizable loop 163 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec341[i as usize] = 0.0 - fZec327[i as usize];
			}
			/* Vectorizable loop 164 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec349[i as usize] = (F32::min(self.fConst8, (std::cmp::max(0, iZec322[i as usize])) as F32)) as i32;
			}
			/* Recursive loop 165 */
			/* Pre code */
			for j384 in 0..4 {
				fRec315_tmp[j384 as usize] = self.fRec315_perm[j384 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec315_tmp[(i32::wrapping_add(4, i)) as usize] = fZec153[i as usize] - (fRec315_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] * ((fZec352[i as usize] + -1.4142135) / fZec351[i as usize] + 1.0) + 2.0 * fRec315_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] * (1.0 - 1.0 / mydsp_faustpower2_f(fZec351[i as usize]))) / fZec353[i as usize];
			}
			/* Post code */
			for j385 in 0..4 {
				self.fRec315_perm[j385 as usize] = fRec315_tmp[(i32::wrapping_add(vsize, j385)) as usize];
			}
			/* Vectorizable loop 166 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec342[i as usize] = (F32::min(self.fConst8, (std::cmp::max(0, i32::wrapping_add(iZec322[i as usize], 1))) as F32)) as i32;
			}
			/* Vectorizable loop 167 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec346[i as usize] = 0.0 - 0.33333334 * fZec326[i as usize];
			}
			/* Vectorizable loop 168 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec347[i as usize] = 0.0 - 0.5 * fZec327[i as usize];
			}
			/* Vectorizable loop 169 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec348[i as usize] = 0.0 - fZec328[i as usize];
			}
			/* Vectorizable loop 170 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec361[i as usize] = (iRec316_tmp[(i32::wrapping_add(4, i)) as usize]) as F32 / F32::max(1.0, self.fConst15 * mydsp_faustpower2_f(1.0 - 0.0005 * fRec312_tmp[(i32::wrapping_add(4, i)) as usize]));
			}
			/* Vectorizable loop 171 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec345[i as usize] = 0.0 - 0.25 * fZec331[i as usize];
			}
			/* Vectorizable loop 172 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec365[i as usize] = F32::tan(self.fConst7 * F32::max(2e+01, F32::min(1e+04, fRec324_tmp[(i32::wrapping_add(4, i)) as usize])));
			}
			/* Recursive loop 173 */
			/* Pre code */
			for j50 in 0..4 {
				fRec38_tmp[j50 as usize] = self.fRec38_perm[j50 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec22[i as usize] = if iZec0[i as usize] != 0 {0.0} else {fRec38_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + self.fConst4 * fZec21[i as usize]};
				fRec38_tmp[(i32::wrapping_add(4, i)) as usize] = fZec22[i as usize] - F32::floor(fZec22[i as usize]);
			}
			/* Post code */
			for j51 in 0..4 {
				self.fRec38_perm[j51 as usize] = fRec38_tmp[(i32::wrapping_add(vsize, j51)) as usize];
			}
			/* Recursive loop 174 */
			/* Pre code */
			for j58 in 0..4 {
				fRec41_tmp[j58 as usize] = self.fRec41_perm[j58 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec28[i as usize] = if iZec0[i as usize] != 0 {0.0} else {fRec41_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + self.fConst4 * fZec27[i as usize]};
				fRec41_tmp[(i32::wrapping_add(4, i)) as usize] = fZec28[i as usize] - F32::floor(fZec28[i as usize]);
			}
			/* Post code */
			for j59 in 0..4 {
				self.fRec41_perm[j59 as usize] = fRec41_tmp[(i32::wrapping_add(vsize, j59)) as usize];
			}
			/* Recursive loop 175 */
			/* Pre code */
			for j40 in 0..4 {
				fRec34_tmp[j40 as usize] = self.fRec34_perm[j40 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec16[i as usize] = if iZec0[i as usize] != 0 {0.0} else {fRec34_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + self.fConst4 * fZec15[i as usize]};
				fRec34_tmp[(i32::wrapping_add(4, i)) as usize] = fZec16[i as usize] - F32::floor(fZec16[i as usize]);
			}
			/* Post code */
			for j41 in 0..4 {
				self.fRec34_perm[j41 as usize] = fRec34_tmp[(i32::wrapping_add(vsize, j41)) as usize];
			}
			/* Recursive loop 176 */
			/* Pre code */
			for j32 in 0..4 {
				fRec31_tmp[j32 as usize] = self.fRec31_perm[j32 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec10[i as usize] = if iZec0[i as usize] != 0 {0.0} else {fRec31_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + self.fConst4 * fZec9[i as usize]};
				fRec31_tmp[(i32::wrapping_add(4, i)) as usize] = fZec10[i as usize] - F32::floor(fZec10[i as usize]);
			}
			/* Post code */
			for j33 in 0..4 {
				self.fRec31_perm[j33 as usize] = fRec31_tmp[(i32::wrapping_add(vsize, j33)) as usize];
			}
			/* Recursive loop 177 */
			/* Pre code */
			for j102 in 0..4 {
				fRec58_tmp[j102 as usize] = self.fRec58_perm[j102 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec58[i as usize] = if iZec0[i as usize] != 0 {0.0} else {fRec58_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + self.fConst4 * fZec57[i as usize]};
				fRec58_tmp[(i32::wrapping_add(4, i)) as usize] = fZec58[i as usize] - F32::floor(fZec58[i as usize]);
			}
			/* Post code */
			for j103 in 0..4 {
				self.fRec58_perm[j103 as usize] = fRec58_tmp[(i32::wrapping_add(vsize, j103)) as usize];
			}
			/* Recursive loop 178 */
			/* Pre code */
			for j92 in 0..4 {
				fRec54_tmp[j92 as usize] = self.fRec54_perm[j92 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec52[i as usize] = if iZec0[i as usize] != 0 {0.0} else {fRec54_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + self.fConst4 * fZec51[i as usize]};
				fRec54_tmp[(i32::wrapping_add(4, i)) as usize] = fZec52[i as usize] - F32::floor(fZec52[i as usize]);
			}
			/* Post code */
			for j93 in 0..4 {
				self.fRec54_perm[j93 as usize] = fRec54_tmp[(i32::wrapping_add(vsize, j93)) as usize];
			}
			/* Recursive loop 179 */
			/* Pre code */
			for j110 in 0..4 {
				fRec61_tmp[j110 as usize] = self.fRec61_perm[j110 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec64[i as usize] = if iZec0[i as usize] != 0 {0.0} else {fRec61_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + self.fConst4 * fZec63[i as usize]};
				fRec61_tmp[(i32::wrapping_add(4, i)) as usize] = fZec64[i as usize] - F32::floor(fZec64[i as usize]);
			}
			/* Post code */
			for j111 in 0..4 {
				self.fRec61_perm[j111 as usize] = fRec61_tmp[(i32::wrapping_add(vsize, j111)) as usize];
			}
			/* Vectorizable loop 180 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec81[i as usize] = 1.4142135 * fZec80[i as usize];
			}
			/* Recursive loop 181 */
			/* Pre code */
			for j118 in 0..4 {
				fRec64_tmp[j118 as usize] = self.fRec64_perm[j118 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec70[i as usize] = if iZec0[i as usize] != 0 {0.0} else {fRec64_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + self.fConst4 * fZec69[i as usize]};
				fRec64_tmp[(i32::wrapping_add(4, i)) as usize] = fZec70[i as usize] - F32::floor(fZec70[i as usize]);
			}
			/* Post code */
			for j119 in 0..4 {
				self.fRec64_perm[j119 as usize] = fRec64_tmp[(i32::wrapping_add(vsize, j119)) as usize];
			}
			/* Vectorizable loop 182 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec79[i as usize] = F32::tan(self.fConst7 * F32::max(2e+01, F32::min(1e+04, fRec75_tmp[(i32::wrapping_add(4, i)) as usize])));
			}
			/* Vectorizable loop 183 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec84[i as usize] = mydsp_faustpower2_f(fZec80[i as usize]);
			}
			/* Recursive loop 184 */
			/* Pre code */
			for j132 in 0..4 {
				fRec72_tmp[j132 as usize] = self.fRec72_perm[j132 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec72_tmp[(i32::wrapping_add(4, i)) as usize] = self.fConst1 * unsafe { ftbl1mydspSIG1[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow39 + fRec73_tmp[(i32::wrapping_add(4, i)) as usize]) + 0.5) as i32, 2047))) as usize] } + self.fConst2 * fRec72_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
			}
			/* Post code */
			for j133 in 0..4 {
				self.fRec72_perm[j133 as usize] = fRec72_tmp[(i32::wrapping_add(vsize, j133)) as usize];
			}
			/* Recursive loop 185 */
			/* Pre code */
			for j172 in 0..4 {
				fRec96_tmp[j172 as usize] = self.fRec96_perm[j172 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec96_tmp[(i32::wrapping_add(4, i)) as usize] = self.fConst1 * unsafe { ftbl1mydspSIG1[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow47 + fRec73_tmp[(i32::wrapping_add(4, i)) as usize]) + 0.5) as i32, 2047))) as usize] } + self.fConst2 * fRec96_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
			}
			/* Post code */
			for j173 in 0..4 {
				self.fRec96_perm[j173 as usize] = fRec96_tmp[(i32::wrapping_add(vsize, j173)) as usize];
			}
			/* Recursive loop 186 */
			/* Pre code */
			for j160 in 0..4 {
				fRec89_tmp[j160 as usize] = self.fRec89_perm[j160 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec89_tmp[(i32::wrapping_add(4, i)) as usize] = self.fConst1 * unsafe { ftbl1mydspSIG1[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow45 + fRec73_tmp[(i32::wrapping_add(4, i)) as usize]) + 0.5) as i32, 2047))) as usize] } + self.fConst2 * fRec89_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
			}
			/* Post code */
			for j161 in 0..4 {
				self.fRec89_perm[j161 as usize] = fRec89_tmp[(i32::wrapping_add(vsize, j161)) as usize];
			}
			/* Vectorizable loop 187 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec91[i as usize] = F32::max(1.1920929e-07, F32::abs(fRec82_tmp[(i32::wrapping_add(4, i)) as usize]));
			}
			/* Vectorizable loop 188 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec105[i as usize] = F32::tan(self.fConst7 * F32::max(2e+01, F32::min(1e+04, fRec90_tmp[(i32::wrapping_add(4, i)) as usize])));
			}
			/* Vectorizable loop 189 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec96[i as usize] = F32::tan(self.fConst7 * F32::max(2e+01, F32::min(1e+04, fRec83_tmp[(i32::wrapping_add(4, i)) as usize])));
			}
			/* Recursive loop 190 */
			/* Pre code */
			for j76 in 0..4 {
				fRec48_tmp[j76 as usize] = self.fRec48_perm[j76 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec40[i as usize] = if iZec0[i as usize] != 0 {0.0} else {fRec48_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + self.fConst4 * fZec39[i as usize]};
				fRec48_tmp[(i32::wrapping_add(4, i)) as usize] = fZec40[i as usize] - F32::floor(fZec40[i as usize]);
			}
			/* Post code */
			for j77 in 0..4 {
				self.fRec48_perm[j77 as usize] = fRec48_tmp[(i32::wrapping_add(vsize, j77)) as usize];
			}
			/* Recursive loop 191 */
			/* Pre code */
			for j66 in 0..4 {
				fRec44_tmp[j66 as usize] = self.fRec44_perm[j66 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec34[i as usize] = if iZec0[i as usize] != 0 {0.0} else {fRec44_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + self.fConst4 * fZec33[i as usize]};
				fRec44_tmp[(i32::wrapping_add(4, i)) as usize] = fZec34[i as usize] - F32::floor(fZec34[i as usize]);
			}
			/* Post code */
			for j67 in 0..4 {
				self.fRec44_perm[j67 as usize] = fRec44_tmp[(i32::wrapping_add(vsize, j67)) as usize];
			}
			/* Recursive loop 192 */
			/* Pre code */
			for j84 in 0..4 {
				fRec51_tmp[j84 as usize] = self.fRec51_perm[j84 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec46[i as usize] = if iZec0[i as usize] != 0 {0.0} else {fRec51_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + self.fConst4 * fZec45[i as usize]};
				fRec51_tmp[(i32::wrapping_add(4, i)) as usize] = fZec46[i as usize] - F32::floor(fZec46[i as usize]);
			}
			/* Post code */
			for j85 in 0..4 {
				self.fRec51_perm[j85 as usize] = fRec51_tmp[(i32::wrapping_add(vsize, j85)) as usize];
			}
			/* Vectorizable loop 193 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec182[i as usize] = 0.0 - fZec181[i as usize];
			}
			/* Vectorizable loop 194 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec183[i as usize] = (F32::min(self.fConst8, (std::cmp::max(0, i32::wrapping_add(iZec172[i as usize], 3))) as F32)) as i32;
			}
			/* Vectorizable loop 195 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec185[i as usize] = 0.0 - 0.5 * fZec181[i as usize];
			}
			/* Vectorizable loop 196 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec186[i as usize] = 0.0 - fZec176[i as usize];
			}
			/* Vectorizable loop 197 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec173[i as usize] = (F32::min(self.fConst8, (std::cmp::max(0, i32::wrapping_add(iZec172[i as usize], 4))) as F32)) as i32;
			}
			/* Vectorizable loop 198 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec179[i as usize] = fZec178[i as usize] * fZec177[i as usize];
			}
			/* Vectorizable loop 199 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec165[i as usize] = F32::tan(self.fConst7 * F32::max(2e+01, F32::min(1e+04, fRec144_tmp[(i32::wrapping_add(4, i)) as usize])));
			}
			/* Vectorizable loop 200 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec187[i as usize] = (F32::min(self.fConst8, (std::cmp::max(0, i32::wrapping_add(iZec172[i as usize], 2))) as F32)) as i32;
			}
			/* Vectorizable loop 201 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec189[i as usize] = 0.0 - 0.33333334 * fZec181[i as usize];
			}
			/* Vectorizable loop 202 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec211[i as usize] = (iRec181_tmp[(i32::wrapping_add(4, i)) as usize]) as F32 / F32::max(1.0, self.fConst15 * mydsp_faustpower2_f(1.0 - 0.0005 * fRec177_tmp[(i32::wrapping_add(4, i)) as usize]));
			}
			/* Vectorizable loop 203 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec196[i as usize] = 0.0 - 0.33333334 * fZec176[i as usize];
			}
			/* Vectorizable loop 204 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec215[i as usize] = F32::tan(self.fConst7 * F32::max(2e+01, F32::min(1e+04, fRec189_tmp[(i32::wrapping_add(4, i)) as usize])));
			}
			/* Vectorizable loop 205 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec190[i as usize] = 0.0 - 0.5 * fZec176[i as usize];
			}
			/* Vectorizable loop 206 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec223[i as usize] = (F32::min(self.fConst8, (std::cmp::max(0, i32::wrapping_add(iZec222[i as usize], 4))) as F32)) as i32;
			}
			/* Vectorizable loop 207 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec195[i as usize] = 0.0 - 0.25 * fZec181[i as usize];
			}
			/* Vectorizable loop 208 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec197[i as usize] = 0.0 - 0.5 * fZec177[i as usize];
			}
			/* Vectorizable loop 209 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec199[i as usize] = (F32::min(self.fConst8, (std::cmp::max(0, iZec172[i as usize])) as F32)) as i32;
			}
			/* Vectorizable loop 210 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec192[i as usize] = (F32::min(self.fConst8, (std::cmp::max(0, i32::wrapping_add(iZec172[i as usize], 1))) as F32)) as i32;
			}
			/* Vectorizable loop 211 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec198[i as usize] = 0.0 - fZec178[i as usize];
			}
			/* Recursive loop 212 */
			/* Pre code */
			for j240 in 0..4 {
				fRec180_tmp[j240 as usize] = self.fRec180_perm[j240 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec180_tmp[(i32::wrapping_add(4, i)) as usize] = fZec153[i as usize] - (fRec180_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] * ((fZec202[i as usize] + -1.4142135) / fZec201[i as usize] + 1.0) + 2.0 * fRec180_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] * (1.0 - 1.0 / mydsp_faustpower2_f(fZec201[i as usize]))) / fZec203[i as usize];
			}
			/* Post code */
			for j241 in 0..4 {
				self.fRec180_perm[j241 as usize] = fRec180_tmp[(i32::wrapping_add(vsize, j241)) as usize];
			}
			/* Vectorizable loop 213 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec191[i as usize] = 0.0 - fZec177[i as usize];
			}
			/* Vectorizable loop 214 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec233[i as usize] = (F32::min(self.fConst8, (std::cmp::max(0, i32::wrapping_add(iZec222[i as usize], 3))) as F32)) as i32;
			}
			/* Vectorizable loop 215 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec235[i as usize] = 0.0 - 0.5 * fZec231[i as usize];
			}
			/* Vectorizable loop 216 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec237[i as usize] = (F32::min(self.fConst8, (std::cmp::max(0, i32::wrapping_add(iZec222[i as usize], 2))) as F32)) as i32;
			}
			/* Vectorizable loop 217 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec240[i as usize] = 0.0 - 0.5 * fZec226[i as usize];
			}
			/* Vectorizable loop 218 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec239[i as usize] = 0.0 - 0.33333334 * fZec231[i as usize];
			}
			/* Vectorizable loop 219 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec232[i as usize] = 0.0 - fZec231[i as usize];
			}
			/* Vectorizable loop 220 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec236[i as usize] = 0.0 - fZec226[i as usize];
			}
			/* Vectorizable loop 221 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec229[i as usize] = fZec228[i as usize] * fZec227[i as usize];
			}
			/* Vectorizable loop 222 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec265[i as usize] = F32::tan(self.fConst7 * F32::max(2e+01, F32::min(1e+04, fRec234_tmp[(i32::wrapping_add(4, i)) as usize])));
			}
			/* Vectorizable loop 223 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec242[i as usize] = (F32::min(self.fConst8, (std::cmp::max(0, i32::wrapping_add(iZec222[i as usize], 1))) as F32)) as i32;
			}
			/* Vectorizable loop 224 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec248[i as usize] = 0.0 - fZec228[i as usize];
			}
			/* Vectorizable loop 225 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec247[i as usize] = 0.0 - 0.5 * fZec227[i as usize];
			}
			/* Vectorizable loop 226 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec273[i as usize] = (F32::min(self.fConst8, (std::cmp::max(0, i32::wrapping_add(iZec272[i as usize], 4))) as F32)) as i32;
			}
			/* Vectorizable loop 227 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec249[i as usize] = (F32::min(self.fConst8, (std::cmp::max(0, iZec222[i as usize])) as F32)) as i32;
			}
			/* Vectorizable loop 228 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec261[i as usize] = (iRec226_tmp[(i32::wrapping_add(4, i)) as usize]) as F32 / F32::max(1.0, self.fConst15 * mydsp_faustpower2_f(1.0 - 0.0005 * fRec222_tmp[(i32::wrapping_add(4, i)) as usize]));
			}
			/* Vectorizable loop 229 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec245[i as usize] = 0.0 - 0.25 * fZec231[i as usize];
			}
			/* Vectorizable loop 230 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec246[i as usize] = 0.0 - 0.33333334 * fZec226[i as usize];
			}
			/* Recursive loop 231 */
			/* Pre code */
			for j288 in 0..4 {
				fRec225_tmp[j288 as usize] = self.fRec225_perm[j288 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec225_tmp[(i32::wrapping_add(4, i)) as usize] = fZec153[i as usize] - (fRec225_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] * ((fZec252[i as usize] + -1.4142135) / fZec251[i as usize] + 1.0) + 2.0 * fRec225_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] * (1.0 - 1.0 / mydsp_faustpower2_f(fZec251[i as usize]))) / fZec253[i as usize];
			}
			/* Post code */
			for j289 in 0..4 {
				self.fRec225_perm[j289 as usize] = fRec225_tmp[(i32::wrapping_add(vsize, j289)) as usize];
			}
			/* Vectorizable loop 232 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec241[i as usize] = 0.0 - fZec227[i as usize];
			}
			/* Vectorizable loop 233 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec279[i as usize] = fZec278[i as usize] * fZec277[i as usize];
			}
			/* Vectorizable loop 234 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec283[i as usize] = (F32::min(self.fConst8, (std::cmp::max(0, i32::wrapping_add(iZec272[i as usize], 3))) as F32)) as i32;
			}
			/* Vectorizable loop 235 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec286[i as usize] = 0.0 - fZec276[i as usize];
			}
			/* Vectorizable loop 236 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec285[i as usize] = 0.0 - 0.5 * fZec281[i as usize];
			}
			/* Vectorizable loop 237 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec282[i as usize] = 0.0 - fZec281[i as usize];
			}
			/* Vectorizable loop 238 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec287[i as usize] = (F32::min(self.fConst8, (std::cmp::max(0, i32::wrapping_add(iZec272[i as usize], 2))) as F32)) as i32;
			}
			/* Vectorizable loop 239 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec289[i as usize] = 0.0 - 0.33333334 * fZec281[i as usize];
			}
			/* Vectorizable loop 240 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec290[i as usize] = 0.0 - 0.5 * fZec276[i as usize];
			}
			/* Vectorizable loop 241 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec291[i as usize] = 0.0 - fZec277[i as usize];
			}
			/* Vectorizable loop 242 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec4[i as usize] = F32::max(0.0, F32::min(2047.0, self.fConst5 / fZec1[i as usize]));
			}
			/* Vectorizable loop 243 */
			/* Pre code */
			for j26 in 0..4 {
				fYec0_tmp[j26 as usize] = self.fYec0_perm[j26 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fYec0_tmp[(i32::wrapping_add(4, i)) as usize] = mydsp_faustpower2_f(2.0 * fRec28_tmp[(i32::wrapping_add(4, i)) as usize] + -1.0);
			}
			/* Post code */
			for j27 in 0..4 {
				self.fYec0_perm[j27 as usize] = fYec0_tmp[(i32::wrapping_add(vsize, j27)) as usize];
			}
			/* Vectorizable loop 244 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec123[i as usize] = i32::wrapping_add(iZec122[i as usize], 1);
			}
			/* Vectorizable loop 245 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec137[i as usize] = i32::wrapping_add(iZec136[i as usize], 1);
			}
			/* Vectorizable loop 246 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec115[i as usize] = 1.0 / fZec114[i as usize];
			}
			/* Vectorizable loop 247 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec129[i as usize] = fZec128[i as usize] * fZec125[i as usize];
			}
			/* Vectorizable loop 248 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec133[i as usize] = i32::wrapping_add(iZec132[i as usize], 1);
			}
			/* Vectorizable loop 249 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec159[i as usize] = fZec140[i as usize] * fZec139[i as usize] * fZec138[i as usize];
			}
			/* Vectorizable loop 250 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec160[i as usize] = fZec147[i as usize] * fZec146[i as usize] * fZec145[i as usize] * fZec144[i as usize];
			}
			/* Vectorizable loop 251 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec142[i as usize] = i32::wrapping_add(iZec141[i as usize], 1);
			}
			/* Vectorizable loop 252 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec157[i as usize] = fZec128[i as usize] * fZec131[i as usize];
			}
			/* Vectorizable loop 253 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec158[i as usize] = fZec127[i as usize] * fZec135[i as usize] * fZec134[i as usize];
			}
			/* Vectorizable loop 254 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec162[i as usize] = 0.5 * ((fRec134_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + fRec134_tmp[(i32::wrapping_add(4, i)) as usize] + 2.0 * fRec134_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) * F32::max(0.0, F32::min(fZec161[i as usize], 2.0 - fZec161[i as usize])) / fZec152[i as usize]);
			}
			/* Vectorizable loop 255 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec149[i as usize] = i32::wrapping_add(iZec148[i as usize], 1);
			}
			/* Vectorizable loop 256 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec143[i as usize] = fZec119[i as usize] - fZec124[i as usize];
			}
			/* Vectorizable loop 257 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec316[i as usize] = 1.0 / fZec315[i as usize];
			}
			/* Vectorizable loop 258 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec309[i as usize] = fZec291[i as usize] * fZec290[i as usize] * fZec289[i as usize];
			}
			/* Vectorizable loop 259 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec330[i as usize] = fZec329[i as usize] * fZec326[i as usize];
			}
			/* Vectorizable loop 260 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec294[i as usize] = fZec270[i as usize] - fZec275[i as usize];
			}
			/* Vectorizable loop 261 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec293[i as usize] = i32::wrapping_add(iZec292[i as usize], 1);
			}
			/* Vectorizable loop 262 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec312[i as usize] = 0.5 * ((fRec270_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + fRec270_tmp[(i32::wrapping_add(4, i)) as usize] + 2.0 * fRec270_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) * F32::max(0.0, F32::min(fZec311[i as usize], 2.0 - fZec311[i as usize])) / fZec303[i as usize]);
			}
			/* Vectorizable loop 263 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec307[i as usize] = fZec279[i as usize] * fZec282[i as usize];
			}
			/* Vectorizable loop 264 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec308[i as usize] = fZec278[i as usize] * fZec286[i as usize] * fZec285[i as usize];
			}
			/* Vectorizable loop 265 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec300[i as usize] = i32::wrapping_add(iZec299[i as usize], 1);
			}
			/* Vectorizable loop 266 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec310[i as usize] = fZec298[i as usize] * fZec297[i as usize] * fZec296[i as usize] * fZec295[i as usize];
			}
			/* Vectorizable loop 267 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec324[i as usize] = i32::wrapping_add(iZec323[i as usize], 1);
			}
			/* Vectorizable loop 268 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec334[i as usize] = i32::wrapping_add(iZec333[i as usize], 1);
			}
			/* Vectorizable loop 269 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec338[i as usize] = i32::wrapping_add(iZec337[i as usize], 1);
			}
			/* Vectorizable loop 270 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec343[i as usize] = i32::wrapping_add(iZec342[i as usize], 1);
			}
			/* Vectorizable loop 271 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec357[i as usize] = fZec329[i as usize] * fZec332[i as usize];
			}
			/* Vectorizable loop 272 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec360[i as usize] = fZec348[i as usize] * fZec347[i as usize] * fZec346[i as usize] * fZec345[i as usize];
			}
			/* Vectorizable loop 273 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec358[i as usize] = fZec328[i as usize] * fZec336[i as usize] * fZec335[i as usize];
			}
			/* Vectorizable loop 274 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec344[i as usize] = fZec320[i as usize] - fZec325[i as usize];
			}
			/* Vectorizable loop 275 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec359[i as usize] = fZec341[i as usize] * fZec340[i as usize] * fZec339[i as usize];
			}
			/* Vectorizable loop 276 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec366[i as usize] = 1.0 / fZec365[i as usize];
			}
			/* Vectorizable loop 277 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec362[i as usize] = 0.5 * ((fRec315_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + fRec315_tmp[(i32::wrapping_add(4, i)) as usize] + 2.0 * fRec315_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) * F32::max(0.0, F32::min(fZec361[i as usize], 2.0 - fZec361[i as usize])) / fZec353[i as usize]);
			}
			/* Vectorizable loop 278 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec350[i as usize] = i32::wrapping_add(iZec349[i as usize], 1);
			}
			/* Vectorizable loop 279 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec23[i as usize] = F32::max(0.0, F32::min(2047.0, self.fConst5 / fZec20[i as usize]));
			}
			/* Vectorizable loop 280 */
			/* Pre code */
			for j52 in 0..4 {
				fYec6_tmp[j52 as usize] = self.fYec6_perm[j52 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fYec6_tmp[(i32::wrapping_add(4, i)) as usize] = mydsp_faustpower2_f(2.0 * fRec38_tmp[(i32::wrapping_add(4, i)) as usize] + -1.0);
			}
			/* Post code */
			for j53 in 0..4 {
				self.fYec6_perm[j53 as usize] = fYec6_tmp[(i32::wrapping_add(vsize, j53)) as usize];
			}
			/* Vectorizable loop 281 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec17[i as usize] = F32::max(0.0, F32::min(2047.0, self.fConst5 / fZec14[i as usize]));
			}
			/* Vectorizable loop 282 */
			/* Pre code */
			for j42 in 0..4 {
				fYec4_tmp[j42 as usize] = self.fYec4_perm[j42 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fYec4_tmp[(i32::wrapping_add(4, i)) as usize] = mydsp_faustpower2_f(2.0 * fRec34_tmp[(i32::wrapping_add(4, i)) as usize] + -1.0);
			}
			/* Post code */
			for j43 in 0..4 {
				self.fYec4_perm[j43 as usize] = fYec4_tmp[(i32::wrapping_add(vsize, j43)) as usize];
			}
			/* Vectorizable loop 283 */
			/* Pre code */
			for j34 in 0..4 {
				fYec2_tmp[j34 as usize] = self.fYec2_perm[j34 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fYec2_tmp[(i32::wrapping_add(4, i)) as usize] = mydsp_faustpower2_f(2.0 * fRec31_tmp[(i32::wrapping_add(4, i)) as usize] + -1.0);
			}
			/* Post code */
			for j35 in 0..4 {
				self.fYec2_perm[j35 as usize] = fYec2_tmp[(i32::wrapping_add(vsize, j35)) as usize];
			}
			/* Vectorizable loop 284 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec6[i as usize] = (iVec0_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) as F32;
			}
			/* Vectorizable loop 285 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec11[i as usize] = F32::max(0.0, F32::min(2047.0, self.fConst5 / fZec8[i as usize]));
			}
			/* Vectorizable loop 286 */
			/* Pre code */
			for j112 in 0..4 {
				fYec20_tmp[j112 as usize] = self.fYec20_perm[j112 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fYec20_tmp[(i32::wrapping_add(4, i)) as usize] = mydsp_faustpower2_f(2.0 * fRec61_tmp[(i32::wrapping_add(4, i)) as usize] + -1.0);
			}
			/* Post code */
			for j113 in 0..4 {
				self.fYec20_perm[j113 as usize] = fYec20_tmp[(i32::wrapping_add(vsize, j113)) as usize];
			}
			/* Vectorizable loop 287 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec59[i as usize] = F32::max(0.0, F32::min(2047.0, self.fConst5 / fZec56[i as usize]));
			}
			/* Vectorizable loop 288 */
			/* Pre code */
			for j86 in 0..4 {
				fYec14_tmp[j86 as usize] = self.fYec14_perm[j86 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fYec14_tmp[(i32::wrapping_add(4, i)) as usize] = mydsp_faustpower2_f(2.0 * fRec51_tmp[(i32::wrapping_add(4, i)) as usize] + -1.0);
			}
			/* Post code */
			for j87 in 0..4 {
				self.fYec14_perm[j87 as usize] = fYec14_tmp[(i32::wrapping_add(vsize, j87)) as usize];
			}
			/* Vectorizable loop 289 */
			/* Pre code */
			for j94 in 0..4 {
				fYec16_tmp[j94 as usize] = self.fYec16_perm[j94 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fYec16_tmp[(i32::wrapping_add(4, i)) as usize] = mydsp_faustpower2_f(2.0 * fRec54_tmp[(i32::wrapping_add(4, i)) as usize] + -1.0);
			}
			/* Post code */
			for j95 in 0..4 {
				self.fYec16_perm[j95 as usize] = fYec16_tmp[(i32::wrapping_add(vsize, j95)) as usize];
			}
			/* Vectorizable loop 290 */
			/* Pre code */
			for j104 in 0..4 {
				fYec18_tmp[j104 as usize] = self.fYec18_perm[j104 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fYec18_tmp[(i32::wrapping_add(4, i)) as usize] = mydsp_faustpower2_f(2.0 * fRec58_tmp[(i32::wrapping_add(4, i)) as usize] + -1.0);
			}
			/* Post code */
			for j105 in 0..4 {
				self.fYec18_perm[j105 as usize] = fYec18_tmp[(i32::wrapping_add(vsize, j105)) as usize];
			}
			/* Vectorizable loop 291 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec53[i as usize] = F32::max(0.0, F32::min(2047.0, self.fConst5 / fZec50[i as usize]));
			}
			/* Vectorizable loop 292 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec65[i as usize] = F32::max(0.0, F32::min(2047.0, self.fConst5 / fZec62[i as usize]));
			}
			/* Vectorizable loop 293 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec71[i as usize] = F32::max(0.0, F32::min(2047.0, self.fConst5 / fZec68[i as usize]));
			}
			/* Vectorizable loop 294 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec74[i as usize] = F32::max(1.1920929e-07, F32::abs(fRec72_tmp[(i32::wrapping_add(4, i)) as usize]));
			}
			/* Vectorizable loop 295 */
			/* Pre code */
			for j120 in 0..4 {
				fYec22_tmp[j120 as usize] = self.fYec22_perm[j120 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fYec22_tmp[(i32::wrapping_add(4, i)) as usize] = mydsp_faustpower2_f(2.0 * fRec64_tmp[(i32::wrapping_add(4, i)) as usize] + -1.0);
			}
			/* Post code */
			for j121 in 0..4 {
				self.fYec22_perm[j121 as usize] = fYec22_tmp[(i32::wrapping_add(vsize, j121)) as usize];
			}
			/* Vectorizable loop 296 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec82[i as usize] = fZec81[i as usize] + 2.0;
			}
			/* Vectorizable loop 297 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec85[i as usize] = fZec81[i as usize] + fZec84[i as usize];
			}
			/* Vectorizable loop 298 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec83[i as usize] = 1.0 / fZec79[i as usize];
			}
			/* Vectorizable loop 299 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec97[i as usize] = 1.0 / fZec96[i as usize];
			}
			/* Vectorizable loop 300 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec106[i as usize] = 1.0 / fZec105[i as usize];
			}
			/* Vectorizable loop 301 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec110[i as usize] = F32::max(1.1920929e-07, F32::abs(fRec96_tmp[(i32::wrapping_add(4, i)) as usize]));
			}
			/* Vectorizable loop 302 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec101[i as usize] = F32::max(1.1920929e-07, F32::abs(fRec89_tmp[(i32::wrapping_add(4, i)) as usize]));
			}
			/* Vectorizable loop 303 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec92[i as usize] = self.fConst4 * fZec91[i as usize];
			}
			/* Vectorizable loop 304 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec47[i as usize] = F32::max(0.0, F32::min(2047.0, self.fConst5 / fZec44[i as usize]));
			}
			/* Vectorizable loop 305 */
			/* Pre code */
			for j68 in 0..4 {
				fYec10_tmp[j68 as usize] = self.fYec10_perm[j68 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fYec10_tmp[(i32::wrapping_add(4, i)) as usize] = mydsp_faustpower2_f(2.0 * fRec44_tmp[(i32::wrapping_add(4, i)) as usize] + -1.0);
			}
			/* Post code */
			for j69 in 0..4 {
				self.fYec10_perm[j69 as usize] = fYec10_tmp[(i32::wrapping_add(vsize, j69)) as usize];
			}
			/* Vectorizable loop 306 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec35[i as usize] = F32::max(0.0, F32::min(2047.0, self.fConst5 / fZec32[i as usize]));
			}
			/* Vectorizable loop 307 */
			/* Pre code */
			for j78 in 0..4 {
				fYec12_tmp[j78 as usize] = self.fYec12_perm[j78 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fYec12_tmp[(i32::wrapping_add(4, i)) as usize] = mydsp_faustpower2_f(2.0 * fRec48_tmp[(i32::wrapping_add(4, i)) as usize] + -1.0);
			}
			/* Post code */
			for j79 in 0..4 {
				self.fYec12_perm[j79 as usize] = fYec12_tmp[(i32::wrapping_add(vsize, j79)) as usize];
			}
			/* Vectorizable loop 308 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec29[i as usize] = F32::max(0.0, F32::min(2047.0, self.fConst5 / fZec26[i as usize]));
			}
			/* Vectorizable loop 309 */
			/* Pre code */
			for j60 in 0..4 {
				fYec8_tmp[j60 as usize] = self.fYec8_perm[j60 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fYec8_tmp[(i32::wrapping_add(4, i)) as usize] = mydsp_faustpower2_f(2.0 * fRec41_tmp[(i32::wrapping_add(4, i)) as usize] + -1.0);
			}
			/* Post code */
			for j61 in 0..4 {
				self.fYec8_perm[j61 as usize] = fYec8_tmp[(i32::wrapping_add(vsize, j61)) as usize];
			}
			/* Vectorizable loop 310 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec41[i as usize] = F32::max(0.0, F32::min(2047.0, self.fConst5 / fZec38[i as usize]));
			}
			/* Vectorizable loop 311 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec174[i as usize] = i32::wrapping_add(iZec173[i as usize], 1);
			}
			/* Vectorizable loop 312 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec166[i as usize] = 1.0 / fZec165[i as usize];
			}
			/* Vectorizable loop 313 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec180[i as usize] = fZec179[i as usize] * fZec176[i as usize];
			}
			/* Vectorizable loop 314 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec188[i as usize] = i32::wrapping_add(iZec187[i as usize], 1);
			}
			/* Vectorizable loop 315 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec184[i as usize] = i32::wrapping_add(iZec183[i as usize], 1);
			}
			/* Vectorizable loop 316 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec210[i as usize] = fZec198[i as usize] * fZec197[i as usize] * fZec196[i as usize] * fZec195[i as usize];
			}
			/* Vectorizable loop 317 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec216[i as usize] = 1.0 / fZec215[i as usize];
			}
			/* Vectorizable loop 318 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec193[i as usize] = i32::wrapping_add(iZec192[i as usize], 1);
			}
			/* Vectorizable loop 319 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec194[i as usize] = fZec170[i as usize] - fZec175[i as usize];
			}
			/* Vectorizable loop 320 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec209[i as usize] = fZec191[i as usize] * fZec190[i as usize] * fZec189[i as usize];
			}
			/* Vectorizable loop 321 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec208[i as usize] = fZec178[i as usize] * fZec186[i as usize] * fZec185[i as usize];
			}
			/* Vectorizable loop 322 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec207[i as usize] = fZec179[i as usize] * fZec182[i as usize];
			}
			/* Vectorizable loop 323 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec224[i as usize] = i32::wrapping_add(iZec223[i as usize], 1);
			}
			/* Vectorizable loop 324 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec200[i as usize] = i32::wrapping_add(iZec199[i as usize], 1);
			}
			/* Vectorizable loop 325 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec230[i as usize] = fZec229[i as usize] * fZec226[i as usize];
			}
			/* Vectorizable loop 326 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec212[i as usize] = 0.5 * ((fRec180_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + fRec180_tmp[(i32::wrapping_add(4, i)) as usize] + 2.0 * fRec180_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) * F32::max(0.0, F32::min(fZec211[i as usize], 2.0 - fZec211[i as usize])) / fZec203[i as usize]);
			}
			/* Vectorizable loop 327 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec234[i as usize] = i32::wrapping_add(iZec233[i as usize], 1);
			}
			/* Vectorizable loop 328 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec238[i as usize] = i32::wrapping_add(iZec237[i as usize], 1);
			}
			/* Vectorizable loop 329 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec274[i as usize] = i32::wrapping_add(iZec273[i as usize], 1);
			}
			/* Vectorizable loop 330 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec280[i as usize] = fZec279[i as usize] * fZec276[i as usize];
			}
			/* Vectorizable loop 331 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec259[i as usize] = fZec241[i as usize] * fZec240[i as usize] * fZec239[i as usize];
			}
			/* Vectorizable loop 332 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec250[i as usize] = i32::wrapping_add(iZec249[i as usize], 1);
			}
			/* Vectorizable loop 333 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec262[i as usize] = 0.5 * ((fRec225_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + fRec225_tmp[(i32::wrapping_add(4, i)) as usize] + 2.0 * fRec225_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) * F32::max(0.0, F32::min(fZec261[i as usize], 2.0 - fZec261[i as usize])) / fZec253[i as usize]);
			}
			/* Vectorizable loop 334 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec243[i as usize] = i32::wrapping_add(iZec242[i as usize], 1);
			}
			/* Vectorizable loop 335 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec266[i as usize] = 1.0 / fZec265[i as usize];
			}
			/* Vectorizable loop 336 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec244[i as usize] = fZec220[i as usize] - fZec225[i as usize];
			}
			/* Vectorizable loop 337 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec257[i as usize] = fZec229[i as usize] * fZec232[i as usize];
			}
			/* Vectorizable loop 338 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec258[i as usize] = fZec228[i as usize] * fZec236[i as usize] * fZec235[i as usize];
			}
			/* Vectorizable loop 339 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec260[i as usize] = fZec248[i as usize] * fZec247[i as usize] * fZec246[i as usize] * fZec245[i as usize];
			}
			/* Vectorizable loop 340 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec284[i as usize] = i32::wrapping_add(iZec283[i as usize], 1);
			}
			/* Vectorizable loop 341 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec288[i as usize] = i32::wrapping_add(iZec287[i as usize], 1);
			}
			/* Vectorizable loop 342 */
			/* Pre code */
			self.fYec1_idx = (i32::wrapping_add(self.fYec1_idx, self.fYec1_idx_save)) & 4095;
			/* Compute code */
			for i in 0..output0.len() as i32 {
				self.fYec1[((i32::wrapping_add(i, self.fYec1_idx)) & 4095) as usize] = fZec6[i as usize] * (fYec0_tmp[(i32::wrapping_add(4, i)) as usize] - fYec0_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) / fZec2[i as usize];
			}
			/* Post code */
			self.fYec1_idx_save = vsize;
			/* Recursive loop 343 */
			/* Pre code */
			for j10 in 0..4 {
				fRec19_tmp[j10 as usize] = self.fRec19_perm[j10 as usize];
			}
			for j12 in 0..4 {
				fRec20_tmp[j12 as usize] = self.fRec20_perm[j12 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec19_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow10 * fRec20_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + fSlow9 * fRec19_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fRec20_tmp[(i32::wrapping_add(4, i)) as usize] = (iZec0[i as usize]) as F32 + fSlow9 * fRec20_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] - fSlow10 * fRec19_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
			}
			/* Post code */
			for j11 in 0..4 {
				self.fRec19_perm[j11 as usize] = fRec19_tmp[(i32::wrapping_add(vsize, j11)) as usize];
			}
			for j13 in 0..4 {
				self.fRec20_perm[j13 as usize] = fRec20_tmp[(i32::wrapping_add(vsize, j13)) as usize];
			}
			/* Vectorizable loop 344 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec5[i as usize] = F32::floor(fZec4[i as usize]);
			}
			/* Vectorizable loop 345 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec116[i as usize] = fZec85[i as usize] + (fZec82[i as usize] + fZec115[i as usize]) / fZec114[i as usize] + 1.0;
			}
			/* Recursive loop 346 */
			/* Pre code */
			for j186 in 0..4 {
				fRec127_tmp[j186 as usize] = self.fRec127_perm[j186 as usize];
			}
			for j188 in 0..4 {
				fRec132_tmp[j188 as usize] = self.fRec132_perm[j188 as usize];
			}
			for j198 in 0..4 {
				fRec137_tmp[j198 as usize] = self.fRec137_perm[j198 as usize];
			}
			self.fRec138_idx = (i32::wrapping_add(self.fRec138_idx, self.fRec138_idx_save)) & 2047;
			for j200 in 0..4 {
				fRec140_tmp[j200 as usize] = self.fRec140_perm[j200 as usize];
			}
			for j202 in 0..4 {
				iYec24_tmp[j202 as usize] = self.iYec24_perm[j202 as usize];
			}
			for j204 in 0..4 {
				iRec141_tmp[j204 as usize] = self.iRec141_perm[j204 as usize];
			}
			for j206 in 0..4 {
				fRec139_tmp[j206 as usize] = self.fRec139_perm[j206 as usize];
			}
			for j208 in 0..4 {
				fYec25_tmp[j208 as usize] = self.fYec25_perm[j208 as usize];
			}
			for j210 in 0..4 {
				fYec26_tmp[j210 as usize] = self.fYec26_perm[j210 as usize];
			}
			self.fRec133_idx = (i32::wrapping_add(self.fRec133_idx, self.fRec133_idx_save)) & 2047;
			for j212 in 0..4 {
				fRec123_tmp[j212 as usize] = self.fRec123_perm[j212 as usize];
			}
			for j214 in 0..4 {
				fRec119_tmp[j214 as usize] = self.fRec119_perm[j214 as usize];
			}
			self.fRec115_idx = (i32::wrapping_add(self.fRec115_idx, self.fRec115_idx_save)) & 2047;
			for j216 in 0..4 {
				fRec117_tmp[j216 as usize] = self.fRec117_perm[j216 as usize];
			}
			for j218 in 0..4 {
				fRec113_tmp[j218 as usize] = self.fRec113_perm[j218 as usize];
			}
			for j220 in 0..4 {
				fRec108_tmp[j220 as usize] = self.fRec108_perm[j220 as usize];
			}
			self.fRec104_idx = (i32::wrapping_add(self.fRec104_idx, self.fRec104_idx_save)) & 2047;
			for j222 in 0..4 {
				fRec102_tmp[j222 as usize] = self.fRec102_perm[j222 as usize];
			}
			for j224 in 0..4 {
				fRec143_tmp[j224 as usize] = self.fRec143_perm[j224 as usize];
			}
			for j226 in 0..4 {
				fRec101_tmp[j226 as usize] = self.fRec101_perm[j226 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec112[i as usize] = -1.0 * 0.9973053 * (0.9 * fRec113_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + 0.05 * (fRec113_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + fRec113_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 3))) as usize]));
				fRec127_tmp[(i32::wrapping_add(4, i)) as usize] = self.fRec104[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec104_idx), iZec149[i as usize])) & 2047) as usize] * fZec147[i as usize] * fZec146[i as usize] * fZec145[i as usize] * fZec144[i as usize] + fZec143[i as usize] * (self.fRec104[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec104_idx), iZec142[i as usize])) & 2047) as usize] * fZec140[i as usize] * fZec139[i as usize] * fZec138[i as usize] + 0.5 * fZec127[i as usize] * self.fRec104[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec104_idx), iZec137[i as usize])) & 2047) as usize] * fZec135[i as usize] * fZec134[i as usize] + 0.16666667 * fZec128[i as usize] * self.fRec104[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec104_idx), iZec133[i as usize])) & 2047) as usize] * fZec131[i as usize] + 0.041666668 * fZec129[i as usize] * self.fRec104[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec104_idx), iZec123[i as usize])) & 2047) as usize]);
				fRec132_tmp[(i32::wrapping_add(4, i)) as usize] = 0.95 * fRec127_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + 0.05 * fRec132_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fRec128[i as usize] = fRec132_tmp[(i32::wrapping_add(4, i)) as usize];
				fRec137_tmp[(i32::wrapping_add(4, i)) as usize] = fRec102_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				self.fRec138[((i32::wrapping_add(i, self.fRec138_idx)) & 2047) as usize] = -1.0 * 0.9973053 * (0.9 * fRec137_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + 0.05 * (fRec137_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + fRec137_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 3))) as usize]));
				fRec140_tmp[(i32::wrapping_add(4, i)) as usize] = self.fConst12 * F32::abs(fRec101_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) + self.fConst11 * fRec140_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				iYec24_tmp[(i32::wrapping_add(4, i)) as usize] = (fRec140_tmp[(i32::wrapping_add(4, i)) as usize] > 0.1) as i32;
				iRec141_tmp[(i32::wrapping_add(4, i)) as usize] = std::cmp::max(i32::wrapping_mul(self.iConst13, (iYec24_tmp[(i32::wrapping_add(4, i)) as usize] < iYec24_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) as i32), i32::wrapping_add(iRec141_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize], -1));
				fZec154[i as usize] = F32::abs(F32::max((iYec24_tmp[(i32::wrapping_add(4, i)) as usize]) as F32, ((iRec141_tmp[(i32::wrapping_add(4, i)) as usize] > 0) as i32) as u32 as F32));
				fZec155[i as usize] = if (fZec154[i as usize] > fRec139_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) as i32 != 0 {self.fConst11} else {self.fConst14};
				fRec139_tmp[(i32::wrapping_add(4, i)) as usize] = fZec154[i as usize] * (1.0 - fZec155[i as usize]) + fRec139_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] * fZec155[i as usize];
				fZec156[i as usize] = 0.005 * fRec139_tmp[(i32::wrapping_add(4, i)) as usize] * fRec101_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fYec25_tmp[(i32::wrapping_add(4, i)) as usize] = fZec160[i as usize] * self.fRec138[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec138_idx), i32::wrapping_add(iZec148[i as usize], 2))) & 2047) as usize] + fZec143[i as usize] * (fZec159[i as usize] * self.fRec138[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec138_idx), i32::wrapping_add(iZec141[i as usize], 2))) & 2047) as usize] + 0.5 * fZec158[i as usize] * self.fRec138[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec138_idx), i32::wrapping_add(iZec136[i as usize], 2))) & 2047) as usize] + 0.16666667 * fZec157[i as usize] * self.fRec138[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec138_idx), i32::wrapping_add(iZec132[i as usize], 2))) & 2047) as usize] + 0.041666668 * fZec129[i as usize] * self.fRec138[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec138_idx), i32::wrapping_add(iZec122[i as usize], 2))) & 2047) as usize]);
				fYec26_tmp[(i32::wrapping_add(4, i)) as usize] = fZec162[i as usize] + fYec25_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + fZec156[i as usize];
				self.fRec133[((i32::wrapping_add(i, self.fRec133_idx)) & 2047) as usize] = 0.95 * fYec26_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + 0.05 * self.fRec133[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec133_idx), 1)) & 2047) as usize];
				fRec129[i as usize] = fZec160[i as usize] * self.fRec133[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec133_idx), iZec148[i as usize])) & 2047) as usize] + fZec143[i as usize] * (fZec159[i as usize] * self.fRec133[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec133_idx), iZec141[i as usize])) & 2047) as usize] + 0.5 * fZec158[i as usize] * self.fRec133[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec133_idx), iZec136[i as usize])) & 2047) as usize] + 0.16666667 * fZec157[i as usize] * self.fRec133[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec133_idx), iZec132[i as usize])) & 2047) as usize] + 0.041666668 * fZec129[i as usize] * self.fRec133[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec133_idx), iZec122[i as usize])) & 2047) as usize]);
				fRec130[i as usize] = fYec26_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + fRec123_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fRec123_tmp[(i32::wrapping_add(4, i)) as usize] = fRec128[i as usize];
				fRec124[i as usize] = fRec123_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fRec125[i as usize] = fRec129[i as usize];
				fRec126[i as usize] = fRec130[i as usize];
				fRec119_tmp[(i32::wrapping_add(4, i)) as usize] = fRec124[i as usize];
				fRec120[i as usize] = fZec162[i as usize] + fZec156[i as usize] + fRec119_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fRec121[i as usize] = fRec125[i as usize];
				fRec122[i as usize] = fRec126[i as usize];
				self.fRec115[((i32::wrapping_add(i, self.fRec115_idx)) & 2047) as usize] = fRec120[i as usize];
				fRec116[i as usize] = fZec160[i as usize] * self.fRec115[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec115_idx), iZec149[i as usize])) & 2047) as usize] + fZec143[i as usize] * (fZec159[i as usize] * self.fRec115[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec115_idx), iZec142[i as usize])) & 2047) as usize] + 0.5 * fZec158[i as usize] * self.fRec115[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec115_idx), iZec137[i as usize])) & 2047) as usize] + 0.16666667 * fZec157[i as usize] * self.fRec115[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec115_idx), iZec133[i as usize])) & 2047) as usize] + 0.041666668 * fZec129[i as usize] * self.fRec115[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec115_idx), iZec123[i as usize])) & 2047) as usize]);
				fRec117_tmp[(i32::wrapping_add(4, i)) as usize] = fRec121[i as usize];
				fRec118[i as usize] = fRec122[i as usize];
				fRec113_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow51 * fRec117_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fRec114[i as usize] = fRec118[i as usize];
				fRec108_tmp[(i32::wrapping_add(4, i)) as usize] = fRec112[i as usize];
				fRec109[i as usize] = fSlow51 * fRec108_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fRec110[i as usize] = fRec113_tmp[(i32::wrapping_add(4, i)) as usize];
				fRec111[i as usize] = fRec114[i as usize];
				self.fRec104[((i32::wrapping_add(i, self.fRec104_idx)) & 2047) as usize] = fRec109[i as usize];
				fRec105[i as usize] = fRec116[i as usize];
				fRec106[i as usize] = fRec110[i as usize];
				fRec107[i as usize] = fRec111[i as usize];
				fRec102_tmp[(i32::wrapping_add(4, i)) as usize] = fRec105[i as usize];
				fRec103[i as usize] = fRec107[i as usize];
				fZec163[i as usize] = F32::abs(fRec103[i as usize]);
				fZec164[i as usize] = if (fZec163[i as usize] > fRec143_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) as i32 != 0 {0.0} else {self.fConst16};
				fRec143_tmp[(i32::wrapping_add(4, i)) as usize] = fZec163[i as usize] * (1.0 - fZec164[i as usize]) + fRec143_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] * fZec164[i as usize];
				fRec142[i as usize] = 0.0 - 0.95 * F32::max(2e+01 * F32::log10(F32::max(1.1754944e-38, fRec143_tmp[(i32::wrapping_add(4, i)) as usize])) + 1e+01, 0.0);
				fRec101_tmp[(i32::wrapping_add(4, i)) as usize] = 1.5 * fRec103[i as usize] * F32::powf(1e+01, 0.05 * fRec142[i as usize]);
			}
			/* Post code */
			for j225 in 0..4 {
				self.fRec143_perm[j225 as usize] = fRec143_tmp[(i32::wrapping_add(vsize, j225)) as usize];
			}
			for j209 in 0..4 {
				self.fYec25_perm[j209 as usize] = fYec25_tmp[(i32::wrapping_add(vsize, j209)) as usize];
			}
			for j211 in 0..4 {
				self.fYec26_perm[j211 as usize] = fYec26_tmp[(i32::wrapping_add(vsize, j211)) as usize];
			}
			for j203 in 0..4 {
				self.iYec24_perm[j203 as usize] = iYec24_tmp[(i32::wrapping_add(vsize, j203)) as usize];
			}
			for j205 in 0..4 {
				self.iRec141_perm[j205 as usize] = iRec141_tmp[(i32::wrapping_add(vsize, j205)) as usize];
			}
			for j201 in 0..4 {
				self.fRec140_perm[j201 as usize] = fRec140_tmp[(i32::wrapping_add(vsize, j201)) as usize];
			}
			for j207 in 0..4 {
				self.fRec139_perm[j207 as usize] = fRec139_tmp[(i32::wrapping_add(vsize, j207)) as usize];
			}
			for j199 in 0..4 {
				self.fRec137_perm[j199 as usize] = fRec137_tmp[(i32::wrapping_add(vsize, j199)) as usize];
			}
			self.fRec138_idx_save = vsize;
			self.fRec133_idx_save = vsize;
			for j189 in 0..4 {
				self.fRec132_perm[j189 as usize] = fRec132_tmp[(i32::wrapping_add(vsize, j189)) as usize];
			}
			for j187 in 0..4 {
				self.fRec127_perm[j187 as usize] = fRec127_tmp[(i32::wrapping_add(vsize, j187)) as usize];
			}
			for j213 in 0..4 {
				self.fRec123_perm[j213 as usize] = fRec123_tmp[(i32::wrapping_add(vsize, j213)) as usize];
			}
			for j215 in 0..4 {
				self.fRec119_perm[j215 as usize] = fRec119_tmp[(i32::wrapping_add(vsize, j215)) as usize];
			}
			self.fRec115_idx_save = vsize;
			for j217 in 0..4 {
				self.fRec117_perm[j217 as usize] = fRec117_tmp[(i32::wrapping_add(vsize, j217)) as usize];
			}
			for j219 in 0..4 {
				self.fRec113_perm[j219 as usize] = fRec113_tmp[(i32::wrapping_add(vsize, j219)) as usize];
			}
			for j221 in 0..4 {
				self.fRec108_perm[j221 as usize] = fRec108_tmp[(i32::wrapping_add(vsize, j221)) as usize];
			}
			self.fRec104_idx_save = vsize;
			for j223 in 0..4 {
				self.fRec102_perm[j223 as usize] = fRec102_tmp[(i32::wrapping_add(vsize, j223)) as usize];
			}
			for j227 in 0..4 {
				self.fRec101_perm[j227 as usize] = fRec101_tmp[(i32::wrapping_add(vsize, j227)) as usize];
			}
			/* Vectorizable loop 347 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec117[i as usize] = 1.0 / mydsp_faustpower2_f(fZec114[i as usize]);
			}
			/* Recursive loop 348 */
			/* Pre code */
			for j380 in 0..4 {
				fRec308_tmp[j380 as usize] = self.fRec308_perm[j380 as usize];
			}
			for j382 in 0..4 {
				fRec313_tmp[j382 as usize] = self.fRec313_perm[j382 as usize];
			}
			for j390 in 0..4 {
				fRec317_tmp[j390 as usize] = self.fRec317_perm[j390 as usize];
			}
			self.fRec318_idx = (i32::wrapping_add(self.fRec318_idx, self.fRec318_idx_save)) & 2047;
			for j392 in 0..4 {
				fRec320_tmp[j392 as usize] = self.fRec320_perm[j392 as usize];
			}
			for j394 in 0..4 {
				iYec36_tmp[j394 as usize] = self.iYec36_perm[j394 as usize];
			}
			for j396 in 0..4 {
				iRec321_tmp[j396 as usize] = self.iRec321_perm[j396 as usize];
			}
			for j398 in 0..4 {
				fRec319_tmp[j398 as usize] = self.fRec319_perm[j398 as usize];
			}
			for j400 in 0..4 {
				fYec37_tmp[j400 as usize] = self.fYec37_perm[j400 as usize];
			}
			for j402 in 0..4 {
				fYec38_tmp[j402 as usize] = self.fYec38_perm[j402 as usize];
			}
			self.fRec314_idx = (i32::wrapping_add(self.fRec314_idx, self.fRec314_idx_save)) & 2047;
			for j404 in 0..4 {
				fRec304_tmp[j404 as usize] = self.fRec304_perm[j404 as usize];
			}
			for j406 in 0..4 {
				fRec300_tmp[j406 as usize] = self.fRec300_perm[j406 as usize];
			}
			self.fRec296_idx = (i32::wrapping_add(self.fRec296_idx, self.fRec296_idx_save)) & 2047;
			for j408 in 0..4 {
				fRec298_tmp[j408 as usize] = self.fRec298_perm[j408 as usize];
			}
			for j410 in 0..4 {
				fRec294_tmp[j410 as usize] = self.fRec294_perm[j410 as usize];
			}
			for j412 in 0..4 {
				fRec289_tmp[j412 as usize] = self.fRec289_perm[j412 as usize];
			}
			self.fRec285_idx = (i32::wrapping_add(self.fRec285_idx, self.fRec285_idx_save)) & 2047;
			for j414 in 0..4 {
				fRec283_tmp[j414 as usize] = self.fRec283_perm[j414 as usize];
			}
			for j416 in 0..4 {
				fRec323_tmp[j416 as usize] = self.fRec323_perm[j416 as usize];
			}
			for j418 in 0..4 {
				fRec282_tmp[j418 as usize] = self.fRec282_perm[j418 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec293[i as usize] = -1.0 * 0.9973053 * (0.9 * fRec294_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + 0.05 * (fRec294_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + fRec294_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 3))) as usize]));
				fRec308_tmp[(i32::wrapping_add(4, i)) as usize] = self.fRec285[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec285_idx), iZec350[i as usize])) & 2047) as usize] * fZec348[i as usize] * fZec347[i as usize] * fZec346[i as usize] * fZec345[i as usize] + fZec344[i as usize] * (self.fRec285[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec285_idx), iZec343[i as usize])) & 2047) as usize] * fZec341[i as usize] * fZec340[i as usize] * fZec339[i as usize] + 0.5 * fZec328[i as usize] * self.fRec285[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec285_idx), iZec338[i as usize])) & 2047) as usize] * fZec336[i as usize] * fZec335[i as usize] + 0.16666667 * fZec329[i as usize] * self.fRec285[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec285_idx), iZec334[i as usize])) & 2047) as usize] * fZec332[i as usize] + 0.041666668 * fZec330[i as usize] * self.fRec285[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec285_idx), iZec324[i as usize])) & 2047) as usize]);
				fRec313_tmp[(i32::wrapping_add(4, i)) as usize] = 0.95 * fRec308_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + 0.05 * fRec313_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fRec309[i as usize] = fRec313_tmp[(i32::wrapping_add(4, i)) as usize];
				fRec317_tmp[(i32::wrapping_add(4, i)) as usize] = fRec283_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				self.fRec318[((i32::wrapping_add(i, self.fRec318_idx)) & 2047) as usize] = -1.0 * 0.9973053 * (0.9 * fRec317_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + 0.05 * (fRec317_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + fRec317_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 3))) as usize]));
				fRec320_tmp[(i32::wrapping_add(4, i)) as usize] = self.fConst12 * F32::abs(fRec282_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) + self.fConst11 * fRec320_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				iYec36_tmp[(i32::wrapping_add(4, i)) as usize] = (fRec320_tmp[(i32::wrapping_add(4, i)) as usize] > 0.1) as i32;
				iRec321_tmp[(i32::wrapping_add(4, i)) as usize] = std::cmp::max(i32::wrapping_mul(self.iConst13, (iYec36_tmp[(i32::wrapping_add(4, i)) as usize] < iYec36_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) as i32), i32::wrapping_add(iRec321_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize], -1));
				fZec354[i as usize] = F32::abs(F32::max((iYec36_tmp[(i32::wrapping_add(4, i)) as usize]) as F32, ((iRec321_tmp[(i32::wrapping_add(4, i)) as usize] > 0) as i32) as u32 as F32));
				fZec355[i as usize] = if (fZec354[i as usize] > fRec319_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) as i32 != 0 {self.fConst11} else {self.fConst14};
				fRec319_tmp[(i32::wrapping_add(4, i)) as usize] = fZec354[i as usize] * (1.0 - fZec355[i as usize]) + fRec319_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] * fZec355[i as usize];
				fZec356[i as usize] = 0.005 * fRec319_tmp[(i32::wrapping_add(4, i)) as usize] * fRec282_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fYec37_tmp[(i32::wrapping_add(4, i)) as usize] = fZec360[i as usize] * self.fRec318[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec318_idx), i32::wrapping_add(iZec349[i as usize], 2))) & 2047) as usize] + fZec344[i as usize] * (fZec359[i as usize] * self.fRec318[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec318_idx), i32::wrapping_add(iZec342[i as usize], 2))) & 2047) as usize] + 0.5 * fZec358[i as usize] * self.fRec318[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec318_idx), i32::wrapping_add(iZec337[i as usize], 2))) & 2047) as usize] + 0.16666667 * fZec357[i as usize] * self.fRec318[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec318_idx), i32::wrapping_add(iZec333[i as usize], 2))) & 2047) as usize] + 0.041666668 * fZec330[i as usize] * self.fRec318[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec318_idx), i32::wrapping_add(iZec323[i as usize], 2))) & 2047) as usize]);
				fYec38_tmp[(i32::wrapping_add(4, i)) as usize] = fZec362[i as usize] + fYec37_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + fZec356[i as usize];
				self.fRec314[((i32::wrapping_add(i, self.fRec314_idx)) & 2047) as usize] = 0.95 * fYec38_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + 0.05 * self.fRec314[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec314_idx), 1)) & 2047) as usize];
				fRec310[i as usize] = fZec360[i as usize] * self.fRec314[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec314_idx), iZec349[i as usize])) & 2047) as usize] + fZec344[i as usize] * (fZec359[i as usize] * self.fRec314[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec314_idx), iZec342[i as usize])) & 2047) as usize] + 0.5 * fZec358[i as usize] * self.fRec314[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec314_idx), iZec337[i as usize])) & 2047) as usize] + 0.16666667 * fZec357[i as usize] * self.fRec314[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec314_idx), iZec333[i as usize])) & 2047) as usize] + 0.041666668 * fZec330[i as usize] * self.fRec314[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec314_idx), iZec323[i as usize])) & 2047) as usize]);
				fRec311[i as usize] = fYec38_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + fRec304_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fRec304_tmp[(i32::wrapping_add(4, i)) as usize] = fRec309[i as usize];
				fRec305[i as usize] = fRec304_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fRec306[i as usize] = fRec310[i as usize];
				fRec307[i as usize] = fRec311[i as usize];
				fRec300_tmp[(i32::wrapping_add(4, i)) as usize] = fRec305[i as usize];
				fRec301[i as usize] = fZec356[i as usize] + fZec362[i as usize] + fRec300_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fRec302[i as usize] = fRec306[i as usize];
				fRec303[i as usize] = fRec307[i as usize];
				self.fRec296[((i32::wrapping_add(i, self.fRec296_idx)) & 2047) as usize] = fRec301[i as usize];
				fRec297[i as usize] = fZec360[i as usize] * self.fRec296[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec296_idx), iZec350[i as usize])) & 2047) as usize] + fZec344[i as usize] * (fZec359[i as usize] * self.fRec296[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec296_idx), iZec343[i as usize])) & 2047) as usize] + 0.5 * fZec358[i as usize] * self.fRec296[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec296_idx), iZec338[i as usize])) & 2047) as usize] + 0.16666667 * fZec357[i as usize] * self.fRec296[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec296_idx), iZec334[i as usize])) & 2047) as usize] + 0.041666668 * fZec330[i as usize] * self.fRec296[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec296_idx), iZec324[i as usize])) & 2047) as usize]);
				fRec298_tmp[(i32::wrapping_add(4, i)) as usize] = fRec302[i as usize];
				fRec299[i as usize] = fRec303[i as usize];
				fRec294_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow51 * fRec298_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fRec295[i as usize] = fRec299[i as usize];
				fRec289_tmp[(i32::wrapping_add(4, i)) as usize] = fRec293[i as usize];
				fRec290[i as usize] = fSlow51 * fRec289_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fRec291[i as usize] = fRec294_tmp[(i32::wrapping_add(4, i)) as usize];
				fRec292[i as usize] = fRec295[i as usize];
				self.fRec285[((i32::wrapping_add(i, self.fRec285_idx)) & 2047) as usize] = fRec290[i as usize];
				fRec286[i as usize] = fRec297[i as usize];
				fRec287[i as usize] = fRec291[i as usize];
				fRec288[i as usize] = fRec292[i as usize];
				fRec283_tmp[(i32::wrapping_add(4, i)) as usize] = fRec286[i as usize];
				fRec284[i as usize] = fRec288[i as usize];
				fZec363[i as usize] = F32::abs(fRec284[i as usize]);
				fZec364[i as usize] = if (fZec363[i as usize] > fRec323_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) as i32 != 0 {0.0} else {self.fConst16};
				fRec323_tmp[(i32::wrapping_add(4, i)) as usize] = fZec363[i as usize] * (1.0 - fZec364[i as usize]) + fRec323_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] * fZec364[i as usize];
				fRec322[i as usize] = 0.0 - 0.95 * F32::max(2e+01 * F32::log10(F32::max(1.1754944e-38, fRec323_tmp[(i32::wrapping_add(4, i)) as usize])) + 1e+01, 0.0);
				fRec282_tmp[(i32::wrapping_add(4, i)) as usize] = 1.5 * fRec284[i as usize] * F32::powf(1e+01, 0.05 * fRec322[i as usize]);
			}
			/* Post code */
			for j417 in 0..4 {
				self.fRec323_perm[j417 as usize] = fRec323_tmp[(i32::wrapping_add(vsize, j417)) as usize];
			}
			for j401 in 0..4 {
				self.fYec37_perm[j401 as usize] = fYec37_tmp[(i32::wrapping_add(vsize, j401)) as usize];
			}
			for j403 in 0..4 {
				self.fYec38_perm[j403 as usize] = fYec38_tmp[(i32::wrapping_add(vsize, j403)) as usize];
			}
			for j395 in 0..4 {
				self.iYec36_perm[j395 as usize] = iYec36_tmp[(i32::wrapping_add(vsize, j395)) as usize];
			}
			for j397 in 0..4 {
				self.iRec321_perm[j397 as usize] = iRec321_tmp[(i32::wrapping_add(vsize, j397)) as usize];
			}
			for j393 in 0..4 {
				self.fRec320_perm[j393 as usize] = fRec320_tmp[(i32::wrapping_add(vsize, j393)) as usize];
			}
			for j399 in 0..4 {
				self.fRec319_perm[j399 as usize] = fRec319_tmp[(i32::wrapping_add(vsize, j399)) as usize];
			}
			for j391 in 0..4 {
				self.fRec317_perm[j391 as usize] = fRec317_tmp[(i32::wrapping_add(vsize, j391)) as usize];
			}
			self.fRec318_idx_save = vsize;
			self.fRec314_idx_save = vsize;
			for j383 in 0..4 {
				self.fRec313_perm[j383 as usize] = fRec313_tmp[(i32::wrapping_add(vsize, j383)) as usize];
			}
			for j381 in 0..4 {
				self.fRec308_perm[j381 as usize] = fRec308_tmp[(i32::wrapping_add(vsize, j381)) as usize];
			}
			for j405 in 0..4 {
				self.fRec304_perm[j405 as usize] = fRec304_tmp[(i32::wrapping_add(vsize, j405)) as usize];
			}
			for j407 in 0..4 {
				self.fRec300_perm[j407 as usize] = fRec300_tmp[(i32::wrapping_add(vsize, j407)) as usize];
			}
			self.fRec296_idx_save = vsize;
			for j409 in 0..4 {
				self.fRec298_perm[j409 as usize] = fRec298_tmp[(i32::wrapping_add(vsize, j409)) as usize];
			}
			for j411 in 0..4 {
				self.fRec294_perm[j411 as usize] = fRec294_tmp[(i32::wrapping_add(vsize, j411)) as usize];
			}
			for j413 in 0..4 {
				self.fRec289_perm[j413 as usize] = fRec289_tmp[(i32::wrapping_add(vsize, j413)) as usize];
			}
			self.fRec285_idx_save = vsize;
			for j415 in 0..4 {
				self.fRec283_perm[j415 as usize] = fRec283_tmp[(i32::wrapping_add(vsize, j415)) as usize];
			}
			for j419 in 0..4 {
				self.fRec282_perm[j419 as usize] = fRec282_tmp[(i32::wrapping_add(vsize, j419)) as usize];
			}
			/* Vectorizable loop 349 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec318[i as usize] = 1.0 / mydsp_faustpower2_f(fZec315[i as usize]);
			}
			/* Vectorizable loop 350 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec317[i as usize] = fZec85[i as usize] + (fZec82[i as usize] + fZec316[i as usize]) / fZec315[i as usize] + 1.0;
			}
			/* Vectorizable loop 351 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec367[i as usize] = fZec85[i as usize] + (fZec82[i as usize] + fZec366[i as usize]) / fZec365[i as usize] + 1.0;
			}
			/* Vectorizable loop 352 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec368[i as usize] = 1.0 / mydsp_faustpower2_f(fZec365[i as usize]);
			}
			/* Vectorizable loop 353 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec12[i as usize] = (fZec11[i as usize]) as i32;
			}
			/* Vectorizable loop 354 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec19[i as usize] = F32::floor(fZec17[i as usize]);
			}
			/* Vectorizable loop 355 */
			/* Pre code */
			self.fYec3_idx = (i32::wrapping_add(self.fYec3_idx, self.fYec3_idx_save)) & 4095;
			/* Compute code */
			for i in 0..output0.len() as i32 {
				self.fYec3[((i32::wrapping_add(i, self.fYec3_idx)) & 4095) as usize] = fZec6[i as usize] * (fYec2_tmp[(i32::wrapping_add(4, i)) as usize] - fYec2_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) / fZec9[i as usize];
			}
			/* Post code */
			self.fYec3_idx_save = vsize;
			/* Vectorizable loop 356 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec25[i as usize] = F32::floor(fZec23[i as usize]);
			}
			/* Vectorizable loop 357 */
			/* Pre code */
			self.fYec5_idx = (i32::wrapping_add(self.fYec5_idx, self.fYec5_idx_save)) & 4095;
			/* Compute code */
			for i in 0..output0.len() as i32 {
				self.fYec5[((i32::wrapping_add(i, self.fYec5_idx)) & 4095) as usize] = fZec6[i as usize] * (fYec4_tmp[(i32::wrapping_add(4, i)) as usize] - fYec4_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) / fZec15[i as usize];
			}
			/* Post code */
			self.fYec5_idx_save = vsize;
			/* Vectorizable loop 358 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec7[i as usize] = (fZec4[i as usize]) as i32;
			}
			/* Vectorizable loop 359 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec24[i as usize] = (fZec23[i as usize]) as i32;
			}
			/* Vectorizable loop 360 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec13[i as usize] = F32::floor(fZec11[i as usize]);
			}
			/* Vectorizable loop 361 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec18[i as usize] = (fZec17[i as usize]) as i32;
			}
			/* Vectorizable loop 362 */
			/* Pre code */
			self.fYec7_idx = (i32::wrapping_add(self.fYec7_idx, self.fYec7_idx_save)) & 4095;
			/* Compute code */
			for i in 0..output0.len() as i32 {
				self.fYec7[((i32::wrapping_add(i, self.fYec7_idx)) & 4095) as usize] = fZec6[i as usize] * (fYec6_tmp[(i32::wrapping_add(4, i)) as usize] - fYec6_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) / fZec21[i as usize];
			}
			/* Post code */
			self.fYec7_idx_save = vsize;
			/* Vectorizable loop 363 */
			/* Pre code */
			self.fYec15_idx = (i32::wrapping_add(self.fYec15_idx, self.fYec15_idx_save)) & 4095;
			/* Compute code */
			for i in 0..output0.len() as i32 {
				self.fYec15[((i32::wrapping_add(i, self.fYec15_idx)) & 4095) as usize] = fZec6[i as usize] * (fYec14_tmp[(i32::wrapping_add(4, i)) as usize] - fYec14_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) / fZec45[i as usize];
			}
			/* Post code */
			self.fYec15_idx_save = vsize;
			/* Vectorizable loop 364 */
			/* Pre code */
			self.fYec19_idx = (i32::wrapping_add(self.fYec19_idx, self.fYec19_idx_save)) & 4095;
			/* Compute code */
			for i in 0..output0.len() as i32 {
				self.fYec19[((i32::wrapping_add(i, self.fYec19_idx)) & 4095) as usize] = fZec6[i as usize] * (fYec18_tmp[(i32::wrapping_add(4, i)) as usize] - fYec18_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) / fZec57[i as usize];
			}
			/* Post code */
			self.fYec19_idx_save = vsize;
			/* Vectorizable loop 365 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec67[i as usize] = (fZec65[i as usize]) as i32;
			}
			/* Vectorizable loop 366 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec49[i as usize] = (fZec47[i as usize]) as i32;
			}
			/* Vectorizable loop 367 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec60[i as usize] = (fZec59[i as usize]) as i32;
			}
			/* Vectorizable loop 368 */
			/* Pre code */
			self.fYec17_idx = (i32::wrapping_add(self.fYec17_idx, self.fYec17_idx_save)) & 4095;
			/* Compute code */
			for i in 0..output0.len() as i32 {
				self.fYec17[((i32::wrapping_add(i, self.fYec17_idx)) & 4095) as usize] = fZec6[i as usize] * (fYec16_tmp[(i32::wrapping_add(4, i)) as usize] - fYec16_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) / fZec51[i as usize];
			}
			/* Post code */
			self.fYec17_idx_save = vsize;
			/* Vectorizable loop 369 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec54[i as usize] = (fZec53[i as usize]) as i32;
			}
			/* Vectorizable loop 370 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec61[i as usize] = F32::floor(fZec59[i as usize]);
			}
			/* Vectorizable loop 371 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec66[i as usize] = F32::floor(fZec65[i as usize]);
			}
			/* Vectorizable loop 372 */
			/* Pre code */
			self.fYec21_idx = (i32::wrapping_add(self.fYec21_idx, self.fYec21_idx_save)) & 4095;
			/* Compute code */
			for i in 0..output0.len() as i32 {
				self.fYec21[((i32::wrapping_add(i, self.fYec21_idx)) & 4095) as usize] = fZec6[i as usize] * (fYec20_tmp[(i32::wrapping_add(4, i)) as usize] - fYec20_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) / fZec63[i as usize];
			}
			/* Post code */
			self.fYec21_idx_save = vsize;
			/* Vectorizable loop 373 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec55[i as usize] = F32::floor(fZec53[i as usize]);
			}
			/* Vectorizable loop 374 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec88[i as usize] = 1.0 - fZec81[i as usize];
			}
			/* Vectorizable loop 375 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec72[i as usize] = (fZec71[i as usize]) as i32;
			}
			/* Vectorizable loop 376 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec89[i as usize] = 2.0 - fZec81[i as usize];
			}
			/* Vectorizable loop 377 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec73[i as usize] = F32::floor(fZec71[i as usize]);
			}
			/* Recursive loop 378 */
			/* Pre code */
			for j128 in 0..4 {
				fRec69_tmp[j128 as usize] = self.fRec69_perm[j128 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec69_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow37 + self.fConst2 * fRec69_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
			}
			/* Post code */
			for j129 in 0..4 {
				self.fRec69_perm[j129 as usize] = fRec69_tmp[(i32::wrapping_add(vsize, j129)) as usize];
			}
			/* Vectorizable loop 379 */
			/* Pre code */
			self.fYec23_idx = (i32::wrapping_add(self.fYec23_idx, self.fYec23_idx_save)) & 4095;
			/* Compute code */
			for i in 0..output0.len() as i32 {
				self.fYec23[((i32::wrapping_add(i, self.fYec23_idx)) & 4095) as usize] = fZec6[i as usize] * (fYec22_tmp[(i32::wrapping_add(4, i)) as usize] - fYec22_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) / fZec69[i as usize];
			}
			/* Post code */
			self.fYec23_idx_save = vsize;
			/* Recursive loop 380 */
			/* Pre code */
			for j134 in 0..4 {
				fRec70_tmp[j134 as usize] = self.fRec70_perm[j134 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec75[i as usize] = fRec70_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + self.fConst4 * fZec74[i as usize];
				fZec76[i as usize] = fZec75[i as usize] + -1.0;
				iZec77[i as usize] = (fZec76[i as usize] < 0.0) as i32;
				fRec70_tmp[(i32::wrapping_add(4, i)) as usize] = if iZec77[i as usize] != 0 {fZec75[i as usize]} else {fZec76[i as usize]};
				fRec71[i as usize] = if iZec77[i as usize] != 0 {fZec75[i as usize]} else {fZec75[i as usize] + (1.0 - self.fConst0 / fZec74[i as usize]) * fZec76[i as usize]};
			}
			/* Post code */
			for j135 in 0..4 {
				self.fRec70_perm[j135 as usize] = fRec70_tmp[(i32::wrapping_add(vsize, j135)) as usize];
			}
			/* Vectorizable loop 381 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec86[i as usize] = fZec85[i as usize] + (fZec83[i as usize] + fZec82[i as usize]) / fZec79[i as usize] + 1.0;
			}
			/* Vectorizable loop 382 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec87[i as usize] = 1.0 / mydsp_faustpower2_f(fZec79[i as usize]);
			}
			/* Recursive loop 383 */
			/* Pre code */
			for j170 in 0..4 {
				fRec93_tmp[j170 as usize] = self.fRec93_perm[j170 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec93_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow46 + self.fConst2 * fRec93_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
			}
			/* Post code */
			for j171 in 0..4 {
				self.fRec93_perm[j171 as usize] = fRec93_tmp[(i32::wrapping_add(vsize, j171)) as usize];
			}
			/* Recursive loop 384 */
			/* Pre code */
			for j174 in 0..4 {
				fRec94_tmp[j174 as usize] = self.fRec94_perm[j174 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec111[i as usize] = fRec94_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + self.fConst4 * fZec110[i as usize];
				fZec112[i as usize] = fZec111[i as usize] + -1.0;
				iZec113[i as usize] = (fZec112[i as usize] < 0.0) as i32;
				fRec94_tmp[(i32::wrapping_add(4, i)) as usize] = if iZec113[i as usize] != 0 {fZec111[i as usize]} else {fZec112[i as usize]};
				fRec95[i as usize] = if iZec113[i as usize] != 0 {fZec111[i as usize]} else {fZec111[i as usize] + (1.0 - self.fConst0 / fZec110[i as usize]) * fZec112[i as usize]};
			}
			/* Post code */
			for j175 in 0..4 {
				self.fRec94_perm[j175 as usize] = fRec94_tmp[(i32::wrapping_add(vsize, j175)) as usize];
			}
			/* Recursive loop 385 */
			/* Pre code */
			for j158 in 0..4 {
				fRec86_tmp[j158 as usize] = self.fRec86_perm[j158 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec86_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow44 + self.fConst2 * fRec86_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
			}
			/* Post code */
			for j159 in 0..4 {
				self.fRec86_perm[j159 as usize] = fRec86_tmp[(i32::wrapping_add(vsize, j159)) as usize];
			}
			/* Vectorizable loop 386 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec99[i as usize] = 1.0 / mydsp_faustpower2_f(fZec96[i as usize]);
			}
			/* Recursive loop 387 */
			/* Pre code */
			for j162 in 0..4 {
				fRec87_tmp[j162 as usize] = self.fRec87_perm[j162 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec102[i as usize] = fRec87_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + self.fConst4 * fZec101[i as usize];
				fZec103[i as usize] = fZec102[i as usize] + -1.0;
				iZec104[i as usize] = (fZec103[i as usize] < 0.0) as i32;
				fRec87_tmp[(i32::wrapping_add(4, i)) as usize] = if iZec104[i as usize] != 0 {fZec102[i as usize]} else {fZec103[i as usize]};
				fRec88[i as usize] = if iZec104[i as usize] != 0 {fZec102[i as usize]} else {fZec102[i as usize] + (1.0 - self.fConst0 / fZec101[i as usize]) * fZec103[i as usize]};
			}
			/* Post code */
			for j163 in 0..4 {
				self.fRec87_perm[j163 as usize] = fRec87_tmp[(i32::wrapping_add(vsize, j163)) as usize];
			}
			/* Vectorizable loop 388 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec107[i as usize] = fZec85[i as usize] + (fZec82[i as usize] + fZec106[i as usize]) / fZec105[i as usize] + 1.0;
			}
			/* Vectorizable loop 389 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec98[i as usize] = fZec85[i as usize] + (fZec82[i as usize] + fZec97[i as usize]) / fZec96[i as usize] + 1.0;
			}
			/* Recursive loop 390 */
			/* Pre code */
			for j150 in 0..4 {
				fRec80_tmp[j150 as usize] = self.fRec80_perm[j150 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec93[i as usize] = fRec80_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + fZec92[i as usize];
				fZec94[i as usize] = fZec93[i as usize] + -1.0;
				iZec95[i as usize] = (fZec94[i as usize] < 0.0) as i32;
				fRec80_tmp[(i32::wrapping_add(4, i)) as usize] = if iZec95[i as usize] != 0 {fZec93[i as usize]} else {fZec94[i as usize]};
				fRec81[i as usize] = if iZec95[i as usize] != 0 {fZec93[i as usize]} else {fRec80_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + fZec92[i as usize] + (1.0 - self.fConst0 / fZec91[i as usize]) * fZec94[i as usize]};
			}
			/* Post code */
			for j151 in 0..4 {
				self.fRec80_perm[j151 as usize] = fRec80_tmp[(i32::wrapping_add(vsize, j151)) as usize];
			}
			/* Recursive loop 391 */
			/* Pre code */
			for j146 in 0..4 {
				fRec79_tmp[j146 as usize] = self.fRec79_perm[j146 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec79_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow42 + self.fConst2 * fRec79_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
			}
			/* Post code */
			for j147 in 0..4 {
				self.fRec79_perm[j147 as usize] = fRec79_tmp[(i32::wrapping_add(vsize, j147)) as usize];
			}
			/* Vectorizable loop 392 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec108[i as usize] = 1.0 / mydsp_faustpower2_f(fZec105[i as usize]);
			}
			/* Vectorizable loop 393 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec31[i as usize] = (fZec29[i as usize]) as i32;
			}
			/* Vectorizable loop 394 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec37[i as usize] = F32::floor(fZec35[i as usize]);
			}
			/* Vectorizable loop 395 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec48[i as usize] = F32::floor(fZec47[i as usize]);
			}
			/* Vectorizable loop 396 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec42[i as usize] = (fZec41[i as usize]) as i32;
			}
			/* Vectorizable loop 397 */
			/* Pre code */
			self.fYec9_idx = (i32::wrapping_add(self.fYec9_idx, self.fYec9_idx_save)) & 4095;
			/* Compute code */
			for i in 0..output0.len() as i32 {
				self.fYec9[((i32::wrapping_add(i, self.fYec9_idx)) & 4095) as usize] = fZec6[i as usize] * (fYec8_tmp[(i32::wrapping_add(4, i)) as usize] - fYec8_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) / fZec27[i as usize];
			}
			/* Post code */
			self.fYec9_idx_save = vsize;
			/* Vectorizable loop 398 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec30[i as usize] = F32::floor(fZec29[i as usize]);
			}
			/* Vectorizable loop 399 */
			/* Pre code */
			self.fYec13_idx = (i32::wrapping_add(self.fYec13_idx, self.fYec13_idx_save)) & 4095;
			/* Compute code */
			for i in 0..output0.len() as i32 {
				self.fYec13[((i32::wrapping_add(i, self.fYec13_idx)) & 4095) as usize] = fZec6[i as usize] * (fYec12_tmp[(i32::wrapping_add(4, i)) as usize] - fYec12_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) / fZec39[i as usize];
			}
			/* Post code */
			self.fYec13_idx_save = vsize;
			/* Vectorizable loop 400 */
			/* Pre code */
			self.fYec11_idx = (i32::wrapping_add(self.fYec11_idx, self.fYec11_idx_save)) & 4095;
			/* Compute code */
			for i in 0..output0.len() as i32 {
				self.fYec11[((i32::wrapping_add(i, self.fYec11_idx)) & 4095) as usize] = fZec6[i as usize] * (fYec10_tmp[(i32::wrapping_add(4, i)) as usize] - fYec10_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) / fZec33[i as usize];
			}
			/* Post code */
			self.fYec11_idx_save = vsize;
			/* Vectorizable loop 401 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec43[i as usize] = F32::floor(fZec41[i as usize]);
			}
			/* Vectorizable loop 402 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec36[i as usize] = (fZec35[i as usize]) as i32;
			}
			/* Vectorizable loop 403 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec168[i as usize] = 1.0 / mydsp_faustpower2_f(fZec165[i as usize]);
			}
			/* Recursive loop 404 */
			/* Pre code */
			for j236 in 0..4 {
				fRec173_tmp[j236 as usize] = self.fRec173_perm[j236 as usize];
			}
			for j238 in 0..4 {
				fRec178_tmp[j238 as usize] = self.fRec178_perm[j238 as usize];
			}
			for j246 in 0..4 {
				fRec182_tmp[j246 as usize] = self.fRec182_perm[j246 as usize];
			}
			self.fRec183_idx = (i32::wrapping_add(self.fRec183_idx, self.fRec183_idx_save)) & 2047;
			for j248 in 0..4 {
				fRec185_tmp[j248 as usize] = self.fRec185_perm[j248 as usize];
			}
			for j250 in 0..4 {
				iYec27_tmp[j250 as usize] = self.iYec27_perm[j250 as usize];
			}
			for j252 in 0..4 {
				iRec186_tmp[j252 as usize] = self.iRec186_perm[j252 as usize];
			}
			for j254 in 0..4 {
				fRec184_tmp[j254 as usize] = self.fRec184_perm[j254 as usize];
			}
			for j256 in 0..4 {
				fYec28_tmp[j256 as usize] = self.fYec28_perm[j256 as usize];
			}
			for j258 in 0..4 {
				fYec29_tmp[j258 as usize] = self.fYec29_perm[j258 as usize];
			}
			self.fRec179_idx = (i32::wrapping_add(self.fRec179_idx, self.fRec179_idx_save)) & 2047;
			for j260 in 0..4 {
				fRec169_tmp[j260 as usize] = self.fRec169_perm[j260 as usize];
			}
			for j262 in 0..4 {
				fRec165_tmp[j262 as usize] = self.fRec165_perm[j262 as usize];
			}
			self.fRec161_idx = (i32::wrapping_add(self.fRec161_idx, self.fRec161_idx_save)) & 2047;
			for j264 in 0..4 {
				fRec163_tmp[j264 as usize] = self.fRec163_perm[j264 as usize];
			}
			for j266 in 0..4 {
				fRec159_tmp[j266 as usize] = self.fRec159_perm[j266 as usize];
			}
			for j268 in 0..4 {
				fRec154_tmp[j268 as usize] = self.fRec154_perm[j268 as usize];
			}
			self.fRec150_idx = (i32::wrapping_add(self.fRec150_idx, self.fRec150_idx_save)) & 2047;
			for j270 in 0..4 {
				fRec148_tmp[j270 as usize] = self.fRec148_perm[j270 as usize];
			}
			for j272 in 0..4 {
				fRec188_tmp[j272 as usize] = self.fRec188_perm[j272 as usize];
			}
			for j274 in 0..4 {
				fRec147_tmp[j274 as usize] = self.fRec147_perm[j274 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec158[i as usize] = -1.0 * 0.9973053 * (0.9 * fRec159_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + 0.05 * (fRec159_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + fRec159_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 3))) as usize]));
				fRec173_tmp[(i32::wrapping_add(4, i)) as usize] = self.fRec150[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec150_idx), iZec200[i as usize])) & 2047) as usize] * fZec198[i as usize] * fZec197[i as usize] * fZec196[i as usize] * fZec195[i as usize] + fZec194[i as usize] * (self.fRec150[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec150_idx), iZec193[i as usize])) & 2047) as usize] * fZec191[i as usize] * fZec190[i as usize] * fZec189[i as usize] + 0.5 * fZec178[i as usize] * self.fRec150[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec150_idx), iZec188[i as usize])) & 2047) as usize] * fZec186[i as usize] * fZec185[i as usize] + 0.16666667 * fZec179[i as usize] * self.fRec150[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec150_idx), iZec184[i as usize])) & 2047) as usize] * fZec182[i as usize] + 0.041666668 * fZec180[i as usize] * self.fRec150[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec150_idx), iZec174[i as usize])) & 2047) as usize]);
				fRec178_tmp[(i32::wrapping_add(4, i)) as usize] = 0.95 * fRec173_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + 0.05 * fRec178_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fRec174[i as usize] = fRec178_tmp[(i32::wrapping_add(4, i)) as usize];
				fRec182_tmp[(i32::wrapping_add(4, i)) as usize] = fRec148_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				self.fRec183[((i32::wrapping_add(i, self.fRec183_idx)) & 2047) as usize] = -1.0 * 0.9973053 * (0.9 * fRec182_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + 0.05 * (fRec182_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + fRec182_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 3))) as usize]));
				fRec185_tmp[(i32::wrapping_add(4, i)) as usize] = self.fConst12 * F32::abs(fRec147_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) + self.fConst11 * fRec185_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				iYec27_tmp[(i32::wrapping_add(4, i)) as usize] = (fRec185_tmp[(i32::wrapping_add(4, i)) as usize] > 0.1) as i32;
				iRec186_tmp[(i32::wrapping_add(4, i)) as usize] = std::cmp::max(i32::wrapping_mul(self.iConst13, (iYec27_tmp[(i32::wrapping_add(4, i)) as usize] < iYec27_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) as i32), i32::wrapping_add(iRec186_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize], -1));
				fZec204[i as usize] = F32::abs(F32::max((iYec27_tmp[(i32::wrapping_add(4, i)) as usize]) as F32, ((iRec186_tmp[(i32::wrapping_add(4, i)) as usize] > 0) as i32) as u32 as F32));
				fZec205[i as usize] = if (fZec204[i as usize] > fRec184_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) as i32 != 0 {self.fConst11} else {self.fConst14};
				fRec184_tmp[(i32::wrapping_add(4, i)) as usize] = fZec204[i as usize] * (1.0 - fZec205[i as usize]) + fRec184_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] * fZec205[i as usize];
				fZec206[i as usize] = 0.005 * fRec184_tmp[(i32::wrapping_add(4, i)) as usize] * fRec147_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fYec28_tmp[(i32::wrapping_add(4, i)) as usize] = fZec210[i as usize] * self.fRec183[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec183_idx), i32::wrapping_add(iZec199[i as usize], 2))) & 2047) as usize] + fZec194[i as usize] * (fZec209[i as usize] * self.fRec183[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec183_idx), i32::wrapping_add(iZec192[i as usize], 2))) & 2047) as usize] + 0.5 * fZec208[i as usize] * self.fRec183[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec183_idx), i32::wrapping_add(iZec187[i as usize], 2))) & 2047) as usize] + 0.16666667 * fZec207[i as usize] * self.fRec183[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec183_idx), i32::wrapping_add(iZec183[i as usize], 2))) & 2047) as usize] + 0.041666668 * fZec180[i as usize] * self.fRec183[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec183_idx), i32::wrapping_add(iZec173[i as usize], 2))) & 2047) as usize]);
				fYec29_tmp[(i32::wrapping_add(4, i)) as usize] = fZec212[i as usize] + fYec28_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + fZec206[i as usize];
				self.fRec179[((i32::wrapping_add(i, self.fRec179_idx)) & 2047) as usize] = 0.95 * fYec29_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + 0.05 * self.fRec179[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec179_idx), 1)) & 2047) as usize];
				fRec175[i as usize] = fZec210[i as usize] * self.fRec179[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec179_idx), iZec199[i as usize])) & 2047) as usize] + fZec194[i as usize] * (fZec209[i as usize] * self.fRec179[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec179_idx), iZec192[i as usize])) & 2047) as usize] + 0.5 * fZec208[i as usize] * self.fRec179[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec179_idx), iZec187[i as usize])) & 2047) as usize] + 0.16666667 * fZec207[i as usize] * self.fRec179[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec179_idx), iZec183[i as usize])) & 2047) as usize] + 0.041666668 * fZec180[i as usize] * self.fRec179[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec179_idx), iZec173[i as usize])) & 2047) as usize]);
				fRec176[i as usize] = fYec29_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + fRec169_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fRec169_tmp[(i32::wrapping_add(4, i)) as usize] = fRec174[i as usize];
				fRec170[i as usize] = fRec169_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fRec171[i as usize] = fRec175[i as usize];
				fRec172[i as usize] = fRec176[i as usize];
				fRec165_tmp[(i32::wrapping_add(4, i)) as usize] = fRec170[i as usize];
				fRec166[i as usize] = fZec206[i as usize] + fZec212[i as usize] + fRec165_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fRec167[i as usize] = fRec171[i as usize];
				fRec168[i as usize] = fRec172[i as usize];
				self.fRec161[((i32::wrapping_add(i, self.fRec161_idx)) & 2047) as usize] = fRec166[i as usize];
				fRec162[i as usize] = fZec210[i as usize] * self.fRec161[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec161_idx), iZec200[i as usize])) & 2047) as usize] + fZec194[i as usize] * (fZec209[i as usize] * self.fRec161[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec161_idx), iZec193[i as usize])) & 2047) as usize] + 0.5 * fZec208[i as usize] * self.fRec161[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec161_idx), iZec188[i as usize])) & 2047) as usize] + 0.16666667 * fZec207[i as usize] * self.fRec161[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec161_idx), iZec184[i as usize])) & 2047) as usize] + 0.041666668 * fZec180[i as usize] * self.fRec161[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec161_idx), iZec174[i as usize])) & 2047) as usize]);
				fRec163_tmp[(i32::wrapping_add(4, i)) as usize] = fRec167[i as usize];
				fRec164[i as usize] = fRec168[i as usize];
				fRec159_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow51 * fRec163_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fRec160[i as usize] = fRec164[i as usize];
				fRec154_tmp[(i32::wrapping_add(4, i)) as usize] = fRec158[i as usize];
				fRec155[i as usize] = fSlow51 * fRec154_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fRec156[i as usize] = fRec159_tmp[(i32::wrapping_add(4, i)) as usize];
				fRec157[i as usize] = fRec160[i as usize];
				self.fRec150[((i32::wrapping_add(i, self.fRec150_idx)) & 2047) as usize] = fRec155[i as usize];
				fRec151[i as usize] = fRec162[i as usize];
				fRec152[i as usize] = fRec156[i as usize];
				fRec153[i as usize] = fRec157[i as usize];
				fRec148_tmp[(i32::wrapping_add(4, i)) as usize] = fRec151[i as usize];
				fRec149[i as usize] = fRec153[i as usize];
				fZec213[i as usize] = F32::abs(fRec149[i as usize]);
				fZec214[i as usize] = if (fZec213[i as usize] > fRec188_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) as i32 != 0 {0.0} else {self.fConst16};
				fRec188_tmp[(i32::wrapping_add(4, i)) as usize] = fZec213[i as usize] * (1.0 - fZec214[i as usize]) + fRec188_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] * fZec214[i as usize];
				fRec187[i as usize] = 0.0 - 0.95 * F32::max(2e+01 * F32::log10(F32::max(1.1754944e-38, fRec188_tmp[(i32::wrapping_add(4, i)) as usize])) + 1e+01, 0.0);
				fRec147_tmp[(i32::wrapping_add(4, i)) as usize] = 1.5 * fRec149[i as usize] * F32::powf(1e+01, 0.05 * fRec187[i as usize]);
			}
			/* Post code */
			for j273 in 0..4 {
				self.fRec188_perm[j273 as usize] = fRec188_tmp[(i32::wrapping_add(vsize, j273)) as usize];
			}
			for j257 in 0..4 {
				self.fYec28_perm[j257 as usize] = fYec28_tmp[(i32::wrapping_add(vsize, j257)) as usize];
			}
			for j259 in 0..4 {
				self.fYec29_perm[j259 as usize] = fYec29_tmp[(i32::wrapping_add(vsize, j259)) as usize];
			}
			for j251 in 0..4 {
				self.iYec27_perm[j251 as usize] = iYec27_tmp[(i32::wrapping_add(vsize, j251)) as usize];
			}
			for j253 in 0..4 {
				self.iRec186_perm[j253 as usize] = iRec186_tmp[(i32::wrapping_add(vsize, j253)) as usize];
			}
			for j249 in 0..4 {
				self.fRec185_perm[j249 as usize] = fRec185_tmp[(i32::wrapping_add(vsize, j249)) as usize];
			}
			for j255 in 0..4 {
				self.fRec184_perm[j255 as usize] = fRec184_tmp[(i32::wrapping_add(vsize, j255)) as usize];
			}
			for j247 in 0..4 {
				self.fRec182_perm[j247 as usize] = fRec182_tmp[(i32::wrapping_add(vsize, j247)) as usize];
			}
			self.fRec183_idx_save = vsize;
			self.fRec179_idx_save = vsize;
			for j239 in 0..4 {
				self.fRec178_perm[j239 as usize] = fRec178_tmp[(i32::wrapping_add(vsize, j239)) as usize];
			}
			for j237 in 0..4 {
				self.fRec173_perm[j237 as usize] = fRec173_tmp[(i32::wrapping_add(vsize, j237)) as usize];
			}
			for j261 in 0..4 {
				self.fRec169_perm[j261 as usize] = fRec169_tmp[(i32::wrapping_add(vsize, j261)) as usize];
			}
			for j263 in 0..4 {
				self.fRec165_perm[j263 as usize] = fRec165_tmp[(i32::wrapping_add(vsize, j263)) as usize];
			}
			self.fRec161_idx_save = vsize;
			for j265 in 0..4 {
				self.fRec163_perm[j265 as usize] = fRec163_tmp[(i32::wrapping_add(vsize, j265)) as usize];
			}
			for j267 in 0..4 {
				self.fRec159_perm[j267 as usize] = fRec159_tmp[(i32::wrapping_add(vsize, j267)) as usize];
			}
			for j269 in 0..4 {
				self.fRec154_perm[j269 as usize] = fRec154_tmp[(i32::wrapping_add(vsize, j269)) as usize];
			}
			self.fRec150_idx_save = vsize;
			for j271 in 0..4 {
				self.fRec148_perm[j271 as usize] = fRec148_tmp[(i32::wrapping_add(vsize, j271)) as usize];
			}
			for j275 in 0..4 {
				self.fRec147_perm[j275 as usize] = fRec147_tmp[(i32::wrapping_add(vsize, j275)) as usize];
			}
			/* Vectorizable loop 405 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec167[i as usize] = fZec85[i as usize] + (fZec82[i as usize] + fZec166[i as usize]) / fZec165[i as usize] + 1.0;
			}
			/* Vectorizable loop 406 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec217[i as usize] = fZec85[i as usize] + (fZec82[i as usize] + fZec216[i as usize]) / fZec215[i as usize] + 1.0;
			}
			/* Recursive loop 407 */
			/* Pre code */
			for j284 in 0..4 {
				fRec218_tmp[j284 as usize] = self.fRec218_perm[j284 as usize];
			}
			for j286 in 0..4 {
				fRec223_tmp[j286 as usize] = self.fRec223_perm[j286 as usize];
			}
			for j294 in 0..4 {
				fRec227_tmp[j294 as usize] = self.fRec227_perm[j294 as usize];
			}
			self.fRec228_idx = (i32::wrapping_add(self.fRec228_idx, self.fRec228_idx_save)) & 2047;
			for j296 in 0..4 {
				fRec230_tmp[j296 as usize] = self.fRec230_perm[j296 as usize];
			}
			for j298 in 0..4 {
				iYec30_tmp[j298 as usize] = self.iYec30_perm[j298 as usize];
			}
			for j300 in 0..4 {
				iRec231_tmp[j300 as usize] = self.iRec231_perm[j300 as usize];
			}
			for j302 in 0..4 {
				fRec229_tmp[j302 as usize] = self.fRec229_perm[j302 as usize];
			}
			for j304 in 0..4 {
				fYec31_tmp[j304 as usize] = self.fYec31_perm[j304 as usize];
			}
			for j306 in 0..4 {
				fYec32_tmp[j306 as usize] = self.fYec32_perm[j306 as usize];
			}
			self.fRec224_idx = (i32::wrapping_add(self.fRec224_idx, self.fRec224_idx_save)) & 2047;
			for j308 in 0..4 {
				fRec214_tmp[j308 as usize] = self.fRec214_perm[j308 as usize];
			}
			for j310 in 0..4 {
				fRec210_tmp[j310 as usize] = self.fRec210_perm[j310 as usize];
			}
			self.fRec206_idx = (i32::wrapping_add(self.fRec206_idx, self.fRec206_idx_save)) & 2047;
			for j312 in 0..4 {
				fRec208_tmp[j312 as usize] = self.fRec208_perm[j312 as usize];
			}
			for j314 in 0..4 {
				fRec204_tmp[j314 as usize] = self.fRec204_perm[j314 as usize];
			}
			for j316 in 0..4 {
				fRec199_tmp[j316 as usize] = self.fRec199_perm[j316 as usize];
			}
			self.fRec195_idx = (i32::wrapping_add(self.fRec195_idx, self.fRec195_idx_save)) & 2047;
			for j318 in 0..4 {
				fRec193_tmp[j318 as usize] = self.fRec193_perm[j318 as usize];
			}
			for j320 in 0..4 {
				fRec233_tmp[j320 as usize] = self.fRec233_perm[j320 as usize];
			}
			for j322 in 0..4 {
				fRec192_tmp[j322 as usize] = self.fRec192_perm[j322 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec203[i as usize] = -1.0 * 0.9973053 * (0.9 * fRec204_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + 0.05 * (fRec204_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + fRec204_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 3))) as usize]));
				fRec218_tmp[(i32::wrapping_add(4, i)) as usize] = self.fRec195[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec195_idx), iZec250[i as usize])) & 2047) as usize] * fZec248[i as usize] * fZec247[i as usize] * fZec246[i as usize] * fZec245[i as usize] + fZec244[i as usize] * (self.fRec195[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec195_idx), iZec243[i as usize])) & 2047) as usize] * fZec241[i as usize] * fZec240[i as usize] * fZec239[i as usize] + 0.5 * fZec228[i as usize] * self.fRec195[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec195_idx), iZec238[i as usize])) & 2047) as usize] * fZec236[i as usize] * fZec235[i as usize] + 0.16666667 * fZec229[i as usize] * self.fRec195[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec195_idx), iZec234[i as usize])) & 2047) as usize] * fZec232[i as usize] + 0.041666668 * fZec230[i as usize] * self.fRec195[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec195_idx), iZec224[i as usize])) & 2047) as usize]);
				fRec223_tmp[(i32::wrapping_add(4, i)) as usize] = 0.95 * fRec218_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + 0.05 * fRec223_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fRec219[i as usize] = fRec223_tmp[(i32::wrapping_add(4, i)) as usize];
				fRec227_tmp[(i32::wrapping_add(4, i)) as usize] = fRec193_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				self.fRec228[((i32::wrapping_add(i, self.fRec228_idx)) & 2047) as usize] = -1.0 * 0.9973053 * (0.9 * fRec227_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + 0.05 * (fRec227_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + fRec227_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 3))) as usize]));
				fRec230_tmp[(i32::wrapping_add(4, i)) as usize] = self.fConst12 * F32::abs(fRec192_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) + self.fConst11 * fRec230_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				iYec30_tmp[(i32::wrapping_add(4, i)) as usize] = (fRec230_tmp[(i32::wrapping_add(4, i)) as usize] > 0.1) as i32;
				iRec231_tmp[(i32::wrapping_add(4, i)) as usize] = std::cmp::max(i32::wrapping_mul(self.iConst13, (iYec30_tmp[(i32::wrapping_add(4, i)) as usize] < iYec30_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) as i32), i32::wrapping_add(iRec231_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize], -1));
				fZec254[i as usize] = F32::abs(F32::max((iYec30_tmp[(i32::wrapping_add(4, i)) as usize]) as F32, ((iRec231_tmp[(i32::wrapping_add(4, i)) as usize] > 0) as i32) as u32 as F32));
				fZec255[i as usize] = if (fZec254[i as usize] > fRec229_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) as i32 != 0 {self.fConst11} else {self.fConst14};
				fRec229_tmp[(i32::wrapping_add(4, i)) as usize] = fZec254[i as usize] * (1.0 - fZec255[i as usize]) + fRec229_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] * fZec255[i as usize];
				fZec256[i as usize] = 0.005 * fRec229_tmp[(i32::wrapping_add(4, i)) as usize] * fRec192_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fYec31_tmp[(i32::wrapping_add(4, i)) as usize] = fZec260[i as usize] * self.fRec228[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec228_idx), i32::wrapping_add(iZec249[i as usize], 2))) & 2047) as usize] + fZec244[i as usize] * (fZec259[i as usize] * self.fRec228[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec228_idx), i32::wrapping_add(iZec242[i as usize], 2))) & 2047) as usize] + 0.5 * fZec258[i as usize] * self.fRec228[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec228_idx), i32::wrapping_add(iZec237[i as usize], 2))) & 2047) as usize] + 0.16666667 * fZec257[i as usize] * self.fRec228[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec228_idx), i32::wrapping_add(iZec233[i as usize], 2))) & 2047) as usize] + 0.041666668 * fZec230[i as usize] * self.fRec228[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec228_idx), i32::wrapping_add(iZec223[i as usize], 2))) & 2047) as usize]);
				fYec32_tmp[(i32::wrapping_add(4, i)) as usize] = fZec262[i as usize] + fYec31_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + fZec256[i as usize];
				self.fRec224[((i32::wrapping_add(i, self.fRec224_idx)) & 2047) as usize] = 0.95 * fYec32_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + 0.05 * self.fRec224[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec224_idx), 1)) & 2047) as usize];
				fRec220[i as usize] = fZec260[i as usize] * self.fRec224[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec224_idx), iZec249[i as usize])) & 2047) as usize] + fZec244[i as usize] * (fZec259[i as usize] * self.fRec224[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec224_idx), iZec242[i as usize])) & 2047) as usize] + 0.5 * fZec258[i as usize] * self.fRec224[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec224_idx), iZec237[i as usize])) & 2047) as usize] + 0.16666667 * fZec257[i as usize] * self.fRec224[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec224_idx), iZec233[i as usize])) & 2047) as usize] + 0.041666668 * fZec230[i as usize] * self.fRec224[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec224_idx), iZec223[i as usize])) & 2047) as usize]);
				fRec221[i as usize] = fYec32_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + fRec214_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fRec214_tmp[(i32::wrapping_add(4, i)) as usize] = fRec219[i as usize];
				fRec215[i as usize] = fRec214_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fRec216[i as usize] = fRec220[i as usize];
				fRec217[i as usize] = fRec221[i as usize];
				fRec210_tmp[(i32::wrapping_add(4, i)) as usize] = fRec215[i as usize];
				fRec211[i as usize] = fZec256[i as usize] + fZec262[i as usize] + fRec210_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fRec212[i as usize] = fRec216[i as usize];
				fRec213[i as usize] = fRec217[i as usize];
				self.fRec206[((i32::wrapping_add(i, self.fRec206_idx)) & 2047) as usize] = fRec211[i as usize];
				fRec207[i as usize] = fZec260[i as usize] * self.fRec206[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec206_idx), iZec250[i as usize])) & 2047) as usize] + fZec244[i as usize] * (fZec259[i as usize] * self.fRec206[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec206_idx), iZec243[i as usize])) & 2047) as usize] + 0.5 * fZec258[i as usize] * self.fRec206[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec206_idx), iZec238[i as usize])) & 2047) as usize] + 0.16666667 * fZec257[i as usize] * self.fRec206[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec206_idx), iZec234[i as usize])) & 2047) as usize] + 0.041666668 * fZec230[i as usize] * self.fRec206[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec206_idx), iZec224[i as usize])) & 2047) as usize]);
				fRec208_tmp[(i32::wrapping_add(4, i)) as usize] = fRec212[i as usize];
				fRec209[i as usize] = fRec213[i as usize];
				fRec204_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow51 * fRec208_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fRec205[i as usize] = fRec209[i as usize];
				fRec199_tmp[(i32::wrapping_add(4, i)) as usize] = fRec203[i as usize];
				fRec200[i as usize] = fSlow51 * fRec199_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fRec201[i as usize] = fRec204_tmp[(i32::wrapping_add(4, i)) as usize];
				fRec202[i as usize] = fRec205[i as usize];
				self.fRec195[((i32::wrapping_add(i, self.fRec195_idx)) & 2047) as usize] = fRec200[i as usize];
				fRec196[i as usize] = fRec207[i as usize];
				fRec197[i as usize] = fRec201[i as usize];
				fRec198[i as usize] = fRec202[i as usize];
				fRec193_tmp[(i32::wrapping_add(4, i)) as usize] = fRec196[i as usize];
				fRec194[i as usize] = fRec198[i as usize];
				fZec263[i as usize] = F32::abs(fRec194[i as usize]);
				fZec264[i as usize] = if (fZec263[i as usize] > fRec233_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) as i32 != 0 {0.0} else {self.fConst16};
				fRec233_tmp[(i32::wrapping_add(4, i)) as usize] = fZec263[i as usize] * (1.0 - fZec264[i as usize]) + fRec233_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] * fZec264[i as usize];
				fRec232[i as usize] = 0.0 - 0.95 * F32::max(2e+01 * F32::log10(F32::max(1.1754944e-38, fRec233_tmp[(i32::wrapping_add(4, i)) as usize])) + 1e+01, 0.0);
				fRec192_tmp[(i32::wrapping_add(4, i)) as usize] = 1.5 * fRec194[i as usize] * F32::powf(1e+01, 0.05 * fRec232[i as usize]);
			}
			/* Post code */
			for j321 in 0..4 {
				self.fRec233_perm[j321 as usize] = fRec233_tmp[(i32::wrapping_add(vsize, j321)) as usize];
			}
			for j305 in 0..4 {
				self.fYec31_perm[j305 as usize] = fYec31_tmp[(i32::wrapping_add(vsize, j305)) as usize];
			}
			for j307 in 0..4 {
				self.fYec32_perm[j307 as usize] = fYec32_tmp[(i32::wrapping_add(vsize, j307)) as usize];
			}
			for j299 in 0..4 {
				self.iYec30_perm[j299 as usize] = iYec30_tmp[(i32::wrapping_add(vsize, j299)) as usize];
			}
			for j301 in 0..4 {
				self.iRec231_perm[j301 as usize] = iRec231_tmp[(i32::wrapping_add(vsize, j301)) as usize];
			}
			for j297 in 0..4 {
				self.fRec230_perm[j297 as usize] = fRec230_tmp[(i32::wrapping_add(vsize, j297)) as usize];
			}
			for j303 in 0..4 {
				self.fRec229_perm[j303 as usize] = fRec229_tmp[(i32::wrapping_add(vsize, j303)) as usize];
			}
			for j295 in 0..4 {
				self.fRec227_perm[j295 as usize] = fRec227_tmp[(i32::wrapping_add(vsize, j295)) as usize];
			}
			self.fRec228_idx_save = vsize;
			self.fRec224_idx_save = vsize;
			for j287 in 0..4 {
				self.fRec223_perm[j287 as usize] = fRec223_tmp[(i32::wrapping_add(vsize, j287)) as usize];
			}
			for j285 in 0..4 {
				self.fRec218_perm[j285 as usize] = fRec218_tmp[(i32::wrapping_add(vsize, j285)) as usize];
			}
			for j309 in 0..4 {
				self.fRec214_perm[j309 as usize] = fRec214_tmp[(i32::wrapping_add(vsize, j309)) as usize];
			}
			for j311 in 0..4 {
				self.fRec210_perm[j311 as usize] = fRec210_tmp[(i32::wrapping_add(vsize, j311)) as usize];
			}
			self.fRec206_idx_save = vsize;
			for j313 in 0..4 {
				self.fRec208_perm[j313 as usize] = fRec208_tmp[(i32::wrapping_add(vsize, j313)) as usize];
			}
			for j315 in 0..4 {
				self.fRec204_perm[j315 as usize] = fRec204_tmp[(i32::wrapping_add(vsize, j315)) as usize];
			}
			for j317 in 0..4 {
				self.fRec199_perm[j317 as usize] = fRec199_tmp[(i32::wrapping_add(vsize, j317)) as usize];
			}
			self.fRec195_idx_save = vsize;
			for j319 in 0..4 {
				self.fRec193_perm[j319 as usize] = fRec193_tmp[(i32::wrapping_add(vsize, j319)) as usize];
			}
			for j323 in 0..4 {
				self.fRec192_perm[j323 as usize] = fRec192_tmp[(i32::wrapping_add(vsize, j323)) as usize];
			}
			/* Vectorizable loop 408 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec218[i as usize] = 1.0 / mydsp_faustpower2_f(fZec215[i as usize]);
			}
			/* Recursive loop 409 */
			/* Pre code */
			for j332 in 0..4 {
				fRec263_tmp[j332 as usize] = self.fRec263_perm[j332 as usize];
			}
			for j334 in 0..4 {
				fRec268_tmp[j334 as usize] = self.fRec268_perm[j334 as usize];
			}
			for j342 in 0..4 {
				fRec272_tmp[j342 as usize] = self.fRec272_perm[j342 as usize];
			}
			self.fRec273_idx = (i32::wrapping_add(self.fRec273_idx, self.fRec273_idx_save)) & 2047;
			for j344 in 0..4 {
				fRec275_tmp[j344 as usize] = self.fRec275_perm[j344 as usize];
			}
			for j346 in 0..4 {
				iYec33_tmp[j346 as usize] = self.iYec33_perm[j346 as usize];
			}
			for j348 in 0..4 {
				iRec276_tmp[j348 as usize] = self.iRec276_perm[j348 as usize];
			}
			for j350 in 0..4 {
				fRec274_tmp[j350 as usize] = self.fRec274_perm[j350 as usize];
			}
			for j352 in 0..4 {
				fYec34_tmp[j352 as usize] = self.fYec34_perm[j352 as usize];
			}
			for j354 in 0..4 {
				fYec35_tmp[j354 as usize] = self.fYec35_perm[j354 as usize];
			}
			self.fRec269_idx = (i32::wrapping_add(self.fRec269_idx, self.fRec269_idx_save)) & 2047;
			for j356 in 0..4 {
				fRec259_tmp[j356 as usize] = self.fRec259_perm[j356 as usize];
			}
			for j358 in 0..4 {
				fRec255_tmp[j358 as usize] = self.fRec255_perm[j358 as usize];
			}
			self.fRec251_idx = (i32::wrapping_add(self.fRec251_idx, self.fRec251_idx_save)) & 2047;
			for j360 in 0..4 {
				fRec253_tmp[j360 as usize] = self.fRec253_perm[j360 as usize];
			}
			for j362 in 0..4 {
				fRec249_tmp[j362 as usize] = self.fRec249_perm[j362 as usize];
			}
			for j364 in 0..4 {
				fRec244_tmp[j364 as usize] = self.fRec244_perm[j364 as usize];
			}
			self.fRec240_idx = (i32::wrapping_add(self.fRec240_idx, self.fRec240_idx_save)) & 2047;
			for j366 in 0..4 {
				fRec238_tmp[j366 as usize] = self.fRec238_perm[j366 as usize];
			}
			for j368 in 0..4 {
				fRec278_tmp[j368 as usize] = self.fRec278_perm[j368 as usize];
			}
			for j370 in 0..4 {
				fRec237_tmp[j370 as usize] = self.fRec237_perm[j370 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec248[i as usize] = -1.0 * 0.9973053 * (0.9 * fRec249_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + 0.05 * (fRec249_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + fRec249_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 3))) as usize]));
				fRec263_tmp[(i32::wrapping_add(4, i)) as usize] = self.fRec240[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec240_idx), iZec300[i as usize])) & 2047) as usize] * fZec298[i as usize] * fZec297[i as usize] * fZec296[i as usize] * fZec295[i as usize] + fZec294[i as usize] * (self.fRec240[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec240_idx), iZec293[i as usize])) & 2047) as usize] * fZec291[i as usize] * fZec290[i as usize] * fZec289[i as usize] + 0.5 * fZec278[i as usize] * self.fRec240[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec240_idx), iZec288[i as usize])) & 2047) as usize] * fZec286[i as usize] * fZec285[i as usize] + 0.16666667 * fZec279[i as usize] * self.fRec240[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec240_idx), iZec284[i as usize])) & 2047) as usize] * fZec282[i as usize] + 0.041666668 * fZec280[i as usize] * self.fRec240[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec240_idx), iZec274[i as usize])) & 2047) as usize]);
				fRec268_tmp[(i32::wrapping_add(4, i)) as usize] = 0.95 * fRec263_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + 0.05 * fRec268_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fRec264[i as usize] = fRec268_tmp[(i32::wrapping_add(4, i)) as usize];
				fRec272_tmp[(i32::wrapping_add(4, i)) as usize] = fRec238_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				self.fRec273[((i32::wrapping_add(i, self.fRec273_idx)) & 2047) as usize] = -1.0 * 0.9973053 * (0.9 * fRec272_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + 0.05 * (fRec272_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + fRec272_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 3))) as usize]));
				fRec275_tmp[(i32::wrapping_add(4, i)) as usize] = self.fConst12 * F32::abs(fRec237_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) + self.fConst11 * fRec275_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				iYec33_tmp[(i32::wrapping_add(4, i)) as usize] = (fRec275_tmp[(i32::wrapping_add(4, i)) as usize] > 0.1) as i32;
				iRec276_tmp[(i32::wrapping_add(4, i)) as usize] = std::cmp::max(i32::wrapping_mul(self.iConst13, (iYec33_tmp[(i32::wrapping_add(4, i)) as usize] < iYec33_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) as i32), i32::wrapping_add(iRec276_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize], -1));
				fZec304[i as usize] = F32::abs(F32::max((iYec33_tmp[(i32::wrapping_add(4, i)) as usize]) as F32, ((iRec276_tmp[(i32::wrapping_add(4, i)) as usize] > 0) as i32) as u32 as F32));
				fZec305[i as usize] = if (fZec304[i as usize] > fRec274_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) as i32 != 0 {self.fConst11} else {self.fConst14};
				fRec274_tmp[(i32::wrapping_add(4, i)) as usize] = fZec304[i as usize] * (1.0 - fZec305[i as usize]) + fRec274_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] * fZec305[i as usize];
				fZec306[i as usize] = 0.005 * fRec274_tmp[(i32::wrapping_add(4, i)) as usize] * fRec237_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fYec34_tmp[(i32::wrapping_add(4, i)) as usize] = fZec310[i as usize] * self.fRec273[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec273_idx), i32::wrapping_add(iZec299[i as usize], 2))) & 2047) as usize] + fZec294[i as usize] * (fZec309[i as usize] * self.fRec273[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec273_idx), i32::wrapping_add(iZec292[i as usize], 2))) & 2047) as usize] + 0.5 * fZec308[i as usize] * self.fRec273[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec273_idx), i32::wrapping_add(iZec287[i as usize], 2))) & 2047) as usize] + 0.16666667 * fZec307[i as usize] * self.fRec273[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec273_idx), i32::wrapping_add(iZec283[i as usize], 2))) & 2047) as usize] + 0.041666668 * fZec280[i as usize] * self.fRec273[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec273_idx), i32::wrapping_add(iZec273[i as usize], 2))) & 2047) as usize]);
				fYec35_tmp[(i32::wrapping_add(4, i)) as usize] = fZec312[i as usize] + fYec34_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + fZec306[i as usize];
				self.fRec269[((i32::wrapping_add(i, self.fRec269_idx)) & 2047) as usize] = 0.95 * fYec35_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + 0.05 * self.fRec269[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec269_idx), 1)) & 2047) as usize];
				fRec265[i as usize] = fZec310[i as usize] * self.fRec269[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec269_idx), iZec299[i as usize])) & 2047) as usize] + fZec294[i as usize] * (fZec309[i as usize] * self.fRec269[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec269_idx), iZec292[i as usize])) & 2047) as usize] + 0.5 * fZec308[i as usize] * self.fRec269[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec269_idx), iZec287[i as usize])) & 2047) as usize] + 0.16666667 * fZec307[i as usize] * self.fRec269[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec269_idx), iZec283[i as usize])) & 2047) as usize] + 0.041666668 * fZec280[i as usize] * self.fRec269[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec269_idx), iZec273[i as usize])) & 2047) as usize]);
				fRec266[i as usize] = fYec35_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + fRec259_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fRec259_tmp[(i32::wrapping_add(4, i)) as usize] = fRec264[i as usize];
				fRec260[i as usize] = fRec259_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fRec261[i as usize] = fRec265[i as usize];
				fRec262[i as usize] = fRec266[i as usize];
				fRec255_tmp[(i32::wrapping_add(4, i)) as usize] = fRec260[i as usize];
				fRec256[i as usize] = fZec306[i as usize] + fZec312[i as usize] + fRec255_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fRec257[i as usize] = fRec261[i as usize];
				fRec258[i as usize] = fRec262[i as usize];
				self.fRec251[((i32::wrapping_add(i, self.fRec251_idx)) & 2047) as usize] = fRec256[i as usize];
				fRec252[i as usize] = fZec310[i as usize] * self.fRec251[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec251_idx), iZec300[i as usize])) & 2047) as usize] + fZec294[i as usize] * (fZec309[i as usize] * self.fRec251[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec251_idx), iZec293[i as usize])) & 2047) as usize] + 0.5 * fZec308[i as usize] * self.fRec251[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec251_idx), iZec288[i as usize])) & 2047) as usize] + 0.16666667 * fZec307[i as usize] * self.fRec251[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec251_idx), iZec284[i as usize])) & 2047) as usize] + 0.041666668 * fZec280[i as usize] * self.fRec251[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec251_idx), iZec274[i as usize])) & 2047) as usize]);
				fRec253_tmp[(i32::wrapping_add(4, i)) as usize] = fRec257[i as usize];
				fRec254[i as usize] = fRec258[i as usize];
				fRec249_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow51 * fRec253_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fRec250[i as usize] = fRec254[i as usize];
				fRec244_tmp[(i32::wrapping_add(4, i)) as usize] = fRec248[i as usize];
				fRec245[i as usize] = fSlow51 * fRec244_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fRec246[i as usize] = fRec249_tmp[(i32::wrapping_add(4, i)) as usize];
				fRec247[i as usize] = fRec250[i as usize];
				self.fRec240[((i32::wrapping_add(i, self.fRec240_idx)) & 2047) as usize] = fRec245[i as usize];
				fRec241[i as usize] = fRec252[i as usize];
				fRec242[i as usize] = fRec246[i as usize];
				fRec243[i as usize] = fRec247[i as usize];
				fRec238_tmp[(i32::wrapping_add(4, i)) as usize] = fRec241[i as usize];
				fRec239[i as usize] = fRec243[i as usize];
				fZec313[i as usize] = F32::abs(fRec239[i as usize]);
				fZec314[i as usize] = if (fZec313[i as usize] > fRec278_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) as i32 != 0 {0.0} else {self.fConst16};
				fRec278_tmp[(i32::wrapping_add(4, i)) as usize] = fZec313[i as usize] * (1.0 - fZec314[i as usize]) + fRec278_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] * fZec314[i as usize];
				fRec277[i as usize] = 0.0 - 0.95 * F32::max(2e+01 * F32::log10(F32::max(1.1754944e-38, fRec278_tmp[(i32::wrapping_add(4, i)) as usize])) + 1e+01, 0.0);
				fRec237_tmp[(i32::wrapping_add(4, i)) as usize] = 1.5 * fRec239[i as usize] * F32::powf(1e+01, 0.05 * fRec277[i as usize]);
			}
			/* Post code */
			for j369 in 0..4 {
				self.fRec278_perm[j369 as usize] = fRec278_tmp[(i32::wrapping_add(vsize, j369)) as usize];
			}
			for j353 in 0..4 {
				self.fYec34_perm[j353 as usize] = fYec34_tmp[(i32::wrapping_add(vsize, j353)) as usize];
			}
			for j355 in 0..4 {
				self.fYec35_perm[j355 as usize] = fYec35_tmp[(i32::wrapping_add(vsize, j355)) as usize];
			}
			for j347 in 0..4 {
				self.iYec33_perm[j347 as usize] = iYec33_tmp[(i32::wrapping_add(vsize, j347)) as usize];
			}
			for j349 in 0..4 {
				self.iRec276_perm[j349 as usize] = iRec276_tmp[(i32::wrapping_add(vsize, j349)) as usize];
			}
			for j345 in 0..4 {
				self.fRec275_perm[j345 as usize] = fRec275_tmp[(i32::wrapping_add(vsize, j345)) as usize];
			}
			for j351 in 0..4 {
				self.fRec274_perm[j351 as usize] = fRec274_tmp[(i32::wrapping_add(vsize, j351)) as usize];
			}
			for j343 in 0..4 {
				self.fRec272_perm[j343 as usize] = fRec272_tmp[(i32::wrapping_add(vsize, j343)) as usize];
			}
			self.fRec273_idx_save = vsize;
			self.fRec269_idx_save = vsize;
			for j335 in 0..4 {
				self.fRec268_perm[j335 as usize] = fRec268_tmp[(i32::wrapping_add(vsize, j335)) as usize];
			}
			for j333 in 0..4 {
				self.fRec263_perm[j333 as usize] = fRec263_tmp[(i32::wrapping_add(vsize, j333)) as usize];
			}
			for j357 in 0..4 {
				self.fRec259_perm[j357 as usize] = fRec259_tmp[(i32::wrapping_add(vsize, j357)) as usize];
			}
			for j359 in 0..4 {
				self.fRec255_perm[j359 as usize] = fRec255_tmp[(i32::wrapping_add(vsize, j359)) as usize];
			}
			self.fRec251_idx_save = vsize;
			for j361 in 0..4 {
				self.fRec253_perm[j361 as usize] = fRec253_tmp[(i32::wrapping_add(vsize, j361)) as usize];
			}
			for j363 in 0..4 {
				self.fRec249_perm[j363 as usize] = fRec249_tmp[(i32::wrapping_add(vsize, j363)) as usize];
			}
			for j365 in 0..4 {
				self.fRec244_perm[j365 as usize] = fRec244_tmp[(i32::wrapping_add(vsize, j365)) as usize];
			}
			self.fRec240_idx_save = vsize;
			for j367 in 0..4 {
				self.fRec238_perm[j367 as usize] = fRec238_tmp[(i32::wrapping_add(vsize, j367)) as usize];
			}
			for j371 in 0..4 {
				self.fRec237_perm[j371 as usize] = fRec237_tmp[(i32::wrapping_add(vsize, j371)) as usize];
			}
			/* Vectorizable loop 410 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec268[i as usize] = 1.0 / mydsp_faustpower2_f(fZec265[i as usize]);
			}
			/* Vectorizable loop 411 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec267[i as usize] = fZec85[i as usize] + (fZec82[i as usize] + fZec266[i as usize]) / fZec265[i as usize] + 1.0;
			}
			/* Recursive loop 412 */
			/* Pre code */
			for j18 in 0..4 {
				fRec23_tmp[j18 as usize] = self.fRec23_perm[j18 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec23_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow13 + self.fConst2 * fRec23_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
			}
			/* Post code */
			for j19 in 0..4 {
				self.fRec23_perm[j19 as usize] = fRec23_tmp[(i32::wrapping_add(vsize, j19)) as usize];
			}
			/* Recursive loop 413 */
			/* Pre code */
			for j28 in 0..4 {
				fRec27_tmp[j28 as usize] = self.fRec27_perm[j28 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec27_tmp[(i32::wrapping_add(4, i)) as usize] = 0.999 * fRec27_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] - self.fConst6 * ((fZec4[i as usize] - fZec5[i as usize]) * self.fYec1[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec1_idx), i32::wrapping_add(iZec7[i as usize], 1))) & 4095) as usize] - (self.fYec1[((i32::wrapping_add(i, self.fYec1_idx)) & 4095) as usize] - self.fYec1[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec1_idx), iZec7[i as usize])) & 4095) as usize] * (fZec5[i as usize] + (1.0 - fZec4[i as usize]))));
			}
			/* Post code */
			for j29 in 0..4 {
				self.fRec27_perm[j29 as usize] = fRec27_tmp[(i32::wrapping_add(vsize, j29)) as usize];
			}
			/* Recursive loop 414 */
			/* Pre code */
			for j20 in 0..4 {
				fRec24_tmp[j20 as usize] = self.fRec24_perm[j20 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec24_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow14 + self.fConst2 * fRec24_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
			}
			/* Post code */
			for j21 in 0..4 {
				self.fRec24_perm[j21 as usize] = fRec24_tmp[(i32::wrapping_add(vsize, j21)) as usize];
			}
			/* Vectorizable loop 415 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec118[i as usize] = fZec84[i as usize] + (fZec89[i as usize] + fZec115[i as usize]) / fZec114[i as usize] + fZec88[i as usize];
			}
			/* Recursive loop 416 */
			/* Pre code */
			for j230 in 0..4 {
				fRec100_tmp[j230 as usize] = self.fRec100_perm[j230 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec100_tmp[(i32::wrapping_add(4, i)) as usize] = fRec101_tmp[(i32::wrapping_add(4, i)) as usize] - (fRec100_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] * (fZec85[i as usize] + (fZec166[i as usize] - fZec82[i as usize]) / fZec165[i as usize] + 1.0) + 2.0 * fRec100_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] * (fZec85[i as usize] + (1.0 - fZec168[i as usize]))) / fZec167[i as usize];
			}
			/* Post code */
			for j231 in 0..4 {
				self.fRec100_perm[j231 as usize] = fRec100_tmp[(i32::wrapping_add(vsize, j231)) as usize];
			}
			/* Vectorizable loop 417 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec319[i as usize] = fZec84[i as usize] + (fZec89[i as usize] + fZec316[i as usize]) / fZec315[i as usize] + fZec88[i as usize];
			}
			/* Recursive loop 418 */
			/* Pre code */
			for j422 in 0..4 {
				fRec281_tmp[j422 as usize] = self.fRec281_perm[j422 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec281_tmp[(i32::wrapping_add(4, i)) as usize] = fRec282_tmp[(i32::wrapping_add(4, i)) as usize] - (fRec281_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] * (fZec85[i as usize] + (fZec366[i as usize] - fZec82[i as usize]) / fZec365[i as usize] + 1.0) + 2.0 * fRec281_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] * (fZec85[i as usize] + (1.0 - fZec368[i as usize]))) / fZec367[i as usize];
			}
			/* Post code */
			for j423 in 0..4 {
				self.fRec281_perm[j423 as usize] = fRec281_tmp[(i32::wrapping_add(vsize, j423)) as usize];
			}
			/* Vectorizable loop 419 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec369[i as usize] = fZec84[i as usize] + (fZec89[i as usize] + fZec366[i as usize]) / fZec365[i as usize] + fZec88[i as usize];
			}
			/* Recursive loop 420 */
			/* Pre code */
			for j62 in 0..4 {
				fRec40_tmp[j62 as usize] = self.fRec40_perm[j62 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec40_tmp[(i32::wrapping_add(4, i)) as usize] = 0.999 * fRec40_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] - self.fConst6 * ((fZec29[i as usize] - fZec30[i as usize]) * self.fYec9[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec9_idx), i32::wrapping_add(iZec31[i as usize], 1))) & 4095) as usize] - (self.fYec9[((i32::wrapping_add(i, self.fYec9_idx)) & 4095) as usize] - self.fYec9[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec9_idx), iZec31[i as usize])) & 4095) as usize] * (fZec30[i as usize] + (1.0 - fZec29[i as usize]))));
			}
			/* Post code */
			for j63 in 0..4 {
				self.fRec40_perm[j63 as usize] = fRec40_tmp[(i32::wrapping_add(vsize, j63)) as usize];
			}
			/* Recursive loop 421 */
			/* Pre code */
			for j46 in 0..4 {
				fRec35_tmp[j46 as usize] = self.fRec35_perm[j46 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec35_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow20 + self.fConst2 * fRec35_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
			}
			/* Post code */
			for j47 in 0..4 {
				self.fRec35_perm[j47 as usize] = fRec35_tmp[(i32::wrapping_add(vsize, j47)) as usize];
			}
			/* Recursive loop 422 */
			/* Pre code */
			for j44 in 0..4 {
				fRec33_tmp[j44 as usize] = self.fRec33_perm[j44 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec33_tmp[(i32::wrapping_add(4, i)) as usize] = 0.999 * fRec33_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + self.fConst6 * (self.fYec5[((i32::wrapping_add(i, self.fYec5_idx)) & 4095) as usize] - self.fYec5[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec5_idx), iZec18[i as usize])) & 4095) as usize] * (fZec19[i as usize] + (1.0 - fZec17[i as usize])) - (fZec17[i as usize] - fZec19[i as usize]) * self.fYec5[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec5_idx), i32::wrapping_add(iZec18[i as usize], 1))) & 4095) as usize]);
			}
			/* Post code */
			for j45 in 0..4 {
				self.fRec33_perm[j45 as usize] = fRec33_tmp[(i32::wrapping_add(vsize, j45)) as usize];
			}
			/* Recursive loop 423 */
			/* Pre code */
			for j54 in 0..4 {
				fRec37_tmp[j54 as usize] = self.fRec37_perm[j54 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec37_tmp[(i32::wrapping_add(4, i)) as usize] = 0.999 * fRec37_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] - self.fConst6 * (self.fYec7[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec7_idx), iZec24[i as usize])) & 4095) as usize] * (fZec25[i as usize] + (1.0 - fZec23[i as usize])) - self.fYec7[((i32::wrapping_add(i, self.fYec7_idx)) & 4095) as usize] + (fZec23[i as usize] - fZec25[i as usize]) * self.fYec7[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec7_idx), i32::wrapping_add(iZec24[i as usize], 1))) & 4095) as usize]);
			}
			/* Post code */
			for j55 in 0..4 {
				self.fRec37_perm[j55 as usize] = fRec37_tmp[(i32::wrapping_add(vsize, j55)) as usize];
			}
			/* Recursive loop 424 */
			/* Pre code */
			for j36 in 0..4 {
				fRec30_tmp[j36 as usize] = self.fRec30_perm[j36 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec30_tmp[(i32::wrapping_add(4, i)) as usize] = 0.999 * fRec30_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] - self.fConst6 * (self.fYec3[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec3_idx), iZec12[i as usize])) & 4095) as usize] * (fZec13[i as usize] + (1.0 - fZec11[i as usize])) - self.fYec3[((i32::wrapping_add(i, self.fYec3_idx)) & 4095) as usize] + (fZec11[i as usize] - fZec13[i as usize]) * self.fYec3[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec3_idx), i32::wrapping_add(iZec12[i as usize], 1))) & 4095) as usize]);
			}
			/* Post code */
			for j37 in 0..4 {
				self.fRec30_perm[j37 as usize] = fRec30_tmp[(i32::wrapping_add(vsize, j37)) as usize];
			}
			/* Recursive loop 425 */
			/* Pre code */
			for j106 in 0..4 {
				fRec57_tmp[j106 as usize] = self.fRec57_perm[j106 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec57_tmp[(i32::wrapping_add(4, i)) as usize] = 0.999 * fRec57_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] - self.fConst6 * (self.fYec19[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec19_idx), iZec60[i as usize])) & 4095) as usize] * (fZec61[i as usize] + (1.0 - fZec59[i as usize])) - self.fYec19[((i32::wrapping_add(i, self.fYec19_idx)) & 4095) as usize] + (fZec59[i as usize] - fZec61[i as usize]) * self.fYec19[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec19_idx), i32::wrapping_add(iZec60[i as usize], 1))) & 4095) as usize]);
			}
			/* Post code */
			for j107 in 0..4 {
				self.fRec57_perm[j107 as usize] = fRec57_tmp[(i32::wrapping_add(vsize, j107)) as usize];
			}
			/* Recursive loop 426 */
			/* Pre code */
			for j122 in 0..4 {
				fRec63_tmp[j122 as usize] = self.fRec63_perm[j122 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec63_tmp[(i32::wrapping_add(4, i)) as usize] = 0.999 * fRec63_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] - self.fConst6 * (self.fYec23[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec23_idx), iZec72[i as usize])) & 4095) as usize] * (fZec73[i as usize] + (1.0 - fZec71[i as usize])) - self.fYec23[((i32::wrapping_add(i, self.fYec23_idx)) & 4095) as usize] + (fZec71[i as usize] - fZec73[i as usize]) * self.fYec23[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec23_idx), i32::wrapping_add(iZec72[i as usize], 1))) & 4095) as usize]);
			}
			/* Post code */
			for j123 in 0..4 {
				self.fRec63_perm[j123 as usize] = fRec63_tmp[(i32::wrapping_add(vsize, j123)) as usize];
			}
			/* Recursive loop 427 */
			/* Pre code */
			for j98 in 0..4 {
				fRec55_tmp[j98 as usize] = self.fRec55_perm[j98 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec55_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow30 + self.fConst2 * fRec55_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
			}
			/* Post code */
			for j99 in 0..4 {
				self.fRec55_perm[j99 as usize] = fRec55_tmp[(i32::wrapping_add(vsize, j99)) as usize];
			}
			/* Recursive loop 428 */
			/* Pre code */
			for j114 in 0..4 {
				fRec60_tmp[j114 as usize] = self.fRec60_perm[j114 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec60_tmp[(i32::wrapping_add(4, i)) as usize] = 0.999 * fRec60_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] - self.fConst6 * ((fZec65[i as usize] - fZec66[i as usize]) * self.fYec21[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec21_idx), i32::wrapping_add(iZec67[i as usize], 1))) & 4095) as usize] - (self.fYec21[((i32::wrapping_add(i, self.fYec21_idx)) & 4095) as usize] - self.fYec21[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec21_idx), iZec67[i as usize])) & 4095) as usize] * (fZec66[i as usize] + (1.0 - fZec65[i as usize]))));
			}
			/* Post code */
			for j115 in 0..4 {
				self.fRec60_perm[j115 as usize] = fRec60_tmp[(i32::wrapping_add(vsize, j115)) as usize];
			}
			/* Recursive loop 429 */
			/* Pre code */
			for j96 in 0..4 {
				fRec53_tmp[j96 as usize] = self.fRec53_perm[j96 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec53_tmp[(i32::wrapping_add(4, i)) as usize] = 0.999 * fRec53_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + self.fConst6 * (self.fYec17[((i32::wrapping_add(i, self.fYec17_idx)) & 4095) as usize] - self.fYec17[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec17_idx), iZec54[i as usize])) & 4095) as usize] * (fZec55[i as usize] + (1.0 - fZec53[i as usize])) - (fZec53[i as usize] - fZec55[i as usize]) * self.fYec17[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec17_idx), i32::wrapping_add(iZec54[i as usize], 1))) & 4095) as usize]);
			}
			/* Post code */
			for j97 in 0..4 {
				self.fRec53_perm[j97 as usize] = fRec53_tmp[(i32::wrapping_add(vsize, j97)) as usize];
			}
			/* Vectorizable loop 430 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec90[i as usize] = fZec84[i as usize] + (fZec89[i as usize] + fZec83[i as usize]) / fZec79[i as usize] + fZec88[i as usize];
			}
			/* Recursive loop 431 */
			/* Pre code */
			for j154 in 0..4 {
				fRec78_tmp[j154 as usize] = self.fRec78_perm[j154 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec78_tmp[(i32::wrapping_add(4, i)) as usize] = fRec79_tmp[(i32::wrapping_add(4, i)) as usize] * (2.0 * fRec81[i as usize] + -1.0) - (fRec78_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] * (fZec85[i as usize] + (1.0 - (fZec82[i as usize] - fZec97[i as usize]) / fZec96[i as usize])) + 2.0 * fRec78_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] * (fZec85[i as usize] + (1.0 - fZec99[i as usize]))) / fZec98[i as usize];
			}
			/* Post code */
			for j155 in 0..4 {
				self.fRec78_perm[j155 as usize] = fRec78_tmp[(i32::wrapping_add(vsize, j155)) as usize];
			}
			/* Recursive loop 432 */
			/* Pre code */
			for j142 in 0..4 {
				fRec68_tmp[j142 as usize] = self.fRec68_perm[j142 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec68_tmp[(i32::wrapping_add(4, i)) as usize] = fRec69_tmp[(i32::wrapping_add(4, i)) as usize] * (2.0 * fRec71[i as usize] + -1.0) - (fRec68_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] * (fZec85[i as usize] + (fZec83[i as usize] - fZec82[i as usize]) / fZec79[i as usize] + 1.0) + 2.0 * fRec68_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] * (fZec85[i as usize] + (1.0 - fZec87[i as usize]))) / fZec86[i as usize];
			}
			/* Post code */
			for j143 in 0..4 {
				self.fRec68_perm[j143 as usize] = fRec68_tmp[(i32::wrapping_add(vsize, j143)) as usize];
			}
			/* Vectorizable loop 433 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec109[i as usize] = fZec84[i as usize] + (fZec89[i as usize] + fZec106[i as usize]) / fZec105[i as usize] + fZec88[i as usize];
			}
			/* Vectorizable loop 434 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec100[i as usize] = fZec84[i as usize] + (fZec89[i as usize] + fZec97[i as usize]) / fZec96[i as usize] + fZec88[i as usize];
			}
			/* Recursive loop 435 */
			/* Pre code */
			for j166 in 0..4 {
				fRec85_tmp[j166 as usize] = self.fRec85_perm[j166 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec85_tmp[(i32::wrapping_add(4, i)) as usize] = fRec86_tmp[(i32::wrapping_add(4, i)) as usize] * (2.0 * fRec88[i as usize] + -1.0) - (fRec85_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] * (fZec85[i as usize] + (fZec106[i as usize] - fZec82[i as usize]) / fZec105[i as usize] + 1.0) + 2.0 * fRec85_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] * (fZec85[i as usize] + (1.0 - fZec108[i as usize]))) / fZec107[i as usize];
			}
			/* Post code */
			for j167 in 0..4 {
				self.fRec85_perm[j167 as usize] = fRec85_tmp[(i32::wrapping_add(vsize, j167)) as usize];
			}
			/* Recursive loop 436 */
			/* Pre code */
			for j178 in 0..4 {
				fRec92_tmp[j178 as usize] = self.fRec92_perm[j178 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec92_tmp[(i32::wrapping_add(4, i)) as usize] = fRec93_tmp[(i32::wrapping_add(4, i)) as usize] * (2.0 * fRec95[i as usize] + -1.0) - (fRec92_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] * (fZec85[i as usize] + (fZec115[i as usize] - fZec82[i as usize]) / fZec114[i as usize] + 1.0) + 2.0 * fRec92_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] * (fZec85[i as usize] + (1.0 - fZec117[i as usize]))) / fZec116[i as usize];
			}
			/* Post code */
			for j179 in 0..4 {
				self.fRec92_perm[j179 as usize] = fRec92_tmp[(i32::wrapping_add(vsize, j179)) as usize];
			}
			/* Recursive loop 437 */
			/* Pre code */
			for j70 in 0..4 {
				fRec43_tmp[j70 as usize] = self.fRec43_perm[j70 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec43_tmp[(i32::wrapping_add(4, i)) as usize] = 0.999 * fRec43_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] - self.fConst6 * (self.fYec11[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec11_idx), iZec36[i as usize])) & 4095) as usize] * (fZec37[i as usize] + (1.0 - fZec35[i as usize])) - self.fYec11[((i32::wrapping_add(i, self.fYec11_idx)) & 4095) as usize] + (fZec35[i as usize] - fZec37[i as usize]) * self.fYec11[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec11_idx), i32::wrapping_add(iZec36[i as usize], 1))) & 4095) as usize]);
			}
			/* Post code */
			for j71 in 0..4 {
				self.fRec43_perm[j71 as usize] = fRec43_tmp[(i32::wrapping_add(vsize, j71)) as usize];
			}
			/* Recursive loop 438 */
			/* Pre code */
			for j80 in 0..4 {
				fRec47_tmp[j80 as usize] = self.fRec47_perm[j80 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec47_tmp[(i32::wrapping_add(4, i)) as usize] = 0.999 * fRec47_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + self.fConst6 * (self.fYec13[((i32::wrapping_add(i, self.fYec13_idx)) & 4095) as usize] - self.fYec13[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec13_idx), iZec42[i as usize])) & 4095) as usize] * (fZec43[i as usize] + (1.0 - fZec41[i as usize])) - (fZec41[i as usize] - fZec43[i as usize]) * self.fYec13[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec13_idx), i32::wrapping_add(iZec42[i as usize], 1))) & 4095) as usize]);
			}
			/* Post code */
			for j81 in 0..4 {
				self.fRec47_perm[j81 as usize] = fRec47_tmp[(i32::wrapping_add(vsize, j81)) as usize];
			}
			/* Recursive loop 439 */
			/* Pre code */
			for j72 in 0..4 {
				fRec45_tmp[j72 as usize] = self.fRec45_perm[j72 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec45_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow25 + self.fConst2 * fRec45_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
			}
			/* Post code */
			for j73 in 0..4 {
				self.fRec45_perm[j73 as usize] = fRec45_tmp[(i32::wrapping_add(vsize, j73)) as usize];
			}
			/* Recursive loop 440 */
			/* Pre code */
			for j88 in 0..4 {
				fRec50_tmp[j88 as usize] = self.fRec50_perm[j88 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec50_tmp[(i32::wrapping_add(4, i)) as usize] = 0.999 * fRec50_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] - self.fConst6 * ((fZec47[i as usize] - fZec48[i as usize]) * self.fYec15[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec15_idx), i32::wrapping_add(iZec49[i as usize], 1))) & 4095) as usize] - (self.fYec15[((i32::wrapping_add(i, self.fYec15_idx)) & 4095) as usize] - self.fYec15[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec15_idx), iZec49[i as usize])) & 4095) as usize] * (fZec48[i as usize] + (1.0 - fZec47[i as usize]))));
			}
			/* Post code */
			for j89 in 0..4 {
				self.fRec50_perm[j89 as usize] = fRec50_tmp[(i32::wrapping_add(vsize, j89)) as usize];
			}
			/* Vectorizable loop 441 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec169[i as usize] = fZec84[i as usize] + (fZec89[i as usize] + fZec166[i as usize]) / fZec165[i as usize] + fZec88[i as usize];
			}
			/* Recursive loop 442 */
			/* Pre code */
			for j278 in 0..4 {
				fRec146_tmp[j278 as usize] = self.fRec146_perm[j278 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec146_tmp[(i32::wrapping_add(4, i)) as usize] = fRec147_tmp[(i32::wrapping_add(4, i)) as usize] - (fRec146_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] * (fZec85[i as usize] + (fZec216[i as usize] - fZec82[i as usize]) / fZec215[i as usize] + 1.0) + 2.0 * fRec146_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] * (fZec85[i as usize] + (1.0 - fZec218[i as usize]))) / fZec217[i as usize];
			}
			/* Post code */
			for j279 in 0..4 {
				self.fRec146_perm[j279 as usize] = fRec146_tmp[(i32::wrapping_add(vsize, j279)) as usize];
			}
			/* Recursive loop 443 */
			/* Pre code */
			for j326 in 0..4 {
				fRec191_tmp[j326 as usize] = self.fRec191_perm[j326 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec191_tmp[(i32::wrapping_add(4, i)) as usize] = fRec192_tmp[(i32::wrapping_add(4, i)) as usize] - (fRec191_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] * (fZec85[i as usize] + (1.0 - (fZec82[i as usize] - fZec266[i as usize]) / fZec265[i as usize])) + 2.0 * fRec191_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] * (fZec85[i as usize] + (1.0 - fZec268[i as usize]))) / fZec267[i as usize];
			}
			/* Post code */
			for j327 in 0..4 {
				self.fRec191_perm[j327 as usize] = fRec191_tmp[(i32::wrapping_add(vsize, j327)) as usize];
			}
			/* Vectorizable loop 444 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec219[i as usize] = fZec84[i as usize] + (fZec89[i as usize] + fZec216[i as usize]) / fZec215[i as usize] + fZec88[i as usize];
			}
			/* Vectorizable loop 445 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec269[i as usize] = fZec84[i as usize] + (fZec89[i as usize] + fZec266[i as usize]) / fZec265[i as usize] + fZec88[i as usize];
			}
			/* Recursive loop 446 */
			/* Pre code */
			for j374 in 0..4 {
				fRec236_tmp[j374 as usize] = self.fRec236_perm[j374 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec236_tmp[(i32::wrapping_add(4, i)) as usize] = fRec237_tmp[(i32::wrapping_add(4, i)) as usize] - (fRec236_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] * (fZec85[i as usize] + (1.0 - (fZec82[i as usize] - fZec316[i as usize]) / fZec315[i as usize])) + 2.0 * fRec236_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] * (fZec85[i as usize] + (1.0 - fZec318[i as usize]))) / fZec317[i as usize];
			}
			/* Post code */
			for j375 in 0..4 {
				self.fRec236_perm[j375 as usize] = fRec236_tmp[(i32::wrapping_add(vsize, j375)) as usize];
			}
			/* Vectorizable loop 447 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec430[i as usize] = fSlow92 * (fRec20_tmp[(i32::wrapping_add(4, i)) as usize] + 1.0);
			}
			/* Vectorizable loop 448 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec532[i as usize] = fSlow92 * (fRec19_tmp[(i32::wrapping_add(4, i)) as usize] + 1.0);
			}
			/* Recursive loop 449 */
			/* Pre code */
			for j16 in 0..4 {
				fRec22_tmp[j16 as usize] = self.fRec22_perm[j16 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec22_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow12 + self.fConst2 * fRec22_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
			}
			/* Post code */
			for j17 in 0..4 {
				self.fRec22_perm[j17 as usize] = fRec22_tmp[(i32::wrapping_add(vsize, j17)) as usize];
			}
			/* Recursive loop 450 */
			/* Pre code */
			for j232 in 0..4 {
				fRec99_tmp[j232 as usize] = self.fRec99_perm[j232 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec99_tmp[(i32::wrapping_add(4, i)) as usize] = (fRec100_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + fRec100_tmp[(i32::wrapping_add(4, i)) as usize] + 2.0 * fRec100_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) / fZec167[i as usize] - (fRec99_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] * (fZec84[i as usize] + (fZec166[i as usize] - fZec89[i as usize]) / fZec165[i as usize] + fZec88[i as usize]) + 2.0 * fRec99_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] * (fZec84[i as usize] + (1.0 - (fZec81[i as usize] + fZec168[i as usize])))) / fZec169[i as usize];
			}
			/* Post code */
			for j233 in 0..4 {
				self.fRec99_perm[j233 as usize] = fRec99_tmp[(i32::wrapping_add(vsize, j233)) as usize];
			}
			/* Recursive loop 451 */
			/* Pre code */
			for j182 in 0..4 {
				fRec98_tmp[j182 as usize] = self.fRec98_perm[j182 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec98_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow48 + self.fConst2 * fRec98_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
			}
			/* Post code */
			for j183 in 0..4 {
				self.fRec98_perm[j183 as usize] = fRec98_tmp[(i32::wrapping_add(vsize, j183)) as usize];
			}
			/* Recursive loop 452 */
			/* Pre code */
			for j424 in 0..4 {
				fRec280_tmp[j424 as usize] = self.fRec280_perm[j424 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec280_tmp[(i32::wrapping_add(4, i)) as usize] = (fRec281_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + fRec281_tmp[(i32::wrapping_add(4, i)) as usize] + 2.0 * fRec281_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) / fZec367[i as usize] - (fRec280_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] * (fZec84[i as usize] + (fZec366[i as usize] - fZec89[i as usize]) / fZec365[i as usize] + fZec88[i as usize]) + 2.0 * fRec280_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] * (fZec84[i as usize] + (1.0 - (fZec81[i as usize] + fZec368[i as usize])))) / fZec369[i as usize];
			}
			/* Post code */
			for j425 in 0..4 {
				self.fRec280_perm[j425 as usize] = fRec280_tmp[(i32::wrapping_add(vsize, j425)) as usize];
			}
			/* Vectorizable loop 453 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec370[i as usize] = F32::max(-1.0, F32::min(1.0, fRec23_tmp[(i32::wrapping_add(4, i)) as usize] + self.fConst17 * (fRec24_tmp[(i32::wrapping_add(4, i)) as usize] * (fRec25_tmp[(i32::wrapping_add(4, i)) as usize] * fRec27_tmp[(i32::wrapping_add(4, i)) as usize] + fRec29_tmp[(i32::wrapping_add(4, i)) as usize] * fRec30_tmp[(i32::wrapping_add(4, i)) as usize] + fRec32_tmp[(i32::wrapping_add(4, i)) as usize] * fRec33_tmp[(i32::wrapping_add(4, i)) as usize]) + fRec35_tmp[(i32::wrapping_add(4, i)) as usize] * (fRec36_tmp[(i32::wrapping_add(4, i)) as usize] * fRec37_tmp[(i32::wrapping_add(4, i)) as usize] + fRec39_tmp[(i32::wrapping_add(4, i)) as usize] * fRec40_tmp[(i32::wrapping_add(4, i)) as usize] + fRec42_tmp[(i32::wrapping_add(4, i)) as usize] * fRec43_tmp[(i32::wrapping_add(4, i)) as usize]) + fRec45_tmp[(i32::wrapping_add(4, i)) as usize] * (fRec46_tmp[(i32::wrapping_add(4, i)) as usize] * fRec47_tmp[(i32::wrapping_add(4, i)) as usize] + fRec49_tmp[(i32::wrapping_add(4, i)) as usize] * fRec50_tmp[(i32::wrapping_add(4, i)) as usize] + fRec52_tmp[(i32::wrapping_add(4, i)) as usize] * fRec53_tmp[(i32::wrapping_add(4, i)) as usize]) + fRec55_tmp[(i32::wrapping_add(4, i)) as usize] * (fRec56_tmp[(i32::wrapping_add(4, i)) as usize] * fRec57_tmp[(i32::wrapping_add(4, i)) as usize] + fRec59_tmp[(i32::wrapping_add(4, i)) as usize] * fRec60_tmp[(i32::wrapping_add(4, i)) as usize] + fRec62_tmp[(i32::wrapping_add(4, i)) as usize] * fRec63_tmp[(i32::wrapping_add(4, i)) as usize])) * F32::powf(1e+01, 0.6666667 * fRec23_tmp[(i32::wrapping_add(4, i)) as usize])));
			}
			/* Recursive loop 454 */
			/* Pre code */
			for j124 in 0..4 {
				fRec65_tmp[j124 as usize] = self.fRec65_perm[j124 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec65_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow35 + self.fConst2 * fRec65_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
			}
			/* Post code */
			for j125 in 0..4 {
				self.fRec65_perm[j125 as usize] = fRec65_tmp[(i32::wrapping_add(vsize, j125)) as usize];
			}
			/* Recursive loop 455 */
			/* Pre code */
			for j156 in 0..4 {
				fRec77_tmp[j156 as usize] = self.fRec77_perm[j156 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec77_tmp[(i32::wrapping_add(4, i)) as usize] = (fRec78_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + fRec78_tmp[(i32::wrapping_add(4, i)) as usize] + 2.0 * fRec78_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) / fZec98[i as usize] - (fRec77_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] * (fZec84[i as usize] + (fZec97[i as usize] - fZec89[i as usize]) / fZec96[i as usize] + fZec88[i as usize]) + 2.0 * fRec77_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] * (fZec84[i as usize] + (1.0 - (fZec81[i as usize] + fZec99[i as usize])))) / fZec100[i as usize];
			}
			/* Post code */
			for j157 in 0..4 {
				self.fRec77_perm[j157 as usize] = fRec77_tmp[(i32::wrapping_add(vsize, j157)) as usize];
			}
			/* Recursive loop 456 */
			/* Pre code */
			for j144 in 0..4 {
				fRec67_tmp[j144 as usize] = self.fRec67_perm[j144 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec67_tmp[(i32::wrapping_add(4, i)) as usize] = (fRec68_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + fRec68_tmp[(i32::wrapping_add(4, i)) as usize] + 2.0 * fRec68_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) / fZec86[i as usize] - (fRec67_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] * (fZec84[i as usize] + (fZec83[i as usize] - fZec89[i as usize]) / fZec79[i as usize] + fZec88[i as usize]) + 2.0 * fRec67_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] * (fZec84[i as usize] + (1.0 - (fZec81[i as usize] + fZec87[i as usize])))) / fZec90[i as usize];
			}
			/* Post code */
			for j145 in 0..4 {
				self.fRec67_perm[j145 as usize] = fRec67_tmp[(i32::wrapping_add(vsize, j145)) as usize];
			}
			/* Recursive loop 457 */
			/* Pre code */
			for j126 in 0..4 {
				fRec66_tmp[j126 as usize] = self.fRec66_perm[j126 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec66_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow36 + self.fConst2 * fRec66_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
			}
			/* Post code */
			for j127 in 0..4 {
				self.fRec66_perm[j127 as usize] = fRec66_tmp[(i32::wrapping_add(vsize, j127)) as usize];
			}
			/* Recursive loop 458 */
			/* Pre code */
			for j168 in 0..4 {
				fRec84_tmp[j168 as usize] = self.fRec84_perm[j168 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec84_tmp[(i32::wrapping_add(4, i)) as usize] = (fRec85_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + fRec85_tmp[(i32::wrapping_add(4, i)) as usize] + 2.0 * fRec85_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) / fZec107[i as usize] - (fRec84_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] * (fZec84[i as usize] + (fZec106[i as usize] - fZec89[i as usize]) / fZec105[i as usize] + fZec88[i as usize]) + 2.0 * fRec84_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] * (fZec84[i as usize] + (1.0 - (fZec81[i as usize] + fZec108[i as usize])))) / fZec109[i as usize];
			}
			/* Post code */
			for j169 in 0..4 {
				self.fRec84_perm[j169 as usize] = fRec84_tmp[(i32::wrapping_add(vsize, j169)) as usize];
			}
			/* Recursive loop 459 */
			/* Pre code */
			for j180 in 0..4 {
				fRec91_tmp[j180 as usize] = self.fRec91_perm[j180 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec91_tmp[(i32::wrapping_add(4, i)) as usize] = (fRec92_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + fRec92_tmp[(i32::wrapping_add(4, i)) as usize] + 2.0 * fRec92_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) / fZec116[i as usize] - (fRec91_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] * (fZec84[i as usize] + (fZec115[i as usize] - fZec89[i as usize]) / fZec114[i as usize] + fZec88[i as usize]) + 2.0 * fRec91_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] * (fZec84[i as usize] + (1.0 - (fZec81[i as usize] + fZec117[i as usize])))) / fZec118[i as usize];
			}
			/* Post code */
			for j181 in 0..4 {
				self.fRec91_perm[j181 as usize] = fRec91_tmp[(i32::wrapping_add(vsize, j181)) as usize];
			}
			/* Recursive loop 460 */
			/* Pre code */
			for j280 in 0..4 {
				fRec145_tmp[j280 as usize] = self.fRec145_perm[j280 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec145_tmp[(i32::wrapping_add(4, i)) as usize] = (fRec146_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + fRec146_tmp[(i32::wrapping_add(4, i)) as usize] + 2.0 * fRec146_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) / fZec217[i as usize] - (fRec145_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] * (fZec84[i as usize] + (fZec216[i as usize] - fZec89[i as usize]) / fZec215[i as usize] + fZec88[i as usize]) + 2.0 * fRec145_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] * (fZec84[i as usize] + (1.0 - (fZec81[i as usize] + fZec218[i as usize])))) / fZec219[i as usize];
			}
			/* Post code */
			for j281 in 0..4 {
				self.fRec145_perm[j281 as usize] = fRec145_tmp[(i32::wrapping_add(vsize, j281)) as usize];
			}
			/* Recursive loop 461 */
			/* Pre code */
			for j328 in 0..4 {
				fRec190_tmp[j328 as usize] = self.fRec190_perm[j328 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec190_tmp[(i32::wrapping_add(4, i)) as usize] = (fRec191_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + fRec191_tmp[(i32::wrapping_add(4, i)) as usize] + 2.0 * fRec191_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) / fZec267[i as usize] - (fRec190_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] * (fZec84[i as usize] + (fZec266[i as usize] - fZec89[i as usize]) / fZec265[i as usize] + fZec88[i as usize]) + 2.0 * fRec190_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] * (fZec84[i as usize] + (1.0 - (fZec81[i as usize] + fZec268[i as usize])))) / fZec269[i as usize];
			}
			/* Post code */
			for j329 in 0..4 {
				self.fRec190_perm[j329 as usize] = fRec190_tmp[(i32::wrapping_add(vsize, j329)) as usize];
			}
			/* Recursive loop 462 */
			/* Pre code */
			for j376 in 0..4 {
				fRec235_tmp[j376 as usize] = self.fRec235_perm[j376 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec235_tmp[(i32::wrapping_add(4, i)) as usize] = (fRec236_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + fRec236_tmp[(i32::wrapping_add(4, i)) as usize] + 2.0 * fRec236_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) / fZec317[i as usize] - (fRec235_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] * (fZec84[i as usize] + (fZec316[i as usize] - fZec89[i as usize]) / fZec315[i as usize] + fZec88[i as usize]) + 2.0 * fRec235_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] * (fZec84[i as usize] + (1.0 - (fZec81[i as usize] + fZec318[i as usize])))) / fZec319[i as usize];
			}
			/* Post code */
			for j377 in 0..4 {
				self.fRec235_perm[j377 as usize] = fRec235_tmp[(i32::wrapping_add(vsize, j377)) as usize];
			}
			/* Vectorizable loop 463 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec431[i as usize] = fZec430[i as usize] + 3.500005;
			}
			/* Vectorizable loop 464 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec458[i as usize] = fSlow95 * fRec20_tmp[(i32::wrapping_add(4, i)) as usize];
			}
			/* Vectorizable loop 465 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec563[i as usize] = fSlow95 * fRec19_tmp[(i32::wrapping_add(4, i)) as usize];
			}
			/* Vectorizable loop 466 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec533[i as usize] = fZec532[i as usize] + 3.500005;
			}
			/* Recursive loop 467 */
			/* Pre code */
			for j6 in 0..4 {
				fRec13_tmp[j6 as usize] = self.fRec13_perm[j6 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec13_tmp[(i32::wrapping_add(4, i)) as usize] = 0.9999 * (fRec13_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + (i32::wrapping_mul(iZec0[i as usize], iSlow4)) as F32) + fSlow5;
			}
			/* Post code */
			for j7 in 0..4 {
				self.fRec13_perm[j7 as usize] = fRec13_tmp[(i32::wrapping_add(vsize, j7)) as usize];
			}
			/* Recursive loop 468 */
			/* Pre code */
			for j8 in 0..4 {
				fRec17_tmp[j8 as usize] = self.fRec17_perm[j8 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec17_tmp[(i32::wrapping_add(4, i)) as usize] = 0.9999 * (fRec17_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + (i32::wrapping_mul(iZec0[i as usize], iSlow6)) as F32) + fSlow7;
			}
			/* Post code */
			for j9 in 0..4 {
				self.fRec17_perm[j9 as usize] = fRec17_tmp[(i32::wrapping_add(vsize, j9)) as usize];
			}
			/* Recursive loop 469 */
			/* Pre code */
			for j4 in 0..4 {
				fRec9_tmp[j4 as usize] = self.fRec9_perm[j4 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec9_tmp[(i32::wrapping_add(4, i)) as usize] = 0.9999 * (fRec9_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + (i32::wrapping_mul(iZec0[i as usize], iSlow2)) as F32) + fSlow3;
			}
			/* Post code */
			for j5 in 0..4 {
				self.fRec9_perm[j5 as usize] = fRec9_tmp[(i32::wrapping_add(vsize, j5)) as usize];
			}
			/* Recursive loop 470 */
			/* Pre code */
			for j430 in 0..4 {
				fRec342_tmp[j430 as usize] = self.fRec342_perm[j430 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec342_tmp[(i32::wrapping_add(4, i)) as usize] = 0.9999 * (fRec342_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + (i32::wrapping_mul(iZec0[i as usize], iSlow64)) as F32) + fSlow65;
			}
			/* Post code */
			for j431 in 0..4 {
				self.fRec342_perm[j431 as usize] = fRec342_tmp[(i32::wrapping_add(vsize, j431)) as usize];
			}
			/* Recursive loop 471 */
			/* Pre code */
			for j432 in 0..4 {
				fRec346_tmp[j432 as usize] = self.fRec346_perm[j432 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec346_tmp[(i32::wrapping_add(4, i)) as usize] = 0.9999 * (fRec346_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + (i32::wrapping_mul(iZec0[i as usize], iSlow66)) as F32) + fSlow67;
			}
			/* Post code */
			for j433 in 0..4 {
				self.fRec346_perm[j433 as usize] = fRec346_tmp[(i32::wrapping_add(vsize, j433)) as usize];
			}
			/* Recursive loop 472 */
			/* Pre code */
			for j426 in 0..4 {
				fRec326_tmp[j426 as usize] = self.fRec326_perm[j426 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec326_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow60 + self.fConst2 * fRec326_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
			}
			/* Post code */
			for j427 in 0..4 {
				self.fRec326_perm[j427 as usize] = fRec326_tmp[(i32::wrapping_add(vsize, j427)) as usize];
			}
			/* Vectorizable loop 473 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec372[i as usize] = fRec65_tmp[(i32::wrapping_add(4, i)) as usize] * fRec66_tmp[(i32::wrapping_add(4, i)) as usize] * ((fRec67_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + fRec67_tmp[(i32::wrapping_add(4, i)) as usize] + 2.0 * fRec67_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) / fZec90[i as usize] + (fRec77_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + fRec77_tmp[(i32::wrapping_add(4, i)) as usize] + 2.0 * fRec77_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) / fZec100[i as usize] + (fRec84_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + fRec84_tmp[(i32::wrapping_add(4, i)) as usize] + 2.0 * fRec84_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) / fZec109[i as usize] + (fRec91_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + fRec91_tmp[(i32::wrapping_add(4, i)) as usize] + 2.0 * fRec91_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) / fZec118[i as usize]);
			}
			/* Vectorizable loop 474 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec373[i as usize] = fRec98_tmp[(i32::wrapping_add(4, i)) as usize] * ((fRec99_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + fRec99_tmp[(i32::wrapping_add(4, i)) as usize] + 2.0 * fRec99_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) / fZec169[i as usize] + (fRec145_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + fRec145_tmp[(i32::wrapping_add(4, i)) as usize] + 2.0 * fRec145_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) / fZec219[i as usize] + (fRec190_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + fRec190_tmp[(i32::wrapping_add(4, i)) as usize] + 2.0 * fRec190_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) / fZec269[i as usize] + (fRec235_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + fRec235_tmp[(i32::wrapping_add(4, i)) as usize] + 2.0 * fRec235_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) / fZec319[i as usize] + (fRec280_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + fRec280_tmp[(i32::wrapping_add(4, i)) as usize] + 2.0 * fRec280_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) / fZec369[i as usize]);
			}
			/* Vectorizable loop 475 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec371[i as usize] = fRec22_tmp[(i32::wrapping_add(4, i)) as usize] * fZec370[i as usize] * (1.0 - 0.33333334 * mydsp_faustpower2_f(fZec370[i as usize]));
			}
			/* Recursive loop 476 */
			/* Pre code */
			for j428 in 0..4 {
				fRec335_tmp[j428 as usize] = self.fRec335_perm[j428 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec335_tmp[(i32::wrapping_add(4, i)) as usize] = 0.9999 * (fRec335_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + (i32::wrapping_mul(iZec0[i as usize], iSlow62)) as F32) + fSlow63;
			}
			/* Post code */
			for j429 in 0..4 {
				self.fRec335_perm[j429 as usize] = fRec335_tmp[(i32::wrapping_add(vsize, j429)) as usize];
			}
			/* Recursive loop 477 */
			/* Pre code */
			for j436 in 0..4 {
				fRec358_tmp[j436 as usize] = self.fRec358_perm[j436 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec358_tmp[(i32::wrapping_add(4, i)) as usize] = 0.9999 * (fRec358_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + (i32::wrapping_mul(iZec0[i as usize], iSlow70)) as F32) + fSlow71;
			}
			/* Post code */
			for j437 in 0..4 {
				self.fRec358_perm[j437 as usize] = fRec358_tmp[(i32::wrapping_add(vsize, j437)) as usize];
			}
			/* Recursive loop 478 */
			/* Pre code */
			for j438 in 0..4 {
				fRec362_tmp[j438 as usize] = self.fRec362_perm[j438 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec362_tmp[(i32::wrapping_add(4, i)) as usize] = 0.9999 * (fRec362_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + (i32::wrapping_mul(iZec0[i as usize], iSlow72)) as F32) + fSlow73;
			}
			/* Post code */
			for j439 in 0..4 {
				self.fRec362_perm[j439 as usize] = fRec362_tmp[(i32::wrapping_add(vsize, j439)) as usize];
			}
			/* Recursive loop 479 */
			/* Pre code */
			for j434 in 0..4 {
				fRec350_tmp[j434 as usize] = self.fRec350_perm[j434 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec350_tmp[(i32::wrapping_add(4, i)) as usize] = 0.9999 * (fRec350_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + (i32::wrapping_mul(iZec0[i as usize], iSlow68)) as F32) + fSlow69;
			}
			/* Post code */
			for j435 in 0..4 {
				self.fRec350_perm[j435 as usize] = fRec350_tmp[(i32::wrapping_add(vsize, j435)) as usize];
			}
			/* Recursive loop 480 */
			/* Pre code */
			for j464 in 0..4 {
				fRec374_tmp[j464 as usize] = self.fRec374_perm[j464 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec374_tmp[(i32::wrapping_add(4, i)) as usize] = 0.9999 * (fRec374_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + (i32::wrapping_mul(iZec0[i as usize], iSlow80)) as F32) + fSlow81;
			}
			/* Post code */
			for j465 in 0..4 {
				self.fRec374_perm[j465 as usize] = fRec374_tmp[(i32::wrapping_add(vsize, j465)) as usize];
			}
			/* Recursive loop 481 */
			/* Pre code */
			for j478 in 0..4 {
				fRec376_tmp[j478 as usize] = self.fRec376_perm[j478 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec376_tmp[(i32::wrapping_add(4, i)) as usize] = 0.9999 * (fRec376_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + (i32::wrapping_mul(iZec0[i as usize], iSlow82)) as F32) + fSlow83;
			}
			/* Post code */
			for j479 in 0..4 {
				self.fRec376_perm[j479 as usize] = fRec376_tmp[(i32::wrapping_add(vsize, j479)) as usize];
			}
			/* Recursive loop 482 */
			/* Pre code */
			for j448 in 0..4 {
				fRec371_tmp[j448 as usize] = self.fRec371_perm[j448 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec371_tmp[(i32::wrapping_add(4, i)) as usize] = 0.9999 * (fRec371_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + (i32::wrapping_mul(iZec0[i as usize], iSlow76)) as F32) + fSlow77;
			}
			/* Post code */
			for j449 in 0..4 {
				self.fRec371_perm[j449 as usize] = fRec371_tmp[(i32::wrapping_add(vsize, j449)) as usize];
			}
			/* Recursive loop 483 */
			/* Pre code */
			for j440 in 0..4 {
				fRec369_tmp[j440 as usize] = self.fRec369_perm[j440 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec369_tmp[(i32::wrapping_add(4, i)) as usize] = 0.9999 * (fRec369_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + (i32::wrapping_mul(iZec0[i as usize], iSlow74)) as F32) + fSlow75;
			}
			/* Post code */
			for j441 in 0..4 {
				self.fRec369_perm[j441 as usize] = fRec369_tmp[(i32::wrapping_add(vsize, j441)) as usize];
			}
			/* Recursive loop 484 */
			/* Pre code */
			for j492 in 0..4 {
				fRec378_tmp[j492 as usize] = self.fRec378_perm[j492 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec378_tmp[(i32::wrapping_add(4, i)) as usize] = 0.9999 * (fRec378_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + (i32::wrapping_mul(iZec0[i as usize], iSlow84)) as F32) + fSlow85;
			}
			/* Post code */
			for j493 in 0..4 {
				self.fRec378_perm[j493 as usize] = fRec378_tmp[(i32::wrapping_add(vsize, j493)) as usize];
			}
			/* Vectorizable loop 485 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec434[i as usize] = F32::floor(fZec431[i as usize]);
			}
			/* Recursive loop 486 */
			/* Pre code */
			for j508 in 0..4 {
				fRec381_tmp[j508 as usize] = self.fRec381_perm[j508 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec381_tmp[(i32::wrapping_add(4, i)) as usize] = 0.9999 * (fRec381_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + (i32::wrapping_mul(iZec0[i as usize], iSlow88)) as F32) + fSlow89;
			}
			/* Post code */
			for j509 in 0..4 {
				self.fRec381_perm[j509 as usize] = fRec381_tmp[(i32::wrapping_add(vsize, j509)) as usize];
			}
			/* Recursive loop 487 */
			/* Pre code */
			for j534 in 0..4 {
				fRec386_tmp[j534 as usize] = self.fRec386_perm[j534 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec386_tmp[(i32::wrapping_add(4, i)) as usize] = 0.9999 * (fRec386_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + (i32::wrapping_mul(iZec0[i as usize], iSlow96)) as F32) + fSlow97;
			}
			/* Post code */
			for j535 in 0..4 {
				self.fRec386_perm[j535 as usize] = fRec386_tmp[(i32::wrapping_add(vsize, j535)) as usize];
			}
			/* Vectorizable loop 488 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec459[i as usize] = fSlow92 + fZec458[i as usize] + 3.500005;
			}
			/* Vectorizable loop 489 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec564[i as usize] = fSlow92 + fZec563[i as usize] + 3.500005;
			}
			/* Recursive loop 490 */
			/* Pre code */
			for j672 in 0..4 {
				fRec420_tmp[j672 as usize] = self.fRec420_perm[j672 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec420_tmp[(i32::wrapping_add(4, i)) as usize] = 0.9999 * (fRec420_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + (i32::wrapping_mul(iZec0[i as usize], iSlow112)) as F32) + fSlow113;
			}
			/* Post code */
			for j673 in 0..4 {
				self.fRec420_perm[j673 as usize] = fRec420_tmp[(i32::wrapping_add(vsize, j673)) as usize];
			}
			/* Recursive loop 491 */
			/* Pre code */
			for j560 in 0..4 {
				fRec389_tmp[j560 as usize] = self.fRec389_perm[j560 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec389_tmp[(i32::wrapping_add(4, i)) as usize] = 0.9999 * (fRec389_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + (i32::wrapping_mul(iZec0[i as usize], iSlow98)) as F32) + fSlow99;
			}
			/* Post code */
			for j561 in 0..4 {
				self.fRec389_perm[j561 as usize] = fRec389_tmp[(i32::wrapping_add(vsize, j561)) as usize];
			}
			/* Recursive loop 492 */
			/* Pre code */
			for j586 in 0..4 {
				fRec392_tmp[j586 as usize] = self.fRec392_perm[j586 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec392_tmp[(i32::wrapping_add(4, i)) as usize] = 0.9999 * (fRec392_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + (i32::wrapping_mul(iZec0[i as usize], iSlow100)) as F32) + fSlow101;
			}
			/* Post code */
			for j587 in 0..4 {
				self.fRec392_perm[j587 as usize] = fRec392_tmp[(i32::wrapping_add(vsize, j587)) as usize];
			}
			/* Vectorizable loop 493 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec536[i as usize] = F32::floor(fZec533[i as usize]);
			}
			/* Recursive loop 494 */
			/* Pre code */
			for j698 in 0..4 {
				fRec423_tmp[j698 as usize] = self.fRec423_perm[j698 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec423_tmp[(i32::wrapping_add(4, i)) as usize] = 0.9999 * (fRec423_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + (i32::wrapping_mul(iZec0[i as usize], iSlow114)) as F32) + fSlow115;
			}
			/* Post code */
			for j699 in 0..4 {
				self.fRec423_perm[j699 as usize] = fRec423_tmp[(i32::wrapping_add(vsize, j699)) as usize];
			}
			/* Recursive loop 495 */
			/* Pre code */
			for j14 in 0..4 {
				fRec21_tmp[j14 as usize] = self.fRec21_perm[j14 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec21_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow11 + self.fConst2 * fRec21_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
			}
			/* Post code */
			for j15 in 0..4 {
				self.fRec21_perm[j15 as usize] = fRec21_tmp[(i32::wrapping_add(vsize, j15)) as usize];
			}
			/* Recursive loop 496 */
			/* Pre code */
			self.fRec325_idx = (i32::wrapping_add(self.fRec325_idx, self.fRec325_idx_save)) & 2097151;
			/* Compute code */
			for i in 0..output0.len() as i32 {
				self.fRec325[((i32::wrapping_add(i, self.fRec325_idx)) & 2097151) as usize] = fZec373[i as usize] + fZec372[i as usize] + fSlow61 * self.fRec325[((i32::wrapping_sub(i32::wrapping_add(i, self.fRec325_idx), i32::wrapping_add((F32::min(self.fConst18, F32::max(0.0, self.fConst0 * fRec326_tmp[(i32::wrapping_add(4, i)) as usize]))) as i32, 1))) & 2097151) as usize] + fZec371[i as usize];
			}
			/* Post code */
			self.fRec325_idx_save = vsize;
			/* Vectorizable loop 497 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec380[i as usize] = fRec371_tmp[(i32::wrapping_add(4, i)) as usize] + -1.49999;
			}
			/* Vectorizable loop 498 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec390[i as usize] = fRec374_tmp[(i32::wrapping_add(4, i)) as usize] + -1.49999;
			}
			/* Vectorizable loop 499 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec402[i as usize] = fRec376_tmp[(i32::wrapping_add(4, i)) as usize] + -1.49999;
			}
			/* Vectorizable loop 500 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec374[i as usize] = fRec369_tmp[(i32::wrapping_add(4, i)) as usize] + -1.49999;
			}
			/* Vectorizable loop 501 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec406[i as usize] = fRec358_tmp[(i32::wrapping_add(4, i)) as usize] + -1.49999;
			}
			/* Vectorizable loop 502 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec394[i as usize] = fRec362_tmp[(i32::wrapping_add(4, i)) as usize] + -1.49999;
			}
			/* Recursive loop 503 */
			/* Pre code */
			for j456 in 0..4 {
				fRec372_tmp[j456 as usize] = self.fRec372_perm[j456 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec372_tmp[(i32::wrapping_add(4, i)) as usize] = 0.9999 * (fRec372_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + (i32::wrapping_mul(iZec0[i as usize], iSlow78)) as F32) + fSlow79;
			}
			/* Post code */
			for j457 in 0..4 {
				self.fRec372_perm[j457 as usize] = fRec372_tmp[(i32::wrapping_add(vsize, j457)) as usize];
			}
			/* Vectorizable loop 504 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec414[i as usize] = fRec378_tmp[(i32::wrapping_add(4, i)) as usize] + -1.49999;
			}
			/* Recursive loop 505 */
			/* Pre code */
			for j500 in 0..4 {
				fRec379_tmp[j500 as usize] = self.fRec379_perm[j500 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec379_tmp[(i32::wrapping_add(4, i)) as usize] = 0.9999 * (fRec379_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + (i32::wrapping_mul(iZec0[i as usize], iSlow86)) as F32) + fSlow87;
			}
			/* Post code */
			for j501 in 0..4 {
				self.fRec379_perm[j501 as usize] = fRec379_tmp[(i32::wrapping_add(vsize, j501)) as usize];
			}
			/* Vectorizable loop 506 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec437[i as usize] = fZec430[i as usize] + (4.0 - fZec434[i as usize]);
			}
			/* Recursive loop 507 */
			/* Pre code */
			for j516 in 0..4 {
				fRec382_tmp[j516 as usize] = self.fRec382_perm[j516 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec382_tmp[(i32::wrapping_add(4, i)) as usize] = 0.995 * (fRec382_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + (i32::wrapping_mul(iZec0[i as usize], iSlow90)) as F32) + fSlow91;
			}
			/* Post code */
			for j517 in 0..4 {
				self.fRec382_perm[j517 as usize] = fRec382_tmp[(i32::wrapping_add(vsize, j517)) as usize];
			}
			/* Vectorizable loop 508 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec436[i as usize] = fZec430[i as usize] + (3.0 - fZec434[i as usize]);
			}
			/* Vectorizable loop 509 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec424[i as usize] = fRec381_tmp[(i32::wrapping_add(4, i)) as usize] + -1.49999;
			}
			/* Recursive loop 510 */
			/* Pre code */
			for j522 in 0..4 {
				fRec384_tmp[j522 as usize] = self.fRec384_perm[j522 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec384_tmp[(i32::wrapping_add(4, i)) as usize] = 0.995 * (fRec384_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + (i32::wrapping_mul(iZec0[i as usize], iSlow93)) as F32) + fSlow94;
			}
			/* Post code */
			for j523 in 0..4 {
				self.fRec384_perm[j523 as usize] = fRec384_tmp[(i32::wrapping_add(vsize, j523)) as usize];
			}
			/* Vectorizable loop 511 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec461[i as usize] = F32::floor(fZec459[i as usize]);
			}
			/* Vectorizable loop 512 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec477[i as usize] = fRec346_tmp[(i32::wrapping_add(4, i)) as usize] + -1.49999;
			}
			/* Vectorizable loop 513 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec473[i as usize] = fRec386_tmp[(i32::wrapping_add(4, i)) as usize] + -1.49999;
			}
			/* Vectorizable loop 514 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec467[i as usize] = fRec350_tmp[(i32::wrapping_add(4, i)) as usize] + -1.49999;
			}
			/* Recursive loop 515 */
			/* Pre code */
			for j630 in 0..4 {
				fRec409_tmp[j630 as usize] = self.fRec409_perm[j630 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec409_tmp[(i32::wrapping_add(4, i)) as usize] = 0.995 * (fRec409_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + (i32::wrapping_mul(iZec0[i as usize], iSlow107)) as F32) + fSlow108;
			}
			/* Post code */
			for j631 in 0..4 {
				self.fRec409_perm[j631 as usize] = fRec409_tmp[(i32::wrapping_add(vsize, j631)) as usize];
			}
			/* Vectorizable loop 516 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec566[i as usize] = F32::floor(fZec564[i as usize]);
			}
			/* Vectorizable loop 517 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec485[i as usize] = fRec17_tmp[(i32::wrapping_add(4, i)) as usize] + -1.49999;
			}
			/* Vectorizable loop 518 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec503[i as usize] = fRec13_tmp[(i32::wrapping_add(4, i)) as usize] + -1.49999;
			}
			/* Vectorizable loop 519 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec512[i as usize] = fRec9_tmp[(i32::wrapping_add(4, i)) as usize] + -1.49999;
			}
			/* Vectorizable loop 520 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec490[i as usize] = fRec342_tmp[(i32::wrapping_add(4, i)) as usize] + -1.49999;
			}
			/* Vectorizable loop 521 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec498[i as usize] = fRec389_tmp[(i32::wrapping_add(4, i)) as usize] + -1.49999;
			}
			/* Vectorizable loop 522 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec517[i as usize] = fRec335_tmp[(i32::wrapping_add(4, i)) as usize] + -1.49999;
			}
			/* Vectorizable loop 523 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec538[i as usize] = fZec532[i as usize] + (3.0 - fZec536[i as usize]);
			}
			/* Vectorizable loop 524 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec539[i as usize] = fZec532[i as usize] + (4.0 - fZec536[i as usize]);
			}
			/* Recursive loop 525 */
			/* Pre code */
			for j594 in 0..4 {
				fRec393_tmp[j594 as usize] = self.fRec393_perm[j594 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec393_tmp[(i32::wrapping_add(4, i)) as usize] = 0.995 * (fRec393_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + (i32::wrapping_mul(iZec0[i as usize], iSlow102)) as F32) + fSlow103;
			}
			/* Post code */
			for j595 in 0..4 {
				self.fRec393_perm[j595 as usize] = fRec393_tmp[(i32::wrapping_add(vsize, j595)) as usize];
			}
			/* Vectorizable loop 526 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec525[i as usize] = fRec392_tmp[(i32::wrapping_add(4, i)) as usize] + -1.49999;
			}
			/* Vectorizable loop 527 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec576[i as usize] = fRec420_tmp[(i32::wrapping_add(4, i)) as usize] + -1.49999;
			}
			/* Vectorizable loop 528 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec588[i as usize] = fRec423_tmp[(i32::wrapping_add(4, i)) as usize] + -1.49999;
			}
			/* Recursive loop 529 */
			/* Pre code */
			for j706 in 0..4 {
				fRec424_tmp[j706 as usize] = self.fRec424_perm[j706 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec424_tmp[(i32::wrapping_add(4, i)) as usize] = 0.9999 * (fRec424_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + (i32::wrapping_mul(iZec0[i as usize], iSlow116)) as F32) + fSlow117;
			}
			/* Post code */
			for j707 in 0..4 {
				self.fRec424_perm[j707 as usize] = fRec424_tmp[(i32::wrapping_add(vsize, j707)) as usize];
			}
			/* Vectorizable loop 530 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec395[i as usize] = F32::floor(fZec394[i as usize]);
			}
			/* Vectorizable loop 531 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec391[i as usize] = F32::floor(fZec390[i as usize]);
			}
			/* Vectorizable loop 532 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec403[i as usize] = F32::floor(fZec402[i as usize]);
			}
			/* Vectorizable loop 533 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec381[i as usize] = F32::floor(fZec380[i as usize]);
			}
			/* Vectorizable loop 534 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec407[i as usize] = F32::floor(fZec406[i as usize]);
			}
			/* Vectorizable loop 535 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec375[i as usize] = F32::floor(fZec374[i as usize]);
			}
			/* Vectorizable loop 536 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec384[i as usize] = fRec372_tmp[(i32::wrapping_add(4, i)) as usize] + -1.49999;
			}
			/* Vectorizable loop 537 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec418[i as usize] = fRec379_tmp[(i32::wrapping_add(4, i)) as usize] + -1.49999;
			}
			/* Vectorizable loop 538 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec435[i as usize] = fZec430[i as usize] + (2.0 - fZec434[i as usize]);
			}
			/* Vectorizable loop 539 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec440[i as usize] = fZec430[i as usize] + (1.0 - fZec434[i as usize]);
			}
			/* Vectorizable loop 540 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec432[i as usize] = (fZec431[i as usize]) as i32;
			}
			/* Vectorizable loop 541 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec425[i as usize] = F32::floor(fZec424[i as usize]);
			}
			/* Vectorizable loop 542 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec438[i as usize] = fZec437[i as usize] * fZec436[i as usize];
			}
			/* Vectorizable loop 543 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec415[i as usize] = F32::floor(fZec414[i as usize]);
			}
			/* Vectorizable loop 544 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec456[i as usize] = fRec384_tmp[(i32::wrapping_add(4, i)) as usize] + -1.49999;
			}
			/* Vectorizable loop 545 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec428[i as usize] = fRec382_tmp[(i32::wrapping_add(4, i)) as usize] + -1.49999;
			}
			/* Vectorizable loop 546 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec464[i as usize] = fSlow92 + fZec458[i as usize] + (4.0 - fZec461[i as usize]);
			}
			/* Vectorizable loop 547 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec468[i as usize] = F32::floor(fZec467[i as usize]);
			}
			/* Vectorizable loop 548 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec478[i as usize] = F32::floor(fZec477[i as usize]);
			}
			/* Vectorizable loop 549 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec463[i as usize] = fSlow92 + fZec458[i as usize] + (3.0 - fZec461[i as usize]);
			}
			/* Vectorizable loop 550 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec474[i as usize] = F32::floor(fZec473[i as usize]);
			}
			/* Vectorizable loop 551 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec560[i as usize] = (1.0 - fRec21_tmp[(i32::wrapping_add(4, i)) as usize]) * (fZec371[i as usize] + fZec372[i as usize] + fZec373[i as usize]);
			}
			/* Vectorizable loop 552 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec568[i as usize] = fSlow92 + fZec563[i as usize] + (3.0 - fZec566[i as usize]);
			}
			/* Vectorizable loop 553 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec569[i as usize] = fSlow92 + fZec563[i as usize] + (4.0 - fZec566[i as usize]);
			}
			/* Vectorizable loop 554 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec561[i as usize] = fRec409_tmp[(i32::wrapping_add(4, i)) as usize] + -1.49999;
			}
			/* Vectorizable loop 555 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec559[i as usize] = self.fRec325[((i32::wrapping_add(i, self.fRec325_idx)) & 2097151) as usize] * fRec21_tmp[(i32::wrapping_add(4, i)) as usize];
			}
			/* Vectorizable loop 556 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec499[i as usize] = F32::floor(fZec498[i as usize]);
			}
			/* Vectorizable loop 557 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec491[i as usize] = F32::floor(fZec490[i as usize]);
			}
			/* Vectorizable loop 558 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec504[i as usize] = F32::floor(fZec503[i as usize]);
			}
			/* Vectorizable loop 559 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec513[i as usize] = F32::floor(fZec512[i as usize]);
			}
			/* Vectorizable loop 560 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec518[i as usize] = F32::floor(fZec517[i as usize]);
			}
			/* Vectorizable loop 561 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec486[i as usize] = F32::floor(fZec485[i as usize]);
			}
			/* Vectorizable loop 562 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec537[i as usize] = fZec532[i as usize] + (2.0 - fZec536[i as usize]);
			}
			/* Vectorizable loop 563 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec526[i as usize] = F32::floor(fZec525[i as usize]);
			}
			/* Vectorizable loop 564 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec530[i as usize] = fRec393_tmp[(i32::wrapping_add(4, i)) as usize] + -1.49999;
			}
			/* Vectorizable loop 565 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec542[i as usize] = fZec532[i as usize] + (1.0 - fZec536[i as usize]);
			}
			/* Vectorizable loop 566 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec534[i as usize] = (fZec533[i as usize]) as i32;
			}
			/* Vectorizable loop 567 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec540[i as usize] = fZec539[i as usize] * fZec538[i as usize];
			}
			/* Vectorizable loop 568 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec577[i as usize] = F32::floor(fZec576[i as usize]);
			}
			/* Vectorizable loop 569 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec589[i as usize] = F32::floor(fZec588[i as usize]);
			}
			/* Vectorizable loop 570 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec592[i as usize] = fRec424_tmp[(i32::wrapping_add(4, i)) as usize] + -1.49999;
			}
			/* Vectorizable loop 571 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec396[i as usize] = fRec362_tmp[(i32::wrapping_add(4, i)) as usize] - fZec395[i as usize];
			}
			/* Vectorizable loop 572 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec392[i as usize] = fRec374_tmp[(i32::wrapping_add(4, i)) as usize] - fZec391[i as usize];
			}
			/* Vectorizable loop 573 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec385[i as usize] = F32::floor(fZec384[i as usize]);
			}
			/* Vectorizable loop 574 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec397[i as usize] = fZec395[i as usize] + (2.0 - fRec362_tmp[(i32::wrapping_add(4, i)) as usize]);
			}
			/* Vectorizable loop 575 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec408[i as usize] = fRec358_tmp[(i32::wrapping_add(4, i)) as usize] - fZec407[i as usize];
			}
			/* Vectorizable loop 576 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec383[i as usize] = fZec381[i as usize] + (2.0 - fRec371_tmp[(i32::wrapping_add(4, i)) as usize]);
			}
			/* Vectorizable loop 577 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec376[i as usize] = fRec369_tmp[(i32::wrapping_add(4, i)) as usize] - fZec375[i as usize];
			}
			/* Vectorizable loop 578 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec405[i as usize] = fZec403[i as usize] + (2.0 - fRec376_tmp[(i32::wrapping_add(4, i)) as usize]);
			}
			/* Vectorizable loop 579 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec393[i as usize] = fZec391[i as usize] + (2.0 - fRec374_tmp[(i32::wrapping_add(4, i)) as usize]);
			}
			/* Vectorizable loop 580 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec404[i as usize] = fRec376_tmp[(i32::wrapping_add(4, i)) as usize] - fZec403[i as usize];
			}
			/* Vectorizable loop 581 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec382[i as usize] = fRec371_tmp[(i32::wrapping_add(4, i)) as usize] - fZec381[i as usize];
			}
			/* Vectorizable loop 582 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec377[i as usize] = fZec375[i as usize] + (2.0 - fRec369_tmp[(i32::wrapping_add(4, i)) as usize]);
			}
			/* Vectorizable loop 583 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec409[i as usize] = fZec407[i as usize] + (2.0 - fRec358_tmp[(i32::wrapping_add(4, i)) as usize]);
			}
			/* Vectorizable loop 584 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec433[i as usize] = std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iZec432[i as usize], 4)));
			}
			/* Vectorizable loop 585 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec443[i as usize] = 0.0 - 0.5 * fZec440[i as usize];
			}
			/* Vectorizable loop 586 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec417[i as usize] = fZec415[i as usize] + (2.0 - fRec378_tmp[(i32::wrapping_add(4, i)) as usize]);
			}
			/* Vectorizable loop 587 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec441[i as usize] = 0.0 - fZec440[i as usize];
			}
			/* Vectorizable loop 588 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec439[i as usize] = fZec438[i as usize] * fZec435[i as usize];
			}
			/* Vectorizable loop 589 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec444[i as usize] = 0.0 - fZec435[i as usize];
			}
			/* Vectorizable loop 590 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec445[i as usize] = std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iZec432[i as usize], 2)));
			}
			/* Vectorizable loop 591 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec429[i as usize] = F32::floor(fZec428[i as usize]);
			}
			/* Vectorizable loop 592 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec427[i as usize] = fZec425[i as usize] + (2.0 - fRec381_tmp[(i32::wrapping_add(4, i)) as usize]);
			}
			/* Vectorizable loop 593 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec442[i as usize] = std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iZec432[i as usize], 3)));
			}
			/* Vectorizable loop 594 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec447[i as usize] = 0.0 - 0.5 * fZec435[i as usize];
			}
			/* Vectorizable loop 595 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec449[i as usize] = std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iZec432[i as usize], 1)));
			}
			/* Vectorizable loop 596 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec452[i as usize] = 0.0 - 0.33333334 * fZec435[i as usize];
			}
			/* Vectorizable loop 597 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec453[i as usize] = 0.0 - 0.5 * fZec436[i as usize];
			}
			/* Vectorizable loop 598 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec454[i as usize] = 0.0 - fZec437[i as usize];
			}
			/* Vectorizable loop 599 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec455[i as usize] = std::cmp::min(512, std::cmp::max(0, iZec432[i as usize]));
			}
			/* Vectorizable loop 600 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec457[i as usize] = F32::floor(fZec456[i as usize]);
			}
			/* Vectorizable loop 601 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec450[i as usize] = fZec430[i as usize] + (5.0 - fZec434[i as usize]);
			}
			/* Vectorizable loop 602 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec419[i as usize] = F32::floor(fZec418[i as usize]);
			}
			/* Vectorizable loop 603 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec426[i as usize] = fRec381_tmp[(i32::wrapping_add(4, i)) as usize] - fZec425[i as usize];
			}
			/* Vectorizable loop 604 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec451[i as usize] = 0.0 - 0.25 * fZec440[i as usize];
			}
			/* Vectorizable loop 605 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec416[i as usize] = fRec378_tmp[(i32::wrapping_add(4, i)) as usize] - fZec415[i as usize];
			}
			/* Vectorizable loop 606 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec446[i as usize] = 0.0 - 0.33333334 * fZec440[i as usize];
			}
			/* Vectorizable loop 607 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec448[i as usize] = 0.0 - fZec436[i as usize];
			}
			/* Vectorizable loop 608 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec462[i as usize] = fSlow92 + fZec458[i as usize] + (2.0 - fZec461[i as usize]);
			}
			/* Vectorizable loop 609 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec465[i as usize] = fZec464[i as usize] * fZec463[i as usize];
			}
			/* Vectorizable loop 610 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec479[i as usize] = fRec346_tmp[(i32::wrapping_add(4, i)) as usize] - fZec478[i as usize];
			}
			/* Vectorizable loop 611 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec466[i as usize] = fSlow92 + fZec458[i as usize] + (1.0 - fZec461[i as usize]);
			}
			/* Vectorizable loop 612 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec476[i as usize] = fZec474[i as usize] + (2.0 - fRec386_tmp[(i32::wrapping_add(4, i)) as usize]);
			}
			/* Vectorizable loop 613 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec469[i as usize] = fRec350_tmp[(i32::wrapping_add(4, i)) as usize] - fZec468[i as usize];
			}
			/* Vectorizable loop 614 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec480[i as usize] = fZec478[i as usize] + (2.0 - fRec346_tmp[(i32::wrapping_add(4, i)) as usize]);
			}
			/* Vectorizable loop 615 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec470[i as usize] = fZec468[i as usize] + (2.0 - fRec350_tmp[(i32::wrapping_add(4, i)) as usize]);
			}
			/* Vectorizable loop 616 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec475[i as usize] = fRec386_tmp[(i32::wrapping_add(4, i)) as usize] - fZec474[i as usize];
			}
			/* Vectorizable loop 617 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec460[i as usize] = (fZec459[i as usize]) as i32;
			}
			/* Vectorizable loop 618 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec554[i as usize] = 0.0 - 0.33333334 * fZec537[i as usize];
			}
			/* Vectorizable loop 619 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec550[i as usize] = 0.0 - fZec538[i as usize];
			}
			/* Vectorizable loop 620 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec567[i as usize] = fSlow92 + fZec563[i as usize] + (2.0 - fZec566[i as usize]);
			}
			/* Vectorizable loop 621 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec571[i as usize] = fSlow92 + fZec563[i as usize] + (1.0 - fZec566[i as usize]);
			}
			/* Vectorizable loop 622 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec557[i as usize] = std::cmp::max(0, iZec534[i as usize]);
			}
			/* Vectorizable loop 623 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec573[i as usize] = fZec559[i as usize] + fZec560[i as usize];
			}
			/* Vectorizable loop 624 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec562[i as usize] = F32::floor(fZec561[i as usize]);
			}
			/* Vectorizable loop 625 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec565[i as usize] = (fZec564[i as usize]) as i32;
			}
			/* Vectorizable loop 626 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec570[i as usize] = fZec569[i as usize] * fZec568[i as usize];
			}
			/* Vectorizable loop 627 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec555[i as usize] = 0.0 - 0.5 * fZec538[i as usize];
			}
			/* Vectorizable loop 628 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec549[i as usize] = 0.0 - 0.5 * fZec537[i as usize];
			}
			/* Vectorizable loop 629 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec556[i as usize] = 0.0 - fZec539[i as usize];
			}
			/* Vectorizable loop 630 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec553[i as usize] = 0.0 - 0.25 * fZec542[i as usize];
			}
			/* Vectorizable loop 631 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec548[i as usize] = 0.0 - 0.33333334 * fZec542[i as usize];
			}
			/* Vectorizable loop 632 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec552[i as usize] = fZec532[i as usize] + (5.0 - fZec536[i as usize]);
			}
			/* Vectorizable loop 633 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec551[i as usize] = std::cmp::max(0, i32::wrapping_add(iZec534[i as usize], 1));
			}
			/* Vectorizable loop 634 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec489[i as usize] = std::cmp::min(8192, std::cmp::max(0, (fZec485[i as usize]) as i32));
			}
			/* Vectorizable loop 635 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec506[i as usize] = fZec504[i as usize] + (2.0 - fRec13_tmp[(i32::wrapping_add(4, i)) as usize]);
			}
			/* Vectorizable loop 636 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec511[i as usize] = std::cmp::min(8192, std::cmp::max(0, (fZec503[i as usize]) as i32));
			}
			/* Vectorizable loop 637 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec488[i as usize] = fZec486[i as usize] + (2.0 - fRec17_tmp[(i32::wrapping_add(4, i)) as usize]);
			}
			/* Vectorizable loop 638 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec493[i as usize] = fZec491[i as usize] + (2.0 - fRec342_tmp[(i32::wrapping_add(4, i)) as usize]);
			}
			/* Vectorizable loop 639 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec505[i as usize] = fRec13_tmp[(i32::wrapping_add(4, i)) as usize] - fZec504[i as usize];
			}
			/* Vectorizable loop 640 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec514[i as usize] = fRec9_tmp[(i32::wrapping_add(4, i)) as usize] - fZec513[i as usize];
			}
			/* Vectorizable loop 641 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec515[i as usize] = fZec513[i as usize] + (2.0 - fRec9_tmp[(i32::wrapping_add(4, i)) as usize]);
			}
			/* Vectorizable loop 642 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec516[i as usize] = std::cmp::min(8192, std::cmp::max(0, (fZec512[i as usize]) as i32));
			}
			/* Vectorizable loop 643 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec492[i as usize] = fRec342_tmp[(i32::wrapping_add(4, i)) as usize] - fZec491[i as usize];
			}
			/* Vectorizable loop 644 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec520[i as usize] = fZec518[i as usize] + (2.0 - fRec335_tmp[(i32::wrapping_add(4, i)) as usize]);
			}
			/* Vectorizable loop 645 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec519[i as usize] = fRec335_tmp[(i32::wrapping_add(4, i)) as usize] - fZec518[i as usize];
			}
			/* Vectorizable loop 646 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec502[i as usize] = std::cmp::min(8192, std::cmp::max(0, (fZec498[i as usize]) as i32));
			}
			/* Vectorizable loop 647 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec487[i as usize] = fRec17_tmp[(i32::wrapping_add(4, i)) as usize] - fZec486[i as usize];
			}
			/* Vectorizable loop 648 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec500[i as usize] = fRec389_tmp[(i32::wrapping_add(4, i)) as usize] - fZec499[i as usize];
			}
			/* Vectorizable loop 649 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec501[i as usize] = fZec499[i as usize] + (2.0 - fRec389_tmp[(i32::wrapping_add(4, i)) as usize]);
			}
			/* Vectorizable loop 650 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec544[i as usize] = std::cmp::max(0, i32::wrapping_add(iZec534[i as usize], 3));
			}
			/* Vectorizable loop 651 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec527[i as usize] = fRec392_tmp[(i32::wrapping_add(4, i)) as usize] - fZec526[i as usize];
			}
			/* Vectorizable loop 652 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec529[i as usize] = std::cmp::min(8192, std::cmp::max(0, (fZec525[i as usize]) as i32));
			}
			/* Vectorizable loop 653 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec543[i as usize] = 0.0 - fZec542[i as usize];
			}
			/* Vectorizable loop 654 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec528[i as usize] = fZec526[i as usize] + (2.0 - fRec392_tmp[(i32::wrapping_add(4, i)) as usize]);
			}
			/* Vectorizable loop 655 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec545[i as usize] = 0.0 - 0.5 * fZec542[i as usize];
			}
			/* Vectorizable loop 656 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec535[i as usize] = std::cmp::max(0, i32::wrapping_add(iZec534[i as usize], 4));
			}
			/* Vectorizable loop 657 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec546[i as usize] = 0.0 - fZec537[i as usize];
			}
			/* Vectorizable loop 658 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				iZec547[i as usize] = std::cmp::max(0, i32::wrapping_add(iZec534[i as usize], 2));
			}
			/* Vectorizable loop 659 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec541[i as usize] = fZec540[i as usize] * fZec537[i as usize];
			}
			/* Vectorizable loop 660 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec531[i as usize] = F32::floor(fZec530[i as usize]);
			}
			/* Vectorizable loop 661 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec579[i as usize] = fZec577[i as usize] + (2.0 - fRec420_tmp[(i32::wrapping_add(4, i)) as usize]);
			}
			/* Vectorizable loop 662 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec578[i as usize] = fRec420_tmp[(i32::wrapping_add(4, i)) as usize] - fZec577[i as usize];
			}
			/* Vectorizable loop 663 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec591[i as usize] = fZec589[i as usize] + (2.0 - fRec423_tmp[(i32::wrapping_add(4, i)) as usize]);
			}
			/* Vectorizable loop 664 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec590[i as usize] = fRec423_tmp[(i32::wrapping_add(4, i)) as usize] - fZec589[i as usize];
			}
			/* Vectorizable loop 665 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec593[i as usize] = F32::floor(fZec592[i as usize]);
			}
			/* Recursive loop 666 */
			/* Pre code */
			self.fYec39_idx = (i32::wrapping_add(self.fYec39_idx, self.fYec39_idx_save)) & 16383;
			for j442 in 0..4 {
				fYec40_tmp[j442 as usize] = self.fYec40_perm[j442 as usize];
			}
			for j444 in 0..4 {
				fRec368_tmp[j444 as usize] = self.fRec368_perm[j444 as usize];
			}
			for j446 in 0..4 {
				fRec366_tmp[j446 as usize] = self.fRec366_perm[j446 as usize];
			}
			self.fYec41_idx = (i32::wrapping_add(self.fYec41_idx, self.fYec41_idx_save)) & 16383;
			for j450 in 0..4 {
				fYec42_tmp[j450 as usize] = self.fYec42_perm[j450 as usize];
			}
			for j452 in 0..4 {
				fRec370_tmp[j452 as usize] = self.fRec370_perm[j452 as usize];
			}
			for j454 in 0..4 {
				fRec367_tmp[j454 as usize] = self.fRec367_perm[j454 as usize];
			}
			self.fYec43_idx = (i32::wrapping_add(self.fYec43_idx, self.fYec43_idx_save)) & 16383;
			for j458 in 0..4 {
				fYec44_tmp[j458 as usize] = self.fYec44_perm[j458 as usize];
			}
			for j460 in 0..4 {
				fRec365_tmp[j460 as usize] = self.fRec365_perm[j460 as usize];
			}
			for j462 in 0..4 {
				fRec363_tmp[j462 as usize] = self.fRec363_perm[j462 as usize];
			}
			self.fYec45_idx = (i32::wrapping_add(self.fYec45_idx, self.fYec45_idx_save)) & 16383;
			for j466 in 0..4 {
				fYec46_tmp[j466 as usize] = self.fYec46_perm[j466 as usize];
			}
			for j468 in 0..4 {
				fRec373_tmp[j468 as usize] = self.fRec373_perm[j468 as usize];
			}
			for j470 in 0..4 {
				fRec364_tmp[j470 as usize] = self.fRec364_perm[j470 as usize];
			}
			self.fYec47_idx = (i32::wrapping_add(self.fYec47_idx, self.fYec47_idx_save)) & 16383;
			for j472 in 0..4 {
				fYec48_tmp[j472 as usize] = self.fYec48_perm[j472 as usize];
			}
			for j474 in 0..4 {
				fRec361_tmp[j474 as usize] = self.fRec361_perm[j474 as usize];
			}
			for j476 in 0..4 {
				fRec359_tmp[j476 as usize] = self.fRec359_perm[j476 as usize];
			}
			self.fYec49_idx = (i32::wrapping_add(self.fYec49_idx, self.fYec49_idx_save)) & 16383;
			for j480 in 0..4 {
				fYec50_tmp[j480 as usize] = self.fYec50_perm[j480 as usize];
			}
			for j482 in 0..4 {
				fRec375_tmp[j482 as usize] = self.fRec375_perm[j482 as usize];
			}
			for j484 in 0..4 {
				fRec360_tmp[j484 as usize] = self.fRec360_perm[j484 as usize];
			}
			self.fYec51_idx = (i32::wrapping_add(self.fYec51_idx, self.fYec51_idx_save)) & 16383;
			for j486 in 0..4 {
				fYec52_tmp[j486 as usize] = self.fYec52_perm[j486 as usize];
			}
			for j488 in 0..4 {
				fRec357_tmp[j488 as usize] = self.fRec357_perm[j488 as usize];
			}
			for j490 in 0..4 {
				fRec355_tmp[j490 as usize] = self.fRec355_perm[j490 as usize];
			}
			self.fYec53_idx = (i32::wrapping_add(self.fYec53_idx, self.fYec53_idx_save)) & 16383;
			for j494 in 0..4 {
				fYec54_tmp[j494 as usize] = self.fYec54_perm[j494 as usize];
			}
			for j496 in 0..4 {
				fRec377_tmp[j496 as usize] = self.fRec377_perm[j496 as usize];
			}
			for j498 in 0..4 {
				fRec356_tmp[j498 as usize] = self.fRec356_perm[j498 as usize];
			}
			self.fYec55_idx = (i32::wrapping_add(self.fYec55_idx, self.fYec55_idx_save)) & 16383;
			for j502 in 0..4 {
				fYec56_tmp[j502 as usize] = self.fYec56_perm[j502 as usize];
			}
			for j504 in 0..4 {
				fRec354_tmp[j504 as usize] = self.fRec354_perm[j504 as usize];
			}
			for j506 in 0..4 {
				fRec352_tmp[j506 as usize] = self.fRec352_perm[j506 as usize];
			}
			self.fYec57_idx = (i32::wrapping_add(self.fYec57_idx, self.fYec57_idx_save)) & 16383;
			for j510 in 0..4 {
				fYec58_tmp[j510 as usize] = self.fYec58_perm[j510 as usize];
			}
			for j512 in 0..4 {
				fRec380_tmp[j512 as usize] = self.fRec380_perm[j512 as usize];
			}
			for j514 in 0..4 {
				fRec353_tmp[j514 as usize] = self.fRec353_perm[j514 as usize];
			}
			self.fYec59_idx = (i32::wrapping_add(self.fYec59_idx, self.fYec59_idx_save)) & 1023;
			self.fYec60_idx = (i32::wrapping_add(self.fYec60_idx, self.fYec60_idx_save)) & 16383;
			for j518 in 0..4 {
				fYec61_tmp[j518 as usize] = self.fYec61_perm[j518 as usize];
			}
			for j520 in 0..4 {
				fRec351_tmp[j520 as usize] = self.fRec351_perm[j520 as usize];
			}
			self.fYec62_idx = (i32::wrapping_add(self.fYec62_idx, self.fYec62_idx_save)) & 1023;
			self.fYec63_idx = (i32::wrapping_add(self.fYec63_idx, self.fYec63_idx_save)) & 16383;
			for j524 in 0..4 {
				fYec64_tmp[j524 as usize] = self.fYec64_perm[j524 as usize];
			}
			for j526 in 0..4 {
				fRec383_tmp[j526 as usize] = self.fRec383_perm[j526 as usize];
			}
			self.fYec65_idx = (i32::wrapping_add(self.fYec65_idx, self.fYec65_idx_save)) & 16383;
			for j528 in 0..4 {
				fYec66_tmp[j528 as usize] = self.fYec66_perm[j528 as usize];
			}
			for j530 in 0..4 {
				fRec349_tmp[j530 as usize] = self.fRec349_perm[j530 as usize];
			}
			for j532 in 0..4 {
				fRec347_tmp[j532 as usize] = self.fRec347_perm[j532 as usize];
			}
			self.fYec67_idx = (i32::wrapping_add(self.fYec67_idx, self.fYec67_idx_save)) & 16383;
			for j536 in 0..4 {
				fYec68_tmp[j536 as usize] = self.fYec68_perm[j536 as usize];
			}
			for j538 in 0..4 {
				fRec385_tmp[j538 as usize] = self.fRec385_perm[j538 as usize];
			}
			for j540 in 0..4 {
				fRec348_tmp[j540 as usize] = self.fRec348_perm[j540 as usize];
			}
			self.fYec69_idx = (i32::wrapping_add(self.fYec69_idx, self.fYec69_idx_save)) & 16383;
			for j542 in 0..4 {
				fYec70_tmp[j542 as usize] = self.fYec70_perm[j542 as usize];
			}
			for j544 in 0..4 {
				fRec345_tmp[j544 as usize] = self.fRec345_perm[j544 as usize];
			}
			for j546 in 0..4 {
				fRec343_tmp[j546 as usize] = self.fRec343_perm[j546 as usize];
			}
			self.fYec71_idx = (i32::wrapping_add(self.fYec71_idx, self.fYec71_idx_save)) & 16383;
			for j548 in 0..4 {
				fYec72_tmp[j548 as usize] = self.fYec72_perm[j548 as usize];
			}
			for j550 in 0..4 {
				fRec387_tmp[j550 as usize] = self.fRec387_perm[j550 as usize];
			}
			for j552 in 0..4 {
				fRec344_tmp[j552 as usize] = self.fRec344_perm[j552 as usize];
			}
			self.fYec73_idx = (i32::wrapping_add(self.fYec73_idx, self.fYec73_idx_save)) & 16383;
			for j554 in 0..4 {
				fYec74_tmp[j554 as usize] = self.fYec74_perm[j554 as usize];
			}
			for j556 in 0..4 {
				fRec341_tmp[j556 as usize] = self.fRec341_perm[j556 as usize];
			}
			for j558 in 0..4 {
				fRec339_tmp[j558 as usize] = self.fRec339_perm[j558 as usize];
			}
			self.fYec75_idx = (i32::wrapping_add(self.fYec75_idx, self.fYec75_idx_save)) & 16383;
			for j562 in 0..4 {
				fYec76_tmp[j562 as usize] = self.fYec76_perm[j562 as usize];
			}
			for j564 in 0..4 {
				fRec388_tmp[j564 as usize] = self.fRec388_perm[j564 as usize];
			}
			for j566 in 0..4 {
				fRec340_tmp[j566 as usize] = self.fRec340_perm[j566 as usize];
			}
			self.fYec77_idx = (i32::wrapping_add(self.fYec77_idx, self.fYec77_idx_save)) & 16383;
			for j568 in 0..4 {
				fYec78_tmp[j568 as usize] = self.fYec78_perm[j568 as usize];
			}
			for j570 in 0..4 {
				fRec338_tmp[j570 as usize] = self.fRec338_perm[j570 as usize];
			}
			for j572 in 0..4 {
				fRec336_tmp[j572 as usize] = self.fRec336_perm[j572 as usize];
			}
			self.fYec79_idx = (i32::wrapping_add(self.fYec79_idx, self.fYec79_idx_save)) & 16383;
			for j574 in 0..4 {
				fYec80_tmp[j574 as usize] = self.fYec80_perm[j574 as usize];
			}
			for j576 in 0..4 {
				fRec390_tmp[j576 as usize] = self.fRec390_perm[j576 as usize];
			}
			for j578 in 0..4 {
				fRec337_tmp[j578 as usize] = self.fRec337_perm[j578 as usize];
			}
			self.fYec81_idx = (i32::wrapping_add(self.fYec81_idx, self.fYec81_idx_save)) & 16383;
			for j580 in 0..4 {
				fYec82_tmp[j580 as usize] = self.fYec82_perm[j580 as usize];
			}
			for j582 in 0..4 {
				fRec334_tmp[j582 as usize] = self.fRec334_perm[j582 as usize];
			}
			for j584 in 0..4 {
				fRec332_tmp[j584 as usize] = self.fRec332_perm[j584 as usize];
			}
			self.fYec83_idx = (i32::wrapping_add(self.fYec83_idx, self.fYec83_idx_save)) & 16383;
			for j588 in 0..4 {
				fYec84_tmp[j588 as usize] = self.fYec84_perm[j588 as usize];
			}
			for j590 in 0..4 {
				fRec391_tmp[j590 as usize] = self.fRec391_perm[j590 as usize];
			}
			for j592 in 0..4 {
				fRec333_tmp[j592 as usize] = self.fRec333_perm[j592 as usize];
			}
			self.fYec85_idx = (i32::wrapping_add(self.fYec85_idx, self.fYec85_idx_save)) & 16383;
			self.fYec86_idx = (i32::wrapping_add(self.fYec86_idx, self.fYec86_idx_save)) & 16383;
			for j596 in 0..4 {
				fYec87_tmp[j596 as usize] = self.fYec87_perm[j596 as usize];
			}
			for j598 in 0..4 {
				fRec331_tmp[j598 as usize] = self.fRec331_perm[j598 as usize];
			}
			for j600 in 0..4 {
				fRec330_tmp[j600 as usize] = self.fRec330_perm[j600 as usize];
			}
			for j602 in 0..4 {
				fRec329_tmp[j602 as usize] = self.fRec329_perm[j602 as usize];
			}
			for j604 in 0..4 {
				fRec328_tmp[j604 as usize] = self.fRec328_perm[j604 as usize];
			}
			for j606 in 0..4 {
				fRec327_tmp[j606 as usize] = self.fRec327_perm[j606 as usize];
			}
			for j608 in 0..4 {
				fRec399_tmp[j608 as usize] = self.fRec399_perm[j608 as usize];
			}
			for j610 in 0..4 {
				fRec398_tmp[j610 as usize] = self.fRec398_perm[j610 as usize];
			}
			for j612 in 0..4 {
				fRec397_tmp[j612 as usize] = self.fRec397_perm[j612 as usize];
			}
			for j614 in 0..4 {
				fYec88_tmp[j614 as usize] = self.fYec88_perm[j614 as usize];
			}
			for j616 in 0..4 {
				fRec396_tmp[j616 as usize] = self.fRec396_perm[j616 as usize];
			}
			for j618 in 0..4 {
				fRec395_tmp[j618 as usize] = self.fRec395_perm[j618 as usize];
			}
			for j620 in 0..4 {
				fRec394_tmp[j620 as usize] = self.fRec394_perm[j620 as usize];
			}
			for j622 in 0..4 {
				fRec402_tmp[j622 as usize] = self.fRec402_perm[j622 as usize];
			}
			for j624 in 0..4 {
				fRec401_tmp[j624 as usize] = self.fRec401_perm[j624 as usize];
			}
			for j626 in 0..4 {
				fRec400_tmp[j626 as usize] = self.fRec400_perm[j626 as usize];
			}
			self.fYec89_idx = (i32::wrapping_add(self.fYec89_idx, self.fYec89_idx_save)) & 1023;
			for j628 in 0..4 {
				fRec18_tmp[j628 as usize] = self.fRec18_perm[j628 as usize];
			}
			self.fYec90_idx = (i32::wrapping_add(self.fYec90_idx, self.fYec90_idx_save)) & 16383;
			self.fYec91_idx = (i32::wrapping_add(self.fYec91_idx, self.fYec91_idx_save)) & 16383;
			for j632 in 0..4 {
				fYec92_tmp[j632 as usize] = self.fYec92_perm[j632 as usize];
			}
			for j634 in 0..4 {
				fRec408_tmp[j634 as usize] = self.fRec408_perm[j634 as usize];
			}
			for j636 in 0..4 {
				fRec407_tmp[j636 as usize] = self.fRec407_perm[j636 as usize];
			}
			for j638 in 0..4 {
				fRec406_tmp[j638 as usize] = self.fRec406_perm[j638 as usize];
			}
			for j640 in 0..4 {
				fRec405_tmp[j640 as usize] = self.fRec405_perm[j640 as usize];
			}
			for j642 in 0..4 {
				fRec404_tmp[j642 as usize] = self.fRec404_perm[j642 as usize];
			}
			for j644 in 0..4 {
				fRec415_tmp[j644 as usize] = self.fRec415_perm[j644 as usize];
			}
			for j646 in 0..4 {
				fRec414_tmp[j646 as usize] = self.fRec414_perm[j646 as usize];
			}
			for j648 in 0..4 {
				fRec413_tmp[j648 as usize] = self.fRec413_perm[j648 as usize];
			}
			for j650 in 0..4 {
				fYec93_tmp[j650 as usize] = self.fYec93_perm[j650 as usize];
			}
			for j652 in 0..4 {
				fRec412_tmp[j652 as usize] = self.fRec412_perm[j652 as usize];
			}
			for j654 in 0..4 {
				fRec411_tmp[j654 as usize] = self.fRec411_perm[j654 as usize];
			}
			for j656 in 0..4 {
				fRec410_tmp[j656 as usize] = self.fRec410_perm[j656 as usize];
			}
			for j658 in 0..4 {
				fRec418_tmp[j658 as usize] = self.fRec418_perm[j658 as usize];
			}
			for j660 in 0..4 {
				fRec417_tmp[j660 as usize] = self.fRec417_perm[j660 as usize];
			}
			for j662 in 0..4 {
				fRec416_tmp[j662 as usize] = self.fRec416_perm[j662 as usize];
			}
			self.fYec94_idx = (i32::wrapping_add(self.fYec94_idx, self.fYec94_idx_save)) & 1023;
			for j664 in 0..4 {
				fRec403_tmp[j664 as usize] = self.fRec403_perm[j664 as usize];
			}
			self.fYec95_idx = (i32::wrapping_add(self.fYec95_idx, self.fYec95_idx_save)) & 16383;
			for j666 in 0..4 {
				fYec96_tmp[j666 as usize] = self.fYec96_perm[j666 as usize];
			}
			for j668 in 0..4 {
				fRec16_tmp[j668 as usize] = self.fRec16_perm[j668 as usize];
			}
			for j670 in 0..4 {
				fRec14_tmp[j670 as usize] = self.fRec14_perm[j670 as usize];
			}
			self.fYec97_idx = (i32::wrapping_add(self.fYec97_idx, self.fYec97_idx_save)) & 16383;
			for j674 in 0..4 {
				fYec98_tmp[j674 as usize] = self.fYec98_perm[j674 as usize];
			}
			for j676 in 0..4 {
				fRec419_tmp[j676 as usize] = self.fRec419_perm[j676 as usize];
			}
			for j678 in 0..4 {
				fRec15_tmp[j678 as usize] = self.fRec15_perm[j678 as usize];
			}
			self.fYec99_idx = (i32::wrapping_add(self.fYec99_idx, self.fYec99_idx_save)) & 16383;
			for j680 in 0..4 {
				fYec100_tmp[j680 as usize] = self.fYec100_perm[j680 as usize];
			}
			for j682 in 0..4 {
				fRec12_tmp[j682 as usize] = self.fRec12_perm[j682 as usize];
			}
			for j684 in 0..4 {
				fRec10_tmp[j684 as usize] = self.fRec10_perm[j684 as usize];
			}
			self.fYec101_idx = (i32::wrapping_add(self.fYec101_idx, self.fYec101_idx_save)) & 16383;
			for j686 in 0..4 {
				fYec102_tmp[j686 as usize] = self.fYec102_perm[j686 as usize];
			}
			for j688 in 0..4 {
				fRec421_tmp[j688 as usize] = self.fRec421_perm[j688 as usize];
			}
			for j690 in 0..4 {
				fRec11_tmp[j690 as usize] = self.fRec11_perm[j690 as usize];
			}
			self.fYec103_idx = (i32::wrapping_add(self.fYec103_idx, self.fYec103_idx_save)) & 16383;
			for j692 in 0..4 {
				fYec104_tmp[j692 as usize] = self.fYec104_perm[j692 as usize];
			}
			for j694 in 0..4 {
				fRec8_tmp[j694 as usize] = self.fRec8_perm[j694 as usize];
			}
			for j696 in 0..4 {
				fRec6_tmp[j696 as usize] = self.fRec6_perm[j696 as usize];
			}
			self.fYec105_idx = (i32::wrapping_add(self.fYec105_idx, self.fYec105_idx_save)) & 16383;
			for j700 in 0..4 {
				fYec106_tmp[j700 as usize] = self.fYec106_perm[j700 as usize];
			}
			for j702 in 0..4 {
				fRec422_tmp[j702 as usize] = self.fRec422_perm[j702 as usize];
			}
			for j704 in 0..4 {
				fRec7_tmp[j704 as usize] = self.fRec7_perm[j704 as usize];
			}
			self.fYec107_idx = (i32::wrapping_add(self.fYec107_idx, self.fYec107_idx_save)) & 16383;
			for j708 in 0..4 {
				fYec108_tmp[j708 as usize] = self.fYec108_perm[j708 as usize];
			}
			for j710 in 0..4 {
				fRec5_tmp[j710 as usize] = self.fRec5_perm[j710 as usize];
			}
			for j712 in 0..4 {
				fRec3_tmp[j712 as usize] = self.fRec3_perm[j712 as usize];
			}
			self.fYec109_idx = (i32::wrapping_add(self.fYec109_idx, self.fYec109_idx_save)) & 16383;
			for j714 in 0..4 {
				fYec110_tmp[j714 as usize] = self.fYec110_perm[j714 as usize];
			}
			for j716 in 0..4 {
				fRec425_tmp[j716 as usize] = self.fRec425_perm[j716 as usize];
			}
			for j718 in 0..4 {
				fRec4_tmp[j718 as usize] = self.fRec4_perm[j718 as usize];
			}
			for j720 in 0..4 {
				fRec1_tmp[j720 as usize] = self.fRec1_perm[j720 as usize];
			}
			for j722 in 0..4 {
				fRec2_tmp[j722 as usize] = self.fRec2_perm[j722 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec378[i as usize] = 0.760314 * fRec2_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] - 0.64955574 * fRec367_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fZec379[i as usize] = 0.760314 * fRec1_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] - 0.64955574 * fRec366_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				self.fYec39[((i32::wrapping_add(i, self.fYec39_idx)) & 16383) as usize] = fZec379[i as usize] - fZec378[i as usize];
				fYec40_tmp[(i32::wrapping_add(4, i)) as usize] = self.fYec39[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec39_idx), std::cmp::min(8192, std::cmp::max(0, (fZec374[i as usize]) as i32)))) & 16383) as usize];
				fRec368_tmp[(i32::wrapping_add(4, i)) as usize] = 0.70710677 * (fZec377[i as usize] * fYec40_tmp[(i32::wrapping_add(4, i)) as usize] / fZec376[i as usize] + fYec40_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) - fRec368_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] * fZec377[i as usize] / fZec376[i as usize];
				fRec366_tmp[(i32::wrapping_add(4, i)) as usize] = fRec368_tmp[(i32::wrapping_add(4, i)) as usize];
				self.fYec41[((i32::wrapping_add(i, self.fYec41_idx)) & 16383) as usize] = fZec379[i as usize] + fZec378[i as usize];
				fYec42_tmp[(i32::wrapping_add(4, i)) as usize] = self.fYec41[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec41_idx), std::cmp::min(8192, std::cmp::max(0, (fZec380[i as usize]) as i32)))) & 16383) as usize];
				fRec370_tmp[(i32::wrapping_add(4, i)) as usize] = 0.70710677 * (fZec383[i as usize] * fYec42_tmp[(i32::wrapping_add(4, i)) as usize] / fZec382[i as usize] + fYec42_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) - fRec370_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] * fZec383[i as usize] / fZec382[i as usize];
				fRec367_tmp[(i32::wrapping_add(4, i)) as usize] = fRec370_tmp[(i32::wrapping_add(4, i)) as usize];
				fZec386[i as usize] = 0.760314 * fRec366_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + 0.64955574 * fRec1_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fZec387[i as usize] = 0.760314 * fZec386[i as usize] - 0.64955574 * fRec363_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fZec388[i as usize] = 0.760314 * fRec367_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + 0.64955574 * fRec2_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fZec389[i as usize] = 0.760314 * fZec388[i as usize] - 0.64955574 * fRec364_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				self.fYec43[((i32::wrapping_add(i, self.fYec43_idx)) & 16383) as usize] = 0.0 - 0.70710677 * (fZec389[i as usize] - fZec387[i as usize]);
				fYec44_tmp[(i32::wrapping_add(4, i)) as usize] = self.fYec43[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec43_idx), std::cmp::min(8192, std::cmp::max(0, (fZec384[i as usize]) as i32)))) & 16383) as usize];
				fRec365_tmp[(i32::wrapping_add(4, i)) as usize] = fYec44_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] - (fZec385[i as usize] + (2.0 - fRec372_tmp[(i32::wrapping_add(4, i)) as usize])) * (fRec365_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] - fYec44_tmp[(i32::wrapping_add(4, i)) as usize]) / (fRec372_tmp[(i32::wrapping_add(4, i)) as usize] - fZec385[i as usize]);
				fRec363_tmp[(i32::wrapping_add(4, i)) as usize] = fRec365_tmp[(i32::wrapping_add(4, i)) as usize];
				self.fYec45[((i32::wrapping_add(i, self.fYec45_idx)) & 16383) as usize] = fZec387[i as usize] + fZec389[i as usize];
				fYec46_tmp[(i32::wrapping_add(4, i)) as usize] = self.fYec45[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec45_idx), std::cmp::min(8192, std::cmp::max(0, (fZec390[i as usize]) as i32)))) & 16383) as usize];
				fRec373_tmp[(i32::wrapping_add(4, i)) as usize] = 0.70710677 * (fZec393[i as usize] * fYec46_tmp[(i32::wrapping_add(4, i)) as usize] / fZec392[i as usize] + fYec46_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) - fRec373_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] * fZec393[i as usize] / fZec392[i as usize];
				fRec364_tmp[(i32::wrapping_add(4, i)) as usize] = fRec373_tmp[(i32::wrapping_add(4, i)) as usize];
				fZec398[i as usize] = 0.760314 * fRec364_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + 0.64955574 * fZec388[i as usize];
				fZec399[i as usize] = 0.760314 * fZec398[i as usize] - 0.64955574 * fRec360_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fZec400[i as usize] = 0.760314 * fRec363_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + 0.64955574 * fZec386[i as usize];
				fZec401[i as usize] = 0.760314 * fZec400[i as usize] - 0.64955574 * fRec359_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				self.fYec47[((i32::wrapping_add(i, self.fYec47_idx)) & 16383) as usize] = fZec401[i as usize] - fZec399[i as usize];
				fYec48_tmp[(i32::wrapping_add(4, i)) as usize] = self.fYec47[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec47_idx), std::cmp::min(8192, std::cmp::max(0, (fZec394[i as usize]) as i32)))) & 16383) as usize];
				fRec361_tmp[(i32::wrapping_add(4, i)) as usize] = 0.70710677 * (fZec397[i as usize] * fYec48_tmp[(i32::wrapping_add(4, i)) as usize] / fZec396[i as usize] + fYec48_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) - fRec361_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] * fZec397[i as usize] / fZec396[i as usize];
				fRec359_tmp[(i32::wrapping_add(4, i)) as usize] = fRec361_tmp[(i32::wrapping_add(4, i)) as usize];
				self.fYec49[((i32::wrapping_add(i, self.fYec49_idx)) & 16383) as usize] = fZec401[i as usize] + fZec399[i as usize];
				fYec50_tmp[(i32::wrapping_add(4, i)) as usize] = self.fYec49[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec49_idx), std::cmp::min(8192, std::cmp::max(0, (fZec402[i as usize]) as i32)))) & 16383) as usize];
				fRec375_tmp[(i32::wrapping_add(4, i)) as usize] = 0.70710677 * (fZec405[i as usize] * fYec50_tmp[(i32::wrapping_add(4, i)) as usize] / fZec404[i as usize] + fYec50_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) - fRec375_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] * fZec405[i as usize] / fZec404[i as usize];
				fRec360_tmp[(i32::wrapping_add(4, i)) as usize] = fRec375_tmp[(i32::wrapping_add(4, i)) as usize];
				fZec410[i as usize] = 0.760314 * fRec360_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + 0.64955574 * fZec398[i as usize];
				fZec411[i as usize] = 0.760314 * fZec410[i as usize] - 0.64955574 * fRec356_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fZec412[i as usize] = 0.760314 * fRec359_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + 0.64955574 * fZec400[i as usize];
				fZec413[i as usize] = 0.760314 * fZec412[i as usize] - 0.64955574 * fRec355_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				self.fYec51[((i32::wrapping_add(i, self.fYec51_idx)) & 16383) as usize] = fZec413[i as usize] - fZec411[i as usize];
				fYec52_tmp[(i32::wrapping_add(4, i)) as usize] = self.fYec51[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec51_idx), std::cmp::min(8192, std::cmp::max(0, (fZec406[i as usize]) as i32)))) & 16383) as usize];
				fRec357_tmp[(i32::wrapping_add(4, i)) as usize] = 0.70710677 * (fZec409[i as usize] * fYec52_tmp[(i32::wrapping_add(4, i)) as usize] / fZec408[i as usize] + fYec52_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) - fRec357_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] * fZec409[i as usize] / fZec408[i as usize];
				fRec355_tmp[(i32::wrapping_add(4, i)) as usize] = fRec357_tmp[(i32::wrapping_add(4, i)) as usize];
				self.fYec53[((i32::wrapping_add(i, self.fYec53_idx)) & 16383) as usize] = fZec413[i as usize] + fZec411[i as usize];
				fYec54_tmp[(i32::wrapping_add(4, i)) as usize] = self.fYec53[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec53_idx), std::cmp::min(8192, std::cmp::max(0, (fZec414[i as usize]) as i32)))) & 16383) as usize];
				fRec377_tmp[(i32::wrapping_add(4, i)) as usize] = 0.70710677 * (fZec417[i as usize] * fYec54_tmp[(i32::wrapping_add(4, i)) as usize] / fZec416[i as usize] + fYec54_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) - fRec377_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] * fZec417[i as usize] / fZec416[i as usize];
				fRec356_tmp[(i32::wrapping_add(4, i)) as usize] = fRec377_tmp[(i32::wrapping_add(4, i)) as usize];
				fZec420[i as usize] = 0.760314 * fRec355_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + 0.64955574 * fZec412[i as usize];
				fZec421[i as usize] = 0.760314 * fZec420[i as usize] - 0.64955574 * fRec352_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fZec422[i as usize] = 0.760314 * fRec356_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + 0.64955574 * fZec410[i as usize];
				fZec423[i as usize] = 0.760314 * fZec422[i as usize] - 0.64955574 * fRec353_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				self.fYec55[((i32::wrapping_add(i, self.fYec55_idx)) & 16383) as usize] = 0.0 - 0.70710677 * (fZec423[i as usize] - fZec421[i as usize]);
				fYec56_tmp[(i32::wrapping_add(4, i)) as usize] = self.fYec55[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec55_idx), std::cmp::min(8192, std::cmp::max(0, (fZec418[i as usize]) as i32)))) & 16383) as usize];
				fRec354_tmp[(i32::wrapping_add(4, i)) as usize] = fYec56_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] - (fZec419[i as usize] + (2.0 - fRec379_tmp[(i32::wrapping_add(4, i)) as usize])) * (fRec354_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] - fYec56_tmp[(i32::wrapping_add(4, i)) as usize]) / (fRec379_tmp[(i32::wrapping_add(4, i)) as usize] - fZec419[i as usize]);
				fRec352_tmp[(i32::wrapping_add(4, i)) as usize] = fRec354_tmp[(i32::wrapping_add(4, i)) as usize];
				self.fYec57[((i32::wrapping_add(i, self.fYec57_idx)) & 16383) as usize] = fZec421[i as usize] + fZec423[i as usize];
				fYec58_tmp[(i32::wrapping_add(4, i)) as usize] = self.fYec57[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec57_idx), std::cmp::min(8192, std::cmp::max(0, (fZec424[i as usize]) as i32)))) & 16383) as usize];
				fRec380_tmp[(i32::wrapping_add(4, i)) as usize] = 0.70710677 * (fZec427[i as usize] * fYec58_tmp[(i32::wrapping_add(4, i)) as usize] / fZec426[i as usize] + fYec58_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) - fRec380_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] * fZec427[i as usize] / fZec426[i as usize];
				fRec353_tmp[(i32::wrapping_add(4, i)) as usize] = fRec380_tmp[(i32::wrapping_add(4, i)) as usize];
				self.fYec59[((i32::wrapping_add(i, self.fYec59_idx)) & 1023) as usize] = 0.760314 * fRec352_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + 0.64955574 * fZec420[i as usize];
				self.fYec60[((i32::wrapping_add(i, self.fYec60_idx)) & 16383) as usize] = self.fYec59[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec59_idx), iZec455[i as usize])) & 1023) as usize] * fZec454[i as usize] * fZec453[i as usize] * fZec452[i as usize] * fZec451[i as usize] + fZec450[i as usize] * (self.fYec59[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec59_idx), iZec449[i as usize])) & 1023) as usize] * fZec448[i as usize] * fZec447[i as usize] * fZec446[i as usize] + 0.5 * fZec437[i as usize] * self.fYec59[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec59_idx), iZec445[i as usize])) & 1023) as usize] * fZec444[i as usize] * fZec443[i as usize] + 0.16666667 * fZec438[i as usize] * self.fYec59[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec59_idx), iZec442[i as usize])) & 1023) as usize] * fZec441[i as usize] + 0.041666668 * fZec439[i as usize] * self.fYec59[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec59_idx), iZec433[i as usize])) & 1023) as usize]);
				fYec61_tmp[(i32::wrapping_add(4, i)) as usize] = self.fYec60[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec60_idx), std::cmp::min(8192, std::cmp::max(0, (fZec428[i as usize]) as i32)))) & 16383) as usize];
				fRec351_tmp[(i32::wrapping_add(4, i)) as usize] = fYec61_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] - (fZec429[i as usize] + (2.0 - fRec382_tmp[(i32::wrapping_add(4, i)) as usize])) * (fRec351_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] - fYec61_tmp[(i32::wrapping_add(4, i)) as usize]) / (fRec382_tmp[(i32::wrapping_add(4, i)) as usize] - fZec429[i as usize]);
				self.fYec62[((i32::wrapping_add(i, self.fYec62_idx)) & 1023) as usize] = 0.760314 * fRec353_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + 0.64955574 * fZec422[i as usize];
				self.fYec63[((i32::wrapping_add(i, self.fYec63_idx)) & 16383) as usize] = self.fYec62[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec62_idx), std::cmp::min(512, std::cmp::max(0, iZec460[i as usize])))) & 1023) as usize] * (0.0 - fZec464[i as usize]) * (0.0 - 0.5 * fZec463[i as usize]) * (0.0 - 0.33333334 * fZec462[i as usize]) * (0.0 - 0.25 * fZec466[i as usize]) + (fSlow92 + fZec458[i as usize] + (5.0 - fZec461[i as usize])) * (self.fYec62[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec62_idx), std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iZec460[i as usize], 1))))) & 1023) as usize] * (0.0 - fZec463[i as usize]) * (0.0 - 0.5 * fZec462[i as usize]) * (0.0 - 0.33333334 * fZec466[i as usize]) + 0.5 * fZec464[i as usize] * self.fYec62[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec62_idx), std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iZec460[i as usize], 2))))) & 1023) as usize] * (0.0 - fZec462[i as usize]) * (0.0 - 0.5 * fZec466[i as usize]) + 0.16666667 * fZec465[i as usize] * self.fYec62[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec62_idx), std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iZec460[i as usize], 3))))) & 1023) as usize] * (0.0 - fZec466[i as usize]) + 0.041666668 * fZec465[i as usize] * fZec462[i as usize] * self.fYec62[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec62_idx), std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iZec460[i as usize], 4))))) & 1023) as usize]);
				fYec64_tmp[(i32::wrapping_add(4, i)) as usize] = self.fYec63[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec63_idx), std::cmp::min(8192, std::cmp::max(0, (fZec456[i as usize]) as i32)))) & 16383) as usize];
				fRec383_tmp[(i32::wrapping_add(4, i)) as usize] = fYec64_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] - (fZec457[i as usize] + (2.0 - fRec384_tmp[(i32::wrapping_add(4, i)) as usize])) * (fRec383_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] - fYec64_tmp[(i32::wrapping_add(4, i)) as usize]) / (fRec384_tmp[(i32::wrapping_add(4, i)) as usize] - fZec457[i as usize]);
				fZec471[i as usize] = 0.760314 * fRec383_tmp[(i32::wrapping_add(4, i)) as usize] - 0.64955574 * fRec348_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fZec472[i as usize] = 0.760314 * fRec351_tmp[(i32::wrapping_add(4, i)) as usize] - 0.64955574 * fRec347_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				self.fYec65[((i32::wrapping_add(i, self.fYec65_idx)) & 16383) as usize] = fZec472[i as usize] - fZec471[i as usize];
				fYec66_tmp[(i32::wrapping_add(4, i)) as usize] = self.fYec65[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec65_idx), std::cmp::min(8192, std::cmp::max(0, (fZec467[i as usize]) as i32)))) & 16383) as usize];
				fRec349_tmp[(i32::wrapping_add(4, i)) as usize] = 0.70710677 * (fZec470[i as usize] * fYec66_tmp[(i32::wrapping_add(4, i)) as usize] / fZec469[i as usize] + fYec66_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) - fRec349_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] * fZec470[i as usize] / fZec469[i as usize];
				fRec347_tmp[(i32::wrapping_add(4, i)) as usize] = fRec349_tmp[(i32::wrapping_add(4, i)) as usize];
				self.fYec67[((i32::wrapping_add(i, self.fYec67_idx)) & 16383) as usize] = fZec472[i as usize] + fZec471[i as usize];
				fYec68_tmp[(i32::wrapping_add(4, i)) as usize] = self.fYec67[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec67_idx), std::cmp::min(8192, std::cmp::max(0, (fZec473[i as usize]) as i32)))) & 16383) as usize];
				fRec385_tmp[(i32::wrapping_add(4, i)) as usize] = 0.70710677 * (fZec476[i as usize] * fYec68_tmp[(i32::wrapping_add(4, i)) as usize] / fZec475[i as usize] + fYec68_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) - fRec385_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] * fZec476[i as usize] / fZec475[i as usize];
				fRec348_tmp[(i32::wrapping_add(4, i)) as usize] = fRec385_tmp[(i32::wrapping_add(4, i)) as usize];
				fZec481[i as usize] = 0.760314 * fRec348_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + 0.64955574 * fRec383_tmp[(i32::wrapping_add(4, i)) as usize];
				fZec482[i as usize] = 0.760314 * fZec481[i as usize] - 0.64955574 * fRec344_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fZec483[i as usize] = 0.760314 * fRec347_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + 0.64955574 * fRec351_tmp[(i32::wrapping_add(4, i)) as usize];
				fZec484[i as usize] = 0.760314 * fZec483[i as usize] - 0.64955574 * fRec343_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				self.fYec69[((i32::wrapping_add(i, self.fYec69_idx)) & 16383) as usize] = fZec484[i as usize] - fZec482[i as usize];
				fYec70_tmp[(i32::wrapping_add(4, i)) as usize] = self.fYec69[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec69_idx), std::cmp::min(8192, std::cmp::max(0, (fZec477[i as usize]) as i32)))) & 16383) as usize];
				fRec345_tmp[(i32::wrapping_add(4, i)) as usize] = 0.70710677 * (fZec480[i as usize] * fYec70_tmp[(i32::wrapping_add(4, i)) as usize] / fZec479[i as usize] + fYec70_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) - fRec345_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] * fZec480[i as usize] / fZec479[i as usize];
				fRec343_tmp[(i32::wrapping_add(4, i)) as usize] = fRec345_tmp[(i32::wrapping_add(4, i)) as usize];
				self.fYec71[((i32::wrapping_add(i, self.fYec71_idx)) & 16383) as usize] = fZec484[i as usize] + fZec482[i as usize];
				fYec72_tmp[(i32::wrapping_add(4, i)) as usize] = self.fYec71[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec71_idx), iZec489[i as usize])) & 16383) as usize];
				fRec387_tmp[(i32::wrapping_add(4, i)) as usize] = 0.70710677 * (fZec488[i as usize] * fYec72_tmp[(i32::wrapping_add(4, i)) as usize] / fZec487[i as usize] + fYec72_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) - fZec488[i as usize] * fRec387_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] / fZec487[i as usize];
				fRec344_tmp[(i32::wrapping_add(4, i)) as usize] = fRec387_tmp[(i32::wrapping_add(4, i)) as usize];
				fZec494[i as usize] = 0.760314 * fRec344_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + 0.64955574 * fZec481[i as usize];
				fZec495[i as usize] = 0.760314 * fZec494[i as usize] - 0.64955574 * fRec340_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fZec496[i as usize] = 0.760314 * fRec343_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + 0.64955574 * fZec483[i as usize];
				fZec497[i as usize] = 0.760314 * fZec496[i as usize] - 0.64955574 * fRec339_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				self.fYec73[((i32::wrapping_add(i, self.fYec73_idx)) & 16383) as usize] = fZec497[i as usize] - fZec495[i as usize];
				fYec74_tmp[(i32::wrapping_add(4, i)) as usize] = self.fYec73[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec73_idx), std::cmp::min(8192, std::cmp::max(0, (fZec490[i as usize]) as i32)))) & 16383) as usize];
				fRec341_tmp[(i32::wrapping_add(4, i)) as usize] = 0.70710677 * (fZec493[i as usize] * fYec74_tmp[(i32::wrapping_add(4, i)) as usize] / fZec492[i as usize] + fYec74_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) - fRec341_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] * fZec493[i as usize] / fZec492[i as usize];
				fRec339_tmp[(i32::wrapping_add(4, i)) as usize] = fRec341_tmp[(i32::wrapping_add(4, i)) as usize];
				self.fYec75[((i32::wrapping_add(i, self.fYec75_idx)) & 16383) as usize] = fZec497[i as usize] + fZec495[i as usize];
				fYec76_tmp[(i32::wrapping_add(4, i)) as usize] = self.fYec75[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec75_idx), iZec502[i as usize])) & 16383) as usize];
				fRec388_tmp[(i32::wrapping_add(4, i)) as usize] = 0.70710677 * (fZec501[i as usize] * fYec76_tmp[(i32::wrapping_add(4, i)) as usize] / fZec500[i as usize] + fYec76_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) - fRec388_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] * fZec501[i as usize] / fZec500[i as usize];
				fRec340_tmp[(i32::wrapping_add(4, i)) as usize] = fRec388_tmp[(i32::wrapping_add(4, i)) as usize];
				fZec507[i as usize] = 0.760314 * fRec340_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + 0.64955574 * fZec494[i as usize];
				fZec508[i as usize] = 0.760314 * fZec507[i as usize] - 0.64955574 * fRec337_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fZec509[i as usize] = 0.760314 * fRec339_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + 0.64955574 * fZec496[i as usize];
				fZec510[i as usize] = 0.760314 * fZec509[i as usize] - 0.64955574 * fRec336_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				self.fYec77[((i32::wrapping_add(i, self.fYec77_idx)) & 16383) as usize] = fZec510[i as usize] - fZec508[i as usize];
				fYec78_tmp[(i32::wrapping_add(4, i)) as usize] = self.fYec77[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec77_idx), iZec511[i as usize])) & 16383) as usize];
				fRec338_tmp[(i32::wrapping_add(4, i)) as usize] = 0.70710677 * (fZec506[i as usize] * fYec78_tmp[(i32::wrapping_add(4, i)) as usize] / fZec505[i as usize] + fYec78_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) - fZec506[i as usize] * fRec338_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] / fZec505[i as usize];
				fRec336_tmp[(i32::wrapping_add(4, i)) as usize] = fRec338_tmp[(i32::wrapping_add(4, i)) as usize];
				self.fYec79[((i32::wrapping_add(i, self.fYec79_idx)) & 16383) as usize] = fZec510[i as usize] + fZec508[i as usize];
				fYec80_tmp[(i32::wrapping_add(4, i)) as usize] = self.fYec79[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec79_idx), iZec516[i as usize])) & 16383) as usize];
				fRec390_tmp[(i32::wrapping_add(4, i)) as usize] = 0.70710677 * (fZec515[i as usize] * fYec80_tmp[(i32::wrapping_add(4, i)) as usize] / fZec514[i as usize] + fYec80_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) - fZec515[i as usize] * fRec390_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] / fZec514[i as usize];
				fRec337_tmp[(i32::wrapping_add(4, i)) as usize] = fRec390_tmp[(i32::wrapping_add(4, i)) as usize];
				fZec521[i as usize] = 0.760314 * fRec337_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + 0.64955574 * fZec507[i as usize];
				fZec522[i as usize] = 0.760314 * fZec521[i as usize] - 0.64955574 * fRec333_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fZec523[i as usize] = 0.760314 * fRec336_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + 0.64955574 * fZec509[i as usize];
				fZec524[i as usize] = 0.760314 * fZec523[i as usize] - 0.64955574 * fRec332_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				self.fYec81[((i32::wrapping_add(i, self.fYec81_idx)) & 16383) as usize] = fZec524[i as usize] - fZec522[i as usize];
				fYec82_tmp[(i32::wrapping_add(4, i)) as usize] = self.fYec81[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec81_idx), std::cmp::min(8192, std::cmp::max(0, (fZec517[i as usize]) as i32)))) & 16383) as usize];
				fRec334_tmp[(i32::wrapping_add(4, i)) as usize] = 0.70710677 * (fZec520[i as usize] * fYec82_tmp[(i32::wrapping_add(4, i)) as usize] / fZec519[i as usize] + fYec82_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) - fRec334_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] * fZec520[i as usize] / fZec519[i as usize];
				fRec332_tmp[(i32::wrapping_add(4, i)) as usize] = fRec334_tmp[(i32::wrapping_add(4, i)) as usize];
				self.fYec83[((i32::wrapping_add(i, self.fYec83_idx)) & 16383) as usize] = fZec524[i as usize] + fZec522[i as usize];
				fYec84_tmp[(i32::wrapping_add(4, i)) as usize] = self.fYec83[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec83_idx), iZec529[i as usize])) & 16383) as usize];
				fRec391_tmp[(i32::wrapping_add(4, i)) as usize] = 0.70710677 * (fZec528[i as usize] * fYec84_tmp[(i32::wrapping_add(4, i)) as usize] / fZec527[i as usize] + fYec84_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) - fRec391_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] * fZec528[i as usize] / fZec527[i as usize];
				fRec333_tmp[(i32::wrapping_add(4, i)) as usize] = fRec391_tmp[(i32::wrapping_add(4, i)) as usize];
				self.fYec85[((i32::wrapping_add(i, self.fYec85_idx)) & 16383) as usize] = 0.760314 * fRec332_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + 0.64955574 * fZec523[i as usize];
				self.fYec86[((i32::wrapping_add(i, self.fYec86_idx)) & 16383) as usize] = self.fYec85[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec85_idx), std::cmp::min(8192, iZec557[i as usize]))) & 16383) as usize] * fZec556[i as usize] * fZec555[i as usize] * fZec554[i as usize] * fZec553[i as usize] + fZec552[i as usize] * (self.fYec85[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec85_idx), std::cmp::min(8192, iZec551[i as usize]))) & 16383) as usize] * fZec550[i as usize] * fZec549[i as usize] * fZec548[i as usize] + 0.5 * fZec539[i as usize] * self.fYec85[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec85_idx), std::cmp::min(8192, iZec547[i as usize]))) & 16383) as usize] * fZec546[i as usize] * fZec545[i as usize] + 0.16666667 * fZec540[i as usize] * self.fYec85[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec85_idx), std::cmp::min(8192, iZec544[i as usize]))) & 16383) as usize] * fZec543[i as usize] + 0.041666668 * fZec541[i as usize] * self.fYec85[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec85_idx), std::cmp::min(8192, iZec535[i as usize]))) & 16383) as usize]);
				fYec87_tmp[(i32::wrapping_add(4, i)) as usize] = self.fYec86[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec86_idx), std::cmp::min(8192, std::cmp::max(0, (fZec530[i as usize]) as i32)))) & 16383) as usize];
				fRec331_tmp[(i32::wrapping_add(4, i)) as usize] = fYec87_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] - (fZec531[i as usize] + (2.0 - fRec393_tmp[(i32::wrapping_add(4, i)) as usize])) * (fRec331_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] - fYec87_tmp[(i32::wrapping_add(4, i)) as usize]) / (fRec393_tmp[(i32::wrapping_add(4, i)) as usize] - fZec531[i as usize]);
				fRec330_tmp[(i32::wrapping_add(4, i)) as usize] = self.fConst24 * fRec331_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] - self.fConst23 * (self.fConst21 * fRec330_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] - self.fConst20 * fRec331_tmp[(i32::wrapping_add(4, i)) as usize]);
				fRec329_tmp[(i32::wrapping_add(4, i)) as usize] = fRec330_tmp[(i32::wrapping_add(4, i)) as usize] - self.fConst29 * (self.fConst28 * fRec329_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + self.fConst27 * fRec329_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]);
				fRec328_tmp[(i32::wrapping_add(4, i)) as usize] = self.fConst29 * (self.fConst26 * fRec329_tmp[(i32::wrapping_add(4, i)) as usize] + self.fConst32 * fRec329_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + self.fConst26 * fRec329_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize]) - self.fConst31 * (self.fConst30 * fRec328_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + self.fConst27 * fRec328_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]);
				fZec558[i as usize] = self.fConst36 * fRec327_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fRec327_tmp[(i32::wrapping_add(4, i)) as usize] = self.fConst31 * (self.fConst32 * fRec328_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + self.fConst26 * fRec328_tmp[(i32::wrapping_add(4, i)) as usize] + self.fConst26 * fRec328_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize]) - self.fConst39 * (self.fConst38 * fRec327_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + fZec558[i as usize]);
				fRec399_tmp[(i32::wrapping_add(4, i)) as usize] = self.fConst23 * (fRec331_tmp[(i32::wrapping_add(4, i)) as usize] + fRec331_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] - self.fConst21 * fRec399_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]);
				fRec398_tmp[(i32::wrapping_add(4, i)) as usize] = fRec399_tmp[(i32::wrapping_add(4, i)) as usize] - self.fConst29 * (self.fConst28 * fRec398_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + self.fConst27 * fRec398_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]);
				fRec397_tmp[(i32::wrapping_add(4, i)) as usize] = self.fConst29 * (fRec398_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + fRec398_tmp[(i32::wrapping_add(4, i)) as usize] + 2.0 * fRec398_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) - self.fConst31 * (self.fConst30 * fRec397_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + self.fConst27 * fRec397_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]);
				fYec88_tmp[(i32::wrapping_add(4, i)) as usize] = fRec397_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + fRec397_tmp[(i32::wrapping_add(4, i)) as usize] + 2.0 * fRec397_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fRec396_tmp[(i32::wrapping_add(4, i)) as usize] = self.fConst31 * (self.fConst43 * fYec88_tmp[(i32::wrapping_add(4, i)) as usize] + self.fConst44 * fYec88_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) - self.fConst42 * fRec396_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fRec395_tmp[(i32::wrapping_add(4, i)) as usize] = fRec396_tmp[(i32::wrapping_add(4, i)) as usize] - self.fConst46 * (self.fConst45 * fRec395_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + self.fConst36 * fRec395_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]);
				fRec394_tmp[(i32::wrapping_add(4, i)) as usize] = self.fConst46 * (self.fConst49 * fRec395_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + self.fConst35 * fRec395_tmp[(i32::wrapping_add(4, i)) as usize] + self.fConst35 * fRec395_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize]) - self.fConst48 * (self.fConst47 * fRec394_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + self.fConst36 * fRec394_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]);
				fRec402_tmp[(i32::wrapping_add(4, i)) as usize] = 0.0 - self.fConst50 * (self.fConst41 * fRec402_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] - self.fConst31 * (fYec88_tmp[(i32::wrapping_add(4, i)) as usize] + fYec88_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]));
				fRec401_tmp[(i32::wrapping_add(4, i)) as usize] = fRec402_tmp[(i32::wrapping_add(4, i)) as usize] - self.fConst46 * (self.fConst45 * fRec401_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + self.fConst36 * fRec401_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]);
				fRec400_tmp[(i32::wrapping_add(4, i)) as usize] = self.fConst46 * (fRec401_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + fRec401_tmp[(i32::wrapping_add(4, i)) as usize] + 2.0 * fRec401_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) - self.fConst48 * (self.fConst47 * fRec400_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + self.fConst36 * fRec400_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]);
				self.fYec89[((i32::wrapping_add(i, self.fYec89_idx)) & 1023) as usize] = fZec560[i as usize] + fZec559[i as usize] + fSlow105 * (fRec327_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + self.fConst39 * (fZec558[i as usize] + self.fConst38 * fRec327_tmp[(i32::wrapping_add(4, i)) as usize]) + self.fConst48 * (self.fConst49 * fRec394_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + self.fConst35 * fRec394_tmp[(i32::wrapping_add(4, i)) as usize] + self.fConst35 * fRec394_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + fRec400_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + fRec400_tmp[(i32::wrapping_add(4, i)) as usize] + 2.0 * fRec400_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]));
				fRec18_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow106 * (fZec556[i as usize] * fZec555[i as usize] * fZec554[i as usize] * fZec553[i as usize] * self.fYec89[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec89_idx), std::cmp::min(512, iZec557[i as usize]))) & 1023) as usize] + fZec552[i as usize] * (fZec550[i as usize] * fZec549[i as usize] * fZec548[i as usize] * self.fYec89[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec89_idx), std::cmp::min(512, iZec551[i as usize]))) & 1023) as usize] + 0.5 * fZec539[i as usize] * fZec546[i as usize] * fZec545[i as usize] * self.fYec89[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec89_idx), std::cmp::min(512, iZec547[i as usize]))) & 1023) as usize] + 0.16666667 * fZec540[i as usize] * fZec543[i as usize] * self.fYec89[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec89_idx), std::cmp::min(512, iZec544[i as usize]))) & 1023) as usize] + 0.041666668 * fZec541[i as usize] * self.fYec89[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec89_idx), std::cmp::min(512, iZec535[i as usize]))) & 1023) as usize])) + fSlow104 * fRec18_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				self.fYec90[((i32::wrapping_add(i, self.fYec90_idx)) & 16383) as usize] = 0.760314 * fRec333_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + 0.64955574 * fZec521[i as usize];
				self.fYec91[((i32::wrapping_add(i, self.fYec91_idx)) & 16383) as usize] = self.fYec90[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec90_idx), std::cmp::min(8192, std::cmp::max(0, iZec565[i as usize])))) & 16383) as usize] * (0.0 - fZec569[i as usize]) * (0.0 - 0.5 * fZec568[i as usize]) * (0.0 - 0.33333334 * fZec567[i as usize]) * (0.0 - 0.25 * fZec571[i as usize]) + (fSlow92 + fZec563[i as usize] + (5.0 - fZec566[i as usize])) * (self.fYec90[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec90_idx), std::cmp::min(8192, std::cmp::max(0, i32::wrapping_add(iZec565[i as usize], 1))))) & 16383) as usize] * (0.0 - fZec568[i as usize]) * (0.0 - 0.5 * fZec567[i as usize]) * (0.0 - 0.33333334 * fZec571[i as usize]) + 0.5 * fZec569[i as usize] * self.fYec90[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec90_idx), std::cmp::min(8192, std::cmp::max(0, i32::wrapping_add(iZec565[i as usize], 2))))) & 16383) as usize] * (0.0 - fZec567[i as usize]) * (0.0 - 0.5 * fZec571[i as usize]) + 0.16666667 * fZec570[i as usize] * self.fYec90[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec90_idx), std::cmp::min(8192, std::cmp::max(0, i32::wrapping_add(iZec565[i as usize], 3))))) & 16383) as usize] * (0.0 - fZec571[i as usize]) + 0.041666668 * fZec570[i as usize] * fZec567[i as usize] * self.fYec90[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec90_idx), std::cmp::min(8192, std::cmp::max(0, i32::wrapping_add(iZec565[i as usize], 4))))) & 16383) as usize]);
				fYec92_tmp[(i32::wrapping_add(4, i)) as usize] = self.fYec91[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec91_idx), std::cmp::min(8192, std::cmp::max(0, (fZec561[i as usize]) as i32)))) & 16383) as usize];
				fRec408_tmp[(i32::wrapping_add(4, i)) as usize] = fYec92_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] - (fZec562[i as usize] + (2.0 - fRec409_tmp[(i32::wrapping_add(4, i)) as usize])) * (fRec408_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] - fYec92_tmp[(i32::wrapping_add(4, i)) as usize]) / (fRec409_tmp[(i32::wrapping_add(4, i)) as usize] - fZec562[i as usize]);
				fRec407_tmp[(i32::wrapping_add(4, i)) as usize] = self.fConst24 * fRec408_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] - self.fConst23 * (self.fConst21 * fRec407_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] - self.fConst20 * fRec408_tmp[(i32::wrapping_add(4, i)) as usize]);
				fRec406_tmp[(i32::wrapping_add(4, i)) as usize] = fRec407_tmp[(i32::wrapping_add(4, i)) as usize] - self.fConst29 * (self.fConst28 * fRec406_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + self.fConst27 * fRec406_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]);
				fRec405_tmp[(i32::wrapping_add(4, i)) as usize] = self.fConst29 * (self.fConst32 * fRec406_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + self.fConst26 * fRec406_tmp[(i32::wrapping_add(4, i)) as usize] + self.fConst26 * fRec406_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize]) - self.fConst31 * (self.fConst30 * fRec405_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + self.fConst27 * fRec405_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]);
				fZec572[i as usize] = self.fConst36 * fRec404_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fRec404_tmp[(i32::wrapping_add(4, i)) as usize] = self.fConst31 * (self.fConst26 * fRec405_tmp[(i32::wrapping_add(4, i)) as usize] + self.fConst32 * fRec405_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + self.fConst26 * fRec405_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize]) - self.fConst39 * (self.fConst38 * fRec404_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + fZec572[i as usize]);
				fRec415_tmp[(i32::wrapping_add(4, i)) as usize] = self.fConst23 * (fRec408_tmp[(i32::wrapping_add(4, i)) as usize] + fRec408_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] - self.fConst21 * fRec415_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]);
				fRec414_tmp[(i32::wrapping_add(4, i)) as usize] = fRec415_tmp[(i32::wrapping_add(4, i)) as usize] - self.fConst29 * (self.fConst28 * fRec414_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + self.fConst27 * fRec414_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]);
				fRec413_tmp[(i32::wrapping_add(4, i)) as usize] = self.fConst29 * (fRec414_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + fRec414_tmp[(i32::wrapping_add(4, i)) as usize] + 2.0 * fRec414_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) - self.fConst31 * (self.fConst30 * fRec413_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + self.fConst27 * fRec413_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]);
				fYec93_tmp[(i32::wrapping_add(4, i)) as usize] = fRec413_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + fRec413_tmp[(i32::wrapping_add(4, i)) as usize] + 2.0 * fRec413_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fRec412_tmp[(i32::wrapping_add(4, i)) as usize] = self.fConst31 * (self.fConst44 * fYec93_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + self.fConst43 * fYec93_tmp[(i32::wrapping_add(4, i)) as usize]) - self.fConst42 * fRec412_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fRec411_tmp[(i32::wrapping_add(4, i)) as usize] = fRec412_tmp[(i32::wrapping_add(4, i)) as usize] - self.fConst46 * (self.fConst45 * fRec411_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + self.fConst36 * fRec411_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]);
				fRec410_tmp[(i32::wrapping_add(4, i)) as usize] = self.fConst46 * (self.fConst35 * fRec411_tmp[(i32::wrapping_add(4, i)) as usize] + self.fConst49 * fRec411_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + self.fConst35 * fRec411_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize]) - self.fConst48 * (self.fConst47 * fRec410_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + self.fConst36 * fRec410_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]);
				fRec418_tmp[(i32::wrapping_add(4, i)) as usize] = self.fConst50 * (self.fConst31 * (fYec93_tmp[(i32::wrapping_add(4, i)) as usize] + fYec93_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) - self.fConst41 * fRec418_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]);
				fRec417_tmp[(i32::wrapping_add(4, i)) as usize] = fRec418_tmp[(i32::wrapping_add(4, i)) as usize] - self.fConst46 * (self.fConst45 * fRec417_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + self.fConst36 * fRec417_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]);
				fRec416_tmp[(i32::wrapping_add(4, i)) as usize] = self.fConst46 * (fRec417_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + fRec417_tmp[(i32::wrapping_add(4, i)) as usize] + 2.0 * fRec417_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) - self.fConst48 * (self.fConst47 * fRec416_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + self.fConst36 * fRec416_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]);
				self.fYec94[((i32::wrapping_add(i, self.fYec94_idx)) & 1023) as usize] = fZec573[i as usize] + fSlow105 * (fRec404_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + self.fConst39 * (fZec572[i as usize] + self.fConst38 * fRec404_tmp[(i32::wrapping_add(4, i)) as usize]) + self.fConst48 * (self.fConst49 * fRec410_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + self.fConst35 * fRec410_tmp[(i32::wrapping_add(4, i)) as usize] + self.fConst35 * fRec410_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + fRec416_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + fRec416_tmp[(i32::wrapping_add(4, i)) as usize] + 2.0 * fRec416_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]));
				fRec403_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow106 * (fZec454[i as usize] * fZec453[i as usize] * fZec452[i as usize] * fZec451[i as usize] * self.fYec94[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec94_idx), iZec455[i as usize])) & 1023) as usize] + fZec450[i as usize] * (fZec448[i as usize] * fZec447[i as usize] * fZec446[i as usize] * self.fYec94[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec94_idx), iZec449[i as usize])) & 1023) as usize] + 0.5 * fZec437[i as usize] * fZec444[i as usize] * fZec443[i as usize] * self.fYec94[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec94_idx), iZec445[i as usize])) & 1023) as usize] + 0.16666667 * fZec438[i as usize] * fZec441[i as usize] * self.fYec94[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec94_idx), iZec442[i as usize])) & 1023) as usize] + 0.041666668 * fZec439[i as usize] * self.fYec94[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec94_idx), iZec433[i as usize])) & 1023) as usize])) + fSlow104 * fRec403_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fZec574[i as usize] = fSlow111 * fRec403_tmp[(i32::wrapping_add(4, i)) as usize] - fSlow110 * fRec15_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fZec575[i as usize] = fSlow111 * fRec18_tmp[(i32::wrapping_add(4, i)) as usize] - fSlow110 * fRec14_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				self.fYec95[((i32::wrapping_add(i, self.fYec95_idx)) & 16383) as usize] = fZec575[i as usize] - fZec574[i as usize];
				fYec96_tmp[(i32::wrapping_add(4, i)) as usize] = self.fYec95[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec95_idx), iZec489[i as usize])) & 16383) as usize];
				fRec16_tmp[(i32::wrapping_add(4, i)) as usize] = 0.70710677 * (fZec488[i as usize] * fYec96_tmp[(i32::wrapping_add(4, i)) as usize] / fZec487[i as usize] + fYec96_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) - fRec16_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] * fZec488[i as usize] / fZec487[i as usize];
				fRec14_tmp[(i32::wrapping_add(4, i)) as usize] = fRec16_tmp[(i32::wrapping_add(4, i)) as usize];
				self.fYec97[((i32::wrapping_add(i, self.fYec97_idx)) & 16383) as usize] = fZec575[i as usize] + fZec574[i as usize];
				fYec98_tmp[(i32::wrapping_add(4, i)) as usize] = self.fYec97[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec97_idx), std::cmp::min(8192, std::cmp::max(0, (fZec576[i as usize]) as i32)))) & 16383) as usize];
				fRec419_tmp[(i32::wrapping_add(4, i)) as usize] = 0.70710677 * (fZec579[i as usize] * fYec98_tmp[(i32::wrapping_add(4, i)) as usize] / fZec578[i as usize] + fYec98_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) - fRec419_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] * fZec579[i as usize] / fZec578[i as usize];
				fRec15_tmp[(i32::wrapping_add(4, i)) as usize] = fRec419_tmp[(i32::wrapping_add(4, i)) as usize];
				fZec580[i as usize] = fSlow111 * fRec15_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + fSlow110 * fRec403_tmp[(i32::wrapping_add(4, i)) as usize];
				fZec581[i as usize] = fSlow111 * fZec580[i as usize] - fSlow110 * fRec11_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fZec582[i as usize] = fSlow111 * fRec14_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + fSlow110 * fRec18_tmp[(i32::wrapping_add(4, i)) as usize];
				fZec583[i as usize] = fSlow111 * fZec582[i as usize] - fSlow110 * fRec10_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				self.fYec99[((i32::wrapping_add(i, self.fYec99_idx)) & 16383) as usize] = fZec583[i as usize] - fZec581[i as usize];
				fYec100_tmp[(i32::wrapping_add(4, i)) as usize] = self.fYec99[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec99_idx), iZec511[i as usize])) & 16383) as usize];
				fRec12_tmp[(i32::wrapping_add(4, i)) as usize] = 0.70710677 * (fZec506[i as usize] * fYec100_tmp[(i32::wrapping_add(4, i)) as usize] / fZec505[i as usize] + fYec100_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) - fRec12_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] * fZec506[i as usize] / fZec505[i as usize];
				fRec10_tmp[(i32::wrapping_add(4, i)) as usize] = fRec12_tmp[(i32::wrapping_add(4, i)) as usize];
				self.fYec101[((i32::wrapping_add(i, self.fYec101_idx)) & 16383) as usize] = fZec583[i as usize] + fZec581[i as usize];
				fYec102_tmp[(i32::wrapping_add(4, i)) as usize] = self.fYec101[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec101_idx), iZec502[i as usize])) & 16383) as usize];
				fRec421_tmp[(i32::wrapping_add(4, i)) as usize] = 0.70710677 * (fZec501[i as usize] * fYec102_tmp[(i32::wrapping_add(4, i)) as usize] / fZec500[i as usize] + fYec102_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) - fZec501[i as usize] * fRec421_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] / fZec500[i as usize];
				fRec11_tmp[(i32::wrapping_add(4, i)) as usize] = fRec421_tmp[(i32::wrapping_add(4, i)) as usize];
				fZec584[i as usize] = fSlow111 * fRec11_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + fSlow110 * fZec580[i as usize];
				fZec585[i as usize] = fSlow111 * fZec584[i as usize] - fSlow110 * fRec7_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fZec586[i as usize] = fSlow111 * fRec10_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + fSlow110 * fZec582[i as usize];
				fZec587[i as usize] = fSlow111 * fZec586[i as usize] - fSlow110 * fRec6_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				self.fYec103[((i32::wrapping_add(i, self.fYec103_idx)) & 16383) as usize] = fZec587[i as usize] - fZec585[i as usize];
				fYec104_tmp[(i32::wrapping_add(4, i)) as usize] = self.fYec103[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec103_idx), iZec516[i as usize])) & 16383) as usize];
				fRec8_tmp[(i32::wrapping_add(4, i)) as usize] = 0.70710677 * (fZec515[i as usize] * fYec104_tmp[(i32::wrapping_add(4, i)) as usize] / fZec514[i as usize] + fYec104_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) - fRec8_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] * fZec515[i as usize] / fZec514[i as usize];
				fRec6_tmp[(i32::wrapping_add(4, i)) as usize] = fRec8_tmp[(i32::wrapping_add(4, i)) as usize];
				self.fYec105[((i32::wrapping_add(i, self.fYec105_idx)) & 16383) as usize] = fZec587[i as usize] + fZec585[i as usize];
				fYec106_tmp[(i32::wrapping_add(4, i)) as usize] = self.fYec105[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec105_idx), std::cmp::min(8192, std::cmp::max(0, (fZec588[i as usize]) as i32)))) & 16383) as usize];
				fRec422_tmp[(i32::wrapping_add(4, i)) as usize] = 0.70710677 * (fZec591[i as usize] * fYec106_tmp[(i32::wrapping_add(4, i)) as usize] / fZec590[i as usize] + fYec106_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) - fRec422_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] * fZec591[i as usize] / fZec590[i as usize];
				fRec7_tmp[(i32::wrapping_add(4, i)) as usize] = fRec422_tmp[(i32::wrapping_add(4, i)) as usize];
				fZec594[i as usize] = fSlow111 * fRec6_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + fSlow110 * fZec586[i as usize];
				fZec595[i as usize] = fSlow111 * fZec594[i as usize] - fSlow110 * fRec3_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fZec596[i as usize] = fSlow111 * fRec7_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + fSlow110 * fZec584[i as usize];
				fZec597[i as usize] = fSlow111 * fZec596[i as usize] - fSlow110 * fRec4_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				self.fYec107[((i32::wrapping_add(i, self.fYec107_idx)) & 16383) as usize] = 0.0 - 0.70710677 * (fZec597[i as usize] - fZec595[i as usize]);
				fYec108_tmp[(i32::wrapping_add(4, i)) as usize] = self.fYec107[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec107_idx), std::cmp::min(8192, std::cmp::max(0, (fZec592[i as usize]) as i32)))) & 16383) as usize];
				fRec5_tmp[(i32::wrapping_add(4, i)) as usize] = fYec108_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] - (fZec593[i as usize] + (2.0 - fRec424_tmp[(i32::wrapping_add(4, i)) as usize])) * (fRec5_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] - fYec108_tmp[(i32::wrapping_add(4, i)) as usize]) / (fRec424_tmp[(i32::wrapping_add(4, i)) as usize] - fZec593[i as usize]);
				fRec3_tmp[(i32::wrapping_add(4, i)) as usize] = fRec5_tmp[(i32::wrapping_add(4, i)) as usize];
				self.fYec109[((i32::wrapping_add(i, self.fYec109_idx)) & 16383) as usize] = fZec595[i as usize] + fZec597[i as usize];
				fYec110_tmp[(i32::wrapping_add(4, i)) as usize] = self.fYec109[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec109_idx), iZec529[i as usize])) & 16383) as usize];
				fRec425_tmp[(i32::wrapping_add(4, i)) as usize] = 0.70710677 * (fZec528[i as usize] * fYec110_tmp[(i32::wrapping_add(4, i)) as usize] / fZec527[i as usize] + fYec110_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]) - fZec528[i as usize] * fRec425_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] / fZec527[i as usize];
				fRec4_tmp[(i32::wrapping_add(4, i)) as usize] = fRec425_tmp[(i32::wrapping_add(4, i)) as usize];
				fRec1_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow111 * fRec3_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + fSlow110 * fZec594[i as usize];
				fRec2_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow111 * fRec4_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + fSlow110 * fZec596[i as usize];
			}
			/* Post code */
			self.fYec109_idx_save = vsize;
			for j715 in 0..4 {
				self.fYec110_perm[j715 as usize] = fYec110_tmp[(i32::wrapping_add(vsize, j715)) as usize];
			}
			for j717 in 0..4 {
				self.fRec425_perm[j717 as usize] = fRec425_tmp[(i32::wrapping_add(vsize, j717)) as usize];
			}
			self.fYec107_idx_save = vsize;
			for j709 in 0..4 {
				self.fYec108_perm[j709 as usize] = fYec108_tmp[(i32::wrapping_add(vsize, j709)) as usize];
			}
			self.fYec105_idx_save = vsize;
			for j701 in 0..4 {
				self.fYec106_perm[j701 as usize] = fYec106_tmp[(i32::wrapping_add(vsize, j701)) as usize];
			}
			for j703 in 0..4 {
				self.fRec422_perm[j703 as usize] = fRec422_tmp[(i32::wrapping_add(vsize, j703)) as usize];
			}
			self.fYec103_idx_save = vsize;
			for j693 in 0..4 {
				self.fYec104_perm[j693 as usize] = fYec104_tmp[(i32::wrapping_add(vsize, j693)) as usize];
			}
			self.fYec101_idx_save = vsize;
			for j687 in 0..4 {
				self.fYec102_perm[j687 as usize] = fYec102_tmp[(i32::wrapping_add(vsize, j687)) as usize];
			}
			for j689 in 0..4 {
				self.fRec421_perm[j689 as usize] = fRec421_tmp[(i32::wrapping_add(vsize, j689)) as usize];
			}
			self.fYec99_idx_save = vsize;
			for j681 in 0..4 {
				self.fYec100_perm[j681 as usize] = fYec100_tmp[(i32::wrapping_add(vsize, j681)) as usize];
			}
			self.fYec97_idx_save = vsize;
			for j675 in 0..4 {
				self.fYec98_perm[j675 as usize] = fYec98_tmp[(i32::wrapping_add(vsize, j675)) as usize];
			}
			for j677 in 0..4 {
				self.fRec419_perm[j677 as usize] = fRec419_tmp[(i32::wrapping_add(vsize, j677)) as usize];
			}
			self.fYec95_idx_save = vsize;
			for j667 in 0..4 {
				self.fYec96_perm[j667 as usize] = fYec96_tmp[(i32::wrapping_add(vsize, j667)) as usize];
			}
			self.fYec94_idx_save = vsize;
			for j659 in 0..4 {
				self.fRec418_perm[j659 as usize] = fRec418_tmp[(i32::wrapping_add(vsize, j659)) as usize];
			}
			for j661 in 0..4 {
				self.fRec417_perm[j661 as usize] = fRec417_tmp[(i32::wrapping_add(vsize, j661)) as usize];
			}
			for j663 in 0..4 {
				self.fRec416_perm[j663 as usize] = fRec416_tmp[(i32::wrapping_add(vsize, j663)) as usize];
			}
			for j651 in 0..4 {
				self.fYec93_perm[j651 as usize] = fYec93_tmp[(i32::wrapping_add(vsize, j651)) as usize];
			}
			for j645 in 0..4 {
				self.fRec415_perm[j645 as usize] = fRec415_tmp[(i32::wrapping_add(vsize, j645)) as usize];
			}
			for j647 in 0..4 {
				self.fRec414_perm[j647 as usize] = fRec414_tmp[(i32::wrapping_add(vsize, j647)) as usize];
			}
			for j649 in 0..4 {
				self.fRec413_perm[j649 as usize] = fRec413_tmp[(i32::wrapping_add(vsize, j649)) as usize];
			}
			for j653 in 0..4 {
				self.fRec412_perm[j653 as usize] = fRec412_tmp[(i32::wrapping_add(vsize, j653)) as usize];
			}
			for j655 in 0..4 {
				self.fRec411_perm[j655 as usize] = fRec411_tmp[(i32::wrapping_add(vsize, j655)) as usize];
			}
			for j657 in 0..4 {
				self.fRec410_perm[j657 as usize] = fRec410_tmp[(i32::wrapping_add(vsize, j657)) as usize];
			}
			self.fYec90_idx_save = vsize;
			self.fYec91_idx_save = vsize;
			for j633 in 0..4 {
				self.fYec92_perm[j633 as usize] = fYec92_tmp[(i32::wrapping_add(vsize, j633)) as usize];
			}
			for j635 in 0..4 {
				self.fRec408_perm[j635 as usize] = fRec408_tmp[(i32::wrapping_add(vsize, j635)) as usize];
			}
			for j637 in 0..4 {
				self.fRec407_perm[j637 as usize] = fRec407_tmp[(i32::wrapping_add(vsize, j637)) as usize];
			}
			for j639 in 0..4 {
				self.fRec406_perm[j639 as usize] = fRec406_tmp[(i32::wrapping_add(vsize, j639)) as usize];
			}
			for j641 in 0..4 {
				self.fRec405_perm[j641 as usize] = fRec405_tmp[(i32::wrapping_add(vsize, j641)) as usize];
			}
			for j643 in 0..4 {
				self.fRec404_perm[j643 as usize] = fRec404_tmp[(i32::wrapping_add(vsize, j643)) as usize];
			}
			for j665 in 0..4 {
				self.fRec403_perm[j665 as usize] = fRec403_tmp[(i32::wrapping_add(vsize, j665)) as usize];
			}
			self.fYec89_idx_save = vsize;
			for j623 in 0..4 {
				self.fRec402_perm[j623 as usize] = fRec402_tmp[(i32::wrapping_add(vsize, j623)) as usize];
			}
			for j625 in 0..4 {
				self.fRec401_perm[j625 as usize] = fRec401_tmp[(i32::wrapping_add(vsize, j625)) as usize];
			}
			for j627 in 0..4 {
				self.fRec400_perm[j627 as usize] = fRec400_tmp[(i32::wrapping_add(vsize, j627)) as usize];
			}
			for j615 in 0..4 {
				self.fYec88_perm[j615 as usize] = fYec88_tmp[(i32::wrapping_add(vsize, j615)) as usize];
			}
			for j609 in 0..4 {
				self.fRec399_perm[j609 as usize] = fRec399_tmp[(i32::wrapping_add(vsize, j609)) as usize];
			}
			for j611 in 0..4 {
				self.fRec398_perm[j611 as usize] = fRec398_tmp[(i32::wrapping_add(vsize, j611)) as usize];
			}
			for j613 in 0..4 {
				self.fRec397_perm[j613 as usize] = fRec397_tmp[(i32::wrapping_add(vsize, j613)) as usize];
			}
			for j617 in 0..4 {
				self.fRec396_perm[j617 as usize] = fRec396_tmp[(i32::wrapping_add(vsize, j617)) as usize];
			}
			for j619 in 0..4 {
				self.fRec395_perm[j619 as usize] = fRec395_tmp[(i32::wrapping_add(vsize, j619)) as usize];
			}
			for j621 in 0..4 {
				self.fRec394_perm[j621 as usize] = fRec394_tmp[(i32::wrapping_add(vsize, j621)) as usize];
			}
			self.fYec85_idx_save = vsize;
			self.fYec86_idx_save = vsize;
			for j597 in 0..4 {
				self.fYec87_perm[j597 as usize] = fYec87_tmp[(i32::wrapping_add(vsize, j597)) as usize];
			}
			self.fYec83_idx_save = vsize;
			for j589 in 0..4 {
				self.fYec84_perm[j589 as usize] = fYec84_tmp[(i32::wrapping_add(vsize, j589)) as usize];
			}
			for j591 in 0..4 {
				self.fRec391_perm[j591 as usize] = fRec391_tmp[(i32::wrapping_add(vsize, j591)) as usize];
			}
			self.fYec81_idx_save = vsize;
			for j581 in 0..4 {
				self.fYec82_perm[j581 as usize] = fYec82_tmp[(i32::wrapping_add(vsize, j581)) as usize];
			}
			self.fYec79_idx_save = vsize;
			for j575 in 0..4 {
				self.fYec80_perm[j575 as usize] = fYec80_tmp[(i32::wrapping_add(vsize, j575)) as usize];
			}
			for j577 in 0..4 {
				self.fRec390_perm[j577 as usize] = fRec390_tmp[(i32::wrapping_add(vsize, j577)) as usize];
			}
			self.fYec77_idx_save = vsize;
			for j569 in 0..4 {
				self.fYec78_perm[j569 as usize] = fYec78_tmp[(i32::wrapping_add(vsize, j569)) as usize];
			}
			self.fYec75_idx_save = vsize;
			for j563 in 0..4 {
				self.fYec76_perm[j563 as usize] = fYec76_tmp[(i32::wrapping_add(vsize, j563)) as usize];
			}
			for j565 in 0..4 {
				self.fRec388_perm[j565 as usize] = fRec388_tmp[(i32::wrapping_add(vsize, j565)) as usize];
			}
			self.fYec73_idx_save = vsize;
			for j555 in 0..4 {
				self.fYec74_perm[j555 as usize] = fYec74_tmp[(i32::wrapping_add(vsize, j555)) as usize];
			}
			self.fYec71_idx_save = vsize;
			for j549 in 0..4 {
				self.fYec72_perm[j549 as usize] = fYec72_tmp[(i32::wrapping_add(vsize, j549)) as usize];
			}
			for j551 in 0..4 {
				self.fRec387_perm[j551 as usize] = fRec387_tmp[(i32::wrapping_add(vsize, j551)) as usize];
			}
			self.fYec69_idx_save = vsize;
			for j543 in 0..4 {
				self.fYec70_perm[j543 as usize] = fYec70_tmp[(i32::wrapping_add(vsize, j543)) as usize];
			}
			self.fYec67_idx_save = vsize;
			for j537 in 0..4 {
				self.fYec68_perm[j537 as usize] = fYec68_tmp[(i32::wrapping_add(vsize, j537)) as usize];
			}
			for j539 in 0..4 {
				self.fRec385_perm[j539 as usize] = fRec385_tmp[(i32::wrapping_add(vsize, j539)) as usize];
			}
			self.fYec65_idx_save = vsize;
			for j529 in 0..4 {
				self.fYec66_perm[j529 as usize] = fYec66_tmp[(i32::wrapping_add(vsize, j529)) as usize];
			}
			self.fYec62_idx_save = vsize;
			self.fYec63_idx_save = vsize;
			for j525 in 0..4 {
				self.fYec64_perm[j525 as usize] = fYec64_tmp[(i32::wrapping_add(vsize, j525)) as usize];
			}
			for j527 in 0..4 {
				self.fRec383_perm[j527 as usize] = fRec383_tmp[(i32::wrapping_add(vsize, j527)) as usize];
			}
			self.fYec59_idx_save = vsize;
			self.fYec60_idx_save = vsize;
			for j519 in 0..4 {
				self.fYec61_perm[j519 as usize] = fYec61_tmp[(i32::wrapping_add(vsize, j519)) as usize];
			}
			self.fYec57_idx_save = vsize;
			for j511 in 0..4 {
				self.fYec58_perm[j511 as usize] = fYec58_tmp[(i32::wrapping_add(vsize, j511)) as usize];
			}
			for j513 in 0..4 {
				self.fRec380_perm[j513 as usize] = fRec380_tmp[(i32::wrapping_add(vsize, j513)) as usize];
			}
			self.fYec55_idx_save = vsize;
			for j503 in 0..4 {
				self.fYec56_perm[j503 as usize] = fYec56_tmp[(i32::wrapping_add(vsize, j503)) as usize];
			}
			self.fYec53_idx_save = vsize;
			for j495 in 0..4 {
				self.fYec54_perm[j495 as usize] = fYec54_tmp[(i32::wrapping_add(vsize, j495)) as usize];
			}
			for j497 in 0..4 {
				self.fRec377_perm[j497 as usize] = fRec377_tmp[(i32::wrapping_add(vsize, j497)) as usize];
			}
			self.fYec51_idx_save = vsize;
			for j487 in 0..4 {
				self.fYec52_perm[j487 as usize] = fYec52_tmp[(i32::wrapping_add(vsize, j487)) as usize];
			}
			self.fYec49_idx_save = vsize;
			for j481 in 0..4 {
				self.fYec50_perm[j481 as usize] = fYec50_tmp[(i32::wrapping_add(vsize, j481)) as usize];
			}
			for j483 in 0..4 {
				self.fRec375_perm[j483 as usize] = fRec375_tmp[(i32::wrapping_add(vsize, j483)) as usize];
			}
			self.fYec47_idx_save = vsize;
			for j473 in 0..4 {
				self.fYec48_perm[j473 as usize] = fYec48_tmp[(i32::wrapping_add(vsize, j473)) as usize];
			}
			self.fYec45_idx_save = vsize;
			for j467 in 0..4 {
				self.fYec46_perm[j467 as usize] = fYec46_tmp[(i32::wrapping_add(vsize, j467)) as usize];
			}
			for j469 in 0..4 {
				self.fRec373_perm[j469 as usize] = fRec373_tmp[(i32::wrapping_add(vsize, j469)) as usize];
			}
			self.fYec43_idx_save = vsize;
			for j459 in 0..4 {
				self.fYec44_perm[j459 as usize] = fYec44_tmp[(i32::wrapping_add(vsize, j459)) as usize];
			}
			self.fYec41_idx_save = vsize;
			for j451 in 0..4 {
				self.fYec42_perm[j451 as usize] = fYec42_tmp[(i32::wrapping_add(vsize, j451)) as usize];
			}
			for j453 in 0..4 {
				self.fRec370_perm[j453 as usize] = fRec370_tmp[(i32::wrapping_add(vsize, j453)) as usize];
			}
			self.fYec39_idx_save = vsize;
			for j443 in 0..4 {
				self.fYec40_perm[j443 as usize] = fYec40_tmp[(i32::wrapping_add(vsize, j443)) as usize];
			}
			for j445 in 0..4 {
				self.fRec368_perm[j445 as usize] = fRec368_tmp[(i32::wrapping_add(vsize, j445)) as usize];
			}
			for j447 in 0..4 {
				self.fRec366_perm[j447 as usize] = fRec366_tmp[(i32::wrapping_add(vsize, j447)) as usize];
			}
			for j455 in 0..4 {
				self.fRec367_perm[j455 as usize] = fRec367_tmp[(i32::wrapping_add(vsize, j455)) as usize];
			}
			for j461 in 0..4 {
				self.fRec365_perm[j461 as usize] = fRec365_tmp[(i32::wrapping_add(vsize, j461)) as usize];
			}
			for j463 in 0..4 {
				self.fRec363_perm[j463 as usize] = fRec363_tmp[(i32::wrapping_add(vsize, j463)) as usize];
			}
			for j471 in 0..4 {
				self.fRec364_perm[j471 as usize] = fRec364_tmp[(i32::wrapping_add(vsize, j471)) as usize];
			}
			for j475 in 0..4 {
				self.fRec361_perm[j475 as usize] = fRec361_tmp[(i32::wrapping_add(vsize, j475)) as usize];
			}
			for j477 in 0..4 {
				self.fRec359_perm[j477 as usize] = fRec359_tmp[(i32::wrapping_add(vsize, j477)) as usize];
			}
			for j485 in 0..4 {
				self.fRec360_perm[j485 as usize] = fRec360_tmp[(i32::wrapping_add(vsize, j485)) as usize];
			}
			for j489 in 0..4 {
				self.fRec357_perm[j489 as usize] = fRec357_tmp[(i32::wrapping_add(vsize, j489)) as usize];
			}
			for j491 in 0..4 {
				self.fRec355_perm[j491 as usize] = fRec355_tmp[(i32::wrapping_add(vsize, j491)) as usize];
			}
			for j499 in 0..4 {
				self.fRec356_perm[j499 as usize] = fRec356_tmp[(i32::wrapping_add(vsize, j499)) as usize];
			}
			for j505 in 0..4 {
				self.fRec354_perm[j505 as usize] = fRec354_tmp[(i32::wrapping_add(vsize, j505)) as usize];
			}
			for j507 in 0..4 {
				self.fRec352_perm[j507 as usize] = fRec352_tmp[(i32::wrapping_add(vsize, j507)) as usize];
			}
			for j515 in 0..4 {
				self.fRec353_perm[j515 as usize] = fRec353_tmp[(i32::wrapping_add(vsize, j515)) as usize];
			}
			for j521 in 0..4 {
				self.fRec351_perm[j521 as usize] = fRec351_tmp[(i32::wrapping_add(vsize, j521)) as usize];
			}
			for j531 in 0..4 {
				self.fRec349_perm[j531 as usize] = fRec349_tmp[(i32::wrapping_add(vsize, j531)) as usize];
			}
			for j533 in 0..4 {
				self.fRec347_perm[j533 as usize] = fRec347_tmp[(i32::wrapping_add(vsize, j533)) as usize];
			}
			for j541 in 0..4 {
				self.fRec348_perm[j541 as usize] = fRec348_tmp[(i32::wrapping_add(vsize, j541)) as usize];
			}
			for j545 in 0..4 {
				self.fRec345_perm[j545 as usize] = fRec345_tmp[(i32::wrapping_add(vsize, j545)) as usize];
			}
			for j547 in 0..4 {
				self.fRec343_perm[j547 as usize] = fRec343_tmp[(i32::wrapping_add(vsize, j547)) as usize];
			}
			for j553 in 0..4 {
				self.fRec344_perm[j553 as usize] = fRec344_tmp[(i32::wrapping_add(vsize, j553)) as usize];
			}
			for j557 in 0..4 {
				self.fRec341_perm[j557 as usize] = fRec341_tmp[(i32::wrapping_add(vsize, j557)) as usize];
			}
			for j559 in 0..4 {
				self.fRec339_perm[j559 as usize] = fRec339_tmp[(i32::wrapping_add(vsize, j559)) as usize];
			}
			for j567 in 0..4 {
				self.fRec340_perm[j567 as usize] = fRec340_tmp[(i32::wrapping_add(vsize, j567)) as usize];
			}
			for j571 in 0..4 {
				self.fRec338_perm[j571 as usize] = fRec338_tmp[(i32::wrapping_add(vsize, j571)) as usize];
			}
			for j573 in 0..4 {
				self.fRec336_perm[j573 as usize] = fRec336_tmp[(i32::wrapping_add(vsize, j573)) as usize];
			}
			for j579 in 0..4 {
				self.fRec337_perm[j579 as usize] = fRec337_tmp[(i32::wrapping_add(vsize, j579)) as usize];
			}
			for j583 in 0..4 {
				self.fRec334_perm[j583 as usize] = fRec334_tmp[(i32::wrapping_add(vsize, j583)) as usize];
			}
			for j585 in 0..4 {
				self.fRec332_perm[j585 as usize] = fRec332_tmp[(i32::wrapping_add(vsize, j585)) as usize];
			}
			for j593 in 0..4 {
				self.fRec333_perm[j593 as usize] = fRec333_tmp[(i32::wrapping_add(vsize, j593)) as usize];
			}
			for j599 in 0..4 {
				self.fRec331_perm[j599 as usize] = fRec331_tmp[(i32::wrapping_add(vsize, j599)) as usize];
			}
			for j601 in 0..4 {
				self.fRec330_perm[j601 as usize] = fRec330_tmp[(i32::wrapping_add(vsize, j601)) as usize];
			}
			for j603 in 0..4 {
				self.fRec329_perm[j603 as usize] = fRec329_tmp[(i32::wrapping_add(vsize, j603)) as usize];
			}
			for j605 in 0..4 {
				self.fRec328_perm[j605 as usize] = fRec328_tmp[(i32::wrapping_add(vsize, j605)) as usize];
			}
			for j607 in 0..4 {
				self.fRec327_perm[j607 as usize] = fRec327_tmp[(i32::wrapping_add(vsize, j607)) as usize];
			}
			for j629 in 0..4 {
				self.fRec18_perm[j629 as usize] = fRec18_tmp[(i32::wrapping_add(vsize, j629)) as usize];
			}
			for j669 in 0..4 {
				self.fRec16_perm[j669 as usize] = fRec16_tmp[(i32::wrapping_add(vsize, j669)) as usize];
			}
			for j671 in 0..4 {
				self.fRec14_perm[j671 as usize] = fRec14_tmp[(i32::wrapping_add(vsize, j671)) as usize];
			}
			for j679 in 0..4 {
				self.fRec15_perm[j679 as usize] = fRec15_tmp[(i32::wrapping_add(vsize, j679)) as usize];
			}
			for j683 in 0..4 {
				self.fRec12_perm[j683 as usize] = fRec12_tmp[(i32::wrapping_add(vsize, j683)) as usize];
			}
			for j685 in 0..4 {
				self.fRec10_perm[j685 as usize] = fRec10_tmp[(i32::wrapping_add(vsize, j685)) as usize];
			}
			for j691 in 0..4 {
				self.fRec11_perm[j691 as usize] = fRec11_tmp[(i32::wrapping_add(vsize, j691)) as usize];
			}
			for j695 in 0..4 {
				self.fRec8_perm[j695 as usize] = fRec8_tmp[(i32::wrapping_add(vsize, j695)) as usize];
			}
			for j697 in 0..4 {
				self.fRec6_perm[j697 as usize] = fRec6_tmp[(i32::wrapping_add(vsize, j697)) as usize];
			}
			for j705 in 0..4 {
				self.fRec7_perm[j705 as usize] = fRec7_tmp[(i32::wrapping_add(vsize, j705)) as usize];
			}
			for j711 in 0..4 {
				self.fRec5_perm[j711 as usize] = fRec5_tmp[(i32::wrapping_add(vsize, j711)) as usize];
			}
			for j713 in 0..4 {
				self.fRec3_perm[j713 as usize] = fRec3_tmp[(i32::wrapping_add(vsize, j713)) as usize];
			}
			for j719 in 0..4 {
				self.fRec4_perm[j719 as usize] = fRec4_tmp[(i32::wrapping_add(vsize, j719)) as usize];
			}
			for j721 in 0..4 {
				self.fRec1_perm[j721 as usize] = fRec1_tmp[(i32::wrapping_add(vsize, j721)) as usize];
			}
			for j723 in 0..4 {
				self.fRec2_perm[j723 as usize] = fRec2_tmp[(i32::wrapping_add(vsize, j723)) as usize];
			}
			/* Recursive loop 667 */
			/* Pre code */
			for j0 in 0..4 {
				fRec0_tmp[j0 as usize] = self.fRec0_perm[j0 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fRec0_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow0 + self.fConst2 * fRec0_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
			}
			/* Post code */
			for j1 in 0..4 {
				self.fRec0_perm[j1 as usize] = fRec0_tmp[(i32::wrapping_add(vsize, j1)) as usize];
			}
			/* Vectorizable loop 668 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec598[i as usize] = fRec0_tmp[(i32::wrapping_add(4, i)) as usize] * (fSlow118 * (fRec1_tmp[(i32::wrapping_add(4, i)) as usize] + fRec2_tmp[(i32::wrapping_add(4, i)) as usize]) + fSlow119 * fZec573[i as usize]);
			}
			/* Vectorizable loop 669 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				output0[i as usize] = fZec598[i as usize];
			}
			/* Vectorizable loop 670 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				output1[i as usize] = fZec598[i as usize];
			}
		}
	}

}


}
pub use dsp::mydsp as Instrument;
