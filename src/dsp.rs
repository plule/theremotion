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
	iVec0: [i32;2],
	fSampleRate: i32,
	fConst0: F32,
	fConst1: F32,
	fConst2: F32,
	fHslider0: F32,
	fRec0: [F32;2],
	fHslider1: F32,
	fRec2: [F32;2],
	fHslider2: F32,
	fConst3: F32,
	fRec1: [F32;2],
	fConst4: F32,
	fRec7: [F32;2],
	fConst5: F32,
	fRec5: [F32;2],
	fHslider3: F32,
	fRec8: [F32;2],
	fRec4: [F32;3],
	fRec3: [F32;3],
	fHslider4: F32,
	fRec9: [F32;2],
	fRec14: [F32;2],
	fRec12: [F32;2],
	fHslider5: F32,
	fRec15: [F32;2],
	fRec11: [F32;3],
	fRec10: [F32;3],
	fHslider6: F32,
	fRec16: [F32;2],
	fRec21: [F32;2],
	fRec19: [F32;2],
	fHslider7: F32,
	fRec22: [F32;2],
	fRec18: [F32;3],
	fRec17: [F32;3],
	fHslider8: F32,
	fRec23: [F32;2],
	fRec28: [F32;2],
	fRec26: [F32;2],
	fHslider9: F32,
	fRec29: [F32;2],
	fRec25: [F32;3],
	fRec24: [F32;3],
	fHslider10: F32,
	fRec30: [F32;2],
	fHslider11: F32,
	fRec31: [F32;2],
	fHslider12: F32,
	fRec32: [F32;2],
	fRec34: [F32;2],
	fVec1: [F32;2],
	IOTA0: i32,
	fVec2: [F32;4096],
	fConst6: F32,
	fConst7: F32,
	fRec33: [F32;2],
	fRec36: [F32;2],
	fVec3: [F32;2],
	fVec4: [F32;4096],
	fRec35: [F32;2],
	fRec38: [F32;2],
	fVec5: [F32;2],
	fVec6: [F32;4096],
	fRec37: [F32;2],
	fRec40: [F32;2],
	fVec7: [F32;2],
	fVec8: [F32;4096],
	fRec39: [F32;2],
	fHslider13: F32,
	fRec41: [F32;2],
	fHslider14: F32,
	fRec42: [F32;2],
	fConst8: F32,
	fHslider15: F32,
	fRec43: [F32;2],
	fHslider16: F32,
	fRec45: [F32;2],
	fHslider17: F32,
	fRec46: [F32;2],
	fConst9: F32,
	fConst10: F32,
	fConst11: F32,
	fRec76: [F32;2],
	fRec80: [F32;2],
	fConst12: F32,
	fConst13: F32,
	fRec85: [F32;2],
	iVec9: [i32;2],
	iConst14: i32,
	iRec86: [i32;2],
	fConst15: F32,
	fRec83: [F32;2],
	fRec82: [F32;2],
	fRec87: [F32;4],
	fRec88: [F32;2048],
	fVec10: [F32;2],
	fConst16: F32,
	fConst17: F32,
	fButton0: F32,
	fVec11: [F32;2],
	iRec89: [i32;2],
	iRec91: [i32;2],
	fRec90: [F32;3],
	fVec12: [F32;3],
	fRec81: [F32;2048],
	fRec72: [F32;2],
	fRec68: [F32;2],
	fRec64: [F32;2048],
	fRec66: [F32;2],
	fHslider18: F32,
	fRec62: [F32;4],
	fRec57: [F32;2],
	fRec53: [F32;2048],
	fRec51: [F32;2],
	fConst18: F32,
	fRec50: [F32;2],
	fRec49: [F32;2],
	fRec47: [F32;2],
	fRec44: [F32;2],
	fHslider19: F32,
	fRec93: [F32;2],
	fRec123: [F32;2],
	fRec127: [F32;2],
	fRec132: [F32;2],
	iVec13: [i32;2],
	iRec133: [i32;2],
	fRec130: [F32;2],
	fRec129: [F32;2],
	fRec134: [F32;4],
	fRec135: [F32;2048],
	fVec14: [F32;2],
	fButton1: F32,
	fVec15: [F32;2],
	iRec136: [i32;2],
	fRec137: [F32;3],
	fVec16: [F32;3],
	fRec128: [F32;2048],
	fRec119: [F32;2],
	fRec115: [F32;2],
	fRec111: [F32;2048],
	fRec113: [F32;2],
	fRec109: [F32;4],
	fRec104: [F32;2],
	fRec100: [F32;2048],
	fRec98: [F32;2],
	fRec97: [F32;2],
	fRec96: [F32;2],
	fRec94: [F32;2],
	fRec92: [F32;2],
	fHslider20: F32,
	fRec139: [F32;2],
	fRec169: [F32;2],
	fRec173: [F32;2],
	fRec178: [F32;2],
	iVec17: [i32;2],
	iRec179: [i32;2],
	fRec176: [F32;2],
	fRec175: [F32;2],
	fRec180: [F32;4],
	fRec181: [F32;2048],
	fVec18: [F32;2],
	fButton2: F32,
	fVec19: [F32;2],
	iRec182: [i32;2],
	fRec183: [F32;3],
	fVec20: [F32;3],
	fRec174: [F32;2048],
	fRec165: [F32;2],
	fRec161: [F32;2],
	fRec157: [F32;2048],
	fRec159: [F32;2],
	fRec155: [F32;4],
	fRec150: [F32;2],
	fRec146: [F32;2048],
	fRec144: [F32;2],
	fRec143: [F32;2],
	fRec142: [F32;2],
	fRec140: [F32;2],
	fRec138: [F32;2],
	fHslider21: F32,
	fRec185: [F32;2],
	fRec215: [F32;2],
	fRec219: [F32;2],
	fRec224: [F32;2],
	iVec21: [i32;2],
	iRec225: [i32;2],
	fRec222: [F32;2],
	fRec221: [F32;2],
	fRec226: [F32;4],
	fRec227: [F32;2048],
	fVec22: [F32;2],
	fButton3: F32,
	fVec23: [F32;2],
	iRec228: [i32;2],
	fRec229: [F32;3],
	fVec24: [F32;3],
	fRec220: [F32;2048],
	fRec211: [F32;2],
	fRec207: [F32;2],
	fRec203: [F32;2048],
	fRec205: [F32;2],
	fRec201: [F32;4],
	fRec196: [F32;2],
	fRec192: [F32;2048],
	fRec190: [F32;2],
	fRec189: [F32;2],
	fRec188: [F32;2],
	fRec186: [F32;2],
	fRec184: [F32;2],
	fHslider22: F32,
	fRec230: [F32;2],
	fConst19: F32,
	fHslider23: F32,
	fRec232: [F32;2],
	fHslider24: F32,
	fRec231: [F32;2097152],
	fHslider25: F32,
	fConst22: F32,
	fConst23: F32,
	fConst25: F32,
	fConst26: F32,
	fConst27: F32,
	fConst28: F32,
	fConst29: F32,
	fConst32: F32,
	fConst33: F32,
	fConst34: F32,
	fConst35: F32,
	fConst36: F32,
	fConst37: F32,
	fConst38: F32,
	fConst39: F32,
	fHslider26: F32,
	fRec243: [F32;2],
	fRec245: [F32;2],
	fRec249: [F32;2],
	fVec25: [F32;16384],
	fVec26: [F32;2],
	fRec248: [F32;2],
	fRec246: [F32;2],
	fRec251: [F32;2],
	fVec27: [F32;16384],
	fVec28: [F32;2],
	fRec250: [F32;2],
	fRec247: [F32;2],
	fRec255: [F32;2],
	fVec29: [F32;16384],
	fVec30: [F32;2],
	fRec254: [F32;2],
	fRec252: [F32;2],
	fRec257: [F32;2],
	fVec31: [F32;16384],
	fVec32: [F32;2],
	fRec256: [F32;2],
	fRec253: [F32;2],
	fRec261: [F32;2],
	fVec33: [F32;16384],
	fVec34: [F32;2],
	fRec260: [F32;2],
	fRec258: [F32;2],
	fRec263: [F32;2],
	fVec35: [F32;16384],
	fVec36: [F32;2],
	fRec262: [F32;2],
	fRec259: [F32;2],
	fRec267: [F32;2],
	fVec37: [F32;16384],
	fVec38: [F32;2],
	fRec266: [F32;2],
	fRec264: [F32;2],
	fRec269: [F32;2],
	fVec39: [F32;16384],
	fVec40: [F32;2],
	fRec268: [F32;2],
	fRec265: [F32;2],
	fRec273: [F32;2],
	fVec41: [F32;16384],
	fVec42: [F32;2],
	fRec272: [F32;2],
	fRec270: [F32;2],
	fRec275: [F32;2],
	fVec43: [F32;16384],
	fVec44: [F32;2],
	fRec274: [F32;2],
	fRec271: [F32;2],
	fVec45: [F32;1024],
	fHslider27: F32,
	fConst40: F32,
	fRec276: [F32;2],
	fRec277: [F32;2],
	fHslider28: F32,
	fVec46: [F32;16384],
	fVec47: [F32;2],
	fRec244: [F32;2],
	fRec281: [F32;2],
	fRec283: [F32;2],
	fVec48: [F32;1024],
	fVec49: [F32;16384],
	fVec50: [F32;2],
	fRec282: [F32;2],
	fVec51: [F32;16384],
	fVec52: [F32;2],
	fRec280: [F32;2],
	fRec278: [F32;2],
	fRec285: [F32;2],
	fVec53: [F32;16384],
	fVec54: [F32;2],
	fRec284: [F32;2],
	fRec279: [F32;2],
	fRec289: [F32;2],
	fVec55: [F32;16384],
	fVec56: [F32;2],
	fRec288: [F32;2],
	fRec286: [F32;2],
	fRec291: [F32;2],
	fVec57: [F32;16384],
	fVec58: [F32;2],
	fRec290: [F32;2],
	fRec287: [F32;2],
	fRec295: [F32;2],
	fVec59: [F32;16384],
	fVec60: [F32;2],
	fRec294: [F32;2],
	fRec292: [F32;2],
	fRec297: [F32;2],
	fVec61: [F32;16384],
	fVec62: [F32;2],
	fRec296: [F32;2],
	fRec293: [F32;2],
	fRec301: [F32;2],
	fVec63: [F32;16384],
	fVec64: [F32;2],
	fRec300: [F32;2],
	fRec298: [F32;2],
	fRec303: [F32;2],
	fVec65: [F32;16384],
	fVec66: [F32;2],
	fRec302: [F32;2],
	fRec299: [F32;2],
	fRec307: [F32;2],
	fVec67: [F32;16384],
	fVec68: [F32;2],
	fRec306: [F32;2],
	fRec304: [F32;2],
	fRec309: [F32;2],
	fVec69: [F32;16384],
	fVec70: [F32;2],
	fRec308: [F32;2],
	fRec305: [F32;2],
	fVec71: [F32;16384],
	fVec72: [F32;16384],
	fVec73: [F32;2],
	fRec242: [F32;2],
	fConst42: F32,
	fRec241: [F32;2],
	fRec240: [F32;3],
	fRec239: [F32;3],
	fVec74: [F32;2],
	fConst44: F32,
	fRec238: [F32;2],
	fRec237: [F32;3],
	fRec236: [F32;3],
	fConst45: F32,
	fConst46: F32,
	fConst47: F32,
	fRec312: [F32;2],
	fRec311: [F32;3],
	fConst48: F32,
	fRec310: [F32;3],
	fConst49: F32,
	fConst50: F32,
	fConst51: F32,
	fRec316: [F32;2],
	fRec315: [F32;3],
	fConst52: F32,
	fRec314: [F32;3],
	fRec313: [F32;3],
	fHslider29: F32,
	fVec75: [F32;1024],
	fHslider30: F32,
	fRec235: [F32;2],
	fHslider31: F32,
	fRec328: [F32;2],
	fVec76: [F32;16384],
	fVec77: [F32;16384],
	fVec78: [F32;2],
	fRec327: [F32;2],
	fRec326: [F32;2],
	fRec325: [F32;3],
	fRec324: [F32;3],
	fVec79: [F32;2],
	fRec323: [F32;2],
	fRec322: [F32;3],
	fRec321: [F32;3],
	fRec331: [F32;2],
	fRec330: [F32;3],
	fRec329: [F32;3],
	fRec335: [F32;2],
	fRec334: [F32;3],
	fRec333: [F32;3],
	fRec332: [F32;3],
	fVec80: [F32;1024],
	fRec320: [F32;2],
	fVec81: [F32;16384],
	fVec82: [F32;2],
	fRec319: [F32;2],
	fRec317: [F32;2],
	fRec337: [F32;2],
	fVec83: [F32;16384],
	fVec84: [F32;2],
	fRec336: [F32;2],
	fRec318: [F32;2],
	fVec85: [F32;16384],
	fVec86: [F32;2],
	fRec340: [F32;2],
	fRec338: [F32;2],
	fVec87: [F32;16384],
	fVec88: [F32;2],
	fRec341: [F32;2],
	fRec339: [F32;2],
	fVec89: [F32;16384],
	fVec90: [F32;2],
	fRec344: [F32;2],
	fRec342: [F32;2],
	fRec346: [F32;2],
	fVec91: [F32;16384],
	fVec92: [F32;2],
	fRec345: [F32;2],
	fRec343: [F32;2],
	fRec350: [F32;2],
	fVec93: [F32;16384],
	fVec94: [F32;2],
	fRec349: [F32;2],
	fRec347: [F32;2],
	fVec95: [F32;16384],
	fVec96: [F32;2],
	fRec351: [F32;2],
	fRec348: [F32;2],
	fRec233: [F32;2],
	fRec234: [F32;2],
	fHslider32: F32,
	fRec352: [F32;2],
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
			fRec2: [0.0;2],
			fHslider2: 0.0,
			fConst3: 0.0,
			fRec1: [0.0;2],
			fConst4: 0.0,
			fRec7: [0.0;2],
			fConst5: 0.0,
			fRec5: [0.0;2],
			fHslider3: 0.0,
			fRec8: [0.0;2],
			fRec4: [0.0;3],
			fRec3: [0.0;3],
			fHslider4: 0.0,
			fRec9: [0.0;2],
			fRec14: [0.0;2],
			fRec12: [0.0;2],
			fHslider5: 0.0,
			fRec15: [0.0;2],
			fRec11: [0.0;3],
			fRec10: [0.0;3],
			fHslider6: 0.0,
			fRec16: [0.0;2],
			fRec21: [0.0;2],
			fRec19: [0.0;2],
			fHslider7: 0.0,
			fRec22: [0.0;2],
			fRec18: [0.0;3],
			fRec17: [0.0;3],
			fHslider8: 0.0,
			fRec23: [0.0;2],
			fRec28: [0.0;2],
			fRec26: [0.0;2],
			fHslider9: 0.0,
			fRec29: [0.0;2],
			fRec25: [0.0;3],
			fRec24: [0.0;3],
			fHslider10: 0.0,
			fRec30: [0.0;2],
			fHslider11: 0.0,
			fRec31: [0.0;2],
			fHslider12: 0.0,
			fRec32: [0.0;2],
			fRec34: [0.0;2],
			fVec1: [0.0;2],
			IOTA0: 0,
			fVec2: [0.0;4096],
			fConst6: 0.0,
			fConst7: 0.0,
			fRec33: [0.0;2],
			fRec36: [0.0;2],
			fVec3: [0.0;2],
			fVec4: [0.0;4096],
			fRec35: [0.0;2],
			fRec38: [0.0;2],
			fVec5: [0.0;2],
			fVec6: [0.0;4096],
			fRec37: [0.0;2],
			fRec40: [0.0;2],
			fVec7: [0.0;2],
			fVec8: [0.0;4096],
			fRec39: [0.0;2],
			fHslider13: 0.0,
			fRec41: [0.0;2],
			fHslider14: 0.0,
			fRec42: [0.0;2],
			fConst8: 0.0,
			fHslider15: 0.0,
			fRec43: [0.0;2],
			fHslider16: 0.0,
			fRec45: [0.0;2],
			fHslider17: 0.0,
			fRec46: [0.0;2],
			fConst9: 0.0,
			fConst10: 0.0,
			fConst11: 0.0,
			fRec76: [0.0;2],
			fRec80: [0.0;2],
			fConst12: 0.0,
			fConst13: 0.0,
			fRec85: [0.0;2],
			iVec9: [0;2],
			iConst14: 0,
			iRec86: [0;2],
			fConst15: 0.0,
			fRec83: [0.0;2],
			fRec82: [0.0;2],
			fRec87: [0.0;4],
			fRec88: [0.0;2048],
			fVec10: [0.0;2],
			fConst16: 0.0,
			fConst17: 0.0,
			fButton0: 0.0,
			fVec11: [0.0;2],
			iRec89: [0;2],
			iRec91: [0;2],
			fRec90: [0.0;3],
			fVec12: [0.0;3],
			fRec81: [0.0;2048],
			fRec72: [0.0;2],
			fRec68: [0.0;2],
			fRec64: [0.0;2048],
			fRec66: [0.0;2],
			fHslider18: 0.0,
			fRec62: [0.0;4],
			fRec57: [0.0;2],
			fRec53: [0.0;2048],
			fRec51: [0.0;2],
			fConst18: 0.0,
			fRec50: [0.0;2],
			fRec49: [0.0;2],
			fRec47: [0.0;2],
			fRec44: [0.0;2],
			fHslider19: 0.0,
			fRec93: [0.0;2],
			fRec123: [0.0;2],
			fRec127: [0.0;2],
			fRec132: [0.0;2],
			iVec13: [0;2],
			iRec133: [0;2],
			fRec130: [0.0;2],
			fRec129: [0.0;2],
			fRec134: [0.0;4],
			fRec135: [0.0;2048],
			fVec14: [0.0;2],
			fButton1: 0.0,
			fVec15: [0.0;2],
			iRec136: [0;2],
			fRec137: [0.0;3],
			fVec16: [0.0;3],
			fRec128: [0.0;2048],
			fRec119: [0.0;2],
			fRec115: [0.0;2],
			fRec111: [0.0;2048],
			fRec113: [0.0;2],
			fRec109: [0.0;4],
			fRec104: [0.0;2],
			fRec100: [0.0;2048],
			fRec98: [0.0;2],
			fRec97: [0.0;2],
			fRec96: [0.0;2],
			fRec94: [0.0;2],
			fRec92: [0.0;2],
			fHslider20: 0.0,
			fRec139: [0.0;2],
			fRec169: [0.0;2],
			fRec173: [0.0;2],
			fRec178: [0.0;2],
			iVec17: [0;2],
			iRec179: [0;2],
			fRec176: [0.0;2],
			fRec175: [0.0;2],
			fRec180: [0.0;4],
			fRec181: [0.0;2048],
			fVec18: [0.0;2],
			fButton2: 0.0,
			fVec19: [0.0;2],
			iRec182: [0;2],
			fRec183: [0.0;3],
			fVec20: [0.0;3],
			fRec174: [0.0;2048],
			fRec165: [0.0;2],
			fRec161: [0.0;2],
			fRec157: [0.0;2048],
			fRec159: [0.0;2],
			fRec155: [0.0;4],
			fRec150: [0.0;2],
			fRec146: [0.0;2048],
			fRec144: [0.0;2],
			fRec143: [0.0;2],
			fRec142: [0.0;2],
			fRec140: [0.0;2],
			fRec138: [0.0;2],
			fHslider21: 0.0,
			fRec185: [0.0;2],
			fRec215: [0.0;2],
			fRec219: [0.0;2],
			fRec224: [0.0;2],
			iVec21: [0;2],
			iRec225: [0;2],
			fRec222: [0.0;2],
			fRec221: [0.0;2],
			fRec226: [0.0;4],
			fRec227: [0.0;2048],
			fVec22: [0.0;2],
			fButton3: 0.0,
			fVec23: [0.0;2],
			iRec228: [0;2],
			fRec229: [0.0;3],
			fVec24: [0.0;3],
			fRec220: [0.0;2048],
			fRec211: [0.0;2],
			fRec207: [0.0;2],
			fRec203: [0.0;2048],
			fRec205: [0.0;2],
			fRec201: [0.0;4],
			fRec196: [0.0;2],
			fRec192: [0.0;2048],
			fRec190: [0.0;2],
			fRec189: [0.0;2],
			fRec188: [0.0;2],
			fRec186: [0.0;2],
			fRec184: [0.0;2],
			fHslider22: 0.0,
			fRec230: [0.0;2],
			fConst19: 0.0,
			fHslider23: 0.0,
			fRec232: [0.0;2],
			fHslider24: 0.0,
			fRec231: [0.0;2097152],
			fHslider25: 0.0,
			fConst22: 0.0,
			fConst23: 0.0,
			fConst25: 0.0,
			fConst26: 0.0,
			fConst27: 0.0,
			fConst28: 0.0,
			fConst29: 0.0,
			fConst32: 0.0,
			fConst33: 0.0,
			fConst34: 0.0,
			fConst35: 0.0,
			fConst36: 0.0,
			fConst37: 0.0,
			fConst38: 0.0,
			fConst39: 0.0,
			fHslider26: 0.0,
			fRec243: [0.0;2],
			fRec245: [0.0;2],
			fRec249: [0.0;2],
			fVec25: [0.0;16384],
			fVec26: [0.0;2],
			fRec248: [0.0;2],
			fRec246: [0.0;2],
			fRec251: [0.0;2],
			fVec27: [0.0;16384],
			fVec28: [0.0;2],
			fRec250: [0.0;2],
			fRec247: [0.0;2],
			fRec255: [0.0;2],
			fVec29: [0.0;16384],
			fVec30: [0.0;2],
			fRec254: [0.0;2],
			fRec252: [0.0;2],
			fRec257: [0.0;2],
			fVec31: [0.0;16384],
			fVec32: [0.0;2],
			fRec256: [0.0;2],
			fRec253: [0.0;2],
			fRec261: [0.0;2],
			fVec33: [0.0;16384],
			fVec34: [0.0;2],
			fRec260: [0.0;2],
			fRec258: [0.0;2],
			fRec263: [0.0;2],
			fVec35: [0.0;16384],
			fVec36: [0.0;2],
			fRec262: [0.0;2],
			fRec259: [0.0;2],
			fRec267: [0.0;2],
			fVec37: [0.0;16384],
			fVec38: [0.0;2],
			fRec266: [0.0;2],
			fRec264: [0.0;2],
			fRec269: [0.0;2],
			fVec39: [0.0;16384],
			fVec40: [0.0;2],
			fRec268: [0.0;2],
			fRec265: [0.0;2],
			fRec273: [0.0;2],
			fVec41: [0.0;16384],
			fVec42: [0.0;2],
			fRec272: [0.0;2],
			fRec270: [0.0;2],
			fRec275: [0.0;2],
			fVec43: [0.0;16384],
			fVec44: [0.0;2],
			fRec274: [0.0;2],
			fRec271: [0.0;2],
			fVec45: [0.0;1024],
			fHslider27: 0.0,
			fConst40: 0.0,
			fRec276: [0.0;2],
			fRec277: [0.0;2],
			fHslider28: 0.0,
			fVec46: [0.0;16384],
			fVec47: [0.0;2],
			fRec244: [0.0;2],
			fRec281: [0.0;2],
			fRec283: [0.0;2],
			fVec48: [0.0;1024],
			fVec49: [0.0;16384],
			fVec50: [0.0;2],
			fRec282: [0.0;2],
			fVec51: [0.0;16384],
			fVec52: [0.0;2],
			fRec280: [0.0;2],
			fRec278: [0.0;2],
			fRec285: [0.0;2],
			fVec53: [0.0;16384],
			fVec54: [0.0;2],
			fRec284: [0.0;2],
			fRec279: [0.0;2],
			fRec289: [0.0;2],
			fVec55: [0.0;16384],
			fVec56: [0.0;2],
			fRec288: [0.0;2],
			fRec286: [0.0;2],
			fRec291: [0.0;2],
			fVec57: [0.0;16384],
			fVec58: [0.0;2],
			fRec290: [0.0;2],
			fRec287: [0.0;2],
			fRec295: [0.0;2],
			fVec59: [0.0;16384],
			fVec60: [0.0;2],
			fRec294: [0.0;2],
			fRec292: [0.0;2],
			fRec297: [0.0;2],
			fVec61: [0.0;16384],
			fVec62: [0.0;2],
			fRec296: [0.0;2],
			fRec293: [0.0;2],
			fRec301: [0.0;2],
			fVec63: [0.0;16384],
			fVec64: [0.0;2],
			fRec300: [0.0;2],
			fRec298: [0.0;2],
			fRec303: [0.0;2],
			fVec65: [0.0;16384],
			fVec66: [0.0;2],
			fRec302: [0.0;2],
			fRec299: [0.0;2],
			fRec307: [0.0;2],
			fVec67: [0.0;16384],
			fVec68: [0.0;2],
			fRec306: [0.0;2],
			fRec304: [0.0;2],
			fRec309: [0.0;2],
			fVec69: [0.0;16384],
			fVec70: [0.0;2],
			fRec308: [0.0;2],
			fRec305: [0.0;2],
			fVec71: [0.0;16384],
			fVec72: [0.0;16384],
			fVec73: [0.0;2],
			fRec242: [0.0;2],
			fConst42: 0.0,
			fRec241: [0.0;2],
			fRec240: [0.0;3],
			fRec239: [0.0;3],
			fVec74: [0.0;2],
			fConst44: 0.0,
			fRec238: [0.0;2],
			fRec237: [0.0;3],
			fRec236: [0.0;3],
			fConst45: 0.0,
			fConst46: 0.0,
			fConst47: 0.0,
			fRec312: [0.0;2],
			fRec311: [0.0;3],
			fConst48: 0.0,
			fRec310: [0.0;3],
			fConst49: 0.0,
			fConst50: 0.0,
			fConst51: 0.0,
			fRec316: [0.0;2],
			fRec315: [0.0;3],
			fConst52: 0.0,
			fRec314: [0.0;3],
			fRec313: [0.0;3],
			fHslider29: 0.0,
			fVec75: [0.0;1024],
			fHslider30: 0.0,
			fRec235: [0.0;2],
			fHslider31: 0.0,
			fRec328: [0.0;2],
			fVec76: [0.0;16384],
			fVec77: [0.0;16384],
			fVec78: [0.0;2],
			fRec327: [0.0;2],
			fRec326: [0.0;2],
			fRec325: [0.0;3],
			fRec324: [0.0;3],
			fVec79: [0.0;2],
			fRec323: [0.0;2],
			fRec322: [0.0;3],
			fRec321: [0.0;3],
			fRec331: [0.0;2],
			fRec330: [0.0;3],
			fRec329: [0.0;3],
			fRec335: [0.0;2],
			fRec334: [0.0;3],
			fRec333: [0.0;3],
			fRec332: [0.0;3],
			fVec80: [0.0;1024],
			fRec320: [0.0;2],
			fVec81: [0.0;16384],
			fVec82: [0.0;2],
			fRec319: [0.0;2],
			fRec317: [0.0;2],
			fRec337: [0.0;2],
			fVec83: [0.0;16384],
			fVec84: [0.0;2],
			fRec336: [0.0;2],
			fRec318: [0.0;2],
			fVec85: [0.0;16384],
			fVec86: [0.0;2],
			fRec340: [0.0;2],
			fRec338: [0.0;2],
			fVec87: [0.0;16384],
			fVec88: [0.0;2],
			fRec341: [0.0;2],
			fRec339: [0.0;2],
			fVec89: [0.0;16384],
			fVec90: [0.0;2],
			fRec344: [0.0;2],
			fRec342: [0.0;2],
			fRec346: [0.0;2],
			fVec91: [0.0;16384],
			fVec92: [0.0;2],
			fRec345: [0.0;2],
			fRec343: [0.0;2],
			fRec350: [0.0;2],
			fVec93: [0.0;16384],
			fVec94: [0.0;2],
			fRec349: [0.0;2],
			fRec347: [0.0;2],
			fVec95: [0.0;16384],
			fVec96: [0.0;2],
			fRec351: [0.0;2],
			fRec348: [0.0;2],
			fRec233: [0.0;2],
			fRec234: [0.0;2],
			fHslider32: 0.0,
			fRec352: [0.0;2],
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
		self.fHslider0 = 0.0;
		self.fHslider1 = 0.0;
		self.fHslider2 = 6e+01;
		self.fHslider3 = 0.0;
		self.fHslider4 = 6e+01;
		self.fHslider5 = 0.0;
		self.fHslider6 = 6e+01;
		self.fHslider7 = 0.0;
		self.fHslider8 = 6e+01;
		self.fHslider9 = 0.0;
		self.fHslider10 = 1.0;
		self.fHslider11 = 0.0;
		self.fHslider12 = 6e+01;
		self.fHslider13 = 1.0;
		self.fHslider14 = 0.0;
		self.fHslider15 = 1.0;
		self.fHslider16 = 8e+01;
		self.fHslider17 = 0.0;
		self.fButton0 = 0.0;
		self.fHslider18 = 1.0;
		self.fHslider19 = 8e+01;
		self.fButton1 = 0.0;
		self.fHslider20 = 8e+01;
		self.fButton2 = 0.0;
		self.fHslider21 = 8e+01;
		self.fButton3 = 0.0;
		self.fHslider22 = 1.0;
		self.fHslider23 = 0.3;
		self.fHslider24 = 0.3;
		self.fHslider25 = 0.11;
		self.fHslider26 = 5.0;
		self.fHslider27 = 0.6;
		self.fHslider28 = 0.98;
		self.fHslider29 = 3.5;
		self.fHslider30 = 0.88;
		self.fHslider31 = 0.75;
		self.fHslider32 = 1.0;
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
			self.fRec1[(l3) as usize] = 0.0;
		}
		for l4 in 0..2 {
			self.fRec7[(l4) as usize] = 0.0;
		}
		for l5 in 0..2 {
			self.fRec5[(l5) as usize] = 0.0;
		}
		for l6 in 0..2 {
			self.fRec8[(l6) as usize] = 0.0;
		}
		for l7 in 0..3 {
			self.fRec4[(l7) as usize] = 0.0;
		}
		for l8 in 0..3 {
			self.fRec3[(l8) as usize] = 0.0;
		}
		for l9 in 0..2 {
			self.fRec9[(l9) as usize] = 0.0;
		}
		for l10 in 0..2 {
			self.fRec14[(l10) as usize] = 0.0;
		}
		for l11 in 0..2 {
			self.fRec12[(l11) as usize] = 0.0;
		}
		for l12 in 0..2 {
			self.fRec15[(l12) as usize] = 0.0;
		}
		for l13 in 0..3 {
			self.fRec11[(l13) as usize] = 0.0;
		}
		for l14 in 0..3 {
			self.fRec10[(l14) as usize] = 0.0;
		}
		for l15 in 0..2 {
			self.fRec16[(l15) as usize] = 0.0;
		}
		for l16 in 0..2 {
			self.fRec21[(l16) as usize] = 0.0;
		}
		for l17 in 0..2 {
			self.fRec19[(l17) as usize] = 0.0;
		}
		for l18 in 0..2 {
			self.fRec22[(l18) as usize] = 0.0;
		}
		for l19 in 0..3 {
			self.fRec18[(l19) as usize] = 0.0;
		}
		for l20 in 0..3 {
			self.fRec17[(l20) as usize] = 0.0;
		}
		for l21 in 0..2 {
			self.fRec23[(l21) as usize] = 0.0;
		}
		for l22 in 0..2 {
			self.fRec28[(l22) as usize] = 0.0;
		}
		for l23 in 0..2 {
			self.fRec26[(l23) as usize] = 0.0;
		}
		for l24 in 0..2 {
			self.fRec29[(l24) as usize] = 0.0;
		}
		for l25 in 0..3 {
			self.fRec25[(l25) as usize] = 0.0;
		}
		for l26 in 0..3 {
			self.fRec24[(l26) as usize] = 0.0;
		}
		for l27 in 0..2 {
			self.fRec30[(l27) as usize] = 0.0;
		}
		for l28 in 0..2 {
			self.fRec31[(l28) as usize] = 0.0;
		}
		for l29 in 0..2 {
			self.fRec32[(l29) as usize] = 0.0;
		}
		for l30 in 0..2 {
			self.fRec34[(l30) as usize] = 0.0;
		}
		for l31 in 0..2 {
			self.fVec1[(l31) as usize] = 0.0;
		}
		self.IOTA0 = 0;
		for l32 in 0..4096 {
			self.fVec2[(l32) as usize] = 0.0;
		}
		for l33 in 0..2 {
			self.fRec33[(l33) as usize] = 0.0;
		}
		for l34 in 0..2 {
			self.fRec36[(l34) as usize] = 0.0;
		}
		for l35 in 0..2 {
			self.fVec3[(l35) as usize] = 0.0;
		}
		for l36 in 0..4096 {
			self.fVec4[(l36) as usize] = 0.0;
		}
		for l37 in 0..2 {
			self.fRec35[(l37) as usize] = 0.0;
		}
		for l38 in 0..2 {
			self.fRec38[(l38) as usize] = 0.0;
		}
		for l39 in 0..2 {
			self.fVec5[(l39) as usize] = 0.0;
		}
		for l40 in 0..4096 {
			self.fVec6[(l40) as usize] = 0.0;
		}
		for l41 in 0..2 {
			self.fRec37[(l41) as usize] = 0.0;
		}
		for l42 in 0..2 {
			self.fRec40[(l42) as usize] = 0.0;
		}
		for l43 in 0..2 {
			self.fVec7[(l43) as usize] = 0.0;
		}
		for l44 in 0..4096 {
			self.fVec8[(l44) as usize] = 0.0;
		}
		for l45 in 0..2 {
			self.fRec39[(l45) as usize] = 0.0;
		}
		for l46 in 0..2 {
			self.fRec41[(l46) as usize] = 0.0;
		}
		for l47 in 0..2 {
			self.fRec42[(l47) as usize] = 0.0;
		}
		for l48 in 0..2 {
			self.fRec43[(l48) as usize] = 0.0;
		}
		for l49 in 0..2 {
			self.fRec45[(l49) as usize] = 0.0;
		}
		for l50 in 0..2 {
			self.fRec46[(l50) as usize] = 0.0;
		}
		for l51 in 0..2 {
			self.fRec76[(l51) as usize] = 0.0;
		}
		for l52 in 0..2 {
			self.fRec80[(l52) as usize] = 0.0;
		}
		for l53 in 0..2 {
			self.fRec85[(l53) as usize] = 0.0;
		}
		for l54 in 0..2 {
			self.iVec9[(l54) as usize] = 0;
		}
		for l55 in 0..2 {
			self.iRec86[(l55) as usize] = 0;
		}
		for l56 in 0..2 {
			self.fRec83[(l56) as usize] = 0.0;
		}
		for l57 in 0..2 {
			self.fRec82[(l57) as usize] = 0.0;
		}
		for l58 in 0..4 {
			self.fRec87[(l58) as usize] = 0.0;
		}
		for l59 in 0..2048 {
			self.fRec88[(l59) as usize] = 0.0;
		}
		for l60 in 0..2 {
			self.fVec10[(l60) as usize] = 0.0;
		}
		for l61 in 0..2 {
			self.fVec11[(l61) as usize] = 0.0;
		}
		for l62 in 0..2 {
			self.iRec89[(l62) as usize] = 0;
		}
		for l63 in 0..2 {
			self.iRec91[(l63) as usize] = 0;
		}
		for l64 in 0..3 {
			self.fRec90[(l64) as usize] = 0.0;
		}
		for l65 in 0..3 {
			self.fVec12[(l65) as usize] = 0.0;
		}
		for l66 in 0..2048 {
			self.fRec81[(l66) as usize] = 0.0;
		}
		for l67 in 0..2 {
			self.fRec72[(l67) as usize] = 0.0;
		}
		for l68 in 0..2 {
			self.fRec68[(l68) as usize] = 0.0;
		}
		for l69 in 0..2048 {
			self.fRec64[(l69) as usize] = 0.0;
		}
		for l70 in 0..2 {
			self.fRec66[(l70) as usize] = 0.0;
		}
		for l71 in 0..4 {
			self.fRec62[(l71) as usize] = 0.0;
		}
		for l72 in 0..2 {
			self.fRec57[(l72) as usize] = 0.0;
		}
		for l73 in 0..2048 {
			self.fRec53[(l73) as usize] = 0.0;
		}
		for l74 in 0..2 {
			self.fRec51[(l74) as usize] = 0.0;
		}
		for l75 in 0..2 {
			self.fRec50[(l75) as usize] = 0.0;
		}
		for l76 in 0..2 {
			self.fRec49[(l76) as usize] = 0.0;
		}
		for l77 in 0..2 {
			self.fRec47[(l77) as usize] = 0.0;
		}
		for l78 in 0..2 {
			self.fRec44[(l78) as usize] = 0.0;
		}
		for l79 in 0..2 {
			self.fRec93[(l79) as usize] = 0.0;
		}
		for l80 in 0..2 {
			self.fRec123[(l80) as usize] = 0.0;
		}
		for l81 in 0..2 {
			self.fRec127[(l81) as usize] = 0.0;
		}
		for l82 in 0..2 {
			self.fRec132[(l82) as usize] = 0.0;
		}
		for l83 in 0..2 {
			self.iVec13[(l83) as usize] = 0;
		}
		for l84 in 0..2 {
			self.iRec133[(l84) as usize] = 0;
		}
		for l85 in 0..2 {
			self.fRec130[(l85) as usize] = 0.0;
		}
		for l86 in 0..2 {
			self.fRec129[(l86) as usize] = 0.0;
		}
		for l87 in 0..4 {
			self.fRec134[(l87) as usize] = 0.0;
		}
		for l88 in 0..2048 {
			self.fRec135[(l88) as usize] = 0.0;
		}
		for l89 in 0..2 {
			self.fVec14[(l89) as usize] = 0.0;
		}
		for l90 in 0..2 {
			self.fVec15[(l90) as usize] = 0.0;
		}
		for l91 in 0..2 {
			self.iRec136[(l91) as usize] = 0;
		}
		for l92 in 0..3 {
			self.fRec137[(l92) as usize] = 0.0;
		}
		for l93 in 0..3 {
			self.fVec16[(l93) as usize] = 0.0;
		}
		for l94 in 0..2048 {
			self.fRec128[(l94) as usize] = 0.0;
		}
		for l95 in 0..2 {
			self.fRec119[(l95) as usize] = 0.0;
		}
		for l96 in 0..2 {
			self.fRec115[(l96) as usize] = 0.0;
		}
		for l97 in 0..2048 {
			self.fRec111[(l97) as usize] = 0.0;
		}
		for l98 in 0..2 {
			self.fRec113[(l98) as usize] = 0.0;
		}
		for l99 in 0..4 {
			self.fRec109[(l99) as usize] = 0.0;
		}
		for l100 in 0..2 {
			self.fRec104[(l100) as usize] = 0.0;
		}
		for l101 in 0..2048 {
			self.fRec100[(l101) as usize] = 0.0;
		}
		for l102 in 0..2 {
			self.fRec98[(l102) as usize] = 0.0;
		}
		for l103 in 0..2 {
			self.fRec97[(l103) as usize] = 0.0;
		}
		for l104 in 0..2 {
			self.fRec96[(l104) as usize] = 0.0;
		}
		for l105 in 0..2 {
			self.fRec94[(l105) as usize] = 0.0;
		}
		for l106 in 0..2 {
			self.fRec92[(l106) as usize] = 0.0;
		}
		for l107 in 0..2 {
			self.fRec139[(l107) as usize] = 0.0;
		}
		for l108 in 0..2 {
			self.fRec169[(l108) as usize] = 0.0;
		}
		for l109 in 0..2 {
			self.fRec173[(l109) as usize] = 0.0;
		}
		for l110 in 0..2 {
			self.fRec178[(l110) as usize] = 0.0;
		}
		for l111 in 0..2 {
			self.iVec17[(l111) as usize] = 0;
		}
		for l112 in 0..2 {
			self.iRec179[(l112) as usize] = 0;
		}
		for l113 in 0..2 {
			self.fRec176[(l113) as usize] = 0.0;
		}
		for l114 in 0..2 {
			self.fRec175[(l114) as usize] = 0.0;
		}
		for l115 in 0..4 {
			self.fRec180[(l115) as usize] = 0.0;
		}
		for l116 in 0..2048 {
			self.fRec181[(l116) as usize] = 0.0;
		}
		for l117 in 0..2 {
			self.fVec18[(l117) as usize] = 0.0;
		}
		for l118 in 0..2 {
			self.fVec19[(l118) as usize] = 0.0;
		}
		for l119 in 0..2 {
			self.iRec182[(l119) as usize] = 0;
		}
		for l120 in 0..3 {
			self.fRec183[(l120) as usize] = 0.0;
		}
		for l121 in 0..3 {
			self.fVec20[(l121) as usize] = 0.0;
		}
		for l122 in 0..2048 {
			self.fRec174[(l122) as usize] = 0.0;
		}
		for l123 in 0..2 {
			self.fRec165[(l123) as usize] = 0.0;
		}
		for l124 in 0..2 {
			self.fRec161[(l124) as usize] = 0.0;
		}
		for l125 in 0..2048 {
			self.fRec157[(l125) as usize] = 0.0;
		}
		for l126 in 0..2 {
			self.fRec159[(l126) as usize] = 0.0;
		}
		for l127 in 0..4 {
			self.fRec155[(l127) as usize] = 0.0;
		}
		for l128 in 0..2 {
			self.fRec150[(l128) as usize] = 0.0;
		}
		for l129 in 0..2048 {
			self.fRec146[(l129) as usize] = 0.0;
		}
		for l130 in 0..2 {
			self.fRec144[(l130) as usize] = 0.0;
		}
		for l131 in 0..2 {
			self.fRec143[(l131) as usize] = 0.0;
		}
		for l132 in 0..2 {
			self.fRec142[(l132) as usize] = 0.0;
		}
		for l133 in 0..2 {
			self.fRec140[(l133) as usize] = 0.0;
		}
		for l134 in 0..2 {
			self.fRec138[(l134) as usize] = 0.0;
		}
		for l135 in 0..2 {
			self.fRec185[(l135) as usize] = 0.0;
		}
		for l136 in 0..2 {
			self.fRec215[(l136) as usize] = 0.0;
		}
		for l137 in 0..2 {
			self.fRec219[(l137) as usize] = 0.0;
		}
		for l138 in 0..2 {
			self.fRec224[(l138) as usize] = 0.0;
		}
		for l139 in 0..2 {
			self.iVec21[(l139) as usize] = 0;
		}
		for l140 in 0..2 {
			self.iRec225[(l140) as usize] = 0;
		}
		for l141 in 0..2 {
			self.fRec222[(l141) as usize] = 0.0;
		}
		for l142 in 0..2 {
			self.fRec221[(l142) as usize] = 0.0;
		}
		for l143 in 0..4 {
			self.fRec226[(l143) as usize] = 0.0;
		}
		for l144 in 0..2048 {
			self.fRec227[(l144) as usize] = 0.0;
		}
		for l145 in 0..2 {
			self.fVec22[(l145) as usize] = 0.0;
		}
		for l146 in 0..2 {
			self.fVec23[(l146) as usize] = 0.0;
		}
		for l147 in 0..2 {
			self.iRec228[(l147) as usize] = 0;
		}
		for l148 in 0..3 {
			self.fRec229[(l148) as usize] = 0.0;
		}
		for l149 in 0..3 {
			self.fVec24[(l149) as usize] = 0.0;
		}
		for l150 in 0..2048 {
			self.fRec220[(l150) as usize] = 0.0;
		}
		for l151 in 0..2 {
			self.fRec211[(l151) as usize] = 0.0;
		}
		for l152 in 0..2 {
			self.fRec207[(l152) as usize] = 0.0;
		}
		for l153 in 0..2048 {
			self.fRec203[(l153) as usize] = 0.0;
		}
		for l154 in 0..2 {
			self.fRec205[(l154) as usize] = 0.0;
		}
		for l155 in 0..4 {
			self.fRec201[(l155) as usize] = 0.0;
		}
		for l156 in 0..2 {
			self.fRec196[(l156) as usize] = 0.0;
		}
		for l157 in 0..2048 {
			self.fRec192[(l157) as usize] = 0.0;
		}
		for l158 in 0..2 {
			self.fRec190[(l158) as usize] = 0.0;
		}
		for l159 in 0..2 {
			self.fRec189[(l159) as usize] = 0.0;
		}
		for l160 in 0..2 {
			self.fRec188[(l160) as usize] = 0.0;
		}
		for l161 in 0..2 {
			self.fRec186[(l161) as usize] = 0.0;
		}
		for l162 in 0..2 {
			self.fRec184[(l162) as usize] = 0.0;
		}
		for l163 in 0..2 {
			self.fRec230[(l163) as usize] = 0.0;
		}
		for l164 in 0..2 {
			self.fRec232[(l164) as usize] = 0.0;
		}
		for l165 in 0..2097152 {
			self.fRec231[(l165) as usize] = 0.0;
		}
		for l166 in 0..2 {
			self.fRec243[(l166) as usize] = 0.0;
		}
		for l167 in 0..2 {
			self.fRec245[(l167) as usize] = 0.0;
		}
		for l168 in 0..2 {
			self.fRec249[(l168) as usize] = 0.0;
		}
		for l169 in 0..16384 {
			self.fVec25[(l169) as usize] = 0.0;
		}
		for l170 in 0..2 {
			self.fVec26[(l170) as usize] = 0.0;
		}
		for l171 in 0..2 {
			self.fRec248[(l171) as usize] = 0.0;
		}
		for l172 in 0..2 {
			self.fRec246[(l172) as usize] = 0.0;
		}
		for l173 in 0..2 {
			self.fRec251[(l173) as usize] = 0.0;
		}
		for l174 in 0..16384 {
			self.fVec27[(l174) as usize] = 0.0;
		}
		for l175 in 0..2 {
			self.fVec28[(l175) as usize] = 0.0;
		}
		for l176 in 0..2 {
			self.fRec250[(l176) as usize] = 0.0;
		}
		for l177 in 0..2 {
			self.fRec247[(l177) as usize] = 0.0;
		}
		for l178 in 0..2 {
			self.fRec255[(l178) as usize] = 0.0;
		}
		for l179 in 0..16384 {
			self.fVec29[(l179) as usize] = 0.0;
		}
		for l180 in 0..2 {
			self.fVec30[(l180) as usize] = 0.0;
		}
		for l181 in 0..2 {
			self.fRec254[(l181) as usize] = 0.0;
		}
		for l182 in 0..2 {
			self.fRec252[(l182) as usize] = 0.0;
		}
		for l183 in 0..2 {
			self.fRec257[(l183) as usize] = 0.0;
		}
		for l184 in 0..16384 {
			self.fVec31[(l184) as usize] = 0.0;
		}
		for l185 in 0..2 {
			self.fVec32[(l185) as usize] = 0.0;
		}
		for l186 in 0..2 {
			self.fRec256[(l186) as usize] = 0.0;
		}
		for l187 in 0..2 {
			self.fRec253[(l187) as usize] = 0.0;
		}
		for l188 in 0..2 {
			self.fRec261[(l188) as usize] = 0.0;
		}
		for l189 in 0..16384 {
			self.fVec33[(l189) as usize] = 0.0;
		}
		for l190 in 0..2 {
			self.fVec34[(l190) as usize] = 0.0;
		}
		for l191 in 0..2 {
			self.fRec260[(l191) as usize] = 0.0;
		}
		for l192 in 0..2 {
			self.fRec258[(l192) as usize] = 0.0;
		}
		for l193 in 0..2 {
			self.fRec263[(l193) as usize] = 0.0;
		}
		for l194 in 0..16384 {
			self.fVec35[(l194) as usize] = 0.0;
		}
		for l195 in 0..2 {
			self.fVec36[(l195) as usize] = 0.0;
		}
		for l196 in 0..2 {
			self.fRec262[(l196) as usize] = 0.0;
		}
		for l197 in 0..2 {
			self.fRec259[(l197) as usize] = 0.0;
		}
		for l198 in 0..2 {
			self.fRec267[(l198) as usize] = 0.0;
		}
		for l199 in 0..16384 {
			self.fVec37[(l199) as usize] = 0.0;
		}
		for l200 in 0..2 {
			self.fVec38[(l200) as usize] = 0.0;
		}
		for l201 in 0..2 {
			self.fRec266[(l201) as usize] = 0.0;
		}
		for l202 in 0..2 {
			self.fRec264[(l202) as usize] = 0.0;
		}
		for l203 in 0..2 {
			self.fRec269[(l203) as usize] = 0.0;
		}
		for l204 in 0..16384 {
			self.fVec39[(l204) as usize] = 0.0;
		}
		for l205 in 0..2 {
			self.fVec40[(l205) as usize] = 0.0;
		}
		for l206 in 0..2 {
			self.fRec268[(l206) as usize] = 0.0;
		}
		for l207 in 0..2 {
			self.fRec265[(l207) as usize] = 0.0;
		}
		for l208 in 0..2 {
			self.fRec273[(l208) as usize] = 0.0;
		}
		for l209 in 0..16384 {
			self.fVec41[(l209) as usize] = 0.0;
		}
		for l210 in 0..2 {
			self.fVec42[(l210) as usize] = 0.0;
		}
		for l211 in 0..2 {
			self.fRec272[(l211) as usize] = 0.0;
		}
		for l212 in 0..2 {
			self.fRec270[(l212) as usize] = 0.0;
		}
		for l213 in 0..2 {
			self.fRec275[(l213) as usize] = 0.0;
		}
		for l214 in 0..16384 {
			self.fVec43[(l214) as usize] = 0.0;
		}
		for l215 in 0..2 {
			self.fVec44[(l215) as usize] = 0.0;
		}
		for l216 in 0..2 {
			self.fRec274[(l216) as usize] = 0.0;
		}
		for l217 in 0..2 {
			self.fRec271[(l217) as usize] = 0.0;
		}
		for l218 in 0..1024 {
			self.fVec45[(l218) as usize] = 0.0;
		}
		for l219 in 0..2 {
			self.fRec276[(l219) as usize] = 0.0;
		}
		for l220 in 0..2 {
			self.fRec277[(l220) as usize] = 0.0;
		}
		for l221 in 0..16384 {
			self.fVec46[(l221) as usize] = 0.0;
		}
		for l222 in 0..2 {
			self.fVec47[(l222) as usize] = 0.0;
		}
		for l223 in 0..2 {
			self.fRec244[(l223) as usize] = 0.0;
		}
		for l224 in 0..2 {
			self.fRec281[(l224) as usize] = 0.0;
		}
		for l225 in 0..2 {
			self.fRec283[(l225) as usize] = 0.0;
		}
		for l226 in 0..1024 {
			self.fVec48[(l226) as usize] = 0.0;
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
		for l230 in 0..16384 {
			self.fVec51[(l230) as usize] = 0.0;
		}
		for l231 in 0..2 {
			self.fVec52[(l231) as usize] = 0.0;
		}
		for l232 in 0..2 {
			self.fRec280[(l232) as usize] = 0.0;
		}
		for l233 in 0..2 {
			self.fRec278[(l233) as usize] = 0.0;
		}
		for l234 in 0..2 {
			self.fRec285[(l234) as usize] = 0.0;
		}
		for l235 in 0..16384 {
			self.fVec53[(l235) as usize] = 0.0;
		}
		for l236 in 0..2 {
			self.fVec54[(l236) as usize] = 0.0;
		}
		for l237 in 0..2 {
			self.fRec284[(l237) as usize] = 0.0;
		}
		for l238 in 0..2 {
			self.fRec279[(l238) as usize] = 0.0;
		}
		for l239 in 0..2 {
			self.fRec289[(l239) as usize] = 0.0;
		}
		for l240 in 0..16384 {
			self.fVec55[(l240) as usize] = 0.0;
		}
		for l241 in 0..2 {
			self.fVec56[(l241) as usize] = 0.0;
		}
		for l242 in 0..2 {
			self.fRec288[(l242) as usize] = 0.0;
		}
		for l243 in 0..2 {
			self.fRec286[(l243) as usize] = 0.0;
		}
		for l244 in 0..2 {
			self.fRec291[(l244) as usize] = 0.0;
		}
		for l245 in 0..16384 {
			self.fVec57[(l245) as usize] = 0.0;
		}
		for l246 in 0..2 {
			self.fVec58[(l246) as usize] = 0.0;
		}
		for l247 in 0..2 {
			self.fRec290[(l247) as usize] = 0.0;
		}
		for l248 in 0..2 {
			self.fRec287[(l248) as usize] = 0.0;
		}
		for l249 in 0..2 {
			self.fRec295[(l249) as usize] = 0.0;
		}
		for l250 in 0..16384 {
			self.fVec59[(l250) as usize] = 0.0;
		}
		for l251 in 0..2 {
			self.fVec60[(l251) as usize] = 0.0;
		}
		for l252 in 0..2 {
			self.fRec294[(l252) as usize] = 0.0;
		}
		for l253 in 0..2 {
			self.fRec292[(l253) as usize] = 0.0;
		}
		for l254 in 0..2 {
			self.fRec297[(l254) as usize] = 0.0;
		}
		for l255 in 0..16384 {
			self.fVec61[(l255) as usize] = 0.0;
		}
		for l256 in 0..2 {
			self.fVec62[(l256) as usize] = 0.0;
		}
		for l257 in 0..2 {
			self.fRec296[(l257) as usize] = 0.0;
		}
		for l258 in 0..2 {
			self.fRec293[(l258) as usize] = 0.0;
		}
		for l259 in 0..2 {
			self.fRec301[(l259) as usize] = 0.0;
		}
		for l260 in 0..16384 {
			self.fVec63[(l260) as usize] = 0.0;
		}
		for l261 in 0..2 {
			self.fVec64[(l261) as usize] = 0.0;
		}
		for l262 in 0..2 {
			self.fRec300[(l262) as usize] = 0.0;
		}
		for l263 in 0..2 {
			self.fRec298[(l263) as usize] = 0.0;
		}
		for l264 in 0..2 {
			self.fRec303[(l264) as usize] = 0.0;
		}
		for l265 in 0..16384 {
			self.fVec65[(l265) as usize] = 0.0;
		}
		for l266 in 0..2 {
			self.fVec66[(l266) as usize] = 0.0;
		}
		for l267 in 0..2 {
			self.fRec302[(l267) as usize] = 0.0;
		}
		for l268 in 0..2 {
			self.fRec299[(l268) as usize] = 0.0;
		}
		for l269 in 0..2 {
			self.fRec307[(l269) as usize] = 0.0;
		}
		for l270 in 0..16384 {
			self.fVec67[(l270) as usize] = 0.0;
		}
		for l271 in 0..2 {
			self.fVec68[(l271) as usize] = 0.0;
		}
		for l272 in 0..2 {
			self.fRec306[(l272) as usize] = 0.0;
		}
		for l273 in 0..2 {
			self.fRec304[(l273) as usize] = 0.0;
		}
		for l274 in 0..2 {
			self.fRec309[(l274) as usize] = 0.0;
		}
		for l275 in 0..16384 {
			self.fVec69[(l275) as usize] = 0.0;
		}
		for l276 in 0..2 {
			self.fVec70[(l276) as usize] = 0.0;
		}
		for l277 in 0..2 {
			self.fRec308[(l277) as usize] = 0.0;
		}
		for l278 in 0..2 {
			self.fRec305[(l278) as usize] = 0.0;
		}
		for l279 in 0..16384 {
			self.fVec71[(l279) as usize] = 0.0;
		}
		for l280 in 0..16384 {
			self.fVec72[(l280) as usize] = 0.0;
		}
		for l281 in 0..2 {
			self.fVec73[(l281) as usize] = 0.0;
		}
		for l282 in 0..2 {
			self.fRec242[(l282) as usize] = 0.0;
		}
		for l283 in 0..2 {
			self.fRec241[(l283) as usize] = 0.0;
		}
		for l284 in 0..3 {
			self.fRec240[(l284) as usize] = 0.0;
		}
		for l285 in 0..3 {
			self.fRec239[(l285) as usize] = 0.0;
		}
		for l286 in 0..2 {
			self.fVec74[(l286) as usize] = 0.0;
		}
		for l287 in 0..2 {
			self.fRec238[(l287) as usize] = 0.0;
		}
		for l288 in 0..3 {
			self.fRec237[(l288) as usize] = 0.0;
		}
		for l289 in 0..3 {
			self.fRec236[(l289) as usize] = 0.0;
		}
		for l290 in 0..2 {
			self.fRec312[(l290) as usize] = 0.0;
		}
		for l291 in 0..3 {
			self.fRec311[(l291) as usize] = 0.0;
		}
		for l292 in 0..3 {
			self.fRec310[(l292) as usize] = 0.0;
		}
		for l293 in 0..2 {
			self.fRec316[(l293) as usize] = 0.0;
		}
		for l294 in 0..3 {
			self.fRec315[(l294) as usize] = 0.0;
		}
		for l295 in 0..3 {
			self.fRec314[(l295) as usize] = 0.0;
		}
		for l296 in 0..3 {
			self.fRec313[(l296) as usize] = 0.0;
		}
		for l297 in 0..1024 {
			self.fVec75[(l297) as usize] = 0.0;
		}
		for l298 in 0..2 {
			self.fRec235[(l298) as usize] = 0.0;
		}
		for l299 in 0..2 {
			self.fRec328[(l299) as usize] = 0.0;
		}
		for l300 in 0..16384 {
			self.fVec76[(l300) as usize] = 0.0;
		}
		for l301 in 0..16384 {
			self.fVec77[(l301) as usize] = 0.0;
		}
		for l302 in 0..2 {
			self.fVec78[(l302) as usize] = 0.0;
		}
		for l303 in 0..2 {
			self.fRec327[(l303) as usize] = 0.0;
		}
		for l304 in 0..2 {
			self.fRec326[(l304) as usize] = 0.0;
		}
		for l305 in 0..3 {
			self.fRec325[(l305) as usize] = 0.0;
		}
		for l306 in 0..3 {
			self.fRec324[(l306) as usize] = 0.0;
		}
		for l307 in 0..2 {
			self.fVec79[(l307) as usize] = 0.0;
		}
		for l308 in 0..2 {
			self.fRec323[(l308) as usize] = 0.0;
		}
		for l309 in 0..3 {
			self.fRec322[(l309) as usize] = 0.0;
		}
		for l310 in 0..3 {
			self.fRec321[(l310) as usize] = 0.0;
		}
		for l311 in 0..2 {
			self.fRec331[(l311) as usize] = 0.0;
		}
		for l312 in 0..3 {
			self.fRec330[(l312) as usize] = 0.0;
		}
		for l313 in 0..3 {
			self.fRec329[(l313) as usize] = 0.0;
		}
		for l314 in 0..2 {
			self.fRec335[(l314) as usize] = 0.0;
		}
		for l315 in 0..3 {
			self.fRec334[(l315) as usize] = 0.0;
		}
		for l316 in 0..3 {
			self.fRec333[(l316) as usize] = 0.0;
		}
		for l317 in 0..3 {
			self.fRec332[(l317) as usize] = 0.0;
		}
		for l318 in 0..1024 {
			self.fVec80[(l318) as usize] = 0.0;
		}
		for l319 in 0..2 {
			self.fRec320[(l319) as usize] = 0.0;
		}
		for l320 in 0..16384 {
			self.fVec81[(l320) as usize] = 0.0;
		}
		for l321 in 0..2 {
			self.fVec82[(l321) as usize] = 0.0;
		}
		for l322 in 0..2 {
			self.fRec319[(l322) as usize] = 0.0;
		}
		for l323 in 0..2 {
			self.fRec317[(l323) as usize] = 0.0;
		}
		for l324 in 0..2 {
			self.fRec337[(l324) as usize] = 0.0;
		}
		for l325 in 0..16384 {
			self.fVec83[(l325) as usize] = 0.0;
		}
		for l326 in 0..2 {
			self.fVec84[(l326) as usize] = 0.0;
		}
		for l327 in 0..2 {
			self.fRec336[(l327) as usize] = 0.0;
		}
		for l328 in 0..2 {
			self.fRec318[(l328) as usize] = 0.0;
		}
		for l329 in 0..16384 {
			self.fVec85[(l329) as usize] = 0.0;
		}
		for l330 in 0..2 {
			self.fVec86[(l330) as usize] = 0.0;
		}
		for l331 in 0..2 {
			self.fRec340[(l331) as usize] = 0.0;
		}
		for l332 in 0..2 {
			self.fRec338[(l332) as usize] = 0.0;
		}
		for l333 in 0..16384 {
			self.fVec87[(l333) as usize] = 0.0;
		}
		for l334 in 0..2 {
			self.fVec88[(l334) as usize] = 0.0;
		}
		for l335 in 0..2 {
			self.fRec341[(l335) as usize] = 0.0;
		}
		for l336 in 0..2 {
			self.fRec339[(l336) as usize] = 0.0;
		}
		for l337 in 0..16384 {
			self.fVec89[(l337) as usize] = 0.0;
		}
		for l338 in 0..2 {
			self.fVec90[(l338) as usize] = 0.0;
		}
		for l339 in 0..2 {
			self.fRec344[(l339) as usize] = 0.0;
		}
		for l340 in 0..2 {
			self.fRec342[(l340) as usize] = 0.0;
		}
		for l341 in 0..2 {
			self.fRec346[(l341) as usize] = 0.0;
		}
		for l342 in 0..16384 {
			self.fVec91[(l342) as usize] = 0.0;
		}
		for l343 in 0..2 {
			self.fVec92[(l343) as usize] = 0.0;
		}
		for l344 in 0..2 {
			self.fRec345[(l344) as usize] = 0.0;
		}
		for l345 in 0..2 {
			self.fRec343[(l345) as usize] = 0.0;
		}
		for l346 in 0..2 {
			self.fRec350[(l346) as usize] = 0.0;
		}
		for l347 in 0..16384 {
			self.fVec93[(l347) as usize] = 0.0;
		}
		for l348 in 0..2 {
			self.fVec94[(l348) as usize] = 0.0;
		}
		for l349 in 0..2 {
			self.fRec349[(l349) as usize] = 0.0;
		}
		for l350 in 0..2 {
			self.fRec347[(l350) as usize] = 0.0;
		}
		for l351 in 0..16384 {
			self.fVec95[(l351) as usize] = 0.0;
		}
		for l352 in 0..2 {
			self.fVec96[(l352) as usize] = 0.0;
		}
		for l353 in 0..2 {
			self.fRec351[(l353) as usize] = 0.0;
		}
		for l354 in 0..2 {
			self.fRec348[(l354) as usize] = 0.0;
		}
		for l355 in 0..2 {
			self.fRec233[(l355) as usize] = 0.0;
		}
		for l356 in 0..2 {
			self.fRec234[(l356) as usize] = 0.0;
		}
		for l357 in 0..2 {
			self.fRec352[(l357) as usize] = 0.0;
		}
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		self.fConst0 = F32::min(1.92e+05, F32::max(1.0, ((self.fSampleRate) as F32)));
		self.fConst1 = 44.1 / self.fConst0;
		self.fConst2 = 1.0 - self.fConst1;
		self.fConst3 = 19404.0 / self.fConst0;
		self.fConst4 = 3.1415927 / self.fConst0;
		self.fConst5 = 1.0 / self.fConst0;
		self.fConst6 = 0.5 * self.fConst0;
		self.fConst7 = 0.25 * self.fConst0;
		self.fConst8 = 352.0 / self.fConst0;
		self.fConst9 = 2764.6016 / self.fConst0;
		self.fConst10 = 0.00882353 * self.fConst0;
		self.fConst11 = 0.00073529413 * self.fConst0;
		self.fConst12 = F32::exp(0.0 - 1e+04 / self.fConst0);
		self.fConst13 = 1.0 - self.fConst12;
		self.iConst14 = ((0.1 * self.fConst0) as i32);
		self.fConst15 = F32::exp(0.0 - 5e+01 / self.fConst0);
		self.fConst16 = 6911.504 / self.fConst0;
		self.fConst17 = 0.002 * self.fConst0;
		self.fConst18 = F32::exp(0.0 - 1e+01 / self.fConst0);
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
		self.fConst29 = 1.0 - fConst24;
		let mut fConst30: F32 = F32::tan(25132.742 / self.fConst0);
		let mut fConst31: F32 = mydsp_faustpower2_f(fConst30);
		self.fConst32 = 1.0 / fConst31;
		self.fConst33 = 2.0 * (1.0 - self.fConst32);
		self.fConst34 = 1.0 / fConst30;
		self.fConst35 = (self.fConst34 + -0.618034) / fConst30 + 1.0;
		self.fConst36 = 1.0 / ((self.fConst34 + 0.618034) / fConst30 + 1.0);
		self.fConst37 = (self.fConst34 + -1.618034) / fConst30 + 1.0;
		self.fConst38 = 1.0 / ((self.fConst34 + 1.618034) / fConst30 + 1.0);
		self.fConst39 = 1.0 - self.fConst34;
		self.fConst40 = 6.2831855 / self.fConst0;
		let mut fConst41: F32 = self.fConst34 + 1.0;
		self.fConst42 = 1.0 / fConst41;
		let mut fConst43: F32 = fConst24 + 1.0;
		self.fConst44 = 1.0 / fConst43;
		self.fConst45 = self.fConst29 / fConst43;
		self.fConst46 = 1.0 / (fConst20 * fConst43);
		self.fConst47 = 0.0 - self.fConst46;
		self.fConst48 = 0.0 - 2.0 / fConst21;
		self.fConst49 = (fConst24 + -1.618034) / fConst20 + 1.0;
		self.fConst50 = 1.0 / ((fConst24 + 1.618034) / fConst20 + 1.0);
		self.fConst51 = 0.0 - 1.0 / (fConst30 * fConst41);
		self.fConst52 = 0.0 - 2.0 / fConst31;
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
		ui_interface.add_horizontal_slider("volume", ParamIndex(21), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(22)), "1", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(22), 6e+01, 0.0, 127.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "2", "");
		ui_interface.open_horizontal_box("fx");
		ui_interface.declare(None, "0", "");
		ui_interface.open_vertical_box("echo");
		ui_interface.declare(Some(ParamIndex(23)), "0", "");
		ui_interface.declare(Some(ParamIndex(23)), "scale", "log");
		ui_interface.add_horizontal_slider("duration", ParamIndex(23), 0.3, 0.01, 3.0, 0.001);
		ui_interface.declare(Some(ParamIndex(24)), "0", "");
		ui_interface.add_horizontal_slider("mix", ParamIndex(24), 1.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(25)), "1", "");
		ui_interface.add_horizontal_slider("feedback", ParamIndex(25), 0.3, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "1", "");
		ui_interface.open_vertical_box("reverb");
		ui_interface.declare(Some(ParamIndex(26)), "0", "");
		ui_interface.add_horizontal_slider("mix", ParamIndex(26), 0.11, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(27)), "1", "");
		ui_interface.add_horizontal_slider("time", ParamIndex(27), 3.5, 0.1, 6e+01, 0.001);
		ui_interface.declare(Some(ParamIndex(28)), "2", "");
		ui_interface.add_horizontal_slider("damp", ParamIndex(28), 0.88, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(29)), "3", "");
		ui_interface.add_horizontal_slider("size", ParamIndex(29), 5.0, 0.5, 5.0, 0.001);
		ui_interface.declare(Some(ParamIndex(30)), "4", "");
		ui_interface.add_horizontal_slider("early_diff", ParamIndex(30), 0.75, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(31)), "5", "");
		ui_interface.add_horizontal_slider("mod_depth", ParamIndex(31), 0.98, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(32)), "6", "");
		ui_interface.add_horizontal_slider("mod_freq", ParamIndex(32), 0.6, 0.0, 1e+01, 0.001);
		ui_interface.close_box();
		ui_interface.close_box();
		ui_interface.declare(None, "3", "");
		ui_interface.open_vertical_box("mix");
		ui_interface.declare(Some(ParamIndex(33)), "0", "");
		ui_interface.add_horizontal_slider("master", ParamIndex(33), 1.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(34)), "1", "");
		ui_interface.add_horizontal_slider("drone", ParamIndex(34), 1.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(35)), "2", "");
		ui_interface.add_horizontal_slider("lead", ParamIndex(35), 1.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(36)), "3", "");
		ui_interface.add_horizontal_slider("pluck", ParamIndex(36), 1.0, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			18 => Some(self.fButton0),
			16 => Some(self.fButton1),
			14 => Some(self.fButton2),
			12 => Some(self.fButton3),
			2 => Some(self.fHslider0),
			1 => Some(self.fHslider1),
			35 => Some(self.fHslider10),
			0 => Some(self.fHslider11),
			22 => Some(self.fHslider12),
			34 => Some(self.fHslider13),
			21 => Some(self.fHslider14),
			36 => Some(self.fHslider15),
			19 => Some(self.fHslider16),
			20 => Some(self.fHslider17),
			11 => Some(self.fHslider18),
			17 => Some(self.fHslider19),
			9 => Some(self.fHslider2),
			15 => Some(self.fHslider20),
			13 => Some(self.fHslider21),
			24 => Some(self.fHslider22),
			23 => Some(self.fHslider23),
			25 => Some(self.fHslider24),
			26 => Some(self.fHslider25),
			29 => Some(self.fHslider26),
			32 => Some(self.fHslider27),
			31 => Some(self.fHslider28),
			27 => Some(self.fHslider29),
			10 => Some(self.fHslider3),
			28 => Some(self.fHslider30),
			30 => Some(self.fHslider31),
			33 => Some(self.fHslider32),
			7 => Some(self.fHslider4),
			8 => Some(self.fHslider5),
			5 => Some(self.fHslider6),
			6 => Some(self.fHslider7),
			3 => Some(self.fHslider8),
			4 => Some(self.fHslider9),
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			18 => { self.fButton0 = value }
			16 => { self.fButton1 = value }
			14 => { self.fButton2 = value }
			12 => { self.fButton3 = value }
			2 => { self.fHslider0 = value }
			1 => { self.fHslider1 = value }
			35 => { self.fHslider10 = value }
			0 => { self.fHslider11 = value }
			22 => { self.fHslider12 = value }
			34 => { self.fHslider13 = value }
			21 => { self.fHslider14 = value }
			36 => { self.fHslider15 = value }
			19 => { self.fHslider16 = value }
			20 => { self.fHslider17 = value }
			11 => { self.fHslider18 = value }
			17 => { self.fHslider19 = value }
			9 => { self.fHslider2 = value }
			15 => { self.fHslider20 = value }
			13 => { self.fHslider21 = value }
			24 => { self.fHslider22 = value }
			23 => { self.fHslider23 = value }
			25 => { self.fHslider24 = value }
			26 => { self.fHslider25 = value }
			29 => { self.fHslider26 = value }
			32 => { self.fHslider27 = value }
			31 => { self.fHslider28 = value }
			27 => { self.fHslider29 = value }
			10 => { self.fHslider3 = value }
			28 => { self.fHslider30 = value }
			30 => { self.fHslider31 = value }
			33 => { self.fHslider32 = value }
			7 => { self.fHslider4 = value }
			8 => { self.fHslider5 = value }
			5 => { self.fHslider6 = value }
			6 => { self.fHslider7 = value }
			3 => { self.fHslider8 = value }
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
		let mut fSlow0: F32 = self.fConst1 * self.fHslider0;
		let mut fSlow1: F32 = self.fConst1 * self.fHslider1;
		let mut fSlow2: F32 = self.fHslider2;
		let mut fSlow3: F32 = self.fConst3 * F32::powf(2.0, 0.083333336 * (fSlow2 + -69.0));
		let mut fSlow4: F32 = self.fConst1 * self.fHslider3;
		let mut fSlow5: F32 = self.fHslider4;
		let mut fSlow6: F32 = self.fConst3 * F32::powf(2.0, 0.083333336 * (fSlow5 + -69.0));
		let mut fSlow7: F32 = self.fConst1 * self.fHslider5;
		let mut fSlow8: F32 = self.fHslider6;
		let mut fSlow9: F32 = self.fConst3 * F32::powf(2.0, 0.083333336 * (fSlow8 + -69.0));
		let mut fSlow10: F32 = self.fConst1 * self.fHslider7;
		let mut fSlow11: F32 = self.fHslider8;
		let mut fSlow12: F32 = self.fConst3 * F32::powf(2.0, 0.083333336 * (fSlow11 + -69.0));
		let mut fSlow13: F32 = self.fConst1 * self.fHslider9;
		let mut fSlow14: F32 = self.fConst1 * self.fHslider10;
		let mut fSlow15: F32 = self.fConst1 * self.fHslider11;
		let mut fSlow16: F32 = self.fConst1 * self.fHslider12;
		let mut fSlow17: F32 = self.fConst1 * self.fHslider13;
		let mut fSlow18: F32 = self.fConst1 * self.fHslider14;
		let mut fSlow19: F32 = self.fConst1 * self.fHslider15;
		let mut fSlow20: F32 = self.fConst1 * self.fHslider16;
		let mut fSlow21: F32 = self.fConst1 * self.fHslider17;
		let mut fSlow22: F32 = self.fButton0;
		let mut fSlow23: F32 = self.fHslider18;
		let mut fSlow24: F32 = self.fConst1 * self.fHslider19;
		let mut fSlow25: F32 = self.fButton1;
		let mut fSlow26: F32 = self.fConst1 * self.fHslider20;
		let mut fSlow27: F32 = self.fButton2;
		let mut fSlow28: F32 = self.fConst1 * self.fHslider21;
		let mut fSlow29: F32 = self.fButton3;
		let mut fSlow30: F32 = self.fConst1 * self.fHslider22;
		let mut fSlow31: F32 = self.fConst1 * self.fHslider23;
		let mut fSlow32: F32 = self.fHslider24;
		let mut fSlow33: F32 = self.fHslider25;
		let mut fSlow34: F32 = 1.0 - fSlow33;
		let mut fSlow35: F32 = self.fHslider26;
		let mut iSlow36: i32 = unsafe { itbl0mydspSIG0[(((134.0 * fSlow35) as i32)) as usize] };
		let mut fSlow37: F32 = 0.005 * ((iSlow36) as F32);
		let mut iSlow38: i32 = unsafe { itbl0mydspSIG0[(((54.0 * fSlow35) as i32)) as usize] };
		let mut fSlow39: F32 = 0.005 * ((iSlow38) as F32);
		let mut iSlow40: i32 = unsafe { itbl0mydspSIG0[(((1e+01 * fSlow35) as i32)) as usize] };
		let mut fSlow41: F32 = 0.0001 * ((iSlow40) as F32);
		let mut iSlow42: i32 = unsafe { itbl0mydspSIG0[(((1.1e+02 * fSlow35) as i32)) as usize] };
		let mut fSlow43: F32 = 0.0001 * ((iSlow42) as F32);
		let mut iSlow44: i32 = unsafe { itbl0mydspSIG0[(((4e+01 * fSlow35) as i32)) as usize] };
		let mut fSlow45: F32 = 0.0001 * ((iSlow44) as F32);
		let mut iSlow46: i32 = unsafe { itbl0mydspSIG0[(((1.4e+02 * fSlow35) as i32)) as usize] };
		let mut fSlow47: F32 = 0.0001 * ((iSlow46) as F32);
		let mut iSlow48: i32 = unsafe { itbl0mydspSIG0[(((7e+01 * fSlow35) as i32)) as usize] };
		let mut fSlow49: F32 = 0.0001 * ((iSlow48) as F32);
		let mut iSlow50: i32 = unsafe { itbl0mydspSIG0[(((1.7e+02 * fSlow35) as i32)) as usize] };
		let mut fSlow51: F32 = 0.0001 * ((iSlow50) as F32);
		let mut iSlow52: i32 = unsafe { itbl0mydspSIG0[(((1e+02 * fSlow35) as i32)) as usize] };
		let mut fSlow53: F32 = 0.0001 * ((iSlow52) as F32);
		let mut iSlow54: i32 = unsafe { itbl0mydspSIG0[(((2e+02 * fSlow35) as i32)) as usize] };
		let mut fSlow55: F32 = 0.0001 * ((iSlow54) as F32);
		let mut iSlow56: i32 = unsafe { itbl0mydspSIG0[(((1.3e+02 * fSlow35) as i32)) as usize] };
		let mut fSlow57: F32 = 0.0001 * ((iSlow56) as F32);
		let mut iSlow58: i32 = unsafe { itbl0mydspSIG0[(((2.3e+02 * fSlow35) as i32)) as usize] };
		let mut fSlow59: F32 = 0.0001 * ((iSlow58) as F32);
		let mut fSlow60: F32 = self.fConst40 * self.fHslider27;
		let mut fSlow61: F32 = F32::cos(fSlow60);
		let mut fSlow62: F32 = F32::sin(fSlow60);
		let mut fSlow63: F32 = 5e+01 * self.fHslider28;
		let mut iSlow64: i32 = unsafe { itbl0mydspSIG0[(((125.0 * fSlow35) as i32)) as usize] };
		let mut fSlow65: F32 = 0.0001 * ((iSlow64) as F32);
		let mut iSlow66: i32 = unsafe { itbl0mydspSIG0[(((204.0 * fSlow35) as i32)) as usize] };
		let mut fSlow67: F32 = 0.005 * ((iSlow66) as F32);
		let mut fSlow68: F32 = 0.0 - fSlow63;
		let mut iSlow69: i32 = unsafe { itbl0mydspSIG0[(((25.0 * fSlow35) as i32)) as usize] };
		let mut fSlow70: F32 = 0.0001 * ((iSlow69) as F32);
		let mut iSlow71: i32 = unsafe { itbl0mydspSIG0[(((155.0 * fSlow35) as i32)) as usize] };
		let mut fSlow72: F32 = 0.0001 * ((iSlow71) as F32);
		let mut iSlow73: i32 = unsafe { itbl0mydspSIG0[(((55.0 * fSlow35) as i32)) as usize] };
		let mut fSlow74: F32 = 0.0001 * ((iSlow73) as F32);
		let mut iSlow75: i32 = unsafe { itbl0mydspSIG0[(((185.0 * fSlow35) as i32)) as usize] };
		let mut fSlow76: F32 = 0.0001 * ((iSlow75) as F32);
		let mut iSlow77: i32 = unsafe { itbl0mydspSIG0[(((85.0 * fSlow35) as i32)) as usize] };
		let mut fSlow78: F32 = 0.0001 * ((iSlow77) as F32);
		let mut iSlow79: i32 = unsafe { itbl0mydspSIG0[(((215.0 * fSlow35) as i32)) as usize] };
		let mut fSlow80: F32 = 0.0001 * ((iSlow79) as F32);
		let mut iSlow81: i32 = unsafe { itbl0mydspSIG0[(((115.0 * fSlow35) as i32)) as usize] };
		let mut fSlow82: F32 = 0.0001 * ((iSlow81) as F32);
		let mut iSlow83: i32 = unsafe { itbl0mydspSIG0[(((245.0 * fSlow35) as i32)) as usize] };
		let mut fSlow84: F32 = 0.0001 * ((iSlow83) as F32);
		let mut iSlow85: i32 = unsafe { itbl0mydspSIG0[(((145.0 * fSlow35) as i32)) as usize] };
		let mut fSlow86: F32 = 0.0001 * ((iSlow85) as F32);
		let mut fSlow87: F32 = F32::powf(1e+01, 0.0 - 0.51 * ((1.25 * fSlow35 + -0.25) / self.fHslider29));
		let mut fSlow88: F32 = self.fHslider30;
		let mut fSlow89: F32 = 1.0 - fSlow88;
		let mut fSlow90: F32 = self.fHslider31;
		let mut fSlow91: F32 = F32::sin(fSlow90);
		let mut iSlow92: i32 = unsafe { itbl0mydspSIG0[(((34.0 * fSlow35) as i32)) as usize] };
		let mut fSlow93: F32 = 0.005 * ((iSlow92) as F32);
		let mut fSlow94: F32 = F32::cos(fSlow90);
		let mut iSlow95: i32 = unsafe { itbl0mydspSIG0[(((2.4e+02 * fSlow35) as i32)) as usize] };
		let mut fSlow96: F32 = 0.0001 * ((iSlow95) as F32);
		let mut iSlow97: i32 = unsafe { itbl0mydspSIG0[(((1.9e+02 * fSlow35) as i32)) as usize] };
		let mut fSlow98: F32 = 0.0001 * ((iSlow97) as F32);
		let mut iSlow99: i32 = unsafe { itbl0mydspSIG0[(((175.0 * fSlow35) as i32)) as usize] };
		let mut fSlow100: F32 = 0.0001 * ((iSlow99) as F32);
		let mut fSlow101: F32 = self.fConst1 * self.fHslider32;
		let zipped_iterators = outputs0.zip(outputs1);
		for (output0, output1) in zipped_iterators {
			self.iVec0[0] = 1;
			self.fRec0[0] = fSlow0 + self.fConst2 * self.fRec0[1];
			let mut fTemp0: F32 = F32::min(1.4141995, 1.4142135 * self.fRec0[0]);
			let mut fTemp1: F32 = 1.4142135 * fTemp0;
			let mut fTemp2: F32 = 1.0 - fTemp1;
			self.fRec2[0] = fSlow1 + self.fConst2 * self.fRec2[1];
			let mut fTemp3: F32 = self.fRec2[0] + -69.0;
			self.fRec1[0] = self.fConst2 * self.fRec1[1] + self.fConst3 * F32::powf(2.0, 0.083333336 * (fSlow2 + fTemp3));
			let mut fTemp4: F32 = F32::tan(self.fConst4 * F32::max(2e+01, F32::min(1e+04, self.fRec1[0])));
			let mut fTemp5: F32 = 1.0 / fTemp4;
			let mut fTemp6: F32 = 2.0 - fTemp1;
			let mut fTemp7: F32 = mydsp_faustpower2_f(fTemp0);
			let mut fTemp8: F32 = fTemp7 + (fTemp6 + fTemp5) / fTemp4 + fTemp2;
			let mut fTemp9: F32 = 1.0 / mydsp_faustpower2_f(fTemp4);
			let mut fTemp10: F32 = fTemp1 + 2.0;
			let mut fTemp11: F32 = fTemp7 + fTemp1 + (fTemp10 + fTemp5) / fTemp4 + 1.0;
			let mut fTemp12: F32 = fTemp1 + fTemp7;
			self.fRec7[0] = fSlow3 + self.fConst2 * self.fRec7[1];
			let mut fTemp13: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec7[0]));
			let mut fTemp14: F32 = self.fRec5[1] + self.fConst5 * fTemp13;
			let mut fTemp15: F32 = fTemp14 + -1.0;
			let mut iTemp16: i32 = ((fTemp15 < 0.0) as i32);
			self.fRec5[0] = if (iTemp16 as i32 != 0) { fTemp14 } else { fTemp15 };
			let mut fThen1: F32 = fTemp14 + (1.0 - self.fConst0 / fTemp13) * fTemp15;
			let mut fRec6: F32 = if (iTemp16 as i32 != 0) { fTemp14 } else { fThen1 };
			self.fRec8[0] = fSlow4 + self.fConst2 * self.fRec8[1];
			self.fRec4[0] = self.fRec8[0] * (2.0 * fRec6 + -1.0) - (self.fRec4[2] * (fTemp12 + (1.0 - (fTemp10 - fTemp5) / fTemp4)) + 2.0 * self.fRec4[1] * (fTemp12 + (1.0 - fTemp9))) / fTemp11;
			self.fRec3[0] = (self.fRec4[2] + self.fRec4[0] + 2.0 * self.fRec4[1]) / fTemp11 - (self.fRec3[2] * (fTemp7 + (fTemp5 - fTemp6) / fTemp4 + fTemp2) + 2.0 * self.fRec3[1] * (fTemp7 + (1.0 - (fTemp1 + fTemp9)))) / fTemp8;
			self.fRec9[0] = self.fConst2 * self.fRec9[1] + self.fConst3 * F32::powf(2.0, 0.083333336 * (fSlow5 + fTemp3));
			let mut fTemp17: F32 = F32::tan(self.fConst4 * F32::max(2e+01, F32::min(1e+04, self.fRec9[0])));
			let mut fTemp18: F32 = 1.0 / fTemp17;
			let mut fTemp19: F32 = fTemp7 + (fTemp6 + fTemp18) / fTemp17 + fTemp2;
			let mut fTemp20: F32 = 1.0 / mydsp_faustpower2_f(fTemp17);
			let mut fTemp21: F32 = fTemp7 + fTemp1 + (fTemp10 + fTemp18) / fTemp17 + 1.0;
			self.fRec14[0] = fSlow6 + self.fConst2 * self.fRec14[1];
			let mut fTemp22: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec14[0]));
			let mut fTemp23: F32 = self.fRec12[1] + self.fConst5 * fTemp22;
			let mut fTemp24: F32 = fTemp23 + -1.0;
			let mut iTemp25: i32 = ((fTemp24 < 0.0) as i32);
			self.fRec12[0] = if (iTemp25 as i32 != 0) { fTemp23 } else { fTemp24 };
			let mut fThen3: F32 = fTemp23 + (1.0 - self.fConst0 / fTemp22) * fTemp24;
			let mut fRec13: F32 = if (iTemp25 as i32 != 0) { fTemp23 } else { fThen3 };
			self.fRec15[0] = fSlow7 + self.fConst2 * self.fRec15[1];
			self.fRec11[0] = self.fRec15[0] * (2.0 * fRec13 + -1.0) - (self.fRec11[2] * (fTemp12 + (1.0 - (fTemp10 - fTemp18) / fTemp17)) + 2.0 * self.fRec11[1] * (fTemp12 + (1.0 - fTemp20))) / fTemp21;
			self.fRec10[0] = (self.fRec11[2] + self.fRec11[0] + 2.0 * self.fRec11[1]) / fTemp21 - (self.fRec10[2] * (fTemp7 + (fTemp18 - fTemp6) / fTemp17 + fTemp2) + 2.0 * self.fRec10[1] * (fTemp7 + (1.0 - (fTemp1 + fTemp20)))) / fTemp19;
			self.fRec16[0] = self.fConst2 * self.fRec16[1] + self.fConst3 * F32::powf(2.0, 0.083333336 * (fSlow8 + fTemp3));
			let mut fTemp26: F32 = F32::tan(self.fConst4 * F32::max(2e+01, F32::min(1e+04, self.fRec16[0])));
			let mut fTemp27: F32 = 1.0 / fTemp26;
			let mut fTemp28: F32 = fTemp7 + (fTemp6 + fTemp27) / fTemp26 + fTemp2;
			let mut fTemp29: F32 = 1.0 / mydsp_faustpower2_f(fTemp26);
			let mut fTemp30: F32 = fTemp7 + fTemp1 + (fTemp10 + fTemp27) / fTemp26 + 1.0;
			self.fRec21[0] = fSlow9 + self.fConst2 * self.fRec21[1];
			let mut fTemp31: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec21[0]));
			let mut fTemp32: F32 = self.fConst5 * fTemp31;
			let mut fTemp33: F32 = self.fRec19[1] + fTemp32;
			let mut fTemp34: F32 = fTemp33 + -1.0;
			let mut iTemp35: i32 = ((fTemp34 < 0.0) as i32);
			self.fRec19[0] = if (iTemp35 as i32 != 0) { fTemp33 } else { fTemp34 };
			let mut fThen5: F32 = self.fRec19[1] + fTemp32 + (1.0 - self.fConst0 / fTemp31) * fTemp34;
			let mut fRec20: F32 = if (iTemp35 as i32 != 0) { fTemp33 } else { fThen5 };
			self.fRec22[0] = fSlow10 + self.fConst2 * self.fRec22[1];
			self.fRec18[0] = self.fRec22[0] * (2.0 * fRec20 + -1.0) - (self.fRec18[2] * (fTemp7 + fTemp1 + (fTemp27 - fTemp10) / fTemp26 + 1.0) + 2.0 * self.fRec18[1] * (fTemp12 + (1.0 - fTemp29))) / fTemp30;
			self.fRec17[0] = (self.fRec18[2] + self.fRec18[0] + 2.0 * self.fRec18[1]) / fTemp30 - (self.fRec17[2] * (fTemp7 + (fTemp27 - fTemp6) / fTemp26 + fTemp2) + 2.0 * self.fRec17[1] * (fTemp7 + (1.0 - (fTemp1 + fTemp29)))) / fTemp28;
			self.fRec23[0] = self.fConst2 * self.fRec23[1] + self.fConst3 * F32::powf(2.0, 0.083333336 * (fSlow11 + fTemp3));
			let mut fTemp36: F32 = F32::tan(self.fConst4 * F32::max(2e+01, F32::min(1e+04, self.fRec23[0])));
			let mut fTemp37: F32 = 1.0 / fTemp36;
			let mut fTemp38: F32 = fTemp7 + (fTemp6 + fTemp37) / fTemp36 + fTemp2;
			let mut fTemp39: F32 = 1.0 / mydsp_faustpower2_f(fTemp36);
			let mut fTemp40: F32 = fTemp7 + fTemp1 + (fTemp37 + fTemp10) / fTemp36 + 1.0;
			self.fRec28[0] = fSlow12 + self.fConst2 * self.fRec28[1];
			let mut fTemp41: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec28[0]));
			let mut fTemp42: F32 = self.fRec26[1] + self.fConst5 * fTemp41;
			let mut fTemp43: F32 = fTemp42 + -1.0;
			let mut iTemp44: i32 = ((fTemp43 < 0.0) as i32);
			self.fRec26[0] = if (iTemp44 as i32 != 0) { fTemp42 } else { fTemp43 };
			let mut fThen7: F32 = fTemp42 + (1.0 - self.fConst0 / fTemp41) * fTemp43;
			let mut fRec27: F32 = if (iTemp44 as i32 != 0) { fTemp42 } else { fThen7 };
			self.fRec29[0] = fSlow13 + self.fConst2 * self.fRec29[1];
			self.fRec25[0] = self.fRec29[0] * (2.0 * fRec27 + -1.0) - (self.fRec25[2] * (fTemp7 + fTemp1 + (fTemp37 - fTemp10) / fTemp36 + 1.0) + 2.0 * self.fRec25[1] * (fTemp12 + (1.0 - fTemp39))) / fTemp40;
			self.fRec24[0] = (self.fRec25[2] + self.fRec25[0] + 2.0 * self.fRec25[1]) / fTemp40 - (self.fRec24[2] * (fTemp7 + (fTemp37 - fTemp6) / fTemp36 + fTemp2) + 2.0 * self.fRec24[1] * (fTemp7 + (1.0 - (fTemp1 + fTemp39)))) / fTemp38;
			self.fRec30[0] = fSlow14 + self.fConst2 * self.fRec30[1];
			self.fRec31[0] = fSlow15 + self.fConst2 * self.fRec31[1];
			let mut fTemp45: F32 = self.fRec31[0] * self.fRec30[0] * ((self.fRec24[2] + self.fRec24[0] + 2.0 * self.fRec24[1]) / fTemp38 + (self.fRec17[2] + self.fRec17[0] + 2.0 * self.fRec17[1]) / fTemp28 + (self.fRec10[2] + self.fRec10[0] + 2.0 * self.fRec10[1]) / fTemp19 + (self.fRec3[2] + self.fRec3[0] + 2.0 * self.fRec3[1]) / fTemp8);
			self.fRec32[0] = fSlow16 + self.fConst2 * self.fRec32[1];
			let mut fTemp46: F32 = F32::powf(2.0, 0.083333336 * (self.fRec32[0] + -61.88));
			let mut fTemp47: F32 = F32::max(4.4e+02 * fTemp46, 23.44895);
			let mut fTemp48: F32 = F32::max(2e+01, F32::abs(fTemp47));
			let mut fTemp49: F32 = self.fRec34[1] + self.fConst5 * fTemp48;
			self.fRec34[0] = fTemp49 - F32::floor(fTemp49);
			let mut fTemp50: F32 = mydsp_faustpower2_f(2.0 * self.fRec34[0] + -1.0);
			self.fVec1[0] = fTemp50;
			let mut fTemp51: F32 = ((self.iVec0[1]) as F32);
			let mut fTemp52: F32 = fTemp51 * (fTemp50 - self.fVec1[1]) / fTemp48;
			self.fVec2[(self.IOTA0 & 4095) as usize] = fTemp52;
			let mut fTemp53: F32 = F32::max(0.0, F32::min(2047.0, self.fConst6 / fTemp47));
			let mut iTemp54: i32 = ((fTemp53) as i32);
			let mut fTemp55: F32 = F32::floor(fTemp53);
			self.fRec33[0] = 0.999 * self.fRec33[1] + self.fConst7 * (fTemp52 - self.fVec2[((i32::wrapping_sub(self.IOTA0, iTemp54)) & 4095) as usize] * (fTemp55 + (1.0 - fTemp53)) - (fTemp53 - fTemp55) * self.fVec2[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp54, 1))) & 4095) as usize]);
			let mut fTemp56: F32 = F32::powf(2.0, 0.083333336 * (self.fRec32[0] + -81.11));
			let mut fTemp57: F32 = F32::max(4.4e+02 * fTemp56, 23.44895);
			let mut fTemp58: F32 = F32::max(2e+01, F32::abs(fTemp57));
			let mut fTemp59: F32 = self.fRec36[1] + self.fConst5 * fTemp58;
			self.fRec36[0] = fTemp59 - F32::floor(fTemp59);
			let mut fTemp60: F32 = mydsp_faustpower2_f(2.0 * self.fRec36[0] + -1.0);
			self.fVec3[0] = fTemp60;
			let mut fTemp61: F32 = fTemp51 * (fTemp60 - self.fVec3[1]) / fTemp58;
			self.fVec4[(self.IOTA0 & 4095) as usize] = fTemp61;
			let mut fTemp62: F32 = F32::max(0.0, F32::min(2047.0, self.fConst6 / fTemp57));
			let mut iTemp63: i32 = ((fTemp62) as i32);
			let mut fTemp64: F32 = F32::floor(fTemp62);
			self.fRec35[0] = 0.999 * self.fRec35[1] + self.fConst7 * (fTemp61 - self.fVec4[((i32::wrapping_sub(self.IOTA0, iTemp63)) & 4095) as usize] * (fTemp64 + (1.0 - fTemp62)) - (fTemp62 - fTemp64) * self.fVec4[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp63, 1))) & 4095) as usize]);
			let mut fTemp65: F32 = F32::powf(2.0, 0.083333336 * (self.fRec32[0] + -56.9));
			let mut fTemp66: F32 = F32::max(4.4e+02 * fTemp65, 23.44895);
			let mut fTemp67: F32 = F32::max(2e+01, F32::abs(fTemp66));
			let mut fTemp68: F32 = self.fRec38[1] + self.fConst5 * fTemp67;
			self.fRec38[0] = fTemp68 - F32::floor(fTemp68);
			let mut fTemp69: F32 = mydsp_faustpower2_f(2.0 * self.fRec38[0] + -1.0);
			self.fVec5[0] = fTemp69;
			let mut fTemp70: F32 = fTemp51 * (fTemp69 - self.fVec5[1]) / fTemp67;
			self.fVec6[(self.IOTA0 & 4095) as usize] = fTemp70;
			let mut fTemp71: F32 = F32::max(0.0, F32::min(2047.0, self.fConst6 / fTemp66));
			let mut iTemp72: i32 = ((fTemp71) as i32);
			let mut fTemp73: F32 = F32::floor(fTemp71);
			self.fRec37[0] = 0.999 * self.fRec37[1] - self.fConst7 * (self.fVec6[((i32::wrapping_sub(self.IOTA0, iTemp72)) & 4095) as usize] * (fTemp73 + (1.0 - fTemp71)) - fTemp70 + (fTemp71 - fTemp73) * self.fVec6[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp72, 1))) & 4095) as usize]);
			let mut fTemp74: F32 = F32::powf(2.0, 0.083333336 * (self.fRec32[0] + -69.0));
			let mut fTemp75: F32 = F32::max(4.4e+02 * fTemp74, 23.44895);
			let mut fTemp76: F32 = F32::max(2e+01, F32::abs(fTemp75));
			let mut fTemp77: F32 = self.fRec40[1] + self.fConst5 * fTemp76;
			self.fRec40[0] = fTemp77 - F32::floor(fTemp77);
			let mut fTemp78: F32 = mydsp_faustpower2_f(2.0 * self.fRec40[0] + -1.0);
			self.fVec7[0] = fTemp78;
			let mut fTemp79: F32 = fTemp51 * (fTemp78 - self.fVec7[1]) / fTemp76;
			self.fVec8[(self.IOTA0 & 4095) as usize] = fTemp79;
			let mut fTemp80: F32 = F32::max(0.0, F32::min(2047.0, self.fConst6 / fTemp75));
			let mut iTemp81: i32 = ((fTemp80) as i32);
			let mut fTemp82: F32 = F32::floor(fTemp80);
			self.fRec39[0] = 0.999 * self.fRec39[1] + self.fConst7 * (fTemp79 - self.fVec8[((i32::wrapping_sub(self.IOTA0, iTemp81)) & 4095) as usize] * (fTemp82 + (1.0 - fTemp80)) - (fTemp80 - fTemp82) * self.fVec8[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp81, 1))) & 4095) as usize]);
			self.fRec41[0] = fSlow17 + self.fConst2 * self.fRec41[1];
			self.fRec42[0] = fSlow18 + self.fConst2 * self.fRec42[1];
			let mut fTemp83: F32 = self.fConst8 * self.fRec42[0] * self.fRec41[0] * (self.fRec39[0] * fTemp74 + self.fRec37[0] * fTemp65 + self.fRec35[0] * fTemp56 + self.fRec33[0] * fTemp46);
			self.fRec43[0] = fSlow19 + self.fConst2 * self.fRec43[1];
			self.fRec45[0] = fSlow20 + self.fConst2 * self.fRec45[1];
			self.fRec46[0] = fSlow21 + self.fConst2 * self.fRec46[1];
			let mut fTemp84: F32 = F32::powf(2.0, 0.083333336 * (self.fRec46[0] + self.fRec45[0] + -69.0));
			let mut fTemp85: F32 = 1.0 / F32::tan(self.fConst9 * fTemp84);
			let mut fRec61: F32 = -1.0 * 0.9973053 * (0.9 * self.fRec62[2] + 0.05 * (self.fRec62[1] + self.fRec62[3]));
			let mut fTemp86: F32 = self.fConst11 * (0.77272725 / fTemp84 + -0.11);
			let mut fTemp87: F32 = fTemp86 + -1.499995;
			let mut iTemp88: i32 = ((fTemp87) as i32);
			let mut iTemp89: i32 = ((F32::min(self.fConst10, ((std::cmp::max(0, i32::wrapping_add(iTemp88, 4))) as F32))) as i32);
			let mut iTemp90: i32 = i32::wrapping_add(iTemp89, 1);
			let mut fTemp91: F32 = F32::floor(fTemp87);
			let mut fTemp92: F32 = fTemp86 + (-3.0 - fTemp91);
			let mut fTemp93: F32 = fTemp86 + (-2.0 - fTemp91);
			let mut fTemp94: F32 = fTemp86 + (-1.0 - fTemp91);
			let mut fTemp95: F32 = fTemp94 * fTemp93;
			let mut fTemp96: F32 = fTemp95 * fTemp92;
			let mut fTemp97: F32 = fTemp86 + (-4.0 - fTemp91);
			let mut fTemp98: F32 = 0.0 - fTemp97;
			let mut iTemp99: i32 = ((F32::min(self.fConst10, ((std::cmp::max(0, i32::wrapping_add(iTemp88, 3))) as F32))) as i32);
			let mut iTemp100: i32 = i32::wrapping_add(iTemp99, 1);
			let mut fTemp101: F32 = 0.0 - 0.5 * fTemp97;
			let mut fTemp102: F32 = 0.0 - fTemp92;
			let mut iTemp103: i32 = ((F32::min(self.fConst10, ((std::cmp::max(0, i32::wrapping_add(iTemp88, 2))) as F32))) as i32);
			let mut iTemp104: i32 = i32::wrapping_add(iTemp103, 1);
			let mut fTemp105: F32 = 0.0 - 0.33333334 * fTemp97;
			let mut fTemp106: F32 = 0.0 - 0.5 * fTemp92;
			let mut fTemp107: F32 = 0.0 - fTemp93;
			let mut iTemp108: i32 = ((F32::min(self.fConst10, ((std::cmp::max(0, i32::wrapping_add(iTemp88, 1))) as F32))) as i32);
			let mut iTemp109: i32 = i32::wrapping_add(iTemp108, 1);
			let mut fTemp110: F32 = fTemp86 - fTemp91;
			let mut fTemp111: F32 = 0.0 - 0.25 * fTemp97;
			let mut fTemp112: F32 = 0.0 - 0.33333334 * fTemp92;
			let mut fTemp113: F32 = 0.0 - 0.5 * fTemp93;
			let mut fTemp114: F32 = 0.0 - fTemp94;
			let mut iTemp115: i32 = ((F32::min(self.fConst10, ((std::cmp::max(0, iTemp88)) as F32))) as i32);
			let mut iTemp116: i32 = i32::wrapping_add(iTemp115, 1);
			self.fRec76[0] = self.fRec53[((i32::wrapping_sub(self.IOTA0, iTemp116)) & 2047) as usize] * fTemp114 * fTemp113 * fTemp112 * fTemp111 + fTemp110 * (self.fRec53[((i32::wrapping_sub(self.IOTA0, iTemp109)) & 2047) as usize] * fTemp107 * fTemp106 * fTemp105 + 0.5 * fTemp94 * self.fRec53[((i32::wrapping_sub(self.IOTA0, iTemp104)) & 2047) as usize] * fTemp102 * fTemp101 + 0.16666667 * fTemp95 * self.fRec53[((i32::wrapping_sub(self.IOTA0, iTemp100)) & 2047) as usize] * fTemp98 + 0.041666668 * fTemp96 * self.fRec53[((i32::wrapping_sub(self.IOTA0, iTemp90)) & 2047) as usize]);
			self.fRec80[0] = 0.05 * self.fRec80[1] + 0.95 * self.fRec76[1];
			let mut fRec77: F32 = self.fRec80[0];
			self.fRec85[0] = self.fConst12 * self.fRec85[1] + self.fConst13 * F32::abs(self.fRec47[1]);
			let mut fRec84: F32 = self.fRec85[0];
			let mut iTemp117: i32 = ((fRec84 > 0.1) as i32);
			self.iVec9[0] = iTemp117;
			self.iRec86[0] = std::cmp::max(i32::wrapping_mul(self.iConst14, ((iTemp117 < self.iVec9[1]) as i32)), i32::wrapping_add(self.iRec86[1], -1));
			let mut fTemp118: F32 = F32::abs(F32::max(((iTemp117) as F32), ((((self.iRec86[0] > 0) as i32)) as F32)));
			let mut fTemp119: F32 = if (((self.fRec82[1] > fTemp118) as i32) as i32 != 0) { self.fConst15 } else { self.fConst12 };
			self.fRec83[0] = self.fRec83[1] * fTemp119 + fTemp118 * (1.0 - fTemp119);
			self.fRec82[0] = self.fRec83[0];
			let mut fTemp120: F32 = 0.005 * self.fRec82[0] * self.fRec47[1];
			self.fRec87[0] = self.fRec51[1];
			self.fRec88[(self.IOTA0 & 2047) as usize] = -1.0 * 0.9973053 * (0.9 * self.fRec87[2] + 0.05 * (self.fRec87[1] + self.fRec87[3]));
			let mut fTemp121: F32 = fTemp95 * fTemp98;
			let mut fTemp122: F32 = fTemp94 * fTemp102 * fTemp101;
			let mut fTemp123: F32 = fTemp107 * fTemp106 * fTemp105;
			let mut fTemp124: F32 = fTemp114 * fTemp113 * fTemp112 * fTemp111;
			self.fVec10[0] = fTemp124 * self.fRec88[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp115, 2))) & 2047) as usize] + fTemp110 * (fTemp123 * self.fRec88[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp108, 2))) & 2047) as usize] + 0.5 * fTemp122 * self.fRec88[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp103, 2))) & 2047) as usize] + 0.16666667 * fTemp121 * self.fRec88[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp99, 2))) & 2047) as usize] + 0.041666668 * fTemp96 * self.fRec88[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp89, 2))) & 2047) as usize]);
			let mut fTemp125: F32 = F32::tan(self.fConst16 * fTemp84);
			let mut fTemp126: F32 = 1.0 / fTemp125;
			let mut fTemp127: F32 = (fTemp126 + 1.4142135) / fTemp125 + 1.0;
			self.fVec11[0] = fSlow22;
			self.iRec89[0] = i32::wrapping_add(i32::wrapping_mul(i32::wrapping_add(self.iRec89[1], ((self.iRec89[1] > 0) as i32)), ((fSlow22 <= self.fVec11[1]) as i32)), ((fSlow22 > self.fVec11[1]) as i32));
			let mut fTemp128: F32 = ((self.iRec89[0]) as F32) / F32::max(1.0, self.fConst17 * mydsp_faustpower2_f(1.0 - 0.22 * fTemp84));
			self.iRec91[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, self.iRec91[1]), 12345);
			let mut fTemp129: F32 = 4.656613e-10 * ((self.iRec91[0]) as F32);
			self.fRec90[0] = fTemp129 - (self.fRec90[2] * ((fTemp126 + -1.4142135) / fTemp125 + 1.0) + 2.0 * self.fRec90[1] * (1.0 - 1.0 / mydsp_faustpower2_f(fTemp125))) / fTemp127;
			let mut fTemp130: F32 = 0.5 * ((self.fRec90[2] + self.fRec90[0] + 2.0 * self.fRec90[1]) * F32::max(0.0, F32::min(fTemp128, 2.0 - fTemp128)) / fTemp127);
			let mut fTemp131: F32 = fTemp130 + self.fVec10[1] + fTemp120;
			self.fVec12[0] = fTemp131;
			self.fRec81[(self.IOTA0 & 2047) as usize] = 0.05 * self.fRec81[((i32::wrapping_sub(self.IOTA0, 1)) & 2047) as usize] + 0.95 * self.fVec12[2];
			let mut fRec78: F32 = fTemp124 * self.fRec81[((i32::wrapping_sub(self.IOTA0, iTemp115)) & 2047) as usize] + fTemp110 * (fTemp123 * self.fRec81[((i32::wrapping_sub(self.IOTA0, iTemp108)) & 2047) as usize] + 0.5 * fTemp122 * self.fRec81[((i32::wrapping_sub(self.IOTA0, iTemp103)) & 2047) as usize] + 0.16666667 * fTemp121 * self.fRec81[((i32::wrapping_sub(self.IOTA0, iTemp99)) & 2047) as usize] + 0.041666668 * fTemp96 * self.fRec81[((i32::wrapping_sub(self.IOTA0, iTemp89)) & 2047) as usize]);
			let mut fRec79: F32 = self.fVec12[1] + self.fRec72[1];
			self.fRec72[0] = fRec77;
			let mut fRec73: F32 = self.fRec72[1];
			let mut fRec74: F32 = fRec78;
			let mut fRec75: F32 = fRec79;
			self.fRec68[0] = fRec73;
			let mut fRec69: F32 = fTemp120 + fTemp130 + self.fRec68[1];
			let mut fRec70: F32 = fRec74;
			let mut fRec71: F32 = fRec75;
			self.fRec64[(self.IOTA0 & 2047) as usize] = fRec69;
			let mut fRec65: F32 = fTemp124 * self.fRec64[((i32::wrapping_sub(self.IOTA0, iTemp116)) & 2047) as usize] + fTemp110 * (fTemp123 * self.fRec64[((i32::wrapping_sub(self.IOTA0, iTemp109)) & 2047) as usize] + 0.5 * fTemp122 * self.fRec64[((i32::wrapping_sub(self.IOTA0, iTemp104)) & 2047) as usize] + 0.16666667 * fTemp121 * self.fRec64[((i32::wrapping_sub(self.IOTA0, iTemp100)) & 2047) as usize] + 0.041666668 * fTemp96 * self.fRec64[((i32::wrapping_sub(self.IOTA0, iTemp90)) & 2047) as usize]);
			self.fRec66[0] = fRec70;
			let mut fRec67: F32 = fRec71;
			self.fRec62[0] = fSlow23 * self.fRec66[1];
			let mut fRec63: F32 = fRec67;
			self.fRec57[0] = fRec61;
			let mut fRec58: F32 = fSlow23 * self.fRec57[1];
			let mut fRec59: F32 = self.fRec62[0];
			let mut fRec60: F32 = fRec63;
			self.fRec53[(self.IOTA0 & 2047) as usize] = fRec58;
			let mut fRec54: F32 = fRec65;
			let mut fRec55: F32 = fRec59;
			let mut fRec56: F32 = fRec60;
			self.fRec51[0] = fRec54;
			let mut fRec52: F32 = fRec56;
			let mut fTemp132: F32 = F32::abs(fRec52);
			let mut fTemp133: F32 = if (((self.fRec49[1] > fTemp132) as i32) as i32 != 0) { self.fConst18 } else { 0.0 };
			self.fRec50[0] = self.fRec50[1] * fTemp133 + fTemp132 * (1.0 - fTemp133);
			self.fRec49[0] = self.fRec50[0];
			let mut fRec48: F32 = 0.0 - 0.95 * F32::max(2e+01 * F32::log10(self.fRec49[0]) + 1e+01, 0.0);
			self.fRec47[0] = fRec52 * F32::powf(1e+01, 0.05 * fRec48);
			self.fRec44[0] = 0.0 - (self.fRec44[1] * (1.0 - fTemp85) - (self.fRec47[0] + self.fRec47[1])) / (fTemp85 + 1.0);
			self.fRec93[0] = fSlow24 + self.fConst2 * self.fRec93[1];
			let mut fTemp134: F32 = F32::powf(2.0, 0.083333336 * (self.fRec46[0] + self.fRec93[0] + -69.0));
			let mut fTemp135: F32 = 1.0 / F32::tan(self.fConst9 * fTemp134);
			let mut fRec108: F32 = -1.0 * 0.9973053 * (0.9 * self.fRec109[2] + 0.05 * (self.fRec109[1] + self.fRec109[3]));
			let mut fTemp136: F32 = self.fConst11 * (0.77272725 / fTemp134 + -0.11);
			let mut fTemp137: F32 = fTemp136 + -1.499995;
			let mut iTemp138: i32 = ((fTemp137) as i32);
			let mut iTemp139: i32 = ((F32::min(self.fConst10, ((std::cmp::max(0, i32::wrapping_add(iTemp138, 4))) as F32))) as i32);
			let mut iTemp140: i32 = i32::wrapping_add(iTemp139, 1);
			let mut fTemp141: F32 = F32::floor(fTemp137);
			let mut fTemp142: F32 = fTemp136 + (-3.0 - fTemp141);
			let mut fTemp143: F32 = fTemp136 + (-2.0 - fTemp141);
			let mut fTemp144: F32 = fTemp136 + (-1.0 - fTemp141);
			let mut fTemp145: F32 = fTemp144 * fTemp143;
			let mut fTemp146: F32 = fTemp145 * fTemp142;
			let mut fTemp147: F32 = fTemp136 + (-4.0 - fTemp141);
			let mut fTemp148: F32 = 0.0 - fTemp147;
			let mut iTemp149: i32 = ((F32::min(self.fConst10, ((std::cmp::max(0, i32::wrapping_add(iTemp138, 3))) as F32))) as i32);
			let mut iTemp150: i32 = i32::wrapping_add(iTemp149, 1);
			let mut fTemp151: F32 = 0.0 - 0.5 * fTemp147;
			let mut fTemp152: F32 = 0.0 - fTemp142;
			let mut iTemp153: i32 = ((F32::min(self.fConst10, ((std::cmp::max(0, i32::wrapping_add(iTemp138, 2))) as F32))) as i32);
			let mut iTemp154: i32 = i32::wrapping_add(iTemp153, 1);
			let mut fTemp155: F32 = 0.0 - 0.33333334 * fTemp147;
			let mut fTemp156: F32 = 0.0 - 0.5 * fTemp142;
			let mut fTemp157: F32 = 0.0 - fTemp143;
			let mut iTemp158: i32 = ((F32::min(self.fConst10, ((std::cmp::max(0, i32::wrapping_add(iTemp138, 1))) as F32))) as i32);
			let mut iTemp159: i32 = i32::wrapping_add(iTemp158, 1);
			let mut fTemp160: F32 = fTemp136 - fTemp141;
			let mut fTemp161: F32 = 0.0 - 0.25 * fTemp147;
			let mut fTemp162: F32 = 0.0 - 0.33333334 * fTemp142;
			let mut fTemp163: F32 = 0.0 - 0.5 * fTemp143;
			let mut fTemp164: F32 = 0.0 - fTemp144;
			let mut iTemp165: i32 = ((F32::min(self.fConst10, ((std::cmp::max(0, iTemp138)) as F32))) as i32);
			let mut iTemp166: i32 = i32::wrapping_add(iTemp165, 1);
			self.fRec123[0] = self.fRec100[((i32::wrapping_sub(self.IOTA0, iTemp166)) & 2047) as usize] * fTemp164 * fTemp163 * fTemp162 * fTemp161 + fTemp160 * (self.fRec100[((i32::wrapping_sub(self.IOTA0, iTemp159)) & 2047) as usize] * fTemp157 * fTemp156 * fTemp155 + 0.5 * fTemp144 * self.fRec100[((i32::wrapping_sub(self.IOTA0, iTemp154)) & 2047) as usize] * fTemp152 * fTemp151 + 0.16666667 * fTemp145 * self.fRec100[((i32::wrapping_sub(self.IOTA0, iTemp150)) & 2047) as usize] * fTemp148 + 0.041666668 * fTemp146 * self.fRec100[((i32::wrapping_sub(self.IOTA0, iTemp140)) & 2047) as usize]);
			self.fRec127[0] = 0.05 * self.fRec127[1] + 0.95 * self.fRec123[1];
			let mut fRec124: F32 = self.fRec127[0];
			self.fRec132[0] = self.fConst12 * self.fRec132[1] + self.fConst13 * F32::abs(self.fRec94[1]);
			let mut fRec131: F32 = self.fRec132[0];
			let mut iTemp167: i32 = ((fRec131 > 0.1) as i32);
			self.iVec13[0] = iTemp167;
			self.iRec133[0] = std::cmp::max(i32::wrapping_mul(self.iConst14, ((iTemp167 < self.iVec13[1]) as i32)), i32::wrapping_add(self.iRec133[1], -1));
			let mut fTemp168: F32 = F32::abs(F32::max(((iTemp167) as F32), ((((self.iRec133[0] > 0) as i32)) as F32)));
			let mut fTemp169: F32 = if (((self.fRec129[1] > fTemp168) as i32) as i32 != 0) { self.fConst15 } else { self.fConst12 };
			self.fRec130[0] = self.fRec130[1] * fTemp169 + fTemp168 * (1.0 - fTemp169);
			self.fRec129[0] = self.fRec130[0];
			let mut fTemp170: F32 = 0.005 * self.fRec129[0] * self.fRec94[1];
			self.fRec134[0] = self.fRec98[1];
			self.fRec135[(self.IOTA0 & 2047) as usize] = -1.0 * 0.9973053 * (0.9 * self.fRec134[2] + 0.05 * (self.fRec134[1] + self.fRec134[3]));
			let mut fTemp171: F32 = fTemp145 * fTemp148;
			let mut fTemp172: F32 = fTemp144 * fTemp152 * fTemp151;
			let mut fTemp173: F32 = fTemp157 * fTemp156 * fTemp155;
			let mut fTemp174: F32 = fTemp164 * fTemp163 * fTemp162 * fTemp161;
			self.fVec14[0] = fTemp174 * self.fRec135[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp165, 2))) & 2047) as usize] + fTemp160 * (fTemp173 * self.fRec135[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp158, 2))) & 2047) as usize] + 0.5 * fTemp172 * self.fRec135[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp153, 2))) & 2047) as usize] + 0.16666667 * fTemp171 * self.fRec135[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp149, 2))) & 2047) as usize] + 0.041666668 * fTemp146 * self.fRec135[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp139, 2))) & 2047) as usize]);
			let mut fTemp175: F32 = F32::tan(self.fConst16 * fTemp134);
			let mut fTemp176: F32 = 1.0 / fTemp175;
			let mut fTemp177: F32 = (fTemp176 + 1.4142135) / fTemp175 + 1.0;
			self.fVec15[0] = fSlow25;
			self.iRec136[0] = i32::wrapping_add(i32::wrapping_mul(i32::wrapping_add(self.iRec136[1], ((self.iRec136[1] > 0) as i32)), ((fSlow25 <= self.fVec15[1]) as i32)), ((fSlow25 > self.fVec15[1]) as i32));
			let mut fTemp178: F32 = ((self.iRec136[0]) as F32) / F32::max(1.0, self.fConst17 * mydsp_faustpower2_f(1.0 - 0.22 * fTemp134));
			self.fRec137[0] = fTemp129 - (self.fRec137[2] * ((fTemp176 + -1.4142135) / fTemp175 + 1.0) + 2.0 * self.fRec137[1] * (1.0 - 1.0 / mydsp_faustpower2_f(fTemp175))) / fTemp177;
			let mut fTemp179: F32 = 0.5 * ((self.fRec137[2] + self.fRec137[0] + 2.0 * self.fRec137[1]) * F32::max(0.0, F32::min(fTemp178, 2.0 - fTemp178)) / fTemp177);
			let mut fTemp180: F32 = fTemp179 + self.fVec14[1] + fTemp170;
			self.fVec16[0] = fTemp180;
			self.fRec128[(self.IOTA0 & 2047) as usize] = 0.05 * self.fRec128[((i32::wrapping_sub(self.IOTA0, 1)) & 2047) as usize] + 0.95 * self.fVec16[2];
			let mut fRec125: F32 = fTemp174 * self.fRec128[((i32::wrapping_sub(self.IOTA0, iTemp165)) & 2047) as usize] + fTemp160 * (fTemp173 * self.fRec128[((i32::wrapping_sub(self.IOTA0, iTemp158)) & 2047) as usize] + 0.5 * fTemp172 * self.fRec128[((i32::wrapping_sub(self.IOTA0, iTemp153)) & 2047) as usize] + 0.16666667 * fTemp171 * self.fRec128[((i32::wrapping_sub(self.IOTA0, iTemp149)) & 2047) as usize] + 0.041666668 * fTemp146 * self.fRec128[((i32::wrapping_sub(self.IOTA0, iTemp139)) & 2047) as usize]);
			let mut fRec126: F32 = self.fVec16[1] + self.fRec119[1];
			self.fRec119[0] = fRec124;
			let mut fRec120: F32 = self.fRec119[1];
			let mut fRec121: F32 = fRec125;
			let mut fRec122: F32 = fRec126;
			self.fRec115[0] = fRec120;
			let mut fRec116: F32 = fTemp170 + fTemp179 + self.fRec115[1];
			let mut fRec117: F32 = fRec121;
			let mut fRec118: F32 = fRec122;
			self.fRec111[(self.IOTA0 & 2047) as usize] = fRec116;
			let mut fRec112: F32 = fTemp174 * self.fRec111[((i32::wrapping_sub(self.IOTA0, iTemp166)) & 2047) as usize] + fTemp160 * (fTemp173 * self.fRec111[((i32::wrapping_sub(self.IOTA0, iTemp159)) & 2047) as usize] + 0.5 * fTemp172 * self.fRec111[((i32::wrapping_sub(self.IOTA0, iTemp154)) & 2047) as usize] + 0.16666667 * fTemp171 * self.fRec111[((i32::wrapping_sub(self.IOTA0, iTemp150)) & 2047) as usize] + 0.041666668 * fTemp146 * self.fRec111[((i32::wrapping_sub(self.IOTA0, iTemp140)) & 2047) as usize]);
			self.fRec113[0] = fRec117;
			let mut fRec114: F32 = fRec118;
			self.fRec109[0] = fSlow23 * self.fRec113[1];
			let mut fRec110: F32 = fRec114;
			self.fRec104[0] = fRec108;
			let mut fRec105: F32 = fSlow23 * self.fRec104[1];
			let mut fRec106: F32 = self.fRec109[0];
			let mut fRec107: F32 = fRec110;
			self.fRec100[(self.IOTA0 & 2047) as usize] = fRec105;
			let mut fRec101: F32 = fRec112;
			let mut fRec102: F32 = fRec106;
			let mut fRec103: F32 = fRec107;
			self.fRec98[0] = fRec101;
			let mut fRec99: F32 = fRec103;
			let mut fTemp181: F32 = F32::abs(fRec99);
			let mut fTemp182: F32 = if (((self.fRec96[1] > fTemp181) as i32) as i32 != 0) { self.fConst18 } else { 0.0 };
			self.fRec97[0] = self.fRec97[1] * fTemp182 + fTemp181 * (1.0 - fTemp182);
			self.fRec96[0] = self.fRec97[0];
			let mut fRec95: F32 = 0.0 - 0.95 * F32::max(2e+01 * F32::log10(self.fRec96[0]) + 1e+01, 0.0);
			self.fRec94[0] = fRec99 * F32::powf(1e+01, 0.05 * fRec95);
			self.fRec92[0] = 0.0 - (self.fRec92[1] * (1.0 - fTemp135) - (self.fRec94[0] + self.fRec94[1])) / (fTemp135 + 1.0);
			self.fRec139[0] = fSlow26 + self.fConst2 * self.fRec139[1];
			let mut fTemp183: F32 = F32::powf(2.0, 0.083333336 * (self.fRec46[0] + self.fRec139[0] + -69.0));
			let mut fTemp184: F32 = 1.0 / F32::tan(self.fConst9 * fTemp183);
			let mut fRec154: F32 = -1.0 * 0.9973053 * (0.9 * self.fRec155[2] + 0.05 * (self.fRec155[1] + self.fRec155[3]));
			let mut fTemp185: F32 = self.fConst11 * (0.77272725 / fTemp183 + -0.11);
			let mut fTemp186: F32 = fTemp185 + -1.499995;
			let mut iTemp187: i32 = ((fTemp186) as i32);
			let mut iTemp188: i32 = ((F32::min(self.fConst10, ((std::cmp::max(0, i32::wrapping_add(iTemp187, 4))) as F32))) as i32);
			let mut iTemp189: i32 = i32::wrapping_add(iTemp188, 1);
			let mut fTemp190: F32 = F32::floor(fTemp186);
			let mut fTemp191: F32 = fTemp185 + (-3.0 - fTemp190);
			let mut fTemp192: F32 = fTemp185 + (-2.0 - fTemp190);
			let mut fTemp193: F32 = fTemp185 + (-1.0 - fTemp190);
			let mut fTemp194: F32 = fTemp193 * fTemp192;
			let mut fTemp195: F32 = fTemp194 * fTemp191;
			let mut fTemp196: F32 = fTemp185 + (-4.0 - fTemp190);
			let mut fTemp197: F32 = 0.0 - fTemp196;
			let mut iTemp198: i32 = ((F32::min(self.fConst10, ((std::cmp::max(0, i32::wrapping_add(iTemp187, 3))) as F32))) as i32);
			let mut iTemp199: i32 = i32::wrapping_add(iTemp198, 1);
			let mut fTemp200: F32 = 0.0 - 0.5 * fTemp196;
			let mut fTemp201: F32 = 0.0 - fTemp191;
			let mut iTemp202: i32 = ((F32::min(self.fConst10, ((std::cmp::max(0, i32::wrapping_add(iTemp187, 2))) as F32))) as i32);
			let mut iTemp203: i32 = i32::wrapping_add(iTemp202, 1);
			let mut fTemp204: F32 = 0.0 - 0.33333334 * fTemp196;
			let mut fTemp205: F32 = 0.0 - 0.5 * fTemp191;
			let mut fTemp206: F32 = 0.0 - fTemp192;
			let mut iTemp207: i32 = ((F32::min(self.fConst10, ((std::cmp::max(0, i32::wrapping_add(iTemp187, 1))) as F32))) as i32);
			let mut iTemp208: i32 = i32::wrapping_add(iTemp207, 1);
			let mut fTemp209: F32 = fTemp185 - fTemp190;
			let mut fTemp210: F32 = 0.0 - 0.25 * fTemp196;
			let mut fTemp211: F32 = 0.0 - 0.33333334 * fTemp191;
			let mut fTemp212: F32 = 0.0 - 0.5 * fTemp192;
			let mut fTemp213: F32 = 0.0 - fTemp193;
			let mut iTemp214: i32 = ((F32::min(self.fConst10, ((std::cmp::max(0, iTemp187)) as F32))) as i32);
			let mut iTemp215: i32 = i32::wrapping_add(iTemp214, 1);
			self.fRec169[0] = self.fRec146[((i32::wrapping_sub(self.IOTA0, iTemp215)) & 2047) as usize] * fTemp213 * fTemp212 * fTemp211 * fTemp210 + fTemp209 * (self.fRec146[((i32::wrapping_sub(self.IOTA0, iTemp208)) & 2047) as usize] * fTemp206 * fTemp205 * fTemp204 + 0.5 * fTemp193 * self.fRec146[((i32::wrapping_sub(self.IOTA0, iTemp203)) & 2047) as usize] * fTemp201 * fTemp200 + 0.16666667 * fTemp194 * self.fRec146[((i32::wrapping_sub(self.IOTA0, iTemp199)) & 2047) as usize] * fTemp197 + 0.041666668 * fTemp195 * self.fRec146[((i32::wrapping_sub(self.IOTA0, iTemp189)) & 2047) as usize]);
			self.fRec173[0] = 0.05 * self.fRec173[1] + 0.95 * self.fRec169[1];
			let mut fRec170: F32 = self.fRec173[0];
			self.fRec178[0] = self.fConst12 * self.fRec178[1] + self.fConst13 * F32::abs(self.fRec140[1]);
			let mut fRec177: F32 = self.fRec178[0];
			let mut iTemp216: i32 = ((fRec177 > 0.1) as i32);
			self.iVec17[0] = iTemp216;
			self.iRec179[0] = std::cmp::max(i32::wrapping_mul(self.iConst14, ((iTemp216 < self.iVec17[1]) as i32)), i32::wrapping_add(self.iRec179[1], -1));
			let mut fTemp217: F32 = F32::abs(F32::max(((iTemp216) as F32), ((((self.iRec179[0] > 0) as i32)) as F32)));
			let mut fTemp218: F32 = if (((self.fRec175[1] > fTemp217) as i32) as i32 != 0) { self.fConst15 } else { self.fConst12 };
			self.fRec176[0] = self.fRec176[1] * fTemp218 + fTemp217 * (1.0 - fTemp218);
			self.fRec175[0] = self.fRec176[0];
			let mut fTemp219: F32 = 0.005 * self.fRec175[0] * self.fRec140[1];
			self.fRec180[0] = self.fRec144[1];
			self.fRec181[(self.IOTA0 & 2047) as usize] = -1.0 * 0.9973053 * (0.9 * self.fRec180[2] + 0.05 * (self.fRec180[1] + self.fRec180[3]));
			let mut fTemp220: F32 = fTemp194 * fTemp197;
			let mut fTemp221: F32 = fTemp193 * fTemp201 * fTemp200;
			let mut fTemp222: F32 = fTemp206 * fTemp205 * fTemp204;
			let mut fTemp223: F32 = fTemp213 * fTemp212 * fTemp211 * fTemp210;
			self.fVec18[0] = fTemp223 * self.fRec181[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp214, 2))) & 2047) as usize] + fTemp209 * (fTemp222 * self.fRec181[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp207, 2))) & 2047) as usize] + 0.5 * fTemp221 * self.fRec181[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp202, 2))) & 2047) as usize] + 0.16666667 * fTemp220 * self.fRec181[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp198, 2))) & 2047) as usize] + 0.041666668 * fTemp195 * self.fRec181[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp188, 2))) & 2047) as usize]);
			let mut fTemp224: F32 = F32::tan(self.fConst16 * fTemp183);
			let mut fTemp225: F32 = 1.0 / fTemp224;
			let mut fTemp226: F32 = (fTemp225 + 1.4142135) / fTemp224 + 1.0;
			self.fVec19[0] = fSlow27;
			self.iRec182[0] = i32::wrapping_add(i32::wrapping_mul(i32::wrapping_add(self.iRec182[1], ((self.iRec182[1] > 0) as i32)), ((fSlow27 <= self.fVec19[1]) as i32)), ((fSlow27 > self.fVec19[1]) as i32));
			let mut fTemp227: F32 = ((self.iRec182[0]) as F32) / F32::max(1.0, self.fConst17 * mydsp_faustpower2_f(1.0 - 0.22 * fTemp183));
			self.fRec183[0] = fTemp129 - (self.fRec183[2] * ((fTemp225 + -1.4142135) / fTemp224 + 1.0) + 2.0 * self.fRec183[1] * (1.0 - 1.0 / mydsp_faustpower2_f(fTemp224))) / fTemp226;
			let mut fTemp228: F32 = 0.5 * ((self.fRec183[2] + self.fRec183[0] + 2.0 * self.fRec183[1]) * F32::max(0.0, F32::min(fTemp227, 2.0 - fTemp227)) / fTemp226);
			let mut fTemp229: F32 = fTemp228 + self.fVec18[1] + fTemp219;
			self.fVec20[0] = fTemp229;
			self.fRec174[(self.IOTA0 & 2047) as usize] = 0.05 * self.fRec174[((i32::wrapping_sub(self.IOTA0, 1)) & 2047) as usize] + 0.95 * self.fVec20[2];
			let mut fRec171: F32 = fTemp223 * self.fRec174[((i32::wrapping_sub(self.IOTA0, iTemp214)) & 2047) as usize] + fTemp209 * (fTemp222 * self.fRec174[((i32::wrapping_sub(self.IOTA0, iTemp207)) & 2047) as usize] + 0.5 * fTemp221 * self.fRec174[((i32::wrapping_sub(self.IOTA0, iTemp202)) & 2047) as usize] + 0.16666667 * fTemp220 * self.fRec174[((i32::wrapping_sub(self.IOTA0, iTemp198)) & 2047) as usize] + 0.041666668 * fTemp195 * self.fRec174[((i32::wrapping_sub(self.IOTA0, iTemp188)) & 2047) as usize]);
			let mut fRec172: F32 = self.fVec20[1] + self.fRec165[1];
			self.fRec165[0] = fRec170;
			let mut fRec166: F32 = self.fRec165[1];
			let mut fRec167: F32 = fRec171;
			let mut fRec168: F32 = fRec172;
			self.fRec161[0] = fRec166;
			let mut fRec162: F32 = fTemp219 + fTemp228 + self.fRec161[1];
			let mut fRec163: F32 = fRec167;
			let mut fRec164: F32 = fRec168;
			self.fRec157[(self.IOTA0 & 2047) as usize] = fRec162;
			let mut fRec158: F32 = fTemp223 * self.fRec157[((i32::wrapping_sub(self.IOTA0, iTemp215)) & 2047) as usize] + fTemp209 * (fTemp222 * self.fRec157[((i32::wrapping_sub(self.IOTA0, iTemp208)) & 2047) as usize] + 0.5 * fTemp221 * self.fRec157[((i32::wrapping_sub(self.IOTA0, iTemp203)) & 2047) as usize] + 0.16666667 * fTemp220 * self.fRec157[((i32::wrapping_sub(self.IOTA0, iTemp199)) & 2047) as usize] + 0.041666668 * fTemp195 * self.fRec157[((i32::wrapping_sub(self.IOTA0, iTemp189)) & 2047) as usize]);
			self.fRec159[0] = fRec163;
			let mut fRec160: F32 = fRec164;
			self.fRec155[0] = fSlow23 * self.fRec159[1];
			let mut fRec156: F32 = fRec160;
			self.fRec150[0] = fRec154;
			let mut fRec151: F32 = fSlow23 * self.fRec150[1];
			let mut fRec152: F32 = self.fRec155[0];
			let mut fRec153: F32 = fRec156;
			self.fRec146[(self.IOTA0 & 2047) as usize] = fRec151;
			let mut fRec147: F32 = fRec158;
			let mut fRec148: F32 = fRec152;
			let mut fRec149: F32 = fRec153;
			self.fRec144[0] = fRec147;
			let mut fRec145: F32 = fRec149;
			let mut fTemp230: F32 = F32::abs(fRec145);
			let mut fTemp231: F32 = if (((self.fRec142[1] > fTemp230) as i32) as i32 != 0) { self.fConst18 } else { 0.0 };
			self.fRec143[0] = self.fRec143[1] * fTemp231 + fTemp230 * (1.0 - fTemp231);
			self.fRec142[0] = self.fRec143[0];
			let mut fRec141: F32 = 0.0 - 0.95 * F32::max(2e+01 * F32::log10(self.fRec142[0]) + 1e+01, 0.0);
			self.fRec140[0] = fRec145 * F32::powf(1e+01, 0.05 * fRec141);
			self.fRec138[0] = 0.0 - (self.fRec138[1] * (1.0 - fTemp184) - (self.fRec140[0] + self.fRec140[1])) / (fTemp184 + 1.0);
			self.fRec185[0] = fSlow28 + self.fConst2 * self.fRec185[1];
			let mut fTemp232: F32 = F32::powf(2.0, 0.083333336 * (self.fRec185[0] + self.fRec46[0] + -69.0));
			let mut fTemp233: F32 = 1.0 / F32::tan(self.fConst9 * fTemp232);
			let mut fRec200: F32 = -1.0 * 0.9973053 * (0.9 * self.fRec201[2] + 0.05 * (self.fRec201[1] + self.fRec201[3]));
			let mut fTemp234: F32 = self.fConst11 * (0.77272725 / fTemp232 + -0.11);
			let mut fTemp235: F32 = fTemp234 + -1.499995;
			let mut iTemp236: i32 = ((fTemp235) as i32);
			let mut iTemp237: i32 = ((F32::min(self.fConst10, ((std::cmp::max(0, i32::wrapping_add(iTemp236, 4))) as F32))) as i32);
			let mut iTemp238: i32 = i32::wrapping_add(iTemp237, 1);
			let mut fTemp239: F32 = F32::floor(fTemp235);
			let mut fTemp240: F32 = fTemp234 + (-3.0 - fTemp239);
			let mut fTemp241: F32 = fTemp234 + (-2.0 - fTemp239);
			let mut fTemp242: F32 = fTemp234 + (-1.0 - fTemp239);
			let mut fTemp243: F32 = fTemp242 * fTemp241;
			let mut fTemp244: F32 = fTemp243 * fTemp240;
			let mut fTemp245: F32 = fTemp234 + (-4.0 - fTemp239);
			let mut fTemp246: F32 = 0.0 - fTemp245;
			let mut iTemp247: i32 = ((F32::min(self.fConst10, ((std::cmp::max(0, i32::wrapping_add(iTemp236, 3))) as F32))) as i32);
			let mut iTemp248: i32 = i32::wrapping_add(iTemp247, 1);
			let mut fTemp249: F32 = 0.0 - 0.5 * fTemp245;
			let mut fTemp250: F32 = 0.0 - fTemp240;
			let mut iTemp251: i32 = ((F32::min(self.fConst10, ((std::cmp::max(0, i32::wrapping_add(iTemp236, 2))) as F32))) as i32);
			let mut iTemp252: i32 = i32::wrapping_add(iTemp251, 1);
			let mut fTemp253: F32 = 0.0 - 0.33333334 * fTemp245;
			let mut fTemp254: F32 = 0.0 - 0.5 * fTemp240;
			let mut fTemp255: F32 = 0.0 - fTemp241;
			let mut iTemp256: i32 = ((F32::min(self.fConst10, ((std::cmp::max(0, i32::wrapping_add(iTemp236, 1))) as F32))) as i32);
			let mut iTemp257: i32 = i32::wrapping_add(iTemp256, 1);
			let mut fTemp258: F32 = fTemp234 - fTemp239;
			let mut fTemp259: F32 = 0.0 - 0.25 * fTemp245;
			let mut fTemp260: F32 = 0.0 - 0.33333334 * fTemp240;
			let mut fTemp261: F32 = 0.0 - 0.5 * fTemp241;
			let mut fTemp262: F32 = 0.0 - fTemp242;
			let mut iTemp263: i32 = ((F32::min(self.fConst10, ((std::cmp::max(0, iTemp236)) as F32))) as i32);
			let mut iTemp264: i32 = i32::wrapping_add(iTemp263, 1);
			self.fRec215[0] = self.fRec192[((i32::wrapping_sub(self.IOTA0, iTemp264)) & 2047) as usize] * fTemp262 * fTemp261 * fTemp260 * fTemp259 + fTemp258 * (self.fRec192[((i32::wrapping_sub(self.IOTA0, iTemp257)) & 2047) as usize] * fTemp255 * fTemp254 * fTemp253 + 0.5 * fTemp242 * self.fRec192[((i32::wrapping_sub(self.IOTA0, iTemp252)) & 2047) as usize] * fTemp250 * fTemp249 + 0.16666667 * fTemp243 * self.fRec192[((i32::wrapping_sub(self.IOTA0, iTemp248)) & 2047) as usize] * fTemp246 + 0.041666668 * fTemp244 * self.fRec192[((i32::wrapping_sub(self.IOTA0, iTemp238)) & 2047) as usize]);
			self.fRec219[0] = 0.05 * self.fRec219[1] + 0.95 * self.fRec215[1];
			let mut fRec216: F32 = self.fRec219[0];
			self.fRec224[0] = self.fConst12 * self.fRec224[1] + self.fConst13 * F32::abs(self.fRec186[1]);
			let mut fRec223: F32 = self.fRec224[0];
			let mut iTemp265: i32 = ((fRec223 > 0.1) as i32);
			self.iVec21[0] = iTemp265;
			self.iRec225[0] = std::cmp::max(i32::wrapping_mul(self.iConst14, ((iTemp265 < self.iVec21[1]) as i32)), i32::wrapping_add(self.iRec225[1], -1));
			let mut fTemp266: F32 = F32::abs(F32::max(((iTemp265) as F32), ((((self.iRec225[0] > 0) as i32)) as F32)));
			let mut fTemp267: F32 = if (((self.fRec221[1] > fTemp266) as i32) as i32 != 0) { self.fConst15 } else { self.fConst12 };
			self.fRec222[0] = self.fRec222[1] * fTemp267 + fTemp266 * (1.0 - fTemp267);
			self.fRec221[0] = self.fRec222[0];
			let mut fTemp268: F32 = 0.005 * self.fRec221[0] * self.fRec186[1];
			self.fRec226[0] = self.fRec190[1];
			self.fRec227[(self.IOTA0 & 2047) as usize] = -1.0 * 0.9973053 * (0.9 * self.fRec226[2] + 0.05 * (self.fRec226[1] + self.fRec226[3]));
			let mut fTemp269: F32 = fTemp243 * fTemp246;
			let mut fTemp270: F32 = fTemp242 * fTemp250 * fTemp249;
			let mut fTemp271: F32 = fTemp255 * fTemp254 * fTemp253;
			let mut fTemp272: F32 = fTemp262 * fTemp261 * fTemp260 * fTemp259;
			self.fVec22[0] = fTemp272 * self.fRec227[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp263, 2))) & 2047) as usize] + fTemp258 * (fTemp271 * self.fRec227[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp256, 2))) & 2047) as usize] + 0.5 * fTemp270 * self.fRec227[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp251, 2))) & 2047) as usize] + 0.16666667 * fTemp269 * self.fRec227[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp247, 2))) & 2047) as usize] + 0.041666668 * fTemp244 * self.fRec227[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp237, 2))) & 2047) as usize]);
			let mut fTemp273: F32 = F32::tan(self.fConst16 * fTemp232);
			let mut fTemp274: F32 = 1.0 / fTemp273;
			let mut fTemp275: F32 = (fTemp274 + 1.4142135) / fTemp273 + 1.0;
			self.fVec23[0] = fSlow29;
			self.iRec228[0] = i32::wrapping_add(i32::wrapping_mul(i32::wrapping_add(self.iRec228[1], ((self.iRec228[1] > 0) as i32)), ((fSlow29 <= self.fVec23[1]) as i32)), ((fSlow29 > self.fVec23[1]) as i32));
			let mut fTemp276: F32 = ((self.iRec228[0]) as F32) / F32::max(1.0, self.fConst17 * mydsp_faustpower2_f(1.0 - 0.22 * fTemp232));
			self.fRec229[0] = fTemp129 - (self.fRec229[2] * ((fTemp274 + -1.4142135) / fTemp273 + 1.0) + 2.0 * self.fRec229[1] * (1.0 - 1.0 / mydsp_faustpower2_f(fTemp273))) / fTemp275;
			let mut fTemp277: F32 = 0.5 * ((self.fRec229[2] + self.fRec229[0] + 2.0 * self.fRec229[1]) * F32::max(0.0, F32::min(fTemp276, 2.0 - fTemp276)) / fTemp275);
			let mut fTemp278: F32 = fTemp277 + self.fVec22[1] + fTemp268;
			self.fVec24[0] = fTemp278;
			self.fRec220[(self.IOTA0 & 2047) as usize] = 0.05 * self.fRec220[((i32::wrapping_sub(self.IOTA0, 1)) & 2047) as usize] + 0.95 * self.fVec24[2];
			let mut fRec217: F32 = fTemp272 * self.fRec220[((i32::wrapping_sub(self.IOTA0, iTemp263)) & 2047) as usize] + fTemp258 * (fTemp271 * self.fRec220[((i32::wrapping_sub(self.IOTA0, iTemp256)) & 2047) as usize] + 0.5 * fTemp270 * self.fRec220[((i32::wrapping_sub(self.IOTA0, iTemp251)) & 2047) as usize] + 0.16666667 * fTemp269 * self.fRec220[((i32::wrapping_sub(self.IOTA0, iTemp247)) & 2047) as usize] + 0.041666668 * fTemp244 * self.fRec220[((i32::wrapping_sub(self.IOTA0, iTemp237)) & 2047) as usize]);
			let mut fRec218: F32 = self.fVec24[1] + self.fRec211[1];
			self.fRec211[0] = fRec216;
			let mut fRec212: F32 = self.fRec211[1];
			let mut fRec213: F32 = fRec217;
			let mut fRec214: F32 = fRec218;
			self.fRec207[0] = fRec212;
			let mut fRec208: F32 = fTemp268 + fTemp277 + self.fRec207[1];
			let mut fRec209: F32 = fRec213;
			let mut fRec210: F32 = fRec214;
			self.fRec203[(self.IOTA0 & 2047) as usize] = fRec208;
			let mut fRec204: F32 = fTemp272 * self.fRec203[((i32::wrapping_sub(self.IOTA0, iTemp264)) & 2047) as usize] + fTemp258 * (fTemp271 * self.fRec203[((i32::wrapping_sub(self.IOTA0, iTemp257)) & 2047) as usize] + 0.5 * fTemp270 * self.fRec203[((i32::wrapping_sub(self.IOTA0, iTemp252)) & 2047) as usize] + 0.16666667 * fTemp269 * self.fRec203[((i32::wrapping_sub(self.IOTA0, iTemp248)) & 2047) as usize] + 0.041666668 * fTemp244 * self.fRec203[((i32::wrapping_sub(self.IOTA0, iTemp238)) & 2047) as usize]);
			self.fRec205[0] = fRec209;
			let mut fRec206: F32 = fRec210;
			self.fRec201[0] = fSlow23 * self.fRec205[1];
			let mut fRec202: F32 = fRec206;
			self.fRec196[0] = fRec200;
			let mut fRec197: F32 = fSlow23 * self.fRec196[1];
			let mut fRec198: F32 = self.fRec201[0];
			let mut fRec199: F32 = fRec202;
			self.fRec192[(self.IOTA0 & 2047) as usize] = fRec197;
			let mut fRec193: F32 = fRec204;
			let mut fRec194: F32 = fRec198;
			let mut fRec195: F32 = fRec199;
			self.fRec190[0] = fRec193;
			let mut fRec191: F32 = fRec195;
			let mut fTemp279: F32 = F32::abs(fRec191);
			let mut fTemp280: F32 = if (((self.fRec188[1] > fTemp279) as i32) as i32 != 0) { self.fConst18 } else { 0.0 };
			self.fRec189[0] = self.fRec189[1] * fTemp280 + fTemp279 * (1.0 - fTemp280);
			self.fRec188[0] = self.fRec189[0];
			let mut fRec187: F32 = 0.0 - 0.95 * F32::max(2e+01 * F32::log10(self.fRec188[0]) + 1e+01, 0.0);
			self.fRec186[0] = fRec191 * F32::powf(1e+01, 0.05 * fRec187);
			self.fRec184[0] = 0.0 - (self.fRec184[1] * (1.0 - fTemp233) - (self.fRec186[0] + self.fRec186[1])) / (fTemp233 + 1.0);
			let mut fTemp281: F32 = (self.fRec184[0] + self.fRec138[0] + self.fRec92[0] + self.fRec44[0]) * self.fRec43[0];
			self.fRec230[0] = fSlow30 + self.fConst2 * self.fRec230[1];
			let mut fTemp282: F32 = (1.0 - self.fRec230[0]) * (fTemp281 + fTemp83 + fTemp45);
			self.fRec232[0] = fSlow31 + self.fConst2 * self.fRec232[1];
			self.fRec231[(self.IOTA0 & 2097151) as usize] = fTemp281 + fTemp45 + fSlow32 * self.fRec231[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(((F32::min(self.fConst19, F32::max(0.0, self.fConst0 * self.fRec232[0]))) as i32), 1))) & 2097151) as usize] + fTemp83;
			let mut fTemp283: F32 = self.fRec231[(self.IOTA0 & 2097151) as usize] * self.fRec230[0];
			let mut iTemp284: i32 = i32::wrapping_sub(1, self.iVec0[1]);
			self.fRec243[0] = 0.995 * (self.fRec243[1] + ((i32::wrapping_mul(iTemp284, iSlow36)) as F32)) + fSlow37;
			let mut fTemp285: F32 = self.fRec243[0] + -1.49999;
			let mut fTemp286: F32 = F32::floor(fTemp285);
			self.fRec245[0] = 0.995 * (self.fRec245[1] + ((i32::wrapping_mul(iTemp284, iSlow38)) as F32)) + fSlow39;
			let mut fTemp287: F32 = self.fRec245[0] + -1.49999;
			let mut fTemp288: F32 = F32::floor(fTemp287);
			self.fRec249[0] = 0.9999 * (self.fRec249[1] + ((i32::wrapping_mul(iTemp284, iSlow40)) as F32)) + fSlow41;
			let mut fTemp289: F32 = self.fRec249[0] + -1.49999;
			let mut fTemp290: F32 = F32::floor(fTemp289);
			let mut fTemp291: F32 = 0.760314 * self.fRec233[1] - 0.64955574 * self.fRec246[1];
			let mut fTemp292: F32 = 0.760314 * self.fRec234[1] - 0.64955574 * self.fRec247[1];
			self.fVec25[(self.IOTA0 & 16383) as usize] = 0.0 - 0.70710677 * (fTemp292 - fTemp291);
			let mut fTemp293: F32 = self.fVec25[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp289) as i32))))) & 16383) as usize];
			self.fVec26[0] = fTemp293;
			self.fRec248[0] = self.fVec26[1] - (fTemp290 + (2.0 - self.fRec249[0])) * (self.fRec248[1] - fTemp293) / (self.fRec249[0] - fTemp290);
			self.fRec246[0] = self.fRec248[0];
			self.fRec251[0] = 0.9999 * (self.fRec251[1] + ((i32::wrapping_mul(iTemp284, iSlow42)) as F32)) + fSlow43;
			let mut fTemp294: F32 = self.fRec251[0] + -1.49999;
			let mut fTemp295: F32 = F32::floor(fTemp294);
			let mut fTemp296: F32 = self.fRec251[0] - fTemp295;
			let mut fTemp297: F32 = fTemp295 + (2.0 - self.fRec251[0]);
			self.fVec27[(self.IOTA0 & 16383) as usize] = fTemp291 + fTemp292;
			let mut fTemp298: F32 = self.fVec27[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp294) as i32))))) & 16383) as usize];
			self.fVec28[0] = fTemp298;
			self.fRec250[0] = 0.70710677 * (fTemp297 * fTemp298 / fTemp296 + self.fVec28[1]) - self.fRec250[1] * fTemp297 / fTemp296;
			self.fRec247[0] = self.fRec250[0];
			let mut fTemp299: F32 = 0.760314 * self.fRec246[1] + 0.64955574 * self.fRec233[1];
			self.fRec255[0] = 0.9999 * (self.fRec255[1] + ((i32::wrapping_mul(iTemp284, iSlow44)) as F32)) + fSlow45;
			let mut fTemp300: F32 = self.fRec255[0] + -1.49999;
			let mut fTemp301: F32 = F32::floor(fTemp300);
			let mut fTemp302: F32 = self.fRec255[0] - fTemp301;
			let mut fTemp303: F32 = fTemp301 + (2.0 - self.fRec255[0]);
			let mut fTemp304: F32 = 0.760314 * self.fRec247[1] + 0.64955574 * self.fRec234[1];
			let mut fTemp305: F32 = 0.760314 * fTemp304 - 0.64955574 * self.fRec253[1];
			let mut fTemp306: F32 = 0.760314 * fTemp299 - 0.64955574 * self.fRec252[1];
			self.fVec29[(self.IOTA0 & 16383) as usize] = fTemp306 - fTemp305;
			let mut fTemp307: F32 = self.fVec29[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp300) as i32))))) & 16383) as usize];
			self.fVec30[0] = fTemp307;
			self.fRec254[0] = 0.70710677 * (fTemp303 * fTemp307 / fTemp302 + self.fVec30[1]) - self.fRec254[1] * fTemp303 / fTemp302;
			self.fRec252[0] = self.fRec254[0];
			self.fRec257[0] = 0.9999 * (self.fRec257[1] + ((i32::wrapping_mul(iTemp284, iSlow46)) as F32)) + fSlow47;
			let mut fTemp308: F32 = self.fRec257[0] + -1.49999;
			let mut fTemp309: F32 = F32::floor(fTemp308);
			let mut fTemp310: F32 = self.fRec257[0] - fTemp309;
			let mut fTemp311: F32 = fTemp309 + (2.0 - self.fRec257[0]);
			self.fVec31[(self.IOTA0 & 16383) as usize] = fTemp306 + fTemp305;
			let mut fTemp312: F32 = self.fVec31[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp308) as i32))))) & 16383) as usize];
			self.fVec32[0] = fTemp312;
			self.fRec256[0] = 0.70710677 * (fTemp311 * fTemp312 / fTemp310 + self.fVec32[1]) - self.fRec256[1] * fTemp311 / fTemp310;
			self.fRec253[0] = self.fRec256[0];
			let mut fTemp313: F32 = 0.760314 * self.fRec252[1] + 0.64955574 * fTemp299;
			self.fRec261[0] = 0.9999 * (self.fRec261[1] + ((i32::wrapping_mul(iTemp284, iSlow48)) as F32)) + fSlow49;
			let mut fTemp314: F32 = self.fRec261[0] + -1.49999;
			let mut fTemp315: F32 = F32::floor(fTemp314);
			let mut fTemp316: F32 = self.fRec261[0] - fTemp315;
			let mut fTemp317: F32 = fTemp315 + (2.0 - self.fRec261[0]);
			let mut fTemp318: F32 = 0.760314 * self.fRec253[1] + 0.64955574 * fTemp304;
			let mut fTemp319: F32 = 0.760314 * fTemp318 - 0.64955574 * self.fRec259[1];
			let mut fTemp320: F32 = 0.760314 * fTemp313 - 0.64955574 * self.fRec258[1];
			self.fVec33[(self.IOTA0 & 16383) as usize] = fTemp320 - fTemp319;
			let mut fTemp321: F32 = self.fVec33[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp314) as i32))))) & 16383) as usize];
			self.fVec34[0] = fTemp321;
			self.fRec260[0] = 0.70710677 * (fTemp317 * fTemp321 / fTemp316 + self.fVec34[1]) - self.fRec260[1] * fTemp317 / fTemp316;
			self.fRec258[0] = self.fRec260[0];
			self.fRec263[0] = 0.9999 * (self.fRec263[1] + ((i32::wrapping_mul(iTemp284, iSlow50)) as F32)) + fSlow51;
			let mut fTemp322: F32 = self.fRec263[0] + -1.49999;
			let mut fTemp323: F32 = F32::floor(fTemp322);
			let mut fTemp324: F32 = self.fRec263[0] - fTemp323;
			let mut fTemp325: F32 = fTemp323 + (2.0 - self.fRec263[0]);
			self.fVec35[(self.IOTA0 & 16383) as usize] = fTemp320 + fTemp319;
			let mut fTemp326: F32 = self.fVec35[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp322) as i32))))) & 16383) as usize];
			self.fVec36[0] = fTemp326;
			self.fRec262[0] = 0.70710677 * (fTemp325 * fTemp326 / fTemp324 + self.fVec36[1]) - self.fRec262[1] * fTemp325 / fTemp324;
			self.fRec259[0] = self.fRec262[0];
			let mut fTemp327: F32 = 0.760314 * self.fRec258[1] + 0.64955574 * fTemp313;
			self.fRec267[0] = 0.9999 * (self.fRec267[1] + ((i32::wrapping_mul(iTemp284, iSlow52)) as F32)) + fSlow53;
			let mut fTemp328: F32 = self.fRec267[0] + -1.49999;
			let mut fTemp329: F32 = F32::floor(fTemp328);
			let mut fTemp330: F32 = 0.760314 * fTemp327 - 0.64955574 * self.fRec264[1];
			let mut fTemp331: F32 = 0.760314 * self.fRec259[1] + 0.64955574 * fTemp318;
			let mut fTemp332: F32 = 0.760314 * fTemp331 - 0.64955574 * self.fRec265[1];
			self.fVec37[(self.IOTA0 & 16383) as usize] = 0.0 - 0.70710677 * (fTemp332 - fTemp330);
			let mut fTemp333: F32 = self.fVec37[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp328) as i32))))) & 16383) as usize];
			self.fVec38[0] = fTemp333;
			self.fRec266[0] = self.fVec38[1] - (fTemp329 + (2.0 - self.fRec267[0])) * (self.fRec266[1] - fTemp333) / (self.fRec267[0] - fTemp329);
			self.fRec264[0] = self.fRec266[0];
			self.fRec269[0] = 0.9999 * (self.fRec269[1] + ((i32::wrapping_mul(iTemp284, iSlow54)) as F32)) + fSlow55;
			let mut fTemp334: F32 = self.fRec269[0] + -1.49999;
			let mut fTemp335: F32 = F32::floor(fTemp334);
			let mut fTemp336: F32 = self.fRec269[0] - fTemp335;
			let mut fTemp337: F32 = fTemp335 + (2.0 - self.fRec269[0]);
			self.fVec39[(self.IOTA0 & 16383) as usize] = fTemp330 + fTemp332;
			let mut fTemp338: F32 = self.fVec39[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp334) as i32))))) & 16383) as usize];
			self.fVec40[0] = fTemp338;
			self.fRec268[0] = 0.70710677 * (fTemp337 * fTemp338 / fTemp336 + self.fVec40[1]) - self.fRec268[1] * fTemp337 / fTemp336;
			self.fRec265[0] = self.fRec268[0];
			let mut fTemp339: F32 = 0.760314 * self.fRec264[1] + 0.64955574 * fTemp327;
			self.fRec273[0] = 0.9999 * (self.fRec273[1] + ((i32::wrapping_mul(iTemp284, iSlow56)) as F32)) + fSlow57;
			let mut fTemp340: F32 = self.fRec273[0] + -1.49999;
			let mut fTemp341: F32 = F32::floor(fTemp340);
			let mut fTemp342: F32 = 0.760314 * fTemp339 - 0.64955574 * self.fRec270[1];
			let mut fTemp343: F32 = 0.760314 * self.fRec265[1] + 0.64955574 * fTemp331;
			let mut fTemp344: F32 = 0.760314 * fTemp343 - 0.64955574 * self.fRec271[1];
			self.fVec41[(self.IOTA0 & 16383) as usize] = 0.0 - 0.70710677 * (fTemp344 - fTemp342);
			let mut fTemp345: F32 = self.fVec41[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp340) as i32))))) & 16383) as usize];
			self.fVec42[0] = fTemp345;
			self.fRec272[0] = self.fVec42[1] + (fTemp341 + (2.0 - self.fRec273[0])) * (fTemp345 - self.fRec272[1]) / (self.fRec273[0] - fTemp341);
			self.fRec270[0] = self.fRec272[0];
			self.fRec275[0] = 0.9999 * (self.fRec275[1] + ((i32::wrapping_mul(iTemp284, iSlow58)) as F32)) + fSlow59;
			let mut fTemp346: F32 = self.fRec275[0] + -1.49999;
			let mut fTemp347: F32 = F32::floor(fTemp346);
			let mut fTemp348: F32 = self.fRec275[0] - fTemp347;
			let mut fTemp349: F32 = fTemp347 + (2.0 - self.fRec275[0]);
			self.fVec43[(self.IOTA0 & 16383) as usize] = fTemp342 + fTemp344;
			let mut fTemp350: F32 = self.fVec43[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp346) as i32))))) & 16383) as usize];
			self.fVec44[0] = fTemp350;
			self.fRec274[0] = 0.70710677 * (fTemp349 * fTemp350 / fTemp348 + self.fVec44[1]) - self.fRec274[1] * fTemp349 / fTemp348;
			self.fRec271[0] = self.fRec274[0];
			let mut fTemp351: F32 = 0.760314 * self.fRec270[1] + 0.64955574 * fTemp339;
			self.fVec45[(self.IOTA0 & 1023) as usize] = fTemp351;
			self.fRec276[0] = fSlow62 * self.fRec277[1] + fSlow61 * self.fRec276[1];
			self.fRec277[0] = ((iTemp284) as F32) + fSlow61 * self.fRec277[1] - fSlow62 * self.fRec276[1];
			let mut fTemp352: F32 = fSlow63 * (self.fRec277[0] + 1.0);
			let mut fTemp353: F32 = fTemp352 + 3.500005;
			let mut iTemp354: i32 = ((fTemp353) as i32);
			let mut iTemp355: i32 = std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp354, 4)));
			let mut fTemp356: F32 = F32::floor(fTemp353);
			let mut fTemp357: F32 = fTemp352 + (2.0 - fTemp356);
			let mut fTemp358: F32 = fTemp352 + (3.0 - fTemp356);
			let mut fTemp359: F32 = fTemp352 + (4.0 - fTemp356);
			let mut fTemp360: F32 = fTemp359 * fTemp358;
			let mut fTemp361: F32 = fTemp360 * fTemp357;
			let mut fTemp362: F32 = fTemp352 + (1.0 - fTemp356);
			let mut fTemp363: F32 = 0.0 - fTemp362;
			let mut iTemp364: i32 = std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp354, 3)));
			let mut fTemp365: F32 = 0.0 - 0.5 * fTemp362;
			let mut fTemp366: F32 = 0.0 - fTemp357;
			let mut iTemp367: i32 = std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp354, 2)));
			let mut fTemp368: F32 = 0.0 - 0.33333334 * fTemp362;
			let mut fTemp369: F32 = 0.0 - 0.5 * fTemp357;
			let mut fTemp370: F32 = 0.0 - fTemp358;
			let mut iTemp371: i32 = std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp354, 1)));
			let mut fTemp372: F32 = fTemp352 + (5.0 - fTemp356);
			let mut fTemp373: F32 = 0.0 - 0.25 * fTemp362;
			let mut fTemp374: F32 = 0.0 - 0.33333334 * fTemp357;
			let mut fTemp375: F32 = 0.0 - 0.5 * fTemp358;
			let mut fTemp376: F32 = 0.0 - fTemp359;
			let mut iTemp377: i32 = std::cmp::min(512, std::cmp::max(0, iTemp354));
			self.fVec46[(self.IOTA0 & 16383) as usize] = self.fVec45[((i32::wrapping_sub(self.IOTA0, iTemp377)) & 1023) as usize] * fTemp376 * fTemp375 * fTemp374 * fTemp373 + fTemp372 * (self.fVec45[((i32::wrapping_sub(self.IOTA0, iTemp371)) & 1023) as usize] * fTemp370 * fTemp369 * fTemp368 + 0.5 * fTemp359 * self.fVec45[((i32::wrapping_sub(self.IOTA0, iTemp367)) & 1023) as usize] * fTemp366 * fTemp365 + 0.16666667 * fTemp360 * self.fVec45[((i32::wrapping_sub(self.IOTA0, iTemp364)) & 1023) as usize] * fTemp363 + 0.041666668 * fTemp361 * self.fVec45[((i32::wrapping_sub(self.IOTA0, iTemp355)) & 1023) as usize]);
			let mut fTemp378: F32 = self.fVec46[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp287) as i32))))) & 16383) as usize];
			self.fVec47[0] = fTemp378;
			self.fRec244[0] = self.fVec47[1] + (fTemp288 + (2.0 - self.fRec245[0])) * (fTemp378 - self.fRec244[1]) / (self.fRec245[0] - fTemp288);
			self.fRec281[0] = 0.9999 * (self.fRec281[1] + ((i32::wrapping_mul(iTemp284, iSlow64)) as F32)) + fSlow65;
			let mut fTemp379: F32 = self.fRec281[0] + -1.49999;
			let mut fTemp380: F32 = F32::floor(fTemp379);
			let mut fTemp381: F32 = self.fRec281[0] - fTemp380;
			let mut fTemp382: F32 = fTemp380 + (2.0 - self.fRec281[0]);
			self.fRec283[0] = 0.995 * (self.fRec283[1] + ((i32::wrapping_mul(iTemp284, iSlow66)) as F32)) + fSlow67;
			let mut fTemp383: F32 = self.fRec283[0] + -1.49999;
			let mut fTemp384: F32 = F32::floor(fTemp383);
			let mut fTemp385: F32 = 0.760314 * self.fRec271[1] + 0.64955574 * fTemp343;
			self.fVec48[(self.IOTA0 & 1023) as usize] = fTemp385;
			let mut fTemp386: F32 = fSlow68 * self.fRec277[0];
			let mut fTemp387: F32 = fSlow63 + fTemp386 + 3.500005;
			let mut iTemp388: i32 = ((fTemp387) as i32);
			let mut fTemp389: F32 = F32::floor(fTemp387);
			let mut fTemp390: F32 = fSlow63 + fTemp386 + (2.0 - fTemp389);
			let mut fTemp391: F32 = fSlow63 + fTemp386 + (3.0 - fTemp389);
			let mut fTemp392: F32 = fSlow63 + fTemp386 + (4.0 - fTemp389);
			let mut fTemp393: F32 = fTemp392 * fTemp391;
			let mut fTemp394: F32 = fSlow63 + fTemp386 + (1.0 - fTemp389);
			self.fVec49[(self.IOTA0 & 16383) as usize] = self.fVec48[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, std::cmp::max(0, iTemp388)))) & 1023) as usize] * (0.0 - fTemp392) * (0.0 - 0.5 * fTemp391) * (0.0 - 0.33333334 * fTemp390) * (0.0 - 0.25 * fTemp394) + (fSlow63 + fTemp386 + (5.0 - fTemp389)) * (self.fVec48[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp388, 1))))) & 1023) as usize] * (0.0 - fTemp391) * (0.0 - 0.5 * fTemp390) * (0.0 - 0.33333334 * fTemp394) + 0.5 * fTemp392 * self.fVec48[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp388, 2))))) & 1023) as usize] * (0.0 - fTemp390) * (0.0 - 0.5 * fTemp394) + 0.16666667 * fTemp393 * self.fVec48[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp388, 3))))) & 1023) as usize] * (0.0 - fTemp394) + 0.041666668 * fTemp393 * fTemp390 * self.fVec48[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp388, 4))))) & 1023) as usize]);
			let mut fTemp395: F32 = self.fVec49[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp383) as i32))))) & 16383) as usize];
			self.fVec50[0] = fTemp395;
			self.fRec282[0] = self.fVec50[1] - (fTemp384 + (2.0 - self.fRec283[0])) * (self.fRec282[1] - fTemp395) / (self.fRec283[0] - fTemp384);
			let mut fTemp396: F32 = 0.760314 * self.fRec282[0] - 0.64955574 * self.fRec279[1];
			let mut fTemp397: F32 = 0.760314 * self.fRec244[0] - 0.64955574 * self.fRec278[1];
			self.fVec51[(self.IOTA0 & 16383) as usize] = fTemp397 - fTemp396;
			let mut fTemp398: F32 = self.fVec51[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp379) as i32))))) & 16383) as usize];
			self.fVec52[0] = fTemp398;
			self.fRec280[0] = 0.70710677 * (fTemp382 * fTemp398 / fTemp381 + self.fVec52[1]) - self.fRec280[1] * fTemp382 / fTemp381;
			self.fRec278[0] = self.fRec280[0];
			self.fRec285[0] = 0.9999 * (self.fRec285[1] + ((i32::wrapping_mul(iTemp284, iSlow69)) as F32)) + fSlow70;
			let mut fTemp399: F32 = self.fRec285[0] + -1.49999;
			let mut fTemp400: F32 = F32::floor(fTemp399);
			let mut fTemp401: F32 = self.fRec285[0] - fTemp400;
			let mut fTemp402: F32 = fTemp400 + (2.0 - self.fRec285[0]);
			self.fVec53[(self.IOTA0 & 16383) as usize] = fTemp397 + fTemp396;
			let mut fTemp403: F32 = self.fVec53[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp399) as i32))))) & 16383) as usize];
			self.fVec54[0] = fTemp403;
			self.fRec284[0] = 0.70710677 * (fTemp402 * fTemp403 / fTemp401 + self.fVec54[1]) - self.fRec284[1] * fTemp402 / fTemp401;
			self.fRec279[0] = self.fRec284[0];
			let mut fTemp404: F32 = 0.760314 * self.fRec278[1] + 0.64955574 * self.fRec244[0];
			self.fRec289[0] = 0.9999 * (self.fRec289[1] + ((i32::wrapping_mul(iTemp284, iSlow71)) as F32)) + fSlow72;
			let mut fTemp405: F32 = self.fRec289[0] + -1.49999;
			let mut fTemp406: F32 = F32::floor(fTemp405);
			let mut fTemp407: F32 = 0.760314 * fTemp404 - 0.64955574 * self.fRec286[1];
			let mut fTemp408: F32 = 0.760314 * self.fRec279[1] + 0.64955574 * self.fRec282[0];
			let mut fTemp409: F32 = 0.760314 * fTemp408 - 0.64955574 * self.fRec287[1];
			self.fVec55[(self.IOTA0 & 16383) as usize] = 0.0 - 0.70710677 * (fTemp409 - fTemp407);
			let mut fTemp410: F32 = self.fVec55[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp405) as i32))))) & 16383) as usize];
			self.fVec56[0] = fTemp410;
			self.fRec288[0] = self.fVec56[1] + (fTemp406 + (2.0 - self.fRec289[0])) * (fTemp410 - self.fRec288[1]) / (self.fRec289[0] - fTemp406);
			self.fRec286[0] = self.fRec288[0];
			self.fRec291[0] = 0.9999 * (self.fRec291[1] + ((i32::wrapping_mul(iTemp284, iSlow73)) as F32)) + fSlow74;
			let mut fTemp411: F32 = self.fRec291[0] + -1.49999;
			let mut fTemp412: F32 = F32::floor(fTemp411);
			let mut fTemp413: F32 = self.fRec291[0] - fTemp412;
			let mut fTemp414: F32 = fTemp412 + (2.0 - self.fRec291[0]);
			self.fVec57[(self.IOTA0 & 16383) as usize] = fTemp407 + fTemp409;
			let mut iTemp415: i32 = std::cmp::min(8192, std::cmp::max(0, ((fTemp411) as i32)));
			let mut fTemp416: F32 = self.fVec57[((i32::wrapping_sub(self.IOTA0, iTemp415)) & 16383) as usize];
			self.fVec58[0] = fTemp416;
			self.fRec290[0] = 0.70710677 * (fTemp414 * fTemp416 / fTemp413 + self.fVec58[1]) - fTemp414 * self.fRec290[1] / fTemp413;
			self.fRec287[0] = self.fRec290[0];
			let mut fTemp417: F32 = 0.760314 * self.fRec286[1] + 0.64955574 * fTemp404;
			self.fRec295[0] = 0.9999 * (self.fRec295[1] + ((i32::wrapping_mul(iTemp284, iSlow75)) as F32)) + fSlow76;
			let mut fTemp418: F32 = self.fRec295[0] + -1.49999;
			let mut fTemp419: F32 = F32::floor(fTemp418);
			let mut fTemp420: F32 = self.fRec295[0] - fTemp419;
			let mut fTemp421: F32 = fTemp419 + (2.0 - self.fRec295[0]);
			let mut fTemp422: F32 = 0.760314 * self.fRec287[1] + 0.64955574 * fTemp408;
			let mut fTemp423: F32 = 0.760314 * fTemp422 - 0.64955574 * self.fRec293[1];
			let mut fTemp424: F32 = 0.760314 * fTemp417 - 0.64955574 * self.fRec292[1];
			self.fVec59[(self.IOTA0 & 16383) as usize] = fTemp424 - fTemp423;
			let mut fTemp425: F32 = self.fVec59[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp418) as i32))))) & 16383) as usize];
			self.fVec60[0] = fTemp425;
			self.fRec294[0] = 0.70710677 * (fTemp421 * fTemp425 / fTemp420 + self.fVec60[1]) - self.fRec294[1] * fTemp421 / fTemp420;
			self.fRec292[0] = self.fRec294[0];
			self.fRec297[0] = 0.9999 * (self.fRec297[1] + ((i32::wrapping_mul(iTemp284, iSlow77)) as F32)) + fSlow78;
			let mut fTemp426: F32 = self.fRec297[0] + -1.49999;
			let mut fTemp427: F32 = F32::floor(fTemp426);
			let mut fTemp428: F32 = self.fRec297[0] - fTemp427;
			let mut fTemp429: F32 = fTemp427 + (2.0 - self.fRec297[0]);
			self.fVec61[(self.IOTA0 & 16383) as usize] = fTemp424 + fTemp423;
			let mut iTemp430: i32 = std::cmp::min(8192, std::cmp::max(0, ((fTemp426) as i32)));
			let mut fTemp431: F32 = self.fVec61[((i32::wrapping_sub(self.IOTA0, iTemp430)) & 16383) as usize];
			self.fVec62[0] = fTemp431;
			self.fRec296[0] = 0.70710677 * (fTemp429 * fTemp431 / fTemp428 + self.fVec62[1]) - self.fRec296[1] * fTemp429 / fTemp428;
			self.fRec293[0] = self.fRec296[0];
			let mut fTemp432: F32 = 0.760314 * self.fRec292[1] + 0.64955574 * fTemp417;
			self.fRec301[0] = 0.9999 * (self.fRec301[1] + ((i32::wrapping_mul(iTemp284, iSlow79)) as F32)) + fSlow80;
			let mut fTemp433: F32 = self.fRec301[0] + -1.49999;
			let mut fTemp434: F32 = F32::floor(fTemp433);
			let mut fTemp435: F32 = self.fRec301[0] - fTemp434;
			let mut fTemp436: F32 = fTemp434 + (2.0 - self.fRec301[0]);
			let mut fTemp437: F32 = 0.760314 * self.fRec293[1] + 0.64955574 * fTemp422;
			let mut fTemp438: F32 = 0.760314 * fTemp437 - 0.64955574 * self.fRec299[1];
			let mut fTemp439: F32 = 0.760314 * fTemp432 - 0.64955574 * self.fRec298[1];
			self.fVec63[(self.IOTA0 & 16383) as usize] = fTemp439 - fTemp438;
			let mut iTemp440: i32 = std::cmp::min(8192, std::cmp::max(0, ((fTemp433) as i32)));
			let mut fTemp441: F32 = self.fVec63[((i32::wrapping_sub(self.IOTA0, iTemp440)) & 16383) as usize];
			self.fVec64[0] = fTemp441;
			self.fRec300[0] = 0.70710677 * (fTemp436 * fTemp441 / fTemp435 + self.fVec64[1]) - fTemp436 * self.fRec300[1] / fTemp435;
			self.fRec298[0] = self.fRec300[0];
			self.fRec303[0] = 0.9999 * (self.fRec303[1] + ((i32::wrapping_mul(iTemp284, iSlow81)) as F32)) + fSlow82;
			let mut fTemp442: F32 = self.fRec303[0] + -1.49999;
			let mut fTemp443: F32 = F32::floor(fTemp442);
			let mut fTemp444: F32 = self.fRec303[0] - fTemp443;
			let mut fTemp445: F32 = fTemp443 + (2.0 - self.fRec303[0]);
			self.fVec65[(self.IOTA0 & 16383) as usize] = fTemp439 + fTemp438;
			let mut iTemp446: i32 = std::cmp::min(8192, std::cmp::max(0, ((fTemp442) as i32)));
			let mut fTemp447: F32 = self.fVec65[((i32::wrapping_sub(self.IOTA0, iTemp446)) & 16383) as usize];
			self.fVec66[0] = fTemp447;
			self.fRec302[0] = 0.70710677 * (fTemp445 * fTemp447 / fTemp444 + self.fVec66[1]) - fTemp445 * self.fRec302[1] / fTemp444;
			self.fRec299[0] = self.fRec302[0];
			let mut fTemp448: F32 = 0.760314 * self.fRec298[1] + 0.64955574 * fTemp432;
			self.fRec307[0] = 0.9999 * (self.fRec307[1] + ((i32::wrapping_mul(iTemp284, iSlow83)) as F32)) + fSlow84;
			let mut fTemp449: F32 = self.fRec307[0] + -1.49999;
			let mut fTemp450: F32 = F32::floor(fTemp449);
			let mut fTemp451: F32 = 0.760314 * fTemp448 - 0.64955574 * self.fRec304[1];
			let mut fTemp452: F32 = 0.760314 * self.fRec299[1] + 0.64955574 * fTemp437;
			let mut fTemp453: F32 = 0.760314 * fTemp452 - 0.64955574 * self.fRec305[1];
			self.fVec67[(self.IOTA0 & 16383) as usize] = 0.0 - 0.70710677 * (fTemp453 - fTemp451);
			let mut fTemp454: F32 = self.fVec67[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp449) as i32))))) & 16383) as usize];
			self.fVec68[0] = fTemp454;
			self.fRec306[0] = self.fVec68[1] + (fTemp450 + (2.0 - self.fRec307[0])) * (fTemp454 - self.fRec306[1]) / (self.fRec307[0] - fTemp450);
			self.fRec304[0] = self.fRec306[0];
			self.fRec309[0] = 0.9999 * (self.fRec309[1] + ((i32::wrapping_mul(iTemp284, iSlow85)) as F32)) + fSlow86;
			let mut fTemp455: F32 = self.fRec309[0] + -1.49999;
			let mut fTemp456: F32 = F32::floor(fTemp455);
			let mut fTemp457: F32 = self.fRec309[0] - fTemp456;
			let mut fTemp458: F32 = fTemp456 + (2.0 - self.fRec309[0]);
			self.fVec69[(self.IOTA0 & 16383) as usize] = fTemp451 + fTemp453;
			let mut iTemp459: i32 = std::cmp::min(8192, std::cmp::max(0, ((fTemp455) as i32)));
			let mut fTemp460: F32 = self.fVec69[((i32::wrapping_sub(self.IOTA0, iTemp459)) & 16383) as usize];
			self.fVec70[0] = fTemp460;
			self.fRec308[0] = 0.70710677 * (fTemp458 * fTemp460 / fTemp457 + self.fVec70[1]) - self.fRec308[1] * fTemp458 / fTemp457;
			self.fRec305[0] = self.fRec308[0];
			let mut fTemp461: F32 = 0.760314 * self.fRec304[1] + 0.64955574 * fTemp448;
			self.fVec71[(self.IOTA0 & 16383) as usize] = fTemp461;
			let mut fTemp462: F32 = fSlow63 * (self.fRec276[0] + 1.0);
			let mut fTemp463: F32 = fTemp462 + 3.500005;
			let mut iTemp464: i32 = ((fTemp463) as i32);
			let mut iTemp465: i32 = std::cmp::max(0, i32::wrapping_add(iTemp464, 4));
			let mut fTemp466: F32 = F32::floor(fTemp463);
			let mut fTemp467: F32 = fTemp462 + (2.0 - fTemp466);
			let mut fTemp468: F32 = fTemp462 + (3.0 - fTemp466);
			let mut fTemp469: F32 = fTemp462 + (4.0 - fTemp466);
			let mut fTemp470: F32 = fTemp469 * fTemp468;
			let mut fTemp471: F32 = fTemp470 * fTemp467;
			let mut fTemp472: F32 = fTemp462 + (1.0 - fTemp466);
			let mut fTemp473: F32 = 0.0 - fTemp472;
			let mut iTemp474: i32 = std::cmp::max(0, i32::wrapping_add(iTemp464, 3));
			let mut fTemp475: F32 = 0.0 - 0.5 * fTemp472;
			let mut fTemp476: F32 = 0.0 - fTemp467;
			let mut iTemp477: i32 = std::cmp::max(0, i32::wrapping_add(iTemp464, 2));
			let mut fTemp478: F32 = 0.0 - 0.33333334 * fTemp472;
			let mut fTemp479: F32 = 0.0 - 0.5 * fTemp467;
			let mut fTemp480: F32 = 0.0 - fTemp468;
			let mut iTemp481: i32 = std::cmp::max(0, i32::wrapping_add(iTemp464, 1));
			let mut fTemp482: F32 = fTemp462 + (5.0 - fTemp466);
			let mut fTemp483: F32 = 0.0 - 0.25 * fTemp472;
			let mut fTemp484: F32 = 0.0 - 0.33333334 * fTemp467;
			let mut fTemp485: F32 = 0.0 - 0.5 * fTemp468;
			let mut fTemp486: F32 = 0.0 - fTemp469;
			let mut iTemp487: i32 = std::cmp::max(0, iTemp464);
			self.fVec72[(self.IOTA0 & 16383) as usize] = self.fVec71[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, iTemp487))) & 16383) as usize] * fTemp486 * fTemp485 * fTemp484 * fTemp483 + fTemp482 * (self.fVec71[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, iTemp481))) & 16383) as usize] * fTemp480 * fTemp479 * fTemp478 + 0.5 * fTemp469 * self.fVec71[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, iTemp477))) & 16383) as usize] * fTemp476 * fTemp475 + 0.16666667 * fTemp470 * self.fVec71[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, iTemp474))) & 16383) as usize] * fTemp473 + 0.041666668 * fTemp471 * self.fVec71[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, iTemp465))) & 16383) as usize]);
			let mut fTemp488: F32 = self.fVec72[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp285) as i32))))) & 16383) as usize];
			self.fVec73[0] = fTemp488;
			self.fRec242[0] = self.fVec73[1] + (fTemp286 + (2.0 - self.fRec243[0])) * (fTemp488 - self.fRec242[1]) / (self.fRec243[0] - fTemp286);
			self.fRec241[0] = self.fConst42 * (self.fRec242[0] + self.fRec242[1] - self.fConst39 * self.fRec241[1]);
			self.fRec240[0] = self.fRec241[0] - self.fConst38 * (self.fConst37 * self.fRec240[2] + self.fConst33 * self.fRec240[1]);
			self.fRec239[0] = self.fConst38 * (self.fRec240[2] + self.fRec240[0] + 2.0 * self.fRec240[1]) - self.fConst36 * (self.fConst35 * self.fRec239[2] + self.fConst33 * self.fRec239[1]);
			let mut fTemp489: F32 = self.fRec239[2] + self.fRec239[0] + 2.0 * self.fRec239[1];
			self.fVec74[0] = fTemp489;
			self.fRec238[0] = self.fConst44 * (self.fConst36 * (fTemp489 + self.fVec74[1]) - self.fConst29 * self.fRec238[1]);
			self.fRec237[0] = self.fRec238[0] - self.fConst28 * (self.fConst27 * self.fRec237[2] + self.fConst23 * self.fRec237[1]);
			self.fRec236[0] = self.fConst28 * (self.fRec237[2] + self.fRec237[0] + 2.0 * self.fRec237[1]) - self.fConst26 * (self.fConst25 * self.fRec236[2] + self.fConst23 * self.fRec236[1]);
			self.fRec312[0] = self.fConst36 * (self.fConst47 * self.fVec74[1] + self.fConst46 * fTemp489) - self.fConst45 * self.fRec312[1];
			self.fRec311[0] = self.fRec312[0] - self.fConst28 * (self.fConst27 * self.fRec311[2] + self.fConst23 * self.fRec311[1]);
			self.fRec310[0] = self.fConst28 * (self.fConst48 * self.fRec311[1] + self.fConst22 * self.fRec311[0] + self.fConst22 * self.fRec311[2]) - self.fConst26 * (self.fConst25 * self.fRec310[2] + self.fConst23 * self.fRec310[1]);
			let mut fTemp490: F32 = self.fConst23 * self.fRec313[1];
			self.fRec316[0] = self.fConst51 * self.fRec242[1] + self.fConst42 * (self.fConst34 * self.fRec242[0] - self.fConst39 * self.fRec316[1]);
			self.fRec315[0] = self.fRec316[0] - self.fConst38 * (self.fConst37 * self.fRec315[2] + self.fConst33 * self.fRec315[1]);
			self.fRec314[0] = self.fConst38 * (self.fConst52 * self.fRec315[1] + self.fConst32 * self.fRec315[0] + self.fConst32 * self.fRec315[2]) - self.fConst36 * (self.fConst35 * self.fRec314[2] + self.fConst33 * self.fRec314[1]);
			self.fRec313[0] = self.fConst36 * (self.fConst52 * self.fRec314[1] + self.fConst32 * self.fRec314[0] + self.fConst32 * self.fRec314[2]) - self.fConst50 * (self.fConst49 * self.fRec313[2] + fTemp490);
			let mut fTemp491: F32 = fTemp283 + fSlow87 * (self.fRec313[2] + self.fConst50 * (fTemp490 + self.fConst49 * self.fRec313[0]) + self.fConst26 * (self.fConst22 * self.fRec310[0] + self.fConst48 * self.fRec310[1] + self.fConst22 * self.fRec310[2] + self.fRec236[2] + self.fRec236[0] + 2.0 * self.fRec236[1])) + fTemp282;
			self.fVec75[(self.IOTA0 & 1023) as usize] = fTemp491;
			self.fRec235[0] = fSlow88 * self.fRec235[1] + fSlow89 * (fTemp486 * fTemp485 * fTemp484 * fTemp483 * self.fVec75[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, iTemp487))) & 1023) as usize] + fTemp482 * (fTemp480 * fTemp479 * fTemp478 * self.fVec75[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, iTemp481))) & 1023) as usize] + 0.5 * fTemp469 * fTemp476 * fTemp475 * self.fVec75[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, iTemp477))) & 1023) as usize] + 0.16666667 * fTemp470 * fTemp473 * self.fVec75[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, iTemp474))) & 1023) as usize] + 0.041666668 * fTemp471 * self.fVec75[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, iTemp465))) & 1023) as usize]));
			self.fRec328[0] = 0.995 * (self.fRec328[1] + ((i32::wrapping_mul(iTemp284, iSlow92)) as F32)) + fSlow93;
			let mut fTemp492: F32 = self.fRec328[0] + -1.49999;
			let mut fTemp493: F32 = F32::floor(fTemp492);
			let mut fTemp494: F32 = 0.760314 * self.fRec305[1] + 0.64955574 * fTemp452;
			self.fVec76[(self.IOTA0 & 16383) as usize] = fTemp494;
			let mut fTemp495: F32 = fSlow68 * self.fRec276[0];
			let mut fTemp496: F32 = fSlow63 + fTemp495 + 3.500005;
			let mut iTemp497: i32 = ((fTemp496) as i32);
			let mut fTemp498: F32 = F32::floor(fTemp496);
			let mut fTemp499: F32 = fSlow63 + fTemp495 + (2.0 - fTemp498);
			let mut fTemp500: F32 = fSlow63 + fTemp495 + (3.0 - fTemp498);
			let mut fTemp501: F32 = fSlow63 + fTemp495 + (4.0 - fTemp498);
			let mut fTemp502: F32 = fTemp501 * fTemp500;
			let mut fTemp503: F32 = fSlow63 + fTemp495 + (1.0 - fTemp498);
			self.fVec77[(self.IOTA0 & 16383) as usize] = self.fVec76[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, iTemp497)))) & 16383) as usize] * (0.0 - fTemp501) * (0.0 - 0.5 * fTemp500) * (0.0 - 0.33333334 * fTemp499) * (0.0 - 0.25 * fTemp503) + (fSlow63 + fTemp495 + (5.0 - fTemp498)) * (self.fVec76[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, i32::wrapping_add(iTemp497, 1))))) & 16383) as usize] * (0.0 - fTemp500) * (0.0 - 0.5 * fTemp499) * (0.0 - 0.33333334 * fTemp503) + 0.5 * fTemp501 * self.fVec76[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, i32::wrapping_add(iTemp497, 2))))) & 16383) as usize] * (0.0 - fTemp499) * (0.0 - 0.5 * fTemp503) + 0.16666667 * fTemp502 * self.fVec76[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, i32::wrapping_add(iTemp497, 3))))) & 16383) as usize] * (0.0 - fTemp503) + 0.041666668 * fTemp502 * fTemp499 * self.fVec76[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, i32::wrapping_add(iTemp497, 4))))) & 16383) as usize]);
			let mut fTemp504: F32 = self.fVec77[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp492) as i32))))) & 16383) as usize];
			self.fVec78[0] = fTemp504;
			self.fRec327[0] = self.fVec78[1] - (fTemp493 + (2.0 - self.fRec328[0])) * (self.fRec327[1] - fTemp504) / (self.fRec328[0] - fTemp493);
			self.fRec326[0] = self.fConst42 * (self.fRec327[0] + self.fRec327[1] - self.fConst39 * self.fRec326[1]);
			self.fRec325[0] = self.fRec326[0] - self.fConst38 * (self.fConst37 * self.fRec325[2] + self.fConst33 * self.fRec325[1]);
			self.fRec324[0] = self.fConst38 * (self.fRec325[2] + self.fRec325[0] + 2.0 * self.fRec325[1]) - self.fConst36 * (self.fConst35 * self.fRec324[2] + self.fConst33 * self.fRec324[1]);
			let mut fTemp505: F32 = self.fRec324[2] + self.fRec324[0] + 2.0 * self.fRec324[1];
			self.fVec79[0] = fTemp505;
			self.fRec323[0] = self.fConst44 * (self.fConst36 * (fTemp505 + self.fVec79[1]) - self.fConst29 * self.fRec323[1]);
			self.fRec322[0] = self.fRec323[0] - self.fConst28 * (self.fConst27 * self.fRec322[2] + self.fConst23 * self.fRec322[1]);
			self.fRec321[0] = self.fConst28 * (self.fRec322[2] + self.fRec322[0] + 2.0 * self.fRec322[1]) - self.fConst26 * (self.fConst25 * self.fRec321[2] + self.fConst23 * self.fRec321[1]);
			self.fRec331[0] = self.fConst36 * (self.fConst46 * fTemp505 + self.fConst47 * self.fVec79[1]) - self.fConst45 * self.fRec331[1];
			self.fRec330[0] = self.fRec331[0] - self.fConst28 * (self.fConst27 * self.fRec330[2] + self.fConst23 * self.fRec330[1]);
			self.fRec329[0] = self.fConst28 * (self.fConst22 * self.fRec330[0] + self.fConst48 * self.fRec330[1] + self.fConst22 * self.fRec330[2]) - self.fConst26 * (self.fConst25 * self.fRec329[2] + self.fConst23 * self.fRec329[1]);
			let mut fTemp506: F32 = self.fConst23 * self.fRec332[1];
			self.fRec335[0] = self.fConst51 * self.fRec327[1] - self.fConst42 * (self.fConst39 * self.fRec335[1] - self.fConst34 * self.fRec327[0]);
			self.fRec334[0] = self.fRec335[0] - self.fConst38 * (self.fConst37 * self.fRec334[2] + self.fConst33 * self.fRec334[1]);
			self.fRec333[0] = self.fConst38 * (self.fConst32 * self.fRec334[0] + self.fConst52 * self.fRec334[1] + self.fConst32 * self.fRec334[2]) - self.fConst36 * (self.fConst35 * self.fRec333[2] + self.fConst33 * self.fRec333[1]);
			self.fRec332[0] = self.fConst36 * (self.fConst52 * self.fRec333[1] + self.fConst32 * self.fRec333[0] + self.fConst32 * self.fRec333[2]) - self.fConst50 * (self.fConst49 * self.fRec332[2] + fTemp506);
			let mut fTemp507: F32 = fTemp283 + fTemp282 + fSlow87 * (self.fRec332[2] + self.fConst50 * (fTemp506 + self.fConst49 * self.fRec332[0]) + self.fConst26 * (self.fConst48 * self.fRec329[1] + self.fConst22 * self.fRec329[0] + self.fConst22 * self.fRec329[2] + self.fRec321[2] + self.fRec321[0] + 2.0 * self.fRec321[1]));
			self.fVec80[(self.IOTA0 & 1023) as usize] = fTemp507;
			self.fRec320[0] = fSlow88 * self.fRec320[1] + fSlow89 * (fTemp376 * fTemp375 * fTemp374 * fTemp373 * self.fVec80[((i32::wrapping_sub(self.IOTA0, iTemp377)) & 1023) as usize] + fTemp372 * (fTemp370 * fTemp369 * fTemp368 * self.fVec80[((i32::wrapping_sub(self.IOTA0, iTemp371)) & 1023) as usize] + 0.5 * fTemp359 * fTemp366 * fTemp365 * self.fVec80[((i32::wrapping_sub(self.IOTA0, iTemp367)) & 1023) as usize] + 0.16666667 * fTemp360 * fTemp363 * self.fVec80[((i32::wrapping_sub(self.IOTA0, iTemp364)) & 1023) as usize] + 0.041666668 * fTemp361 * self.fVec80[((i32::wrapping_sub(self.IOTA0, iTemp355)) & 1023) as usize]));
			let mut fTemp508: F32 = fSlow94 * self.fRec320[0] - fSlow91 * self.fRec318[1];
			let mut fTemp509: F32 = fSlow94 * self.fRec235[0] - fSlow91 * self.fRec317[1];
			self.fVec81[(self.IOTA0 & 16383) as usize] = fTemp509 - fTemp508;
			let mut fTemp510: F32 = self.fVec81[((i32::wrapping_sub(self.IOTA0, iTemp415)) & 16383) as usize];
			self.fVec82[0] = fTemp510;
			self.fRec319[0] = 0.70710677 * (fTemp414 * fTemp510 / fTemp413 + self.fVec82[1]) - self.fRec319[1] * fTemp414 / fTemp413;
			self.fRec317[0] = self.fRec319[0];
			self.fRec337[0] = 0.9999 * (self.fRec337[1] + ((i32::wrapping_mul(iTemp284, iSlow95)) as F32)) + fSlow96;
			let mut fTemp511: F32 = self.fRec337[0] + -1.49999;
			let mut fTemp512: F32 = F32::floor(fTemp511);
			let mut fTemp513: F32 = self.fRec337[0] - fTemp512;
			let mut fTemp514: F32 = fTemp512 + (2.0 - self.fRec337[0]);
			self.fVec83[(self.IOTA0 & 16383) as usize] = fTemp509 + fTemp508;
			let mut fTemp515: F32 = self.fVec83[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp511) as i32))))) & 16383) as usize];
			self.fVec84[0] = fTemp515;
			self.fRec336[0] = 0.70710677 * (fTemp514 * fTemp515 / fTemp513 + self.fVec84[1]) - self.fRec336[1] * fTemp514 / fTemp513;
			self.fRec318[0] = self.fRec336[0];
			let mut fTemp516: F32 = fSlow94 * self.fRec317[1] + fSlow91 * self.fRec235[0];
			let mut fTemp517: F32 = fSlow94 * self.fRec318[1] + fSlow91 * self.fRec320[0];
			let mut fTemp518: F32 = fSlow94 * fTemp517 - fSlow91 * self.fRec339[1];
			let mut fTemp519: F32 = fSlow94 * fTemp516 - fSlow91 * self.fRec338[1];
			self.fVec85[(self.IOTA0 & 16383) as usize] = fTemp519 - fTemp518;
			let mut fTemp520: F32 = self.fVec85[((i32::wrapping_sub(self.IOTA0, iTemp440)) & 16383) as usize];
			self.fVec86[0] = fTemp520;
			self.fRec340[0] = 0.70710677 * (fTemp436 * fTemp520 / fTemp435 + self.fVec86[1]) - self.fRec340[1] * fTemp436 / fTemp435;
			self.fRec338[0] = self.fRec340[0];
			self.fVec87[(self.IOTA0 & 16383) as usize] = fTemp519 + fTemp518;
			let mut fTemp521: F32 = self.fVec87[((i32::wrapping_sub(self.IOTA0, iTemp430)) & 16383) as usize];
			self.fVec88[0] = fTemp521;
			self.fRec341[0] = 0.70710677 * (fTemp429 * fTemp521 / fTemp428 + self.fVec88[1]) - fTemp429 * self.fRec341[1] / fTemp428;
			self.fRec339[0] = self.fRec341[0];
			let mut fTemp522: F32 = fSlow94 * self.fRec338[1] + fSlow91 * fTemp516;
			let mut fTemp523: F32 = fSlow94 * fTemp522 - fSlow91 * self.fRec342[1];
			let mut fTemp524: F32 = fSlow94 * self.fRec339[1] + fSlow91 * fTemp517;
			let mut fTemp525: F32 = fSlow94 * fTemp524 - fSlow91 * self.fRec343[1];
			self.fVec89[(self.IOTA0 & 16383) as usize] = 0.0 - 0.70710677 * (fTemp525 - fTemp523);
			let mut fTemp526: F32 = self.fVec89[((i32::wrapping_sub(self.IOTA0, iTemp446)) & 16383) as usize];
			self.fVec90[0] = fTemp526;
			self.fRec344[0] = self.fVec90[1] + fTemp445 * (fTemp526 - self.fRec344[1]) / fTemp444;
			self.fRec342[0] = self.fRec344[0];
			self.fRec346[0] = 0.9999 * (self.fRec346[1] + ((i32::wrapping_mul(iTemp284, iSlow97)) as F32)) + fSlow98;
			let mut fTemp527: F32 = self.fRec346[0] + -1.49999;
			let mut fTemp528: F32 = F32::floor(fTemp527);
			let mut fTemp529: F32 = self.fRec346[0] - fTemp528;
			let mut fTemp530: F32 = fTemp528 + (2.0 - self.fRec346[0]);
			self.fVec91[(self.IOTA0 & 16383) as usize] = fTemp523 + fTemp525;
			let mut fTemp531: F32 = self.fVec91[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp527) as i32))))) & 16383) as usize];
			self.fVec92[0] = fTemp531;
			self.fRec345[0] = 0.70710677 * (fTemp530 * fTemp531 / fTemp529 + self.fVec92[1]) - self.fRec345[1] * fTemp530 / fTemp529;
			self.fRec343[0] = self.fRec345[0];
			let mut fTemp532: F32 = fSlow94 * self.fRec342[1] + fSlow91 * fTemp522;
			self.fRec350[0] = 0.9999 * (self.fRec350[1] + ((i32::wrapping_mul(iTemp284, iSlow99)) as F32)) + fSlow100;
			let mut fTemp533: F32 = self.fRec350[0] + -1.49999;
			let mut fTemp534: F32 = F32::floor(fTemp533);
			let mut fTemp535: F32 = self.fRec350[0] - fTemp534;
			let mut fTemp536: F32 = fTemp534 + (2.0 - self.fRec350[0]);
			let mut fTemp537: F32 = fSlow94 * self.fRec343[1] + fSlow91 * fTemp524;
			let mut fTemp538: F32 = fSlow94 * fTemp537 - fSlow91 * self.fRec348[1];
			let mut fTemp539: F32 = fSlow94 * fTemp532 - fSlow91 * self.fRec347[1];
			self.fVec93[(self.IOTA0 & 16383) as usize] = fTemp539 - fTemp538;
			let mut fTemp540: F32 = self.fVec93[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp533) as i32))))) & 16383) as usize];
			self.fVec94[0] = fTemp540;
			self.fRec349[0] = 0.70710677 * (fTemp536 * fTemp540 / fTemp535 + self.fVec94[1]) - self.fRec349[1] * fTemp536 / fTemp535;
			self.fRec347[0] = self.fRec349[0];
			self.fVec95[(self.IOTA0 & 16383) as usize] = fTemp539 + fTemp538;
			let mut fTemp541: F32 = self.fVec95[((i32::wrapping_sub(self.IOTA0, iTemp459)) & 16383) as usize];
			self.fVec96[0] = fTemp541;
			self.fRec351[0] = 0.70710677 * (fTemp458 * fTemp541 / fTemp457 + self.fVec96[1]) - fTemp458 * self.fRec351[1] / fTemp457;
			self.fRec348[0] = self.fRec351[0];
			self.fRec233[0] = fSlow94 * self.fRec347[1] + fSlow91 * fTemp532;
			self.fRec234[0] = fSlow94 * self.fRec348[1] + fSlow91 * fTemp537;
			self.fRec352[0] = fSlow101 + self.fConst2 * self.fRec352[1];
			let mut fTemp542: F32 = self.fRec352[0] * (fSlow33 * (self.fRec233[0] + self.fRec234[0]) + fSlow34 * (fTemp283 + fTemp282));
			*output0 = fTemp542;
			*output1 = fTemp542;
			self.iVec0[1] = self.iVec0[0];
			self.fRec0[1] = self.fRec0[0];
			self.fRec2[1] = self.fRec2[0];
			self.fRec1[1] = self.fRec1[0];
			self.fRec7[1] = self.fRec7[0];
			self.fRec5[1] = self.fRec5[0];
			self.fRec8[1] = self.fRec8[0];
			self.fRec4[2] = self.fRec4[1];
			self.fRec4[1] = self.fRec4[0];
			self.fRec3[2] = self.fRec3[1];
			self.fRec3[1] = self.fRec3[0];
			self.fRec9[1] = self.fRec9[0];
			self.fRec14[1] = self.fRec14[0];
			self.fRec12[1] = self.fRec12[0];
			self.fRec15[1] = self.fRec15[0];
			self.fRec11[2] = self.fRec11[1];
			self.fRec11[1] = self.fRec11[0];
			self.fRec10[2] = self.fRec10[1];
			self.fRec10[1] = self.fRec10[0];
			self.fRec16[1] = self.fRec16[0];
			self.fRec21[1] = self.fRec21[0];
			self.fRec19[1] = self.fRec19[0];
			self.fRec22[1] = self.fRec22[0];
			self.fRec18[2] = self.fRec18[1];
			self.fRec18[1] = self.fRec18[0];
			self.fRec17[2] = self.fRec17[1];
			self.fRec17[1] = self.fRec17[0];
			self.fRec23[1] = self.fRec23[0];
			self.fRec28[1] = self.fRec28[0];
			self.fRec26[1] = self.fRec26[0];
			self.fRec29[1] = self.fRec29[0];
			self.fRec25[2] = self.fRec25[1];
			self.fRec25[1] = self.fRec25[0];
			self.fRec24[2] = self.fRec24[1];
			self.fRec24[1] = self.fRec24[0];
			self.fRec30[1] = self.fRec30[0];
			self.fRec31[1] = self.fRec31[0];
			self.fRec32[1] = self.fRec32[0];
			self.fRec34[1] = self.fRec34[0];
			self.fVec1[1] = self.fVec1[0];
			self.IOTA0 = i32::wrapping_add(self.IOTA0, 1);
			self.fRec33[1] = self.fRec33[0];
			self.fRec36[1] = self.fRec36[0];
			self.fVec3[1] = self.fVec3[0];
			self.fRec35[1] = self.fRec35[0];
			self.fRec38[1] = self.fRec38[0];
			self.fVec5[1] = self.fVec5[0];
			self.fRec37[1] = self.fRec37[0];
			self.fRec40[1] = self.fRec40[0];
			self.fVec7[1] = self.fVec7[0];
			self.fRec39[1] = self.fRec39[0];
			self.fRec41[1] = self.fRec41[0];
			self.fRec42[1] = self.fRec42[0];
			self.fRec43[1] = self.fRec43[0];
			self.fRec45[1] = self.fRec45[0];
			self.fRec46[1] = self.fRec46[0];
			self.fRec76[1] = self.fRec76[0];
			self.fRec80[1] = self.fRec80[0];
			self.fRec85[1] = self.fRec85[0];
			self.iVec9[1] = self.iVec9[0];
			self.iRec86[1] = self.iRec86[0];
			self.fRec83[1] = self.fRec83[0];
			self.fRec82[1] = self.fRec82[0];
			for j0 in (1..=3).rev() {
				self.fRec87[(j0) as usize] = self.fRec87[(i32::wrapping_sub(j0, 1)) as usize];
			}
			self.fVec10[1] = self.fVec10[0];
			self.fVec11[1] = self.fVec11[0];
			self.iRec89[1] = self.iRec89[0];
			self.iRec91[1] = self.iRec91[0];
			self.fRec90[2] = self.fRec90[1];
			self.fRec90[1] = self.fRec90[0];
			self.fVec12[2] = self.fVec12[1];
			self.fVec12[1] = self.fVec12[0];
			self.fRec72[1] = self.fRec72[0];
			self.fRec68[1] = self.fRec68[0];
			self.fRec66[1] = self.fRec66[0];
			for j1 in (1..=3).rev() {
				self.fRec62[(j1) as usize] = self.fRec62[(i32::wrapping_sub(j1, 1)) as usize];
			}
			self.fRec57[1] = self.fRec57[0];
			self.fRec51[1] = self.fRec51[0];
			self.fRec50[1] = self.fRec50[0];
			self.fRec49[1] = self.fRec49[0];
			self.fRec47[1] = self.fRec47[0];
			self.fRec44[1] = self.fRec44[0];
			self.fRec93[1] = self.fRec93[0];
			self.fRec123[1] = self.fRec123[0];
			self.fRec127[1] = self.fRec127[0];
			self.fRec132[1] = self.fRec132[0];
			self.iVec13[1] = self.iVec13[0];
			self.iRec133[1] = self.iRec133[0];
			self.fRec130[1] = self.fRec130[0];
			self.fRec129[1] = self.fRec129[0];
			for j2 in (1..=3).rev() {
				self.fRec134[(j2) as usize] = self.fRec134[(i32::wrapping_sub(j2, 1)) as usize];
			}
			self.fVec14[1] = self.fVec14[0];
			self.fVec15[1] = self.fVec15[0];
			self.iRec136[1] = self.iRec136[0];
			self.fRec137[2] = self.fRec137[1];
			self.fRec137[1] = self.fRec137[0];
			self.fVec16[2] = self.fVec16[1];
			self.fVec16[1] = self.fVec16[0];
			self.fRec119[1] = self.fRec119[0];
			self.fRec115[1] = self.fRec115[0];
			self.fRec113[1] = self.fRec113[0];
			for j3 in (1..=3).rev() {
				self.fRec109[(j3) as usize] = self.fRec109[(i32::wrapping_sub(j3, 1)) as usize];
			}
			self.fRec104[1] = self.fRec104[0];
			self.fRec98[1] = self.fRec98[0];
			self.fRec97[1] = self.fRec97[0];
			self.fRec96[1] = self.fRec96[0];
			self.fRec94[1] = self.fRec94[0];
			self.fRec92[1] = self.fRec92[0];
			self.fRec139[1] = self.fRec139[0];
			self.fRec169[1] = self.fRec169[0];
			self.fRec173[1] = self.fRec173[0];
			self.fRec178[1] = self.fRec178[0];
			self.iVec17[1] = self.iVec17[0];
			self.iRec179[1] = self.iRec179[0];
			self.fRec176[1] = self.fRec176[0];
			self.fRec175[1] = self.fRec175[0];
			for j4 in (1..=3).rev() {
				self.fRec180[(j4) as usize] = self.fRec180[(i32::wrapping_sub(j4, 1)) as usize];
			}
			self.fVec18[1] = self.fVec18[0];
			self.fVec19[1] = self.fVec19[0];
			self.iRec182[1] = self.iRec182[0];
			self.fRec183[2] = self.fRec183[1];
			self.fRec183[1] = self.fRec183[0];
			self.fVec20[2] = self.fVec20[1];
			self.fVec20[1] = self.fVec20[0];
			self.fRec165[1] = self.fRec165[0];
			self.fRec161[1] = self.fRec161[0];
			self.fRec159[1] = self.fRec159[0];
			for j5 in (1..=3).rev() {
				self.fRec155[(j5) as usize] = self.fRec155[(i32::wrapping_sub(j5, 1)) as usize];
			}
			self.fRec150[1] = self.fRec150[0];
			self.fRec144[1] = self.fRec144[0];
			self.fRec143[1] = self.fRec143[0];
			self.fRec142[1] = self.fRec142[0];
			self.fRec140[1] = self.fRec140[0];
			self.fRec138[1] = self.fRec138[0];
			self.fRec185[1] = self.fRec185[0];
			self.fRec215[1] = self.fRec215[0];
			self.fRec219[1] = self.fRec219[0];
			self.fRec224[1] = self.fRec224[0];
			self.iVec21[1] = self.iVec21[0];
			self.iRec225[1] = self.iRec225[0];
			self.fRec222[1] = self.fRec222[0];
			self.fRec221[1] = self.fRec221[0];
			for j6 in (1..=3).rev() {
				self.fRec226[(j6) as usize] = self.fRec226[(i32::wrapping_sub(j6, 1)) as usize];
			}
			self.fVec22[1] = self.fVec22[0];
			self.fVec23[1] = self.fVec23[0];
			self.iRec228[1] = self.iRec228[0];
			self.fRec229[2] = self.fRec229[1];
			self.fRec229[1] = self.fRec229[0];
			self.fVec24[2] = self.fVec24[1];
			self.fVec24[1] = self.fVec24[0];
			self.fRec211[1] = self.fRec211[0];
			self.fRec207[1] = self.fRec207[0];
			self.fRec205[1] = self.fRec205[0];
			for j7 in (1..=3).rev() {
				self.fRec201[(j7) as usize] = self.fRec201[(i32::wrapping_sub(j7, 1)) as usize];
			}
			self.fRec196[1] = self.fRec196[0];
			self.fRec190[1] = self.fRec190[0];
			self.fRec189[1] = self.fRec189[0];
			self.fRec188[1] = self.fRec188[0];
			self.fRec186[1] = self.fRec186[0];
			self.fRec184[1] = self.fRec184[0];
			self.fRec230[1] = self.fRec230[0];
			self.fRec232[1] = self.fRec232[0];
			self.fRec243[1] = self.fRec243[0];
			self.fRec245[1] = self.fRec245[0];
			self.fRec249[1] = self.fRec249[0];
			self.fVec26[1] = self.fVec26[0];
			self.fRec248[1] = self.fRec248[0];
			self.fRec246[1] = self.fRec246[0];
			self.fRec251[1] = self.fRec251[0];
			self.fVec28[1] = self.fVec28[0];
			self.fRec250[1] = self.fRec250[0];
			self.fRec247[1] = self.fRec247[0];
			self.fRec255[1] = self.fRec255[0];
			self.fVec30[1] = self.fVec30[0];
			self.fRec254[1] = self.fRec254[0];
			self.fRec252[1] = self.fRec252[0];
			self.fRec257[1] = self.fRec257[0];
			self.fVec32[1] = self.fVec32[0];
			self.fRec256[1] = self.fRec256[0];
			self.fRec253[1] = self.fRec253[0];
			self.fRec261[1] = self.fRec261[0];
			self.fVec34[1] = self.fVec34[0];
			self.fRec260[1] = self.fRec260[0];
			self.fRec258[1] = self.fRec258[0];
			self.fRec263[1] = self.fRec263[0];
			self.fVec36[1] = self.fVec36[0];
			self.fRec262[1] = self.fRec262[0];
			self.fRec259[1] = self.fRec259[0];
			self.fRec267[1] = self.fRec267[0];
			self.fVec38[1] = self.fVec38[0];
			self.fRec266[1] = self.fRec266[0];
			self.fRec264[1] = self.fRec264[0];
			self.fRec269[1] = self.fRec269[0];
			self.fVec40[1] = self.fVec40[0];
			self.fRec268[1] = self.fRec268[0];
			self.fRec265[1] = self.fRec265[0];
			self.fRec273[1] = self.fRec273[0];
			self.fVec42[1] = self.fVec42[0];
			self.fRec272[1] = self.fRec272[0];
			self.fRec270[1] = self.fRec270[0];
			self.fRec275[1] = self.fRec275[0];
			self.fVec44[1] = self.fVec44[0];
			self.fRec274[1] = self.fRec274[0];
			self.fRec271[1] = self.fRec271[0];
			self.fRec276[1] = self.fRec276[0];
			self.fRec277[1] = self.fRec277[0];
			self.fVec47[1] = self.fVec47[0];
			self.fRec244[1] = self.fRec244[0];
			self.fRec281[1] = self.fRec281[0];
			self.fRec283[1] = self.fRec283[0];
			self.fVec50[1] = self.fVec50[0];
			self.fRec282[1] = self.fRec282[0];
			self.fVec52[1] = self.fVec52[0];
			self.fRec280[1] = self.fRec280[0];
			self.fRec278[1] = self.fRec278[0];
			self.fRec285[1] = self.fRec285[0];
			self.fVec54[1] = self.fVec54[0];
			self.fRec284[1] = self.fRec284[0];
			self.fRec279[1] = self.fRec279[0];
			self.fRec289[1] = self.fRec289[0];
			self.fVec56[1] = self.fVec56[0];
			self.fRec288[1] = self.fRec288[0];
			self.fRec286[1] = self.fRec286[0];
			self.fRec291[1] = self.fRec291[0];
			self.fVec58[1] = self.fVec58[0];
			self.fRec290[1] = self.fRec290[0];
			self.fRec287[1] = self.fRec287[0];
			self.fRec295[1] = self.fRec295[0];
			self.fVec60[1] = self.fVec60[0];
			self.fRec294[1] = self.fRec294[0];
			self.fRec292[1] = self.fRec292[0];
			self.fRec297[1] = self.fRec297[0];
			self.fVec62[1] = self.fVec62[0];
			self.fRec296[1] = self.fRec296[0];
			self.fRec293[1] = self.fRec293[0];
			self.fRec301[1] = self.fRec301[0];
			self.fVec64[1] = self.fVec64[0];
			self.fRec300[1] = self.fRec300[0];
			self.fRec298[1] = self.fRec298[0];
			self.fRec303[1] = self.fRec303[0];
			self.fVec66[1] = self.fVec66[0];
			self.fRec302[1] = self.fRec302[0];
			self.fRec299[1] = self.fRec299[0];
			self.fRec307[1] = self.fRec307[0];
			self.fVec68[1] = self.fVec68[0];
			self.fRec306[1] = self.fRec306[0];
			self.fRec304[1] = self.fRec304[0];
			self.fRec309[1] = self.fRec309[0];
			self.fVec70[1] = self.fVec70[0];
			self.fRec308[1] = self.fRec308[0];
			self.fRec305[1] = self.fRec305[0];
			self.fVec73[1] = self.fVec73[0];
			self.fRec242[1] = self.fRec242[0];
			self.fRec241[1] = self.fRec241[0];
			self.fRec240[2] = self.fRec240[1];
			self.fRec240[1] = self.fRec240[0];
			self.fRec239[2] = self.fRec239[1];
			self.fRec239[1] = self.fRec239[0];
			self.fVec74[1] = self.fVec74[0];
			self.fRec238[1] = self.fRec238[0];
			self.fRec237[2] = self.fRec237[1];
			self.fRec237[1] = self.fRec237[0];
			self.fRec236[2] = self.fRec236[1];
			self.fRec236[1] = self.fRec236[0];
			self.fRec312[1] = self.fRec312[0];
			self.fRec311[2] = self.fRec311[1];
			self.fRec311[1] = self.fRec311[0];
			self.fRec310[2] = self.fRec310[1];
			self.fRec310[1] = self.fRec310[0];
			self.fRec316[1] = self.fRec316[0];
			self.fRec315[2] = self.fRec315[1];
			self.fRec315[1] = self.fRec315[0];
			self.fRec314[2] = self.fRec314[1];
			self.fRec314[1] = self.fRec314[0];
			self.fRec313[2] = self.fRec313[1];
			self.fRec313[1] = self.fRec313[0];
			self.fRec235[1] = self.fRec235[0];
			self.fRec328[1] = self.fRec328[0];
			self.fVec78[1] = self.fVec78[0];
			self.fRec327[1] = self.fRec327[0];
			self.fRec326[1] = self.fRec326[0];
			self.fRec325[2] = self.fRec325[1];
			self.fRec325[1] = self.fRec325[0];
			self.fRec324[2] = self.fRec324[1];
			self.fRec324[1] = self.fRec324[0];
			self.fVec79[1] = self.fVec79[0];
			self.fRec323[1] = self.fRec323[0];
			self.fRec322[2] = self.fRec322[1];
			self.fRec322[1] = self.fRec322[0];
			self.fRec321[2] = self.fRec321[1];
			self.fRec321[1] = self.fRec321[0];
			self.fRec331[1] = self.fRec331[0];
			self.fRec330[2] = self.fRec330[1];
			self.fRec330[1] = self.fRec330[0];
			self.fRec329[2] = self.fRec329[1];
			self.fRec329[1] = self.fRec329[0];
			self.fRec335[1] = self.fRec335[0];
			self.fRec334[2] = self.fRec334[1];
			self.fRec334[1] = self.fRec334[0];
			self.fRec333[2] = self.fRec333[1];
			self.fRec333[1] = self.fRec333[0];
			self.fRec332[2] = self.fRec332[1];
			self.fRec332[1] = self.fRec332[0];
			self.fRec320[1] = self.fRec320[0];
			self.fVec82[1] = self.fVec82[0];
			self.fRec319[1] = self.fRec319[0];
			self.fRec317[1] = self.fRec317[0];
			self.fRec337[1] = self.fRec337[0];
			self.fVec84[1] = self.fVec84[0];
			self.fRec336[1] = self.fRec336[0];
			self.fRec318[1] = self.fRec318[0];
			self.fVec86[1] = self.fVec86[0];
			self.fRec340[1] = self.fRec340[0];
			self.fRec338[1] = self.fRec338[0];
			self.fVec88[1] = self.fVec88[0];
			self.fRec341[1] = self.fRec341[0];
			self.fRec339[1] = self.fRec339[0];
			self.fVec90[1] = self.fVec90[0];
			self.fRec344[1] = self.fRec344[0];
			self.fRec342[1] = self.fRec342[0];
			self.fRec346[1] = self.fRec346[0];
			self.fVec92[1] = self.fVec92[0];
			self.fRec345[1] = self.fRec345[0];
			self.fRec343[1] = self.fRec343[0];
			self.fRec350[1] = self.fRec350[0];
			self.fVec94[1] = self.fVec94[0];
			self.fRec349[1] = self.fRec349[0];
			self.fRec347[1] = self.fRec347[0];
			self.fVec96[1] = self.fVec96[0];
			self.fRec351[1] = self.fRec351[0];
			self.fRec348[1] = self.fRec348[0];
			self.fRec233[1] = self.fRec233[0];
			self.fRec234[1] = self.fRec234[0];
			self.fRec352[1] = self.fRec352[0];
		}
	}

}


}
pub use dsp::mydsp as Instrument;
