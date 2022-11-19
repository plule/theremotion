mod dsp {
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
			table[(i1) as usize] = unsafe { imydspSIG0Wave0[(self.imydspSIG0Wave0_idx) as usize] };
			self.imydspSIG0Wave0_idx = (i32::wrapping_add(1, self.imydspSIG0Wave0_idx)) % 1302;
		}
	}

}


pub fn newmydspSIG0() -> mydspSIG0 { 
	mydspSIG0 {
		imydspSIG0Wave0_idx: 0,
	}
}
fn mydsp_faustpower2_f(value: F32) -> F32 {
	return value * value;
}
static mut itbl0mydspSIG0: [i32;1302] = [0;1302];

#[cfg_attr(feature = "default-boxed", derive(default_boxed::DefaultBoxed))]
#[derive(Debug,Clone)]
pub struct mydsp {
	fHslider0: F32,
	iVec0: [i32;2],
	fSampleRate: i32,
	fConst0: F32,
	fConst1: F32,
	fConst2: F32,
	fHslider1: F32,
	fRec0: [F32;2],
	fConst3: F32,
	fRec2: [F32;2],
	fVec1: [F32;2],
	IOTA0: i32,
	fVec2: [F32;4096],
	fConst4: F32,
	fConst5: F32,
	fRec1: [F32;2],
	fRec4: [F32;2],
	fVec3: [F32;2],
	fVec4: [F32;4096],
	fRec3: [F32;2],
	fRec6: [F32;2],
	fVec5: [F32;2],
	fVec6: [F32;4096],
	fRec5: [F32;2],
	fHslider2: F32,
	fRec7: [F32;2],
	fHslider3: F32,
	fRec8: [F32;2],
	fRec10: [F32;2],
	fVec7: [F32;2],
	fVec8: [F32;4096],
	fRec9: [F32;2],
	fRec12: [F32;2],
	fVec9: [F32;2],
	fVec10: [F32;4096],
	fRec11: [F32;2],
	fRec14: [F32;2],
	fVec11: [F32;2],
	fVec12: [F32;4096],
	fRec13: [F32;2],
	fHslider4: F32,
	fRec15: [F32;2],
	fHslider5: F32,
	fRec16: [F32;2],
	fRec18: [F32;2],
	fVec13: [F32;2],
	fVec14: [F32;4096],
	fRec17: [F32;2],
	fRec20: [F32;2],
	fVec15: [F32;2],
	fVec16: [F32;4096],
	fRec19: [F32;2],
	fRec22: [F32;2],
	fVec17: [F32;2],
	fVec18: [F32;4096],
	fRec21: [F32;2],
	fHslider6: F32,
	fRec23: [F32;2],
	fHslider7: F32,
	fRec24: [F32;2],
	fRec26: [F32;2],
	fVec19: [F32;2],
	fVec20: [F32;4096],
	fRec25: [F32;2],
	fRec28: [F32;2],
	fVec21: [F32;2],
	fVec22: [F32;4096],
	fRec27: [F32;2],
	fRec30: [F32;2],
	fVec23: [F32;2],
	fVec24: [F32;4096],
	fRec29: [F32;2],
	fHslider8: F32,
	fRec31: [F32;2],
	fHslider9: F32,
	fRec32: [F32;2],
	fConst6: F32,
	fHslider10: F32,
	fRec33: [F32;2],
	fHslider11: F32,
	fRec35: [F32;2],
	fHslider12: F32,
	fRec36: [F32;2],
	fConst7: F32,
	fConst8: F32,
	fConst9: F32,
	fRec66: [F32;2],
	fRec70: [F32;2],
	fConst10: F32,
	fConst11: F32,
	fRec75: [F32;2],
	iVec25: [i32;2],
	iConst12: i32,
	iRec76: [i32;2],
	fConst13: F32,
	fRec73: [F32;2],
	fRec72: [F32;2],
	fRec77: [F32;4],
	fRec78: [F32;2048],
	fVec26: [F32;2],
	fConst14: F32,
	fConst15: F32,
	fButton0: F32,
	fVec27: [F32;2],
	iRec79: [i32;2],
	iRec81: [i32;2],
	fRec80: [F32;3],
	fVec28: [F32;3],
	fRec71: [F32;2048],
	fRec62: [F32;2],
	fRec58: [F32;2],
	fRec54: [F32;2048],
	fRec56: [F32;2],
	fHslider13: F32,
	fRec52: [F32;4],
	fRec47: [F32;2],
	fRec43: [F32;2048],
	fRec41: [F32;2],
	fConst16: F32,
	fRec40: [F32;2],
	fRec39: [F32;2],
	fRec37: [F32;2],
	fRec34: [F32;2],
	fHslider14: F32,
	fRec83: [F32;2],
	fRec113: [F32;2],
	fRec117: [F32;2],
	fRec122: [F32;2],
	iVec29: [i32;2],
	iRec123: [i32;2],
	fRec120: [F32;2],
	fRec119: [F32;2],
	fRec124: [F32;4],
	fRec125: [F32;2048],
	fVec30: [F32;2],
	fButton1: F32,
	fVec31: [F32;2],
	iRec126: [i32;2],
	fRec127: [F32;3],
	fVec32: [F32;3],
	fRec118: [F32;2048],
	fRec109: [F32;2],
	fRec105: [F32;2],
	fRec101: [F32;2048],
	fRec103: [F32;2],
	fRec99: [F32;4],
	fRec94: [F32;2],
	fRec90: [F32;2048],
	fRec88: [F32;2],
	fRec87: [F32;2],
	fRec86: [F32;2],
	fRec84: [F32;2],
	fRec82: [F32;2],
	fHslider15: F32,
	fRec129: [F32;2],
	fRec159: [F32;2],
	fRec163: [F32;2],
	fRec168: [F32;2],
	iVec33: [i32;2],
	iRec169: [i32;2],
	fRec166: [F32;2],
	fRec165: [F32;2],
	fRec170: [F32;4],
	fRec171: [F32;2048],
	fVec34: [F32;2],
	fButton2: F32,
	fVec35: [F32;2],
	iRec172: [i32;2],
	fRec173: [F32;3],
	fVec36: [F32;3],
	fRec164: [F32;2048],
	fRec155: [F32;2],
	fRec151: [F32;2],
	fRec147: [F32;2048],
	fRec149: [F32;2],
	fRec145: [F32;4],
	fRec140: [F32;2],
	fRec136: [F32;2048],
	fRec134: [F32;2],
	fRec133: [F32;2],
	fRec132: [F32;2],
	fRec130: [F32;2],
	fRec128: [F32;2],
	fHslider16: F32,
	fRec175: [F32;2],
	fRec205: [F32;2],
	fRec209: [F32;2],
	fRec214: [F32;2],
	iVec37: [i32;2],
	iRec215: [i32;2],
	fRec212: [F32;2],
	fRec211: [F32;2],
	fRec216: [F32;4],
	fRec217: [F32;2048],
	fVec38: [F32;2],
	fButton3: F32,
	fVec39: [F32;2],
	iRec218: [i32;2],
	fRec219: [F32;3],
	fVec40: [F32;3],
	fRec210: [F32;2048],
	fRec201: [F32;2],
	fRec197: [F32;2],
	fRec193: [F32;2048],
	fRec195: [F32;2],
	fRec191: [F32;4],
	fRec186: [F32;2],
	fRec182: [F32;2048],
	fRec180: [F32;2],
	fRec179: [F32;2],
	fRec178: [F32;2],
	fRec176: [F32;2],
	fRec174: [F32;2],
	fHslider17: F32,
	fRec220: [F32;2],
	fHslider18: F32,
	fRec222: [F32;2],
	fHslider19: F32,
	fConst17: F32,
	fRec221: [F32;2],
	fConst18: F32,
	fRec227: [F32;2],
	fRec225: [F32;2],
	fHslider20: F32,
	fRec228: [F32;2],
	fRec224: [F32;3],
	fRec223: [F32;3],
	fHslider21: F32,
	fRec229: [F32;2],
	fRec234: [F32;2],
	fRec232: [F32;2],
	fHslider22: F32,
	fRec235: [F32;2],
	fRec231: [F32;3],
	fRec230: [F32;3],
	fHslider23: F32,
	fRec236: [F32;2],
	fRec241: [F32;2],
	fRec239: [F32;2],
	fHslider24: F32,
	fRec242: [F32;2],
	fRec238: [F32;3],
	fRec237: [F32;3],
	fHslider25: F32,
	fRec243: [F32;2],
	fRec248: [F32;2],
	fRec246: [F32;2],
	fHslider26: F32,
	fRec249: [F32;2],
	fRec245: [F32;3],
	fRec244: [F32;3],
	fHslider27: F32,
	fRec250: [F32;2],
	fHslider28: F32,
	fRec251: [F32;2],
	fHslider29: F32,
	fRec252: [F32;2],
	fConst19: F32,
	fHslider30: F32,
	fRec254: [F32;2],
	fHslider31: F32,
	fRec253: [F32;2097152],
	fHslider32: F32,
	fConst22: F32,
	fConst23: F32,
	fConst25: F32,
	fConst26: F32,
	fConst27: F32,
	fConst28: F32,
	fConst31: F32,
	fConst32: F32,
	fConst33: F32,
	fConst34: F32,
	fConst35: F32,
	fConst36: F32,
	fConst37: F32,
	fHslider33: F32,
	fRec265: [F32;2],
	fRec267: [F32;2],
	fRec271: [F32;2],
	fVec41: [F32;16384],
	fVec42: [F32;2],
	fRec270: [F32;2],
	fRec268: [F32;2],
	fRec273: [F32;2],
	fVec43: [F32;16384],
	fVec44: [F32;2],
	fRec272: [F32;2],
	fRec269: [F32;2],
	fRec277: [F32;2],
	fVec45: [F32;16384],
	fVec46: [F32;2],
	fRec276: [F32;2],
	fRec274: [F32;2],
	fRec279: [F32;2],
	fVec47: [F32;16384],
	fVec48: [F32;2],
	fRec278: [F32;2],
	fRec275: [F32;2],
	fRec283: [F32;2],
	fVec49: [F32;16384],
	fVec50: [F32;2],
	fRec282: [F32;2],
	fRec280: [F32;2],
	fRec285: [F32;2],
	fVec51: [F32;16384],
	fVec52: [F32;2],
	fRec284: [F32;2],
	fRec281: [F32;2],
	fRec289: [F32;2],
	fVec53: [F32;16384],
	fVec54: [F32;2],
	fRec288: [F32;2],
	fRec286: [F32;2],
	fRec291: [F32;2],
	fVec55: [F32;16384],
	fVec56: [F32;2],
	fRec290: [F32;2],
	fRec287: [F32;2],
	fRec295: [F32;2],
	fVec57: [F32;16384],
	fVec58: [F32;2],
	fRec294: [F32;2],
	fRec292: [F32;2],
	fRec297: [F32;2],
	fVec59: [F32;16384],
	fVec60: [F32;2],
	fRec296: [F32;2],
	fRec293: [F32;2],
	fVec61: [F32;1024],
	fHslider34: F32,
	fConst38: F32,
	fRec298: [F32;2],
	fRec299: [F32;2],
	fHslider35: F32,
	fVec62: [F32;16384],
	fVec63: [F32;2],
	fRec266: [F32;2],
	fRec303: [F32;2],
	fRec305: [F32;2],
	fVec64: [F32;1024],
	fVec65: [F32;16384],
	fVec66: [F32;2],
	fRec304: [F32;2],
	fVec67: [F32;16384],
	fVec68: [F32;2],
	fRec302: [F32;2],
	fRec300: [F32;2],
	fRec307: [F32;2],
	fVec69: [F32;16384],
	fVec70: [F32;2],
	fRec306: [F32;2],
	fRec301: [F32;2],
	fRec311: [F32;2],
	fVec71: [F32;16384],
	fVec72: [F32;2],
	fRec310: [F32;2],
	fRec308: [F32;2],
	fRec313: [F32;2],
	fVec73: [F32;16384],
	fVec74: [F32;2],
	fRec312: [F32;2],
	fRec309: [F32;2],
	fRec317: [F32;2],
	fVec75: [F32;16384],
	fVec76: [F32;2],
	fRec316: [F32;2],
	fRec314: [F32;2],
	fRec319: [F32;2],
	fVec77: [F32;16384],
	fVec78: [F32;2],
	fRec318: [F32;2],
	fRec315: [F32;2],
	fRec323: [F32;2],
	fVec79: [F32;16384],
	fVec80: [F32;2],
	fRec322: [F32;2],
	fRec320: [F32;2],
	fRec325: [F32;2],
	fVec81: [F32;16384],
	fVec82: [F32;2],
	fRec324: [F32;2],
	fRec321: [F32;2],
	fRec329: [F32;2],
	fVec83: [F32;16384],
	fVec84: [F32;2],
	fRec328: [F32;2],
	fRec326: [F32;2],
	fRec331: [F32;2],
	fVec85: [F32;16384],
	fVec86: [F32;2],
	fRec330: [F32;2],
	fRec327: [F32;2],
	fVec87: [F32;16384],
	fVec88: [F32;16384],
	fVec89: [F32;2],
	fRec264: [F32;2],
	fConst39: F32,
	fConst41: F32,
	fRec263: [F32;2],
	fRec262: [F32;3],
	fRec261: [F32;3],
	fVec90: [F32;2],
	fConst42: F32,
	fConst44: F32,
	fRec260: [F32;2],
	fRec259: [F32;3],
	fRec258: [F32;3],
	fConst45: F32,
	fConst46: F32,
	fConst47: F32,
	fRec334: [F32;2],
	fRec333: [F32;3],
	fConst48: F32,
	fRec332: [F32;3],
	fConst49: F32,
	fConst50: F32,
	fConst51: F32,
	fRec338: [F32;2],
	fRec337: [F32;3],
	fConst52: F32,
	fRec336: [F32;3],
	fRec335: [F32;3],
	fHslider36: F32,
	fVec91: [F32;1024],
	fHslider37: F32,
	fRec257: [F32;2],
	fHslider38: F32,
	fRec350: [F32;2],
	fVec92: [F32;16384],
	fVec93: [F32;16384],
	fVec94: [F32;2],
	fRec349: [F32;2],
	fRec348: [F32;2],
	fRec347: [F32;3],
	fRec346: [F32;3],
	fVec95: [F32;2],
	fRec345: [F32;2],
	fRec344: [F32;3],
	fRec343: [F32;3],
	fRec353: [F32;2],
	fRec352: [F32;3],
	fRec351: [F32;3],
	fRec357: [F32;2],
	fRec356: [F32;3],
	fRec355: [F32;3],
	fRec354: [F32;3],
	fVec96: [F32;1024],
	fRec342: [F32;2],
	fVec97: [F32;16384],
	fVec98: [F32;2],
	fRec341: [F32;2],
	fRec339: [F32;2],
	fRec359: [F32;2],
	fVec99: [F32;16384],
	fVec100: [F32;2],
	fRec358: [F32;2],
	fRec340: [F32;2],
	fVec101: [F32;16384],
	fVec102: [F32;2],
	fRec362: [F32;2],
	fRec360: [F32;2],
	fVec103: [F32;16384],
	fVec104: [F32;2],
	fRec363: [F32;2],
	fRec361: [F32;2],
	fVec105: [F32;16384],
	fVec106: [F32;2],
	fRec366: [F32;2],
	fRec364: [F32;2],
	fRec368: [F32;2],
	fVec107: [F32;16384],
	fVec108: [F32;2],
	fRec367: [F32;2],
	fRec365: [F32;2],
	fRec372: [F32;2],
	fVec109: [F32;16384],
	fVec110: [F32;2],
	fRec371: [F32;2],
	fRec369: [F32;2],
	fVec111: [F32;16384],
	fVec112: [F32;2],
	fRec373: [F32;2],
	fRec370: [F32;2],
	fRec255: [F32;2],
	fRec256: [F32;2],
	fHslider39: F32,
	fRec374: [F32;2],
}

impl FaustDsp for mydsp {
	type T = F32;
		
	fn new() -> mydsp { 
		mydsp {
			fHslider0: 0.0,
			iVec0: [0;2],
			fSampleRate: 0,
			fConst0: 0.0,
			fConst1: 0.0,
			fConst2: 0.0,
			fHslider1: 0.0,
			fRec0: [0.0;2],
			fConst3: 0.0,
			fRec2: [0.0;2],
			fVec1: [0.0;2],
			IOTA0: 0,
			fVec2: [0.0;4096],
			fConst4: 0.0,
			fConst5: 0.0,
			fRec1: [0.0;2],
			fRec4: [0.0;2],
			fVec3: [0.0;2],
			fVec4: [0.0;4096],
			fRec3: [0.0;2],
			fRec6: [0.0;2],
			fVec5: [0.0;2],
			fVec6: [0.0;4096],
			fRec5: [0.0;2],
			fHslider2: 0.0,
			fRec7: [0.0;2],
			fHslider3: 0.0,
			fRec8: [0.0;2],
			fRec10: [0.0;2],
			fVec7: [0.0;2],
			fVec8: [0.0;4096],
			fRec9: [0.0;2],
			fRec12: [0.0;2],
			fVec9: [0.0;2],
			fVec10: [0.0;4096],
			fRec11: [0.0;2],
			fRec14: [0.0;2],
			fVec11: [0.0;2],
			fVec12: [0.0;4096],
			fRec13: [0.0;2],
			fHslider4: 0.0,
			fRec15: [0.0;2],
			fHslider5: 0.0,
			fRec16: [0.0;2],
			fRec18: [0.0;2],
			fVec13: [0.0;2],
			fVec14: [0.0;4096],
			fRec17: [0.0;2],
			fRec20: [0.0;2],
			fVec15: [0.0;2],
			fVec16: [0.0;4096],
			fRec19: [0.0;2],
			fRec22: [0.0;2],
			fVec17: [0.0;2],
			fVec18: [0.0;4096],
			fRec21: [0.0;2],
			fHslider6: 0.0,
			fRec23: [0.0;2],
			fHslider7: 0.0,
			fRec24: [0.0;2],
			fRec26: [0.0;2],
			fVec19: [0.0;2],
			fVec20: [0.0;4096],
			fRec25: [0.0;2],
			fRec28: [0.0;2],
			fVec21: [0.0;2],
			fVec22: [0.0;4096],
			fRec27: [0.0;2],
			fRec30: [0.0;2],
			fVec23: [0.0;2],
			fVec24: [0.0;4096],
			fRec29: [0.0;2],
			fHslider8: 0.0,
			fRec31: [0.0;2],
			fHslider9: 0.0,
			fRec32: [0.0;2],
			fConst6: 0.0,
			fHslider10: 0.0,
			fRec33: [0.0;2],
			fHslider11: 0.0,
			fRec35: [0.0;2],
			fHslider12: 0.0,
			fRec36: [0.0;2],
			fConst7: 0.0,
			fConst8: 0.0,
			fConst9: 0.0,
			fRec66: [0.0;2],
			fRec70: [0.0;2],
			fConst10: 0.0,
			fConst11: 0.0,
			fRec75: [0.0;2],
			iVec25: [0;2],
			iConst12: 0,
			iRec76: [0;2],
			fConst13: 0.0,
			fRec73: [0.0;2],
			fRec72: [0.0;2],
			fRec77: [0.0;4],
			fRec78: [0.0;2048],
			fVec26: [0.0;2],
			fConst14: 0.0,
			fConst15: 0.0,
			fButton0: 0.0,
			fVec27: [0.0;2],
			iRec79: [0;2],
			iRec81: [0;2],
			fRec80: [0.0;3],
			fVec28: [0.0;3],
			fRec71: [0.0;2048],
			fRec62: [0.0;2],
			fRec58: [0.0;2],
			fRec54: [0.0;2048],
			fRec56: [0.0;2],
			fHslider13: 0.0,
			fRec52: [0.0;4],
			fRec47: [0.0;2],
			fRec43: [0.0;2048],
			fRec41: [0.0;2],
			fConst16: 0.0,
			fRec40: [0.0;2],
			fRec39: [0.0;2],
			fRec37: [0.0;2],
			fRec34: [0.0;2],
			fHslider14: 0.0,
			fRec83: [0.0;2],
			fRec113: [0.0;2],
			fRec117: [0.0;2],
			fRec122: [0.0;2],
			iVec29: [0;2],
			iRec123: [0;2],
			fRec120: [0.0;2],
			fRec119: [0.0;2],
			fRec124: [0.0;4],
			fRec125: [0.0;2048],
			fVec30: [0.0;2],
			fButton1: 0.0,
			fVec31: [0.0;2],
			iRec126: [0;2],
			fRec127: [0.0;3],
			fVec32: [0.0;3],
			fRec118: [0.0;2048],
			fRec109: [0.0;2],
			fRec105: [0.0;2],
			fRec101: [0.0;2048],
			fRec103: [0.0;2],
			fRec99: [0.0;4],
			fRec94: [0.0;2],
			fRec90: [0.0;2048],
			fRec88: [0.0;2],
			fRec87: [0.0;2],
			fRec86: [0.0;2],
			fRec84: [0.0;2],
			fRec82: [0.0;2],
			fHslider15: 0.0,
			fRec129: [0.0;2],
			fRec159: [0.0;2],
			fRec163: [0.0;2],
			fRec168: [0.0;2],
			iVec33: [0;2],
			iRec169: [0;2],
			fRec166: [0.0;2],
			fRec165: [0.0;2],
			fRec170: [0.0;4],
			fRec171: [0.0;2048],
			fVec34: [0.0;2],
			fButton2: 0.0,
			fVec35: [0.0;2],
			iRec172: [0;2],
			fRec173: [0.0;3],
			fVec36: [0.0;3],
			fRec164: [0.0;2048],
			fRec155: [0.0;2],
			fRec151: [0.0;2],
			fRec147: [0.0;2048],
			fRec149: [0.0;2],
			fRec145: [0.0;4],
			fRec140: [0.0;2],
			fRec136: [0.0;2048],
			fRec134: [0.0;2],
			fRec133: [0.0;2],
			fRec132: [0.0;2],
			fRec130: [0.0;2],
			fRec128: [0.0;2],
			fHslider16: 0.0,
			fRec175: [0.0;2],
			fRec205: [0.0;2],
			fRec209: [0.0;2],
			fRec214: [0.0;2],
			iVec37: [0;2],
			iRec215: [0;2],
			fRec212: [0.0;2],
			fRec211: [0.0;2],
			fRec216: [0.0;4],
			fRec217: [0.0;2048],
			fVec38: [0.0;2],
			fButton3: 0.0,
			fVec39: [0.0;2],
			iRec218: [0;2],
			fRec219: [0.0;3],
			fVec40: [0.0;3],
			fRec210: [0.0;2048],
			fRec201: [0.0;2],
			fRec197: [0.0;2],
			fRec193: [0.0;2048],
			fRec195: [0.0;2],
			fRec191: [0.0;4],
			fRec186: [0.0;2],
			fRec182: [0.0;2048],
			fRec180: [0.0;2],
			fRec179: [0.0;2],
			fRec178: [0.0;2],
			fRec176: [0.0;2],
			fRec174: [0.0;2],
			fHslider17: 0.0,
			fRec220: [0.0;2],
			fHslider18: 0.0,
			fRec222: [0.0;2],
			fHslider19: 0.0,
			fConst17: 0.0,
			fRec221: [0.0;2],
			fConst18: 0.0,
			fRec227: [0.0;2],
			fRec225: [0.0;2],
			fHslider20: 0.0,
			fRec228: [0.0;2],
			fRec224: [0.0;3],
			fRec223: [0.0;3],
			fHslider21: 0.0,
			fRec229: [0.0;2],
			fRec234: [0.0;2],
			fRec232: [0.0;2],
			fHslider22: 0.0,
			fRec235: [0.0;2],
			fRec231: [0.0;3],
			fRec230: [0.0;3],
			fHslider23: 0.0,
			fRec236: [0.0;2],
			fRec241: [0.0;2],
			fRec239: [0.0;2],
			fHslider24: 0.0,
			fRec242: [0.0;2],
			fRec238: [0.0;3],
			fRec237: [0.0;3],
			fHslider25: 0.0,
			fRec243: [0.0;2],
			fRec248: [0.0;2],
			fRec246: [0.0;2],
			fHslider26: 0.0,
			fRec249: [0.0;2],
			fRec245: [0.0;3],
			fRec244: [0.0;3],
			fHslider27: 0.0,
			fRec250: [0.0;2],
			fHslider28: 0.0,
			fRec251: [0.0;2],
			fHslider29: 0.0,
			fRec252: [0.0;2],
			fConst19: 0.0,
			fHslider30: 0.0,
			fRec254: [0.0;2],
			fHslider31: 0.0,
			fRec253: [0.0;2097152],
			fHslider32: 0.0,
			fConst22: 0.0,
			fConst23: 0.0,
			fConst25: 0.0,
			fConst26: 0.0,
			fConst27: 0.0,
			fConst28: 0.0,
			fConst31: 0.0,
			fConst32: 0.0,
			fConst33: 0.0,
			fConst34: 0.0,
			fConst35: 0.0,
			fConst36: 0.0,
			fConst37: 0.0,
			fHslider33: 0.0,
			fRec265: [0.0;2],
			fRec267: [0.0;2],
			fRec271: [0.0;2],
			fVec41: [0.0;16384],
			fVec42: [0.0;2],
			fRec270: [0.0;2],
			fRec268: [0.0;2],
			fRec273: [0.0;2],
			fVec43: [0.0;16384],
			fVec44: [0.0;2],
			fRec272: [0.0;2],
			fRec269: [0.0;2],
			fRec277: [0.0;2],
			fVec45: [0.0;16384],
			fVec46: [0.0;2],
			fRec276: [0.0;2],
			fRec274: [0.0;2],
			fRec279: [0.0;2],
			fVec47: [0.0;16384],
			fVec48: [0.0;2],
			fRec278: [0.0;2],
			fRec275: [0.0;2],
			fRec283: [0.0;2],
			fVec49: [0.0;16384],
			fVec50: [0.0;2],
			fRec282: [0.0;2],
			fRec280: [0.0;2],
			fRec285: [0.0;2],
			fVec51: [0.0;16384],
			fVec52: [0.0;2],
			fRec284: [0.0;2],
			fRec281: [0.0;2],
			fRec289: [0.0;2],
			fVec53: [0.0;16384],
			fVec54: [0.0;2],
			fRec288: [0.0;2],
			fRec286: [0.0;2],
			fRec291: [0.0;2],
			fVec55: [0.0;16384],
			fVec56: [0.0;2],
			fRec290: [0.0;2],
			fRec287: [0.0;2],
			fRec295: [0.0;2],
			fVec57: [0.0;16384],
			fVec58: [0.0;2],
			fRec294: [0.0;2],
			fRec292: [0.0;2],
			fRec297: [0.0;2],
			fVec59: [0.0;16384],
			fVec60: [0.0;2],
			fRec296: [0.0;2],
			fRec293: [0.0;2],
			fVec61: [0.0;1024],
			fHslider34: 0.0,
			fConst38: 0.0,
			fRec298: [0.0;2],
			fRec299: [0.0;2],
			fHslider35: 0.0,
			fVec62: [0.0;16384],
			fVec63: [0.0;2],
			fRec266: [0.0;2],
			fRec303: [0.0;2],
			fRec305: [0.0;2],
			fVec64: [0.0;1024],
			fVec65: [0.0;16384],
			fVec66: [0.0;2],
			fRec304: [0.0;2],
			fVec67: [0.0;16384],
			fVec68: [0.0;2],
			fRec302: [0.0;2],
			fRec300: [0.0;2],
			fRec307: [0.0;2],
			fVec69: [0.0;16384],
			fVec70: [0.0;2],
			fRec306: [0.0;2],
			fRec301: [0.0;2],
			fRec311: [0.0;2],
			fVec71: [0.0;16384],
			fVec72: [0.0;2],
			fRec310: [0.0;2],
			fRec308: [0.0;2],
			fRec313: [0.0;2],
			fVec73: [0.0;16384],
			fVec74: [0.0;2],
			fRec312: [0.0;2],
			fRec309: [0.0;2],
			fRec317: [0.0;2],
			fVec75: [0.0;16384],
			fVec76: [0.0;2],
			fRec316: [0.0;2],
			fRec314: [0.0;2],
			fRec319: [0.0;2],
			fVec77: [0.0;16384],
			fVec78: [0.0;2],
			fRec318: [0.0;2],
			fRec315: [0.0;2],
			fRec323: [0.0;2],
			fVec79: [0.0;16384],
			fVec80: [0.0;2],
			fRec322: [0.0;2],
			fRec320: [0.0;2],
			fRec325: [0.0;2],
			fVec81: [0.0;16384],
			fVec82: [0.0;2],
			fRec324: [0.0;2],
			fRec321: [0.0;2],
			fRec329: [0.0;2],
			fVec83: [0.0;16384],
			fVec84: [0.0;2],
			fRec328: [0.0;2],
			fRec326: [0.0;2],
			fRec331: [0.0;2],
			fVec85: [0.0;16384],
			fVec86: [0.0;2],
			fRec330: [0.0;2],
			fRec327: [0.0;2],
			fVec87: [0.0;16384],
			fVec88: [0.0;16384],
			fVec89: [0.0;2],
			fRec264: [0.0;2],
			fConst39: 0.0,
			fConst41: 0.0,
			fRec263: [0.0;2],
			fRec262: [0.0;3],
			fRec261: [0.0;3],
			fVec90: [0.0;2],
			fConst42: 0.0,
			fConst44: 0.0,
			fRec260: [0.0;2],
			fRec259: [0.0;3],
			fRec258: [0.0;3],
			fConst45: 0.0,
			fConst46: 0.0,
			fConst47: 0.0,
			fRec334: [0.0;2],
			fRec333: [0.0;3],
			fConst48: 0.0,
			fRec332: [0.0;3],
			fConst49: 0.0,
			fConst50: 0.0,
			fConst51: 0.0,
			fRec338: [0.0;2],
			fRec337: [0.0;3],
			fConst52: 0.0,
			fRec336: [0.0;3],
			fRec335: [0.0;3],
			fHslider36: 0.0,
			fVec91: [0.0;1024],
			fHslider37: 0.0,
			fRec257: [0.0;2],
			fHslider38: 0.0,
			fRec350: [0.0;2],
			fVec92: [0.0;16384],
			fVec93: [0.0;16384],
			fVec94: [0.0;2],
			fRec349: [0.0;2],
			fRec348: [0.0;2],
			fRec347: [0.0;3],
			fRec346: [0.0;3],
			fVec95: [0.0;2],
			fRec345: [0.0;2],
			fRec344: [0.0;3],
			fRec343: [0.0;3],
			fRec353: [0.0;2],
			fRec352: [0.0;3],
			fRec351: [0.0;3],
			fRec357: [0.0;2],
			fRec356: [0.0;3],
			fRec355: [0.0;3],
			fRec354: [0.0;3],
			fVec96: [0.0;1024],
			fRec342: [0.0;2],
			fVec97: [0.0;16384],
			fVec98: [0.0;2],
			fRec341: [0.0;2],
			fRec339: [0.0;2],
			fRec359: [0.0;2],
			fVec99: [0.0;16384],
			fVec100: [0.0;2],
			fRec358: [0.0;2],
			fRec340: [0.0;2],
			fVec101: [0.0;16384],
			fVec102: [0.0;2],
			fRec362: [0.0;2],
			fRec360: [0.0;2],
			fVec103: [0.0;16384],
			fVec104: [0.0;2],
			fRec363: [0.0;2],
			fRec361: [0.0;2],
			fVec105: [0.0;16384],
			fVec106: [0.0;2],
			fRec366: [0.0;2],
			fRec364: [0.0;2],
			fRec368: [0.0;2],
			fVec107: [0.0;16384],
			fVec108: [0.0;2],
			fRec367: [0.0;2],
			fRec365: [0.0;2],
			fRec372: [0.0;2],
			fVec109: [0.0;16384],
			fVec110: [0.0;2],
			fRec371: [0.0;2],
			fRec369: [0.0;2],
			fVec111: [0.0;16384],
			fVec112: [0.0;2],
			fRec373: [0.0;2],
			fRec370: [0.0;2],
			fRec255: [0.0;2],
			fRec256: [0.0;2],
			fHslider39: 0.0,
			fRec374: [0.0;2],
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
		sig0.fillmydspSIG0(1302, unsafe { &mut itbl0mydspSIG0 });
	}
	fn instance_reset_params(&mut self) {
		self.fHslider0 = 0.1;
		self.fHslider1 = 6e+01;
		self.fHslider2 = 0.0;
		self.fHslider3 = 6e+01;
		self.fHslider4 = 0.0;
		self.fHslider5 = 6e+01;
		self.fHslider6 = 0.0;
		self.fHslider7 = 6e+01;
		self.fHslider8 = 0.0;
		self.fHslider9 = 1.0;
		self.fHslider10 = 1.0;
		self.fHslider11 = 8e+01;
		self.fHslider12 = 0.0;
		self.fButton0 = 0.0;
		self.fHslider13 = 1.0;
		self.fHslider14 = 8e+01;
		self.fButton1 = 0.0;
		self.fHslider15 = 8e+01;
		self.fButton2 = 0.0;
		self.fHslider16 = 8e+01;
		self.fButton3 = 0.0;
		self.fHslider17 = 0.0;
		self.fHslider18 = 0.0;
		self.fHslider19 = 6e+01;
		self.fHslider20 = 0.0;
		self.fHslider21 = 6e+01;
		self.fHslider22 = 0.0;
		self.fHslider23 = 6e+01;
		self.fHslider24 = 0.0;
		self.fHslider25 = 6e+01;
		self.fHslider26 = 0.0;
		self.fHslider27 = 1.0;
		self.fHslider28 = 0.0;
		self.fHslider29 = 1.0;
		self.fHslider30 = 0.3;
		self.fHslider31 = 0.3;
		self.fHslider32 = 0.11;
		self.fHslider33 = 5.0;
		self.fHslider34 = 0.6;
		self.fHslider35 = 0.98;
		self.fHslider36 = 3.5;
		self.fHslider37 = 0.88;
		self.fHslider38 = 0.75;
		self.fHslider39 = 1.0;
	}
	fn instance_clear(&mut self) {
		for l0 in 0..2 {
			self.iVec0[(l0) as usize] = 0;
		}
		for l1 in 0..2 {
			self.fRec0[(l1) as usize] = 0.0;
		}
		for l2 in 0..2 {
			self.fRec2[(l2) as usize] = 0.0;
		}
		for l3 in 0..2 {
			self.fVec1[(l3) as usize] = 0.0;
		}
		self.IOTA0 = 0;
		for l4 in 0..4096 {
			self.fVec2[(l4) as usize] = 0.0;
		}
		for l5 in 0..2 {
			self.fRec1[(l5) as usize] = 0.0;
		}
		for l6 in 0..2 {
			self.fRec4[(l6) as usize] = 0.0;
		}
		for l7 in 0..2 {
			self.fVec3[(l7) as usize] = 0.0;
		}
		for l8 in 0..4096 {
			self.fVec4[(l8) as usize] = 0.0;
		}
		for l9 in 0..2 {
			self.fRec3[(l9) as usize] = 0.0;
		}
		for l10 in 0..2 {
			self.fRec6[(l10) as usize] = 0.0;
		}
		for l11 in 0..2 {
			self.fVec5[(l11) as usize] = 0.0;
		}
		for l12 in 0..4096 {
			self.fVec6[(l12) as usize] = 0.0;
		}
		for l13 in 0..2 {
			self.fRec5[(l13) as usize] = 0.0;
		}
		for l14 in 0..2 {
			self.fRec7[(l14) as usize] = 0.0;
		}
		for l15 in 0..2 {
			self.fRec8[(l15) as usize] = 0.0;
		}
		for l16 in 0..2 {
			self.fRec10[(l16) as usize] = 0.0;
		}
		for l17 in 0..2 {
			self.fVec7[(l17) as usize] = 0.0;
		}
		for l18 in 0..4096 {
			self.fVec8[(l18) as usize] = 0.0;
		}
		for l19 in 0..2 {
			self.fRec9[(l19) as usize] = 0.0;
		}
		for l20 in 0..2 {
			self.fRec12[(l20) as usize] = 0.0;
		}
		for l21 in 0..2 {
			self.fVec9[(l21) as usize] = 0.0;
		}
		for l22 in 0..4096 {
			self.fVec10[(l22) as usize] = 0.0;
		}
		for l23 in 0..2 {
			self.fRec11[(l23) as usize] = 0.0;
		}
		for l24 in 0..2 {
			self.fRec14[(l24) as usize] = 0.0;
		}
		for l25 in 0..2 {
			self.fVec11[(l25) as usize] = 0.0;
		}
		for l26 in 0..4096 {
			self.fVec12[(l26) as usize] = 0.0;
		}
		for l27 in 0..2 {
			self.fRec13[(l27) as usize] = 0.0;
		}
		for l28 in 0..2 {
			self.fRec15[(l28) as usize] = 0.0;
		}
		for l29 in 0..2 {
			self.fRec16[(l29) as usize] = 0.0;
		}
		for l30 in 0..2 {
			self.fRec18[(l30) as usize] = 0.0;
		}
		for l31 in 0..2 {
			self.fVec13[(l31) as usize] = 0.0;
		}
		for l32 in 0..4096 {
			self.fVec14[(l32) as usize] = 0.0;
		}
		for l33 in 0..2 {
			self.fRec17[(l33) as usize] = 0.0;
		}
		for l34 in 0..2 {
			self.fRec20[(l34) as usize] = 0.0;
		}
		for l35 in 0..2 {
			self.fVec15[(l35) as usize] = 0.0;
		}
		for l36 in 0..4096 {
			self.fVec16[(l36) as usize] = 0.0;
		}
		for l37 in 0..2 {
			self.fRec19[(l37) as usize] = 0.0;
		}
		for l38 in 0..2 {
			self.fRec22[(l38) as usize] = 0.0;
		}
		for l39 in 0..2 {
			self.fVec17[(l39) as usize] = 0.0;
		}
		for l40 in 0..4096 {
			self.fVec18[(l40) as usize] = 0.0;
		}
		for l41 in 0..2 {
			self.fRec21[(l41) as usize] = 0.0;
		}
		for l42 in 0..2 {
			self.fRec23[(l42) as usize] = 0.0;
		}
		for l43 in 0..2 {
			self.fRec24[(l43) as usize] = 0.0;
		}
		for l44 in 0..2 {
			self.fRec26[(l44) as usize] = 0.0;
		}
		for l45 in 0..2 {
			self.fVec19[(l45) as usize] = 0.0;
		}
		for l46 in 0..4096 {
			self.fVec20[(l46) as usize] = 0.0;
		}
		for l47 in 0..2 {
			self.fRec25[(l47) as usize] = 0.0;
		}
		for l48 in 0..2 {
			self.fRec28[(l48) as usize] = 0.0;
		}
		for l49 in 0..2 {
			self.fVec21[(l49) as usize] = 0.0;
		}
		for l50 in 0..4096 {
			self.fVec22[(l50) as usize] = 0.0;
		}
		for l51 in 0..2 {
			self.fRec27[(l51) as usize] = 0.0;
		}
		for l52 in 0..2 {
			self.fRec30[(l52) as usize] = 0.0;
		}
		for l53 in 0..2 {
			self.fVec23[(l53) as usize] = 0.0;
		}
		for l54 in 0..4096 {
			self.fVec24[(l54) as usize] = 0.0;
		}
		for l55 in 0..2 {
			self.fRec29[(l55) as usize] = 0.0;
		}
		for l56 in 0..2 {
			self.fRec31[(l56) as usize] = 0.0;
		}
		for l57 in 0..2 {
			self.fRec32[(l57) as usize] = 0.0;
		}
		for l58 in 0..2 {
			self.fRec33[(l58) as usize] = 0.0;
		}
		for l59 in 0..2 {
			self.fRec35[(l59) as usize] = 0.0;
		}
		for l60 in 0..2 {
			self.fRec36[(l60) as usize] = 0.0;
		}
		for l61 in 0..2 {
			self.fRec66[(l61) as usize] = 0.0;
		}
		for l62 in 0..2 {
			self.fRec70[(l62) as usize] = 0.0;
		}
		for l63 in 0..2 {
			self.fRec75[(l63) as usize] = 0.0;
		}
		for l64 in 0..2 {
			self.iVec25[(l64) as usize] = 0;
		}
		for l65 in 0..2 {
			self.iRec76[(l65) as usize] = 0;
		}
		for l66 in 0..2 {
			self.fRec73[(l66) as usize] = 0.0;
		}
		for l67 in 0..2 {
			self.fRec72[(l67) as usize] = 0.0;
		}
		for l68 in 0..4 {
			self.fRec77[(l68) as usize] = 0.0;
		}
		for l69 in 0..2048 {
			self.fRec78[(l69) as usize] = 0.0;
		}
		for l70 in 0..2 {
			self.fVec26[(l70) as usize] = 0.0;
		}
		for l71 in 0..2 {
			self.fVec27[(l71) as usize] = 0.0;
		}
		for l72 in 0..2 {
			self.iRec79[(l72) as usize] = 0;
		}
		for l73 in 0..2 {
			self.iRec81[(l73) as usize] = 0;
		}
		for l74 in 0..3 {
			self.fRec80[(l74) as usize] = 0.0;
		}
		for l75 in 0..3 {
			self.fVec28[(l75) as usize] = 0.0;
		}
		for l76 in 0..2048 {
			self.fRec71[(l76) as usize] = 0.0;
		}
		for l77 in 0..2 {
			self.fRec62[(l77) as usize] = 0.0;
		}
		for l78 in 0..2 {
			self.fRec58[(l78) as usize] = 0.0;
		}
		for l79 in 0..2048 {
			self.fRec54[(l79) as usize] = 0.0;
		}
		for l80 in 0..2 {
			self.fRec56[(l80) as usize] = 0.0;
		}
		for l81 in 0..4 {
			self.fRec52[(l81) as usize] = 0.0;
		}
		for l82 in 0..2 {
			self.fRec47[(l82) as usize] = 0.0;
		}
		for l83 in 0..2048 {
			self.fRec43[(l83) as usize] = 0.0;
		}
		for l84 in 0..2 {
			self.fRec41[(l84) as usize] = 0.0;
		}
		for l85 in 0..2 {
			self.fRec40[(l85) as usize] = 0.0;
		}
		for l86 in 0..2 {
			self.fRec39[(l86) as usize] = 0.0;
		}
		for l87 in 0..2 {
			self.fRec37[(l87) as usize] = 0.0;
		}
		for l88 in 0..2 {
			self.fRec34[(l88) as usize] = 0.0;
		}
		for l89 in 0..2 {
			self.fRec83[(l89) as usize] = 0.0;
		}
		for l90 in 0..2 {
			self.fRec113[(l90) as usize] = 0.0;
		}
		for l91 in 0..2 {
			self.fRec117[(l91) as usize] = 0.0;
		}
		for l92 in 0..2 {
			self.fRec122[(l92) as usize] = 0.0;
		}
		for l93 in 0..2 {
			self.iVec29[(l93) as usize] = 0;
		}
		for l94 in 0..2 {
			self.iRec123[(l94) as usize] = 0;
		}
		for l95 in 0..2 {
			self.fRec120[(l95) as usize] = 0.0;
		}
		for l96 in 0..2 {
			self.fRec119[(l96) as usize] = 0.0;
		}
		for l97 in 0..4 {
			self.fRec124[(l97) as usize] = 0.0;
		}
		for l98 in 0..2048 {
			self.fRec125[(l98) as usize] = 0.0;
		}
		for l99 in 0..2 {
			self.fVec30[(l99) as usize] = 0.0;
		}
		for l100 in 0..2 {
			self.fVec31[(l100) as usize] = 0.0;
		}
		for l101 in 0..2 {
			self.iRec126[(l101) as usize] = 0;
		}
		for l102 in 0..3 {
			self.fRec127[(l102) as usize] = 0.0;
		}
		for l103 in 0..3 {
			self.fVec32[(l103) as usize] = 0.0;
		}
		for l104 in 0..2048 {
			self.fRec118[(l104) as usize] = 0.0;
		}
		for l105 in 0..2 {
			self.fRec109[(l105) as usize] = 0.0;
		}
		for l106 in 0..2 {
			self.fRec105[(l106) as usize] = 0.0;
		}
		for l107 in 0..2048 {
			self.fRec101[(l107) as usize] = 0.0;
		}
		for l108 in 0..2 {
			self.fRec103[(l108) as usize] = 0.0;
		}
		for l109 in 0..4 {
			self.fRec99[(l109) as usize] = 0.0;
		}
		for l110 in 0..2 {
			self.fRec94[(l110) as usize] = 0.0;
		}
		for l111 in 0..2048 {
			self.fRec90[(l111) as usize] = 0.0;
		}
		for l112 in 0..2 {
			self.fRec88[(l112) as usize] = 0.0;
		}
		for l113 in 0..2 {
			self.fRec87[(l113) as usize] = 0.0;
		}
		for l114 in 0..2 {
			self.fRec86[(l114) as usize] = 0.0;
		}
		for l115 in 0..2 {
			self.fRec84[(l115) as usize] = 0.0;
		}
		for l116 in 0..2 {
			self.fRec82[(l116) as usize] = 0.0;
		}
		for l117 in 0..2 {
			self.fRec129[(l117) as usize] = 0.0;
		}
		for l118 in 0..2 {
			self.fRec159[(l118) as usize] = 0.0;
		}
		for l119 in 0..2 {
			self.fRec163[(l119) as usize] = 0.0;
		}
		for l120 in 0..2 {
			self.fRec168[(l120) as usize] = 0.0;
		}
		for l121 in 0..2 {
			self.iVec33[(l121) as usize] = 0;
		}
		for l122 in 0..2 {
			self.iRec169[(l122) as usize] = 0;
		}
		for l123 in 0..2 {
			self.fRec166[(l123) as usize] = 0.0;
		}
		for l124 in 0..2 {
			self.fRec165[(l124) as usize] = 0.0;
		}
		for l125 in 0..4 {
			self.fRec170[(l125) as usize] = 0.0;
		}
		for l126 in 0..2048 {
			self.fRec171[(l126) as usize] = 0.0;
		}
		for l127 in 0..2 {
			self.fVec34[(l127) as usize] = 0.0;
		}
		for l128 in 0..2 {
			self.fVec35[(l128) as usize] = 0.0;
		}
		for l129 in 0..2 {
			self.iRec172[(l129) as usize] = 0;
		}
		for l130 in 0..3 {
			self.fRec173[(l130) as usize] = 0.0;
		}
		for l131 in 0..3 {
			self.fVec36[(l131) as usize] = 0.0;
		}
		for l132 in 0..2048 {
			self.fRec164[(l132) as usize] = 0.0;
		}
		for l133 in 0..2 {
			self.fRec155[(l133) as usize] = 0.0;
		}
		for l134 in 0..2 {
			self.fRec151[(l134) as usize] = 0.0;
		}
		for l135 in 0..2048 {
			self.fRec147[(l135) as usize] = 0.0;
		}
		for l136 in 0..2 {
			self.fRec149[(l136) as usize] = 0.0;
		}
		for l137 in 0..4 {
			self.fRec145[(l137) as usize] = 0.0;
		}
		for l138 in 0..2 {
			self.fRec140[(l138) as usize] = 0.0;
		}
		for l139 in 0..2048 {
			self.fRec136[(l139) as usize] = 0.0;
		}
		for l140 in 0..2 {
			self.fRec134[(l140) as usize] = 0.0;
		}
		for l141 in 0..2 {
			self.fRec133[(l141) as usize] = 0.0;
		}
		for l142 in 0..2 {
			self.fRec132[(l142) as usize] = 0.0;
		}
		for l143 in 0..2 {
			self.fRec130[(l143) as usize] = 0.0;
		}
		for l144 in 0..2 {
			self.fRec128[(l144) as usize] = 0.0;
		}
		for l145 in 0..2 {
			self.fRec175[(l145) as usize] = 0.0;
		}
		for l146 in 0..2 {
			self.fRec205[(l146) as usize] = 0.0;
		}
		for l147 in 0..2 {
			self.fRec209[(l147) as usize] = 0.0;
		}
		for l148 in 0..2 {
			self.fRec214[(l148) as usize] = 0.0;
		}
		for l149 in 0..2 {
			self.iVec37[(l149) as usize] = 0;
		}
		for l150 in 0..2 {
			self.iRec215[(l150) as usize] = 0;
		}
		for l151 in 0..2 {
			self.fRec212[(l151) as usize] = 0.0;
		}
		for l152 in 0..2 {
			self.fRec211[(l152) as usize] = 0.0;
		}
		for l153 in 0..4 {
			self.fRec216[(l153) as usize] = 0.0;
		}
		for l154 in 0..2048 {
			self.fRec217[(l154) as usize] = 0.0;
		}
		for l155 in 0..2 {
			self.fVec38[(l155) as usize] = 0.0;
		}
		for l156 in 0..2 {
			self.fVec39[(l156) as usize] = 0.0;
		}
		for l157 in 0..2 {
			self.iRec218[(l157) as usize] = 0;
		}
		for l158 in 0..3 {
			self.fRec219[(l158) as usize] = 0.0;
		}
		for l159 in 0..3 {
			self.fVec40[(l159) as usize] = 0.0;
		}
		for l160 in 0..2048 {
			self.fRec210[(l160) as usize] = 0.0;
		}
		for l161 in 0..2 {
			self.fRec201[(l161) as usize] = 0.0;
		}
		for l162 in 0..2 {
			self.fRec197[(l162) as usize] = 0.0;
		}
		for l163 in 0..2048 {
			self.fRec193[(l163) as usize] = 0.0;
		}
		for l164 in 0..2 {
			self.fRec195[(l164) as usize] = 0.0;
		}
		for l165 in 0..4 {
			self.fRec191[(l165) as usize] = 0.0;
		}
		for l166 in 0..2 {
			self.fRec186[(l166) as usize] = 0.0;
		}
		for l167 in 0..2048 {
			self.fRec182[(l167) as usize] = 0.0;
		}
		for l168 in 0..2 {
			self.fRec180[(l168) as usize] = 0.0;
		}
		for l169 in 0..2 {
			self.fRec179[(l169) as usize] = 0.0;
		}
		for l170 in 0..2 {
			self.fRec178[(l170) as usize] = 0.0;
		}
		for l171 in 0..2 {
			self.fRec176[(l171) as usize] = 0.0;
		}
		for l172 in 0..2 {
			self.fRec174[(l172) as usize] = 0.0;
		}
		for l173 in 0..2 {
			self.fRec220[(l173) as usize] = 0.0;
		}
		for l174 in 0..2 {
			self.fRec222[(l174) as usize] = 0.0;
		}
		for l175 in 0..2 {
			self.fRec221[(l175) as usize] = 0.0;
		}
		for l176 in 0..2 {
			self.fRec227[(l176) as usize] = 0.0;
		}
		for l177 in 0..2 {
			self.fRec225[(l177) as usize] = 0.0;
		}
		for l178 in 0..2 {
			self.fRec228[(l178) as usize] = 0.0;
		}
		for l179 in 0..3 {
			self.fRec224[(l179) as usize] = 0.0;
		}
		for l180 in 0..3 {
			self.fRec223[(l180) as usize] = 0.0;
		}
		for l181 in 0..2 {
			self.fRec229[(l181) as usize] = 0.0;
		}
		for l182 in 0..2 {
			self.fRec234[(l182) as usize] = 0.0;
		}
		for l183 in 0..2 {
			self.fRec232[(l183) as usize] = 0.0;
		}
		for l184 in 0..2 {
			self.fRec235[(l184) as usize] = 0.0;
		}
		for l185 in 0..3 {
			self.fRec231[(l185) as usize] = 0.0;
		}
		for l186 in 0..3 {
			self.fRec230[(l186) as usize] = 0.0;
		}
		for l187 in 0..2 {
			self.fRec236[(l187) as usize] = 0.0;
		}
		for l188 in 0..2 {
			self.fRec241[(l188) as usize] = 0.0;
		}
		for l189 in 0..2 {
			self.fRec239[(l189) as usize] = 0.0;
		}
		for l190 in 0..2 {
			self.fRec242[(l190) as usize] = 0.0;
		}
		for l191 in 0..3 {
			self.fRec238[(l191) as usize] = 0.0;
		}
		for l192 in 0..3 {
			self.fRec237[(l192) as usize] = 0.0;
		}
		for l193 in 0..2 {
			self.fRec243[(l193) as usize] = 0.0;
		}
		for l194 in 0..2 {
			self.fRec248[(l194) as usize] = 0.0;
		}
		for l195 in 0..2 {
			self.fRec246[(l195) as usize] = 0.0;
		}
		for l196 in 0..2 {
			self.fRec249[(l196) as usize] = 0.0;
		}
		for l197 in 0..3 {
			self.fRec245[(l197) as usize] = 0.0;
		}
		for l198 in 0..3 {
			self.fRec244[(l198) as usize] = 0.0;
		}
		for l199 in 0..2 {
			self.fRec250[(l199) as usize] = 0.0;
		}
		for l200 in 0..2 {
			self.fRec251[(l200) as usize] = 0.0;
		}
		for l201 in 0..2 {
			self.fRec252[(l201) as usize] = 0.0;
		}
		for l202 in 0..2 {
			self.fRec254[(l202) as usize] = 0.0;
		}
		for l203 in 0..2097152 {
			self.fRec253[(l203) as usize] = 0.0;
		}
		for l204 in 0..2 {
			self.fRec265[(l204) as usize] = 0.0;
		}
		for l205 in 0..2 {
			self.fRec267[(l205) as usize] = 0.0;
		}
		for l206 in 0..2 {
			self.fRec271[(l206) as usize] = 0.0;
		}
		for l207 in 0..16384 {
			self.fVec41[(l207) as usize] = 0.0;
		}
		for l208 in 0..2 {
			self.fVec42[(l208) as usize] = 0.0;
		}
		for l209 in 0..2 {
			self.fRec270[(l209) as usize] = 0.0;
		}
		for l210 in 0..2 {
			self.fRec268[(l210) as usize] = 0.0;
		}
		for l211 in 0..2 {
			self.fRec273[(l211) as usize] = 0.0;
		}
		for l212 in 0..16384 {
			self.fVec43[(l212) as usize] = 0.0;
		}
		for l213 in 0..2 {
			self.fVec44[(l213) as usize] = 0.0;
		}
		for l214 in 0..2 {
			self.fRec272[(l214) as usize] = 0.0;
		}
		for l215 in 0..2 {
			self.fRec269[(l215) as usize] = 0.0;
		}
		for l216 in 0..2 {
			self.fRec277[(l216) as usize] = 0.0;
		}
		for l217 in 0..16384 {
			self.fVec45[(l217) as usize] = 0.0;
		}
		for l218 in 0..2 {
			self.fVec46[(l218) as usize] = 0.0;
		}
		for l219 in 0..2 {
			self.fRec276[(l219) as usize] = 0.0;
		}
		for l220 in 0..2 {
			self.fRec274[(l220) as usize] = 0.0;
		}
		for l221 in 0..2 {
			self.fRec279[(l221) as usize] = 0.0;
		}
		for l222 in 0..16384 {
			self.fVec47[(l222) as usize] = 0.0;
		}
		for l223 in 0..2 {
			self.fVec48[(l223) as usize] = 0.0;
		}
		for l224 in 0..2 {
			self.fRec278[(l224) as usize] = 0.0;
		}
		for l225 in 0..2 {
			self.fRec275[(l225) as usize] = 0.0;
		}
		for l226 in 0..2 {
			self.fRec283[(l226) as usize] = 0.0;
		}
		for l227 in 0..16384 {
			self.fVec49[(l227) as usize] = 0.0;
		}
		for l228 in 0..2 {
			self.fVec50[(l228) as usize] = 0.0;
		}
		for l229 in 0..2 {
			self.fRec282[(l229) as usize] = 0.0;
		}
		for l230 in 0..2 {
			self.fRec280[(l230) as usize] = 0.0;
		}
		for l231 in 0..2 {
			self.fRec285[(l231) as usize] = 0.0;
		}
		for l232 in 0..16384 {
			self.fVec51[(l232) as usize] = 0.0;
		}
		for l233 in 0..2 {
			self.fVec52[(l233) as usize] = 0.0;
		}
		for l234 in 0..2 {
			self.fRec284[(l234) as usize] = 0.0;
		}
		for l235 in 0..2 {
			self.fRec281[(l235) as usize] = 0.0;
		}
		for l236 in 0..2 {
			self.fRec289[(l236) as usize] = 0.0;
		}
		for l237 in 0..16384 {
			self.fVec53[(l237) as usize] = 0.0;
		}
		for l238 in 0..2 {
			self.fVec54[(l238) as usize] = 0.0;
		}
		for l239 in 0..2 {
			self.fRec288[(l239) as usize] = 0.0;
		}
		for l240 in 0..2 {
			self.fRec286[(l240) as usize] = 0.0;
		}
		for l241 in 0..2 {
			self.fRec291[(l241) as usize] = 0.0;
		}
		for l242 in 0..16384 {
			self.fVec55[(l242) as usize] = 0.0;
		}
		for l243 in 0..2 {
			self.fVec56[(l243) as usize] = 0.0;
		}
		for l244 in 0..2 {
			self.fRec290[(l244) as usize] = 0.0;
		}
		for l245 in 0..2 {
			self.fRec287[(l245) as usize] = 0.0;
		}
		for l246 in 0..2 {
			self.fRec295[(l246) as usize] = 0.0;
		}
		for l247 in 0..16384 {
			self.fVec57[(l247) as usize] = 0.0;
		}
		for l248 in 0..2 {
			self.fVec58[(l248) as usize] = 0.0;
		}
		for l249 in 0..2 {
			self.fRec294[(l249) as usize] = 0.0;
		}
		for l250 in 0..2 {
			self.fRec292[(l250) as usize] = 0.0;
		}
		for l251 in 0..2 {
			self.fRec297[(l251) as usize] = 0.0;
		}
		for l252 in 0..16384 {
			self.fVec59[(l252) as usize] = 0.0;
		}
		for l253 in 0..2 {
			self.fVec60[(l253) as usize] = 0.0;
		}
		for l254 in 0..2 {
			self.fRec296[(l254) as usize] = 0.0;
		}
		for l255 in 0..2 {
			self.fRec293[(l255) as usize] = 0.0;
		}
		for l256 in 0..1024 {
			self.fVec61[(l256) as usize] = 0.0;
		}
		for l257 in 0..2 {
			self.fRec298[(l257) as usize] = 0.0;
		}
		for l258 in 0..2 {
			self.fRec299[(l258) as usize] = 0.0;
		}
		for l259 in 0..16384 {
			self.fVec62[(l259) as usize] = 0.0;
		}
		for l260 in 0..2 {
			self.fVec63[(l260) as usize] = 0.0;
		}
		for l261 in 0..2 {
			self.fRec266[(l261) as usize] = 0.0;
		}
		for l262 in 0..2 {
			self.fRec303[(l262) as usize] = 0.0;
		}
		for l263 in 0..2 {
			self.fRec305[(l263) as usize] = 0.0;
		}
		for l264 in 0..1024 {
			self.fVec64[(l264) as usize] = 0.0;
		}
		for l265 in 0..16384 {
			self.fVec65[(l265) as usize] = 0.0;
		}
		for l266 in 0..2 {
			self.fVec66[(l266) as usize] = 0.0;
		}
		for l267 in 0..2 {
			self.fRec304[(l267) as usize] = 0.0;
		}
		for l268 in 0..16384 {
			self.fVec67[(l268) as usize] = 0.0;
		}
		for l269 in 0..2 {
			self.fVec68[(l269) as usize] = 0.0;
		}
		for l270 in 0..2 {
			self.fRec302[(l270) as usize] = 0.0;
		}
		for l271 in 0..2 {
			self.fRec300[(l271) as usize] = 0.0;
		}
		for l272 in 0..2 {
			self.fRec307[(l272) as usize] = 0.0;
		}
		for l273 in 0..16384 {
			self.fVec69[(l273) as usize] = 0.0;
		}
		for l274 in 0..2 {
			self.fVec70[(l274) as usize] = 0.0;
		}
		for l275 in 0..2 {
			self.fRec306[(l275) as usize] = 0.0;
		}
		for l276 in 0..2 {
			self.fRec301[(l276) as usize] = 0.0;
		}
		for l277 in 0..2 {
			self.fRec311[(l277) as usize] = 0.0;
		}
		for l278 in 0..16384 {
			self.fVec71[(l278) as usize] = 0.0;
		}
		for l279 in 0..2 {
			self.fVec72[(l279) as usize] = 0.0;
		}
		for l280 in 0..2 {
			self.fRec310[(l280) as usize] = 0.0;
		}
		for l281 in 0..2 {
			self.fRec308[(l281) as usize] = 0.0;
		}
		for l282 in 0..2 {
			self.fRec313[(l282) as usize] = 0.0;
		}
		for l283 in 0..16384 {
			self.fVec73[(l283) as usize] = 0.0;
		}
		for l284 in 0..2 {
			self.fVec74[(l284) as usize] = 0.0;
		}
		for l285 in 0..2 {
			self.fRec312[(l285) as usize] = 0.0;
		}
		for l286 in 0..2 {
			self.fRec309[(l286) as usize] = 0.0;
		}
		for l287 in 0..2 {
			self.fRec317[(l287) as usize] = 0.0;
		}
		for l288 in 0..16384 {
			self.fVec75[(l288) as usize] = 0.0;
		}
		for l289 in 0..2 {
			self.fVec76[(l289) as usize] = 0.0;
		}
		for l290 in 0..2 {
			self.fRec316[(l290) as usize] = 0.0;
		}
		for l291 in 0..2 {
			self.fRec314[(l291) as usize] = 0.0;
		}
		for l292 in 0..2 {
			self.fRec319[(l292) as usize] = 0.0;
		}
		for l293 in 0..16384 {
			self.fVec77[(l293) as usize] = 0.0;
		}
		for l294 in 0..2 {
			self.fVec78[(l294) as usize] = 0.0;
		}
		for l295 in 0..2 {
			self.fRec318[(l295) as usize] = 0.0;
		}
		for l296 in 0..2 {
			self.fRec315[(l296) as usize] = 0.0;
		}
		for l297 in 0..2 {
			self.fRec323[(l297) as usize] = 0.0;
		}
		for l298 in 0..16384 {
			self.fVec79[(l298) as usize] = 0.0;
		}
		for l299 in 0..2 {
			self.fVec80[(l299) as usize] = 0.0;
		}
		for l300 in 0..2 {
			self.fRec322[(l300) as usize] = 0.0;
		}
		for l301 in 0..2 {
			self.fRec320[(l301) as usize] = 0.0;
		}
		for l302 in 0..2 {
			self.fRec325[(l302) as usize] = 0.0;
		}
		for l303 in 0..16384 {
			self.fVec81[(l303) as usize] = 0.0;
		}
		for l304 in 0..2 {
			self.fVec82[(l304) as usize] = 0.0;
		}
		for l305 in 0..2 {
			self.fRec324[(l305) as usize] = 0.0;
		}
		for l306 in 0..2 {
			self.fRec321[(l306) as usize] = 0.0;
		}
		for l307 in 0..2 {
			self.fRec329[(l307) as usize] = 0.0;
		}
		for l308 in 0..16384 {
			self.fVec83[(l308) as usize] = 0.0;
		}
		for l309 in 0..2 {
			self.fVec84[(l309) as usize] = 0.0;
		}
		for l310 in 0..2 {
			self.fRec328[(l310) as usize] = 0.0;
		}
		for l311 in 0..2 {
			self.fRec326[(l311) as usize] = 0.0;
		}
		for l312 in 0..2 {
			self.fRec331[(l312) as usize] = 0.0;
		}
		for l313 in 0..16384 {
			self.fVec85[(l313) as usize] = 0.0;
		}
		for l314 in 0..2 {
			self.fVec86[(l314) as usize] = 0.0;
		}
		for l315 in 0..2 {
			self.fRec330[(l315) as usize] = 0.0;
		}
		for l316 in 0..2 {
			self.fRec327[(l316) as usize] = 0.0;
		}
		for l317 in 0..16384 {
			self.fVec87[(l317) as usize] = 0.0;
		}
		for l318 in 0..16384 {
			self.fVec88[(l318) as usize] = 0.0;
		}
		for l319 in 0..2 {
			self.fVec89[(l319) as usize] = 0.0;
		}
		for l320 in 0..2 {
			self.fRec264[(l320) as usize] = 0.0;
		}
		for l321 in 0..2 {
			self.fRec263[(l321) as usize] = 0.0;
		}
		for l322 in 0..3 {
			self.fRec262[(l322) as usize] = 0.0;
		}
		for l323 in 0..3 {
			self.fRec261[(l323) as usize] = 0.0;
		}
		for l324 in 0..2 {
			self.fVec90[(l324) as usize] = 0.0;
		}
		for l325 in 0..2 {
			self.fRec260[(l325) as usize] = 0.0;
		}
		for l326 in 0..3 {
			self.fRec259[(l326) as usize] = 0.0;
		}
		for l327 in 0..3 {
			self.fRec258[(l327) as usize] = 0.0;
		}
		for l328 in 0..2 {
			self.fRec334[(l328) as usize] = 0.0;
		}
		for l329 in 0..3 {
			self.fRec333[(l329) as usize] = 0.0;
		}
		for l330 in 0..3 {
			self.fRec332[(l330) as usize] = 0.0;
		}
		for l331 in 0..2 {
			self.fRec338[(l331) as usize] = 0.0;
		}
		for l332 in 0..3 {
			self.fRec337[(l332) as usize] = 0.0;
		}
		for l333 in 0..3 {
			self.fRec336[(l333) as usize] = 0.0;
		}
		for l334 in 0..3 {
			self.fRec335[(l334) as usize] = 0.0;
		}
		for l335 in 0..1024 {
			self.fVec91[(l335) as usize] = 0.0;
		}
		for l336 in 0..2 {
			self.fRec257[(l336) as usize] = 0.0;
		}
		for l337 in 0..2 {
			self.fRec350[(l337) as usize] = 0.0;
		}
		for l338 in 0..16384 {
			self.fVec92[(l338) as usize] = 0.0;
		}
		for l339 in 0..16384 {
			self.fVec93[(l339) as usize] = 0.0;
		}
		for l340 in 0..2 {
			self.fVec94[(l340) as usize] = 0.0;
		}
		for l341 in 0..2 {
			self.fRec349[(l341) as usize] = 0.0;
		}
		for l342 in 0..2 {
			self.fRec348[(l342) as usize] = 0.0;
		}
		for l343 in 0..3 {
			self.fRec347[(l343) as usize] = 0.0;
		}
		for l344 in 0..3 {
			self.fRec346[(l344) as usize] = 0.0;
		}
		for l345 in 0..2 {
			self.fVec95[(l345) as usize] = 0.0;
		}
		for l346 in 0..2 {
			self.fRec345[(l346) as usize] = 0.0;
		}
		for l347 in 0..3 {
			self.fRec344[(l347) as usize] = 0.0;
		}
		for l348 in 0..3 {
			self.fRec343[(l348) as usize] = 0.0;
		}
		for l349 in 0..2 {
			self.fRec353[(l349) as usize] = 0.0;
		}
		for l350 in 0..3 {
			self.fRec352[(l350) as usize] = 0.0;
		}
		for l351 in 0..3 {
			self.fRec351[(l351) as usize] = 0.0;
		}
		for l352 in 0..2 {
			self.fRec357[(l352) as usize] = 0.0;
		}
		for l353 in 0..3 {
			self.fRec356[(l353) as usize] = 0.0;
		}
		for l354 in 0..3 {
			self.fRec355[(l354) as usize] = 0.0;
		}
		for l355 in 0..3 {
			self.fRec354[(l355) as usize] = 0.0;
		}
		for l356 in 0..1024 {
			self.fVec96[(l356) as usize] = 0.0;
		}
		for l357 in 0..2 {
			self.fRec342[(l357) as usize] = 0.0;
		}
		for l358 in 0..16384 {
			self.fVec97[(l358) as usize] = 0.0;
		}
		for l359 in 0..2 {
			self.fVec98[(l359) as usize] = 0.0;
		}
		for l360 in 0..2 {
			self.fRec341[(l360) as usize] = 0.0;
		}
		for l361 in 0..2 {
			self.fRec339[(l361) as usize] = 0.0;
		}
		for l362 in 0..2 {
			self.fRec359[(l362) as usize] = 0.0;
		}
		for l363 in 0..16384 {
			self.fVec99[(l363) as usize] = 0.0;
		}
		for l364 in 0..2 {
			self.fVec100[(l364) as usize] = 0.0;
		}
		for l365 in 0..2 {
			self.fRec358[(l365) as usize] = 0.0;
		}
		for l366 in 0..2 {
			self.fRec340[(l366) as usize] = 0.0;
		}
		for l367 in 0..16384 {
			self.fVec101[(l367) as usize] = 0.0;
		}
		for l368 in 0..2 {
			self.fVec102[(l368) as usize] = 0.0;
		}
		for l369 in 0..2 {
			self.fRec362[(l369) as usize] = 0.0;
		}
		for l370 in 0..2 {
			self.fRec360[(l370) as usize] = 0.0;
		}
		for l371 in 0..16384 {
			self.fVec103[(l371) as usize] = 0.0;
		}
		for l372 in 0..2 {
			self.fVec104[(l372) as usize] = 0.0;
		}
		for l373 in 0..2 {
			self.fRec363[(l373) as usize] = 0.0;
		}
		for l374 in 0..2 {
			self.fRec361[(l374) as usize] = 0.0;
		}
		for l375 in 0..16384 {
			self.fVec105[(l375) as usize] = 0.0;
		}
		for l376 in 0..2 {
			self.fVec106[(l376) as usize] = 0.0;
		}
		for l377 in 0..2 {
			self.fRec366[(l377) as usize] = 0.0;
		}
		for l378 in 0..2 {
			self.fRec364[(l378) as usize] = 0.0;
		}
		for l379 in 0..2 {
			self.fRec368[(l379) as usize] = 0.0;
		}
		for l380 in 0..16384 {
			self.fVec107[(l380) as usize] = 0.0;
		}
		for l381 in 0..2 {
			self.fVec108[(l381) as usize] = 0.0;
		}
		for l382 in 0..2 {
			self.fRec367[(l382) as usize] = 0.0;
		}
		for l383 in 0..2 {
			self.fRec365[(l383) as usize] = 0.0;
		}
		for l384 in 0..2 {
			self.fRec372[(l384) as usize] = 0.0;
		}
		for l385 in 0..16384 {
			self.fVec109[(l385) as usize] = 0.0;
		}
		for l386 in 0..2 {
			self.fVec110[(l386) as usize] = 0.0;
		}
		for l387 in 0..2 {
			self.fRec371[(l387) as usize] = 0.0;
		}
		for l388 in 0..2 {
			self.fRec369[(l388) as usize] = 0.0;
		}
		for l389 in 0..16384 {
			self.fVec111[(l389) as usize] = 0.0;
		}
		for l390 in 0..2 {
			self.fVec112[(l390) as usize] = 0.0;
		}
		for l391 in 0..2 {
			self.fRec373[(l391) as usize] = 0.0;
		}
		for l392 in 0..2 {
			self.fRec370[(l392) as usize] = 0.0;
		}
		for l393 in 0..2 {
			self.fRec255[(l393) as usize] = 0.0;
		}
		for l394 in 0..2 {
			self.fRec256[(l394) as usize] = 0.0;
		}
		for l395 in 0..2 {
			self.fRec374[(l395) as usize] = 0.0;
		}
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		self.fConst0 = F32::min(1.92e+05, F32::max(1.0, ((self.fSampleRate) as F32)));
		self.fConst1 = 44.1 / self.fConst0;
		self.fConst2 = 1.0 - self.fConst1;
		self.fConst3 = 1.0 / self.fConst0;
		self.fConst4 = 0.5 * self.fConst0;
		self.fConst5 = 0.25 * self.fConst0;
		self.fConst6 = 352.0 / self.fConst0;
		self.fConst7 = 2764.6016 / self.fConst0;
		self.fConst8 = 0.00882353 * self.fConst0;
		self.fConst9 = 0.00073529413 * self.fConst0;
		self.fConst10 = F32::exp(0.0 - 1e+04 / self.fConst0);
		self.fConst11 = 1.0 - self.fConst10;
		self.iConst12 = ((0.1 * self.fConst0) as i32);
		self.fConst13 = F32::exp(0.0 - 5e+01 / self.fConst0);
		self.fConst14 = 6911.504 / self.fConst0;
		self.fConst15 = 0.002 * self.fConst0;
		self.fConst16 = F32::exp(0.0 - 1e+01 / self.fConst0);
		self.fConst17 = 19404.0 / self.fConst0;
		self.fConst18 = 3.1415927 / self.fConst0;
		self.fConst19 = 1e+01 * self.fConst0;
		let mut fConst20: F32 = F32::tan(1382.3008 / self.fConst0);
		let mut fConst21: F32 = mydsp_faustpower2_f(fConst20);
		self.fConst22 = 1.0 / fConst21;
		self.fConst23 = 2.0 * (1.0 - self.fConst22);
		let mut fConst24: F32 = 1.0 / fConst20;
		self.fConst25 = (fConst24 + -0.618034) / fConst20 + 1.0;
		self.fConst26 = 1.0 / ((fConst24 + 0.618034) / fConst20 + 1.0);
		self.fConst27 = (fConst24 + -1.618034) / fConst20 + 1.0;
		self.fConst28 = 1.0 / ((fConst24 + 1.618034) / fConst20 + 1.0);
		let mut fConst29: F32 = F32::tan(25132.742 / self.fConst0);
		let mut fConst30: F32 = mydsp_faustpower2_f(fConst29);
		self.fConst31 = 1.0 / fConst30;
		self.fConst32 = 2.0 * (1.0 - self.fConst31);
		self.fConst33 = 1.0 / fConst29;
		self.fConst34 = (self.fConst33 + -0.618034) / fConst29 + 1.0;
		self.fConst35 = 1.0 / ((self.fConst33 + 0.618034) / fConst29 + 1.0);
		self.fConst36 = (self.fConst33 + -1.618034) / fConst29 + 1.0;
		self.fConst37 = 1.0 / ((self.fConst33 + 1.618034) / fConst29 + 1.0);
		self.fConst38 = 6.2831855 / self.fConst0;
		self.fConst39 = 1.0 - self.fConst33;
		let mut fConst40: F32 = self.fConst33 + 1.0;
		self.fConst41 = 1.0 / fConst40;
		self.fConst42 = 1.0 - fConst24;
		let mut fConst43: F32 = fConst24 + 1.0;
		self.fConst44 = 1.0 / fConst43;
		self.fConst45 = self.fConst42 / fConst43;
		self.fConst46 = 1.0 / (fConst20 * fConst43);
		self.fConst47 = 0.0 - self.fConst46;
		self.fConst48 = 0.0 - 2.0 / fConst21;
		self.fConst49 = (fConst24 + -1.618034) / fConst20 + 1.0;
		self.fConst50 = 1.0 / ((fConst24 + 1.618034) / fConst20 + 1.0);
		self.fConst51 = 0.0 - 1.0 / (fConst29 * fConst40);
		self.fConst52 = 0.0 - 2.0 / fConst30;
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
		ui_interface.add_horizontal_slider("cutoffNote", ParamIndex(1), 0.0, -2e+01, 5e+01, 0.001);
		ui_interface.declare(Some(ParamIndex(2)), "2", "");
		ui_interface.add_horizontal_slider("res", ParamIndex(2), 0.0, 0.0, 0.99, 0.001);
		ui_interface.declare(None, "3", "");
		ui_interface.open_vertical_box("0");
		ui_interface.declare(Some(ParamIndex(3)), "0", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(3), 6e+01, 0.0, 127.0, 0.001);
		ui_interface.declare(Some(ParamIndex(4)), "1", "");
		ui_interface.add_horizontal_slider("volume", ParamIndex(4), 0.0, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "3", "");
		ui_interface.open_vertical_box("1");
		ui_interface.declare(Some(ParamIndex(5)), "0", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(5), 6e+01, 0.0, 127.0, 0.001);
		ui_interface.declare(Some(ParamIndex(6)), "1", "");
		ui_interface.add_horizontal_slider("volume", ParamIndex(6), 0.0, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "3", "");
		ui_interface.open_vertical_box("2");
		ui_interface.declare(Some(ParamIndex(7)), "0", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(7), 6e+01, 0.0, 127.0, 0.001);
		ui_interface.declare(Some(ParamIndex(8)), "1", "");
		ui_interface.add_horizontal_slider("volume", ParamIndex(8), 0.0, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "3", "");
		ui_interface.open_vertical_box("3");
		ui_interface.declare(Some(ParamIndex(9)), "0", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(9), 6e+01, 0.0, 127.0, 0.001);
		ui_interface.declare(Some(ParamIndex(10)), "1", "");
		ui_interface.add_horizontal_slider("volume", ParamIndex(10), 0.0, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.close_box();
		ui_interface.declare(None, "1", "");
		ui_interface.open_horizontal_box("pluck");
		ui_interface.declare(Some(ParamIndex(11)), "2", "");
		ui_interface.add_horizontal_slider("mute", ParamIndex(11), 1.0, 0.9, 1.0, 0.001);
		ui_interface.declare(None, "3", "");
		ui_interface.open_vertical_box("0");
		ui_interface.declare(Some(ParamIndex(12)), "0", "");
		ui_interface.add_button("gate", ParamIndex(12));
		ui_interface.declare(Some(ParamIndex(13)), "1", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(13), 8e+01, 0.0, 127.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "3", "");
		ui_interface.open_vertical_box("1");
		ui_interface.declare(Some(ParamIndex(14)), "0", "");
		ui_interface.add_button("gate", ParamIndex(14));
		ui_interface.declare(Some(ParamIndex(15)), "1", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(15), 8e+01, 0.0, 127.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "3", "");
		ui_interface.open_vertical_box("2");
		ui_interface.declare(Some(ParamIndex(16)), "0", "");
		ui_interface.add_button("gate", ParamIndex(16));
		ui_interface.declare(Some(ParamIndex(17)), "1", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(17), 8e+01, 0.0, 127.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "3", "");
		ui_interface.open_vertical_box("3");
		ui_interface.declare(Some(ParamIndex(18)), "0", "");
		ui_interface.add_button("gate", ParamIndex(18));
		ui_interface.declare(Some(ParamIndex(19)), "1", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(19), 8e+01, 0.0, 127.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(Some(ParamIndex(20)), "3", "");
		ui_interface.add_horizontal_slider("pitchBend", ParamIndex(20), 0.0, -1.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "2", "");
		ui_interface.open_horizontal_box("drone");
		ui_interface.declare(Some(ParamIndex(21)), "0", "");
		ui_interface.add_horizontal_slider("detune", ParamIndex(21), 0.1, 0.0, 0.3, 0.001);
		ui_interface.declare(None, "1", "");
		ui_interface.open_vertical_box("0");
		ui_interface.declare(Some(ParamIndex(22)), "0", "");
		ui_interface.add_horizontal_slider("volume", ParamIndex(22), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(23)), "1", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(23), 6e+01, 0.0, 127.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "1", "");
		ui_interface.open_vertical_box("1");
		ui_interface.declare(Some(ParamIndex(24)), "0", "");
		ui_interface.add_horizontal_slider("volume", ParamIndex(24), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(25)), "1", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(25), 6e+01, 0.0, 127.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "1", "");
		ui_interface.open_vertical_box("2");
		ui_interface.declare(Some(ParamIndex(26)), "0", "");
		ui_interface.add_horizontal_slider("volume", ParamIndex(26), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(27)), "1", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(27), 6e+01, 0.0, 127.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "1", "");
		ui_interface.open_vertical_box("3");
		ui_interface.declare(Some(ParamIndex(28)), "0", "");
		ui_interface.add_horizontal_slider("volume", ParamIndex(28), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(29)), "1", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(29), 6e+01, 0.0, 127.0, 0.001);
		ui_interface.close_box();
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
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			18 => Some(self.fButton0),
			16 => Some(self.fButton1),
			14 => Some(self.fButton2),
			12 => Some(self.fButton3),
			21 => Some(self.fHslider0),
			29 => Some(self.fHslider1),
			43 => Some(self.fHslider10),
			19 => Some(self.fHslider11),
			20 => Some(self.fHslider12),
			11 => Some(self.fHslider13),
			17 => Some(self.fHslider14),
			15 => Some(self.fHslider15),
			13 => Some(self.fHslider16),
			2 => Some(self.fHslider17),
			1 => Some(self.fHslider18),
			9 => Some(self.fHslider19),
			28 => Some(self.fHslider2),
			10 => Some(self.fHslider20),
			7 => Some(self.fHslider21),
			8 => Some(self.fHslider22),
			5 => Some(self.fHslider23),
			6 => Some(self.fHslider24),
			3 => Some(self.fHslider25),
			4 => Some(self.fHslider26),
			42 => Some(self.fHslider27),
			0 => Some(self.fHslider28),
			31 => Some(self.fHslider29),
			27 => Some(self.fHslider3),
			30 => Some(self.fHslider30),
			32 => Some(self.fHslider31),
			33 => Some(self.fHslider32),
			36 => Some(self.fHslider33),
			39 => Some(self.fHslider34),
			38 => Some(self.fHslider35),
			34 => Some(self.fHslider36),
			35 => Some(self.fHslider37),
			37 => Some(self.fHslider38),
			40 => Some(self.fHslider39),
			26 => Some(self.fHslider4),
			25 => Some(self.fHslider5),
			24 => Some(self.fHslider6),
			23 => Some(self.fHslider7),
			22 => Some(self.fHslider8),
			41 => Some(self.fHslider9),
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			18 => { self.fButton0 = value }
			16 => { self.fButton1 = value }
			14 => { self.fButton2 = value }
			12 => { self.fButton3 = value }
			21 => { self.fHslider0 = value }
			29 => { self.fHslider1 = value }
			43 => { self.fHslider10 = value }
			19 => { self.fHslider11 = value }
			20 => { self.fHslider12 = value }
			11 => { self.fHslider13 = value }
			17 => { self.fHslider14 = value }
			15 => { self.fHslider15 = value }
			13 => { self.fHslider16 = value }
			2 => { self.fHslider17 = value }
			1 => { self.fHslider18 = value }
			9 => { self.fHslider19 = value }
			28 => { self.fHslider2 = value }
			10 => { self.fHslider20 = value }
			7 => { self.fHslider21 = value }
			8 => { self.fHslider22 = value }
			5 => { self.fHslider23 = value }
			6 => { self.fHslider24 = value }
			3 => { self.fHslider25 = value }
			4 => { self.fHslider26 = value }
			42 => { self.fHslider27 = value }
			0 => { self.fHslider28 = value }
			31 => { self.fHslider29 = value }
			27 => { self.fHslider3 = value }
			30 => { self.fHslider30 = value }
			32 => { self.fHslider31 = value }
			33 => { self.fHslider32 = value }
			36 => { self.fHslider33 = value }
			39 => { self.fHslider34 = value }
			38 => { self.fHslider35 = value }
			34 => { self.fHslider36 = value }
			35 => { self.fHslider37 = value }
			37 => { self.fHslider38 = value }
			40 => { self.fHslider39 = value }
			26 => { self.fHslider4 = value }
			25 => { self.fHslider5 = value }
			24 => { self.fHslider6 = value }
			23 => { self.fHslider7 = value }
			22 => { self.fHslider8 = value }
			41 => { self.fHslider9 = value }
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
		let mut fSlow0: F32 = self.fHslider0;
		let mut fSlow1: F32 = self.fConst1 * self.fHslider1;
		let mut fSlow2: F32 = self.fConst1 * self.fHslider2;
		let mut fSlow3: F32 = self.fConst1 * self.fHslider3;
		let mut fSlow4: F32 = self.fConst1 * self.fHslider4;
		let mut fSlow5: F32 = self.fConst1 * self.fHslider5;
		let mut fSlow6: F32 = self.fConst1 * self.fHslider6;
		let mut fSlow7: F32 = self.fConst1 * self.fHslider7;
		let mut fSlow8: F32 = self.fConst1 * self.fHslider8;
		let mut fSlow9: F32 = self.fConst1 * self.fHslider9;
		let mut fSlow10: F32 = self.fConst1 * self.fHslider10;
		let mut fSlow11: F32 = self.fConst1 * self.fHslider11;
		let mut fSlow12: F32 = self.fConst1 * self.fHslider12;
		let mut fSlow13: F32 = self.fButton0;
		let mut fSlow14: F32 = self.fHslider13;
		let mut fSlow15: F32 = self.fConst1 * self.fHslider14;
		let mut fSlow16: F32 = self.fButton1;
		let mut fSlow17: F32 = self.fConst1 * self.fHslider15;
		let mut fSlow18: F32 = self.fButton2;
		let mut fSlow19: F32 = self.fConst1 * self.fHslider16;
		let mut fSlow20: F32 = self.fButton3;
		let mut fSlow21: F32 = self.fConst1 * self.fHslider17;
		let mut fSlow22: F32 = self.fConst1 * self.fHslider18;
		let mut fSlow23: F32 = self.fHslider19;
		let mut fSlow24: F32 = self.fConst17 * F32::powf(2.0, 0.083333336 * (fSlow23 + -69.0));
		let mut fSlow25: F32 = self.fConst1 * self.fHslider20;
		let mut fSlow26: F32 = self.fHslider21;
		let mut fSlow27: F32 = self.fConst17 * F32::powf(2.0, 0.083333336 * (fSlow26 + -69.0));
		let mut fSlow28: F32 = self.fConst1 * self.fHslider22;
		let mut fSlow29: F32 = self.fHslider23;
		let mut fSlow30: F32 = self.fConst17 * F32::powf(2.0, 0.083333336 * (fSlow29 + -69.0));
		let mut fSlow31: F32 = self.fConst1 * self.fHslider24;
		let mut fSlow32: F32 = self.fHslider25;
		let mut fSlow33: F32 = self.fConst17 * F32::powf(2.0, 0.083333336 * (fSlow32 + -69.0));
		let mut fSlow34: F32 = self.fConst1 * self.fHslider26;
		let mut fSlow35: F32 = self.fConst1 * self.fHslider27;
		let mut fSlow36: F32 = self.fConst1 * self.fHslider28;
		let mut fSlow37: F32 = self.fConst1 * self.fHslider29;
		let mut fSlow38: F32 = self.fConst1 * self.fHslider30;
		let mut fSlow39: F32 = self.fHslider31;
		let mut fSlow40: F32 = self.fHslider32;
		let mut fSlow41: F32 = 1.0 - fSlow40;
		let mut fSlow42: F32 = self.fHslider33;
		let mut iSlow43: i32 = unsafe { itbl0mydspSIG0[(((134.0 * fSlow42) as i32)) as usize] };
		let mut fSlow44: F32 = 0.005 * ((iSlow43) as F32);
		let mut iSlow45: i32 = unsafe { itbl0mydspSIG0[(((54.0 * fSlow42) as i32)) as usize] };
		let mut fSlow46: F32 = 0.005 * ((iSlow45) as F32);
		let mut iSlow47: i32 = unsafe { itbl0mydspSIG0[(((1e+01 * fSlow42) as i32)) as usize] };
		let mut fSlow48: F32 = 0.0001 * ((iSlow47) as F32);
		let mut iSlow49: i32 = unsafe { itbl0mydspSIG0[(((1.1e+02 * fSlow42) as i32)) as usize] };
		let mut fSlow50: F32 = 0.0001 * ((iSlow49) as F32);
		let mut iSlow51: i32 = unsafe { itbl0mydspSIG0[(((4e+01 * fSlow42) as i32)) as usize] };
		let mut fSlow52: F32 = 0.0001 * ((iSlow51) as F32);
		let mut iSlow53: i32 = unsafe { itbl0mydspSIG0[(((1.4e+02 * fSlow42) as i32)) as usize] };
		let mut fSlow54: F32 = 0.0001 * ((iSlow53) as F32);
		let mut iSlow55: i32 = unsafe { itbl0mydspSIG0[(((7e+01 * fSlow42) as i32)) as usize] };
		let mut fSlow56: F32 = 0.0001 * ((iSlow55) as F32);
		let mut iSlow57: i32 = unsafe { itbl0mydspSIG0[(((1.7e+02 * fSlow42) as i32)) as usize] };
		let mut fSlow58: F32 = 0.0001 * ((iSlow57) as F32);
		let mut iSlow59: i32 = unsafe { itbl0mydspSIG0[(((1e+02 * fSlow42) as i32)) as usize] };
		let mut fSlow60: F32 = 0.0001 * ((iSlow59) as F32);
		let mut iSlow61: i32 = unsafe { itbl0mydspSIG0[(((2e+02 * fSlow42) as i32)) as usize] };
		let mut fSlow62: F32 = 0.0001 * ((iSlow61) as F32);
		let mut iSlow63: i32 = unsafe { itbl0mydspSIG0[(((1.3e+02 * fSlow42) as i32)) as usize] };
		let mut fSlow64: F32 = 0.0001 * ((iSlow63) as F32);
		let mut iSlow65: i32 = unsafe { itbl0mydspSIG0[(((2.3e+02 * fSlow42) as i32)) as usize] };
		let mut fSlow66: F32 = 0.0001 * ((iSlow65) as F32);
		let mut fSlow67: F32 = self.fConst38 * self.fHslider34;
		let mut fSlow68: F32 = F32::cos(fSlow67);
		let mut fSlow69: F32 = F32::sin(fSlow67);
		let mut fSlow70: F32 = 5e+01 * self.fHslider35;
		let mut iSlow71: i32 = unsafe { itbl0mydspSIG0[(((125.0 * fSlow42) as i32)) as usize] };
		let mut fSlow72: F32 = 0.0001 * ((iSlow71) as F32);
		let mut iSlow73: i32 = unsafe { itbl0mydspSIG0[(((204.0 * fSlow42) as i32)) as usize] };
		let mut fSlow74: F32 = 0.005 * ((iSlow73) as F32);
		let mut fSlow75: F32 = 0.0 - fSlow70;
		let mut iSlow76: i32 = unsafe { itbl0mydspSIG0[(((25.0 * fSlow42) as i32)) as usize] };
		let mut fSlow77: F32 = 0.0001 * ((iSlow76) as F32);
		let mut iSlow78: i32 = unsafe { itbl0mydspSIG0[(((155.0 * fSlow42) as i32)) as usize] };
		let mut fSlow79: F32 = 0.0001 * ((iSlow78) as F32);
		let mut iSlow80: i32 = unsafe { itbl0mydspSIG0[(((55.0 * fSlow42) as i32)) as usize] };
		let mut fSlow81: F32 = 0.0001 * ((iSlow80) as F32);
		let mut iSlow82: i32 = unsafe { itbl0mydspSIG0[(((185.0 * fSlow42) as i32)) as usize] };
		let mut fSlow83: F32 = 0.0001 * ((iSlow82) as F32);
		let mut iSlow84: i32 = unsafe { itbl0mydspSIG0[(((85.0 * fSlow42) as i32)) as usize] };
		let mut fSlow85: F32 = 0.0001 * ((iSlow84) as F32);
		let mut iSlow86: i32 = unsafe { itbl0mydspSIG0[(((215.0 * fSlow42) as i32)) as usize] };
		let mut fSlow87: F32 = 0.0001 * ((iSlow86) as F32);
		let mut iSlow88: i32 = unsafe { itbl0mydspSIG0[(((115.0 * fSlow42) as i32)) as usize] };
		let mut fSlow89: F32 = 0.0001 * ((iSlow88) as F32);
		let mut iSlow90: i32 = unsafe { itbl0mydspSIG0[(((245.0 * fSlow42) as i32)) as usize] };
		let mut fSlow91: F32 = 0.0001 * ((iSlow90) as F32);
		let mut iSlow92: i32 = unsafe { itbl0mydspSIG0[(((145.0 * fSlow42) as i32)) as usize] };
		let mut fSlow93: F32 = 0.0001 * ((iSlow92) as F32);
		let mut fSlow94: F32 = F32::powf(1e+01, 0.0 - 0.51 * ((1.25 * fSlow42 + -0.25) / self.fHslider36));
		let mut fSlow95: F32 = self.fHslider37;
		let mut fSlow96: F32 = 1.0 - fSlow95;
		let mut fSlow97: F32 = self.fHslider38;
		let mut fSlow98: F32 = F32::sin(fSlow97);
		let mut iSlow99: i32 = unsafe { itbl0mydspSIG0[(((34.0 * fSlow42) as i32)) as usize] };
		let mut fSlow100: F32 = 0.005 * ((iSlow99) as F32);
		let mut fSlow101: F32 = F32::cos(fSlow97);
		let mut iSlow102: i32 = unsafe { itbl0mydspSIG0[(((2.4e+02 * fSlow42) as i32)) as usize] };
		let mut fSlow103: F32 = 0.0001 * ((iSlow102) as F32);
		let mut iSlow104: i32 = unsafe { itbl0mydspSIG0[(((1.9e+02 * fSlow42) as i32)) as usize] };
		let mut fSlow105: F32 = 0.0001 * ((iSlow104) as F32);
		let mut iSlow106: i32 = unsafe { itbl0mydspSIG0[(((175.0 * fSlow42) as i32)) as usize] };
		let mut fSlow107: F32 = 0.0001 * ((iSlow106) as F32);
		let mut fSlow108: F32 = self.fConst1 * self.fHslider39;
		let zipped_iterators = outputs0.zip(outputs1);
		for (output0, output1) in zipped_iterators {
			self.iVec0[0] = 1;
			self.fRec0[0] = fSlow1 + self.fConst2 * self.fRec0[1];
			let mut fTemp0: F32 = self.fRec0[0] + -69.0;
			let mut fTemp1: F32 = F32::powf(2.0, 0.083333336 * (fTemp0 - fSlow0));
			let mut fTemp2: F32 = F32::max(4.4e+02 * fTemp1, 23.44895);
			let mut fTemp3: F32 = F32::max(2e+01, F32::abs(fTemp2));
			let mut fTemp4: F32 = self.fRec2[1] + self.fConst3 * fTemp3;
			self.fRec2[0] = fTemp4 - F32::floor(fTemp4);
			let mut fTemp5: F32 = mydsp_faustpower2_f(2.0 * self.fRec2[0] + -1.0);
			self.fVec1[0] = fTemp5;
			let mut fTemp6: F32 = ((self.iVec0[1]) as F32);
			let mut fTemp7: F32 = fTemp6 * (fTemp5 - self.fVec1[1]) / fTemp3;
			self.fVec2[(self.IOTA0 & 4095) as usize] = fTemp7;
			let mut fTemp8: F32 = F32::max(0.0, F32::min(2047.0, self.fConst4 / fTemp2));
			let mut iTemp9: i32 = ((fTemp8) as i32);
			let mut fTemp10: F32 = F32::floor(fTemp8);
			self.fRec1[0] = 0.999 * self.fRec1[1] + self.fConst5 * (fTemp7 - self.fVec2[((i32::wrapping_sub(self.IOTA0, iTemp9)) & 4095) as usize] * (fTemp10 + (1.0 - fTemp8)) - (fTemp8 - fTemp10) * self.fVec2[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp9, 1))) & 4095) as usize]);
			let mut fTemp11: F32 = F32::powf(2.0, 0.083333336 * (fSlow0 + fTemp0));
			let mut fTemp12: F32 = F32::max(4.4e+02 * fTemp11, 23.44895);
			let mut fTemp13: F32 = F32::max(2e+01, F32::abs(fTemp12));
			let mut fTemp14: F32 = self.fRec4[1] + self.fConst3 * fTemp13;
			self.fRec4[0] = fTemp14 - F32::floor(fTemp14);
			let mut fTemp15: F32 = mydsp_faustpower2_f(2.0 * self.fRec4[0] + -1.0);
			self.fVec3[0] = fTemp15;
			let mut fTemp16: F32 = fTemp6 * (fTemp15 - self.fVec3[1]) / fTemp13;
			self.fVec4[(self.IOTA0 & 4095) as usize] = fTemp16;
			let mut fTemp17: F32 = F32::max(0.0, F32::min(2047.0, self.fConst4 / fTemp12));
			let mut iTemp18: i32 = ((fTemp17) as i32);
			let mut fTemp19: F32 = F32::floor(fTemp17);
			self.fRec3[0] = 0.999 * self.fRec3[1] + self.fConst5 * (fTemp16 - self.fVec4[((i32::wrapping_sub(self.IOTA0, iTemp18)) & 4095) as usize] * (fTemp19 + (1.0 - fTemp17)) - (fTemp17 - fTemp19) * self.fVec4[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp18, 1))) & 4095) as usize]);
			let mut fTemp20: F32 = F32::powf(2.0, 0.083333336 * fTemp0);
			let mut fTemp21: F32 = F32::max(4.4e+02 * fTemp20, 23.44895);
			let mut fTemp22: F32 = F32::max(2e+01, F32::abs(fTemp21));
			let mut fTemp23: F32 = self.fRec6[1] + self.fConst3 * fTemp22;
			self.fRec6[0] = fTemp23 - F32::floor(fTemp23);
			let mut fTemp24: F32 = mydsp_faustpower2_f(2.0 * self.fRec6[0] + -1.0);
			self.fVec5[0] = fTemp24;
			let mut fTemp25: F32 = fTemp6 * (fTemp24 - self.fVec5[1]) / fTemp22;
			self.fVec6[(self.IOTA0 & 4095) as usize] = fTemp25;
			let mut fTemp26: F32 = F32::max(0.0, F32::min(2047.0, self.fConst4 / fTemp21));
			let mut iTemp27: i32 = ((fTemp26) as i32);
			let mut fTemp28: F32 = F32::floor(fTemp26);
			self.fRec5[0] = 0.999 * self.fRec5[1] - self.fConst5 * (self.fVec6[((i32::wrapping_sub(self.IOTA0, iTemp27)) & 4095) as usize] * (fTemp28 + (1.0 - fTemp26)) - fTemp25 + (fTemp26 - fTemp28) * self.fVec6[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp27, 1))) & 4095) as usize]);
			self.fRec7[0] = fSlow2 + self.fConst2 * self.fRec7[1];
			self.fRec8[0] = fSlow3 + self.fConst2 * self.fRec8[1];
			let mut fTemp29: F32 = self.fRec8[0] + -69.0;
			let mut fTemp30: F32 = F32::powf(2.0, 0.083333336 * (fTemp29 - fSlow0));
			let mut fTemp31: F32 = F32::max(4.4e+02 * fTemp30, 23.44895);
			let mut fTemp32: F32 = F32::max(2e+01, F32::abs(fTemp31));
			let mut fTemp33: F32 = self.fRec10[1] + self.fConst3 * fTemp32;
			self.fRec10[0] = fTemp33 - F32::floor(fTemp33);
			let mut fTemp34: F32 = mydsp_faustpower2_f(2.0 * self.fRec10[0] + -1.0);
			self.fVec7[0] = fTemp34;
			let mut fTemp35: F32 = fTemp6 * (fTemp34 - self.fVec7[1]) / fTemp32;
			self.fVec8[(self.IOTA0 & 4095) as usize] = fTemp35;
			let mut fTemp36: F32 = F32::max(0.0, F32::min(2047.0, self.fConst4 / fTemp31));
			let mut iTemp37: i32 = ((fTemp36) as i32);
			let mut fTemp38: F32 = F32::floor(fTemp36);
			self.fRec9[0] = 0.999 * self.fRec9[1] - self.fConst5 * (self.fVec8[((i32::wrapping_sub(self.IOTA0, iTemp37)) & 4095) as usize] * (fTemp38 + (1.0 - fTemp36)) - fTemp35 + (fTemp36 - fTemp38) * self.fVec8[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp37, 1))) & 4095) as usize]);
			let mut fTemp39: F32 = F32::powf(2.0, 0.083333336 * (fSlow0 + fTemp29));
			let mut fTemp40: F32 = F32::max(4.4e+02 * fTemp39, 23.44895);
			let mut fTemp41: F32 = F32::max(2e+01, F32::abs(fTemp40));
			let mut fTemp42: F32 = self.fRec12[1] + self.fConst3 * fTemp41;
			self.fRec12[0] = fTemp42 - F32::floor(fTemp42);
			let mut fTemp43: F32 = mydsp_faustpower2_f(2.0 * self.fRec12[0] + -1.0);
			self.fVec9[0] = fTemp43;
			let mut fTemp44: F32 = fTemp6 * (fTemp43 - self.fVec9[1]) / fTemp41;
			self.fVec10[(self.IOTA0 & 4095) as usize] = fTemp44;
			let mut fTemp45: F32 = F32::max(0.0, F32::min(2047.0, self.fConst4 / fTemp40));
			let mut iTemp46: i32 = ((fTemp45) as i32);
			let mut fTemp47: F32 = F32::floor(fTemp45);
			self.fRec11[0] = 0.999 * self.fRec11[1] + self.fConst5 * (fTemp44 - self.fVec10[((i32::wrapping_sub(self.IOTA0, iTemp46)) & 4095) as usize] * (fTemp47 + (1.0 - fTemp45)) - (fTemp45 - fTemp47) * self.fVec10[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp46, 1))) & 4095) as usize]);
			let mut fTemp48: F32 = F32::powf(2.0, 0.083333336 * fTemp29);
			let mut fTemp49: F32 = F32::max(4.4e+02 * fTemp48, 23.44895);
			let mut fTemp50: F32 = F32::max(2e+01, F32::abs(fTemp49));
			let mut fTemp51: F32 = self.fRec14[1] + self.fConst3 * fTemp50;
			self.fRec14[0] = fTemp51 - F32::floor(fTemp51);
			let mut fTemp52: F32 = mydsp_faustpower2_f(2.0 * self.fRec14[0] + -1.0);
			self.fVec11[0] = fTemp52;
			let mut fTemp53: F32 = fTemp6 * (fTemp52 - self.fVec11[1]) / fTemp50;
			self.fVec12[(self.IOTA0 & 4095) as usize] = fTemp53;
			let mut fTemp54: F32 = F32::max(0.0, F32::min(2047.0, self.fConst4 / fTemp49));
			let mut iTemp55: i32 = ((fTemp54) as i32);
			let mut fTemp56: F32 = F32::floor(fTemp54);
			self.fRec13[0] = 0.999 * self.fRec13[1] - self.fConst5 * (self.fVec12[((i32::wrapping_sub(self.IOTA0, iTemp55)) & 4095) as usize] * (fTemp56 + (1.0 - fTemp54)) - fTemp53 + (fTemp54 - fTemp56) * self.fVec12[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp55, 1))) & 4095) as usize]);
			self.fRec15[0] = fSlow4 + self.fConst2 * self.fRec15[1];
			self.fRec16[0] = fSlow5 + self.fConst2 * self.fRec16[1];
			let mut fTemp57: F32 = self.fRec16[0] + -69.0;
			let mut fTemp58: F32 = F32::powf(2.0, 0.083333336 * (fTemp57 - fSlow0));
			let mut fTemp59: F32 = F32::max(4.4e+02 * fTemp58, 23.44895);
			let mut fTemp60: F32 = F32::max(2e+01, F32::abs(fTemp59));
			let mut fTemp61: F32 = self.fRec18[1] + self.fConst3 * fTemp60;
			self.fRec18[0] = fTemp61 - F32::floor(fTemp61);
			let mut fTemp62: F32 = mydsp_faustpower2_f(2.0 * self.fRec18[0] + -1.0);
			self.fVec13[0] = fTemp62;
			let mut fTemp63: F32 = fTemp6 * (fTemp62 - self.fVec13[1]) / fTemp60;
			self.fVec14[(self.IOTA0 & 4095) as usize] = fTemp63;
			let mut fTemp64: F32 = F32::max(0.0, F32::min(2047.0, self.fConst4 / fTemp59));
			let mut iTemp65: i32 = ((fTemp64) as i32);
			let mut fTemp66: F32 = F32::floor(fTemp64);
			self.fRec17[0] = 0.999 * self.fRec17[1] + self.fConst5 * (fTemp63 - self.fVec14[((i32::wrapping_sub(self.IOTA0, iTemp65)) & 4095) as usize] * (fTemp66 + (1.0 - fTemp64)) - (fTemp64 - fTemp66) * self.fVec14[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp65, 1))) & 4095) as usize]);
			let mut fTemp67: F32 = F32::powf(2.0, 0.083333336 * (fSlow0 + fTemp57));
			let mut fTemp68: F32 = F32::max(4.4e+02 * fTemp67, 23.44895);
			let mut fTemp69: F32 = F32::max(2e+01, F32::abs(fTemp68));
			let mut fTemp70: F32 = self.fRec20[1] + self.fConst3 * fTemp69;
			self.fRec20[0] = fTemp70 - F32::floor(fTemp70);
			let mut fTemp71: F32 = mydsp_faustpower2_f(2.0 * self.fRec20[0] + -1.0);
			self.fVec15[0] = fTemp71;
			let mut fTemp72: F32 = fTemp6 * (fTemp71 - self.fVec15[1]) / fTemp69;
			self.fVec16[(self.IOTA0 & 4095) as usize] = fTemp72;
			let mut fTemp73: F32 = F32::max(0.0, F32::min(2047.0, self.fConst4 / fTemp68));
			let mut iTemp74: i32 = ((fTemp73) as i32);
			let mut fTemp75: F32 = F32::floor(fTemp73);
			self.fRec19[0] = 0.999 * self.fRec19[1] + self.fConst5 * (fTemp72 - self.fVec16[((i32::wrapping_sub(self.IOTA0, iTemp74)) & 4095) as usize] * (fTemp75 + (1.0 - fTemp73)) - (fTemp73 - fTemp75) * self.fVec16[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp74, 1))) & 4095) as usize]);
			let mut fTemp76: F32 = F32::powf(2.0, 0.083333336 * fTemp57);
			let mut fTemp77: F32 = F32::max(4.4e+02 * fTemp76, 23.44895);
			let mut fTemp78: F32 = F32::max(2e+01, F32::abs(fTemp77));
			let mut fTemp79: F32 = self.fRec22[1] + self.fConst3 * fTemp78;
			self.fRec22[0] = fTemp79 - F32::floor(fTemp79);
			let mut fTemp80: F32 = mydsp_faustpower2_f(2.0 * self.fRec22[0] + -1.0);
			self.fVec17[0] = fTemp80;
			let mut fTemp81: F32 = fTemp6 * (fTemp80 - self.fVec17[1]) / fTemp78;
			self.fVec18[(self.IOTA0 & 4095) as usize] = fTemp81;
			let mut fTemp82: F32 = F32::max(0.0, F32::min(2047.0, self.fConst4 / fTemp77));
			let mut iTemp83: i32 = ((fTemp82) as i32);
			let mut fTemp84: F32 = F32::floor(fTemp82);
			self.fRec21[0] = 0.999 * self.fRec21[1] + self.fConst5 * (fTemp81 - self.fVec18[((i32::wrapping_sub(self.IOTA0, iTemp83)) & 4095) as usize] * (fTemp84 + (1.0 - fTemp82)) - (fTemp82 - fTemp84) * self.fVec18[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp83, 1))) & 4095) as usize]);
			self.fRec23[0] = fSlow6 + self.fConst2 * self.fRec23[1];
			self.fRec24[0] = fSlow7 + self.fConst2 * self.fRec24[1];
			let mut fTemp85: F32 = self.fRec24[0] + -69.0;
			let mut fTemp86: F32 = F32::powf(2.0, 0.083333336 * (fTemp85 - fSlow0));
			let mut fTemp87: F32 = F32::max(4.4e+02 * fTemp86, 23.44895);
			let mut fTemp88: F32 = F32::max(2e+01, F32::abs(fTemp87));
			let mut fTemp89: F32 = self.fRec26[1] + self.fConst3 * fTemp88;
			self.fRec26[0] = fTemp89 - F32::floor(fTemp89);
			let mut fTemp90: F32 = mydsp_faustpower2_f(2.0 * self.fRec26[0] + -1.0);
			self.fVec19[0] = fTemp90;
			let mut fTemp91: F32 = fTemp6 * (fTemp90 - self.fVec19[1]) / fTemp88;
			self.fVec20[(self.IOTA0 & 4095) as usize] = fTemp91;
			let mut fTemp92: F32 = F32::max(0.0, F32::min(2047.0, self.fConst4 / fTemp87));
			let mut iTemp93: i32 = ((fTemp92) as i32);
			let mut fTemp94: F32 = F32::floor(fTemp92);
			self.fRec25[0] = 0.999 * self.fRec25[1] + self.fConst5 * (fTemp91 - self.fVec20[((i32::wrapping_sub(self.IOTA0, iTemp93)) & 4095) as usize] * (fTemp94 + (1.0 - fTemp92)) - (fTemp92 - fTemp94) * self.fVec20[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp93, 1))) & 4095) as usize]);
			let mut fTemp95: F32 = F32::powf(2.0, 0.083333336 * (fSlow0 + fTemp85));
			let mut fTemp96: F32 = F32::max(4.4e+02 * fTemp95, 23.44895);
			let mut fTemp97: F32 = F32::max(2e+01, F32::abs(fTemp96));
			let mut fTemp98: F32 = self.fRec28[1] + self.fConst3 * fTemp97;
			self.fRec28[0] = fTemp98 - F32::floor(fTemp98);
			let mut fTemp99: F32 = mydsp_faustpower2_f(2.0 * self.fRec28[0] + -1.0);
			self.fVec21[0] = fTemp99;
			let mut fTemp100: F32 = fTemp6 * (fTemp99 - self.fVec21[1]) / fTemp97;
			self.fVec22[(self.IOTA0 & 4095) as usize] = fTemp100;
			let mut fTemp101: F32 = F32::max(0.0, F32::min(2047.0, self.fConst4 / fTemp96));
			let mut iTemp102: i32 = ((fTemp101) as i32);
			let mut fTemp103: F32 = F32::floor(fTemp101);
			self.fRec27[0] = 0.999 * self.fRec27[1] - self.fConst5 * (self.fVec22[((i32::wrapping_sub(self.IOTA0, iTemp102)) & 4095) as usize] * (fTemp103 + (1.0 - fTemp101)) - fTemp100 + (fTemp101 - fTemp103) * self.fVec22[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp102, 1))) & 4095) as usize]);
			let mut fTemp104: F32 = F32::powf(2.0, 0.083333336 * fTemp85);
			let mut fTemp105: F32 = F32::max(4.4e+02 * fTemp104, 23.44895);
			let mut fTemp106: F32 = F32::max(2e+01, F32::abs(fTemp105));
			let mut fTemp107: F32 = self.fRec30[1] + self.fConst3 * fTemp106;
			self.fRec30[0] = fTemp107 - F32::floor(fTemp107);
			let mut fTemp108: F32 = mydsp_faustpower2_f(2.0 * self.fRec30[0] + -1.0);
			self.fVec23[0] = fTemp108;
			let mut fTemp109: F32 = fTemp6 * (fTemp108 - self.fVec23[1]) / fTemp106;
			self.fVec24[(self.IOTA0 & 4095) as usize] = fTemp109;
			let mut fTemp110: F32 = F32::max(0.0, F32::min(2047.0, self.fConst4 / fTemp105));
			let mut iTemp111: i32 = ((fTemp110) as i32);
			let mut fTemp112: F32 = F32::floor(fTemp110);
			self.fRec29[0] = 0.999 * self.fRec29[1] + self.fConst5 * (fTemp109 - self.fVec24[((i32::wrapping_sub(self.IOTA0, iTemp111)) & 4095) as usize] * (fTemp112 + (1.0 - fTemp110)) - (fTemp110 - fTemp112) * self.fVec24[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp111, 1))) & 4095) as usize]);
			self.fRec31[0] = fSlow8 + self.fConst2 * self.fRec31[1];
			self.fRec32[0] = fSlow9 + self.fConst2 * self.fRec32[1];
			let mut fTemp113: F32 = self.fConst6 * self.fRec32[0] * (self.fRec31[0] * (self.fRec29[0] * fTemp104 + self.fRec27[0] * fTemp95 + self.fRec25[0] * fTemp86) + self.fRec23[0] * (self.fRec21[0] * fTemp76 + self.fRec19[0] * fTemp67 + self.fRec17[0] * fTemp58) + self.fRec15[0] * (self.fRec13[0] * fTemp48 + self.fRec11[0] * fTemp39 + self.fRec9[0] * fTemp30) + self.fRec7[0] * (self.fRec5[0] * fTemp20 + self.fRec3[0] * fTemp11 + self.fRec1[0] * fTemp1));
			self.fRec33[0] = fSlow10 + self.fConst2 * self.fRec33[1];
			self.fRec35[0] = fSlow11 + self.fConst2 * self.fRec35[1];
			self.fRec36[0] = fSlow12 + self.fConst2 * self.fRec36[1];
			let mut fTemp114: F32 = F32::powf(2.0, 0.083333336 * (self.fRec36[0] + self.fRec35[0] + -69.0));
			let mut fTemp115: F32 = 1.0 / F32::tan(self.fConst7 * fTemp114);
			let mut fRec51: F32 = -1.0 * 0.9973053 * (0.9 * self.fRec52[2] + 0.05 * (self.fRec52[1] + self.fRec52[3]));
			let mut fTemp116: F32 = self.fConst9 * (0.77272725 / fTemp114 + -0.11);
			let mut fTemp117: F32 = fTemp116 + -1.499995;
			let mut iTemp118: i32 = ((fTemp117) as i32);
			let mut iTemp119: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, i32::wrapping_add(iTemp118, 4))) as F32))) as i32);
			let mut iTemp120: i32 = i32::wrapping_add(iTemp119, 1);
			let mut fTemp121: F32 = F32::floor(fTemp117);
			let mut fTemp122: F32 = fTemp116 + (-3.0 - fTemp121);
			let mut fTemp123: F32 = fTemp116 + (-2.0 - fTemp121);
			let mut fTemp124: F32 = fTemp116 + (-1.0 - fTemp121);
			let mut fTemp125: F32 = fTemp124 * fTemp123;
			let mut fTemp126: F32 = fTemp125 * fTemp122;
			let mut fTemp127: F32 = fTemp116 + (-4.0 - fTemp121);
			let mut fTemp128: F32 = 0.0 - fTemp127;
			let mut iTemp129: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, i32::wrapping_add(iTemp118, 3))) as F32))) as i32);
			let mut iTemp130: i32 = i32::wrapping_add(iTemp129, 1);
			let mut fTemp131: F32 = 0.0 - 0.5 * fTemp127;
			let mut fTemp132: F32 = 0.0 - fTemp122;
			let mut iTemp133: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, i32::wrapping_add(iTemp118, 2))) as F32))) as i32);
			let mut iTemp134: i32 = i32::wrapping_add(iTemp133, 1);
			let mut fTemp135: F32 = 0.0 - 0.33333334 * fTemp127;
			let mut fTemp136: F32 = 0.0 - 0.5 * fTemp122;
			let mut fTemp137: F32 = 0.0 - fTemp123;
			let mut iTemp138: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, i32::wrapping_add(iTemp118, 1))) as F32))) as i32);
			let mut iTemp139: i32 = i32::wrapping_add(iTemp138, 1);
			let mut fTemp140: F32 = fTemp116 - fTemp121;
			let mut fTemp141: F32 = 0.0 - 0.25 * fTemp127;
			let mut fTemp142: F32 = 0.0 - 0.33333334 * fTemp122;
			let mut fTemp143: F32 = 0.0 - 0.5 * fTemp123;
			let mut fTemp144: F32 = 0.0 - fTemp124;
			let mut iTemp145: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, iTemp118)) as F32))) as i32);
			let mut iTemp146: i32 = i32::wrapping_add(iTemp145, 1);
			self.fRec66[0] = self.fRec43[((i32::wrapping_sub(self.IOTA0, iTemp146)) & 2047) as usize] * fTemp144 * fTemp143 * fTemp142 * fTemp141 + fTemp140 * (self.fRec43[((i32::wrapping_sub(self.IOTA0, iTemp139)) & 2047) as usize] * fTemp137 * fTemp136 * fTemp135 + 0.5 * fTemp124 * self.fRec43[((i32::wrapping_sub(self.IOTA0, iTemp134)) & 2047) as usize] * fTemp132 * fTemp131 + 0.16666667 * fTemp125 * self.fRec43[((i32::wrapping_sub(self.IOTA0, iTemp130)) & 2047) as usize] * fTemp128 + 0.041666668 * fTemp126 * self.fRec43[((i32::wrapping_sub(self.IOTA0, iTemp120)) & 2047) as usize]);
			self.fRec70[0] = 0.05 * self.fRec70[1] + 0.95 * self.fRec66[1];
			let mut fRec67: F32 = self.fRec70[0];
			self.fRec75[0] = self.fConst10 * self.fRec75[1] + self.fConst11 * F32::abs(self.fRec37[1]);
			let mut fRec74: F32 = self.fRec75[0];
			let mut iTemp147: i32 = ((fRec74 > 0.1) as i32);
			self.iVec25[0] = iTemp147;
			self.iRec76[0] = std::cmp::max(i32::wrapping_mul(self.iConst12, ((iTemp147 < self.iVec25[1]) as i32)), i32::wrapping_add(self.iRec76[1], -1));
			let mut fTemp148: F32 = F32::abs(F32::max(((iTemp147) as F32), ((((self.iRec76[0] > 0) as i32)) as F32)));
			let mut fTemp149: F32 = if (((self.fRec72[1] > fTemp148) as i32) as i32 != 0) { self.fConst13 } else { self.fConst10 };
			self.fRec73[0] = self.fRec73[1] * fTemp149 + fTemp148 * (1.0 - fTemp149);
			self.fRec72[0] = self.fRec73[0];
			let mut fTemp150: F32 = 0.005 * self.fRec72[0] * self.fRec37[1];
			self.fRec77[0] = self.fRec41[1];
			self.fRec78[(self.IOTA0 & 2047) as usize] = -1.0 * 0.9973053 * (0.9 * self.fRec77[2] + 0.05 * (self.fRec77[1] + self.fRec77[3]));
			let mut fTemp151: F32 = fTemp125 * fTemp128;
			let mut fTemp152: F32 = fTemp124 * fTemp132 * fTemp131;
			let mut fTemp153: F32 = fTemp137 * fTemp136 * fTemp135;
			let mut fTemp154: F32 = fTemp144 * fTemp143 * fTemp142 * fTemp141;
			self.fVec26[0] = fTemp154 * self.fRec78[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp145, 2))) & 2047) as usize] + fTemp140 * (fTemp153 * self.fRec78[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp138, 2))) & 2047) as usize] + 0.5 * fTemp152 * self.fRec78[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp133, 2))) & 2047) as usize] + 0.16666667 * fTemp151 * self.fRec78[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp129, 2))) & 2047) as usize] + 0.041666668 * fTemp126 * self.fRec78[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp119, 2))) & 2047) as usize]);
			let mut fTemp155: F32 = F32::tan(self.fConst14 * fTemp114);
			let mut fTemp156: F32 = 1.0 / fTemp155;
			let mut fTemp157: F32 = (fTemp156 + 1.4142135) / fTemp155 + 1.0;
			self.fVec27[0] = fSlow13;
			self.iRec79[0] = i32::wrapping_add(i32::wrapping_mul(i32::wrapping_add(self.iRec79[1], ((self.iRec79[1] > 0) as i32)), ((fSlow13 <= self.fVec27[1]) as i32)), ((fSlow13 > self.fVec27[1]) as i32));
			let mut fTemp158: F32 = ((self.iRec79[0]) as F32) / F32::max(1.0, self.fConst15 * mydsp_faustpower2_f(1.0 - 0.22 * fTemp114));
			self.iRec81[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, self.iRec81[1]), 12345);
			let mut fTemp159: F32 = 4.656613e-10 * ((self.iRec81[0]) as F32);
			self.fRec80[0] = fTemp159 - (self.fRec80[2] * ((fTemp156 + -1.4142135) / fTemp155 + 1.0) + 2.0 * self.fRec80[1] * (1.0 - 1.0 / mydsp_faustpower2_f(fTemp155))) / fTemp157;
			let mut fTemp160: F32 = 0.5 * ((self.fRec80[2] + self.fRec80[0] + 2.0 * self.fRec80[1]) * F32::max(0.0, F32::min(fTemp158, 2.0 - fTemp158)) / fTemp157);
			let mut fTemp161: F32 = fTemp160 + self.fVec26[1] + fTemp150;
			self.fVec28[0] = fTemp161;
			self.fRec71[(self.IOTA0 & 2047) as usize] = 0.05 * self.fRec71[((i32::wrapping_sub(self.IOTA0, 1)) & 2047) as usize] + 0.95 * self.fVec28[2];
			let mut fRec68: F32 = fTemp154 * self.fRec71[((i32::wrapping_sub(self.IOTA0, iTemp145)) & 2047) as usize] + fTemp140 * (fTemp153 * self.fRec71[((i32::wrapping_sub(self.IOTA0, iTemp138)) & 2047) as usize] + 0.5 * fTemp152 * self.fRec71[((i32::wrapping_sub(self.IOTA0, iTemp133)) & 2047) as usize] + 0.16666667 * fTemp151 * self.fRec71[((i32::wrapping_sub(self.IOTA0, iTemp129)) & 2047) as usize] + 0.041666668 * fTemp126 * self.fRec71[((i32::wrapping_sub(self.IOTA0, iTemp119)) & 2047) as usize]);
			let mut fRec69: F32 = self.fVec28[1] + self.fRec62[1];
			self.fRec62[0] = fRec67;
			let mut fRec63: F32 = self.fRec62[1];
			let mut fRec64: F32 = fRec68;
			let mut fRec65: F32 = fRec69;
			self.fRec58[0] = fRec63;
			let mut fRec59: F32 = fTemp150 + fTemp160 + self.fRec58[1];
			let mut fRec60: F32 = fRec64;
			let mut fRec61: F32 = fRec65;
			self.fRec54[(self.IOTA0 & 2047) as usize] = fRec59;
			let mut fRec55: F32 = fTemp154 * self.fRec54[((i32::wrapping_sub(self.IOTA0, iTemp146)) & 2047) as usize] + fTemp140 * (fTemp153 * self.fRec54[((i32::wrapping_sub(self.IOTA0, iTemp139)) & 2047) as usize] + 0.5 * fTemp152 * self.fRec54[((i32::wrapping_sub(self.IOTA0, iTemp134)) & 2047) as usize] + 0.16666667 * fTemp151 * self.fRec54[((i32::wrapping_sub(self.IOTA0, iTemp130)) & 2047) as usize] + 0.041666668 * fTemp126 * self.fRec54[((i32::wrapping_sub(self.IOTA0, iTemp120)) & 2047) as usize]);
			self.fRec56[0] = fRec60;
			let mut fRec57: F32 = fRec61;
			self.fRec52[0] = fSlow14 * self.fRec56[1];
			let mut fRec53: F32 = fRec57;
			self.fRec47[0] = fRec51;
			let mut fRec48: F32 = fSlow14 * self.fRec47[1];
			let mut fRec49: F32 = self.fRec52[0];
			let mut fRec50: F32 = fRec53;
			self.fRec43[(self.IOTA0 & 2047) as usize] = fRec48;
			let mut fRec44: F32 = fRec55;
			let mut fRec45: F32 = fRec49;
			let mut fRec46: F32 = fRec50;
			self.fRec41[0] = fRec44;
			let mut fRec42: F32 = fRec46;
			let mut fTemp162: F32 = F32::abs(fRec42);
			let mut fTemp163: F32 = if (((self.fRec39[1] > fTemp162) as i32) as i32 != 0) { self.fConst16 } else { 0.0 };
			self.fRec40[0] = self.fRec40[1] * fTemp163 + fTemp162 * (1.0 - fTemp163);
			self.fRec39[0] = self.fRec40[0];
			let mut fRec38: F32 = 0.0 - 0.95 * F32::max(2e+01 * F32::log10(self.fRec39[0]) + 1e+01, 0.0);
			self.fRec37[0] = fRec42 * F32::powf(1e+01, 0.05 * fRec38);
			self.fRec34[0] = (self.fRec37[0] + self.fRec37[1] - self.fRec34[1] * (1.0 - fTemp115)) / (fTemp115 + 1.0);
			self.fRec83[0] = fSlow15 + self.fConst2 * self.fRec83[1];
			let mut fTemp164: F32 = F32::powf(2.0, 0.083333336 * (self.fRec36[0] + self.fRec83[0] + -69.0));
			let mut fTemp165: F32 = 1.0 / F32::tan(self.fConst7 * fTemp164);
			let mut fRec98: F32 = -1.0 * 0.9973053 * (0.9 * self.fRec99[2] + 0.05 * (self.fRec99[1] + self.fRec99[3]));
			let mut fTemp166: F32 = self.fConst9 * (0.77272725 / fTemp164 + -0.11);
			let mut fTemp167: F32 = fTemp166 + -1.499995;
			let mut iTemp168: i32 = ((fTemp167) as i32);
			let mut iTemp169: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, i32::wrapping_add(iTemp168, 4))) as F32))) as i32);
			let mut iTemp170: i32 = i32::wrapping_add(iTemp169, 1);
			let mut fTemp171: F32 = F32::floor(fTemp167);
			let mut fTemp172: F32 = fTemp166 + (-3.0 - fTemp171);
			let mut fTemp173: F32 = fTemp166 + (-2.0 - fTemp171);
			let mut fTemp174: F32 = fTemp166 + (-1.0 - fTemp171);
			let mut fTemp175: F32 = fTemp174 * fTemp173;
			let mut fTemp176: F32 = fTemp175 * fTemp172;
			let mut fTemp177: F32 = fTemp166 + (-4.0 - fTemp171);
			let mut fTemp178: F32 = 0.0 - fTemp177;
			let mut iTemp179: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, i32::wrapping_add(iTemp168, 3))) as F32))) as i32);
			let mut iTemp180: i32 = i32::wrapping_add(iTemp179, 1);
			let mut fTemp181: F32 = 0.0 - 0.5 * fTemp177;
			let mut fTemp182: F32 = 0.0 - fTemp172;
			let mut iTemp183: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, i32::wrapping_add(iTemp168, 2))) as F32))) as i32);
			let mut iTemp184: i32 = i32::wrapping_add(iTemp183, 1);
			let mut fTemp185: F32 = 0.0 - 0.33333334 * fTemp177;
			let mut fTemp186: F32 = 0.0 - 0.5 * fTemp172;
			let mut fTemp187: F32 = 0.0 - fTemp173;
			let mut iTemp188: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, i32::wrapping_add(iTemp168, 1))) as F32))) as i32);
			let mut iTemp189: i32 = i32::wrapping_add(iTemp188, 1);
			let mut fTemp190: F32 = fTemp166 - fTemp171;
			let mut fTemp191: F32 = 0.0 - 0.25 * fTemp177;
			let mut fTemp192: F32 = 0.0 - 0.33333334 * fTemp172;
			let mut fTemp193: F32 = 0.0 - 0.5 * fTemp173;
			let mut fTemp194: F32 = 0.0 - fTemp174;
			let mut iTemp195: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, iTemp168)) as F32))) as i32);
			let mut iTemp196: i32 = i32::wrapping_add(iTemp195, 1);
			self.fRec113[0] = self.fRec90[((i32::wrapping_sub(self.IOTA0, iTemp196)) & 2047) as usize] * fTemp194 * fTemp193 * fTemp192 * fTemp191 + fTemp190 * (self.fRec90[((i32::wrapping_sub(self.IOTA0, iTemp189)) & 2047) as usize] * fTemp187 * fTemp186 * fTemp185 + 0.5 * fTemp174 * self.fRec90[((i32::wrapping_sub(self.IOTA0, iTemp184)) & 2047) as usize] * fTemp182 * fTemp181 + 0.16666667 * fTemp175 * self.fRec90[((i32::wrapping_sub(self.IOTA0, iTemp180)) & 2047) as usize] * fTemp178 + 0.041666668 * fTemp176 * self.fRec90[((i32::wrapping_sub(self.IOTA0, iTemp170)) & 2047) as usize]);
			self.fRec117[0] = 0.05 * self.fRec117[1] + 0.95 * self.fRec113[1];
			let mut fRec114: F32 = self.fRec117[0];
			self.fRec122[0] = self.fConst10 * self.fRec122[1] + self.fConst11 * F32::abs(self.fRec84[1]);
			let mut fRec121: F32 = self.fRec122[0];
			let mut iTemp197: i32 = ((fRec121 > 0.1) as i32);
			self.iVec29[0] = iTemp197;
			self.iRec123[0] = std::cmp::max(i32::wrapping_mul(self.iConst12, ((iTemp197 < self.iVec29[1]) as i32)), i32::wrapping_add(self.iRec123[1], -1));
			let mut fTemp198: F32 = F32::abs(F32::max(((iTemp197) as F32), ((((self.iRec123[0] > 0) as i32)) as F32)));
			let mut fTemp199: F32 = if (((self.fRec119[1] > fTemp198) as i32) as i32 != 0) { self.fConst13 } else { self.fConst10 };
			self.fRec120[0] = self.fRec120[1] * fTemp199 + fTemp198 * (1.0 - fTemp199);
			self.fRec119[0] = self.fRec120[0];
			let mut fTemp200: F32 = 0.005 * self.fRec119[0] * self.fRec84[1];
			self.fRec124[0] = self.fRec88[1];
			self.fRec125[(self.IOTA0 & 2047) as usize] = -1.0 * 0.9973053 * (0.9 * self.fRec124[2] + 0.05 * (self.fRec124[1] + self.fRec124[3]));
			let mut fTemp201: F32 = fTemp175 * fTemp178;
			let mut fTemp202: F32 = fTemp174 * fTemp182 * fTemp181;
			let mut fTemp203: F32 = fTemp187 * fTemp186 * fTemp185;
			let mut fTemp204: F32 = fTemp194 * fTemp193 * fTemp192 * fTemp191;
			self.fVec30[0] = fTemp204 * self.fRec125[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp195, 2))) & 2047) as usize] + fTemp190 * (fTemp203 * self.fRec125[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp188, 2))) & 2047) as usize] + 0.5 * fTemp202 * self.fRec125[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp183, 2))) & 2047) as usize] + 0.16666667 * fTemp201 * self.fRec125[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp179, 2))) & 2047) as usize] + 0.041666668 * fTemp176 * self.fRec125[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp169, 2))) & 2047) as usize]);
			let mut fTemp205: F32 = F32::tan(self.fConst14 * fTemp164);
			let mut fTemp206: F32 = 1.0 / fTemp205;
			let mut fTemp207: F32 = (fTemp206 + 1.4142135) / fTemp205 + 1.0;
			self.fVec31[0] = fSlow16;
			self.iRec126[0] = i32::wrapping_add(i32::wrapping_mul(i32::wrapping_add(self.iRec126[1], ((self.iRec126[1] > 0) as i32)), ((fSlow16 <= self.fVec31[1]) as i32)), ((fSlow16 > self.fVec31[1]) as i32));
			let mut fTemp208: F32 = ((self.iRec126[0]) as F32) / F32::max(1.0, self.fConst15 * mydsp_faustpower2_f(1.0 - 0.22 * fTemp164));
			self.fRec127[0] = fTemp159 - (self.fRec127[2] * ((fTemp206 + -1.4142135) / fTemp205 + 1.0) + 2.0 * self.fRec127[1] * (1.0 - 1.0 / mydsp_faustpower2_f(fTemp205))) / fTemp207;
			let mut fTemp209: F32 = 0.5 * ((self.fRec127[2] + self.fRec127[0] + 2.0 * self.fRec127[1]) * F32::max(0.0, F32::min(fTemp208, 2.0 - fTemp208)) / fTemp207);
			let mut fTemp210: F32 = fTemp209 + self.fVec30[1] + fTemp200;
			self.fVec32[0] = fTemp210;
			self.fRec118[(self.IOTA0 & 2047) as usize] = 0.05 * self.fRec118[((i32::wrapping_sub(self.IOTA0, 1)) & 2047) as usize] + 0.95 * self.fVec32[2];
			let mut fRec115: F32 = fTemp204 * self.fRec118[((i32::wrapping_sub(self.IOTA0, iTemp195)) & 2047) as usize] + fTemp190 * (fTemp203 * self.fRec118[((i32::wrapping_sub(self.IOTA0, iTemp188)) & 2047) as usize] + 0.5 * fTemp202 * self.fRec118[((i32::wrapping_sub(self.IOTA0, iTemp183)) & 2047) as usize] + 0.16666667 * fTemp201 * self.fRec118[((i32::wrapping_sub(self.IOTA0, iTemp179)) & 2047) as usize] + 0.041666668 * fTemp176 * self.fRec118[((i32::wrapping_sub(self.IOTA0, iTemp169)) & 2047) as usize]);
			let mut fRec116: F32 = self.fVec32[1] + self.fRec109[1];
			self.fRec109[0] = fRec114;
			let mut fRec110: F32 = self.fRec109[1];
			let mut fRec111: F32 = fRec115;
			let mut fRec112: F32 = fRec116;
			self.fRec105[0] = fRec110;
			let mut fRec106: F32 = fTemp200 + fTemp209 + self.fRec105[1];
			let mut fRec107: F32 = fRec111;
			let mut fRec108: F32 = fRec112;
			self.fRec101[(self.IOTA0 & 2047) as usize] = fRec106;
			let mut fRec102: F32 = fTemp204 * self.fRec101[((i32::wrapping_sub(self.IOTA0, iTemp196)) & 2047) as usize] + fTemp190 * (fTemp203 * self.fRec101[((i32::wrapping_sub(self.IOTA0, iTemp189)) & 2047) as usize] + 0.5 * fTemp202 * self.fRec101[((i32::wrapping_sub(self.IOTA0, iTemp184)) & 2047) as usize] + 0.16666667 * fTemp201 * self.fRec101[((i32::wrapping_sub(self.IOTA0, iTemp180)) & 2047) as usize] + 0.041666668 * fTemp176 * self.fRec101[((i32::wrapping_sub(self.IOTA0, iTemp170)) & 2047) as usize]);
			self.fRec103[0] = fRec107;
			let mut fRec104: F32 = fRec108;
			self.fRec99[0] = fSlow14 * self.fRec103[1];
			let mut fRec100: F32 = fRec104;
			self.fRec94[0] = fRec98;
			let mut fRec95: F32 = fSlow14 * self.fRec94[1];
			let mut fRec96: F32 = self.fRec99[0];
			let mut fRec97: F32 = fRec100;
			self.fRec90[(self.IOTA0 & 2047) as usize] = fRec95;
			let mut fRec91: F32 = fRec102;
			let mut fRec92: F32 = fRec96;
			let mut fRec93: F32 = fRec97;
			self.fRec88[0] = fRec91;
			let mut fRec89: F32 = fRec93;
			let mut fTemp211: F32 = F32::abs(fRec89);
			let mut fTemp212: F32 = if (((self.fRec86[1] > fTemp211) as i32) as i32 != 0) { self.fConst16 } else { 0.0 };
			self.fRec87[0] = self.fRec87[1] * fTemp212 + fTemp211 * (1.0 - fTemp212);
			self.fRec86[0] = self.fRec87[0];
			let mut fRec85: F32 = 0.0 - 0.95 * F32::max(2e+01 * F32::log10(self.fRec86[0]) + 1e+01, 0.0);
			self.fRec84[0] = fRec89 * F32::powf(1e+01, 0.05 * fRec85);
			self.fRec82[0] = 0.0 - (self.fRec82[1] * (1.0 - fTemp165) - (self.fRec84[0] + self.fRec84[1])) / (fTemp165 + 1.0);
			self.fRec129[0] = fSlow17 + self.fConst2 * self.fRec129[1];
			let mut fTemp213: F32 = F32::powf(2.0, 0.083333336 * (self.fRec36[0] + self.fRec129[0] + -69.0));
			let mut fTemp214: F32 = 1.0 / F32::tan(self.fConst7 * fTemp213);
			let mut fRec144: F32 = -1.0 * 0.9973053 * (0.9 * self.fRec145[2] + 0.05 * (self.fRec145[1] + self.fRec145[3]));
			let mut fTemp215: F32 = self.fConst9 * (0.77272725 / fTemp213 + -0.11);
			let mut fTemp216: F32 = fTemp215 + -1.499995;
			let mut iTemp217: i32 = ((fTemp216) as i32);
			let mut iTemp218: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, i32::wrapping_add(iTemp217, 4))) as F32))) as i32);
			let mut iTemp219: i32 = i32::wrapping_add(iTemp218, 1);
			let mut fTemp220: F32 = F32::floor(fTemp216);
			let mut fTemp221: F32 = fTemp215 + (-3.0 - fTemp220);
			let mut fTemp222: F32 = fTemp215 + (-2.0 - fTemp220);
			let mut fTemp223: F32 = fTemp215 + (-1.0 - fTemp220);
			let mut fTemp224: F32 = fTemp223 * fTemp222;
			let mut fTemp225: F32 = fTemp224 * fTemp221;
			let mut fTemp226: F32 = fTemp215 + (-4.0 - fTemp220);
			let mut fTemp227: F32 = 0.0 - fTemp226;
			let mut iTemp228: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, i32::wrapping_add(iTemp217, 3))) as F32))) as i32);
			let mut iTemp229: i32 = i32::wrapping_add(iTemp228, 1);
			let mut fTemp230: F32 = 0.0 - 0.5 * fTemp226;
			let mut fTemp231: F32 = 0.0 - fTemp221;
			let mut iTemp232: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, i32::wrapping_add(iTemp217, 2))) as F32))) as i32);
			let mut iTemp233: i32 = i32::wrapping_add(iTemp232, 1);
			let mut fTemp234: F32 = 0.0 - 0.33333334 * fTemp226;
			let mut fTemp235: F32 = 0.0 - 0.5 * fTemp221;
			let mut fTemp236: F32 = 0.0 - fTemp222;
			let mut iTemp237: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, i32::wrapping_add(iTemp217, 1))) as F32))) as i32);
			let mut iTemp238: i32 = i32::wrapping_add(iTemp237, 1);
			let mut fTemp239: F32 = fTemp215 - fTemp220;
			let mut fTemp240: F32 = 0.0 - 0.25 * fTemp226;
			let mut fTemp241: F32 = 0.0 - 0.33333334 * fTemp221;
			let mut fTemp242: F32 = 0.0 - 0.5 * fTemp222;
			let mut fTemp243: F32 = 0.0 - fTemp223;
			let mut iTemp244: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, iTemp217)) as F32))) as i32);
			let mut iTemp245: i32 = i32::wrapping_add(iTemp244, 1);
			self.fRec159[0] = self.fRec136[((i32::wrapping_sub(self.IOTA0, iTemp245)) & 2047) as usize] * fTemp243 * fTemp242 * fTemp241 * fTemp240 + fTemp239 * (self.fRec136[((i32::wrapping_sub(self.IOTA0, iTemp238)) & 2047) as usize] * fTemp236 * fTemp235 * fTemp234 + 0.5 * fTemp223 * self.fRec136[((i32::wrapping_sub(self.IOTA0, iTemp233)) & 2047) as usize] * fTemp231 * fTemp230 + 0.16666667 * fTemp224 * self.fRec136[((i32::wrapping_sub(self.IOTA0, iTemp229)) & 2047) as usize] * fTemp227 + 0.041666668 * fTemp225 * self.fRec136[((i32::wrapping_sub(self.IOTA0, iTemp219)) & 2047) as usize]);
			self.fRec163[0] = 0.05 * self.fRec163[1] + 0.95 * self.fRec159[1];
			let mut fRec160: F32 = self.fRec163[0];
			self.fRec168[0] = self.fConst10 * self.fRec168[1] + self.fConst11 * F32::abs(self.fRec130[1]);
			let mut fRec167: F32 = self.fRec168[0];
			let mut iTemp246: i32 = ((fRec167 > 0.1) as i32);
			self.iVec33[0] = iTemp246;
			self.iRec169[0] = std::cmp::max(i32::wrapping_mul(self.iConst12, ((iTemp246 < self.iVec33[1]) as i32)), i32::wrapping_add(self.iRec169[1], -1));
			let mut fTemp247: F32 = F32::abs(F32::max(((iTemp246) as F32), ((((self.iRec169[0] > 0) as i32)) as F32)));
			let mut fTemp248: F32 = if (((self.fRec165[1] > fTemp247) as i32) as i32 != 0) { self.fConst13 } else { self.fConst10 };
			self.fRec166[0] = self.fRec166[1] * fTemp248 + fTemp247 * (1.0 - fTemp248);
			self.fRec165[0] = self.fRec166[0];
			let mut fTemp249: F32 = 0.005 * self.fRec165[0] * self.fRec130[1];
			self.fRec170[0] = self.fRec134[1];
			self.fRec171[(self.IOTA0 & 2047) as usize] = -1.0 * 0.9973053 * (0.9 * self.fRec170[2] + 0.05 * (self.fRec170[1] + self.fRec170[3]));
			let mut fTemp250: F32 = fTemp224 * fTemp227;
			let mut fTemp251: F32 = fTemp223 * fTemp231 * fTemp230;
			let mut fTemp252: F32 = fTemp236 * fTemp235 * fTemp234;
			let mut fTemp253: F32 = fTemp243 * fTemp242 * fTemp241 * fTemp240;
			self.fVec34[0] = fTemp253 * self.fRec171[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp244, 2))) & 2047) as usize] + fTemp239 * (fTemp252 * self.fRec171[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp237, 2))) & 2047) as usize] + 0.5 * fTemp251 * self.fRec171[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp232, 2))) & 2047) as usize] + 0.16666667 * fTemp250 * self.fRec171[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp228, 2))) & 2047) as usize] + 0.041666668 * fTemp225 * self.fRec171[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp218, 2))) & 2047) as usize]);
			let mut fTemp254: F32 = F32::tan(self.fConst14 * fTemp213);
			let mut fTemp255: F32 = 1.0 / fTemp254;
			let mut fTemp256: F32 = (fTemp255 + 1.4142135) / fTemp254 + 1.0;
			self.fVec35[0] = fSlow18;
			self.iRec172[0] = i32::wrapping_add(i32::wrapping_mul(i32::wrapping_add(self.iRec172[1], ((self.iRec172[1] > 0) as i32)), ((fSlow18 <= self.fVec35[1]) as i32)), ((fSlow18 > self.fVec35[1]) as i32));
			let mut fTemp257: F32 = ((self.iRec172[0]) as F32) / F32::max(1.0, self.fConst15 * mydsp_faustpower2_f(1.0 - 0.22 * fTemp213));
			self.fRec173[0] = fTemp159 - (self.fRec173[2] * ((fTemp255 + -1.4142135) / fTemp254 + 1.0) + 2.0 * self.fRec173[1] * (1.0 - 1.0 / mydsp_faustpower2_f(fTemp254))) / fTemp256;
			let mut fTemp258: F32 = 0.5 * ((self.fRec173[2] + self.fRec173[0] + 2.0 * self.fRec173[1]) * F32::max(0.0, F32::min(fTemp257, 2.0 - fTemp257)) / fTemp256);
			let mut fTemp259: F32 = fTemp258 + self.fVec34[1] + fTemp249;
			self.fVec36[0] = fTemp259;
			self.fRec164[(self.IOTA0 & 2047) as usize] = 0.05 * self.fRec164[((i32::wrapping_sub(self.IOTA0, 1)) & 2047) as usize] + 0.95 * self.fVec36[2];
			let mut fRec161: F32 = fTemp253 * self.fRec164[((i32::wrapping_sub(self.IOTA0, iTemp244)) & 2047) as usize] + fTemp239 * (fTemp252 * self.fRec164[((i32::wrapping_sub(self.IOTA0, iTemp237)) & 2047) as usize] + 0.5 * fTemp251 * self.fRec164[((i32::wrapping_sub(self.IOTA0, iTemp232)) & 2047) as usize] + 0.16666667 * fTemp250 * self.fRec164[((i32::wrapping_sub(self.IOTA0, iTemp228)) & 2047) as usize] + 0.041666668 * fTemp225 * self.fRec164[((i32::wrapping_sub(self.IOTA0, iTemp218)) & 2047) as usize]);
			let mut fRec162: F32 = self.fVec36[1] + self.fRec155[1];
			self.fRec155[0] = fRec160;
			let mut fRec156: F32 = self.fRec155[1];
			let mut fRec157: F32 = fRec161;
			let mut fRec158: F32 = fRec162;
			self.fRec151[0] = fRec156;
			let mut fRec152: F32 = fTemp249 + fTemp258 + self.fRec151[1];
			let mut fRec153: F32 = fRec157;
			let mut fRec154: F32 = fRec158;
			self.fRec147[(self.IOTA0 & 2047) as usize] = fRec152;
			let mut fRec148: F32 = fTemp253 * self.fRec147[((i32::wrapping_sub(self.IOTA0, iTemp245)) & 2047) as usize] + fTemp239 * (fTemp252 * self.fRec147[((i32::wrapping_sub(self.IOTA0, iTemp238)) & 2047) as usize] + 0.5 * fTemp251 * self.fRec147[((i32::wrapping_sub(self.IOTA0, iTemp233)) & 2047) as usize] + 0.16666667 * fTemp250 * self.fRec147[((i32::wrapping_sub(self.IOTA0, iTemp229)) & 2047) as usize] + 0.041666668 * fTemp225 * self.fRec147[((i32::wrapping_sub(self.IOTA0, iTemp219)) & 2047) as usize]);
			self.fRec149[0] = fRec153;
			let mut fRec150: F32 = fRec154;
			self.fRec145[0] = fSlow14 * self.fRec149[1];
			let mut fRec146: F32 = fRec150;
			self.fRec140[0] = fRec144;
			let mut fRec141: F32 = fSlow14 * self.fRec140[1];
			let mut fRec142: F32 = self.fRec145[0];
			let mut fRec143: F32 = fRec146;
			self.fRec136[(self.IOTA0 & 2047) as usize] = fRec141;
			let mut fRec137: F32 = fRec148;
			let mut fRec138: F32 = fRec142;
			let mut fRec139: F32 = fRec143;
			self.fRec134[0] = fRec137;
			let mut fRec135: F32 = fRec139;
			let mut fTemp260: F32 = F32::abs(fRec135);
			let mut fTemp261: F32 = if (((self.fRec132[1] > fTemp260) as i32) as i32 != 0) { self.fConst16 } else { 0.0 };
			self.fRec133[0] = self.fRec133[1] * fTemp261 + fTemp260 * (1.0 - fTemp261);
			self.fRec132[0] = self.fRec133[0];
			let mut fRec131: F32 = 0.0 - 0.95 * F32::max(2e+01 * F32::log10(self.fRec132[0]) + 1e+01, 0.0);
			self.fRec130[0] = fRec135 * F32::powf(1e+01, 0.05 * fRec131);
			self.fRec128[0] = 0.0 - (self.fRec128[1] * (1.0 - fTemp214) - (self.fRec130[0] + self.fRec130[1])) / (fTemp214 + 1.0);
			self.fRec175[0] = fSlow19 + self.fConst2 * self.fRec175[1];
			let mut fTemp262: F32 = F32::powf(2.0, 0.083333336 * (self.fRec175[0] + self.fRec36[0] + -69.0));
			let mut fTemp263: F32 = 1.0 / F32::tan(self.fConst7 * fTemp262);
			let mut fRec190: F32 = -1.0 * 0.9973053 * (0.9 * self.fRec191[2] + 0.05 * (self.fRec191[1] + self.fRec191[3]));
			let mut fTemp264: F32 = self.fConst9 * (0.77272725 / fTemp262 + -0.11);
			let mut fTemp265: F32 = fTemp264 + -1.499995;
			let mut iTemp266: i32 = ((fTemp265) as i32);
			let mut iTemp267: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, i32::wrapping_add(iTemp266, 4))) as F32))) as i32);
			let mut iTemp268: i32 = i32::wrapping_add(iTemp267, 1);
			let mut fTemp269: F32 = F32::floor(fTemp265);
			let mut fTemp270: F32 = fTemp264 + (-3.0 - fTemp269);
			let mut fTemp271: F32 = fTemp264 + (-2.0 - fTemp269);
			let mut fTemp272: F32 = fTemp264 + (-1.0 - fTemp269);
			let mut fTemp273: F32 = fTemp272 * fTemp271;
			let mut fTemp274: F32 = fTemp273 * fTemp270;
			let mut fTemp275: F32 = fTemp264 + (-4.0 - fTemp269);
			let mut fTemp276: F32 = 0.0 - fTemp275;
			let mut iTemp277: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, i32::wrapping_add(iTemp266, 3))) as F32))) as i32);
			let mut iTemp278: i32 = i32::wrapping_add(iTemp277, 1);
			let mut fTemp279: F32 = 0.0 - 0.5 * fTemp275;
			let mut fTemp280: F32 = 0.0 - fTemp270;
			let mut iTemp281: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, i32::wrapping_add(iTemp266, 2))) as F32))) as i32);
			let mut iTemp282: i32 = i32::wrapping_add(iTemp281, 1);
			let mut fTemp283: F32 = 0.0 - 0.33333334 * fTemp275;
			let mut fTemp284: F32 = 0.0 - 0.5 * fTemp270;
			let mut fTemp285: F32 = 0.0 - fTemp271;
			let mut iTemp286: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, i32::wrapping_add(iTemp266, 1))) as F32))) as i32);
			let mut iTemp287: i32 = i32::wrapping_add(iTemp286, 1);
			let mut fTemp288: F32 = fTemp264 - fTemp269;
			let mut fTemp289: F32 = 0.0 - 0.25 * fTemp275;
			let mut fTemp290: F32 = 0.0 - 0.33333334 * fTemp270;
			let mut fTemp291: F32 = 0.0 - 0.5 * fTemp271;
			let mut fTemp292: F32 = 0.0 - fTemp272;
			let mut iTemp293: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, iTemp266)) as F32))) as i32);
			let mut iTemp294: i32 = i32::wrapping_add(iTemp293, 1);
			self.fRec205[0] = self.fRec182[((i32::wrapping_sub(self.IOTA0, iTemp294)) & 2047) as usize] * fTemp292 * fTemp291 * fTemp290 * fTemp289 + fTemp288 * (self.fRec182[((i32::wrapping_sub(self.IOTA0, iTemp287)) & 2047) as usize] * fTemp285 * fTemp284 * fTemp283 + 0.5 * fTemp272 * self.fRec182[((i32::wrapping_sub(self.IOTA0, iTemp282)) & 2047) as usize] * fTemp280 * fTemp279 + 0.16666667 * fTemp273 * self.fRec182[((i32::wrapping_sub(self.IOTA0, iTemp278)) & 2047) as usize] * fTemp276 + 0.041666668 * fTemp274 * self.fRec182[((i32::wrapping_sub(self.IOTA0, iTemp268)) & 2047) as usize]);
			self.fRec209[0] = 0.05 * self.fRec209[1] + 0.95 * self.fRec205[1];
			let mut fRec206: F32 = self.fRec209[0];
			self.fRec214[0] = self.fConst10 * self.fRec214[1] + self.fConst11 * F32::abs(self.fRec176[1]);
			let mut fRec213: F32 = self.fRec214[0];
			let mut iTemp295: i32 = ((fRec213 > 0.1) as i32);
			self.iVec37[0] = iTemp295;
			self.iRec215[0] = std::cmp::max(i32::wrapping_mul(self.iConst12, ((iTemp295 < self.iVec37[1]) as i32)), i32::wrapping_add(self.iRec215[1], -1));
			let mut fTemp296: F32 = F32::abs(F32::max(((iTemp295) as F32), ((((self.iRec215[0] > 0) as i32)) as F32)));
			let mut fTemp297: F32 = if (((self.fRec211[1] > fTemp296) as i32) as i32 != 0) { self.fConst13 } else { self.fConst10 };
			self.fRec212[0] = self.fRec212[1] * fTemp297 + fTemp296 * (1.0 - fTemp297);
			self.fRec211[0] = self.fRec212[0];
			let mut fTemp298: F32 = 0.005 * self.fRec211[0] * self.fRec176[1];
			self.fRec216[0] = self.fRec180[1];
			self.fRec217[(self.IOTA0 & 2047) as usize] = -1.0 * 0.9973053 * (0.9 * self.fRec216[2] + 0.05 * (self.fRec216[1] + self.fRec216[3]));
			let mut fTemp299: F32 = fTemp273 * fTemp276;
			let mut fTemp300: F32 = fTemp272 * fTemp280 * fTemp279;
			let mut fTemp301: F32 = fTemp285 * fTemp284 * fTemp283;
			let mut fTemp302: F32 = fTemp292 * fTemp291 * fTemp290 * fTemp289;
			self.fVec38[0] = fTemp302 * self.fRec217[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp293, 2))) & 2047) as usize] + fTemp288 * (fTemp301 * self.fRec217[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp286, 2))) & 2047) as usize] + 0.5 * fTemp300 * self.fRec217[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp281, 2))) & 2047) as usize] + 0.16666667 * fTemp299 * self.fRec217[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp277, 2))) & 2047) as usize] + 0.041666668 * fTemp274 * self.fRec217[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp267, 2))) & 2047) as usize]);
			let mut fTemp303: F32 = F32::tan(self.fConst14 * fTemp262);
			let mut fTemp304: F32 = 1.0 / fTemp303;
			let mut fTemp305: F32 = (fTemp304 + 1.4142135) / fTemp303 + 1.0;
			self.fVec39[0] = fSlow20;
			self.iRec218[0] = i32::wrapping_add(i32::wrapping_mul(i32::wrapping_add(self.iRec218[1], ((self.iRec218[1] > 0) as i32)), ((fSlow20 <= self.fVec39[1]) as i32)), ((fSlow20 > self.fVec39[1]) as i32));
			let mut fTemp306: F32 = ((self.iRec218[0]) as F32) / F32::max(1.0, self.fConst15 * mydsp_faustpower2_f(1.0 - 0.22 * fTemp262));
			self.fRec219[0] = fTemp159 - (self.fRec219[2] * ((fTemp304 + -1.4142135) / fTemp303 + 1.0) + 2.0 * self.fRec219[1] * (1.0 - 1.0 / mydsp_faustpower2_f(fTemp303))) / fTemp305;
			let mut fTemp307: F32 = 0.5 * ((self.fRec219[2] + self.fRec219[0] + 2.0 * self.fRec219[1]) * F32::max(0.0, F32::min(fTemp306, 2.0 - fTemp306)) / fTemp305);
			let mut fTemp308: F32 = fTemp307 + self.fVec38[1] + fTemp298;
			self.fVec40[0] = fTemp308;
			self.fRec210[(self.IOTA0 & 2047) as usize] = 0.05 * self.fRec210[((i32::wrapping_sub(self.IOTA0, 1)) & 2047) as usize] + 0.95 * self.fVec40[2];
			let mut fRec207: F32 = fTemp302 * self.fRec210[((i32::wrapping_sub(self.IOTA0, iTemp293)) & 2047) as usize] + fTemp288 * (fTemp301 * self.fRec210[((i32::wrapping_sub(self.IOTA0, iTemp286)) & 2047) as usize] + 0.5 * fTemp300 * self.fRec210[((i32::wrapping_sub(self.IOTA0, iTemp281)) & 2047) as usize] + 0.16666667 * fTemp299 * self.fRec210[((i32::wrapping_sub(self.IOTA0, iTemp277)) & 2047) as usize] + 0.041666668 * fTemp274 * self.fRec210[((i32::wrapping_sub(self.IOTA0, iTemp267)) & 2047) as usize]);
			let mut fRec208: F32 = self.fVec40[1] + self.fRec201[1];
			self.fRec201[0] = fRec206;
			let mut fRec202: F32 = self.fRec201[1];
			let mut fRec203: F32 = fRec207;
			let mut fRec204: F32 = fRec208;
			self.fRec197[0] = fRec202;
			let mut fRec198: F32 = fTemp298 + fTemp307 + self.fRec197[1];
			let mut fRec199: F32 = fRec203;
			let mut fRec200: F32 = fRec204;
			self.fRec193[(self.IOTA0 & 2047) as usize] = fRec198;
			let mut fRec194: F32 = fTemp302 * self.fRec193[((i32::wrapping_sub(self.IOTA0, iTemp294)) & 2047) as usize] + fTemp288 * (fTemp301 * self.fRec193[((i32::wrapping_sub(self.IOTA0, iTemp287)) & 2047) as usize] + 0.5 * fTemp300 * self.fRec193[((i32::wrapping_sub(self.IOTA0, iTemp282)) & 2047) as usize] + 0.16666667 * fTemp299 * self.fRec193[((i32::wrapping_sub(self.IOTA0, iTemp278)) & 2047) as usize] + 0.041666668 * fTemp274 * self.fRec193[((i32::wrapping_sub(self.IOTA0, iTemp268)) & 2047) as usize]);
			self.fRec195[0] = fRec199;
			let mut fRec196: F32 = fRec200;
			self.fRec191[0] = fSlow14 * self.fRec195[1];
			let mut fRec192: F32 = fRec196;
			self.fRec186[0] = fRec190;
			let mut fRec187: F32 = fSlow14 * self.fRec186[1];
			let mut fRec188: F32 = self.fRec191[0];
			let mut fRec189: F32 = fRec192;
			self.fRec182[(self.IOTA0 & 2047) as usize] = fRec187;
			let mut fRec183: F32 = fRec194;
			let mut fRec184: F32 = fRec188;
			let mut fRec185: F32 = fRec189;
			self.fRec180[0] = fRec183;
			let mut fRec181: F32 = fRec185;
			let mut fTemp309: F32 = F32::abs(fRec181);
			let mut fTemp310: F32 = if (((self.fRec178[1] > fTemp309) as i32) as i32 != 0) { self.fConst16 } else { 0.0 };
			self.fRec179[0] = self.fRec179[1] * fTemp310 + fTemp309 * (1.0 - fTemp310);
			self.fRec178[0] = self.fRec179[0];
			let mut fRec177: F32 = 0.0 - 0.95 * F32::max(2e+01 * F32::log10(self.fRec178[0]) + 1e+01, 0.0);
			self.fRec176[0] = fRec181 * F32::powf(1e+01, 0.05 * fRec177);
			self.fRec174[0] = 0.0 - (self.fRec174[1] * (1.0 - fTemp263) - (self.fRec176[0] + self.fRec176[1])) / (fTemp263 + 1.0);
			let mut fTemp311: F32 = (self.fRec174[0] + self.fRec128[0] + self.fRec82[0] + self.fRec34[0]) * self.fRec33[0];
			self.fRec220[0] = fSlow21 + self.fConst2 * self.fRec220[1];
			let mut fTemp312: F32 = F32::min(1.4141995, 1.4142135 * self.fRec220[0]);
			let mut fTemp313: F32 = 1.4142135 * fTemp312;
			let mut fTemp314: F32 = 1.0 - fTemp313;
			self.fRec222[0] = fSlow22 + self.fConst2 * self.fRec222[1];
			let mut fTemp315: F32 = self.fRec222[0] + -69.0;
			self.fRec221[0] = self.fConst2 * self.fRec221[1] + self.fConst17 * F32::powf(2.0, 0.083333336 * (fSlow23 + fTemp315));
			let mut fTemp316: F32 = F32::tan(self.fConst18 * F32::max(2e+01, F32::min(1e+04, self.fRec221[0])));
			let mut fTemp317: F32 = 1.0 / fTemp316;
			let mut fTemp318: F32 = 2.0 - fTemp313;
			let mut fTemp319: F32 = mydsp_faustpower2_f(fTemp312);
			let mut fTemp320: F32 = fTemp319 + (fTemp318 + fTemp317) / fTemp316 + fTemp314;
			let mut fTemp321: F32 = 1.0 / mydsp_faustpower2_f(fTemp316);
			let mut fTemp322: F32 = fTemp313 + 2.0;
			let mut fTemp323: F32 = fTemp313 + fTemp319;
			let mut fTemp324: F32 = fTemp323 + (fTemp322 + fTemp317) / fTemp316 + 1.0;
			self.fRec227[0] = fSlow24 + self.fConst2 * self.fRec227[1];
			let mut fTemp325: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec227[0]));
			let mut fTemp326: F32 = self.fRec225[1] + self.fConst3 * fTemp325;
			let mut fTemp327: F32 = fTemp326 + -1.0;
			let mut iTemp328: i32 = ((fTemp327 < 0.0) as i32);
			self.fRec225[0] = if (iTemp328 as i32 != 0) { fTemp326 } else { fTemp327 };
			let mut fThen9: F32 = fTemp326 + (1.0 - self.fConst0 / fTemp325) * fTemp327;
			let mut fRec226: F32 = if (iTemp328 as i32 != 0) { fTemp326 } else { fThen9 };
			self.fRec228[0] = fSlow25 + self.fConst2 * self.fRec228[1];
			self.fRec224[0] = self.fRec228[0] * (2.0 * fRec226 + -1.0) - (self.fRec224[2] * (fTemp323 + (1.0 - (fTemp322 - fTemp317) / fTemp316)) + 2.0 * self.fRec224[1] * (fTemp323 + (1.0 - fTemp321))) / fTemp324;
			self.fRec223[0] = (self.fRec224[2] + self.fRec224[0] + 2.0 * self.fRec224[1]) / fTemp324 - (self.fRec223[2] * (fTemp319 + (fTemp317 - fTemp318) / fTemp316 + fTemp314) + 2.0 * self.fRec223[1] * (fTemp319 + (1.0 - (fTemp313 + fTemp321)))) / fTemp320;
			self.fRec229[0] = self.fConst2 * self.fRec229[1] + self.fConst17 * F32::powf(2.0, 0.083333336 * (fSlow26 + fTemp315));
			let mut fTemp329: F32 = F32::tan(self.fConst18 * F32::max(2e+01, F32::min(1e+04, self.fRec229[0])));
			let mut fTemp330: F32 = 1.0 / fTemp329;
			let mut fTemp331: F32 = fTemp319 + (fTemp318 + fTemp330) / fTemp329 + fTemp314;
			let mut fTemp332: F32 = 1.0 / mydsp_faustpower2_f(fTemp329);
			let mut fTemp333: F32 = fTemp323 + (fTemp322 + fTemp330) / fTemp329 + 1.0;
			self.fRec234[0] = fSlow27 + self.fConst2 * self.fRec234[1];
			let mut fTemp334: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec234[0]));
			let mut fTemp335: F32 = self.fRec232[1] + self.fConst3 * fTemp334;
			let mut fTemp336: F32 = fTemp335 + -1.0;
			let mut iTemp337: i32 = ((fTemp336 < 0.0) as i32);
			self.fRec232[0] = if (iTemp337 as i32 != 0) { fTemp335 } else { fTemp336 };
			let mut fThen11: F32 = fTemp335 + (1.0 - self.fConst0 / fTemp334) * fTemp336;
			let mut fRec233: F32 = if (iTemp337 as i32 != 0) { fTemp335 } else { fThen11 };
			self.fRec235[0] = fSlow28 + self.fConst2 * self.fRec235[1];
			self.fRec231[0] = self.fRec235[0] * (2.0 * fRec233 + -1.0) - (self.fRec231[2] * (fTemp323 + (fTemp330 - fTemp322) / fTemp329 + 1.0) + 2.0 * self.fRec231[1] * (fTemp323 + (1.0 - fTemp332))) / fTemp333;
			self.fRec230[0] = (self.fRec231[2] + self.fRec231[0] + 2.0 * self.fRec231[1]) / fTemp333 - (self.fRec230[2] * (fTemp319 + (fTemp330 - fTemp318) / fTemp329 + fTemp314) + 2.0 * self.fRec230[1] * (fTemp319 + (1.0 - (fTemp313 + fTemp332)))) / fTemp331;
			self.fRec236[0] = self.fConst2 * self.fRec236[1] + self.fConst17 * F32::powf(2.0, 0.083333336 * (fSlow29 + fTemp315));
			let mut fTemp338: F32 = F32::tan(self.fConst18 * F32::max(2e+01, F32::min(1e+04, self.fRec236[0])));
			let mut fTemp339: F32 = 1.0 / fTemp338;
			let mut fTemp340: F32 = fTemp319 + (fTemp318 + fTemp339) / fTemp338 + fTemp314;
			let mut fTemp341: F32 = 1.0 / mydsp_faustpower2_f(fTemp338);
			let mut fTemp342: F32 = fTemp323 + (fTemp322 + fTemp339) / fTemp338 + 1.0;
			self.fRec241[0] = fSlow30 + self.fConst2 * self.fRec241[1];
			let mut fTemp343: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec241[0]));
			let mut fTemp344: F32 = self.fRec239[1] + self.fConst3 * fTemp343;
			let mut fTemp345: F32 = fTemp344 + -1.0;
			let mut iTemp346: i32 = ((fTemp345 < 0.0) as i32);
			self.fRec239[0] = if (iTemp346 as i32 != 0) { fTemp344 } else { fTemp345 };
			let mut fThen13: F32 = fTemp344 + (1.0 - self.fConst0 / fTemp343) * fTemp345;
			let mut fRec240: F32 = if (iTemp346 as i32 != 0) { fTemp344 } else { fThen13 };
			self.fRec242[0] = fSlow31 + self.fConst2 * self.fRec242[1];
			self.fRec238[0] = self.fRec242[0] * (2.0 * fRec240 + -1.0) - (self.fRec238[2] * (fTemp323 + (1.0 - (fTemp322 - fTemp339) / fTemp338)) + 2.0 * self.fRec238[1] * (fTemp323 + (1.0 - fTemp341))) / fTemp342;
			self.fRec237[0] = (self.fRec238[2] + self.fRec238[0] + 2.0 * self.fRec238[1]) / fTemp342 - (self.fRec237[2] * (fTemp319 + (fTemp339 - fTemp318) / fTemp338 + fTemp314) + 2.0 * self.fRec237[1] * (fTemp319 + (1.0 - (fTemp313 + fTemp341)))) / fTemp340;
			self.fRec243[0] = self.fConst2 * self.fRec243[1] + self.fConst17 * F32::powf(2.0, 0.083333336 * (fSlow32 + fTemp315));
			let mut fTemp347: F32 = F32::tan(self.fConst18 * F32::max(2e+01, F32::min(1e+04, self.fRec243[0])));
			let mut fTemp348: F32 = 1.0 / fTemp347;
			let mut fTemp349: F32 = fTemp319 + (fTemp318 + fTemp348) / fTemp347 + fTemp314;
			let mut fTemp350: F32 = 1.0 / mydsp_faustpower2_f(fTemp347);
			let mut fTemp351: F32 = fTemp323 + (fTemp348 + fTemp322) / fTemp347 + 1.0;
			self.fRec248[0] = fSlow33 + self.fConst2 * self.fRec248[1];
			let mut fTemp352: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec248[0]));
			let mut fTemp353: F32 = self.fConst3 * fTemp352;
			let mut fTemp354: F32 = self.fRec246[1] + fTemp353;
			let mut fTemp355: F32 = fTemp354 + -1.0;
			let mut iTemp356: i32 = ((fTemp355 < 0.0) as i32);
			self.fRec246[0] = if (iTemp356 as i32 != 0) { fTemp354 } else { fTemp355 };
			let mut fThen15: F32 = fTemp353 + self.fRec246[1] + (1.0 - self.fConst0 / fTemp352) * fTemp355;
			let mut fRec247: F32 = if (iTemp356 as i32 != 0) { fTemp354 } else { fThen15 };
			self.fRec249[0] = fSlow34 + self.fConst2 * self.fRec249[1];
			self.fRec245[0] = self.fRec249[0] * (2.0 * fRec247 + -1.0) - (self.fRec245[2] * (fTemp323 + (1.0 - (fTemp322 - fTemp348) / fTemp347)) + 2.0 * self.fRec245[1] * (fTemp323 + (1.0 - fTemp350))) / fTemp351;
			self.fRec244[0] = (self.fRec245[2] + self.fRec245[0] + 2.0 * self.fRec245[1]) / fTemp351 - (self.fRec244[2] * (fTemp319 + (fTemp348 - fTemp318) / fTemp347 + fTemp314) + 2.0 * self.fRec244[1] * (fTemp319 + (1.0 - (fTemp313 + fTemp350)))) / fTemp349;
			self.fRec250[0] = fSlow35 + self.fConst2 * self.fRec250[1];
			self.fRec251[0] = fSlow36 + self.fConst2 * self.fRec251[1];
			let mut fTemp357: F32 = self.fRec251[0] * self.fRec250[0] * ((self.fRec244[2] + self.fRec244[0] + 2.0 * self.fRec244[1]) / fTemp349 + (self.fRec237[2] + self.fRec237[0] + 2.0 * self.fRec237[1]) / fTemp340 + (self.fRec230[2] + self.fRec230[0] + 2.0 * self.fRec230[1]) / fTemp331 + (self.fRec223[2] + self.fRec223[0] + 2.0 * self.fRec223[1]) / fTemp320);
			self.fRec252[0] = fSlow37 + self.fConst2 * self.fRec252[1];
			let mut fTemp358: F32 = (1.0 - self.fRec252[0]) * (fTemp357 + fTemp311 + fTemp113);
			self.fRec254[0] = fSlow38 + self.fConst2 * self.fRec254[1];
			self.fRec253[(self.IOTA0 & 2097151) as usize] = fTemp357 + fTemp113 + fTemp311 + fSlow39 * self.fRec253[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(((F32::min(self.fConst19, F32::max(0.0, self.fConst0 * self.fRec254[0]))) as i32), 1))) & 2097151) as usize];
			let mut fTemp359: F32 = self.fRec253[(self.IOTA0 & 2097151) as usize] * self.fRec252[0];
			let mut fTemp360: F32 = fTemp359 + fTemp358;
			let mut iTemp361: i32 = i32::wrapping_sub(1, self.iVec0[1]);
			self.fRec265[0] = 0.995 * (self.fRec265[1] + ((i32::wrapping_mul(iTemp361, iSlow43)) as F32)) + fSlow44;
			let mut fTemp362: F32 = self.fRec265[0] + -1.49999;
			let mut fTemp363: F32 = F32::floor(fTemp362);
			self.fRec267[0] = 0.995 * (self.fRec267[1] + ((i32::wrapping_mul(iTemp361, iSlow45)) as F32)) + fSlow46;
			let mut fTemp364: F32 = self.fRec267[0] + -1.49999;
			let mut fTemp365: F32 = F32::floor(fTemp364);
			self.fRec271[0] = 0.9999 * (self.fRec271[1] + ((i32::wrapping_mul(iTemp361, iSlow47)) as F32)) + fSlow48;
			let mut fTemp366: F32 = self.fRec271[0] + -1.49999;
			let mut fTemp367: F32 = F32::floor(fTemp366);
			let mut fTemp368: F32 = 0.760314 * self.fRec255[1] - 0.64955574 * self.fRec268[1];
			let mut fTemp369: F32 = 0.760314 * self.fRec256[1] - 0.64955574 * self.fRec269[1];
			self.fVec41[(self.IOTA0 & 16383) as usize] = 0.0 - 0.70710677 * (fTemp369 - fTemp368);
			let mut fTemp370: F32 = self.fVec41[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp366) as i32))))) & 16383) as usize];
			self.fVec42[0] = fTemp370;
			self.fRec270[0] = self.fVec42[1] - (fTemp367 + (2.0 - self.fRec271[0])) * (self.fRec270[1] - fTemp370) / (self.fRec271[0] - fTemp367);
			self.fRec268[0] = self.fRec270[0];
			self.fRec273[0] = 0.9999 * (self.fRec273[1] + ((i32::wrapping_mul(iTemp361, iSlow49)) as F32)) + fSlow50;
			let mut fTemp371: F32 = self.fRec273[0] + -1.49999;
			let mut fTemp372: F32 = F32::floor(fTemp371);
			let mut fTemp373: F32 = self.fRec273[0] - fTemp372;
			let mut fTemp374: F32 = fTemp372 + (2.0 - self.fRec273[0]);
			self.fVec43[(self.IOTA0 & 16383) as usize] = fTemp368 + fTemp369;
			let mut fTemp375: F32 = self.fVec43[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp371) as i32))))) & 16383) as usize];
			self.fVec44[0] = fTemp375;
			self.fRec272[0] = 0.70710677 * (fTemp374 * fTemp375 / fTemp373 + self.fVec44[1]) - self.fRec272[1] * fTemp374 / fTemp373;
			self.fRec269[0] = self.fRec272[0];
			let mut fTemp376: F32 = 0.760314 * self.fRec268[1] + 0.64955574 * self.fRec255[1];
			self.fRec277[0] = 0.9999 * (self.fRec277[1] + ((i32::wrapping_mul(iTemp361, iSlow51)) as F32)) + fSlow52;
			let mut fTemp377: F32 = self.fRec277[0] + -1.49999;
			let mut fTemp378: F32 = F32::floor(fTemp377);
			let mut fTemp379: F32 = self.fRec277[0] - fTemp378;
			let mut fTemp380: F32 = fTemp378 + (2.0 - self.fRec277[0]);
			let mut fTemp381: F32 = 0.760314 * self.fRec269[1] + 0.64955574 * self.fRec256[1];
			let mut fTemp382: F32 = 0.760314 * fTemp381 - 0.64955574 * self.fRec275[1];
			let mut fTemp383: F32 = 0.760314 * fTemp376 - 0.64955574 * self.fRec274[1];
			self.fVec45[(self.IOTA0 & 16383) as usize] = fTemp383 - fTemp382;
			let mut fTemp384: F32 = self.fVec45[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp377) as i32))))) & 16383) as usize];
			self.fVec46[0] = fTemp384;
			self.fRec276[0] = 0.70710677 * (fTemp380 * fTemp384 / fTemp379 + self.fVec46[1]) - self.fRec276[1] * fTemp380 / fTemp379;
			self.fRec274[0] = self.fRec276[0];
			self.fRec279[0] = 0.9999 * (self.fRec279[1] + ((i32::wrapping_mul(iTemp361, iSlow53)) as F32)) + fSlow54;
			let mut fTemp385: F32 = self.fRec279[0] + -1.49999;
			let mut fTemp386: F32 = F32::floor(fTemp385);
			let mut fTemp387: F32 = self.fRec279[0] - fTemp386;
			let mut fTemp388: F32 = fTemp386 + (2.0 - self.fRec279[0]);
			self.fVec47[(self.IOTA0 & 16383) as usize] = fTemp383 + fTemp382;
			let mut fTemp389: F32 = self.fVec47[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp385) as i32))))) & 16383) as usize];
			self.fVec48[0] = fTemp389;
			self.fRec278[0] = 0.70710677 * (fTemp388 * fTemp389 / fTemp387 + self.fVec48[1]) - self.fRec278[1] * fTemp388 / fTemp387;
			self.fRec275[0] = self.fRec278[0];
			let mut fTemp390: F32 = 0.760314 * self.fRec274[1] + 0.64955574 * fTemp376;
			self.fRec283[0] = 0.9999 * (self.fRec283[1] + ((i32::wrapping_mul(iTemp361, iSlow55)) as F32)) + fSlow56;
			let mut fTemp391: F32 = self.fRec283[0] + -1.49999;
			let mut fTemp392: F32 = F32::floor(fTemp391);
			let mut fTemp393: F32 = 0.760314 * fTemp390 - 0.64955574 * self.fRec280[1];
			let mut fTemp394: F32 = 0.760314 * self.fRec275[1] + 0.64955574 * fTemp381;
			let mut fTemp395: F32 = 0.760314 * fTemp394 - 0.64955574 * self.fRec281[1];
			self.fVec49[(self.IOTA0 & 16383) as usize] = 0.0 - 0.70710677 * (fTemp395 - fTemp393);
			let mut fTemp396: F32 = self.fVec49[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp391) as i32))))) & 16383) as usize];
			self.fVec50[0] = fTemp396;
			self.fRec282[0] = self.fVec50[1] - (fTemp392 + (2.0 - self.fRec283[0])) * (self.fRec282[1] - fTemp396) / (self.fRec283[0] - fTemp392);
			self.fRec280[0] = self.fRec282[0];
			self.fRec285[0] = 0.9999 * (self.fRec285[1] + ((i32::wrapping_mul(iTemp361, iSlow57)) as F32)) + fSlow58;
			let mut fTemp397: F32 = self.fRec285[0] + -1.49999;
			let mut fTemp398: F32 = F32::floor(fTemp397);
			let mut fTemp399: F32 = self.fRec285[0] - fTemp398;
			let mut fTemp400: F32 = fTemp398 + (2.0 - self.fRec285[0]);
			self.fVec51[(self.IOTA0 & 16383) as usize] = fTemp393 + fTemp395;
			let mut fTemp401: F32 = self.fVec51[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp397) as i32))))) & 16383) as usize];
			self.fVec52[0] = fTemp401;
			self.fRec284[0] = 0.70710677 * (fTemp400 * fTemp401 / fTemp399 + self.fVec52[1]) - self.fRec284[1] * fTemp400 / fTemp399;
			self.fRec281[0] = self.fRec284[0];
			let mut fTemp402: F32 = 0.760314 * self.fRec280[1] + 0.64955574 * fTemp390;
			self.fRec289[0] = 0.9999 * (self.fRec289[1] + ((i32::wrapping_mul(iTemp361, iSlow59)) as F32)) + fSlow60;
			let mut fTemp403: F32 = self.fRec289[0] + -1.49999;
			let mut fTemp404: F32 = F32::floor(fTemp403);
			let mut fTemp405: F32 = self.fRec289[0] - fTemp404;
			let mut fTemp406: F32 = fTemp404 + (2.0 - self.fRec289[0]);
			let mut fTemp407: F32 = 0.760314 * self.fRec281[1] + 0.64955574 * fTemp394;
			let mut fTemp408: F32 = 0.760314 * fTemp407 - 0.64955574 * self.fRec287[1];
			let mut fTemp409: F32 = 0.760314 * fTemp402 - 0.64955574 * self.fRec286[1];
			self.fVec53[(self.IOTA0 & 16383) as usize] = fTemp409 - fTemp408;
			let mut fTemp410: F32 = self.fVec53[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp403) as i32))))) & 16383) as usize];
			self.fVec54[0] = fTemp410;
			self.fRec288[0] = 0.70710677 * (fTemp406 * fTemp410 / fTemp405 + self.fVec54[1]) - self.fRec288[1] * fTemp406 / fTemp405;
			self.fRec286[0] = self.fRec288[0];
			self.fRec291[0] = 0.9999 * (self.fRec291[1] + ((i32::wrapping_mul(iTemp361, iSlow61)) as F32)) + fSlow62;
			let mut fTemp411: F32 = self.fRec291[0] + -1.49999;
			let mut fTemp412: F32 = F32::floor(fTemp411);
			let mut fTemp413: F32 = self.fRec291[0] - fTemp412;
			let mut fTemp414: F32 = fTemp412 + (2.0 - self.fRec291[0]);
			self.fVec55[(self.IOTA0 & 16383) as usize] = fTemp409 + fTemp408;
			let mut fTemp415: F32 = self.fVec55[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp411) as i32))))) & 16383) as usize];
			self.fVec56[0] = fTemp415;
			self.fRec290[0] = 0.70710677 * (fTemp414 * fTemp415 / fTemp413 + self.fVec56[1]) - self.fRec290[1] * fTemp414 / fTemp413;
			self.fRec287[0] = self.fRec290[0];
			let mut fTemp416: F32 = 0.760314 * self.fRec286[1] + 0.64955574 * fTemp402;
			self.fRec295[0] = 0.9999 * (self.fRec295[1] + ((i32::wrapping_mul(iTemp361, iSlow63)) as F32)) + fSlow64;
			let mut fTemp417: F32 = self.fRec295[0] + -1.49999;
			let mut fTemp418: F32 = F32::floor(fTemp417);
			let mut fTemp419: F32 = 0.760314 * fTemp416 - 0.64955574 * self.fRec292[1];
			let mut fTemp420: F32 = 0.760314 * self.fRec287[1] + 0.64955574 * fTemp407;
			let mut fTemp421: F32 = 0.760314 * fTemp420 - 0.64955574 * self.fRec293[1];
			self.fVec57[(self.IOTA0 & 16383) as usize] = 0.0 - 0.70710677 * (fTemp421 - fTemp419);
			let mut fTemp422: F32 = self.fVec57[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp417) as i32))))) & 16383) as usize];
			self.fVec58[0] = fTemp422;
			self.fRec294[0] = self.fVec58[1] - (fTemp418 + (2.0 - self.fRec295[0])) * (self.fRec294[1] - fTemp422) / (self.fRec295[0] - fTemp418);
			self.fRec292[0] = self.fRec294[0];
			self.fRec297[0] = 0.9999 * (self.fRec297[1] + ((i32::wrapping_mul(iTemp361, iSlow65)) as F32)) + fSlow66;
			let mut fTemp423: F32 = self.fRec297[0] + -1.49999;
			let mut fTemp424: F32 = F32::floor(fTemp423);
			let mut fTemp425: F32 = self.fRec297[0] - fTemp424;
			let mut fTemp426: F32 = fTemp424 + (2.0 - self.fRec297[0]);
			self.fVec59[(self.IOTA0 & 16383) as usize] = fTemp419 + fTemp421;
			let mut fTemp427: F32 = self.fVec59[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp423) as i32))))) & 16383) as usize];
			self.fVec60[0] = fTemp427;
			self.fRec296[0] = 0.70710677 * (fTemp426 * fTemp427 / fTemp425 + self.fVec60[1]) - self.fRec296[1] * fTemp426 / fTemp425;
			self.fRec293[0] = self.fRec296[0];
			let mut fTemp428: F32 = 0.760314 * self.fRec292[1] + 0.64955574 * fTemp416;
			self.fVec61[(self.IOTA0 & 1023) as usize] = fTemp428;
			self.fRec298[0] = fSlow69 * self.fRec299[1] + fSlow68 * self.fRec298[1];
			self.fRec299[0] = ((iTemp361) as F32) + fSlow68 * self.fRec299[1] - fSlow69 * self.fRec298[1];
			let mut fTemp429: F32 = fSlow70 * (self.fRec299[0] + 1.0);
			let mut fTemp430: F32 = fTemp429 + 3.500005;
			let mut iTemp431: i32 = ((fTemp430) as i32);
			let mut iTemp432: i32 = std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp431, 4)));
			let mut fTemp433: F32 = F32::floor(fTemp430);
			let mut fTemp434: F32 = fTemp429 + (2.0 - fTemp433);
			let mut fTemp435: F32 = fTemp429 + (3.0 - fTemp433);
			let mut fTemp436: F32 = fTemp429 + (4.0 - fTemp433);
			let mut fTemp437: F32 = fTemp436 * fTemp435;
			let mut fTemp438: F32 = fTemp437 * fTemp434;
			let mut fTemp439: F32 = fTemp429 + (1.0 - fTemp433);
			let mut fTemp440: F32 = 0.0 - fTemp439;
			let mut iTemp441: i32 = std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp431, 3)));
			let mut fTemp442: F32 = 0.0 - 0.5 * fTemp439;
			let mut fTemp443: F32 = 0.0 - fTemp434;
			let mut iTemp444: i32 = std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp431, 2)));
			let mut fTemp445: F32 = 0.0 - 0.33333334 * fTemp439;
			let mut fTemp446: F32 = 0.0 - 0.5 * fTemp434;
			let mut fTemp447: F32 = 0.0 - fTemp435;
			let mut iTemp448: i32 = std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp431, 1)));
			let mut fTemp449: F32 = fTemp429 + (5.0 - fTemp433);
			let mut fTemp450: F32 = 0.0 - 0.25 * fTemp439;
			let mut fTemp451: F32 = 0.0 - 0.33333334 * fTemp434;
			let mut fTemp452: F32 = 0.0 - 0.5 * fTemp435;
			let mut fTemp453: F32 = 0.0 - fTemp436;
			let mut iTemp454: i32 = std::cmp::min(512, std::cmp::max(0, iTemp431));
			self.fVec62[(self.IOTA0 & 16383) as usize] = self.fVec61[((i32::wrapping_sub(self.IOTA0, iTemp454)) & 1023) as usize] * fTemp453 * fTemp452 * fTemp451 * fTemp450 + fTemp449 * (self.fVec61[((i32::wrapping_sub(self.IOTA0, iTemp448)) & 1023) as usize] * fTemp447 * fTemp446 * fTemp445 + 0.5 * fTemp436 * self.fVec61[((i32::wrapping_sub(self.IOTA0, iTemp444)) & 1023) as usize] * fTemp443 * fTemp442 + 0.16666667 * fTemp437 * self.fVec61[((i32::wrapping_sub(self.IOTA0, iTemp441)) & 1023) as usize] * fTemp440 + 0.041666668 * fTemp438 * self.fVec61[((i32::wrapping_sub(self.IOTA0, iTemp432)) & 1023) as usize]);
			let mut fTemp455: F32 = self.fVec62[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp364) as i32))))) & 16383) as usize];
			self.fVec63[0] = fTemp455;
			self.fRec266[0] = self.fVec63[1] - (fTemp365 + (2.0 - self.fRec267[0])) * (self.fRec266[1] - fTemp455) / (self.fRec267[0] - fTemp365);
			self.fRec303[0] = 0.9999 * (self.fRec303[1] + ((i32::wrapping_mul(iTemp361, iSlow71)) as F32)) + fSlow72;
			let mut fTemp456: F32 = self.fRec303[0] + -1.49999;
			let mut fTemp457: F32 = F32::floor(fTemp456);
			let mut fTemp458: F32 = self.fRec303[0] - fTemp457;
			let mut fTemp459: F32 = fTemp457 + (2.0 - self.fRec303[0]);
			self.fRec305[0] = 0.995 * (self.fRec305[1] + ((i32::wrapping_mul(iTemp361, iSlow73)) as F32)) + fSlow74;
			let mut fTemp460: F32 = self.fRec305[0] + -1.49999;
			let mut fTemp461: F32 = F32::floor(fTemp460);
			let mut fTemp462: F32 = 0.760314 * self.fRec293[1] + 0.64955574 * fTemp420;
			self.fVec64[(self.IOTA0 & 1023) as usize] = fTemp462;
			let mut fTemp463: F32 = fSlow75 * self.fRec299[0];
			let mut fTemp464: F32 = fSlow70 + fTemp463 + 3.500005;
			let mut iTemp465: i32 = ((fTemp464) as i32);
			let mut fTemp466: F32 = F32::floor(fTemp464);
			let mut fTemp467: F32 = fSlow70 + fTemp463 + (2.0 - fTemp466);
			let mut fTemp468: F32 = fSlow70 + fTemp463 + (3.0 - fTemp466);
			let mut fTemp469: F32 = fSlow70 + fTemp463 + (4.0 - fTemp466);
			let mut fTemp470: F32 = fTemp469 * fTemp468;
			let mut fTemp471: F32 = fSlow70 + fTemp463 + (1.0 - fTemp466);
			self.fVec65[(self.IOTA0 & 16383) as usize] = self.fVec64[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, std::cmp::max(0, iTemp465)))) & 1023) as usize] * (0.0 - fTemp469) * (0.0 - 0.5 * fTemp468) * (0.0 - 0.33333334 * fTemp467) * (0.0 - 0.25 * fTemp471) + (fSlow70 + fTemp463 + (5.0 - fTemp466)) * (self.fVec64[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp465, 1))))) & 1023) as usize] * (0.0 - fTemp468) * (0.0 - 0.5 * fTemp467) * (0.0 - 0.33333334 * fTemp471) + 0.5 * fTemp469 * self.fVec64[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp465, 2))))) & 1023) as usize] * (0.0 - fTemp467) * (0.0 - 0.5 * fTemp471) + 0.16666667 * fTemp470 * self.fVec64[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp465, 3))))) & 1023) as usize] * (0.0 - fTemp471) + 0.041666668 * fTemp470 * fTemp467 * self.fVec64[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp465, 4))))) & 1023) as usize]);
			let mut fTemp472: F32 = self.fVec65[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp460) as i32))))) & 16383) as usize];
			self.fVec66[0] = fTemp472;
			self.fRec304[0] = self.fVec66[1] - (fTemp461 + (2.0 - self.fRec305[0])) * (self.fRec304[1] - fTemp472) / (self.fRec305[0] - fTemp461);
			let mut fTemp473: F32 = 0.760314 * self.fRec304[0] - 0.64955574 * self.fRec301[1];
			let mut fTemp474: F32 = 0.760314 * self.fRec266[0] - 0.64955574 * self.fRec300[1];
			self.fVec67[(self.IOTA0 & 16383) as usize] = fTemp474 - fTemp473;
			let mut fTemp475: F32 = self.fVec67[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp456) as i32))))) & 16383) as usize];
			self.fVec68[0] = fTemp475;
			self.fRec302[0] = 0.70710677 * (fTemp459 * fTemp475 / fTemp458 + self.fVec68[1]) - self.fRec302[1] * fTemp459 / fTemp458;
			self.fRec300[0] = self.fRec302[0];
			self.fRec307[0] = 0.9999 * (self.fRec307[1] + ((i32::wrapping_mul(iTemp361, iSlow76)) as F32)) + fSlow77;
			let mut fTemp476: F32 = self.fRec307[0] + -1.49999;
			let mut fTemp477: F32 = F32::floor(fTemp476);
			let mut fTemp478: F32 = self.fRec307[0] - fTemp477;
			let mut fTemp479: F32 = fTemp477 + (2.0 - self.fRec307[0]);
			self.fVec69[(self.IOTA0 & 16383) as usize] = fTemp474 + fTemp473;
			let mut fTemp480: F32 = self.fVec69[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp476) as i32))))) & 16383) as usize];
			self.fVec70[0] = fTemp480;
			self.fRec306[0] = 0.70710677 * (fTemp479 * fTemp480 / fTemp478 + self.fVec70[1]) - self.fRec306[1] * fTemp479 / fTemp478;
			self.fRec301[0] = self.fRec306[0];
			let mut fTemp481: F32 = 0.760314 * self.fRec300[1] + 0.64955574 * self.fRec266[0];
			self.fRec311[0] = 0.9999 * (self.fRec311[1] + ((i32::wrapping_mul(iTemp361, iSlow78)) as F32)) + fSlow79;
			let mut fTemp482: F32 = self.fRec311[0] + -1.49999;
			let mut fTemp483: F32 = F32::floor(fTemp482);
			let mut fTemp484: F32 = self.fRec311[0] - fTemp483;
			let mut fTemp485: F32 = fTemp483 + (2.0 - self.fRec311[0]);
			let mut fTemp486: F32 = 0.760314 * self.fRec301[1] + 0.64955574 * self.fRec304[0];
			let mut fTemp487: F32 = 0.760314 * fTemp486 - 0.64955574 * self.fRec309[1];
			let mut fTemp488: F32 = 0.760314 * fTemp481 - 0.64955574 * self.fRec308[1];
			self.fVec71[(self.IOTA0 & 16383) as usize] = fTemp488 - fTemp487;
			let mut fTemp489: F32 = self.fVec71[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp482) as i32))))) & 16383) as usize];
			self.fVec72[0] = fTemp489;
			self.fRec310[0] = 0.70710677 * (fTemp485 * fTemp489 / fTemp484 + self.fVec72[1]) - self.fRec310[1] * fTemp485 / fTemp484;
			self.fRec308[0] = self.fRec310[0];
			self.fRec313[0] = 0.9999 * (self.fRec313[1] + ((i32::wrapping_mul(iTemp361, iSlow80)) as F32)) + fSlow81;
			let mut fTemp490: F32 = self.fRec313[0] + -1.49999;
			let mut fTemp491: F32 = F32::floor(fTemp490);
			let mut fTemp492: F32 = self.fRec313[0] - fTemp491;
			let mut fTemp493: F32 = fTemp491 + (2.0 - self.fRec313[0]);
			self.fVec73[(self.IOTA0 & 16383) as usize] = fTemp488 + fTemp487;
			let mut iTemp494: i32 = std::cmp::min(8192, std::cmp::max(0, ((fTemp490) as i32)));
			let mut fTemp495: F32 = self.fVec73[((i32::wrapping_sub(self.IOTA0, iTemp494)) & 16383) as usize];
			self.fVec74[0] = fTemp495;
			self.fRec312[0] = 0.70710677 * (fTemp493 * fTemp495 / fTemp492 + self.fVec74[1]) - fTemp493 * self.fRec312[1] / fTemp492;
			self.fRec309[0] = self.fRec312[0];
			let mut fTemp496: F32 = 0.760314 * self.fRec308[1] + 0.64955574 * fTemp481;
			self.fRec317[0] = 0.9999 * (self.fRec317[1] + ((i32::wrapping_mul(iTemp361, iSlow82)) as F32)) + fSlow83;
			let mut fTemp497: F32 = self.fRec317[0] + -1.49999;
			let mut fTemp498: F32 = F32::floor(fTemp497);
			let mut fTemp499: F32 = self.fRec317[0] - fTemp498;
			let mut fTemp500: F32 = fTemp498 + (2.0 - self.fRec317[0]);
			let mut fTemp501: F32 = 0.760314 * self.fRec309[1] + 0.64955574 * fTemp486;
			let mut fTemp502: F32 = 0.760314 * fTemp501 - 0.64955574 * self.fRec315[1];
			let mut fTemp503: F32 = 0.760314 * fTemp496 - 0.64955574 * self.fRec314[1];
			self.fVec75[(self.IOTA0 & 16383) as usize] = fTemp503 - fTemp502;
			let mut fTemp504: F32 = self.fVec75[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp497) as i32))))) & 16383) as usize];
			self.fVec76[0] = fTemp504;
			self.fRec316[0] = 0.70710677 * (fTemp500 * fTemp504 / fTemp499 + self.fVec76[1]) - self.fRec316[1] * fTemp500 / fTemp499;
			self.fRec314[0] = self.fRec316[0];
			self.fRec319[0] = 0.9999 * (self.fRec319[1] + ((i32::wrapping_mul(iTemp361, iSlow84)) as F32)) + fSlow85;
			let mut fTemp505: F32 = self.fRec319[0] + -1.49999;
			let mut fTemp506: F32 = F32::floor(fTemp505);
			let mut fTemp507: F32 = self.fRec319[0] - fTemp506;
			let mut fTemp508: F32 = fTemp506 + (2.0 - self.fRec319[0]);
			self.fVec77[(self.IOTA0 & 16383) as usize] = fTemp503 + fTemp502;
			let mut iTemp509: i32 = std::cmp::min(8192, std::cmp::max(0, ((fTemp505) as i32)));
			let mut fTemp510: F32 = self.fVec77[((i32::wrapping_sub(self.IOTA0, iTemp509)) & 16383) as usize];
			self.fVec78[0] = fTemp510;
			self.fRec318[0] = 0.70710677 * (fTemp508 * fTemp510 / fTemp507 + self.fVec78[1]) - self.fRec318[1] * fTemp508 / fTemp507;
			self.fRec315[0] = self.fRec318[0];
			let mut fTemp511: F32 = 0.760314 * self.fRec314[1] + 0.64955574 * fTemp496;
			self.fRec323[0] = 0.9999 * (self.fRec323[1] + ((i32::wrapping_mul(iTemp361, iSlow86)) as F32)) + fSlow87;
			let mut fTemp512: F32 = self.fRec323[0] + -1.49999;
			let mut fTemp513: F32 = F32::floor(fTemp512);
			let mut fTemp514: F32 = self.fRec323[0] - fTemp513;
			let mut fTemp515: F32 = fTemp513 + (2.0 - self.fRec323[0]);
			let mut fTemp516: F32 = 0.760314 * self.fRec315[1] + 0.64955574 * fTemp501;
			let mut fTemp517: F32 = 0.760314 * fTemp516 - 0.64955574 * self.fRec321[1];
			let mut fTemp518: F32 = 0.760314 * fTemp511 - 0.64955574 * self.fRec320[1];
			self.fVec79[(self.IOTA0 & 16383) as usize] = fTemp518 - fTemp517;
			let mut iTemp519: i32 = std::cmp::min(8192, std::cmp::max(0, ((fTemp512) as i32)));
			let mut fTemp520: F32 = self.fVec79[((i32::wrapping_sub(self.IOTA0, iTemp519)) & 16383) as usize];
			self.fVec80[0] = fTemp520;
			self.fRec322[0] = 0.70710677 * (fTemp515 * fTemp520 / fTemp514 + self.fVec80[1]) - fTemp515 * self.fRec322[1] / fTemp514;
			self.fRec320[0] = self.fRec322[0];
			self.fRec325[0] = 0.9999 * (self.fRec325[1] + ((i32::wrapping_mul(iTemp361, iSlow88)) as F32)) + fSlow89;
			let mut fTemp521: F32 = self.fRec325[0] + -1.49999;
			let mut fTemp522: F32 = F32::floor(fTemp521);
			let mut fTemp523: F32 = self.fRec325[0] - fTemp522;
			let mut fTemp524: F32 = fTemp522 + (2.0 - self.fRec325[0]);
			self.fVec81[(self.IOTA0 & 16383) as usize] = fTemp518 + fTemp517;
			let mut iTemp525: i32 = std::cmp::min(8192, std::cmp::max(0, ((fTemp521) as i32)));
			let mut fTemp526: F32 = self.fVec81[((i32::wrapping_sub(self.IOTA0, iTemp525)) & 16383) as usize];
			self.fVec82[0] = fTemp526;
			self.fRec324[0] = 0.70710677 * (fTemp524 * fTemp526 / fTemp523 + self.fVec82[1]) - fTemp524 * self.fRec324[1] / fTemp523;
			self.fRec321[0] = self.fRec324[0];
			let mut fTemp527: F32 = 0.760314 * self.fRec320[1] + 0.64955574 * fTemp511;
			self.fRec329[0] = 0.9999 * (self.fRec329[1] + ((i32::wrapping_mul(iTemp361, iSlow90)) as F32)) + fSlow91;
			let mut fTemp528: F32 = self.fRec329[0] + -1.49999;
			let mut fTemp529: F32 = F32::floor(fTemp528);
			let mut fTemp530: F32 = self.fRec329[0] - fTemp529;
			let mut fTemp531: F32 = fTemp529 + (2.0 - self.fRec329[0]);
			let mut fTemp532: F32 = 0.760314 * self.fRec321[1] + 0.64955574 * fTemp516;
			let mut fTemp533: F32 = 0.760314 * fTemp532 - 0.64955574 * self.fRec327[1];
			let mut fTemp534: F32 = 0.760314 * fTemp527 - 0.64955574 * self.fRec326[1];
			self.fVec83[(self.IOTA0 & 16383) as usize] = fTemp534 - fTemp533;
			let mut fTemp535: F32 = self.fVec83[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp528) as i32))))) & 16383) as usize];
			self.fVec84[0] = fTemp535;
			self.fRec328[0] = 0.70710677 * (fTemp531 * fTemp535 / fTemp530 + self.fVec84[1]) - self.fRec328[1] * fTemp531 / fTemp530;
			self.fRec326[0] = self.fRec328[0];
			self.fRec331[0] = 0.9999 * (self.fRec331[1] + ((i32::wrapping_mul(iTemp361, iSlow92)) as F32)) + fSlow93;
			let mut fTemp536: F32 = self.fRec331[0] + -1.49999;
			let mut fTemp537: F32 = F32::floor(fTemp536);
			let mut fTemp538: F32 = self.fRec331[0] - fTemp537;
			let mut fTemp539: F32 = fTemp537 + (2.0 - self.fRec331[0]);
			self.fVec85[(self.IOTA0 & 16383) as usize] = fTemp534 + fTemp533;
			let mut iTemp540: i32 = std::cmp::min(8192, std::cmp::max(0, ((fTemp536) as i32)));
			let mut fTemp541: F32 = self.fVec85[((i32::wrapping_sub(self.IOTA0, iTemp540)) & 16383) as usize];
			self.fVec86[0] = fTemp541;
			self.fRec330[0] = 0.70710677 * (fTemp539 * fTemp541 / fTemp538 + self.fVec86[1]) - self.fRec330[1] * fTemp539 / fTemp538;
			self.fRec327[0] = self.fRec330[0];
			let mut fTemp542: F32 = 0.760314 * self.fRec326[1] + 0.64955574 * fTemp527;
			self.fVec87[(self.IOTA0 & 16383) as usize] = fTemp542;
			let mut fTemp543: F32 = fSlow70 * (self.fRec298[0] + 1.0);
			let mut fTemp544: F32 = fTemp543 + 3.500005;
			let mut iTemp545: i32 = ((fTemp544) as i32);
			let mut iTemp546: i32 = std::cmp::max(0, i32::wrapping_add(iTemp545, 4));
			let mut fTemp547: F32 = F32::floor(fTemp544);
			let mut fTemp548: F32 = fTemp543 + (2.0 - fTemp547);
			let mut fTemp549: F32 = fTemp543 + (3.0 - fTemp547);
			let mut fTemp550: F32 = fTemp543 + (4.0 - fTemp547);
			let mut fTemp551: F32 = fTemp550 * fTemp549;
			let mut fTemp552: F32 = fTemp551 * fTemp548;
			let mut fTemp553: F32 = fTemp543 + (1.0 - fTemp547);
			let mut fTemp554: F32 = 0.0 - fTemp553;
			let mut iTemp555: i32 = std::cmp::max(0, i32::wrapping_add(iTemp545, 3));
			let mut fTemp556: F32 = 0.0 - 0.5 * fTemp553;
			let mut fTemp557: F32 = 0.0 - fTemp548;
			let mut iTemp558: i32 = std::cmp::max(0, i32::wrapping_add(iTemp545, 2));
			let mut fTemp559: F32 = 0.0 - 0.33333334 * fTemp553;
			let mut fTemp560: F32 = 0.0 - 0.5 * fTemp548;
			let mut fTemp561: F32 = 0.0 - fTemp549;
			let mut iTemp562: i32 = std::cmp::max(0, i32::wrapping_add(iTemp545, 1));
			let mut fTemp563: F32 = fTemp543 + (5.0 - fTemp547);
			let mut fTemp564: F32 = 0.0 - 0.25 * fTemp553;
			let mut fTemp565: F32 = 0.0 - 0.33333334 * fTemp548;
			let mut fTemp566: F32 = 0.0 - 0.5 * fTemp549;
			let mut fTemp567: F32 = 0.0 - fTemp550;
			let mut iTemp568: i32 = std::cmp::max(0, iTemp545);
			self.fVec88[(self.IOTA0 & 16383) as usize] = self.fVec87[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, iTemp568))) & 16383) as usize] * fTemp567 * fTemp566 * fTemp565 * fTemp564 + fTemp563 * (self.fVec87[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, iTemp562))) & 16383) as usize] * fTemp561 * fTemp560 * fTemp559 + 0.5 * fTemp550 * self.fVec87[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, iTemp558))) & 16383) as usize] * fTemp557 * fTemp556 + 0.16666667 * fTemp551 * self.fVec87[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, iTemp555))) & 16383) as usize] * fTemp554 + 0.041666668 * fTemp552 * self.fVec87[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, iTemp546))) & 16383) as usize]);
			let mut fTemp569: F32 = self.fVec88[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp362) as i32))))) & 16383) as usize];
			self.fVec89[0] = fTemp569;
			self.fRec264[0] = self.fVec89[1] - (fTemp363 + (2.0 - self.fRec265[0])) * (self.fRec264[1] - fTemp569) / (self.fRec265[0] - fTemp363);
			self.fRec263[0] = 0.0 - self.fConst41 * (self.fConst39 * self.fRec263[1] - (self.fRec264[0] + self.fRec264[1]));
			self.fRec262[0] = self.fRec263[0] - self.fConst37 * (self.fConst36 * self.fRec262[2] + self.fConst32 * self.fRec262[1]);
			self.fRec261[0] = self.fConst37 * (self.fRec262[2] + self.fRec262[0] + 2.0 * self.fRec262[1]) - self.fConst35 * (self.fConst34 * self.fRec261[2] + self.fConst32 * self.fRec261[1]);
			let mut fTemp570: F32 = self.fRec261[2] + self.fRec261[0] + 2.0 * self.fRec261[1];
			self.fVec90[0] = fTemp570;
			self.fRec260[0] = 0.0 - self.fConst44 * (self.fConst42 * self.fRec260[1] - self.fConst35 * (fTemp570 + self.fVec90[1]));
			self.fRec259[0] = self.fRec260[0] - self.fConst28 * (self.fConst27 * self.fRec259[2] + self.fConst23 * self.fRec259[1]);
			self.fRec258[0] = self.fConst28 * (self.fRec259[2] + self.fRec259[0] + 2.0 * self.fRec259[1]) - self.fConst26 * (self.fConst25 * self.fRec258[2] + self.fConst23 * self.fRec258[1]);
			self.fRec334[0] = self.fConst35 * (self.fConst46 * fTemp570 + self.fConst47 * self.fVec90[1]) - self.fConst45 * self.fRec334[1];
			self.fRec333[0] = self.fRec334[0] - self.fConst28 * (self.fConst27 * self.fRec333[2] + self.fConst23 * self.fRec333[1]);
			self.fRec332[0] = self.fConst28 * (self.fConst22 * self.fRec333[0] + self.fConst48 * self.fRec333[1] + self.fConst22 * self.fRec333[2]) - self.fConst26 * (self.fConst25 * self.fRec332[2] + self.fConst23 * self.fRec332[1]);
			let mut fTemp571: F32 = self.fConst23 * self.fRec335[1];
			self.fRec338[0] = self.fConst51 * self.fRec264[1] - self.fConst41 * (self.fConst39 * self.fRec338[1] - self.fConst33 * self.fRec264[0]);
			self.fRec337[0] = self.fRec338[0] - self.fConst37 * (self.fConst36 * self.fRec337[2] + self.fConst32 * self.fRec337[1]);
			self.fRec336[0] = self.fConst37 * (self.fConst31 * self.fRec337[0] + self.fConst52 * self.fRec337[1] + self.fConst31 * self.fRec337[2]) - self.fConst35 * (self.fConst34 * self.fRec336[2] + self.fConst32 * self.fRec336[1]);
			self.fRec335[0] = self.fConst35 * (self.fConst31 * self.fRec336[0] + self.fConst52 * self.fRec336[1] + self.fConst31 * self.fRec336[2]) - self.fConst50 * (self.fConst49 * self.fRec335[2] + fTemp571);
			let mut fTemp572: F32 = fTemp358 + fTemp359 + fSlow94 * (self.fRec335[2] + self.fConst50 * (fTemp571 + self.fConst49 * self.fRec335[0]) + self.fConst26 * (self.fConst22 * self.fRec332[0] + self.fConst48 * self.fRec332[1] + self.fConst22 * self.fRec332[2] + self.fRec258[2] + self.fRec258[0] + 2.0 * self.fRec258[1]));
			self.fVec91[(self.IOTA0 & 1023) as usize] = fTemp572;
			self.fRec257[0] = fSlow95 * self.fRec257[1] + fSlow96 * (fTemp567 * fTemp566 * fTemp565 * fTemp564 * self.fVec91[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, iTemp568))) & 1023) as usize] + fTemp563 * (fTemp561 * fTemp560 * fTemp559 * self.fVec91[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, iTemp562))) & 1023) as usize] + 0.5 * fTemp550 * fTemp557 * fTemp556 * self.fVec91[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, iTemp558))) & 1023) as usize] + 0.16666667 * fTemp551 * fTemp554 * self.fVec91[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, iTemp555))) & 1023) as usize] + 0.041666668 * fTemp552 * self.fVec91[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, iTemp546))) & 1023) as usize]));
			self.fRec350[0] = 0.995 * (self.fRec350[1] + ((i32::wrapping_mul(iTemp361, iSlow99)) as F32)) + fSlow100;
			let mut fTemp573: F32 = self.fRec350[0] + -1.49999;
			let mut fTemp574: F32 = F32::floor(fTemp573);
			let mut fTemp575: F32 = 0.760314 * self.fRec327[1] + 0.64955574 * fTemp532;
			self.fVec92[(self.IOTA0 & 16383) as usize] = fTemp575;
			let mut fTemp576: F32 = fSlow75 * self.fRec298[0];
			let mut fTemp577: F32 = fSlow70 + fTemp576 + 3.500005;
			let mut iTemp578: i32 = ((fTemp577) as i32);
			let mut fTemp579: F32 = F32::floor(fTemp577);
			let mut fTemp580: F32 = fSlow70 + fTemp576 + (2.0 - fTemp579);
			let mut fTemp581: F32 = fSlow70 + fTemp576 + (3.0 - fTemp579);
			let mut fTemp582: F32 = fSlow70 + fTemp576 + (4.0 - fTemp579);
			let mut fTemp583: F32 = fTemp582 * fTemp581;
			let mut fTemp584: F32 = fSlow70 + fTemp576 + (1.0 - fTemp579);
			self.fVec93[(self.IOTA0 & 16383) as usize] = self.fVec92[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, iTemp578)))) & 16383) as usize] * (0.0 - fTemp582) * (0.0 - 0.5 * fTemp581) * (0.0 - 0.33333334 * fTemp580) * (0.0 - 0.25 * fTemp584) + (fSlow70 + fTemp576 + (5.0 - fTemp579)) * (self.fVec92[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, i32::wrapping_add(iTemp578, 1))))) & 16383) as usize] * (0.0 - fTemp581) * (0.0 - 0.5 * fTemp580) * (0.0 - 0.33333334 * fTemp584) + 0.5 * fTemp582 * self.fVec92[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, i32::wrapping_add(iTemp578, 2))))) & 16383) as usize] * (0.0 - fTemp580) * (0.0 - 0.5 * fTemp584) + 0.16666667 * fTemp583 * self.fVec92[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, i32::wrapping_add(iTemp578, 3))))) & 16383) as usize] * (0.0 - fTemp584) + 0.041666668 * fTemp583 * fTemp580 * self.fVec92[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, i32::wrapping_add(iTemp578, 4))))) & 16383) as usize]);
			let mut fTemp585: F32 = self.fVec93[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp573) as i32))))) & 16383) as usize];
			self.fVec94[0] = fTemp585;
			self.fRec349[0] = self.fVec94[1] - (fTemp574 + (2.0 - self.fRec350[0])) * (self.fRec349[1] - fTemp585) / (self.fRec350[0] - fTemp574);
			self.fRec348[0] = 0.0 - self.fConst41 * (self.fConst39 * self.fRec348[1] - (self.fRec349[0] + self.fRec349[1]));
			self.fRec347[0] = self.fRec348[0] - self.fConst37 * (self.fConst36 * self.fRec347[2] + self.fConst32 * self.fRec347[1]);
			self.fRec346[0] = self.fConst37 * (self.fRec347[2] + self.fRec347[0] + 2.0 * self.fRec347[1]) - self.fConst35 * (self.fConst34 * self.fRec346[2] + self.fConst32 * self.fRec346[1]);
			let mut fTemp586: F32 = self.fRec346[2] + self.fRec346[0] + 2.0 * self.fRec346[1];
			self.fVec95[0] = fTemp586;
			self.fRec345[0] = self.fConst44 * (self.fConst35 * (fTemp586 + self.fVec95[1]) - self.fConst42 * self.fRec345[1]);
			self.fRec344[0] = self.fRec345[0] - self.fConst28 * (self.fConst27 * self.fRec344[2] + self.fConst23 * self.fRec344[1]);
			self.fRec343[0] = self.fConst28 * (self.fRec344[2] + self.fRec344[0] + 2.0 * self.fRec344[1]) - self.fConst26 * (self.fConst25 * self.fRec343[2] + self.fConst23 * self.fRec343[1]);
			self.fRec353[0] = self.fConst35 * (self.fConst46 * fTemp586 + self.fConst47 * self.fVec95[1]) - self.fConst45 * self.fRec353[1];
			self.fRec352[0] = self.fRec353[0] - self.fConst28 * (self.fConst27 * self.fRec352[2] + self.fConst23 * self.fRec352[1]);
			self.fRec351[0] = self.fConst28 * (self.fConst22 * self.fRec352[0] + self.fConst48 * self.fRec352[1] + self.fConst22 * self.fRec352[2]) - self.fConst26 * (self.fConst25 * self.fRec351[2] + self.fConst23 * self.fRec351[1]);
			let mut fTemp587: F32 = self.fConst23 * self.fRec354[1];
			self.fRec357[0] = self.fConst51 * self.fRec349[1] - self.fConst41 * (self.fConst39 * self.fRec357[1] - self.fConst33 * self.fRec349[0]);
			self.fRec356[0] = self.fRec357[0] - self.fConst37 * (self.fConst36 * self.fRec356[2] + self.fConst32 * self.fRec356[1]);
			self.fRec355[0] = self.fConst37 * (self.fConst52 * self.fRec356[1] + self.fConst31 * self.fRec356[0] + self.fConst31 * self.fRec356[2]) - self.fConst35 * (self.fConst34 * self.fRec355[2] + self.fConst32 * self.fRec355[1]);
			self.fRec354[0] = self.fConst35 * (self.fConst52 * self.fRec355[1] + self.fConst31 * self.fRec355[0] + self.fConst31 * self.fRec355[2]) - self.fConst50 * (self.fConst49 * self.fRec354[2] + fTemp587);
			let mut fTemp588: F32 = fTemp360 + fSlow94 * (self.fRec354[2] + self.fConst50 * (fTemp587 + self.fConst49 * self.fRec354[0]) + self.fConst26 * (self.fConst22 * self.fRec351[0] + self.fConst48 * self.fRec351[1] + self.fConst22 * self.fRec351[2] + self.fRec343[2] + self.fRec343[0] + 2.0 * self.fRec343[1]));
			self.fVec96[(self.IOTA0 & 1023) as usize] = fTemp588;
			self.fRec342[0] = fSlow95 * self.fRec342[1] + fSlow96 * (fTemp453 * fTemp452 * fTemp451 * fTemp450 * self.fVec96[((i32::wrapping_sub(self.IOTA0, iTemp454)) & 1023) as usize] + fTemp449 * (fTemp447 * fTemp446 * fTemp445 * self.fVec96[((i32::wrapping_sub(self.IOTA0, iTemp448)) & 1023) as usize] + 0.5 * fTemp436 * fTemp443 * fTemp442 * self.fVec96[((i32::wrapping_sub(self.IOTA0, iTemp444)) & 1023) as usize] + 0.16666667 * fTemp437 * fTemp440 * self.fVec96[((i32::wrapping_sub(self.IOTA0, iTemp441)) & 1023) as usize] + 0.041666668 * fTemp438 * self.fVec96[((i32::wrapping_sub(self.IOTA0, iTemp432)) & 1023) as usize]));
			let mut fTemp589: F32 = fSlow101 * self.fRec342[0] - fSlow98 * self.fRec340[1];
			let mut fTemp590: F32 = fSlow101 * self.fRec257[0] - fSlow98 * self.fRec339[1];
			self.fVec97[(self.IOTA0 & 16383) as usize] = fTemp590 - fTemp589;
			let mut fTemp591: F32 = self.fVec97[((i32::wrapping_sub(self.IOTA0, iTemp494)) & 16383) as usize];
			self.fVec98[0] = fTemp591;
			self.fRec341[0] = 0.70710677 * (fTemp493 * fTemp591 / fTemp492 + self.fVec98[1]) - self.fRec341[1] * fTemp493 / fTemp492;
			self.fRec339[0] = self.fRec341[0];
			self.fRec359[0] = 0.9999 * (self.fRec359[1] + ((i32::wrapping_mul(iTemp361, iSlow102)) as F32)) + fSlow103;
			let mut fTemp592: F32 = self.fRec359[0] + -1.49999;
			let mut fTemp593: F32 = F32::floor(fTemp592);
			let mut fTemp594: F32 = self.fRec359[0] - fTemp593;
			let mut fTemp595: F32 = fTemp593 + (2.0 - self.fRec359[0]);
			self.fVec99[(self.IOTA0 & 16383) as usize] = fTemp590 + fTemp589;
			let mut fTemp596: F32 = self.fVec99[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp592) as i32))))) & 16383) as usize];
			self.fVec100[0] = fTemp596;
			self.fRec358[0] = 0.70710677 * (fTemp595 * fTemp596 / fTemp594 + self.fVec100[1]) - self.fRec358[1] * fTemp595 / fTemp594;
			self.fRec340[0] = self.fRec358[0];
			let mut fTemp597: F32 = fSlow101 * self.fRec339[1] + fSlow98 * self.fRec257[0];
			let mut fTemp598: F32 = fSlow101 * fTemp597 - fSlow98 * self.fRec360[1];
			let mut fTemp599: F32 = fSlow101 * self.fRec340[1] + fSlow98 * self.fRec342[0];
			let mut fTemp600: F32 = fSlow101 * fTemp599 - fSlow98 * self.fRec361[1];
			self.fVec101[(self.IOTA0 & 16383) as usize] = 0.0 - 0.70710677 * (fTemp600 - fTemp598);
			let mut fTemp601: F32 = self.fVec101[((i32::wrapping_sub(self.IOTA0, iTemp519)) & 16383) as usize];
			self.fVec102[0] = fTemp601;
			self.fRec362[0] = self.fVec102[1] - fTemp515 * (self.fRec362[1] - fTemp601) / fTemp514;
			self.fRec360[0] = self.fRec362[0];
			self.fVec103[(self.IOTA0 & 16383) as usize] = fTemp598 + fTemp600;
			let mut fTemp602: F32 = self.fVec103[((i32::wrapping_sub(self.IOTA0, iTemp509)) & 16383) as usize];
			self.fVec104[0] = fTemp602;
			self.fRec363[0] = 0.70710677 * (fTemp508 * fTemp602 / fTemp507 + self.fVec104[1]) - fTemp508 * self.fRec363[1] / fTemp507;
			self.fRec361[0] = self.fRec363[0];
			let mut fTemp603: F32 = fSlow101 * self.fRec360[1] + fSlow98 * fTemp597;
			let mut fTemp604: F32 = fSlow101 * self.fRec361[1] + fSlow98 * fTemp599;
			let mut fTemp605: F32 = fSlow101 * fTemp604 - fSlow98 * self.fRec365[1];
			let mut fTemp606: F32 = fSlow101 * fTemp603 - fSlow98 * self.fRec364[1];
			self.fVec105[(self.IOTA0 & 16383) as usize] = fTemp606 - fTemp605;
			let mut fTemp607: F32 = self.fVec105[((i32::wrapping_sub(self.IOTA0, iTemp525)) & 16383) as usize];
			self.fVec106[0] = fTemp607;
			self.fRec366[0] = 0.70710677 * (fTemp524 * fTemp607 / fTemp523 + self.fVec106[1]) - self.fRec366[1] * fTemp524 / fTemp523;
			self.fRec364[0] = self.fRec366[0];
			self.fRec368[0] = 0.9999 * (self.fRec368[1] + ((i32::wrapping_mul(iTemp361, iSlow104)) as F32)) + fSlow105;
			let mut fTemp608: F32 = self.fRec368[0] + -1.49999;
			let mut fTemp609: F32 = F32::floor(fTemp608);
			let mut fTemp610: F32 = self.fRec368[0] - fTemp609;
			let mut fTemp611: F32 = fTemp609 + (2.0 - self.fRec368[0]);
			self.fVec107[(self.IOTA0 & 16383) as usize] = fTemp606 + fTemp605;
			let mut fTemp612: F32 = self.fVec107[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp608) as i32))))) & 16383) as usize];
			self.fVec108[0] = fTemp612;
			self.fRec367[0] = 0.70710677 * (fTemp611 * fTemp612 / fTemp610 + self.fVec108[1]) - self.fRec367[1] * fTemp611 / fTemp610;
			self.fRec365[0] = self.fRec367[0];
			let mut fTemp613: F32 = fSlow101 * self.fRec364[1] + fSlow98 * fTemp603;
			self.fRec372[0] = 0.9999 * (self.fRec372[1] + ((i32::wrapping_mul(iTemp361, iSlow106)) as F32)) + fSlow107;
			let mut fTemp614: F32 = self.fRec372[0] + -1.49999;
			let mut fTemp615: F32 = F32::floor(fTemp614);
			let mut fTemp616: F32 = self.fRec372[0] - fTemp615;
			let mut fTemp617: F32 = fTemp615 + (2.0 - self.fRec372[0]);
			let mut fTemp618: F32 = fSlow101 * self.fRec365[1] + fSlow98 * fTemp604;
			let mut fTemp619: F32 = fSlow101 * fTemp618 - fSlow98 * self.fRec370[1];
			let mut fTemp620: F32 = fSlow101 * fTemp613 - fSlow98 * self.fRec369[1];
			self.fVec109[(self.IOTA0 & 16383) as usize] = fTemp620 - fTemp619;
			let mut fTemp621: F32 = self.fVec109[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp614) as i32))))) & 16383) as usize];
			self.fVec110[0] = fTemp621;
			self.fRec371[0] = 0.70710677 * (fTemp617 * fTemp621 / fTemp616 + self.fVec110[1]) - self.fRec371[1] * fTemp617 / fTemp616;
			self.fRec369[0] = self.fRec371[0];
			self.fVec111[(self.IOTA0 & 16383) as usize] = fTemp620 + fTemp619;
			let mut fTemp622: F32 = self.fVec111[((i32::wrapping_sub(self.IOTA0, iTemp540)) & 16383) as usize];
			self.fVec112[0] = fTemp622;
			self.fRec373[0] = 0.70710677 * (fTemp539 * fTemp622 / fTemp538 + self.fVec112[1]) - fTemp539 * self.fRec373[1] / fTemp538;
			self.fRec370[0] = self.fRec373[0];
			self.fRec255[0] = fSlow101 * self.fRec369[1] + fSlow98 * fTemp613;
			self.fRec256[0] = fSlow101 * self.fRec370[1] + fSlow98 * fTemp618;
			self.fRec374[0] = fSlow108 + self.fConst2 * self.fRec374[1];
			let mut fTemp623: F32 = self.fRec374[0] * (fSlow40 * (self.fRec255[0] + self.fRec256[0]) + fSlow41 * fTemp360);
			*output0 = fTemp623;
			*output1 = fTemp623;
			self.iVec0[1] = self.iVec0[0];
			self.fRec0[1] = self.fRec0[0];
			self.fRec2[1] = self.fRec2[0];
			self.fVec1[1] = self.fVec1[0];
			self.IOTA0 = i32::wrapping_add(self.IOTA0, 1);
			self.fRec1[1] = self.fRec1[0];
			self.fRec4[1] = self.fRec4[0];
			self.fVec3[1] = self.fVec3[0];
			self.fRec3[1] = self.fRec3[0];
			self.fRec6[1] = self.fRec6[0];
			self.fVec5[1] = self.fVec5[0];
			self.fRec5[1] = self.fRec5[0];
			self.fRec7[1] = self.fRec7[0];
			self.fRec8[1] = self.fRec8[0];
			self.fRec10[1] = self.fRec10[0];
			self.fVec7[1] = self.fVec7[0];
			self.fRec9[1] = self.fRec9[0];
			self.fRec12[1] = self.fRec12[0];
			self.fVec9[1] = self.fVec9[0];
			self.fRec11[1] = self.fRec11[0];
			self.fRec14[1] = self.fRec14[0];
			self.fVec11[1] = self.fVec11[0];
			self.fRec13[1] = self.fRec13[0];
			self.fRec15[1] = self.fRec15[0];
			self.fRec16[1] = self.fRec16[0];
			self.fRec18[1] = self.fRec18[0];
			self.fVec13[1] = self.fVec13[0];
			self.fRec17[1] = self.fRec17[0];
			self.fRec20[1] = self.fRec20[0];
			self.fVec15[1] = self.fVec15[0];
			self.fRec19[1] = self.fRec19[0];
			self.fRec22[1] = self.fRec22[0];
			self.fVec17[1] = self.fVec17[0];
			self.fRec21[1] = self.fRec21[0];
			self.fRec23[1] = self.fRec23[0];
			self.fRec24[1] = self.fRec24[0];
			self.fRec26[1] = self.fRec26[0];
			self.fVec19[1] = self.fVec19[0];
			self.fRec25[1] = self.fRec25[0];
			self.fRec28[1] = self.fRec28[0];
			self.fVec21[1] = self.fVec21[0];
			self.fRec27[1] = self.fRec27[0];
			self.fRec30[1] = self.fRec30[0];
			self.fVec23[1] = self.fVec23[0];
			self.fRec29[1] = self.fRec29[0];
			self.fRec31[1] = self.fRec31[0];
			self.fRec32[1] = self.fRec32[0];
			self.fRec33[1] = self.fRec33[0];
			self.fRec35[1] = self.fRec35[0];
			self.fRec36[1] = self.fRec36[0];
			self.fRec66[1] = self.fRec66[0];
			self.fRec70[1] = self.fRec70[0];
			self.fRec75[1] = self.fRec75[0];
			self.iVec25[1] = self.iVec25[0];
			self.iRec76[1] = self.iRec76[0];
			self.fRec73[1] = self.fRec73[0];
			self.fRec72[1] = self.fRec72[0];
			for j0 in (1..=3).rev() {
				self.fRec77[(j0) as usize] = self.fRec77[(i32::wrapping_sub(j0, 1)) as usize];
			}
			self.fVec26[1] = self.fVec26[0];
			self.fVec27[1] = self.fVec27[0];
			self.iRec79[1] = self.iRec79[0];
			self.iRec81[1] = self.iRec81[0];
			self.fRec80[2] = self.fRec80[1];
			self.fRec80[1] = self.fRec80[0];
			self.fVec28[2] = self.fVec28[1];
			self.fVec28[1] = self.fVec28[0];
			self.fRec62[1] = self.fRec62[0];
			self.fRec58[1] = self.fRec58[0];
			self.fRec56[1] = self.fRec56[0];
			for j1 in (1..=3).rev() {
				self.fRec52[(j1) as usize] = self.fRec52[(i32::wrapping_sub(j1, 1)) as usize];
			}
			self.fRec47[1] = self.fRec47[0];
			self.fRec41[1] = self.fRec41[0];
			self.fRec40[1] = self.fRec40[0];
			self.fRec39[1] = self.fRec39[0];
			self.fRec37[1] = self.fRec37[0];
			self.fRec34[1] = self.fRec34[0];
			self.fRec83[1] = self.fRec83[0];
			self.fRec113[1] = self.fRec113[0];
			self.fRec117[1] = self.fRec117[0];
			self.fRec122[1] = self.fRec122[0];
			self.iVec29[1] = self.iVec29[0];
			self.iRec123[1] = self.iRec123[0];
			self.fRec120[1] = self.fRec120[0];
			self.fRec119[1] = self.fRec119[0];
			for j2 in (1..=3).rev() {
				self.fRec124[(j2) as usize] = self.fRec124[(i32::wrapping_sub(j2, 1)) as usize];
			}
			self.fVec30[1] = self.fVec30[0];
			self.fVec31[1] = self.fVec31[0];
			self.iRec126[1] = self.iRec126[0];
			self.fRec127[2] = self.fRec127[1];
			self.fRec127[1] = self.fRec127[0];
			self.fVec32[2] = self.fVec32[1];
			self.fVec32[1] = self.fVec32[0];
			self.fRec109[1] = self.fRec109[0];
			self.fRec105[1] = self.fRec105[0];
			self.fRec103[1] = self.fRec103[0];
			for j3 in (1..=3).rev() {
				self.fRec99[(j3) as usize] = self.fRec99[(i32::wrapping_sub(j3, 1)) as usize];
			}
			self.fRec94[1] = self.fRec94[0];
			self.fRec88[1] = self.fRec88[0];
			self.fRec87[1] = self.fRec87[0];
			self.fRec86[1] = self.fRec86[0];
			self.fRec84[1] = self.fRec84[0];
			self.fRec82[1] = self.fRec82[0];
			self.fRec129[1] = self.fRec129[0];
			self.fRec159[1] = self.fRec159[0];
			self.fRec163[1] = self.fRec163[0];
			self.fRec168[1] = self.fRec168[0];
			self.iVec33[1] = self.iVec33[0];
			self.iRec169[1] = self.iRec169[0];
			self.fRec166[1] = self.fRec166[0];
			self.fRec165[1] = self.fRec165[0];
			for j4 in (1..=3).rev() {
				self.fRec170[(j4) as usize] = self.fRec170[(i32::wrapping_sub(j4, 1)) as usize];
			}
			self.fVec34[1] = self.fVec34[0];
			self.fVec35[1] = self.fVec35[0];
			self.iRec172[1] = self.iRec172[0];
			self.fRec173[2] = self.fRec173[1];
			self.fRec173[1] = self.fRec173[0];
			self.fVec36[2] = self.fVec36[1];
			self.fVec36[1] = self.fVec36[0];
			self.fRec155[1] = self.fRec155[0];
			self.fRec151[1] = self.fRec151[0];
			self.fRec149[1] = self.fRec149[0];
			for j5 in (1..=3).rev() {
				self.fRec145[(j5) as usize] = self.fRec145[(i32::wrapping_sub(j5, 1)) as usize];
			}
			self.fRec140[1] = self.fRec140[0];
			self.fRec134[1] = self.fRec134[0];
			self.fRec133[1] = self.fRec133[0];
			self.fRec132[1] = self.fRec132[0];
			self.fRec130[1] = self.fRec130[0];
			self.fRec128[1] = self.fRec128[0];
			self.fRec175[1] = self.fRec175[0];
			self.fRec205[1] = self.fRec205[0];
			self.fRec209[1] = self.fRec209[0];
			self.fRec214[1] = self.fRec214[0];
			self.iVec37[1] = self.iVec37[0];
			self.iRec215[1] = self.iRec215[0];
			self.fRec212[1] = self.fRec212[0];
			self.fRec211[1] = self.fRec211[0];
			for j6 in (1..=3).rev() {
				self.fRec216[(j6) as usize] = self.fRec216[(i32::wrapping_sub(j6, 1)) as usize];
			}
			self.fVec38[1] = self.fVec38[0];
			self.fVec39[1] = self.fVec39[0];
			self.iRec218[1] = self.iRec218[0];
			self.fRec219[2] = self.fRec219[1];
			self.fRec219[1] = self.fRec219[0];
			self.fVec40[2] = self.fVec40[1];
			self.fVec40[1] = self.fVec40[0];
			self.fRec201[1] = self.fRec201[0];
			self.fRec197[1] = self.fRec197[0];
			self.fRec195[1] = self.fRec195[0];
			for j7 in (1..=3).rev() {
				self.fRec191[(j7) as usize] = self.fRec191[(i32::wrapping_sub(j7, 1)) as usize];
			}
			self.fRec186[1] = self.fRec186[0];
			self.fRec180[1] = self.fRec180[0];
			self.fRec179[1] = self.fRec179[0];
			self.fRec178[1] = self.fRec178[0];
			self.fRec176[1] = self.fRec176[0];
			self.fRec174[1] = self.fRec174[0];
			self.fRec220[1] = self.fRec220[0];
			self.fRec222[1] = self.fRec222[0];
			self.fRec221[1] = self.fRec221[0];
			self.fRec227[1] = self.fRec227[0];
			self.fRec225[1] = self.fRec225[0];
			self.fRec228[1] = self.fRec228[0];
			self.fRec224[2] = self.fRec224[1];
			self.fRec224[1] = self.fRec224[0];
			self.fRec223[2] = self.fRec223[1];
			self.fRec223[1] = self.fRec223[0];
			self.fRec229[1] = self.fRec229[0];
			self.fRec234[1] = self.fRec234[0];
			self.fRec232[1] = self.fRec232[0];
			self.fRec235[1] = self.fRec235[0];
			self.fRec231[2] = self.fRec231[1];
			self.fRec231[1] = self.fRec231[0];
			self.fRec230[2] = self.fRec230[1];
			self.fRec230[1] = self.fRec230[0];
			self.fRec236[1] = self.fRec236[0];
			self.fRec241[1] = self.fRec241[0];
			self.fRec239[1] = self.fRec239[0];
			self.fRec242[1] = self.fRec242[0];
			self.fRec238[2] = self.fRec238[1];
			self.fRec238[1] = self.fRec238[0];
			self.fRec237[2] = self.fRec237[1];
			self.fRec237[1] = self.fRec237[0];
			self.fRec243[1] = self.fRec243[0];
			self.fRec248[1] = self.fRec248[0];
			self.fRec246[1] = self.fRec246[0];
			self.fRec249[1] = self.fRec249[0];
			self.fRec245[2] = self.fRec245[1];
			self.fRec245[1] = self.fRec245[0];
			self.fRec244[2] = self.fRec244[1];
			self.fRec244[1] = self.fRec244[0];
			self.fRec250[1] = self.fRec250[0];
			self.fRec251[1] = self.fRec251[0];
			self.fRec252[1] = self.fRec252[0];
			self.fRec254[1] = self.fRec254[0];
			self.fRec265[1] = self.fRec265[0];
			self.fRec267[1] = self.fRec267[0];
			self.fRec271[1] = self.fRec271[0];
			self.fVec42[1] = self.fVec42[0];
			self.fRec270[1] = self.fRec270[0];
			self.fRec268[1] = self.fRec268[0];
			self.fRec273[1] = self.fRec273[0];
			self.fVec44[1] = self.fVec44[0];
			self.fRec272[1] = self.fRec272[0];
			self.fRec269[1] = self.fRec269[0];
			self.fRec277[1] = self.fRec277[0];
			self.fVec46[1] = self.fVec46[0];
			self.fRec276[1] = self.fRec276[0];
			self.fRec274[1] = self.fRec274[0];
			self.fRec279[1] = self.fRec279[0];
			self.fVec48[1] = self.fVec48[0];
			self.fRec278[1] = self.fRec278[0];
			self.fRec275[1] = self.fRec275[0];
			self.fRec283[1] = self.fRec283[0];
			self.fVec50[1] = self.fVec50[0];
			self.fRec282[1] = self.fRec282[0];
			self.fRec280[1] = self.fRec280[0];
			self.fRec285[1] = self.fRec285[0];
			self.fVec52[1] = self.fVec52[0];
			self.fRec284[1] = self.fRec284[0];
			self.fRec281[1] = self.fRec281[0];
			self.fRec289[1] = self.fRec289[0];
			self.fVec54[1] = self.fVec54[0];
			self.fRec288[1] = self.fRec288[0];
			self.fRec286[1] = self.fRec286[0];
			self.fRec291[1] = self.fRec291[0];
			self.fVec56[1] = self.fVec56[0];
			self.fRec290[1] = self.fRec290[0];
			self.fRec287[1] = self.fRec287[0];
			self.fRec295[1] = self.fRec295[0];
			self.fVec58[1] = self.fVec58[0];
			self.fRec294[1] = self.fRec294[0];
			self.fRec292[1] = self.fRec292[0];
			self.fRec297[1] = self.fRec297[0];
			self.fVec60[1] = self.fVec60[0];
			self.fRec296[1] = self.fRec296[0];
			self.fRec293[1] = self.fRec293[0];
			self.fRec298[1] = self.fRec298[0];
			self.fRec299[1] = self.fRec299[0];
			self.fVec63[1] = self.fVec63[0];
			self.fRec266[1] = self.fRec266[0];
			self.fRec303[1] = self.fRec303[0];
			self.fRec305[1] = self.fRec305[0];
			self.fVec66[1] = self.fVec66[0];
			self.fRec304[1] = self.fRec304[0];
			self.fVec68[1] = self.fVec68[0];
			self.fRec302[1] = self.fRec302[0];
			self.fRec300[1] = self.fRec300[0];
			self.fRec307[1] = self.fRec307[0];
			self.fVec70[1] = self.fVec70[0];
			self.fRec306[1] = self.fRec306[0];
			self.fRec301[1] = self.fRec301[0];
			self.fRec311[1] = self.fRec311[0];
			self.fVec72[1] = self.fVec72[0];
			self.fRec310[1] = self.fRec310[0];
			self.fRec308[1] = self.fRec308[0];
			self.fRec313[1] = self.fRec313[0];
			self.fVec74[1] = self.fVec74[0];
			self.fRec312[1] = self.fRec312[0];
			self.fRec309[1] = self.fRec309[0];
			self.fRec317[1] = self.fRec317[0];
			self.fVec76[1] = self.fVec76[0];
			self.fRec316[1] = self.fRec316[0];
			self.fRec314[1] = self.fRec314[0];
			self.fRec319[1] = self.fRec319[0];
			self.fVec78[1] = self.fVec78[0];
			self.fRec318[1] = self.fRec318[0];
			self.fRec315[1] = self.fRec315[0];
			self.fRec323[1] = self.fRec323[0];
			self.fVec80[1] = self.fVec80[0];
			self.fRec322[1] = self.fRec322[0];
			self.fRec320[1] = self.fRec320[0];
			self.fRec325[1] = self.fRec325[0];
			self.fVec82[1] = self.fVec82[0];
			self.fRec324[1] = self.fRec324[0];
			self.fRec321[1] = self.fRec321[0];
			self.fRec329[1] = self.fRec329[0];
			self.fVec84[1] = self.fVec84[0];
			self.fRec328[1] = self.fRec328[0];
			self.fRec326[1] = self.fRec326[0];
			self.fRec331[1] = self.fRec331[0];
			self.fVec86[1] = self.fVec86[0];
			self.fRec330[1] = self.fRec330[0];
			self.fRec327[1] = self.fRec327[0];
			self.fVec89[1] = self.fVec89[0];
			self.fRec264[1] = self.fRec264[0];
			self.fRec263[1] = self.fRec263[0];
			self.fRec262[2] = self.fRec262[1];
			self.fRec262[1] = self.fRec262[0];
			self.fRec261[2] = self.fRec261[1];
			self.fRec261[1] = self.fRec261[0];
			self.fVec90[1] = self.fVec90[0];
			self.fRec260[1] = self.fRec260[0];
			self.fRec259[2] = self.fRec259[1];
			self.fRec259[1] = self.fRec259[0];
			self.fRec258[2] = self.fRec258[1];
			self.fRec258[1] = self.fRec258[0];
			self.fRec334[1] = self.fRec334[0];
			self.fRec333[2] = self.fRec333[1];
			self.fRec333[1] = self.fRec333[0];
			self.fRec332[2] = self.fRec332[1];
			self.fRec332[1] = self.fRec332[0];
			self.fRec338[1] = self.fRec338[0];
			self.fRec337[2] = self.fRec337[1];
			self.fRec337[1] = self.fRec337[0];
			self.fRec336[2] = self.fRec336[1];
			self.fRec336[1] = self.fRec336[0];
			self.fRec335[2] = self.fRec335[1];
			self.fRec335[1] = self.fRec335[0];
			self.fRec257[1] = self.fRec257[0];
			self.fRec350[1] = self.fRec350[0];
			self.fVec94[1] = self.fVec94[0];
			self.fRec349[1] = self.fRec349[0];
			self.fRec348[1] = self.fRec348[0];
			self.fRec347[2] = self.fRec347[1];
			self.fRec347[1] = self.fRec347[0];
			self.fRec346[2] = self.fRec346[1];
			self.fRec346[1] = self.fRec346[0];
			self.fVec95[1] = self.fVec95[0];
			self.fRec345[1] = self.fRec345[0];
			self.fRec344[2] = self.fRec344[1];
			self.fRec344[1] = self.fRec344[0];
			self.fRec343[2] = self.fRec343[1];
			self.fRec343[1] = self.fRec343[0];
			self.fRec353[1] = self.fRec353[0];
			self.fRec352[2] = self.fRec352[1];
			self.fRec352[1] = self.fRec352[0];
			self.fRec351[2] = self.fRec351[1];
			self.fRec351[1] = self.fRec351[0];
			self.fRec357[1] = self.fRec357[0];
			self.fRec356[2] = self.fRec356[1];
			self.fRec356[1] = self.fRec356[0];
			self.fRec355[2] = self.fRec355[1];
			self.fRec355[1] = self.fRec355[0];
			self.fRec354[2] = self.fRec354[1];
			self.fRec354[1] = self.fRec354[0];
			self.fRec342[1] = self.fRec342[0];
			self.fVec98[1] = self.fVec98[0];
			self.fRec341[1] = self.fRec341[0];
			self.fRec339[1] = self.fRec339[0];
			self.fRec359[1] = self.fRec359[0];
			self.fVec100[1] = self.fVec100[0];
			self.fRec358[1] = self.fRec358[0];
			self.fRec340[1] = self.fRec340[0];
			self.fVec102[1] = self.fVec102[0];
			self.fRec362[1] = self.fRec362[0];
			self.fRec360[1] = self.fRec360[0];
			self.fVec104[1] = self.fVec104[0];
			self.fRec363[1] = self.fRec363[0];
			self.fRec361[1] = self.fRec361[0];
			self.fVec106[1] = self.fVec106[0];
			self.fRec366[1] = self.fRec366[0];
			self.fRec364[1] = self.fRec364[0];
			self.fRec368[1] = self.fRec368[0];
			self.fVec108[1] = self.fVec108[0];
			self.fRec367[1] = self.fRec367[0];
			self.fRec365[1] = self.fRec365[0];
			self.fRec372[1] = self.fRec372[0];
			self.fVec110[1] = self.fVec110[0];
			self.fRec371[1] = self.fRec371[0];
			self.fRec369[1] = self.fRec369[0];
			self.fVec112[1] = self.fVec112[0];
			self.fRec373[1] = self.fRec373[0];
			self.fRec370[1] = self.fRec370[0];
			self.fRec255[1] = self.fRec255[0];
			self.fRec256[1] = self.fRec256[0];
			self.fRec374[1] = self.fRec374[0];
		}
	}

}


}
pub use dsp::mydsp as Instrument;
