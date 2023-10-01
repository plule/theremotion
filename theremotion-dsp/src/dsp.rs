mod dsp {
    /* ------------------------------------------------------------
author: "Pierre Lulé"
license: "BSD"
name: "theremotion"
version: "1.0"
Code generated with Faust 2.68.1 (https://faust.grame.fr)
Compilation options: -a /tmp/.tmpYmp5tR -lang rust -ct 1 -es 1 -mcd 16 -single -ftz 0
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



#[derive(Debug,Clone)]
pub struct mydspSIG0 {
	iRec4: [i32;2],
}

impl mydspSIG0 {
	
	fn get_num_inputsmydspSIG0(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsmydspSIG0(&self) -> i32 {
		return 1;
	}
	
	fn instance_initmydspSIG0(&mut self, sample_rate: i32) {
		for l2 in 0..2 {
			self.iRec4[l2 as usize] = 0;
		}
	}
	
	fn fillmydspSIG0(&mut self, count: i32, table: &mut[F32]) {
		for i1 in 0..count {
			self.iRec4[0] = i32::wrapping_add(self.iRec4[1], 1);
			table[i1 as usize] = 4.4e+02 * F32::powf(2.0, 0.083333336 * (0.062042013 * (i32::wrapping_add(self.iRec4[0], -1)) as F32 + -69.0));
			self.iRec4[1] = self.iRec4[0];
		}
	}

}


pub fn newmydspSIG0() -> mydspSIG0 { 
	mydspSIG0 {
		iRec4: [0;2],
	}
}
static mut imydspSIG1Wave0: [i32;1302] = [2,3,5,7,11,13,17,19,23,29,31,37,41,43,47,53,59,61,67,71,73,79,83,89,97,101,103,107,109,113,127,131,137,139,149,151,157,163,167,173,179,181,191,193,197,199,211,223,227,229,233,239,241,251,257,263,269,271,277,281,283,293,307,311,313,317,331,337,347,349,353,359,367,373,379,383,389,397,401,409,419,421,431,433,439,443,449,457,461,463,467,479,487,491,499,503,509,521,523,541,547,557,563,569,571,577,587,593,599,601,607,613,617,619,631,641,643,647,653,659,661,673,677,683,691,701,709,719,727,733,739,743,751,757,761,769,773,787,797,809,811,821,823,827,829,839,853,857,859,863,877,881,883,887,907,911,919,929,937,941,947,953,967,971,977,983,991,997,1009,1013,1019,1021,1031,1033,1039,1049,1051,1061,1063,1069,1087,1091,1093,1097,1103,1109,1117,1123,1129,1151,1153,1163,1171,1181,1187,1193,1201,1213,1217,1223,1229,1231,1237,1249,1259,1277,1279,1283,1289,1291,1297,1301,1303,1307,1319,1321,1327,1361,1367,1373,1381,1399,1409,1423,1427,1429,1433,1439,1447,1451,1453,1459,1471,1481,1483,1487,1489,1493,1499,1511,1523,1531,1543,1549,1553,1559,1567,1571,1579,1583,1597,1601,1607,1609,1613,1619,1621,1627,1637,1657,1663,1667,1669,1693,1697,1699,1709,1721,1723,1733,1741,1747,1753,1759,1777,1783,1787,1789,1801,1811,1823,1831,1847,1861,1867,1871,1873,1877,1879,1889,1901,1907,1913,1931,1933,1949,1951,1973,1979,1987,1993,1997,1999,2003,2011,2017,2027,2029,2039,2053,2063,2069,2081,2083,2087,2089,2099,2111,2113,2129,2131,2137,2141,2143,2153,2161,2179,2203,2207,2213,2221,2237,2239,2243,2251,2267,2269,2273,2281,2287,2293,2297,2309,2311,2333,2339,2341,2347,2351,2357,2371,2377,2381,2383,2389,2393,2399,2411,2417,2423,2437,2441,2447,2459,2467,2473,2477,2503,2521,2531,2539,2543,2549,2551,2557,2579,2591,2593,2609,2617,2621,2633,2647,2657,2659,2663,2671,2677,2683,2687,2689,2693,2699,2707,2711,2713,2719,2729,2731,2741,2749,2753,2767,2777,2789,2791,2797,2801,2803,2819,2833,2837,2843,2851,2857,2861,2879,2887,2897,2903,2909,2917,2927,2939,2953,2957,2963,2969,2971,2999,3001,3011,3019,3023,3037,3041,3049,3061,3067,3079,3083,3089,3109,3119,3121,3137,3163,3167,3169,3181,3187,3191,3203,3209,3217,3221,3229,3251,3253,3257,3259,3271,3299,3301,3307,3313,3319,3323,3329,3331,3343,3347,3359,3361,3371,3373,3389,3391,3407,3413,3433,3449,3457,3461,3463,3467,3469,3491,3499,3511,3517,3527,3529,3533,3539,3541,3547,3557,3559,3571,3581,3583,3593,3607,3613,3617,3623,3631,3637,3643,3659,3671,3673,3677,3691,3697,3701,3709,3719,3727,3733,3739,3761,3767,3769,3779,3793,3797,3803,3821,3823,3833,3847,3851,3853,3863,3877,3881,3889,3907,3911,3917,3919,3923,3929,3931,3943,3947,3967,3989,4001,4003,4007,4013,4019,4021,4027,4049,4051,4057,4073,4079,4091,4093,4099,4111,4127,4129,4133,4139,4153,4157,4159,4177,4201,4211,4217,4219,4229,4231,4241,4243,4253,4259,4261,4271,4273,4283,4289,4297,4327,4337,4339,4349,4357,4363,4373,4391,4397,4409,4421,4423,4441,4447,4451,4457,4463,4481,4483,4493,4507,4513,4517,4519,4523,4547,4549,4561,4567,4583,4591,4597,4603,4621,4637,4639,4643,4649,4651,4657,4663,4673,4679,4691,4703,4721,4723,4729,4733,4751,4759,4783,4787,4789,4793,4799,4801,4813,4817,4831,4861,4871,4877,4889,4903,4909,4919,4931,4933,4937,4943,4951,4957,4967,4969,4973,4987,4993,4999,5003,5009,5011,5021,5023,5039,5051,5059,5077,5081,5087,5099,5101,5107,5113,5119,5147,5153,5167,5171,5179,5189,5197,5209,5227,5231,5233,5237,5261,5273,5279,5281,5297,5303,5309,5323,5333,5347,5351,5381,5387,5393,5399,5407,5413,5417,5419,5431,5437,5441,5443,5449,5471,5477,5479,5483,5501,5503,5507,5519,5521,5527,5531,5557,5563,5569,5573,5581,5591,5623,5639,5641,5647,5651,5653,5657,5659,5669,5683,5689,5693,5701,5711,5717,5737,5741,5743,5749,5779,5783,5791,5801,5807,5813,5821,5827,5839,5843,5849,5851,5857,5861,5867,5869,5879,5881,5897,5903,5923,5927,5939,5953,5981,5987,6007,6011,6029,6037,6043,6047,6053,6067,6073,6079,6089,6091,6101,6113,6121,6131,6133,6143,6151,6163,6173,6197,6199,6203,6211,6217,6221,6229,6247,6257,6263,6269,6271,6277,6287,6299,6301,6311,6317,6323,6329,6337,6343,6353,6359,6361,6367,6373,6379,6389,6397,6421,6427,6449,6451,6469,6473,6481,6491,6521,6529,6547,6551,6553,6563,6569,6571,6577,6581,6599,6607,6619,6637,6653,6659,6661,6673,6679,6689,6691,6701,6703,6709,6719,6733,6737,6761,6763,6779,6781,6791,6793,6803,6823,6827,6829,6833,6841,6857,6863,6869,6871,6883,6899,6907,6911,6917,6947,6949,6959,6961,6967,6971,6977,6983,6991,6997,7001,7013,7019,7027,7039,7043,7057,7069,7079,7103,7109,7121,7127,7129,7151,7159,7177,7187,7193,7207,7211,7213,7219,7229,7237,7243,7247,7253,7283,7297,7307,7309,7321,7331,7333,7349,7351,7369,7393,7411,7417,7433,7451,7457,7459,7477,7481,7487,7489,7499,7507,7517,7523,7529,7537,7541,7547,7549,7559,7561,7573,7577,7583,7589,7591,7603,7607,7621,7639,7643,7649,7669,7673,7681,7687,7691,7699,7703,7717,7723,7727,7741,7753,7757,7759,7789,7793,7817,7823,7829,7841,7853,7867,7873,7877,7879,7883,7901,7907,7919,7927,7933,7937,7949,7951,7963,7993,8009,8011,8017,8039,8053,8059,8069,8081,8087,8089,8093,8101,8111,8117,8123,8147,8161,8167,8171,8179,8191,8209,8219,8221,8231,8233,8237,8243,8263,8269,8273,8287,8291,8293,8297,8311,8317,8329,8353,8363,8369,8377,8387,8389,8419,8423,8429,8431,8443,8447,8461,8467,8501,8513,8521,8527,8537,8539,8543,8563,8573,8581,8597,8599,8609,8623,8627,8629,8641,8647,8663,8669,8677,8681,8689,8693,8699,8707,8713,8719,8731,8737,8741,8747,8753,8761,8779,8783,8803,8807,8819,8821,8831,8837,8839,8849,8861,8863,8867,8887,8893,8923,8929,8933,8941,8951,8963,8969,8971,8999,9001,9007,9011,9013,9029,9041,9043,9049,9059,9067,9091,9103,9109,9127,9133,9137,9151,9157,9161,9173,9181,9187,9199,9203,9209,9221,9227,9239,9241,9257,9277,9281,9283,9293,9311,9319,9323,9337,9341,9343,9349,9371,9377,9391,9397,9403,9413,9419,9421,9431,9433,9437,9439,9461,9463,9467,9473,9479,9491,9497,9511,9521,9533,9539,9547,9551,9587,9601,9613,9619,9623,9629,9631,9643,9649,9661,9677,9679,9689,9697,9719,9721,9733,9739,9743,9749,9767,9769,9781,9787,9791,9803,9811,9817,9829,9833,9839,9851,9857,9859,9871,9883,9887,9901,9907,9923,9929,9931,9941,9949,9967,9973,10007,10009,10037,10039,10061,10067,10069,10079,10091,10093,10099,10103,10111,10133,10139,10141,10151,10159,10163,10169,10177,10181,10193,10211,10223,10243,10247,10253,10259,10267,10271,10273,10289,10301,10303,10313,10321,10331,10333,10337,10343,10357,10369,10391,10399,10427,10429,10433,10453,10457,10459,10463,10477,10487,10499,10501,10513,10529,10531,10559,10567,10589,10597,10601,10607,10613,10627,10631,10639,10651,10657,10663,10667];

#[derive(Debug,Clone)]
pub struct mydspSIG1 {
	imydspSIG1Wave0_idx: i32,
}

impl mydspSIG1 {
	
	fn get_num_inputsmydspSIG1(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsmydspSIG1(&self) -> i32 {
		return 1;
	}
	
	fn instance_initmydspSIG1(&mut self, sample_rate: i32) {
		self.imydspSIG1Wave0_idx = 0;
	}
	
	fn fillmydspSIG1(&mut self, count: i32, table: &mut[i32]) {
		for i2 in 0..count {
			table[i2 as usize] = unsafe { imydspSIG1Wave0[self.imydspSIG1Wave0_idx as usize] };
			self.imydspSIG1Wave0_idx = (i32::wrapping_add(1, self.imydspSIG1Wave0_idx)) % 1302;
		}
	}

}


pub fn newmydspSIG1() -> mydspSIG1 { 
	mydspSIG1 {
		imydspSIG1Wave0_idx: 0,
	}
}
static mut ftbl0mydspSIG0: [F32;2048] = [0.0;2048];
fn mydsp_faustpower2_f(value: F32) -> F32 {
	return value * value;
}
static mut itbl1mydspSIG1: [i32;1302] = [0;1302];

#[cfg_attr(feature = "default-boxed", derive(default_boxed::DefaultBoxed))]
#[repr(C)]
#[derive(Debug,Clone)]
pub struct mydsp {
	iVec0: [i32;2],
	fSampleRate: i32,
	fConst0: F32,
	fConst1: F32,
	fConst2: F32,
	fConst3: F32,
	fHslider0: F32,
	fRec2: [F32;2],
	fHslider1: F32,
	fRec5: [F32;2],
	fHslider2: F32,
	fRec3: [F32;2],
	fConst4: F32,
	IOTA0: i32,
	fConst5: F32,
	fHslider3: F32,
	fRec41: [F32;2],
	fRec40: [F32;2],
	fConst6: F32,
	fRec36: [F32;2],
	fRec42: [F32;2],
	fConst7: F32,
	fConst8: F32,
	fRec45: [F32;2],
	iVec1: [i32;2],
	iConst9: i32,
	iRec46: [i32;2],
	fConst10: F32,
	fRec44: [F32;2],
	fRec47: [F32;4],
	fRec48: [F32;2048],
	fVec2: [F32;2],
	fConst11: F32,
	fConst12: F32,
	fButton0: F32,
	fVec3: [F32;2],
	iRec49: [i32;2],
	iRec51: [i32;2],
	fRec50: [F32;3],
	fVec4: [F32;3],
	fRec43: [F32;2048],
	fRec32: [F32;2],
	fRec28: [F32;2],
	fRec24: [F32;2048],
	fRec26: [F32;2],
	fHslider4: F32,
	fRec22: [F32;4],
	fRec17: [F32;2],
	fRec13: [F32;2048],
	fRec11: [F32;2],
	fConst13: F32,
	fRec10: [F32;2],
	fRec8: [F32;2],
	fRec7: [F32;3],
	fRec6: [F32;3],
	fHslider5: F32,
	fRec52: [F32;2],
	fRec87: [F32;2],
	fRec83: [F32;2],
	fRec88: [F32;2],
	fRec91: [F32;2],
	iVec5: [i32;2],
	iRec92: [i32;2],
	fRec90: [F32;2],
	fRec93: [F32;4],
	fRec94: [F32;2048],
	fVec6: [F32;2],
	fButton1: F32,
	fVec7: [F32;2],
	iRec95: [i32;2],
	fRec96: [F32;3],
	fVec8: [F32;3],
	fRec89: [F32;2048],
	fRec79: [F32;2],
	fRec75: [F32;2],
	fRec71: [F32;2048],
	fRec73: [F32;2],
	fRec69: [F32;4],
	fRec64: [F32;2],
	fRec60: [F32;2048],
	fRec58: [F32;2],
	fRec57: [F32;2],
	fRec55: [F32;2],
	fRec54: [F32;3],
	fRec53: [F32;3],
	fHslider6: F32,
	fRec97: [F32;2],
	fRec132: [F32;2],
	fRec128: [F32;2],
	fRec133: [F32;2],
	fRec136: [F32;2],
	iVec9: [i32;2],
	iRec137: [i32;2],
	fRec135: [F32;2],
	fRec138: [F32;4],
	fRec139: [F32;2048],
	fVec10: [F32;2],
	fButton2: F32,
	fVec11: [F32;2],
	iRec140: [i32;2],
	fRec141: [F32;3],
	fVec12: [F32;3],
	fRec134: [F32;2048],
	fRec124: [F32;2],
	fRec120: [F32;2],
	fRec116: [F32;2048],
	fRec118: [F32;2],
	fRec114: [F32;4],
	fRec109: [F32;2],
	fRec105: [F32;2048],
	fRec103: [F32;2],
	fRec102: [F32;2],
	fRec100: [F32;2],
	fRec99: [F32;3],
	fRec98: [F32;3],
	fHslider7: F32,
	fRec142: [F32;2],
	fRec177: [F32;2],
	fRec173: [F32;2],
	fRec178: [F32;2],
	fRec181: [F32;2],
	iVec13: [i32;2],
	iRec182: [i32;2],
	fRec180: [F32;2],
	fRec183: [F32;4],
	fRec184: [F32;2048],
	fVec14: [F32;2],
	fButton3: F32,
	fVec15: [F32;2],
	iRec185: [i32;2],
	fRec186: [F32;3],
	fVec16: [F32;3],
	fRec179: [F32;2048],
	fRec169: [F32;2],
	fRec165: [F32;2],
	fRec161: [F32;2048],
	fRec163: [F32;2],
	fRec159: [F32;4],
	fRec154: [F32;2],
	fRec150: [F32;2048],
	fRec148: [F32;2],
	fRec147: [F32;2],
	fRec145: [F32;2],
	fRec144: [F32;3],
	fRec143: [F32;3],
	fHslider8: F32,
	fRec187: [F32;2],
	fRec222: [F32;2],
	fRec218: [F32;2],
	fRec223: [F32;2],
	fRec226: [F32;2],
	iVec17: [i32;2],
	iRec227: [i32;2],
	fRec225: [F32;2],
	fRec228: [F32;4],
	fRec229: [F32;2048],
	fVec18: [F32;2],
	fButton4: F32,
	fVec19: [F32;2],
	iRec230: [i32;2],
	fRec231: [F32;3],
	fVec20: [F32;3],
	fRec224: [F32;2048],
	fRec214: [F32;2],
	fRec210: [F32;2],
	fRec206: [F32;2048],
	fRec208: [F32;2],
	fRec204: [F32;4],
	fRec199: [F32;2],
	fRec195: [F32;2048],
	fRec193: [F32;2],
	fRec192: [F32;2],
	fRec190: [F32;2],
	fRec189: [F32;3],
	fRec188: [F32;3],
	fHslider9: F32,
	fRec232: [F32;2],
	fHslider10: F32,
	fRec233: [F32;2],
	fRec238: [F32;2],
	fConst14: F32,
	fRec236: [F32;2],
	fHslider11: F32,
	fRec239: [F32;2],
	fRec235: [F32;3],
	fRec234: [F32;3],
	fHslider12: F32,
	fRec240: [F32;2],
	fRec245: [F32;2],
	fRec243: [F32;2],
	fHslider13: F32,
	fRec246: [F32;2],
	fRec242: [F32;3],
	fRec241: [F32;3],
	fHslider14: F32,
	fRec247: [F32;2],
	fRec252: [F32;2],
	fRec250: [F32;2],
	fHslider15: F32,
	fRec253: [F32;2],
	fRec249: [F32;3],
	fRec248: [F32;3],
	fHslider16: F32,
	fRec254: [F32;2],
	fRec259: [F32;2],
	fRec257: [F32;2],
	fHslider17: F32,
	fRec260: [F32;2],
	fRec256: [F32;3],
	fRec255: [F32;3],
	fHslider18: F32,
	fRec261: [F32;2],
	fHslider19: F32,
	fRec262: [F32;2],
	fHslider20: F32,
	fRec263: [F32;2],
	fHslider21: F32,
	fHslider22: F32,
	fRec265: [F32;2],
	fRec266: [F32;2],
	fVec21: [F32;2],
	fVec22: [F32;4096],
	fConst15: F32,
	fConst16: F32,
	fRec264: [F32;2],
	fRec268: [F32;2],
	fRec269: [F32;2],
	fVec23: [F32;2],
	fVec24: [F32;4096],
	fRec267: [F32;2],
	fRec271: [F32;2],
	fRec272: [F32;2],
	fVec25: [F32;2],
	fVec26: [F32;4096],
	fRec270: [F32;2],
	fHslider23: F32,
	fRec273: [F32;2],
	fHslider24: F32,
	fRec275: [F32;2],
	fRec276: [F32;2],
	fVec27: [F32;2],
	fVec28: [F32;4096],
	fRec274: [F32;2],
	fRec278: [F32;2],
	fRec279: [F32;2],
	fVec29: [F32;2],
	fVec30: [F32;4096],
	fRec277: [F32;2],
	fRec281: [F32;2],
	fRec282: [F32;2],
	fVec31: [F32;2],
	fVec32: [F32;4096],
	fRec280: [F32;2],
	fHslider25: F32,
	fRec283: [F32;2],
	fHslider26: F32,
	fRec285: [F32;2],
	fRec286: [F32;2],
	fVec33: [F32;2],
	fVec34: [F32;4096],
	fRec284: [F32;2],
	fRec288: [F32;2],
	fRec289: [F32;2],
	fVec35: [F32;2],
	fVec36: [F32;4096],
	fRec287: [F32;2],
	fRec291: [F32;2],
	fRec292: [F32;2],
	fVec37: [F32;2],
	fVec38: [F32;4096],
	fRec290: [F32;2],
	fHslider27: F32,
	fRec293: [F32;2],
	fHslider28: F32,
	fRec295: [F32;2],
	fRec296: [F32;2],
	fVec39: [F32;2],
	fVec40: [F32;4096],
	fRec294: [F32;2],
	fRec298: [F32;2],
	fRec299: [F32;2],
	fVec41: [F32;2],
	fVec42: [F32;4096],
	fRec297: [F32;2],
	fRec301: [F32;2],
	fRec302: [F32;2],
	fVec43: [F32;2],
	fVec44: [F32;4096],
	fRec300: [F32;2],
	fHslider29: F32,
	fRec303: [F32;2],
	fConst17: F32,
	fHslider30: F32,
	fRec304: [F32;2],
	fHslider31: F32,
	fRec305: [F32;2],
	fConst18: F32,
	fHslider32: F32,
	fRec307: [F32;2],
	fHslider33: F32,
	fRec306: [F32;2097152],
	fHslider34: F32,
	fHslider35: F32,
	fConst21: F32,
	fConst22: F32,
	fConst24: F32,
	fConst25: F32,
	fConst26: F32,
	fConst27: F32,
	fConst30: F32,
	fConst31: F32,
	fConst32: F32,
	fConst33: F32,
	fConst34: F32,
	fConst35: F32,
	fConst36: F32,
	fHslider36: F32,
	fRec318: [F32;2],
	fRec320: [F32;2],
	fRec324: [F32;2],
	fVec45: [F32;16384],
	fVec46: [F32;2],
	fRec323: [F32;2],
	fRec321: [F32;2],
	fRec326: [F32;2],
	fVec47: [F32;16384],
	fVec48: [F32;2],
	fRec325: [F32;2],
	fRec322: [F32;2],
	fRec330: [F32;2],
	fVec49: [F32;16384],
	fVec50: [F32;2],
	fRec329: [F32;2],
	fRec327: [F32;2],
	fRec332: [F32;2],
	fVec51: [F32;16384],
	fVec52: [F32;2],
	fRec331: [F32;2],
	fRec328: [F32;2],
	fRec336: [F32;2],
	fVec53: [F32;16384],
	fVec54: [F32;2],
	fRec335: [F32;2],
	fRec333: [F32;2],
	fRec338: [F32;2],
	fVec55: [F32;16384],
	fVec56: [F32;2],
	fRec337: [F32;2],
	fRec334: [F32;2],
	fRec342: [F32;2],
	fVec57: [F32;16384],
	fVec58: [F32;2],
	fRec341: [F32;2],
	fRec339: [F32;2],
	fRec344: [F32;2],
	fVec59: [F32;16384],
	fVec60: [F32;2],
	fRec343: [F32;2],
	fRec340: [F32;2],
	fRec348: [F32;2],
	fVec61: [F32;16384],
	fVec62: [F32;2],
	fRec347: [F32;2],
	fRec345: [F32;2],
	fRec350: [F32;2],
	fVec63: [F32;16384],
	fVec64: [F32;2],
	fRec349: [F32;2],
	fRec346: [F32;2],
	fVec65: [F32;1024],
	fHslider37: F32,
	fConst37: F32,
	fRec351: [F32;2],
	fRec352: [F32;2],
	fHslider38: F32,
	fVec66: [F32;16384],
	fVec67: [F32;2],
	fRec319: [F32;2],
	fRec356: [F32;2],
	fRec358: [F32;2],
	fVec68: [F32;1024],
	fVec69: [F32;16384],
	fVec70: [F32;2],
	fRec357: [F32;2],
	fVec71: [F32;16384],
	fVec72: [F32;2],
	fRec355: [F32;2],
	fRec353: [F32;2],
	fRec360: [F32;2],
	fVec73: [F32;16384],
	fVec74: [F32;2],
	fRec359: [F32;2],
	fRec354: [F32;2],
	fRec364: [F32;2],
	fVec75: [F32;16384],
	fVec76: [F32;2],
	fRec363: [F32;2],
	fRec361: [F32;2],
	fRec366: [F32;2],
	fVec77: [F32;16384],
	fVec78: [F32;2],
	fRec365: [F32;2],
	fRec362: [F32;2],
	fRec370: [F32;2],
	fVec79: [F32;16384],
	fVec80: [F32;2],
	fRec369: [F32;2],
	fRec367: [F32;2],
	fRec372: [F32;2],
	fVec81: [F32;16384],
	fVec82: [F32;2],
	fRec371: [F32;2],
	fRec368: [F32;2],
	fRec376: [F32;2],
	fVec83: [F32;16384],
	fVec84: [F32;2],
	fRec375: [F32;2],
	fRec373: [F32;2],
	fRec378: [F32;2],
	fVec85: [F32;16384],
	fVec86: [F32;2],
	fRec377: [F32;2],
	fRec374: [F32;2],
	fRec382: [F32;2],
	fVec87: [F32;16384],
	fVec88: [F32;2],
	fRec381: [F32;2],
	fRec379: [F32;2],
	fRec384: [F32;2],
	fVec89: [F32;16384],
	fVec90: [F32;2],
	fRec383: [F32;2],
	fRec380: [F32;2],
	fVec91: [F32;16384],
	fVec92: [F32;16384],
	fVec93: [F32;2],
	fRec317: [F32;2],
	fConst38: F32,
	fConst40: F32,
	fRec316: [F32;2],
	fRec315: [F32;3],
	fRec314: [F32;3],
	fVec94: [F32;2],
	fConst41: F32,
	fConst43: F32,
	fRec313: [F32;2],
	fRec312: [F32;3],
	fRec311: [F32;3],
	fConst44: F32,
	fConst45: F32,
	fConst46: F32,
	fRec387: [F32;2],
	fRec386: [F32;3],
	fConst47: F32,
	fRec385: [F32;3],
	fConst48: F32,
	fConst49: F32,
	fConst50: F32,
	fRec391: [F32;2],
	fRec390: [F32;3],
	fConst51: F32,
	fRec389: [F32;3],
	fRec388: [F32;3],
	fHslider39: F32,
	fVec95: [F32;1024],
	fRec310: [F32;2],
	fHslider40: F32,
	fRec403: [F32;2],
	fVec96: [F32;16384],
	fVec97: [F32;16384],
	fVec98: [F32;2],
	fRec402: [F32;2],
	fRec401: [F32;2],
	fRec400: [F32;3],
	fRec399: [F32;3],
	fVec99: [F32;2],
	fRec398: [F32;2],
	fRec397: [F32;3],
	fRec396: [F32;3],
	fRec406: [F32;2],
	fRec405: [F32;3],
	fRec404: [F32;3],
	fRec410: [F32;2],
	fRec409: [F32;3],
	fRec408: [F32;3],
	fRec407: [F32;3],
	fVec100: [F32;1024],
	fRec395: [F32;2],
	fVec101: [F32;16384],
	fVec102: [F32;2],
	fRec394: [F32;2],
	fRec392: [F32;2],
	fRec412: [F32;2],
	fVec103: [F32;16384],
	fVec104: [F32;2],
	fRec411: [F32;2],
	fRec393: [F32;2],
	fVec105: [F32;16384],
	fVec106: [F32;2],
	fRec415: [F32;2],
	fRec413: [F32;2],
	fVec107: [F32;16384],
	fVec108: [F32;2],
	fRec416: [F32;2],
	fRec414: [F32;2],
	fVec109: [F32;16384],
	fVec110: [F32;2],
	fRec419: [F32;2],
	fRec417: [F32;2],
	fRec421: [F32;2],
	fVec111: [F32;16384],
	fVec112: [F32;2],
	fRec420: [F32;2],
	fRec418: [F32;2],
	fRec425: [F32;2],
	fVec113: [F32;16384],
	fVec114: [F32;2],
	fRec424: [F32;2],
	fRec422: [F32;2],
	fVec115: [F32;16384],
	fVec116: [F32;2],
	fRec426: [F32;2],
	fRec423: [F32;2],
	fRec308: [F32;2],
	fRec309: [F32;2],
	fConst52: F32,
	fConst53: F32,
	fRec1: [F32;2],
	fConst54: F32,
	fRec0: [F32;2],
	fHslider41: F32,
	fRec427: [F32;2],
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
			fConst3: 0.0,
			fHslider0: 0.0,
			fRec2: [0.0;2],
			fHslider1: 0.0,
			fRec5: [0.0;2],
			fHslider2: 0.0,
			fRec3: [0.0;2],
			fConst4: 0.0,
			IOTA0: 0,
			fConst5: 0.0,
			fHslider3: 0.0,
			fRec41: [0.0;2],
			fRec40: [0.0;2],
			fConst6: 0.0,
			fRec36: [0.0;2],
			fRec42: [0.0;2],
			fConst7: 0.0,
			fConst8: 0.0,
			fRec45: [0.0;2],
			iVec1: [0;2],
			iConst9: 0,
			iRec46: [0;2],
			fConst10: 0.0,
			fRec44: [0.0;2],
			fRec47: [0.0;4],
			fRec48: [0.0;2048],
			fVec2: [0.0;2],
			fConst11: 0.0,
			fConst12: 0.0,
			fButton0: 0.0,
			fVec3: [0.0;2],
			iRec49: [0;2],
			iRec51: [0;2],
			fRec50: [0.0;3],
			fVec4: [0.0;3],
			fRec43: [0.0;2048],
			fRec32: [0.0;2],
			fRec28: [0.0;2],
			fRec24: [0.0;2048],
			fRec26: [0.0;2],
			fHslider4: 0.0,
			fRec22: [0.0;4],
			fRec17: [0.0;2],
			fRec13: [0.0;2048],
			fRec11: [0.0;2],
			fConst13: 0.0,
			fRec10: [0.0;2],
			fRec8: [0.0;2],
			fRec7: [0.0;3],
			fRec6: [0.0;3],
			fHslider5: 0.0,
			fRec52: [0.0;2],
			fRec87: [0.0;2],
			fRec83: [0.0;2],
			fRec88: [0.0;2],
			fRec91: [0.0;2],
			iVec5: [0;2],
			iRec92: [0;2],
			fRec90: [0.0;2],
			fRec93: [0.0;4],
			fRec94: [0.0;2048],
			fVec6: [0.0;2],
			fButton1: 0.0,
			fVec7: [0.0;2],
			iRec95: [0;2],
			fRec96: [0.0;3],
			fVec8: [0.0;3],
			fRec89: [0.0;2048],
			fRec79: [0.0;2],
			fRec75: [0.0;2],
			fRec71: [0.0;2048],
			fRec73: [0.0;2],
			fRec69: [0.0;4],
			fRec64: [0.0;2],
			fRec60: [0.0;2048],
			fRec58: [0.0;2],
			fRec57: [0.0;2],
			fRec55: [0.0;2],
			fRec54: [0.0;3],
			fRec53: [0.0;3],
			fHslider6: 0.0,
			fRec97: [0.0;2],
			fRec132: [0.0;2],
			fRec128: [0.0;2],
			fRec133: [0.0;2],
			fRec136: [0.0;2],
			iVec9: [0;2],
			iRec137: [0;2],
			fRec135: [0.0;2],
			fRec138: [0.0;4],
			fRec139: [0.0;2048],
			fVec10: [0.0;2],
			fButton2: 0.0,
			fVec11: [0.0;2],
			iRec140: [0;2],
			fRec141: [0.0;3],
			fVec12: [0.0;3],
			fRec134: [0.0;2048],
			fRec124: [0.0;2],
			fRec120: [0.0;2],
			fRec116: [0.0;2048],
			fRec118: [0.0;2],
			fRec114: [0.0;4],
			fRec109: [0.0;2],
			fRec105: [0.0;2048],
			fRec103: [0.0;2],
			fRec102: [0.0;2],
			fRec100: [0.0;2],
			fRec99: [0.0;3],
			fRec98: [0.0;3],
			fHslider7: 0.0,
			fRec142: [0.0;2],
			fRec177: [0.0;2],
			fRec173: [0.0;2],
			fRec178: [0.0;2],
			fRec181: [0.0;2],
			iVec13: [0;2],
			iRec182: [0;2],
			fRec180: [0.0;2],
			fRec183: [0.0;4],
			fRec184: [0.0;2048],
			fVec14: [0.0;2],
			fButton3: 0.0,
			fVec15: [0.0;2],
			iRec185: [0;2],
			fRec186: [0.0;3],
			fVec16: [0.0;3],
			fRec179: [0.0;2048],
			fRec169: [0.0;2],
			fRec165: [0.0;2],
			fRec161: [0.0;2048],
			fRec163: [0.0;2],
			fRec159: [0.0;4],
			fRec154: [0.0;2],
			fRec150: [0.0;2048],
			fRec148: [0.0;2],
			fRec147: [0.0;2],
			fRec145: [0.0;2],
			fRec144: [0.0;3],
			fRec143: [0.0;3],
			fHslider8: 0.0,
			fRec187: [0.0;2],
			fRec222: [0.0;2],
			fRec218: [0.0;2],
			fRec223: [0.0;2],
			fRec226: [0.0;2],
			iVec17: [0;2],
			iRec227: [0;2],
			fRec225: [0.0;2],
			fRec228: [0.0;4],
			fRec229: [0.0;2048],
			fVec18: [0.0;2],
			fButton4: 0.0,
			fVec19: [0.0;2],
			iRec230: [0;2],
			fRec231: [0.0;3],
			fVec20: [0.0;3],
			fRec224: [0.0;2048],
			fRec214: [0.0;2],
			fRec210: [0.0;2],
			fRec206: [0.0;2048],
			fRec208: [0.0;2],
			fRec204: [0.0;4],
			fRec199: [0.0;2],
			fRec195: [0.0;2048],
			fRec193: [0.0;2],
			fRec192: [0.0;2],
			fRec190: [0.0;2],
			fRec189: [0.0;3],
			fRec188: [0.0;3],
			fHslider9: 0.0,
			fRec232: [0.0;2],
			fHslider10: 0.0,
			fRec233: [0.0;2],
			fRec238: [0.0;2],
			fConst14: 0.0,
			fRec236: [0.0;2],
			fHslider11: 0.0,
			fRec239: [0.0;2],
			fRec235: [0.0;3],
			fRec234: [0.0;3],
			fHslider12: 0.0,
			fRec240: [0.0;2],
			fRec245: [0.0;2],
			fRec243: [0.0;2],
			fHslider13: 0.0,
			fRec246: [0.0;2],
			fRec242: [0.0;3],
			fRec241: [0.0;3],
			fHslider14: 0.0,
			fRec247: [0.0;2],
			fRec252: [0.0;2],
			fRec250: [0.0;2],
			fHslider15: 0.0,
			fRec253: [0.0;2],
			fRec249: [0.0;3],
			fRec248: [0.0;3],
			fHslider16: 0.0,
			fRec254: [0.0;2],
			fRec259: [0.0;2],
			fRec257: [0.0;2],
			fHslider17: 0.0,
			fRec260: [0.0;2],
			fRec256: [0.0;3],
			fRec255: [0.0;3],
			fHslider18: 0.0,
			fRec261: [0.0;2],
			fHslider19: 0.0,
			fRec262: [0.0;2],
			fHslider20: 0.0,
			fRec263: [0.0;2],
			fHslider21: 0.0,
			fHslider22: 0.0,
			fRec265: [0.0;2],
			fRec266: [0.0;2],
			fVec21: [0.0;2],
			fVec22: [0.0;4096],
			fConst15: 0.0,
			fConst16: 0.0,
			fRec264: [0.0;2],
			fRec268: [0.0;2],
			fRec269: [0.0;2],
			fVec23: [0.0;2],
			fVec24: [0.0;4096],
			fRec267: [0.0;2],
			fRec271: [0.0;2],
			fRec272: [0.0;2],
			fVec25: [0.0;2],
			fVec26: [0.0;4096],
			fRec270: [0.0;2],
			fHslider23: 0.0,
			fRec273: [0.0;2],
			fHslider24: 0.0,
			fRec275: [0.0;2],
			fRec276: [0.0;2],
			fVec27: [0.0;2],
			fVec28: [0.0;4096],
			fRec274: [0.0;2],
			fRec278: [0.0;2],
			fRec279: [0.0;2],
			fVec29: [0.0;2],
			fVec30: [0.0;4096],
			fRec277: [0.0;2],
			fRec281: [0.0;2],
			fRec282: [0.0;2],
			fVec31: [0.0;2],
			fVec32: [0.0;4096],
			fRec280: [0.0;2],
			fHslider25: 0.0,
			fRec283: [0.0;2],
			fHslider26: 0.0,
			fRec285: [0.0;2],
			fRec286: [0.0;2],
			fVec33: [0.0;2],
			fVec34: [0.0;4096],
			fRec284: [0.0;2],
			fRec288: [0.0;2],
			fRec289: [0.0;2],
			fVec35: [0.0;2],
			fVec36: [0.0;4096],
			fRec287: [0.0;2],
			fRec291: [0.0;2],
			fRec292: [0.0;2],
			fVec37: [0.0;2],
			fVec38: [0.0;4096],
			fRec290: [0.0;2],
			fHslider27: 0.0,
			fRec293: [0.0;2],
			fHslider28: 0.0,
			fRec295: [0.0;2],
			fRec296: [0.0;2],
			fVec39: [0.0;2],
			fVec40: [0.0;4096],
			fRec294: [0.0;2],
			fRec298: [0.0;2],
			fRec299: [0.0;2],
			fVec41: [0.0;2],
			fVec42: [0.0;4096],
			fRec297: [0.0;2],
			fRec301: [0.0;2],
			fRec302: [0.0;2],
			fVec43: [0.0;2],
			fVec44: [0.0;4096],
			fRec300: [0.0;2],
			fHslider29: 0.0,
			fRec303: [0.0;2],
			fConst17: 0.0,
			fHslider30: 0.0,
			fRec304: [0.0;2],
			fHslider31: 0.0,
			fRec305: [0.0;2],
			fConst18: 0.0,
			fHslider32: 0.0,
			fRec307: [0.0;2],
			fHslider33: 0.0,
			fRec306: [0.0;2097152],
			fHslider34: 0.0,
			fHslider35: 0.0,
			fConst21: 0.0,
			fConst22: 0.0,
			fConst24: 0.0,
			fConst25: 0.0,
			fConst26: 0.0,
			fConst27: 0.0,
			fConst30: 0.0,
			fConst31: 0.0,
			fConst32: 0.0,
			fConst33: 0.0,
			fConst34: 0.0,
			fConst35: 0.0,
			fConst36: 0.0,
			fHslider36: 0.0,
			fRec318: [0.0;2],
			fRec320: [0.0;2],
			fRec324: [0.0;2],
			fVec45: [0.0;16384],
			fVec46: [0.0;2],
			fRec323: [0.0;2],
			fRec321: [0.0;2],
			fRec326: [0.0;2],
			fVec47: [0.0;16384],
			fVec48: [0.0;2],
			fRec325: [0.0;2],
			fRec322: [0.0;2],
			fRec330: [0.0;2],
			fVec49: [0.0;16384],
			fVec50: [0.0;2],
			fRec329: [0.0;2],
			fRec327: [0.0;2],
			fRec332: [0.0;2],
			fVec51: [0.0;16384],
			fVec52: [0.0;2],
			fRec331: [0.0;2],
			fRec328: [0.0;2],
			fRec336: [0.0;2],
			fVec53: [0.0;16384],
			fVec54: [0.0;2],
			fRec335: [0.0;2],
			fRec333: [0.0;2],
			fRec338: [0.0;2],
			fVec55: [0.0;16384],
			fVec56: [0.0;2],
			fRec337: [0.0;2],
			fRec334: [0.0;2],
			fRec342: [0.0;2],
			fVec57: [0.0;16384],
			fVec58: [0.0;2],
			fRec341: [0.0;2],
			fRec339: [0.0;2],
			fRec344: [0.0;2],
			fVec59: [0.0;16384],
			fVec60: [0.0;2],
			fRec343: [0.0;2],
			fRec340: [0.0;2],
			fRec348: [0.0;2],
			fVec61: [0.0;16384],
			fVec62: [0.0;2],
			fRec347: [0.0;2],
			fRec345: [0.0;2],
			fRec350: [0.0;2],
			fVec63: [0.0;16384],
			fVec64: [0.0;2],
			fRec349: [0.0;2],
			fRec346: [0.0;2],
			fVec65: [0.0;1024],
			fHslider37: 0.0,
			fConst37: 0.0,
			fRec351: [0.0;2],
			fRec352: [0.0;2],
			fHslider38: 0.0,
			fVec66: [0.0;16384],
			fVec67: [0.0;2],
			fRec319: [0.0;2],
			fRec356: [0.0;2],
			fRec358: [0.0;2],
			fVec68: [0.0;1024],
			fVec69: [0.0;16384],
			fVec70: [0.0;2],
			fRec357: [0.0;2],
			fVec71: [0.0;16384],
			fVec72: [0.0;2],
			fRec355: [0.0;2],
			fRec353: [0.0;2],
			fRec360: [0.0;2],
			fVec73: [0.0;16384],
			fVec74: [0.0;2],
			fRec359: [0.0;2],
			fRec354: [0.0;2],
			fRec364: [0.0;2],
			fVec75: [0.0;16384],
			fVec76: [0.0;2],
			fRec363: [0.0;2],
			fRec361: [0.0;2],
			fRec366: [0.0;2],
			fVec77: [0.0;16384],
			fVec78: [0.0;2],
			fRec365: [0.0;2],
			fRec362: [0.0;2],
			fRec370: [0.0;2],
			fVec79: [0.0;16384],
			fVec80: [0.0;2],
			fRec369: [0.0;2],
			fRec367: [0.0;2],
			fRec372: [0.0;2],
			fVec81: [0.0;16384],
			fVec82: [0.0;2],
			fRec371: [0.0;2],
			fRec368: [0.0;2],
			fRec376: [0.0;2],
			fVec83: [0.0;16384],
			fVec84: [0.0;2],
			fRec375: [0.0;2],
			fRec373: [0.0;2],
			fRec378: [0.0;2],
			fVec85: [0.0;16384],
			fVec86: [0.0;2],
			fRec377: [0.0;2],
			fRec374: [0.0;2],
			fRec382: [0.0;2],
			fVec87: [0.0;16384],
			fVec88: [0.0;2],
			fRec381: [0.0;2],
			fRec379: [0.0;2],
			fRec384: [0.0;2],
			fVec89: [0.0;16384],
			fVec90: [0.0;2],
			fRec383: [0.0;2],
			fRec380: [0.0;2],
			fVec91: [0.0;16384],
			fVec92: [0.0;16384],
			fVec93: [0.0;2],
			fRec317: [0.0;2],
			fConst38: 0.0,
			fConst40: 0.0,
			fRec316: [0.0;2],
			fRec315: [0.0;3],
			fRec314: [0.0;3],
			fVec94: [0.0;2],
			fConst41: 0.0,
			fConst43: 0.0,
			fRec313: [0.0;2],
			fRec312: [0.0;3],
			fRec311: [0.0;3],
			fConst44: 0.0,
			fConst45: 0.0,
			fConst46: 0.0,
			fRec387: [0.0;2],
			fRec386: [0.0;3],
			fConst47: 0.0,
			fRec385: [0.0;3],
			fConst48: 0.0,
			fConst49: 0.0,
			fConst50: 0.0,
			fRec391: [0.0;2],
			fRec390: [0.0;3],
			fConst51: 0.0,
			fRec389: [0.0;3],
			fRec388: [0.0;3],
			fHslider39: 0.0,
			fVec95: [0.0;1024],
			fRec310: [0.0;2],
			fHslider40: 0.0,
			fRec403: [0.0;2],
			fVec96: [0.0;16384],
			fVec97: [0.0;16384],
			fVec98: [0.0;2],
			fRec402: [0.0;2],
			fRec401: [0.0;2],
			fRec400: [0.0;3],
			fRec399: [0.0;3],
			fVec99: [0.0;2],
			fRec398: [0.0;2],
			fRec397: [0.0;3],
			fRec396: [0.0;3],
			fRec406: [0.0;2],
			fRec405: [0.0;3],
			fRec404: [0.0;3],
			fRec410: [0.0;2],
			fRec409: [0.0;3],
			fRec408: [0.0;3],
			fRec407: [0.0;3],
			fVec100: [0.0;1024],
			fRec395: [0.0;2],
			fVec101: [0.0;16384],
			fVec102: [0.0;2],
			fRec394: [0.0;2],
			fRec392: [0.0;2],
			fRec412: [0.0;2],
			fVec103: [0.0;16384],
			fVec104: [0.0;2],
			fRec411: [0.0;2],
			fRec393: [0.0;2],
			fVec105: [0.0;16384],
			fVec106: [0.0;2],
			fRec415: [0.0;2],
			fRec413: [0.0;2],
			fVec107: [0.0;16384],
			fVec108: [0.0;2],
			fRec416: [0.0;2],
			fRec414: [0.0;2],
			fVec109: [0.0;16384],
			fVec110: [0.0;2],
			fRec419: [0.0;2],
			fRec417: [0.0;2],
			fRec421: [0.0;2],
			fVec111: [0.0;16384],
			fVec112: [0.0;2],
			fRec420: [0.0;2],
			fRec418: [0.0;2],
			fRec425: [0.0;2],
			fVec113: [0.0;16384],
			fVec114: [0.0;2],
			fRec424: [0.0;2],
			fRec422: [0.0;2],
			fVec115: [0.0;16384],
			fVec116: [0.0;2],
			fRec426: [0.0;2],
			fRec423: [0.0;2],
			fRec308: [0.0;2],
			fRec309: [0.0;2],
			fConst52: 0.0,
			fConst53: 0.0,
			fRec1: [0.0;2],
			fConst54: 0.0,
			fRec0: [0.0;2],
			fHslider41: 0.0,
			fRec427: [0.0;2],
		}
	}
	fn metadata(&self, m: &mut dyn Meta) { 
		m.declare("analyzers.lib/amp_follower_ar:author", r"Jonatan Liljedahl, revised by Romain Michon");
		m.declare("analyzers.lib/name", r"Faust Analyzer Library");
		m.declare("analyzers.lib/version", r"1.2.0");
		m.declare("author", r"Pierre Lulé");
		m.declare("basics.lib/name", r"Faust Basic Element Library");
		m.declare("basics.lib/tabulate:author", r"Stephane Letz");
		m.declare("basics.lib/tabulateNd", r"Copyright (C) 2023 Bart Brouns <bart@magnetophon.nl>");
		m.declare("basics.lib/version", r"1.11.1");
		m.declare("compile_options", r"-a /tmp/.tmpYmp5tR -lang rust -ct 1 -es 1 -mcd 16 -single -ftz 0");
		m.declare("compressors.lib/compression_gain_mono:author", r"Julius O. Smith III");
		m.declare("compressors.lib/compression_gain_mono:copyright", r"Copyright (C) 2014-2020 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("compressors.lib/compression_gain_mono:license", r"MIT-style STK-4.3 license");
		m.declare("compressors.lib/compressor_lad_mono:author", r"Julius O. Smith III");
		m.declare("compressors.lib/compressor_lad_mono:copyright", r"Copyright (C) 2014-2020 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("compressors.lib/compressor_lad_mono:license", r"MIT-style STK-4.3 license");
		m.declare("compressors.lib/compressor_mono:author", r"Julius O. Smith III");
		m.declare("compressors.lib/compressor_mono:copyright", r"Copyright (C) 2014-2020 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("compressors.lib/compressor_mono:license", r"MIT-style STK-4.3 license");
		m.declare("compressors.lib/name", r"Faust Compressor Effect Library");
		m.declare("compressors.lib/version", r"1.5.0");
		m.declare("delays.lib/fdelay1a:author", r"Julius O. Smith III");
		m.declare("delays.lib/fdelay4:author", r"Julius O. Smith III");
		m.declare("delays.lib/fdelayltv:author", r"Julius O. Smith III");
		m.declare("delays.lib/name", r"Faust Delay Library");
		m.declare("delays.lib/version", r"1.1.0");
		m.declare("envelopes.lib/ar:author", r"Yann Orlarey, Stéphane Letz");
		m.declare("envelopes.lib/author", r"GRAME");
		m.declare("envelopes.lib/copyright", r"GRAME");
		m.declare("envelopes.lib/license", r"LGPL with exception");
		m.declare("envelopes.lib/name", r"Faust Envelope Library");
		m.declare("envelopes.lib/version", r"1.2.0");
		m.declare("filename", r"instrument.dsp");
		m.declare("filters.lib/filterbank:author", r"Julius O. Smith III");
		m.declare("filters.lib/filterbank:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/filterbank:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/fir:author", r"Julius O. Smith III");
		m.declare("filters.lib/fir:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/fir:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/highpass:author", r"Julius O. Smith III");
		m.declare("filters.lib/highpass:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/highpass_plus_lowpass:author", r"Julius O. Smith III");
		m.declare("filters.lib/highpass_plus_lowpass:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/highpass_plus_lowpass:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/iir:author", r"Julius O. Smith III");
		m.declare("filters.lib/iir:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/iir:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/lowpass0_highpass1", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/lowpass0_highpass1:author", r"Julius O. Smith III");
		m.declare("filters.lib/lowpass:author", r"Julius O. Smith III");
		m.declare("filters.lib/lowpass:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/lowpass:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/name", r"Faust Filters Library");
		m.declare("filters.lib/nlf2:author", r"Julius O. Smith III");
		m.declare("filters.lib/nlf2:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/nlf2:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/pole:author", r"Julius O. Smith III");
		m.declare("filters.lib/pole:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/pole:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/tf1:author", r"Julius O. Smith III");
		m.declare("filters.lib/tf1:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf1:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/tf1s:author", r"Julius O. Smith III");
		m.declare("filters.lib/tf1s:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf1s:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/tf2:author", r"Julius O. Smith III");
		m.declare("filters.lib/tf2:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf2:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/tf2s:author", r"Julius O. Smith III");
		m.declare("filters.lib/tf2s:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf2s:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/version", r"1.3.0");
		m.declare("license", r"BSD");
		m.declare("maths.lib/author", r"GRAME");
		m.declare("maths.lib/copyright", r"GRAME");
		m.declare("maths.lib/license", r"LGPL with exception");
		m.declare("maths.lib/name", r"Faust Math Library");
		m.declare("maths.lib/version", r"2.6.0");
		m.declare("misceffects.lib/echo:author", r"Romain Michon");
		m.declare("misceffects.lib/name", r"Misc Effects Library");
		m.declare("misceffects.lib/version", r"2.1.0");
		m.declare("name", r"theremotion");
		m.declare("noises.lib/name", r"Faust Noise Generator Library");
		m.declare("noises.lib/version", r"1.4.0");
		m.declare("oscillators.lib/lf_sawpos:author", r"Bart Brouns, revised by Stéphane Letz");
		m.declare("oscillators.lib/lf_sawpos:licence", r"STK-4.3");
		m.declare("oscillators.lib/name", r"Faust Oscillator Library");
		m.declare("oscillators.lib/saw2ptr:author", r"Julius O. Smith III");
		m.declare("oscillators.lib/saw2ptr:license", r"STK-4.3");
		m.declare("oscillators.lib/sawN:author", r"Julius O. Smith III");
		m.declare("oscillators.lib/sawN:license", r"STK-4.3");
		m.declare("oscillators.lib/version", r"1.4.0");
		m.declare("physmodels.lib/name", r"Faust Physical Models Library");
		m.declare("physmodels.lib/version", r"1.1.0");
		m.declare("platform.lib/name", r"Generic Platform Library");
		m.declare("platform.lib/version", r"1.3.0");
		m.declare("reverbs.lib/jpverb:author", r"Julian Parker, bug fixes and minor interface changes by Till Bovermann");
		m.declare("reverbs.lib/jpverb:license", r"GPL2+");
		m.declare("reverbs.lib/name", r"Faust Reverb Library");
		m.declare("reverbs.lib/version", r"1.2.0");
		m.declare("routes.lib/name", r"Faust Signal Routing Library");
		m.declare("routes.lib/version", r"1.2.0");
		m.declare("signals.lib/name", r"Faust Signal Routing Library");
		m.declare("signals.lib/onePoleSwitching:author", r"Jonatan Liljedahl, revised by Dario Sanfilippo");
		m.declare("signals.lib/onePoleSwitching:licence", r"STK-4.3");
		m.declare("signals.lib/version", r"1.3.0");
		m.declare("vaeffects.lib/moog_vcf_2b:author", r"Julius O. Smith III");
		m.declare("vaeffects.lib/moog_vcf_2b:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("vaeffects.lib/moog_vcf_2b:license", r"MIT-style STK-4.3 license");
		m.declare("vaeffects.lib/name", r"Faust Virtual Analog Filter Effect Library");
		m.declare("vaeffects.lib/version", r"1.2.0");
		m.declare("version", r"1.0");
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
		sig0.fillmydspSIG0(2048, unsafe { &mut ftbl0mydspSIG0 });
		let mut sig1: mydspSIG1 = newmydspSIG1();
		sig1.instance_initmydspSIG1(sample_rate);
		sig1.fillmydspSIG1(1302, unsafe { &mut itbl1mydspSIG1 });
	}
	fn instance_reset_params(&mut self) {
		self.fHslider0 = 0.0;
		self.fHslider1 = 0.0;
		self.fHslider2 = 8e+01;
		self.fHslider3 = 0.0;
		self.fButton0 = 0.0;
		self.fHslider4 = 1.0;
		self.fHslider5 = 8e+01;
		self.fButton1 = 0.0;
		self.fHslider6 = 8e+01;
		self.fButton2 = 0.0;
		self.fHslider7 = 8e+01;
		self.fButton3 = 0.0;
		self.fHslider8 = 8e+01;
		self.fButton4 = 0.0;
		self.fHslider9 = 1.0;
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
		self.fHslider20 = 0.0;
		self.fHslider21 = 0.1;
		self.fHslider22 = 6e+01;
		self.fHslider23 = 0.0;
		self.fHslider24 = 6e+01;
		self.fHslider25 = 0.0;
		self.fHslider26 = 6e+01;
		self.fHslider27 = 0.0;
		self.fHslider28 = 6e+01;
		self.fHslider29 = 0.0;
		self.fHslider30 = 1.0;
		self.fHslider31 = 1.0;
		self.fHslider32 = 0.3;
		self.fHslider33 = 0.3;
		self.fHslider34 = 0.11;
		self.fHslider35 = 0.88;
		self.fHslider36 = 5.0;
		self.fHslider37 = 0.6;
		self.fHslider38 = 0.98;
		self.fHslider39 = 3.5;
		self.fHslider40 = 0.75;
		self.fHslider41 = 1.0;
	}
	fn instance_clear(&mut self) {
		for l0 in 0..2 {
			self.iVec0[l0 as usize] = 0;
		}
		for l1 in 0..2 {
			self.fRec2[l1 as usize] = 0.0;
		}
		for l3 in 0..2 {
			self.fRec5[l3 as usize] = 0.0;
		}
		for l4 in 0..2 {
			self.fRec3[l4 as usize] = 0.0;
		}
		self.IOTA0 = 0;
		for l5 in 0..2 {
			self.fRec41[l5 as usize] = 0.0;
		}
		for l6 in 0..2 {
			self.fRec40[l6 as usize] = 0.0;
		}
		for l7 in 0..2 {
			self.fRec36[l7 as usize] = 0.0;
		}
		for l8 in 0..2 {
			self.fRec42[l8 as usize] = 0.0;
		}
		for l9 in 0..2 {
			self.fRec45[l9 as usize] = 0.0;
		}
		for l10 in 0..2 {
			self.iVec1[l10 as usize] = 0;
		}
		for l11 in 0..2 {
			self.iRec46[l11 as usize] = 0;
		}
		for l12 in 0..2 {
			self.fRec44[l12 as usize] = 0.0;
		}
		for l13 in 0..4 {
			self.fRec47[l13 as usize] = 0.0;
		}
		for l14 in 0..2048 {
			self.fRec48[l14 as usize] = 0.0;
		}
		for l15 in 0..2 {
			self.fVec2[l15 as usize] = 0.0;
		}
		for l16 in 0..2 {
			self.fVec3[l16 as usize] = 0.0;
		}
		for l17 in 0..2 {
			self.iRec49[l17 as usize] = 0;
		}
		for l18 in 0..2 {
			self.iRec51[l18 as usize] = 0;
		}
		for l19 in 0..3 {
			self.fRec50[l19 as usize] = 0.0;
		}
		for l20 in 0..3 {
			self.fVec4[l20 as usize] = 0.0;
		}
		for l21 in 0..2048 {
			self.fRec43[l21 as usize] = 0.0;
		}
		for l22 in 0..2 {
			self.fRec32[l22 as usize] = 0.0;
		}
		for l23 in 0..2 {
			self.fRec28[l23 as usize] = 0.0;
		}
		for l24 in 0..2048 {
			self.fRec24[l24 as usize] = 0.0;
		}
		for l25 in 0..2 {
			self.fRec26[l25 as usize] = 0.0;
		}
		for l26 in 0..4 {
			self.fRec22[l26 as usize] = 0.0;
		}
		for l27 in 0..2 {
			self.fRec17[l27 as usize] = 0.0;
		}
		for l28 in 0..2048 {
			self.fRec13[l28 as usize] = 0.0;
		}
		for l29 in 0..2 {
			self.fRec11[l29 as usize] = 0.0;
		}
		for l30 in 0..2 {
			self.fRec10[l30 as usize] = 0.0;
		}
		for l31 in 0..2 {
			self.fRec8[l31 as usize] = 0.0;
		}
		for l32 in 0..3 {
			self.fRec7[l32 as usize] = 0.0;
		}
		for l33 in 0..3 {
			self.fRec6[l33 as usize] = 0.0;
		}
		for l34 in 0..2 {
			self.fRec52[l34 as usize] = 0.0;
		}
		for l35 in 0..2 {
			self.fRec87[l35 as usize] = 0.0;
		}
		for l36 in 0..2 {
			self.fRec83[l36 as usize] = 0.0;
		}
		for l37 in 0..2 {
			self.fRec88[l37 as usize] = 0.0;
		}
		for l38 in 0..2 {
			self.fRec91[l38 as usize] = 0.0;
		}
		for l39 in 0..2 {
			self.iVec5[l39 as usize] = 0;
		}
		for l40 in 0..2 {
			self.iRec92[l40 as usize] = 0;
		}
		for l41 in 0..2 {
			self.fRec90[l41 as usize] = 0.0;
		}
		for l42 in 0..4 {
			self.fRec93[l42 as usize] = 0.0;
		}
		for l43 in 0..2048 {
			self.fRec94[l43 as usize] = 0.0;
		}
		for l44 in 0..2 {
			self.fVec6[l44 as usize] = 0.0;
		}
		for l45 in 0..2 {
			self.fVec7[l45 as usize] = 0.0;
		}
		for l46 in 0..2 {
			self.iRec95[l46 as usize] = 0;
		}
		for l47 in 0..3 {
			self.fRec96[l47 as usize] = 0.0;
		}
		for l48 in 0..3 {
			self.fVec8[l48 as usize] = 0.0;
		}
		for l49 in 0..2048 {
			self.fRec89[l49 as usize] = 0.0;
		}
		for l50 in 0..2 {
			self.fRec79[l50 as usize] = 0.0;
		}
		for l51 in 0..2 {
			self.fRec75[l51 as usize] = 0.0;
		}
		for l52 in 0..2048 {
			self.fRec71[l52 as usize] = 0.0;
		}
		for l53 in 0..2 {
			self.fRec73[l53 as usize] = 0.0;
		}
		for l54 in 0..4 {
			self.fRec69[l54 as usize] = 0.0;
		}
		for l55 in 0..2 {
			self.fRec64[l55 as usize] = 0.0;
		}
		for l56 in 0..2048 {
			self.fRec60[l56 as usize] = 0.0;
		}
		for l57 in 0..2 {
			self.fRec58[l57 as usize] = 0.0;
		}
		for l58 in 0..2 {
			self.fRec57[l58 as usize] = 0.0;
		}
		for l59 in 0..2 {
			self.fRec55[l59 as usize] = 0.0;
		}
		for l60 in 0..3 {
			self.fRec54[l60 as usize] = 0.0;
		}
		for l61 in 0..3 {
			self.fRec53[l61 as usize] = 0.0;
		}
		for l62 in 0..2 {
			self.fRec97[l62 as usize] = 0.0;
		}
		for l63 in 0..2 {
			self.fRec132[l63 as usize] = 0.0;
		}
		for l64 in 0..2 {
			self.fRec128[l64 as usize] = 0.0;
		}
		for l65 in 0..2 {
			self.fRec133[l65 as usize] = 0.0;
		}
		for l66 in 0..2 {
			self.fRec136[l66 as usize] = 0.0;
		}
		for l67 in 0..2 {
			self.iVec9[l67 as usize] = 0;
		}
		for l68 in 0..2 {
			self.iRec137[l68 as usize] = 0;
		}
		for l69 in 0..2 {
			self.fRec135[l69 as usize] = 0.0;
		}
		for l70 in 0..4 {
			self.fRec138[l70 as usize] = 0.0;
		}
		for l71 in 0..2048 {
			self.fRec139[l71 as usize] = 0.0;
		}
		for l72 in 0..2 {
			self.fVec10[l72 as usize] = 0.0;
		}
		for l73 in 0..2 {
			self.fVec11[l73 as usize] = 0.0;
		}
		for l74 in 0..2 {
			self.iRec140[l74 as usize] = 0;
		}
		for l75 in 0..3 {
			self.fRec141[l75 as usize] = 0.0;
		}
		for l76 in 0..3 {
			self.fVec12[l76 as usize] = 0.0;
		}
		for l77 in 0..2048 {
			self.fRec134[l77 as usize] = 0.0;
		}
		for l78 in 0..2 {
			self.fRec124[l78 as usize] = 0.0;
		}
		for l79 in 0..2 {
			self.fRec120[l79 as usize] = 0.0;
		}
		for l80 in 0..2048 {
			self.fRec116[l80 as usize] = 0.0;
		}
		for l81 in 0..2 {
			self.fRec118[l81 as usize] = 0.0;
		}
		for l82 in 0..4 {
			self.fRec114[l82 as usize] = 0.0;
		}
		for l83 in 0..2 {
			self.fRec109[l83 as usize] = 0.0;
		}
		for l84 in 0..2048 {
			self.fRec105[l84 as usize] = 0.0;
		}
		for l85 in 0..2 {
			self.fRec103[l85 as usize] = 0.0;
		}
		for l86 in 0..2 {
			self.fRec102[l86 as usize] = 0.0;
		}
		for l87 in 0..2 {
			self.fRec100[l87 as usize] = 0.0;
		}
		for l88 in 0..3 {
			self.fRec99[l88 as usize] = 0.0;
		}
		for l89 in 0..3 {
			self.fRec98[l89 as usize] = 0.0;
		}
		for l90 in 0..2 {
			self.fRec142[l90 as usize] = 0.0;
		}
		for l91 in 0..2 {
			self.fRec177[l91 as usize] = 0.0;
		}
		for l92 in 0..2 {
			self.fRec173[l92 as usize] = 0.0;
		}
		for l93 in 0..2 {
			self.fRec178[l93 as usize] = 0.0;
		}
		for l94 in 0..2 {
			self.fRec181[l94 as usize] = 0.0;
		}
		for l95 in 0..2 {
			self.iVec13[l95 as usize] = 0;
		}
		for l96 in 0..2 {
			self.iRec182[l96 as usize] = 0;
		}
		for l97 in 0..2 {
			self.fRec180[l97 as usize] = 0.0;
		}
		for l98 in 0..4 {
			self.fRec183[l98 as usize] = 0.0;
		}
		for l99 in 0..2048 {
			self.fRec184[l99 as usize] = 0.0;
		}
		for l100 in 0..2 {
			self.fVec14[l100 as usize] = 0.0;
		}
		for l101 in 0..2 {
			self.fVec15[l101 as usize] = 0.0;
		}
		for l102 in 0..2 {
			self.iRec185[l102 as usize] = 0;
		}
		for l103 in 0..3 {
			self.fRec186[l103 as usize] = 0.0;
		}
		for l104 in 0..3 {
			self.fVec16[l104 as usize] = 0.0;
		}
		for l105 in 0..2048 {
			self.fRec179[l105 as usize] = 0.0;
		}
		for l106 in 0..2 {
			self.fRec169[l106 as usize] = 0.0;
		}
		for l107 in 0..2 {
			self.fRec165[l107 as usize] = 0.0;
		}
		for l108 in 0..2048 {
			self.fRec161[l108 as usize] = 0.0;
		}
		for l109 in 0..2 {
			self.fRec163[l109 as usize] = 0.0;
		}
		for l110 in 0..4 {
			self.fRec159[l110 as usize] = 0.0;
		}
		for l111 in 0..2 {
			self.fRec154[l111 as usize] = 0.0;
		}
		for l112 in 0..2048 {
			self.fRec150[l112 as usize] = 0.0;
		}
		for l113 in 0..2 {
			self.fRec148[l113 as usize] = 0.0;
		}
		for l114 in 0..2 {
			self.fRec147[l114 as usize] = 0.0;
		}
		for l115 in 0..2 {
			self.fRec145[l115 as usize] = 0.0;
		}
		for l116 in 0..3 {
			self.fRec144[l116 as usize] = 0.0;
		}
		for l117 in 0..3 {
			self.fRec143[l117 as usize] = 0.0;
		}
		for l118 in 0..2 {
			self.fRec187[l118 as usize] = 0.0;
		}
		for l119 in 0..2 {
			self.fRec222[l119 as usize] = 0.0;
		}
		for l120 in 0..2 {
			self.fRec218[l120 as usize] = 0.0;
		}
		for l121 in 0..2 {
			self.fRec223[l121 as usize] = 0.0;
		}
		for l122 in 0..2 {
			self.fRec226[l122 as usize] = 0.0;
		}
		for l123 in 0..2 {
			self.iVec17[l123 as usize] = 0;
		}
		for l124 in 0..2 {
			self.iRec227[l124 as usize] = 0;
		}
		for l125 in 0..2 {
			self.fRec225[l125 as usize] = 0.0;
		}
		for l126 in 0..4 {
			self.fRec228[l126 as usize] = 0.0;
		}
		for l127 in 0..2048 {
			self.fRec229[l127 as usize] = 0.0;
		}
		for l128 in 0..2 {
			self.fVec18[l128 as usize] = 0.0;
		}
		for l129 in 0..2 {
			self.fVec19[l129 as usize] = 0.0;
		}
		for l130 in 0..2 {
			self.iRec230[l130 as usize] = 0;
		}
		for l131 in 0..3 {
			self.fRec231[l131 as usize] = 0.0;
		}
		for l132 in 0..3 {
			self.fVec20[l132 as usize] = 0.0;
		}
		for l133 in 0..2048 {
			self.fRec224[l133 as usize] = 0.0;
		}
		for l134 in 0..2 {
			self.fRec214[l134 as usize] = 0.0;
		}
		for l135 in 0..2 {
			self.fRec210[l135 as usize] = 0.0;
		}
		for l136 in 0..2048 {
			self.fRec206[l136 as usize] = 0.0;
		}
		for l137 in 0..2 {
			self.fRec208[l137 as usize] = 0.0;
		}
		for l138 in 0..4 {
			self.fRec204[l138 as usize] = 0.0;
		}
		for l139 in 0..2 {
			self.fRec199[l139 as usize] = 0.0;
		}
		for l140 in 0..2048 {
			self.fRec195[l140 as usize] = 0.0;
		}
		for l141 in 0..2 {
			self.fRec193[l141 as usize] = 0.0;
		}
		for l142 in 0..2 {
			self.fRec192[l142 as usize] = 0.0;
		}
		for l143 in 0..2 {
			self.fRec190[l143 as usize] = 0.0;
		}
		for l144 in 0..3 {
			self.fRec189[l144 as usize] = 0.0;
		}
		for l145 in 0..3 {
			self.fRec188[l145 as usize] = 0.0;
		}
		for l146 in 0..2 {
			self.fRec232[l146 as usize] = 0.0;
		}
		for l147 in 0..2 {
			self.fRec233[l147 as usize] = 0.0;
		}
		for l148 in 0..2 {
			self.fRec238[l148 as usize] = 0.0;
		}
		for l149 in 0..2 {
			self.fRec236[l149 as usize] = 0.0;
		}
		for l150 in 0..2 {
			self.fRec239[l150 as usize] = 0.0;
		}
		for l151 in 0..3 {
			self.fRec235[l151 as usize] = 0.0;
		}
		for l152 in 0..3 {
			self.fRec234[l152 as usize] = 0.0;
		}
		for l153 in 0..2 {
			self.fRec240[l153 as usize] = 0.0;
		}
		for l154 in 0..2 {
			self.fRec245[l154 as usize] = 0.0;
		}
		for l155 in 0..2 {
			self.fRec243[l155 as usize] = 0.0;
		}
		for l156 in 0..2 {
			self.fRec246[l156 as usize] = 0.0;
		}
		for l157 in 0..3 {
			self.fRec242[l157 as usize] = 0.0;
		}
		for l158 in 0..3 {
			self.fRec241[l158 as usize] = 0.0;
		}
		for l159 in 0..2 {
			self.fRec247[l159 as usize] = 0.0;
		}
		for l160 in 0..2 {
			self.fRec252[l160 as usize] = 0.0;
		}
		for l161 in 0..2 {
			self.fRec250[l161 as usize] = 0.0;
		}
		for l162 in 0..2 {
			self.fRec253[l162 as usize] = 0.0;
		}
		for l163 in 0..3 {
			self.fRec249[l163 as usize] = 0.0;
		}
		for l164 in 0..3 {
			self.fRec248[l164 as usize] = 0.0;
		}
		for l165 in 0..2 {
			self.fRec254[l165 as usize] = 0.0;
		}
		for l166 in 0..2 {
			self.fRec259[l166 as usize] = 0.0;
		}
		for l167 in 0..2 {
			self.fRec257[l167 as usize] = 0.0;
		}
		for l168 in 0..2 {
			self.fRec260[l168 as usize] = 0.0;
		}
		for l169 in 0..3 {
			self.fRec256[l169 as usize] = 0.0;
		}
		for l170 in 0..3 {
			self.fRec255[l170 as usize] = 0.0;
		}
		for l171 in 0..2 {
			self.fRec261[l171 as usize] = 0.0;
		}
		for l172 in 0..2 {
			self.fRec262[l172 as usize] = 0.0;
		}
		for l173 in 0..2 {
			self.fRec263[l173 as usize] = 0.0;
		}
		for l174 in 0..2 {
			self.fRec265[l174 as usize] = 0.0;
		}
		for l175 in 0..2 {
			self.fRec266[l175 as usize] = 0.0;
		}
		for l176 in 0..2 {
			self.fVec21[l176 as usize] = 0.0;
		}
		for l177 in 0..4096 {
			self.fVec22[l177 as usize] = 0.0;
		}
		for l178 in 0..2 {
			self.fRec264[l178 as usize] = 0.0;
		}
		for l179 in 0..2 {
			self.fRec268[l179 as usize] = 0.0;
		}
		for l180 in 0..2 {
			self.fRec269[l180 as usize] = 0.0;
		}
		for l181 in 0..2 {
			self.fVec23[l181 as usize] = 0.0;
		}
		for l182 in 0..4096 {
			self.fVec24[l182 as usize] = 0.0;
		}
		for l183 in 0..2 {
			self.fRec267[l183 as usize] = 0.0;
		}
		for l184 in 0..2 {
			self.fRec271[l184 as usize] = 0.0;
		}
		for l185 in 0..2 {
			self.fRec272[l185 as usize] = 0.0;
		}
		for l186 in 0..2 {
			self.fVec25[l186 as usize] = 0.0;
		}
		for l187 in 0..4096 {
			self.fVec26[l187 as usize] = 0.0;
		}
		for l188 in 0..2 {
			self.fRec270[l188 as usize] = 0.0;
		}
		for l189 in 0..2 {
			self.fRec273[l189 as usize] = 0.0;
		}
		for l190 in 0..2 {
			self.fRec275[l190 as usize] = 0.0;
		}
		for l191 in 0..2 {
			self.fRec276[l191 as usize] = 0.0;
		}
		for l192 in 0..2 {
			self.fVec27[l192 as usize] = 0.0;
		}
		for l193 in 0..4096 {
			self.fVec28[l193 as usize] = 0.0;
		}
		for l194 in 0..2 {
			self.fRec274[l194 as usize] = 0.0;
		}
		for l195 in 0..2 {
			self.fRec278[l195 as usize] = 0.0;
		}
		for l196 in 0..2 {
			self.fRec279[l196 as usize] = 0.0;
		}
		for l197 in 0..2 {
			self.fVec29[l197 as usize] = 0.0;
		}
		for l198 in 0..4096 {
			self.fVec30[l198 as usize] = 0.0;
		}
		for l199 in 0..2 {
			self.fRec277[l199 as usize] = 0.0;
		}
		for l200 in 0..2 {
			self.fRec281[l200 as usize] = 0.0;
		}
		for l201 in 0..2 {
			self.fRec282[l201 as usize] = 0.0;
		}
		for l202 in 0..2 {
			self.fVec31[l202 as usize] = 0.0;
		}
		for l203 in 0..4096 {
			self.fVec32[l203 as usize] = 0.0;
		}
		for l204 in 0..2 {
			self.fRec280[l204 as usize] = 0.0;
		}
		for l205 in 0..2 {
			self.fRec283[l205 as usize] = 0.0;
		}
		for l206 in 0..2 {
			self.fRec285[l206 as usize] = 0.0;
		}
		for l207 in 0..2 {
			self.fRec286[l207 as usize] = 0.0;
		}
		for l208 in 0..2 {
			self.fVec33[l208 as usize] = 0.0;
		}
		for l209 in 0..4096 {
			self.fVec34[l209 as usize] = 0.0;
		}
		for l210 in 0..2 {
			self.fRec284[l210 as usize] = 0.0;
		}
		for l211 in 0..2 {
			self.fRec288[l211 as usize] = 0.0;
		}
		for l212 in 0..2 {
			self.fRec289[l212 as usize] = 0.0;
		}
		for l213 in 0..2 {
			self.fVec35[l213 as usize] = 0.0;
		}
		for l214 in 0..4096 {
			self.fVec36[l214 as usize] = 0.0;
		}
		for l215 in 0..2 {
			self.fRec287[l215 as usize] = 0.0;
		}
		for l216 in 0..2 {
			self.fRec291[l216 as usize] = 0.0;
		}
		for l217 in 0..2 {
			self.fRec292[l217 as usize] = 0.0;
		}
		for l218 in 0..2 {
			self.fVec37[l218 as usize] = 0.0;
		}
		for l219 in 0..4096 {
			self.fVec38[l219 as usize] = 0.0;
		}
		for l220 in 0..2 {
			self.fRec290[l220 as usize] = 0.0;
		}
		for l221 in 0..2 {
			self.fRec293[l221 as usize] = 0.0;
		}
		for l222 in 0..2 {
			self.fRec295[l222 as usize] = 0.0;
		}
		for l223 in 0..2 {
			self.fRec296[l223 as usize] = 0.0;
		}
		for l224 in 0..2 {
			self.fVec39[l224 as usize] = 0.0;
		}
		for l225 in 0..4096 {
			self.fVec40[l225 as usize] = 0.0;
		}
		for l226 in 0..2 {
			self.fRec294[l226 as usize] = 0.0;
		}
		for l227 in 0..2 {
			self.fRec298[l227 as usize] = 0.0;
		}
		for l228 in 0..2 {
			self.fRec299[l228 as usize] = 0.0;
		}
		for l229 in 0..2 {
			self.fVec41[l229 as usize] = 0.0;
		}
		for l230 in 0..4096 {
			self.fVec42[l230 as usize] = 0.0;
		}
		for l231 in 0..2 {
			self.fRec297[l231 as usize] = 0.0;
		}
		for l232 in 0..2 {
			self.fRec301[l232 as usize] = 0.0;
		}
		for l233 in 0..2 {
			self.fRec302[l233 as usize] = 0.0;
		}
		for l234 in 0..2 {
			self.fVec43[l234 as usize] = 0.0;
		}
		for l235 in 0..4096 {
			self.fVec44[l235 as usize] = 0.0;
		}
		for l236 in 0..2 {
			self.fRec300[l236 as usize] = 0.0;
		}
		for l237 in 0..2 {
			self.fRec303[l237 as usize] = 0.0;
		}
		for l238 in 0..2 {
			self.fRec304[l238 as usize] = 0.0;
		}
		for l239 in 0..2 {
			self.fRec305[l239 as usize] = 0.0;
		}
		for l240 in 0..2 {
			self.fRec307[l240 as usize] = 0.0;
		}
		for l241 in 0..2097152 {
			self.fRec306[l241 as usize] = 0.0;
		}
		for l242 in 0..2 {
			self.fRec318[l242 as usize] = 0.0;
		}
		for l243 in 0..2 {
			self.fRec320[l243 as usize] = 0.0;
		}
		for l244 in 0..2 {
			self.fRec324[l244 as usize] = 0.0;
		}
		for l245 in 0..16384 {
			self.fVec45[l245 as usize] = 0.0;
		}
		for l246 in 0..2 {
			self.fVec46[l246 as usize] = 0.0;
		}
		for l247 in 0..2 {
			self.fRec323[l247 as usize] = 0.0;
		}
		for l248 in 0..2 {
			self.fRec321[l248 as usize] = 0.0;
		}
		for l249 in 0..2 {
			self.fRec326[l249 as usize] = 0.0;
		}
		for l250 in 0..16384 {
			self.fVec47[l250 as usize] = 0.0;
		}
		for l251 in 0..2 {
			self.fVec48[l251 as usize] = 0.0;
		}
		for l252 in 0..2 {
			self.fRec325[l252 as usize] = 0.0;
		}
		for l253 in 0..2 {
			self.fRec322[l253 as usize] = 0.0;
		}
		for l254 in 0..2 {
			self.fRec330[l254 as usize] = 0.0;
		}
		for l255 in 0..16384 {
			self.fVec49[l255 as usize] = 0.0;
		}
		for l256 in 0..2 {
			self.fVec50[l256 as usize] = 0.0;
		}
		for l257 in 0..2 {
			self.fRec329[l257 as usize] = 0.0;
		}
		for l258 in 0..2 {
			self.fRec327[l258 as usize] = 0.0;
		}
		for l259 in 0..2 {
			self.fRec332[l259 as usize] = 0.0;
		}
		for l260 in 0..16384 {
			self.fVec51[l260 as usize] = 0.0;
		}
		for l261 in 0..2 {
			self.fVec52[l261 as usize] = 0.0;
		}
		for l262 in 0..2 {
			self.fRec331[l262 as usize] = 0.0;
		}
		for l263 in 0..2 {
			self.fRec328[l263 as usize] = 0.0;
		}
		for l264 in 0..2 {
			self.fRec336[l264 as usize] = 0.0;
		}
		for l265 in 0..16384 {
			self.fVec53[l265 as usize] = 0.0;
		}
		for l266 in 0..2 {
			self.fVec54[l266 as usize] = 0.0;
		}
		for l267 in 0..2 {
			self.fRec335[l267 as usize] = 0.0;
		}
		for l268 in 0..2 {
			self.fRec333[l268 as usize] = 0.0;
		}
		for l269 in 0..2 {
			self.fRec338[l269 as usize] = 0.0;
		}
		for l270 in 0..16384 {
			self.fVec55[l270 as usize] = 0.0;
		}
		for l271 in 0..2 {
			self.fVec56[l271 as usize] = 0.0;
		}
		for l272 in 0..2 {
			self.fRec337[l272 as usize] = 0.0;
		}
		for l273 in 0..2 {
			self.fRec334[l273 as usize] = 0.0;
		}
		for l274 in 0..2 {
			self.fRec342[l274 as usize] = 0.0;
		}
		for l275 in 0..16384 {
			self.fVec57[l275 as usize] = 0.0;
		}
		for l276 in 0..2 {
			self.fVec58[l276 as usize] = 0.0;
		}
		for l277 in 0..2 {
			self.fRec341[l277 as usize] = 0.0;
		}
		for l278 in 0..2 {
			self.fRec339[l278 as usize] = 0.0;
		}
		for l279 in 0..2 {
			self.fRec344[l279 as usize] = 0.0;
		}
		for l280 in 0..16384 {
			self.fVec59[l280 as usize] = 0.0;
		}
		for l281 in 0..2 {
			self.fVec60[l281 as usize] = 0.0;
		}
		for l282 in 0..2 {
			self.fRec343[l282 as usize] = 0.0;
		}
		for l283 in 0..2 {
			self.fRec340[l283 as usize] = 0.0;
		}
		for l284 in 0..2 {
			self.fRec348[l284 as usize] = 0.0;
		}
		for l285 in 0..16384 {
			self.fVec61[l285 as usize] = 0.0;
		}
		for l286 in 0..2 {
			self.fVec62[l286 as usize] = 0.0;
		}
		for l287 in 0..2 {
			self.fRec347[l287 as usize] = 0.0;
		}
		for l288 in 0..2 {
			self.fRec345[l288 as usize] = 0.0;
		}
		for l289 in 0..2 {
			self.fRec350[l289 as usize] = 0.0;
		}
		for l290 in 0..16384 {
			self.fVec63[l290 as usize] = 0.0;
		}
		for l291 in 0..2 {
			self.fVec64[l291 as usize] = 0.0;
		}
		for l292 in 0..2 {
			self.fRec349[l292 as usize] = 0.0;
		}
		for l293 in 0..2 {
			self.fRec346[l293 as usize] = 0.0;
		}
		for l294 in 0..1024 {
			self.fVec65[l294 as usize] = 0.0;
		}
		for l295 in 0..2 {
			self.fRec351[l295 as usize] = 0.0;
		}
		for l296 in 0..2 {
			self.fRec352[l296 as usize] = 0.0;
		}
		for l297 in 0..16384 {
			self.fVec66[l297 as usize] = 0.0;
		}
		for l298 in 0..2 {
			self.fVec67[l298 as usize] = 0.0;
		}
		for l299 in 0..2 {
			self.fRec319[l299 as usize] = 0.0;
		}
		for l300 in 0..2 {
			self.fRec356[l300 as usize] = 0.0;
		}
		for l301 in 0..2 {
			self.fRec358[l301 as usize] = 0.0;
		}
		for l302 in 0..1024 {
			self.fVec68[l302 as usize] = 0.0;
		}
		for l303 in 0..16384 {
			self.fVec69[l303 as usize] = 0.0;
		}
		for l304 in 0..2 {
			self.fVec70[l304 as usize] = 0.0;
		}
		for l305 in 0..2 {
			self.fRec357[l305 as usize] = 0.0;
		}
		for l306 in 0..16384 {
			self.fVec71[l306 as usize] = 0.0;
		}
		for l307 in 0..2 {
			self.fVec72[l307 as usize] = 0.0;
		}
		for l308 in 0..2 {
			self.fRec355[l308 as usize] = 0.0;
		}
		for l309 in 0..2 {
			self.fRec353[l309 as usize] = 0.0;
		}
		for l310 in 0..2 {
			self.fRec360[l310 as usize] = 0.0;
		}
		for l311 in 0..16384 {
			self.fVec73[l311 as usize] = 0.0;
		}
		for l312 in 0..2 {
			self.fVec74[l312 as usize] = 0.0;
		}
		for l313 in 0..2 {
			self.fRec359[l313 as usize] = 0.0;
		}
		for l314 in 0..2 {
			self.fRec354[l314 as usize] = 0.0;
		}
		for l315 in 0..2 {
			self.fRec364[l315 as usize] = 0.0;
		}
		for l316 in 0..16384 {
			self.fVec75[l316 as usize] = 0.0;
		}
		for l317 in 0..2 {
			self.fVec76[l317 as usize] = 0.0;
		}
		for l318 in 0..2 {
			self.fRec363[l318 as usize] = 0.0;
		}
		for l319 in 0..2 {
			self.fRec361[l319 as usize] = 0.0;
		}
		for l320 in 0..2 {
			self.fRec366[l320 as usize] = 0.0;
		}
		for l321 in 0..16384 {
			self.fVec77[l321 as usize] = 0.0;
		}
		for l322 in 0..2 {
			self.fVec78[l322 as usize] = 0.0;
		}
		for l323 in 0..2 {
			self.fRec365[l323 as usize] = 0.0;
		}
		for l324 in 0..2 {
			self.fRec362[l324 as usize] = 0.0;
		}
		for l325 in 0..2 {
			self.fRec370[l325 as usize] = 0.0;
		}
		for l326 in 0..16384 {
			self.fVec79[l326 as usize] = 0.0;
		}
		for l327 in 0..2 {
			self.fVec80[l327 as usize] = 0.0;
		}
		for l328 in 0..2 {
			self.fRec369[l328 as usize] = 0.0;
		}
		for l329 in 0..2 {
			self.fRec367[l329 as usize] = 0.0;
		}
		for l330 in 0..2 {
			self.fRec372[l330 as usize] = 0.0;
		}
		for l331 in 0..16384 {
			self.fVec81[l331 as usize] = 0.0;
		}
		for l332 in 0..2 {
			self.fVec82[l332 as usize] = 0.0;
		}
		for l333 in 0..2 {
			self.fRec371[l333 as usize] = 0.0;
		}
		for l334 in 0..2 {
			self.fRec368[l334 as usize] = 0.0;
		}
		for l335 in 0..2 {
			self.fRec376[l335 as usize] = 0.0;
		}
		for l336 in 0..16384 {
			self.fVec83[l336 as usize] = 0.0;
		}
		for l337 in 0..2 {
			self.fVec84[l337 as usize] = 0.0;
		}
		for l338 in 0..2 {
			self.fRec375[l338 as usize] = 0.0;
		}
		for l339 in 0..2 {
			self.fRec373[l339 as usize] = 0.0;
		}
		for l340 in 0..2 {
			self.fRec378[l340 as usize] = 0.0;
		}
		for l341 in 0..16384 {
			self.fVec85[l341 as usize] = 0.0;
		}
		for l342 in 0..2 {
			self.fVec86[l342 as usize] = 0.0;
		}
		for l343 in 0..2 {
			self.fRec377[l343 as usize] = 0.0;
		}
		for l344 in 0..2 {
			self.fRec374[l344 as usize] = 0.0;
		}
		for l345 in 0..2 {
			self.fRec382[l345 as usize] = 0.0;
		}
		for l346 in 0..16384 {
			self.fVec87[l346 as usize] = 0.0;
		}
		for l347 in 0..2 {
			self.fVec88[l347 as usize] = 0.0;
		}
		for l348 in 0..2 {
			self.fRec381[l348 as usize] = 0.0;
		}
		for l349 in 0..2 {
			self.fRec379[l349 as usize] = 0.0;
		}
		for l350 in 0..2 {
			self.fRec384[l350 as usize] = 0.0;
		}
		for l351 in 0..16384 {
			self.fVec89[l351 as usize] = 0.0;
		}
		for l352 in 0..2 {
			self.fVec90[l352 as usize] = 0.0;
		}
		for l353 in 0..2 {
			self.fRec383[l353 as usize] = 0.0;
		}
		for l354 in 0..2 {
			self.fRec380[l354 as usize] = 0.0;
		}
		for l355 in 0..16384 {
			self.fVec91[l355 as usize] = 0.0;
		}
		for l356 in 0..16384 {
			self.fVec92[l356 as usize] = 0.0;
		}
		for l357 in 0..2 {
			self.fVec93[l357 as usize] = 0.0;
		}
		for l358 in 0..2 {
			self.fRec317[l358 as usize] = 0.0;
		}
		for l359 in 0..2 {
			self.fRec316[l359 as usize] = 0.0;
		}
		for l360 in 0..3 {
			self.fRec315[l360 as usize] = 0.0;
		}
		for l361 in 0..3 {
			self.fRec314[l361 as usize] = 0.0;
		}
		for l362 in 0..2 {
			self.fVec94[l362 as usize] = 0.0;
		}
		for l363 in 0..2 {
			self.fRec313[l363 as usize] = 0.0;
		}
		for l364 in 0..3 {
			self.fRec312[l364 as usize] = 0.0;
		}
		for l365 in 0..3 {
			self.fRec311[l365 as usize] = 0.0;
		}
		for l366 in 0..2 {
			self.fRec387[l366 as usize] = 0.0;
		}
		for l367 in 0..3 {
			self.fRec386[l367 as usize] = 0.0;
		}
		for l368 in 0..3 {
			self.fRec385[l368 as usize] = 0.0;
		}
		for l369 in 0..2 {
			self.fRec391[l369 as usize] = 0.0;
		}
		for l370 in 0..3 {
			self.fRec390[l370 as usize] = 0.0;
		}
		for l371 in 0..3 {
			self.fRec389[l371 as usize] = 0.0;
		}
		for l372 in 0..3 {
			self.fRec388[l372 as usize] = 0.0;
		}
		for l373 in 0..1024 {
			self.fVec95[l373 as usize] = 0.0;
		}
		for l374 in 0..2 {
			self.fRec310[l374 as usize] = 0.0;
		}
		for l375 in 0..2 {
			self.fRec403[l375 as usize] = 0.0;
		}
		for l376 in 0..16384 {
			self.fVec96[l376 as usize] = 0.0;
		}
		for l377 in 0..16384 {
			self.fVec97[l377 as usize] = 0.0;
		}
		for l378 in 0..2 {
			self.fVec98[l378 as usize] = 0.0;
		}
		for l379 in 0..2 {
			self.fRec402[l379 as usize] = 0.0;
		}
		for l380 in 0..2 {
			self.fRec401[l380 as usize] = 0.0;
		}
		for l381 in 0..3 {
			self.fRec400[l381 as usize] = 0.0;
		}
		for l382 in 0..3 {
			self.fRec399[l382 as usize] = 0.0;
		}
		for l383 in 0..2 {
			self.fVec99[l383 as usize] = 0.0;
		}
		for l384 in 0..2 {
			self.fRec398[l384 as usize] = 0.0;
		}
		for l385 in 0..3 {
			self.fRec397[l385 as usize] = 0.0;
		}
		for l386 in 0..3 {
			self.fRec396[l386 as usize] = 0.0;
		}
		for l387 in 0..2 {
			self.fRec406[l387 as usize] = 0.0;
		}
		for l388 in 0..3 {
			self.fRec405[l388 as usize] = 0.0;
		}
		for l389 in 0..3 {
			self.fRec404[l389 as usize] = 0.0;
		}
		for l390 in 0..2 {
			self.fRec410[l390 as usize] = 0.0;
		}
		for l391 in 0..3 {
			self.fRec409[l391 as usize] = 0.0;
		}
		for l392 in 0..3 {
			self.fRec408[l392 as usize] = 0.0;
		}
		for l393 in 0..3 {
			self.fRec407[l393 as usize] = 0.0;
		}
		for l394 in 0..1024 {
			self.fVec100[l394 as usize] = 0.0;
		}
		for l395 in 0..2 {
			self.fRec395[l395 as usize] = 0.0;
		}
		for l396 in 0..16384 {
			self.fVec101[l396 as usize] = 0.0;
		}
		for l397 in 0..2 {
			self.fVec102[l397 as usize] = 0.0;
		}
		for l398 in 0..2 {
			self.fRec394[l398 as usize] = 0.0;
		}
		for l399 in 0..2 {
			self.fRec392[l399 as usize] = 0.0;
		}
		for l400 in 0..2 {
			self.fRec412[l400 as usize] = 0.0;
		}
		for l401 in 0..16384 {
			self.fVec103[l401 as usize] = 0.0;
		}
		for l402 in 0..2 {
			self.fVec104[l402 as usize] = 0.0;
		}
		for l403 in 0..2 {
			self.fRec411[l403 as usize] = 0.0;
		}
		for l404 in 0..2 {
			self.fRec393[l404 as usize] = 0.0;
		}
		for l405 in 0..16384 {
			self.fVec105[l405 as usize] = 0.0;
		}
		for l406 in 0..2 {
			self.fVec106[l406 as usize] = 0.0;
		}
		for l407 in 0..2 {
			self.fRec415[l407 as usize] = 0.0;
		}
		for l408 in 0..2 {
			self.fRec413[l408 as usize] = 0.0;
		}
		for l409 in 0..16384 {
			self.fVec107[l409 as usize] = 0.0;
		}
		for l410 in 0..2 {
			self.fVec108[l410 as usize] = 0.0;
		}
		for l411 in 0..2 {
			self.fRec416[l411 as usize] = 0.0;
		}
		for l412 in 0..2 {
			self.fRec414[l412 as usize] = 0.0;
		}
		for l413 in 0..16384 {
			self.fVec109[l413 as usize] = 0.0;
		}
		for l414 in 0..2 {
			self.fVec110[l414 as usize] = 0.0;
		}
		for l415 in 0..2 {
			self.fRec419[l415 as usize] = 0.0;
		}
		for l416 in 0..2 {
			self.fRec417[l416 as usize] = 0.0;
		}
		for l417 in 0..2 {
			self.fRec421[l417 as usize] = 0.0;
		}
		for l418 in 0..16384 {
			self.fVec111[l418 as usize] = 0.0;
		}
		for l419 in 0..2 {
			self.fVec112[l419 as usize] = 0.0;
		}
		for l420 in 0..2 {
			self.fRec420[l420 as usize] = 0.0;
		}
		for l421 in 0..2 {
			self.fRec418[l421 as usize] = 0.0;
		}
		for l422 in 0..2 {
			self.fRec425[l422 as usize] = 0.0;
		}
		for l423 in 0..16384 {
			self.fVec113[l423 as usize] = 0.0;
		}
		for l424 in 0..2 {
			self.fVec114[l424 as usize] = 0.0;
		}
		for l425 in 0..2 {
			self.fRec424[l425 as usize] = 0.0;
		}
		for l426 in 0..2 {
			self.fRec422[l426 as usize] = 0.0;
		}
		for l427 in 0..16384 {
			self.fVec115[l427 as usize] = 0.0;
		}
		for l428 in 0..2 {
			self.fVec116[l428 as usize] = 0.0;
		}
		for l429 in 0..2 {
			self.fRec426[l429 as usize] = 0.0;
		}
		for l430 in 0..2 {
			self.fRec423[l430 as usize] = 0.0;
		}
		for l431 in 0..2 {
			self.fRec308[l431 as usize] = 0.0;
		}
		for l432 in 0..2 {
			self.fRec309[l432 as usize] = 0.0;
		}
		for l433 in 0..2 {
			self.fRec1[l433 as usize] = 0.0;
		}
		for l434 in 0..2 {
			self.fRec0[l434 as usize] = 0.0;
		}
		for l435 in 0..2 {
			self.fRec427[l435 as usize] = 0.0;
		}
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		self.fConst0 = F32::min(1.92e+05, F32::max(1.0, (self.fSampleRate) as F32));
		self.fConst1 = F32::exp(0.0 - 2.5e+03 / self.fConst0);
		self.fConst2 = 44.1 / self.fConst0;
		self.fConst3 = 1.0 - self.fConst2;
		self.fConst4 = 3.1415927 / self.fConst0;
		self.fConst5 = 0.00882353 * self.fConst0;
		self.fConst6 = 0.00073529413 * self.fConst0;
		self.fConst7 = F32::exp(0.0 - 1e+04 / self.fConst0);
		self.fConst8 = 1.0 - self.fConst7;
		self.iConst9 = (0.1 * self.fConst0) as i32;
		self.fConst10 = F32::exp(0.0 - 5e+01 / self.fConst0);
		self.fConst11 = 15.707963 / self.fConst0;
		self.fConst12 = 0.002 * self.fConst0;
		self.fConst13 = F32::exp(0.0 - 1e+01 / self.fConst0);
		self.fConst14 = 1.0 / self.fConst0;
		self.fConst15 = 0.5 * self.fConst0;
		self.fConst16 = 0.25 * self.fConst0;
		self.fConst17 = 1.3333334 / self.fConst0;
		self.fConst18 = 1e+01 * self.fConst0;
		let mut fConst19: F32 = F32::tan(1382.3008 / self.fConst0);
		let mut fConst20: F32 = mydsp_faustpower2_f(fConst19);
		self.fConst21 = 1.0 / fConst20;
		self.fConst22 = 2.0 * (1.0 - self.fConst21);
		let mut fConst23: F32 = 1.0 / fConst19;
		self.fConst24 = (fConst23 + -0.618034) / fConst19 + 1.0;
		self.fConst25 = 1.0 / ((fConst23 + 0.618034) / fConst19 + 1.0);
		self.fConst26 = (fConst23 + -1.618034) / fConst19 + 1.0;
		self.fConst27 = 1.0 / ((fConst23 + 1.618034) / fConst19 + 1.0);
		let mut fConst28: F32 = F32::tan(25132.742 / self.fConst0);
		let mut fConst29: F32 = mydsp_faustpower2_f(fConst28);
		self.fConst30 = 1.0 / fConst29;
		self.fConst31 = 2.0 * (1.0 - self.fConst30);
		self.fConst32 = 1.0 / fConst28;
		self.fConst33 = (self.fConst32 + -0.618034) / fConst28 + 1.0;
		self.fConst34 = 1.0 / ((self.fConst32 + 0.618034) / fConst28 + 1.0);
		self.fConst35 = (self.fConst32 + -1.618034) / fConst28 + 1.0;
		self.fConst36 = 1.0 / ((self.fConst32 + 1.618034) / fConst28 + 1.0);
		self.fConst37 = 6.2831855 / self.fConst0;
		self.fConst38 = 1.0 - self.fConst32;
		let mut fConst39: F32 = self.fConst32 + 1.0;
		self.fConst40 = 1.0 / fConst39;
		self.fConst41 = 1.0 - fConst23;
		let mut fConst42: F32 = fConst23 + 1.0;
		self.fConst43 = 1.0 / fConst42;
		self.fConst44 = self.fConst41 / fConst42;
		self.fConst45 = 1.0 / (fConst19 * fConst42);
		self.fConst46 = 0.0 - self.fConst45;
		self.fConst47 = 0.0 - 2.0 / fConst20;
		self.fConst48 = (fConst23 + -1.618034) / fConst19 + 1.0;
		self.fConst49 = 1.0 / ((fConst23 + 1.618034) / fConst19 + 1.0);
		self.fConst50 = 0.0 - 1.0 / (fConst28 * fConst39);
		self.fConst51 = 0.0 - 2.0 / fConst29;
		self.fConst52 = F32::exp(0.0 - 2.0 / self.fConst0);
		self.fConst53 = F32::exp(0.0 - 1.25e+03 / self.fConst0);
		self.fConst54 = 1.0 - self.fConst1;
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
			18 => Some(self.fButton0),
			16 => Some(self.fButton1),
			14 => Some(self.fButton2),
			12 => Some(self.fButton3),
			10 => Some(self.fButton4),
			45 => Some(self.fHslider0),
			44 => Some(self.fHslider1),
			7 => Some(self.fHslider10),
			8 => Some(self.fHslider11),
			5 => Some(self.fHslider12),
			6 => Some(self.fHslider13),
			3 => Some(self.fHslider14),
			4 => Some(self.fHslider15),
			1 => Some(self.fHslider16),
			2 => Some(self.fHslider17),
			42 => Some(self.fHslider18),
			0 => Some(self.fHslider19),
			19 => Some(self.fHslider2),
			29 => Some(self.fHslider20),
			20 => Some(self.fHslider21),
			28 => Some(self.fHslider22),
			27 => Some(self.fHslider23),
			26 => Some(self.fHslider24),
			25 => Some(self.fHslider25),
			24 => Some(self.fHslider26),
			23 => Some(self.fHslider27),
			22 => Some(self.fHslider28),
			21 => Some(self.fHslider29),
			46 => Some(self.fHslider3),
			41 => Some(self.fHslider30),
			31 => Some(self.fHslider31),
			30 => Some(self.fHslider32),
			32 => Some(self.fHslider33),
			33 => Some(self.fHslider34),
			35 => Some(self.fHslider35),
			36 => Some(self.fHslider36),
			39 => Some(self.fHslider37),
			38 => Some(self.fHslider38),
			34 => Some(self.fHslider39),
			9 => Some(self.fHslider4),
			37 => Some(self.fHslider40),
			40 => Some(self.fHslider41),
			17 => Some(self.fHslider5),
			15 => Some(self.fHslider6),
			13 => Some(self.fHslider7),
			11 => Some(self.fHslider8),
			43 => Some(self.fHslider9),
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			18 => { self.fButton0 = value }
			16 => { self.fButton1 = value }
			14 => { self.fButton2 = value }
			12 => { self.fButton3 = value }
			10 => { self.fButton4 = value }
			45 => { self.fHslider0 = value }
			44 => { self.fHslider1 = value }
			7 => { self.fHslider10 = value }
			8 => { self.fHslider11 = value }
			5 => { self.fHslider12 = value }
			6 => { self.fHslider13 = value }
			3 => { self.fHslider14 = value }
			4 => { self.fHslider15 = value }
			1 => { self.fHslider16 = value }
			2 => { self.fHslider17 = value }
			42 => { self.fHslider18 = value }
			0 => { self.fHslider19 = value }
			19 => { self.fHslider2 = value }
			29 => { self.fHslider20 = value }
			20 => { self.fHslider21 = value }
			28 => { self.fHslider22 = value }
			27 => { self.fHslider23 = value }
			26 => { self.fHslider24 = value }
			25 => { self.fHslider25 = value }
			24 => { self.fHslider26 = value }
			23 => { self.fHslider27 = value }
			22 => { self.fHslider28 = value }
			21 => { self.fHslider29 = value }
			46 => { self.fHslider3 = value }
			41 => { self.fHslider30 = value }
			31 => { self.fHslider31 = value }
			30 => { self.fHslider32 = value }
			32 => { self.fHslider33 = value }
			33 => { self.fHslider34 = value }
			35 => { self.fHslider35 = value }
			36 => { self.fHslider36 = value }
			39 => { self.fHslider37 = value }
			38 => { self.fHslider38 = value }
			34 => { self.fHslider39 = value }
			9 => { self.fHslider4 = value }
			37 => { self.fHslider40 = value }
			40 => { self.fHslider41 = value }
			17 => { self.fHslider5 = value }
			15 => { self.fHslider6 = value }
			13 => { self.fHslider7 = value }
			11 => { self.fHslider8 = value }
			43 => { self.fHslider9 = value }
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
		let mut fSlow0: F32 = self.fConst2 * self.fHslider0;
		let mut fSlow1: F32 = self.fConst2 * self.fHslider1;
		let mut fSlow2: F32 = self.fHslider2;
		let mut fSlow3: F32 = self.fConst2 * self.fHslider3;
		let mut fSlow4: F32 = self.fButton0;
		let mut fSlow5: F32 = self.fHslider4;
		let mut fSlow6: F32 = self.fHslider5;
		let mut fSlow7: F32 = self.fButton1;
		let mut fSlow8: F32 = self.fHslider6;
		let mut fSlow9: F32 = self.fButton2;
		let mut fSlow10: F32 = self.fHslider7;
		let mut fSlow11: F32 = self.fButton3;
		let mut fSlow12: F32 = self.fHslider8;
		let mut fSlow13: F32 = self.fButton4;
		let mut fSlow14: F32 = self.fConst2 * self.fHslider9;
		let mut fSlow15: F32 = self.fHslider10;
		let mut fSlow16: F32 = self.fConst2 * self.fHslider11;
		let mut fSlow17: F32 = self.fHslider12;
		let mut fSlow18: F32 = self.fConst2 * self.fHslider13;
		let mut fSlow19: F32 = self.fHslider14;
		let mut fSlow20: F32 = self.fConst2 * self.fHslider15;
		let mut fSlow21: F32 = self.fHslider16;
		let mut fSlow22: F32 = self.fConst2 * self.fHslider17;
		let mut fSlow23: F32 = self.fConst2 * self.fHslider18;
		let mut fSlow24: F32 = self.fConst2 * self.fHslider19;
		let mut fSlow25: F32 = self.fConst2 * self.fHslider20;
		let mut fSlow26: F32 = self.fHslider21;
		let mut fSlow27: F32 = self.fHslider22;
		let mut fSlow28: F32 = self.fConst2 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow27 - fSlow26) + 0.5) as i32, 2047))) as usize] };
		let mut fSlow29: F32 = self.fConst2 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow26 + fSlow27) + 0.5) as i32, 2047))) as usize] };
		let mut fSlow30: F32 = self.fConst2 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * fSlow27 + 0.5) as i32, 2047))) as usize] };
		let mut fSlow31: F32 = self.fConst2 * self.fHslider23;
		let mut fSlow32: F32 = self.fHslider24;
		let mut fSlow33: F32 = self.fConst2 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow32 - fSlow26) + 0.5) as i32, 2047))) as usize] };
		let mut fSlow34: F32 = self.fConst2 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow26 + fSlow32) + 0.5) as i32, 2047))) as usize] };
		let mut fSlow35: F32 = self.fConst2 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * fSlow32 + 0.5) as i32, 2047))) as usize] };
		let mut fSlow36: F32 = self.fConst2 * self.fHslider25;
		let mut fSlow37: F32 = self.fHslider26;
		let mut fSlow38: F32 = self.fConst2 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow37 - fSlow26) + 0.5) as i32, 2047))) as usize] };
		let mut fSlow39: F32 = self.fConst2 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow26 + fSlow37) + 0.5) as i32, 2047))) as usize] };
		let mut fSlow40: F32 = self.fConst2 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * fSlow37 + 0.5) as i32, 2047))) as usize] };
		let mut fSlow41: F32 = self.fConst2 * self.fHslider27;
		let mut fSlow42: F32 = self.fHslider28;
		let mut fSlow43: F32 = self.fConst2 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow42 - fSlow26) + 0.5) as i32, 2047))) as usize] };
		let mut fSlow44: F32 = self.fConst2 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow26 + fSlow42) + 0.5) as i32, 2047))) as usize] };
		let mut fSlow45: F32 = self.fConst2 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * fSlow42 + 0.5) as i32, 2047))) as usize] };
		let mut fSlow46: F32 = self.fConst2 * self.fHslider29;
		let mut fSlow47: F32 = self.fConst2 * self.fHslider30;
		let mut fSlow48: F32 = self.fConst2 * self.fHslider31;
		let mut fSlow49: F32 = self.fConst2 * self.fHslider32;
		let mut fSlow50: F32 = self.fHslider33;
		let mut fSlow51: F32 = self.fHslider34;
		let mut fSlow52: F32 = 1.0 - fSlow51;
		let mut fSlow53: F32 = self.fHslider35;
		let mut fSlow54: F32 = self.fHslider36;
		let mut iSlow55: i32 = unsafe { itbl1mydspSIG1[((134.0 * fSlow54) as i32) as usize] };
		let mut fSlow56: F32 = 0.005 * (iSlow55) as F32;
		let mut iSlow57: i32 = unsafe { itbl1mydspSIG1[((54.0 * fSlow54) as i32) as usize] };
		let mut fSlow58: F32 = 0.005 * (iSlow57) as F32;
		let mut iSlow59: i32 = unsafe { itbl1mydspSIG1[((1e+01 * fSlow54) as i32) as usize] };
		let mut fSlow60: F32 = 0.0001 * (iSlow59) as F32;
		let mut iSlow61: i32 = unsafe { itbl1mydspSIG1[((1.1e+02 * fSlow54) as i32) as usize] };
		let mut fSlow62: F32 = 0.0001 * (iSlow61) as F32;
		let mut iSlow63: i32 = unsafe { itbl1mydspSIG1[((4e+01 * fSlow54) as i32) as usize] };
		let mut fSlow64: F32 = 0.0001 * (iSlow63) as F32;
		let mut iSlow65: i32 = unsafe { itbl1mydspSIG1[((1.4e+02 * fSlow54) as i32) as usize] };
		let mut fSlow66: F32 = 0.0001 * (iSlow65) as F32;
		let mut iSlow67: i32 = unsafe { itbl1mydspSIG1[((7e+01 * fSlow54) as i32) as usize] };
		let mut fSlow68: F32 = 0.0001 * (iSlow67) as F32;
		let mut iSlow69: i32 = unsafe { itbl1mydspSIG1[((1.7e+02 * fSlow54) as i32) as usize] };
		let mut fSlow70: F32 = 0.0001 * (iSlow69) as F32;
		let mut iSlow71: i32 = unsafe { itbl1mydspSIG1[((1e+02 * fSlow54) as i32) as usize] };
		let mut fSlow72: F32 = 0.0001 * (iSlow71) as F32;
		let mut iSlow73: i32 = unsafe { itbl1mydspSIG1[((2e+02 * fSlow54) as i32) as usize] };
		let mut fSlow74: F32 = 0.0001 * (iSlow73) as F32;
		let mut iSlow75: i32 = unsafe { itbl1mydspSIG1[((1.3e+02 * fSlow54) as i32) as usize] };
		let mut fSlow76: F32 = 0.0001 * (iSlow75) as F32;
		let mut iSlow77: i32 = unsafe { itbl1mydspSIG1[((2.3e+02 * fSlow54) as i32) as usize] };
		let mut fSlow78: F32 = 0.0001 * (iSlow77) as F32;
		let mut fSlow79: F32 = self.fConst37 * self.fHslider37;
		let mut fSlow80: F32 = F32::cos(fSlow79);
		let mut fSlow81: F32 = F32::sin(fSlow79);
		let mut fSlow82: F32 = 5e+01 * self.fHslider38;
		let mut iSlow83: i32 = unsafe { itbl1mydspSIG1[((125.0 * fSlow54) as i32) as usize] };
		let mut fSlow84: F32 = 0.0001 * (iSlow83) as F32;
		let mut iSlow85: i32 = unsafe { itbl1mydspSIG1[((204.0 * fSlow54) as i32) as usize] };
		let mut fSlow86: F32 = 0.005 * (iSlow85) as F32;
		let mut fSlow87: F32 = 0.0 - fSlow82;
		let mut iSlow88: i32 = unsafe { itbl1mydspSIG1[((25.0 * fSlow54) as i32) as usize] };
		let mut fSlow89: F32 = 0.0001 * (iSlow88) as F32;
		let mut iSlow90: i32 = unsafe { itbl1mydspSIG1[((155.0 * fSlow54) as i32) as usize] };
		let mut fSlow91: F32 = 0.0001 * (iSlow90) as F32;
		let mut iSlow92: i32 = unsafe { itbl1mydspSIG1[((55.0 * fSlow54) as i32) as usize] };
		let mut fSlow93: F32 = 0.0001 * (iSlow92) as F32;
		let mut iSlow94: i32 = unsafe { itbl1mydspSIG1[((185.0 * fSlow54) as i32) as usize] };
		let mut fSlow95: F32 = 0.0001 * (iSlow94) as F32;
		let mut iSlow96: i32 = unsafe { itbl1mydspSIG1[((85.0 * fSlow54) as i32) as usize] };
		let mut fSlow97: F32 = 0.0001 * (iSlow96) as F32;
		let mut iSlow98: i32 = unsafe { itbl1mydspSIG1[((215.0 * fSlow54) as i32) as usize] };
		let mut fSlow99: F32 = 0.0001 * (iSlow98) as F32;
		let mut iSlow100: i32 = unsafe { itbl1mydspSIG1[((115.0 * fSlow54) as i32) as usize] };
		let mut fSlow101: F32 = 0.0001 * (iSlow100) as F32;
		let mut iSlow102: i32 = unsafe { itbl1mydspSIG1[((245.0 * fSlow54) as i32) as usize] };
		let mut fSlow103: F32 = 0.0001 * (iSlow102) as F32;
		let mut iSlow104: i32 = unsafe { itbl1mydspSIG1[((145.0 * fSlow54) as i32) as usize] };
		let mut fSlow105: F32 = 0.0001 * (iSlow104) as F32;
		let mut fSlow106: F32 = F32::powf(1e+01, 0.0 - 0.51 * ((1.25 * fSlow54 + -0.25) / self.fHslider39));
		let mut fSlow107: F32 = 1.0 - fSlow53;
		let mut fSlow108: F32 = self.fHslider40;
		let mut fSlow109: F32 = F32::sin(fSlow108);
		let mut iSlow110: i32 = unsafe { itbl1mydspSIG1[((34.0 * fSlow54) as i32) as usize] };
		let mut fSlow111: F32 = 0.005 * (iSlow110) as F32;
		let mut fSlow112: F32 = F32::cos(fSlow108);
		let mut iSlow113: i32 = unsafe { itbl1mydspSIG1[((2.4e+02 * fSlow54) as i32) as usize] };
		let mut fSlow114: F32 = 0.0001 * (iSlow113) as F32;
		let mut iSlow115: i32 = unsafe { itbl1mydspSIG1[((1.9e+02 * fSlow54) as i32) as usize] };
		let mut fSlow116: F32 = 0.0001 * (iSlow115) as F32;
		let mut iSlow117: i32 = unsafe { itbl1mydspSIG1[((175.0 * fSlow54) as i32) as usize] };
		let mut fSlow118: F32 = 0.0001 * (iSlow117) as F32;
		let mut fSlow119: F32 = self.fConst2 * self.fHslider41;
		let zipped_iterators = outputs0.zip(outputs1);
		for (output0, output1) in zipped_iterators {
			self.iVec0[0] = 1;
			self.fRec2[0] = fSlow0 + self.fConst3 * self.fRec2[1];
			let mut fTemp0: F32 = F32::min(1.4141995, 1.4142135 * self.fRec2[0]);
			let mut fTemp1: F32 = 1.4142135 * fTemp0;
			let mut fTemp2: F32 = 1.0 - fTemp1;
			self.fRec5[0] = fSlow1 + self.fConst3 * self.fRec5[1];
			self.fRec3[0] = self.fConst2 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow2 + self.fRec5[0]) + 0.5) as i32, 2047))) as usize] } + self.fConst3 * self.fRec3[1];
			let mut fTemp3: F32 = F32::tan(self.fConst4 * F32::max(2e+01, F32::min(1e+04, self.fRec3[0])));
			let mut fTemp4: F32 = 1.0 / fTemp3;
			let mut fTemp5: F32 = 2.0 - fTemp1;
			let mut fTemp6: F32 = mydsp_faustpower2_f(fTemp0);
			let mut fTemp7: F32 = fTemp6 + (fTemp5 + fTemp4) / fTemp3 + fTemp2;
			let mut fTemp8: F32 = 1.0 / mydsp_faustpower2_f(fTemp3);
			let mut fTemp9: F32 = fTemp1 + 2.0;
			let mut fTemp10: F32 = fTemp1 + fTemp6;
			let mut fTemp11: F32 = fTemp10 + (fTemp9 + fTemp4) / fTemp3 + 1.0;
			let mut fRec21: F32 = -1.0 * 0.9973053 * (0.9 * self.fRec22[2] + 0.05 * (self.fRec22[1] + self.fRec22[3]));
			self.fRec41[0] = fSlow3 + self.fConst3 * self.fRec41[1];
			self.fRec40[0] = self.fConst2 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow2 + self.fRec41[0]) + 0.5) as i32, 2047))) as usize] } + self.fConst3 * self.fRec40[1];
			let mut fTemp12: F32 = self.fConst6 * (3.4e+02 / self.fRec40[0] + -0.11);
			let mut fTemp13: F32 = fTemp12 + -1.499995;
			let mut iTemp14: i32 = (fTemp13) as i32;
			let mut iTemp15: i32 = (F32::min(self.fConst5, (std::cmp::max(0, i32::wrapping_add(iTemp14, 4))) as F32)) as i32;
			let mut iTemp16: i32 = i32::wrapping_add(iTemp15, 1);
			let mut fTemp17: F32 = F32::floor(fTemp13);
			let mut fTemp18: F32 = fTemp12 + (-3.0 - fTemp17);
			let mut fTemp19: F32 = fTemp12 + (-2.0 - fTemp17);
			let mut fTemp20: F32 = fTemp12 + (-1.0 - fTemp17);
			let mut fTemp21: F32 = fTemp20 * fTemp19;
			let mut fTemp22: F32 = fTemp21 * fTemp18;
			let mut fTemp23: F32 = fTemp12 + (-4.0 - fTemp17);
			let mut fTemp24: F32 = 0.0 - fTemp23;
			let mut iTemp25: i32 = (F32::min(self.fConst5, (std::cmp::max(0, i32::wrapping_add(iTemp14, 3))) as F32)) as i32;
			let mut iTemp26: i32 = i32::wrapping_add(iTemp25, 1);
			let mut fTemp27: F32 = 0.0 - 0.5 * fTemp23;
			let mut fTemp28: F32 = 0.0 - fTemp18;
			let mut iTemp29: i32 = (F32::min(self.fConst5, (std::cmp::max(0, i32::wrapping_add(iTemp14, 2))) as F32)) as i32;
			let mut iTemp30: i32 = i32::wrapping_add(iTemp29, 1);
			let mut fTemp31: F32 = 0.0 - 0.33333334 * fTemp23;
			let mut fTemp32: F32 = 0.0 - 0.5 * fTemp18;
			let mut fTemp33: F32 = 0.0 - fTemp19;
			let mut iTemp34: i32 = (F32::min(self.fConst5, (std::cmp::max(0, i32::wrapping_add(iTemp14, 1))) as F32)) as i32;
			let mut iTemp35: i32 = i32::wrapping_add(iTemp34, 1);
			let mut fTemp36: F32 = fTemp12 - fTemp17;
			let mut fTemp37: F32 = 0.0 - 0.25 * fTemp23;
			let mut fTemp38: F32 = 0.0 - 0.33333334 * fTemp18;
			let mut fTemp39: F32 = 0.0 - 0.5 * fTemp19;
			let mut fTemp40: F32 = 0.0 - fTemp20;
			let mut iTemp41: i32 = (F32::min(self.fConst5, (std::cmp::max(0, iTemp14)) as F32)) as i32;
			let mut iTemp42: i32 = i32::wrapping_add(iTemp41, 1);
			self.fRec36[0] = self.fRec13[((i32::wrapping_sub(self.IOTA0, iTemp42)) & 2047) as usize] * fTemp40 * fTemp39 * fTemp38 * fTemp37 + fTemp36 * (self.fRec13[((i32::wrapping_sub(self.IOTA0, iTemp35)) & 2047) as usize] * fTemp33 * fTemp32 * fTemp31 + 0.5 * fTemp20 * self.fRec13[((i32::wrapping_sub(self.IOTA0, iTemp30)) & 2047) as usize] * fTemp28 * fTemp27 + 0.16666667 * fTemp21 * self.fRec13[((i32::wrapping_sub(self.IOTA0, iTemp26)) & 2047) as usize] * fTemp24 + 0.041666668 * fTemp22 * self.fRec13[((i32::wrapping_sub(self.IOTA0, iTemp16)) & 2047) as usize]);
			self.fRec42[0] = 0.95 * self.fRec36[1] + 0.05 * self.fRec42[1];
			let mut fRec37: F32 = self.fRec42[0];
			self.fRec45[0] = self.fConst8 * F32::abs(self.fRec8[1]) + self.fConst7 * self.fRec45[1];
			let mut iTemp43: i32 = (self.fRec45[0] > 0.1) as i32;
			self.iVec1[0] = iTemp43;
			self.iRec46[0] = std::cmp::max(i32::wrapping_mul(self.iConst9, (iTemp43 < self.iVec1[1]) as i32), i32::wrapping_add(self.iRec46[1], -1));
			let mut fTemp44: F32 = F32::abs(F32::max((iTemp43) as F32, ((self.iRec46[0] > 0) as i32) as u32 as F32));
			let mut fTemp45: F32 = if (fTemp44 > self.fRec44[1]) as i32 != 0 {self.fConst7} else {self.fConst10};
			self.fRec44[0] = fTemp44 * (1.0 - fTemp45) + self.fRec44[1] * fTemp45;
			let mut fTemp46: F32 = 0.005 * self.fRec44[0] * self.fRec8[1];
			self.fRec47[0] = self.fRec11[1];
			self.fRec48[(self.IOTA0 & 2047) as usize] = -1.0 * 0.9973053 * (0.9 * self.fRec47[2] + 0.05 * (self.fRec47[1] + self.fRec47[3]));
			let mut fTemp47: F32 = fTemp21 * fTemp24;
			let mut fTemp48: F32 = fTemp20 * fTemp28 * fTemp27;
			let mut fTemp49: F32 = fTemp33 * fTemp32 * fTemp31;
			let mut fTemp50: F32 = fTemp40 * fTemp39 * fTemp38 * fTemp37;
			self.fVec2[0] = fTemp50 * self.fRec48[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp41, 2))) & 2047) as usize] + fTemp36 * (fTemp49 * self.fRec48[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp34, 2))) & 2047) as usize] + 0.5 * fTemp48 * self.fRec48[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp29, 2))) & 2047) as usize] + 0.16666667 * fTemp47 * self.fRec48[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp25, 2))) & 2047) as usize] + 0.041666668 * fTemp22 * self.fRec48[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp15, 2))) & 2047) as usize]);
			let mut fTemp51: F32 = F32::tan(self.fConst11 * self.fRec40[0]);
			let mut fTemp52: F32 = 1.0 / fTemp51;
			let mut fTemp53: F32 = (fTemp52 + 1.4142135) / fTemp51 + 1.0;
			self.fVec3[0] = fSlow4;
			self.iRec49[0] = i32::wrapping_add(i32::wrapping_mul(i32::wrapping_add(self.iRec49[1], (self.iRec49[1] > 0) as i32), (fSlow4 <= self.fVec3[1]) as i32), (fSlow4 > self.fVec3[1]) as i32);
			let mut fTemp54: F32 = (self.iRec49[0]) as F32 / F32::max(1.0, self.fConst12 * mydsp_faustpower2_f(1.0 - 0.0005 * self.fRec40[0]));
			self.iRec51[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, self.iRec51[1]), 12345);
			let mut fTemp55: F32 = 4.656613e-10 * (self.iRec51[0]) as F32;
			self.fRec50[0] = fTemp55 - (self.fRec50[2] * ((fTemp52 + -1.4142135) / fTemp51 + 1.0) + 2.0 * self.fRec50[1] * (1.0 - 1.0 / mydsp_faustpower2_f(fTemp51))) / fTemp53;
			let mut fTemp56: F32 = 0.5 * ((self.fRec50[2] + self.fRec50[0] + 2.0 * self.fRec50[1]) * F32::max(0.0, F32::min(fTemp54, 2.0 - fTemp54)) / fTemp53);
			let mut fTemp57: F32 = fTemp56 + self.fVec2[1] + fTemp46;
			self.fVec4[0] = fTemp57;
			self.fRec43[(self.IOTA0 & 2047) as usize] = 0.95 * self.fVec4[2] + 0.05 * self.fRec43[((i32::wrapping_sub(self.IOTA0, 1)) & 2047) as usize];
			let mut fRec38: F32 = fTemp50 * self.fRec43[((i32::wrapping_sub(self.IOTA0, iTemp41)) & 2047) as usize] + fTemp36 * (fTemp49 * self.fRec43[((i32::wrapping_sub(self.IOTA0, iTemp34)) & 2047) as usize] + 0.5 * fTemp48 * self.fRec43[((i32::wrapping_sub(self.IOTA0, iTemp29)) & 2047) as usize] + 0.16666667 * fTemp47 * self.fRec43[((i32::wrapping_sub(self.IOTA0, iTemp25)) & 2047) as usize] + 0.041666668 * fTemp22 * self.fRec43[((i32::wrapping_sub(self.IOTA0, iTemp15)) & 2047) as usize]);
			let mut fRec39: F32 = self.fVec4[1] + self.fRec32[1];
			self.fRec32[0] = fRec37;
			let mut fRec33: F32 = self.fRec32[1];
			let mut fRec34: F32 = fRec38;
			let mut fRec35: F32 = fRec39;
			self.fRec28[0] = fRec33;
			let mut fRec29: F32 = fTemp46 + fTemp56 + self.fRec28[1];
			let mut fRec30: F32 = fRec34;
			let mut fRec31: F32 = fRec35;
			self.fRec24[(self.IOTA0 & 2047) as usize] = fRec29;
			let mut fRec25: F32 = fTemp50 * self.fRec24[((i32::wrapping_sub(self.IOTA0, iTemp42)) & 2047) as usize] + fTemp36 * (fTemp49 * self.fRec24[((i32::wrapping_sub(self.IOTA0, iTemp35)) & 2047) as usize] + 0.5 * fTemp48 * self.fRec24[((i32::wrapping_sub(self.IOTA0, iTemp30)) & 2047) as usize] + 0.16666667 * fTemp47 * self.fRec24[((i32::wrapping_sub(self.IOTA0, iTemp26)) & 2047) as usize] + 0.041666668 * fTemp22 * self.fRec24[((i32::wrapping_sub(self.IOTA0, iTemp16)) & 2047) as usize]);
			self.fRec26[0] = fRec30;
			let mut fRec27: F32 = fRec31;
			self.fRec22[0] = fSlow5 * self.fRec26[1];
			let mut fRec23: F32 = fRec27;
			self.fRec17[0] = fRec21;
			let mut fRec18: F32 = fSlow5 * self.fRec17[1];
			let mut fRec19: F32 = self.fRec22[0];
			let mut fRec20: F32 = fRec23;
			self.fRec13[(self.IOTA0 & 2047) as usize] = fRec18;
			let mut fRec14: F32 = fRec25;
			let mut fRec15: F32 = fRec19;
			let mut fRec16: F32 = fRec20;
			self.fRec11[0] = fRec14;
			let mut fRec12: F32 = fRec16;
			let mut fTemp58: F32 = F32::abs(fRec12);
			let mut fTemp59: F32 = if (fTemp58 > self.fRec10[1]) as i32 != 0 {0.0} else {self.fConst13};
			self.fRec10[0] = fTemp58 * (1.0 - fTemp59) + self.fRec10[1] * fTemp59;
			let mut fRec9: F32 = 0.0 - 0.95 * F32::max(2e+01 * F32::log10(F32::max(1.1754944e-38, self.fRec10[0])) + 1e+01, 0.0);
			self.fRec8[0] = 1.5 * fRec12 * F32::powf(1e+01, 0.05 * fRec9);
			self.fRec7[0] = self.fRec8[0] - (self.fRec7[2] * (fTemp10 + (fTemp4 - fTemp9) / fTemp3 + 1.0) + 2.0 * self.fRec7[1] * (fTemp10 + (1.0 - fTemp8))) / fTemp11;
			self.fRec6[0] = (self.fRec7[2] + self.fRec7[0] + 2.0 * self.fRec7[1]) / fTemp11 - (self.fRec6[2] * (fTemp6 + (fTemp4 - fTemp5) / fTemp3 + fTemp2) + 2.0 * self.fRec6[1] * (fTemp6 + (1.0 - (fTemp1 + fTemp8)))) / fTemp7;
			self.fRec52[0] = self.fConst2 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow6 + self.fRec5[0]) + 0.5) as i32, 2047))) as usize] } + self.fConst3 * self.fRec52[1];
			let mut fTemp60: F32 = F32::tan(self.fConst4 * F32::max(2e+01, F32::min(1e+04, self.fRec52[0])));
			let mut fTemp61: F32 = 1.0 / fTemp60;
			let mut fTemp62: F32 = fTemp6 + (fTemp5 + fTemp61) / fTemp60 + fTemp2;
			let mut fTemp63: F32 = 1.0 / mydsp_faustpower2_f(fTemp60);
			let mut fTemp64: F32 = fTemp10 + (fTemp9 + fTemp61) / fTemp60 + 1.0;
			let mut fRec68: F32 = -1.0 * 0.9973053 * (0.9 * self.fRec69[2] + 0.05 * (self.fRec69[1] + self.fRec69[3]));
			self.fRec87[0] = self.fConst2 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow6 + self.fRec41[0]) + 0.5) as i32, 2047))) as usize] } + self.fConst3 * self.fRec87[1];
			let mut fTemp65: F32 = self.fConst6 * (3.4e+02 / self.fRec87[0] + -0.11);
			let mut fTemp66: F32 = fTemp65 + -1.499995;
			let mut iTemp67: i32 = (fTemp66) as i32;
			let mut iTemp68: i32 = (F32::min(self.fConst5, (std::cmp::max(0, i32::wrapping_add(iTemp67, 4))) as F32)) as i32;
			let mut iTemp69: i32 = i32::wrapping_add(iTemp68, 1);
			let mut fTemp70: F32 = F32::floor(fTemp66);
			let mut fTemp71: F32 = fTemp65 + (-3.0 - fTemp70);
			let mut fTemp72: F32 = fTemp65 + (-2.0 - fTemp70);
			let mut fTemp73: F32 = fTemp65 + (-1.0 - fTemp70);
			let mut fTemp74: F32 = fTemp73 * fTemp72;
			let mut fTemp75: F32 = fTemp74 * fTemp71;
			let mut fTemp76: F32 = fTemp65 + (-4.0 - fTemp70);
			let mut fTemp77: F32 = 0.0 - fTemp76;
			let mut iTemp78: i32 = (F32::min(self.fConst5, (std::cmp::max(0, i32::wrapping_add(iTemp67, 3))) as F32)) as i32;
			let mut iTemp79: i32 = i32::wrapping_add(iTemp78, 1);
			let mut fTemp80: F32 = 0.0 - 0.5 * fTemp76;
			let mut fTemp81: F32 = 0.0 - fTemp71;
			let mut iTemp82: i32 = (F32::min(self.fConst5, (std::cmp::max(0, i32::wrapping_add(iTemp67, 2))) as F32)) as i32;
			let mut iTemp83: i32 = i32::wrapping_add(iTemp82, 1);
			let mut fTemp84: F32 = 0.0 - 0.33333334 * fTemp76;
			let mut fTemp85: F32 = 0.0 - 0.5 * fTemp71;
			let mut fTemp86: F32 = 0.0 - fTemp72;
			let mut iTemp87: i32 = (F32::min(self.fConst5, (std::cmp::max(0, i32::wrapping_add(iTemp67, 1))) as F32)) as i32;
			let mut iTemp88: i32 = i32::wrapping_add(iTemp87, 1);
			let mut fTemp89: F32 = fTemp65 - fTemp70;
			let mut fTemp90: F32 = 0.0 - 0.25 * fTemp76;
			let mut fTemp91: F32 = 0.0 - 0.33333334 * fTemp71;
			let mut fTemp92: F32 = 0.0 - 0.5 * fTemp72;
			let mut fTemp93: F32 = 0.0 - fTemp73;
			let mut iTemp94: i32 = (F32::min(self.fConst5, (std::cmp::max(0, iTemp67)) as F32)) as i32;
			let mut iTemp95: i32 = i32::wrapping_add(iTemp94, 1);
			self.fRec83[0] = self.fRec60[((i32::wrapping_sub(self.IOTA0, iTemp95)) & 2047) as usize] * fTemp93 * fTemp92 * fTemp91 * fTemp90 + fTemp89 * (self.fRec60[((i32::wrapping_sub(self.IOTA0, iTemp88)) & 2047) as usize] * fTemp86 * fTemp85 * fTemp84 + 0.5 * fTemp73 * self.fRec60[((i32::wrapping_sub(self.IOTA0, iTemp83)) & 2047) as usize] * fTemp81 * fTemp80 + 0.16666667 * fTemp74 * self.fRec60[((i32::wrapping_sub(self.IOTA0, iTemp79)) & 2047) as usize] * fTemp77 + 0.041666668 * fTemp75 * self.fRec60[((i32::wrapping_sub(self.IOTA0, iTemp69)) & 2047) as usize]);
			self.fRec88[0] = 0.95 * self.fRec83[1] + 0.05 * self.fRec88[1];
			let mut fRec84: F32 = self.fRec88[0];
			self.fRec91[0] = self.fConst8 * F32::abs(self.fRec55[1]) + self.fConst7 * self.fRec91[1];
			let mut iTemp96: i32 = (self.fRec91[0] > 0.1) as i32;
			self.iVec5[0] = iTemp96;
			self.iRec92[0] = std::cmp::max(i32::wrapping_mul(self.iConst9, (iTemp96 < self.iVec5[1]) as i32), i32::wrapping_add(self.iRec92[1], -1));
			let mut fTemp97: F32 = F32::abs(F32::max((iTemp96) as F32, ((self.iRec92[0] > 0) as i32) as u32 as F32));
			let mut fTemp98: F32 = if (fTemp97 > self.fRec90[1]) as i32 != 0 {self.fConst7} else {self.fConst10};
			self.fRec90[0] = fTemp97 * (1.0 - fTemp98) + self.fRec90[1] * fTemp98;
			let mut fTemp99: F32 = 0.005 * self.fRec90[0] * self.fRec55[1];
			self.fRec93[0] = self.fRec58[1];
			self.fRec94[(self.IOTA0 & 2047) as usize] = -1.0 * 0.9973053 * (0.9 * self.fRec93[2] + 0.05 * (self.fRec93[1] + self.fRec93[3]));
			let mut fTemp100: F32 = fTemp74 * fTemp77;
			let mut fTemp101: F32 = fTemp73 * fTemp81 * fTemp80;
			let mut fTemp102: F32 = fTemp86 * fTemp85 * fTemp84;
			let mut fTemp103: F32 = fTemp93 * fTemp92 * fTemp91 * fTemp90;
			self.fVec6[0] = fTemp103 * self.fRec94[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp94, 2))) & 2047) as usize] + fTemp89 * (fTemp102 * self.fRec94[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp87, 2))) & 2047) as usize] + 0.5 * fTemp101 * self.fRec94[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp82, 2))) & 2047) as usize] + 0.16666667 * fTemp100 * self.fRec94[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp78, 2))) & 2047) as usize] + 0.041666668 * fTemp75 * self.fRec94[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp68, 2))) & 2047) as usize]);
			let mut fTemp104: F32 = F32::tan(self.fConst11 * self.fRec87[0]);
			let mut fTemp105: F32 = 1.0 / fTemp104;
			let mut fTemp106: F32 = (fTemp105 + 1.4142135) / fTemp104 + 1.0;
			self.fVec7[0] = fSlow7;
			self.iRec95[0] = i32::wrapping_add(i32::wrapping_mul(i32::wrapping_add(self.iRec95[1], (self.iRec95[1] > 0) as i32), (fSlow7 <= self.fVec7[1]) as i32), (fSlow7 > self.fVec7[1]) as i32);
			let mut fTemp107: F32 = (self.iRec95[0]) as F32 / F32::max(1.0, self.fConst12 * mydsp_faustpower2_f(1.0 - 0.0005 * self.fRec87[0]));
			self.fRec96[0] = fTemp55 - (self.fRec96[2] * ((fTemp105 + -1.4142135) / fTemp104 + 1.0) + 2.0 * self.fRec96[1] * (1.0 - 1.0 / mydsp_faustpower2_f(fTemp104))) / fTemp106;
			let mut fTemp108: F32 = 0.5 * ((self.fRec96[2] + self.fRec96[0] + 2.0 * self.fRec96[1]) * F32::max(0.0, F32::min(fTemp107, 2.0 - fTemp107)) / fTemp106);
			let mut fTemp109: F32 = fTemp108 + self.fVec6[1] + fTemp99;
			self.fVec8[0] = fTemp109;
			self.fRec89[(self.IOTA0 & 2047) as usize] = 0.95 * self.fVec8[2] + 0.05 * self.fRec89[((i32::wrapping_sub(self.IOTA0, 1)) & 2047) as usize];
			let mut fRec85: F32 = fTemp103 * self.fRec89[((i32::wrapping_sub(self.IOTA0, iTemp94)) & 2047) as usize] + fTemp89 * (fTemp102 * self.fRec89[((i32::wrapping_sub(self.IOTA0, iTemp87)) & 2047) as usize] + 0.5 * fTemp101 * self.fRec89[((i32::wrapping_sub(self.IOTA0, iTemp82)) & 2047) as usize] + 0.16666667 * fTemp100 * self.fRec89[((i32::wrapping_sub(self.IOTA0, iTemp78)) & 2047) as usize] + 0.041666668 * fTemp75 * self.fRec89[((i32::wrapping_sub(self.IOTA0, iTemp68)) & 2047) as usize]);
			let mut fRec86: F32 = self.fVec8[1] + self.fRec79[1];
			self.fRec79[0] = fRec84;
			let mut fRec80: F32 = self.fRec79[1];
			let mut fRec81: F32 = fRec85;
			let mut fRec82: F32 = fRec86;
			self.fRec75[0] = fRec80;
			let mut fRec76: F32 = fTemp99 + fTemp108 + self.fRec75[1];
			let mut fRec77: F32 = fRec81;
			let mut fRec78: F32 = fRec82;
			self.fRec71[(self.IOTA0 & 2047) as usize] = fRec76;
			let mut fRec72: F32 = fTemp103 * self.fRec71[((i32::wrapping_sub(self.IOTA0, iTemp95)) & 2047) as usize] + fTemp89 * (fTemp102 * self.fRec71[((i32::wrapping_sub(self.IOTA0, iTemp88)) & 2047) as usize] + 0.5 * fTemp101 * self.fRec71[((i32::wrapping_sub(self.IOTA0, iTemp83)) & 2047) as usize] + 0.16666667 * fTemp100 * self.fRec71[((i32::wrapping_sub(self.IOTA0, iTemp79)) & 2047) as usize] + 0.041666668 * fTemp75 * self.fRec71[((i32::wrapping_sub(self.IOTA0, iTemp69)) & 2047) as usize]);
			self.fRec73[0] = fRec77;
			let mut fRec74: F32 = fRec78;
			self.fRec69[0] = fSlow5 * self.fRec73[1];
			let mut fRec70: F32 = fRec74;
			self.fRec64[0] = fRec68;
			let mut fRec65: F32 = fSlow5 * self.fRec64[1];
			let mut fRec66: F32 = self.fRec69[0];
			let mut fRec67: F32 = fRec70;
			self.fRec60[(self.IOTA0 & 2047) as usize] = fRec65;
			let mut fRec61: F32 = fRec72;
			let mut fRec62: F32 = fRec66;
			let mut fRec63: F32 = fRec67;
			self.fRec58[0] = fRec61;
			let mut fRec59: F32 = fRec63;
			let mut fTemp110: F32 = F32::abs(fRec59);
			let mut fTemp111: F32 = if (fTemp110 > self.fRec57[1]) as i32 != 0 {0.0} else {self.fConst13};
			self.fRec57[0] = fTemp110 * (1.0 - fTemp111) + self.fRec57[1] * fTemp111;
			let mut fRec56: F32 = 0.0 - 0.95 * F32::max(2e+01 * F32::log10(F32::max(1.1754944e-38, self.fRec57[0])) + 1e+01, 0.0);
			self.fRec55[0] = 1.5 * fRec59 * F32::powf(1e+01, 0.05 * fRec56);
			self.fRec54[0] = self.fRec55[0] - (self.fRec54[2] * (fTemp10 + (fTemp61 - fTemp9) / fTemp60 + 1.0) + 2.0 * self.fRec54[1] * (fTemp10 + (1.0 - fTemp63))) / fTemp64;
			self.fRec53[0] = (self.fRec54[2] + self.fRec54[0] + 2.0 * self.fRec54[1]) / fTemp64 - (self.fRec53[2] * (fTemp6 + (fTemp61 - fTemp5) / fTemp60 + fTemp2) + 2.0 * self.fRec53[1] * (fTemp6 + (1.0 - (fTemp1 + fTemp63)))) / fTemp62;
			self.fRec97[0] = self.fConst2 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow8 + self.fRec5[0]) + 0.5) as i32, 2047))) as usize] } + self.fConst3 * self.fRec97[1];
			let mut fTemp112: F32 = F32::tan(self.fConst4 * F32::max(2e+01, F32::min(1e+04, self.fRec97[0])));
			let mut fTemp113: F32 = 1.0 / fTemp112;
			let mut fTemp114: F32 = fTemp6 + (fTemp5 + fTemp113) / fTemp112 + fTemp2;
			let mut fTemp115: F32 = 1.0 / mydsp_faustpower2_f(fTemp112);
			let mut fTemp116: F32 = fTemp10 + (fTemp9 + fTemp113) / fTemp112 + 1.0;
			let mut fRec113: F32 = -1.0 * 0.9973053 * (0.9 * self.fRec114[2] + 0.05 * (self.fRec114[1] + self.fRec114[3]));
			self.fRec132[0] = self.fConst2 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow8 + self.fRec41[0]) + 0.5) as i32, 2047))) as usize] } + self.fConst3 * self.fRec132[1];
			let mut fTemp117: F32 = self.fConst6 * (3.4e+02 / self.fRec132[0] + -0.11);
			let mut fTemp118: F32 = fTemp117 + -1.499995;
			let mut iTemp119: i32 = (fTemp118) as i32;
			let mut iTemp120: i32 = (F32::min(self.fConst5, (std::cmp::max(0, i32::wrapping_add(iTemp119, 4))) as F32)) as i32;
			let mut iTemp121: i32 = i32::wrapping_add(iTemp120, 1);
			let mut fTemp122: F32 = F32::floor(fTemp118);
			let mut fTemp123: F32 = fTemp117 + (-3.0 - fTemp122);
			let mut fTemp124: F32 = fTemp117 + (-2.0 - fTemp122);
			let mut fTemp125: F32 = fTemp117 + (-1.0 - fTemp122);
			let mut fTemp126: F32 = fTemp125 * fTemp124;
			let mut fTemp127: F32 = fTemp126 * fTemp123;
			let mut fTemp128: F32 = fTemp117 + (-4.0 - fTemp122);
			let mut fTemp129: F32 = 0.0 - fTemp128;
			let mut iTemp130: i32 = (F32::min(self.fConst5, (std::cmp::max(0, i32::wrapping_add(iTemp119, 3))) as F32)) as i32;
			let mut iTemp131: i32 = i32::wrapping_add(iTemp130, 1);
			let mut fTemp132: F32 = 0.0 - 0.5 * fTemp128;
			let mut fTemp133: F32 = 0.0 - fTemp123;
			let mut iTemp134: i32 = (F32::min(self.fConst5, (std::cmp::max(0, i32::wrapping_add(iTemp119, 2))) as F32)) as i32;
			let mut iTemp135: i32 = i32::wrapping_add(iTemp134, 1);
			let mut fTemp136: F32 = 0.0 - 0.33333334 * fTemp128;
			let mut fTemp137: F32 = 0.0 - 0.5 * fTemp123;
			let mut fTemp138: F32 = 0.0 - fTemp124;
			let mut iTemp139: i32 = (F32::min(self.fConst5, (std::cmp::max(0, i32::wrapping_add(iTemp119, 1))) as F32)) as i32;
			let mut iTemp140: i32 = i32::wrapping_add(iTemp139, 1);
			let mut fTemp141: F32 = fTemp117 - fTemp122;
			let mut fTemp142: F32 = 0.0 - 0.25 * fTemp128;
			let mut fTemp143: F32 = 0.0 - 0.33333334 * fTemp123;
			let mut fTemp144: F32 = 0.0 - 0.5 * fTemp124;
			let mut fTemp145: F32 = 0.0 - fTemp125;
			let mut iTemp146: i32 = (F32::min(self.fConst5, (std::cmp::max(0, iTemp119)) as F32)) as i32;
			let mut iTemp147: i32 = i32::wrapping_add(iTemp146, 1);
			self.fRec128[0] = self.fRec105[((i32::wrapping_sub(self.IOTA0, iTemp147)) & 2047) as usize] * fTemp145 * fTemp144 * fTemp143 * fTemp142 + fTemp141 * (self.fRec105[((i32::wrapping_sub(self.IOTA0, iTemp140)) & 2047) as usize] * fTemp138 * fTemp137 * fTemp136 + 0.5 * fTemp125 * self.fRec105[((i32::wrapping_sub(self.IOTA0, iTemp135)) & 2047) as usize] * fTemp133 * fTemp132 + 0.16666667 * fTemp126 * self.fRec105[((i32::wrapping_sub(self.IOTA0, iTemp131)) & 2047) as usize] * fTemp129 + 0.041666668 * fTemp127 * self.fRec105[((i32::wrapping_sub(self.IOTA0, iTemp121)) & 2047) as usize]);
			self.fRec133[0] = 0.95 * self.fRec128[1] + 0.05 * self.fRec133[1];
			let mut fRec129: F32 = self.fRec133[0];
			self.fRec136[0] = self.fConst8 * F32::abs(self.fRec100[1]) + self.fConst7 * self.fRec136[1];
			let mut iTemp148: i32 = (self.fRec136[0] > 0.1) as i32;
			self.iVec9[0] = iTemp148;
			self.iRec137[0] = std::cmp::max(i32::wrapping_mul(self.iConst9, (iTemp148 < self.iVec9[1]) as i32), i32::wrapping_add(self.iRec137[1], -1));
			let mut fTemp149: F32 = F32::abs(F32::max((iTemp148) as F32, ((self.iRec137[0] > 0) as i32) as u32 as F32));
			let mut fTemp150: F32 = if (fTemp149 > self.fRec135[1]) as i32 != 0 {self.fConst7} else {self.fConst10};
			self.fRec135[0] = fTemp149 * (1.0 - fTemp150) + self.fRec135[1] * fTemp150;
			let mut fTemp151: F32 = 0.005 * self.fRec135[0] * self.fRec100[1];
			self.fRec138[0] = self.fRec103[1];
			self.fRec139[(self.IOTA0 & 2047) as usize] = -1.0 * 0.9973053 * (0.9 * self.fRec138[2] + 0.05 * (self.fRec138[1] + self.fRec138[3]));
			let mut fTemp152: F32 = fTemp126 * fTemp129;
			let mut fTemp153: F32 = fTemp125 * fTemp133 * fTemp132;
			let mut fTemp154: F32 = fTemp138 * fTemp137 * fTemp136;
			let mut fTemp155: F32 = fTemp145 * fTemp144 * fTemp143 * fTemp142;
			self.fVec10[0] = fTemp155 * self.fRec139[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp146, 2))) & 2047) as usize] + fTemp141 * (fTemp154 * self.fRec139[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp139, 2))) & 2047) as usize] + 0.5 * fTemp153 * self.fRec139[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp134, 2))) & 2047) as usize] + 0.16666667 * fTemp152 * self.fRec139[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp130, 2))) & 2047) as usize] + 0.041666668 * fTemp127 * self.fRec139[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp120, 2))) & 2047) as usize]);
			let mut fTemp156: F32 = F32::tan(self.fConst11 * self.fRec132[0]);
			let mut fTemp157: F32 = 1.0 / fTemp156;
			let mut fTemp158: F32 = (fTemp157 + 1.4142135) / fTemp156 + 1.0;
			self.fVec11[0] = fSlow9;
			self.iRec140[0] = i32::wrapping_add(i32::wrapping_mul(i32::wrapping_add(self.iRec140[1], (self.iRec140[1] > 0) as i32), (fSlow9 <= self.fVec11[1]) as i32), (fSlow9 > self.fVec11[1]) as i32);
			let mut fTemp159: F32 = (self.iRec140[0]) as F32 / F32::max(1.0, self.fConst12 * mydsp_faustpower2_f(1.0 - 0.0005 * self.fRec132[0]));
			self.fRec141[0] = fTemp55 - (self.fRec141[2] * ((fTemp157 + -1.4142135) / fTemp156 + 1.0) + 2.0 * self.fRec141[1] * (1.0 - 1.0 / mydsp_faustpower2_f(fTemp156))) / fTemp158;
			let mut fTemp160: F32 = 0.5 * ((self.fRec141[2] + self.fRec141[0] + 2.0 * self.fRec141[1]) * F32::max(0.0, F32::min(fTemp159, 2.0 - fTemp159)) / fTemp158);
			let mut fTemp161: F32 = fTemp160 + self.fVec10[1] + fTemp151;
			self.fVec12[0] = fTemp161;
			self.fRec134[(self.IOTA0 & 2047) as usize] = 0.95 * self.fVec12[2] + 0.05 * self.fRec134[((i32::wrapping_sub(self.IOTA0, 1)) & 2047) as usize];
			let mut fRec130: F32 = fTemp155 * self.fRec134[((i32::wrapping_sub(self.IOTA0, iTemp146)) & 2047) as usize] + fTemp141 * (fTemp154 * self.fRec134[((i32::wrapping_sub(self.IOTA0, iTemp139)) & 2047) as usize] + 0.5 * fTemp153 * self.fRec134[((i32::wrapping_sub(self.IOTA0, iTemp134)) & 2047) as usize] + 0.16666667 * fTemp152 * self.fRec134[((i32::wrapping_sub(self.IOTA0, iTemp130)) & 2047) as usize] + 0.041666668 * fTemp127 * self.fRec134[((i32::wrapping_sub(self.IOTA0, iTemp120)) & 2047) as usize]);
			let mut fRec131: F32 = self.fVec12[1] + self.fRec124[1];
			self.fRec124[0] = fRec129;
			let mut fRec125: F32 = self.fRec124[1];
			let mut fRec126: F32 = fRec130;
			let mut fRec127: F32 = fRec131;
			self.fRec120[0] = fRec125;
			let mut fRec121: F32 = fTemp151 + fTemp160 + self.fRec120[1];
			let mut fRec122: F32 = fRec126;
			let mut fRec123: F32 = fRec127;
			self.fRec116[(self.IOTA0 & 2047) as usize] = fRec121;
			let mut fRec117: F32 = fTemp155 * self.fRec116[((i32::wrapping_sub(self.IOTA0, iTemp147)) & 2047) as usize] + fTemp141 * (fTemp154 * self.fRec116[((i32::wrapping_sub(self.IOTA0, iTemp140)) & 2047) as usize] + 0.5 * fTemp153 * self.fRec116[((i32::wrapping_sub(self.IOTA0, iTemp135)) & 2047) as usize] + 0.16666667 * fTemp152 * self.fRec116[((i32::wrapping_sub(self.IOTA0, iTemp131)) & 2047) as usize] + 0.041666668 * fTemp127 * self.fRec116[((i32::wrapping_sub(self.IOTA0, iTemp121)) & 2047) as usize]);
			self.fRec118[0] = fRec122;
			let mut fRec119: F32 = fRec123;
			self.fRec114[0] = fSlow5 * self.fRec118[1];
			let mut fRec115: F32 = fRec119;
			self.fRec109[0] = fRec113;
			let mut fRec110: F32 = fSlow5 * self.fRec109[1];
			let mut fRec111: F32 = self.fRec114[0];
			let mut fRec112: F32 = fRec115;
			self.fRec105[(self.IOTA0 & 2047) as usize] = fRec110;
			let mut fRec106: F32 = fRec117;
			let mut fRec107: F32 = fRec111;
			let mut fRec108: F32 = fRec112;
			self.fRec103[0] = fRec106;
			let mut fRec104: F32 = fRec108;
			let mut fTemp162: F32 = F32::abs(fRec104);
			let mut fTemp163: F32 = if (fTemp162 > self.fRec102[1]) as i32 != 0 {0.0} else {self.fConst13};
			self.fRec102[0] = fTemp162 * (1.0 - fTemp163) + self.fRec102[1] * fTemp163;
			let mut fRec101: F32 = 0.0 - 0.95 * F32::max(2e+01 * F32::log10(F32::max(1.1754944e-38, self.fRec102[0])) + 1e+01, 0.0);
			self.fRec100[0] = 1.5 * fRec104 * F32::powf(1e+01, 0.05 * fRec101);
			self.fRec99[0] = self.fRec100[0] - (self.fRec99[2] * (fTemp10 + (fTemp113 - fTemp9) / fTemp112 + 1.0) + 2.0 * self.fRec99[1] * (fTemp10 + (1.0 - fTemp115))) / fTemp116;
			self.fRec98[0] = (self.fRec99[2] + self.fRec99[0] + 2.0 * self.fRec99[1]) / fTemp116 - (self.fRec98[2] * (fTemp6 + (fTemp113 - fTemp5) / fTemp112 + fTemp2) + 2.0 * self.fRec98[1] * (fTemp6 + (1.0 - (fTemp1 + fTemp115)))) / fTemp114;
			self.fRec142[0] = self.fConst2 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow10 + self.fRec5[0]) + 0.5) as i32, 2047))) as usize] } + self.fConst3 * self.fRec142[1];
			let mut fTemp164: F32 = F32::tan(self.fConst4 * F32::max(2e+01, F32::min(1e+04, self.fRec142[0])));
			let mut fTemp165: F32 = 1.0 / fTemp164;
			let mut fTemp166: F32 = fTemp6 + (fTemp5 + fTemp165) / fTemp164 + fTemp2;
			let mut fTemp167: F32 = 1.0 / mydsp_faustpower2_f(fTemp164);
			let mut fTemp168: F32 = fTemp10 + (fTemp9 + fTemp165) / fTemp164 + 1.0;
			let mut fRec158: F32 = -1.0 * 0.9973053 * (0.9 * self.fRec159[2] + 0.05 * (self.fRec159[1] + self.fRec159[3]));
			self.fRec177[0] = self.fConst2 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow10 + self.fRec41[0]) + 0.5) as i32, 2047))) as usize] } + self.fConst3 * self.fRec177[1];
			let mut fTemp169: F32 = self.fConst6 * (3.4e+02 / self.fRec177[0] + -0.11);
			let mut fTemp170: F32 = fTemp169 + -1.499995;
			let mut iTemp171: i32 = (fTemp170) as i32;
			let mut iTemp172: i32 = (F32::min(self.fConst5, (std::cmp::max(0, i32::wrapping_add(iTemp171, 4))) as F32)) as i32;
			let mut iTemp173: i32 = i32::wrapping_add(iTemp172, 1);
			let mut fTemp174: F32 = F32::floor(fTemp170);
			let mut fTemp175: F32 = fTemp169 + (-3.0 - fTemp174);
			let mut fTemp176: F32 = fTemp169 + (-2.0 - fTemp174);
			let mut fTemp177: F32 = fTemp169 + (-1.0 - fTemp174);
			let mut fTemp178: F32 = fTemp177 * fTemp176;
			let mut fTemp179: F32 = fTemp178 * fTemp175;
			let mut fTemp180: F32 = fTemp169 + (-4.0 - fTemp174);
			let mut fTemp181: F32 = 0.0 - fTemp180;
			let mut iTemp182: i32 = (F32::min(self.fConst5, (std::cmp::max(0, i32::wrapping_add(iTemp171, 3))) as F32)) as i32;
			let mut iTemp183: i32 = i32::wrapping_add(iTemp182, 1);
			let mut fTemp184: F32 = 0.0 - 0.5 * fTemp180;
			let mut fTemp185: F32 = 0.0 - fTemp175;
			let mut iTemp186: i32 = (F32::min(self.fConst5, (std::cmp::max(0, i32::wrapping_add(iTemp171, 2))) as F32)) as i32;
			let mut iTemp187: i32 = i32::wrapping_add(iTemp186, 1);
			let mut fTemp188: F32 = 0.0 - 0.33333334 * fTemp180;
			let mut fTemp189: F32 = 0.0 - 0.5 * fTemp175;
			let mut fTemp190: F32 = 0.0 - fTemp176;
			let mut iTemp191: i32 = (F32::min(self.fConst5, (std::cmp::max(0, i32::wrapping_add(iTemp171, 1))) as F32)) as i32;
			let mut iTemp192: i32 = i32::wrapping_add(iTemp191, 1);
			let mut fTemp193: F32 = fTemp169 - fTemp174;
			let mut fTemp194: F32 = 0.0 - 0.25 * fTemp180;
			let mut fTemp195: F32 = 0.0 - 0.33333334 * fTemp175;
			let mut fTemp196: F32 = 0.0 - 0.5 * fTemp176;
			let mut fTemp197: F32 = 0.0 - fTemp177;
			let mut iTemp198: i32 = (F32::min(self.fConst5, (std::cmp::max(0, iTemp171)) as F32)) as i32;
			let mut iTemp199: i32 = i32::wrapping_add(iTemp198, 1);
			self.fRec173[0] = self.fRec150[((i32::wrapping_sub(self.IOTA0, iTemp199)) & 2047) as usize] * fTemp197 * fTemp196 * fTemp195 * fTemp194 + fTemp193 * (self.fRec150[((i32::wrapping_sub(self.IOTA0, iTemp192)) & 2047) as usize] * fTemp190 * fTemp189 * fTemp188 + 0.5 * fTemp177 * self.fRec150[((i32::wrapping_sub(self.IOTA0, iTemp187)) & 2047) as usize] * fTemp185 * fTemp184 + 0.16666667 * fTemp178 * self.fRec150[((i32::wrapping_sub(self.IOTA0, iTemp183)) & 2047) as usize] * fTemp181 + 0.041666668 * fTemp179 * self.fRec150[((i32::wrapping_sub(self.IOTA0, iTemp173)) & 2047) as usize]);
			self.fRec178[0] = 0.95 * self.fRec173[1] + 0.05 * self.fRec178[1];
			let mut fRec174: F32 = self.fRec178[0];
			self.fRec181[0] = self.fConst8 * F32::abs(self.fRec145[1]) + self.fConst7 * self.fRec181[1];
			let mut iTemp200: i32 = (self.fRec181[0] > 0.1) as i32;
			self.iVec13[0] = iTemp200;
			self.iRec182[0] = std::cmp::max(i32::wrapping_mul(self.iConst9, (iTemp200 < self.iVec13[1]) as i32), i32::wrapping_add(self.iRec182[1], -1));
			let mut fTemp201: F32 = F32::abs(F32::max((iTemp200) as F32, ((self.iRec182[0] > 0) as i32) as u32 as F32));
			let mut fTemp202: F32 = if (fTemp201 > self.fRec180[1]) as i32 != 0 {self.fConst7} else {self.fConst10};
			self.fRec180[0] = fTemp201 * (1.0 - fTemp202) + self.fRec180[1] * fTemp202;
			let mut fTemp203: F32 = 0.005 * self.fRec180[0] * self.fRec145[1];
			self.fRec183[0] = self.fRec148[1];
			self.fRec184[(self.IOTA0 & 2047) as usize] = -1.0 * 0.9973053 * (0.9 * self.fRec183[2] + 0.05 * (self.fRec183[1] + self.fRec183[3]));
			let mut fTemp204: F32 = fTemp178 * fTemp181;
			let mut fTemp205: F32 = fTemp177 * fTemp185 * fTemp184;
			let mut fTemp206: F32 = fTemp190 * fTemp189 * fTemp188;
			let mut fTemp207: F32 = fTemp197 * fTemp196 * fTemp195 * fTemp194;
			self.fVec14[0] = fTemp207 * self.fRec184[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp198, 2))) & 2047) as usize] + fTemp193 * (fTemp206 * self.fRec184[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp191, 2))) & 2047) as usize] + 0.5 * fTemp205 * self.fRec184[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp186, 2))) & 2047) as usize] + 0.16666667 * fTemp204 * self.fRec184[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp182, 2))) & 2047) as usize] + 0.041666668 * fTemp179 * self.fRec184[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp172, 2))) & 2047) as usize]);
			let mut fTemp208: F32 = F32::tan(self.fConst11 * self.fRec177[0]);
			let mut fTemp209: F32 = 1.0 / fTemp208;
			let mut fTemp210: F32 = (fTemp209 + 1.4142135) / fTemp208 + 1.0;
			self.fVec15[0] = fSlow11;
			self.iRec185[0] = i32::wrapping_add(i32::wrapping_mul(i32::wrapping_add(self.iRec185[1], (self.iRec185[1] > 0) as i32), (fSlow11 <= self.fVec15[1]) as i32), (fSlow11 > self.fVec15[1]) as i32);
			let mut fTemp211: F32 = (self.iRec185[0]) as F32 / F32::max(1.0, self.fConst12 * mydsp_faustpower2_f(1.0 - 0.0005 * self.fRec177[0]));
			self.fRec186[0] = fTemp55 - (self.fRec186[2] * ((fTemp209 + -1.4142135) / fTemp208 + 1.0) + 2.0 * self.fRec186[1] * (1.0 - 1.0 / mydsp_faustpower2_f(fTemp208))) / fTemp210;
			let mut fTemp212: F32 = 0.5 * ((self.fRec186[2] + self.fRec186[0] + 2.0 * self.fRec186[1]) * F32::max(0.0, F32::min(fTemp211, 2.0 - fTemp211)) / fTemp210);
			let mut fTemp213: F32 = fTemp212 + self.fVec14[1] + fTemp203;
			self.fVec16[0] = fTemp213;
			self.fRec179[(self.IOTA0 & 2047) as usize] = 0.95 * self.fVec16[2] + 0.05 * self.fRec179[((i32::wrapping_sub(self.IOTA0, 1)) & 2047) as usize];
			let mut fRec175: F32 = fTemp207 * self.fRec179[((i32::wrapping_sub(self.IOTA0, iTemp198)) & 2047) as usize] + fTemp193 * (fTemp206 * self.fRec179[((i32::wrapping_sub(self.IOTA0, iTemp191)) & 2047) as usize] + 0.5 * fTemp205 * self.fRec179[((i32::wrapping_sub(self.IOTA0, iTemp186)) & 2047) as usize] + 0.16666667 * fTemp204 * self.fRec179[((i32::wrapping_sub(self.IOTA0, iTemp182)) & 2047) as usize] + 0.041666668 * fTemp179 * self.fRec179[((i32::wrapping_sub(self.IOTA0, iTemp172)) & 2047) as usize]);
			let mut fRec176: F32 = self.fVec16[1] + self.fRec169[1];
			self.fRec169[0] = fRec174;
			let mut fRec170: F32 = self.fRec169[1];
			let mut fRec171: F32 = fRec175;
			let mut fRec172: F32 = fRec176;
			self.fRec165[0] = fRec170;
			let mut fRec166: F32 = fTemp203 + fTemp212 + self.fRec165[1];
			let mut fRec167: F32 = fRec171;
			let mut fRec168: F32 = fRec172;
			self.fRec161[(self.IOTA0 & 2047) as usize] = fRec166;
			let mut fRec162: F32 = fTemp207 * self.fRec161[((i32::wrapping_sub(self.IOTA0, iTemp199)) & 2047) as usize] + fTemp193 * (fTemp206 * self.fRec161[((i32::wrapping_sub(self.IOTA0, iTemp192)) & 2047) as usize] + 0.5 * fTemp205 * self.fRec161[((i32::wrapping_sub(self.IOTA0, iTemp187)) & 2047) as usize] + 0.16666667 * fTemp204 * self.fRec161[((i32::wrapping_sub(self.IOTA0, iTemp183)) & 2047) as usize] + 0.041666668 * fTemp179 * self.fRec161[((i32::wrapping_sub(self.IOTA0, iTemp173)) & 2047) as usize]);
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
			let mut fTemp214: F32 = F32::abs(fRec149);
			let mut fTemp215: F32 = if (fTemp214 > self.fRec147[1]) as i32 != 0 {0.0} else {self.fConst13};
			self.fRec147[0] = fTemp214 * (1.0 - fTemp215) + self.fRec147[1] * fTemp215;
			let mut fRec146: F32 = 0.0 - 0.95 * F32::max(2e+01 * F32::log10(F32::max(1.1754944e-38, self.fRec147[0])) + 1e+01, 0.0);
			self.fRec145[0] = 1.5 * fRec149 * F32::powf(1e+01, 0.05 * fRec146);
			self.fRec144[0] = self.fRec145[0] - (self.fRec144[2] * (fTemp10 + (fTemp165 - fTemp9) / fTemp164 + 1.0) + 2.0 * self.fRec144[1] * (fTemp10 + (1.0 - fTemp167))) / fTemp168;
			self.fRec143[0] = (self.fRec144[2] + self.fRec144[0] + 2.0 * self.fRec144[1]) / fTemp168 - (self.fRec143[2] * (fTemp6 + (fTemp165 - fTemp5) / fTemp164 + fTemp2) + 2.0 * self.fRec143[1] * (fTemp6 + (1.0 - (fTemp1 + fTemp167)))) / fTemp166;
			self.fRec187[0] = self.fConst2 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow12 + self.fRec5[0]) + 0.5) as i32, 2047))) as usize] } + self.fConst3 * self.fRec187[1];
			let mut fTemp216: F32 = F32::tan(self.fConst4 * F32::max(2e+01, F32::min(1e+04, self.fRec187[0])));
			let mut fTemp217: F32 = 1.0 / fTemp216;
			let mut fTemp218: F32 = fTemp6 + (fTemp5 + fTemp217) / fTemp216 + fTemp2;
			let mut fTemp219: F32 = 1.0 / mydsp_faustpower2_f(fTemp216);
			let mut fTemp220: F32 = fTemp10 + (fTemp9 + fTemp217) / fTemp216 + 1.0;
			let mut fRec203: F32 = -1.0 * 0.9973053 * (0.9 * self.fRec204[2] + 0.05 * (self.fRec204[1] + self.fRec204[3]));
			self.fRec222[0] = self.fConst2 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow12 + self.fRec41[0]) + 0.5) as i32, 2047))) as usize] } + self.fConst3 * self.fRec222[1];
			let mut fTemp221: F32 = self.fConst6 * (3.4e+02 / self.fRec222[0] + -0.11);
			let mut fTemp222: F32 = fTemp221 + -1.499995;
			let mut iTemp223: i32 = (fTemp222) as i32;
			let mut iTemp224: i32 = (F32::min(self.fConst5, (std::cmp::max(0, i32::wrapping_add(iTemp223, 4))) as F32)) as i32;
			let mut iTemp225: i32 = i32::wrapping_add(iTemp224, 1);
			let mut fTemp226: F32 = F32::floor(fTemp222);
			let mut fTemp227: F32 = fTemp221 + (-3.0 - fTemp226);
			let mut fTemp228: F32 = fTemp221 + (-2.0 - fTemp226);
			let mut fTemp229: F32 = fTemp221 + (-1.0 - fTemp226);
			let mut fTemp230: F32 = fTemp229 * fTemp228;
			let mut fTemp231: F32 = fTemp230 * fTemp227;
			let mut fTemp232: F32 = fTemp221 + (-4.0 - fTemp226);
			let mut fTemp233: F32 = 0.0 - fTemp232;
			let mut iTemp234: i32 = (F32::min(self.fConst5, (std::cmp::max(0, i32::wrapping_add(iTemp223, 3))) as F32)) as i32;
			let mut iTemp235: i32 = i32::wrapping_add(iTemp234, 1);
			let mut fTemp236: F32 = 0.0 - 0.5 * fTemp232;
			let mut fTemp237: F32 = 0.0 - fTemp227;
			let mut iTemp238: i32 = (F32::min(self.fConst5, (std::cmp::max(0, i32::wrapping_add(iTemp223, 2))) as F32)) as i32;
			let mut iTemp239: i32 = i32::wrapping_add(iTemp238, 1);
			let mut fTemp240: F32 = 0.0 - 0.33333334 * fTemp232;
			let mut fTemp241: F32 = 0.0 - 0.5 * fTemp227;
			let mut fTemp242: F32 = 0.0 - fTemp228;
			let mut iTemp243: i32 = (F32::min(self.fConst5, (std::cmp::max(0, i32::wrapping_add(iTemp223, 1))) as F32)) as i32;
			let mut iTemp244: i32 = i32::wrapping_add(iTemp243, 1);
			let mut fTemp245: F32 = fTemp221 - fTemp226;
			let mut fTemp246: F32 = 0.0 - 0.25 * fTemp232;
			let mut fTemp247: F32 = 0.0 - 0.33333334 * fTemp227;
			let mut fTemp248: F32 = 0.0 - 0.5 * fTemp228;
			let mut fTemp249: F32 = 0.0 - fTemp229;
			let mut iTemp250: i32 = (F32::min(self.fConst5, (std::cmp::max(0, iTemp223)) as F32)) as i32;
			let mut iTemp251: i32 = i32::wrapping_add(iTemp250, 1);
			self.fRec218[0] = self.fRec195[((i32::wrapping_sub(self.IOTA0, iTemp251)) & 2047) as usize] * fTemp249 * fTemp248 * fTemp247 * fTemp246 + fTemp245 * (self.fRec195[((i32::wrapping_sub(self.IOTA0, iTemp244)) & 2047) as usize] * fTemp242 * fTemp241 * fTemp240 + 0.5 * fTemp229 * self.fRec195[((i32::wrapping_sub(self.IOTA0, iTemp239)) & 2047) as usize] * fTemp237 * fTemp236 + 0.16666667 * fTemp230 * self.fRec195[((i32::wrapping_sub(self.IOTA0, iTemp235)) & 2047) as usize] * fTemp233 + 0.041666668 * fTemp231 * self.fRec195[((i32::wrapping_sub(self.IOTA0, iTemp225)) & 2047) as usize]);
			self.fRec223[0] = 0.95 * self.fRec218[1] + 0.05 * self.fRec223[1];
			let mut fRec219: F32 = self.fRec223[0];
			self.fRec226[0] = self.fConst8 * F32::abs(self.fRec190[1]) + self.fConst7 * self.fRec226[1];
			let mut iTemp252: i32 = (self.fRec226[0] > 0.1) as i32;
			self.iVec17[0] = iTemp252;
			self.iRec227[0] = std::cmp::max(i32::wrapping_mul(self.iConst9, (iTemp252 < self.iVec17[1]) as i32), i32::wrapping_add(self.iRec227[1], -1));
			let mut fTemp253: F32 = F32::abs(F32::max((iTemp252) as F32, ((self.iRec227[0] > 0) as i32) as u32 as F32));
			let mut fTemp254: F32 = if (fTemp253 > self.fRec225[1]) as i32 != 0 {self.fConst7} else {self.fConst10};
			self.fRec225[0] = fTemp253 * (1.0 - fTemp254) + self.fRec225[1] * fTemp254;
			let mut fTemp255: F32 = 0.005 * self.fRec225[0] * self.fRec190[1];
			self.fRec228[0] = self.fRec193[1];
			self.fRec229[(self.IOTA0 & 2047) as usize] = -1.0 * 0.9973053 * (0.9 * self.fRec228[2] + 0.05 * (self.fRec228[1] + self.fRec228[3]));
			let mut fTemp256: F32 = fTemp230 * fTemp233;
			let mut fTemp257: F32 = fTemp229 * fTemp237 * fTemp236;
			let mut fTemp258: F32 = fTemp242 * fTemp241 * fTemp240;
			let mut fTemp259: F32 = fTemp249 * fTemp248 * fTemp247 * fTemp246;
			self.fVec18[0] = fTemp259 * self.fRec229[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp250, 2))) & 2047) as usize] + fTemp245 * (fTemp258 * self.fRec229[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp243, 2))) & 2047) as usize] + 0.5 * fTemp257 * self.fRec229[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp238, 2))) & 2047) as usize] + 0.16666667 * fTemp256 * self.fRec229[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp234, 2))) & 2047) as usize] + 0.041666668 * fTemp231 * self.fRec229[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp224, 2))) & 2047) as usize]);
			let mut fTemp260: F32 = F32::tan(self.fConst11 * self.fRec222[0]);
			let mut fTemp261: F32 = 1.0 / fTemp260;
			let mut fTemp262: F32 = (fTemp261 + 1.4142135) / fTemp260 + 1.0;
			self.fVec19[0] = fSlow13;
			self.iRec230[0] = i32::wrapping_add(i32::wrapping_mul(i32::wrapping_add(self.iRec230[1], (self.iRec230[1] > 0) as i32), (fSlow13 <= self.fVec19[1]) as i32), (fSlow13 > self.fVec19[1]) as i32);
			let mut fTemp263: F32 = (self.iRec230[0]) as F32 / F32::max(1.0, self.fConst12 * mydsp_faustpower2_f(1.0 - 0.0005 * self.fRec222[0]));
			self.fRec231[0] = fTemp55 - (self.fRec231[2] * ((fTemp261 + -1.4142135) / fTemp260 + 1.0) + 2.0 * self.fRec231[1] * (1.0 - 1.0 / mydsp_faustpower2_f(fTemp260))) / fTemp262;
			let mut fTemp264: F32 = 0.5 * ((self.fRec231[2] + self.fRec231[0] + 2.0 * self.fRec231[1]) * F32::max(0.0, F32::min(fTemp263, 2.0 - fTemp263)) / fTemp262);
			let mut fTemp265: F32 = fTemp264 + self.fVec18[1] + fTemp255;
			self.fVec20[0] = fTemp265;
			self.fRec224[(self.IOTA0 & 2047) as usize] = 0.95 * self.fVec20[2] + 0.05 * self.fRec224[((i32::wrapping_sub(self.IOTA0, 1)) & 2047) as usize];
			let mut fRec220: F32 = fTemp259 * self.fRec224[((i32::wrapping_sub(self.IOTA0, iTemp250)) & 2047) as usize] + fTemp245 * (fTemp258 * self.fRec224[((i32::wrapping_sub(self.IOTA0, iTemp243)) & 2047) as usize] + 0.5 * fTemp257 * self.fRec224[((i32::wrapping_sub(self.IOTA0, iTemp238)) & 2047) as usize] + 0.16666667 * fTemp256 * self.fRec224[((i32::wrapping_sub(self.IOTA0, iTemp234)) & 2047) as usize] + 0.041666668 * fTemp231 * self.fRec224[((i32::wrapping_sub(self.IOTA0, iTemp224)) & 2047) as usize]);
			let mut fRec221: F32 = self.fVec20[1] + self.fRec214[1];
			self.fRec214[0] = fRec219;
			let mut fRec215: F32 = self.fRec214[1];
			let mut fRec216: F32 = fRec220;
			let mut fRec217: F32 = fRec221;
			self.fRec210[0] = fRec215;
			let mut fRec211: F32 = fTemp255 + fTemp264 + self.fRec210[1];
			let mut fRec212: F32 = fRec216;
			let mut fRec213: F32 = fRec217;
			self.fRec206[(self.IOTA0 & 2047) as usize] = fRec211;
			let mut fRec207: F32 = fTemp259 * self.fRec206[((i32::wrapping_sub(self.IOTA0, iTemp251)) & 2047) as usize] + fTemp245 * (fTemp258 * self.fRec206[((i32::wrapping_sub(self.IOTA0, iTemp244)) & 2047) as usize] + 0.5 * fTemp257 * self.fRec206[((i32::wrapping_sub(self.IOTA0, iTemp239)) & 2047) as usize] + 0.16666667 * fTemp256 * self.fRec206[((i32::wrapping_sub(self.IOTA0, iTemp235)) & 2047) as usize] + 0.041666668 * fTemp231 * self.fRec206[((i32::wrapping_sub(self.IOTA0, iTemp225)) & 2047) as usize]);
			self.fRec208[0] = fRec212;
			let mut fRec209: F32 = fRec213;
			self.fRec204[0] = fSlow5 * self.fRec208[1];
			let mut fRec205: F32 = fRec209;
			self.fRec199[0] = fRec203;
			let mut fRec200: F32 = fSlow5 * self.fRec199[1];
			let mut fRec201: F32 = self.fRec204[0];
			let mut fRec202: F32 = fRec205;
			self.fRec195[(self.IOTA0 & 2047) as usize] = fRec200;
			let mut fRec196: F32 = fRec207;
			let mut fRec197: F32 = fRec201;
			let mut fRec198: F32 = fRec202;
			self.fRec193[0] = fRec196;
			let mut fRec194: F32 = fRec198;
			let mut fTemp266: F32 = F32::abs(fRec194);
			let mut fTemp267: F32 = if (fTemp266 > self.fRec192[1]) as i32 != 0 {0.0} else {self.fConst13};
			self.fRec192[0] = fTemp266 * (1.0 - fTemp267) + self.fRec192[1] * fTemp267;
			let mut fRec191: F32 = 0.0 - 0.95 * F32::max(2e+01 * F32::log10(F32::max(1.1754944e-38, self.fRec192[0])) + 1e+01, 0.0);
			self.fRec190[0] = 1.5 * fRec194 * F32::powf(1e+01, 0.05 * fRec191);
			self.fRec189[0] = self.fRec190[0] - (self.fRec189[2] * (fTemp10 + (fTemp217 - fTemp9) / fTemp216 + 1.0) + 2.0 * self.fRec189[1] * (fTemp10 + (1.0 - fTemp219))) / fTemp220;
			self.fRec188[0] = (self.fRec189[2] + self.fRec189[0] + 2.0 * self.fRec189[1]) / fTemp220 - (self.fRec188[2] * (fTemp6 + (fTemp217 - fTemp5) / fTemp216 + fTemp2) + 2.0 * self.fRec188[1] * (fTemp6 + (1.0 - (fTemp1 + fTemp219)))) / fTemp218;
			self.fRec232[0] = fSlow14 + self.fConst3 * self.fRec232[1];
			let mut fTemp268: F32 = self.fRec232[0] * ((self.fRec188[2] + self.fRec188[0] + 2.0 * self.fRec188[1]) / fTemp218 + (self.fRec143[2] + self.fRec143[0] + 2.0 * self.fRec143[1]) / fTemp166 + (self.fRec98[2] + self.fRec98[0] + 2.0 * self.fRec98[1]) / fTemp114 + (self.fRec53[2] + self.fRec53[0] + 2.0 * self.fRec53[1]) / fTemp62 + (self.fRec6[2] + self.fRec6[0] + 2.0 * self.fRec6[1]) / fTemp7);
			let mut fTemp269: F32 = self.fRec41[0] + self.fRec5[0];
			self.fRec233[0] = self.fConst2 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow15 + fTemp269) + 0.5) as i32, 2047))) as usize] } + self.fConst3 * self.fRec233[1];
			let mut fTemp270: F32 = F32::tan(self.fConst4 * F32::max(2e+01, F32::min(1e+04, self.fRec233[0])));
			let mut fTemp271: F32 = 1.0 / fTemp270;
			let mut fTemp272: F32 = fTemp6 + (fTemp5 + fTemp271) / fTemp270 + fTemp2;
			let mut fTemp273: F32 = 1.0 / mydsp_faustpower2_f(fTemp270);
			let mut fTemp274: F32 = fTemp10 + (fTemp9 + fTemp271) / fTemp270 + 1.0;
			self.fRec238[0] = self.fConst2 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow15 + self.fRec41[0]) + 0.5) as i32, 2047))) as usize] } + self.fConst3 * self.fRec238[1];
			let mut fTemp275: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec238[0]));
			let mut fTemp276: F32 = self.fRec236[1] + self.fConst14 * fTemp275;
			let mut fTemp277: F32 = fTemp276 + -1.0;
			let mut iTemp278: i32 = (fTemp277 < 0.0) as i32;
			self.fRec236[0] = if iTemp278 != 0 {fTemp276} else {fTemp277};
			let mut fRec237: F32 = if iTemp278 != 0 {fTemp276} else {fTemp276 + (1.0 - self.fConst0 / fTemp275) * fTemp277};
			self.fRec239[0] = fSlow16 + self.fConst3 * self.fRec239[1];
			self.fRec235[0] = self.fRec239[0] * (2.0 * fRec237 + -1.0) - (self.fRec235[2] * (fTemp10 + (fTemp271 - fTemp9) / fTemp270 + 1.0) + 2.0 * self.fRec235[1] * (fTemp10 + (1.0 - fTemp273))) / fTemp274;
			self.fRec234[0] = (self.fRec235[2] + self.fRec235[0] + 2.0 * self.fRec235[1]) / fTemp274 - (self.fRec234[2] * (fTemp6 + (fTemp271 - fTemp5) / fTemp270 + fTemp2) + 2.0 * self.fRec234[1] * (fTemp6 + (1.0 - (fTemp1 + fTemp273)))) / fTemp272;
			self.fRec240[0] = self.fConst2 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow17 + fTemp269) + 0.5) as i32, 2047))) as usize] } + self.fConst3 * self.fRec240[1];
			let mut fTemp279: F32 = F32::tan(self.fConst4 * F32::max(2e+01, F32::min(1e+04, self.fRec240[0])));
			let mut fTemp280: F32 = 1.0 / fTemp279;
			let mut fTemp281: F32 = fTemp6 + (fTemp5 + fTemp280) / fTemp279 + fTemp2;
			let mut fTemp282: F32 = 1.0 / mydsp_faustpower2_f(fTemp279);
			let mut fTemp283: F32 = fTemp10 + (fTemp9 + fTemp280) / fTemp279 + 1.0;
			self.fRec245[0] = self.fConst2 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow17 + self.fRec41[0]) + 0.5) as i32, 2047))) as usize] } + self.fConst3 * self.fRec245[1];
			let mut fTemp284: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec245[0]));
			let mut fTemp285: F32 = self.fRec243[1] + self.fConst14 * fTemp284;
			let mut fTemp286: F32 = fTemp285 + -1.0;
			let mut iTemp287: i32 = (fTemp286 < 0.0) as i32;
			self.fRec243[0] = if iTemp287 != 0 {fTemp285} else {fTemp286};
			let mut fRec244: F32 = if iTemp287 != 0 {fTemp285} else {fTemp285 + (1.0 - self.fConst0 / fTemp284) * fTemp286};
			self.fRec246[0] = fSlow18 + self.fConst3 * self.fRec246[1];
			self.fRec242[0] = self.fRec246[0] * (2.0 * fRec244 + -1.0) - (self.fRec242[2] * (fTemp10 + (fTemp280 - fTemp9) / fTemp279 + 1.0) + 2.0 * self.fRec242[1] * (fTemp10 + (1.0 - fTemp282))) / fTemp283;
			self.fRec241[0] = (self.fRec242[2] + self.fRec242[0] + 2.0 * self.fRec242[1]) / fTemp283 - (self.fRec241[2] * (fTemp6 + (fTemp280 - fTemp5) / fTemp279 + fTemp2) + 2.0 * self.fRec241[1] * (fTemp6 + (1.0 - (fTemp1 + fTemp282)))) / fTemp281;
			self.fRec247[0] = self.fConst2 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow19 + fTemp269) + 0.5) as i32, 2047))) as usize] } + self.fConst3 * self.fRec247[1];
			let mut fTemp288: F32 = F32::tan(self.fConst4 * F32::max(2e+01, F32::min(1e+04, self.fRec247[0])));
			let mut fTemp289: F32 = 1.0 / fTemp288;
			let mut fTemp290: F32 = fTemp6 + (fTemp5 + fTemp289) / fTemp288 + fTemp2;
			let mut fTemp291: F32 = 1.0 / mydsp_faustpower2_f(fTemp288);
			let mut fTemp292: F32 = fTemp10 + (fTemp9 + fTemp289) / fTemp288 + 1.0;
			self.fRec252[0] = self.fConst2 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow19 + self.fRec41[0]) + 0.5) as i32, 2047))) as usize] } + self.fConst3 * self.fRec252[1];
			let mut fTemp293: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec252[0]));
			let mut fTemp294: F32 = self.fRec250[1] + self.fConst14 * fTemp293;
			let mut fTemp295: F32 = fTemp294 + -1.0;
			let mut iTemp296: i32 = (fTemp295 < 0.0) as i32;
			self.fRec250[0] = if iTemp296 != 0 {fTemp294} else {fTemp295};
			let mut fRec251: F32 = if iTemp296 != 0 {fTemp294} else {fTemp294 + (1.0 - self.fConst0 / fTemp293) * fTemp295};
			self.fRec253[0] = fSlow20 + self.fConst3 * self.fRec253[1];
			self.fRec249[0] = self.fRec253[0] * (2.0 * fRec251 + -1.0) - (self.fRec249[2] * (fTemp10 + (fTemp289 - fTemp9) / fTemp288 + 1.0) + 2.0 * self.fRec249[1] * (fTemp10 + (1.0 - fTemp291))) / fTemp292;
			self.fRec248[0] = (self.fRec249[2] + self.fRec249[0] + 2.0 * self.fRec249[1]) / fTemp292 - (self.fRec248[2] * (fTemp6 + (fTemp289 - fTemp5) / fTemp288 + fTemp2) + 2.0 * self.fRec248[1] * (fTemp6 + (1.0 - (fTemp1 + fTemp291)))) / fTemp290;
			self.fRec254[0] = self.fConst2 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow21 + fTemp269) + 0.5) as i32, 2047))) as usize] } + self.fConst3 * self.fRec254[1];
			let mut fTemp297: F32 = F32::tan(self.fConst4 * F32::max(2e+01, F32::min(1e+04, self.fRec254[0])));
			let mut fTemp298: F32 = 1.0 / fTemp297;
			let mut fTemp299: F32 = fTemp6 + (fTemp5 + fTemp298) / fTemp297 + fTemp2;
			let mut fTemp300: F32 = 1.0 / mydsp_faustpower2_f(fTemp297);
			let mut fTemp301: F32 = fTemp10 + (fTemp298 + fTemp9) / fTemp297 + 1.0;
			self.fRec259[0] = self.fConst2 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow21 + self.fRec41[0]) + 0.5) as i32, 2047))) as usize] } + self.fConst3 * self.fRec259[1];
			let mut fTemp302: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec259[0]));
			let mut fTemp303: F32 = self.fRec257[1] + self.fConst14 * fTemp302;
			let mut fTemp304: F32 = fTemp303 + -1.0;
			let mut iTemp305: i32 = (fTemp304 < 0.0) as i32;
			self.fRec257[0] = if iTemp305 != 0 {fTemp303} else {fTemp304};
			let mut fRec258: F32 = if iTemp305 != 0 {fTemp303} else {fTemp303 + (1.0 - self.fConst0 / fTemp302) * fTemp304};
			self.fRec260[0] = fSlow22 + self.fConst3 * self.fRec260[1];
			self.fRec256[0] = self.fRec260[0] * (2.0 * fRec258 + -1.0) - (self.fRec256[2] * (fTemp10 + (fTemp298 - fTemp9) / fTemp297 + 1.0) + 2.0 * self.fRec256[1] * (fTemp10 + (1.0 - fTemp300))) / fTemp301;
			self.fRec255[0] = (self.fRec256[2] + self.fRec256[0] + 2.0 * self.fRec256[1]) / fTemp301 - (self.fRec255[2] * (fTemp6 + (fTemp298 - fTemp5) / fTemp297 + fTemp2) + 2.0 * self.fRec255[1] * (fTemp6 + (1.0 - (fTemp1 + fTemp300)))) / fTemp299;
			self.fRec261[0] = fSlow23 + self.fConst3 * self.fRec261[1];
			self.fRec262[0] = fSlow24 + self.fConst3 * self.fRec262[1];
			let mut fTemp306: F32 = self.fRec262[0] * self.fRec261[0] * ((self.fRec255[2] + self.fRec255[0] + 2.0 * self.fRec255[1]) / fTemp299 + (self.fRec248[2] + self.fRec248[0] + 2.0 * self.fRec248[1]) / fTemp290 + (self.fRec241[2] + self.fRec241[0] + 2.0 * self.fRec241[1]) / fTemp281 + (self.fRec234[2] + self.fRec234[0] + 2.0 * self.fRec234[1]) / fTemp272);
			self.fRec263[0] = fSlow25 + self.fConst3 * self.fRec263[1];
			self.fRec265[0] = fSlow28 + self.fConst3 * self.fRec265[1];
			let mut fTemp307: F32 = F32::max(self.fRec265[0], 23.44895);
			let mut fTemp308: F32 = F32::max(2e+01, F32::abs(fTemp307));
			let mut iTemp309: i32 = i32::wrapping_sub(1, self.iVec0[1]);
			let mut fTemp310: F32 = if iTemp309 != 0 {0.0} else {self.fRec266[1] + self.fConst14 * fTemp308};
			self.fRec266[0] = fTemp310 - F32::floor(fTemp310);
			let mut fTemp311: F32 = mydsp_faustpower2_f(2.0 * self.fRec266[0] + -1.0);
			self.fVec21[0] = fTemp311;
			let mut fTemp312: F32 = (self.iVec0[1]) as F32;
			let mut fTemp313: F32 = fTemp312 * (fTemp311 - self.fVec21[1]) / fTemp308;
			self.fVec22[(self.IOTA0 & 4095) as usize] = fTemp313;
			let mut fTemp314: F32 = F32::max(0.0, F32::min(2047.0, self.fConst15 / fTemp307));
			let mut iTemp315: i32 = (fTemp314) as i32;
			let mut fTemp316: F32 = F32::floor(fTemp314);
			self.fRec264[0] = 0.999 * self.fRec264[1] + self.fConst16 * (fTemp313 - self.fVec22[((i32::wrapping_sub(self.IOTA0, iTemp315)) & 4095) as usize] * (fTemp316 + (1.0 - fTemp314)) - (fTemp314 - fTemp316) * self.fVec22[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp315, 1))) & 4095) as usize]);
			self.fRec268[0] = fSlow29 + self.fConst3 * self.fRec268[1];
			let mut fTemp317: F32 = F32::max(self.fRec268[0], 23.44895);
			let mut fTemp318: F32 = F32::max(2e+01, F32::abs(fTemp317));
			let mut fTemp319: F32 = if iTemp309 != 0 {0.0} else {self.fRec269[1] + self.fConst14 * fTemp318};
			self.fRec269[0] = fTemp319 - F32::floor(fTemp319);
			let mut fTemp320: F32 = mydsp_faustpower2_f(2.0 * self.fRec269[0] + -1.0);
			self.fVec23[0] = fTemp320;
			let mut fTemp321: F32 = fTemp312 * (fTemp320 - self.fVec23[1]) / fTemp318;
			self.fVec24[(self.IOTA0 & 4095) as usize] = fTemp321;
			let mut fTemp322: F32 = F32::max(0.0, F32::min(2047.0, self.fConst15 / fTemp317));
			let mut iTemp323: i32 = (fTemp322) as i32;
			let mut fTemp324: F32 = F32::floor(fTemp322);
			self.fRec267[0] = 0.999 * self.fRec267[1] + self.fConst16 * (fTemp321 - self.fVec24[((i32::wrapping_sub(self.IOTA0, iTemp323)) & 4095) as usize] * (fTemp324 + (1.0 - fTemp322)) - (fTemp322 - fTemp324) * self.fVec24[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp323, 1))) & 4095) as usize]);
			self.fRec271[0] = fSlow30 + self.fConst3 * self.fRec271[1];
			let mut fTemp325: F32 = F32::max(self.fRec271[0], 23.44895);
			let mut fTemp326: F32 = F32::max(2e+01, F32::abs(fTemp325));
			let mut fTemp327: F32 = if iTemp309 != 0 {0.0} else {self.fRec272[1] + self.fConst14 * fTemp326};
			self.fRec272[0] = fTemp327 - F32::floor(fTemp327);
			let mut fTemp328: F32 = mydsp_faustpower2_f(2.0 * self.fRec272[0] + -1.0);
			self.fVec25[0] = fTemp328;
			let mut fTemp329: F32 = fTemp312 * (fTemp328 - self.fVec25[1]) / fTemp326;
			self.fVec26[(self.IOTA0 & 4095) as usize] = fTemp329;
			let mut fTemp330: F32 = F32::max(0.0, F32::min(2047.0, self.fConst15 / fTemp325));
			let mut iTemp331: i32 = (fTemp330) as i32;
			let mut fTemp332: F32 = F32::floor(fTemp330);
			self.fRec270[0] = 0.999 * self.fRec270[1] + self.fConst16 * (fTemp329 - self.fVec26[((i32::wrapping_sub(self.IOTA0, iTemp331)) & 4095) as usize] * (fTemp332 + (1.0 - fTemp330)) - (fTemp330 - fTemp332) * self.fVec26[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp331, 1))) & 4095) as usize]);
			self.fRec273[0] = fSlow31 + self.fConst3 * self.fRec273[1];
			self.fRec275[0] = fSlow33 + self.fConst3 * self.fRec275[1];
			let mut fTemp333: F32 = F32::max(self.fRec275[0], 23.44895);
			let mut fTemp334: F32 = F32::max(2e+01, F32::abs(fTemp333));
			let mut fTemp335: F32 = if iTemp309 != 0 {0.0} else {self.fRec276[1] + self.fConst14 * fTemp334};
			self.fRec276[0] = fTemp335 - F32::floor(fTemp335);
			let mut fTemp336: F32 = mydsp_faustpower2_f(2.0 * self.fRec276[0] + -1.0);
			self.fVec27[0] = fTemp336;
			let mut fTemp337: F32 = fTemp312 * (fTemp336 - self.fVec27[1]) / fTemp334;
			self.fVec28[(self.IOTA0 & 4095) as usize] = fTemp337;
			let mut fTemp338: F32 = F32::max(0.0, F32::min(2047.0, self.fConst15 / fTemp333));
			let mut iTemp339: i32 = (fTemp338) as i32;
			let mut fTemp340: F32 = F32::floor(fTemp338);
			self.fRec274[0] = 0.999 * self.fRec274[1] + self.fConst16 * (fTemp337 - self.fVec28[((i32::wrapping_sub(self.IOTA0, iTemp339)) & 4095) as usize] * (fTemp340 + (1.0 - fTemp338)) - (fTemp338 - fTemp340) * self.fVec28[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp339, 1))) & 4095) as usize]);
			self.fRec278[0] = fSlow34 + self.fConst3 * self.fRec278[1];
			let mut fTemp341: F32 = F32::max(self.fRec278[0], 23.44895);
			let mut fTemp342: F32 = F32::max(2e+01, F32::abs(fTemp341));
			let mut fTemp343: F32 = if iTemp309 != 0 {0.0} else {self.fRec279[1] + self.fConst14 * fTemp342};
			self.fRec279[0] = fTemp343 - F32::floor(fTemp343);
			let mut fTemp344: F32 = mydsp_faustpower2_f(2.0 * self.fRec279[0] + -1.0);
			self.fVec29[0] = fTemp344;
			let mut fTemp345: F32 = fTemp312 * (fTemp344 - self.fVec29[1]) / fTemp342;
			self.fVec30[(self.IOTA0 & 4095) as usize] = fTemp345;
			let mut fTemp346: F32 = F32::max(0.0, F32::min(2047.0, self.fConst15 / fTemp341));
			let mut iTemp347: i32 = (fTemp346) as i32;
			let mut fTemp348: F32 = F32::floor(fTemp346);
			self.fRec277[0] = 0.999 * self.fRec277[1] + self.fConst16 * (fTemp345 - self.fVec30[((i32::wrapping_sub(self.IOTA0, iTemp347)) & 4095) as usize] * (fTemp348 + (1.0 - fTemp346)) - (fTemp346 - fTemp348) * self.fVec30[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp347, 1))) & 4095) as usize]);
			self.fRec281[0] = fSlow35 + self.fConst3 * self.fRec281[1];
			let mut fTemp349: F32 = F32::max(self.fRec281[0], 23.44895);
			let mut fTemp350: F32 = F32::max(2e+01, F32::abs(fTemp349));
			let mut fTemp351: F32 = if iTemp309 != 0 {0.0} else {self.fRec282[1] + self.fConst14 * fTemp350};
			self.fRec282[0] = fTemp351 - F32::floor(fTemp351);
			let mut fTemp352: F32 = mydsp_faustpower2_f(2.0 * self.fRec282[0] + -1.0);
			self.fVec31[0] = fTemp352;
			let mut fTemp353: F32 = fTemp312 * (fTemp352 - self.fVec31[1]) / fTemp350;
			self.fVec32[(self.IOTA0 & 4095) as usize] = fTemp353;
			let mut fTemp354: F32 = F32::max(0.0, F32::min(2047.0, self.fConst15 / fTemp349));
			let mut iTemp355: i32 = (fTemp354) as i32;
			let mut fTemp356: F32 = F32::floor(fTemp354);
			self.fRec280[0] = 0.999 * self.fRec280[1] + self.fConst16 * (fTemp353 - self.fVec32[((i32::wrapping_sub(self.IOTA0, iTemp355)) & 4095) as usize] * (fTemp356 + (1.0 - fTemp354)) - (fTemp354 - fTemp356) * self.fVec32[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp355, 1))) & 4095) as usize]);
			self.fRec283[0] = fSlow36 + self.fConst3 * self.fRec283[1];
			self.fRec285[0] = fSlow38 + self.fConst3 * self.fRec285[1];
			let mut fTemp357: F32 = F32::max(self.fRec285[0], 23.44895);
			let mut fTemp358: F32 = F32::max(2e+01, F32::abs(fTemp357));
			let mut fTemp359: F32 = if iTemp309 != 0 {0.0} else {self.fRec286[1] + self.fConst14 * fTemp358};
			self.fRec286[0] = fTemp359 - F32::floor(fTemp359);
			let mut fTemp360: F32 = mydsp_faustpower2_f(2.0 * self.fRec286[0] + -1.0);
			self.fVec33[0] = fTemp360;
			let mut fTemp361: F32 = fTemp312 * (fTemp360 - self.fVec33[1]) / fTemp358;
			self.fVec34[(self.IOTA0 & 4095) as usize] = fTemp361;
			let mut fTemp362: F32 = F32::max(0.0, F32::min(2047.0, self.fConst15 / fTemp357));
			let mut iTemp363: i32 = (fTemp362) as i32;
			let mut fTemp364: F32 = F32::floor(fTemp362);
			self.fRec284[0] = 0.999 * self.fRec284[1] + self.fConst16 * (fTemp361 - self.fVec34[((i32::wrapping_sub(self.IOTA0, iTemp363)) & 4095) as usize] * (fTemp364 + (1.0 - fTemp362)) - (fTemp362 - fTemp364) * self.fVec34[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp363, 1))) & 4095) as usize]);
			self.fRec288[0] = fSlow39 + self.fConst3 * self.fRec288[1];
			let mut fTemp365: F32 = F32::max(self.fRec288[0], 23.44895);
			let mut fTemp366: F32 = F32::max(2e+01, F32::abs(fTemp365));
			let mut fTemp367: F32 = if iTemp309 != 0 {0.0} else {self.fRec289[1] + self.fConst14 * fTemp366};
			self.fRec289[0] = fTemp367 - F32::floor(fTemp367);
			let mut fTemp368: F32 = mydsp_faustpower2_f(2.0 * self.fRec289[0] + -1.0);
			self.fVec35[0] = fTemp368;
			let mut fTemp369: F32 = fTemp312 * (fTemp368 - self.fVec35[1]) / fTemp366;
			self.fVec36[(self.IOTA0 & 4095) as usize] = fTemp369;
			let mut fTemp370: F32 = F32::max(0.0, F32::min(2047.0, self.fConst15 / fTemp365));
			let mut iTemp371: i32 = (fTemp370) as i32;
			let mut fTemp372: F32 = F32::floor(fTemp370);
			self.fRec287[0] = 0.999 * self.fRec287[1] + self.fConst16 * (fTemp369 - self.fVec36[((i32::wrapping_sub(self.IOTA0, iTemp371)) & 4095) as usize] * (fTemp372 + (1.0 - fTemp370)) - (fTemp370 - fTemp372) * self.fVec36[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp371, 1))) & 4095) as usize]);
			self.fRec291[0] = fSlow40 + self.fConst3 * self.fRec291[1];
			let mut fTemp373: F32 = F32::max(self.fRec291[0], 23.44895);
			let mut fTemp374: F32 = F32::max(2e+01, F32::abs(fTemp373));
			let mut fTemp375: F32 = if iTemp309 != 0 {0.0} else {self.fRec292[1] + self.fConst14 * fTemp374};
			self.fRec292[0] = fTemp375 - F32::floor(fTemp375);
			let mut fTemp376: F32 = mydsp_faustpower2_f(2.0 * self.fRec292[0] + -1.0);
			self.fVec37[0] = fTemp376;
			let mut fTemp377: F32 = fTemp312 * (fTemp376 - self.fVec37[1]) / fTemp374;
			self.fVec38[(self.IOTA0 & 4095) as usize] = fTemp377;
			let mut fTemp378: F32 = F32::max(0.0, F32::min(2047.0, self.fConst15 / fTemp373));
			let mut iTemp379: i32 = (fTemp378) as i32;
			let mut fTemp380: F32 = F32::floor(fTemp378);
			self.fRec290[0] = 0.999 * self.fRec290[1] + self.fConst16 * (fTemp377 - self.fVec38[((i32::wrapping_sub(self.IOTA0, iTemp379)) & 4095) as usize] * (fTemp380 + (1.0 - fTemp378)) - (fTemp378 - fTemp380) * self.fVec38[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp379, 1))) & 4095) as usize]);
			self.fRec293[0] = fSlow41 + self.fConst3 * self.fRec293[1];
			self.fRec295[0] = fSlow43 + self.fConst3 * self.fRec295[1];
			let mut fTemp381: F32 = F32::max(self.fRec295[0], 23.44895);
			let mut fTemp382: F32 = F32::max(2e+01, F32::abs(fTemp381));
			let mut fTemp383: F32 = if iTemp309 != 0 {0.0} else {self.fRec296[1] + self.fConst14 * fTemp382};
			self.fRec296[0] = fTemp383 - F32::floor(fTemp383);
			let mut fTemp384: F32 = mydsp_faustpower2_f(2.0 * self.fRec296[0] + -1.0);
			self.fVec39[0] = fTemp384;
			let mut fTemp385: F32 = fTemp312 * (fTemp384 - self.fVec39[1]) / fTemp382;
			self.fVec40[(self.IOTA0 & 4095) as usize] = fTemp385;
			let mut fTemp386: F32 = F32::max(0.0, F32::min(2047.0, self.fConst15 / fTemp381));
			let mut iTemp387: i32 = (fTemp386) as i32;
			let mut fTemp388: F32 = F32::floor(fTemp386);
			self.fRec294[0] = 0.999 * self.fRec294[1] + self.fConst16 * (fTemp385 - self.fVec40[((i32::wrapping_sub(self.IOTA0, iTemp387)) & 4095) as usize] * (fTemp388 + (1.0 - fTemp386)) - (fTemp386 - fTemp388) * self.fVec40[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp387, 1))) & 4095) as usize]);
			self.fRec298[0] = fSlow44 + self.fConst3 * self.fRec298[1];
			let mut fTemp389: F32 = F32::max(self.fRec298[0], 23.44895);
			let mut fTemp390: F32 = F32::max(2e+01, F32::abs(fTemp389));
			let mut fTemp391: F32 = if iTemp309 != 0 {0.0} else {self.fRec299[1] + self.fConst14 * fTemp390};
			self.fRec299[0] = fTemp391 - F32::floor(fTemp391);
			let mut fTemp392: F32 = mydsp_faustpower2_f(2.0 * self.fRec299[0] + -1.0);
			self.fVec41[0] = fTemp392;
			let mut fTemp393: F32 = fTemp312 * (fTemp392 - self.fVec41[1]) / fTemp390;
			self.fVec42[(self.IOTA0 & 4095) as usize] = fTemp393;
			let mut fTemp394: F32 = F32::max(0.0, F32::min(2047.0, self.fConst15 / fTemp389));
			let mut iTemp395: i32 = (fTemp394) as i32;
			let mut fTemp396: F32 = F32::floor(fTemp394);
			self.fRec297[0] = 0.999 * self.fRec297[1] + self.fConst16 * (fTemp393 - self.fVec42[((i32::wrapping_sub(self.IOTA0, iTemp395)) & 4095) as usize] * (fTemp396 + (1.0 - fTemp394)) - (fTemp394 - fTemp396) * self.fVec42[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp395, 1))) & 4095) as usize]);
			self.fRec301[0] = fSlow45 + self.fConst3 * self.fRec301[1];
			let mut fTemp397: F32 = F32::max(self.fRec301[0], 23.44895);
			let mut fTemp398: F32 = F32::max(2e+01, F32::abs(fTemp397));
			let mut fTemp399: F32 = if iTemp309 != 0 {0.0} else {self.fRec302[1] + self.fConst14 * fTemp398};
			self.fRec302[0] = fTemp399 - F32::floor(fTemp399);
			let mut fTemp400: F32 = mydsp_faustpower2_f(2.0 * self.fRec302[0] + -1.0);
			self.fVec43[0] = fTemp400;
			let mut fTemp401: F32 = fTemp312 * (fTemp400 - self.fVec43[1]) / fTemp398;
			self.fVec44[(self.IOTA0 & 4095) as usize] = fTemp401;
			let mut fTemp402: F32 = F32::max(0.0, F32::min(2047.0, self.fConst15 / fTemp397));
			let mut iTemp403: i32 = (fTemp402) as i32;
			let mut fTemp404: F32 = F32::floor(fTemp402);
			self.fRec300[0] = 0.999 * self.fRec300[1] + self.fConst16 * (fTemp401 - self.fVec44[((i32::wrapping_sub(self.IOTA0, iTemp403)) & 4095) as usize] * (fTemp404 + (1.0 - fTemp402)) - (fTemp402 - fTemp404) * self.fVec44[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp403, 1))) & 4095) as usize]);
			self.fRec303[0] = fSlow46 + self.fConst3 * self.fRec303[1];
			let mut fTemp405: F32 = F32::max(-1.0, F32::min(1.0, self.fRec263[0] + self.fConst17 * (self.fRec303[0] * (self.fRec301[0] * self.fRec300[0] + self.fRec298[0] * self.fRec297[0] + self.fRec295[0] * self.fRec294[0]) + self.fRec293[0] * (self.fRec291[0] * self.fRec290[0] + self.fRec288[0] * self.fRec287[0] + self.fRec285[0] * self.fRec284[0]) + self.fRec283[0] * (self.fRec281[0] * self.fRec280[0] + self.fRec278[0] * self.fRec277[0] + self.fRec275[0] * self.fRec274[0]) + self.fRec273[0] * (self.fRec271[0] * self.fRec270[0] + self.fRec268[0] * self.fRec267[0] + self.fRec265[0] * self.fRec264[0])) * F32::powf(1e+01, 0.6666667 * self.fRec263[0])));
			self.fRec304[0] = fSlow47 + self.fConst3 * self.fRec304[1];
			let mut fTemp406: F32 = self.fRec304[0] * fTemp405 * (1.0 - 0.33333334 * mydsp_faustpower2_f(fTemp405));
			self.fRec305[0] = fSlow48 + self.fConst3 * self.fRec305[1];
			let mut fTemp407: F32 = (1.0 - self.fRec305[0]) * (fTemp406 + fTemp306 + fTemp268);
			self.fRec307[0] = fSlow49 + self.fConst3 * self.fRec307[1];
			self.fRec306[(self.IOTA0 & 2097151) as usize] = fTemp268 + fTemp306 + fSlow50 * self.fRec306[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add((F32::min(self.fConst18, F32::max(0.0, self.fConst0 * self.fRec307[0]))) as i32, 1))) & 2097151) as usize] + fTemp406;
			let mut fTemp408: F32 = self.fRec306[(self.IOTA0 & 2097151) as usize] * self.fRec305[0];
			let mut fTemp409: F32 = fTemp408 + fTemp407;
			self.fRec318[0] = 0.995 * (self.fRec318[1] + (i32::wrapping_mul(iTemp309, iSlow55)) as F32) + fSlow56;
			let mut fTemp410: F32 = self.fRec318[0] + -1.49999;
			let mut fTemp411: F32 = F32::floor(fTemp410);
			self.fRec320[0] = 0.995 * (self.fRec320[1] + (i32::wrapping_mul(iTemp309, iSlow57)) as F32) + fSlow58;
			let mut fTemp412: F32 = self.fRec320[0] + -1.49999;
			let mut fTemp413: F32 = F32::floor(fTemp412);
			self.fRec324[0] = 0.9999 * (self.fRec324[1] + (i32::wrapping_mul(iTemp309, iSlow59)) as F32) + fSlow60;
			let mut fTemp414: F32 = self.fRec324[0] + -1.49999;
			let mut fTemp415: F32 = F32::floor(fTemp414);
			let mut fTemp416: F32 = 0.760314 * self.fRec309[1] - 0.64955574 * self.fRec322[1];
			let mut fTemp417: F32 = 0.760314 * self.fRec308[1] - 0.64955574 * self.fRec321[1];
			self.fVec45[(self.IOTA0 & 16383) as usize] = 0.70710677 * fTemp417 - 0.70710677 * fTemp416;
			let mut fTemp418: F32 = self.fVec45[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, (fTemp414) as i32)))) & 16383) as usize];
			self.fVec46[0] = fTemp418;
			self.fRec323[0] = self.fVec46[1] - (fTemp415 + (2.0 - self.fRec324[0])) * (self.fRec323[1] - fTemp418) / (self.fRec324[0] - fTemp415);
			self.fRec321[0] = self.fRec323[0];
			self.fRec326[0] = 0.9999 * (self.fRec326[1] + (i32::wrapping_mul(iTemp309, iSlow61)) as F32) + fSlow62;
			let mut fTemp419: F32 = self.fRec326[0] + -1.49999;
			let mut fTemp420: F32 = F32::floor(fTemp419);
			self.fVec47[(self.IOTA0 & 16383) as usize] = 0.70710677 * fTemp417 + 0.70710677 * fTemp416;
			let mut fTemp421: F32 = self.fVec47[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, (fTemp419) as i32)))) & 16383) as usize];
			self.fVec48[0] = fTemp421;
			self.fRec325[0] = self.fVec48[1] - (fTemp420 + (2.0 - self.fRec326[0])) * (self.fRec325[1] - fTemp421) / (self.fRec326[0] - fTemp420);
			self.fRec322[0] = self.fRec325[0];
			let mut fTemp422: F32 = 0.760314 * self.fRec321[1] + 0.64955574 * self.fRec308[1];
			self.fRec330[0] = 0.9999 * (self.fRec330[1] + (i32::wrapping_mul(iTemp309, iSlow63)) as F32) + fSlow64;
			let mut fTemp423: F32 = self.fRec330[0] + -1.49999;
			let mut fTemp424: F32 = F32::floor(fTemp423);
			let mut fTemp425: F32 = 0.760314 * self.fRec322[1] + 0.64955574 * self.fRec309[1];
			let mut fTemp426: F32 = 0.760314 * fTemp425 - 0.64955574 * self.fRec328[1];
			let mut fTemp427: F32 = 0.760314 * fTemp422 - 0.64955574 * self.fRec327[1];
			self.fVec49[(self.IOTA0 & 16383) as usize] = 0.70710677 * fTemp427 - 0.70710677 * fTemp426;
			let mut fTemp428: F32 = self.fVec49[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, (fTemp423) as i32)))) & 16383) as usize];
			self.fVec50[0] = fTemp428;
			self.fRec329[0] = self.fVec50[1] - (fTemp424 + (2.0 - self.fRec330[0])) * (self.fRec329[1] - fTemp428) / (self.fRec330[0] - fTemp424);
			self.fRec327[0] = self.fRec329[0];
			self.fRec332[0] = 0.9999 * (self.fRec332[1] + (i32::wrapping_mul(iTemp309, iSlow65)) as F32) + fSlow66;
			let mut fTemp429: F32 = self.fRec332[0] + -1.49999;
			let mut fTemp430: F32 = F32::floor(fTemp429);
			self.fVec51[(self.IOTA0 & 16383) as usize] = 0.70710677 * fTemp427 + 0.70710677 * fTemp426;
			let mut fTemp431: F32 = self.fVec51[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, (fTemp429) as i32)))) & 16383) as usize];
			self.fVec52[0] = fTemp431;
			self.fRec331[0] = self.fVec52[1] - (fTemp430 + (2.0 - self.fRec332[0])) * (self.fRec331[1] - fTemp431) / (self.fRec332[0] - fTemp430);
			self.fRec328[0] = self.fRec331[0];
			let mut fTemp432: F32 = 0.760314 * self.fRec327[1] + 0.64955574 * fTemp422;
			self.fRec336[0] = 0.9999 * (self.fRec336[1] + (i32::wrapping_mul(iTemp309, iSlow67)) as F32) + fSlow68;
			let mut fTemp433: F32 = self.fRec336[0] + -1.49999;
			let mut fTemp434: F32 = F32::floor(fTemp433);
			let mut fTemp435: F32 = 0.760314 * self.fRec328[1] + 0.64955574 * fTemp425;
			let mut fTemp436: F32 = 0.760314 * fTemp435 - 0.64955574 * self.fRec334[1];
			let mut fTemp437: F32 = 0.760314 * fTemp432 - 0.64955574 * self.fRec333[1];
			self.fVec53[(self.IOTA0 & 16383) as usize] = 0.70710677 * fTemp437 - 0.70710677 * fTemp436;
			let mut fTemp438: F32 = self.fVec53[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, (fTemp433) as i32)))) & 16383) as usize];
			self.fVec54[0] = fTemp438;
			self.fRec335[0] = self.fVec54[1] - (fTemp434 + (2.0 - self.fRec336[0])) * (self.fRec335[1] - fTemp438) / (self.fRec336[0] - fTemp434);
			self.fRec333[0] = self.fRec335[0];
			self.fRec338[0] = 0.9999 * (self.fRec338[1] + (i32::wrapping_mul(iTemp309, iSlow69)) as F32) + fSlow70;
			let mut fTemp439: F32 = self.fRec338[0] + -1.49999;
			let mut fTemp440: F32 = F32::floor(fTemp439);
			self.fVec55[(self.IOTA0 & 16383) as usize] = 0.70710677 * fTemp437 + 0.70710677 * fTemp436;
			let mut fTemp441: F32 = self.fVec55[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, (fTemp439) as i32)))) & 16383) as usize];
			self.fVec56[0] = fTemp441;
			self.fRec337[0] = self.fVec56[1] - (fTemp440 + (2.0 - self.fRec338[0])) * (self.fRec337[1] - fTemp441) / (self.fRec338[0] - fTemp440);
			self.fRec334[0] = self.fRec337[0];
			let mut fTemp442: F32 = 0.760314 * self.fRec333[1] + 0.64955574 * fTemp432;
			self.fRec342[0] = 0.9999 * (self.fRec342[1] + (i32::wrapping_mul(iTemp309, iSlow71)) as F32) + fSlow72;
			let mut fTemp443: F32 = self.fRec342[0] + -1.49999;
			let mut fTemp444: F32 = F32::floor(fTemp443);
			let mut fTemp445: F32 = 0.760314 * self.fRec334[1] + 0.64955574 * fTemp435;
			let mut fTemp446: F32 = 0.760314 * fTemp445 - 0.64955574 * self.fRec340[1];
			let mut fTemp447: F32 = 0.760314 * fTemp442 - 0.64955574 * self.fRec339[1];
			self.fVec57[(self.IOTA0 & 16383) as usize] = 0.70710677 * fTemp447 - 0.70710677 * fTemp446;
			let mut fTemp448: F32 = self.fVec57[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, (fTemp443) as i32)))) & 16383) as usize];
			self.fVec58[0] = fTemp448;
			self.fRec341[0] = self.fVec58[1] - (fTemp444 + (2.0 - self.fRec342[0])) * (self.fRec341[1] - fTemp448) / (self.fRec342[0] - fTemp444);
			self.fRec339[0] = self.fRec341[0];
			self.fRec344[0] = 0.9999 * (self.fRec344[1] + (i32::wrapping_mul(iTemp309, iSlow73)) as F32) + fSlow74;
			let mut fTemp449: F32 = self.fRec344[0] + -1.49999;
			let mut fTemp450: F32 = F32::floor(fTemp449);
			self.fVec59[(self.IOTA0 & 16383) as usize] = 0.70710677 * fTemp447 + 0.70710677 * fTemp446;
			let mut fTemp451: F32 = self.fVec59[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, (fTemp449) as i32)))) & 16383) as usize];
			self.fVec60[0] = fTemp451;
			self.fRec343[0] = self.fVec60[1] - (fTemp450 + (2.0 - self.fRec344[0])) * (self.fRec343[1] - fTemp451) / (self.fRec344[0] - fTemp450);
			self.fRec340[0] = self.fRec343[0];
			let mut fTemp452: F32 = 0.760314 * self.fRec339[1] + 0.64955574 * fTemp442;
			self.fRec348[0] = 0.9999 * (self.fRec348[1] + (i32::wrapping_mul(iTemp309, iSlow75)) as F32) + fSlow76;
			let mut fTemp453: F32 = self.fRec348[0] + -1.49999;
			let mut fTemp454: F32 = F32::floor(fTemp453);
			let mut fTemp455: F32 = 0.760314 * self.fRec340[1] + 0.64955574 * fTemp445;
			let mut fTemp456: F32 = 0.760314 * fTemp455 - 0.64955574 * self.fRec346[1];
			let mut fTemp457: F32 = 0.760314 * fTemp452 - 0.64955574 * self.fRec345[1];
			self.fVec61[(self.IOTA0 & 16383) as usize] = 0.70710677 * fTemp457 - 0.70710677 * fTemp456;
			let mut fTemp458: F32 = self.fVec61[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, (fTemp453) as i32)))) & 16383) as usize];
			self.fVec62[0] = fTemp458;
			self.fRec347[0] = self.fVec62[1] - (fTemp454 + (2.0 - self.fRec348[0])) * (self.fRec347[1] - fTemp458) / (self.fRec348[0] - fTemp454);
			self.fRec345[0] = self.fRec347[0];
			self.fRec350[0] = 0.9999 * (self.fRec350[1] + (i32::wrapping_mul(iTemp309, iSlow77)) as F32) + fSlow78;
			let mut fTemp459: F32 = self.fRec350[0] + -1.49999;
			let mut fTemp460: F32 = F32::floor(fTemp459);
			self.fVec63[(self.IOTA0 & 16383) as usize] = 0.70710677 * fTemp457 + 0.70710677 * fTemp456;
			let mut fTemp461: F32 = self.fVec63[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, (fTemp459) as i32)))) & 16383) as usize];
			self.fVec64[0] = fTemp461;
			self.fRec349[0] = self.fVec64[1] - (fTemp460 + (2.0 - self.fRec350[0])) * (self.fRec349[1] - fTemp461) / (self.fRec350[0] - fTemp460);
			self.fRec346[0] = self.fRec349[0];
			let mut fTemp462: F32 = 0.760314 * self.fRec345[1] + 0.64955574 * fTemp452;
			self.fVec65[(self.IOTA0 & 1023) as usize] = fTemp462;
			self.fRec351[0] = fSlow81 * self.fRec352[1] + fSlow80 * self.fRec351[1];
			self.fRec352[0] = (iTemp309) as F32 + fSlow80 * self.fRec352[1] - fSlow81 * self.fRec351[1];
			let mut fTemp463: F32 = fSlow82 * (self.fRec352[0] + 1.0);
			let mut fTemp464: F32 = fTemp463 + 3.500005;
			let mut iTemp465: i32 = (fTemp464) as i32;
			let mut iTemp466: i32 = std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp465, 4)));
			let mut fTemp467: F32 = F32::floor(fTemp464);
			let mut fTemp468: F32 = fTemp463 + (2.0 - fTemp467);
			let mut fTemp469: F32 = fTemp463 + (3.0 - fTemp467);
			let mut fTemp470: F32 = fTemp463 + (4.0 - fTemp467);
			let mut fTemp471: F32 = fTemp470 * fTemp469;
			let mut fTemp472: F32 = fTemp471 * fTemp468;
			let mut fTemp473: F32 = fTemp463 + (1.0 - fTemp467);
			let mut fTemp474: F32 = 0.0 - fTemp473;
			let mut iTemp475: i32 = std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp465, 3)));
			let mut fTemp476: F32 = 0.0 - 0.5 * fTemp473;
			let mut fTemp477: F32 = 0.0 - fTemp468;
			let mut iTemp478: i32 = std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp465, 2)));
			let mut fTemp479: F32 = 0.0 - 0.33333334 * fTemp473;
			let mut fTemp480: F32 = 0.0 - 0.5 * fTemp468;
			let mut fTemp481: F32 = 0.0 - fTemp469;
			let mut iTemp482: i32 = std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp465, 1)));
			let mut fTemp483: F32 = fTemp463 + (5.0 - fTemp467);
			let mut fTemp484: F32 = 0.0 - 0.25 * fTemp473;
			let mut fTemp485: F32 = 0.0 - 0.33333334 * fTemp468;
			let mut fTemp486: F32 = 0.0 - 0.5 * fTemp469;
			let mut fTemp487: F32 = 0.0 - fTemp470;
			let mut iTemp488: i32 = std::cmp::min(512, std::cmp::max(0, iTemp465));
			self.fVec66[(self.IOTA0 & 16383) as usize] = self.fVec65[((i32::wrapping_sub(self.IOTA0, iTemp488)) & 1023) as usize] * fTemp487 * fTemp486 * fTemp485 * fTemp484 + fTemp483 * (self.fVec65[((i32::wrapping_sub(self.IOTA0, iTemp482)) & 1023) as usize] * fTemp481 * fTemp480 * fTemp479 + 0.5 * fTemp470 * self.fVec65[((i32::wrapping_sub(self.IOTA0, iTemp478)) & 1023) as usize] * fTemp477 * fTemp476 + 0.16666667 * fTemp471 * self.fVec65[((i32::wrapping_sub(self.IOTA0, iTemp475)) & 1023) as usize] * fTemp474 + 0.041666668 * fTemp472 * self.fVec65[((i32::wrapping_sub(self.IOTA0, iTemp466)) & 1023) as usize]);
			let mut fTemp489: F32 = self.fVec66[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, (fTemp412) as i32)))) & 16383) as usize];
			self.fVec67[0] = fTemp489;
			self.fRec319[0] = self.fVec67[1] - (fTemp413 + (2.0 - self.fRec320[0])) * (self.fRec319[1] - fTemp489) / (self.fRec320[0] - fTemp413);
			self.fRec356[0] = 0.9999 * (self.fRec356[1] + (i32::wrapping_mul(iTemp309, iSlow83)) as F32) + fSlow84;
			let mut fTemp490: F32 = self.fRec356[0] + -1.49999;
			let mut fTemp491: F32 = F32::floor(fTemp490);
			self.fRec358[0] = 0.995 * (self.fRec358[1] + (i32::wrapping_mul(iTemp309, iSlow85)) as F32) + fSlow86;
			let mut fTemp492: F32 = self.fRec358[0] + -1.49999;
			let mut fTemp493: F32 = F32::floor(fTemp492);
			let mut fTemp494: F32 = 0.760314 * self.fRec346[1] + 0.64955574 * fTemp455;
			self.fVec68[(self.IOTA0 & 1023) as usize] = fTemp494;
			let mut fTemp495: F32 = fSlow87 * self.fRec352[0];
			let mut fTemp496: F32 = fSlow82 + fTemp495 + 3.500005;
			let mut iTemp497: i32 = (fTemp496) as i32;
			let mut fTemp498: F32 = F32::floor(fTemp496);
			let mut fTemp499: F32 = fSlow82 + fTemp495 + (2.0 - fTemp498);
			let mut fTemp500: F32 = fSlow82 + fTemp495 + (3.0 - fTemp498);
			let mut fTemp501: F32 = fSlow82 + fTemp495 + (4.0 - fTemp498);
			let mut fTemp502: F32 = fTemp501 * fTemp500;
			let mut fTemp503: F32 = fSlow82 + fTemp495 + (1.0 - fTemp498);
			self.fVec69[(self.IOTA0 & 16383) as usize] = self.fVec68[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, std::cmp::max(0, iTemp497)))) & 1023) as usize] * (0.0 - fTemp501) * (0.0 - 0.5 * fTemp500) * (0.0 - 0.33333334 * fTemp499) * (0.0 - 0.25 * fTemp503) + (fSlow82 + fTemp495 + (5.0 - fTemp498)) * (self.fVec68[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp497, 1))))) & 1023) as usize] * (0.0 - fTemp500) * (0.0 - 0.5 * fTemp499) * (0.0 - 0.33333334 * fTemp503) + 0.5 * fTemp501 * self.fVec68[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp497, 2))))) & 1023) as usize] * (0.0 - fTemp499) * (0.0 - 0.5 * fTemp503) + 0.16666667 * fTemp502 * self.fVec68[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp497, 3))))) & 1023) as usize] * (0.0 - fTemp503) + 0.041666668 * fTemp502 * fTemp499 * self.fVec68[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp497, 4))))) & 1023) as usize]);
			let mut fTemp504: F32 = self.fVec69[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, (fTemp492) as i32)))) & 16383) as usize];
			self.fVec70[0] = fTemp504;
			self.fRec357[0] = self.fVec70[1] - (fTemp493 + (2.0 - self.fRec358[0])) * (self.fRec357[1] - fTemp504) / (self.fRec358[0] - fTemp493);
			let mut fTemp505: F32 = 0.760314 * self.fRec357[0] - 0.64955574 * self.fRec354[1];
			let mut fTemp506: F32 = 0.760314 * self.fRec319[0] - 0.64955574 * self.fRec353[1];
			self.fVec71[(self.IOTA0 & 16383) as usize] = 0.70710677 * fTemp506 - 0.70710677 * fTemp505;
			let mut fTemp507: F32 = self.fVec71[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, (fTemp490) as i32)))) & 16383) as usize];
			self.fVec72[0] = fTemp507;
			self.fRec355[0] = self.fVec72[1] - (fTemp491 + (2.0 - self.fRec356[0])) * (self.fRec355[1] - fTemp507) / (self.fRec356[0] - fTemp491);
			self.fRec353[0] = self.fRec355[0];
			self.fRec360[0] = 0.9999 * (self.fRec360[1] + (i32::wrapping_mul(iTemp309, iSlow88)) as F32) + fSlow89;
			let mut fTemp508: F32 = self.fRec360[0] + -1.49999;
			let mut fTemp509: F32 = F32::floor(fTemp508);
			self.fVec73[(self.IOTA0 & 16383) as usize] = 0.70710677 * fTemp506 + 0.70710677 * fTemp505;
			let mut fTemp510: F32 = self.fVec73[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, (fTemp508) as i32)))) & 16383) as usize];
			self.fVec74[0] = fTemp510;
			self.fRec359[0] = self.fVec74[1] - (fTemp509 + (2.0 - self.fRec360[0])) * (self.fRec359[1] - fTemp510) / (self.fRec360[0] - fTemp509);
			self.fRec354[0] = self.fRec359[0];
			let mut fTemp511: F32 = 0.760314 * self.fRec353[1] + 0.64955574 * self.fRec319[0];
			self.fRec364[0] = 0.9999 * (self.fRec364[1] + (i32::wrapping_mul(iTemp309, iSlow90)) as F32) + fSlow91;
			let mut fTemp512: F32 = self.fRec364[0] + -1.49999;
			let mut fTemp513: F32 = F32::floor(fTemp512);
			let mut fTemp514: F32 = 0.760314 * self.fRec354[1] + 0.64955574 * self.fRec357[0];
			let mut fTemp515: F32 = 0.760314 * fTemp514 - 0.64955574 * self.fRec362[1];
			let mut fTemp516: F32 = 0.760314 * fTemp511 - 0.64955574 * self.fRec361[1];
			self.fVec75[(self.IOTA0 & 16383) as usize] = 0.70710677 * fTemp516 - 0.70710677 * fTemp515;
			let mut fTemp517: F32 = self.fVec75[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, (fTemp512) as i32)))) & 16383) as usize];
			self.fVec76[0] = fTemp517;
			self.fRec363[0] = self.fVec76[1] - (fTemp513 + (2.0 - self.fRec364[0])) * (self.fRec363[1] - fTemp517) / (self.fRec364[0] - fTemp513);
			self.fRec361[0] = self.fRec363[0];
			self.fRec366[0] = 0.9999 * (self.fRec366[1] + (i32::wrapping_mul(iTemp309, iSlow92)) as F32) + fSlow93;
			let mut fTemp518: F32 = self.fRec366[0] + -1.49999;
			let mut fTemp519: F32 = F32::floor(fTemp518);
			let mut fTemp520: F32 = self.fRec366[0] - fTemp519;
			self.fVec77[(self.IOTA0 & 16383) as usize] = 0.70710677 * fTemp516 + 0.70710677 * fTemp515;
			let mut iTemp521: i32 = std::cmp::min(8192, std::cmp::max(0, (fTemp518) as i32));
			let mut fTemp522: F32 = self.fVec77[((i32::wrapping_sub(self.IOTA0, iTemp521)) & 16383) as usize];
			self.fVec78[0] = fTemp522;
			let mut fTemp523: F32 = fTemp519 + (2.0 - self.fRec366[0]);
			self.fRec365[0] = self.fVec78[1] - fTemp523 * (self.fRec365[1] - fTemp522) / fTemp520;
			self.fRec362[0] = self.fRec365[0];
			let mut fTemp524: F32 = 0.760314 * self.fRec361[1] + 0.64955574 * fTemp511;
			self.fRec370[0] = 0.9999 * (self.fRec370[1] + (i32::wrapping_mul(iTemp309, iSlow94)) as F32) + fSlow95;
			let mut fTemp525: F32 = self.fRec370[0] + -1.49999;
			let mut fTemp526: F32 = F32::floor(fTemp525);
			let mut fTemp527: F32 = 0.760314 * self.fRec362[1] + 0.64955574 * fTemp514;
			let mut fTemp528: F32 = 0.760314 * fTemp527 - 0.64955574 * self.fRec368[1];
			let mut fTemp529: F32 = 0.760314 * fTemp524 - 0.64955574 * self.fRec367[1];
			self.fVec79[(self.IOTA0 & 16383) as usize] = 0.70710677 * fTemp529 - 0.70710677 * fTemp528;
			let mut fTemp530: F32 = self.fVec79[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, (fTemp525) as i32)))) & 16383) as usize];
			self.fVec80[0] = fTemp530;
			self.fRec369[0] = self.fVec80[1] - (fTemp526 + (2.0 - self.fRec370[0])) * (self.fRec369[1] - fTemp530) / (self.fRec370[0] - fTemp526);
			self.fRec367[0] = self.fRec369[0];
			self.fRec372[0] = 0.9999 * (self.fRec372[1] + (i32::wrapping_mul(iTemp309, iSlow96)) as F32) + fSlow97;
			let mut fTemp531: F32 = self.fRec372[0] + -1.49999;
			let mut fTemp532: F32 = F32::floor(fTemp531);
			let mut fTemp533: F32 = self.fRec372[0] - fTemp532;
			self.fVec81[(self.IOTA0 & 16383) as usize] = 0.70710677 * fTemp529 + 0.70710677 * fTemp528;
			let mut iTemp534: i32 = std::cmp::min(8192, std::cmp::max(0, (fTemp531) as i32));
			let mut fTemp535: F32 = self.fVec81[((i32::wrapping_sub(self.IOTA0, iTemp534)) & 16383) as usize];
			self.fVec82[0] = fTemp535;
			let mut fTemp536: F32 = fTemp532 + (2.0 - self.fRec372[0]);
			self.fRec371[0] = self.fVec82[1] - fTemp536 * (self.fRec371[1] - fTemp535) / fTemp533;
			self.fRec368[0] = self.fRec371[0];
			let mut fTemp537: F32 = 0.760314 * self.fRec367[1] + 0.64955574 * fTemp524;
			self.fRec376[0] = 0.9999 * (self.fRec376[1] + (i32::wrapping_mul(iTemp309, iSlow98)) as F32) + fSlow99;
			let mut fTemp538: F32 = self.fRec376[0] + -1.49999;
			let mut fTemp539: F32 = F32::floor(fTemp538);
			let mut fTemp540: F32 = self.fRec376[0] - fTemp539;
			let mut fTemp541: F32 = 0.760314 * self.fRec368[1] + 0.64955574 * fTemp527;
			let mut fTemp542: F32 = 0.760314 * fTemp541 - 0.64955574 * self.fRec374[1];
			let mut fTemp543: F32 = 0.760314 * fTemp537 - 0.64955574 * self.fRec373[1];
			self.fVec83[(self.IOTA0 & 16383) as usize] = 0.70710677 * fTemp543 - 0.70710677 * fTemp542;
			let mut iTemp544: i32 = std::cmp::min(8192, std::cmp::max(0, (fTemp538) as i32));
			let mut fTemp545: F32 = self.fVec83[((i32::wrapping_sub(self.IOTA0, iTemp544)) & 16383) as usize];
			self.fVec84[0] = fTemp545;
			let mut fTemp546: F32 = fTemp539 + (2.0 - self.fRec376[0]);
			self.fRec375[0] = self.fVec84[1] - fTemp546 * (self.fRec375[1] - fTemp545) / fTemp540;
			self.fRec373[0] = self.fRec375[0];
			self.fRec378[0] = 0.9999 * (self.fRec378[1] + (i32::wrapping_mul(iTemp309, iSlow100)) as F32) + fSlow101;
			let mut fTemp547: F32 = self.fRec378[0] + -1.49999;
			let mut fTemp548: F32 = F32::floor(fTemp547);
			let mut fTemp549: F32 = self.fRec378[0] - fTemp548;
			self.fVec85[(self.IOTA0 & 16383) as usize] = 0.70710677 * fTemp543 + 0.70710677 * fTemp542;
			let mut iTemp550: i32 = std::cmp::min(8192, std::cmp::max(0, (fTemp547) as i32));
			let mut fTemp551: F32 = self.fVec85[((i32::wrapping_sub(self.IOTA0, iTemp550)) & 16383) as usize];
			self.fVec86[0] = fTemp551;
			let mut fTemp552: F32 = fTemp548 + (2.0 - self.fRec378[0]);
			self.fRec377[0] = self.fVec86[1] - fTemp552 * (self.fRec377[1] - fTemp551) / fTemp549;
			self.fRec374[0] = self.fRec377[0];
			let mut fTemp553: F32 = 0.760314 * self.fRec373[1] + 0.64955574 * fTemp537;
			self.fRec382[0] = 0.9999 * (self.fRec382[1] + (i32::wrapping_mul(iTemp309, iSlow102)) as F32) + fSlow103;
			let mut fTemp554: F32 = self.fRec382[0] + -1.49999;
			let mut fTemp555: F32 = F32::floor(fTemp554);
			let mut fTemp556: F32 = 0.760314 * self.fRec374[1] + 0.64955574 * fTemp541;
			let mut fTemp557: F32 = 0.760314 * fTemp556 - 0.64955574 * self.fRec380[1];
			let mut fTemp558: F32 = 0.760314 * fTemp553 - 0.64955574 * self.fRec379[1];
			self.fVec87[(self.IOTA0 & 16383) as usize] = 0.70710677 * fTemp558 - 0.70710677 * fTemp557;
			let mut fTemp559: F32 = self.fVec87[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, (fTemp554) as i32)))) & 16383) as usize];
			self.fVec88[0] = fTemp559;
			self.fRec381[0] = self.fVec88[1] - (fTemp555 + (2.0 - self.fRec382[0])) * (self.fRec381[1] - fTemp559) / (self.fRec382[0] - fTemp555);
			self.fRec379[0] = self.fRec381[0];
			self.fRec384[0] = 0.9999 * (self.fRec384[1] + (i32::wrapping_mul(iTemp309, iSlow104)) as F32) + fSlow105;
			let mut fTemp560: F32 = self.fRec384[0] + -1.49999;
			let mut fTemp561: F32 = F32::floor(fTemp560);
			let mut fTemp562: F32 = self.fRec384[0] - fTemp561;
			self.fVec89[(self.IOTA0 & 16383) as usize] = 0.70710677 * fTemp558 + 0.70710677 * fTemp557;
			let mut iTemp563: i32 = std::cmp::min(8192, std::cmp::max(0, (fTemp560) as i32));
			let mut fTemp564: F32 = self.fVec89[((i32::wrapping_sub(self.IOTA0, iTemp563)) & 16383) as usize];
			self.fVec90[0] = fTemp564;
			let mut fTemp565: F32 = fTemp561 + (2.0 - self.fRec384[0]);
			self.fRec383[0] = self.fVec90[1] - fTemp565 * (self.fRec383[1] - fTemp564) / fTemp562;
			self.fRec380[0] = self.fRec383[0];
			let mut fTemp566: F32 = 0.760314 * self.fRec379[1] + 0.64955574 * fTemp553;
			self.fVec91[(self.IOTA0 & 16383) as usize] = fTemp566;
			let mut fTemp567: F32 = fSlow82 * (self.fRec351[0] + 1.0);
			let mut fTemp568: F32 = fTemp567 + 3.500005;
			let mut iTemp569: i32 = (fTemp568) as i32;
			let mut iTemp570: i32 = std::cmp::max(0, i32::wrapping_add(iTemp569, 4));
			let mut fTemp571: F32 = F32::floor(fTemp568);
			let mut fTemp572: F32 = fTemp567 + (2.0 - fTemp571);
			let mut fTemp573: F32 = fTemp567 + (3.0 - fTemp571);
			let mut fTemp574: F32 = fTemp567 + (4.0 - fTemp571);
			let mut fTemp575: F32 = fTemp574 * fTemp573;
			let mut fTemp576: F32 = fTemp575 * fTemp572;
			let mut fTemp577: F32 = fTemp567 + (1.0 - fTemp571);
			let mut fTemp578: F32 = 0.0 - fTemp577;
			let mut iTemp579: i32 = std::cmp::max(0, i32::wrapping_add(iTemp569, 3));
			let mut fTemp580: F32 = 0.0 - 0.5 * fTemp577;
			let mut fTemp581: F32 = 0.0 - fTemp572;
			let mut iTemp582: i32 = std::cmp::max(0, i32::wrapping_add(iTemp569, 2));
			let mut fTemp583: F32 = 0.0 - 0.33333334 * fTemp577;
			let mut fTemp584: F32 = 0.0 - 0.5 * fTemp572;
			let mut fTemp585: F32 = 0.0 - fTemp573;
			let mut iTemp586: i32 = std::cmp::max(0, i32::wrapping_add(iTemp569, 1));
			let mut fTemp587: F32 = fTemp567 + (5.0 - fTemp571);
			let mut fTemp588: F32 = 0.0 - 0.25 * fTemp577;
			let mut fTemp589: F32 = 0.0 - 0.33333334 * fTemp572;
			let mut fTemp590: F32 = 0.0 - 0.5 * fTemp573;
			let mut fTemp591: F32 = 0.0 - fTemp574;
			let mut iTemp592: i32 = std::cmp::max(0, iTemp569);
			self.fVec92[(self.IOTA0 & 16383) as usize] = self.fVec91[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, iTemp592))) & 16383) as usize] * fTemp591 * fTemp590 * fTemp589 * fTemp588 + fTemp587 * (self.fVec91[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, iTemp586))) & 16383) as usize] * fTemp585 * fTemp584 * fTemp583 + 0.5 * fTemp574 * self.fVec91[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, iTemp582))) & 16383) as usize] * fTemp581 * fTemp580 + 0.16666667 * fTemp575 * self.fVec91[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, iTemp579))) & 16383) as usize] * fTemp578 + 0.041666668 * fTemp576 * self.fVec91[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, iTemp570))) & 16383) as usize]);
			let mut fTemp593: F32 = self.fVec92[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, (fTemp410) as i32)))) & 16383) as usize];
			self.fVec93[0] = fTemp593;
			self.fRec317[0] = self.fVec93[1] - (fTemp411 + (2.0 - self.fRec318[0])) * (self.fRec317[1] - fTemp593) / (self.fRec318[0] - fTemp411);
			self.fRec316[0] = 0.0 - self.fConst40 * (self.fConst38 * self.fRec316[1] - (self.fRec317[0] + self.fRec317[1]));
			self.fRec315[0] = self.fRec316[0] - self.fConst36 * (self.fConst35 * self.fRec315[2] + self.fConst31 * self.fRec315[1]);
			self.fRec314[0] = self.fConst36 * (self.fRec315[2] + self.fRec315[0] + 2.0 * self.fRec315[1]) - self.fConst34 * (self.fConst33 * self.fRec314[2] + self.fConst31 * self.fRec314[1]);
			let mut fTemp594: F32 = self.fRec314[2] + self.fRec314[0] + 2.0 * self.fRec314[1];
			self.fVec94[0] = fTemp594;
			self.fRec313[0] = 0.0 - self.fConst43 * (self.fConst41 * self.fRec313[1] - self.fConst34 * (fTemp594 + self.fVec94[1]));
			self.fRec312[0] = self.fRec313[0] - self.fConst27 * (self.fConst26 * self.fRec312[2] + self.fConst22 * self.fRec312[1]);
			self.fRec311[0] = self.fConst27 * (self.fRec312[2] + self.fRec312[0] + 2.0 * self.fRec312[1]) - self.fConst25 * (self.fConst24 * self.fRec311[2] + self.fConst22 * self.fRec311[1]);
			self.fRec387[0] = self.fConst34 * (self.fConst45 * fTemp594 + self.fConst46 * self.fVec94[1]) - self.fConst44 * self.fRec387[1];
			self.fRec386[0] = self.fRec387[0] - self.fConst27 * (self.fConst26 * self.fRec386[2] + self.fConst22 * self.fRec386[1]);
			self.fRec385[0] = self.fConst27 * (self.fConst21 * self.fRec386[0] + self.fConst47 * self.fRec386[1] + self.fConst21 * self.fRec386[2]) - self.fConst25 * (self.fConst24 * self.fRec385[2] + self.fConst22 * self.fRec385[1]);
			let mut fTemp595: F32 = self.fConst22 * self.fRec388[1];
			self.fRec391[0] = self.fConst50 * self.fRec317[1] - self.fConst40 * (self.fConst38 * self.fRec391[1] - self.fConst32 * self.fRec317[0]);
			self.fRec390[0] = self.fRec391[0] - self.fConst36 * (self.fConst35 * self.fRec390[2] + self.fConst31 * self.fRec390[1]);
			self.fRec389[0] = self.fConst36 * (self.fConst30 * self.fRec390[0] + self.fConst51 * self.fRec390[1] + self.fConst30 * self.fRec390[2]) - self.fConst34 * (self.fConst33 * self.fRec389[2] + self.fConst31 * self.fRec389[1]);
			self.fRec388[0] = self.fConst34 * (self.fConst30 * self.fRec389[0] + self.fConst51 * self.fRec389[1] + self.fConst30 * self.fRec389[2]) - self.fConst49 * (self.fConst48 * self.fRec388[2] + fTemp595);
			let mut fTemp596: F32 = fTemp407 + fTemp408 + fSlow106 * (self.fRec388[2] + self.fConst49 * (fTemp595 + self.fConst48 * self.fRec388[0]) + self.fConst25 * (self.fConst21 * self.fRec385[0] + self.fConst47 * self.fRec385[1] + self.fConst21 * self.fRec385[2] + self.fRec311[2] + self.fRec311[0] + 2.0 * self.fRec311[1]));
			self.fVec95[(self.IOTA0 & 1023) as usize] = fTemp596;
			self.fRec310[0] = fSlow107 * (fTemp591 * fTemp590 * fTemp589 * fTemp588 * self.fVec95[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, iTemp592))) & 1023) as usize] + fTemp587 * (fTemp585 * fTemp584 * fTemp583 * self.fVec95[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, iTemp586))) & 1023) as usize] + 0.5 * fTemp574 * fTemp581 * fTemp580 * self.fVec95[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, iTemp582))) & 1023) as usize] + 0.16666667 * fTemp575 * fTemp578 * self.fVec95[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, iTemp579))) & 1023) as usize] + 0.041666668 * fTemp576 * self.fVec95[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, iTemp570))) & 1023) as usize])) + fSlow53 * self.fRec310[1];
			self.fRec403[0] = 0.995 * (self.fRec403[1] + (i32::wrapping_mul(iTemp309, iSlow110)) as F32) + fSlow111;
			let mut fTemp597: F32 = self.fRec403[0] + -1.49999;
			let mut fTemp598: F32 = F32::floor(fTemp597);
			let mut fTemp599: F32 = 0.760314 * self.fRec380[1] + 0.64955574 * fTemp556;
			self.fVec96[(self.IOTA0 & 16383) as usize] = fTemp599;
			let mut fTemp600: F32 = fSlow87 * self.fRec351[0];
			let mut fTemp601: F32 = fSlow82 + fTemp600 + 3.500005;
			let mut iTemp602: i32 = (fTemp601) as i32;
			let mut fTemp603: F32 = F32::floor(fTemp601);
			let mut fTemp604: F32 = fSlow82 + fTemp600 + (2.0 - fTemp603);
			let mut fTemp605: F32 = fSlow82 + fTemp600 + (3.0 - fTemp603);
			let mut fTemp606: F32 = fSlow82 + fTemp600 + (4.0 - fTemp603);
			let mut fTemp607: F32 = fTemp606 * fTemp605;
			let mut fTemp608: F32 = fSlow82 + fTemp600 + (1.0 - fTemp603);
			self.fVec97[(self.IOTA0 & 16383) as usize] = self.fVec96[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, iTemp602)))) & 16383) as usize] * (0.0 - fTemp606) * (0.0 - 0.5 * fTemp605) * (0.0 - 0.33333334 * fTemp604) * (0.0 - 0.25 * fTemp608) + (fSlow82 + fTemp600 + (5.0 - fTemp603)) * (self.fVec96[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, i32::wrapping_add(iTemp602, 1))))) & 16383) as usize] * (0.0 - fTemp605) * (0.0 - 0.5 * fTemp604) * (0.0 - 0.33333334 * fTemp608) + 0.5 * fTemp606 * self.fVec96[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, i32::wrapping_add(iTemp602, 2))))) & 16383) as usize] * (0.0 - fTemp604) * (0.0 - 0.5 * fTemp608) + 0.16666667 * fTemp607 * self.fVec96[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, i32::wrapping_add(iTemp602, 3))))) & 16383) as usize] * (0.0 - fTemp608) + 0.041666668 * fTemp607 * fTemp604 * self.fVec96[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, i32::wrapping_add(iTemp602, 4))))) & 16383) as usize]);
			let mut fTemp609: F32 = self.fVec97[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, (fTemp597) as i32)))) & 16383) as usize];
			self.fVec98[0] = fTemp609;
			self.fRec402[0] = self.fVec98[1] - (fTemp598 + (2.0 - self.fRec403[0])) * (self.fRec402[1] - fTemp609) / (self.fRec403[0] - fTemp598);
			self.fRec401[0] = 0.0 - self.fConst40 * (self.fConst38 * self.fRec401[1] - (self.fRec402[0] + self.fRec402[1]));
			self.fRec400[0] = self.fRec401[0] - self.fConst36 * (self.fConst35 * self.fRec400[2] + self.fConst31 * self.fRec400[1]);
			self.fRec399[0] = self.fConst36 * (self.fRec400[2] + self.fRec400[0] + 2.0 * self.fRec400[1]) - self.fConst34 * (self.fConst33 * self.fRec399[2] + self.fConst31 * self.fRec399[1]);
			let mut fTemp610: F32 = self.fRec399[2] + self.fRec399[0] + 2.0 * self.fRec399[1];
			self.fVec99[0] = fTemp610;
			self.fRec398[0] = 0.0 - self.fConst43 * (self.fConst41 * self.fRec398[1] - self.fConst34 * (fTemp610 + self.fVec99[1]));
			self.fRec397[0] = self.fRec398[0] - self.fConst27 * (self.fConst26 * self.fRec397[2] + self.fConst22 * self.fRec397[1]);
			self.fRec396[0] = self.fConst27 * (self.fRec397[2] + self.fRec397[0] + 2.0 * self.fRec397[1]) - self.fConst25 * (self.fConst24 * self.fRec396[2] + self.fConst22 * self.fRec396[1]);
			self.fRec406[0] = self.fConst34 * (self.fConst45 * fTemp610 + self.fConst46 * self.fVec99[1]) - self.fConst44 * self.fRec406[1];
			self.fRec405[0] = self.fRec406[0] - self.fConst27 * (self.fConst26 * self.fRec405[2] + self.fConst22 * self.fRec405[1]);
			self.fRec404[0] = self.fConst27 * (self.fConst21 * self.fRec405[0] + self.fConst47 * self.fRec405[1] + self.fConst21 * self.fRec405[2]) - self.fConst25 * (self.fConst24 * self.fRec404[2] + self.fConst22 * self.fRec404[1]);
			let mut fTemp611: F32 = self.fConst22 * self.fRec407[1];
			self.fRec410[0] = self.fConst50 * self.fRec402[1] - self.fConst40 * (self.fConst38 * self.fRec410[1] - self.fConst32 * self.fRec402[0]);
			self.fRec409[0] = self.fRec410[0] - self.fConst36 * (self.fConst35 * self.fRec409[2] + self.fConst31 * self.fRec409[1]);
			self.fRec408[0] = self.fConst36 * (self.fConst30 * self.fRec409[0] + self.fConst51 * self.fRec409[1] + self.fConst30 * self.fRec409[2]) - self.fConst34 * (self.fConst33 * self.fRec408[2] + self.fConst31 * self.fRec408[1]);
			self.fRec407[0] = self.fConst34 * (self.fConst30 * self.fRec408[0] + self.fConst51 * self.fRec408[1] + self.fConst30 * self.fRec408[2]) - self.fConst49 * (self.fConst48 * self.fRec407[2] + fTemp611);
			let mut fTemp612: F32 = fTemp409 + fSlow106 * (self.fRec407[2] + self.fConst49 * (fTemp611 + self.fConst48 * self.fRec407[0]) + self.fConst25 * (self.fConst21 * self.fRec404[0] + self.fConst47 * self.fRec404[1] + self.fConst21 * self.fRec404[2] + self.fRec396[2] + self.fRec396[0] + 2.0 * self.fRec396[1]));
			self.fVec100[(self.IOTA0 & 1023) as usize] = fTemp612;
			self.fRec395[0] = fSlow107 * (fTemp487 * fTemp486 * fTemp485 * fTemp484 * self.fVec100[((i32::wrapping_sub(self.IOTA0, iTemp488)) & 1023) as usize] + fTemp483 * (fTemp481 * fTemp480 * fTemp479 * self.fVec100[((i32::wrapping_sub(self.IOTA0, iTemp482)) & 1023) as usize] + 0.5 * fTemp470 * fTemp477 * fTemp476 * self.fVec100[((i32::wrapping_sub(self.IOTA0, iTemp478)) & 1023) as usize] + 0.16666667 * fTemp471 * fTemp474 * self.fVec100[((i32::wrapping_sub(self.IOTA0, iTemp475)) & 1023) as usize] + 0.041666668 * fTemp472 * self.fVec100[((i32::wrapping_sub(self.IOTA0, iTemp466)) & 1023) as usize])) + fSlow53 * self.fRec395[1];
			let mut fTemp613: F32 = fSlow112 * self.fRec395[0] - fSlow109 * self.fRec393[1];
			let mut fTemp614: F32 = fSlow112 * self.fRec310[0] - fSlow109 * self.fRec392[1];
			self.fVec101[(self.IOTA0 & 16383) as usize] = 0.70710677 * fTemp614 - 0.70710677 * fTemp613;
			let mut fTemp615: F32 = self.fVec101[((i32::wrapping_sub(self.IOTA0, iTemp521)) & 16383) as usize];
			self.fVec102[0] = fTemp615;
			self.fRec394[0] = self.fVec102[1] - fTemp523 * (self.fRec394[1] - fTemp615) / fTemp520;
			self.fRec392[0] = self.fRec394[0];
			self.fRec412[0] = 0.9999 * (self.fRec412[1] + (i32::wrapping_mul(iTemp309, iSlow113)) as F32) + fSlow114;
			let mut fTemp616: F32 = self.fRec412[0] + -1.49999;
			let mut fTemp617: F32 = F32::floor(fTemp616);
			self.fVec103[(self.IOTA0 & 16383) as usize] = 0.70710677 * fTemp614 + 0.70710677 * fTemp613;
			let mut fTemp618: F32 = self.fVec103[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, (fTemp616) as i32)))) & 16383) as usize];
			self.fVec104[0] = fTemp618;
			self.fRec411[0] = self.fVec104[1] - (fTemp617 + (2.0 - self.fRec412[0])) * (self.fRec411[1] - fTemp618) / (self.fRec412[0] - fTemp617);
			self.fRec393[0] = self.fRec411[0];
			let mut fTemp619: F32 = fSlow112 * self.fRec392[1] + fSlow109 * self.fRec310[0];
			let mut fTemp620: F32 = fSlow112 * self.fRec393[1] + fSlow109 * self.fRec395[0];
			let mut fTemp621: F32 = fSlow112 * fTemp620 - fSlow109 * self.fRec414[1];
			let mut fTemp622: F32 = fSlow112 * fTemp619 - fSlow109 * self.fRec413[1];
			self.fVec105[(self.IOTA0 & 16383) as usize] = 0.70710677 * fTemp622 - 0.70710677 * fTemp621;
			let mut fTemp623: F32 = self.fVec105[((i32::wrapping_sub(self.IOTA0, iTemp544)) & 16383) as usize];
			self.fVec106[0] = fTemp623;
			self.fRec415[0] = self.fVec106[1] - fTemp546 * (self.fRec415[1] - fTemp623) / fTemp540;
			self.fRec413[0] = self.fRec415[0];
			self.fVec107[(self.IOTA0 & 16383) as usize] = 0.70710677 * fTemp622 + 0.70710677 * fTemp621;
			let mut fTemp624: F32 = self.fVec107[((i32::wrapping_sub(self.IOTA0, iTemp534)) & 16383) as usize];
			self.fVec108[0] = fTemp624;
			self.fRec416[0] = self.fVec108[1] - fTemp536 * (self.fRec416[1] - fTemp624) / fTemp533;
			self.fRec414[0] = self.fRec416[0];
			let mut fTemp625: F32 = fSlow112 * self.fRec413[1] + fSlow109 * fTemp619;
			let mut fTemp626: F32 = fSlow112 * self.fRec414[1] + fSlow109 * fTemp620;
			let mut fTemp627: F32 = fSlow112 * fTemp626 - fSlow109 * self.fRec418[1];
			let mut fTemp628: F32 = fSlow112 * fTemp625 - fSlow109 * self.fRec417[1];
			self.fVec109[(self.IOTA0 & 16383) as usize] = 0.70710677 * fTemp628 - 0.70710677 * fTemp627;
			let mut fTemp629: F32 = self.fVec109[((i32::wrapping_sub(self.IOTA0, iTemp550)) & 16383) as usize];
			self.fVec110[0] = fTemp629;
			self.fRec419[0] = self.fVec110[1] - fTemp552 * (self.fRec419[1] - fTemp629) / fTemp549;
			self.fRec417[0] = self.fRec419[0];
			self.fRec421[0] = 0.9999 * (self.fRec421[1] + (i32::wrapping_mul(iTemp309, iSlow115)) as F32) + fSlow116;
			let mut fTemp630: F32 = self.fRec421[0] + -1.49999;
			let mut fTemp631: F32 = F32::floor(fTemp630);
			self.fVec111[(self.IOTA0 & 16383) as usize] = 0.70710677 * fTemp628 + 0.70710677 * fTemp627;
			let mut fTemp632: F32 = self.fVec111[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, (fTemp630) as i32)))) & 16383) as usize];
			self.fVec112[0] = fTemp632;
			self.fRec420[0] = self.fVec112[1] - (fTemp631 + (2.0 - self.fRec421[0])) * (self.fRec420[1] - fTemp632) / (self.fRec421[0] - fTemp631);
			self.fRec418[0] = self.fRec420[0];
			let mut fTemp633: F32 = fSlow112 * self.fRec417[1] + fSlow109 * fTemp625;
			self.fRec425[0] = 0.9999 * (self.fRec425[1] + (i32::wrapping_mul(iTemp309, iSlow117)) as F32) + fSlow118;
			let mut fTemp634: F32 = self.fRec425[0] + -1.49999;
			let mut fTemp635: F32 = F32::floor(fTemp634);
			let mut fTemp636: F32 = fSlow112 * self.fRec418[1] + fSlow109 * fTemp626;
			let mut fTemp637: F32 = fSlow112 * fTemp636 - fSlow109 * self.fRec423[1];
			let mut fTemp638: F32 = fSlow112 * fTemp633 - fSlow109 * self.fRec422[1];
			self.fVec113[(self.IOTA0 & 16383) as usize] = 0.70710677 * fTemp638 - 0.70710677 * fTemp637;
			let mut fTemp639: F32 = self.fVec113[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, (fTemp634) as i32)))) & 16383) as usize];
			self.fVec114[0] = fTemp639;
			self.fRec424[0] = self.fVec114[1] - (fTemp635 + (2.0 - self.fRec425[0])) * (self.fRec424[1] - fTemp639) / (self.fRec425[0] - fTemp635);
			self.fRec422[0] = self.fRec424[0];
			self.fVec115[(self.IOTA0 & 16383) as usize] = 0.70710677 * fTemp638 + 0.70710677 * fTemp637;
			let mut fTemp640: F32 = self.fVec115[((i32::wrapping_sub(self.IOTA0, iTemp563)) & 16383) as usize];
			self.fVec116[0] = fTemp640;
			self.fRec426[0] = self.fVec116[1] - fTemp565 * (self.fRec426[1] - fTemp640) / fTemp562;
			self.fRec423[0] = self.fRec426[0];
			self.fRec308[0] = fSlow112 * self.fRec422[1] + fSlow109 * fTemp633;
			self.fRec309[0] = fSlow112 * self.fRec423[1] + fSlow109 * fTemp636;
			let mut fTemp641: F32 = fSlow51 * (self.fRec308[0] + self.fRec309[0]) + fSlow52 * fTemp409;
			let mut fTemp642: F32 = F32::abs(fTemp641);
			let mut fTemp643: F32 = if (fTemp642 > self.fRec1[1]) as i32 != 0 {self.fConst53} else {self.fConst52};
			self.fRec1[0] = fTemp642 * (1.0 - fTemp643) + self.fRec1[1] * fTemp643;
			self.fRec0[0] = self.fConst54 * (0.0 - 0.9166667 * F32::max(2e+01 * F32::log10(F32::max(1.1754944e-38, self.fRec1[0])) + 4.0, 0.0)) + self.fConst1 * self.fRec0[1];
			self.fRec427[0] = fSlow119 + self.fConst3 * self.fRec427[1];
			let mut fTemp644: F32 = self.fRec427[0] * fTemp641 * F32::powf(1e+01, 0.05 * self.fRec0[0]);
			*output0 = fTemp644;
			*output1 = fTemp644;
			self.iVec0[1] = self.iVec0[0];
			self.fRec2[1] = self.fRec2[0];
			self.fRec5[1] = self.fRec5[0];
			self.fRec3[1] = self.fRec3[0];
			self.IOTA0 = i32::wrapping_add(self.IOTA0, 1);
			self.fRec41[1] = self.fRec41[0];
			self.fRec40[1] = self.fRec40[0];
			self.fRec36[1] = self.fRec36[0];
			self.fRec42[1] = self.fRec42[0];
			self.fRec45[1] = self.fRec45[0];
			self.iVec1[1] = self.iVec1[0];
			self.iRec46[1] = self.iRec46[0];
			self.fRec44[1] = self.fRec44[0];
			for j0 in (1..=3).rev() {
				self.fRec47[j0 as usize] = self.fRec47[(i32::wrapping_sub(j0, 1)) as usize];
			}
			self.fVec2[1] = self.fVec2[0];
			self.fVec3[1] = self.fVec3[0];
			self.iRec49[1] = self.iRec49[0];
			self.iRec51[1] = self.iRec51[0];
			self.fRec50[2] = self.fRec50[1];
			self.fRec50[1] = self.fRec50[0];
			self.fVec4[2] = self.fVec4[1];
			self.fVec4[1] = self.fVec4[0];
			self.fRec32[1] = self.fRec32[0];
			self.fRec28[1] = self.fRec28[0];
			self.fRec26[1] = self.fRec26[0];
			for j1 in (1..=3).rev() {
				self.fRec22[j1 as usize] = self.fRec22[(i32::wrapping_sub(j1, 1)) as usize];
			}
			self.fRec17[1] = self.fRec17[0];
			self.fRec11[1] = self.fRec11[0];
			self.fRec10[1] = self.fRec10[0];
			self.fRec8[1] = self.fRec8[0];
			self.fRec7[2] = self.fRec7[1];
			self.fRec7[1] = self.fRec7[0];
			self.fRec6[2] = self.fRec6[1];
			self.fRec6[1] = self.fRec6[0];
			self.fRec52[1] = self.fRec52[0];
			self.fRec87[1] = self.fRec87[0];
			self.fRec83[1] = self.fRec83[0];
			self.fRec88[1] = self.fRec88[0];
			self.fRec91[1] = self.fRec91[0];
			self.iVec5[1] = self.iVec5[0];
			self.iRec92[1] = self.iRec92[0];
			self.fRec90[1] = self.fRec90[0];
			for j2 in (1..=3).rev() {
				self.fRec93[j2 as usize] = self.fRec93[(i32::wrapping_sub(j2, 1)) as usize];
			}
			self.fVec6[1] = self.fVec6[0];
			self.fVec7[1] = self.fVec7[0];
			self.iRec95[1] = self.iRec95[0];
			self.fRec96[2] = self.fRec96[1];
			self.fRec96[1] = self.fRec96[0];
			self.fVec8[2] = self.fVec8[1];
			self.fVec8[1] = self.fVec8[0];
			self.fRec79[1] = self.fRec79[0];
			self.fRec75[1] = self.fRec75[0];
			self.fRec73[1] = self.fRec73[0];
			for j3 in (1..=3).rev() {
				self.fRec69[j3 as usize] = self.fRec69[(i32::wrapping_sub(j3, 1)) as usize];
			}
			self.fRec64[1] = self.fRec64[0];
			self.fRec58[1] = self.fRec58[0];
			self.fRec57[1] = self.fRec57[0];
			self.fRec55[1] = self.fRec55[0];
			self.fRec54[2] = self.fRec54[1];
			self.fRec54[1] = self.fRec54[0];
			self.fRec53[2] = self.fRec53[1];
			self.fRec53[1] = self.fRec53[0];
			self.fRec97[1] = self.fRec97[0];
			self.fRec132[1] = self.fRec132[0];
			self.fRec128[1] = self.fRec128[0];
			self.fRec133[1] = self.fRec133[0];
			self.fRec136[1] = self.fRec136[0];
			self.iVec9[1] = self.iVec9[0];
			self.iRec137[1] = self.iRec137[0];
			self.fRec135[1] = self.fRec135[0];
			for j4 in (1..=3).rev() {
				self.fRec138[j4 as usize] = self.fRec138[(i32::wrapping_sub(j4, 1)) as usize];
			}
			self.fVec10[1] = self.fVec10[0];
			self.fVec11[1] = self.fVec11[0];
			self.iRec140[1] = self.iRec140[0];
			self.fRec141[2] = self.fRec141[1];
			self.fRec141[1] = self.fRec141[0];
			self.fVec12[2] = self.fVec12[1];
			self.fVec12[1] = self.fVec12[0];
			self.fRec124[1] = self.fRec124[0];
			self.fRec120[1] = self.fRec120[0];
			self.fRec118[1] = self.fRec118[0];
			for j5 in (1..=3).rev() {
				self.fRec114[j5 as usize] = self.fRec114[(i32::wrapping_sub(j5, 1)) as usize];
			}
			self.fRec109[1] = self.fRec109[0];
			self.fRec103[1] = self.fRec103[0];
			self.fRec102[1] = self.fRec102[0];
			self.fRec100[1] = self.fRec100[0];
			self.fRec99[2] = self.fRec99[1];
			self.fRec99[1] = self.fRec99[0];
			self.fRec98[2] = self.fRec98[1];
			self.fRec98[1] = self.fRec98[0];
			self.fRec142[1] = self.fRec142[0];
			self.fRec177[1] = self.fRec177[0];
			self.fRec173[1] = self.fRec173[0];
			self.fRec178[1] = self.fRec178[0];
			self.fRec181[1] = self.fRec181[0];
			self.iVec13[1] = self.iVec13[0];
			self.iRec182[1] = self.iRec182[0];
			self.fRec180[1] = self.fRec180[0];
			for j6 in (1..=3).rev() {
				self.fRec183[j6 as usize] = self.fRec183[(i32::wrapping_sub(j6, 1)) as usize];
			}
			self.fVec14[1] = self.fVec14[0];
			self.fVec15[1] = self.fVec15[0];
			self.iRec185[1] = self.iRec185[0];
			self.fRec186[2] = self.fRec186[1];
			self.fRec186[1] = self.fRec186[0];
			self.fVec16[2] = self.fVec16[1];
			self.fVec16[1] = self.fVec16[0];
			self.fRec169[1] = self.fRec169[0];
			self.fRec165[1] = self.fRec165[0];
			self.fRec163[1] = self.fRec163[0];
			for j7 in (1..=3).rev() {
				self.fRec159[j7 as usize] = self.fRec159[(i32::wrapping_sub(j7, 1)) as usize];
			}
			self.fRec154[1] = self.fRec154[0];
			self.fRec148[1] = self.fRec148[0];
			self.fRec147[1] = self.fRec147[0];
			self.fRec145[1] = self.fRec145[0];
			self.fRec144[2] = self.fRec144[1];
			self.fRec144[1] = self.fRec144[0];
			self.fRec143[2] = self.fRec143[1];
			self.fRec143[1] = self.fRec143[0];
			self.fRec187[1] = self.fRec187[0];
			self.fRec222[1] = self.fRec222[0];
			self.fRec218[1] = self.fRec218[0];
			self.fRec223[1] = self.fRec223[0];
			self.fRec226[1] = self.fRec226[0];
			self.iVec17[1] = self.iVec17[0];
			self.iRec227[1] = self.iRec227[0];
			self.fRec225[1] = self.fRec225[0];
			for j8 in (1..=3).rev() {
				self.fRec228[j8 as usize] = self.fRec228[(i32::wrapping_sub(j8, 1)) as usize];
			}
			self.fVec18[1] = self.fVec18[0];
			self.fVec19[1] = self.fVec19[0];
			self.iRec230[1] = self.iRec230[0];
			self.fRec231[2] = self.fRec231[1];
			self.fRec231[1] = self.fRec231[0];
			self.fVec20[2] = self.fVec20[1];
			self.fVec20[1] = self.fVec20[0];
			self.fRec214[1] = self.fRec214[0];
			self.fRec210[1] = self.fRec210[0];
			self.fRec208[1] = self.fRec208[0];
			for j9 in (1..=3).rev() {
				self.fRec204[j9 as usize] = self.fRec204[(i32::wrapping_sub(j9, 1)) as usize];
			}
			self.fRec199[1] = self.fRec199[0];
			self.fRec193[1] = self.fRec193[0];
			self.fRec192[1] = self.fRec192[0];
			self.fRec190[1] = self.fRec190[0];
			self.fRec189[2] = self.fRec189[1];
			self.fRec189[1] = self.fRec189[0];
			self.fRec188[2] = self.fRec188[1];
			self.fRec188[1] = self.fRec188[0];
			self.fRec232[1] = self.fRec232[0];
			self.fRec233[1] = self.fRec233[0];
			self.fRec238[1] = self.fRec238[0];
			self.fRec236[1] = self.fRec236[0];
			self.fRec239[1] = self.fRec239[0];
			self.fRec235[2] = self.fRec235[1];
			self.fRec235[1] = self.fRec235[0];
			self.fRec234[2] = self.fRec234[1];
			self.fRec234[1] = self.fRec234[0];
			self.fRec240[1] = self.fRec240[0];
			self.fRec245[1] = self.fRec245[0];
			self.fRec243[1] = self.fRec243[0];
			self.fRec246[1] = self.fRec246[0];
			self.fRec242[2] = self.fRec242[1];
			self.fRec242[1] = self.fRec242[0];
			self.fRec241[2] = self.fRec241[1];
			self.fRec241[1] = self.fRec241[0];
			self.fRec247[1] = self.fRec247[0];
			self.fRec252[1] = self.fRec252[0];
			self.fRec250[1] = self.fRec250[0];
			self.fRec253[1] = self.fRec253[0];
			self.fRec249[2] = self.fRec249[1];
			self.fRec249[1] = self.fRec249[0];
			self.fRec248[2] = self.fRec248[1];
			self.fRec248[1] = self.fRec248[0];
			self.fRec254[1] = self.fRec254[0];
			self.fRec259[1] = self.fRec259[0];
			self.fRec257[1] = self.fRec257[0];
			self.fRec260[1] = self.fRec260[0];
			self.fRec256[2] = self.fRec256[1];
			self.fRec256[1] = self.fRec256[0];
			self.fRec255[2] = self.fRec255[1];
			self.fRec255[1] = self.fRec255[0];
			self.fRec261[1] = self.fRec261[0];
			self.fRec262[1] = self.fRec262[0];
			self.fRec263[1] = self.fRec263[0];
			self.fRec265[1] = self.fRec265[0];
			self.fRec266[1] = self.fRec266[0];
			self.fVec21[1] = self.fVec21[0];
			self.fRec264[1] = self.fRec264[0];
			self.fRec268[1] = self.fRec268[0];
			self.fRec269[1] = self.fRec269[0];
			self.fVec23[1] = self.fVec23[0];
			self.fRec267[1] = self.fRec267[0];
			self.fRec271[1] = self.fRec271[0];
			self.fRec272[1] = self.fRec272[0];
			self.fVec25[1] = self.fVec25[0];
			self.fRec270[1] = self.fRec270[0];
			self.fRec273[1] = self.fRec273[0];
			self.fRec275[1] = self.fRec275[0];
			self.fRec276[1] = self.fRec276[0];
			self.fVec27[1] = self.fVec27[0];
			self.fRec274[1] = self.fRec274[0];
			self.fRec278[1] = self.fRec278[0];
			self.fRec279[1] = self.fRec279[0];
			self.fVec29[1] = self.fVec29[0];
			self.fRec277[1] = self.fRec277[0];
			self.fRec281[1] = self.fRec281[0];
			self.fRec282[1] = self.fRec282[0];
			self.fVec31[1] = self.fVec31[0];
			self.fRec280[1] = self.fRec280[0];
			self.fRec283[1] = self.fRec283[0];
			self.fRec285[1] = self.fRec285[0];
			self.fRec286[1] = self.fRec286[0];
			self.fVec33[1] = self.fVec33[0];
			self.fRec284[1] = self.fRec284[0];
			self.fRec288[1] = self.fRec288[0];
			self.fRec289[1] = self.fRec289[0];
			self.fVec35[1] = self.fVec35[0];
			self.fRec287[1] = self.fRec287[0];
			self.fRec291[1] = self.fRec291[0];
			self.fRec292[1] = self.fRec292[0];
			self.fVec37[1] = self.fVec37[0];
			self.fRec290[1] = self.fRec290[0];
			self.fRec293[1] = self.fRec293[0];
			self.fRec295[1] = self.fRec295[0];
			self.fRec296[1] = self.fRec296[0];
			self.fVec39[1] = self.fVec39[0];
			self.fRec294[1] = self.fRec294[0];
			self.fRec298[1] = self.fRec298[0];
			self.fRec299[1] = self.fRec299[0];
			self.fVec41[1] = self.fVec41[0];
			self.fRec297[1] = self.fRec297[0];
			self.fRec301[1] = self.fRec301[0];
			self.fRec302[1] = self.fRec302[0];
			self.fVec43[1] = self.fVec43[0];
			self.fRec300[1] = self.fRec300[0];
			self.fRec303[1] = self.fRec303[0];
			self.fRec304[1] = self.fRec304[0];
			self.fRec305[1] = self.fRec305[0];
			self.fRec307[1] = self.fRec307[0];
			self.fRec318[1] = self.fRec318[0];
			self.fRec320[1] = self.fRec320[0];
			self.fRec324[1] = self.fRec324[0];
			self.fVec46[1] = self.fVec46[0];
			self.fRec323[1] = self.fRec323[0];
			self.fRec321[1] = self.fRec321[0];
			self.fRec326[1] = self.fRec326[0];
			self.fVec48[1] = self.fVec48[0];
			self.fRec325[1] = self.fRec325[0];
			self.fRec322[1] = self.fRec322[0];
			self.fRec330[1] = self.fRec330[0];
			self.fVec50[1] = self.fVec50[0];
			self.fRec329[1] = self.fRec329[0];
			self.fRec327[1] = self.fRec327[0];
			self.fRec332[1] = self.fRec332[0];
			self.fVec52[1] = self.fVec52[0];
			self.fRec331[1] = self.fRec331[0];
			self.fRec328[1] = self.fRec328[0];
			self.fRec336[1] = self.fRec336[0];
			self.fVec54[1] = self.fVec54[0];
			self.fRec335[1] = self.fRec335[0];
			self.fRec333[1] = self.fRec333[0];
			self.fRec338[1] = self.fRec338[0];
			self.fVec56[1] = self.fVec56[0];
			self.fRec337[1] = self.fRec337[0];
			self.fRec334[1] = self.fRec334[0];
			self.fRec342[1] = self.fRec342[0];
			self.fVec58[1] = self.fVec58[0];
			self.fRec341[1] = self.fRec341[0];
			self.fRec339[1] = self.fRec339[0];
			self.fRec344[1] = self.fRec344[0];
			self.fVec60[1] = self.fVec60[0];
			self.fRec343[1] = self.fRec343[0];
			self.fRec340[1] = self.fRec340[0];
			self.fRec348[1] = self.fRec348[0];
			self.fVec62[1] = self.fVec62[0];
			self.fRec347[1] = self.fRec347[0];
			self.fRec345[1] = self.fRec345[0];
			self.fRec350[1] = self.fRec350[0];
			self.fVec64[1] = self.fVec64[0];
			self.fRec349[1] = self.fRec349[0];
			self.fRec346[1] = self.fRec346[0];
			self.fRec351[1] = self.fRec351[0];
			self.fRec352[1] = self.fRec352[0];
			self.fVec67[1] = self.fVec67[0];
			self.fRec319[1] = self.fRec319[0];
			self.fRec356[1] = self.fRec356[0];
			self.fRec358[1] = self.fRec358[0];
			self.fVec70[1] = self.fVec70[0];
			self.fRec357[1] = self.fRec357[0];
			self.fVec72[1] = self.fVec72[0];
			self.fRec355[1] = self.fRec355[0];
			self.fRec353[1] = self.fRec353[0];
			self.fRec360[1] = self.fRec360[0];
			self.fVec74[1] = self.fVec74[0];
			self.fRec359[1] = self.fRec359[0];
			self.fRec354[1] = self.fRec354[0];
			self.fRec364[1] = self.fRec364[0];
			self.fVec76[1] = self.fVec76[0];
			self.fRec363[1] = self.fRec363[0];
			self.fRec361[1] = self.fRec361[0];
			self.fRec366[1] = self.fRec366[0];
			self.fVec78[1] = self.fVec78[0];
			self.fRec365[1] = self.fRec365[0];
			self.fRec362[1] = self.fRec362[0];
			self.fRec370[1] = self.fRec370[0];
			self.fVec80[1] = self.fVec80[0];
			self.fRec369[1] = self.fRec369[0];
			self.fRec367[1] = self.fRec367[0];
			self.fRec372[1] = self.fRec372[0];
			self.fVec82[1] = self.fVec82[0];
			self.fRec371[1] = self.fRec371[0];
			self.fRec368[1] = self.fRec368[0];
			self.fRec376[1] = self.fRec376[0];
			self.fVec84[1] = self.fVec84[0];
			self.fRec375[1] = self.fRec375[0];
			self.fRec373[1] = self.fRec373[0];
			self.fRec378[1] = self.fRec378[0];
			self.fVec86[1] = self.fVec86[0];
			self.fRec377[1] = self.fRec377[0];
			self.fRec374[1] = self.fRec374[0];
			self.fRec382[1] = self.fRec382[0];
			self.fVec88[1] = self.fVec88[0];
			self.fRec381[1] = self.fRec381[0];
			self.fRec379[1] = self.fRec379[0];
			self.fRec384[1] = self.fRec384[0];
			self.fVec90[1] = self.fVec90[0];
			self.fRec383[1] = self.fRec383[0];
			self.fRec380[1] = self.fRec380[0];
			self.fVec93[1] = self.fVec93[0];
			self.fRec317[1] = self.fRec317[0];
			self.fRec316[1] = self.fRec316[0];
			self.fRec315[2] = self.fRec315[1];
			self.fRec315[1] = self.fRec315[0];
			self.fRec314[2] = self.fRec314[1];
			self.fRec314[1] = self.fRec314[0];
			self.fVec94[1] = self.fVec94[0];
			self.fRec313[1] = self.fRec313[0];
			self.fRec312[2] = self.fRec312[1];
			self.fRec312[1] = self.fRec312[0];
			self.fRec311[2] = self.fRec311[1];
			self.fRec311[1] = self.fRec311[0];
			self.fRec387[1] = self.fRec387[0];
			self.fRec386[2] = self.fRec386[1];
			self.fRec386[1] = self.fRec386[0];
			self.fRec385[2] = self.fRec385[1];
			self.fRec385[1] = self.fRec385[0];
			self.fRec391[1] = self.fRec391[0];
			self.fRec390[2] = self.fRec390[1];
			self.fRec390[1] = self.fRec390[0];
			self.fRec389[2] = self.fRec389[1];
			self.fRec389[1] = self.fRec389[0];
			self.fRec388[2] = self.fRec388[1];
			self.fRec388[1] = self.fRec388[0];
			self.fRec310[1] = self.fRec310[0];
			self.fRec403[1] = self.fRec403[0];
			self.fVec98[1] = self.fVec98[0];
			self.fRec402[1] = self.fRec402[0];
			self.fRec401[1] = self.fRec401[0];
			self.fRec400[2] = self.fRec400[1];
			self.fRec400[1] = self.fRec400[0];
			self.fRec399[2] = self.fRec399[1];
			self.fRec399[1] = self.fRec399[0];
			self.fVec99[1] = self.fVec99[0];
			self.fRec398[1] = self.fRec398[0];
			self.fRec397[2] = self.fRec397[1];
			self.fRec397[1] = self.fRec397[0];
			self.fRec396[2] = self.fRec396[1];
			self.fRec396[1] = self.fRec396[0];
			self.fRec406[1] = self.fRec406[0];
			self.fRec405[2] = self.fRec405[1];
			self.fRec405[1] = self.fRec405[0];
			self.fRec404[2] = self.fRec404[1];
			self.fRec404[1] = self.fRec404[0];
			self.fRec410[1] = self.fRec410[0];
			self.fRec409[2] = self.fRec409[1];
			self.fRec409[1] = self.fRec409[0];
			self.fRec408[2] = self.fRec408[1];
			self.fRec408[1] = self.fRec408[0];
			self.fRec407[2] = self.fRec407[1];
			self.fRec407[1] = self.fRec407[0];
			self.fRec395[1] = self.fRec395[0];
			self.fVec102[1] = self.fVec102[0];
			self.fRec394[1] = self.fRec394[0];
			self.fRec392[1] = self.fRec392[0];
			self.fRec412[1] = self.fRec412[0];
			self.fVec104[1] = self.fVec104[0];
			self.fRec411[1] = self.fRec411[0];
			self.fRec393[1] = self.fRec393[0];
			self.fVec106[1] = self.fVec106[0];
			self.fRec415[1] = self.fRec415[0];
			self.fRec413[1] = self.fRec413[0];
			self.fVec108[1] = self.fVec108[0];
			self.fRec416[1] = self.fRec416[0];
			self.fRec414[1] = self.fRec414[0];
			self.fVec110[1] = self.fVec110[0];
			self.fRec419[1] = self.fRec419[0];
			self.fRec417[1] = self.fRec417[0];
			self.fRec421[1] = self.fRec421[0];
			self.fVec112[1] = self.fVec112[0];
			self.fRec420[1] = self.fRec420[0];
			self.fRec418[1] = self.fRec418[0];
			self.fRec425[1] = self.fRec425[0];
			self.fVec114[1] = self.fVec114[0];
			self.fRec424[1] = self.fRec424[0];
			self.fRec422[1] = self.fRec422[0];
			self.fVec116[1] = self.fVec116[0];
			self.fRec426[1] = self.fRec426[0];
			self.fRec423[1] = self.fRec423[0];
			self.fRec308[1] = self.fRec308[0];
			self.fRec309[1] = self.fRec309[0];
			self.fRec1[1] = self.fRec1[0];
			self.fRec0[1] = self.fRec0[0];
			self.fRec427[1] = self.fRec427[0];
		}
	}

}


}
pub use dsp::mydsp as Instrument;
