mod dsp {
    /* ------------------------------------------------------------
author: "Pierre Lulé"
license: "BSD"
name: "theremotion"
version: "1.0"
Code generated with Faust 2.68.1 (https://faust.grame.fr)
Compilation options: -a C:\Users\Pierre\AppData\Local\Temp\.tmpq2mSuW -lang rust -ct 1 -es 1 -mcd 16 -single -ftz 0
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
	iRec2: [i32;2],
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
			self.iRec2[l2 as usize] = 0;
		}
	}
	
	fn fillmydspSIG0(&mut self, count: i32, table: &mut[F32]) {
		for i1 in 0..count {
			self.iRec2[0] = i32::wrapping_add(self.iRec2[1], 1);
			table[i1 as usize] = 4.4e+02 * F32::powf(2.0, 0.083333336 * (0.062042013 * (i32::wrapping_add(self.iRec2[0], -1)) as F32 + -69.0));
			self.iRec2[1] = self.iRec2[0];
		}
	}

}


pub fn newmydspSIG0() -> mydspSIG0 { 
	mydspSIG0 {
		iRec2: [0;2],
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
	fHslider0: F32,
	fRec0: [F32;2],
	fHslider1: F32,
	fRec3: [F32;2],
	fHslider2: F32,
	fRec1: [F32;2],
	fConst3: F32,
	IOTA0: i32,
	fConst4: F32,
	fHslider3: F32,
	fRec39: [F32;2],
	fRec38: [F32;2],
	fConst5: F32,
	fRec34: [F32;2],
	fRec40: [F32;2],
	fConst6: F32,
	fConst7: F32,
	fRec43: [F32;2],
	iVec1: [i32;2],
	iConst8: i32,
	iRec44: [i32;2],
	fConst9: F32,
	fRec42: [F32;2],
	fRec45: [F32;4],
	fRec46: [F32;2048],
	fVec2: [F32;2],
	fConst10: F32,
	fConst11: F32,
	fButton0: F32,
	fVec3: [F32;2],
	iRec47: [i32;2],
	iRec49: [i32;2],
	fRec48: [F32;3],
	fVec4: [F32;3],
	fRec41: [F32;2048],
	fRec30: [F32;2],
	fRec26: [F32;2],
	fRec22: [F32;2048],
	fRec24: [F32;2],
	fHslider4: F32,
	fRec20: [F32;4],
	fRec15: [F32;2],
	fRec11: [F32;2048],
	fRec9: [F32;2],
	fConst12: F32,
	fRec8: [F32;2],
	fRec6: [F32;2],
	fRec5: [F32;3],
	fRec4: [F32;3],
	fHslider5: F32,
	fRec50: [F32;2],
	fRec85: [F32;2],
	fRec81: [F32;2],
	fRec86: [F32;2],
	fRec89: [F32;2],
	iVec5: [i32;2],
	iRec90: [i32;2],
	fRec88: [F32;2],
	fRec91: [F32;4],
	fRec92: [F32;2048],
	fVec6: [F32;2],
	fButton1: F32,
	fVec7: [F32;2],
	iRec93: [i32;2],
	fRec94: [F32;3],
	fVec8: [F32;3],
	fRec87: [F32;2048],
	fRec77: [F32;2],
	fRec73: [F32;2],
	fRec69: [F32;2048],
	fRec71: [F32;2],
	fRec67: [F32;4],
	fRec62: [F32;2],
	fRec58: [F32;2048],
	fRec56: [F32;2],
	fRec55: [F32;2],
	fRec53: [F32;2],
	fRec52: [F32;3],
	fRec51: [F32;3],
	fHslider6: F32,
	fRec95: [F32;2],
	fRec130: [F32;2],
	fRec126: [F32;2],
	fRec131: [F32;2],
	fRec134: [F32;2],
	iVec9: [i32;2],
	iRec135: [i32;2],
	fRec133: [F32;2],
	fRec136: [F32;4],
	fRec137: [F32;2048],
	fVec10: [F32;2],
	fButton2: F32,
	fVec11: [F32;2],
	iRec138: [i32;2],
	fRec139: [F32;3],
	fVec12: [F32;3],
	fRec132: [F32;2048],
	fRec122: [F32;2],
	fRec118: [F32;2],
	fRec114: [F32;2048],
	fRec116: [F32;2],
	fRec112: [F32;4],
	fRec107: [F32;2],
	fRec103: [F32;2048],
	fRec101: [F32;2],
	fRec100: [F32;2],
	fRec98: [F32;2],
	fRec97: [F32;3],
	fRec96: [F32;3],
	fHslider7: F32,
	fRec140: [F32;2],
	fRec175: [F32;2],
	fRec171: [F32;2],
	fRec176: [F32;2],
	fRec179: [F32;2],
	iVec13: [i32;2],
	iRec180: [i32;2],
	fRec178: [F32;2],
	fRec181: [F32;4],
	fRec182: [F32;2048],
	fVec14: [F32;2],
	fButton3: F32,
	fVec15: [F32;2],
	iRec183: [i32;2],
	fRec184: [F32;3],
	fVec16: [F32;3],
	fRec177: [F32;2048],
	fRec167: [F32;2],
	fRec163: [F32;2],
	fRec159: [F32;2048],
	fRec161: [F32;2],
	fRec157: [F32;4],
	fRec152: [F32;2],
	fRec148: [F32;2048],
	fRec146: [F32;2],
	fRec145: [F32;2],
	fRec143: [F32;2],
	fRec142: [F32;3],
	fRec141: [F32;3],
	fHslider8: F32,
	fRec185: [F32;2],
	fRec220: [F32;2],
	fRec216: [F32;2],
	fRec221: [F32;2],
	fRec224: [F32;2],
	iVec17: [i32;2],
	iRec225: [i32;2],
	fRec223: [F32;2],
	fRec226: [F32;4],
	fRec227: [F32;2048],
	fVec18: [F32;2],
	fButton4: F32,
	fVec19: [F32;2],
	iRec228: [i32;2],
	fRec229: [F32;3],
	fVec20: [F32;3],
	fRec222: [F32;2048],
	fRec212: [F32;2],
	fRec208: [F32;2],
	fRec204: [F32;2048],
	fRec206: [F32;2],
	fRec202: [F32;4],
	fRec197: [F32;2],
	fRec193: [F32;2048],
	fRec191: [F32;2],
	fRec190: [F32;2],
	fRec188: [F32;2],
	fRec187: [F32;3],
	fRec186: [F32;3],
	fHslider9: F32,
	fRec230: [F32;2],
	fHslider10: F32,
	fRec231: [F32;2],
	fRec236: [F32;2],
	fConst13: F32,
	fRec234: [F32;2],
	fHslider11: F32,
	fRec237: [F32;2],
	fRec233: [F32;3],
	fRec232: [F32;3],
	fHslider12: F32,
	fRec238: [F32;2],
	fRec243: [F32;2],
	fRec241: [F32;2],
	fHslider13: F32,
	fRec244: [F32;2],
	fRec240: [F32;3],
	fRec239: [F32;3],
	fHslider14: F32,
	fRec245: [F32;2],
	fRec250: [F32;2],
	fRec248: [F32;2],
	fHslider15: F32,
	fRec251: [F32;2],
	fRec247: [F32;3],
	fRec246: [F32;3],
	fHslider16: F32,
	fRec252: [F32;2],
	fRec257: [F32;2],
	fRec255: [F32;2],
	fHslider17: F32,
	fRec258: [F32;2],
	fRec254: [F32;3],
	fRec253: [F32;3],
	fHslider18: F32,
	fRec259: [F32;2],
	fHslider19: F32,
	fRec260: [F32;2],
	fHslider20: F32,
	fRec261: [F32;2],
	fHslider21: F32,
	fHslider22: F32,
	fRec263: [F32;2],
	fConst14: F32,
	fRec264: [F32;2],
	fVec21: [F32;2],
	fVec22: [F32;4096],
	fConst15: F32,
	fRec262: [F32;2],
	fRec266: [F32;2],
	fRec267: [F32;2],
	fVec23: [F32;2],
	fVec24: [F32;4096],
	fRec265: [F32;2],
	fRec269: [F32;2],
	fRec270: [F32;2],
	fVec25: [F32;2],
	fVec26: [F32;4096],
	fRec268: [F32;2],
	fHslider23: F32,
	fRec271: [F32;2],
	fHslider24: F32,
	fRec273: [F32;2],
	fRec274: [F32;2],
	fVec27: [F32;2],
	fVec28: [F32;4096],
	fRec272: [F32;2],
	fRec276: [F32;2],
	fRec277: [F32;2],
	fVec29: [F32;2],
	fVec30: [F32;4096],
	fRec275: [F32;2],
	fRec279: [F32;2],
	fRec280: [F32;2],
	fVec31: [F32;2],
	fVec32: [F32;4096],
	fRec278: [F32;2],
	fHslider25: F32,
	fRec281: [F32;2],
	fHslider26: F32,
	fRec283: [F32;2],
	fRec284: [F32;2],
	fVec33: [F32;2],
	fVec34: [F32;4096],
	fRec282: [F32;2],
	fRec286: [F32;2],
	fRec287: [F32;2],
	fVec35: [F32;2],
	fVec36: [F32;4096],
	fRec285: [F32;2],
	fRec289: [F32;2],
	fRec290: [F32;2],
	fVec37: [F32;2],
	fVec38: [F32;4096],
	fRec288: [F32;2],
	fHslider27: F32,
	fRec291: [F32;2],
	fHslider28: F32,
	fRec293: [F32;2],
	fRec294: [F32;2],
	fVec39: [F32;2],
	fVec40: [F32;4096],
	fRec292: [F32;2],
	fRec296: [F32;2],
	fRec297: [F32;2],
	fVec41: [F32;2],
	fVec42: [F32;4096],
	fRec295: [F32;2],
	fRec299: [F32;2],
	fRec300: [F32;2],
	fVec43: [F32;2],
	fVec44: [F32;4096],
	fRec298: [F32;2],
	fHslider29: F32,
	fRec301: [F32;2],
	fConst16: F32,
	fHslider30: F32,
	fRec302: [F32;2],
	fHslider31: F32,
	fRec303: [F32;2],
	fConst17: F32,
	fHslider32: F32,
	fRec305: [F32;2],
	fHslider33: F32,
	fRec304: [F32;2097152],
	fHslider34: F32,
	fHslider35: F32,
	fConst20: F32,
	fConst21: F32,
	fConst23: F32,
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
	fRec316: [F32;2],
	fRec318: [F32;2],
	fRec322: [F32;2],
	fVec45: [F32;16384],
	fVec46: [F32;2],
	fRec321: [F32;2],
	fRec319: [F32;2],
	fRec324: [F32;2],
	fVec47: [F32;16384],
	fVec48: [F32;2],
	fRec323: [F32;2],
	fRec320: [F32;2],
	fRec328: [F32;2],
	fVec49: [F32;16384],
	fVec50: [F32;2],
	fRec327: [F32;2],
	fRec325: [F32;2],
	fRec330: [F32;2],
	fVec51: [F32;16384],
	fVec52: [F32;2],
	fRec329: [F32;2],
	fRec326: [F32;2],
	fRec334: [F32;2],
	fVec53: [F32;16384],
	fVec54: [F32;2],
	fRec333: [F32;2],
	fRec331: [F32;2],
	fRec336: [F32;2],
	fVec55: [F32;16384],
	fVec56: [F32;2],
	fRec335: [F32;2],
	fRec332: [F32;2],
	fRec340: [F32;2],
	fVec57: [F32;16384],
	fVec58: [F32;2],
	fRec339: [F32;2],
	fRec337: [F32;2],
	fRec342: [F32;2],
	fVec59: [F32;16384],
	fVec60: [F32;2],
	fRec341: [F32;2],
	fRec338: [F32;2],
	fRec346: [F32;2],
	fVec61: [F32;16384],
	fVec62: [F32;2],
	fRec345: [F32;2],
	fRec343: [F32;2],
	fRec348: [F32;2],
	fVec63: [F32;16384],
	fVec64: [F32;2],
	fRec347: [F32;2],
	fRec344: [F32;2],
	fVec65: [F32;1024],
	fHslider37: F32,
	fConst37: F32,
	fRec349: [F32;2],
	fRec350: [F32;2],
	fHslider38: F32,
	fVec66: [F32;16384],
	fVec67: [F32;2],
	fRec317: [F32;2],
	fRec354: [F32;2],
	fRec356: [F32;2],
	fVec68: [F32;1024],
	fVec69: [F32;16384],
	fVec70: [F32;2],
	fRec355: [F32;2],
	fVec71: [F32;16384],
	fVec72: [F32;2],
	fRec353: [F32;2],
	fRec351: [F32;2],
	fRec358: [F32;2],
	fVec73: [F32;16384],
	fVec74: [F32;2],
	fRec357: [F32;2],
	fRec352: [F32;2],
	fRec362: [F32;2],
	fVec75: [F32;16384],
	fVec76: [F32;2],
	fRec361: [F32;2],
	fRec359: [F32;2],
	fRec364: [F32;2],
	fVec77: [F32;16384],
	fVec78: [F32;2],
	fRec363: [F32;2],
	fRec360: [F32;2],
	fRec368: [F32;2],
	fVec79: [F32;16384],
	fVec80: [F32;2],
	fRec367: [F32;2],
	fRec365: [F32;2],
	fRec370: [F32;2],
	fVec81: [F32;16384],
	fVec82: [F32;2],
	fRec369: [F32;2],
	fRec366: [F32;2],
	fRec374: [F32;2],
	fVec83: [F32;16384],
	fVec84: [F32;2],
	fRec373: [F32;2],
	fRec371: [F32;2],
	fRec376: [F32;2],
	fVec85: [F32;16384],
	fVec86: [F32;2],
	fRec375: [F32;2],
	fRec372: [F32;2],
	fRec380: [F32;2],
	fVec87: [F32;16384],
	fVec88: [F32;2],
	fRec379: [F32;2],
	fRec377: [F32;2],
	fRec382: [F32;2],
	fVec89: [F32;16384],
	fVec90: [F32;2],
	fRec381: [F32;2],
	fRec378: [F32;2],
	fVec91: [F32;16384],
	fVec92: [F32;16384],
	fVec93: [F32;2],
	fRec315: [F32;2],
	fConst38: F32,
	fConst40: F32,
	fRec314: [F32;2],
	fRec313: [F32;3],
	fRec312: [F32;3],
	fVec94: [F32;2],
	fConst42: F32,
	fRec311: [F32;2],
	fRec310: [F32;3],
	fRec309: [F32;3],
	fConst43: F32,
	fConst44: F32,
	fConst45: F32,
	fRec385: [F32;2],
	fRec384: [F32;3],
	fConst46: F32,
	fRec383: [F32;3],
	fConst47: F32,
	fConst48: F32,
	fConst49: F32,
	fRec389: [F32;2],
	fRec388: [F32;3],
	fConst50: F32,
	fRec387: [F32;3],
	fRec386: [F32;3],
	fHslider39: F32,
	fVec95: [F32;1024],
	fRec308: [F32;2],
	fHslider40: F32,
	fRec401: [F32;2],
	fVec96: [F32;16384],
	fVec97: [F32;16384],
	fVec98: [F32;2],
	fRec400: [F32;2],
	fRec399: [F32;2],
	fRec398: [F32;3],
	fRec397: [F32;3],
	fVec99: [F32;2],
	fRec396: [F32;2],
	fRec395: [F32;3],
	fRec394: [F32;3],
	fRec404: [F32;2],
	fRec403: [F32;3],
	fRec402: [F32;3],
	fRec408: [F32;2],
	fRec407: [F32;3],
	fRec406: [F32;3],
	fRec405: [F32;3],
	fVec100: [F32;1024],
	fRec393: [F32;2],
	fVec101: [F32;16384],
	fVec102: [F32;2],
	fRec392: [F32;2],
	fRec390: [F32;2],
	fRec410: [F32;2],
	fVec103: [F32;16384],
	fVec104: [F32;2],
	fRec409: [F32;2],
	fRec391: [F32;2],
	fVec105: [F32;16384],
	fVec106: [F32;2],
	fRec413: [F32;2],
	fRec411: [F32;2],
	fVec107: [F32;16384],
	fVec108: [F32;2],
	fRec414: [F32;2],
	fRec412: [F32;2],
	fVec109: [F32;16384],
	fVec110: [F32;2],
	fRec417: [F32;2],
	fRec415: [F32;2],
	fRec419: [F32;2],
	fVec111: [F32;16384],
	fVec112: [F32;2],
	fRec418: [F32;2],
	fRec416: [F32;2],
	fRec423: [F32;2],
	fVec113: [F32;16384],
	fVec114: [F32;2],
	fRec422: [F32;2],
	fRec420: [F32;2],
	fVec115: [F32;16384],
	fVec116: [F32;2],
	fRec424: [F32;2],
	fRec421: [F32;2],
	fRec306: [F32;2],
	fRec307: [F32;2],
	fHslider41: F32,
	fRec425: [F32;2],
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
			fHslider0: 0.0,
			fRec0: [0.0;2],
			fHslider1: 0.0,
			fRec3: [0.0;2],
			fHslider2: 0.0,
			fRec1: [0.0;2],
			fConst3: 0.0,
			IOTA0: 0,
			fConst4: 0.0,
			fHslider3: 0.0,
			fRec39: [0.0;2],
			fRec38: [0.0;2],
			fConst5: 0.0,
			fRec34: [0.0;2],
			fRec40: [0.0;2],
			fConst6: 0.0,
			fConst7: 0.0,
			fRec43: [0.0;2],
			iVec1: [0;2],
			iConst8: 0,
			iRec44: [0;2],
			fConst9: 0.0,
			fRec42: [0.0;2],
			fRec45: [0.0;4],
			fRec46: [0.0;2048],
			fVec2: [0.0;2],
			fConst10: 0.0,
			fConst11: 0.0,
			fButton0: 0.0,
			fVec3: [0.0;2],
			iRec47: [0;2],
			iRec49: [0;2],
			fRec48: [0.0;3],
			fVec4: [0.0;3],
			fRec41: [0.0;2048],
			fRec30: [0.0;2],
			fRec26: [0.0;2],
			fRec22: [0.0;2048],
			fRec24: [0.0;2],
			fHslider4: 0.0,
			fRec20: [0.0;4],
			fRec15: [0.0;2],
			fRec11: [0.0;2048],
			fRec9: [0.0;2],
			fConst12: 0.0,
			fRec8: [0.0;2],
			fRec6: [0.0;2],
			fRec5: [0.0;3],
			fRec4: [0.0;3],
			fHslider5: 0.0,
			fRec50: [0.0;2],
			fRec85: [0.0;2],
			fRec81: [0.0;2],
			fRec86: [0.0;2],
			fRec89: [0.0;2],
			iVec5: [0;2],
			iRec90: [0;2],
			fRec88: [0.0;2],
			fRec91: [0.0;4],
			fRec92: [0.0;2048],
			fVec6: [0.0;2],
			fButton1: 0.0,
			fVec7: [0.0;2],
			iRec93: [0;2],
			fRec94: [0.0;3],
			fVec8: [0.0;3],
			fRec87: [0.0;2048],
			fRec77: [0.0;2],
			fRec73: [0.0;2],
			fRec69: [0.0;2048],
			fRec71: [0.0;2],
			fRec67: [0.0;4],
			fRec62: [0.0;2],
			fRec58: [0.0;2048],
			fRec56: [0.0;2],
			fRec55: [0.0;2],
			fRec53: [0.0;2],
			fRec52: [0.0;3],
			fRec51: [0.0;3],
			fHslider6: 0.0,
			fRec95: [0.0;2],
			fRec130: [0.0;2],
			fRec126: [0.0;2],
			fRec131: [0.0;2],
			fRec134: [0.0;2],
			iVec9: [0;2],
			iRec135: [0;2],
			fRec133: [0.0;2],
			fRec136: [0.0;4],
			fRec137: [0.0;2048],
			fVec10: [0.0;2],
			fButton2: 0.0,
			fVec11: [0.0;2],
			iRec138: [0;2],
			fRec139: [0.0;3],
			fVec12: [0.0;3],
			fRec132: [0.0;2048],
			fRec122: [0.0;2],
			fRec118: [0.0;2],
			fRec114: [0.0;2048],
			fRec116: [0.0;2],
			fRec112: [0.0;4],
			fRec107: [0.0;2],
			fRec103: [0.0;2048],
			fRec101: [0.0;2],
			fRec100: [0.0;2],
			fRec98: [0.0;2],
			fRec97: [0.0;3],
			fRec96: [0.0;3],
			fHslider7: 0.0,
			fRec140: [0.0;2],
			fRec175: [0.0;2],
			fRec171: [0.0;2],
			fRec176: [0.0;2],
			fRec179: [0.0;2],
			iVec13: [0;2],
			iRec180: [0;2],
			fRec178: [0.0;2],
			fRec181: [0.0;4],
			fRec182: [0.0;2048],
			fVec14: [0.0;2],
			fButton3: 0.0,
			fVec15: [0.0;2],
			iRec183: [0;2],
			fRec184: [0.0;3],
			fVec16: [0.0;3],
			fRec177: [0.0;2048],
			fRec167: [0.0;2],
			fRec163: [0.0;2],
			fRec159: [0.0;2048],
			fRec161: [0.0;2],
			fRec157: [0.0;4],
			fRec152: [0.0;2],
			fRec148: [0.0;2048],
			fRec146: [0.0;2],
			fRec145: [0.0;2],
			fRec143: [0.0;2],
			fRec142: [0.0;3],
			fRec141: [0.0;3],
			fHslider8: 0.0,
			fRec185: [0.0;2],
			fRec220: [0.0;2],
			fRec216: [0.0;2],
			fRec221: [0.0;2],
			fRec224: [0.0;2],
			iVec17: [0;2],
			iRec225: [0;2],
			fRec223: [0.0;2],
			fRec226: [0.0;4],
			fRec227: [0.0;2048],
			fVec18: [0.0;2],
			fButton4: 0.0,
			fVec19: [0.0;2],
			iRec228: [0;2],
			fRec229: [0.0;3],
			fVec20: [0.0;3],
			fRec222: [0.0;2048],
			fRec212: [0.0;2],
			fRec208: [0.0;2],
			fRec204: [0.0;2048],
			fRec206: [0.0;2],
			fRec202: [0.0;4],
			fRec197: [0.0;2],
			fRec193: [0.0;2048],
			fRec191: [0.0;2],
			fRec190: [0.0;2],
			fRec188: [0.0;2],
			fRec187: [0.0;3],
			fRec186: [0.0;3],
			fHslider9: 0.0,
			fRec230: [0.0;2],
			fHslider10: 0.0,
			fRec231: [0.0;2],
			fRec236: [0.0;2],
			fConst13: 0.0,
			fRec234: [0.0;2],
			fHslider11: 0.0,
			fRec237: [0.0;2],
			fRec233: [0.0;3],
			fRec232: [0.0;3],
			fHslider12: 0.0,
			fRec238: [0.0;2],
			fRec243: [0.0;2],
			fRec241: [0.0;2],
			fHslider13: 0.0,
			fRec244: [0.0;2],
			fRec240: [0.0;3],
			fRec239: [0.0;3],
			fHslider14: 0.0,
			fRec245: [0.0;2],
			fRec250: [0.0;2],
			fRec248: [0.0;2],
			fHslider15: 0.0,
			fRec251: [0.0;2],
			fRec247: [0.0;3],
			fRec246: [0.0;3],
			fHslider16: 0.0,
			fRec252: [0.0;2],
			fRec257: [0.0;2],
			fRec255: [0.0;2],
			fHslider17: 0.0,
			fRec258: [0.0;2],
			fRec254: [0.0;3],
			fRec253: [0.0;3],
			fHslider18: 0.0,
			fRec259: [0.0;2],
			fHslider19: 0.0,
			fRec260: [0.0;2],
			fHslider20: 0.0,
			fRec261: [0.0;2],
			fHslider21: 0.0,
			fHslider22: 0.0,
			fRec263: [0.0;2],
			fConst14: 0.0,
			fRec264: [0.0;2],
			fVec21: [0.0;2],
			fVec22: [0.0;4096],
			fConst15: 0.0,
			fRec262: [0.0;2],
			fRec266: [0.0;2],
			fRec267: [0.0;2],
			fVec23: [0.0;2],
			fVec24: [0.0;4096],
			fRec265: [0.0;2],
			fRec269: [0.0;2],
			fRec270: [0.0;2],
			fVec25: [0.0;2],
			fVec26: [0.0;4096],
			fRec268: [0.0;2],
			fHslider23: 0.0,
			fRec271: [0.0;2],
			fHslider24: 0.0,
			fRec273: [0.0;2],
			fRec274: [0.0;2],
			fVec27: [0.0;2],
			fVec28: [0.0;4096],
			fRec272: [0.0;2],
			fRec276: [0.0;2],
			fRec277: [0.0;2],
			fVec29: [0.0;2],
			fVec30: [0.0;4096],
			fRec275: [0.0;2],
			fRec279: [0.0;2],
			fRec280: [0.0;2],
			fVec31: [0.0;2],
			fVec32: [0.0;4096],
			fRec278: [0.0;2],
			fHslider25: 0.0,
			fRec281: [0.0;2],
			fHslider26: 0.0,
			fRec283: [0.0;2],
			fRec284: [0.0;2],
			fVec33: [0.0;2],
			fVec34: [0.0;4096],
			fRec282: [0.0;2],
			fRec286: [0.0;2],
			fRec287: [0.0;2],
			fVec35: [0.0;2],
			fVec36: [0.0;4096],
			fRec285: [0.0;2],
			fRec289: [0.0;2],
			fRec290: [0.0;2],
			fVec37: [0.0;2],
			fVec38: [0.0;4096],
			fRec288: [0.0;2],
			fHslider27: 0.0,
			fRec291: [0.0;2],
			fHslider28: 0.0,
			fRec293: [0.0;2],
			fRec294: [0.0;2],
			fVec39: [0.0;2],
			fVec40: [0.0;4096],
			fRec292: [0.0;2],
			fRec296: [0.0;2],
			fRec297: [0.0;2],
			fVec41: [0.0;2],
			fVec42: [0.0;4096],
			fRec295: [0.0;2],
			fRec299: [0.0;2],
			fRec300: [0.0;2],
			fVec43: [0.0;2],
			fVec44: [0.0;4096],
			fRec298: [0.0;2],
			fHslider29: 0.0,
			fRec301: [0.0;2],
			fConst16: 0.0,
			fHslider30: 0.0,
			fRec302: [0.0;2],
			fHslider31: 0.0,
			fRec303: [0.0;2],
			fConst17: 0.0,
			fHslider32: 0.0,
			fRec305: [0.0;2],
			fHslider33: 0.0,
			fRec304: [0.0;2097152],
			fHslider34: 0.0,
			fHslider35: 0.0,
			fConst20: 0.0,
			fConst21: 0.0,
			fConst23: 0.0,
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
			fRec316: [0.0;2],
			fRec318: [0.0;2],
			fRec322: [0.0;2],
			fVec45: [0.0;16384],
			fVec46: [0.0;2],
			fRec321: [0.0;2],
			fRec319: [0.0;2],
			fRec324: [0.0;2],
			fVec47: [0.0;16384],
			fVec48: [0.0;2],
			fRec323: [0.0;2],
			fRec320: [0.0;2],
			fRec328: [0.0;2],
			fVec49: [0.0;16384],
			fVec50: [0.0;2],
			fRec327: [0.0;2],
			fRec325: [0.0;2],
			fRec330: [0.0;2],
			fVec51: [0.0;16384],
			fVec52: [0.0;2],
			fRec329: [0.0;2],
			fRec326: [0.0;2],
			fRec334: [0.0;2],
			fVec53: [0.0;16384],
			fVec54: [0.0;2],
			fRec333: [0.0;2],
			fRec331: [0.0;2],
			fRec336: [0.0;2],
			fVec55: [0.0;16384],
			fVec56: [0.0;2],
			fRec335: [0.0;2],
			fRec332: [0.0;2],
			fRec340: [0.0;2],
			fVec57: [0.0;16384],
			fVec58: [0.0;2],
			fRec339: [0.0;2],
			fRec337: [0.0;2],
			fRec342: [0.0;2],
			fVec59: [0.0;16384],
			fVec60: [0.0;2],
			fRec341: [0.0;2],
			fRec338: [0.0;2],
			fRec346: [0.0;2],
			fVec61: [0.0;16384],
			fVec62: [0.0;2],
			fRec345: [0.0;2],
			fRec343: [0.0;2],
			fRec348: [0.0;2],
			fVec63: [0.0;16384],
			fVec64: [0.0;2],
			fRec347: [0.0;2],
			fRec344: [0.0;2],
			fVec65: [0.0;1024],
			fHslider37: 0.0,
			fConst37: 0.0,
			fRec349: [0.0;2],
			fRec350: [0.0;2],
			fHslider38: 0.0,
			fVec66: [0.0;16384],
			fVec67: [0.0;2],
			fRec317: [0.0;2],
			fRec354: [0.0;2],
			fRec356: [0.0;2],
			fVec68: [0.0;1024],
			fVec69: [0.0;16384],
			fVec70: [0.0;2],
			fRec355: [0.0;2],
			fVec71: [0.0;16384],
			fVec72: [0.0;2],
			fRec353: [0.0;2],
			fRec351: [0.0;2],
			fRec358: [0.0;2],
			fVec73: [0.0;16384],
			fVec74: [0.0;2],
			fRec357: [0.0;2],
			fRec352: [0.0;2],
			fRec362: [0.0;2],
			fVec75: [0.0;16384],
			fVec76: [0.0;2],
			fRec361: [0.0;2],
			fRec359: [0.0;2],
			fRec364: [0.0;2],
			fVec77: [0.0;16384],
			fVec78: [0.0;2],
			fRec363: [0.0;2],
			fRec360: [0.0;2],
			fRec368: [0.0;2],
			fVec79: [0.0;16384],
			fVec80: [0.0;2],
			fRec367: [0.0;2],
			fRec365: [0.0;2],
			fRec370: [0.0;2],
			fVec81: [0.0;16384],
			fVec82: [0.0;2],
			fRec369: [0.0;2],
			fRec366: [0.0;2],
			fRec374: [0.0;2],
			fVec83: [0.0;16384],
			fVec84: [0.0;2],
			fRec373: [0.0;2],
			fRec371: [0.0;2],
			fRec376: [0.0;2],
			fVec85: [0.0;16384],
			fVec86: [0.0;2],
			fRec375: [0.0;2],
			fRec372: [0.0;2],
			fRec380: [0.0;2],
			fVec87: [0.0;16384],
			fVec88: [0.0;2],
			fRec379: [0.0;2],
			fRec377: [0.0;2],
			fRec382: [0.0;2],
			fVec89: [0.0;16384],
			fVec90: [0.0;2],
			fRec381: [0.0;2],
			fRec378: [0.0;2],
			fVec91: [0.0;16384],
			fVec92: [0.0;16384],
			fVec93: [0.0;2],
			fRec315: [0.0;2],
			fConst38: 0.0,
			fConst40: 0.0,
			fRec314: [0.0;2],
			fRec313: [0.0;3],
			fRec312: [0.0;3],
			fVec94: [0.0;2],
			fConst42: 0.0,
			fRec311: [0.0;2],
			fRec310: [0.0;3],
			fRec309: [0.0;3],
			fConst43: 0.0,
			fConst44: 0.0,
			fConst45: 0.0,
			fRec385: [0.0;2],
			fRec384: [0.0;3],
			fConst46: 0.0,
			fRec383: [0.0;3],
			fConst47: 0.0,
			fConst48: 0.0,
			fConst49: 0.0,
			fRec389: [0.0;2],
			fRec388: [0.0;3],
			fConst50: 0.0,
			fRec387: [0.0;3],
			fRec386: [0.0;3],
			fHslider39: 0.0,
			fVec95: [0.0;1024],
			fRec308: [0.0;2],
			fHslider40: 0.0,
			fRec401: [0.0;2],
			fVec96: [0.0;16384],
			fVec97: [0.0;16384],
			fVec98: [0.0;2],
			fRec400: [0.0;2],
			fRec399: [0.0;2],
			fRec398: [0.0;3],
			fRec397: [0.0;3],
			fVec99: [0.0;2],
			fRec396: [0.0;2],
			fRec395: [0.0;3],
			fRec394: [0.0;3],
			fRec404: [0.0;2],
			fRec403: [0.0;3],
			fRec402: [0.0;3],
			fRec408: [0.0;2],
			fRec407: [0.0;3],
			fRec406: [0.0;3],
			fRec405: [0.0;3],
			fVec100: [0.0;1024],
			fRec393: [0.0;2],
			fVec101: [0.0;16384],
			fVec102: [0.0;2],
			fRec392: [0.0;2],
			fRec390: [0.0;2],
			fRec410: [0.0;2],
			fVec103: [0.0;16384],
			fVec104: [0.0;2],
			fRec409: [0.0;2],
			fRec391: [0.0;2],
			fVec105: [0.0;16384],
			fVec106: [0.0;2],
			fRec413: [0.0;2],
			fRec411: [0.0;2],
			fVec107: [0.0;16384],
			fVec108: [0.0;2],
			fRec414: [0.0;2],
			fRec412: [0.0;2],
			fVec109: [0.0;16384],
			fVec110: [0.0;2],
			fRec417: [0.0;2],
			fRec415: [0.0;2],
			fRec419: [0.0;2],
			fVec111: [0.0;16384],
			fVec112: [0.0;2],
			fRec418: [0.0;2],
			fRec416: [0.0;2],
			fRec423: [0.0;2],
			fVec113: [0.0;16384],
			fVec114: [0.0;2],
			fRec422: [0.0;2],
			fRec420: [0.0;2],
			fVec115: [0.0;16384],
			fVec116: [0.0;2],
			fRec424: [0.0;2],
			fRec421: [0.0;2],
			fRec306: [0.0;2],
			fRec307: [0.0;2],
			fHslider41: 0.0,
			fRec425: [0.0;2],
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
		m.declare("compile_options", r"-a C:\Users\Pierre\AppData\Local\Temp\.tmpq2mSuW -lang rust -ct 1 -es 1 -mcd 16 -single -ftz 0");
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
			self.fRec0[l1 as usize] = 0.0;
		}
		for l3 in 0..2 {
			self.fRec3[l3 as usize] = 0.0;
		}
		for l4 in 0..2 {
			self.fRec1[l4 as usize] = 0.0;
		}
		self.IOTA0 = 0;
		for l5 in 0..2 {
			self.fRec39[l5 as usize] = 0.0;
		}
		for l6 in 0..2 {
			self.fRec38[l6 as usize] = 0.0;
		}
		for l7 in 0..2 {
			self.fRec34[l7 as usize] = 0.0;
		}
		for l8 in 0..2 {
			self.fRec40[l8 as usize] = 0.0;
		}
		for l9 in 0..2 {
			self.fRec43[l9 as usize] = 0.0;
		}
		for l10 in 0..2 {
			self.iVec1[l10 as usize] = 0;
		}
		for l11 in 0..2 {
			self.iRec44[l11 as usize] = 0;
		}
		for l12 in 0..2 {
			self.fRec42[l12 as usize] = 0.0;
		}
		for l13 in 0..4 {
			self.fRec45[l13 as usize] = 0.0;
		}
		for l14 in 0..2048 {
			self.fRec46[l14 as usize] = 0.0;
		}
		for l15 in 0..2 {
			self.fVec2[l15 as usize] = 0.0;
		}
		for l16 in 0..2 {
			self.fVec3[l16 as usize] = 0.0;
		}
		for l17 in 0..2 {
			self.iRec47[l17 as usize] = 0;
		}
		for l18 in 0..2 {
			self.iRec49[l18 as usize] = 0;
		}
		for l19 in 0..3 {
			self.fRec48[l19 as usize] = 0.0;
		}
		for l20 in 0..3 {
			self.fVec4[l20 as usize] = 0.0;
		}
		for l21 in 0..2048 {
			self.fRec41[l21 as usize] = 0.0;
		}
		for l22 in 0..2 {
			self.fRec30[l22 as usize] = 0.0;
		}
		for l23 in 0..2 {
			self.fRec26[l23 as usize] = 0.0;
		}
		for l24 in 0..2048 {
			self.fRec22[l24 as usize] = 0.0;
		}
		for l25 in 0..2 {
			self.fRec24[l25 as usize] = 0.0;
		}
		for l26 in 0..4 {
			self.fRec20[l26 as usize] = 0.0;
		}
		for l27 in 0..2 {
			self.fRec15[l27 as usize] = 0.0;
		}
		for l28 in 0..2048 {
			self.fRec11[l28 as usize] = 0.0;
		}
		for l29 in 0..2 {
			self.fRec9[l29 as usize] = 0.0;
		}
		for l30 in 0..2 {
			self.fRec8[l30 as usize] = 0.0;
		}
		for l31 in 0..2 {
			self.fRec6[l31 as usize] = 0.0;
		}
		for l32 in 0..3 {
			self.fRec5[l32 as usize] = 0.0;
		}
		for l33 in 0..3 {
			self.fRec4[l33 as usize] = 0.0;
		}
		for l34 in 0..2 {
			self.fRec50[l34 as usize] = 0.0;
		}
		for l35 in 0..2 {
			self.fRec85[l35 as usize] = 0.0;
		}
		for l36 in 0..2 {
			self.fRec81[l36 as usize] = 0.0;
		}
		for l37 in 0..2 {
			self.fRec86[l37 as usize] = 0.0;
		}
		for l38 in 0..2 {
			self.fRec89[l38 as usize] = 0.0;
		}
		for l39 in 0..2 {
			self.iVec5[l39 as usize] = 0;
		}
		for l40 in 0..2 {
			self.iRec90[l40 as usize] = 0;
		}
		for l41 in 0..2 {
			self.fRec88[l41 as usize] = 0.0;
		}
		for l42 in 0..4 {
			self.fRec91[l42 as usize] = 0.0;
		}
		for l43 in 0..2048 {
			self.fRec92[l43 as usize] = 0.0;
		}
		for l44 in 0..2 {
			self.fVec6[l44 as usize] = 0.0;
		}
		for l45 in 0..2 {
			self.fVec7[l45 as usize] = 0.0;
		}
		for l46 in 0..2 {
			self.iRec93[l46 as usize] = 0;
		}
		for l47 in 0..3 {
			self.fRec94[l47 as usize] = 0.0;
		}
		for l48 in 0..3 {
			self.fVec8[l48 as usize] = 0.0;
		}
		for l49 in 0..2048 {
			self.fRec87[l49 as usize] = 0.0;
		}
		for l50 in 0..2 {
			self.fRec77[l50 as usize] = 0.0;
		}
		for l51 in 0..2 {
			self.fRec73[l51 as usize] = 0.0;
		}
		for l52 in 0..2048 {
			self.fRec69[l52 as usize] = 0.0;
		}
		for l53 in 0..2 {
			self.fRec71[l53 as usize] = 0.0;
		}
		for l54 in 0..4 {
			self.fRec67[l54 as usize] = 0.0;
		}
		for l55 in 0..2 {
			self.fRec62[l55 as usize] = 0.0;
		}
		for l56 in 0..2048 {
			self.fRec58[l56 as usize] = 0.0;
		}
		for l57 in 0..2 {
			self.fRec56[l57 as usize] = 0.0;
		}
		for l58 in 0..2 {
			self.fRec55[l58 as usize] = 0.0;
		}
		for l59 in 0..2 {
			self.fRec53[l59 as usize] = 0.0;
		}
		for l60 in 0..3 {
			self.fRec52[l60 as usize] = 0.0;
		}
		for l61 in 0..3 {
			self.fRec51[l61 as usize] = 0.0;
		}
		for l62 in 0..2 {
			self.fRec95[l62 as usize] = 0.0;
		}
		for l63 in 0..2 {
			self.fRec130[l63 as usize] = 0.0;
		}
		for l64 in 0..2 {
			self.fRec126[l64 as usize] = 0.0;
		}
		for l65 in 0..2 {
			self.fRec131[l65 as usize] = 0.0;
		}
		for l66 in 0..2 {
			self.fRec134[l66 as usize] = 0.0;
		}
		for l67 in 0..2 {
			self.iVec9[l67 as usize] = 0;
		}
		for l68 in 0..2 {
			self.iRec135[l68 as usize] = 0;
		}
		for l69 in 0..2 {
			self.fRec133[l69 as usize] = 0.0;
		}
		for l70 in 0..4 {
			self.fRec136[l70 as usize] = 0.0;
		}
		for l71 in 0..2048 {
			self.fRec137[l71 as usize] = 0.0;
		}
		for l72 in 0..2 {
			self.fVec10[l72 as usize] = 0.0;
		}
		for l73 in 0..2 {
			self.fVec11[l73 as usize] = 0.0;
		}
		for l74 in 0..2 {
			self.iRec138[l74 as usize] = 0;
		}
		for l75 in 0..3 {
			self.fRec139[l75 as usize] = 0.0;
		}
		for l76 in 0..3 {
			self.fVec12[l76 as usize] = 0.0;
		}
		for l77 in 0..2048 {
			self.fRec132[l77 as usize] = 0.0;
		}
		for l78 in 0..2 {
			self.fRec122[l78 as usize] = 0.0;
		}
		for l79 in 0..2 {
			self.fRec118[l79 as usize] = 0.0;
		}
		for l80 in 0..2048 {
			self.fRec114[l80 as usize] = 0.0;
		}
		for l81 in 0..2 {
			self.fRec116[l81 as usize] = 0.0;
		}
		for l82 in 0..4 {
			self.fRec112[l82 as usize] = 0.0;
		}
		for l83 in 0..2 {
			self.fRec107[l83 as usize] = 0.0;
		}
		for l84 in 0..2048 {
			self.fRec103[l84 as usize] = 0.0;
		}
		for l85 in 0..2 {
			self.fRec101[l85 as usize] = 0.0;
		}
		for l86 in 0..2 {
			self.fRec100[l86 as usize] = 0.0;
		}
		for l87 in 0..2 {
			self.fRec98[l87 as usize] = 0.0;
		}
		for l88 in 0..3 {
			self.fRec97[l88 as usize] = 0.0;
		}
		for l89 in 0..3 {
			self.fRec96[l89 as usize] = 0.0;
		}
		for l90 in 0..2 {
			self.fRec140[l90 as usize] = 0.0;
		}
		for l91 in 0..2 {
			self.fRec175[l91 as usize] = 0.0;
		}
		for l92 in 0..2 {
			self.fRec171[l92 as usize] = 0.0;
		}
		for l93 in 0..2 {
			self.fRec176[l93 as usize] = 0.0;
		}
		for l94 in 0..2 {
			self.fRec179[l94 as usize] = 0.0;
		}
		for l95 in 0..2 {
			self.iVec13[l95 as usize] = 0;
		}
		for l96 in 0..2 {
			self.iRec180[l96 as usize] = 0;
		}
		for l97 in 0..2 {
			self.fRec178[l97 as usize] = 0.0;
		}
		for l98 in 0..4 {
			self.fRec181[l98 as usize] = 0.0;
		}
		for l99 in 0..2048 {
			self.fRec182[l99 as usize] = 0.0;
		}
		for l100 in 0..2 {
			self.fVec14[l100 as usize] = 0.0;
		}
		for l101 in 0..2 {
			self.fVec15[l101 as usize] = 0.0;
		}
		for l102 in 0..2 {
			self.iRec183[l102 as usize] = 0;
		}
		for l103 in 0..3 {
			self.fRec184[l103 as usize] = 0.0;
		}
		for l104 in 0..3 {
			self.fVec16[l104 as usize] = 0.0;
		}
		for l105 in 0..2048 {
			self.fRec177[l105 as usize] = 0.0;
		}
		for l106 in 0..2 {
			self.fRec167[l106 as usize] = 0.0;
		}
		for l107 in 0..2 {
			self.fRec163[l107 as usize] = 0.0;
		}
		for l108 in 0..2048 {
			self.fRec159[l108 as usize] = 0.0;
		}
		for l109 in 0..2 {
			self.fRec161[l109 as usize] = 0.0;
		}
		for l110 in 0..4 {
			self.fRec157[l110 as usize] = 0.0;
		}
		for l111 in 0..2 {
			self.fRec152[l111 as usize] = 0.0;
		}
		for l112 in 0..2048 {
			self.fRec148[l112 as usize] = 0.0;
		}
		for l113 in 0..2 {
			self.fRec146[l113 as usize] = 0.0;
		}
		for l114 in 0..2 {
			self.fRec145[l114 as usize] = 0.0;
		}
		for l115 in 0..2 {
			self.fRec143[l115 as usize] = 0.0;
		}
		for l116 in 0..3 {
			self.fRec142[l116 as usize] = 0.0;
		}
		for l117 in 0..3 {
			self.fRec141[l117 as usize] = 0.0;
		}
		for l118 in 0..2 {
			self.fRec185[l118 as usize] = 0.0;
		}
		for l119 in 0..2 {
			self.fRec220[l119 as usize] = 0.0;
		}
		for l120 in 0..2 {
			self.fRec216[l120 as usize] = 0.0;
		}
		for l121 in 0..2 {
			self.fRec221[l121 as usize] = 0.0;
		}
		for l122 in 0..2 {
			self.fRec224[l122 as usize] = 0.0;
		}
		for l123 in 0..2 {
			self.iVec17[l123 as usize] = 0;
		}
		for l124 in 0..2 {
			self.iRec225[l124 as usize] = 0;
		}
		for l125 in 0..2 {
			self.fRec223[l125 as usize] = 0.0;
		}
		for l126 in 0..4 {
			self.fRec226[l126 as usize] = 0.0;
		}
		for l127 in 0..2048 {
			self.fRec227[l127 as usize] = 0.0;
		}
		for l128 in 0..2 {
			self.fVec18[l128 as usize] = 0.0;
		}
		for l129 in 0..2 {
			self.fVec19[l129 as usize] = 0.0;
		}
		for l130 in 0..2 {
			self.iRec228[l130 as usize] = 0;
		}
		for l131 in 0..3 {
			self.fRec229[l131 as usize] = 0.0;
		}
		for l132 in 0..3 {
			self.fVec20[l132 as usize] = 0.0;
		}
		for l133 in 0..2048 {
			self.fRec222[l133 as usize] = 0.0;
		}
		for l134 in 0..2 {
			self.fRec212[l134 as usize] = 0.0;
		}
		for l135 in 0..2 {
			self.fRec208[l135 as usize] = 0.0;
		}
		for l136 in 0..2048 {
			self.fRec204[l136 as usize] = 0.0;
		}
		for l137 in 0..2 {
			self.fRec206[l137 as usize] = 0.0;
		}
		for l138 in 0..4 {
			self.fRec202[l138 as usize] = 0.0;
		}
		for l139 in 0..2 {
			self.fRec197[l139 as usize] = 0.0;
		}
		for l140 in 0..2048 {
			self.fRec193[l140 as usize] = 0.0;
		}
		for l141 in 0..2 {
			self.fRec191[l141 as usize] = 0.0;
		}
		for l142 in 0..2 {
			self.fRec190[l142 as usize] = 0.0;
		}
		for l143 in 0..2 {
			self.fRec188[l143 as usize] = 0.0;
		}
		for l144 in 0..3 {
			self.fRec187[l144 as usize] = 0.0;
		}
		for l145 in 0..3 {
			self.fRec186[l145 as usize] = 0.0;
		}
		for l146 in 0..2 {
			self.fRec230[l146 as usize] = 0.0;
		}
		for l147 in 0..2 {
			self.fRec231[l147 as usize] = 0.0;
		}
		for l148 in 0..2 {
			self.fRec236[l148 as usize] = 0.0;
		}
		for l149 in 0..2 {
			self.fRec234[l149 as usize] = 0.0;
		}
		for l150 in 0..2 {
			self.fRec237[l150 as usize] = 0.0;
		}
		for l151 in 0..3 {
			self.fRec233[l151 as usize] = 0.0;
		}
		for l152 in 0..3 {
			self.fRec232[l152 as usize] = 0.0;
		}
		for l153 in 0..2 {
			self.fRec238[l153 as usize] = 0.0;
		}
		for l154 in 0..2 {
			self.fRec243[l154 as usize] = 0.0;
		}
		for l155 in 0..2 {
			self.fRec241[l155 as usize] = 0.0;
		}
		for l156 in 0..2 {
			self.fRec244[l156 as usize] = 0.0;
		}
		for l157 in 0..3 {
			self.fRec240[l157 as usize] = 0.0;
		}
		for l158 in 0..3 {
			self.fRec239[l158 as usize] = 0.0;
		}
		for l159 in 0..2 {
			self.fRec245[l159 as usize] = 0.0;
		}
		for l160 in 0..2 {
			self.fRec250[l160 as usize] = 0.0;
		}
		for l161 in 0..2 {
			self.fRec248[l161 as usize] = 0.0;
		}
		for l162 in 0..2 {
			self.fRec251[l162 as usize] = 0.0;
		}
		for l163 in 0..3 {
			self.fRec247[l163 as usize] = 0.0;
		}
		for l164 in 0..3 {
			self.fRec246[l164 as usize] = 0.0;
		}
		for l165 in 0..2 {
			self.fRec252[l165 as usize] = 0.0;
		}
		for l166 in 0..2 {
			self.fRec257[l166 as usize] = 0.0;
		}
		for l167 in 0..2 {
			self.fRec255[l167 as usize] = 0.0;
		}
		for l168 in 0..2 {
			self.fRec258[l168 as usize] = 0.0;
		}
		for l169 in 0..3 {
			self.fRec254[l169 as usize] = 0.0;
		}
		for l170 in 0..3 {
			self.fRec253[l170 as usize] = 0.0;
		}
		for l171 in 0..2 {
			self.fRec259[l171 as usize] = 0.0;
		}
		for l172 in 0..2 {
			self.fRec260[l172 as usize] = 0.0;
		}
		for l173 in 0..2 {
			self.fRec261[l173 as usize] = 0.0;
		}
		for l174 in 0..2 {
			self.fRec263[l174 as usize] = 0.0;
		}
		for l175 in 0..2 {
			self.fRec264[l175 as usize] = 0.0;
		}
		for l176 in 0..2 {
			self.fVec21[l176 as usize] = 0.0;
		}
		for l177 in 0..4096 {
			self.fVec22[l177 as usize] = 0.0;
		}
		for l178 in 0..2 {
			self.fRec262[l178 as usize] = 0.0;
		}
		for l179 in 0..2 {
			self.fRec266[l179 as usize] = 0.0;
		}
		for l180 in 0..2 {
			self.fRec267[l180 as usize] = 0.0;
		}
		for l181 in 0..2 {
			self.fVec23[l181 as usize] = 0.0;
		}
		for l182 in 0..4096 {
			self.fVec24[l182 as usize] = 0.0;
		}
		for l183 in 0..2 {
			self.fRec265[l183 as usize] = 0.0;
		}
		for l184 in 0..2 {
			self.fRec269[l184 as usize] = 0.0;
		}
		for l185 in 0..2 {
			self.fRec270[l185 as usize] = 0.0;
		}
		for l186 in 0..2 {
			self.fVec25[l186 as usize] = 0.0;
		}
		for l187 in 0..4096 {
			self.fVec26[l187 as usize] = 0.0;
		}
		for l188 in 0..2 {
			self.fRec268[l188 as usize] = 0.0;
		}
		for l189 in 0..2 {
			self.fRec271[l189 as usize] = 0.0;
		}
		for l190 in 0..2 {
			self.fRec273[l190 as usize] = 0.0;
		}
		for l191 in 0..2 {
			self.fRec274[l191 as usize] = 0.0;
		}
		for l192 in 0..2 {
			self.fVec27[l192 as usize] = 0.0;
		}
		for l193 in 0..4096 {
			self.fVec28[l193 as usize] = 0.0;
		}
		for l194 in 0..2 {
			self.fRec272[l194 as usize] = 0.0;
		}
		for l195 in 0..2 {
			self.fRec276[l195 as usize] = 0.0;
		}
		for l196 in 0..2 {
			self.fRec277[l196 as usize] = 0.0;
		}
		for l197 in 0..2 {
			self.fVec29[l197 as usize] = 0.0;
		}
		for l198 in 0..4096 {
			self.fVec30[l198 as usize] = 0.0;
		}
		for l199 in 0..2 {
			self.fRec275[l199 as usize] = 0.0;
		}
		for l200 in 0..2 {
			self.fRec279[l200 as usize] = 0.0;
		}
		for l201 in 0..2 {
			self.fRec280[l201 as usize] = 0.0;
		}
		for l202 in 0..2 {
			self.fVec31[l202 as usize] = 0.0;
		}
		for l203 in 0..4096 {
			self.fVec32[l203 as usize] = 0.0;
		}
		for l204 in 0..2 {
			self.fRec278[l204 as usize] = 0.0;
		}
		for l205 in 0..2 {
			self.fRec281[l205 as usize] = 0.0;
		}
		for l206 in 0..2 {
			self.fRec283[l206 as usize] = 0.0;
		}
		for l207 in 0..2 {
			self.fRec284[l207 as usize] = 0.0;
		}
		for l208 in 0..2 {
			self.fVec33[l208 as usize] = 0.0;
		}
		for l209 in 0..4096 {
			self.fVec34[l209 as usize] = 0.0;
		}
		for l210 in 0..2 {
			self.fRec282[l210 as usize] = 0.0;
		}
		for l211 in 0..2 {
			self.fRec286[l211 as usize] = 0.0;
		}
		for l212 in 0..2 {
			self.fRec287[l212 as usize] = 0.0;
		}
		for l213 in 0..2 {
			self.fVec35[l213 as usize] = 0.0;
		}
		for l214 in 0..4096 {
			self.fVec36[l214 as usize] = 0.0;
		}
		for l215 in 0..2 {
			self.fRec285[l215 as usize] = 0.0;
		}
		for l216 in 0..2 {
			self.fRec289[l216 as usize] = 0.0;
		}
		for l217 in 0..2 {
			self.fRec290[l217 as usize] = 0.0;
		}
		for l218 in 0..2 {
			self.fVec37[l218 as usize] = 0.0;
		}
		for l219 in 0..4096 {
			self.fVec38[l219 as usize] = 0.0;
		}
		for l220 in 0..2 {
			self.fRec288[l220 as usize] = 0.0;
		}
		for l221 in 0..2 {
			self.fRec291[l221 as usize] = 0.0;
		}
		for l222 in 0..2 {
			self.fRec293[l222 as usize] = 0.0;
		}
		for l223 in 0..2 {
			self.fRec294[l223 as usize] = 0.0;
		}
		for l224 in 0..2 {
			self.fVec39[l224 as usize] = 0.0;
		}
		for l225 in 0..4096 {
			self.fVec40[l225 as usize] = 0.0;
		}
		for l226 in 0..2 {
			self.fRec292[l226 as usize] = 0.0;
		}
		for l227 in 0..2 {
			self.fRec296[l227 as usize] = 0.0;
		}
		for l228 in 0..2 {
			self.fRec297[l228 as usize] = 0.0;
		}
		for l229 in 0..2 {
			self.fVec41[l229 as usize] = 0.0;
		}
		for l230 in 0..4096 {
			self.fVec42[l230 as usize] = 0.0;
		}
		for l231 in 0..2 {
			self.fRec295[l231 as usize] = 0.0;
		}
		for l232 in 0..2 {
			self.fRec299[l232 as usize] = 0.0;
		}
		for l233 in 0..2 {
			self.fRec300[l233 as usize] = 0.0;
		}
		for l234 in 0..2 {
			self.fVec43[l234 as usize] = 0.0;
		}
		for l235 in 0..4096 {
			self.fVec44[l235 as usize] = 0.0;
		}
		for l236 in 0..2 {
			self.fRec298[l236 as usize] = 0.0;
		}
		for l237 in 0..2 {
			self.fRec301[l237 as usize] = 0.0;
		}
		for l238 in 0..2 {
			self.fRec302[l238 as usize] = 0.0;
		}
		for l239 in 0..2 {
			self.fRec303[l239 as usize] = 0.0;
		}
		for l240 in 0..2 {
			self.fRec305[l240 as usize] = 0.0;
		}
		for l241 in 0..2097152 {
			self.fRec304[l241 as usize] = 0.0;
		}
		for l242 in 0..2 {
			self.fRec316[l242 as usize] = 0.0;
		}
		for l243 in 0..2 {
			self.fRec318[l243 as usize] = 0.0;
		}
		for l244 in 0..2 {
			self.fRec322[l244 as usize] = 0.0;
		}
		for l245 in 0..16384 {
			self.fVec45[l245 as usize] = 0.0;
		}
		for l246 in 0..2 {
			self.fVec46[l246 as usize] = 0.0;
		}
		for l247 in 0..2 {
			self.fRec321[l247 as usize] = 0.0;
		}
		for l248 in 0..2 {
			self.fRec319[l248 as usize] = 0.0;
		}
		for l249 in 0..2 {
			self.fRec324[l249 as usize] = 0.0;
		}
		for l250 in 0..16384 {
			self.fVec47[l250 as usize] = 0.0;
		}
		for l251 in 0..2 {
			self.fVec48[l251 as usize] = 0.0;
		}
		for l252 in 0..2 {
			self.fRec323[l252 as usize] = 0.0;
		}
		for l253 in 0..2 {
			self.fRec320[l253 as usize] = 0.0;
		}
		for l254 in 0..2 {
			self.fRec328[l254 as usize] = 0.0;
		}
		for l255 in 0..16384 {
			self.fVec49[l255 as usize] = 0.0;
		}
		for l256 in 0..2 {
			self.fVec50[l256 as usize] = 0.0;
		}
		for l257 in 0..2 {
			self.fRec327[l257 as usize] = 0.0;
		}
		for l258 in 0..2 {
			self.fRec325[l258 as usize] = 0.0;
		}
		for l259 in 0..2 {
			self.fRec330[l259 as usize] = 0.0;
		}
		for l260 in 0..16384 {
			self.fVec51[l260 as usize] = 0.0;
		}
		for l261 in 0..2 {
			self.fVec52[l261 as usize] = 0.0;
		}
		for l262 in 0..2 {
			self.fRec329[l262 as usize] = 0.0;
		}
		for l263 in 0..2 {
			self.fRec326[l263 as usize] = 0.0;
		}
		for l264 in 0..2 {
			self.fRec334[l264 as usize] = 0.0;
		}
		for l265 in 0..16384 {
			self.fVec53[l265 as usize] = 0.0;
		}
		for l266 in 0..2 {
			self.fVec54[l266 as usize] = 0.0;
		}
		for l267 in 0..2 {
			self.fRec333[l267 as usize] = 0.0;
		}
		for l268 in 0..2 {
			self.fRec331[l268 as usize] = 0.0;
		}
		for l269 in 0..2 {
			self.fRec336[l269 as usize] = 0.0;
		}
		for l270 in 0..16384 {
			self.fVec55[l270 as usize] = 0.0;
		}
		for l271 in 0..2 {
			self.fVec56[l271 as usize] = 0.0;
		}
		for l272 in 0..2 {
			self.fRec335[l272 as usize] = 0.0;
		}
		for l273 in 0..2 {
			self.fRec332[l273 as usize] = 0.0;
		}
		for l274 in 0..2 {
			self.fRec340[l274 as usize] = 0.0;
		}
		for l275 in 0..16384 {
			self.fVec57[l275 as usize] = 0.0;
		}
		for l276 in 0..2 {
			self.fVec58[l276 as usize] = 0.0;
		}
		for l277 in 0..2 {
			self.fRec339[l277 as usize] = 0.0;
		}
		for l278 in 0..2 {
			self.fRec337[l278 as usize] = 0.0;
		}
		for l279 in 0..2 {
			self.fRec342[l279 as usize] = 0.0;
		}
		for l280 in 0..16384 {
			self.fVec59[l280 as usize] = 0.0;
		}
		for l281 in 0..2 {
			self.fVec60[l281 as usize] = 0.0;
		}
		for l282 in 0..2 {
			self.fRec341[l282 as usize] = 0.0;
		}
		for l283 in 0..2 {
			self.fRec338[l283 as usize] = 0.0;
		}
		for l284 in 0..2 {
			self.fRec346[l284 as usize] = 0.0;
		}
		for l285 in 0..16384 {
			self.fVec61[l285 as usize] = 0.0;
		}
		for l286 in 0..2 {
			self.fVec62[l286 as usize] = 0.0;
		}
		for l287 in 0..2 {
			self.fRec345[l287 as usize] = 0.0;
		}
		for l288 in 0..2 {
			self.fRec343[l288 as usize] = 0.0;
		}
		for l289 in 0..2 {
			self.fRec348[l289 as usize] = 0.0;
		}
		for l290 in 0..16384 {
			self.fVec63[l290 as usize] = 0.0;
		}
		for l291 in 0..2 {
			self.fVec64[l291 as usize] = 0.0;
		}
		for l292 in 0..2 {
			self.fRec347[l292 as usize] = 0.0;
		}
		for l293 in 0..2 {
			self.fRec344[l293 as usize] = 0.0;
		}
		for l294 in 0..1024 {
			self.fVec65[l294 as usize] = 0.0;
		}
		for l295 in 0..2 {
			self.fRec349[l295 as usize] = 0.0;
		}
		for l296 in 0..2 {
			self.fRec350[l296 as usize] = 0.0;
		}
		for l297 in 0..16384 {
			self.fVec66[l297 as usize] = 0.0;
		}
		for l298 in 0..2 {
			self.fVec67[l298 as usize] = 0.0;
		}
		for l299 in 0..2 {
			self.fRec317[l299 as usize] = 0.0;
		}
		for l300 in 0..2 {
			self.fRec354[l300 as usize] = 0.0;
		}
		for l301 in 0..2 {
			self.fRec356[l301 as usize] = 0.0;
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
			self.fRec355[l305 as usize] = 0.0;
		}
		for l306 in 0..16384 {
			self.fVec71[l306 as usize] = 0.0;
		}
		for l307 in 0..2 {
			self.fVec72[l307 as usize] = 0.0;
		}
		for l308 in 0..2 {
			self.fRec353[l308 as usize] = 0.0;
		}
		for l309 in 0..2 {
			self.fRec351[l309 as usize] = 0.0;
		}
		for l310 in 0..2 {
			self.fRec358[l310 as usize] = 0.0;
		}
		for l311 in 0..16384 {
			self.fVec73[l311 as usize] = 0.0;
		}
		for l312 in 0..2 {
			self.fVec74[l312 as usize] = 0.0;
		}
		for l313 in 0..2 {
			self.fRec357[l313 as usize] = 0.0;
		}
		for l314 in 0..2 {
			self.fRec352[l314 as usize] = 0.0;
		}
		for l315 in 0..2 {
			self.fRec362[l315 as usize] = 0.0;
		}
		for l316 in 0..16384 {
			self.fVec75[l316 as usize] = 0.0;
		}
		for l317 in 0..2 {
			self.fVec76[l317 as usize] = 0.0;
		}
		for l318 in 0..2 {
			self.fRec361[l318 as usize] = 0.0;
		}
		for l319 in 0..2 {
			self.fRec359[l319 as usize] = 0.0;
		}
		for l320 in 0..2 {
			self.fRec364[l320 as usize] = 0.0;
		}
		for l321 in 0..16384 {
			self.fVec77[l321 as usize] = 0.0;
		}
		for l322 in 0..2 {
			self.fVec78[l322 as usize] = 0.0;
		}
		for l323 in 0..2 {
			self.fRec363[l323 as usize] = 0.0;
		}
		for l324 in 0..2 {
			self.fRec360[l324 as usize] = 0.0;
		}
		for l325 in 0..2 {
			self.fRec368[l325 as usize] = 0.0;
		}
		for l326 in 0..16384 {
			self.fVec79[l326 as usize] = 0.0;
		}
		for l327 in 0..2 {
			self.fVec80[l327 as usize] = 0.0;
		}
		for l328 in 0..2 {
			self.fRec367[l328 as usize] = 0.0;
		}
		for l329 in 0..2 {
			self.fRec365[l329 as usize] = 0.0;
		}
		for l330 in 0..2 {
			self.fRec370[l330 as usize] = 0.0;
		}
		for l331 in 0..16384 {
			self.fVec81[l331 as usize] = 0.0;
		}
		for l332 in 0..2 {
			self.fVec82[l332 as usize] = 0.0;
		}
		for l333 in 0..2 {
			self.fRec369[l333 as usize] = 0.0;
		}
		for l334 in 0..2 {
			self.fRec366[l334 as usize] = 0.0;
		}
		for l335 in 0..2 {
			self.fRec374[l335 as usize] = 0.0;
		}
		for l336 in 0..16384 {
			self.fVec83[l336 as usize] = 0.0;
		}
		for l337 in 0..2 {
			self.fVec84[l337 as usize] = 0.0;
		}
		for l338 in 0..2 {
			self.fRec373[l338 as usize] = 0.0;
		}
		for l339 in 0..2 {
			self.fRec371[l339 as usize] = 0.0;
		}
		for l340 in 0..2 {
			self.fRec376[l340 as usize] = 0.0;
		}
		for l341 in 0..16384 {
			self.fVec85[l341 as usize] = 0.0;
		}
		for l342 in 0..2 {
			self.fVec86[l342 as usize] = 0.0;
		}
		for l343 in 0..2 {
			self.fRec375[l343 as usize] = 0.0;
		}
		for l344 in 0..2 {
			self.fRec372[l344 as usize] = 0.0;
		}
		for l345 in 0..2 {
			self.fRec380[l345 as usize] = 0.0;
		}
		for l346 in 0..16384 {
			self.fVec87[l346 as usize] = 0.0;
		}
		for l347 in 0..2 {
			self.fVec88[l347 as usize] = 0.0;
		}
		for l348 in 0..2 {
			self.fRec379[l348 as usize] = 0.0;
		}
		for l349 in 0..2 {
			self.fRec377[l349 as usize] = 0.0;
		}
		for l350 in 0..2 {
			self.fRec382[l350 as usize] = 0.0;
		}
		for l351 in 0..16384 {
			self.fVec89[l351 as usize] = 0.0;
		}
		for l352 in 0..2 {
			self.fVec90[l352 as usize] = 0.0;
		}
		for l353 in 0..2 {
			self.fRec381[l353 as usize] = 0.0;
		}
		for l354 in 0..2 {
			self.fRec378[l354 as usize] = 0.0;
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
			self.fRec315[l358 as usize] = 0.0;
		}
		for l359 in 0..2 {
			self.fRec314[l359 as usize] = 0.0;
		}
		for l360 in 0..3 {
			self.fRec313[l360 as usize] = 0.0;
		}
		for l361 in 0..3 {
			self.fRec312[l361 as usize] = 0.0;
		}
		for l362 in 0..2 {
			self.fVec94[l362 as usize] = 0.0;
		}
		for l363 in 0..2 {
			self.fRec311[l363 as usize] = 0.0;
		}
		for l364 in 0..3 {
			self.fRec310[l364 as usize] = 0.0;
		}
		for l365 in 0..3 {
			self.fRec309[l365 as usize] = 0.0;
		}
		for l366 in 0..2 {
			self.fRec385[l366 as usize] = 0.0;
		}
		for l367 in 0..3 {
			self.fRec384[l367 as usize] = 0.0;
		}
		for l368 in 0..3 {
			self.fRec383[l368 as usize] = 0.0;
		}
		for l369 in 0..2 {
			self.fRec389[l369 as usize] = 0.0;
		}
		for l370 in 0..3 {
			self.fRec388[l370 as usize] = 0.0;
		}
		for l371 in 0..3 {
			self.fRec387[l371 as usize] = 0.0;
		}
		for l372 in 0..3 {
			self.fRec386[l372 as usize] = 0.0;
		}
		for l373 in 0..1024 {
			self.fVec95[l373 as usize] = 0.0;
		}
		for l374 in 0..2 {
			self.fRec308[l374 as usize] = 0.0;
		}
		for l375 in 0..2 {
			self.fRec401[l375 as usize] = 0.0;
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
			self.fRec400[l379 as usize] = 0.0;
		}
		for l380 in 0..2 {
			self.fRec399[l380 as usize] = 0.0;
		}
		for l381 in 0..3 {
			self.fRec398[l381 as usize] = 0.0;
		}
		for l382 in 0..3 {
			self.fRec397[l382 as usize] = 0.0;
		}
		for l383 in 0..2 {
			self.fVec99[l383 as usize] = 0.0;
		}
		for l384 in 0..2 {
			self.fRec396[l384 as usize] = 0.0;
		}
		for l385 in 0..3 {
			self.fRec395[l385 as usize] = 0.0;
		}
		for l386 in 0..3 {
			self.fRec394[l386 as usize] = 0.0;
		}
		for l387 in 0..2 {
			self.fRec404[l387 as usize] = 0.0;
		}
		for l388 in 0..3 {
			self.fRec403[l388 as usize] = 0.0;
		}
		for l389 in 0..3 {
			self.fRec402[l389 as usize] = 0.0;
		}
		for l390 in 0..2 {
			self.fRec408[l390 as usize] = 0.0;
		}
		for l391 in 0..3 {
			self.fRec407[l391 as usize] = 0.0;
		}
		for l392 in 0..3 {
			self.fRec406[l392 as usize] = 0.0;
		}
		for l393 in 0..3 {
			self.fRec405[l393 as usize] = 0.0;
		}
		for l394 in 0..1024 {
			self.fVec100[l394 as usize] = 0.0;
		}
		for l395 in 0..2 {
			self.fRec393[l395 as usize] = 0.0;
		}
		for l396 in 0..16384 {
			self.fVec101[l396 as usize] = 0.0;
		}
		for l397 in 0..2 {
			self.fVec102[l397 as usize] = 0.0;
		}
		for l398 in 0..2 {
			self.fRec392[l398 as usize] = 0.0;
		}
		for l399 in 0..2 {
			self.fRec390[l399 as usize] = 0.0;
		}
		for l400 in 0..2 {
			self.fRec410[l400 as usize] = 0.0;
		}
		for l401 in 0..16384 {
			self.fVec103[l401 as usize] = 0.0;
		}
		for l402 in 0..2 {
			self.fVec104[l402 as usize] = 0.0;
		}
		for l403 in 0..2 {
			self.fRec409[l403 as usize] = 0.0;
		}
		for l404 in 0..2 {
			self.fRec391[l404 as usize] = 0.0;
		}
		for l405 in 0..16384 {
			self.fVec105[l405 as usize] = 0.0;
		}
		for l406 in 0..2 {
			self.fVec106[l406 as usize] = 0.0;
		}
		for l407 in 0..2 {
			self.fRec413[l407 as usize] = 0.0;
		}
		for l408 in 0..2 {
			self.fRec411[l408 as usize] = 0.0;
		}
		for l409 in 0..16384 {
			self.fVec107[l409 as usize] = 0.0;
		}
		for l410 in 0..2 {
			self.fVec108[l410 as usize] = 0.0;
		}
		for l411 in 0..2 {
			self.fRec414[l411 as usize] = 0.0;
		}
		for l412 in 0..2 {
			self.fRec412[l412 as usize] = 0.0;
		}
		for l413 in 0..16384 {
			self.fVec109[l413 as usize] = 0.0;
		}
		for l414 in 0..2 {
			self.fVec110[l414 as usize] = 0.0;
		}
		for l415 in 0..2 {
			self.fRec417[l415 as usize] = 0.0;
		}
		for l416 in 0..2 {
			self.fRec415[l416 as usize] = 0.0;
		}
		for l417 in 0..2 {
			self.fRec419[l417 as usize] = 0.0;
		}
		for l418 in 0..16384 {
			self.fVec111[l418 as usize] = 0.0;
		}
		for l419 in 0..2 {
			self.fVec112[l419 as usize] = 0.0;
		}
		for l420 in 0..2 {
			self.fRec418[l420 as usize] = 0.0;
		}
		for l421 in 0..2 {
			self.fRec416[l421 as usize] = 0.0;
		}
		for l422 in 0..2 {
			self.fRec423[l422 as usize] = 0.0;
		}
		for l423 in 0..16384 {
			self.fVec113[l423 as usize] = 0.0;
		}
		for l424 in 0..2 {
			self.fVec114[l424 as usize] = 0.0;
		}
		for l425 in 0..2 {
			self.fRec422[l425 as usize] = 0.0;
		}
		for l426 in 0..2 {
			self.fRec420[l426 as usize] = 0.0;
		}
		for l427 in 0..16384 {
			self.fVec115[l427 as usize] = 0.0;
		}
		for l428 in 0..2 {
			self.fVec116[l428 as usize] = 0.0;
		}
		for l429 in 0..2 {
			self.fRec424[l429 as usize] = 0.0;
		}
		for l430 in 0..2 {
			self.fRec421[l430 as usize] = 0.0;
		}
		for l431 in 0..2 {
			self.fRec306[l431 as usize] = 0.0;
		}
		for l432 in 0..2 {
			self.fRec307[l432 as usize] = 0.0;
		}
		for l433 in 0..2 {
			self.fRec425[l433 as usize] = 0.0;
		}
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		self.fConst0 = F32::min(1.92e+05, F32::max(1.0, (self.fSampleRate) as F32));
		self.fConst1 = 44.1 / self.fConst0;
		self.fConst2 = 1.0 - self.fConst1;
		self.fConst3 = 3.1415927 / self.fConst0;
		self.fConst4 = 0.00882353 * self.fConst0;
		self.fConst5 = 0.00073529413 * self.fConst0;
		self.fConst6 = F32::exp(0.0 - 1e+04 / self.fConst0);
		self.fConst7 = 1.0 - self.fConst6;
		self.iConst8 = (0.1 * self.fConst0) as i32;
		self.fConst9 = F32::exp(0.0 - 5e+01 / self.fConst0);
		self.fConst10 = 15.707963 / self.fConst0;
		self.fConst11 = 0.002 * self.fConst0;
		self.fConst12 = F32::exp(0.0 - 1e+01 / self.fConst0);
		self.fConst13 = 1.0 / self.fConst0;
		self.fConst14 = 0.5 * self.fConst0;
		self.fConst15 = 0.25 * self.fConst0;
		self.fConst16 = 1.3333334 / self.fConst0;
		self.fConst17 = 1e+01 * self.fConst0;
		let mut fConst18: F32 = F32::tan(1382.3008 / self.fConst0);
		let mut fConst19: F32 = mydsp_faustpower2_f(fConst18);
		self.fConst20 = 1.0 / fConst19;
		self.fConst21 = 2.0 * (1.0 - self.fConst20);
		let mut fConst22: F32 = 1.0 / fConst18;
		self.fConst23 = (fConst22 + -0.618034) / fConst18 + 1.0;
		self.fConst24 = 1.0 / ((fConst22 + 0.618034) / fConst18 + 1.0);
		self.fConst25 = (fConst22 + -1.618034) / fConst18 + 1.0;
		self.fConst26 = 1.0 / ((fConst22 + 1.618034) / fConst18 + 1.0);
		self.fConst27 = 1.0 - fConst22;
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
		let mut fConst41: F32 = fConst22 + 1.0;
		self.fConst42 = 1.0 / fConst41;
		self.fConst43 = self.fConst27 / fConst41;
		self.fConst44 = 1.0 / (fConst18 * fConst41);
		self.fConst45 = 0.0 - self.fConst44;
		self.fConst46 = 0.0 - 2.0 / fConst19;
		self.fConst47 = (fConst22 + -1.618034) / fConst18 + 1.0;
		self.fConst48 = 1.0 / ((fConst22 + 1.618034) / fConst18 + 1.0);
		self.fConst49 = 0.0 - 1.0 / (fConst28 * fConst39);
		self.fConst50 = 0.0 - 2.0 / fConst29;
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
			1 => Some(self.fHslider12),
			2 => Some(self.fHslider13),
			3 => Some(self.fHslider14),
			4 => Some(self.fHslider15),
			5 => Some(self.fHslider16),
			6 => Some(self.fHslider17),
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
			1 => { self.fHslider12 = value }
			2 => { self.fHslider13 = value }
			3 => { self.fHslider14 = value }
			4 => { self.fHslider15 = value }
			5 => { self.fHslider16 = value }
			6 => { self.fHslider17 = value }
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
		let mut fSlow0: F32 = self.fConst1 * self.fHslider0;
		let mut fSlow1: F32 = self.fConst1 * self.fHslider1;
		let mut fSlow2: F32 = self.fHslider2;
		let mut fSlow3: F32 = self.fConst1 * self.fHslider3;
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
		let mut fSlow14: F32 = self.fConst1 * self.fHslider9;
		let mut fSlow15: F32 = self.fHslider10;
		let mut fSlow16: F32 = self.fConst1 * self.fHslider11;
		let mut fSlow17: F32 = self.fHslider12;
		let mut fSlow18: F32 = self.fConst1 * self.fHslider13;
		let mut fSlow19: F32 = self.fHslider14;
		let mut fSlow20: F32 = self.fConst1 * self.fHslider15;
		let mut fSlow21: F32 = self.fHslider16;
		let mut fSlow22: F32 = self.fConst1 * self.fHslider17;
		let mut fSlow23: F32 = self.fConst1 * self.fHslider18;
		let mut fSlow24: F32 = self.fConst1 * self.fHslider19;
		let mut fSlow25: F32 = self.fConst1 * self.fHslider20;
		let mut fSlow26: F32 = self.fHslider21;
		let mut fSlow27: F32 = self.fHslider22;
		let mut fSlow28: F32 = self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow27 - fSlow26) + 0.5) as i32, 2047))) as usize] };
		let mut fSlow29: F32 = self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow26 + fSlow27) + 0.5) as i32, 2047))) as usize] };
		let mut fSlow30: F32 = self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * fSlow27 + 0.5) as i32, 2047))) as usize] };
		let mut fSlow31: F32 = self.fConst1 * self.fHslider23;
		let mut fSlow32: F32 = self.fHslider24;
		let mut fSlow33: F32 = self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow32 - fSlow26) + 0.5) as i32, 2047))) as usize] };
		let mut fSlow34: F32 = self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow26 + fSlow32) + 0.5) as i32, 2047))) as usize] };
		let mut fSlow35: F32 = self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * fSlow32 + 0.5) as i32, 2047))) as usize] };
		let mut fSlow36: F32 = self.fConst1 * self.fHslider25;
		let mut fSlow37: F32 = self.fHslider26;
		let mut fSlow38: F32 = self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow37 - fSlow26) + 0.5) as i32, 2047))) as usize] };
		let mut fSlow39: F32 = self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow26 + fSlow37) + 0.5) as i32, 2047))) as usize] };
		let mut fSlow40: F32 = self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * fSlow37 + 0.5) as i32, 2047))) as usize] };
		let mut fSlow41: F32 = self.fConst1 * self.fHslider27;
		let mut fSlow42: F32 = self.fHslider28;
		let mut fSlow43: F32 = self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow42 - fSlow26) + 0.5) as i32, 2047))) as usize] };
		let mut fSlow44: F32 = self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow26 + fSlow42) + 0.5) as i32, 2047))) as usize] };
		let mut fSlow45: F32 = self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * fSlow42 + 0.5) as i32, 2047))) as usize] };
		let mut fSlow46: F32 = self.fConst1 * self.fHslider29;
		let mut fSlow47: F32 = self.fConst1 * self.fHslider30;
		let mut fSlow48: F32 = self.fConst1 * self.fHslider31;
		let mut fSlow49: F32 = self.fConst1 * self.fHslider32;
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
		let mut fSlow119: F32 = self.fConst1 * self.fHslider41;
		let zipped_iterators = outputs0.zip(outputs1);
		for (output0, output1) in zipped_iterators {
			self.iVec0[0] = 1;
			self.fRec0[0] = fSlow0 + self.fConst2 * self.fRec0[1];
			let mut fTemp0: F32 = F32::min(1.4141995, 1.4142135 * self.fRec0[0]);
			let mut fTemp1: F32 = 1.4142135 * fTemp0;
			let mut fTemp2: F32 = 1.0 - fTemp1;
			self.fRec3[0] = fSlow1 + self.fConst2 * self.fRec3[1];
			self.fRec1[0] = self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow2 + self.fRec3[0]) + 0.5) as i32, 2047))) as usize] } + self.fConst2 * self.fRec1[1];
			let mut fTemp3: F32 = F32::tan(self.fConst3 * F32::max(2e+01, F32::min(1e+04, self.fRec1[0])));
			let mut fTemp4: F32 = 1.0 / fTemp3;
			let mut fTemp5: F32 = 2.0 - fTemp1;
			let mut fTemp6: F32 = mydsp_faustpower2_f(fTemp0);
			let mut fTemp7: F32 = fTemp6 + (fTemp5 + fTemp4) / fTemp3 + fTemp2;
			let mut fTemp8: F32 = 1.0 / mydsp_faustpower2_f(fTemp3);
			let mut fTemp9: F32 = fTemp1 + 2.0;
			let mut fTemp10: F32 = fTemp1 + fTemp6;
			let mut fTemp11: F32 = fTemp10 + (fTemp9 + fTemp4) / fTemp3 + 1.0;
			let mut fRec19: F32 = -1.0 * 0.9973053 * (0.9 * self.fRec20[2] + 0.05 * (self.fRec20[1] + self.fRec20[3]));
			self.fRec39[0] = fSlow3 + self.fConst2 * self.fRec39[1];
			self.fRec38[0] = self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow2 + self.fRec39[0]) + 0.5) as i32, 2047))) as usize] } + self.fConst2 * self.fRec38[1];
			let mut fTemp12: F32 = self.fConst5 * (3.4e+02 / self.fRec38[0] + -0.11);
			let mut fTemp13: F32 = fTemp12 + -1.499995;
			let mut iTemp14: i32 = (fTemp13) as i32;
			let mut iTemp15: i32 = (F32::min(self.fConst4, (std::cmp::max(0, i32::wrapping_add(iTemp14, 4))) as F32)) as i32;
			let mut iTemp16: i32 = i32::wrapping_add(iTemp15, 1);
			let mut fTemp17: F32 = F32::floor(fTemp13);
			let mut fTemp18: F32 = fTemp12 + (-3.0 - fTemp17);
			let mut fTemp19: F32 = fTemp12 + (-2.0 - fTemp17);
			let mut fTemp20: F32 = fTemp12 + (-1.0 - fTemp17);
			let mut fTemp21: F32 = fTemp20 * fTemp19;
			let mut fTemp22: F32 = fTemp21 * fTemp18;
			let mut fTemp23: F32 = fTemp12 + (-4.0 - fTemp17);
			let mut fTemp24: F32 = 0.0 - fTemp23;
			let mut iTemp25: i32 = (F32::min(self.fConst4, (std::cmp::max(0, i32::wrapping_add(iTemp14, 3))) as F32)) as i32;
			let mut iTemp26: i32 = i32::wrapping_add(iTemp25, 1);
			let mut fTemp27: F32 = 0.0 - 0.5 * fTemp23;
			let mut fTemp28: F32 = 0.0 - fTemp18;
			let mut iTemp29: i32 = (F32::min(self.fConst4, (std::cmp::max(0, i32::wrapping_add(iTemp14, 2))) as F32)) as i32;
			let mut iTemp30: i32 = i32::wrapping_add(iTemp29, 1);
			let mut fTemp31: F32 = 0.0 - 0.33333334 * fTemp23;
			let mut fTemp32: F32 = 0.0 - 0.5 * fTemp18;
			let mut fTemp33: F32 = 0.0 - fTemp19;
			let mut iTemp34: i32 = (F32::min(self.fConst4, (std::cmp::max(0, i32::wrapping_add(iTemp14, 1))) as F32)) as i32;
			let mut iTemp35: i32 = i32::wrapping_add(iTemp34, 1);
			let mut fTemp36: F32 = fTemp12 - fTemp17;
			let mut fTemp37: F32 = 0.0 - 0.25 * fTemp23;
			let mut fTemp38: F32 = 0.0 - 0.33333334 * fTemp18;
			let mut fTemp39: F32 = 0.0 - 0.5 * fTemp19;
			let mut fTemp40: F32 = 0.0 - fTemp20;
			let mut iTemp41: i32 = (F32::min(self.fConst4, (std::cmp::max(0, iTemp14)) as F32)) as i32;
			let mut iTemp42: i32 = i32::wrapping_add(iTemp41, 1);
			self.fRec34[0] = self.fRec11[((i32::wrapping_sub(self.IOTA0, iTemp42)) & 2047) as usize] * fTemp40 * fTemp39 * fTemp38 * fTemp37 + fTemp36 * (self.fRec11[((i32::wrapping_sub(self.IOTA0, iTemp35)) & 2047) as usize] * fTemp33 * fTemp32 * fTemp31 + 0.5 * fTemp20 * self.fRec11[((i32::wrapping_sub(self.IOTA0, iTemp30)) & 2047) as usize] * fTemp28 * fTemp27 + 0.16666667 * fTemp21 * self.fRec11[((i32::wrapping_sub(self.IOTA0, iTemp26)) & 2047) as usize] * fTemp24 + 0.041666668 * fTemp22 * self.fRec11[((i32::wrapping_sub(self.IOTA0, iTemp16)) & 2047) as usize]);
			self.fRec40[0] = 0.95 * self.fRec34[1] + 0.05 * self.fRec40[1];
			let mut fRec35: F32 = self.fRec40[0];
			self.fRec43[0] = self.fConst7 * F32::abs(self.fRec6[1]) + self.fConst6 * self.fRec43[1];
			let mut iTemp43: i32 = (self.fRec43[0] > 0.1) as i32;
			self.iVec1[0] = iTemp43;
			self.iRec44[0] = std::cmp::max(i32::wrapping_mul(self.iConst8, (iTemp43 < self.iVec1[1]) as i32), i32::wrapping_add(self.iRec44[1], -1));
			let mut fTemp44: F32 = F32::abs(F32::max((iTemp43) as F32, ((self.iRec44[0] > 0) as i32) as u32 as F32));
			let mut fTemp45: F32 = if (fTemp44 > self.fRec42[1]) as i32 != 0 {self.fConst6} else {self.fConst9};
			self.fRec42[0] = fTemp44 * (1.0 - fTemp45) + self.fRec42[1] * fTemp45;
			let mut fTemp46: F32 = 0.005 * self.fRec42[0] * self.fRec6[1];
			self.fRec45[0] = self.fRec9[1];
			self.fRec46[(self.IOTA0 & 2047) as usize] = -1.0 * 0.9973053 * (0.9 * self.fRec45[2] + 0.05 * (self.fRec45[1] + self.fRec45[3]));
			let mut fTemp47: F32 = fTemp21 * fTemp24;
			let mut fTemp48: F32 = fTemp20 * fTemp28 * fTemp27;
			let mut fTemp49: F32 = fTemp33 * fTemp32 * fTemp31;
			let mut fTemp50: F32 = fTemp40 * fTemp39 * fTemp38 * fTemp37;
			self.fVec2[0] = fTemp50 * self.fRec46[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp41, 2))) & 2047) as usize] + fTemp36 * (fTemp49 * self.fRec46[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp34, 2))) & 2047) as usize] + 0.5 * fTemp48 * self.fRec46[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp29, 2))) & 2047) as usize] + 0.16666667 * fTemp47 * self.fRec46[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp25, 2))) & 2047) as usize] + 0.041666668 * fTemp22 * self.fRec46[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp15, 2))) & 2047) as usize]);
			let mut fTemp51: F32 = F32::tan(self.fConst10 * self.fRec38[0]);
			let mut fTemp52: F32 = 1.0 / fTemp51;
			let mut fTemp53: F32 = (fTemp52 + 1.4142135) / fTemp51 + 1.0;
			self.fVec3[0] = fSlow4;
			self.iRec47[0] = i32::wrapping_add(i32::wrapping_mul(i32::wrapping_add(self.iRec47[1], (self.iRec47[1] > 0) as i32), (fSlow4 <= self.fVec3[1]) as i32), (fSlow4 > self.fVec3[1]) as i32);
			let mut fTemp54: F32 = (self.iRec47[0]) as F32 / F32::max(1.0, self.fConst11 * mydsp_faustpower2_f(1.0 - 0.0005 * self.fRec38[0]));
			self.iRec49[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, self.iRec49[1]), 12345);
			let mut fTemp55: F32 = 4.656613e-10 * (self.iRec49[0]) as F32;
			self.fRec48[0] = fTemp55 - (self.fRec48[2] * ((fTemp52 + -1.4142135) / fTemp51 + 1.0) + 2.0 * self.fRec48[1] * (1.0 - 1.0 / mydsp_faustpower2_f(fTemp51))) / fTemp53;
			let mut fTemp56: F32 = 0.5 * ((self.fRec48[2] + self.fRec48[0] + 2.0 * self.fRec48[1]) * F32::max(0.0, F32::min(fTemp54, 2.0 - fTemp54)) / fTemp53);
			let mut fTemp57: F32 = fTemp56 + self.fVec2[1] + fTemp46;
			self.fVec4[0] = fTemp57;
			self.fRec41[(self.IOTA0 & 2047) as usize] = 0.95 * self.fVec4[2] + 0.05 * self.fRec41[((i32::wrapping_sub(self.IOTA0, 1)) & 2047) as usize];
			let mut fRec36: F32 = fTemp50 * self.fRec41[((i32::wrapping_sub(self.IOTA0, iTemp41)) & 2047) as usize] + fTemp36 * (fTemp49 * self.fRec41[((i32::wrapping_sub(self.IOTA0, iTemp34)) & 2047) as usize] + 0.5 * fTemp48 * self.fRec41[((i32::wrapping_sub(self.IOTA0, iTemp29)) & 2047) as usize] + 0.16666667 * fTemp47 * self.fRec41[((i32::wrapping_sub(self.IOTA0, iTemp25)) & 2047) as usize] + 0.041666668 * fTemp22 * self.fRec41[((i32::wrapping_sub(self.IOTA0, iTemp15)) & 2047) as usize]);
			let mut fRec37: F32 = self.fVec4[1] + self.fRec30[1];
			self.fRec30[0] = fRec35;
			let mut fRec31: F32 = self.fRec30[1];
			let mut fRec32: F32 = fRec36;
			let mut fRec33: F32 = fRec37;
			self.fRec26[0] = fRec31;
			let mut fRec27: F32 = fTemp46 + fTemp56 + self.fRec26[1];
			let mut fRec28: F32 = fRec32;
			let mut fRec29: F32 = fRec33;
			self.fRec22[(self.IOTA0 & 2047) as usize] = fRec27;
			let mut fRec23: F32 = fTemp50 * self.fRec22[((i32::wrapping_sub(self.IOTA0, iTemp42)) & 2047) as usize] + fTemp36 * (fTemp49 * self.fRec22[((i32::wrapping_sub(self.IOTA0, iTemp35)) & 2047) as usize] + 0.5 * fTemp48 * self.fRec22[((i32::wrapping_sub(self.IOTA0, iTemp30)) & 2047) as usize] + 0.16666667 * fTemp47 * self.fRec22[((i32::wrapping_sub(self.IOTA0, iTemp26)) & 2047) as usize] + 0.041666668 * fTemp22 * self.fRec22[((i32::wrapping_sub(self.IOTA0, iTemp16)) & 2047) as usize]);
			self.fRec24[0] = fRec28;
			let mut fRec25: F32 = fRec29;
			self.fRec20[0] = fSlow5 * self.fRec24[1];
			let mut fRec21: F32 = fRec25;
			self.fRec15[0] = fRec19;
			let mut fRec16: F32 = fSlow5 * self.fRec15[1];
			let mut fRec17: F32 = self.fRec20[0];
			let mut fRec18: F32 = fRec21;
			self.fRec11[(self.IOTA0 & 2047) as usize] = fRec16;
			let mut fRec12: F32 = fRec23;
			let mut fRec13: F32 = fRec17;
			let mut fRec14: F32 = fRec18;
			self.fRec9[0] = fRec12;
			let mut fRec10: F32 = fRec14;
			let mut fTemp58: F32 = F32::abs(fRec10);
			let mut fTemp59: F32 = if (fTemp58 > self.fRec8[1]) as i32 != 0 {0.0} else {self.fConst12};
			self.fRec8[0] = fTemp58 * (1.0 - fTemp59) + self.fRec8[1] * fTemp59;
			let mut fRec7: F32 = 0.0 - 0.95 * F32::max(2e+01 * F32::log10(F32::max(1.1754944e-38, self.fRec8[0])) + 1e+01, 0.0);
			self.fRec6[0] = 1.5 * fRec10 * F32::powf(1e+01, 0.05 * fRec7);
			self.fRec5[0] = self.fRec6[0] - (self.fRec5[2] * (fTemp10 + (1.0 - (fTemp9 - fTemp4) / fTemp3)) + 2.0 * self.fRec5[1] * (fTemp10 + (1.0 - fTemp8))) / fTemp11;
			self.fRec4[0] = (self.fRec5[2] + self.fRec5[0] + 2.0 * self.fRec5[1]) / fTemp11 - (self.fRec4[2] * (fTemp6 + (fTemp4 - fTemp5) / fTemp3 + fTemp2) + 2.0 * self.fRec4[1] * (fTemp6 + (1.0 - (fTemp1 + fTemp8)))) / fTemp7;
			self.fRec50[0] = self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow6 + self.fRec3[0]) + 0.5) as i32, 2047))) as usize] } + self.fConst2 * self.fRec50[1];
			let mut fTemp60: F32 = F32::tan(self.fConst3 * F32::max(2e+01, F32::min(1e+04, self.fRec50[0])));
			let mut fTemp61: F32 = 1.0 / fTemp60;
			let mut fTemp62: F32 = fTemp6 + (fTemp5 + fTemp61) / fTemp60 + fTemp2;
			let mut fTemp63: F32 = 1.0 / mydsp_faustpower2_f(fTemp60);
			let mut fTemp64: F32 = fTemp10 + (fTemp9 + fTemp61) / fTemp60 + 1.0;
			let mut fRec66: F32 = -1.0 * 0.9973053 * (0.9 * self.fRec67[2] + 0.05 * (self.fRec67[1] + self.fRec67[3]));
			self.fRec85[0] = self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow6 + self.fRec39[0]) + 0.5) as i32, 2047))) as usize] } + self.fConst2 * self.fRec85[1];
			let mut fTemp65: F32 = self.fConst5 * (3.4e+02 / self.fRec85[0] + -0.11);
			let mut fTemp66: F32 = fTemp65 + -1.499995;
			let mut iTemp67: i32 = (fTemp66) as i32;
			let mut iTemp68: i32 = (F32::min(self.fConst4, (std::cmp::max(0, i32::wrapping_add(iTemp67, 4))) as F32)) as i32;
			let mut iTemp69: i32 = i32::wrapping_add(iTemp68, 1);
			let mut fTemp70: F32 = F32::floor(fTemp66);
			let mut fTemp71: F32 = fTemp65 + (-3.0 - fTemp70);
			let mut fTemp72: F32 = fTemp65 + (-2.0 - fTemp70);
			let mut fTemp73: F32 = fTemp65 + (-1.0 - fTemp70);
			let mut fTemp74: F32 = fTemp73 * fTemp72;
			let mut fTemp75: F32 = fTemp74 * fTemp71;
			let mut fTemp76: F32 = fTemp65 + (-4.0 - fTemp70);
			let mut fTemp77: F32 = 0.0 - fTemp76;
			let mut iTemp78: i32 = (F32::min(self.fConst4, (std::cmp::max(0, i32::wrapping_add(iTemp67, 3))) as F32)) as i32;
			let mut iTemp79: i32 = i32::wrapping_add(iTemp78, 1);
			let mut fTemp80: F32 = 0.0 - 0.5 * fTemp76;
			let mut fTemp81: F32 = 0.0 - fTemp71;
			let mut iTemp82: i32 = (F32::min(self.fConst4, (std::cmp::max(0, i32::wrapping_add(iTemp67, 2))) as F32)) as i32;
			let mut iTemp83: i32 = i32::wrapping_add(iTemp82, 1);
			let mut fTemp84: F32 = 0.0 - 0.33333334 * fTemp76;
			let mut fTemp85: F32 = 0.0 - 0.5 * fTemp71;
			let mut fTemp86: F32 = 0.0 - fTemp72;
			let mut iTemp87: i32 = (F32::min(self.fConst4, (std::cmp::max(0, i32::wrapping_add(iTemp67, 1))) as F32)) as i32;
			let mut iTemp88: i32 = i32::wrapping_add(iTemp87, 1);
			let mut fTemp89: F32 = fTemp65 - fTemp70;
			let mut fTemp90: F32 = 0.0 - 0.25 * fTemp76;
			let mut fTemp91: F32 = 0.0 - 0.33333334 * fTemp71;
			let mut fTemp92: F32 = 0.0 - 0.5 * fTemp72;
			let mut fTemp93: F32 = 0.0 - fTemp73;
			let mut iTemp94: i32 = (F32::min(self.fConst4, (std::cmp::max(0, iTemp67)) as F32)) as i32;
			let mut iTemp95: i32 = i32::wrapping_add(iTemp94, 1);
			self.fRec81[0] = self.fRec58[((i32::wrapping_sub(self.IOTA0, iTemp95)) & 2047) as usize] * fTemp93 * fTemp92 * fTemp91 * fTemp90 + fTemp89 * (self.fRec58[((i32::wrapping_sub(self.IOTA0, iTemp88)) & 2047) as usize] * fTemp86 * fTemp85 * fTemp84 + 0.5 * fTemp73 * self.fRec58[((i32::wrapping_sub(self.IOTA0, iTemp83)) & 2047) as usize] * fTemp81 * fTemp80 + 0.16666667 * fTemp74 * self.fRec58[((i32::wrapping_sub(self.IOTA0, iTemp79)) & 2047) as usize] * fTemp77 + 0.041666668 * fTemp75 * self.fRec58[((i32::wrapping_sub(self.IOTA0, iTemp69)) & 2047) as usize]);
			self.fRec86[0] = 0.95 * self.fRec81[1] + 0.05 * self.fRec86[1];
			let mut fRec82: F32 = self.fRec86[0];
			self.fRec89[0] = self.fConst7 * F32::abs(self.fRec53[1]) + self.fConst6 * self.fRec89[1];
			let mut iTemp96: i32 = (self.fRec89[0] > 0.1) as i32;
			self.iVec5[0] = iTemp96;
			self.iRec90[0] = std::cmp::max(i32::wrapping_mul(self.iConst8, (iTemp96 < self.iVec5[1]) as i32), i32::wrapping_add(self.iRec90[1], -1));
			let mut fTemp97: F32 = F32::abs(F32::max((iTemp96) as F32, ((self.iRec90[0] > 0) as i32) as u32 as F32));
			let mut fTemp98: F32 = if (fTemp97 > self.fRec88[1]) as i32 != 0 {self.fConst6} else {self.fConst9};
			self.fRec88[0] = fTemp97 * (1.0 - fTemp98) + self.fRec88[1] * fTemp98;
			let mut fTemp99: F32 = 0.005 * self.fRec88[0] * self.fRec53[1];
			self.fRec91[0] = self.fRec56[1];
			self.fRec92[(self.IOTA0 & 2047) as usize] = -1.0 * 0.9973053 * (0.9 * self.fRec91[2] + 0.05 * (self.fRec91[1] + self.fRec91[3]));
			let mut fTemp100: F32 = fTemp74 * fTemp77;
			let mut fTemp101: F32 = fTemp73 * fTemp81 * fTemp80;
			let mut fTemp102: F32 = fTemp86 * fTemp85 * fTemp84;
			let mut fTemp103: F32 = fTemp93 * fTemp92 * fTemp91 * fTemp90;
			self.fVec6[0] = fTemp103 * self.fRec92[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp94, 2))) & 2047) as usize] + fTemp89 * (fTemp102 * self.fRec92[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp87, 2))) & 2047) as usize] + 0.5 * fTemp101 * self.fRec92[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp82, 2))) & 2047) as usize] + 0.16666667 * fTemp100 * self.fRec92[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp78, 2))) & 2047) as usize] + 0.041666668 * fTemp75 * self.fRec92[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp68, 2))) & 2047) as usize]);
			let mut fTemp104: F32 = F32::tan(self.fConst10 * self.fRec85[0]);
			let mut fTemp105: F32 = 1.0 / fTemp104;
			let mut fTemp106: F32 = (fTemp105 + 1.4142135) / fTemp104 + 1.0;
			self.fVec7[0] = fSlow7;
			self.iRec93[0] = i32::wrapping_add(i32::wrapping_mul(i32::wrapping_add(self.iRec93[1], (self.iRec93[1] > 0) as i32), (fSlow7 <= self.fVec7[1]) as i32), (fSlow7 > self.fVec7[1]) as i32);
			let mut fTemp107: F32 = (self.iRec93[0]) as F32 / F32::max(1.0, self.fConst11 * mydsp_faustpower2_f(1.0 - 0.0005 * self.fRec85[0]));
			self.fRec94[0] = fTemp55 - (self.fRec94[2] * ((fTemp105 + -1.4142135) / fTemp104 + 1.0) + 2.0 * self.fRec94[1] * (1.0 - 1.0 / mydsp_faustpower2_f(fTemp104))) / fTemp106;
			let mut fTemp108: F32 = 0.5 * ((self.fRec94[2] + self.fRec94[0] + 2.0 * self.fRec94[1]) * F32::max(0.0, F32::min(fTemp107, 2.0 - fTemp107)) / fTemp106);
			let mut fTemp109: F32 = fTemp108 + self.fVec6[1] + fTemp99;
			self.fVec8[0] = fTemp109;
			self.fRec87[(self.IOTA0 & 2047) as usize] = 0.95 * self.fVec8[2] + 0.05 * self.fRec87[((i32::wrapping_sub(self.IOTA0, 1)) & 2047) as usize];
			let mut fRec83: F32 = fTemp103 * self.fRec87[((i32::wrapping_sub(self.IOTA0, iTemp94)) & 2047) as usize] + fTemp89 * (fTemp102 * self.fRec87[((i32::wrapping_sub(self.IOTA0, iTemp87)) & 2047) as usize] + 0.5 * fTemp101 * self.fRec87[((i32::wrapping_sub(self.IOTA0, iTemp82)) & 2047) as usize] + 0.16666667 * fTemp100 * self.fRec87[((i32::wrapping_sub(self.IOTA0, iTemp78)) & 2047) as usize] + 0.041666668 * fTemp75 * self.fRec87[((i32::wrapping_sub(self.IOTA0, iTemp68)) & 2047) as usize]);
			let mut fRec84: F32 = self.fVec8[1] + self.fRec77[1];
			self.fRec77[0] = fRec82;
			let mut fRec78: F32 = self.fRec77[1];
			let mut fRec79: F32 = fRec83;
			let mut fRec80: F32 = fRec84;
			self.fRec73[0] = fRec78;
			let mut fRec74: F32 = fTemp99 + fTemp108 + self.fRec73[1];
			let mut fRec75: F32 = fRec79;
			let mut fRec76: F32 = fRec80;
			self.fRec69[(self.IOTA0 & 2047) as usize] = fRec74;
			let mut fRec70: F32 = fTemp103 * self.fRec69[((i32::wrapping_sub(self.IOTA0, iTemp95)) & 2047) as usize] + fTemp89 * (fTemp102 * self.fRec69[((i32::wrapping_sub(self.IOTA0, iTemp88)) & 2047) as usize] + 0.5 * fTemp101 * self.fRec69[((i32::wrapping_sub(self.IOTA0, iTemp83)) & 2047) as usize] + 0.16666667 * fTemp100 * self.fRec69[((i32::wrapping_sub(self.IOTA0, iTemp79)) & 2047) as usize] + 0.041666668 * fTemp75 * self.fRec69[((i32::wrapping_sub(self.IOTA0, iTemp69)) & 2047) as usize]);
			self.fRec71[0] = fRec75;
			let mut fRec72: F32 = fRec76;
			self.fRec67[0] = fSlow5 * self.fRec71[1];
			let mut fRec68: F32 = fRec72;
			self.fRec62[0] = fRec66;
			let mut fRec63: F32 = fSlow5 * self.fRec62[1];
			let mut fRec64: F32 = self.fRec67[0];
			let mut fRec65: F32 = fRec68;
			self.fRec58[(self.IOTA0 & 2047) as usize] = fRec63;
			let mut fRec59: F32 = fRec70;
			let mut fRec60: F32 = fRec64;
			let mut fRec61: F32 = fRec65;
			self.fRec56[0] = fRec59;
			let mut fRec57: F32 = fRec61;
			let mut fTemp110: F32 = F32::abs(fRec57);
			let mut fTemp111: F32 = if (fTemp110 > self.fRec55[1]) as i32 != 0 {0.0} else {self.fConst12};
			self.fRec55[0] = fTemp110 * (1.0 - fTemp111) + self.fRec55[1] * fTemp111;
			let mut fRec54: F32 = 0.0 - 0.95 * F32::max(2e+01 * F32::log10(F32::max(1.1754944e-38, self.fRec55[0])) + 1e+01, 0.0);
			self.fRec53[0] = 1.5 * fRec57 * F32::powf(1e+01, 0.05 * fRec54);
			self.fRec52[0] = self.fRec53[0] - (self.fRec52[2] * (fTemp10 + (fTemp61 - fTemp9) / fTemp60 + 1.0) + 2.0 * self.fRec52[1] * (fTemp10 + (1.0 - fTemp63))) / fTemp64;
			self.fRec51[0] = (self.fRec52[2] + self.fRec52[0] + 2.0 * self.fRec52[1]) / fTemp64 - (self.fRec51[2] * (fTemp6 + (fTemp61 - fTemp5) / fTemp60 + fTemp2) + 2.0 * self.fRec51[1] * (fTemp6 + (1.0 - (fTemp1 + fTemp63)))) / fTemp62;
			self.fRec95[0] = self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow8 + self.fRec3[0]) + 0.5) as i32, 2047))) as usize] } + self.fConst2 * self.fRec95[1];
			let mut fTemp112: F32 = F32::tan(self.fConst3 * F32::max(2e+01, F32::min(1e+04, self.fRec95[0])));
			let mut fTemp113: F32 = 1.0 / fTemp112;
			let mut fTemp114: F32 = fTemp6 + (fTemp5 + fTemp113) / fTemp112 + fTemp2;
			let mut fTemp115: F32 = 1.0 / mydsp_faustpower2_f(fTemp112);
			let mut fTemp116: F32 = fTemp10 + (fTemp9 + fTemp113) / fTemp112 + 1.0;
			let mut fRec111: F32 = -1.0 * 0.9973053 * (0.9 * self.fRec112[2] + 0.05 * (self.fRec112[1] + self.fRec112[3]));
			self.fRec130[0] = self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow8 + self.fRec39[0]) + 0.5) as i32, 2047))) as usize] } + self.fConst2 * self.fRec130[1];
			let mut fTemp117: F32 = self.fConst5 * (3.4e+02 / self.fRec130[0] + -0.11);
			let mut fTemp118: F32 = fTemp117 + -1.499995;
			let mut iTemp119: i32 = (fTemp118) as i32;
			let mut iTemp120: i32 = (F32::min(self.fConst4, (std::cmp::max(0, i32::wrapping_add(iTemp119, 4))) as F32)) as i32;
			let mut iTemp121: i32 = i32::wrapping_add(iTemp120, 1);
			let mut fTemp122: F32 = F32::floor(fTemp118);
			let mut fTemp123: F32 = fTemp117 + (-3.0 - fTemp122);
			let mut fTemp124: F32 = fTemp117 + (-2.0 - fTemp122);
			let mut fTemp125: F32 = fTemp117 + (-1.0 - fTemp122);
			let mut fTemp126: F32 = fTemp125 * fTemp124;
			let mut fTemp127: F32 = fTemp126 * fTemp123;
			let mut fTemp128: F32 = fTemp117 + (-4.0 - fTemp122);
			let mut fTemp129: F32 = 0.0 - fTemp128;
			let mut iTemp130: i32 = (F32::min(self.fConst4, (std::cmp::max(0, i32::wrapping_add(iTemp119, 3))) as F32)) as i32;
			let mut iTemp131: i32 = i32::wrapping_add(iTemp130, 1);
			let mut fTemp132: F32 = 0.0 - 0.5 * fTemp128;
			let mut fTemp133: F32 = 0.0 - fTemp123;
			let mut iTemp134: i32 = (F32::min(self.fConst4, (std::cmp::max(0, i32::wrapping_add(iTemp119, 2))) as F32)) as i32;
			let mut iTemp135: i32 = i32::wrapping_add(iTemp134, 1);
			let mut fTemp136: F32 = 0.0 - 0.33333334 * fTemp128;
			let mut fTemp137: F32 = 0.0 - 0.5 * fTemp123;
			let mut fTemp138: F32 = 0.0 - fTemp124;
			let mut iTemp139: i32 = (F32::min(self.fConst4, (std::cmp::max(0, i32::wrapping_add(iTemp119, 1))) as F32)) as i32;
			let mut iTemp140: i32 = i32::wrapping_add(iTemp139, 1);
			let mut fTemp141: F32 = fTemp117 - fTemp122;
			let mut fTemp142: F32 = 0.0 - 0.25 * fTemp128;
			let mut fTemp143: F32 = 0.0 - 0.33333334 * fTemp123;
			let mut fTemp144: F32 = 0.0 - 0.5 * fTemp124;
			let mut fTemp145: F32 = 0.0 - fTemp125;
			let mut iTemp146: i32 = (F32::min(self.fConst4, (std::cmp::max(0, iTemp119)) as F32)) as i32;
			let mut iTemp147: i32 = i32::wrapping_add(iTemp146, 1);
			self.fRec126[0] = self.fRec103[((i32::wrapping_sub(self.IOTA0, iTemp147)) & 2047) as usize] * fTemp145 * fTemp144 * fTemp143 * fTemp142 + fTemp141 * (self.fRec103[((i32::wrapping_sub(self.IOTA0, iTemp140)) & 2047) as usize] * fTemp138 * fTemp137 * fTemp136 + 0.5 * fTemp125 * self.fRec103[((i32::wrapping_sub(self.IOTA0, iTemp135)) & 2047) as usize] * fTemp133 * fTemp132 + 0.16666667 * fTemp126 * self.fRec103[((i32::wrapping_sub(self.IOTA0, iTemp131)) & 2047) as usize] * fTemp129 + 0.041666668 * fTemp127 * self.fRec103[((i32::wrapping_sub(self.IOTA0, iTemp121)) & 2047) as usize]);
			self.fRec131[0] = 0.95 * self.fRec126[1] + 0.05 * self.fRec131[1];
			let mut fRec127: F32 = self.fRec131[0];
			self.fRec134[0] = self.fConst7 * F32::abs(self.fRec98[1]) + self.fConst6 * self.fRec134[1];
			let mut iTemp148: i32 = (self.fRec134[0] > 0.1) as i32;
			self.iVec9[0] = iTemp148;
			self.iRec135[0] = std::cmp::max(i32::wrapping_mul(self.iConst8, (iTemp148 < self.iVec9[1]) as i32), i32::wrapping_add(self.iRec135[1], -1));
			let mut fTemp149: F32 = F32::abs(F32::max((iTemp148) as F32, ((self.iRec135[0] > 0) as i32) as u32 as F32));
			let mut fTemp150: F32 = if (fTemp149 > self.fRec133[1]) as i32 != 0 {self.fConst6} else {self.fConst9};
			self.fRec133[0] = fTemp149 * (1.0 - fTemp150) + self.fRec133[1] * fTemp150;
			let mut fTemp151: F32 = 0.005 * self.fRec133[0] * self.fRec98[1];
			self.fRec136[0] = self.fRec101[1];
			self.fRec137[(self.IOTA0 & 2047) as usize] = -1.0 * 0.9973053 * (0.9 * self.fRec136[2] + 0.05 * (self.fRec136[1] + self.fRec136[3]));
			let mut fTemp152: F32 = fTemp126 * fTemp129;
			let mut fTemp153: F32 = fTemp125 * fTemp133 * fTemp132;
			let mut fTemp154: F32 = fTemp138 * fTemp137 * fTemp136;
			let mut fTemp155: F32 = fTemp145 * fTemp144 * fTemp143 * fTemp142;
			self.fVec10[0] = fTemp155 * self.fRec137[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp146, 2))) & 2047) as usize] + fTemp141 * (fTemp154 * self.fRec137[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp139, 2))) & 2047) as usize] + 0.5 * fTemp153 * self.fRec137[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp134, 2))) & 2047) as usize] + 0.16666667 * fTemp152 * self.fRec137[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp130, 2))) & 2047) as usize] + 0.041666668 * fTemp127 * self.fRec137[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp120, 2))) & 2047) as usize]);
			let mut fTemp156: F32 = F32::tan(self.fConst10 * self.fRec130[0]);
			let mut fTemp157: F32 = 1.0 / fTemp156;
			let mut fTemp158: F32 = (fTemp157 + 1.4142135) / fTemp156 + 1.0;
			self.fVec11[0] = fSlow9;
			self.iRec138[0] = i32::wrapping_add(i32::wrapping_mul(i32::wrapping_add(self.iRec138[1], (self.iRec138[1] > 0) as i32), (fSlow9 <= self.fVec11[1]) as i32), (fSlow9 > self.fVec11[1]) as i32);
			let mut fTemp159: F32 = (self.iRec138[0]) as F32 / F32::max(1.0, self.fConst11 * mydsp_faustpower2_f(1.0 - 0.0005 * self.fRec130[0]));
			self.fRec139[0] = fTemp55 - (self.fRec139[2] * ((fTemp157 + -1.4142135) / fTemp156 + 1.0) + 2.0 * self.fRec139[1] * (1.0 - 1.0 / mydsp_faustpower2_f(fTemp156))) / fTemp158;
			let mut fTemp160: F32 = 0.5 * ((self.fRec139[2] + self.fRec139[0] + 2.0 * self.fRec139[1]) * F32::max(0.0, F32::min(fTemp159, 2.0 - fTemp159)) / fTemp158);
			let mut fTemp161: F32 = fTemp160 + self.fVec10[1] + fTemp151;
			self.fVec12[0] = fTemp161;
			self.fRec132[(self.IOTA0 & 2047) as usize] = 0.95 * self.fVec12[2] + 0.05 * self.fRec132[((i32::wrapping_sub(self.IOTA0, 1)) & 2047) as usize];
			let mut fRec128: F32 = fTemp155 * self.fRec132[((i32::wrapping_sub(self.IOTA0, iTemp146)) & 2047) as usize] + fTemp141 * (fTemp154 * self.fRec132[((i32::wrapping_sub(self.IOTA0, iTemp139)) & 2047) as usize] + 0.5 * fTemp153 * self.fRec132[((i32::wrapping_sub(self.IOTA0, iTemp134)) & 2047) as usize] + 0.16666667 * fTemp152 * self.fRec132[((i32::wrapping_sub(self.IOTA0, iTemp130)) & 2047) as usize] + 0.041666668 * fTemp127 * self.fRec132[((i32::wrapping_sub(self.IOTA0, iTemp120)) & 2047) as usize]);
			let mut fRec129: F32 = self.fVec12[1] + self.fRec122[1];
			self.fRec122[0] = fRec127;
			let mut fRec123: F32 = self.fRec122[1];
			let mut fRec124: F32 = fRec128;
			let mut fRec125: F32 = fRec129;
			self.fRec118[0] = fRec123;
			let mut fRec119: F32 = fTemp151 + fTemp160 + self.fRec118[1];
			let mut fRec120: F32 = fRec124;
			let mut fRec121: F32 = fRec125;
			self.fRec114[(self.IOTA0 & 2047) as usize] = fRec119;
			let mut fRec115: F32 = fTemp155 * self.fRec114[((i32::wrapping_sub(self.IOTA0, iTemp147)) & 2047) as usize] + fTemp141 * (fTemp154 * self.fRec114[((i32::wrapping_sub(self.IOTA0, iTemp140)) & 2047) as usize] + 0.5 * fTemp153 * self.fRec114[((i32::wrapping_sub(self.IOTA0, iTemp135)) & 2047) as usize] + 0.16666667 * fTemp152 * self.fRec114[((i32::wrapping_sub(self.IOTA0, iTemp131)) & 2047) as usize] + 0.041666668 * fTemp127 * self.fRec114[((i32::wrapping_sub(self.IOTA0, iTemp121)) & 2047) as usize]);
			self.fRec116[0] = fRec120;
			let mut fRec117: F32 = fRec121;
			self.fRec112[0] = fSlow5 * self.fRec116[1];
			let mut fRec113: F32 = fRec117;
			self.fRec107[0] = fRec111;
			let mut fRec108: F32 = fSlow5 * self.fRec107[1];
			let mut fRec109: F32 = self.fRec112[0];
			let mut fRec110: F32 = fRec113;
			self.fRec103[(self.IOTA0 & 2047) as usize] = fRec108;
			let mut fRec104: F32 = fRec115;
			let mut fRec105: F32 = fRec109;
			let mut fRec106: F32 = fRec110;
			self.fRec101[0] = fRec104;
			let mut fRec102: F32 = fRec106;
			let mut fTemp162: F32 = F32::abs(fRec102);
			let mut fTemp163: F32 = if (fTemp162 > self.fRec100[1]) as i32 != 0 {0.0} else {self.fConst12};
			self.fRec100[0] = fTemp162 * (1.0 - fTemp163) + self.fRec100[1] * fTemp163;
			let mut fRec99: F32 = 0.0 - 0.95 * F32::max(2e+01 * F32::log10(F32::max(1.1754944e-38, self.fRec100[0])) + 1e+01, 0.0);
			self.fRec98[0] = 1.5 * fRec102 * F32::powf(1e+01, 0.05 * fRec99);
			self.fRec97[0] = self.fRec98[0] - (self.fRec97[2] * (fTemp10 + (fTemp113 - fTemp9) / fTemp112 + 1.0) + 2.0 * self.fRec97[1] * (fTemp10 + (1.0 - fTemp115))) / fTemp116;
			self.fRec96[0] = (self.fRec97[2] + self.fRec97[0] + 2.0 * self.fRec97[1]) / fTemp116 - (self.fRec96[2] * (fTemp6 + (fTemp113 - fTemp5) / fTemp112 + fTemp2) + 2.0 * self.fRec96[1] * (fTemp6 + (1.0 - (fTemp1 + fTemp115)))) / fTemp114;
			self.fRec140[0] = self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow10 + self.fRec3[0]) + 0.5) as i32, 2047))) as usize] } + self.fConst2 * self.fRec140[1];
			let mut fTemp164: F32 = F32::tan(self.fConst3 * F32::max(2e+01, F32::min(1e+04, self.fRec140[0])));
			let mut fTemp165: F32 = 1.0 / fTemp164;
			let mut fTemp166: F32 = fTemp6 + (fTemp5 + fTemp165) / fTemp164 + fTemp2;
			let mut fTemp167: F32 = 1.0 / mydsp_faustpower2_f(fTemp164);
			let mut fTemp168: F32 = fTemp10 + (fTemp9 + fTemp165) / fTemp164 + 1.0;
			let mut fRec156: F32 = -1.0 * 0.9973053 * (0.9 * self.fRec157[2] + 0.05 * (self.fRec157[1] + self.fRec157[3]));
			self.fRec175[0] = self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow10 + self.fRec39[0]) + 0.5) as i32, 2047))) as usize] } + self.fConst2 * self.fRec175[1];
			let mut fTemp169: F32 = self.fConst5 * (3.4e+02 / self.fRec175[0] + -0.11);
			let mut fTemp170: F32 = fTemp169 + -1.499995;
			let mut iTemp171: i32 = (fTemp170) as i32;
			let mut iTemp172: i32 = (F32::min(self.fConst4, (std::cmp::max(0, i32::wrapping_add(iTemp171, 4))) as F32)) as i32;
			let mut iTemp173: i32 = i32::wrapping_add(iTemp172, 1);
			let mut fTemp174: F32 = F32::floor(fTemp170);
			let mut fTemp175: F32 = fTemp169 + (-3.0 - fTemp174);
			let mut fTemp176: F32 = fTemp169 + (-2.0 - fTemp174);
			let mut fTemp177: F32 = fTemp169 + (-1.0 - fTemp174);
			let mut fTemp178: F32 = fTemp177 * fTemp176;
			let mut fTemp179: F32 = fTemp178 * fTemp175;
			let mut fTemp180: F32 = fTemp169 + (-4.0 - fTemp174);
			let mut fTemp181: F32 = 0.0 - fTemp180;
			let mut iTemp182: i32 = (F32::min(self.fConst4, (std::cmp::max(0, i32::wrapping_add(iTemp171, 3))) as F32)) as i32;
			let mut iTemp183: i32 = i32::wrapping_add(iTemp182, 1);
			let mut fTemp184: F32 = 0.0 - 0.5 * fTemp180;
			let mut fTemp185: F32 = 0.0 - fTemp175;
			let mut iTemp186: i32 = (F32::min(self.fConst4, (std::cmp::max(0, i32::wrapping_add(iTemp171, 2))) as F32)) as i32;
			let mut iTemp187: i32 = i32::wrapping_add(iTemp186, 1);
			let mut fTemp188: F32 = 0.0 - 0.33333334 * fTemp180;
			let mut fTemp189: F32 = 0.0 - 0.5 * fTemp175;
			let mut fTemp190: F32 = 0.0 - fTemp176;
			let mut iTemp191: i32 = (F32::min(self.fConst4, (std::cmp::max(0, i32::wrapping_add(iTemp171, 1))) as F32)) as i32;
			let mut iTemp192: i32 = i32::wrapping_add(iTemp191, 1);
			let mut fTemp193: F32 = fTemp169 - fTemp174;
			let mut fTemp194: F32 = 0.0 - 0.25 * fTemp180;
			let mut fTemp195: F32 = 0.0 - 0.33333334 * fTemp175;
			let mut fTemp196: F32 = 0.0 - 0.5 * fTemp176;
			let mut fTemp197: F32 = 0.0 - fTemp177;
			let mut iTemp198: i32 = (F32::min(self.fConst4, (std::cmp::max(0, iTemp171)) as F32)) as i32;
			let mut iTemp199: i32 = i32::wrapping_add(iTemp198, 1);
			self.fRec171[0] = self.fRec148[((i32::wrapping_sub(self.IOTA0, iTemp199)) & 2047) as usize] * fTemp197 * fTemp196 * fTemp195 * fTemp194 + fTemp193 * (self.fRec148[((i32::wrapping_sub(self.IOTA0, iTemp192)) & 2047) as usize] * fTemp190 * fTemp189 * fTemp188 + 0.5 * fTemp177 * self.fRec148[((i32::wrapping_sub(self.IOTA0, iTemp187)) & 2047) as usize] * fTemp185 * fTemp184 + 0.16666667 * fTemp178 * self.fRec148[((i32::wrapping_sub(self.IOTA0, iTemp183)) & 2047) as usize] * fTemp181 + 0.041666668 * fTemp179 * self.fRec148[((i32::wrapping_sub(self.IOTA0, iTemp173)) & 2047) as usize]);
			self.fRec176[0] = 0.95 * self.fRec171[1] + 0.05 * self.fRec176[1];
			let mut fRec172: F32 = self.fRec176[0];
			self.fRec179[0] = self.fConst7 * F32::abs(self.fRec143[1]) + self.fConst6 * self.fRec179[1];
			let mut iTemp200: i32 = (self.fRec179[0] > 0.1) as i32;
			self.iVec13[0] = iTemp200;
			self.iRec180[0] = std::cmp::max(i32::wrapping_mul(self.iConst8, (iTemp200 < self.iVec13[1]) as i32), i32::wrapping_add(self.iRec180[1], -1));
			let mut fTemp201: F32 = F32::abs(F32::max((iTemp200) as F32, ((self.iRec180[0] > 0) as i32) as u32 as F32));
			let mut fTemp202: F32 = if (fTemp201 > self.fRec178[1]) as i32 != 0 {self.fConst6} else {self.fConst9};
			self.fRec178[0] = fTemp201 * (1.0 - fTemp202) + self.fRec178[1] * fTemp202;
			let mut fTemp203: F32 = 0.005 * self.fRec178[0] * self.fRec143[1];
			self.fRec181[0] = self.fRec146[1];
			self.fRec182[(self.IOTA0 & 2047) as usize] = -1.0 * 0.9973053 * (0.9 * self.fRec181[2] + 0.05 * (self.fRec181[1] + self.fRec181[3]));
			let mut fTemp204: F32 = fTemp178 * fTemp181;
			let mut fTemp205: F32 = fTemp177 * fTemp185 * fTemp184;
			let mut fTemp206: F32 = fTemp190 * fTemp189 * fTemp188;
			let mut fTemp207: F32 = fTemp197 * fTemp196 * fTemp195 * fTemp194;
			self.fVec14[0] = fTemp207 * self.fRec182[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp198, 2))) & 2047) as usize] + fTemp193 * (fTemp206 * self.fRec182[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp191, 2))) & 2047) as usize] + 0.5 * fTemp205 * self.fRec182[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp186, 2))) & 2047) as usize] + 0.16666667 * fTemp204 * self.fRec182[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp182, 2))) & 2047) as usize] + 0.041666668 * fTemp179 * self.fRec182[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp172, 2))) & 2047) as usize]);
			let mut fTemp208: F32 = F32::tan(self.fConst10 * self.fRec175[0]);
			let mut fTemp209: F32 = 1.0 / fTemp208;
			let mut fTemp210: F32 = (fTemp209 + 1.4142135) / fTemp208 + 1.0;
			self.fVec15[0] = fSlow11;
			self.iRec183[0] = i32::wrapping_add(i32::wrapping_mul(i32::wrapping_add(self.iRec183[1], (self.iRec183[1] > 0) as i32), (fSlow11 <= self.fVec15[1]) as i32), (fSlow11 > self.fVec15[1]) as i32);
			let mut fTemp211: F32 = (self.iRec183[0]) as F32 / F32::max(1.0, self.fConst11 * mydsp_faustpower2_f(1.0 - 0.0005 * self.fRec175[0]));
			self.fRec184[0] = fTemp55 - (self.fRec184[2] * ((fTemp209 + -1.4142135) / fTemp208 + 1.0) + 2.0 * self.fRec184[1] * (1.0 - 1.0 / mydsp_faustpower2_f(fTemp208))) / fTemp210;
			let mut fTemp212: F32 = 0.5 * ((self.fRec184[2] + self.fRec184[0] + 2.0 * self.fRec184[1]) * F32::max(0.0, F32::min(fTemp211, 2.0 - fTemp211)) / fTemp210);
			let mut fTemp213: F32 = fTemp212 + self.fVec14[1] + fTemp203;
			self.fVec16[0] = fTemp213;
			self.fRec177[(self.IOTA0 & 2047) as usize] = 0.95 * self.fVec16[2] + 0.05 * self.fRec177[((i32::wrapping_sub(self.IOTA0, 1)) & 2047) as usize];
			let mut fRec173: F32 = fTemp207 * self.fRec177[((i32::wrapping_sub(self.IOTA0, iTemp198)) & 2047) as usize] + fTemp193 * (fTemp206 * self.fRec177[((i32::wrapping_sub(self.IOTA0, iTemp191)) & 2047) as usize] + 0.5 * fTemp205 * self.fRec177[((i32::wrapping_sub(self.IOTA0, iTemp186)) & 2047) as usize] + 0.16666667 * fTemp204 * self.fRec177[((i32::wrapping_sub(self.IOTA0, iTemp182)) & 2047) as usize] + 0.041666668 * fTemp179 * self.fRec177[((i32::wrapping_sub(self.IOTA0, iTemp172)) & 2047) as usize]);
			let mut fRec174: F32 = self.fVec16[1] + self.fRec167[1];
			self.fRec167[0] = fRec172;
			let mut fRec168: F32 = self.fRec167[1];
			let mut fRec169: F32 = fRec173;
			let mut fRec170: F32 = fRec174;
			self.fRec163[0] = fRec168;
			let mut fRec164: F32 = fTemp203 + fTemp212 + self.fRec163[1];
			let mut fRec165: F32 = fRec169;
			let mut fRec166: F32 = fRec170;
			self.fRec159[(self.IOTA0 & 2047) as usize] = fRec164;
			let mut fRec160: F32 = fTemp207 * self.fRec159[((i32::wrapping_sub(self.IOTA0, iTemp199)) & 2047) as usize] + fTemp193 * (fTemp206 * self.fRec159[((i32::wrapping_sub(self.IOTA0, iTemp192)) & 2047) as usize] + 0.5 * fTemp205 * self.fRec159[((i32::wrapping_sub(self.IOTA0, iTemp187)) & 2047) as usize] + 0.16666667 * fTemp204 * self.fRec159[((i32::wrapping_sub(self.IOTA0, iTemp183)) & 2047) as usize] + 0.041666668 * fTemp179 * self.fRec159[((i32::wrapping_sub(self.IOTA0, iTemp173)) & 2047) as usize]);
			self.fRec161[0] = fRec165;
			let mut fRec162: F32 = fRec166;
			self.fRec157[0] = fSlow5 * self.fRec161[1];
			let mut fRec158: F32 = fRec162;
			self.fRec152[0] = fRec156;
			let mut fRec153: F32 = fSlow5 * self.fRec152[1];
			let mut fRec154: F32 = self.fRec157[0];
			let mut fRec155: F32 = fRec158;
			self.fRec148[(self.IOTA0 & 2047) as usize] = fRec153;
			let mut fRec149: F32 = fRec160;
			let mut fRec150: F32 = fRec154;
			let mut fRec151: F32 = fRec155;
			self.fRec146[0] = fRec149;
			let mut fRec147: F32 = fRec151;
			let mut fTemp214: F32 = F32::abs(fRec147);
			let mut fTemp215: F32 = if (fTemp214 > self.fRec145[1]) as i32 != 0 {0.0} else {self.fConst12};
			self.fRec145[0] = fTemp214 * (1.0 - fTemp215) + self.fRec145[1] * fTemp215;
			let mut fRec144: F32 = 0.0 - 0.95 * F32::max(2e+01 * F32::log10(F32::max(1.1754944e-38, self.fRec145[0])) + 1e+01, 0.0);
			self.fRec143[0] = 1.5 * fRec147 * F32::powf(1e+01, 0.05 * fRec144);
			self.fRec142[0] = self.fRec143[0] - (self.fRec142[2] * (fTemp10 + (1.0 - (fTemp9 - fTemp165) / fTemp164)) + 2.0 * self.fRec142[1] * (fTemp10 + (1.0 - fTemp167))) / fTemp168;
			self.fRec141[0] = (self.fRec142[2] + self.fRec142[0] + 2.0 * self.fRec142[1]) / fTemp168 - (self.fRec141[2] * (fTemp6 + (fTemp165 - fTemp5) / fTemp164 + fTemp2) + 2.0 * self.fRec141[1] * (fTemp6 + (1.0 - (fTemp1 + fTemp167)))) / fTemp166;
			self.fRec185[0] = self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow12 + self.fRec3[0]) + 0.5) as i32, 2047))) as usize] } + self.fConst2 * self.fRec185[1];
			let mut fTemp216: F32 = F32::tan(self.fConst3 * F32::max(2e+01, F32::min(1e+04, self.fRec185[0])));
			let mut fTemp217: F32 = 1.0 / fTemp216;
			let mut fTemp218: F32 = fTemp6 + (fTemp5 + fTemp217) / fTemp216 + fTemp2;
			let mut fTemp219: F32 = 1.0 / mydsp_faustpower2_f(fTemp216);
			let mut fTemp220: F32 = fTemp10 + (fTemp9 + fTemp217) / fTemp216 + 1.0;
			let mut fRec201: F32 = -1.0 * 0.9973053 * (0.9 * self.fRec202[2] + 0.05 * (self.fRec202[1] + self.fRec202[3]));
			self.fRec220[0] = self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow12 + self.fRec39[0]) + 0.5) as i32, 2047))) as usize] } + self.fConst2 * self.fRec220[1];
			let mut fTemp221: F32 = self.fConst5 * (3.4e+02 / self.fRec220[0] + -0.11);
			let mut fTemp222: F32 = fTemp221 + -1.499995;
			let mut iTemp223: i32 = (fTemp222) as i32;
			let mut iTemp224: i32 = (F32::min(self.fConst4, (std::cmp::max(0, i32::wrapping_add(iTemp223, 4))) as F32)) as i32;
			let mut iTemp225: i32 = i32::wrapping_add(iTemp224, 1);
			let mut fTemp226: F32 = F32::floor(fTemp222);
			let mut fTemp227: F32 = fTemp221 + (-3.0 - fTemp226);
			let mut fTemp228: F32 = fTemp221 + (-2.0 - fTemp226);
			let mut fTemp229: F32 = fTemp221 + (-1.0 - fTemp226);
			let mut fTemp230: F32 = fTemp229 * fTemp228;
			let mut fTemp231: F32 = fTemp230 * fTemp227;
			let mut fTemp232: F32 = fTemp221 + (-4.0 - fTemp226);
			let mut fTemp233: F32 = 0.0 - fTemp232;
			let mut iTemp234: i32 = (F32::min(self.fConst4, (std::cmp::max(0, i32::wrapping_add(iTemp223, 3))) as F32)) as i32;
			let mut iTemp235: i32 = i32::wrapping_add(iTemp234, 1);
			let mut fTemp236: F32 = 0.0 - 0.5 * fTemp232;
			let mut fTemp237: F32 = 0.0 - fTemp227;
			let mut iTemp238: i32 = (F32::min(self.fConst4, (std::cmp::max(0, i32::wrapping_add(iTemp223, 2))) as F32)) as i32;
			let mut iTemp239: i32 = i32::wrapping_add(iTemp238, 1);
			let mut fTemp240: F32 = 0.0 - 0.33333334 * fTemp232;
			let mut fTemp241: F32 = 0.0 - 0.5 * fTemp227;
			let mut fTemp242: F32 = 0.0 - fTemp228;
			let mut iTemp243: i32 = (F32::min(self.fConst4, (std::cmp::max(0, i32::wrapping_add(iTemp223, 1))) as F32)) as i32;
			let mut iTemp244: i32 = i32::wrapping_add(iTemp243, 1);
			let mut fTemp245: F32 = fTemp221 - fTemp226;
			let mut fTemp246: F32 = 0.0 - 0.25 * fTemp232;
			let mut fTemp247: F32 = 0.0 - 0.33333334 * fTemp227;
			let mut fTemp248: F32 = 0.0 - 0.5 * fTemp228;
			let mut fTemp249: F32 = 0.0 - fTemp229;
			let mut iTemp250: i32 = (F32::min(self.fConst4, (std::cmp::max(0, iTemp223)) as F32)) as i32;
			let mut iTemp251: i32 = i32::wrapping_add(iTemp250, 1);
			self.fRec216[0] = self.fRec193[((i32::wrapping_sub(self.IOTA0, iTemp251)) & 2047) as usize] * fTemp249 * fTemp248 * fTemp247 * fTemp246 + fTemp245 * (self.fRec193[((i32::wrapping_sub(self.IOTA0, iTemp244)) & 2047) as usize] * fTemp242 * fTemp241 * fTemp240 + 0.5 * fTemp229 * self.fRec193[((i32::wrapping_sub(self.IOTA0, iTemp239)) & 2047) as usize] * fTemp237 * fTemp236 + 0.16666667 * fTemp230 * self.fRec193[((i32::wrapping_sub(self.IOTA0, iTemp235)) & 2047) as usize] * fTemp233 + 0.041666668 * fTemp231 * self.fRec193[((i32::wrapping_sub(self.IOTA0, iTemp225)) & 2047) as usize]);
			self.fRec221[0] = 0.95 * self.fRec216[1] + 0.05 * self.fRec221[1];
			let mut fRec217: F32 = self.fRec221[0];
			self.fRec224[0] = self.fConst7 * F32::abs(self.fRec188[1]) + self.fConst6 * self.fRec224[1];
			let mut iTemp252: i32 = (self.fRec224[0] > 0.1) as i32;
			self.iVec17[0] = iTemp252;
			self.iRec225[0] = std::cmp::max(i32::wrapping_mul(self.iConst8, (iTemp252 < self.iVec17[1]) as i32), i32::wrapping_add(self.iRec225[1], -1));
			let mut fTemp253: F32 = F32::abs(F32::max((iTemp252) as F32, ((self.iRec225[0] > 0) as i32) as u32 as F32));
			let mut fTemp254: F32 = if (fTemp253 > self.fRec223[1]) as i32 != 0 {self.fConst6} else {self.fConst9};
			self.fRec223[0] = fTemp253 * (1.0 - fTemp254) + self.fRec223[1] * fTemp254;
			let mut fTemp255: F32 = 0.005 * self.fRec223[0] * self.fRec188[1];
			self.fRec226[0] = self.fRec191[1];
			self.fRec227[(self.IOTA0 & 2047) as usize] = -1.0 * 0.9973053 * (0.9 * self.fRec226[2] + 0.05 * (self.fRec226[1] + self.fRec226[3]));
			let mut fTemp256: F32 = fTemp230 * fTemp233;
			let mut fTemp257: F32 = fTemp229 * fTemp237 * fTemp236;
			let mut fTemp258: F32 = fTemp242 * fTemp241 * fTemp240;
			let mut fTemp259: F32 = fTemp249 * fTemp248 * fTemp247 * fTemp246;
			self.fVec18[0] = fTemp259 * self.fRec227[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp250, 2))) & 2047) as usize] + fTemp245 * (fTemp258 * self.fRec227[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp243, 2))) & 2047) as usize] + 0.5 * fTemp257 * self.fRec227[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp238, 2))) & 2047) as usize] + 0.16666667 * fTemp256 * self.fRec227[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp234, 2))) & 2047) as usize] + 0.041666668 * fTemp231 * self.fRec227[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp224, 2))) & 2047) as usize]);
			let mut fTemp260: F32 = F32::tan(self.fConst10 * self.fRec220[0]);
			let mut fTemp261: F32 = 1.0 / fTemp260;
			let mut fTemp262: F32 = (fTemp261 + 1.4142135) / fTemp260 + 1.0;
			self.fVec19[0] = fSlow13;
			self.iRec228[0] = i32::wrapping_add(i32::wrapping_mul(i32::wrapping_add(self.iRec228[1], (self.iRec228[1] > 0) as i32), (fSlow13 <= self.fVec19[1]) as i32), (fSlow13 > self.fVec19[1]) as i32);
			let mut fTemp263: F32 = (self.iRec228[0]) as F32 / F32::max(1.0, self.fConst11 * mydsp_faustpower2_f(1.0 - 0.0005 * self.fRec220[0]));
			self.fRec229[0] = fTemp55 - (self.fRec229[2] * ((fTemp261 + -1.4142135) / fTemp260 + 1.0) + 2.0 * self.fRec229[1] * (1.0 - 1.0 / mydsp_faustpower2_f(fTemp260))) / fTemp262;
			let mut fTemp264: F32 = 0.5 * ((self.fRec229[2] + self.fRec229[0] + 2.0 * self.fRec229[1]) * F32::max(0.0, F32::min(fTemp263, 2.0 - fTemp263)) / fTemp262);
			let mut fTemp265: F32 = fTemp264 + self.fVec18[1] + fTemp255;
			self.fVec20[0] = fTemp265;
			self.fRec222[(self.IOTA0 & 2047) as usize] = 0.95 * self.fVec20[2] + 0.05 * self.fRec222[((i32::wrapping_sub(self.IOTA0, 1)) & 2047) as usize];
			let mut fRec218: F32 = fTemp259 * self.fRec222[((i32::wrapping_sub(self.IOTA0, iTemp250)) & 2047) as usize] + fTemp245 * (fTemp258 * self.fRec222[((i32::wrapping_sub(self.IOTA0, iTemp243)) & 2047) as usize] + 0.5 * fTemp257 * self.fRec222[((i32::wrapping_sub(self.IOTA0, iTemp238)) & 2047) as usize] + 0.16666667 * fTemp256 * self.fRec222[((i32::wrapping_sub(self.IOTA0, iTemp234)) & 2047) as usize] + 0.041666668 * fTemp231 * self.fRec222[((i32::wrapping_sub(self.IOTA0, iTemp224)) & 2047) as usize]);
			let mut fRec219: F32 = self.fVec20[1] + self.fRec212[1];
			self.fRec212[0] = fRec217;
			let mut fRec213: F32 = self.fRec212[1];
			let mut fRec214: F32 = fRec218;
			let mut fRec215: F32 = fRec219;
			self.fRec208[0] = fRec213;
			let mut fRec209: F32 = fTemp255 + fTemp264 + self.fRec208[1];
			let mut fRec210: F32 = fRec214;
			let mut fRec211: F32 = fRec215;
			self.fRec204[(self.IOTA0 & 2047) as usize] = fRec209;
			let mut fRec205: F32 = fTemp259 * self.fRec204[((i32::wrapping_sub(self.IOTA0, iTemp251)) & 2047) as usize] + fTemp245 * (fTemp258 * self.fRec204[((i32::wrapping_sub(self.IOTA0, iTemp244)) & 2047) as usize] + 0.5 * fTemp257 * self.fRec204[((i32::wrapping_sub(self.IOTA0, iTemp239)) & 2047) as usize] + 0.16666667 * fTemp256 * self.fRec204[((i32::wrapping_sub(self.IOTA0, iTemp235)) & 2047) as usize] + 0.041666668 * fTemp231 * self.fRec204[((i32::wrapping_sub(self.IOTA0, iTemp225)) & 2047) as usize]);
			self.fRec206[0] = fRec210;
			let mut fRec207: F32 = fRec211;
			self.fRec202[0] = fSlow5 * self.fRec206[1];
			let mut fRec203: F32 = fRec207;
			self.fRec197[0] = fRec201;
			let mut fRec198: F32 = fSlow5 * self.fRec197[1];
			let mut fRec199: F32 = self.fRec202[0];
			let mut fRec200: F32 = fRec203;
			self.fRec193[(self.IOTA0 & 2047) as usize] = fRec198;
			let mut fRec194: F32 = fRec205;
			let mut fRec195: F32 = fRec199;
			let mut fRec196: F32 = fRec200;
			self.fRec191[0] = fRec194;
			let mut fRec192: F32 = fRec196;
			let mut fTemp266: F32 = F32::abs(fRec192);
			let mut fTemp267: F32 = if (fTemp266 > self.fRec190[1]) as i32 != 0 {0.0} else {self.fConst12};
			self.fRec190[0] = fTemp266 * (1.0 - fTemp267) + self.fRec190[1] * fTemp267;
			let mut fRec189: F32 = 0.0 - 0.95 * F32::max(2e+01 * F32::log10(F32::max(1.1754944e-38, self.fRec190[0])) + 1e+01, 0.0);
			self.fRec188[0] = 1.5 * fRec192 * F32::powf(1e+01, 0.05 * fRec189);
			self.fRec187[0] = self.fRec188[0] - (self.fRec187[2] * (fTemp10 + (fTemp217 - fTemp9) / fTemp216 + 1.0) + 2.0 * self.fRec187[1] * (fTemp10 + (1.0 - fTemp219))) / fTemp220;
			self.fRec186[0] = (self.fRec187[2] + self.fRec187[0] + 2.0 * self.fRec187[1]) / fTemp220 - (self.fRec186[2] * (fTemp6 + (fTemp217 - fTemp5) / fTemp216 + fTemp2) + 2.0 * self.fRec186[1] * (fTemp6 + (1.0 - (fTemp1 + fTemp219)))) / fTemp218;
			self.fRec230[0] = fSlow14 + self.fConst2 * self.fRec230[1];
			let mut fTemp268: F32 = self.fRec230[0] * ((self.fRec186[2] + self.fRec186[0] + 2.0 * self.fRec186[1]) / fTemp218 + (self.fRec141[2] + self.fRec141[0] + 2.0 * self.fRec141[1]) / fTemp166 + (self.fRec96[2] + self.fRec96[0] + 2.0 * self.fRec96[1]) / fTemp114 + (self.fRec51[2] + self.fRec51[0] + 2.0 * self.fRec51[1]) / fTemp62 + (self.fRec4[2] + self.fRec4[0] + 2.0 * self.fRec4[1]) / fTemp7);
			let mut fTemp269: F32 = self.fRec39[0] + self.fRec3[0];
			self.fRec231[0] = self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow15 + fTemp269) + 0.5) as i32, 2047))) as usize] } + self.fConst2 * self.fRec231[1];
			let mut fTemp270: F32 = F32::tan(self.fConst3 * F32::max(2e+01, F32::min(1e+04, self.fRec231[0])));
			let mut fTemp271: F32 = 1.0 / fTemp270;
			let mut fTemp272: F32 = fTemp6 + (fTemp5 + fTemp271) / fTemp270 + fTemp2;
			let mut fTemp273: F32 = 1.0 / mydsp_faustpower2_f(fTemp270);
			let mut fTemp274: F32 = fTemp10 + (fTemp9 + fTemp271) / fTemp270 + 1.0;
			self.fRec236[0] = self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow15 + self.fRec39[0]) + 0.5) as i32, 2047))) as usize] } + self.fConst2 * self.fRec236[1];
			let mut fTemp275: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec236[0]));
			let mut fTemp276: F32 = self.fConst13 * fTemp275;
			let mut fTemp277: F32 = self.fRec234[1] + fTemp276;
			let mut fTemp278: F32 = fTemp277 + -1.0;
			let mut iTemp279: i32 = (fTemp278 < 0.0) as i32;
			self.fRec234[0] = if iTemp279 != 0 {fTemp277} else {fTemp278};
			let mut fRec235: F32 = if iTemp279 != 0 {fTemp277} else {self.fRec234[1] + fTemp276 + (1.0 - self.fConst0 / fTemp275) * fTemp278};
			self.fRec237[0] = fSlow16 + self.fConst2 * self.fRec237[1];
			self.fRec233[0] = self.fRec237[0] * (2.0 * fRec235 + -1.0) - (self.fRec233[2] * (fTemp10 + (1.0 - (fTemp9 - fTemp271) / fTemp270)) + 2.0 * self.fRec233[1] * (fTemp10 + (1.0 - fTemp273))) / fTemp274;
			self.fRec232[0] = (self.fRec233[2] + self.fRec233[0] + 2.0 * self.fRec233[1]) / fTemp274 - (self.fRec232[2] * (fTemp6 + (fTemp271 - fTemp5) / fTemp270 + fTemp2) + 2.0 * self.fRec232[1] * (fTemp6 + (1.0 - (fTemp1 + fTemp273)))) / fTemp272;
			self.fRec238[0] = self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow17 + fTemp269) + 0.5) as i32, 2047))) as usize] } + self.fConst2 * self.fRec238[1];
			let mut fTemp280: F32 = F32::tan(self.fConst3 * F32::max(2e+01, F32::min(1e+04, self.fRec238[0])));
			let mut fTemp281: F32 = 1.0 / fTemp280;
			let mut fTemp282: F32 = fTemp6 + (fTemp5 + fTemp281) / fTemp280 + fTemp2;
			let mut fTemp283: F32 = 1.0 / mydsp_faustpower2_f(fTemp280);
			let mut fTemp284: F32 = fTemp10 + (fTemp281 + fTemp9) / fTemp280 + 1.0;
			self.fRec243[0] = self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow17 + self.fRec39[0]) + 0.5) as i32, 2047))) as usize] } + self.fConst2 * self.fRec243[1];
			let mut fTemp285: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec243[0]));
			let mut fTemp286: F32 = self.fConst13 * fTemp285;
			let mut fTemp287: F32 = self.fRec241[1] + fTemp286;
			let mut fTemp288: F32 = fTemp287 + -1.0;
			let mut iTemp289: i32 = (fTemp288 < 0.0) as i32;
			self.fRec241[0] = if iTemp289 != 0 {fTemp287} else {fTemp288};
			let mut fRec242: F32 = if iTemp289 != 0 {fTemp287} else {fTemp286 + self.fRec241[1] + (1.0 - self.fConst0 / fTemp285) * fTemp288};
			self.fRec244[0] = fSlow18 + self.fConst2 * self.fRec244[1];
			self.fRec240[0] = self.fRec244[0] * (2.0 * fRec242 + -1.0) - (self.fRec240[2] * (fTemp10 + (fTemp281 - fTemp9) / fTemp280 + 1.0) + 2.0 * self.fRec240[1] * (fTemp10 + (1.0 - fTemp283))) / fTemp284;
			self.fRec239[0] = (self.fRec240[2] + self.fRec240[0] + 2.0 * self.fRec240[1]) / fTemp284 - (self.fRec239[2] * (fTemp6 + (fTemp281 - fTemp5) / fTemp280 + fTemp2) + 2.0 * self.fRec239[1] * (fTemp6 + (1.0 - (fTemp1 + fTemp283)))) / fTemp282;
			self.fRec245[0] = self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow19 + fTemp269) + 0.5) as i32, 2047))) as usize] } + self.fConst2 * self.fRec245[1];
			let mut fTemp290: F32 = F32::tan(self.fConst3 * F32::max(2e+01, F32::min(1e+04, self.fRec245[0])));
			let mut fTemp291: F32 = 1.0 / fTemp290;
			let mut fTemp292: F32 = fTemp6 + (fTemp5 + fTemp291) / fTemp290 + fTemp2;
			let mut fTemp293: F32 = 1.0 / mydsp_faustpower2_f(fTemp290);
			let mut fTemp294: F32 = fTemp10 + (fTemp9 + fTemp291) / fTemp290 + 1.0;
			self.fRec250[0] = self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow19 + self.fRec39[0]) + 0.5) as i32, 2047))) as usize] } + self.fConst2 * self.fRec250[1];
			let mut fTemp295: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec250[0]));
			let mut fTemp296: F32 = self.fConst13 * fTemp295;
			let mut fTemp297: F32 = self.fRec248[1] + fTemp296;
			let mut fTemp298: F32 = fTemp297 + -1.0;
			let mut iTemp299: i32 = (fTemp298 < 0.0) as i32;
			self.fRec248[0] = if iTemp299 != 0 {fTemp297} else {fTemp298};
			let mut fRec249: F32 = if iTemp299 != 0 {fTemp297} else {self.fRec248[1] + fTemp296 + (1.0 - self.fConst0 / fTemp295) * fTemp298};
			self.fRec251[0] = fSlow20 + self.fConst2 * self.fRec251[1];
			self.fRec247[0] = self.fRec251[0] * (2.0 * fRec249 + -1.0) - (self.fRec247[2] * (fTemp10 + (1.0 - (fTemp9 - fTemp291) / fTemp290)) + 2.0 * self.fRec247[1] * (fTemp10 + (1.0 - fTemp293))) / fTemp294;
			self.fRec246[0] = (self.fRec247[2] + self.fRec247[0] + 2.0 * self.fRec247[1]) / fTemp294 - (self.fRec246[2] * (fTemp6 + (fTemp291 - fTemp5) / fTemp290 + fTemp2) + 2.0 * self.fRec246[1] * (fTemp6 + (1.0 - (fTemp1 + fTemp293)))) / fTemp292;
			self.fRec252[0] = self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow21 + fTemp269) + 0.5) as i32, 2047))) as usize] } + self.fConst2 * self.fRec252[1];
			let mut fTemp300: F32 = F32::tan(self.fConst3 * F32::max(2e+01, F32::min(1e+04, self.fRec252[0])));
			let mut fTemp301: F32 = 1.0 / fTemp300;
			let mut fTemp302: F32 = fTemp6 + (fTemp5 + fTemp301) / fTemp300 + fTemp2;
			let mut fTemp303: F32 = 1.0 / mydsp_faustpower2_f(fTemp300);
			let mut fTemp304: F32 = fTemp10 + (fTemp9 + fTemp301) / fTemp300 + 1.0;
			self.fRec257[0] = self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((16.11811 * (fSlow21 + self.fRec39[0]) + 0.5) as i32, 2047))) as usize] } + self.fConst2 * self.fRec257[1];
			let mut fTemp305: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec257[0]));
			let mut fTemp306: F32 = self.fConst13 * fTemp305;
			let mut fTemp307: F32 = self.fRec255[1] + fTemp306;
			let mut fTemp308: F32 = fTemp307 + -1.0;
			let mut iTemp309: i32 = (fTemp308 < 0.0) as i32;
			self.fRec255[0] = if iTemp309 != 0 {fTemp307} else {fTemp308};
			let mut fRec256: F32 = if iTemp309 != 0 {fTemp307} else {fTemp306 + self.fRec255[1] + (1.0 - self.fConst0 / fTemp305) * fTemp308};
			self.fRec258[0] = fSlow22 + self.fConst2 * self.fRec258[1];
			self.fRec254[0] = self.fRec258[0] * (2.0 * fRec256 + -1.0) - (self.fRec254[2] * (fTemp10 + (1.0 - (fTemp9 - fTemp301) / fTemp300)) + 2.0 * self.fRec254[1] * (fTemp10 + (1.0 - fTemp303))) / fTemp304;
			self.fRec253[0] = (self.fRec254[2] + self.fRec254[0] + 2.0 * self.fRec254[1]) / fTemp304 - (self.fRec253[2] * (fTemp6 + (fTemp301 - fTemp5) / fTemp300 + fTemp2) + 2.0 * self.fRec253[1] * (fTemp6 + (1.0 - (fTemp1 + fTemp303)))) / fTemp302;
			self.fRec259[0] = fSlow23 + self.fConst2 * self.fRec259[1];
			self.fRec260[0] = fSlow24 + self.fConst2 * self.fRec260[1];
			let mut fTemp310: F32 = self.fRec260[0] * self.fRec259[0] * ((self.fRec253[2] + self.fRec253[0] + 2.0 * self.fRec253[1]) / fTemp302 + (self.fRec246[2] + self.fRec246[0] + 2.0 * self.fRec246[1]) / fTemp292 + (self.fRec239[2] + self.fRec239[0] + 2.0 * self.fRec239[1]) / fTemp282 + (self.fRec232[2] + self.fRec232[0] + 2.0 * self.fRec232[1]) / fTemp272);
			self.fRec261[0] = fSlow25 + self.fConst2 * self.fRec261[1];
			self.fRec263[0] = fSlow28 + self.fConst2 * self.fRec263[1];
			let mut fTemp311: F32 = F32::max(self.fRec263[0], 23.44895);
			let mut fTemp312: F32 = F32::max(0.0, F32::min(2047.0, self.fConst14 / fTemp311));
			let mut fTemp313: F32 = F32::floor(fTemp312);
			let mut fTemp314: F32 = F32::max(2e+01, F32::abs(fTemp311));
			let mut iTemp315: i32 = i32::wrapping_sub(1, self.iVec0[1]);
			let mut fTemp316: F32 = if iTemp315 != 0 {0.0} else {self.fRec264[1] + self.fConst13 * fTemp314};
			self.fRec264[0] = fTemp316 - F32::floor(fTemp316);
			let mut fTemp317: F32 = mydsp_faustpower2_f(2.0 * self.fRec264[0] + -1.0);
			self.fVec21[0] = fTemp317;
			let mut fTemp318: F32 = (self.iVec0[1]) as F32;
			let mut fTemp319: F32 = fTemp318 * (fTemp317 - self.fVec21[1]) / fTemp314;
			self.fVec22[(self.IOTA0 & 4095) as usize] = fTemp319;
			let mut iTemp320: i32 = (fTemp312) as i32;
			self.fRec262[0] = 0.999 * self.fRec262[1] - self.fConst15 * ((fTemp312 - fTemp313) * self.fVec22[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp320, 1))) & 4095) as usize] - (fTemp319 - self.fVec22[((i32::wrapping_sub(self.IOTA0, iTemp320)) & 4095) as usize] * (fTemp313 + (1.0 - fTemp312))));
			self.fRec266[0] = fSlow29 + self.fConst2 * self.fRec266[1];
			let mut fTemp321: F32 = F32::max(self.fRec266[0], 23.44895);
			let mut fTemp322: F32 = F32::max(0.0, F32::min(2047.0, self.fConst14 / fTemp321));
			let mut fTemp323: F32 = F32::floor(fTemp322);
			let mut fTemp324: F32 = F32::max(2e+01, F32::abs(fTemp321));
			let mut fTemp325: F32 = if iTemp315 != 0 {0.0} else {self.fRec267[1] + self.fConst13 * fTemp324};
			self.fRec267[0] = fTemp325 - F32::floor(fTemp325);
			let mut fTemp326: F32 = mydsp_faustpower2_f(2.0 * self.fRec267[0] + -1.0);
			self.fVec23[0] = fTemp326;
			let mut fTemp327: F32 = fTemp318 * (fTemp326 - self.fVec23[1]) / fTemp324;
			self.fVec24[(self.IOTA0 & 4095) as usize] = fTemp327;
			let mut iTemp328: i32 = (fTemp322) as i32;
			self.fRec265[0] = 0.999 * self.fRec265[1] - self.fConst15 * ((fTemp322 - fTemp323) * self.fVec24[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp328, 1))) & 4095) as usize] - (fTemp327 - self.fVec24[((i32::wrapping_sub(self.IOTA0, iTemp328)) & 4095) as usize] * (fTemp323 + (1.0 - fTemp322))));
			self.fRec269[0] = fSlow30 + self.fConst2 * self.fRec269[1];
			let mut fTemp329: F32 = F32::max(self.fRec269[0], 23.44895);
			let mut fTemp330: F32 = F32::max(2e+01, F32::abs(fTemp329));
			let mut fTemp331: F32 = if iTemp315 != 0 {0.0} else {self.fRec270[1] + self.fConst13 * fTemp330};
			self.fRec270[0] = fTemp331 - F32::floor(fTemp331);
			let mut fTemp332: F32 = mydsp_faustpower2_f(2.0 * self.fRec270[0] + -1.0);
			self.fVec25[0] = fTemp332;
			let mut fTemp333: F32 = fTemp318 * (fTemp332 - self.fVec25[1]) / fTemp330;
			self.fVec26[(self.IOTA0 & 4095) as usize] = fTemp333;
			let mut fTemp334: F32 = F32::max(0.0, F32::min(2047.0, self.fConst14 / fTemp329));
			let mut iTemp335: i32 = (fTemp334) as i32;
			let mut fTemp336: F32 = F32::floor(fTemp334);
			self.fRec268[0] = 0.999 * self.fRec268[1] - self.fConst15 * (self.fVec26[((i32::wrapping_sub(self.IOTA0, iTemp335)) & 4095) as usize] * (fTemp336 + (1.0 - fTemp334)) - fTemp333 + (fTemp334 - fTemp336) * self.fVec26[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp335, 1))) & 4095) as usize]);
			self.fRec271[0] = fSlow31 + self.fConst2 * self.fRec271[1];
			self.fRec273[0] = fSlow33 + self.fConst2 * self.fRec273[1];
			let mut fTemp337: F32 = F32::max(self.fRec273[0], 23.44895);
			let mut fTemp338: F32 = F32::max(2e+01, F32::abs(fTemp337));
			let mut fTemp339: F32 = if iTemp315 != 0 {0.0} else {self.fRec274[1] + self.fConst13 * fTemp338};
			self.fRec274[0] = fTemp339 - F32::floor(fTemp339);
			let mut fTemp340: F32 = mydsp_faustpower2_f(2.0 * self.fRec274[0] + -1.0);
			self.fVec27[0] = fTemp340;
			let mut fTemp341: F32 = fTemp318 * (fTemp340 - self.fVec27[1]) / fTemp338;
			self.fVec28[(self.IOTA0 & 4095) as usize] = fTemp341;
			let mut fTemp342: F32 = F32::max(0.0, F32::min(2047.0, self.fConst14 / fTemp337));
			let mut iTemp343: i32 = (fTemp342) as i32;
			let mut fTemp344: F32 = F32::floor(fTemp342);
			self.fRec272[0] = 0.999 * self.fRec272[1] + self.fConst15 * (fTemp341 - self.fVec28[((i32::wrapping_sub(self.IOTA0, iTemp343)) & 4095) as usize] * (fTemp344 + (1.0 - fTemp342)) - (fTemp342 - fTemp344) * self.fVec28[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp343, 1))) & 4095) as usize]);
			self.fRec276[0] = fSlow34 + self.fConst2 * self.fRec276[1];
			let mut fTemp345: F32 = F32::max(self.fRec276[0], 23.44895);
			let mut fTemp346: F32 = F32::max(0.0, F32::min(2047.0, self.fConst14 / fTemp345));
			let mut fTemp347: F32 = F32::floor(fTemp346);
			let mut fTemp348: F32 = F32::max(2e+01, F32::abs(fTemp345));
			let mut fTemp349: F32 = if iTemp315 != 0 {0.0} else {self.fRec277[1] + self.fConst13 * fTemp348};
			self.fRec277[0] = fTemp349 - F32::floor(fTemp349);
			let mut fTemp350: F32 = mydsp_faustpower2_f(2.0 * self.fRec277[0] + -1.0);
			self.fVec29[0] = fTemp350;
			let mut fTemp351: F32 = fTemp318 * (fTemp350 - self.fVec29[1]) / fTemp348;
			self.fVec30[(self.IOTA0 & 4095) as usize] = fTemp351;
			let mut iTemp352: i32 = (fTemp346) as i32;
			self.fRec275[0] = 0.999 * self.fRec275[1] - self.fConst15 * ((fTemp346 - fTemp347) * self.fVec30[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp352, 1))) & 4095) as usize] - (fTemp351 - self.fVec30[((i32::wrapping_sub(self.IOTA0, iTemp352)) & 4095) as usize] * (fTemp347 + (1.0 - fTemp346))));
			self.fRec279[0] = fSlow35 + self.fConst2 * self.fRec279[1];
			let mut fTemp353: F32 = F32::max(self.fRec279[0], 23.44895);
			let mut fTemp354: F32 = F32::max(0.0, F32::min(2047.0, self.fConst14 / fTemp353));
			let mut fTemp355: F32 = F32::floor(fTemp354);
			let mut fTemp356: F32 = F32::max(2e+01, F32::abs(fTemp353));
			let mut fTemp357: F32 = if iTemp315 != 0 {0.0} else {self.fRec280[1] + self.fConst13 * fTemp356};
			self.fRec280[0] = fTemp357 - F32::floor(fTemp357);
			let mut fTemp358: F32 = mydsp_faustpower2_f(2.0 * self.fRec280[0] + -1.0);
			self.fVec31[0] = fTemp358;
			let mut fTemp359: F32 = fTemp318 * (fTemp358 - self.fVec31[1]) / fTemp356;
			self.fVec32[(self.IOTA0 & 4095) as usize] = fTemp359;
			let mut iTemp360: i32 = (fTemp354) as i32;
			self.fRec278[0] = 0.999 * self.fRec278[1] - self.fConst15 * ((fTemp354 - fTemp355) * self.fVec32[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp360, 1))) & 4095) as usize] - (fTemp359 - self.fVec32[((i32::wrapping_sub(self.IOTA0, iTemp360)) & 4095) as usize] * (fTemp355 + (1.0 - fTemp354))));
			self.fRec281[0] = fSlow36 + self.fConst2 * self.fRec281[1];
			self.fRec283[0] = fSlow38 + self.fConst2 * self.fRec283[1];
			let mut fTemp361: F32 = F32::max(self.fRec283[0], 23.44895);
			let mut fTemp362: F32 = F32::max(2e+01, F32::abs(fTemp361));
			let mut fTemp363: F32 = if iTemp315 != 0 {0.0} else {self.fRec284[1] + self.fConst13 * fTemp362};
			self.fRec284[0] = fTemp363 - F32::floor(fTemp363);
			let mut fTemp364: F32 = mydsp_faustpower2_f(2.0 * self.fRec284[0] + -1.0);
			self.fVec33[0] = fTemp364;
			let mut fTemp365: F32 = fTemp318 * (fTemp364 - self.fVec33[1]) / fTemp362;
			self.fVec34[(self.IOTA0 & 4095) as usize] = fTemp365;
			let mut fTemp366: F32 = F32::max(0.0, F32::min(2047.0, self.fConst14 / fTemp361));
			let mut iTemp367: i32 = (fTemp366) as i32;
			let mut fTemp368: F32 = F32::floor(fTemp366);
			self.fRec282[0] = 0.999 * self.fRec282[1] - self.fConst15 * (self.fVec34[((i32::wrapping_sub(self.IOTA0, iTemp367)) & 4095) as usize] * (fTemp368 + (1.0 - fTemp366)) - fTemp365 + (fTemp366 - fTemp368) * self.fVec34[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp367, 1))) & 4095) as usize]);
			self.fRec286[0] = fSlow39 + self.fConst2 * self.fRec286[1];
			let mut fTemp369: F32 = F32::max(self.fRec286[0], 23.44895);
			let mut fTemp370: F32 = F32::max(2e+01, F32::abs(fTemp369));
			let mut fTemp371: F32 = if iTemp315 != 0 {0.0} else {self.fRec287[1] + self.fConst13 * fTemp370};
			self.fRec287[0] = fTemp371 - F32::floor(fTemp371);
			let mut fTemp372: F32 = mydsp_faustpower2_f(2.0 * self.fRec287[0] + -1.0);
			self.fVec35[0] = fTemp372;
			let mut fTemp373: F32 = fTemp318 * (fTemp372 - self.fVec35[1]) / fTemp370;
			self.fVec36[(self.IOTA0 & 4095) as usize] = fTemp373;
			let mut fTemp374: F32 = F32::max(0.0, F32::min(2047.0, self.fConst14 / fTemp369));
			let mut iTemp375: i32 = (fTemp374) as i32;
			let mut fTemp376: F32 = F32::floor(fTemp374);
			self.fRec285[0] = 0.999 * self.fRec285[1] + self.fConst15 * (fTemp373 - self.fVec36[((i32::wrapping_sub(self.IOTA0, iTemp375)) & 4095) as usize] * (fTemp376 + (1.0 - fTemp374)) - (fTemp374 - fTemp376) * self.fVec36[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp375, 1))) & 4095) as usize]);
			self.fRec289[0] = fSlow40 + self.fConst2 * self.fRec289[1];
			let mut fTemp377: F32 = F32::max(self.fRec289[0], 23.44895);
			let mut fTemp378: F32 = F32::max(2e+01, F32::abs(fTemp377));
			let mut fTemp379: F32 = if iTemp315 != 0 {0.0} else {self.fRec290[1] + self.fConst13 * fTemp378};
			self.fRec290[0] = fTemp379 - F32::floor(fTemp379);
			let mut fTemp380: F32 = mydsp_faustpower2_f(2.0 * self.fRec290[0] + -1.0);
			self.fVec37[0] = fTemp380;
			let mut fTemp381: F32 = fTemp318 * (fTemp380 - self.fVec37[1]) / fTemp378;
			self.fVec38[(self.IOTA0 & 4095) as usize] = fTemp381;
			let mut fTemp382: F32 = F32::max(0.0, F32::min(2047.0, self.fConst14 / fTemp377));
			let mut iTemp383: i32 = (fTemp382) as i32;
			let mut fTemp384: F32 = F32::floor(fTemp382);
			self.fRec288[0] = 0.999 * self.fRec288[1] + self.fConst15 * (fTemp381 - self.fVec38[((i32::wrapping_sub(self.IOTA0, iTemp383)) & 4095) as usize] * (fTemp384 + (1.0 - fTemp382)) - (fTemp382 - fTemp384) * self.fVec38[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp383, 1))) & 4095) as usize]);
			self.fRec291[0] = fSlow41 + self.fConst2 * self.fRec291[1];
			self.fRec293[0] = fSlow43 + self.fConst2 * self.fRec293[1];
			let mut fTemp385: F32 = F32::max(self.fRec293[0], 23.44895);
			let mut fTemp386: F32 = F32::max(0.0, F32::min(2047.0, self.fConst14 / fTemp385));
			let mut fTemp387: F32 = F32::floor(fTemp386);
			let mut fTemp388: F32 = F32::max(2e+01, F32::abs(fTemp385));
			let mut fTemp389: F32 = if iTemp315 != 0 {0.0} else {self.fRec294[1] + self.fConst13 * fTemp388};
			self.fRec294[0] = fTemp389 - F32::floor(fTemp389);
			let mut fTemp390: F32 = mydsp_faustpower2_f(2.0 * self.fRec294[0] + -1.0);
			self.fVec39[0] = fTemp390;
			let mut fTemp391: F32 = fTemp318 * (fTemp390 - self.fVec39[1]) / fTemp388;
			self.fVec40[(self.IOTA0 & 4095) as usize] = fTemp391;
			let mut iTemp392: i32 = (fTemp386) as i32;
			self.fRec292[0] = 0.999 * self.fRec292[1] - self.fConst15 * ((fTemp386 - fTemp387) * self.fVec40[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp392, 1))) & 4095) as usize] - (fTemp391 - self.fVec40[((i32::wrapping_sub(self.IOTA0, iTemp392)) & 4095) as usize] * (fTemp387 + (1.0 - fTemp386))));
			self.fRec296[0] = fSlow44 + self.fConst2 * self.fRec296[1];
			let mut fTemp393: F32 = F32::max(self.fRec296[0], 23.44895);
			let mut fTemp394: F32 = F32::max(2e+01, F32::abs(fTemp393));
			let mut fTemp395: F32 = if iTemp315 != 0 {0.0} else {self.fRec297[1] + self.fConst13 * fTemp394};
			self.fRec297[0] = fTemp395 - F32::floor(fTemp395);
			let mut fTemp396: F32 = mydsp_faustpower2_f(2.0 * self.fRec297[0] + -1.0);
			self.fVec41[0] = fTemp396;
			let mut fTemp397: F32 = fTemp318 * (fTemp396 - self.fVec41[1]) / fTemp394;
			self.fVec42[(self.IOTA0 & 4095) as usize] = fTemp397;
			let mut fTemp398: F32 = F32::max(0.0, F32::min(2047.0, self.fConst14 / fTemp393));
			let mut iTemp399: i32 = (fTemp398) as i32;
			let mut fTemp400: F32 = F32::floor(fTemp398);
			self.fRec295[0] = 0.999 * self.fRec295[1] + self.fConst15 * (fTemp397 - self.fVec42[((i32::wrapping_sub(self.IOTA0, iTemp399)) & 4095) as usize] * (fTemp400 + (1.0 - fTemp398)) - (fTemp398 - fTemp400) * self.fVec42[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp399, 1))) & 4095) as usize]);
			self.fRec299[0] = fSlow45 + self.fConst2 * self.fRec299[1];
			let mut fTemp401: F32 = F32::max(self.fRec299[0], 23.44895);
			let mut fTemp402: F32 = F32::max(0.0, F32::min(2047.0, self.fConst14 / fTemp401));
			let mut fTemp403: F32 = F32::floor(fTemp402);
			let mut fTemp404: F32 = F32::max(2e+01, F32::abs(fTemp401));
			let mut fTemp405: F32 = if iTemp315 != 0 {0.0} else {self.fRec300[1] + self.fConst13 * fTemp404};
			self.fRec300[0] = fTemp405 - F32::floor(fTemp405);
			let mut fTemp406: F32 = mydsp_faustpower2_f(2.0 * self.fRec300[0] + -1.0);
			self.fVec43[0] = fTemp406;
			let mut fTemp407: F32 = fTemp318 * (fTemp406 - self.fVec43[1]) / fTemp404;
			self.fVec44[(self.IOTA0 & 4095) as usize] = fTemp407;
			let mut iTemp408: i32 = (fTemp402) as i32;
			self.fRec298[0] = 0.999 * self.fRec298[1] - self.fConst15 * ((fTemp402 - fTemp403) * self.fVec44[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp408, 1))) & 4095) as usize] - (fTemp407 - self.fVec44[((i32::wrapping_sub(self.IOTA0, iTemp408)) & 4095) as usize] * (fTemp403 + (1.0 - fTemp402))));
			self.fRec301[0] = fSlow46 + self.fConst2 * self.fRec301[1];
			let mut fTemp409: F32 = F32::max(-1.0, F32::min(1.0, self.fRec261[0] + self.fConst16 * (self.fRec301[0] * (self.fRec299[0] * self.fRec298[0] + self.fRec296[0] * self.fRec295[0] + self.fRec293[0] * self.fRec292[0]) + self.fRec291[0] * (self.fRec289[0] * self.fRec288[0] + self.fRec286[0] * self.fRec285[0] + self.fRec283[0] * self.fRec282[0]) + self.fRec281[0] * (self.fRec279[0] * self.fRec278[0] + self.fRec276[0] * self.fRec275[0] + self.fRec273[0] * self.fRec272[0]) + self.fRec271[0] * (self.fRec269[0] * self.fRec268[0] + self.fRec266[0] * self.fRec265[0] + self.fRec263[0] * self.fRec262[0])) * F32::powf(1e+01, 0.6666667 * self.fRec261[0])));
			self.fRec302[0] = fSlow47 + self.fConst2 * self.fRec302[1];
			let mut fTemp410: F32 = self.fRec302[0] * fTemp409 * (1.0 - 0.33333334 * mydsp_faustpower2_f(fTemp409));
			self.fRec303[0] = fSlow48 + self.fConst2 * self.fRec303[1];
			let mut fTemp411: F32 = (1.0 - self.fRec303[0]) * (fTemp410 + fTemp310 + fTemp268);
			self.fRec305[0] = fSlow49 + self.fConst2 * self.fRec305[1];
			self.fRec304[(self.IOTA0 & 2097151) as usize] = fTemp268 + fTemp410 + fSlow50 * self.fRec304[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add((F32::min(self.fConst17, F32::max(0.0, self.fConst0 * self.fRec305[0]))) as i32, 1))) & 2097151) as usize] + fTemp310;
			let mut fTemp412: F32 = self.fRec304[(self.IOTA0 & 2097151) as usize] * self.fRec303[0];
			let mut fTemp413: F32 = fTemp412 + fTemp411;
			self.fRec316[0] = 0.995 * (self.fRec316[1] + (i32::wrapping_mul(iTemp315, iSlow55)) as F32) + fSlow56;
			let mut fTemp414: F32 = self.fRec316[0] + -1.49999;
			let mut fTemp415: F32 = F32::floor(fTemp414);
			self.fRec318[0] = 0.995 * (self.fRec318[1] + (i32::wrapping_mul(iTemp315, iSlow57)) as F32) + fSlow58;
			let mut fTemp416: F32 = self.fRec318[0] + -1.49999;
			let mut fTemp417: F32 = F32::floor(fTemp416);
			self.fRec322[0] = 0.9999 * (self.fRec322[1] + (i32::wrapping_mul(iTemp315, iSlow59)) as F32) + fSlow60;
			let mut fTemp418: F32 = self.fRec322[0] + -1.49999;
			let mut fTemp419: F32 = F32::floor(fTemp418);
			let mut fTemp420: F32 = self.fRec322[0] - fTemp419;
			let mut fTemp421: F32 = fTemp419 + (2.0 - self.fRec322[0]);
			let mut fTemp422: F32 = 0.760314 * self.fRec307[1] - 0.64955574 * self.fRec320[1];
			let mut fTemp423: F32 = 0.760314 * self.fRec306[1] - 0.64955574 * self.fRec319[1];
			self.fVec45[(self.IOTA0 & 16383) as usize] = fTemp423 - fTemp422;
			let mut fTemp424: F32 = self.fVec45[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, (fTemp418) as i32)))) & 16383) as usize];
			self.fVec46[0] = fTemp424;
			self.fRec321[0] = 0.70710677 * (fTemp421 * fTemp424 / fTemp420 + self.fVec46[1]) - self.fRec321[1] * fTemp421 / fTemp420;
			self.fRec319[0] = self.fRec321[0];
			self.fRec324[0] = 0.9999 * (self.fRec324[1] + (i32::wrapping_mul(iTemp315, iSlow61)) as F32) + fSlow62;
			let mut fTemp425: F32 = self.fRec324[0] + -1.49999;
			let mut fTemp426: F32 = F32::floor(fTemp425);
			let mut fTemp427: F32 = self.fRec324[0] - fTemp426;
			let mut fTemp428: F32 = fTemp426 + (2.0 - self.fRec324[0]);
			self.fVec47[(self.IOTA0 & 16383) as usize] = fTemp423 + fTemp422;
			let mut fTemp429: F32 = self.fVec47[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, (fTemp425) as i32)))) & 16383) as usize];
			self.fVec48[0] = fTemp429;
			self.fRec323[0] = 0.70710677 * (fTemp428 * fTemp429 / fTemp427 + self.fVec48[1]) - self.fRec323[1] * fTemp428 / fTemp427;
			self.fRec320[0] = self.fRec323[0];
			let mut fTemp430: F32 = 0.760314 * self.fRec319[1] + 0.64955574 * self.fRec306[1];
			self.fRec328[0] = 0.9999 * (self.fRec328[1] + (i32::wrapping_mul(iTemp315, iSlow63)) as F32) + fSlow64;
			let mut fTemp431: F32 = self.fRec328[0] + -1.49999;
			let mut fTemp432: F32 = F32::floor(fTemp431);
			let mut fTemp433: F32 = 0.760314 * fTemp430 - 0.64955574 * self.fRec325[1];
			let mut fTemp434: F32 = 0.760314 * self.fRec320[1] + 0.64955574 * self.fRec307[1];
			let mut fTemp435: F32 = 0.760314 * fTemp434 - 0.64955574 * self.fRec326[1];
			self.fVec49[(self.IOTA0 & 16383) as usize] = 0.0 - 0.70710677 * (fTemp435 - fTemp433);
			let mut fTemp436: F32 = self.fVec49[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, (fTemp431) as i32)))) & 16383) as usize];
			self.fVec50[0] = fTemp436;
			self.fRec327[0] = self.fVec50[1] - (fTemp432 + (2.0 - self.fRec328[0])) * (self.fRec327[1] - fTemp436) / (self.fRec328[0] - fTemp432);
			self.fRec325[0] = self.fRec327[0];
			self.fRec330[0] = 0.9999 * (self.fRec330[1] + (i32::wrapping_mul(iTemp315, iSlow65)) as F32) + fSlow66;
			let mut fTemp437: F32 = self.fRec330[0] + -1.49999;
			let mut fTemp438: F32 = F32::floor(fTemp437);
			let mut fTemp439: F32 = self.fRec330[0] - fTemp438;
			let mut fTemp440: F32 = fTemp438 + (2.0 - self.fRec330[0]);
			self.fVec51[(self.IOTA0 & 16383) as usize] = fTemp433 + fTemp435;
			let mut fTemp441: F32 = self.fVec51[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, (fTemp437) as i32)))) & 16383) as usize];
			self.fVec52[0] = fTemp441;
			self.fRec329[0] = 0.70710677 * (fTemp440 * fTemp441 / fTemp439 + self.fVec52[1]) - self.fRec329[1] * fTemp440 / fTemp439;
			self.fRec326[0] = self.fRec329[0];
			let mut fTemp442: F32 = 0.760314 * self.fRec325[1] + 0.64955574 * fTemp430;
			self.fRec334[0] = 0.9999 * (self.fRec334[1] + (i32::wrapping_mul(iTemp315, iSlow67)) as F32) + fSlow68;
			let mut fTemp443: F32 = self.fRec334[0] + -1.49999;
			let mut fTemp444: F32 = F32::floor(fTemp443);
			let mut fTemp445: F32 = self.fRec334[0] - fTemp444;
			let mut fTemp446: F32 = fTemp444 + (2.0 - self.fRec334[0]);
			let mut fTemp447: F32 = 0.760314 * self.fRec326[1] + 0.64955574 * fTemp434;
			let mut fTemp448: F32 = 0.760314 * fTemp447 - 0.64955574 * self.fRec332[1];
			let mut fTemp449: F32 = 0.760314 * fTemp442 - 0.64955574 * self.fRec331[1];
			self.fVec53[(self.IOTA0 & 16383) as usize] = fTemp449 - fTemp448;
			let mut fTemp450: F32 = self.fVec53[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, (fTemp443) as i32)))) & 16383) as usize];
			self.fVec54[0] = fTemp450;
			self.fRec333[0] = 0.70710677 * (fTemp446 * fTemp450 / fTemp445 + self.fVec54[1]) - self.fRec333[1] * fTemp446 / fTemp445;
			self.fRec331[0] = self.fRec333[0];
			self.fRec336[0] = 0.9999 * (self.fRec336[1] + (i32::wrapping_mul(iTemp315, iSlow69)) as F32) + fSlow70;
			let mut fTemp451: F32 = self.fRec336[0] + -1.49999;
			let mut fTemp452: F32 = F32::floor(fTemp451);
			let mut fTemp453: F32 = self.fRec336[0] - fTemp452;
			let mut fTemp454: F32 = fTemp452 + (2.0 - self.fRec336[0]);
			self.fVec55[(self.IOTA0 & 16383) as usize] = fTemp449 + fTemp448;
			let mut fTemp455: F32 = self.fVec55[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, (fTemp451) as i32)))) & 16383) as usize];
			self.fVec56[0] = fTemp455;
			self.fRec335[0] = 0.70710677 * (fTemp454 * fTemp455 / fTemp453 + self.fVec56[1]) - self.fRec335[1] * fTemp454 / fTemp453;
			self.fRec332[0] = self.fRec335[0];
			let mut fTemp456: F32 = 0.760314 * self.fRec331[1] + 0.64955574 * fTemp442;
			self.fRec340[0] = 0.9999 * (self.fRec340[1] + (i32::wrapping_mul(iTemp315, iSlow71)) as F32) + fSlow72;
			let mut fTemp457: F32 = self.fRec340[0] + -1.49999;
			let mut fTemp458: F32 = F32::floor(fTemp457);
			let mut fTemp459: F32 = self.fRec340[0] - fTemp458;
			let mut fTemp460: F32 = fTemp458 + (2.0 - self.fRec340[0]);
			let mut fTemp461: F32 = 0.760314 * self.fRec332[1] + 0.64955574 * fTemp447;
			let mut fTemp462: F32 = 0.760314 * fTemp461 - 0.64955574 * self.fRec338[1];
			let mut fTemp463: F32 = 0.760314 * fTemp456 - 0.64955574 * self.fRec337[1];
			self.fVec57[(self.IOTA0 & 16383) as usize] = fTemp463 - fTemp462;
			let mut fTemp464: F32 = self.fVec57[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, (fTemp457) as i32)))) & 16383) as usize];
			self.fVec58[0] = fTemp464;
			self.fRec339[0] = 0.70710677 * (fTemp460 * fTemp464 / fTemp459 + self.fVec58[1]) - self.fRec339[1] * fTemp460 / fTemp459;
			self.fRec337[0] = self.fRec339[0];
			self.fRec342[0] = 0.9999 * (self.fRec342[1] + (i32::wrapping_mul(iTemp315, iSlow73)) as F32) + fSlow74;
			let mut fTemp465: F32 = self.fRec342[0] + -1.49999;
			let mut fTemp466: F32 = F32::floor(fTemp465);
			let mut fTemp467: F32 = self.fRec342[0] - fTemp466;
			let mut fTemp468: F32 = fTemp466 + (2.0 - self.fRec342[0]);
			self.fVec59[(self.IOTA0 & 16383) as usize] = fTemp463 + fTemp462;
			let mut fTemp469: F32 = self.fVec59[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, (fTemp465) as i32)))) & 16383) as usize];
			self.fVec60[0] = fTemp469;
			self.fRec341[0] = 0.70710677 * (fTemp468 * fTemp469 / fTemp467 + self.fVec60[1]) - self.fRec341[1] * fTemp468 / fTemp467;
			self.fRec338[0] = self.fRec341[0];
			let mut fTemp470: F32 = 0.760314 * self.fRec337[1] + 0.64955574 * fTemp456;
			self.fRec346[0] = 0.9999 * (self.fRec346[1] + (i32::wrapping_mul(iTemp315, iSlow75)) as F32) + fSlow76;
			let mut fTemp471: F32 = self.fRec346[0] + -1.49999;
			let mut fTemp472: F32 = F32::floor(fTemp471);
			let mut fTemp473: F32 = 0.760314 * fTemp470 - 0.64955574 * self.fRec343[1];
			let mut fTemp474: F32 = 0.760314 * self.fRec338[1] + 0.64955574 * fTemp461;
			let mut fTemp475: F32 = 0.760314 * fTemp474 - 0.64955574 * self.fRec344[1];
			self.fVec61[(self.IOTA0 & 16383) as usize] = 0.0 - 0.70710677 * (fTemp475 - fTemp473);
			let mut fTemp476: F32 = self.fVec61[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, (fTemp471) as i32)))) & 16383) as usize];
			self.fVec62[0] = fTemp476;
			self.fRec345[0] = self.fVec62[1] - (fTemp472 + (2.0 - self.fRec346[0])) * (self.fRec345[1] - fTemp476) / (self.fRec346[0] - fTemp472);
			self.fRec343[0] = self.fRec345[0];
			self.fRec348[0] = 0.9999 * (self.fRec348[1] + (i32::wrapping_mul(iTemp315, iSlow77)) as F32) + fSlow78;
			let mut fTemp477: F32 = self.fRec348[0] + -1.49999;
			let mut fTemp478: F32 = F32::floor(fTemp477);
			let mut fTemp479: F32 = self.fRec348[0] - fTemp478;
			let mut fTemp480: F32 = fTemp478 + (2.0 - self.fRec348[0]);
			self.fVec63[(self.IOTA0 & 16383) as usize] = fTemp473 + fTemp475;
			let mut fTemp481: F32 = self.fVec63[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, (fTemp477) as i32)))) & 16383) as usize];
			self.fVec64[0] = fTemp481;
			self.fRec347[0] = 0.70710677 * (fTemp480 * fTemp481 / fTemp479 + self.fVec64[1]) - self.fRec347[1] * fTemp480 / fTemp479;
			self.fRec344[0] = self.fRec347[0];
			let mut fTemp482: F32 = 0.760314 * self.fRec343[1] + 0.64955574 * fTemp470;
			self.fVec65[(self.IOTA0 & 1023) as usize] = fTemp482;
			self.fRec349[0] = fSlow81 * self.fRec350[1] + fSlow80 * self.fRec349[1];
			self.fRec350[0] = (iTemp315) as F32 + fSlow80 * self.fRec350[1] - fSlow81 * self.fRec349[1];
			let mut fTemp483: F32 = fSlow82 * (self.fRec350[0] + 1.0);
			let mut fTemp484: F32 = fTemp483 + 3.500005;
			let mut iTemp485: i32 = (fTemp484) as i32;
			let mut iTemp486: i32 = std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp485, 4)));
			let mut fTemp487: F32 = F32::floor(fTemp484);
			let mut fTemp488: F32 = fTemp483 + (2.0 - fTemp487);
			let mut fTemp489: F32 = fTemp483 + (3.0 - fTemp487);
			let mut fTemp490: F32 = fTemp483 + (4.0 - fTemp487);
			let mut fTemp491: F32 = fTemp490 * fTemp489;
			let mut fTemp492: F32 = fTemp491 * fTemp488;
			let mut fTemp493: F32 = fTemp483 + (1.0 - fTemp487);
			let mut fTemp494: F32 = 0.0 - fTemp493;
			let mut iTemp495: i32 = std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp485, 3)));
			let mut fTemp496: F32 = 0.0 - 0.5 * fTemp493;
			let mut fTemp497: F32 = 0.0 - fTemp488;
			let mut iTemp498: i32 = std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp485, 2)));
			let mut fTemp499: F32 = 0.0 - 0.33333334 * fTemp493;
			let mut fTemp500: F32 = 0.0 - 0.5 * fTemp488;
			let mut fTemp501: F32 = 0.0 - fTemp489;
			let mut iTemp502: i32 = std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp485, 1)));
			let mut fTemp503: F32 = fTemp483 + (5.0 - fTemp487);
			let mut fTemp504: F32 = 0.0 - 0.25 * fTemp493;
			let mut fTemp505: F32 = 0.0 - 0.33333334 * fTemp488;
			let mut fTemp506: F32 = 0.0 - 0.5 * fTemp489;
			let mut fTemp507: F32 = 0.0 - fTemp490;
			let mut iTemp508: i32 = std::cmp::min(512, std::cmp::max(0, iTemp485));
			self.fVec66[(self.IOTA0 & 16383) as usize] = self.fVec65[((i32::wrapping_sub(self.IOTA0, iTemp508)) & 1023) as usize] * fTemp507 * fTemp506 * fTemp505 * fTemp504 + fTemp503 * (self.fVec65[((i32::wrapping_sub(self.IOTA0, iTemp502)) & 1023) as usize] * fTemp501 * fTemp500 * fTemp499 + 0.5 * fTemp490 * self.fVec65[((i32::wrapping_sub(self.IOTA0, iTemp498)) & 1023) as usize] * fTemp497 * fTemp496 + 0.16666667 * fTemp491 * self.fVec65[((i32::wrapping_sub(self.IOTA0, iTemp495)) & 1023) as usize] * fTemp494 + 0.041666668 * fTemp492 * self.fVec65[((i32::wrapping_sub(self.IOTA0, iTemp486)) & 1023) as usize]);
			let mut fTemp509: F32 = self.fVec66[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, (fTemp416) as i32)))) & 16383) as usize];
			self.fVec67[0] = fTemp509;
			self.fRec317[0] = self.fVec67[1] - (fTemp417 + (2.0 - self.fRec318[0])) * (self.fRec317[1] - fTemp509) / (self.fRec318[0] - fTemp417);
			self.fRec354[0] = 0.9999 * (self.fRec354[1] + (i32::wrapping_mul(iTemp315, iSlow83)) as F32) + fSlow84;
			let mut fTemp510: F32 = self.fRec354[0] + -1.49999;
			let mut fTemp511: F32 = F32::floor(fTemp510);
			let mut fTemp512: F32 = self.fRec354[0] - fTemp511;
			let mut fTemp513: F32 = fTemp511 + (2.0 - self.fRec354[0]);
			self.fRec356[0] = 0.995 * (self.fRec356[1] + (i32::wrapping_mul(iTemp315, iSlow85)) as F32) + fSlow86;
			let mut fTemp514: F32 = self.fRec356[0] + -1.49999;
			let mut fTemp515: F32 = F32::floor(fTemp514);
			let mut fTemp516: F32 = 0.760314 * self.fRec344[1] + 0.64955574 * fTemp474;
			self.fVec68[(self.IOTA0 & 1023) as usize] = fTemp516;
			let mut fTemp517: F32 = fSlow87 * self.fRec350[0];
			let mut fTemp518: F32 = fSlow82 + fTemp517 + 3.500005;
			let mut iTemp519: i32 = (fTemp518) as i32;
			let mut fTemp520: F32 = F32::floor(fTemp518);
			let mut fTemp521: F32 = fSlow82 + fTemp517 + (2.0 - fTemp520);
			let mut fTemp522: F32 = fSlow82 + fTemp517 + (3.0 - fTemp520);
			let mut fTemp523: F32 = fSlow82 + fTemp517 + (4.0 - fTemp520);
			let mut fTemp524: F32 = fTemp523 * fTemp522;
			let mut fTemp525: F32 = fSlow82 + fTemp517 + (1.0 - fTemp520);
			self.fVec69[(self.IOTA0 & 16383) as usize] = self.fVec68[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, std::cmp::max(0, iTemp519)))) & 1023) as usize] * (0.0 - fTemp523) * (0.0 - 0.5 * fTemp522) * (0.0 - 0.33333334 * fTemp521) * (0.0 - 0.25 * fTemp525) + (fSlow82 + fTemp517 + (5.0 - fTemp520)) * (self.fVec68[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp519, 1))))) & 1023) as usize] * (0.0 - fTemp522) * (0.0 - 0.5 * fTemp521) * (0.0 - 0.33333334 * fTemp525) + 0.5 * fTemp523 * self.fVec68[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp519, 2))))) & 1023) as usize] * (0.0 - fTemp521) * (0.0 - 0.5 * fTemp525) + 0.16666667 * fTemp524 * self.fVec68[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp519, 3))))) & 1023) as usize] * (0.0 - fTemp525) + 0.041666668 * fTemp524 * fTemp521 * self.fVec68[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp519, 4))))) & 1023) as usize]);
			let mut fTemp526: F32 = self.fVec69[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, (fTemp514) as i32)))) & 16383) as usize];
			self.fVec70[0] = fTemp526;
			self.fRec355[0] = self.fVec70[1] - (fTemp515 + (2.0 - self.fRec356[0])) * (self.fRec355[1] - fTemp526) / (self.fRec356[0] - fTemp515);
			let mut fTemp527: F32 = 0.760314 * self.fRec355[0] - 0.64955574 * self.fRec352[1];
			let mut fTemp528: F32 = 0.760314 * self.fRec317[0] - 0.64955574 * self.fRec351[1];
			self.fVec71[(self.IOTA0 & 16383) as usize] = fTemp528 - fTemp527;
			let mut fTemp529: F32 = self.fVec71[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, (fTemp510) as i32)))) & 16383) as usize];
			self.fVec72[0] = fTemp529;
			self.fRec353[0] = 0.70710677 * (fTemp513 * fTemp529 / fTemp512 + self.fVec72[1]) - self.fRec353[1] * fTemp513 / fTemp512;
			self.fRec351[0] = self.fRec353[0];
			self.fRec358[0] = 0.9999 * (self.fRec358[1] + (i32::wrapping_mul(iTemp315, iSlow88)) as F32) + fSlow89;
			let mut fTemp530: F32 = self.fRec358[0] + -1.49999;
			let mut fTemp531: F32 = F32::floor(fTemp530);
			let mut fTemp532: F32 = self.fRec358[0] - fTemp531;
			let mut fTemp533: F32 = fTemp531 + (2.0 - self.fRec358[0]);
			self.fVec73[(self.IOTA0 & 16383) as usize] = fTemp528 + fTemp527;
			let mut fTemp534: F32 = self.fVec73[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, (fTemp530) as i32)))) & 16383) as usize];
			self.fVec74[0] = fTemp534;
			self.fRec357[0] = 0.70710677 * (fTemp533 * fTemp534 / fTemp532 + self.fVec74[1]) - self.fRec357[1] * fTemp533 / fTemp532;
			self.fRec352[0] = self.fRec357[0];
			let mut fTemp535: F32 = 0.760314 * self.fRec351[1] + 0.64955574 * self.fRec317[0];
			self.fRec362[0] = 0.9999 * (self.fRec362[1] + (i32::wrapping_mul(iTemp315, iSlow90)) as F32) + fSlow91;
			let mut fTemp536: F32 = self.fRec362[0] + -1.49999;
			let mut fTemp537: F32 = F32::floor(fTemp536);
			let mut fTemp538: F32 = 0.760314 * fTemp535 - 0.64955574 * self.fRec359[1];
			let mut fTemp539: F32 = 0.760314 * self.fRec352[1] + 0.64955574 * self.fRec355[0];
			let mut fTemp540: F32 = 0.760314 * fTemp539 - 0.64955574 * self.fRec360[1];
			self.fVec75[(self.IOTA0 & 16383) as usize] = 0.0 - 0.70710677 * (fTemp540 - fTemp538);
			let mut fTemp541: F32 = self.fVec75[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, (fTemp536) as i32)))) & 16383) as usize];
			self.fVec76[0] = fTemp541;
			self.fRec361[0] = self.fVec76[1] - (fTemp537 + (2.0 - self.fRec362[0])) * (self.fRec361[1] - fTemp541) / (self.fRec362[0] - fTemp537);
			self.fRec359[0] = self.fRec361[0];
			self.fRec364[0] = 0.9999 * (self.fRec364[1] + (i32::wrapping_mul(iTemp315, iSlow92)) as F32) + fSlow93;
			let mut fTemp542: F32 = self.fRec364[0] + -1.49999;
			let mut fTemp543: F32 = F32::floor(fTemp542);
			let mut fTemp544: F32 = self.fRec364[0] - fTemp543;
			let mut fTemp545: F32 = fTemp543 + (2.0 - self.fRec364[0]);
			self.fVec77[(self.IOTA0 & 16383) as usize] = fTemp538 + fTemp540;
			let mut iTemp546: i32 = std::cmp::min(8192, std::cmp::max(0, (fTemp542) as i32));
			let mut fTemp547: F32 = self.fVec77[((i32::wrapping_sub(self.IOTA0, iTemp546)) & 16383) as usize];
			self.fVec78[0] = fTemp547;
			self.fRec363[0] = 0.70710677 * (fTemp545 * fTemp547 / fTemp544 + self.fVec78[1]) - fTemp545 * self.fRec363[1] / fTemp544;
			self.fRec360[0] = self.fRec363[0];
			let mut fTemp548: F32 = 0.760314 * self.fRec359[1] + 0.64955574 * fTemp535;
			self.fRec368[0] = 0.9999 * (self.fRec368[1] + (i32::wrapping_mul(iTemp315, iSlow94)) as F32) + fSlow95;
			let mut fTemp549: F32 = self.fRec368[0] + -1.49999;
			let mut fTemp550: F32 = F32::floor(fTemp549);
			let mut fTemp551: F32 = self.fRec368[0] - fTemp550;
			let mut fTemp552: F32 = fTemp550 + (2.0 - self.fRec368[0]);
			let mut fTemp553: F32 = 0.760314 * self.fRec360[1] + 0.64955574 * fTemp539;
			let mut fTemp554: F32 = 0.760314 * fTemp553 - 0.64955574 * self.fRec366[1];
			let mut fTemp555: F32 = 0.760314 * fTemp548 - 0.64955574 * self.fRec365[1];
			self.fVec79[(self.IOTA0 & 16383) as usize] = fTemp555 - fTemp554;
			let mut fTemp556: F32 = self.fVec79[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, (fTemp549) as i32)))) & 16383) as usize];
			self.fVec80[0] = fTemp556;
			self.fRec367[0] = 0.70710677 * (fTemp552 * fTemp556 / fTemp551 + self.fVec80[1]) - self.fRec367[1] * fTemp552 / fTemp551;
			self.fRec365[0] = self.fRec367[0];
			self.fRec370[0] = 0.9999 * (self.fRec370[1] + (i32::wrapping_mul(iTemp315, iSlow96)) as F32) + fSlow97;
			let mut fTemp557: F32 = self.fRec370[0] + -1.49999;
			let mut fTemp558: F32 = F32::floor(fTemp557);
			let mut fTemp559: F32 = self.fRec370[0] - fTemp558;
			let mut fTemp560: F32 = fTemp558 + (2.0 - self.fRec370[0]);
			self.fVec81[(self.IOTA0 & 16383) as usize] = fTemp555 + fTemp554;
			let mut iTemp561: i32 = std::cmp::min(8192, std::cmp::max(0, (fTemp557) as i32));
			let mut fTemp562: F32 = self.fVec81[((i32::wrapping_sub(self.IOTA0, iTemp561)) & 16383) as usize];
			self.fVec82[0] = fTemp562;
			self.fRec369[0] = 0.70710677 * (fTemp560 * fTemp562 / fTemp559 + self.fVec82[1]) - self.fRec369[1] * fTemp560 / fTemp559;
			self.fRec366[0] = self.fRec369[0];
			let mut fTemp563: F32 = 0.760314 * self.fRec365[1] + 0.64955574 * fTemp548;
			self.fRec374[0] = 0.9999 * (self.fRec374[1] + (i32::wrapping_mul(iTemp315, iSlow98)) as F32) + fSlow99;
			let mut fTemp564: F32 = self.fRec374[0] + -1.49999;
			let mut fTemp565: F32 = F32::floor(fTemp564);
			let mut fTemp566: F32 = self.fRec374[0] - fTemp565;
			let mut fTemp567: F32 = 0.760314 * fTemp563 - 0.64955574 * self.fRec371[1];
			let mut fTemp568: F32 = 0.760314 * self.fRec366[1] + 0.64955574 * fTemp553;
			let mut fTemp569: F32 = 0.760314 * fTemp568 - 0.64955574 * self.fRec372[1];
			self.fVec83[(self.IOTA0 & 16383) as usize] = 0.0 - 0.70710677 * (fTemp569 - fTemp567);
			let mut iTemp570: i32 = std::cmp::min(8192, std::cmp::max(0, (fTemp564) as i32));
			let mut fTemp571: F32 = self.fVec83[((i32::wrapping_sub(self.IOTA0, iTemp570)) & 16383) as usize];
			self.fVec84[0] = fTemp571;
			let mut fTemp572: F32 = fTemp565 + (2.0 - self.fRec374[0]);
			self.fRec373[0] = self.fVec84[1] - fTemp572 * (self.fRec373[1] - fTemp571) / fTemp566;
			self.fRec371[0] = self.fRec373[0];
			self.fRec376[0] = 0.9999 * (self.fRec376[1] + (i32::wrapping_mul(iTemp315, iSlow100)) as F32) + fSlow101;
			let mut fTemp573: F32 = self.fRec376[0] + -1.49999;
			let mut fTemp574: F32 = F32::floor(fTemp573);
			let mut fTemp575: F32 = self.fRec376[0] - fTemp574;
			let mut fTemp576: F32 = fTemp574 + (2.0 - self.fRec376[0]);
			self.fVec85[(self.IOTA0 & 16383) as usize] = fTemp567 + fTemp569;
			let mut iTemp577: i32 = std::cmp::min(8192, std::cmp::max(0, (fTemp573) as i32));
			let mut fTemp578: F32 = self.fVec85[((i32::wrapping_sub(self.IOTA0, iTemp577)) & 16383) as usize];
			self.fVec86[0] = fTemp578;
			self.fRec375[0] = 0.70710677 * (fTemp576 * fTemp578 / fTemp575 + self.fVec86[1]) - fTemp576 * self.fRec375[1] / fTemp575;
			self.fRec372[0] = self.fRec375[0];
			let mut fTemp579: F32 = 0.760314 * self.fRec371[1] + 0.64955574 * fTemp563;
			self.fRec380[0] = 0.9999 * (self.fRec380[1] + (i32::wrapping_mul(iTemp315, iSlow102)) as F32) + fSlow103;
			let mut fTemp580: F32 = self.fRec380[0] + -1.49999;
			let mut fTemp581: F32 = F32::floor(fTemp580);
			let mut fTemp582: F32 = self.fRec380[0] - fTemp581;
			let mut fTemp583: F32 = fTemp581 + (2.0 - self.fRec380[0]);
			let mut fTemp584: F32 = 0.760314 * self.fRec372[1] + 0.64955574 * fTemp568;
			let mut fTemp585: F32 = 0.760314 * fTemp584 - 0.64955574 * self.fRec378[1];
			let mut fTemp586: F32 = 0.760314 * fTemp579 - 0.64955574 * self.fRec377[1];
			self.fVec87[(self.IOTA0 & 16383) as usize] = fTemp586 - fTemp585;
			let mut fTemp587: F32 = self.fVec87[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, (fTemp580) as i32)))) & 16383) as usize];
			self.fVec88[0] = fTemp587;
			self.fRec379[0] = 0.70710677 * (fTemp583 * fTemp587 / fTemp582 + self.fVec88[1]) - self.fRec379[1] * fTemp583 / fTemp582;
			self.fRec377[0] = self.fRec379[0];
			self.fRec382[0] = 0.9999 * (self.fRec382[1] + (i32::wrapping_mul(iTemp315, iSlow104)) as F32) + fSlow105;
			let mut fTemp588: F32 = self.fRec382[0] + -1.49999;
			let mut fTemp589: F32 = F32::floor(fTemp588);
			let mut fTemp590: F32 = self.fRec382[0] - fTemp589;
			let mut fTemp591: F32 = fTemp589 + (2.0 - self.fRec382[0]);
			self.fVec89[(self.IOTA0 & 16383) as usize] = fTemp586 + fTemp585;
			let mut iTemp592: i32 = std::cmp::min(8192, std::cmp::max(0, (fTemp588) as i32));
			let mut fTemp593: F32 = self.fVec89[((i32::wrapping_sub(self.IOTA0, iTemp592)) & 16383) as usize];
			self.fVec90[0] = fTemp593;
			self.fRec381[0] = 0.70710677 * (fTemp591 * fTemp593 / fTemp590 + self.fVec90[1]) - self.fRec381[1] * fTemp591 / fTemp590;
			self.fRec378[0] = self.fRec381[0];
			let mut fTemp594: F32 = 0.760314 * self.fRec377[1] + 0.64955574 * fTemp579;
			self.fVec91[(self.IOTA0 & 16383) as usize] = fTemp594;
			let mut fTemp595: F32 = fSlow82 * (self.fRec349[0] + 1.0);
			let mut fTemp596: F32 = fTemp595 + 3.500005;
			let mut iTemp597: i32 = (fTemp596) as i32;
			let mut iTemp598: i32 = std::cmp::max(0, i32::wrapping_add(iTemp597, 4));
			let mut fTemp599: F32 = F32::floor(fTemp596);
			let mut fTemp600: F32 = fTemp595 + (2.0 - fTemp599);
			let mut fTemp601: F32 = fTemp595 + (3.0 - fTemp599);
			let mut fTemp602: F32 = fTemp595 + (4.0 - fTemp599);
			let mut fTemp603: F32 = fTemp602 * fTemp601;
			let mut fTemp604: F32 = fTemp603 * fTemp600;
			let mut fTemp605: F32 = fTemp595 + (1.0 - fTemp599);
			let mut fTemp606: F32 = 0.0 - fTemp605;
			let mut iTemp607: i32 = std::cmp::max(0, i32::wrapping_add(iTemp597, 3));
			let mut fTemp608: F32 = 0.0 - 0.5 * fTemp605;
			let mut fTemp609: F32 = 0.0 - fTemp600;
			let mut iTemp610: i32 = std::cmp::max(0, i32::wrapping_add(iTemp597, 2));
			let mut fTemp611: F32 = 0.0 - 0.33333334 * fTemp605;
			let mut fTemp612: F32 = 0.0 - 0.5 * fTemp600;
			let mut fTemp613: F32 = 0.0 - fTemp601;
			let mut iTemp614: i32 = std::cmp::max(0, i32::wrapping_add(iTemp597, 1));
			let mut fTemp615: F32 = fTemp595 + (5.0 - fTemp599);
			let mut fTemp616: F32 = 0.0 - 0.25 * fTemp605;
			let mut fTemp617: F32 = 0.0 - 0.33333334 * fTemp600;
			let mut fTemp618: F32 = 0.0 - 0.5 * fTemp601;
			let mut fTemp619: F32 = 0.0 - fTemp602;
			let mut iTemp620: i32 = std::cmp::max(0, iTemp597);
			self.fVec92[(self.IOTA0 & 16383) as usize] = self.fVec91[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, iTemp620))) & 16383) as usize] * fTemp619 * fTemp618 * fTemp617 * fTemp616 + fTemp615 * (self.fVec91[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, iTemp614))) & 16383) as usize] * fTemp613 * fTemp612 * fTemp611 + 0.5 * fTemp602 * self.fVec91[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, iTemp610))) & 16383) as usize] * fTemp609 * fTemp608 + 0.16666667 * fTemp603 * self.fVec91[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, iTemp607))) & 16383) as usize] * fTemp606 + 0.041666668 * fTemp604 * self.fVec91[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, iTemp598))) & 16383) as usize]);
			let mut fTemp621: F32 = self.fVec92[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, (fTemp414) as i32)))) & 16383) as usize];
			self.fVec93[0] = fTemp621;
			self.fRec315[0] = self.fVec93[1] - (fTemp415 + (2.0 - self.fRec316[0])) * (self.fRec315[1] - fTemp621) / (self.fRec316[0] - fTemp415);
			self.fRec314[0] = 0.0 - self.fConst40 * (self.fConst38 * self.fRec314[1] - (self.fRec315[0] + self.fRec315[1]));
			self.fRec313[0] = self.fRec314[0] - self.fConst36 * (self.fConst35 * self.fRec313[2] + self.fConst31 * self.fRec313[1]);
			self.fRec312[0] = self.fConst36 * (self.fRec313[2] + self.fRec313[0] + 2.0 * self.fRec313[1]) - self.fConst34 * (self.fConst33 * self.fRec312[2] + self.fConst31 * self.fRec312[1]);
			let mut fTemp622: F32 = self.fRec312[2] + self.fRec312[0] + 2.0 * self.fRec312[1];
			self.fVec94[0] = fTemp622;
			self.fRec311[0] = self.fConst42 * (self.fConst34 * (fTemp622 + self.fVec94[1]) - self.fConst27 * self.fRec311[1]);
			self.fRec310[0] = self.fRec311[0] - self.fConst26 * (self.fConst25 * self.fRec310[2] + self.fConst21 * self.fRec310[1]);
			self.fRec309[0] = self.fConst26 * (self.fRec310[2] + self.fRec310[0] + 2.0 * self.fRec310[1]) - self.fConst24 * (self.fConst23 * self.fRec309[2] + self.fConst21 * self.fRec309[1]);
			self.fRec385[0] = self.fConst34 * (self.fConst44 * fTemp622 + self.fConst45 * self.fVec94[1]) - self.fConst43 * self.fRec385[1];
			self.fRec384[0] = self.fRec385[0] - self.fConst26 * (self.fConst25 * self.fRec384[2] + self.fConst21 * self.fRec384[1]);
			self.fRec383[0] = self.fConst26 * (self.fConst46 * self.fRec384[1] + self.fConst20 * self.fRec384[0] + self.fConst20 * self.fRec384[2]) - self.fConst24 * (self.fConst23 * self.fRec383[2] + self.fConst21 * self.fRec383[1]);
			let mut fTemp623: F32 = self.fConst21 * self.fRec386[1];
			self.fRec389[0] = self.fConst49 * self.fRec315[1] - self.fConst40 * (self.fConst38 * self.fRec389[1] - self.fConst32 * self.fRec315[0]);
			self.fRec388[0] = self.fRec389[0] - self.fConst36 * (self.fConst35 * self.fRec388[2] + self.fConst31 * self.fRec388[1]);
			self.fRec387[0] = self.fConst36 * (self.fConst50 * self.fRec388[1] + self.fConst30 * self.fRec388[0] + self.fConst30 * self.fRec388[2]) - self.fConst34 * (self.fConst33 * self.fRec387[2] + self.fConst31 * self.fRec387[1]);
			self.fRec386[0] = self.fConst34 * (self.fConst30 * self.fRec387[0] + self.fConst50 * self.fRec387[1] + self.fConst30 * self.fRec387[2]) - self.fConst48 * (self.fConst47 * self.fRec386[2] + fTemp623);
			let mut fTemp624: F32 = fTemp411 + fTemp412 + fSlow106 * (self.fRec386[2] + self.fConst48 * (fTemp623 + self.fConst47 * self.fRec386[0]) + self.fConst24 * (self.fConst20 * self.fRec383[0] + self.fConst46 * self.fRec383[1] + self.fConst20 * self.fRec383[2] + self.fRec309[2] + self.fRec309[0] + 2.0 * self.fRec309[1]));
			self.fVec95[(self.IOTA0 & 1023) as usize] = fTemp624;
			self.fRec308[0] = fSlow107 * (fTemp619 * fTemp618 * fTemp617 * fTemp616 * self.fVec95[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, iTemp620))) & 1023) as usize] + fTemp615 * (fTemp613 * fTemp612 * fTemp611 * self.fVec95[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, iTemp614))) & 1023) as usize] + 0.5 * fTemp602 * fTemp609 * fTemp608 * self.fVec95[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, iTemp610))) & 1023) as usize] + 0.16666667 * fTemp603 * fTemp606 * self.fVec95[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, iTemp607))) & 1023) as usize] + 0.041666668 * fTemp604 * self.fVec95[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, iTemp598))) & 1023) as usize])) + fSlow53 * self.fRec308[1];
			self.fRec401[0] = 0.995 * (self.fRec401[1] + (i32::wrapping_mul(iTemp315, iSlow110)) as F32) + fSlow111;
			let mut fTemp625: F32 = self.fRec401[0] + -1.49999;
			let mut fTemp626: F32 = F32::floor(fTemp625);
			let mut fTemp627: F32 = 0.760314 * self.fRec378[1] + 0.64955574 * fTemp584;
			self.fVec96[(self.IOTA0 & 16383) as usize] = fTemp627;
			let mut fTemp628: F32 = fSlow87 * self.fRec349[0];
			let mut fTemp629: F32 = fSlow82 + fTemp628 + 3.500005;
			let mut iTemp630: i32 = (fTemp629) as i32;
			let mut fTemp631: F32 = F32::floor(fTemp629);
			let mut fTemp632: F32 = fSlow82 + fTemp628 + (2.0 - fTemp631);
			let mut fTemp633: F32 = fSlow82 + fTemp628 + (3.0 - fTemp631);
			let mut fTemp634: F32 = fSlow82 + fTemp628 + (4.0 - fTemp631);
			let mut fTemp635: F32 = fTemp634 * fTemp633;
			let mut fTemp636: F32 = fSlow82 + fTemp628 + (1.0 - fTemp631);
			self.fVec97[(self.IOTA0 & 16383) as usize] = self.fVec96[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, iTemp630)))) & 16383) as usize] * (0.0 - fTemp634) * (0.0 - 0.5 * fTemp633) * (0.0 - 0.33333334 * fTemp632) * (0.0 - 0.25 * fTemp636) + (fSlow82 + fTemp628 + (5.0 - fTemp631)) * (self.fVec96[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, i32::wrapping_add(iTemp630, 1))))) & 16383) as usize] * (0.0 - fTemp633) * (0.0 - 0.5 * fTemp632) * (0.0 - 0.33333334 * fTemp636) + 0.5 * fTemp634 * self.fVec96[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, i32::wrapping_add(iTemp630, 2))))) & 16383) as usize] * (0.0 - fTemp632) * (0.0 - 0.5 * fTemp636) + 0.16666667 * fTemp635 * self.fVec96[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, i32::wrapping_add(iTemp630, 3))))) & 16383) as usize] * (0.0 - fTemp636) + 0.041666668 * fTemp635 * fTemp632 * self.fVec96[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, i32::wrapping_add(iTemp630, 4))))) & 16383) as usize]);
			let mut fTemp637: F32 = self.fVec97[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, (fTemp625) as i32)))) & 16383) as usize];
			self.fVec98[0] = fTemp637;
			self.fRec400[0] = self.fVec98[1] - (fTemp626 + (2.0 - self.fRec401[0])) * (self.fRec400[1] - fTemp637) / (self.fRec401[0] - fTemp626);
			self.fRec399[0] = 0.0 - self.fConst40 * (self.fConst38 * self.fRec399[1] - (self.fRec400[0] + self.fRec400[1]));
			self.fRec398[0] = self.fRec399[0] - self.fConst36 * (self.fConst35 * self.fRec398[2] + self.fConst31 * self.fRec398[1]);
			self.fRec397[0] = self.fConst36 * (self.fRec398[2] + self.fRec398[0] + 2.0 * self.fRec398[1]) - self.fConst34 * (self.fConst33 * self.fRec397[2] + self.fConst31 * self.fRec397[1]);
			let mut fTemp638: F32 = self.fRec397[2] + self.fRec397[0] + 2.0 * self.fRec397[1];
			self.fVec99[0] = fTemp638;
			self.fRec396[0] = 0.0 - self.fConst42 * (self.fConst27 * self.fRec396[1] - self.fConst34 * (fTemp638 + self.fVec99[1]));
			self.fRec395[0] = self.fRec396[0] - self.fConst26 * (self.fConst25 * self.fRec395[2] + self.fConst21 * self.fRec395[1]);
			self.fRec394[0] = self.fConst26 * (self.fRec395[2] + self.fRec395[0] + 2.0 * self.fRec395[1]) - self.fConst24 * (self.fConst23 * self.fRec394[2] + self.fConst21 * self.fRec394[1]);
			self.fRec404[0] = self.fConst34 * (self.fConst45 * self.fVec99[1] + self.fConst44 * fTemp638) - self.fConst43 * self.fRec404[1];
			self.fRec403[0] = self.fRec404[0] - self.fConst26 * (self.fConst25 * self.fRec403[2] + self.fConst21 * self.fRec403[1]);
			self.fRec402[0] = self.fConst26 * (self.fConst46 * self.fRec403[1] + self.fConst20 * self.fRec403[0] + self.fConst20 * self.fRec403[2]) - self.fConst24 * (self.fConst23 * self.fRec402[2] + self.fConst21 * self.fRec402[1]);
			let mut fTemp639: F32 = self.fConst21 * self.fRec405[1];
			self.fRec408[0] = self.fConst49 * self.fRec400[1] - self.fConst40 * (self.fConst38 * self.fRec408[1] - self.fConst32 * self.fRec400[0]);
			self.fRec407[0] = self.fRec408[0] - self.fConst36 * (self.fConst35 * self.fRec407[2] + self.fConst31 * self.fRec407[1]);
			self.fRec406[0] = self.fConst36 * (self.fConst50 * self.fRec407[1] + self.fConst30 * self.fRec407[0] + self.fConst30 * self.fRec407[2]) - self.fConst34 * (self.fConst33 * self.fRec406[2] + self.fConst31 * self.fRec406[1]);
			self.fRec405[0] = self.fConst34 * (self.fConst50 * self.fRec406[1] + self.fConst30 * self.fRec406[0] + self.fConst30 * self.fRec406[2]) - self.fConst48 * (self.fConst47 * self.fRec405[2] + fTemp639);
			let mut fTemp640: F32 = fTemp413 + fSlow106 * (self.fRec405[2] + self.fConst48 * (fTemp639 + self.fConst47 * self.fRec405[0]) + self.fConst24 * (self.fConst20 * self.fRec402[0] + self.fConst46 * self.fRec402[1] + self.fConst20 * self.fRec402[2] + self.fRec394[2] + self.fRec394[0] + 2.0 * self.fRec394[1]));
			self.fVec100[(self.IOTA0 & 1023) as usize] = fTemp640;
			self.fRec393[0] = fSlow107 * (fTemp507 * fTemp506 * fTemp505 * fTemp504 * self.fVec100[((i32::wrapping_sub(self.IOTA0, iTemp508)) & 1023) as usize] + fTemp503 * (fTemp501 * fTemp500 * fTemp499 * self.fVec100[((i32::wrapping_sub(self.IOTA0, iTemp502)) & 1023) as usize] + 0.5 * fTemp490 * fTemp497 * fTemp496 * self.fVec100[((i32::wrapping_sub(self.IOTA0, iTemp498)) & 1023) as usize] + 0.16666667 * fTemp491 * fTemp494 * self.fVec100[((i32::wrapping_sub(self.IOTA0, iTemp495)) & 1023) as usize] + 0.041666668 * fTemp492 * self.fVec100[((i32::wrapping_sub(self.IOTA0, iTemp486)) & 1023) as usize])) + fSlow53 * self.fRec393[1];
			let mut fTemp641: F32 = fSlow112 * self.fRec393[0] - fSlow109 * self.fRec391[1];
			let mut fTemp642: F32 = fSlow112 * self.fRec308[0] - fSlow109 * self.fRec390[1];
			self.fVec101[(self.IOTA0 & 16383) as usize] = fTemp642 - fTemp641;
			let mut fTemp643: F32 = self.fVec101[((i32::wrapping_sub(self.IOTA0, iTemp546)) & 16383) as usize];
			self.fVec102[0] = fTemp643;
			self.fRec392[0] = 0.70710677 * (fTemp545 * fTemp643 / fTemp544 + self.fVec102[1]) - self.fRec392[1] * fTemp545 / fTemp544;
			self.fRec390[0] = self.fRec392[0];
			self.fRec410[0] = 0.9999 * (self.fRec410[1] + (i32::wrapping_mul(iTemp315, iSlow113)) as F32) + fSlow114;
			let mut fTemp644: F32 = self.fRec410[0] + -1.49999;
			let mut fTemp645: F32 = F32::floor(fTemp644);
			let mut fTemp646: F32 = self.fRec410[0] - fTemp645;
			let mut fTemp647: F32 = fTemp645 + (2.0 - self.fRec410[0]);
			self.fVec103[(self.IOTA0 & 16383) as usize] = fTemp642 + fTemp641;
			let mut fTemp648: F32 = self.fVec103[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, (fTemp644) as i32)))) & 16383) as usize];
			self.fVec104[0] = fTemp648;
			self.fRec409[0] = 0.70710677 * (fTemp647 * fTemp648 / fTemp646 + self.fVec104[1]) - self.fRec409[1] * fTemp647 / fTemp646;
			self.fRec391[0] = self.fRec409[0];
			let mut fTemp649: F32 = fSlow112 * self.fRec390[1] + fSlow109 * self.fRec308[0];
			let mut fTemp650: F32 = fSlow112 * self.fRec391[1] + fSlow109 * self.fRec393[0];
			let mut fTemp651: F32 = fSlow112 * fTemp650 - fSlow109 * self.fRec412[1];
			let mut fTemp652: F32 = fSlow112 * fTemp649 - fSlow109 * self.fRec411[1];
			self.fVec105[(self.IOTA0 & 16383) as usize] = fTemp652 - fTemp651;
			let mut fTemp653: F32 = self.fVec105[((i32::wrapping_sub(self.IOTA0, iTemp570)) & 16383) as usize];
			self.fVec106[0] = fTemp653;
			self.fRec413[0] = 0.70710677 * (fTemp572 * fTemp653 / fTemp566 + self.fVec106[1]) - self.fRec413[1] * fTemp572 / fTemp566;
			self.fRec411[0] = self.fRec413[0];
			self.fVec107[(self.IOTA0 & 16383) as usize] = fTemp652 + fTemp651;
			let mut fTemp654: F32 = self.fVec107[((i32::wrapping_sub(self.IOTA0, iTemp561)) & 16383) as usize];
			self.fVec108[0] = fTemp654;
			self.fRec414[0] = 0.70710677 * (fTemp560 * fTemp654 / fTemp559 + self.fVec108[1]) - fTemp560 * self.fRec414[1] / fTemp559;
			self.fRec412[0] = self.fRec414[0];
			let mut fTemp655: F32 = fSlow112 * self.fRec411[1] + fSlow109 * fTemp649;
			let mut fTemp656: F32 = fSlow112 * fTemp655 - fSlow109 * self.fRec415[1];
			let mut fTemp657: F32 = fSlow112 * self.fRec412[1] + fSlow109 * fTemp650;
			let mut fTemp658: F32 = fSlow112 * fTemp657 - fSlow109 * self.fRec416[1];
			self.fVec109[(self.IOTA0 & 16383) as usize] = 0.0 - 0.70710677 * (fTemp658 - fTemp656);
			let mut fTemp659: F32 = self.fVec109[((i32::wrapping_sub(self.IOTA0, iTemp577)) & 16383) as usize];
			self.fVec110[0] = fTemp659;
			self.fRec417[0] = self.fVec110[1] - fTemp576 * (self.fRec417[1] - fTemp659) / fTemp575;
			self.fRec415[0] = self.fRec417[0];
			self.fRec419[0] = 0.9999 * (self.fRec419[1] + (i32::wrapping_mul(iTemp315, iSlow115)) as F32) + fSlow116;
			let mut fTemp660: F32 = self.fRec419[0] + -1.49999;
			let mut fTemp661: F32 = F32::floor(fTemp660);
			let mut fTemp662: F32 = self.fRec419[0] - fTemp661;
			let mut fTemp663: F32 = fTemp661 + (2.0 - self.fRec419[0]);
			self.fVec111[(self.IOTA0 & 16383) as usize] = fTemp656 + fTemp658;
			let mut fTemp664: F32 = self.fVec111[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, (fTemp660) as i32)))) & 16383) as usize];
			self.fVec112[0] = fTemp664;
			self.fRec418[0] = 0.70710677 * (fTemp663 * fTemp664 / fTemp662 + self.fVec112[1]) - self.fRec418[1] * fTemp663 / fTemp662;
			self.fRec416[0] = self.fRec418[0];
			let mut fTemp665: F32 = fSlow112 * self.fRec415[1] + fSlow109 * fTemp655;
			self.fRec423[0] = 0.9999 * (self.fRec423[1] + (i32::wrapping_mul(iTemp315, iSlow117)) as F32) + fSlow118;
			let mut fTemp666: F32 = self.fRec423[0] + -1.49999;
			let mut fTemp667: F32 = F32::floor(fTemp666);
			let mut fTemp668: F32 = self.fRec423[0] - fTemp667;
			let mut fTemp669: F32 = fTemp667 + (2.0 - self.fRec423[0]);
			let mut fTemp670: F32 = fSlow112 * self.fRec416[1] + fSlow109 * fTemp657;
			let mut fTemp671: F32 = fSlow112 * fTemp670 - fSlow109 * self.fRec421[1];
			let mut fTemp672: F32 = fSlow112 * fTemp665 - fSlow109 * self.fRec420[1];
			self.fVec113[(self.IOTA0 & 16383) as usize] = fTemp672 - fTemp671;
			let mut fTemp673: F32 = self.fVec113[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, (fTemp666) as i32)))) & 16383) as usize];
			self.fVec114[0] = fTemp673;
			self.fRec422[0] = 0.70710677 * (fTemp669 * fTemp673 / fTemp668 + self.fVec114[1]) - self.fRec422[1] * fTemp669 / fTemp668;
			self.fRec420[0] = self.fRec422[0];
			self.fVec115[(self.IOTA0 & 16383) as usize] = fTemp672 + fTemp671;
			let mut fTemp674: F32 = self.fVec115[((i32::wrapping_sub(self.IOTA0, iTemp592)) & 16383) as usize];
			self.fVec116[0] = fTemp674;
			self.fRec424[0] = 0.70710677 * (fTemp591 * fTemp674 / fTemp590 + self.fVec116[1]) - fTemp591 * self.fRec424[1] / fTemp590;
			self.fRec421[0] = self.fRec424[0];
			self.fRec306[0] = fSlow112 * self.fRec420[1] + fSlow109 * fTemp665;
			self.fRec307[0] = fSlow112 * self.fRec421[1] + fSlow109 * fTemp670;
			self.fRec425[0] = fSlow119 + self.fConst2 * self.fRec425[1];
			let mut fTemp675: F32 = self.fRec425[0] * (fSlow51 * (self.fRec306[0] + self.fRec307[0]) + fSlow52 * fTemp413);
			*output0 = fTemp675;
			*output1 = fTemp675;
			self.iVec0[1] = self.iVec0[0];
			self.fRec0[1] = self.fRec0[0];
			self.fRec3[1] = self.fRec3[0];
			self.fRec1[1] = self.fRec1[0];
			self.IOTA0 = i32::wrapping_add(self.IOTA0, 1);
			self.fRec39[1] = self.fRec39[0];
			self.fRec38[1] = self.fRec38[0];
			self.fRec34[1] = self.fRec34[0];
			self.fRec40[1] = self.fRec40[0];
			self.fRec43[1] = self.fRec43[0];
			self.iVec1[1] = self.iVec1[0];
			self.iRec44[1] = self.iRec44[0];
			self.fRec42[1] = self.fRec42[0];
			for j0 in (1..=3).rev() {
				self.fRec45[j0 as usize] = self.fRec45[(i32::wrapping_sub(j0, 1)) as usize];
			}
			self.fVec2[1] = self.fVec2[0];
			self.fVec3[1] = self.fVec3[0];
			self.iRec47[1] = self.iRec47[0];
			self.iRec49[1] = self.iRec49[0];
			self.fRec48[2] = self.fRec48[1];
			self.fRec48[1] = self.fRec48[0];
			self.fVec4[2] = self.fVec4[1];
			self.fVec4[1] = self.fVec4[0];
			self.fRec30[1] = self.fRec30[0];
			self.fRec26[1] = self.fRec26[0];
			self.fRec24[1] = self.fRec24[0];
			for j1 in (1..=3).rev() {
				self.fRec20[j1 as usize] = self.fRec20[(i32::wrapping_sub(j1, 1)) as usize];
			}
			self.fRec15[1] = self.fRec15[0];
			self.fRec9[1] = self.fRec9[0];
			self.fRec8[1] = self.fRec8[0];
			self.fRec6[1] = self.fRec6[0];
			self.fRec5[2] = self.fRec5[1];
			self.fRec5[1] = self.fRec5[0];
			self.fRec4[2] = self.fRec4[1];
			self.fRec4[1] = self.fRec4[0];
			self.fRec50[1] = self.fRec50[0];
			self.fRec85[1] = self.fRec85[0];
			self.fRec81[1] = self.fRec81[0];
			self.fRec86[1] = self.fRec86[0];
			self.fRec89[1] = self.fRec89[0];
			self.iVec5[1] = self.iVec5[0];
			self.iRec90[1] = self.iRec90[0];
			self.fRec88[1] = self.fRec88[0];
			for j2 in (1..=3).rev() {
				self.fRec91[j2 as usize] = self.fRec91[(i32::wrapping_sub(j2, 1)) as usize];
			}
			self.fVec6[1] = self.fVec6[0];
			self.fVec7[1] = self.fVec7[0];
			self.iRec93[1] = self.iRec93[0];
			self.fRec94[2] = self.fRec94[1];
			self.fRec94[1] = self.fRec94[0];
			self.fVec8[2] = self.fVec8[1];
			self.fVec8[1] = self.fVec8[0];
			self.fRec77[1] = self.fRec77[0];
			self.fRec73[1] = self.fRec73[0];
			self.fRec71[1] = self.fRec71[0];
			for j3 in (1..=3).rev() {
				self.fRec67[j3 as usize] = self.fRec67[(i32::wrapping_sub(j3, 1)) as usize];
			}
			self.fRec62[1] = self.fRec62[0];
			self.fRec56[1] = self.fRec56[0];
			self.fRec55[1] = self.fRec55[0];
			self.fRec53[1] = self.fRec53[0];
			self.fRec52[2] = self.fRec52[1];
			self.fRec52[1] = self.fRec52[0];
			self.fRec51[2] = self.fRec51[1];
			self.fRec51[1] = self.fRec51[0];
			self.fRec95[1] = self.fRec95[0];
			self.fRec130[1] = self.fRec130[0];
			self.fRec126[1] = self.fRec126[0];
			self.fRec131[1] = self.fRec131[0];
			self.fRec134[1] = self.fRec134[0];
			self.iVec9[1] = self.iVec9[0];
			self.iRec135[1] = self.iRec135[0];
			self.fRec133[1] = self.fRec133[0];
			for j4 in (1..=3).rev() {
				self.fRec136[j4 as usize] = self.fRec136[(i32::wrapping_sub(j4, 1)) as usize];
			}
			self.fVec10[1] = self.fVec10[0];
			self.fVec11[1] = self.fVec11[0];
			self.iRec138[1] = self.iRec138[0];
			self.fRec139[2] = self.fRec139[1];
			self.fRec139[1] = self.fRec139[0];
			self.fVec12[2] = self.fVec12[1];
			self.fVec12[1] = self.fVec12[0];
			self.fRec122[1] = self.fRec122[0];
			self.fRec118[1] = self.fRec118[0];
			self.fRec116[1] = self.fRec116[0];
			for j5 in (1..=3).rev() {
				self.fRec112[j5 as usize] = self.fRec112[(i32::wrapping_sub(j5, 1)) as usize];
			}
			self.fRec107[1] = self.fRec107[0];
			self.fRec101[1] = self.fRec101[0];
			self.fRec100[1] = self.fRec100[0];
			self.fRec98[1] = self.fRec98[0];
			self.fRec97[2] = self.fRec97[1];
			self.fRec97[1] = self.fRec97[0];
			self.fRec96[2] = self.fRec96[1];
			self.fRec96[1] = self.fRec96[0];
			self.fRec140[1] = self.fRec140[0];
			self.fRec175[1] = self.fRec175[0];
			self.fRec171[1] = self.fRec171[0];
			self.fRec176[1] = self.fRec176[0];
			self.fRec179[1] = self.fRec179[0];
			self.iVec13[1] = self.iVec13[0];
			self.iRec180[1] = self.iRec180[0];
			self.fRec178[1] = self.fRec178[0];
			for j6 in (1..=3).rev() {
				self.fRec181[j6 as usize] = self.fRec181[(i32::wrapping_sub(j6, 1)) as usize];
			}
			self.fVec14[1] = self.fVec14[0];
			self.fVec15[1] = self.fVec15[0];
			self.iRec183[1] = self.iRec183[0];
			self.fRec184[2] = self.fRec184[1];
			self.fRec184[1] = self.fRec184[0];
			self.fVec16[2] = self.fVec16[1];
			self.fVec16[1] = self.fVec16[0];
			self.fRec167[1] = self.fRec167[0];
			self.fRec163[1] = self.fRec163[0];
			self.fRec161[1] = self.fRec161[0];
			for j7 in (1..=3).rev() {
				self.fRec157[j7 as usize] = self.fRec157[(i32::wrapping_sub(j7, 1)) as usize];
			}
			self.fRec152[1] = self.fRec152[0];
			self.fRec146[1] = self.fRec146[0];
			self.fRec145[1] = self.fRec145[0];
			self.fRec143[1] = self.fRec143[0];
			self.fRec142[2] = self.fRec142[1];
			self.fRec142[1] = self.fRec142[0];
			self.fRec141[2] = self.fRec141[1];
			self.fRec141[1] = self.fRec141[0];
			self.fRec185[1] = self.fRec185[0];
			self.fRec220[1] = self.fRec220[0];
			self.fRec216[1] = self.fRec216[0];
			self.fRec221[1] = self.fRec221[0];
			self.fRec224[1] = self.fRec224[0];
			self.iVec17[1] = self.iVec17[0];
			self.iRec225[1] = self.iRec225[0];
			self.fRec223[1] = self.fRec223[0];
			for j8 in (1..=3).rev() {
				self.fRec226[j8 as usize] = self.fRec226[(i32::wrapping_sub(j8, 1)) as usize];
			}
			self.fVec18[1] = self.fVec18[0];
			self.fVec19[1] = self.fVec19[0];
			self.iRec228[1] = self.iRec228[0];
			self.fRec229[2] = self.fRec229[1];
			self.fRec229[1] = self.fRec229[0];
			self.fVec20[2] = self.fVec20[1];
			self.fVec20[1] = self.fVec20[0];
			self.fRec212[1] = self.fRec212[0];
			self.fRec208[1] = self.fRec208[0];
			self.fRec206[1] = self.fRec206[0];
			for j9 in (1..=3).rev() {
				self.fRec202[j9 as usize] = self.fRec202[(i32::wrapping_sub(j9, 1)) as usize];
			}
			self.fRec197[1] = self.fRec197[0];
			self.fRec191[1] = self.fRec191[0];
			self.fRec190[1] = self.fRec190[0];
			self.fRec188[1] = self.fRec188[0];
			self.fRec187[2] = self.fRec187[1];
			self.fRec187[1] = self.fRec187[0];
			self.fRec186[2] = self.fRec186[1];
			self.fRec186[1] = self.fRec186[0];
			self.fRec230[1] = self.fRec230[0];
			self.fRec231[1] = self.fRec231[0];
			self.fRec236[1] = self.fRec236[0];
			self.fRec234[1] = self.fRec234[0];
			self.fRec237[1] = self.fRec237[0];
			self.fRec233[2] = self.fRec233[1];
			self.fRec233[1] = self.fRec233[0];
			self.fRec232[2] = self.fRec232[1];
			self.fRec232[1] = self.fRec232[0];
			self.fRec238[1] = self.fRec238[0];
			self.fRec243[1] = self.fRec243[0];
			self.fRec241[1] = self.fRec241[0];
			self.fRec244[1] = self.fRec244[0];
			self.fRec240[2] = self.fRec240[1];
			self.fRec240[1] = self.fRec240[0];
			self.fRec239[2] = self.fRec239[1];
			self.fRec239[1] = self.fRec239[0];
			self.fRec245[1] = self.fRec245[0];
			self.fRec250[1] = self.fRec250[0];
			self.fRec248[1] = self.fRec248[0];
			self.fRec251[1] = self.fRec251[0];
			self.fRec247[2] = self.fRec247[1];
			self.fRec247[1] = self.fRec247[0];
			self.fRec246[2] = self.fRec246[1];
			self.fRec246[1] = self.fRec246[0];
			self.fRec252[1] = self.fRec252[0];
			self.fRec257[1] = self.fRec257[0];
			self.fRec255[1] = self.fRec255[0];
			self.fRec258[1] = self.fRec258[0];
			self.fRec254[2] = self.fRec254[1];
			self.fRec254[1] = self.fRec254[0];
			self.fRec253[2] = self.fRec253[1];
			self.fRec253[1] = self.fRec253[0];
			self.fRec259[1] = self.fRec259[0];
			self.fRec260[1] = self.fRec260[0];
			self.fRec261[1] = self.fRec261[0];
			self.fRec263[1] = self.fRec263[0];
			self.fRec264[1] = self.fRec264[0];
			self.fVec21[1] = self.fVec21[0];
			self.fRec262[1] = self.fRec262[0];
			self.fRec266[1] = self.fRec266[0];
			self.fRec267[1] = self.fRec267[0];
			self.fVec23[1] = self.fVec23[0];
			self.fRec265[1] = self.fRec265[0];
			self.fRec269[1] = self.fRec269[0];
			self.fRec270[1] = self.fRec270[0];
			self.fVec25[1] = self.fVec25[0];
			self.fRec268[1] = self.fRec268[0];
			self.fRec271[1] = self.fRec271[0];
			self.fRec273[1] = self.fRec273[0];
			self.fRec274[1] = self.fRec274[0];
			self.fVec27[1] = self.fVec27[0];
			self.fRec272[1] = self.fRec272[0];
			self.fRec276[1] = self.fRec276[0];
			self.fRec277[1] = self.fRec277[0];
			self.fVec29[1] = self.fVec29[0];
			self.fRec275[1] = self.fRec275[0];
			self.fRec279[1] = self.fRec279[0];
			self.fRec280[1] = self.fRec280[0];
			self.fVec31[1] = self.fVec31[0];
			self.fRec278[1] = self.fRec278[0];
			self.fRec281[1] = self.fRec281[0];
			self.fRec283[1] = self.fRec283[0];
			self.fRec284[1] = self.fRec284[0];
			self.fVec33[1] = self.fVec33[0];
			self.fRec282[1] = self.fRec282[0];
			self.fRec286[1] = self.fRec286[0];
			self.fRec287[1] = self.fRec287[0];
			self.fVec35[1] = self.fVec35[0];
			self.fRec285[1] = self.fRec285[0];
			self.fRec289[1] = self.fRec289[0];
			self.fRec290[1] = self.fRec290[0];
			self.fVec37[1] = self.fVec37[0];
			self.fRec288[1] = self.fRec288[0];
			self.fRec291[1] = self.fRec291[0];
			self.fRec293[1] = self.fRec293[0];
			self.fRec294[1] = self.fRec294[0];
			self.fVec39[1] = self.fVec39[0];
			self.fRec292[1] = self.fRec292[0];
			self.fRec296[1] = self.fRec296[0];
			self.fRec297[1] = self.fRec297[0];
			self.fVec41[1] = self.fVec41[0];
			self.fRec295[1] = self.fRec295[0];
			self.fRec299[1] = self.fRec299[0];
			self.fRec300[1] = self.fRec300[0];
			self.fVec43[1] = self.fVec43[0];
			self.fRec298[1] = self.fRec298[0];
			self.fRec301[1] = self.fRec301[0];
			self.fRec302[1] = self.fRec302[0];
			self.fRec303[1] = self.fRec303[0];
			self.fRec305[1] = self.fRec305[0];
			self.fRec316[1] = self.fRec316[0];
			self.fRec318[1] = self.fRec318[0];
			self.fRec322[1] = self.fRec322[0];
			self.fVec46[1] = self.fVec46[0];
			self.fRec321[1] = self.fRec321[0];
			self.fRec319[1] = self.fRec319[0];
			self.fRec324[1] = self.fRec324[0];
			self.fVec48[1] = self.fVec48[0];
			self.fRec323[1] = self.fRec323[0];
			self.fRec320[1] = self.fRec320[0];
			self.fRec328[1] = self.fRec328[0];
			self.fVec50[1] = self.fVec50[0];
			self.fRec327[1] = self.fRec327[0];
			self.fRec325[1] = self.fRec325[0];
			self.fRec330[1] = self.fRec330[0];
			self.fVec52[1] = self.fVec52[0];
			self.fRec329[1] = self.fRec329[0];
			self.fRec326[1] = self.fRec326[0];
			self.fRec334[1] = self.fRec334[0];
			self.fVec54[1] = self.fVec54[0];
			self.fRec333[1] = self.fRec333[0];
			self.fRec331[1] = self.fRec331[0];
			self.fRec336[1] = self.fRec336[0];
			self.fVec56[1] = self.fVec56[0];
			self.fRec335[1] = self.fRec335[0];
			self.fRec332[1] = self.fRec332[0];
			self.fRec340[1] = self.fRec340[0];
			self.fVec58[1] = self.fVec58[0];
			self.fRec339[1] = self.fRec339[0];
			self.fRec337[1] = self.fRec337[0];
			self.fRec342[1] = self.fRec342[0];
			self.fVec60[1] = self.fVec60[0];
			self.fRec341[1] = self.fRec341[0];
			self.fRec338[1] = self.fRec338[0];
			self.fRec346[1] = self.fRec346[0];
			self.fVec62[1] = self.fVec62[0];
			self.fRec345[1] = self.fRec345[0];
			self.fRec343[1] = self.fRec343[0];
			self.fRec348[1] = self.fRec348[0];
			self.fVec64[1] = self.fVec64[0];
			self.fRec347[1] = self.fRec347[0];
			self.fRec344[1] = self.fRec344[0];
			self.fRec349[1] = self.fRec349[0];
			self.fRec350[1] = self.fRec350[0];
			self.fVec67[1] = self.fVec67[0];
			self.fRec317[1] = self.fRec317[0];
			self.fRec354[1] = self.fRec354[0];
			self.fRec356[1] = self.fRec356[0];
			self.fVec70[1] = self.fVec70[0];
			self.fRec355[1] = self.fRec355[0];
			self.fVec72[1] = self.fVec72[0];
			self.fRec353[1] = self.fRec353[0];
			self.fRec351[1] = self.fRec351[0];
			self.fRec358[1] = self.fRec358[0];
			self.fVec74[1] = self.fVec74[0];
			self.fRec357[1] = self.fRec357[0];
			self.fRec352[1] = self.fRec352[0];
			self.fRec362[1] = self.fRec362[0];
			self.fVec76[1] = self.fVec76[0];
			self.fRec361[1] = self.fRec361[0];
			self.fRec359[1] = self.fRec359[0];
			self.fRec364[1] = self.fRec364[0];
			self.fVec78[1] = self.fVec78[0];
			self.fRec363[1] = self.fRec363[0];
			self.fRec360[1] = self.fRec360[0];
			self.fRec368[1] = self.fRec368[0];
			self.fVec80[1] = self.fVec80[0];
			self.fRec367[1] = self.fRec367[0];
			self.fRec365[1] = self.fRec365[0];
			self.fRec370[1] = self.fRec370[0];
			self.fVec82[1] = self.fVec82[0];
			self.fRec369[1] = self.fRec369[0];
			self.fRec366[1] = self.fRec366[0];
			self.fRec374[1] = self.fRec374[0];
			self.fVec84[1] = self.fVec84[0];
			self.fRec373[1] = self.fRec373[0];
			self.fRec371[1] = self.fRec371[0];
			self.fRec376[1] = self.fRec376[0];
			self.fVec86[1] = self.fVec86[0];
			self.fRec375[1] = self.fRec375[0];
			self.fRec372[1] = self.fRec372[0];
			self.fRec380[1] = self.fRec380[0];
			self.fVec88[1] = self.fVec88[0];
			self.fRec379[1] = self.fRec379[0];
			self.fRec377[1] = self.fRec377[0];
			self.fRec382[1] = self.fRec382[0];
			self.fVec90[1] = self.fVec90[0];
			self.fRec381[1] = self.fRec381[0];
			self.fRec378[1] = self.fRec378[0];
			self.fVec93[1] = self.fVec93[0];
			self.fRec315[1] = self.fRec315[0];
			self.fRec314[1] = self.fRec314[0];
			self.fRec313[2] = self.fRec313[1];
			self.fRec313[1] = self.fRec313[0];
			self.fRec312[2] = self.fRec312[1];
			self.fRec312[1] = self.fRec312[0];
			self.fVec94[1] = self.fVec94[0];
			self.fRec311[1] = self.fRec311[0];
			self.fRec310[2] = self.fRec310[1];
			self.fRec310[1] = self.fRec310[0];
			self.fRec309[2] = self.fRec309[1];
			self.fRec309[1] = self.fRec309[0];
			self.fRec385[1] = self.fRec385[0];
			self.fRec384[2] = self.fRec384[1];
			self.fRec384[1] = self.fRec384[0];
			self.fRec383[2] = self.fRec383[1];
			self.fRec383[1] = self.fRec383[0];
			self.fRec389[1] = self.fRec389[0];
			self.fRec388[2] = self.fRec388[1];
			self.fRec388[1] = self.fRec388[0];
			self.fRec387[2] = self.fRec387[1];
			self.fRec387[1] = self.fRec387[0];
			self.fRec386[2] = self.fRec386[1];
			self.fRec386[1] = self.fRec386[0];
			self.fRec308[1] = self.fRec308[0];
			self.fRec401[1] = self.fRec401[0];
			self.fVec98[1] = self.fVec98[0];
			self.fRec400[1] = self.fRec400[0];
			self.fRec399[1] = self.fRec399[0];
			self.fRec398[2] = self.fRec398[1];
			self.fRec398[1] = self.fRec398[0];
			self.fRec397[2] = self.fRec397[1];
			self.fRec397[1] = self.fRec397[0];
			self.fVec99[1] = self.fVec99[0];
			self.fRec396[1] = self.fRec396[0];
			self.fRec395[2] = self.fRec395[1];
			self.fRec395[1] = self.fRec395[0];
			self.fRec394[2] = self.fRec394[1];
			self.fRec394[1] = self.fRec394[0];
			self.fRec404[1] = self.fRec404[0];
			self.fRec403[2] = self.fRec403[1];
			self.fRec403[1] = self.fRec403[0];
			self.fRec402[2] = self.fRec402[1];
			self.fRec402[1] = self.fRec402[0];
			self.fRec408[1] = self.fRec408[0];
			self.fRec407[2] = self.fRec407[1];
			self.fRec407[1] = self.fRec407[0];
			self.fRec406[2] = self.fRec406[1];
			self.fRec406[1] = self.fRec406[0];
			self.fRec405[2] = self.fRec405[1];
			self.fRec405[1] = self.fRec405[0];
			self.fRec393[1] = self.fRec393[0];
			self.fVec102[1] = self.fVec102[0];
			self.fRec392[1] = self.fRec392[0];
			self.fRec390[1] = self.fRec390[0];
			self.fRec410[1] = self.fRec410[0];
			self.fVec104[1] = self.fVec104[0];
			self.fRec409[1] = self.fRec409[0];
			self.fRec391[1] = self.fRec391[0];
			self.fVec106[1] = self.fVec106[0];
			self.fRec413[1] = self.fRec413[0];
			self.fRec411[1] = self.fRec411[0];
			self.fVec108[1] = self.fVec108[0];
			self.fRec414[1] = self.fRec414[0];
			self.fRec412[1] = self.fRec412[0];
			self.fVec110[1] = self.fVec110[0];
			self.fRec417[1] = self.fRec417[0];
			self.fRec415[1] = self.fRec415[0];
			self.fRec419[1] = self.fRec419[0];
			self.fVec112[1] = self.fVec112[0];
			self.fRec418[1] = self.fRec418[0];
			self.fRec416[1] = self.fRec416[0];
			self.fRec423[1] = self.fRec423[0];
			self.fVec114[1] = self.fVec114[0];
			self.fRec422[1] = self.fRec422[0];
			self.fRec420[1] = self.fRec420[0];
			self.fVec116[1] = self.fVec116[0];
			self.fRec424[1] = self.fRec424[0];
			self.fRec421[1] = self.fRec421[0];
			self.fRec306[1] = self.fRec306[0];
			self.fRec307[1] = self.fRec307[0];
			self.fRec425[1] = self.fRec425[0];
		}
	}

}


}
pub use dsp::mydsp as Instrument;
