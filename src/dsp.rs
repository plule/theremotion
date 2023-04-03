mod dsp {
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
			self.iRec2[(l2) as usize] = 0;
		}
	}
	
	fn fillmydspSIG0(&mut self, count: i32, table: &mut[F32]) {
		for i1 in 0..count {
			self.iRec2[0] = i32::wrapping_add(self.iRec2[1], 1);
			table[(i1) as usize] = 4.4e+02 * F32::powf(2.0, 0.083333336 * (0.062042013 * ((i32::wrapping_add(self.iRec2[0], -1)) as F32) + -69.0));
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
			table[(i2) as usize] = unsafe { imydspSIG1Wave0[(self.imydspSIG1Wave0_idx) as usize] };
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
	fRec40: [F32;2],
	fRec39: [F32;2],
	fConst5: F32,
	fRec35: [F32;2],
	fRec41: [F32;2],
	fConst6: F32,
	fConst7: F32,
	fRec46: [F32;2],
	iVec1: [i32;2],
	iConst8: i32,
	iRec47: [i32;2],
	fConst9: F32,
	fRec44: [F32;2],
	fRec43: [F32;2],
	fRec48: [F32;4],
	fRec49: [F32;2048],
	fVec2: [F32;2],
	fConst10: F32,
	fConst11: F32,
	fButton0: F32,
	fVec3: [F32;2],
	iRec50: [i32;2],
	iRec52: [i32;2],
	fRec51: [F32;3],
	fVec4: [F32;3],
	fRec42: [F32;2048],
	fRec31: [F32;2],
	fRec27: [F32;2],
	fRec23: [F32;2048],
	fRec25: [F32;2],
	fHslider4: F32,
	fRec21: [F32;4],
	fRec16: [F32;2],
	fRec12: [F32;2048],
	fRec10: [F32;2],
	fConst12: F32,
	fRec9: [F32;2],
	fRec8: [F32;2],
	fRec6: [F32;2],
	fRec5: [F32;3],
	fRec4: [F32;3],
	fHslider5: F32,
	fRec53: [F32;2],
	fRec89: [F32;2],
	fRec85: [F32;2],
	fRec90: [F32;2],
	fRec95: [F32;2],
	iVec5: [i32;2],
	iRec96: [i32;2],
	fRec93: [F32;2],
	fRec92: [F32;2],
	fRec97: [F32;4],
	fRec98: [F32;2048],
	fVec6: [F32;2],
	fButton1: F32,
	fVec7: [F32;2],
	iRec99: [i32;2],
	fRec100: [F32;3],
	fVec8: [F32;3],
	fRec91: [F32;2048],
	fRec81: [F32;2],
	fRec77: [F32;2],
	fRec73: [F32;2048],
	fRec75: [F32;2],
	fRec71: [F32;4],
	fRec66: [F32;2],
	fRec62: [F32;2048],
	fRec60: [F32;2],
	fRec59: [F32;2],
	fRec58: [F32;2],
	fRec56: [F32;2],
	fRec55: [F32;3],
	fRec54: [F32;3],
	fHslider6: F32,
	fRec101: [F32;2],
	fRec137: [F32;2],
	fRec133: [F32;2],
	fRec138: [F32;2],
	fRec143: [F32;2],
	iVec9: [i32;2],
	iRec144: [i32;2],
	fRec141: [F32;2],
	fRec140: [F32;2],
	fRec145: [F32;4],
	fRec146: [F32;2048],
	fVec10: [F32;2],
	fButton2: F32,
	fVec11: [F32;2],
	iRec147: [i32;2],
	fRec148: [F32;3],
	fVec12: [F32;3],
	fRec139: [F32;2048],
	fRec129: [F32;2],
	fRec125: [F32;2],
	fRec121: [F32;2048],
	fRec123: [F32;2],
	fRec119: [F32;4],
	fRec114: [F32;2],
	fRec110: [F32;2048],
	fRec108: [F32;2],
	fRec107: [F32;2],
	fRec106: [F32;2],
	fRec104: [F32;2],
	fRec103: [F32;3],
	fRec102: [F32;3],
	fHslider7: F32,
	fRec149: [F32;2],
	fRec185: [F32;2],
	fRec181: [F32;2],
	fRec186: [F32;2],
	fRec191: [F32;2],
	iVec13: [i32;2],
	iRec192: [i32;2],
	fRec189: [F32;2],
	fRec188: [F32;2],
	fRec193: [F32;4],
	fRec194: [F32;2048],
	fVec14: [F32;2],
	fButton3: F32,
	fVec15: [F32;2],
	iRec195: [i32;2],
	fRec196: [F32;3],
	fVec16: [F32;3],
	fRec187: [F32;2048],
	fRec177: [F32;2],
	fRec173: [F32;2],
	fRec169: [F32;2048],
	fRec171: [F32;2],
	fRec167: [F32;4],
	fRec162: [F32;2],
	fRec158: [F32;2048],
	fRec156: [F32;2],
	fRec155: [F32;2],
	fRec154: [F32;2],
	fRec152: [F32;2],
	fRec151: [F32;3],
	fRec150: [F32;3],
	fHslider8: F32,
	fRec197: [F32;2],
	fRec233: [F32;2],
	fRec229: [F32;2],
	fRec234: [F32;2],
	fRec239: [F32;2],
	iVec17: [i32;2],
	iRec240: [i32;2],
	fRec237: [F32;2],
	fRec236: [F32;2],
	fRec241: [F32;4],
	fRec242: [F32;2048],
	fVec18: [F32;2],
	fButton4: F32,
	fVec19: [F32;2],
	iRec243: [i32;2],
	fRec244: [F32;3],
	fVec20: [F32;3],
	fRec235: [F32;2048],
	fRec225: [F32;2],
	fRec221: [F32;2],
	fRec217: [F32;2048],
	fRec219: [F32;2],
	fRec215: [F32;4],
	fRec210: [F32;2],
	fRec206: [F32;2048],
	fRec204: [F32;2],
	fRec203: [F32;2],
	fRec202: [F32;2],
	fRec200: [F32;2],
	fRec199: [F32;3],
	fRec198: [F32;3],
	fHslider9: F32,
	fRec245: [F32;2],
	fHslider10: F32,
	fRec246: [F32;2],
	fRec251: [F32;2],
	fConst13: F32,
	fRec249: [F32;2],
	fHslider11: F32,
	fRec252: [F32;2],
	fRec248: [F32;3],
	fRec247: [F32;3],
	fHslider12: F32,
	fRec253: [F32;2],
	fRec258: [F32;2],
	fRec256: [F32;2],
	fHslider13: F32,
	fRec259: [F32;2],
	fRec255: [F32;3],
	fRec254: [F32;3],
	fHslider14: F32,
	fRec260: [F32;2],
	fRec265: [F32;2],
	fRec263: [F32;2],
	fHslider15: F32,
	fRec266: [F32;2],
	fRec262: [F32;3],
	fRec261: [F32;3],
	fHslider16: F32,
	fRec267: [F32;2],
	fRec272: [F32;2],
	fRec270: [F32;2],
	fHslider17: F32,
	fRec273: [F32;2],
	fRec269: [F32;3],
	fRec268: [F32;3],
	fHslider18: F32,
	fRec274: [F32;2],
	fHslider19: F32,
	fRec275: [F32;2],
	fHslider20: F32,
	fRec276: [F32;2],
	fHslider21: F32,
	fHslider22: F32,
	fRec278: [F32;2],
	fRec279: [F32;2],
	fVec21: [F32;2],
	fVec22: [F32;4096],
	fConst14: F32,
	fConst15: F32,
	fRec277: [F32;2],
	fRec281: [F32;2],
	fRec282: [F32;2],
	fVec23: [F32;2],
	fVec24: [F32;4096],
	fRec280: [F32;2],
	fRec284: [F32;2],
	fRec285: [F32;2],
	fVec25: [F32;2],
	fVec26: [F32;4096],
	fRec283: [F32;2],
	fHslider23: F32,
	fRec286: [F32;2],
	fHslider24: F32,
	fRec288: [F32;2],
	fRec289: [F32;2],
	fVec27: [F32;2],
	fVec28: [F32;4096],
	fRec287: [F32;2],
	fRec291: [F32;2],
	fRec292: [F32;2],
	fVec29: [F32;2],
	fVec30: [F32;4096],
	fRec290: [F32;2],
	fRec294: [F32;2],
	fRec295: [F32;2],
	fVec31: [F32;2],
	fVec32: [F32;4096],
	fRec293: [F32;2],
	fHslider25: F32,
	fRec296: [F32;2],
	fHslider26: F32,
	fRec298: [F32;2],
	fRec299: [F32;2],
	fVec33: [F32;2],
	fVec34: [F32;4096],
	fRec297: [F32;2],
	fRec301: [F32;2],
	fRec302: [F32;2],
	fVec35: [F32;2],
	fVec36: [F32;4096],
	fRec300: [F32;2],
	fRec304: [F32;2],
	fRec305: [F32;2],
	fVec37: [F32;2],
	fVec38: [F32;4096],
	fRec303: [F32;2],
	fHslider27: F32,
	fRec306: [F32;2],
	fHslider28: F32,
	fRec308: [F32;2],
	fRec309: [F32;2],
	fVec39: [F32;2],
	fVec40: [F32;4096],
	fRec307: [F32;2],
	fRec311: [F32;2],
	fRec312: [F32;2],
	fVec41: [F32;2],
	fVec42: [F32;4096],
	fRec310: [F32;2],
	fRec314: [F32;2],
	fRec315: [F32;2],
	fVec43: [F32;2],
	fVec44: [F32;4096],
	fRec313: [F32;2],
	fHslider29: F32,
	fRec316: [F32;2],
	fConst16: F32,
	fHslider30: F32,
	fRec317: [F32;2],
	fHslider31: F32,
	fRec318: [F32;2],
	fConst17: F32,
	fHslider32: F32,
	fRec320: [F32;2],
	fHslider33: F32,
	fRec319: [F32;2097152],
	fHslider34: F32,
	fConst20: F32,
	fConst21: F32,
	fConst23: F32,
	fConst24: F32,
	fConst25: F32,
	fConst26: F32,
	fConst29: F32,
	fConst30: F32,
	fConst31: F32,
	fConst32: F32,
	fConst33: F32,
	fConst34: F32,
	fConst35: F32,
	fHslider35: F32,
	fRec331: [F32;2],
	fRec333: [F32;2],
	fRec337: [F32;2],
	fVec45: [F32;16384],
	fVec46: [F32;2],
	fRec336: [F32;2],
	fRec334: [F32;2],
	fRec339: [F32;2],
	fVec47: [F32;16384],
	fVec48: [F32;2],
	fRec338: [F32;2],
	fRec335: [F32;2],
	fRec343: [F32;2],
	fVec49: [F32;16384],
	fVec50: [F32;2],
	fRec342: [F32;2],
	fRec340: [F32;2],
	fRec345: [F32;2],
	fVec51: [F32;16384],
	fVec52: [F32;2],
	fRec344: [F32;2],
	fRec341: [F32;2],
	fRec349: [F32;2],
	fVec53: [F32;16384],
	fVec54: [F32;2],
	fRec348: [F32;2],
	fRec346: [F32;2],
	fRec351: [F32;2],
	fVec55: [F32;16384],
	fVec56: [F32;2],
	fRec350: [F32;2],
	fRec347: [F32;2],
	fRec355: [F32;2],
	fVec57: [F32;16384],
	fVec58: [F32;2],
	fRec354: [F32;2],
	fRec352: [F32;2],
	fRec357: [F32;2],
	fVec59: [F32;16384],
	fVec60: [F32;2],
	fRec356: [F32;2],
	fRec353: [F32;2],
	fRec361: [F32;2],
	fVec61: [F32;16384],
	fVec62: [F32;2],
	fRec360: [F32;2],
	fRec358: [F32;2],
	fRec363: [F32;2],
	fVec63: [F32;16384],
	fVec64: [F32;2],
	fRec362: [F32;2],
	fRec359: [F32;2],
	fVec65: [F32;1024],
	fHslider36: F32,
	fConst36: F32,
	fRec364: [F32;2],
	fRec365: [F32;2],
	fHslider37: F32,
	fVec66: [F32;16384],
	fVec67: [F32;2],
	fRec332: [F32;2],
	fRec369: [F32;2],
	fRec371: [F32;2],
	fVec68: [F32;1024],
	fVec69: [F32;16384],
	fVec70: [F32;2],
	fRec370: [F32;2],
	fVec71: [F32;16384],
	fVec72: [F32;2],
	fRec368: [F32;2],
	fRec366: [F32;2],
	fRec373: [F32;2],
	fVec73: [F32;16384],
	fVec74: [F32;2],
	fRec372: [F32;2],
	fRec367: [F32;2],
	fRec377: [F32;2],
	fVec75: [F32;16384],
	fVec76: [F32;2],
	fRec376: [F32;2],
	fRec374: [F32;2],
	fRec379: [F32;2],
	fVec77: [F32;16384],
	fVec78: [F32;2],
	fRec378: [F32;2],
	fRec375: [F32;2],
	fRec383: [F32;2],
	fVec79: [F32;16384],
	fVec80: [F32;2],
	fRec382: [F32;2],
	fRec380: [F32;2],
	fRec385: [F32;2],
	fVec81: [F32;16384],
	fVec82: [F32;2],
	fRec384: [F32;2],
	fRec381: [F32;2],
	fRec389: [F32;2],
	fVec83: [F32;16384],
	fVec84: [F32;2],
	fRec388: [F32;2],
	fRec386: [F32;2],
	fRec391: [F32;2],
	fVec85: [F32;16384],
	fVec86: [F32;2],
	fRec390: [F32;2],
	fRec387: [F32;2],
	fRec395: [F32;2],
	fVec87: [F32;16384],
	fVec88: [F32;2],
	fRec394: [F32;2],
	fRec392: [F32;2],
	fRec397: [F32;2],
	fVec89: [F32;16384],
	fVec90: [F32;2],
	fRec396: [F32;2],
	fRec393: [F32;2],
	fVec91: [F32;16384],
	fVec92: [F32;16384],
	fVec93: [F32;2],
	fRec330: [F32;2],
	fConst37: F32,
	fConst39: F32,
	fRec329: [F32;2],
	fRec328: [F32;3],
	fRec327: [F32;3],
	fVec94: [F32;2],
	fConst40: F32,
	fConst42: F32,
	fRec326: [F32;2],
	fRec325: [F32;3],
	fRec324: [F32;3],
	fConst43: F32,
	fConst44: F32,
	fConst45: F32,
	fRec400: [F32;2],
	fRec399: [F32;3],
	fConst46: F32,
	fRec398: [F32;3],
	fConst47: F32,
	fConst48: F32,
	fConst49: F32,
	fRec404: [F32;2],
	fRec403: [F32;3],
	fConst50: F32,
	fRec402: [F32;3],
	fRec401: [F32;3],
	fHslider38: F32,
	fVec95: [F32;1024],
	fHslider39: F32,
	fRec323: [F32;2],
	fHslider40: F32,
	fRec416: [F32;2],
	fVec96: [F32;16384],
	fVec97: [F32;16384],
	fVec98: [F32;2],
	fRec415: [F32;2],
	fRec414: [F32;2],
	fRec413: [F32;3],
	fRec412: [F32;3],
	fVec99: [F32;2],
	fRec411: [F32;2],
	fRec410: [F32;3],
	fRec409: [F32;3],
	fRec419: [F32;2],
	fRec418: [F32;3],
	fRec417: [F32;3],
	fRec423: [F32;2],
	fRec422: [F32;3],
	fRec421: [F32;3],
	fRec420: [F32;3],
	fVec100: [F32;1024],
	fRec408: [F32;2],
	fVec101: [F32;16384],
	fVec102: [F32;2],
	fRec407: [F32;2],
	fRec405: [F32;2],
	fRec425: [F32;2],
	fVec103: [F32;16384],
	fVec104: [F32;2],
	fRec424: [F32;2],
	fRec406: [F32;2],
	fVec105: [F32;16384],
	fVec106: [F32;2],
	fRec428: [F32;2],
	fRec426: [F32;2],
	fVec107: [F32;16384],
	fVec108: [F32;2],
	fRec429: [F32;2],
	fRec427: [F32;2],
	fVec109: [F32;16384],
	fVec110: [F32;2],
	fRec432: [F32;2],
	fRec430: [F32;2],
	fRec434: [F32;2],
	fVec111: [F32;16384],
	fVec112: [F32;2],
	fRec433: [F32;2],
	fRec431: [F32;2],
	fRec438: [F32;2],
	fVec113: [F32;16384],
	fVec114: [F32;2],
	fRec437: [F32;2],
	fRec435: [F32;2],
	fVec115: [F32;16384],
	fVec116: [F32;2],
	fRec439: [F32;2],
	fRec436: [F32;2],
	fRec321: [F32;2],
	fRec322: [F32;2],
	fHslider41: F32,
	fRec440: [F32;2],
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
			fRec40: [0.0;2],
			fRec39: [0.0;2],
			fConst5: 0.0,
			fRec35: [0.0;2],
			fRec41: [0.0;2],
			fConst6: 0.0,
			fConst7: 0.0,
			fRec46: [0.0;2],
			iVec1: [0;2],
			iConst8: 0,
			iRec47: [0;2],
			fConst9: 0.0,
			fRec44: [0.0;2],
			fRec43: [0.0;2],
			fRec48: [0.0;4],
			fRec49: [0.0;2048],
			fVec2: [0.0;2],
			fConst10: 0.0,
			fConst11: 0.0,
			fButton0: 0.0,
			fVec3: [0.0;2],
			iRec50: [0;2],
			iRec52: [0;2],
			fRec51: [0.0;3],
			fVec4: [0.0;3],
			fRec42: [0.0;2048],
			fRec31: [0.0;2],
			fRec27: [0.0;2],
			fRec23: [0.0;2048],
			fRec25: [0.0;2],
			fHslider4: 0.0,
			fRec21: [0.0;4],
			fRec16: [0.0;2],
			fRec12: [0.0;2048],
			fRec10: [0.0;2],
			fConst12: 0.0,
			fRec9: [0.0;2],
			fRec8: [0.0;2],
			fRec6: [0.0;2],
			fRec5: [0.0;3],
			fRec4: [0.0;3],
			fHslider5: 0.0,
			fRec53: [0.0;2],
			fRec89: [0.0;2],
			fRec85: [0.0;2],
			fRec90: [0.0;2],
			fRec95: [0.0;2],
			iVec5: [0;2],
			iRec96: [0;2],
			fRec93: [0.0;2],
			fRec92: [0.0;2],
			fRec97: [0.0;4],
			fRec98: [0.0;2048],
			fVec6: [0.0;2],
			fButton1: 0.0,
			fVec7: [0.0;2],
			iRec99: [0;2],
			fRec100: [0.0;3],
			fVec8: [0.0;3],
			fRec91: [0.0;2048],
			fRec81: [0.0;2],
			fRec77: [0.0;2],
			fRec73: [0.0;2048],
			fRec75: [0.0;2],
			fRec71: [0.0;4],
			fRec66: [0.0;2],
			fRec62: [0.0;2048],
			fRec60: [0.0;2],
			fRec59: [0.0;2],
			fRec58: [0.0;2],
			fRec56: [0.0;2],
			fRec55: [0.0;3],
			fRec54: [0.0;3],
			fHslider6: 0.0,
			fRec101: [0.0;2],
			fRec137: [0.0;2],
			fRec133: [0.0;2],
			fRec138: [0.0;2],
			fRec143: [0.0;2],
			iVec9: [0;2],
			iRec144: [0;2],
			fRec141: [0.0;2],
			fRec140: [0.0;2],
			fRec145: [0.0;4],
			fRec146: [0.0;2048],
			fVec10: [0.0;2],
			fButton2: 0.0,
			fVec11: [0.0;2],
			iRec147: [0;2],
			fRec148: [0.0;3],
			fVec12: [0.0;3],
			fRec139: [0.0;2048],
			fRec129: [0.0;2],
			fRec125: [0.0;2],
			fRec121: [0.0;2048],
			fRec123: [0.0;2],
			fRec119: [0.0;4],
			fRec114: [0.0;2],
			fRec110: [0.0;2048],
			fRec108: [0.0;2],
			fRec107: [0.0;2],
			fRec106: [0.0;2],
			fRec104: [0.0;2],
			fRec103: [0.0;3],
			fRec102: [0.0;3],
			fHslider7: 0.0,
			fRec149: [0.0;2],
			fRec185: [0.0;2],
			fRec181: [0.0;2],
			fRec186: [0.0;2],
			fRec191: [0.0;2],
			iVec13: [0;2],
			iRec192: [0;2],
			fRec189: [0.0;2],
			fRec188: [0.0;2],
			fRec193: [0.0;4],
			fRec194: [0.0;2048],
			fVec14: [0.0;2],
			fButton3: 0.0,
			fVec15: [0.0;2],
			iRec195: [0;2],
			fRec196: [0.0;3],
			fVec16: [0.0;3],
			fRec187: [0.0;2048],
			fRec177: [0.0;2],
			fRec173: [0.0;2],
			fRec169: [0.0;2048],
			fRec171: [0.0;2],
			fRec167: [0.0;4],
			fRec162: [0.0;2],
			fRec158: [0.0;2048],
			fRec156: [0.0;2],
			fRec155: [0.0;2],
			fRec154: [0.0;2],
			fRec152: [0.0;2],
			fRec151: [0.0;3],
			fRec150: [0.0;3],
			fHslider8: 0.0,
			fRec197: [0.0;2],
			fRec233: [0.0;2],
			fRec229: [0.0;2],
			fRec234: [0.0;2],
			fRec239: [0.0;2],
			iVec17: [0;2],
			iRec240: [0;2],
			fRec237: [0.0;2],
			fRec236: [0.0;2],
			fRec241: [0.0;4],
			fRec242: [0.0;2048],
			fVec18: [0.0;2],
			fButton4: 0.0,
			fVec19: [0.0;2],
			iRec243: [0;2],
			fRec244: [0.0;3],
			fVec20: [0.0;3],
			fRec235: [0.0;2048],
			fRec225: [0.0;2],
			fRec221: [0.0;2],
			fRec217: [0.0;2048],
			fRec219: [0.0;2],
			fRec215: [0.0;4],
			fRec210: [0.0;2],
			fRec206: [0.0;2048],
			fRec204: [0.0;2],
			fRec203: [0.0;2],
			fRec202: [0.0;2],
			fRec200: [0.0;2],
			fRec199: [0.0;3],
			fRec198: [0.0;3],
			fHslider9: 0.0,
			fRec245: [0.0;2],
			fHslider10: 0.0,
			fRec246: [0.0;2],
			fRec251: [0.0;2],
			fConst13: 0.0,
			fRec249: [0.0;2],
			fHslider11: 0.0,
			fRec252: [0.0;2],
			fRec248: [0.0;3],
			fRec247: [0.0;3],
			fHslider12: 0.0,
			fRec253: [0.0;2],
			fRec258: [0.0;2],
			fRec256: [0.0;2],
			fHslider13: 0.0,
			fRec259: [0.0;2],
			fRec255: [0.0;3],
			fRec254: [0.0;3],
			fHslider14: 0.0,
			fRec260: [0.0;2],
			fRec265: [0.0;2],
			fRec263: [0.0;2],
			fHslider15: 0.0,
			fRec266: [0.0;2],
			fRec262: [0.0;3],
			fRec261: [0.0;3],
			fHslider16: 0.0,
			fRec267: [0.0;2],
			fRec272: [0.0;2],
			fRec270: [0.0;2],
			fHslider17: 0.0,
			fRec273: [0.0;2],
			fRec269: [0.0;3],
			fRec268: [0.0;3],
			fHslider18: 0.0,
			fRec274: [0.0;2],
			fHslider19: 0.0,
			fRec275: [0.0;2],
			fHslider20: 0.0,
			fRec276: [0.0;2],
			fHslider21: 0.0,
			fHslider22: 0.0,
			fRec278: [0.0;2],
			fRec279: [0.0;2],
			fVec21: [0.0;2],
			fVec22: [0.0;4096],
			fConst14: 0.0,
			fConst15: 0.0,
			fRec277: [0.0;2],
			fRec281: [0.0;2],
			fRec282: [0.0;2],
			fVec23: [0.0;2],
			fVec24: [0.0;4096],
			fRec280: [0.0;2],
			fRec284: [0.0;2],
			fRec285: [0.0;2],
			fVec25: [0.0;2],
			fVec26: [0.0;4096],
			fRec283: [0.0;2],
			fHslider23: 0.0,
			fRec286: [0.0;2],
			fHslider24: 0.0,
			fRec288: [0.0;2],
			fRec289: [0.0;2],
			fVec27: [0.0;2],
			fVec28: [0.0;4096],
			fRec287: [0.0;2],
			fRec291: [0.0;2],
			fRec292: [0.0;2],
			fVec29: [0.0;2],
			fVec30: [0.0;4096],
			fRec290: [0.0;2],
			fRec294: [0.0;2],
			fRec295: [0.0;2],
			fVec31: [0.0;2],
			fVec32: [0.0;4096],
			fRec293: [0.0;2],
			fHslider25: 0.0,
			fRec296: [0.0;2],
			fHslider26: 0.0,
			fRec298: [0.0;2],
			fRec299: [0.0;2],
			fVec33: [0.0;2],
			fVec34: [0.0;4096],
			fRec297: [0.0;2],
			fRec301: [0.0;2],
			fRec302: [0.0;2],
			fVec35: [0.0;2],
			fVec36: [0.0;4096],
			fRec300: [0.0;2],
			fRec304: [0.0;2],
			fRec305: [0.0;2],
			fVec37: [0.0;2],
			fVec38: [0.0;4096],
			fRec303: [0.0;2],
			fHslider27: 0.0,
			fRec306: [0.0;2],
			fHslider28: 0.0,
			fRec308: [0.0;2],
			fRec309: [0.0;2],
			fVec39: [0.0;2],
			fVec40: [0.0;4096],
			fRec307: [0.0;2],
			fRec311: [0.0;2],
			fRec312: [0.0;2],
			fVec41: [0.0;2],
			fVec42: [0.0;4096],
			fRec310: [0.0;2],
			fRec314: [0.0;2],
			fRec315: [0.0;2],
			fVec43: [0.0;2],
			fVec44: [0.0;4096],
			fRec313: [0.0;2],
			fHslider29: 0.0,
			fRec316: [0.0;2],
			fConst16: 0.0,
			fHslider30: 0.0,
			fRec317: [0.0;2],
			fHslider31: 0.0,
			fRec318: [0.0;2],
			fConst17: 0.0,
			fHslider32: 0.0,
			fRec320: [0.0;2],
			fHslider33: 0.0,
			fRec319: [0.0;2097152],
			fHslider34: 0.0,
			fConst20: 0.0,
			fConst21: 0.0,
			fConst23: 0.0,
			fConst24: 0.0,
			fConst25: 0.0,
			fConst26: 0.0,
			fConst29: 0.0,
			fConst30: 0.0,
			fConst31: 0.0,
			fConst32: 0.0,
			fConst33: 0.0,
			fConst34: 0.0,
			fConst35: 0.0,
			fHslider35: 0.0,
			fRec331: [0.0;2],
			fRec333: [0.0;2],
			fRec337: [0.0;2],
			fVec45: [0.0;16384],
			fVec46: [0.0;2],
			fRec336: [0.0;2],
			fRec334: [0.0;2],
			fRec339: [0.0;2],
			fVec47: [0.0;16384],
			fVec48: [0.0;2],
			fRec338: [0.0;2],
			fRec335: [0.0;2],
			fRec343: [0.0;2],
			fVec49: [0.0;16384],
			fVec50: [0.0;2],
			fRec342: [0.0;2],
			fRec340: [0.0;2],
			fRec345: [0.0;2],
			fVec51: [0.0;16384],
			fVec52: [0.0;2],
			fRec344: [0.0;2],
			fRec341: [0.0;2],
			fRec349: [0.0;2],
			fVec53: [0.0;16384],
			fVec54: [0.0;2],
			fRec348: [0.0;2],
			fRec346: [0.0;2],
			fRec351: [0.0;2],
			fVec55: [0.0;16384],
			fVec56: [0.0;2],
			fRec350: [0.0;2],
			fRec347: [0.0;2],
			fRec355: [0.0;2],
			fVec57: [0.0;16384],
			fVec58: [0.0;2],
			fRec354: [0.0;2],
			fRec352: [0.0;2],
			fRec357: [0.0;2],
			fVec59: [0.0;16384],
			fVec60: [0.0;2],
			fRec356: [0.0;2],
			fRec353: [0.0;2],
			fRec361: [0.0;2],
			fVec61: [0.0;16384],
			fVec62: [0.0;2],
			fRec360: [0.0;2],
			fRec358: [0.0;2],
			fRec363: [0.0;2],
			fVec63: [0.0;16384],
			fVec64: [0.0;2],
			fRec362: [0.0;2],
			fRec359: [0.0;2],
			fVec65: [0.0;1024],
			fHslider36: 0.0,
			fConst36: 0.0,
			fRec364: [0.0;2],
			fRec365: [0.0;2],
			fHslider37: 0.0,
			fVec66: [0.0;16384],
			fVec67: [0.0;2],
			fRec332: [0.0;2],
			fRec369: [0.0;2],
			fRec371: [0.0;2],
			fVec68: [0.0;1024],
			fVec69: [0.0;16384],
			fVec70: [0.0;2],
			fRec370: [0.0;2],
			fVec71: [0.0;16384],
			fVec72: [0.0;2],
			fRec368: [0.0;2],
			fRec366: [0.0;2],
			fRec373: [0.0;2],
			fVec73: [0.0;16384],
			fVec74: [0.0;2],
			fRec372: [0.0;2],
			fRec367: [0.0;2],
			fRec377: [0.0;2],
			fVec75: [0.0;16384],
			fVec76: [0.0;2],
			fRec376: [0.0;2],
			fRec374: [0.0;2],
			fRec379: [0.0;2],
			fVec77: [0.0;16384],
			fVec78: [0.0;2],
			fRec378: [0.0;2],
			fRec375: [0.0;2],
			fRec383: [0.0;2],
			fVec79: [0.0;16384],
			fVec80: [0.0;2],
			fRec382: [0.0;2],
			fRec380: [0.0;2],
			fRec385: [0.0;2],
			fVec81: [0.0;16384],
			fVec82: [0.0;2],
			fRec384: [0.0;2],
			fRec381: [0.0;2],
			fRec389: [0.0;2],
			fVec83: [0.0;16384],
			fVec84: [0.0;2],
			fRec388: [0.0;2],
			fRec386: [0.0;2],
			fRec391: [0.0;2],
			fVec85: [0.0;16384],
			fVec86: [0.0;2],
			fRec390: [0.0;2],
			fRec387: [0.0;2],
			fRec395: [0.0;2],
			fVec87: [0.0;16384],
			fVec88: [0.0;2],
			fRec394: [0.0;2],
			fRec392: [0.0;2],
			fRec397: [0.0;2],
			fVec89: [0.0;16384],
			fVec90: [0.0;2],
			fRec396: [0.0;2],
			fRec393: [0.0;2],
			fVec91: [0.0;16384],
			fVec92: [0.0;16384],
			fVec93: [0.0;2],
			fRec330: [0.0;2],
			fConst37: 0.0,
			fConst39: 0.0,
			fRec329: [0.0;2],
			fRec328: [0.0;3],
			fRec327: [0.0;3],
			fVec94: [0.0;2],
			fConst40: 0.0,
			fConst42: 0.0,
			fRec326: [0.0;2],
			fRec325: [0.0;3],
			fRec324: [0.0;3],
			fConst43: 0.0,
			fConst44: 0.0,
			fConst45: 0.0,
			fRec400: [0.0;2],
			fRec399: [0.0;3],
			fConst46: 0.0,
			fRec398: [0.0;3],
			fConst47: 0.0,
			fConst48: 0.0,
			fConst49: 0.0,
			fRec404: [0.0;2],
			fRec403: [0.0;3],
			fConst50: 0.0,
			fRec402: [0.0;3],
			fRec401: [0.0;3],
			fHslider38: 0.0,
			fVec95: [0.0;1024],
			fHslider39: 0.0,
			fRec323: [0.0;2],
			fHslider40: 0.0,
			fRec416: [0.0;2],
			fVec96: [0.0;16384],
			fVec97: [0.0;16384],
			fVec98: [0.0;2],
			fRec415: [0.0;2],
			fRec414: [0.0;2],
			fRec413: [0.0;3],
			fRec412: [0.0;3],
			fVec99: [0.0;2],
			fRec411: [0.0;2],
			fRec410: [0.0;3],
			fRec409: [0.0;3],
			fRec419: [0.0;2],
			fRec418: [0.0;3],
			fRec417: [0.0;3],
			fRec423: [0.0;2],
			fRec422: [0.0;3],
			fRec421: [0.0;3],
			fRec420: [0.0;3],
			fVec100: [0.0;1024],
			fRec408: [0.0;2],
			fVec101: [0.0;16384],
			fVec102: [0.0;2],
			fRec407: [0.0;2],
			fRec405: [0.0;2],
			fRec425: [0.0;2],
			fVec103: [0.0;16384],
			fVec104: [0.0;2],
			fRec424: [0.0;2],
			fRec406: [0.0;2],
			fVec105: [0.0;16384],
			fVec106: [0.0;2],
			fRec428: [0.0;2],
			fRec426: [0.0;2],
			fVec107: [0.0;16384],
			fVec108: [0.0;2],
			fRec429: [0.0;2],
			fRec427: [0.0;2],
			fVec109: [0.0;16384],
			fVec110: [0.0;2],
			fRec432: [0.0;2],
			fRec430: [0.0;2],
			fRec434: [0.0;2],
			fVec111: [0.0;16384],
			fVec112: [0.0;2],
			fRec433: [0.0;2],
			fRec431: [0.0;2],
			fRec438: [0.0;2],
			fVec113: [0.0;16384],
			fVec114: [0.0;2],
			fRec437: [0.0;2],
			fRec435: [0.0;2],
			fVec115: [0.0;16384],
			fVec116: [0.0;2],
			fRec439: [0.0;2],
			fRec436: [0.0;2],
			fRec321: [0.0;2],
			fRec322: [0.0;2],
			fHslider41: 0.0,
			fRec440: [0.0;2],
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
		m.declare("filters.lib/lowpass0_highpass1", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
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
		m.declare("reverbs.lib/jpverb:author", "Julian Parker, bug fixes and minor interface changes by Till Bovermann");
		m.declare("reverbs.lib/jpverb:license", "GPL2+");
		m.declare("reverbs.lib/name", "Faust Reverb Library");
		m.declare("reverbs.lib/version", "0.2");
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
		self.fHslider35 = 5.0;
		self.fHslider36 = 0.6;
		self.fHslider37 = 0.98;
		self.fHslider38 = 3.5;
		self.fHslider39 = 0.88;
		self.fHslider40 = 0.75;
		self.fHslider41 = 1.0;
	}
	fn instance_clear(&mut self) {
		for l0 in 0..2 {
			self.iVec0[(l0) as usize] = 0;
		}
		for l1 in 0..2 {
			self.fRec0[(l1) as usize] = 0.0;
		}
		for l3 in 0..2 {
			self.fRec3[(l3) as usize] = 0.0;
		}
		for l4 in 0..2 {
			self.fRec1[(l4) as usize] = 0.0;
		}
		self.IOTA0 = 0;
		for l5 in 0..2 {
			self.fRec40[(l5) as usize] = 0.0;
		}
		for l6 in 0..2 {
			self.fRec39[(l6) as usize] = 0.0;
		}
		for l7 in 0..2 {
			self.fRec35[(l7) as usize] = 0.0;
		}
		for l8 in 0..2 {
			self.fRec41[(l8) as usize] = 0.0;
		}
		for l9 in 0..2 {
			self.fRec46[(l9) as usize] = 0.0;
		}
		for l10 in 0..2 {
			self.iVec1[(l10) as usize] = 0;
		}
		for l11 in 0..2 {
			self.iRec47[(l11) as usize] = 0;
		}
		for l12 in 0..2 {
			self.fRec44[(l12) as usize] = 0.0;
		}
		for l13 in 0..2 {
			self.fRec43[(l13) as usize] = 0.0;
		}
		for l14 in 0..4 {
			self.fRec48[(l14) as usize] = 0.0;
		}
		for l15 in 0..2048 {
			self.fRec49[(l15) as usize] = 0.0;
		}
		for l16 in 0..2 {
			self.fVec2[(l16) as usize] = 0.0;
		}
		for l17 in 0..2 {
			self.fVec3[(l17) as usize] = 0.0;
		}
		for l18 in 0..2 {
			self.iRec50[(l18) as usize] = 0;
		}
		for l19 in 0..2 {
			self.iRec52[(l19) as usize] = 0;
		}
		for l20 in 0..3 {
			self.fRec51[(l20) as usize] = 0.0;
		}
		for l21 in 0..3 {
			self.fVec4[(l21) as usize] = 0.0;
		}
		for l22 in 0..2048 {
			self.fRec42[(l22) as usize] = 0.0;
		}
		for l23 in 0..2 {
			self.fRec31[(l23) as usize] = 0.0;
		}
		for l24 in 0..2 {
			self.fRec27[(l24) as usize] = 0.0;
		}
		for l25 in 0..2048 {
			self.fRec23[(l25) as usize] = 0.0;
		}
		for l26 in 0..2 {
			self.fRec25[(l26) as usize] = 0.0;
		}
		for l27 in 0..4 {
			self.fRec21[(l27) as usize] = 0.0;
		}
		for l28 in 0..2 {
			self.fRec16[(l28) as usize] = 0.0;
		}
		for l29 in 0..2048 {
			self.fRec12[(l29) as usize] = 0.0;
		}
		for l30 in 0..2 {
			self.fRec10[(l30) as usize] = 0.0;
		}
		for l31 in 0..2 {
			self.fRec9[(l31) as usize] = 0.0;
		}
		for l32 in 0..2 {
			self.fRec8[(l32) as usize] = 0.0;
		}
		for l33 in 0..2 {
			self.fRec6[(l33) as usize] = 0.0;
		}
		for l34 in 0..3 {
			self.fRec5[(l34) as usize] = 0.0;
		}
		for l35 in 0..3 {
			self.fRec4[(l35) as usize] = 0.0;
		}
		for l36 in 0..2 {
			self.fRec53[(l36) as usize] = 0.0;
		}
		for l37 in 0..2 {
			self.fRec89[(l37) as usize] = 0.0;
		}
		for l38 in 0..2 {
			self.fRec85[(l38) as usize] = 0.0;
		}
		for l39 in 0..2 {
			self.fRec90[(l39) as usize] = 0.0;
		}
		for l40 in 0..2 {
			self.fRec95[(l40) as usize] = 0.0;
		}
		for l41 in 0..2 {
			self.iVec5[(l41) as usize] = 0;
		}
		for l42 in 0..2 {
			self.iRec96[(l42) as usize] = 0;
		}
		for l43 in 0..2 {
			self.fRec93[(l43) as usize] = 0.0;
		}
		for l44 in 0..2 {
			self.fRec92[(l44) as usize] = 0.0;
		}
		for l45 in 0..4 {
			self.fRec97[(l45) as usize] = 0.0;
		}
		for l46 in 0..2048 {
			self.fRec98[(l46) as usize] = 0.0;
		}
		for l47 in 0..2 {
			self.fVec6[(l47) as usize] = 0.0;
		}
		for l48 in 0..2 {
			self.fVec7[(l48) as usize] = 0.0;
		}
		for l49 in 0..2 {
			self.iRec99[(l49) as usize] = 0;
		}
		for l50 in 0..3 {
			self.fRec100[(l50) as usize] = 0.0;
		}
		for l51 in 0..3 {
			self.fVec8[(l51) as usize] = 0.0;
		}
		for l52 in 0..2048 {
			self.fRec91[(l52) as usize] = 0.0;
		}
		for l53 in 0..2 {
			self.fRec81[(l53) as usize] = 0.0;
		}
		for l54 in 0..2 {
			self.fRec77[(l54) as usize] = 0.0;
		}
		for l55 in 0..2048 {
			self.fRec73[(l55) as usize] = 0.0;
		}
		for l56 in 0..2 {
			self.fRec75[(l56) as usize] = 0.0;
		}
		for l57 in 0..4 {
			self.fRec71[(l57) as usize] = 0.0;
		}
		for l58 in 0..2 {
			self.fRec66[(l58) as usize] = 0.0;
		}
		for l59 in 0..2048 {
			self.fRec62[(l59) as usize] = 0.0;
		}
		for l60 in 0..2 {
			self.fRec60[(l60) as usize] = 0.0;
		}
		for l61 in 0..2 {
			self.fRec59[(l61) as usize] = 0.0;
		}
		for l62 in 0..2 {
			self.fRec58[(l62) as usize] = 0.0;
		}
		for l63 in 0..2 {
			self.fRec56[(l63) as usize] = 0.0;
		}
		for l64 in 0..3 {
			self.fRec55[(l64) as usize] = 0.0;
		}
		for l65 in 0..3 {
			self.fRec54[(l65) as usize] = 0.0;
		}
		for l66 in 0..2 {
			self.fRec101[(l66) as usize] = 0.0;
		}
		for l67 in 0..2 {
			self.fRec137[(l67) as usize] = 0.0;
		}
		for l68 in 0..2 {
			self.fRec133[(l68) as usize] = 0.0;
		}
		for l69 in 0..2 {
			self.fRec138[(l69) as usize] = 0.0;
		}
		for l70 in 0..2 {
			self.fRec143[(l70) as usize] = 0.0;
		}
		for l71 in 0..2 {
			self.iVec9[(l71) as usize] = 0;
		}
		for l72 in 0..2 {
			self.iRec144[(l72) as usize] = 0;
		}
		for l73 in 0..2 {
			self.fRec141[(l73) as usize] = 0.0;
		}
		for l74 in 0..2 {
			self.fRec140[(l74) as usize] = 0.0;
		}
		for l75 in 0..4 {
			self.fRec145[(l75) as usize] = 0.0;
		}
		for l76 in 0..2048 {
			self.fRec146[(l76) as usize] = 0.0;
		}
		for l77 in 0..2 {
			self.fVec10[(l77) as usize] = 0.0;
		}
		for l78 in 0..2 {
			self.fVec11[(l78) as usize] = 0.0;
		}
		for l79 in 0..2 {
			self.iRec147[(l79) as usize] = 0;
		}
		for l80 in 0..3 {
			self.fRec148[(l80) as usize] = 0.0;
		}
		for l81 in 0..3 {
			self.fVec12[(l81) as usize] = 0.0;
		}
		for l82 in 0..2048 {
			self.fRec139[(l82) as usize] = 0.0;
		}
		for l83 in 0..2 {
			self.fRec129[(l83) as usize] = 0.0;
		}
		for l84 in 0..2 {
			self.fRec125[(l84) as usize] = 0.0;
		}
		for l85 in 0..2048 {
			self.fRec121[(l85) as usize] = 0.0;
		}
		for l86 in 0..2 {
			self.fRec123[(l86) as usize] = 0.0;
		}
		for l87 in 0..4 {
			self.fRec119[(l87) as usize] = 0.0;
		}
		for l88 in 0..2 {
			self.fRec114[(l88) as usize] = 0.0;
		}
		for l89 in 0..2048 {
			self.fRec110[(l89) as usize] = 0.0;
		}
		for l90 in 0..2 {
			self.fRec108[(l90) as usize] = 0.0;
		}
		for l91 in 0..2 {
			self.fRec107[(l91) as usize] = 0.0;
		}
		for l92 in 0..2 {
			self.fRec106[(l92) as usize] = 0.0;
		}
		for l93 in 0..2 {
			self.fRec104[(l93) as usize] = 0.0;
		}
		for l94 in 0..3 {
			self.fRec103[(l94) as usize] = 0.0;
		}
		for l95 in 0..3 {
			self.fRec102[(l95) as usize] = 0.0;
		}
		for l96 in 0..2 {
			self.fRec149[(l96) as usize] = 0.0;
		}
		for l97 in 0..2 {
			self.fRec185[(l97) as usize] = 0.0;
		}
		for l98 in 0..2 {
			self.fRec181[(l98) as usize] = 0.0;
		}
		for l99 in 0..2 {
			self.fRec186[(l99) as usize] = 0.0;
		}
		for l100 in 0..2 {
			self.fRec191[(l100) as usize] = 0.0;
		}
		for l101 in 0..2 {
			self.iVec13[(l101) as usize] = 0;
		}
		for l102 in 0..2 {
			self.iRec192[(l102) as usize] = 0;
		}
		for l103 in 0..2 {
			self.fRec189[(l103) as usize] = 0.0;
		}
		for l104 in 0..2 {
			self.fRec188[(l104) as usize] = 0.0;
		}
		for l105 in 0..4 {
			self.fRec193[(l105) as usize] = 0.0;
		}
		for l106 in 0..2048 {
			self.fRec194[(l106) as usize] = 0.0;
		}
		for l107 in 0..2 {
			self.fVec14[(l107) as usize] = 0.0;
		}
		for l108 in 0..2 {
			self.fVec15[(l108) as usize] = 0.0;
		}
		for l109 in 0..2 {
			self.iRec195[(l109) as usize] = 0;
		}
		for l110 in 0..3 {
			self.fRec196[(l110) as usize] = 0.0;
		}
		for l111 in 0..3 {
			self.fVec16[(l111) as usize] = 0.0;
		}
		for l112 in 0..2048 {
			self.fRec187[(l112) as usize] = 0.0;
		}
		for l113 in 0..2 {
			self.fRec177[(l113) as usize] = 0.0;
		}
		for l114 in 0..2 {
			self.fRec173[(l114) as usize] = 0.0;
		}
		for l115 in 0..2048 {
			self.fRec169[(l115) as usize] = 0.0;
		}
		for l116 in 0..2 {
			self.fRec171[(l116) as usize] = 0.0;
		}
		for l117 in 0..4 {
			self.fRec167[(l117) as usize] = 0.0;
		}
		for l118 in 0..2 {
			self.fRec162[(l118) as usize] = 0.0;
		}
		for l119 in 0..2048 {
			self.fRec158[(l119) as usize] = 0.0;
		}
		for l120 in 0..2 {
			self.fRec156[(l120) as usize] = 0.0;
		}
		for l121 in 0..2 {
			self.fRec155[(l121) as usize] = 0.0;
		}
		for l122 in 0..2 {
			self.fRec154[(l122) as usize] = 0.0;
		}
		for l123 in 0..2 {
			self.fRec152[(l123) as usize] = 0.0;
		}
		for l124 in 0..3 {
			self.fRec151[(l124) as usize] = 0.0;
		}
		for l125 in 0..3 {
			self.fRec150[(l125) as usize] = 0.0;
		}
		for l126 in 0..2 {
			self.fRec197[(l126) as usize] = 0.0;
		}
		for l127 in 0..2 {
			self.fRec233[(l127) as usize] = 0.0;
		}
		for l128 in 0..2 {
			self.fRec229[(l128) as usize] = 0.0;
		}
		for l129 in 0..2 {
			self.fRec234[(l129) as usize] = 0.0;
		}
		for l130 in 0..2 {
			self.fRec239[(l130) as usize] = 0.0;
		}
		for l131 in 0..2 {
			self.iVec17[(l131) as usize] = 0;
		}
		for l132 in 0..2 {
			self.iRec240[(l132) as usize] = 0;
		}
		for l133 in 0..2 {
			self.fRec237[(l133) as usize] = 0.0;
		}
		for l134 in 0..2 {
			self.fRec236[(l134) as usize] = 0.0;
		}
		for l135 in 0..4 {
			self.fRec241[(l135) as usize] = 0.0;
		}
		for l136 in 0..2048 {
			self.fRec242[(l136) as usize] = 0.0;
		}
		for l137 in 0..2 {
			self.fVec18[(l137) as usize] = 0.0;
		}
		for l138 in 0..2 {
			self.fVec19[(l138) as usize] = 0.0;
		}
		for l139 in 0..2 {
			self.iRec243[(l139) as usize] = 0;
		}
		for l140 in 0..3 {
			self.fRec244[(l140) as usize] = 0.0;
		}
		for l141 in 0..3 {
			self.fVec20[(l141) as usize] = 0.0;
		}
		for l142 in 0..2048 {
			self.fRec235[(l142) as usize] = 0.0;
		}
		for l143 in 0..2 {
			self.fRec225[(l143) as usize] = 0.0;
		}
		for l144 in 0..2 {
			self.fRec221[(l144) as usize] = 0.0;
		}
		for l145 in 0..2048 {
			self.fRec217[(l145) as usize] = 0.0;
		}
		for l146 in 0..2 {
			self.fRec219[(l146) as usize] = 0.0;
		}
		for l147 in 0..4 {
			self.fRec215[(l147) as usize] = 0.0;
		}
		for l148 in 0..2 {
			self.fRec210[(l148) as usize] = 0.0;
		}
		for l149 in 0..2048 {
			self.fRec206[(l149) as usize] = 0.0;
		}
		for l150 in 0..2 {
			self.fRec204[(l150) as usize] = 0.0;
		}
		for l151 in 0..2 {
			self.fRec203[(l151) as usize] = 0.0;
		}
		for l152 in 0..2 {
			self.fRec202[(l152) as usize] = 0.0;
		}
		for l153 in 0..2 {
			self.fRec200[(l153) as usize] = 0.0;
		}
		for l154 in 0..3 {
			self.fRec199[(l154) as usize] = 0.0;
		}
		for l155 in 0..3 {
			self.fRec198[(l155) as usize] = 0.0;
		}
		for l156 in 0..2 {
			self.fRec245[(l156) as usize] = 0.0;
		}
		for l157 in 0..2 {
			self.fRec246[(l157) as usize] = 0.0;
		}
		for l158 in 0..2 {
			self.fRec251[(l158) as usize] = 0.0;
		}
		for l159 in 0..2 {
			self.fRec249[(l159) as usize] = 0.0;
		}
		for l160 in 0..2 {
			self.fRec252[(l160) as usize] = 0.0;
		}
		for l161 in 0..3 {
			self.fRec248[(l161) as usize] = 0.0;
		}
		for l162 in 0..3 {
			self.fRec247[(l162) as usize] = 0.0;
		}
		for l163 in 0..2 {
			self.fRec253[(l163) as usize] = 0.0;
		}
		for l164 in 0..2 {
			self.fRec258[(l164) as usize] = 0.0;
		}
		for l165 in 0..2 {
			self.fRec256[(l165) as usize] = 0.0;
		}
		for l166 in 0..2 {
			self.fRec259[(l166) as usize] = 0.0;
		}
		for l167 in 0..3 {
			self.fRec255[(l167) as usize] = 0.0;
		}
		for l168 in 0..3 {
			self.fRec254[(l168) as usize] = 0.0;
		}
		for l169 in 0..2 {
			self.fRec260[(l169) as usize] = 0.0;
		}
		for l170 in 0..2 {
			self.fRec265[(l170) as usize] = 0.0;
		}
		for l171 in 0..2 {
			self.fRec263[(l171) as usize] = 0.0;
		}
		for l172 in 0..2 {
			self.fRec266[(l172) as usize] = 0.0;
		}
		for l173 in 0..3 {
			self.fRec262[(l173) as usize] = 0.0;
		}
		for l174 in 0..3 {
			self.fRec261[(l174) as usize] = 0.0;
		}
		for l175 in 0..2 {
			self.fRec267[(l175) as usize] = 0.0;
		}
		for l176 in 0..2 {
			self.fRec272[(l176) as usize] = 0.0;
		}
		for l177 in 0..2 {
			self.fRec270[(l177) as usize] = 0.0;
		}
		for l178 in 0..2 {
			self.fRec273[(l178) as usize] = 0.0;
		}
		for l179 in 0..3 {
			self.fRec269[(l179) as usize] = 0.0;
		}
		for l180 in 0..3 {
			self.fRec268[(l180) as usize] = 0.0;
		}
		for l181 in 0..2 {
			self.fRec274[(l181) as usize] = 0.0;
		}
		for l182 in 0..2 {
			self.fRec275[(l182) as usize] = 0.0;
		}
		for l183 in 0..2 {
			self.fRec276[(l183) as usize] = 0.0;
		}
		for l184 in 0..2 {
			self.fRec278[(l184) as usize] = 0.0;
		}
		for l185 in 0..2 {
			self.fRec279[(l185) as usize] = 0.0;
		}
		for l186 in 0..2 {
			self.fVec21[(l186) as usize] = 0.0;
		}
		for l187 in 0..4096 {
			self.fVec22[(l187) as usize] = 0.0;
		}
		for l188 in 0..2 {
			self.fRec277[(l188) as usize] = 0.0;
		}
		for l189 in 0..2 {
			self.fRec281[(l189) as usize] = 0.0;
		}
		for l190 in 0..2 {
			self.fRec282[(l190) as usize] = 0.0;
		}
		for l191 in 0..2 {
			self.fVec23[(l191) as usize] = 0.0;
		}
		for l192 in 0..4096 {
			self.fVec24[(l192) as usize] = 0.0;
		}
		for l193 in 0..2 {
			self.fRec280[(l193) as usize] = 0.0;
		}
		for l194 in 0..2 {
			self.fRec284[(l194) as usize] = 0.0;
		}
		for l195 in 0..2 {
			self.fRec285[(l195) as usize] = 0.0;
		}
		for l196 in 0..2 {
			self.fVec25[(l196) as usize] = 0.0;
		}
		for l197 in 0..4096 {
			self.fVec26[(l197) as usize] = 0.0;
		}
		for l198 in 0..2 {
			self.fRec283[(l198) as usize] = 0.0;
		}
		for l199 in 0..2 {
			self.fRec286[(l199) as usize] = 0.0;
		}
		for l200 in 0..2 {
			self.fRec288[(l200) as usize] = 0.0;
		}
		for l201 in 0..2 {
			self.fRec289[(l201) as usize] = 0.0;
		}
		for l202 in 0..2 {
			self.fVec27[(l202) as usize] = 0.0;
		}
		for l203 in 0..4096 {
			self.fVec28[(l203) as usize] = 0.0;
		}
		for l204 in 0..2 {
			self.fRec287[(l204) as usize] = 0.0;
		}
		for l205 in 0..2 {
			self.fRec291[(l205) as usize] = 0.0;
		}
		for l206 in 0..2 {
			self.fRec292[(l206) as usize] = 0.0;
		}
		for l207 in 0..2 {
			self.fVec29[(l207) as usize] = 0.0;
		}
		for l208 in 0..4096 {
			self.fVec30[(l208) as usize] = 0.0;
		}
		for l209 in 0..2 {
			self.fRec290[(l209) as usize] = 0.0;
		}
		for l210 in 0..2 {
			self.fRec294[(l210) as usize] = 0.0;
		}
		for l211 in 0..2 {
			self.fRec295[(l211) as usize] = 0.0;
		}
		for l212 in 0..2 {
			self.fVec31[(l212) as usize] = 0.0;
		}
		for l213 in 0..4096 {
			self.fVec32[(l213) as usize] = 0.0;
		}
		for l214 in 0..2 {
			self.fRec293[(l214) as usize] = 0.0;
		}
		for l215 in 0..2 {
			self.fRec296[(l215) as usize] = 0.0;
		}
		for l216 in 0..2 {
			self.fRec298[(l216) as usize] = 0.0;
		}
		for l217 in 0..2 {
			self.fRec299[(l217) as usize] = 0.0;
		}
		for l218 in 0..2 {
			self.fVec33[(l218) as usize] = 0.0;
		}
		for l219 in 0..4096 {
			self.fVec34[(l219) as usize] = 0.0;
		}
		for l220 in 0..2 {
			self.fRec297[(l220) as usize] = 0.0;
		}
		for l221 in 0..2 {
			self.fRec301[(l221) as usize] = 0.0;
		}
		for l222 in 0..2 {
			self.fRec302[(l222) as usize] = 0.0;
		}
		for l223 in 0..2 {
			self.fVec35[(l223) as usize] = 0.0;
		}
		for l224 in 0..4096 {
			self.fVec36[(l224) as usize] = 0.0;
		}
		for l225 in 0..2 {
			self.fRec300[(l225) as usize] = 0.0;
		}
		for l226 in 0..2 {
			self.fRec304[(l226) as usize] = 0.0;
		}
		for l227 in 0..2 {
			self.fRec305[(l227) as usize] = 0.0;
		}
		for l228 in 0..2 {
			self.fVec37[(l228) as usize] = 0.0;
		}
		for l229 in 0..4096 {
			self.fVec38[(l229) as usize] = 0.0;
		}
		for l230 in 0..2 {
			self.fRec303[(l230) as usize] = 0.0;
		}
		for l231 in 0..2 {
			self.fRec306[(l231) as usize] = 0.0;
		}
		for l232 in 0..2 {
			self.fRec308[(l232) as usize] = 0.0;
		}
		for l233 in 0..2 {
			self.fRec309[(l233) as usize] = 0.0;
		}
		for l234 in 0..2 {
			self.fVec39[(l234) as usize] = 0.0;
		}
		for l235 in 0..4096 {
			self.fVec40[(l235) as usize] = 0.0;
		}
		for l236 in 0..2 {
			self.fRec307[(l236) as usize] = 0.0;
		}
		for l237 in 0..2 {
			self.fRec311[(l237) as usize] = 0.0;
		}
		for l238 in 0..2 {
			self.fRec312[(l238) as usize] = 0.0;
		}
		for l239 in 0..2 {
			self.fVec41[(l239) as usize] = 0.0;
		}
		for l240 in 0..4096 {
			self.fVec42[(l240) as usize] = 0.0;
		}
		for l241 in 0..2 {
			self.fRec310[(l241) as usize] = 0.0;
		}
		for l242 in 0..2 {
			self.fRec314[(l242) as usize] = 0.0;
		}
		for l243 in 0..2 {
			self.fRec315[(l243) as usize] = 0.0;
		}
		for l244 in 0..2 {
			self.fVec43[(l244) as usize] = 0.0;
		}
		for l245 in 0..4096 {
			self.fVec44[(l245) as usize] = 0.0;
		}
		for l246 in 0..2 {
			self.fRec313[(l246) as usize] = 0.0;
		}
		for l247 in 0..2 {
			self.fRec316[(l247) as usize] = 0.0;
		}
		for l248 in 0..2 {
			self.fRec317[(l248) as usize] = 0.0;
		}
		for l249 in 0..2 {
			self.fRec318[(l249) as usize] = 0.0;
		}
		for l250 in 0..2 {
			self.fRec320[(l250) as usize] = 0.0;
		}
		for l251 in 0..2097152 {
			self.fRec319[(l251) as usize] = 0.0;
		}
		for l252 in 0..2 {
			self.fRec331[(l252) as usize] = 0.0;
		}
		for l253 in 0..2 {
			self.fRec333[(l253) as usize] = 0.0;
		}
		for l254 in 0..2 {
			self.fRec337[(l254) as usize] = 0.0;
		}
		for l255 in 0..16384 {
			self.fVec45[(l255) as usize] = 0.0;
		}
		for l256 in 0..2 {
			self.fVec46[(l256) as usize] = 0.0;
		}
		for l257 in 0..2 {
			self.fRec336[(l257) as usize] = 0.0;
		}
		for l258 in 0..2 {
			self.fRec334[(l258) as usize] = 0.0;
		}
		for l259 in 0..2 {
			self.fRec339[(l259) as usize] = 0.0;
		}
		for l260 in 0..16384 {
			self.fVec47[(l260) as usize] = 0.0;
		}
		for l261 in 0..2 {
			self.fVec48[(l261) as usize] = 0.0;
		}
		for l262 in 0..2 {
			self.fRec338[(l262) as usize] = 0.0;
		}
		for l263 in 0..2 {
			self.fRec335[(l263) as usize] = 0.0;
		}
		for l264 in 0..2 {
			self.fRec343[(l264) as usize] = 0.0;
		}
		for l265 in 0..16384 {
			self.fVec49[(l265) as usize] = 0.0;
		}
		for l266 in 0..2 {
			self.fVec50[(l266) as usize] = 0.0;
		}
		for l267 in 0..2 {
			self.fRec342[(l267) as usize] = 0.0;
		}
		for l268 in 0..2 {
			self.fRec340[(l268) as usize] = 0.0;
		}
		for l269 in 0..2 {
			self.fRec345[(l269) as usize] = 0.0;
		}
		for l270 in 0..16384 {
			self.fVec51[(l270) as usize] = 0.0;
		}
		for l271 in 0..2 {
			self.fVec52[(l271) as usize] = 0.0;
		}
		for l272 in 0..2 {
			self.fRec344[(l272) as usize] = 0.0;
		}
		for l273 in 0..2 {
			self.fRec341[(l273) as usize] = 0.0;
		}
		for l274 in 0..2 {
			self.fRec349[(l274) as usize] = 0.0;
		}
		for l275 in 0..16384 {
			self.fVec53[(l275) as usize] = 0.0;
		}
		for l276 in 0..2 {
			self.fVec54[(l276) as usize] = 0.0;
		}
		for l277 in 0..2 {
			self.fRec348[(l277) as usize] = 0.0;
		}
		for l278 in 0..2 {
			self.fRec346[(l278) as usize] = 0.0;
		}
		for l279 in 0..2 {
			self.fRec351[(l279) as usize] = 0.0;
		}
		for l280 in 0..16384 {
			self.fVec55[(l280) as usize] = 0.0;
		}
		for l281 in 0..2 {
			self.fVec56[(l281) as usize] = 0.0;
		}
		for l282 in 0..2 {
			self.fRec350[(l282) as usize] = 0.0;
		}
		for l283 in 0..2 {
			self.fRec347[(l283) as usize] = 0.0;
		}
		for l284 in 0..2 {
			self.fRec355[(l284) as usize] = 0.0;
		}
		for l285 in 0..16384 {
			self.fVec57[(l285) as usize] = 0.0;
		}
		for l286 in 0..2 {
			self.fVec58[(l286) as usize] = 0.0;
		}
		for l287 in 0..2 {
			self.fRec354[(l287) as usize] = 0.0;
		}
		for l288 in 0..2 {
			self.fRec352[(l288) as usize] = 0.0;
		}
		for l289 in 0..2 {
			self.fRec357[(l289) as usize] = 0.0;
		}
		for l290 in 0..16384 {
			self.fVec59[(l290) as usize] = 0.0;
		}
		for l291 in 0..2 {
			self.fVec60[(l291) as usize] = 0.0;
		}
		for l292 in 0..2 {
			self.fRec356[(l292) as usize] = 0.0;
		}
		for l293 in 0..2 {
			self.fRec353[(l293) as usize] = 0.0;
		}
		for l294 in 0..2 {
			self.fRec361[(l294) as usize] = 0.0;
		}
		for l295 in 0..16384 {
			self.fVec61[(l295) as usize] = 0.0;
		}
		for l296 in 0..2 {
			self.fVec62[(l296) as usize] = 0.0;
		}
		for l297 in 0..2 {
			self.fRec360[(l297) as usize] = 0.0;
		}
		for l298 in 0..2 {
			self.fRec358[(l298) as usize] = 0.0;
		}
		for l299 in 0..2 {
			self.fRec363[(l299) as usize] = 0.0;
		}
		for l300 in 0..16384 {
			self.fVec63[(l300) as usize] = 0.0;
		}
		for l301 in 0..2 {
			self.fVec64[(l301) as usize] = 0.0;
		}
		for l302 in 0..2 {
			self.fRec362[(l302) as usize] = 0.0;
		}
		for l303 in 0..2 {
			self.fRec359[(l303) as usize] = 0.0;
		}
		for l304 in 0..1024 {
			self.fVec65[(l304) as usize] = 0.0;
		}
		for l305 in 0..2 {
			self.fRec364[(l305) as usize] = 0.0;
		}
		for l306 in 0..2 {
			self.fRec365[(l306) as usize] = 0.0;
		}
		for l307 in 0..16384 {
			self.fVec66[(l307) as usize] = 0.0;
		}
		for l308 in 0..2 {
			self.fVec67[(l308) as usize] = 0.0;
		}
		for l309 in 0..2 {
			self.fRec332[(l309) as usize] = 0.0;
		}
		for l310 in 0..2 {
			self.fRec369[(l310) as usize] = 0.0;
		}
		for l311 in 0..2 {
			self.fRec371[(l311) as usize] = 0.0;
		}
		for l312 in 0..1024 {
			self.fVec68[(l312) as usize] = 0.0;
		}
		for l313 in 0..16384 {
			self.fVec69[(l313) as usize] = 0.0;
		}
		for l314 in 0..2 {
			self.fVec70[(l314) as usize] = 0.0;
		}
		for l315 in 0..2 {
			self.fRec370[(l315) as usize] = 0.0;
		}
		for l316 in 0..16384 {
			self.fVec71[(l316) as usize] = 0.0;
		}
		for l317 in 0..2 {
			self.fVec72[(l317) as usize] = 0.0;
		}
		for l318 in 0..2 {
			self.fRec368[(l318) as usize] = 0.0;
		}
		for l319 in 0..2 {
			self.fRec366[(l319) as usize] = 0.0;
		}
		for l320 in 0..2 {
			self.fRec373[(l320) as usize] = 0.0;
		}
		for l321 in 0..16384 {
			self.fVec73[(l321) as usize] = 0.0;
		}
		for l322 in 0..2 {
			self.fVec74[(l322) as usize] = 0.0;
		}
		for l323 in 0..2 {
			self.fRec372[(l323) as usize] = 0.0;
		}
		for l324 in 0..2 {
			self.fRec367[(l324) as usize] = 0.0;
		}
		for l325 in 0..2 {
			self.fRec377[(l325) as usize] = 0.0;
		}
		for l326 in 0..16384 {
			self.fVec75[(l326) as usize] = 0.0;
		}
		for l327 in 0..2 {
			self.fVec76[(l327) as usize] = 0.0;
		}
		for l328 in 0..2 {
			self.fRec376[(l328) as usize] = 0.0;
		}
		for l329 in 0..2 {
			self.fRec374[(l329) as usize] = 0.0;
		}
		for l330 in 0..2 {
			self.fRec379[(l330) as usize] = 0.0;
		}
		for l331 in 0..16384 {
			self.fVec77[(l331) as usize] = 0.0;
		}
		for l332 in 0..2 {
			self.fVec78[(l332) as usize] = 0.0;
		}
		for l333 in 0..2 {
			self.fRec378[(l333) as usize] = 0.0;
		}
		for l334 in 0..2 {
			self.fRec375[(l334) as usize] = 0.0;
		}
		for l335 in 0..2 {
			self.fRec383[(l335) as usize] = 0.0;
		}
		for l336 in 0..16384 {
			self.fVec79[(l336) as usize] = 0.0;
		}
		for l337 in 0..2 {
			self.fVec80[(l337) as usize] = 0.0;
		}
		for l338 in 0..2 {
			self.fRec382[(l338) as usize] = 0.0;
		}
		for l339 in 0..2 {
			self.fRec380[(l339) as usize] = 0.0;
		}
		for l340 in 0..2 {
			self.fRec385[(l340) as usize] = 0.0;
		}
		for l341 in 0..16384 {
			self.fVec81[(l341) as usize] = 0.0;
		}
		for l342 in 0..2 {
			self.fVec82[(l342) as usize] = 0.0;
		}
		for l343 in 0..2 {
			self.fRec384[(l343) as usize] = 0.0;
		}
		for l344 in 0..2 {
			self.fRec381[(l344) as usize] = 0.0;
		}
		for l345 in 0..2 {
			self.fRec389[(l345) as usize] = 0.0;
		}
		for l346 in 0..16384 {
			self.fVec83[(l346) as usize] = 0.0;
		}
		for l347 in 0..2 {
			self.fVec84[(l347) as usize] = 0.0;
		}
		for l348 in 0..2 {
			self.fRec388[(l348) as usize] = 0.0;
		}
		for l349 in 0..2 {
			self.fRec386[(l349) as usize] = 0.0;
		}
		for l350 in 0..2 {
			self.fRec391[(l350) as usize] = 0.0;
		}
		for l351 in 0..16384 {
			self.fVec85[(l351) as usize] = 0.0;
		}
		for l352 in 0..2 {
			self.fVec86[(l352) as usize] = 0.0;
		}
		for l353 in 0..2 {
			self.fRec390[(l353) as usize] = 0.0;
		}
		for l354 in 0..2 {
			self.fRec387[(l354) as usize] = 0.0;
		}
		for l355 in 0..2 {
			self.fRec395[(l355) as usize] = 0.0;
		}
		for l356 in 0..16384 {
			self.fVec87[(l356) as usize] = 0.0;
		}
		for l357 in 0..2 {
			self.fVec88[(l357) as usize] = 0.0;
		}
		for l358 in 0..2 {
			self.fRec394[(l358) as usize] = 0.0;
		}
		for l359 in 0..2 {
			self.fRec392[(l359) as usize] = 0.0;
		}
		for l360 in 0..2 {
			self.fRec397[(l360) as usize] = 0.0;
		}
		for l361 in 0..16384 {
			self.fVec89[(l361) as usize] = 0.0;
		}
		for l362 in 0..2 {
			self.fVec90[(l362) as usize] = 0.0;
		}
		for l363 in 0..2 {
			self.fRec396[(l363) as usize] = 0.0;
		}
		for l364 in 0..2 {
			self.fRec393[(l364) as usize] = 0.0;
		}
		for l365 in 0..16384 {
			self.fVec91[(l365) as usize] = 0.0;
		}
		for l366 in 0..16384 {
			self.fVec92[(l366) as usize] = 0.0;
		}
		for l367 in 0..2 {
			self.fVec93[(l367) as usize] = 0.0;
		}
		for l368 in 0..2 {
			self.fRec330[(l368) as usize] = 0.0;
		}
		for l369 in 0..2 {
			self.fRec329[(l369) as usize] = 0.0;
		}
		for l370 in 0..3 {
			self.fRec328[(l370) as usize] = 0.0;
		}
		for l371 in 0..3 {
			self.fRec327[(l371) as usize] = 0.0;
		}
		for l372 in 0..2 {
			self.fVec94[(l372) as usize] = 0.0;
		}
		for l373 in 0..2 {
			self.fRec326[(l373) as usize] = 0.0;
		}
		for l374 in 0..3 {
			self.fRec325[(l374) as usize] = 0.0;
		}
		for l375 in 0..3 {
			self.fRec324[(l375) as usize] = 0.0;
		}
		for l376 in 0..2 {
			self.fRec400[(l376) as usize] = 0.0;
		}
		for l377 in 0..3 {
			self.fRec399[(l377) as usize] = 0.0;
		}
		for l378 in 0..3 {
			self.fRec398[(l378) as usize] = 0.0;
		}
		for l379 in 0..2 {
			self.fRec404[(l379) as usize] = 0.0;
		}
		for l380 in 0..3 {
			self.fRec403[(l380) as usize] = 0.0;
		}
		for l381 in 0..3 {
			self.fRec402[(l381) as usize] = 0.0;
		}
		for l382 in 0..3 {
			self.fRec401[(l382) as usize] = 0.0;
		}
		for l383 in 0..1024 {
			self.fVec95[(l383) as usize] = 0.0;
		}
		for l384 in 0..2 {
			self.fRec323[(l384) as usize] = 0.0;
		}
		for l385 in 0..2 {
			self.fRec416[(l385) as usize] = 0.0;
		}
		for l386 in 0..16384 {
			self.fVec96[(l386) as usize] = 0.0;
		}
		for l387 in 0..16384 {
			self.fVec97[(l387) as usize] = 0.0;
		}
		for l388 in 0..2 {
			self.fVec98[(l388) as usize] = 0.0;
		}
		for l389 in 0..2 {
			self.fRec415[(l389) as usize] = 0.0;
		}
		for l390 in 0..2 {
			self.fRec414[(l390) as usize] = 0.0;
		}
		for l391 in 0..3 {
			self.fRec413[(l391) as usize] = 0.0;
		}
		for l392 in 0..3 {
			self.fRec412[(l392) as usize] = 0.0;
		}
		for l393 in 0..2 {
			self.fVec99[(l393) as usize] = 0.0;
		}
		for l394 in 0..2 {
			self.fRec411[(l394) as usize] = 0.0;
		}
		for l395 in 0..3 {
			self.fRec410[(l395) as usize] = 0.0;
		}
		for l396 in 0..3 {
			self.fRec409[(l396) as usize] = 0.0;
		}
		for l397 in 0..2 {
			self.fRec419[(l397) as usize] = 0.0;
		}
		for l398 in 0..3 {
			self.fRec418[(l398) as usize] = 0.0;
		}
		for l399 in 0..3 {
			self.fRec417[(l399) as usize] = 0.0;
		}
		for l400 in 0..2 {
			self.fRec423[(l400) as usize] = 0.0;
		}
		for l401 in 0..3 {
			self.fRec422[(l401) as usize] = 0.0;
		}
		for l402 in 0..3 {
			self.fRec421[(l402) as usize] = 0.0;
		}
		for l403 in 0..3 {
			self.fRec420[(l403) as usize] = 0.0;
		}
		for l404 in 0..1024 {
			self.fVec100[(l404) as usize] = 0.0;
		}
		for l405 in 0..2 {
			self.fRec408[(l405) as usize] = 0.0;
		}
		for l406 in 0..16384 {
			self.fVec101[(l406) as usize] = 0.0;
		}
		for l407 in 0..2 {
			self.fVec102[(l407) as usize] = 0.0;
		}
		for l408 in 0..2 {
			self.fRec407[(l408) as usize] = 0.0;
		}
		for l409 in 0..2 {
			self.fRec405[(l409) as usize] = 0.0;
		}
		for l410 in 0..2 {
			self.fRec425[(l410) as usize] = 0.0;
		}
		for l411 in 0..16384 {
			self.fVec103[(l411) as usize] = 0.0;
		}
		for l412 in 0..2 {
			self.fVec104[(l412) as usize] = 0.0;
		}
		for l413 in 0..2 {
			self.fRec424[(l413) as usize] = 0.0;
		}
		for l414 in 0..2 {
			self.fRec406[(l414) as usize] = 0.0;
		}
		for l415 in 0..16384 {
			self.fVec105[(l415) as usize] = 0.0;
		}
		for l416 in 0..2 {
			self.fVec106[(l416) as usize] = 0.0;
		}
		for l417 in 0..2 {
			self.fRec428[(l417) as usize] = 0.0;
		}
		for l418 in 0..2 {
			self.fRec426[(l418) as usize] = 0.0;
		}
		for l419 in 0..16384 {
			self.fVec107[(l419) as usize] = 0.0;
		}
		for l420 in 0..2 {
			self.fVec108[(l420) as usize] = 0.0;
		}
		for l421 in 0..2 {
			self.fRec429[(l421) as usize] = 0.0;
		}
		for l422 in 0..2 {
			self.fRec427[(l422) as usize] = 0.0;
		}
		for l423 in 0..16384 {
			self.fVec109[(l423) as usize] = 0.0;
		}
		for l424 in 0..2 {
			self.fVec110[(l424) as usize] = 0.0;
		}
		for l425 in 0..2 {
			self.fRec432[(l425) as usize] = 0.0;
		}
		for l426 in 0..2 {
			self.fRec430[(l426) as usize] = 0.0;
		}
		for l427 in 0..2 {
			self.fRec434[(l427) as usize] = 0.0;
		}
		for l428 in 0..16384 {
			self.fVec111[(l428) as usize] = 0.0;
		}
		for l429 in 0..2 {
			self.fVec112[(l429) as usize] = 0.0;
		}
		for l430 in 0..2 {
			self.fRec433[(l430) as usize] = 0.0;
		}
		for l431 in 0..2 {
			self.fRec431[(l431) as usize] = 0.0;
		}
		for l432 in 0..2 {
			self.fRec438[(l432) as usize] = 0.0;
		}
		for l433 in 0..16384 {
			self.fVec113[(l433) as usize] = 0.0;
		}
		for l434 in 0..2 {
			self.fVec114[(l434) as usize] = 0.0;
		}
		for l435 in 0..2 {
			self.fRec437[(l435) as usize] = 0.0;
		}
		for l436 in 0..2 {
			self.fRec435[(l436) as usize] = 0.0;
		}
		for l437 in 0..16384 {
			self.fVec115[(l437) as usize] = 0.0;
		}
		for l438 in 0..2 {
			self.fVec116[(l438) as usize] = 0.0;
		}
		for l439 in 0..2 {
			self.fRec439[(l439) as usize] = 0.0;
		}
		for l440 in 0..2 {
			self.fRec436[(l440) as usize] = 0.0;
		}
		for l441 in 0..2 {
			self.fRec321[(l441) as usize] = 0.0;
		}
		for l442 in 0..2 {
			self.fRec322[(l442) as usize] = 0.0;
		}
		for l443 in 0..2 {
			self.fRec440[(l443) as usize] = 0.0;
		}
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		self.fConst0 = F32::min(1.92e+05, F32::max(1.0, ((self.fSampleRate) as F32)));
		self.fConst1 = 44.1 / self.fConst0;
		self.fConst2 = 1.0 - self.fConst1;
		self.fConst3 = 3.1415927 / self.fConst0;
		self.fConst4 = 0.00882353 * self.fConst0;
		self.fConst5 = 0.00073529413 * self.fConst0;
		self.fConst6 = F32::exp(0.0 - 1e+04 / self.fConst0);
		self.fConst7 = 1.0 - self.fConst6;
		self.iConst8 = ((0.1 * self.fConst0) as i32);
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
		let mut fConst27: F32 = F32::tan(25132.742 / self.fConst0);
		let mut fConst28: F32 = mydsp_faustpower2_f(fConst27);
		self.fConst29 = 1.0 / fConst28;
		self.fConst30 = 2.0 * (1.0 - self.fConst29);
		self.fConst31 = 1.0 / fConst27;
		self.fConst32 = (self.fConst31 + -0.618034) / fConst27 + 1.0;
		self.fConst33 = 1.0 / ((self.fConst31 + 0.618034) / fConst27 + 1.0);
		self.fConst34 = (self.fConst31 + -1.618034) / fConst27 + 1.0;
		self.fConst35 = 1.0 / ((self.fConst31 + 1.618034) / fConst27 + 1.0);
		self.fConst36 = 6.2831855 / self.fConst0;
		self.fConst37 = 1.0 - self.fConst31;
		let mut fConst38: F32 = self.fConst31 + 1.0;
		self.fConst39 = 1.0 / fConst38;
		self.fConst40 = 1.0 - fConst22;
		let mut fConst41: F32 = fConst22 + 1.0;
		self.fConst42 = 1.0 / fConst41;
		self.fConst43 = self.fConst40 / fConst41;
		self.fConst44 = 1.0 / (fConst18 * fConst41);
		self.fConst45 = 0.0 - self.fConst44;
		self.fConst46 = 0.0 - 2.0 / fConst19;
		self.fConst47 = (fConst22 + -1.618034) / fConst18 + 1.0;
		self.fConst48 = 1.0 / ((fConst22 + 1.618034) / fConst18 + 1.0);
		self.fConst49 = 0.0 - 1.0 / (fConst27 * fConst38);
		self.fConst50 = 0.0 - 2.0 / fConst28;
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
			36 => Some(self.fHslider35),
			39 => Some(self.fHslider36),
			38 => Some(self.fHslider37),
			34 => Some(self.fHslider38),
			35 => Some(self.fHslider39),
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
			36 => { self.fHslider35 = value }
			39 => { self.fHslider36 = value }
			38 => { self.fHslider37 = value }
			34 => { self.fHslider38 = value }
			35 => { self.fHslider39 = value }
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
		let mut fSlow28: F32 = self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(((16.11811 * (fSlow27 - fSlow26)) as i32), 2047))) as usize] };
		let mut fSlow29: F32 = self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(((16.11811 * (fSlow26 + fSlow27)) as i32), 2047))) as usize] };
		let mut fSlow30: F32 = self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(((16.11811 * fSlow27) as i32), 2047))) as usize] };
		let mut fSlow31: F32 = self.fConst1 * self.fHslider23;
		let mut fSlow32: F32 = self.fHslider24;
		let mut fSlow33: F32 = self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(((16.11811 * (fSlow32 - fSlow26)) as i32), 2047))) as usize] };
		let mut fSlow34: F32 = self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(((16.11811 * (fSlow26 + fSlow32)) as i32), 2047))) as usize] };
		let mut fSlow35: F32 = self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(((16.11811 * fSlow32) as i32), 2047))) as usize] };
		let mut fSlow36: F32 = self.fConst1 * self.fHslider25;
		let mut fSlow37: F32 = self.fHslider26;
		let mut fSlow38: F32 = self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(((16.11811 * (fSlow37 - fSlow26)) as i32), 2047))) as usize] };
		let mut fSlow39: F32 = self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(((16.11811 * (fSlow26 + fSlow37)) as i32), 2047))) as usize] };
		let mut fSlow40: F32 = self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(((16.11811 * fSlow37) as i32), 2047))) as usize] };
		let mut fSlow41: F32 = self.fConst1 * self.fHslider27;
		let mut fSlow42: F32 = self.fHslider28;
		let mut fSlow43: F32 = self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(((16.11811 * (fSlow42 - fSlow26)) as i32), 2047))) as usize] };
		let mut fSlow44: F32 = self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(((16.11811 * (fSlow26 + fSlow42)) as i32), 2047))) as usize] };
		let mut fSlow45: F32 = self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(((16.11811 * fSlow42) as i32), 2047))) as usize] };
		let mut fSlow46: F32 = self.fConst1 * self.fHslider29;
		let mut fSlow47: F32 = self.fConst1 * self.fHslider30;
		let mut fSlow48: F32 = self.fConst1 * self.fHslider31;
		let mut fSlow49: F32 = self.fConst1 * self.fHslider32;
		let mut fSlow50: F32 = self.fHslider33;
		let mut fSlow51: F32 = self.fHslider34;
		let mut fSlow52: F32 = 1.0 - fSlow51;
		let mut fSlow53: F32 = self.fHslider35;
		let mut iSlow54: i32 = unsafe { itbl1mydspSIG1[(((134.0 * fSlow53) as i32)) as usize] };
		let mut fSlow55: F32 = 0.005 * ((iSlow54) as F32);
		let mut iSlow56: i32 = unsafe { itbl1mydspSIG1[(((54.0 * fSlow53) as i32)) as usize] };
		let mut fSlow57: F32 = 0.005 * ((iSlow56) as F32);
		let mut iSlow58: i32 = unsafe { itbl1mydspSIG1[(((1e+01 * fSlow53) as i32)) as usize] };
		let mut fSlow59: F32 = 0.0001 * ((iSlow58) as F32);
		let mut iSlow60: i32 = unsafe { itbl1mydspSIG1[(((1.1e+02 * fSlow53) as i32)) as usize] };
		let mut fSlow61: F32 = 0.0001 * ((iSlow60) as F32);
		let mut iSlow62: i32 = unsafe { itbl1mydspSIG1[(((4e+01 * fSlow53) as i32)) as usize] };
		let mut fSlow63: F32 = 0.0001 * ((iSlow62) as F32);
		let mut iSlow64: i32 = unsafe { itbl1mydspSIG1[(((1.4e+02 * fSlow53) as i32)) as usize] };
		let mut fSlow65: F32 = 0.0001 * ((iSlow64) as F32);
		let mut iSlow66: i32 = unsafe { itbl1mydspSIG1[(((7e+01 * fSlow53) as i32)) as usize] };
		let mut fSlow67: F32 = 0.0001 * ((iSlow66) as F32);
		let mut iSlow68: i32 = unsafe { itbl1mydspSIG1[(((1.7e+02 * fSlow53) as i32)) as usize] };
		let mut fSlow69: F32 = 0.0001 * ((iSlow68) as F32);
		let mut iSlow70: i32 = unsafe { itbl1mydspSIG1[(((1e+02 * fSlow53) as i32)) as usize] };
		let mut fSlow71: F32 = 0.0001 * ((iSlow70) as F32);
		let mut iSlow72: i32 = unsafe { itbl1mydspSIG1[(((2e+02 * fSlow53) as i32)) as usize] };
		let mut fSlow73: F32 = 0.0001 * ((iSlow72) as F32);
		let mut iSlow74: i32 = unsafe { itbl1mydspSIG1[(((1.3e+02 * fSlow53) as i32)) as usize] };
		let mut fSlow75: F32 = 0.0001 * ((iSlow74) as F32);
		let mut iSlow76: i32 = unsafe { itbl1mydspSIG1[(((2.3e+02 * fSlow53) as i32)) as usize] };
		let mut fSlow77: F32 = 0.0001 * ((iSlow76) as F32);
		let mut fSlow78: F32 = self.fConst36 * self.fHslider36;
		let mut fSlow79: F32 = F32::cos(fSlow78);
		let mut fSlow80: F32 = F32::sin(fSlow78);
		let mut fSlow81: F32 = 5e+01 * self.fHslider37;
		let mut iSlow82: i32 = unsafe { itbl1mydspSIG1[(((125.0 * fSlow53) as i32)) as usize] };
		let mut fSlow83: F32 = 0.0001 * ((iSlow82) as F32);
		let mut iSlow84: i32 = unsafe { itbl1mydspSIG1[(((204.0 * fSlow53) as i32)) as usize] };
		let mut fSlow85: F32 = 0.005 * ((iSlow84) as F32);
		let mut fSlow86: F32 = 0.0 - fSlow81;
		let mut iSlow87: i32 = unsafe { itbl1mydspSIG1[(((25.0 * fSlow53) as i32)) as usize] };
		let mut fSlow88: F32 = 0.0001 * ((iSlow87) as F32);
		let mut iSlow89: i32 = unsafe { itbl1mydspSIG1[(((155.0 * fSlow53) as i32)) as usize] };
		let mut fSlow90: F32 = 0.0001 * ((iSlow89) as F32);
		let mut iSlow91: i32 = unsafe { itbl1mydspSIG1[(((55.0 * fSlow53) as i32)) as usize] };
		let mut fSlow92: F32 = 0.0001 * ((iSlow91) as F32);
		let mut iSlow93: i32 = unsafe { itbl1mydspSIG1[(((185.0 * fSlow53) as i32)) as usize] };
		let mut fSlow94: F32 = 0.0001 * ((iSlow93) as F32);
		let mut iSlow95: i32 = unsafe { itbl1mydspSIG1[(((85.0 * fSlow53) as i32)) as usize] };
		let mut fSlow96: F32 = 0.0001 * ((iSlow95) as F32);
		let mut iSlow97: i32 = unsafe { itbl1mydspSIG1[(((215.0 * fSlow53) as i32)) as usize] };
		let mut fSlow98: F32 = 0.0001 * ((iSlow97) as F32);
		let mut iSlow99: i32 = unsafe { itbl1mydspSIG1[(((115.0 * fSlow53) as i32)) as usize] };
		let mut fSlow100: F32 = 0.0001 * ((iSlow99) as F32);
		let mut iSlow101: i32 = unsafe { itbl1mydspSIG1[(((245.0 * fSlow53) as i32)) as usize] };
		let mut fSlow102: F32 = 0.0001 * ((iSlow101) as F32);
		let mut iSlow103: i32 = unsafe { itbl1mydspSIG1[(((145.0 * fSlow53) as i32)) as usize] };
		let mut fSlow104: F32 = 0.0001 * ((iSlow103) as F32);
		let mut fSlow105: F32 = F32::powf(1e+01, 0.0 - 0.51 * ((1.25 * fSlow53 + -0.25) / self.fHslider38));
		let mut fSlow106: F32 = self.fHslider39;
		let mut fSlow107: F32 = 1.0 - fSlow106;
		let mut fSlow108: F32 = self.fHslider40;
		let mut fSlow109: F32 = F32::sin(fSlow108);
		let mut iSlow110: i32 = unsafe { itbl1mydspSIG1[(((34.0 * fSlow53) as i32)) as usize] };
		let mut fSlow111: F32 = 0.005 * ((iSlow110) as F32);
		let mut fSlow112: F32 = F32::cos(fSlow108);
		let mut iSlow113: i32 = unsafe { itbl1mydspSIG1[(((2.4e+02 * fSlow53) as i32)) as usize] };
		let mut fSlow114: F32 = 0.0001 * ((iSlow113) as F32);
		let mut iSlow115: i32 = unsafe { itbl1mydspSIG1[(((1.9e+02 * fSlow53) as i32)) as usize] };
		let mut fSlow116: F32 = 0.0001 * ((iSlow115) as F32);
		let mut iSlow117: i32 = unsafe { itbl1mydspSIG1[(((175.0 * fSlow53) as i32)) as usize] };
		let mut fSlow118: F32 = 0.0001 * ((iSlow117) as F32);
		let mut fSlow119: F32 = self.fConst1 * self.fHslider41;
		let zipped_iterators = outputs0.zip(outputs1);
		for (output0, output1) in zipped_iterators {
			self.iVec0[0] = 1;
			self.fRec0[0] = fSlow0 + self.fConst2 * self.fRec0[1];
			let mut fTemp0: F32 = F32::min(1.4141995, 1.4142135 * self.fRec0[0]);
			let mut fTemp1: F32 = 1.4142135 * fTemp0;
			let mut fTemp2: F32 = 1.0 - fTemp1;
			self.fRec3[0] = fSlow1 + self.fConst2 * self.fRec3[1];
			self.fRec1[0] = self.fConst2 * self.fRec1[1] + self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(((16.11811 * (fSlow2 + self.fRec3[0])) as i32), 2047))) as usize] };
			let mut fTemp3: F32 = F32::tan(self.fConst3 * F32::max(2e+01, F32::min(1e+04, self.fRec1[0])));
			let mut fTemp4: F32 = 1.0 / fTemp3;
			let mut fTemp5: F32 = 2.0 - fTemp1;
			let mut fTemp6: F32 = mydsp_faustpower2_f(fTemp0);
			let mut fTemp7: F32 = fTemp6 + (fTemp5 + fTemp4) / fTemp3 + fTemp2;
			let mut fTemp8: F32 = 1.0 / mydsp_faustpower2_f(fTemp3);
			let mut fTemp9: F32 = fTemp1 + 2.0;
			let mut fTemp10: F32 = fTemp1 + fTemp6;
			let mut fTemp11: F32 = fTemp10 + (fTemp9 + fTemp4) / fTemp3 + 1.0;
			let mut fRec20: F32 = -1.0 * 0.9973053 * (0.9 * self.fRec21[2] + 0.05 * (self.fRec21[1] + self.fRec21[3]));
			self.fRec40[0] = fSlow3 + self.fConst2 * self.fRec40[1];
			self.fRec39[0] = self.fConst2 * self.fRec39[1] + self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(((16.11811 * (fSlow2 + self.fRec40[0])) as i32), 2047))) as usize] };
			let mut fTemp12: F32 = self.fConst5 * (3.4e+02 / self.fRec39[0] + -0.11);
			let mut fTemp13: F32 = fTemp12 + -1.499995;
			let mut iTemp14: i32 = ((fTemp13) as i32);
			let mut iTemp15: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp14, 4))) as F32))) as i32);
			let mut iTemp16: i32 = i32::wrapping_add(iTemp15, 1);
			let mut fTemp17: F32 = F32::floor(fTemp13);
			let mut fTemp18: F32 = fTemp12 + (-3.0 - fTemp17);
			let mut fTemp19: F32 = fTemp12 + (-2.0 - fTemp17);
			let mut fTemp20: F32 = fTemp12 + (-1.0 - fTemp17);
			let mut fTemp21: F32 = fTemp20 * fTemp19;
			let mut fTemp22: F32 = fTemp21 * fTemp18;
			let mut fTemp23: F32 = fTemp12 + (-4.0 - fTemp17);
			let mut fTemp24: F32 = 0.0 - fTemp23;
			let mut iTemp25: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp14, 3))) as F32))) as i32);
			let mut iTemp26: i32 = i32::wrapping_add(iTemp25, 1);
			let mut fTemp27: F32 = 0.0 - 0.5 * fTemp23;
			let mut fTemp28: F32 = 0.0 - fTemp18;
			let mut iTemp29: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp14, 2))) as F32))) as i32);
			let mut iTemp30: i32 = i32::wrapping_add(iTemp29, 1);
			let mut fTemp31: F32 = 0.0 - 0.33333334 * fTemp23;
			let mut fTemp32: F32 = 0.0 - 0.5 * fTemp18;
			let mut fTemp33: F32 = 0.0 - fTemp19;
			let mut iTemp34: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp14, 1))) as F32))) as i32);
			let mut iTemp35: i32 = i32::wrapping_add(iTemp34, 1);
			let mut fTemp36: F32 = fTemp12 - fTemp17;
			let mut fTemp37: F32 = 0.0 - 0.25 * fTemp23;
			let mut fTemp38: F32 = 0.0 - 0.33333334 * fTemp18;
			let mut fTemp39: F32 = 0.0 - 0.5 * fTemp19;
			let mut fTemp40: F32 = 0.0 - fTemp20;
			let mut iTemp41: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, iTemp14)) as F32))) as i32);
			let mut iTemp42: i32 = i32::wrapping_add(iTemp41, 1);
			self.fRec35[0] = self.fRec12[((i32::wrapping_sub(self.IOTA0, iTemp42)) & 2047) as usize] * fTemp40 * fTemp39 * fTemp38 * fTemp37 + fTemp36 * (self.fRec12[((i32::wrapping_sub(self.IOTA0, iTemp35)) & 2047) as usize] * fTemp33 * fTemp32 * fTemp31 + 0.5 * fTemp20 * self.fRec12[((i32::wrapping_sub(self.IOTA0, iTemp30)) & 2047) as usize] * fTemp28 * fTemp27 + 0.16666667 * fTemp21 * self.fRec12[((i32::wrapping_sub(self.IOTA0, iTemp26)) & 2047) as usize] * fTemp24 + 0.041666668 * fTemp22 * self.fRec12[((i32::wrapping_sub(self.IOTA0, iTemp16)) & 2047) as usize]);
			self.fRec41[0] = 0.05 * self.fRec41[1] + 0.95 * self.fRec35[1];
			let mut fRec36: F32 = self.fRec41[0];
			self.fRec46[0] = self.fConst6 * self.fRec46[1] + self.fConst7 * F32::abs(self.fRec6[1]);
			let mut fRec45: F32 = self.fRec46[0];
			let mut iTemp43: i32 = ((fRec45 > 0.1) as i32);
			self.iVec1[0] = iTemp43;
			self.iRec47[0] = std::cmp::max(i32::wrapping_mul(self.iConst8, ((iTemp43 < self.iVec1[1]) as i32)), i32::wrapping_add(self.iRec47[1], -1));
			let mut fTemp44: F32 = F32::abs(F32::max(((iTemp43) as F32), ((((self.iRec47[0] > 0) as i32)) as F32)));
			let mut fTemp45: F32 = if (((self.fRec43[1] > fTemp44) as i32) as i32 != 0) { self.fConst9 } else { self.fConst6 };
			self.fRec44[0] = self.fRec44[1] * fTemp45 + fTemp44 * (1.0 - fTemp45);
			self.fRec43[0] = self.fRec44[0];
			let mut fTemp46: F32 = 0.005 * self.fRec43[0] * self.fRec6[1];
			self.fRec48[0] = self.fRec10[1];
			self.fRec49[(self.IOTA0 & 2047) as usize] = -1.0 * 0.9973053 * (0.9 * self.fRec48[2] + 0.05 * (self.fRec48[1] + self.fRec48[3]));
			let mut fTemp47: F32 = fTemp21 * fTemp24;
			let mut fTemp48: F32 = fTemp20 * fTemp28 * fTemp27;
			let mut fTemp49: F32 = fTemp33 * fTemp32 * fTemp31;
			let mut fTemp50: F32 = fTemp40 * fTemp39 * fTemp38 * fTemp37;
			self.fVec2[0] = fTemp50 * self.fRec49[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp41, 2))) & 2047) as usize] + fTemp36 * (fTemp49 * self.fRec49[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp34, 2))) & 2047) as usize] + 0.5 * fTemp48 * self.fRec49[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp29, 2))) & 2047) as usize] + 0.16666667 * fTemp47 * self.fRec49[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp25, 2))) & 2047) as usize] + 0.041666668 * fTemp22 * self.fRec49[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp15, 2))) & 2047) as usize]);
			let mut fTemp51: F32 = F32::tan(self.fConst10 * self.fRec39[0]);
			let mut fTemp52: F32 = 1.0 / fTemp51;
			let mut fTemp53: F32 = (fTemp52 + 1.4142135) / fTemp51 + 1.0;
			self.fVec3[0] = fSlow4;
			self.iRec50[0] = i32::wrapping_add(i32::wrapping_mul(i32::wrapping_add(self.iRec50[1], ((self.iRec50[1] > 0) as i32)), ((fSlow4 <= self.fVec3[1]) as i32)), ((fSlow4 > self.fVec3[1]) as i32));
			let mut fTemp54: F32 = ((self.iRec50[0]) as F32) / F32::max(1.0, self.fConst11 * mydsp_faustpower2_f(1.0 - 0.0005 * self.fRec39[0]));
			self.iRec52[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, self.iRec52[1]), 12345);
			let mut fTemp55: F32 = 4.656613e-10 * ((self.iRec52[0]) as F32);
			self.fRec51[0] = fTemp55 - (self.fRec51[2] * ((fTemp52 + -1.4142135) / fTemp51 + 1.0) + 2.0 * self.fRec51[1] * (1.0 - 1.0 / mydsp_faustpower2_f(fTemp51))) / fTemp53;
			let mut fTemp56: F32 = 0.5 * ((self.fRec51[2] + self.fRec51[0] + 2.0 * self.fRec51[1]) * F32::max(0.0, F32::min(fTemp54, 2.0 - fTemp54)) / fTemp53);
			let mut fTemp57: F32 = fTemp56 + self.fVec2[1] + fTemp46;
			self.fVec4[0] = fTemp57;
			self.fRec42[(self.IOTA0 & 2047) as usize] = 0.05 * self.fRec42[((i32::wrapping_sub(self.IOTA0, 1)) & 2047) as usize] + 0.95 * self.fVec4[2];
			let mut fRec37: F32 = fTemp50 * self.fRec42[((i32::wrapping_sub(self.IOTA0, iTemp41)) & 2047) as usize] + fTemp36 * (fTemp49 * self.fRec42[((i32::wrapping_sub(self.IOTA0, iTemp34)) & 2047) as usize] + 0.5 * fTemp48 * self.fRec42[((i32::wrapping_sub(self.IOTA0, iTemp29)) & 2047) as usize] + 0.16666667 * fTemp47 * self.fRec42[((i32::wrapping_sub(self.IOTA0, iTemp25)) & 2047) as usize] + 0.041666668 * fTemp22 * self.fRec42[((i32::wrapping_sub(self.IOTA0, iTemp15)) & 2047) as usize]);
			let mut fRec38: F32 = self.fVec4[1] + self.fRec31[1];
			self.fRec31[0] = fRec36;
			let mut fRec32: F32 = self.fRec31[1];
			let mut fRec33: F32 = fRec37;
			let mut fRec34: F32 = fRec38;
			self.fRec27[0] = fRec32;
			let mut fRec28: F32 = fTemp46 + fTemp56 + self.fRec27[1];
			let mut fRec29: F32 = fRec33;
			let mut fRec30: F32 = fRec34;
			self.fRec23[(self.IOTA0 & 2047) as usize] = fRec28;
			let mut fRec24: F32 = fTemp50 * self.fRec23[((i32::wrapping_sub(self.IOTA0, iTemp42)) & 2047) as usize] + fTemp36 * (fTemp49 * self.fRec23[((i32::wrapping_sub(self.IOTA0, iTemp35)) & 2047) as usize] + 0.5 * fTemp48 * self.fRec23[((i32::wrapping_sub(self.IOTA0, iTemp30)) & 2047) as usize] + 0.16666667 * fTemp47 * self.fRec23[((i32::wrapping_sub(self.IOTA0, iTemp26)) & 2047) as usize] + 0.041666668 * fTemp22 * self.fRec23[((i32::wrapping_sub(self.IOTA0, iTemp16)) & 2047) as usize]);
			self.fRec25[0] = fRec29;
			let mut fRec26: F32 = fRec30;
			self.fRec21[0] = fSlow5 * self.fRec25[1];
			let mut fRec22: F32 = fRec26;
			self.fRec16[0] = fRec20;
			let mut fRec17: F32 = fSlow5 * self.fRec16[1];
			let mut fRec18: F32 = self.fRec21[0];
			let mut fRec19: F32 = fRec22;
			self.fRec12[(self.IOTA0 & 2047) as usize] = fRec17;
			let mut fRec13: F32 = fRec24;
			let mut fRec14: F32 = fRec18;
			let mut fRec15: F32 = fRec19;
			self.fRec10[0] = fRec13;
			let mut fRec11: F32 = fRec15;
			let mut fTemp58: F32 = F32::abs(fRec11);
			let mut fTemp59: F32 = if (((self.fRec8[1] > fTemp58) as i32) as i32 != 0) { self.fConst12 } else { 0.0 };
			self.fRec9[0] = self.fRec9[1] * fTemp59 + fTemp58 * (1.0 - fTemp59);
			self.fRec8[0] = self.fRec9[0];
			let mut fRec7: F32 = 0.0 - 0.95 * F32::max(2e+01 * F32::log10(self.fRec8[0]) + 1e+01, 0.0);
			self.fRec6[0] = 1.5 * fRec11 * F32::powf(1e+01, 0.05 * fRec7);
			self.fRec5[0] = self.fRec6[0] - (self.fRec5[2] * (fTemp10 + (1.0 - (fTemp9 - fTemp4) / fTemp3)) + 2.0 * self.fRec5[1] * (fTemp10 + (1.0 - fTemp8))) / fTemp11;
			self.fRec4[0] = (self.fRec5[2] + self.fRec5[0] + 2.0 * self.fRec5[1]) / fTemp11 - (self.fRec4[2] * (fTemp6 + (fTemp4 - fTemp5) / fTemp3 + fTemp2) + 2.0 * self.fRec4[1] * (fTemp6 + (1.0 - (fTemp1 + fTemp8)))) / fTemp7;
			self.fRec53[0] = self.fConst2 * self.fRec53[1] + self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(((16.11811 * (fSlow6 + self.fRec3[0])) as i32), 2047))) as usize] };
			let mut fTemp60: F32 = F32::tan(self.fConst3 * F32::max(2e+01, F32::min(1e+04, self.fRec53[0])));
			let mut fTemp61: F32 = 1.0 / fTemp60;
			let mut fTemp62: F32 = fTemp6 + (fTemp5 + fTemp61) / fTemp60 + fTemp2;
			let mut fTemp63: F32 = 1.0 / mydsp_faustpower2_f(fTemp60);
			let mut fTemp64: F32 = fTemp10 + (fTemp9 + fTemp61) / fTemp60 + 1.0;
			let mut fRec70: F32 = -1.0 * 0.9973053 * (0.9 * self.fRec71[2] + 0.05 * (self.fRec71[1] + self.fRec71[3]));
			self.fRec89[0] = self.fConst2 * self.fRec89[1] + self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(((16.11811 * (fSlow6 + self.fRec40[0])) as i32), 2047))) as usize] };
			let mut fTemp65: F32 = self.fConst5 * (3.4e+02 / self.fRec89[0] + -0.11);
			let mut fTemp66: F32 = fTemp65 + -1.499995;
			let mut iTemp67: i32 = ((fTemp66) as i32);
			let mut iTemp68: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp67, 4))) as F32))) as i32);
			let mut iTemp69: i32 = i32::wrapping_add(iTemp68, 1);
			let mut fTemp70: F32 = F32::floor(fTemp66);
			let mut fTemp71: F32 = fTemp65 + (-3.0 - fTemp70);
			let mut fTemp72: F32 = fTemp65 + (-2.0 - fTemp70);
			let mut fTemp73: F32 = fTemp65 + (-1.0 - fTemp70);
			let mut fTemp74: F32 = fTemp73 * fTemp72;
			let mut fTemp75: F32 = fTemp74 * fTemp71;
			let mut fTemp76: F32 = fTemp65 + (-4.0 - fTemp70);
			let mut fTemp77: F32 = 0.0 - fTemp76;
			let mut iTemp78: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp67, 3))) as F32))) as i32);
			let mut iTemp79: i32 = i32::wrapping_add(iTemp78, 1);
			let mut fTemp80: F32 = 0.0 - 0.5 * fTemp76;
			let mut fTemp81: F32 = 0.0 - fTemp71;
			let mut iTemp82: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp67, 2))) as F32))) as i32);
			let mut iTemp83: i32 = i32::wrapping_add(iTemp82, 1);
			let mut fTemp84: F32 = 0.0 - 0.33333334 * fTemp76;
			let mut fTemp85: F32 = 0.0 - 0.5 * fTemp71;
			let mut fTemp86: F32 = 0.0 - fTemp72;
			let mut iTemp87: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp67, 1))) as F32))) as i32);
			let mut iTemp88: i32 = i32::wrapping_add(iTemp87, 1);
			let mut fTemp89: F32 = fTemp65 - fTemp70;
			let mut fTemp90: F32 = 0.0 - 0.25 * fTemp76;
			let mut fTemp91: F32 = 0.0 - 0.33333334 * fTemp71;
			let mut fTemp92: F32 = 0.0 - 0.5 * fTemp72;
			let mut fTemp93: F32 = 0.0 - fTemp73;
			let mut iTemp94: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, iTemp67)) as F32))) as i32);
			let mut iTemp95: i32 = i32::wrapping_add(iTemp94, 1);
			self.fRec85[0] = self.fRec62[((i32::wrapping_sub(self.IOTA0, iTemp95)) & 2047) as usize] * fTemp93 * fTemp92 * fTemp91 * fTemp90 + fTemp89 * (self.fRec62[((i32::wrapping_sub(self.IOTA0, iTemp88)) & 2047) as usize] * fTemp86 * fTemp85 * fTemp84 + 0.5 * fTemp73 * self.fRec62[((i32::wrapping_sub(self.IOTA0, iTemp83)) & 2047) as usize] * fTemp81 * fTemp80 + 0.16666667 * fTemp74 * self.fRec62[((i32::wrapping_sub(self.IOTA0, iTemp79)) & 2047) as usize] * fTemp77 + 0.041666668 * fTemp75 * self.fRec62[((i32::wrapping_sub(self.IOTA0, iTemp69)) & 2047) as usize]);
			self.fRec90[0] = 0.05 * self.fRec90[1] + 0.95 * self.fRec85[1];
			let mut fRec86: F32 = self.fRec90[0];
			self.fRec95[0] = self.fConst6 * self.fRec95[1] + self.fConst7 * F32::abs(self.fRec56[1]);
			let mut fRec94: F32 = self.fRec95[0];
			let mut iTemp96: i32 = ((fRec94 > 0.1) as i32);
			self.iVec5[0] = iTemp96;
			self.iRec96[0] = std::cmp::max(i32::wrapping_mul(self.iConst8, ((iTemp96 < self.iVec5[1]) as i32)), i32::wrapping_add(self.iRec96[1], -1));
			let mut fTemp97: F32 = F32::abs(F32::max(((iTemp96) as F32), ((((self.iRec96[0] > 0) as i32)) as F32)));
			let mut fTemp98: F32 = if (((self.fRec92[1] > fTemp97) as i32) as i32 != 0) { self.fConst9 } else { self.fConst6 };
			self.fRec93[0] = self.fRec93[1] * fTemp98 + fTemp97 * (1.0 - fTemp98);
			self.fRec92[0] = self.fRec93[0];
			let mut fTemp99: F32 = 0.005 * self.fRec92[0] * self.fRec56[1];
			self.fRec97[0] = self.fRec60[1];
			self.fRec98[(self.IOTA0 & 2047) as usize] = -1.0 * 0.9973053 * (0.9 * self.fRec97[2] + 0.05 * (self.fRec97[1] + self.fRec97[3]));
			let mut fTemp100: F32 = fTemp74 * fTemp77;
			let mut fTemp101: F32 = fTemp73 * fTemp81 * fTemp80;
			let mut fTemp102: F32 = fTemp86 * fTemp85 * fTemp84;
			let mut fTemp103: F32 = fTemp93 * fTemp92 * fTemp91 * fTemp90;
			self.fVec6[0] = fTemp103 * self.fRec98[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp94, 2))) & 2047) as usize] + fTemp89 * (fTemp102 * self.fRec98[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp87, 2))) & 2047) as usize] + 0.5 * fTemp101 * self.fRec98[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp82, 2))) & 2047) as usize] + 0.16666667 * fTemp100 * self.fRec98[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp78, 2))) & 2047) as usize] + 0.041666668 * fTemp75 * self.fRec98[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp68, 2))) & 2047) as usize]);
			let mut fTemp104: F32 = F32::tan(self.fConst10 * self.fRec89[0]);
			let mut fTemp105: F32 = 1.0 / fTemp104;
			let mut fTemp106: F32 = (fTemp105 + 1.4142135) / fTemp104 + 1.0;
			self.fVec7[0] = fSlow7;
			self.iRec99[0] = i32::wrapping_add(i32::wrapping_mul(i32::wrapping_add(self.iRec99[1], ((self.iRec99[1] > 0) as i32)), ((fSlow7 <= self.fVec7[1]) as i32)), ((fSlow7 > self.fVec7[1]) as i32));
			let mut fTemp107: F32 = ((self.iRec99[0]) as F32) / F32::max(1.0, self.fConst11 * mydsp_faustpower2_f(1.0 - 0.0005 * self.fRec89[0]));
			self.fRec100[0] = fTemp55 - (self.fRec100[2] * ((fTemp105 + -1.4142135) / fTemp104 + 1.0) + 2.0 * self.fRec100[1] * (1.0 - 1.0 / mydsp_faustpower2_f(fTemp104))) / fTemp106;
			let mut fTemp108: F32 = 0.5 * ((self.fRec100[2] + self.fRec100[0] + 2.0 * self.fRec100[1]) * F32::max(0.0, F32::min(fTemp107, 2.0 - fTemp107)) / fTemp106);
			let mut fTemp109: F32 = fTemp108 + self.fVec6[1] + fTemp99;
			self.fVec8[0] = fTemp109;
			self.fRec91[(self.IOTA0 & 2047) as usize] = 0.05 * self.fRec91[((i32::wrapping_sub(self.IOTA0, 1)) & 2047) as usize] + 0.95 * self.fVec8[2];
			let mut fRec87: F32 = fTemp103 * self.fRec91[((i32::wrapping_sub(self.IOTA0, iTemp94)) & 2047) as usize] + fTemp89 * (fTemp102 * self.fRec91[((i32::wrapping_sub(self.IOTA0, iTemp87)) & 2047) as usize] + 0.5 * fTemp101 * self.fRec91[((i32::wrapping_sub(self.IOTA0, iTemp82)) & 2047) as usize] + 0.16666667 * fTemp100 * self.fRec91[((i32::wrapping_sub(self.IOTA0, iTemp78)) & 2047) as usize] + 0.041666668 * fTemp75 * self.fRec91[((i32::wrapping_sub(self.IOTA0, iTemp68)) & 2047) as usize]);
			let mut fRec88: F32 = self.fVec8[1] + self.fRec81[1];
			self.fRec81[0] = fRec86;
			let mut fRec82: F32 = self.fRec81[1];
			let mut fRec83: F32 = fRec87;
			let mut fRec84: F32 = fRec88;
			self.fRec77[0] = fRec82;
			let mut fRec78: F32 = fTemp99 + fTemp108 + self.fRec77[1];
			let mut fRec79: F32 = fRec83;
			let mut fRec80: F32 = fRec84;
			self.fRec73[(self.IOTA0 & 2047) as usize] = fRec78;
			let mut fRec74: F32 = fTemp103 * self.fRec73[((i32::wrapping_sub(self.IOTA0, iTemp95)) & 2047) as usize] + fTemp89 * (fTemp102 * self.fRec73[((i32::wrapping_sub(self.IOTA0, iTemp88)) & 2047) as usize] + 0.5 * fTemp101 * self.fRec73[((i32::wrapping_sub(self.IOTA0, iTemp83)) & 2047) as usize] + 0.16666667 * fTemp100 * self.fRec73[((i32::wrapping_sub(self.IOTA0, iTemp79)) & 2047) as usize] + 0.041666668 * fTemp75 * self.fRec73[((i32::wrapping_sub(self.IOTA0, iTemp69)) & 2047) as usize]);
			self.fRec75[0] = fRec79;
			let mut fRec76: F32 = fRec80;
			self.fRec71[0] = fSlow5 * self.fRec75[1];
			let mut fRec72: F32 = fRec76;
			self.fRec66[0] = fRec70;
			let mut fRec67: F32 = fSlow5 * self.fRec66[1];
			let mut fRec68: F32 = self.fRec71[0];
			let mut fRec69: F32 = fRec72;
			self.fRec62[(self.IOTA0 & 2047) as usize] = fRec67;
			let mut fRec63: F32 = fRec74;
			let mut fRec64: F32 = fRec68;
			let mut fRec65: F32 = fRec69;
			self.fRec60[0] = fRec63;
			let mut fRec61: F32 = fRec65;
			let mut fTemp110: F32 = F32::abs(fRec61);
			let mut fTemp111: F32 = if (((self.fRec58[1] > fTemp110) as i32) as i32 != 0) { self.fConst12 } else { 0.0 };
			self.fRec59[0] = self.fRec59[1] * fTemp111 + fTemp110 * (1.0 - fTemp111);
			self.fRec58[0] = self.fRec59[0];
			let mut fRec57: F32 = 0.0 - 0.95 * F32::max(2e+01 * F32::log10(self.fRec58[0]) + 1e+01, 0.0);
			self.fRec56[0] = 1.5 * fRec61 * F32::powf(1e+01, 0.05 * fRec57);
			self.fRec55[0] = self.fRec56[0] - (self.fRec55[2] * (fTemp10 + (1.0 - (fTemp9 - fTemp61) / fTemp60)) + 2.0 * self.fRec55[1] * (fTemp10 + (1.0 - fTemp63))) / fTemp64;
			self.fRec54[0] = (self.fRec55[2] + self.fRec55[0] + 2.0 * self.fRec55[1]) / fTemp64 - (self.fRec54[2] * (fTemp6 + (fTemp61 - fTemp5) / fTemp60 + fTemp2) + 2.0 * self.fRec54[1] * (fTemp6 + (1.0 - (fTemp1 + fTemp63)))) / fTemp62;
			self.fRec101[0] = self.fConst2 * self.fRec101[1] + self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(((16.11811 * (fSlow8 + self.fRec3[0])) as i32), 2047))) as usize] };
			let mut fTemp112: F32 = F32::tan(self.fConst3 * F32::max(2e+01, F32::min(1e+04, self.fRec101[0])));
			let mut fTemp113: F32 = 1.0 / fTemp112;
			let mut fTemp114: F32 = fTemp6 + (fTemp5 + fTemp113) / fTemp112 + fTemp2;
			let mut fTemp115: F32 = 1.0 / mydsp_faustpower2_f(fTemp112);
			let mut fTemp116: F32 = fTemp10 + (fTemp9 + fTemp113) / fTemp112 + 1.0;
			let mut fRec118: F32 = -1.0 * 0.9973053 * (0.9 * self.fRec119[2] + 0.05 * (self.fRec119[1] + self.fRec119[3]));
			self.fRec137[0] = self.fConst2 * self.fRec137[1] + self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(((16.11811 * (fSlow8 + self.fRec40[0])) as i32), 2047))) as usize] };
			let mut fTemp117: F32 = self.fConst5 * (3.4e+02 / self.fRec137[0] + -0.11);
			let mut fTemp118: F32 = fTemp117 + -1.499995;
			let mut iTemp119: i32 = ((fTemp118) as i32);
			let mut iTemp120: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp119, 4))) as F32))) as i32);
			let mut iTemp121: i32 = i32::wrapping_add(iTemp120, 1);
			let mut fTemp122: F32 = F32::floor(fTemp118);
			let mut fTemp123: F32 = fTemp117 + (-3.0 - fTemp122);
			let mut fTemp124: F32 = fTemp117 + (-2.0 - fTemp122);
			let mut fTemp125: F32 = fTemp117 + (-1.0 - fTemp122);
			let mut fTemp126: F32 = fTemp125 * fTemp124;
			let mut fTemp127: F32 = fTemp126 * fTemp123;
			let mut fTemp128: F32 = fTemp117 + (-4.0 - fTemp122);
			let mut fTemp129: F32 = 0.0 - fTemp128;
			let mut iTemp130: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp119, 3))) as F32))) as i32);
			let mut iTemp131: i32 = i32::wrapping_add(iTemp130, 1);
			let mut fTemp132: F32 = 0.0 - 0.5 * fTemp128;
			let mut fTemp133: F32 = 0.0 - fTemp123;
			let mut iTemp134: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp119, 2))) as F32))) as i32);
			let mut iTemp135: i32 = i32::wrapping_add(iTemp134, 1);
			let mut fTemp136: F32 = 0.0 - 0.33333334 * fTemp128;
			let mut fTemp137: F32 = 0.0 - 0.5 * fTemp123;
			let mut fTemp138: F32 = 0.0 - fTemp124;
			let mut iTemp139: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp119, 1))) as F32))) as i32);
			let mut iTemp140: i32 = i32::wrapping_add(iTemp139, 1);
			let mut fTemp141: F32 = fTemp117 - fTemp122;
			let mut fTemp142: F32 = 0.0 - 0.25 * fTemp128;
			let mut fTemp143: F32 = 0.0 - 0.33333334 * fTemp123;
			let mut fTemp144: F32 = 0.0 - 0.5 * fTemp124;
			let mut fTemp145: F32 = 0.0 - fTemp125;
			let mut iTemp146: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, iTemp119)) as F32))) as i32);
			let mut iTemp147: i32 = i32::wrapping_add(iTemp146, 1);
			self.fRec133[0] = self.fRec110[((i32::wrapping_sub(self.IOTA0, iTemp147)) & 2047) as usize] * fTemp145 * fTemp144 * fTemp143 * fTemp142 + fTemp141 * (self.fRec110[((i32::wrapping_sub(self.IOTA0, iTemp140)) & 2047) as usize] * fTemp138 * fTemp137 * fTemp136 + 0.5 * fTemp125 * self.fRec110[((i32::wrapping_sub(self.IOTA0, iTemp135)) & 2047) as usize] * fTemp133 * fTemp132 + 0.16666667 * fTemp126 * self.fRec110[((i32::wrapping_sub(self.IOTA0, iTemp131)) & 2047) as usize] * fTemp129 + 0.041666668 * fTemp127 * self.fRec110[((i32::wrapping_sub(self.IOTA0, iTemp121)) & 2047) as usize]);
			self.fRec138[0] = 0.05 * self.fRec138[1] + 0.95 * self.fRec133[1];
			let mut fRec134: F32 = self.fRec138[0];
			self.fRec143[0] = self.fConst6 * self.fRec143[1] + self.fConst7 * F32::abs(self.fRec104[1]);
			let mut fRec142: F32 = self.fRec143[0];
			let mut iTemp148: i32 = ((fRec142 > 0.1) as i32);
			self.iVec9[0] = iTemp148;
			self.iRec144[0] = std::cmp::max(i32::wrapping_mul(self.iConst8, ((iTemp148 < self.iVec9[1]) as i32)), i32::wrapping_add(self.iRec144[1], -1));
			let mut fTemp149: F32 = F32::abs(F32::max(((iTemp148) as F32), ((((self.iRec144[0] > 0) as i32)) as F32)));
			let mut fTemp150: F32 = if (((self.fRec140[1] > fTemp149) as i32) as i32 != 0) { self.fConst9 } else { self.fConst6 };
			self.fRec141[0] = self.fRec141[1] * fTemp150 + fTemp149 * (1.0 - fTemp150);
			self.fRec140[0] = self.fRec141[0];
			let mut fTemp151: F32 = 0.005 * self.fRec140[0] * self.fRec104[1];
			self.fRec145[0] = self.fRec108[1];
			self.fRec146[(self.IOTA0 & 2047) as usize] = -1.0 * 0.9973053 * (0.9 * self.fRec145[2] + 0.05 * (self.fRec145[1] + self.fRec145[3]));
			let mut fTemp152: F32 = fTemp126 * fTemp129;
			let mut fTemp153: F32 = fTemp125 * fTemp133 * fTemp132;
			let mut fTemp154: F32 = fTemp138 * fTemp137 * fTemp136;
			let mut fTemp155: F32 = fTemp145 * fTemp144 * fTemp143 * fTemp142;
			self.fVec10[0] = fTemp155 * self.fRec146[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp146, 2))) & 2047) as usize] + fTemp141 * (fTemp154 * self.fRec146[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp139, 2))) & 2047) as usize] + 0.5 * fTemp153 * self.fRec146[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp134, 2))) & 2047) as usize] + 0.16666667 * fTemp152 * self.fRec146[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp130, 2))) & 2047) as usize] + 0.041666668 * fTemp127 * self.fRec146[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp120, 2))) & 2047) as usize]);
			let mut fTemp156: F32 = F32::tan(self.fConst10 * self.fRec137[0]);
			let mut fTemp157: F32 = 1.0 / fTemp156;
			let mut fTemp158: F32 = (fTemp157 + 1.4142135) / fTemp156 + 1.0;
			self.fVec11[0] = fSlow9;
			self.iRec147[0] = i32::wrapping_add(i32::wrapping_mul(i32::wrapping_add(self.iRec147[1], ((self.iRec147[1] > 0) as i32)), ((fSlow9 <= self.fVec11[1]) as i32)), ((fSlow9 > self.fVec11[1]) as i32));
			let mut fTemp159: F32 = ((self.iRec147[0]) as F32) / F32::max(1.0, self.fConst11 * mydsp_faustpower2_f(1.0 - 0.0005 * self.fRec137[0]));
			self.fRec148[0] = fTemp55 - (self.fRec148[2] * ((fTemp157 + -1.4142135) / fTemp156 + 1.0) + 2.0 * self.fRec148[1] * (1.0 - 1.0 / mydsp_faustpower2_f(fTemp156))) / fTemp158;
			let mut fTemp160: F32 = 0.5 * ((self.fRec148[2] + self.fRec148[0] + 2.0 * self.fRec148[1]) * F32::max(0.0, F32::min(fTemp159, 2.0 - fTemp159)) / fTemp158);
			let mut fTemp161: F32 = fTemp160 + self.fVec10[1] + fTemp151;
			self.fVec12[0] = fTemp161;
			self.fRec139[(self.IOTA0 & 2047) as usize] = 0.05 * self.fRec139[((i32::wrapping_sub(self.IOTA0, 1)) & 2047) as usize] + 0.95 * self.fVec12[2];
			let mut fRec135: F32 = fTemp155 * self.fRec139[((i32::wrapping_sub(self.IOTA0, iTemp146)) & 2047) as usize] + fTemp141 * (fTemp154 * self.fRec139[((i32::wrapping_sub(self.IOTA0, iTemp139)) & 2047) as usize] + 0.5 * fTemp153 * self.fRec139[((i32::wrapping_sub(self.IOTA0, iTemp134)) & 2047) as usize] + 0.16666667 * fTemp152 * self.fRec139[((i32::wrapping_sub(self.IOTA0, iTemp130)) & 2047) as usize] + 0.041666668 * fTemp127 * self.fRec139[((i32::wrapping_sub(self.IOTA0, iTemp120)) & 2047) as usize]);
			let mut fRec136: F32 = self.fVec12[1] + self.fRec129[1];
			self.fRec129[0] = fRec134;
			let mut fRec130: F32 = self.fRec129[1];
			let mut fRec131: F32 = fRec135;
			let mut fRec132: F32 = fRec136;
			self.fRec125[0] = fRec130;
			let mut fRec126: F32 = fTemp151 + fTemp160 + self.fRec125[1];
			let mut fRec127: F32 = fRec131;
			let mut fRec128: F32 = fRec132;
			self.fRec121[(self.IOTA0 & 2047) as usize] = fRec126;
			let mut fRec122: F32 = fTemp155 * self.fRec121[((i32::wrapping_sub(self.IOTA0, iTemp147)) & 2047) as usize] + fTemp141 * (fTemp154 * self.fRec121[((i32::wrapping_sub(self.IOTA0, iTemp140)) & 2047) as usize] + 0.5 * fTemp153 * self.fRec121[((i32::wrapping_sub(self.IOTA0, iTemp135)) & 2047) as usize] + 0.16666667 * fTemp152 * self.fRec121[((i32::wrapping_sub(self.IOTA0, iTemp131)) & 2047) as usize] + 0.041666668 * fTemp127 * self.fRec121[((i32::wrapping_sub(self.IOTA0, iTemp121)) & 2047) as usize]);
			self.fRec123[0] = fRec127;
			let mut fRec124: F32 = fRec128;
			self.fRec119[0] = fSlow5 * self.fRec123[1];
			let mut fRec120: F32 = fRec124;
			self.fRec114[0] = fRec118;
			let mut fRec115: F32 = fSlow5 * self.fRec114[1];
			let mut fRec116: F32 = self.fRec119[0];
			let mut fRec117: F32 = fRec120;
			self.fRec110[(self.IOTA0 & 2047) as usize] = fRec115;
			let mut fRec111: F32 = fRec122;
			let mut fRec112: F32 = fRec116;
			let mut fRec113: F32 = fRec117;
			self.fRec108[0] = fRec111;
			let mut fRec109: F32 = fRec113;
			let mut fTemp162: F32 = F32::abs(fRec109);
			let mut fTemp163: F32 = if (((self.fRec106[1] > fTemp162) as i32) as i32 != 0) { self.fConst12 } else { 0.0 };
			self.fRec107[0] = self.fRec107[1] * fTemp163 + fTemp162 * (1.0 - fTemp163);
			self.fRec106[0] = self.fRec107[0];
			let mut fRec105: F32 = 0.0 - 0.95 * F32::max(2e+01 * F32::log10(self.fRec106[0]) + 1e+01, 0.0);
			self.fRec104[0] = 1.5 * fRec109 * F32::powf(1e+01, 0.05 * fRec105);
			self.fRec103[0] = self.fRec104[0] - (self.fRec103[2] * (fTemp10 + (1.0 - (fTemp9 - fTemp113) / fTemp112)) + 2.0 * self.fRec103[1] * (fTemp10 + (1.0 - fTemp115))) / fTemp116;
			self.fRec102[0] = (self.fRec103[2] + self.fRec103[0] + 2.0 * self.fRec103[1]) / fTemp116 - (self.fRec102[2] * (fTemp6 + (fTemp113 - fTemp5) / fTemp112 + fTemp2) + 2.0 * self.fRec102[1] * (fTemp6 + (1.0 - (fTemp1 + fTemp115)))) / fTemp114;
			self.fRec149[0] = self.fConst2 * self.fRec149[1] + self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(((16.11811 * (fSlow10 + self.fRec3[0])) as i32), 2047))) as usize] };
			let mut fTemp164: F32 = F32::tan(self.fConst3 * F32::max(2e+01, F32::min(1e+04, self.fRec149[0])));
			let mut fTemp165: F32 = 1.0 / fTemp164;
			let mut fTemp166: F32 = fTemp6 + (fTemp5 + fTemp165) / fTemp164 + fTemp2;
			let mut fTemp167: F32 = 1.0 / mydsp_faustpower2_f(fTemp164);
			let mut fTemp168: F32 = fTemp10 + (fTemp9 + fTemp165) / fTemp164 + 1.0;
			let mut fRec166: F32 = -1.0 * 0.9973053 * (0.9 * self.fRec167[2] + 0.05 * (self.fRec167[1] + self.fRec167[3]));
			self.fRec185[0] = self.fConst2 * self.fRec185[1] + self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(((16.11811 * (fSlow10 + self.fRec40[0])) as i32), 2047))) as usize] };
			let mut fTemp169: F32 = self.fConst5 * (3.4e+02 / self.fRec185[0] + -0.11);
			let mut fTemp170: F32 = fTemp169 + -1.499995;
			let mut iTemp171: i32 = ((fTemp170) as i32);
			let mut iTemp172: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp171, 4))) as F32))) as i32);
			let mut iTemp173: i32 = i32::wrapping_add(iTemp172, 1);
			let mut fTemp174: F32 = F32::floor(fTemp170);
			let mut fTemp175: F32 = fTemp169 + (-3.0 - fTemp174);
			let mut fTemp176: F32 = fTemp169 + (-2.0 - fTemp174);
			let mut fTemp177: F32 = fTemp169 + (-1.0 - fTemp174);
			let mut fTemp178: F32 = fTemp177 * fTemp176;
			let mut fTemp179: F32 = fTemp178 * fTemp175;
			let mut fTemp180: F32 = fTemp169 + (-4.0 - fTemp174);
			let mut fTemp181: F32 = 0.0 - fTemp180;
			let mut iTemp182: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp171, 3))) as F32))) as i32);
			let mut iTemp183: i32 = i32::wrapping_add(iTemp182, 1);
			let mut fTemp184: F32 = 0.0 - 0.5 * fTemp180;
			let mut fTemp185: F32 = 0.0 - fTemp175;
			let mut iTemp186: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp171, 2))) as F32))) as i32);
			let mut iTemp187: i32 = i32::wrapping_add(iTemp186, 1);
			let mut fTemp188: F32 = 0.0 - 0.33333334 * fTemp180;
			let mut fTemp189: F32 = 0.0 - 0.5 * fTemp175;
			let mut fTemp190: F32 = 0.0 - fTemp176;
			let mut iTemp191: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp171, 1))) as F32))) as i32);
			let mut iTemp192: i32 = i32::wrapping_add(iTemp191, 1);
			let mut fTemp193: F32 = fTemp169 - fTemp174;
			let mut fTemp194: F32 = 0.0 - 0.25 * fTemp180;
			let mut fTemp195: F32 = 0.0 - 0.33333334 * fTemp175;
			let mut fTemp196: F32 = 0.0 - 0.5 * fTemp176;
			let mut fTemp197: F32 = 0.0 - fTemp177;
			let mut iTemp198: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, iTemp171)) as F32))) as i32);
			let mut iTemp199: i32 = i32::wrapping_add(iTemp198, 1);
			self.fRec181[0] = self.fRec158[((i32::wrapping_sub(self.IOTA0, iTemp199)) & 2047) as usize] * fTemp197 * fTemp196 * fTemp195 * fTemp194 + fTemp193 * (self.fRec158[((i32::wrapping_sub(self.IOTA0, iTemp192)) & 2047) as usize] * fTemp190 * fTemp189 * fTemp188 + 0.5 * fTemp177 * self.fRec158[((i32::wrapping_sub(self.IOTA0, iTemp187)) & 2047) as usize] * fTemp185 * fTemp184 + 0.16666667 * fTemp178 * self.fRec158[((i32::wrapping_sub(self.IOTA0, iTemp183)) & 2047) as usize] * fTemp181 + 0.041666668 * fTemp179 * self.fRec158[((i32::wrapping_sub(self.IOTA0, iTemp173)) & 2047) as usize]);
			self.fRec186[0] = 0.05 * self.fRec186[1] + 0.95 * self.fRec181[1];
			let mut fRec182: F32 = self.fRec186[0];
			self.fRec191[0] = self.fConst6 * self.fRec191[1] + self.fConst7 * F32::abs(self.fRec152[1]);
			let mut fRec190: F32 = self.fRec191[0];
			let mut iTemp200: i32 = ((fRec190 > 0.1) as i32);
			self.iVec13[0] = iTemp200;
			self.iRec192[0] = std::cmp::max(i32::wrapping_mul(self.iConst8, ((iTemp200 < self.iVec13[1]) as i32)), i32::wrapping_add(self.iRec192[1], -1));
			let mut fTemp201: F32 = F32::abs(F32::max(((iTemp200) as F32), ((((self.iRec192[0] > 0) as i32)) as F32)));
			let mut fTemp202: F32 = if (((self.fRec188[1] > fTemp201) as i32) as i32 != 0) { self.fConst9 } else { self.fConst6 };
			self.fRec189[0] = self.fRec189[1] * fTemp202 + fTemp201 * (1.0 - fTemp202);
			self.fRec188[0] = self.fRec189[0];
			let mut fTemp203: F32 = 0.005 * self.fRec188[0] * self.fRec152[1];
			self.fRec193[0] = self.fRec156[1];
			self.fRec194[(self.IOTA0 & 2047) as usize] = -1.0 * 0.9973053 * (0.9 * self.fRec193[2] + 0.05 * (self.fRec193[1] + self.fRec193[3]));
			let mut fTemp204: F32 = fTemp178 * fTemp181;
			let mut fTemp205: F32 = fTemp177 * fTemp185 * fTemp184;
			let mut fTemp206: F32 = fTemp190 * fTemp189 * fTemp188;
			let mut fTemp207: F32 = fTemp197 * fTemp196 * fTemp195 * fTemp194;
			self.fVec14[0] = fTemp207 * self.fRec194[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp198, 2))) & 2047) as usize] + fTemp193 * (fTemp206 * self.fRec194[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp191, 2))) & 2047) as usize] + 0.5 * fTemp205 * self.fRec194[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp186, 2))) & 2047) as usize] + 0.16666667 * fTemp204 * self.fRec194[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp182, 2))) & 2047) as usize] + 0.041666668 * fTemp179 * self.fRec194[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp172, 2))) & 2047) as usize]);
			let mut fTemp208: F32 = F32::tan(self.fConst10 * self.fRec185[0]);
			let mut fTemp209: F32 = 1.0 / fTemp208;
			let mut fTemp210: F32 = (fTemp209 + 1.4142135) / fTemp208 + 1.0;
			self.fVec15[0] = fSlow11;
			self.iRec195[0] = i32::wrapping_add(i32::wrapping_mul(i32::wrapping_add(self.iRec195[1], ((self.iRec195[1] > 0) as i32)), ((fSlow11 <= self.fVec15[1]) as i32)), ((fSlow11 > self.fVec15[1]) as i32));
			let mut fTemp211: F32 = ((self.iRec195[0]) as F32) / F32::max(1.0, self.fConst11 * mydsp_faustpower2_f(1.0 - 0.0005 * self.fRec185[0]));
			self.fRec196[0] = fTemp55 - (self.fRec196[2] * ((fTemp209 + -1.4142135) / fTemp208 + 1.0) + 2.0 * self.fRec196[1] * (1.0 - 1.0 / mydsp_faustpower2_f(fTemp208))) / fTemp210;
			let mut fTemp212: F32 = 0.5 * ((self.fRec196[2] + self.fRec196[0] + 2.0 * self.fRec196[1]) * F32::max(0.0, F32::min(fTemp211, 2.0 - fTemp211)) / fTemp210);
			let mut fTemp213: F32 = fTemp212 + self.fVec14[1] + fTemp203;
			self.fVec16[0] = fTemp213;
			self.fRec187[(self.IOTA0 & 2047) as usize] = 0.05 * self.fRec187[((i32::wrapping_sub(self.IOTA0, 1)) & 2047) as usize] + 0.95 * self.fVec16[2];
			let mut fRec183: F32 = fTemp207 * self.fRec187[((i32::wrapping_sub(self.IOTA0, iTemp198)) & 2047) as usize] + fTemp193 * (fTemp206 * self.fRec187[((i32::wrapping_sub(self.IOTA0, iTemp191)) & 2047) as usize] + 0.5 * fTemp205 * self.fRec187[((i32::wrapping_sub(self.IOTA0, iTemp186)) & 2047) as usize] + 0.16666667 * fTemp204 * self.fRec187[((i32::wrapping_sub(self.IOTA0, iTemp182)) & 2047) as usize] + 0.041666668 * fTemp179 * self.fRec187[((i32::wrapping_sub(self.IOTA0, iTemp172)) & 2047) as usize]);
			let mut fRec184: F32 = self.fVec16[1] + self.fRec177[1];
			self.fRec177[0] = fRec182;
			let mut fRec178: F32 = self.fRec177[1];
			let mut fRec179: F32 = fRec183;
			let mut fRec180: F32 = fRec184;
			self.fRec173[0] = fRec178;
			let mut fRec174: F32 = fTemp203 + fTemp212 + self.fRec173[1];
			let mut fRec175: F32 = fRec179;
			let mut fRec176: F32 = fRec180;
			self.fRec169[(self.IOTA0 & 2047) as usize] = fRec174;
			let mut fRec170: F32 = fTemp207 * self.fRec169[((i32::wrapping_sub(self.IOTA0, iTemp199)) & 2047) as usize] + fTemp193 * (fTemp206 * self.fRec169[((i32::wrapping_sub(self.IOTA0, iTemp192)) & 2047) as usize] + 0.5 * fTemp205 * self.fRec169[((i32::wrapping_sub(self.IOTA0, iTemp187)) & 2047) as usize] + 0.16666667 * fTemp204 * self.fRec169[((i32::wrapping_sub(self.IOTA0, iTemp183)) & 2047) as usize] + 0.041666668 * fTemp179 * self.fRec169[((i32::wrapping_sub(self.IOTA0, iTemp173)) & 2047) as usize]);
			self.fRec171[0] = fRec175;
			let mut fRec172: F32 = fRec176;
			self.fRec167[0] = fSlow5 * self.fRec171[1];
			let mut fRec168: F32 = fRec172;
			self.fRec162[0] = fRec166;
			let mut fRec163: F32 = fSlow5 * self.fRec162[1];
			let mut fRec164: F32 = self.fRec167[0];
			let mut fRec165: F32 = fRec168;
			self.fRec158[(self.IOTA0 & 2047) as usize] = fRec163;
			let mut fRec159: F32 = fRec170;
			let mut fRec160: F32 = fRec164;
			let mut fRec161: F32 = fRec165;
			self.fRec156[0] = fRec159;
			let mut fRec157: F32 = fRec161;
			let mut fTemp214: F32 = F32::abs(fRec157);
			let mut fTemp215: F32 = if (((self.fRec154[1] > fTemp214) as i32) as i32 != 0) { self.fConst12 } else { 0.0 };
			self.fRec155[0] = self.fRec155[1] * fTemp215 + fTemp214 * (1.0 - fTemp215);
			self.fRec154[0] = self.fRec155[0];
			let mut fRec153: F32 = 0.0 - 0.95 * F32::max(2e+01 * F32::log10(self.fRec154[0]) + 1e+01, 0.0);
			self.fRec152[0] = 1.5 * fRec157 * F32::powf(1e+01, 0.05 * fRec153);
			self.fRec151[0] = self.fRec152[0] - (self.fRec151[2] * (fTemp10 + (fTemp165 - fTemp9) / fTemp164 + 1.0) + 2.0 * self.fRec151[1] * (fTemp10 + (1.0 - fTemp167))) / fTemp168;
			self.fRec150[0] = (self.fRec151[2] + self.fRec151[0] + 2.0 * self.fRec151[1]) / fTemp168 - (self.fRec150[2] * (fTemp6 + (fTemp165 - fTemp5) / fTemp164 + fTemp2) + 2.0 * self.fRec150[1] * (fTemp6 + (1.0 - (fTemp1 + fTemp167)))) / fTemp166;
			self.fRec197[0] = self.fConst2 * self.fRec197[1] + self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(((16.11811 * (fSlow12 + self.fRec3[0])) as i32), 2047))) as usize] };
			let mut fTemp216: F32 = F32::tan(self.fConst3 * F32::max(2e+01, F32::min(1e+04, self.fRec197[0])));
			let mut fTemp217: F32 = 1.0 / fTemp216;
			let mut fTemp218: F32 = fTemp6 + (fTemp5 + fTemp217) / fTemp216 + fTemp2;
			let mut fTemp219: F32 = 1.0 / mydsp_faustpower2_f(fTemp216);
			let mut fTemp220: F32 = fTemp10 + (fTemp9 + fTemp217) / fTemp216 + 1.0;
			let mut fRec214: F32 = -1.0 * 0.9973053 * (0.9 * self.fRec215[2] + 0.05 * (self.fRec215[1] + self.fRec215[3]));
			self.fRec233[0] = self.fConst2 * self.fRec233[1] + self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(((16.11811 * (fSlow12 + self.fRec40[0])) as i32), 2047))) as usize] };
			let mut fTemp221: F32 = self.fConst5 * (3.4e+02 / self.fRec233[0] + -0.11);
			let mut fTemp222: F32 = fTemp221 + -1.499995;
			let mut iTemp223: i32 = ((fTemp222) as i32);
			let mut iTemp224: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp223, 4))) as F32))) as i32);
			let mut iTemp225: i32 = i32::wrapping_add(iTemp224, 1);
			let mut fTemp226: F32 = F32::floor(fTemp222);
			let mut fTemp227: F32 = fTemp221 + (-3.0 - fTemp226);
			let mut fTemp228: F32 = fTemp221 + (-2.0 - fTemp226);
			let mut fTemp229: F32 = fTemp221 + (-1.0 - fTemp226);
			let mut fTemp230: F32 = fTemp229 * fTemp228;
			let mut fTemp231: F32 = fTemp230 * fTemp227;
			let mut fTemp232: F32 = fTemp221 + (-4.0 - fTemp226);
			let mut fTemp233: F32 = 0.0 - fTemp232;
			let mut iTemp234: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp223, 3))) as F32))) as i32);
			let mut iTemp235: i32 = i32::wrapping_add(iTemp234, 1);
			let mut fTemp236: F32 = 0.0 - 0.5 * fTemp232;
			let mut fTemp237: F32 = 0.0 - fTemp227;
			let mut iTemp238: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp223, 2))) as F32))) as i32);
			let mut iTemp239: i32 = i32::wrapping_add(iTemp238, 1);
			let mut fTemp240: F32 = 0.0 - 0.33333334 * fTemp232;
			let mut fTemp241: F32 = 0.0 - 0.5 * fTemp227;
			let mut fTemp242: F32 = 0.0 - fTemp228;
			let mut iTemp243: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp223, 1))) as F32))) as i32);
			let mut iTemp244: i32 = i32::wrapping_add(iTemp243, 1);
			let mut fTemp245: F32 = fTemp221 - fTemp226;
			let mut fTemp246: F32 = 0.0 - 0.25 * fTemp232;
			let mut fTemp247: F32 = 0.0 - 0.33333334 * fTemp227;
			let mut fTemp248: F32 = 0.0 - 0.5 * fTemp228;
			let mut fTemp249: F32 = 0.0 - fTemp229;
			let mut iTemp250: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, iTemp223)) as F32))) as i32);
			let mut iTemp251: i32 = i32::wrapping_add(iTemp250, 1);
			self.fRec229[0] = self.fRec206[((i32::wrapping_sub(self.IOTA0, iTemp251)) & 2047) as usize] * fTemp249 * fTemp248 * fTemp247 * fTemp246 + fTemp245 * (self.fRec206[((i32::wrapping_sub(self.IOTA0, iTemp244)) & 2047) as usize] * fTemp242 * fTemp241 * fTemp240 + 0.5 * fTemp229 * self.fRec206[((i32::wrapping_sub(self.IOTA0, iTemp239)) & 2047) as usize] * fTemp237 * fTemp236 + 0.16666667 * fTemp230 * self.fRec206[((i32::wrapping_sub(self.IOTA0, iTemp235)) & 2047) as usize] * fTemp233 + 0.041666668 * fTemp231 * self.fRec206[((i32::wrapping_sub(self.IOTA0, iTemp225)) & 2047) as usize]);
			self.fRec234[0] = 0.05 * self.fRec234[1] + 0.95 * self.fRec229[1];
			let mut fRec230: F32 = self.fRec234[0];
			self.fRec239[0] = self.fConst6 * self.fRec239[1] + self.fConst7 * F32::abs(self.fRec200[1]);
			let mut fRec238: F32 = self.fRec239[0];
			let mut iTemp252: i32 = ((fRec238 > 0.1) as i32);
			self.iVec17[0] = iTemp252;
			self.iRec240[0] = std::cmp::max(i32::wrapping_mul(self.iConst8, ((iTemp252 < self.iVec17[1]) as i32)), i32::wrapping_add(self.iRec240[1], -1));
			let mut fTemp253: F32 = F32::abs(F32::max(((iTemp252) as F32), ((((self.iRec240[0] > 0) as i32)) as F32)));
			let mut fTemp254: F32 = if (((self.fRec236[1] > fTemp253) as i32) as i32 != 0) { self.fConst9 } else { self.fConst6 };
			self.fRec237[0] = self.fRec237[1] * fTemp254 + fTemp253 * (1.0 - fTemp254);
			self.fRec236[0] = self.fRec237[0];
			let mut fTemp255: F32 = 0.005 * self.fRec236[0] * self.fRec200[1];
			self.fRec241[0] = self.fRec204[1];
			self.fRec242[(self.IOTA0 & 2047) as usize] = -1.0 * 0.9973053 * (0.9 * self.fRec241[2] + 0.05 * (self.fRec241[1] + self.fRec241[3]));
			let mut fTemp256: F32 = fTemp230 * fTemp233;
			let mut fTemp257: F32 = fTemp229 * fTemp237 * fTemp236;
			let mut fTemp258: F32 = fTemp242 * fTemp241 * fTemp240;
			let mut fTemp259: F32 = fTemp249 * fTemp248 * fTemp247 * fTemp246;
			self.fVec18[0] = fTemp259 * self.fRec242[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp250, 2))) & 2047) as usize] + fTemp245 * (fTemp258 * self.fRec242[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp243, 2))) & 2047) as usize] + 0.5 * fTemp257 * self.fRec242[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp238, 2))) & 2047) as usize] + 0.16666667 * fTemp256 * self.fRec242[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp234, 2))) & 2047) as usize] + 0.041666668 * fTemp231 * self.fRec242[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp224, 2))) & 2047) as usize]);
			let mut fTemp260: F32 = F32::tan(self.fConst10 * self.fRec233[0]);
			let mut fTemp261: F32 = 1.0 / fTemp260;
			let mut fTemp262: F32 = (fTemp261 + 1.4142135) / fTemp260 + 1.0;
			self.fVec19[0] = fSlow13;
			self.iRec243[0] = i32::wrapping_add(i32::wrapping_mul(i32::wrapping_add(self.iRec243[1], ((self.iRec243[1] > 0) as i32)), ((fSlow13 <= self.fVec19[1]) as i32)), ((fSlow13 > self.fVec19[1]) as i32));
			let mut fTemp263: F32 = ((self.iRec243[0]) as F32) / F32::max(1.0, self.fConst11 * mydsp_faustpower2_f(1.0 - 0.0005 * self.fRec233[0]));
			self.fRec244[0] = fTemp55 - (self.fRec244[2] * ((fTemp261 + -1.4142135) / fTemp260 + 1.0) + 2.0 * self.fRec244[1] * (1.0 - 1.0 / mydsp_faustpower2_f(fTemp260))) / fTemp262;
			let mut fTemp264: F32 = 0.5 * ((self.fRec244[2] + self.fRec244[0] + 2.0 * self.fRec244[1]) * F32::max(0.0, F32::min(fTemp263, 2.0 - fTemp263)) / fTemp262);
			let mut fTemp265: F32 = fTemp264 + self.fVec18[1] + fTemp255;
			self.fVec20[0] = fTemp265;
			self.fRec235[(self.IOTA0 & 2047) as usize] = 0.05 * self.fRec235[((i32::wrapping_sub(self.IOTA0, 1)) & 2047) as usize] + 0.95 * self.fVec20[2];
			let mut fRec231: F32 = fTemp259 * self.fRec235[((i32::wrapping_sub(self.IOTA0, iTemp250)) & 2047) as usize] + fTemp245 * (fTemp258 * self.fRec235[((i32::wrapping_sub(self.IOTA0, iTemp243)) & 2047) as usize] + 0.5 * fTemp257 * self.fRec235[((i32::wrapping_sub(self.IOTA0, iTemp238)) & 2047) as usize] + 0.16666667 * fTemp256 * self.fRec235[((i32::wrapping_sub(self.IOTA0, iTemp234)) & 2047) as usize] + 0.041666668 * fTemp231 * self.fRec235[((i32::wrapping_sub(self.IOTA0, iTemp224)) & 2047) as usize]);
			let mut fRec232: F32 = self.fVec20[1] + self.fRec225[1];
			self.fRec225[0] = fRec230;
			let mut fRec226: F32 = self.fRec225[1];
			let mut fRec227: F32 = fRec231;
			let mut fRec228: F32 = fRec232;
			self.fRec221[0] = fRec226;
			let mut fRec222: F32 = fTemp255 + fTemp264 + self.fRec221[1];
			let mut fRec223: F32 = fRec227;
			let mut fRec224: F32 = fRec228;
			self.fRec217[(self.IOTA0 & 2047) as usize] = fRec222;
			let mut fRec218: F32 = fTemp259 * self.fRec217[((i32::wrapping_sub(self.IOTA0, iTemp251)) & 2047) as usize] + fTemp245 * (fTemp258 * self.fRec217[((i32::wrapping_sub(self.IOTA0, iTemp244)) & 2047) as usize] + 0.5 * fTemp257 * self.fRec217[((i32::wrapping_sub(self.IOTA0, iTemp239)) & 2047) as usize] + 0.16666667 * fTemp256 * self.fRec217[((i32::wrapping_sub(self.IOTA0, iTemp235)) & 2047) as usize] + 0.041666668 * fTemp231 * self.fRec217[((i32::wrapping_sub(self.IOTA0, iTemp225)) & 2047) as usize]);
			self.fRec219[0] = fRec223;
			let mut fRec220: F32 = fRec224;
			self.fRec215[0] = fSlow5 * self.fRec219[1];
			let mut fRec216: F32 = fRec220;
			self.fRec210[0] = fRec214;
			let mut fRec211: F32 = fSlow5 * self.fRec210[1];
			let mut fRec212: F32 = self.fRec215[0];
			let mut fRec213: F32 = fRec216;
			self.fRec206[(self.IOTA0 & 2047) as usize] = fRec211;
			let mut fRec207: F32 = fRec218;
			let mut fRec208: F32 = fRec212;
			let mut fRec209: F32 = fRec213;
			self.fRec204[0] = fRec207;
			let mut fRec205: F32 = fRec209;
			let mut fTemp266: F32 = F32::abs(fRec205);
			let mut fTemp267: F32 = if (((self.fRec202[1] > fTemp266) as i32) as i32 != 0) { self.fConst12 } else { 0.0 };
			self.fRec203[0] = self.fRec203[1] * fTemp267 + fTemp266 * (1.0 - fTemp267);
			self.fRec202[0] = self.fRec203[0];
			let mut fRec201: F32 = 0.0 - 0.95 * F32::max(2e+01 * F32::log10(self.fRec202[0]) + 1e+01, 0.0);
			self.fRec200[0] = 1.5 * fRec205 * F32::powf(1e+01, 0.05 * fRec201);
			self.fRec199[0] = self.fRec200[0] - (self.fRec199[2] * (fTemp10 + (fTemp217 - fTemp9) / fTemp216 + 1.0) + 2.0 * self.fRec199[1] * (fTemp10 + (1.0 - fTemp219))) / fTemp220;
			self.fRec198[0] = (self.fRec199[2] + self.fRec199[0] + 2.0 * self.fRec199[1]) / fTemp220 - (self.fRec198[2] * (fTemp6 + (fTemp217 - fTemp5) / fTemp216 + fTemp2) + 2.0 * self.fRec198[1] * (fTemp6 + (1.0 - (fTemp1 + fTemp219)))) / fTemp218;
			self.fRec245[0] = fSlow14 + self.fConst2 * self.fRec245[1];
			let mut fTemp268: F32 = self.fRec245[0] * ((self.fRec198[2] + self.fRec198[0] + 2.0 * self.fRec198[1]) / fTemp218 + (self.fRec150[2] + self.fRec150[0] + 2.0 * self.fRec150[1]) / fTemp166 + (self.fRec102[2] + self.fRec102[0] + 2.0 * self.fRec102[1]) / fTemp114 + (self.fRec54[2] + self.fRec54[0] + 2.0 * self.fRec54[1]) / fTemp62 + (self.fRec4[2] + self.fRec4[0] + 2.0 * self.fRec4[1]) / fTemp7);
			let mut fTemp269: F32 = self.fRec40[0] + self.fRec3[0];
			self.fRec246[0] = self.fConst2 * self.fRec246[1] + self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(((16.11811 * (fSlow15 + fTemp269)) as i32), 2047))) as usize] };
			let mut fTemp270: F32 = F32::tan(self.fConst3 * F32::max(2e+01, F32::min(1e+04, self.fRec246[0])));
			let mut fTemp271: F32 = 1.0 / fTemp270;
			let mut fTemp272: F32 = fTemp6 + (fTemp5 + fTemp271) / fTemp270 + fTemp2;
			let mut fTemp273: F32 = 1.0 / mydsp_faustpower2_f(fTemp270);
			let mut fTemp274: F32 = fTemp10 + (fTemp9 + fTemp271) / fTemp270 + 1.0;
			self.fRec251[0] = self.fConst2 * self.fRec251[1] + self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(((16.11811 * (fSlow15 + self.fRec40[0])) as i32), 2047))) as usize] };
			let mut fTemp275: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec251[0]));
			let mut fTemp276: F32 = self.fConst13 * fTemp275;
			let mut fTemp277: F32 = self.fRec249[1] + fTemp276;
			let mut fTemp278: F32 = fTemp277 + -1.0;
			let mut iTemp279: i32 = ((fTemp278 < 0.0) as i32);
			self.fRec249[0] = if (iTemp279 as i32 != 0) { fTemp277 } else { fTemp278 };
			let mut fThen11: F32 = fTemp276 + self.fRec249[1] + (1.0 - self.fConst0 / fTemp275) * fTemp278;
			let mut fRec250: F32 = if (iTemp279 as i32 != 0) { fTemp277 } else { fThen11 };
			self.fRec252[0] = fSlow16 + self.fConst2 * self.fRec252[1];
			self.fRec248[0] = self.fRec252[0] * (2.0 * fRec250 + -1.0) - (self.fRec248[2] * (fTemp10 + (fTemp271 - fTemp9) / fTemp270 + 1.0) + 2.0 * self.fRec248[1] * (fTemp10 + (1.0 - fTemp273))) / fTemp274;
			self.fRec247[0] = (self.fRec248[2] + self.fRec248[0] + 2.0 * self.fRec248[1]) / fTemp274 - (self.fRec247[2] * (fTemp6 + (fTemp271 - fTemp5) / fTemp270 + fTemp2) + 2.0 * self.fRec247[1] * (fTemp6 + (1.0 - (fTemp1 + fTemp273)))) / fTemp272;
			self.fRec253[0] = self.fConst2 * self.fRec253[1] + self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(((16.11811 * (fSlow17 + fTemp269)) as i32), 2047))) as usize] };
			let mut fTemp280: F32 = F32::tan(self.fConst3 * F32::max(2e+01, F32::min(1e+04, self.fRec253[0])));
			let mut fTemp281: F32 = 1.0 / fTemp280;
			let mut fTemp282: F32 = fTemp6 + (fTemp5 + fTemp281) / fTemp280 + fTemp2;
			let mut fTemp283: F32 = 1.0 / mydsp_faustpower2_f(fTemp280);
			let mut fTemp284: F32 = fTemp10 + (fTemp9 + fTemp281) / fTemp280 + 1.0;
			self.fRec258[0] = self.fConst2 * self.fRec258[1] + self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(((16.11811 * (fSlow17 + self.fRec40[0])) as i32), 2047))) as usize] };
			let mut fTemp285: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec258[0]));
			let mut fTemp286: F32 = self.fConst13 * fTemp285;
			let mut fTemp287: F32 = self.fRec256[1] + fTemp286;
			let mut fTemp288: F32 = fTemp287 + -1.0;
			let mut iTemp289: i32 = ((fTemp288 < 0.0) as i32);
			self.fRec256[0] = if (iTemp289 as i32 != 0) { fTemp287 } else { fTemp288 };
			let mut fThen13: F32 = self.fRec256[1] + fTemp286 + (1.0 - self.fConst0 / fTemp285) * fTemp288;
			let mut fRec257: F32 = if (iTemp289 as i32 != 0) { fTemp287 } else { fThen13 };
			self.fRec259[0] = fSlow18 + self.fConst2 * self.fRec259[1];
			self.fRec255[0] = self.fRec259[0] * (2.0 * fRec257 + -1.0) - (self.fRec255[2] * (fTemp10 + (1.0 - (fTemp9 - fTemp281) / fTemp280)) + 2.0 * self.fRec255[1] * (fTemp10 + (1.0 - fTemp283))) / fTemp284;
			self.fRec254[0] = (self.fRec255[2] + self.fRec255[0] + 2.0 * self.fRec255[1]) / fTemp284 - (self.fRec254[2] * (fTemp6 + (fTemp281 - fTemp5) / fTemp280 + fTemp2) + 2.0 * self.fRec254[1] * (fTemp6 + (1.0 - (fTemp1 + fTemp283)))) / fTemp282;
			self.fRec260[0] = self.fConst2 * self.fRec260[1] + self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(((16.11811 * (fSlow19 + fTemp269)) as i32), 2047))) as usize] };
			let mut fTemp290: F32 = F32::tan(self.fConst3 * F32::max(2e+01, F32::min(1e+04, self.fRec260[0])));
			let mut fTemp291: F32 = 1.0 / fTemp290;
			let mut fTemp292: F32 = fTemp6 + (fTemp5 + fTemp291) / fTemp290 + fTemp2;
			let mut fTemp293: F32 = 1.0 / mydsp_faustpower2_f(fTemp290);
			let mut fTemp294: F32 = fTemp10 + (fTemp9 + fTemp291) / fTemp290 + 1.0;
			self.fRec265[0] = self.fConst2 * self.fRec265[1] + self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(((16.11811 * (fSlow19 + self.fRec40[0])) as i32), 2047))) as usize] };
			let mut fTemp295: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec265[0]));
			let mut fTemp296: F32 = self.fConst13 * fTemp295;
			let mut fTemp297: F32 = self.fRec263[1] + fTemp296;
			let mut fTemp298: F32 = fTemp297 + -1.0;
			let mut iTemp299: i32 = ((fTemp298 < 0.0) as i32);
			self.fRec263[0] = if (iTemp299 as i32 != 0) { fTemp297 } else { fTemp298 };
			let mut fThen15: F32 = self.fRec263[1] + fTemp296 + (1.0 - self.fConst0 / fTemp295) * fTemp298;
			let mut fRec264: F32 = if (iTemp299 as i32 != 0) { fTemp297 } else { fThen15 };
			self.fRec266[0] = fSlow20 + self.fConst2 * self.fRec266[1];
			self.fRec262[0] = self.fRec266[0] * (2.0 * fRec264 + -1.0) - (self.fRec262[2] * (fTemp10 + (fTemp291 - fTemp9) / fTemp290 + 1.0) + 2.0 * self.fRec262[1] * (fTemp10 + (1.0 - fTemp293))) / fTemp294;
			self.fRec261[0] = (self.fRec262[2] + self.fRec262[0] + 2.0 * self.fRec262[1]) / fTemp294 - (self.fRec261[2] * (fTemp6 + (fTemp291 - fTemp5) / fTemp290 + fTemp2) + 2.0 * self.fRec261[1] * (fTemp6 + (1.0 - (fTemp1 + fTemp293)))) / fTemp292;
			self.fRec267[0] = self.fConst2 * self.fRec267[1] + self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(((16.11811 * (fSlow21 + fTemp269)) as i32), 2047))) as usize] };
			let mut fTemp300: F32 = F32::tan(self.fConst3 * F32::max(2e+01, F32::min(1e+04, self.fRec267[0])));
			let mut fTemp301: F32 = 1.0 / fTemp300;
			let mut fTemp302: F32 = fTemp6 + (fTemp5 + fTemp301) / fTemp300 + fTemp2;
			let mut fTemp303: F32 = 1.0 / mydsp_faustpower2_f(fTemp300);
			let mut fTemp304: F32 = fTemp10 + (fTemp301 + fTemp9) / fTemp300 + 1.0;
			self.fRec272[0] = self.fConst2 * self.fRec272[1] + self.fConst1 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min(((16.11811 * (fSlow21 + self.fRec40[0])) as i32), 2047))) as usize] };
			let mut fTemp305: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec272[0]));
			let mut fTemp306: F32 = self.fConst13 * fTemp305;
			let mut fTemp307: F32 = self.fRec270[1] + fTemp306;
			let mut fTemp308: F32 = fTemp307 + -1.0;
			let mut iTemp309: i32 = ((fTemp308 < 0.0) as i32);
			self.fRec270[0] = if (iTemp309 as i32 != 0) { fTemp307 } else { fTemp308 };
			let mut fThen17: F32 = self.fRec270[1] + fTemp306 + (1.0 - self.fConst0 / fTemp305) * fTemp308;
			let mut fRec271: F32 = if (iTemp309 as i32 != 0) { fTemp307 } else { fThen17 };
			self.fRec273[0] = fSlow22 + self.fConst2 * self.fRec273[1];
			self.fRec269[0] = self.fRec273[0] * (2.0 * fRec271 + -1.0) - (self.fRec269[2] * (fTemp10 + (fTemp301 - fTemp9) / fTemp300 + 1.0) + 2.0 * self.fRec269[1] * (fTemp10 + (1.0 - fTemp303))) / fTemp304;
			self.fRec268[0] = (self.fRec269[2] + self.fRec269[0] + 2.0 * self.fRec269[1]) / fTemp304 - (self.fRec268[2] * (fTemp6 + (fTemp301 - fTemp5) / fTemp300 + fTemp2) + 2.0 * self.fRec268[1] * (fTemp6 + (1.0 - (fTemp1 + fTemp303)))) / fTemp302;
			self.fRec274[0] = fSlow23 + self.fConst2 * self.fRec274[1];
			self.fRec275[0] = fSlow24 + self.fConst2 * self.fRec275[1];
			let mut fTemp310: F32 = self.fRec275[0] * self.fRec274[0] * ((self.fRec268[2] + self.fRec268[0] + 2.0 * self.fRec268[1]) / fTemp302 + (self.fRec261[2] + self.fRec261[0] + 2.0 * self.fRec261[1]) / fTemp292 + (self.fRec254[2] + self.fRec254[0] + 2.0 * self.fRec254[1]) / fTemp282 + (self.fRec247[2] + self.fRec247[0] + 2.0 * self.fRec247[1]) / fTemp272);
			self.fRec276[0] = fSlow25 + self.fConst2 * self.fRec276[1];
			self.fRec278[0] = self.fConst2 * self.fRec278[1] + fSlow28;
			let mut fTemp311: F32 = F32::max(self.fRec278[0], 23.44895);
			let mut fTemp312: F32 = F32::max(2e+01, F32::abs(fTemp311));
			let mut fTemp313: F32 = self.fRec279[1] + self.fConst13 * fTemp312;
			self.fRec279[0] = fTemp313 - F32::floor(fTemp313);
			let mut fTemp314: F32 = mydsp_faustpower2_f(2.0 * self.fRec279[0] + -1.0);
			self.fVec21[0] = fTemp314;
			let mut fTemp315: F32 = ((self.iVec0[1]) as F32);
			let mut fTemp316: F32 = fTemp315 * (fTemp314 - self.fVec21[1]) / fTemp312;
			self.fVec22[(self.IOTA0 & 4095) as usize] = fTemp316;
			let mut fTemp317: F32 = F32::max(0.0, F32::min(2047.0, self.fConst14 / fTemp311));
			let mut iTemp318: i32 = ((fTemp317) as i32);
			let mut fTemp319: F32 = F32::floor(fTemp317);
			self.fRec277[0] = 0.999 * self.fRec277[1] + self.fConst15 * (fTemp316 - self.fVec22[((i32::wrapping_sub(self.IOTA0, iTemp318)) & 4095) as usize] * (fTemp319 + (1.0 - fTemp317)) - (fTemp317 - fTemp319) * self.fVec22[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp318, 1))) & 4095) as usize]);
			self.fRec281[0] = self.fConst2 * self.fRec281[1] + fSlow29;
			let mut fTemp320: F32 = F32::max(self.fRec281[0], 23.44895);
			let mut fTemp321: F32 = F32::max(2e+01, F32::abs(fTemp320));
			let mut fTemp322: F32 = self.fRec282[1] + self.fConst13 * fTemp321;
			self.fRec282[0] = fTemp322 - F32::floor(fTemp322);
			let mut fTemp323: F32 = mydsp_faustpower2_f(2.0 * self.fRec282[0] + -1.0);
			self.fVec23[0] = fTemp323;
			let mut fTemp324: F32 = fTemp315 * (fTemp323 - self.fVec23[1]) / fTemp321;
			self.fVec24[(self.IOTA0 & 4095) as usize] = fTemp324;
			let mut fTemp325: F32 = F32::max(0.0, F32::min(2047.0, self.fConst14 / fTemp320));
			let mut iTemp326: i32 = ((fTemp325) as i32);
			let mut fTemp327: F32 = F32::floor(fTemp325);
			self.fRec280[0] = 0.999 * self.fRec280[1] + self.fConst15 * (fTemp324 - self.fVec24[((i32::wrapping_sub(self.IOTA0, iTemp326)) & 4095) as usize] * (fTemp327 + (1.0 - fTemp325)) - (fTemp325 - fTemp327) * self.fVec24[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp326, 1))) & 4095) as usize]);
			self.fRec284[0] = self.fConst2 * self.fRec284[1] + fSlow30;
			let mut fTemp328: F32 = F32::max(self.fRec284[0], 23.44895);
			let mut fTemp329: F32 = F32::max(2e+01, F32::abs(fTemp328));
			let mut fTemp330: F32 = self.fRec285[1] + self.fConst13 * fTemp329;
			self.fRec285[0] = fTemp330 - F32::floor(fTemp330);
			let mut fTemp331: F32 = mydsp_faustpower2_f(2.0 * self.fRec285[0] + -1.0);
			self.fVec25[0] = fTemp331;
			let mut fTemp332: F32 = fTemp315 * (fTemp331 - self.fVec25[1]) / fTemp329;
			self.fVec26[(self.IOTA0 & 4095) as usize] = fTemp332;
			let mut fTemp333: F32 = F32::max(0.0, F32::min(2047.0, self.fConst14 / fTemp328));
			let mut iTemp334: i32 = ((fTemp333) as i32);
			let mut fTemp335: F32 = F32::floor(fTemp333);
			self.fRec283[0] = 0.999 * self.fRec283[1] + self.fConst15 * (fTemp332 - self.fVec26[((i32::wrapping_sub(self.IOTA0, iTemp334)) & 4095) as usize] * (fTemp335 + (1.0 - fTemp333)) - (fTemp333 - fTemp335) * self.fVec26[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp334, 1))) & 4095) as usize]);
			self.fRec286[0] = fSlow31 + self.fConst2 * self.fRec286[1];
			self.fRec288[0] = self.fConst2 * self.fRec288[1] + fSlow33;
			let mut fTemp336: F32 = F32::max(self.fRec288[0], 23.44895);
			let mut fTemp337: F32 = F32::max(2e+01, F32::abs(fTemp336));
			let mut fTemp338: F32 = self.fRec289[1] + self.fConst13 * fTemp337;
			self.fRec289[0] = fTemp338 - F32::floor(fTemp338);
			let mut fTemp339: F32 = mydsp_faustpower2_f(2.0 * self.fRec289[0] + -1.0);
			self.fVec27[0] = fTemp339;
			let mut fTemp340: F32 = fTemp315 * (fTemp339 - self.fVec27[1]) / fTemp337;
			self.fVec28[(self.IOTA0 & 4095) as usize] = fTemp340;
			let mut fTemp341: F32 = F32::max(0.0, F32::min(2047.0, self.fConst14 / fTemp336));
			let mut iTemp342: i32 = ((fTemp341) as i32);
			let mut fTemp343: F32 = F32::floor(fTemp341);
			self.fRec287[0] = 0.999 * self.fRec287[1] + self.fConst15 * (fTemp340 - self.fVec28[((i32::wrapping_sub(self.IOTA0, iTemp342)) & 4095) as usize] * (fTemp343 + (1.0 - fTemp341)) - (fTemp341 - fTemp343) * self.fVec28[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp342, 1))) & 4095) as usize]);
			self.fRec291[0] = self.fConst2 * self.fRec291[1] + fSlow34;
			let mut fTemp344: F32 = F32::max(self.fRec291[0], 23.44895);
			let mut fTemp345: F32 = F32::max(0.0, F32::min(2047.0, self.fConst14 / fTemp344));
			let mut fTemp346: F32 = F32::floor(fTemp345);
			let mut fTemp347: F32 = F32::max(2e+01, F32::abs(fTemp344));
			let mut fTemp348: F32 = self.fRec292[1] + self.fConst13 * fTemp347;
			self.fRec292[0] = fTemp348 - F32::floor(fTemp348);
			let mut fTemp349: F32 = mydsp_faustpower2_f(2.0 * self.fRec292[0] + -1.0);
			self.fVec29[0] = fTemp349;
			let mut fTemp350: F32 = fTemp315 * (fTemp349 - self.fVec29[1]) / fTemp347;
			self.fVec30[(self.IOTA0 & 4095) as usize] = fTemp350;
			let mut iTemp351: i32 = ((fTemp345) as i32);
			self.fRec290[0] = 0.999 * self.fRec290[1] - self.fConst15 * ((fTemp345 - fTemp346) * self.fVec30[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp351, 1))) & 4095) as usize] - (fTemp350 - self.fVec30[((i32::wrapping_sub(self.IOTA0, iTemp351)) & 4095) as usize] * (fTemp346 + (1.0 - fTemp345))));
			self.fRec294[0] = self.fConst2 * self.fRec294[1] + fSlow35;
			let mut fTemp352: F32 = F32::max(self.fRec294[0], 23.44895);
			let mut fTemp353: F32 = F32::max(2e+01, F32::abs(fTemp352));
			let mut fTemp354: F32 = self.fRec295[1] + self.fConst13 * fTemp353;
			self.fRec295[0] = fTemp354 - F32::floor(fTemp354);
			let mut fTemp355: F32 = mydsp_faustpower2_f(2.0 * self.fRec295[0] + -1.0);
			self.fVec31[0] = fTemp355;
			let mut fTemp356: F32 = fTemp315 * (fTemp355 - self.fVec31[1]) / fTemp353;
			self.fVec32[(self.IOTA0 & 4095) as usize] = fTemp356;
			let mut fTemp357: F32 = F32::max(0.0, F32::min(2047.0, self.fConst14 / fTemp352));
			let mut iTemp358: i32 = ((fTemp357) as i32);
			let mut fTemp359: F32 = F32::floor(fTemp357);
			self.fRec293[0] = 0.999 * self.fRec293[1] - self.fConst15 * (self.fVec32[((i32::wrapping_sub(self.IOTA0, iTemp358)) & 4095) as usize] * (fTemp359 + (1.0 - fTemp357)) - fTemp356 + (fTemp357 - fTemp359) * self.fVec32[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp358, 1))) & 4095) as usize]);
			self.fRec296[0] = fSlow36 + self.fConst2 * self.fRec296[1];
			self.fRec298[0] = self.fConst2 * self.fRec298[1] + fSlow38;
			let mut fTemp360: F32 = F32::max(self.fRec298[0], 23.44895);
			let mut fTemp361: F32 = F32::max(2e+01, F32::abs(fTemp360));
			let mut fTemp362: F32 = self.fRec299[1] + self.fConst13 * fTemp361;
			self.fRec299[0] = fTemp362 - F32::floor(fTemp362);
			let mut fTemp363: F32 = mydsp_faustpower2_f(2.0 * self.fRec299[0] + -1.0);
			self.fVec33[0] = fTemp363;
			let mut fTemp364: F32 = fTemp315 * (fTemp363 - self.fVec33[1]) / fTemp361;
			self.fVec34[(self.IOTA0 & 4095) as usize] = fTemp364;
			let mut fTemp365: F32 = F32::max(0.0, F32::min(2047.0, self.fConst14 / fTemp360));
			let mut iTemp366: i32 = ((fTemp365) as i32);
			let mut fTemp367: F32 = F32::floor(fTemp365);
			self.fRec297[0] = 0.999 * self.fRec297[1] - self.fConst15 * (self.fVec34[((i32::wrapping_sub(self.IOTA0, iTemp366)) & 4095) as usize] * (fTemp367 + (1.0 - fTemp365)) - fTemp364 + (fTemp365 - fTemp367) * self.fVec34[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp366, 1))) & 4095) as usize]);
			self.fRec301[0] = self.fConst2 * self.fRec301[1] + fSlow39;
			let mut fTemp368: F32 = F32::max(self.fRec301[0], 23.44895);
			let mut fTemp369: F32 = F32::max(2e+01, F32::abs(fTemp368));
			let mut fTemp370: F32 = self.fRec302[1] + self.fConst13 * fTemp369;
			self.fRec302[0] = fTemp370 - F32::floor(fTemp370);
			let mut fTemp371: F32 = mydsp_faustpower2_f(2.0 * self.fRec302[0] + -1.0);
			self.fVec35[0] = fTemp371;
			let mut fTemp372: F32 = fTemp315 * (fTemp371 - self.fVec35[1]) / fTemp369;
			self.fVec36[(self.IOTA0 & 4095) as usize] = fTemp372;
			let mut fTemp373: F32 = F32::max(0.0, F32::min(2047.0, self.fConst14 / fTemp368));
			let mut iTemp374: i32 = ((fTemp373) as i32);
			let mut fTemp375: F32 = F32::floor(fTemp373);
			self.fRec300[0] = 0.999 * self.fRec300[1] - self.fConst15 * (self.fVec36[((i32::wrapping_sub(self.IOTA0, iTemp374)) & 4095) as usize] * (fTemp375 + (1.0 - fTemp373)) - fTemp372 + (fTemp373 - fTemp375) * self.fVec36[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp374, 1))) & 4095) as usize]);
			self.fRec304[0] = self.fConst2 * self.fRec304[1] + fSlow40;
			let mut fTemp376: F32 = F32::max(self.fRec304[0], 23.44895);
			let mut fTemp377: F32 = F32::max(2e+01, F32::abs(fTemp376));
			let mut fTemp378: F32 = self.fRec305[1] + self.fConst13 * fTemp377;
			self.fRec305[0] = fTemp378 - F32::floor(fTemp378);
			let mut fTemp379: F32 = mydsp_faustpower2_f(2.0 * self.fRec305[0] + -1.0);
			self.fVec37[0] = fTemp379;
			let mut fTemp380: F32 = fTemp315 * (fTemp379 - self.fVec37[1]) / fTemp377;
			self.fVec38[(self.IOTA0 & 4095) as usize] = fTemp380;
			let mut fTemp381: F32 = F32::max(0.0, F32::min(2047.0, self.fConst14 / fTemp376));
			let mut iTemp382: i32 = ((fTemp381) as i32);
			let mut fTemp383: F32 = F32::floor(fTemp381);
			self.fRec303[0] = 0.999 * self.fRec303[1] + self.fConst15 * (fTemp380 - self.fVec38[((i32::wrapping_sub(self.IOTA0, iTemp382)) & 4095) as usize] * (fTemp383 + (1.0 - fTemp381)) - (fTemp381 - fTemp383) * self.fVec38[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp382, 1))) & 4095) as usize]);
			self.fRec306[0] = fSlow41 + self.fConst2 * self.fRec306[1];
			self.fRec308[0] = self.fConst2 * self.fRec308[1] + fSlow43;
			let mut fTemp384: F32 = F32::max(self.fRec308[0], 23.44895);
			let mut fTemp385: F32 = F32::max(2e+01, F32::abs(fTemp384));
			let mut fTemp386: F32 = self.fRec309[1] + self.fConst13 * fTemp385;
			self.fRec309[0] = fTemp386 - F32::floor(fTemp386);
			let mut fTemp387: F32 = mydsp_faustpower2_f(2.0 * self.fRec309[0] + -1.0);
			self.fVec39[0] = fTemp387;
			let mut fTemp388: F32 = fTemp315 * (fTemp387 - self.fVec39[1]) / fTemp385;
			self.fVec40[(self.IOTA0 & 4095) as usize] = fTemp388;
			let mut fTemp389: F32 = F32::max(0.0, F32::min(2047.0, self.fConst14 / fTemp384));
			let mut iTemp390: i32 = ((fTemp389) as i32);
			let mut fTemp391: F32 = F32::floor(fTemp389);
			self.fRec307[0] = 0.999 * self.fRec307[1] + self.fConst15 * (fTemp388 - self.fVec40[((i32::wrapping_sub(self.IOTA0, iTemp390)) & 4095) as usize] * (fTemp391 + (1.0 - fTemp389)) - (fTemp389 - fTemp391) * self.fVec40[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp390, 1))) & 4095) as usize]);
			self.fRec311[0] = self.fConst2 * self.fRec311[1] + fSlow44;
			let mut fTemp392: F32 = F32::max(self.fRec311[0], 23.44895);
			let mut fTemp393: F32 = F32::max(2e+01, F32::abs(fTemp392));
			let mut fTemp394: F32 = self.fRec312[1] + self.fConst13 * fTemp393;
			self.fRec312[0] = fTemp394 - F32::floor(fTemp394);
			let mut fTemp395: F32 = mydsp_faustpower2_f(2.0 * self.fRec312[0] + -1.0);
			self.fVec41[0] = fTemp395;
			let mut fTemp396: F32 = fTemp315 * (fTemp395 - self.fVec41[1]) / fTemp393;
			self.fVec42[(self.IOTA0 & 4095) as usize] = fTemp396;
			let mut fTemp397: F32 = F32::max(0.0, F32::min(2047.0, self.fConst14 / fTemp392));
			let mut iTemp398: i32 = ((fTemp397) as i32);
			let mut fTemp399: F32 = F32::floor(fTemp397);
			self.fRec310[0] = 0.999 * self.fRec310[1] - self.fConst15 * (self.fVec42[((i32::wrapping_sub(self.IOTA0, iTemp398)) & 4095) as usize] * (fTemp399 + (1.0 - fTemp397)) - fTemp396 + (fTemp397 - fTemp399) * self.fVec42[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp398, 1))) & 4095) as usize]);
			self.fRec314[0] = self.fConst2 * self.fRec314[1] + fSlow45;
			let mut fTemp400: F32 = F32::max(self.fRec314[0], 23.44895);
			let mut fTemp401: F32 = F32::max(0.0, F32::min(2047.0, self.fConst14 / fTemp400));
			let mut fTemp402: F32 = F32::floor(fTemp401);
			let mut fTemp403: F32 = F32::max(2e+01, F32::abs(fTemp400));
			let mut fTemp404: F32 = self.fRec315[1] + self.fConst13 * fTemp403;
			self.fRec315[0] = fTemp404 - F32::floor(fTemp404);
			let mut fTemp405: F32 = mydsp_faustpower2_f(2.0 * self.fRec315[0] + -1.0);
			self.fVec43[0] = fTemp405;
			let mut fTemp406: F32 = fTemp315 * (fTemp405 - self.fVec43[1]) / fTemp403;
			self.fVec44[(self.IOTA0 & 4095) as usize] = fTemp406;
			let mut iTemp407: i32 = ((fTemp401) as i32);
			self.fRec313[0] = 0.999 * self.fRec313[1] - self.fConst15 * ((fTemp401 - fTemp402) * self.fVec44[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp407, 1))) & 4095) as usize] - (fTemp406 - self.fVec44[((i32::wrapping_sub(self.IOTA0, iTemp407)) & 4095) as usize] * (fTemp402 + (1.0 - fTemp401))));
			self.fRec316[0] = fSlow46 + self.fConst2 * self.fRec316[1];
			let mut fTemp408: F32 = F32::max(-1.0, F32::min(1.0, self.fRec276[0] + self.fConst16 * (self.fRec316[0] * (self.fRec314[0] * self.fRec313[0] + self.fRec311[0] * self.fRec310[0] + self.fRec308[0] * self.fRec307[0]) + self.fRec306[0] * (self.fRec304[0] * self.fRec303[0] + self.fRec301[0] * self.fRec300[0] + self.fRec298[0] * self.fRec297[0]) + self.fRec296[0] * (self.fRec294[0] * self.fRec293[0] + self.fRec291[0] * self.fRec290[0] + self.fRec288[0] * self.fRec287[0]) + self.fRec286[0] * (self.fRec284[0] * self.fRec283[0] + self.fRec281[0] * self.fRec280[0] + self.fRec278[0] * self.fRec277[0])) * F32::powf(1e+01, 0.6666667 * self.fRec276[0])));
			self.fRec317[0] = fSlow47 + self.fConst2 * self.fRec317[1];
			let mut fTemp409: F32 = self.fRec317[0] * fTemp408 * (1.0 - 0.33333334 * mydsp_faustpower2_f(fTemp408));
			self.fRec318[0] = fSlow48 + self.fConst2 * self.fRec318[1];
			let mut fTemp410: F32 = (1.0 - self.fRec318[0]) * (fTemp409 + fTemp310 + fTemp268);
			self.fRec320[0] = fSlow49 + self.fConst2 * self.fRec320[1];
			self.fRec319[(self.IOTA0 & 2097151) as usize] = fTemp268 + fTemp310 + fSlow50 * self.fRec319[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(((F32::min(self.fConst17, F32::max(0.0, self.fConst0 * self.fRec320[0]))) as i32), 1))) & 2097151) as usize] + fTemp409;
			let mut fTemp411: F32 = self.fRec319[(self.IOTA0 & 2097151) as usize] * self.fRec318[0];
			let mut fTemp412: F32 = fTemp411 + fTemp410;
			let mut iTemp413: i32 = i32::wrapping_sub(1, self.iVec0[1]);
			self.fRec331[0] = 0.995 * (self.fRec331[1] + ((i32::wrapping_mul(iTemp413, iSlow54)) as F32)) + fSlow55;
			let mut fTemp414: F32 = self.fRec331[0] + -1.49999;
			let mut fTemp415: F32 = F32::floor(fTemp414);
			self.fRec333[0] = 0.995 * (self.fRec333[1] + ((i32::wrapping_mul(iTemp413, iSlow56)) as F32)) + fSlow57;
			let mut fTemp416: F32 = self.fRec333[0] + -1.49999;
			let mut fTemp417: F32 = F32::floor(fTemp416);
			self.fRec337[0] = 0.9999 * (self.fRec337[1] + ((i32::wrapping_mul(iTemp413, iSlow58)) as F32)) + fSlow59;
			let mut fTemp418: F32 = self.fRec337[0] + -1.49999;
			let mut fTemp419: F32 = F32::floor(fTemp418);
			let mut fTemp420: F32 = 0.760314 * self.fRec321[1] - 0.64955574 * self.fRec334[1];
			let mut fTemp421: F32 = 0.760314 * self.fRec322[1] - 0.64955574 * self.fRec335[1];
			self.fVec45[(self.IOTA0 & 16383) as usize] = 0.0 - 0.70710677 * (fTemp421 - fTemp420);
			let mut fTemp422: F32 = self.fVec45[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp418) as i32))))) & 16383) as usize];
			self.fVec46[0] = fTemp422;
			self.fRec336[0] = self.fVec46[1] - (fTemp419 + (2.0 - self.fRec337[0])) * (self.fRec336[1] - fTemp422) / (self.fRec337[0] - fTemp419);
			self.fRec334[0] = self.fRec336[0];
			self.fRec339[0] = 0.9999 * (self.fRec339[1] + ((i32::wrapping_mul(iTemp413, iSlow60)) as F32)) + fSlow61;
			let mut fTemp423: F32 = self.fRec339[0] + -1.49999;
			let mut fTemp424: F32 = F32::floor(fTemp423);
			let mut fTemp425: F32 = self.fRec339[0] - fTemp424;
			let mut fTemp426: F32 = fTemp424 + (2.0 - self.fRec339[0]);
			self.fVec47[(self.IOTA0 & 16383) as usize] = fTemp420 + fTemp421;
			let mut fTemp427: F32 = self.fVec47[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp423) as i32))))) & 16383) as usize];
			self.fVec48[0] = fTemp427;
			self.fRec338[0] = 0.70710677 * (fTemp426 * fTemp427 / fTemp425 + self.fVec48[1]) - self.fRec338[1] * fTemp426 / fTemp425;
			self.fRec335[0] = self.fRec338[0];
			let mut fTemp428: F32 = 0.760314 * self.fRec334[1] + 0.64955574 * self.fRec321[1];
			self.fRec343[0] = 0.9999 * (self.fRec343[1] + ((i32::wrapping_mul(iTemp413, iSlow62)) as F32)) + fSlow63;
			let mut fTemp429: F32 = self.fRec343[0] + -1.49999;
			let mut fTemp430: F32 = F32::floor(fTemp429);
			let mut fTemp431: F32 = self.fRec343[0] - fTemp430;
			let mut fTemp432: F32 = fTemp430 + (2.0 - self.fRec343[0]);
			let mut fTemp433: F32 = 0.760314 * self.fRec335[1] + 0.64955574 * self.fRec322[1];
			let mut fTemp434: F32 = 0.760314 * fTemp433 - 0.64955574 * self.fRec341[1];
			let mut fTemp435: F32 = 0.760314 * fTemp428 - 0.64955574 * self.fRec340[1];
			self.fVec49[(self.IOTA0 & 16383) as usize] = fTemp435 - fTemp434;
			let mut fTemp436: F32 = self.fVec49[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp429) as i32))))) & 16383) as usize];
			self.fVec50[0] = fTemp436;
			self.fRec342[0] = 0.70710677 * (fTemp432 * fTemp436 / fTemp431 + self.fVec50[1]) - self.fRec342[1] * fTemp432 / fTemp431;
			self.fRec340[0] = self.fRec342[0];
			self.fRec345[0] = 0.9999 * (self.fRec345[1] + ((i32::wrapping_mul(iTemp413, iSlow64)) as F32)) + fSlow65;
			let mut fTemp437: F32 = self.fRec345[0] + -1.49999;
			let mut fTemp438: F32 = F32::floor(fTemp437);
			let mut fTemp439: F32 = self.fRec345[0] - fTemp438;
			let mut fTemp440: F32 = fTemp438 + (2.0 - self.fRec345[0]);
			self.fVec51[(self.IOTA0 & 16383) as usize] = fTemp435 + fTemp434;
			let mut fTemp441: F32 = self.fVec51[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp437) as i32))))) & 16383) as usize];
			self.fVec52[0] = fTemp441;
			self.fRec344[0] = 0.70710677 * (fTemp440 * fTemp441 / fTemp439 + self.fVec52[1]) - self.fRec344[1] * fTemp440 / fTemp439;
			self.fRec341[0] = self.fRec344[0];
			let mut fTemp442: F32 = 0.760314 * self.fRec340[1] + 0.64955574 * fTemp428;
			self.fRec349[0] = 0.9999 * (self.fRec349[1] + ((i32::wrapping_mul(iTemp413, iSlow66)) as F32)) + fSlow67;
			let mut fTemp443: F32 = self.fRec349[0] + -1.49999;
			let mut fTemp444: F32 = F32::floor(fTemp443);
			let mut fTemp445: F32 = self.fRec349[0] - fTemp444;
			let mut fTemp446: F32 = fTemp444 + (2.0 - self.fRec349[0]);
			let mut fTemp447: F32 = 0.760314 * self.fRec341[1] + 0.64955574 * fTemp433;
			let mut fTemp448: F32 = 0.760314 * fTemp447 - 0.64955574 * self.fRec347[1];
			let mut fTemp449: F32 = 0.760314 * fTemp442 - 0.64955574 * self.fRec346[1];
			self.fVec53[(self.IOTA0 & 16383) as usize] = fTemp449 - fTemp448;
			let mut fTemp450: F32 = self.fVec53[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp443) as i32))))) & 16383) as usize];
			self.fVec54[0] = fTemp450;
			self.fRec348[0] = 0.70710677 * (fTemp446 * fTemp450 / fTemp445 + self.fVec54[1]) - self.fRec348[1] * fTemp446 / fTemp445;
			self.fRec346[0] = self.fRec348[0];
			self.fRec351[0] = 0.9999 * (self.fRec351[1] + ((i32::wrapping_mul(iTemp413, iSlow68)) as F32)) + fSlow69;
			let mut fTemp451: F32 = self.fRec351[0] + -1.49999;
			let mut fTemp452: F32 = F32::floor(fTemp451);
			let mut fTemp453: F32 = self.fRec351[0] - fTemp452;
			let mut fTemp454: F32 = fTemp452 + (2.0 - self.fRec351[0]);
			self.fVec55[(self.IOTA0 & 16383) as usize] = fTemp449 + fTemp448;
			let mut fTemp455: F32 = self.fVec55[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp451) as i32))))) & 16383) as usize];
			self.fVec56[0] = fTemp455;
			self.fRec350[0] = 0.70710677 * (fTemp454 * fTemp455 / fTemp453 + self.fVec56[1]) - self.fRec350[1] * fTemp454 / fTemp453;
			self.fRec347[0] = self.fRec350[0];
			let mut fTemp456: F32 = 0.760314 * self.fRec346[1] + 0.64955574 * fTemp442;
			self.fRec355[0] = 0.9999 * (self.fRec355[1] + ((i32::wrapping_mul(iTemp413, iSlow70)) as F32)) + fSlow71;
			let mut fTemp457: F32 = self.fRec355[0] + -1.49999;
			let mut fTemp458: F32 = F32::floor(fTemp457);
			let mut fTemp459: F32 = self.fRec355[0] - fTemp458;
			let mut fTemp460: F32 = fTemp458 + (2.0 - self.fRec355[0]);
			let mut fTemp461: F32 = 0.760314 * self.fRec347[1] + 0.64955574 * fTemp447;
			let mut fTemp462: F32 = 0.760314 * fTemp461 - 0.64955574 * self.fRec353[1];
			let mut fTemp463: F32 = 0.760314 * fTemp456 - 0.64955574 * self.fRec352[1];
			self.fVec57[(self.IOTA0 & 16383) as usize] = fTemp463 - fTemp462;
			let mut fTemp464: F32 = self.fVec57[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp457) as i32))))) & 16383) as usize];
			self.fVec58[0] = fTemp464;
			self.fRec354[0] = 0.70710677 * (fTemp460 * fTemp464 / fTemp459 + self.fVec58[1]) - self.fRec354[1] * fTemp460 / fTemp459;
			self.fRec352[0] = self.fRec354[0];
			self.fRec357[0] = 0.9999 * (self.fRec357[1] + ((i32::wrapping_mul(iTemp413, iSlow72)) as F32)) + fSlow73;
			let mut fTemp465: F32 = self.fRec357[0] + -1.49999;
			let mut fTemp466: F32 = F32::floor(fTemp465);
			let mut fTemp467: F32 = self.fRec357[0] - fTemp466;
			let mut fTemp468: F32 = fTemp466 + (2.0 - self.fRec357[0]);
			self.fVec59[(self.IOTA0 & 16383) as usize] = fTemp463 + fTemp462;
			let mut fTemp469: F32 = self.fVec59[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp465) as i32))))) & 16383) as usize];
			self.fVec60[0] = fTemp469;
			self.fRec356[0] = 0.70710677 * (fTemp468 * fTemp469 / fTemp467 + self.fVec60[1]) - self.fRec356[1] * fTemp468 / fTemp467;
			self.fRec353[0] = self.fRec356[0];
			let mut fTemp470: F32 = 0.760314 * self.fRec352[1] + 0.64955574 * fTemp456;
			self.fRec361[0] = 0.9999 * (self.fRec361[1] + ((i32::wrapping_mul(iTemp413, iSlow74)) as F32)) + fSlow75;
			let mut fTemp471: F32 = self.fRec361[0] + -1.49999;
			let mut fTemp472: F32 = F32::floor(fTemp471);
			let mut fTemp473: F32 = self.fRec361[0] - fTemp472;
			let mut fTemp474: F32 = fTemp472 + (2.0 - self.fRec361[0]);
			let mut fTemp475: F32 = 0.760314 * self.fRec353[1] + 0.64955574 * fTemp461;
			let mut fTemp476: F32 = 0.760314 * fTemp475 - 0.64955574 * self.fRec359[1];
			let mut fTemp477: F32 = 0.760314 * fTemp470 - 0.64955574 * self.fRec358[1];
			self.fVec61[(self.IOTA0 & 16383) as usize] = fTemp477 - fTemp476;
			let mut fTemp478: F32 = self.fVec61[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp471) as i32))))) & 16383) as usize];
			self.fVec62[0] = fTemp478;
			self.fRec360[0] = 0.70710677 * (fTemp474 * fTemp478 / fTemp473 + self.fVec62[1]) - self.fRec360[1] * fTemp474 / fTemp473;
			self.fRec358[0] = self.fRec360[0];
			self.fRec363[0] = 0.9999 * (self.fRec363[1] + ((i32::wrapping_mul(iTemp413, iSlow76)) as F32)) + fSlow77;
			let mut fTemp479: F32 = self.fRec363[0] + -1.49999;
			let mut fTemp480: F32 = F32::floor(fTemp479);
			let mut fTemp481: F32 = self.fRec363[0] - fTemp480;
			let mut fTemp482: F32 = fTemp480 + (2.0 - self.fRec363[0]);
			self.fVec63[(self.IOTA0 & 16383) as usize] = fTemp477 + fTemp476;
			let mut fTemp483: F32 = self.fVec63[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp479) as i32))))) & 16383) as usize];
			self.fVec64[0] = fTemp483;
			self.fRec362[0] = 0.70710677 * (fTemp482 * fTemp483 / fTemp481 + self.fVec64[1]) - self.fRec362[1] * fTemp482 / fTemp481;
			self.fRec359[0] = self.fRec362[0];
			let mut fTemp484: F32 = 0.760314 * self.fRec358[1] + 0.64955574 * fTemp470;
			self.fVec65[(self.IOTA0 & 1023) as usize] = fTemp484;
			self.fRec364[0] = fSlow80 * self.fRec365[1] + fSlow79 * self.fRec364[1];
			self.fRec365[0] = ((iTemp413) as F32) + fSlow79 * self.fRec365[1] - fSlow80 * self.fRec364[1];
			let mut fTemp485: F32 = fSlow81 * (self.fRec365[0] + 1.0);
			let mut fTemp486: F32 = fTemp485 + 3.500005;
			let mut iTemp487: i32 = ((fTemp486) as i32);
			let mut iTemp488: i32 = std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp487, 4)));
			let mut fTemp489: F32 = F32::floor(fTemp486);
			let mut fTemp490: F32 = fTemp485 + (2.0 - fTemp489);
			let mut fTemp491: F32 = fTemp485 + (3.0 - fTemp489);
			let mut fTemp492: F32 = fTemp485 + (4.0 - fTemp489);
			let mut fTemp493: F32 = fTemp492 * fTemp491;
			let mut fTemp494: F32 = fTemp493 * fTemp490;
			let mut fTemp495: F32 = fTemp485 + (1.0 - fTemp489);
			let mut fTemp496: F32 = 0.0 - fTemp495;
			let mut iTemp497: i32 = std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp487, 3)));
			let mut fTemp498: F32 = 0.0 - 0.5 * fTemp495;
			let mut fTemp499: F32 = 0.0 - fTemp490;
			let mut iTemp500: i32 = std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp487, 2)));
			let mut fTemp501: F32 = 0.0 - 0.33333334 * fTemp495;
			let mut fTemp502: F32 = 0.0 - 0.5 * fTemp490;
			let mut fTemp503: F32 = 0.0 - fTemp491;
			let mut iTemp504: i32 = std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp487, 1)));
			let mut fTemp505: F32 = fTemp485 + (5.0 - fTemp489);
			let mut fTemp506: F32 = 0.0 - 0.25 * fTemp495;
			let mut fTemp507: F32 = 0.0 - 0.33333334 * fTemp490;
			let mut fTemp508: F32 = 0.0 - 0.5 * fTemp491;
			let mut fTemp509: F32 = 0.0 - fTemp492;
			let mut iTemp510: i32 = std::cmp::min(512, std::cmp::max(0, iTemp487));
			self.fVec66[(self.IOTA0 & 16383) as usize] = self.fVec65[((i32::wrapping_sub(self.IOTA0, iTemp510)) & 1023) as usize] * fTemp509 * fTemp508 * fTemp507 * fTemp506 + fTemp505 * (self.fVec65[((i32::wrapping_sub(self.IOTA0, iTemp504)) & 1023) as usize] * fTemp503 * fTemp502 * fTemp501 + 0.5 * fTemp492 * self.fVec65[((i32::wrapping_sub(self.IOTA0, iTemp500)) & 1023) as usize] * fTemp499 * fTemp498 + 0.16666667 * fTemp493 * self.fVec65[((i32::wrapping_sub(self.IOTA0, iTemp497)) & 1023) as usize] * fTemp496 + 0.041666668 * fTemp494 * self.fVec65[((i32::wrapping_sub(self.IOTA0, iTemp488)) & 1023) as usize]);
			let mut fTemp511: F32 = self.fVec66[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp416) as i32))))) & 16383) as usize];
			self.fVec67[0] = fTemp511;
			self.fRec332[0] = self.fVec67[1] - (fTemp417 + (2.0 - self.fRec333[0])) * (self.fRec332[1] - fTemp511) / (self.fRec333[0] - fTemp417);
			self.fRec369[0] = 0.9999 * (self.fRec369[1] + ((i32::wrapping_mul(iTemp413, iSlow82)) as F32)) + fSlow83;
			let mut fTemp512: F32 = self.fRec369[0] + -1.49999;
			let mut fTemp513: F32 = F32::floor(fTemp512);
			let mut fTemp514: F32 = self.fRec369[0] - fTemp513;
			let mut fTemp515: F32 = fTemp513 + (2.0 - self.fRec369[0]);
			self.fRec371[0] = 0.995 * (self.fRec371[1] + ((i32::wrapping_mul(iTemp413, iSlow84)) as F32)) + fSlow85;
			let mut fTemp516: F32 = self.fRec371[0] + -1.49999;
			let mut fTemp517: F32 = F32::floor(fTemp516);
			let mut fTemp518: F32 = 0.760314 * self.fRec359[1] + 0.64955574 * fTemp475;
			self.fVec68[(self.IOTA0 & 1023) as usize] = fTemp518;
			let mut fTemp519: F32 = fSlow86 * self.fRec365[0];
			let mut fTemp520: F32 = fSlow81 + fTemp519 + 3.500005;
			let mut iTemp521: i32 = ((fTemp520) as i32);
			let mut fTemp522: F32 = F32::floor(fTemp520);
			let mut fTemp523: F32 = fSlow81 + fTemp519 + (2.0 - fTemp522);
			let mut fTemp524: F32 = fSlow81 + fTemp519 + (3.0 - fTemp522);
			let mut fTemp525: F32 = fSlow81 + fTemp519 + (4.0 - fTemp522);
			let mut fTemp526: F32 = fTemp525 * fTemp524;
			let mut fTemp527: F32 = fSlow81 + fTemp519 + (1.0 - fTemp522);
			self.fVec69[(self.IOTA0 & 16383) as usize] = self.fVec68[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, std::cmp::max(0, iTemp521)))) & 1023) as usize] * (0.0 - fTemp525) * (0.0 - 0.5 * fTemp524) * (0.0 - 0.33333334 * fTemp523) * (0.0 - 0.25 * fTemp527) + (fSlow81 + fTemp519 + (5.0 - fTemp522)) * (self.fVec68[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp521, 1))))) & 1023) as usize] * (0.0 - fTemp524) * (0.0 - 0.5 * fTemp523) * (0.0 - 0.33333334 * fTemp527) + 0.5 * fTemp525 * self.fVec68[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp521, 2))))) & 1023) as usize] * (0.0 - fTemp523) * (0.0 - 0.5 * fTemp527) + 0.16666667 * fTemp526 * self.fVec68[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp521, 3))))) & 1023) as usize] * (0.0 - fTemp527) + 0.041666668 * fTemp526 * fTemp523 * self.fVec68[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp521, 4))))) & 1023) as usize]);
			let mut fTemp528: F32 = self.fVec69[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp516) as i32))))) & 16383) as usize];
			self.fVec70[0] = fTemp528;
			self.fRec370[0] = self.fVec70[1] - (fTemp517 + (2.0 - self.fRec371[0])) * (self.fRec370[1] - fTemp528) / (self.fRec371[0] - fTemp517);
			let mut fTemp529: F32 = 0.760314 * self.fRec370[0] - 0.64955574 * self.fRec367[1];
			let mut fTemp530: F32 = 0.760314 * self.fRec332[0] - 0.64955574 * self.fRec366[1];
			self.fVec71[(self.IOTA0 & 16383) as usize] = fTemp530 - fTemp529;
			let mut fTemp531: F32 = self.fVec71[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp512) as i32))))) & 16383) as usize];
			self.fVec72[0] = fTemp531;
			self.fRec368[0] = 0.70710677 * (fTemp515 * fTemp531 / fTemp514 + self.fVec72[1]) - self.fRec368[1] * fTemp515 / fTemp514;
			self.fRec366[0] = self.fRec368[0];
			self.fRec373[0] = 0.9999 * (self.fRec373[1] + ((i32::wrapping_mul(iTemp413, iSlow87)) as F32)) + fSlow88;
			let mut fTemp532: F32 = self.fRec373[0] + -1.49999;
			let mut fTemp533: F32 = F32::floor(fTemp532);
			let mut fTemp534: F32 = self.fRec373[0] - fTemp533;
			let mut fTemp535: F32 = fTemp533 + (2.0 - self.fRec373[0]);
			self.fVec73[(self.IOTA0 & 16383) as usize] = fTemp530 + fTemp529;
			let mut fTemp536: F32 = self.fVec73[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp532) as i32))))) & 16383) as usize];
			self.fVec74[0] = fTemp536;
			self.fRec372[0] = 0.70710677 * (fTemp535 * fTemp536 / fTemp534 + self.fVec74[1]) - self.fRec372[1] * fTemp535 / fTemp534;
			self.fRec367[0] = self.fRec372[0];
			let mut fTemp537: F32 = 0.760314 * self.fRec366[1] + 0.64955574 * self.fRec332[0];
			self.fRec377[0] = 0.9999 * (self.fRec377[1] + ((i32::wrapping_mul(iTemp413, iSlow89)) as F32)) + fSlow90;
			let mut fTemp538: F32 = self.fRec377[0] + -1.49999;
			let mut fTemp539: F32 = F32::floor(fTemp538);
			let mut fTemp540: F32 = 0.760314 * fTemp537 - 0.64955574 * self.fRec374[1];
			let mut fTemp541: F32 = 0.760314 * self.fRec367[1] + 0.64955574 * self.fRec370[0];
			let mut fTemp542: F32 = 0.760314 * fTemp541 - 0.64955574 * self.fRec375[1];
			self.fVec75[(self.IOTA0 & 16383) as usize] = 0.0 - 0.70710677 * (fTemp542 - fTemp540);
			let mut fTemp543: F32 = self.fVec75[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp538) as i32))))) & 16383) as usize];
			self.fVec76[0] = fTemp543;
			self.fRec376[0] = self.fVec76[1] - (fTemp539 + (2.0 - self.fRec377[0])) * (self.fRec376[1] - fTemp543) / (self.fRec377[0] - fTemp539);
			self.fRec374[0] = self.fRec376[0];
			self.fRec379[0] = 0.9999 * (self.fRec379[1] + ((i32::wrapping_mul(iTemp413, iSlow91)) as F32)) + fSlow92;
			let mut fTemp544: F32 = self.fRec379[0] + -1.49999;
			let mut fTemp545: F32 = F32::floor(fTemp544);
			let mut fTemp546: F32 = self.fRec379[0] - fTemp545;
			let mut fTemp547: F32 = fTemp545 + (2.0 - self.fRec379[0]);
			self.fVec77[(self.IOTA0 & 16383) as usize] = fTemp540 + fTemp542;
			let mut iTemp548: i32 = std::cmp::min(8192, std::cmp::max(0, ((fTemp544) as i32)));
			let mut fTemp549: F32 = self.fVec77[((i32::wrapping_sub(self.IOTA0, iTemp548)) & 16383) as usize];
			self.fVec78[0] = fTemp549;
			self.fRec378[0] = 0.70710677 * (fTemp547 * fTemp549 / fTemp546 + self.fVec78[1]) - fTemp547 * self.fRec378[1] / fTemp546;
			self.fRec375[0] = self.fRec378[0];
			let mut fTemp550: F32 = 0.760314 * self.fRec374[1] + 0.64955574 * fTemp537;
			self.fRec383[0] = 0.9999 * (self.fRec383[1] + ((i32::wrapping_mul(iTemp413, iSlow93)) as F32)) + fSlow94;
			let mut fTemp551: F32 = self.fRec383[0] + -1.49999;
			let mut fTemp552: F32 = F32::floor(fTemp551);
			let mut fTemp553: F32 = self.fRec383[0] - fTemp552;
			let mut fTemp554: F32 = fTemp552 + (2.0 - self.fRec383[0]);
			let mut fTemp555: F32 = 0.760314 * self.fRec375[1] + 0.64955574 * fTemp541;
			let mut fTemp556: F32 = 0.760314 * fTemp555 - 0.64955574 * self.fRec381[1];
			let mut fTemp557: F32 = 0.760314 * fTemp550 - 0.64955574 * self.fRec380[1];
			self.fVec79[(self.IOTA0 & 16383) as usize] = fTemp557 - fTemp556;
			let mut fTemp558: F32 = self.fVec79[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp551) as i32))))) & 16383) as usize];
			self.fVec80[0] = fTemp558;
			self.fRec382[0] = 0.70710677 * (fTemp554 * fTemp558 / fTemp553 + self.fVec80[1]) - self.fRec382[1] * fTemp554 / fTemp553;
			self.fRec380[0] = self.fRec382[0];
			self.fRec385[0] = 0.9999 * (self.fRec385[1] + ((i32::wrapping_mul(iTemp413, iSlow95)) as F32)) + fSlow96;
			let mut fTemp559: F32 = self.fRec385[0] + -1.49999;
			let mut fTemp560: F32 = F32::floor(fTemp559);
			let mut fTemp561: F32 = self.fRec385[0] - fTemp560;
			let mut fTemp562: F32 = fTemp560 + (2.0 - self.fRec385[0]);
			self.fVec81[(self.IOTA0 & 16383) as usize] = fTemp557 + fTemp556;
			let mut iTemp563: i32 = std::cmp::min(8192, std::cmp::max(0, ((fTemp559) as i32)));
			let mut fTemp564: F32 = self.fVec81[((i32::wrapping_sub(self.IOTA0, iTemp563)) & 16383) as usize];
			self.fVec82[0] = fTemp564;
			self.fRec384[0] = 0.70710677 * (fTemp562 * fTemp564 / fTemp561 + self.fVec82[1]) - self.fRec384[1] * fTemp562 / fTemp561;
			self.fRec381[0] = self.fRec384[0];
			let mut fTemp565: F32 = 0.760314 * self.fRec380[1] + 0.64955574 * fTemp550;
			self.fRec389[0] = 0.9999 * (self.fRec389[1] + ((i32::wrapping_mul(iTemp413, iSlow97)) as F32)) + fSlow98;
			let mut fTemp566: F32 = self.fRec389[0] + -1.49999;
			let mut fTemp567: F32 = F32::floor(fTemp566);
			let mut fTemp568: F32 = self.fRec389[0] - fTemp567;
			let mut fTemp569: F32 = fTemp567 + (2.0 - self.fRec389[0]);
			let mut fTemp570: F32 = 0.760314 * self.fRec381[1] + 0.64955574 * fTemp555;
			let mut fTemp571: F32 = 0.760314 * fTemp570 - 0.64955574 * self.fRec387[1];
			let mut fTemp572: F32 = 0.760314 * fTemp565 - 0.64955574 * self.fRec386[1];
			self.fVec83[(self.IOTA0 & 16383) as usize] = fTemp572 - fTemp571;
			let mut iTemp573: i32 = std::cmp::min(8192, std::cmp::max(0, ((fTemp566) as i32)));
			let mut fTemp574: F32 = self.fVec83[((i32::wrapping_sub(self.IOTA0, iTemp573)) & 16383) as usize];
			self.fVec84[0] = fTemp574;
			self.fRec388[0] = 0.70710677 * (fTemp569 * fTemp574 / fTemp568 + self.fVec84[1]) - fTemp569 * self.fRec388[1] / fTemp568;
			self.fRec386[0] = self.fRec388[0];
			self.fRec391[0] = 0.9999 * (self.fRec391[1] + ((i32::wrapping_mul(iTemp413, iSlow99)) as F32)) + fSlow100;
			let mut fTemp575: F32 = self.fRec391[0] + -1.49999;
			let mut fTemp576: F32 = F32::floor(fTemp575);
			let mut fTemp577: F32 = self.fRec391[0] - fTemp576;
			let mut fTemp578: F32 = fTemp576 + (2.0 - self.fRec391[0]);
			self.fVec85[(self.IOTA0 & 16383) as usize] = fTemp572 + fTemp571;
			let mut iTemp579: i32 = std::cmp::min(8192, std::cmp::max(0, ((fTemp575) as i32)));
			let mut fTemp580: F32 = self.fVec85[((i32::wrapping_sub(self.IOTA0, iTemp579)) & 16383) as usize];
			self.fVec86[0] = fTemp580;
			self.fRec390[0] = 0.70710677 * (fTemp578 * fTemp580 / fTemp577 + self.fVec86[1]) - fTemp578 * self.fRec390[1] / fTemp577;
			self.fRec387[0] = self.fRec390[0];
			let mut fTemp581: F32 = 0.760314 * self.fRec386[1] + 0.64955574 * fTemp565;
			self.fRec395[0] = 0.9999 * (self.fRec395[1] + ((i32::wrapping_mul(iTemp413, iSlow101)) as F32)) + fSlow102;
			let mut fTemp582: F32 = self.fRec395[0] + -1.49999;
			let mut fTemp583: F32 = F32::floor(fTemp582);
			let mut fTemp584: F32 = self.fRec395[0] - fTemp583;
			let mut fTemp585: F32 = fTemp583 + (2.0 - self.fRec395[0]);
			let mut fTemp586: F32 = 0.760314 * self.fRec387[1] + 0.64955574 * fTemp570;
			let mut fTemp587: F32 = 0.760314 * fTemp586 - 0.64955574 * self.fRec393[1];
			let mut fTemp588: F32 = 0.760314 * fTemp581 - 0.64955574 * self.fRec392[1];
			self.fVec87[(self.IOTA0 & 16383) as usize] = fTemp588 - fTemp587;
			let mut fTemp589: F32 = self.fVec87[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp582) as i32))))) & 16383) as usize];
			self.fVec88[0] = fTemp589;
			self.fRec394[0] = 0.70710677 * (fTemp585 * fTemp589 / fTemp584 + self.fVec88[1]) - self.fRec394[1] * fTemp585 / fTemp584;
			self.fRec392[0] = self.fRec394[0];
			self.fRec397[0] = 0.9999 * (self.fRec397[1] + ((i32::wrapping_mul(iTemp413, iSlow103)) as F32)) + fSlow104;
			let mut fTemp590: F32 = self.fRec397[0] + -1.49999;
			let mut fTemp591: F32 = F32::floor(fTemp590);
			let mut fTemp592: F32 = self.fRec397[0] - fTemp591;
			let mut fTemp593: F32 = fTemp591 + (2.0 - self.fRec397[0]);
			self.fVec89[(self.IOTA0 & 16383) as usize] = fTemp588 + fTemp587;
			let mut iTemp594: i32 = std::cmp::min(8192, std::cmp::max(0, ((fTemp590) as i32)));
			let mut fTemp595: F32 = self.fVec89[((i32::wrapping_sub(self.IOTA0, iTemp594)) & 16383) as usize];
			self.fVec90[0] = fTemp595;
			self.fRec396[0] = 0.70710677 * (fTemp593 * fTemp595 / fTemp592 + self.fVec90[1]) - self.fRec396[1] * fTemp593 / fTemp592;
			self.fRec393[0] = self.fRec396[0];
			let mut fTemp596: F32 = 0.760314 * self.fRec392[1] + 0.64955574 * fTemp581;
			self.fVec91[(self.IOTA0 & 16383) as usize] = fTemp596;
			let mut fTemp597: F32 = fSlow81 * (self.fRec364[0] + 1.0);
			let mut fTemp598: F32 = fTemp597 + 3.500005;
			let mut iTemp599: i32 = ((fTemp598) as i32);
			let mut iTemp600: i32 = std::cmp::max(0, i32::wrapping_add(iTemp599, 4));
			let mut fTemp601: F32 = F32::floor(fTemp598);
			let mut fTemp602: F32 = fTemp597 + (2.0 - fTemp601);
			let mut fTemp603: F32 = fTemp597 + (3.0 - fTemp601);
			let mut fTemp604: F32 = fTemp597 + (4.0 - fTemp601);
			let mut fTemp605: F32 = fTemp604 * fTemp603;
			let mut fTemp606: F32 = fTemp605 * fTemp602;
			let mut fTemp607: F32 = fTemp597 + (1.0 - fTemp601);
			let mut fTemp608: F32 = 0.0 - fTemp607;
			let mut iTemp609: i32 = std::cmp::max(0, i32::wrapping_add(iTemp599, 3));
			let mut fTemp610: F32 = 0.0 - 0.5 * fTemp607;
			let mut fTemp611: F32 = 0.0 - fTemp602;
			let mut iTemp612: i32 = std::cmp::max(0, i32::wrapping_add(iTemp599, 2));
			let mut fTemp613: F32 = 0.0 - 0.33333334 * fTemp607;
			let mut fTemp614: F32 = 0.0 - 0.5 * fTemp602;
			let mut fTemp615: F32 = 0.0 - fTemp603;
			let mut iTemp616: i32 = std::cmp::max(0, i32::wrapping_add(iTemp599, 1));
			let mut fTemp617: F32 = fTemp597 + (5.0 - fTemp601);
			let mut fTemp618: F32 = 0.0 - 0.25 * fTemp607;
			let mut fTemp619: F32 = 0.0 - 0.33333334 * fTemp602;
			let mut fTemp620: F32 = 0.0 - 0.5 * fTemp603;
			let mut fTemp621: F32 = 0.0 - fTemp604;
			let mut iTemp622: i32 = std::cmp::max(0, iTemp599);
			self.fVec92[(self.IOTA0 & 16383) as usize] = self.fVec91[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, iTemp622))) & 16383) as usize] * fTemp621 * fTemp620 * fTemp619 * fTemp618 + fTemp617 * (self.fVec91[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, iTemp616))) & 16383) as usize] * fTemp615 * fTemp614 * fTemp613 + 0.5 * fTemp604 * self.fVec91[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, iTemp612))) & 16383) as usize] * fTemp611 * fTemp610 + 0.16666667 * fTemp605 * self.fVec91[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, iTemp609))) & 16383) as usize] * fTemp608 + 0.041666668 * fTemp606 * self.fVec91[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, iTemp600))) & 16383) as usize]);
			let mut fTemp623: F32 = self.fVec92[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp414) as i32))))) & 16383) as usize];
			self.fVec93[0] = fTemp623;
			self.fRec330[0] = self.fVec93[1] - (fTemp415 + (2.0 - self.fRec331[0])) * (self.fRec330[1] - fTemp623) / (self.fRec331[0] - fTemp415);
			self.fRec329[0] = 0.0 - self.fConst39 * (self.fConst37 * self.fRec329[1] - (self.fRec330[0] + self.fRec330[1]));
			self.fRec328[0] = self.fRec329[0] - self.fConst35 * (self.fConst34 * self.fRec328[2] + self.fConst30 * self.fRec328[1]);
			self.fRec327[0] = self.fConst35 * (self.fRec328[2] + self.fRec328[0] + 2.0 * self.fRec328[1]) - self.fConst33 * (self.fConst32 * self.fRec327[2] + self.fConst30 * self.fRec327[1]);
			let mut fTemp624: F32 = self.fRec327[2] + self.fRec327[0] + 2.0 * self.fRec327[1];
			self.fVec94[0] = fTemp624;
			self.fRec326[0] = 0.0 - self.fConst42 * (self.fConst40 * self.fRec326[1] - self.fConst33 * (fTemp624 + self.fVec94[1]));
			self.fRec325[0] = self.fRec326[0] - self.fConst26 * (self.fConst25 * self.fRec325[2] + self.fConst21 * self.fRec325[1]);
			self.fRec324[0] = self.fConst26 * (self.fRec325[2] + self.fRec325[0] + 2.0 * self.fRec325[1]) - self.fConst24 * (self.fConst23 * self.fRec324[2] + self.fConst21 * self.fRec324[1]);
			self.fRec400[0] = self.fConst33 * (self.fConst44 * fTemp624 + self.fConst45 * self.fVec94[1]) - self.fConst43 * self.fRec400[1];
			self.fRec399[0] = self.fRec400[0] - self.fConst26 * (self.fConst25 * self.fRec399[2] + self.fConst21 * self.fRec399[1]);
			self.fRec398[0] = self.fConst26 * (self.fConst46 * self.fRec399[1] + self.fConst20 * self.fRec399[0] + self.fConst20 * self.fRec399[2]) - self.fConst24 * (self.fConst23 * self.fRec398[2] + self.fConst21 * self.fRec398[1]);
			let mut fTemp625: F32 = self.fConst21 * self.fRec401[1];
			self.fRec404[0] = self.fConst49 * self.fRec330[1] - self.fConst39 * (self.fConst37 * self.fRec404[1] - self.fConst31 * self.fRec330[0]);
			self.fRec403[0] = self.fRec404[0] - self.fConst35 * (self.fConst34 * self.fRec403[2] + self.fConst30 * self.fRec403[1]);
			self.fRec402[0] = self.fConst35 * (self.fConst29 * self.fRec403[0] + self.fConst50 * self.fRec403[1] + self.fConst29 * self.fRec403[2]) - self.fConst33 * (self.fConst32 * self.fRec402[2] + self.fConst30 * self.fRec402[1]);
			self.fRec401[0] = self.fConst33 * (self.fConst50 * self.fRec402[1] + self.fConst29 * self.fRec402[0] + self.fConst29 * self.fRec402[2]) - self.fConst48 * (self.fConst47 * self.fRec401[2] + fTemp625);
			let mut fTemp626: F32 = fTemp410 + fTemp411 + fSlow105 * (self.fRec401[2] + self.fConst48 * (fTemp625 + self.fConst47 * self.fRec401[0]) + self.fConst24 * (self.fConst46 * self.fRec398[1] + self.fConst20 * self.fRec398[0] + self.fConst20 * self.fRec398[2] + self.fRec324[2] + self.fRec324[0] + 2.0 * self.fRec324[1]));
			self.fVec95[(self.IOTA0 & 1023) as usize] = fTemp626;
			self.fRec323[0] = fSlow106 * self.fRec323[1] + fSlow107 * (fTemp621 * fTemp620 * fTemp619 * fTemp618 * self.fVec95[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, iTemp622))) & 1023) as usize] + fTemp617 * (fTemp615 * fTemp614 * fTemp613 * self.fVec95[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, iTemp616))) & 1023) as usize] + 0.5 * fTemp604 * fTemp611 * fTemp610 * self.fVec95[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, iTemp612))) & 1023) as usize] + 0.16666667 * fTemp605 * fTemp608 * self.fVec95[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, iTemp609))) & 1023) as usize] + 0.041666668 * fTemp606 * self.fVec95[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, iTemp600))) & 1023) as usize]));
			self.fRec416[0] = 0.995 * (self.fRec416[1] + ((i32::wrapping_mul(iTemp413, iSlow110)) as F32)) + fSlow111;
			let mut fTemp627: F32 = self.fRec416[0] + -1.49999;
			let mut fTemp628: F32 = F32::floor(fTemp627);
			let mut fTemp629: F32 = 0.760314 * self.fRec393[1] + 0.64955574 * fTemp586;
			self.fVec96[(self.IOTA0 & 16383) as usize] = fTemp629;
			let mut fTemp630: F32 = fSlow86 * self.fRec364[0];
			let mut fTemp631: F32 = fSlow81 + fTemp630 + 3.500005;
			let mut iTemp632: i32 = ((fTemp631) as i32);
			let mut fTemp633: F32 = F32::floor(fTemp631);
			let mut fTemp634: F32 = fSlow81 + fTemp630 + (2.0 - fTemp633);
			let mut fTemp635: F32 = fSlow81 + fTemp630 + (3.0 - fTemp633);
			let mut fTemp636: F32 = fSlow81 + fTemp630 + (4.0 - fTemp633);
			let mut fTemp637: F32 = fTemp636 * fTemp635;
			let mut fTemp638: F32 = fSlow81 + fTemp630 + (1.0 - fTemp633);
			self.fVec97[(self.IOTA0 & 16383) as usize] = self.fVec96[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, iTemp632)))) & 16383) as usize] * (0.0 - fTemp636) * (0.0 - 0.5 * fTemp635) * (0.0 - 0.33333334 * fTemp634) * (0.0 - 0.25 * fTemp638) + (fSlow81 + fTemp630 + (5.0 - fTemp633)) * (self.fVec96[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, i32::wrapping_add(iTemp632, 1))))) & 16383) as usize] * (0.0 - fTemp635) * (0.0 - 0.5 * fTemp634) * (0.0 - 0.33333334 * fTemp638) + 0.5 * fTemp636 * self.fVec96[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, i32::wrapping_add(iTemp632, 2))))) & 16383) as usize] * (0.0 - fTemp634) * (0.0 - 0.5 * fTemp638) + 0.16666667 * fTemp637 * self.fVec96[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, i32::wrapping_add(iTemp632, 3))))) & 16383) as usize] * (0.0 - fTemp638) + 0.041666668 * fTemp637 * fTemp634 * self.fVec96[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, i32::wrapping_add(iTemp632, 4))))) & 16383) as usize]);
			let mut fTemp639: F32 = self.fVec97[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp627) as i32))))) & 16383) as usize];
			self.fVec98[0] = fTemp639;
			self.fRec415[0] = self.fVec98[1] - (fTemp628 + (2.0 - self.fRec416[0])) * (self.fRec415[1] - fTemp639) / (self.fRec416[0] - fTemp628);
			self.fRec414[0] = self.fConst39 * (self.fRec415[0] + self.fRec415[1] - self.fConst37 * self.fRec414[1]);
			self.fRec413[0] = self.fRec414[0] - self.fConst35 * (self.fConst34 * self.fRec413[2] + self.fConst30 * self.fRec413[1]);
			self.fRec412[0] = self.fConst35 * (self.fRec413[2] + self.fRec413[0] + 2.0 * self.fRec413[1]) - self.fConst33 * (self.fConst32 * self.fRec412[2] + self.fConst30 * self.fRec412[1]);
			let mut fTemp640: F32 = self.fRec412[2] + self.fRec412[0] + 2.0 * self.fRec412[1];
			self.fVec99[0] = fTemp640;
			self.fRec411[0] = 0.0 - self.fConst42 * (self.fConst40 * self.fRec411[1] - self.fConst33 * (fTemp640 + self.fVec99[1]));
			self.fRec410[0] = self.fRec411[0] - self.fConst26 * (self.fConst25 * self.fRec410[2] + self.fConst21 * self.fRec410[1]);
			self.fRec409[0] = self.fConst26 * (self.fRec410[2] + self.fRec410[0] + 2.0 * self.fRec410[1]) - self.fConst24 * (self.fConst23 * self.fRec409[2] + self.fConst21 * self.fRec409[1]);
			self.fRec419[0] = self.fConst33 * (self.fConst45 * self.fVec99[1] + self.fConst44 * fTemp640) - self.fConst43 * self.fRec419[1];
			self.fRec418[0] = self.fRec419[0] - self.fConst26 * (self.fConst25 * self.fRec418[2] + self.fConst21 * self.fRec418[1]);
			self.fRec417[0] = self.fConst26 * (self.fConst20 * self.fRec418[0] + self.fConst46 * self.fRec418[1] + self.fConst20 * self.fRec418[2]) - self.fConst24 * (self.fConst23 * self.fRec417[2] + self.fConst21 * self.fRec417[1]);
			let mut fTemp641: F32 = self.fConst21 * self.fRec420[1];
			self.fRec423[0] = self.fConst49 * self.fRec415[1] - self.fConst39 * (self.fConst37 * self.fRec423[1] - self.fConst31 * self.fRec415[0]);
			self.fRec422[0] = self.fRec423[0] - self.fConst35 * (self.fConst34 * self.fRec422[2] + self.fConst30 * self.fRec422[1]);
			self.fRec421[0] = self.fConst35 * (self.fConst29 * self.fRec422[0] + self.fConst50 * self.fRec422[1] + self.fConst29 * self.fRec422[2]) - self.fConst33 * (self.fConst32 * self.fRec421[2] + self.fConst30 * self.fRec421[1]);
			self.fRec420[0] = self.fConst33 * (self.fConst50 * self.fRec421[1] + self.fConst29 * self.fRec421[0] + self.fConst29 * self.fRec421[2]) - self.fConst48 * (self.fConst47 * self.fRec420[2] + fTemp641);
			let mut fTemp642: F32 = fTemp412 + fSlow105 * (self.fRec420[2] + self.fConst48 * (fTemp641 + self.fConst47 * self.fRec420[0]) + self.fConst24 * (self.fConst20 * self.fRec417[0] + self.fConst46 * self.fRec417[1] + self.fConst20 * self.fRec417[2] + self.fRec409[2] + self.fRec409[0] + 2.0 * self.fRec409[1]));
			self.fVec100[(self.IOTA0 & 1023) as usize] = fTemp642;
			self.fRec408[0] = fSlow106 * self.fRec408[1] + fSlow107 * (fTemp509 * fTemp508 * fTemp507 * fTemp506 * self.fVec100[((i32::wrapping_sub(self.IOTA0, iTemp510)) & 1023) as usize] + fTemp505 * (fTemp503 * fTemp502 * fTemp501 * self.fVec100[((i32::wrapping_sub(self.IOTA0, iTemp504)) & 1023) as usize] + 0.5 * fTemp492 * fTemp499 * fTemp498 * self.fVec100[((i32::wrapping_sub(self.IOTA0, iTemp500)) & 1023) as usize] + 0.16666667 * fTemp493 * fTemp496 * self.fVec100[((i32::wrapping_sub(self.IOTA0, iTemp497)) & 1023) as usize] + 0.041666668 * fTemp494 * self.fVec100[((i32::wrapping_sub(self.IOTA0, iTemp488)) & 1023) as usize]));
			let mut fTemp643: F32 = fSlow112 * self.fRec408[0] - fSlow109 * self.fRec406[1];
			let mut fTemp644: F32 = fSlow112 * self.fRec323[0] - fSlow109 * self.fRec405[1];
			self.fVec101[(self.IOTA0 & 16383) as usize] = fTemp644 - fTemp643;
			let mut fTemp645: F32 = self.fVec101[((i32::wrapping_sub(self.IOTA0, iTemp548)) & 16383) as usize];
			self.fVec102[0] = fTemp645;
			self.fRec407[0] = 0.70710677 * (fTemp547 * fTemp645 / fTemp546 + self.fVec102[1]) - self.fRec407[1] * fTemp547 / fTemp546;
			self.fRec405[0] = self.fRec407[0];
			self.fRec425[0] = 0.9999 * (self.fRec425[1] + ((i32::wrapping_mul(iTemp413, iSlow113)) as F32)) + fSlow114;
			let mut fTemp646: F32 = self.fRec425[0] + -1.49999;
			let mut fTemp647: F32 = F32::floor(fTemp646);
			let mut fTemp648: F32 = self.fRec425[0] - fTemp647;
			let mut fTemp649: F32 = fTemp647 + (2.0 - self.fRec425[0]);
			self.fVec103[(self.IOTA0 & 16383) as usize] = fTemp644 + fTemp643;
			let mut fTemp650: F32 = self.fVec103[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp646) as i32))))) & 16383) as usize];
			self.fVec104[0] = fTemp650;
			self.fRec424[0] = 0.70710677 * (fTemp649 * fTemp650 / fTemp648 + self.fVec104[1]) - self.fRec424[1] * fTemp649 / fTemp648;
			self.fRec406[0] = self.fRec424[0];
			let mut fTemp651: F32 = fSlow112 * self.fRec405[1] + fSlow109 * self.fRec323[0];
			let mut fTemp652: F32 = fSlow112 * self.fRec406[1] + fSlow109 * self.fRec408[0];
			let mut fTemp653: F32 = fSlow112 * fTemp652 - fSlow109 * self.fRec427[1];
			let mut fTemp654: F32 = fSlow112 * fTemp651 - fSlow109 * self.fRec426[1];
			self.fVec105[(self.IOTA0 & 16383) as usize] = fTemp654 - fTemp653;
			let mut fTemp655: F32 = self.fVec105[((i32::wrapping_sub(self.IOTA0, iTemp573)) & 16383) as usize];
			self.fVec106[0] = fTemp655;
			self.fRec428[0] = 0.70710677 * (fTemp569 * fTemp655 / fTemp568 + self.fVec106[1]) - self.fRec428[1] * fTemp569 / fTemp568;
			self.fRec426[0] = self.fRec428[0];
			self.fVec107[(self.IOTA0 & 16383) as usize] = fTemp654 + fTemp653;
			let mut fTemp656: F32 = self.fVec107[((i32::wrapping_sub(self.IOTA0, iTemp563)) & 16383) as usize];
			self.fVec108[0] = fTemp656;
			self.fRec429[0] = 0.70710677 * (fTemp562 * fTemp656 / fTemp561 + self.fVec108[1]) - fTemp562 * self.fRec429[1] / fTemp561;
			self.fRec427[0] = self.fRec429[0];
			let mut fTemp657: F32 = fSlow112 * self.fRec426[1] + fSlow109 * fTemp651;
			let mut fTemp658: F32 = fSlow112 * fTemp657 - fSlow109 * self.fRec430[1];
			let mut fTemp659: F32 = fSlow112 * self.fRec427[1] + fSlow109 * fTemp652;
			let mut fTemp660: F32 = fSlow112 * fTemp659 - fSlow109 * self.fRec431[1];
			self.fVec109[(self.IOTA0 & 16383) as usize] = 0.0 - 0.70710677 * (fTemp660 - fTemp658);
			let mut fTemp661: F32 = self.fVec109[((i32::wrapping_sub(self.IOTA0, iTemp579)) & 16383) as usize];
			self.fVec110[0] = fTemp661;
			self.fRec432[0] = self.fVec110[1] - fTemp578 * (self.fRec432[1] - fTemp661) / fTemp577;
			self.fRec430[0] = self.fRec432[0];
			self.fRec434[0] = 0.9999 * (self.fRec434[1] + ((i32::wrapping_mul(iTemp413, iSlow115)) as F32)) + fSlow116;
			let mut fTemp662: F32 = self.fRec434[0] + -1.49999;
			let mut fTemp663: F32 = F32::floor(fTemp662);
			let mut fTemp664: F32 = self.fRec434[0] - fTemp663;
			let mut fTemp665: F32 = fTemp663 + (2.0 - self.fRec434[0]);
			self.fVec111[(self.IOTA0 & 16383) as usize] = fTemp658 + fTemp660;
			let mut fTemp666: F32 = self.fVec111[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp662) as i32))))) & 16383) as usize];
			self.fVec112[0] = fTemp666;
			self.fRec433[0] = 0.70710677 * (fTemp665 * fTemp666 / fTemp664 + self.fVec112[1]) - self.fRec433[1] * fTemp665 / fTemp664;
			self.fRec431[0] = self.fRec433[0];
			let mut fTemp667: F32 = fSlow112 * self.fRec430[1] + fSlow109 * fTemp657;
			self.fRec438[0] = 0.9999 * (self.fRec438[1] + ((i32::wrapping_mul(iTemp413, iSlow117)) as F32)) + fSlow118;
			let mut fTemp668: F32 = self.fRec438[0] + -1.49999;
			let mut fTemp669: F32 = F32::floor(fTemp668);
			let mut fTemp670: F32 = self.fRec438[0] - fTemp669;
			let mut fTemp671: F32 = fTemp669 + (2.0 - self.fRec438[0]);
			let mut fTemp672: F32 = fSlow112 * self.fRec431[1] + fSlow109 * fTemp659;
			let mut fTemp673: F32 = fSlow112 * fTemp672 - fSlow109 * self.fRec436[1];
			let mut fTemp674: F32 = fSlow112 * fTemp667 - fSlow109 * self.fRec435[1];
			self.fVec113[(self.IOTA0 & 16383) as usize] = fTemp674 - fTemp673;
			let mut fTemp675: F32 = self.fVec113[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp668) as i32))))) & 16383) as usize];
			self.fVec114[0] = fTemp675;
			self.fRec437[0] = 0.70710677 * (fTemp671 * fTemp675 / fTemp670 + self.fVec114[1]) - self.fRec437[1] * fTemp671 / fTemp670;
			self.fRec435[0] = self.fRec437[0];
			self.fVec115[(self.IOTA0 & 16383) as usize] = fTemp674 + fTemp673;
			let mut fTemp676: F32 = self.fVec115[((i32::wrapping_sub(self.IOTA0, iTemp594)) & 16383) as usize];
			self.fVec116[0] = fTemp676;
			self.fRec439[0] = 0.70710677 * (fTemp593 * fTemp676 / fTemp592 + self.fVec116[1]) - fTemp593 * self.fRec439[1] / fTemp592;
			self.fRec436[0] = self.fRec439[0];
			self.fRec321[0] = fSlow112 * self.fRec435[1] + fSlow109 * fTemp667;
			self.fRec322[0] = fSlow112 * self.fRec436[1] + fSlow109 * fTemp672;
			self.fRec440[0] = fSlow119 + self.fConst2 * self.fRec440[1];
			let mut fTemp677: F32 = self.fRec440[0] * (fSlow51 * (self.fRec321[0] + self.fRec322[0]) + fSlow52 * fTemp412);
			*output0 = fTemp677;
			*output1 = fTemp677;
			self.iVec0[1] = self.iVec0[0];
			self.fRec0[1] = self.fRec0[0];
			self.fRec3[1] = self.fRec3[0];
			self.fRec1[1] = self.fRec1[0];
			self.IOTA0 = i32::wrapping_add(self.IOTA0, 1);
			self.fRec40[1] = self.fRec40[0];
			self.fRec39[1] = self.fRec39[0];
			self.fRec35[1] = self.fRec35[0];
			self.fRec41[1] = self.fRec41[0];
			self.fRec46[1] = self.fRec46[0];
			self.iVec1[1] = self.iVec1[0];
			self.iRec47[1] = self.iRec47[0];
			self.fRec44[1] = self.fRec44[0];
			self.fRec43[1] = self.fRec43[0];
			for j0 in (1..=3).rev() {
				self.fRec48[(j0) as usize] = self.fRec48[(i32::wrapping_sub(j0, 1)) as usize];
			}
			self.fVec2[1] = self.fVec2[0];
			self.fVec3[1] = self.fVec3[0];
			self.iRec50[1] = self.iRec50[0];
			self.iRec52[1] = self.iRec52[0];
			self.fRec51[2] = self.fRec51[1];
			self.fRec51[1] = self.fRec51[0];
			self.fVec4[2] = self.fVec4[1];
			self.fVec4[1] = self.fVec4[0];
			self.fRec31[1] = self.fRec31[0];
			self.fRec27[1] = self.fRec27[0];
			self.fRec25[1] = self.fRec25[0];
			for j1 in (1..=3).rev() {
				self.fRec21[(j1) as usize] = self.fRec21[(i32::wrapping_sub(j1, 1)) as usize];
			}
			self.fRec16[1] = self.fRec16[0];
			self.fRec10[1] = self.fRec10[0];
			self.fRec9[1] = self.fRec9[0];
			self.fRec8[1] = self.fRec8[0];
			self.fRec6[1] = self.fRec6[0];
			self.fRec5[2] = self.fRec5[1];
			self.fRec5[1] = self.fRec5[0];
			self.fRec4[2] = self.fRec4[1];
			self.fRec4[1] = self.fRec4[0];
			self.fRec53[1] = self.fRec53[0];
			self.fRec89[1] = self.fRec89[0];
			self.fRec85[1] = self.fRec85[0];
			self.fRec90[1] = self.fRec90[0];
			self.fRec95[1] = self.fRec95[0];
			self.iVec5[1] = self.iVec5[0];
			self.iRec96[1] = self.iRec96[0];
			self.fRec93[1] = self.fRec93[0];
			self.fRec92[1] = self.fRec92[0];
			for j2 in (1..=3).rev() {
				self.fRec97[(j2) as usize] = self.fRec97[(i32::wrapping_sub(j2, 1)) as usize];
			}
			self.fVec6[1] = self.fVec6[0];
			self.fVec7[1] = self.fVec7[0];
			self.iRec99[1] = self.iRec99[0];
			self.fRec100[2] = self.fRec100[1];
			self.fRec100[1] = self.fRec100[0];
			self.fVec8[2] = self.fVec8[1];
			self.fVec8[1] = self.fVec8[0];
			self.fRec81[1] = self.fRec81[0];
			self.fRec77[1] = self.fRec77[0];
			self.fRec75[1] = self.fRec75[0];
			for j3 in (1..=3).rev() {
				self.fRec71[(j3) as usize] = self.fRec71[(i32::wrapping_sub(j3, 1)) as usize];
			}
			self.fRec66[1] = self.fRec66[0];
			self.fRec60[1] = self.fRec60[0];
			self.fRec59[1] = self.fRec59[0];
			self.fRec58[1] = self.fRec58[0];
			self.fRec56[1] = self.fRec56[0];
			self.fRec55[2] = self.fRec55[1];
			self.fRec55[1] = self.fRec55[0];
			self.fRec54[2] = self.fRec54[1];
			self.fRec54[1] = self.fRec54[0];
			self.fRec101[1] = self.fRec101[0];
			self.fRec137[1] = self.fRec137[0];
			self.fRec133[1] = self.fRec133[0];
			self.fRec138[1] = self.fRec138[0];
			self.fRec143[1] = self.fRec143[0];
			self.iVec9[1] = self.iVec9[0];
			self.iRec144[1] = self.iRec144[0];
			self.fRec141[1] = self.fRec141[0];
			self.fRec140[1] = self.fRec140[0];
			for j4 in (1..=3).rev() {
				self.fRec145[(j4) as usize] = self.fRec145[(i32::wrapping_sub(j4, 1)) as usize];
			}
			self.fVec10[1] = self.fVec10[0];
			self.fVec11[1] = self.fVec11[0];
			self.iRec147[1] = self.iRec147[0];
			self.fRec148[2] = self.fRec148[1];
			self.fRec148[1] = self.fRec148[0];
			self.fVec12[2] = self.fVec12[1];
			self.fVec12[1] = self.fVec12[0];
			self.fRec129[1] = self.fRec129[0];
			self.fRec125[1] = self.fRec125[0];
			self.fRec123[1] = self.fRec123[0];
			for j5 in (1..=3).rev() {
				self.fRec119[(j5) as usize] = self.fRec119[(i32::wrapping_sub(j5, 1)) as usize];
			}
			self.fRec114[1] = self.fRec114[0];
			self.fRec108[1] = self.fRec108[0];
			self.fRec107[1] = self.fRec107[0];
			self.fRec106[1] = self.fRec106[0];
			self.fRec104[1] = self.fRec104[0];
			self.fRec103[2] = self.fRec103[1];
			self.fRec103[1] = self.fRec103[0];
			self.fRec102[2] = self.fRec102[1];
			self.fRec102[1] = self.fRec102[0];
			self.fRec149[1] = self.fRec149[0];
			self.fRec185[1] = self.fRec185[0];
			self.fRec181[1] = self.fRec181[0];
			self.fRec186[1] = self.fRec186[0];
			self.fRec191[1] = self.fRec191[0];
			self.iVec13[1] = self.iVec13[0];
			self.iRec192[1] = self.iRec192[0];
			self.fRec189[1] = self.fRec189[0];
			self.fRec188[1] = self.fRec188[0];
			for j6 in (1..=3).rev() {
				self.fRec193[(j6) as usize] = self.fRec193[(i32::wrapping_sub(j6, 1)) as usize];
			}
			self.fVec14[1] = self.fVec14[0];
			self.fVec15[1] = self.fVec15[0];
			self.iRec195[1] = self.iRec195[0];
			self.fRec196[2] = self.fRec196[1];
			self.fRec196[1] = self.fRec196[0];
			self.fVec16[2] = self.fVec16[1];
			self.fVec16[1] = self.fVec16[0];
			self.fRec177[1] = self.fRec177[0];
			self.fRec173[1] = self.fRec173[0];
			self.fRec171[1] = self.fRec171[0];
			for j7 in (1..=3).rev() {
				self.fRec167[(j7) as usize] = self.fRec167[(i32::wrapping_sub(j7, 1)) as usize];
			}
			self.fRec162[1] = self.fRec162[0];
			self.fRec156[1] = self.fRec156[0];
			self.fRec155[1] = self.fRec155[0];
			self.fRec154[1] = self.fRec154[0];
			self.fRec152[1] = self.fRec152[0];
			self.fRec151[2] = self.fRec151[1];
			self.fRec151[1] = self.fRec151[0];
			self.fRec150[2] = self.fRec150[1];
			self.fRec150[1] = self.fRec150[0];
			self.fRec197[1] = self.fRec197[0];
			self.fRec233[1] = self.fRec233[0];
			self.fRec229[1] = self.fRec229[0];
			self.fRec234[1] = self.fRec234[0];
			self.fRec239[1] = self.fRec239[0];
			self.iVec17[1] = self.iVec17[0];
			self.iRec240[1] = self.iRec240[0];
			self.fRec237[1] = self.fRec237[0];
			self.fRec236[1] = self.fRec236[0];
			for j8 in (1..=3).rev() {
				self.fRec241[(j8) as usize] = self.fRec241[(i32::wrapping_sub(j8, 1)) as usize];
			}
			self.fVec18[1] = self.fVec18[0];
			self.fVec19[1] = self.fVec19[0];
			self.iRec243[1] = self.iRec243[0];
			self.fRec244[2] = self.fRec244[1];
			self.fRec244[1] = self.fRec244[0];
			self.fVec20[2] = self.fVec20[1];
			self.fVec20[1] = self.fVec20[0];
			self.fRec225[1] = self.fRec225[0];
			self.fRec221[1] = self.fRec221[0];
			self.fRec219[1] = self.fRec219[0];
			for j9 in (1..=3).rev() {
				self.fRec215[(j9) as usize] = self.fRec215[(i32::wrapping_sub(j9, 1)) as usize];
			}
			self.fRec210[1] = self.fRec210[0];
			self.fRec204[1] = self.fRec204[0];
			self.fRec203[1] = self.fRec203[0];
			self.fRec202[1] = self.fRec202[0];
			self.fRec200[1] = self.fRec200[0];
			self.fRec199[2] = self.fRec199[1];
			self.fRec199[1] = self.fRec199[0];
			self.fRec198[2] = self.fRec198[1];
			self.fRec198[1] = self.fRec198[0];
			self.fRec245[1] = self.fRec245[0];
			self.fRec246[1] = self.fRec246[0];
			self.fRec251[1] = self.fRec251[0];
			self.fRec249[1] = self.fRec249[0];
			self.fRec252[1] = self.fRec252[0];
			self.fRec248[2] = self.fRec248[1];
			self.fRec248[1] = self.fRec248[0];
			self.fRec247[2] = self.fRec247[1];
			self.fRec247[1] = self.fRec247[0];
			self.fRec253[1] = self.fRec253[0];
			self.fRec258[1] = self.fRec258[0];
			self.fRec256[1] = self.fRec256[0];
			self.fRec259[1] = self.fRec259[0];
			self.fRec255[2] = self.fRec255[1];
			self.fRec255[1] = self.fRec255[0];
			self.fRec254[2] = self.fRec254[1];
			self.fRec254[1] = self.fRec254[0];
			self.fRec260[1] = self.fRec260[0];
			self.fRec265[1] = self.fRec265[0];
			self.fRec263[1] = self.fRec263[0];
			self.fRec266[1] = self.fRec266[0];
			self.fRec262[2] = self.fRec262[1];
			self.fRec262[1] = self.fRec262[0];
			self.fRec261[2] = self.fRec261[1];
			self.fRec261[1] = self.fRec261[0];
			self.fRec267[1] = self.fRec267[0];
			self.fRec272[1] = self.fRec272[0];
			self.fRec270[1] = self.fRec270[0];
			self.fRec273[1] = self.fRec273[0];
			self.fRec269[2] = self.fRec269[1];
			self.fRec269[1] = self.fRec269[0];
			self.fRec268[2] = self.fRec268[1];
			self.fRec268[1] = self.fRec268[0];
			self.fRec274[1] = self.fRec274[0];
			self.fRec275[1] = self.fRec275[0];
			self.fRec276[1] = self.fRec276[0];
			self.fRec278[1] = self.fRec278[0];
			self.fRec279[1] = self.fRec279[0];
			self.fVec21[1] = self.fVec21[0];
			self.fRec277[1] = self.fRec277[0];
			self.fRec281[1] = self.fRec281[0];
			self.fRec282[1] = self.fRec282[0];
			self.fVec23[1] = self.fVec23[0];
			self.fRec280[1] = self.fRec280[0];
			self.fRec284[1] = self.fRec284[0];
			self.fRec285[1] = self.fRec285[0];
			self.fVec25[1] = self.fVec25[0];
			self.fRec283[1] = self.fRec283[0];
			self.fRec286[1] = self.fRec286[0];
			self.fRec288[1] = self.fRec288[0];
			self.fRec289[1] = self.fRec289[0];
			self.fVec27[1] = self.fVec27[0];
			self.fRec287[1] = self.fRec287[0];
			self.fRec291[1] = self.fRec291[0];
			self.fRec292[1] = self.fRec292[0];
			self.fVec29[1] = self.fVec29[0];
			self.fRec290[1] = self.fRec290[0];
			self.fRec294[1] = self.fRec294[0];
			self.fRec295[1] = self.fRec295[0];
			self.fVec31[1] = self.fVec31[0];
			self.fRec293[1] = self.fRec293[0];
			self.fRec296[1] = self.fRec296[0];
			self.fRec298[1] = self.fRec298[0];
			self.fRec299[1] = self.fRec299[0];
			self.fVec33[1] = self.fVec33[0];
			self.fRec297[1] = self.fRec297[0];
			self.fRec301[1] = self.fRec301[0];
			self.fRec302[1] = self.fRec302[0];
			self.fVec35[1] = self.fVec35[0];
			self.fRec300[1] = self.fRec300[0];
			self.fRec304[1] = self.fRec304[0];
			self.fRec305[1] = self.fRec305[0];
			self.fVec37[1] = self.fVec37[0];
			self.fRec303[1] = self.fRec303[0];
			self.fRec306[1] = self.fRec306[0];
			self.fRec308[1] = self.fRec308[0];
			self.fRec309[1] = self.fRec309[0];
			self.fVec39[1] = self.fVec39[0];
			self.fRec307[1] = self.fRec307[0];
			self.fRec311[1] = self.fRec311[0];
			self.fRec312[1] = self.fRec312[0];
			self.fVec41[1] = self.fVec41[0];
			self.fRec310[1] = self.fRec310[0];
			self.fRec314[1] = self.fRec314[0];
			self.fRec315[1] = self.fRec315[0];
			self.fVec43[1] = self.fVec43[0];
			self.fRec313[1] = self.fRec313[0];
			self.fRec316[1] = self.fRec316[0];
			self.fRec317[1] = self.fRec317[0];
			self.fRec318[1] = self.fRec318[0];
			self.fRec320[1] = self.fRec320[0];
			self.fRec331[1] = self.fRec331[0];
			self.fRec333[1] = self.fRec333[0];
			self.fRec337[1] = self.fRec337[0];
			self.fVec46[1] = self.fVec46[0];
			self.fRec336[1] = self.fRec336[0];
			self.fRec334[1] = self.fRec334[0];
			self.fRec339[1] = self.fRec339[0];
			self.fVec48[1] = self.fVec48[0];
			self.fRec338[1] = self.fRec338[0];
			self.fRec335[1] = self.fRec335[0];
			self.fRec343[1] = self.fRec343[0];
			self.fVec50[1] = self.fVec50[0];
			self.fRec342[1] = self.fRec342[0];
			self.fRec340[1] = self.fRec340[0];
			self.fRec345[1] = self.fRec345[0];
			self.fVec52[1] = self.fVec52[0];
			self.fRec344[1] = self.fRec344[0];
			self.fRec341[1] = self.fRec341[0];
			self.fRec349[1] = self.fRec349[0];
			self.fVec54[1] = self.fVec54[0];
			self.fRec348[1] = self.fRec348[0];
			self.fRec346[1] = self.fRec346[0];
			self.fRec351[1] = self.fRec351[0];
			self.fVec56[1] = self.fVec56[0];
			self.fRec350[1] = self.fRec350[0];
			self.fRec347[1] = self.fRec347[0];
			self.fRec355[1] = self.fRec355[0];
			self.fVec58[1] = self.fVec58[0];
			self.fRec354[1] = self.fRec354[0];
			self.fRec352[1] = self.fRec352[0];
			self.fRec357[1] = self.fRec357[0];
			self.fVec60[1] = self.fVec60[0];
			self.fRec356[1] = self.fRec356[0];
			self.fRec353[1] = self.fRec353[0];
			self.fRec361[1] = self.fRec361[0];
			self.fVec62[1] = self.fVec62[0];
			self.fRec360[1] = self.fRec360[0];
			self.fRec358[1] = self.fRec358[0];
			self.fRec363[1] = self.fRec363[0];
			self.fVec64[1] = self.fVec64[0];
			self.fRec362[1] = self.fRec362[0];
			self.fRec359[1] = self.fRec359[0];
			self.fRec364[1] = self.fRec364[0];
			self.fRec365[1] = self.fRec365[0];
			self.fVec67[1] = self.fVec67[0];
			self.fRec332[1] = self.fRec332[0];
			self.fRec369[1] = self.fRec369[0];
			self.fRec371[1] = self.fRec371[0];
			self.fVec70[1] = self.fVec70[0];
			self.fRec370[1] = self.fRec370[0];
			self.fVec72[1] = self.fVec72[0];
			self.fRec368[1] = self.fRec368[0];
			self.fRec366[1] = self.fRec366[0];
			self.fRec373[1] = self.fRec373[0];
			self.fVec74[1] = self.fVec74[0];
			self.fRec372[1] = self.fRec372[0];
			self.fRec367[1] = self.fRec367[0];
			self.fRec377[1] = self.fRec377[0];
			self.fVec76[1] = self.fVec76[0];
			self.fRec376[1] = self.fRec376[0];
			self.fRec374[1] = self.fRec374[0];
			self.fRec379[1] = self.fRec379[0];
			self.fVec78[1] = self.fVec78[0];
			self.fRec378[1] = self.fRec378[0];
			self.fRec375[1] = self.fRec375[0];
			self.fRec383[1] = self.fRec383[0];
			self.fVec80[1] = self.fVec80[0];
			self.fRec382[1] = self.fRec382[0];
			self.fRec380[1] = self.fRec380[0];
			self.fRec385[1] = self.fRec385[0];
			self.fVec82[1] = self.fVec82[0];
			self.fRec384[1] = self.fRec384[0];
			self.fRec381[1] = self.fRec381[0];
			self.fRec389[1] = self.fRec389[0];
			self.fVec84[1] = self.fVec84[0];
			self.fRec388[1] = self.fRec388[0];
			self.fRec386[1] = self.fRec386[0];
			self.fRec391[1] = self.fRec391[0];
			self.fVec86[1] = self.fVec86[0];
			self.fRec390[1] = self.fRec390[0];
			self.fRec387[1] = self.fRec387[0];
			self.fRec395[1] = self.fRec395[0];
			self.fVec88[1] = self.fVec88[0];
			self.fRec394[1] = self.fRec394[0];
			self.fRec392[1] = self.fRec392[0];
			self.fRec397[1] = self.fRec397[0];
			self.fVec90[1] = self.fVec90[0];
			self.fRec396[1] = self.fRec396[0];
			self.fRec393[1] = self.fRec393[0];
			self.fVec93[1] = self.fVec93[0];
			self.fRec330[1] = self.fRec330[0];
			self.fRec329[1] = self.fRec329[0];
			self.fRec328[2] = self.fRec328[1];
			self.fRec328[1] = self.fRec328[0];
			self.fRec327[2] = self.fRec327[1];
			self.fRec327[1] = self.fRec327[0];
			self.fVec94[1] = self.fVec94[0];
			self.fRec326[1] = self.fRec326[0];
			self.fRec325[2] = self.fRec325[1];
			self.fRec325[1] = self.fRec325[0];
			self.fRec324[2] = self.fRec324[1];
			self.fRec324[1] = self.fRec324[0];
			self.fRec400[1] = self.fRec400[0];
			self.fRec399[2] = self.fRec399[1];
			self.fRec399[1] = self.fRec399[0];
			self.fRec398[2] = self.fRec398[1];
			self.fRec398[1] = self.fRec398[0];
			self.fRec404[1] = self.fRec404[0];
			self.fRec403[2] = self.fRec403[1];
			self.fRec403[1] = self.fRec403[0];
			self.fRec402[2] = self.fRec402[1];
			self.fRec402[1] = self.fRec402[0];
			self.fRec401[2] = self.fRec401[1];
			self.fRec401[1] = self.fRec401[0];
			self.fRec323[1] = self.fRec323[0];
			self.fRec416[1] = self.fRec416[0];
			self.fVec98[1] = self.fVec98[0];
			self.fRec415[1] = self.fRec415[0];
			self.fRec414[1] = self.fRec414[0];
			self.fRec413[2] = self.fRec413[1];
			self.fRec413[1] = self.fRec413[0];
			self.fRec412[2] = self.fRec412[1];
			self.fRec412[1] = self.fRec412[0];
			self.fVec99[1] = self.fVec99[0];
			self.fRec411[1] = self.fRec411[0];
			self.fRec410[2] = self.fRec410[1];
			self.fRec410[1] = self.fRec410[0];
			self.fRec409[2] = self.fRec409[1];
			self.fRec409[1] = self.fRec409[0];
			self.fRec419[1] = self.fRec419[0];
			self.fRec418[2] = self.fRec418[1];
			self.fRec418[1] = self.fRec418[0];
			self.fRec417[2] = self.fRec417[1];
			self.fRec417[1] = self.fRec417[0];
			self.fRec423[1] = self.fRec423[0];
			self.fRec422[2] = self.fRec422[1];
			self.fRec422[1] = self.fRec422[0];
			self.fRec421[2] = self.fRec421[1];
			self.fRec421[1] = self.fRec421[0];
			self.fRec420[2] = self.fRec420[1];
			self.fRec420[1] = self.fRec420[0];
			self.fRec408[1] = self.fRec408[0];
			self.fVec102[1] = self.fVec102[0];
			self.fRec407[1] = self.fRec407[0];
			self.fRec405[1] = self.fRec405[0];
			self.fRec425[1] = self.fRec425[0];
			self.fVec104[1] = self.fVec104[0];
			self.fRec424[1] = self.fRec424[0];
			self.fRec406[1] = self.fRec406[0];
			self.fVec106[1] = self.fVec106[0];
			self.fRec428[1] = self.fRec428[0];
			self.fRec426[1] = self.fRec426[0];
			self.fVec108[1] = self.fVec108[0];
			self.fRec429[1] = self.fRec429[0];
			self.fRec427[1] = self.fRec427[0];
			self.fVec110[1] = self.fVec110[0];
			self.fRec432[1] = self.fRec432[0];
			self.fRec430[1] = self.fRec430[0];
			self.fRec434[1] = self.fRec434[0];
			self.fVec112[1] = self.fVec112[0];
			self.fRec433[1] = self.fRec433[0];
			self.fRec431[1] = self.fRec431[0];
			self.fRec438[1] = self.fRec438[0];
			self.fVec114[1] = self.fVec114[0];
			self.fRec437[1] = self.fRec437[0];
			self.fRec435[1] = self.fRec435[0];
			self.fVec116[1] = self.fVec116[0];
			self.fRec439[1] = self.fRec439[0];
			self.fRec436[1] = self.fRec436[0];
			self.fRec321[1] = self.fRec321[0];
			self.fRec322[1] = self.fRec322[0];
			self.fRec440[1] = self.fRec440[0];
		}
	}

}


}
pub use dsp::mydsp as Instrument;
