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
	fButton0: F32,
	fVec0: [F32;2],
	iVec1: [i32;2],
	fHslider0: F32,
	fRec2: [F32;2],
	fSampleRate: i32,
	fConst0: F32,
	fConst1: F32,
	fConst2: F32,
	fRec1: [F32;2],
	fHslider1: F32,
	fRec3: [F32;2],
	fConst3: F32,
	IOTA0: i32,
	fConst4: F32,
	fConst5: F32,
	fRec33: [F32;2],
	fRec37: [F32;2],
	fConst6: F32,
	fConst7: F32,
	iRec39: [i32;2],
	iRec41: [i32;2],
	fRec40: [F32;3],
	fConst8: F32,
	fConst9: F32,
	fRec45: [F32;2],
	iVec2: [i32;2],
	iConst10: i32,
	iRec46: [i32;2],
	fConst11: F32,
	fRec43: [F32;2],
	fRec42: [F32;2],
	fRec47: [F32;4],
	fRec48: [F32;2048],
	fVec3: [F32;2],
	fVec4: [F32;3],
	fRec38: [F32;2048],
	fRec29: [F32;2],
	fRec25: [F32;2],
	fRec21: [F32;2048],
	fRec23: [F32;2],
	fHslider2: F32,
	fRec19: [F32;4],
	fRec14: [F32;2],
	fRec10: [F32;2048],
	fRec8: [F32;2],
	fConst12: F32,
	fRec7: [F32;2],
	fRec6: [F32;2],
	fRec4: [F32;2],
	fRec0: [F32;2],
	fHslider3: F32,
	fRec50: [F32;2],
	fRec80: [F32;2],
	fRec84: [F32;2],
	fRec89: [F32;2],
	iVec5: [i32;2],
	iRec90: [i32;2],
	fRec87: [F32;2],
	fRec86: [F32;2],
	fRec91: [F32;4],
	fRec92: [F32;2048],
	fVec6: [F32;2],
	fButton1: F32,
	fVec7: [F32;2],
	iRec93: [i32;2],
	fRec94: [F32;3],
	fVec8: [F32;3],
	fRec85: [F32;2048],
	fRec76: [F32;2],
	fRec72: [F32;2],
	fRec68: [F32;2048],
	fRec70: [F32;2],
	fRec66: [F32;4],
	fRec61: [F32;2],
	fRec57: [F32;2048],
	fRec55: [F32;2],
	fRec54: [F32;2],
	fRec53: [F32;2],
	fRec51: [F32;2],
	fRec49: [F32;2],
	fButton2: F32,
	fVec9: [F32;2],
	fHslider4: F32,
	fRec97: [F32;2],
	fRec96: [F32;2],
	fRec127: [F32;2],
	fRec131: [F32;2],
	fRec136: [F32;2],
	iVec10: [i32;2],
	iRec137: [i32;2],
	fRec134: [F32;2],
	fRec133: [F32;2],
	fRec138: [F32;4],
	fRec139: [F32;2048],
	fVec11: [F32;2],
	iRec140: [i32;2],
	fRec141: [F32;3],
	fVec12: [F32;3],
	fRec132: [F32;2048],
	fRec123: [F32;2],
	fRec119: [F32;2],
	fRec115: [F32;2048],
	fRec117: [F32;2],
	fRec113: [F32;4],
	fRec108: [F32;2],
	fRec104: [F32;2048],
	fRec102: [F32;2],
	fRec101: [F32;2],
	fRec100: [F32;2],
	fRec98: [F32;2],
	fRec95: [F32;2],
	fButton3: F32,
	fVec13: [F32;2],
	fHslider5: F32,
	fRec144: [F32;2],
	fRec143: [F32;2],
	fRec174: [F32;2],
	fRec178: [F32;2],
	fRec183: [F32;2],
	iVec14: [i32;2],
	iRec184: [i32;2],
	fRec181: [F32;2],
	fRec180: [F32;2],
	fRec185: [F32;4],
	fRec186: [F32;2048],
	fVec15: [F32;2],
	iRec187: [i32;2],
	fRec188: [F32;3],
	fVec16: [F32;3],
	fRec179: [F32;2048],
	fRec170: [F32;2],
	fRec166: [F32;2],
	fRec162: [F32;2048],
	fRec164: [F32;2],
	fRec160: [F32;4],
	fRec155: [F32;2],
	fRec151: [F32;2048],
	fRec149: [F32;2],
	fRec148: [F32;2],
	fRec147: [F32;2],
	fRec145: [F32;2],
	fRec142: [F32;2],
	fButton4: F32,
	fVec17: [F32;2],
	fHslider6: F32,
	fRec191: [F32;2],
	fRec190: [F32;2],
	fRec221: [F32;2],
	fRec225: [F32;2],
	fRec230: [F32;2],
	iVec18: [i32;2],
	iRec231: [i32;2],
	fRec228: [F32;2],
	fRec227: [F32;2],
	fRec232: [F32;4],
	fRec233: [F32;2048],
	fVec19: [F32;2],
	iRec234: [i32;2],
	fRec235: [F32;3],
	fVec20: [F32;3],
	fRec226: [F32;2048],
	fRec217: [F32;2],
	fRec213: [F32;2],
	fRec209: [F32;2048],
	fRec211: [F32;2],
	fRec207: [F32;4],
	fRec202: [F32;2],
	fRec198: [F32;2048],
	fRec196: [F32;2],
	fRec195: [F32;2],
	fRec194: [F32;2],
	fRec192: [F32;2],
	fRec189: [F32;2],
	fHslider7: F32,
	fRec236: [F32;2],
	fHslider8: F32,
	fRec237: [F32;2],
	fHslider9: F32,
	fRec239: [F32;2],
	fHslider10: F32,
	fConst13: F32,
	fRec238: [F32;2],
	fConst14: F32,
	fRec244: [F32;2],
	fConst15: F32,
	fRec242: [F32;2],
	fHslider11: F32,
	fRec245: [F32;2],
	fRec241: [F32;3],
	fRec240: [F32;3],
	fHslider12: F32,
	fRec246: [F32;2],
	fRec251: [F32;2],
	fRec249: [F32;2],
	fHslider13: F32,
	fRec252: [F32;2],
	fRec248: [F32;3],
	fRec247: [F32;3],
	fHslider14: F32,
	fRec253: [F32;2],
	fRec258: [F32;2],
	fRec256: [F32;2],
	fHslider15: F32,
	fRec259: [F32;2],
	fRec255: [F32;3],
	fRec254: [F32;3],
	fHslider16: F32,
	fRec260: [F32;2],
	fRec265: [F32;2],
	fRec263: [F32;2],
	fHslider17: F32,
	fRec266: [F32;2],
	fRec262: [F32;3],
	fRec261: [F32;3],
	fHslider18: F32,
	fRec267: [F32;2],
	fHslider19: F32,
	fRec268: [F32;2],
	fHslider20: F32,
	fRec269: [F32;2],
	fRec271: [F32;2],
	fVec21: [F32;2],
	fVec22: [F32;4096],
	fConst16: F32,
	fConst17: F32,
	fRec270: [F32;2],
	fRec273: [F32;2],
	fVec23: [F32;2],
	fVec24: [F32;4096],
	fRec272: [F32;2],
	fRec275: [F32;2],
	fVec25: [F32;2],
	fVec26: [F32;4096],
	fRec274: [F32;2],
	fRec277: [F32;2],
	fVec27: [F32;2],
	fVec28: [F32;4096],
	fRec276: [F32;2],
	fHslider21: F32,
	fRec278: [F32;2],
	fHslider22: F32,
	fRec279: [F32;2],
	fConst18: F32,
	fHslider23: F32,
	fRec280: [F32;2],
	fConst19: F32,
	fHslider24: F32,
	fRec282: [F32;2],
	fHslider25: F32,
	fRec281: [F32;2097152],
	fHslider26: F32,
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
	fHslider27: F32,
	fRec293: [F32;2],
	fRec295: [F32;2],
	fRec299: [F32;2],
	fVec29: [F32;16384],
	fVec30: [F32;2],
	fRec298: [F32;2],
	fRec296: [F32;2],
	fRec301: [F32;2],
	fVec31: [F32;16384],
	fVec32: [F32;2],
	fRec300: [F32;2],
	fRec297: [F32;2],
	fRec305: [F32;2],
	fVec33: [F32;16384],
	fVec34: [F32;2],
	fRec304: [F32;2],
	fRec302: [F32;2],
	fRec307: [F32;2],
	fVec35: [F32;16384],
	fVec36: [F32;2],
	fRec306: [F32;2],
	fRec303: [F32;2],
	fRec311: [F32;2],
	fVec37: [F32;16384],
	fVec38: [F32;2],
	fRec310: [F32;2],
	fRec308: [F32;2],
	fRec313: [F32;2],
	fVec39: [F32;16384],
	fVec40: [F32;2],
	fRec312: [F32;2],
	fRec309: [F32;2],
	fRec317: [F32;2],
	fVec41: [F32;16384],
	fVec42: [F32;2],
	fRec316: [F32;2],
	fRec314: [F32;2],
	fRec319: [F32;2],
	fVec43: [F32;16384],
	fVec44: [F32;2],
	fRec318: [F32;2],
	fRec315: [F32;2],
	fRec323: [F32;2],
	fVec45: [F32;16384],
	fVec46: [F32;2],
	fRec322: [F32;2],
	fRec320: [F32;2],
	fRec325: [F32;2],
	fVec47: [F32;16384],
	fVec48: [F32;2],
	fRec324: [F32;2],
	fRec321: [F32;2],
	fVec49: [F32;1024],
	fHslider28: F32,
	fConst38: F32,
	fRec326: [F32;2],
	fRec327: [F32;2],
	fHslider29: F32,
	fVec50: [F32;16384],
	fVec51: [F32;2],
	fRec294: [F32;2],
	fRec331: [F32;2],
	fRec333: [F32;2],
	fVec52: [F32;1024],
	fVec53: [F32;16384],
	fVec54: [F32;2],
	fRec332: [F32;2],
	fVec55: [F32;16384],
	fVec56: [F32;2],
	fRec330: [F32;2],
	fRec328: [F32;2],
	fRec335: [F32;2],
	fVec57: [F32;16384],
	fVec58: [F32;2],
	fRec334: [F32;2],
	fRec329: [F32;2],
	fRec339: [F32;2],
	fVec59: [F32;16384],
	fVec60: [F32;2],
	fRec338: [F32;2],
	fRec336: [F32;2],
	fRec341: [F32;2],
	fVec61: [F32;16384],
	fVec62: [F32;2],
	fRec340: [F32;2],
	fRec337: [F32;2],
	fRec345: [F32;2],
	fVec63: [F32;16384],
	fVec64: [F32;2],
	fRec344: [F32;2],
	fRec342: [F32;2],
	fRec347: [F32;2],
	fVec65: [F32;16384],
	fVec66: [F32;2],
	fRec346: [F32;2],
	fRec343: [F32;2],
	fRec351: [F32;2],
	fVec67: [F32;16384],
	fVec68: [F32;2],
	fRec350: [F32;2],
	fRec348: [F32;2],
	fRec353: [F32;2],
	fVec69: [F32;16384],
	fVec70: [F32;2],
	fRec352: [F32;2],
	fRec349: [F32;2],
	fRec357: [F32;2],
	fVec71: [F32;16384],
	fVec72: [F32;2],
	fRec356: [F32;2],
	fRec354: [F32;2],
	fRec359: [F32;2],
	fVec73: [F32;16384],
	fVec74: [F32;2],
	fRec358: [F32;2],
	fRec355: [F32;2],
	fVec75: [F32;16384],
	fVec76: [F32;16384],
	fVec77: [F32;2],
	fRec292: [F32;2],
	fConst39: F32,
	fConst41: F32,
	fRec291: [F32;2],
	fRec290: [F32;3],
	fRec289: [F32;3],
	fVec78: [F32;2],
	fConst42: F32,
	fConst44: F32,
	fRec288: [F32;2],
	fRec287: [F32;3],
	fRec286: [F32;3],
	fConst45: F32,
	fConst46: F32,
	fConst47: F32,
	fRec362: [F32;2],
	fRec361: [F32;3],
	fConst48: F32,
	fRec360: [F32;3],
	fConst49: F32,
	fConst50: F32,
	fConst51: F32,
	fRec366: [F32;2],
	fRec365: [F32;3],
	fConst52: F32,
	fRec364: [F32;3],
	fRec363: [F32;3],
	fHslider30: F32,
	fVec79: [F32;1024],
	fHslider31: F32,
	fRec285: [F32;2],
	fHslider32: F32,
	fRec378: [F32;2],
	fVec80: [F32;16384],
	fVec81: [F32;16384],
	fVec82: [F32;2],
	fRec377: [F32;2],
	fRec376: [F32;2],
	fRec375: [F32;3],
	fRec374: [F32;3],
	fVec83: [F32;2],
	fRec373: [F32;2],
	fRec372: [F32;3],
	fRec371: [F32;3],
	fRec381: [F32;2],
	fRec380: [F32;3],
	fRec379: [F32;3],
	fRec385: [F32;2],
	fRec384: [F32;3],
	fRec383: [F32;3],
	fRec382: [F32;3],
	fVec84: [F32;1024],
	fRec370: [F32;2],
	fVec85: [F32;16384],
	fVec86: [F32;2],
	fRec369: [F32;2],
	fRec367: [F32;2],
	fRec387: [F32;2],
	fVec87: [F32;16384],
	fVec88: [F32;2],
	fRec386: [F32;2],
	fRec368: [F32;2],
	fVec89: [F32;16384],
	fVec90: [F32;2],
	fRec390: [F32;2],
	fRec388: [F32;2],
	fVec91: [F32;16384],
	fVec92: [F32;2],
	fRec391: [F32;2],
	fRec389: [F32;2],
	fVec93: [F32;16384],
	fVec94: [F32;2],
	fRec394: [F32;2],
	fRec392: [F32;2],
	fRec396: [F32;2],
	fVec95: [F32;16384],
	fVec96: [F32;2],
	fRec395: [F32;2],
	fRec393: [F32;2],
	fRec400: [F32;2],
	fVec97: [F32;16384],
	fVec98: [F32;2],
	fRec399: [F32;2],
	fRec397: [F32;2],
	fVec99: [F32;16384],
	fVec100: [F32;2],
	fRec401: [F32;2],
	fRec398: [F32;2],
	fRec283: [F32;2],
	fRec284: [F32;2],
	fHslider33: F32,
	fRec402: [F32;2],
}

impl FaustDsp for mydsp {
	type T = F32;
		
	fn new() -> mydsp { 
		mydsp {
			fButton0: 0.0,
			fVec0: [0.0;2],
			iVec1: [0;2],
			fHslider0: 0.0,
			fRec2: [0.0;2],
			fSampleRate: 0,
			fConst0: 0.0,
			fConst1: 0.0,
			fConst2: 0.0,
			fRec1: [0.0;2],
			fHslider1: 0.0,
			fRec3: [0.0;2],
			fConst3: 0.0,
			IOTA0: 0,
			fConst4: 0.0,
			fConst5: 0.0,
			fRec33: [0.0;2],
			fRec37: [0.0;2],
			fConst6: 0.0,
			fConst7: 0.0,
			iRec39: [0;2],
			iRec41: [0;2],
			fRec40: [0.0;3],
			fConst8: 0.0,
			fConst9: 0.0,
			fRec45: [0.0;2],
			iVec2: [0;2],
			iConst10: 0,
			iRec46: [0;2],
			fConst11: 0.0,
			fRec43: [0.0;2],
			fRec42: [0.0;2],
			fRec47: [0.0;4],
			fRec48: [0.0;2048],
			fVec3: [0.0;2],
			fVec4: [0.0;3],
			fRec38: [0.0;2048],
			fRec29: [0.0;2],
			fRec25: [0.0;2],
			fRec21: [0.0;2048],
			fRec23: [0.0;2],
			fHslider2: 0.0,
			fRec19: [0.0;4],
			fRec14: [0.0;2],
			fRec10: [0.0;2048],
			fRec8: [0.0;2],
			fConst12: 0.0,
			fRec7: [0.0;2],
			fRec6: [0.0;2],
			fRec4: [0.0;2],
			fRec0: [0.0;2],
			fHslider3: 0.0,
			fRec50: [0.0;2],
			fRec80: [0.0;2],
			fRec84: [0.0;2],
			fRec89: [0.0;2],
			iVec5: [0;2],
			iRec90: [0;2],
			fRec87: [0.0;2],
			fRec86: [0.0;2],
			fRec91: [0.0;4],
			fRec92: [0.0;2048],
			fVec6: [0.0;2],
			fButton1: 0.0,
			fVec7: [0.0;2],
			iRec93: [0;2],
			fRec94: [0.0;3],
			fVec8: [0.0;3],
			fRec85: [0.0;2048],
			fRec76: [0.0;2],
			fRec72: [0.0;2],
			fRec68: [0.0;2048],
			fRec70: [0.0;2],
			fRec66: [0.0;4],
			fRec61: [0.0;2],
			fRec57: [0.0;2048],
			fRec55: [0.0;2],
			fRec54: [0.0;2],
			fRec53: [0.0;2],
			fRec51: [0.0;2],
			fRec49: [0.0;2],
			fButton2: 0.0,
			fVec9: [0.0;2],
			fHslider4: 0.0,
			fRec97: [0.0;2],
			fRec96: [0.0;2],
			fRec127: [0.0;2],
			fRec131: [0.0;2],
			fRec136: [0.0;2],
			iVec10: [0;2],
			iRec137: [0;2],
			fRec134: [0.0;2],
			fRec133: [0.0;2],
			fRec138: [0.0;4],
			fRec139: [0.0;2048],
			fVec11: [0.0;2],
			iRec140: [0;2],
			fRec141: [0.0;3],
			fVec12: [0.0;3],
			fRec132: [0.0;2048],
			fRec123: [0.0;2],
			fRec119: [0.0;2],
			fRec115: [0.0;2048],
			fRec117: [0.0;2],
			fRec113: [0.0;4],
			fRec108: [0.0;2],
			fRec104: [0.0;2048],
			fRec102: [0.0;2],
			fRec101: [0.0;2],
			fRec100: [0.0;2],
			fRec98: [0.0;2],
			fRec95: [0.0;2],
			fButton3: 0.0,
			fVec13: [0.0;2],
			fHslider5: 0.0,
			fRec144: [0.0;2],
			fRec143: [0.0;2],
			fRec174: [0.0;2],
			fRec178: [0.0;2],
			fRec183: [0.0;2],
			iVec14: [0;2],
			iRec184: [0;2],
			fRec181: [0.0;2],
			fRec180: [0.0;2],
			fRec185: [0.0;4],
			fRec186: [0.0;2048],
			fVec15: [0.0;2],
			iRec187: [0;2],
			fRec188: [0.0;3],
			fVec16: [0.0;3],
			fRec179: [0.0;2048],
			fRec170: [0.0;2],
			fRec166: [0.0;2],
			fRec162: [0.0;2048],
			fRec164: [0.0;2],
			fRec160: [0.0;4],
			fRec155: [0.0;2],
			fRec151: [0.0;2048],
			fRec149: [0.0;2],
			fRec148: [0.0;2],
			fRec147: [0.0;2],
			fRec145: [0.0;2],
			fRec142: [0.0;2],
			fButton4: 0.0,
			fVec17: [0.0;2],
			fHslider6: 0.0,
			fRec191: [0.0;2],
			fRec190: [0.0;2],
			fRec221: [0.0;2],
			fRec225: [0.0;2],
			fRec230: [0.0;2],
			iVec18: [0;2],
			iRec231: [0;2],
			fRec228: [0.0;2],
			fRec227: [0.0;2],
			fRec232: [0.0;4],
			fRec233: [0.0;2048],
			fVec19: [0.0;2],
			iRec234: [0;2],
			fRec235: [0.0;3],
			fVec20: [0.0;3],
			fRec226: [0.0;2048],
			fRec217: [0.0;2],
			fRec213: [0.0;2],
			fRec209: [0.0;2048],
			fRec211: [0.0;2],
			fRec207: [0.0;4],
			fRec202: [0.0;2],
			fRec198: [0.0;2048],
			fRec196: [0.0;2],
			fRec195: [0.0;2],
			fRec194: [0.0;2],
			fRec192: [0.0;2],
			fRec189: [0.0;2],
			fHslider7: 0.0,
			fRec236: [0.0;2],
			fHslider8: 0.0,
			fRec237: [0.0;2],
			fHslider9: 0.0,
			fRec239: [0.0;2],
			fHslider10: 0.0,
			fConst13: 0.0,
			fRec238: [0.0;2],
			fConst14: 0.0,
			fRec244: [0.0;2],
			fConst15: 0.0,
			fRec242: [0.0;2],
			fHslider11: 0.0,
			fRec245: [0.0;2],
			fRec241: [0.0;3],
			fRec240: [0.0;3],
			fHslider12: 0.0,
			fRec246: [0.0;2],
			fRec251: [0.0;2],
			fRec249: [0.0;2],
			fHslider13: 0.0,
			fRec252: [0.0;2],
			fRec248: [0.0;3],
			fRec247: [0.0;3],
			fHslider14: 0.0,
			fRec253: [0.0;2],
			fRec258: [0.0;2],
			fRec256: [0.0;2],
			fHslider15: 0.0,
			fRec259: [0.0;2],
			fRec255: [0.0;3],
			fRec254: [0.0;3],
			fHslider16: 0.0,
			fRec260: [0.0;2],
			fRec265: [0.0;2],
			fRec263: [0.0;2],
			fHslider17: 0.0,
			fRec266: [0.0;2],
			fRec262: [0.0;3],
			fRec261: [0.0;3],
			fHslider18: 0.0,
			fRec267: [0.0;2],
			fHslider19: 0.0,
			fRec268: [0.0;2],
			fHslider20: 0.0,
			fRec269: [0.0;2],
			fRec271: [0.0;2],
			fVec21: [0.0;2],
			fVec22: [0.0;4096],
			fConst16: 0.0,
			fConst17: 0.0,
			fRec270: [0.0;2],
			fRec273: [0.0;2],
			fVec23: [0.0;2],
			fVec24: [0.0;4096],
			fRec272: [0.0;2],
			fRec275: [0.0;2],
			fVec25: [0.0;2],
			fVec26: [0.0;4096],
			fRec274: [0.0;2],
			fRec277: [0.0;2],
			fVec27: [0.0;2],
			fVec28: [0.0;4096],
			fRec276: [0.0;2],
			fHslider21: 0.0,
			fRec278: [0.0;2],
			fHslider22: 0.0,
			fRec279: [0.0;2],
			fConst18: 0.0,
			fHslider23: 0.0,
			fRec280: [0.0;2],
			fConst19: 0.0,
			fHslider24: 0.0,
			fRec282: [0.0;2],
			fHslider25: 0.0,
			fRec281: [0.0;2097152],
			fHslider26: 0.0,
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
			fHslider27: 0.0,
			fRec293: [0.0;2],
			fRec295: [0.0;2],
			fRec299: [0.0;2],
			fVec29: [0.0;16384],
			fVec30: [0.0;2],
			fRec298: [0.0;2],
			fRec296: [0.0;2],
			fRec301: [0.0;2],
			fVec31: [0.0;16384],
			fVec32: [0.0;2],
			fRec300: [0.0;2],
			fRec297: [0.0;2],
			fRec305: [0.0;2],
			fVec33: [0.0;16384],
			fVec34: [0.0;2],
			fRec304: [0.0;2],
			fRec302: [0.0;2],
			fRec307: [0.0;2],
			fVec35: [0.0;16384],
			fVec36: [0.0;2],
			fRec306: [0.0;2],
			fRec303: [0.0;2],
			fRec311: [0.0;2],
			fVec37: [0.0;16384],
			fVec38: [0.0;2],
			fRec310: [0.0;2],
			fRec308: [0.0;2],
			fRec313: [0.0;2],
			fVec39: [0.0;16384],
			fVec40: [0.0;2],
			fRec312: [0.0;2],
			fRec309: [0.0;2],
			fRec317: [0.0;2],
			fVec41: [0.0;16384],
			fVec42: [0.0;2],
			fRec316: [0.0;2],
			fRec314: [0.0;2],
			fRec319: [0.0;2],
			fVec43: [0.0;16384],
			fVec44: [0.0;2],
			fRec318: [0.0;2],
			fRec315: [0.0;2],
			fRec323: [0.0;2],
			fVec45: [0.0;16384],
			fVec46: [0.0;2],
			fRec322: [0.0;2],
			fRec320: [0.0;2],
			fRec325: [0.0;2],
			fVec47: [0.0;16384],
			fVec48: [0.0;2],
			fRec324: [0.0;2],
			fRec321: [0.0;2],
			fVec49: [0.0;1024],
			fHslider28: 0.0,
			fConst38: 0.0,
			fRec326: [0.0;2],
			fRec327: [0.0;2],
			fHslider29: 0.0,
			fVec50: [0.0;16384],
			fVec51: [0.0;2],
			fRec294: [0.0;2],
			fRec331: [0.0;2],
			fRec333: [0.0;2],
			fVec52: [0.0;1024],
			fVec53: [0.0;16384],
			fVec54: [0.0;2],
			fRec332: [0.0;2],
			fVec55: [0.0;16384],
			fVec56: [0.0;2],
			fRec330: [0.0;2],
			fRec328: [0.0;2],
			fRec335: [0.0;2],
			fVec57: [0.0;16384],
			fVec58: [0.0;2],
			fRec334: [0.0;2],
			fRec329: [0.0;2],
			fRec339: [0.0;2],
			fVec59: [0.0;16384],
			fVec60: [0.0;2],
			fRec338: [0.0;2],
			fRec336: [0.0;2],
			fRec341: [0.0;2],
			fVec61: [0.0;16384],
			fVec62: [0.0;2],
			fRec340: [0.0;2],
			fRec337: [0.0;2],
			fRec345: [0.0;2],
			fVec63: [0.0;16384],
			fVec64: [0.0;2],
			fRec344: [0.0;2],
			fRec342: [0.0;2],
			fRec347: [0.0;2],
			fVec65: [0.0;16384],
			fVec66: [0.0;2],
			fRec346: [0.0;2],
			fRec343: [0.0;2],
			fRec351: [0.0;2],
			fVec67: [0.0;16384],
			fVec68: [0.0;2],
			fRec350: [0.0;2],
			fRec348: [0.0;2],
			fRec353: [0.0;2],
			fVec69: [0.0;16384],
			fVec70: [0.0;2],
			fRec352: [0.0;2],
			fRec349: [0.0;2],
			fRec357: [0.0;2],
			fVec71: [0.0;16384],
			fVec72: [0.0;2],
			fRec356: [0.0;2],
			fRec354: [0.0;2],
			fRec359: [0.0;2],
			fVec73: [0.0;16384],
			fVec74: [0.0;2],
			fRec358: [0.0;2],
			fRec355: [0.0;2],
			fVec75: [0.0;16384],
			fVec76: [0.0;16384],
			fVec77: [0.0;2],
			fRec292: [0.0;2],
			fConst39: 0.0,
			fConst41: 0.0,
			fRec291: [0.0;2],
			fRec290: [0.0;3],
			fRec289: [0.0;3],
			fVec78: [0.0;2],
			fConst42: 0.0,
			fConst44: 0.0,
			fRec288: [0.0;2],
			fRec287: [0.0;3],
			fRec286: [0.0;3],
			fConst45: 0.0,
			fConst46: 0.0,
			fConst47: 0.0,
			fRec362: [0.0;2],
			fRec361: [0.0;3],
			fConst48: 0.0,
			fRec360: [0.0;3],
			fConst49: 0.0,
			fConst50: 0.0,
			fConst51: 0.0,
			fRec366: [0.0;2],
			fRec365: [0.0;3],
			fConst52: 0.0,
			fRec364: [0.0;3],
			fRec363: [0.0;3],
			fHslider30: 0.0,
			fVec79: [0.0;1024],
			fHslider31: 0.0,
			fRec285: [0.0;2],
			fHslider32: 0.0,
			fRec378: [0.0;2],
			fVec80: [0.0;16384],
			fVec81: [0.0;16384],
			fVec82: [0.0;2],
			fRec377: [0.0;2],
			fRec376: [0.0;2],
			fRec375: [0.0;3],
			fRec374: [0.0;3],
			fVec83: [0.0;2],
			fRec373: [0.0;2],
			fRec372: [0.0;3],
			fRec371: [0.0;3],
			fRec381: [0.0;2],
			fRec380: [0.0;3],
			fRec379: [0.0;3],
			fRec385: [0.0;2],
			fRec384: [0.0;3],
			fRec383: [0.0;3],
			fRec382: [0.0;3],
			fVec84: [0.0;1024],
			fRec370: [0.0;2],
			fVec85: [0.0;16384],
			fVec86: [0.0;2],
			fRec369: [0.0;2],
			fRec367: [0.0;2],
			fRec387: [0.0;2],
			fVec87: [0.0;16384],
			fVec88: [0.0;2],
			fRec386: [0.0;2],
			fRec368: [0.0;2],
			fVec89: [0.0;16384],
			fVec90: [0.0;2],
			fRec390: [0.0;2],
			fRec388: [0.0;2],
			fVec91: [0.0;16384],
			fVec92: [0.0;2],
			fRec391: [0.0;2],
			fRec389: [0.0;2],
			fVec93: [0.0;16384],
			fVec94: [0.0;2],
			fRec394: [0.0;2],
			fRec392: [0.0;2],
			fRec396: [0.0;2],
			fVec95: [0.0;16384],
			fVec96: [0.0;2],
			fRec395: [0.0;2],
			fRec393: [0.0;2],
			fRec400: [0.0;2],
			fVec97: [0.0;16384],
			fVec98: [0.0;2],
			fRec399: [0.0;2],
			fRec397: [0.0;2],
			fVec99: [0.0;16384],
			fVec100: [0.0;2],
			fRec401: [0.0;2],
			fRec398: [0.0;2],
			fRec283: [0.0;2],
			fRec284: [0.0;2],
			fHslider33: 0.0,
			fRec402: [0.0;2],
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
		self.fButton0 = 0.0;
		self.fHslider0 = 8e+01;
		self.fHslider1 = 0.0;
		self.fHslider2 = 1.0;
		self.fHslider3 = 8e+01;
		self.fButton1 = 0.0;
		self.fButton2 = 0.0;
		self.fHslider4 = 8e+01;
		self.fButton3 = 0.0;
		self.fHslider5 = 8e+01;
		self.fButton4 = 0.0;
		self.fHslider6 = 8e+01;
		self.fHslider7 = 1.0;
		self.fHslider8 = 0.0;
		self.fHslider9 = 0.0;
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
		self.fHslider20 = 6e+01;
		self.fHslider21 = 1.0;
		self.fHslider22 = 0.0;
		self.fHslider23 = 1.0;
		self.fHslider24 = 0.3;
		self.fHslider25 = 0.3;
		self.fHslider26 = 0.11;
		self.fHslider27 = 5.0;
		self.fHslider28 = 0.6;
		self.fHslider29 = 0.98;
		self.fHslider30 = 3.5;
		self.fHslider31 = 0.88;
		self.fHslider32 = 0.75;
		self.fHslider33 = 1.0;
	}
	fn instance_clear(&mut self) {
		for l0 in 0..2 {
			self.fVec0[(l0) as usize] = 0.0;
		}
		for l1 in 0..2 {
			self.iVec1[(l1) as usize] = 0;
		}
		for l2 in 0..2 {
			self.fRec2[(l2) as usize] = 0.0;
		}
		for l3 in 0..2 {
			self.fRec1[(l3) as usize] = 0.0;
		}
		for l4 in 0..2 {
			self.fRec3[(l4) as usize] = 0.0;
		}
		self.IOTA0 = 0;
		for l5 in 0..2 {
			self.fRec33[(l5) as usize] = 0.0;
		}
		for l6 in 0..2 {
			self.fRec37[(l6) as usize] = 0.0;
		}
		for l7 in 0..2 {
			self.iRec39[(l7) as usize] = 0;
		}
		for l8 in 0..2 {
			self.iRec41[(l8) as usize] = 0;
		}
		for l9 in 0..3 {
			self.fRec40[(l9) as usize] = 0.0;
		}
		for l10 in 0..2 {
			self.fRec45[(l10) as usize] = 0.0;
		}
		for l11 in 0..2 {
			self.iVec2[(l11) as usize] = 0;
		}
		for l12 in 0..2 {
			self.iRec46[(l12) as usize] = 0;
		}
		for l13 in 0..2 {
			self.fRec43[(l13) as usize] = 0.0;
		}
		for l14 in 0..2 {
			self.fRec42[(l14) as usize] = 0.0;
		}
		for l15 in 0..4 {
			self.fRec47[(l15) as usize] = 0.0;
		}
		for l16 in 0..2048 {
			self.fRec48[(l16) as usize] = 0.0;
		}
		for l17 in 0..2 {
			self.fVec3[(l17) as usize] = 0.0;
		}
		for l18 in 0..3 {
			self.fVec4[(l18) as usize] = 0.0;
		}
		for l19 in 0..2048 {
			self.fRec38[(l19) as usize] = 0.0;
		}
		for l20 in 0..2 {
			self.fRec29[(l20) as usize] = 0.0;
		}
		for l21 in 0..2 {
			self.fRec25[(l21) as usize] = 0.0;
		}
		for l22 in 0..2048 {
			self.fRec21[(l22) as usize] = 0.0;
		}
		for l23 in 0..2 {
			self.fRec23[(l23) as usize] = 0.0;
		}
		for l24 in 0..4 {
			self.fRec19[(l24) as usize] = 0.0;
		}
		for l25 in 0..2 {
			self.fRec14[(l25) as usize] = 0.0;
		}
		for l26 in 0..2048 {
			self.fRec10[(l26) as usize] = 0.0;
		}
		for l27 in 0..2 {
			self.fRec8[(l27) as usize] = 0.0;
		}
		for l28 in 0..2 {
			self.fRec7[(l28) as usize] = 0.0;
		}
		for l29 in 0..2 {
			self.fRec6[(l29) as usize] = 0.0;
		}
		for l30 in 0..2 {
			self.fRec4[(l30) as usize] = 0.0;
		}
		for l31 in 0..2 {
			self.fRec0[(l31) as usize] = 0.0;
		}
		for l32 in 0..2 {
			self.fRec50[(l32) as usize] = 0.0;
		}
		for l33 in 0..2 {
			self.fRec80[(l33) as usize] = 0.0;
		}
		for l34 in 0..2 {
			self.fRec84[(l34) as usize] = 0.0;
		}
		for l35 in 0..2 {
			self.fRec89[(l35) as usize] = 0.0;
		}
		for l36 in 0..2 {
			self.iVec5[(l36) as usize] = 0;
		}
		for l37 in 0..2 {
			self.iRec90[(l37) as usize] = 0;
		}
		for l38 in 0..2 {
			self.fRec87[(l38) as usize] = 0.0;
		}
		for l39 in 0..2 {
			self.fRec86[(l39) as usize] = 0.0;
		}
		for l40 in 0..4 {
			self.fRec91[(l40) as usize] = 0.0;
		}
		for l41 in 0..2048 {
			self.fRec92[(l41) as usize] = 0.0;
		}
		for l42 in 0..2 {
			self.fVec6[(l42) as usize] = 0.0;
		}
		for l43 in 0..2 {
			self.fVec7[(l43) as usize] = 0.0;
		}
		for l44 in 0..2 {
			self.iRec93[(l44) as usize] = 0;
		}
		for l45 in 0..3 {
			self.fRec94[(l45) as usize] = 0.0;
		}
		for l46 in 0..3 {
			self.fVec8[(l46) as usize] = 0.0;
		}
		for l47 in 0..2048 {
			self.fRec85[(l47) as usize] = 0.0;
		}
		for l48 in 0..2 {
			self.fRec76[(l48) as usize] = 0.0;
		}
		for l49 in 0..2 {
			self.fRec72[(l49) as usize] = 0.0;
		}
		for l50 in 0..2048 {
			self.fRec68[(l50) as usize] = 0.0;
		}
		for l51 in 0..2 {
			self.fRec70[(l51) as usize] = 0.0;
		}
		for l52 in 0..4 {
			self.fRec66[(l52) as usize] = 0.0;
		}
		for l53 in 0..2 {
			self.fRec61[(l53) as usize] = 0.0;
		}
		for l54 in 0..2048 {
			self.fRec57[(l54) as usize] = 0.0;
		}
		for l55 in 0..2 {
			self.fRec55[(l55) as usize] = 0.0;
		}
		for l56 in 0..2 {
			self.fRec54[(l56) as usize] = 0.0;
		}
		for l57 in 0..2 {
			self.fRec53[(l57) as usize] = 0.0;
		}
		for l58 in 0..2 {
			self.fRec51[(l58) as usize] = 0.0;
		}
		for l59 in 0..2 {
			self.fRec49[(l59) as usize] = 0.0;
		}
		for l60 in 0..2 {
			self.fVec9[(l60) as usize] = 0.0;
		}
		for l61 in 0..2 {
			self.fRec97[(l61) as usize] = 0.0;
		}
		for l62 in 0..2 {
			self.fRec96[(l62) as usize] = 0.0;
		}
		for l63 in 0..2 {
			self.fRec127[(l63) as usize] = 0.0;
		}
		for l64 in 0..2 {
			self.fRec131[(l64) as usize] = 0.0;
		}
		for l65 in 0..2 {
			self.fRec136[(l65) as usize] = 0.0;
		}
		for l66 in 0..2 {
			self.iVec10[(l66) as usize] = 0;
		}
		for l67 in 0..2 {
			self.iRec137[(l67) as usize] = 0;
		}
		for l68 in 0..2 {
			self.fRec134[(l68) as usize] = 0.0;
		}
		for l69 in 0..2 {
			self.fRec133[(l69) as usize] = 0.0;
		}
		for l70 in 0..4 {
			self.fRec138[(l70) as usize] = 0.0;
		}
		for l71 in 0..2048 {
			self.fRec139[(l71) as usize] = 0.0;
		}
		for l72 in 0..2 {
			self.fVec11[(l72) as usize] = 0.0;
		}
		for l73 in 0..2 {
			self.iRec140[(l73) as usize] = 0;
		}
		for l74 in 0..3 {
			self.fRec141[(l74) as usize] = 0.0;
		}
		for l75 in 0..3 {
			self.fVec12[(l75) as usize] = 0.0;
		}
		for l76 in 0..2048 {
			self.fRec132[(l76) as usize] = 0.0;
		}
		for l77 in 0..2 {
			self.fRec123[(l77) as usize] = 0.0;
		}
		for l78 in 0..2 {
			self.fRec119[(l78) as usize] = 0.0;
		}
		for l79 in 0..2048 {
			self.fRec115[(l79) as usize] = 0.0;
		}
		for l80 in 0..2 {
			self.fRec117[(l80) as usize] = 0.0;
		}
		for l81 in 0..4 {
			self.fRec113[(l81) as usize] = 0.0;
		}
		for l82 in 0..2 {
			self.fRec108[(l82) as usize] = 0.0;
		}
		for l83 in 0..2048 {
			self.fRec104[(l83) as usize] = 0.0;
		}
		for l84 in 0..2 {
			self.fRec102[(l84) as usize] = 0.0;
		}
		for l85 in 0..2 {
			self.fRec101[(l85) as usize] = 0.0;
		}
		for l86 in 0..2 {
			self.fRec100[(l86) as usize] = 0.0;
		}
		for l87 in 0..2 {
			self.fRec98[(l87) as usize] = 0.0;
		}
		for l88 in 0..2 {
			self.fRec95[(l88) as usize] = 0.0;
		}
		for l89 in 0..2 {
			self.fVec13[(l89) as usize] = 0.0;
		}
		for l90 in 0..2 {
			self.fRec144[(l90) as usize] = 0.0;
		}
		for l91 in 0..2 {
			self.fRec143[(l91) as usize] = 0.0;
		}
		for l92 in 0..2 {
			self.fRec174[(l92) as usize] = 0.0;
		}
		for l93 in 0..2 {
			self.fRec178[(l93) as usize] = 0.0;
		}
		for l94 in 0..2 {
			self.fRec183[(l94) as usize] = 0.0;
		}
		for l95 in 0..2 {
			self.iVec14[(l95) as usize] = 0;
		}
		for l96 in 0..2 {
			self.iRec184[(l96) as usize] = 0;
		}
		for l97 in 0..2 {
			self.fRec181[(l97) as usize] = 0.0;
		}
		for l98 in 0..2 {
			self.fRec180[(l98) as usize] = 0.0;
		}
		for l99 in 0..4 {
			self.fRec185[(l99) as usize] = 0.0;
		}
		for l100 in 0..2048 {
			self.fRec186[(l100) as usize] = 0.0;
		}
		for l101 in 0..2 {
			self.fVec15[(l101) as usize] = 0.0;
		}
		for l102 in 0..2 {
			self.iRec187[(l102) as usize] = 0;
		}
		for l103 in 0..3 {
			self.fRec188[(l103) as usize] = 0.0;
		}
		for l104 in 0..3 {
			self.fVec16[(l104) as usize] = 0.0;
		}
		for l105 in 0..2048 {
			self.fRec179[(l105) as usize] = 0.0;
		}
		for l106 in 0..2 {
			self.fRec170[(l106) as usize] = 0.0;
		}
		for l107 in 0..2 {
			self.fRec166[(l107) as usize] = 0.0;
		}
		for l108 in 0..2048 {
			self.fRec162[(l108) as usize] = 0.0;
		}
		for l109 in 0..2 {
			self.fRec164[(l109) as usize] = 0.0;
		}
		for l110 in 0..4 {
			self.fRec160[(l110) as usize] = 0.0;
		}
		for l111 in 0..2 {
			self.fRec155[(l111) as usize] = 0.0;
		}
		for l112 in 0..2048 {
			self.fRec151[(l112) as usize] = 0.0;
		}
		for l113 in 0..2 {
			self.fRec149[(l113) as usize] = 0.0;
		}
		for l114 in 0..2 {
			self.fRec148[(l114) as usize] = 0.0;
		}
		for l115 in 0..2 {
			self.fRec147[(l115) as usize] = 0.0;
		}
		for l116 in 0..2 {
			self.fRec145[(l116) as usize] = 0.0;
		}
		for l117 in 0..2 {
			self.fRec142[(l117) as usize] = 0.0;
		}
		for l118 in 0..2 {
			self.fVec17[(l118) as usize] = 0.0;
		}
		for l119 in 0..2 {
			self.fRec191[(l119) as usize] = 0.0;
		}
		for l120 in 0..2 {
			self.fRec190[(l120) as usize] = 0.0;
		}
		for l121 in 0..2 {
			self.fRec221[(l121) as usize] = 0.0;
		}
		for l122 in 0..2 {
			self.fRec225[(l122) as usize] = 0.0;
		}
		for l123 in 0..2 {
			self.fRec230[(l123) as usize] = 0.0;
		}
		for l124 in 0..2 {
			self.iVec18[(l124) as usize] = 0;
		}
		for l125 in 0..2 {
			self.iRec231[(l125) as usize] = 0;
		}
		for l126 in 0..2 {
			self.fRec228[(l126) as usize] = 0.0;
		}
		for l127 in 0..2 {
			self.fRec227[(l127) as usize] = 0.0;
		}
		for l128 in 0..4 {
			self.fRec232[(l128) as usize] = 0.0;
		}
		for l129 in 0..2048 {
			self.fRec233[(l129) as usize] = 0.0;
		}
		for l130 in 0..2 {
			self.fVec19[(l130) as usize] = 0.0;
		}
		for l131 in 0..2 {
			self.iRec234[(l131) as usize] = 0;
		}
		for l132 in 0..3 {
			self.fRec235[(l132) as usize] = 0.0;
		}
		for l133 in 0..3 {
			self.fVec20[(l133) as usize] = 0.0;
		}
		for l134 in 0..2048 {
			self.fRec226[(l134) as usize] = 0.0;
		}
		for l135 in 0..2 {
			self.fRec217[(l135) as usize] = 0.0;
		}
		for l136 in 0..2 {
			self.fRec213[(l136) as usize] = 0.0;
		}
		for l137 in 0..2048 {
			self.fRec209[(l137) as usize] = 0.0;
		}
		for l138 in 0..2 {
			self.fRec211[(l138) as usize] = 0.0;
		}
		for l139 in 0..4 {
			self.fRec207[(l139) as usize] = 0.0;
		}
		for l140 in 0..2 {
			self.fRec202[(l140) as usize] = 0.0;
		}
		for l141 in 0..2048 {
			self.fRec198[(l141) as usize] = 0.0;
		}
		for l142 in 0..2 {
			self.fRec196[(l142) as usize] = 0.0;
		}
		for l143 in 0..2 {
			self.fRec195[(l143) as usize] = 0.0;
		}
		for l144 in 0..2 {
			self.fRec194[(l144) as usize] = 0.0;
		}
		for l145 in 0..2 {
			self.fRec192[(l145) as usize] = 0.0;
		}
		for l146 in 0..2 {
			self.fRec189[(l146) as usize] = 0.0;
		}
		for l147 in 0..2 {
			self.fRec236[(l147) as usize] = 0.0;
		}
		for l148 in 0..2 {
			self.fRec237[(l148) as usize] = 0.0;
		}
		for l149 in 0..2 {
			self.fRec239[(l149) as usize] = 0.0;
		}
		for l150 in 0..2 {
			self.fRec238[(l150) as usize] = 0.0;
		}
		for l151 in 0..2 {
			self.fRec244[(l151) as usize] = 0.0;
		}
		for l152 in 0..2 {
			self.fRec242[(l152) as usize] = 0.0;
		}
		for l153 in 0..2 {
			self.fRec245[(l153) as usize] = 0.0;
		}
		for l154 in 0..3 {
			self.fRec241[(l154) as usize] = 0.0;
		}
		for l155 in 0..3 {
			self.fRec240[(l155) as usize] = 0.0;
		}
		for l156 in 0..2 {
			self.fRec246[(l156) as usize] = 0.0;
		}
		for l157 in 0..2 {
			self.fRec251[(l157) as usize] = 0.0;
		}
		for l158 in 0..2 {
			self.fRec249[(l158) as usize] = 0.0;
		}
		for l159 in 0..2 {
			self.fRec252[(l159) as usize] = 0.0;
		}
		for l160 in 0..3 {
			self.fRec248[(l160) as usize] = 0.0;
		}
		for l161 in 0..3 {
			self.fRec247[(l161) as usize] = 0.0;
		}
		for l162 in 0..2 {
			self.fRec253[(l162) as usize] = 0.0;
		}
		for l163 in 0..2 {
			self.fRec258[(l163) as usize] = 0.0;
		}
		for l164 in 0..2 {
			self.fRec256[(l164) as usize] = 0.0;
		}
		for l165 in 0..2 {
			self.fRec259[(l165) as usize] = 0.0;
		}
		for l166 in 0..3 {
			self.fRec255[(l166) as usize] = 0.0;
		}
		for l167 in 0..3 {
			self.fRec254[(l167) as usize] = 0.0;
		}
		for l168 in 0..2 {
			self.fRec260[(l168) as usize] = 0.0;
		}
		for l169 in 0..2 {
			self.fRec265[(l169) as usize] = 0.0;
		}
		for l170 in 0..2 {
			self.fRec263[(l170) as usize] = 0.0;
		}
		for l171 in 0..2 {
			self.fRec266[(l171) as usize] = 0.0;
		}
		for l172 in 0..3 {
			self.fRec262[(l172) as usize] = 0.0;
		}
		for l173 in 0..3 {
			self.fRec261[(l173) as usize] = 0.0;
		}
		for l174 in 0..2 {
			self.fRec267[(l174) as usize] = 0.0;
		}
		for l175 in 0..2 {
			self.fRec268[(l175) as usize] = 0.0;
		}
		for l176 in 0..2 {
			self.fRec269[(l176) as usize] = 0.0;
		}
		for l177 in 0..2 {
			self.fRec271[(l177) as usize] = 0.0;
		}
		for l178 in 0..2 {
			self.fVec21[(l178) as usize] = 0.0;
		}
		for l179 in 0..4096 {
			self.fVec22[(l179) as usize] = 0.0;
		}
		for l180 in 0..2 {
			self.fRec270[(l180) as usize] = 0.0;
		}
		for l181 in 0..2 {
			self.fRec273[(l181) as usize] = 0.0;
		}
		for l182 in 0..2 {
			self.fVec23[(l182) as usize] = 0.0;
		}
		for l183 in 0..4096 {
			self.fVec24[(l183) as usize] = 0.0;
		}
		for l184 in 0..2 {
			self.fRec272[(l184) as usize] = 0.0;
		}
		for l185 in 0..2 {
			self.fRec275[(l185) as usize] = 0.0;
		}
		for l186 in 0..2 {
			self.fVec25[(l186) as usize] = 0.0;
		}
		for l187 in 0..4096 {
			self.fVec26[(l187) as usize] = 0.0;
		}
		for l188 in 0..2 {
			self.fRec274[(l188) as usize] = 0.0;
		}
		for l189 in 0..2 {
			self.fRec277[(l189) as usize] = 0.0;
		}
		for l190 in 0..2 {
			self.fVec27[(l190) as usize] = 0.0;
		}
		for l191 in 0..4096 {
			self.fVec28[(l191) as usize] = 0.0;
		}
		for l192 in 0..2 {
			self.fRec276[(l192) as usize] = 0.0;
		}
		for l193 in 0..2 {
			self.fRec278[(l193) as usize] = 0.0;
		}
		for l194 in 0..2 {
			self.fRec279[(l194) as usize] = 0.0;
		}
		for l195 in 0..2 {
			self.fRec280[(l195) as usize] = 0.0;
		}
		for l196 in 0..2 {
			self.fRec282[(l196) as usize] = 0.0;
		}
		for l197 in 0..2097152 {
			self.fRec281[(l197) as usize] = 0.0;
		}
		for l198 in 0..2 {
			self.fRec293[(l198) as usize] = 0.0;
		}
		for l199 in 0..2 {
			self.fRec295[(l199) as usize] = 0.0;
		}
		for l200 in 0..2 {
			self.fRec299[(l200) as usize] = 0.0;
		}
		for l201 in 0..16384 {
			self.fVec29[(l201) as usize] = 0.0;
		}
		for l202 in 0..2 {
			self.fVec30[(l202) as usize] = 0.0;
		}
		for l203 in 0..2 {
			self.fRec298[(l203) as usize] = 0.0;
		}
		for l204 in 0..2 {
			self.fRec296[(l204) as usize] = 0.0;
		}
		for l205 in 0..2 {
			self.fRec301[(l205) as usize] = 0.0;
		}
		for l206 in 0..16384 {
			self.fVec31[(l206) as usize] = 0.0;
		}
		for l207 in 0..2 {
			self.fVec32[(l207) as usize] = 0.0;
		}
		for l208 in 0..2 {
			self.fRec300[(l208) as usize] = 0.0;
		}
		for l209 in 0..2 {
			self.fRec297[(l209) as usize] = 0.0;
		}
		for l210 in 0..2 {
			self.fRec305[(l210) as usize] = 0.0;
		}
		for l211 in 0..16384 {
			self.fVec33[(l211) as usize] = 0.0;
		}
		for l212 in 0..2 {
			self.fVec34[(l212) as usize] = 0.0;
		}
		for l213 in 0..2 {
			self.fRec304[(l213) as usize] = 0.0;
		}
		for l214 in 0..2 {
			self.fRec302[(l214) as usize] = 0.0;
		}
		for l215 in 0..2 {
			self.fRec307[(l215) as usize] = 0.0;
		}
		for l216 in 0..16384 {
			self.fVec35[(l216) as usize] = 0.0;
		}
		for l217 in 0..2 {
			self.fVec36[(l217) as usize] = 0.0;
		}
		for l218 in 0..2 {
			self.fRec306[(l218) as usize] = 0.0;
		}
		for l219 in 0..2 {
			self.fRec303[(l219) as usize] = 0.0;
		}
		for l220 in 0..2 {
			self.fRec311[(l220) as usize] = 0.0;
		}
		for l221 in 0..16384 {
			self.fVec37[(l221) as usize] = 0.0;
		}
		for l222 in 0..2 {
			self.fVec38[(l222) as usize] = 0.0;
		}
		for l223 in 0..2 {
			self.fRec310[(l223) as usize] = 0.0;
		}
		for l224 in 0..2 {
			self.fRec308[(l224) as usize] = 0.0;
		}
		for l225 in 0..2 {
			self.fRec313[(l225) as usize] = 0.0;
		}
		for l226 in 0..16384 {
			self.fVec39[(l226) as usize] = 0.0;
		}
		for l227 in 0..2 {
			self.fVec40[(l227) as usize] = 0.0;
		}
		for l228 in 0..2 {
			self.fRec312[(l228) as usize] = 0.0;
		}
		for l229 in 0..2 {
			self.fRec309[(l229) as usize] = 0.0;
		}
		for l230 in 0..2 {
			self.fRec317[(l230) as usize] = 0.0;
		}
		for l231 in 0..16384 {
			self.fVec41[(l231) as usize] = 0.0;
		}
		for l232 in 0..2 {
			self.fVec42[(l232) as usize] = 0.0;
		}
		for l233 in 0..2 {
			self.fRec316[(l233) as usize] = 0.0;
		}
		for l234 in 0..2 {
			self.fRec314[(l234) as usize] = 0.0;
		}
		for l235 in 0..2 {
			self.fRec319[(l235) as usize] = 0.0;
		}
		for l236 in 0..16384 {
			self.fVec43[(l236) as usize] = 0.0;
		}
		for l237 in 0..2 {
			self.fVec44[(l237) as usize] = 0.0;
		}
		for l238 in 0..2 {
			self.fRec318[(l238) as usize] = 0.0;
		}
		for l239 in 0..2 {
			self.fRec315[(l239) as usize] = 0.0;
		}
		for l240 in 0..2 {
			self.fRec323[(l240) as usize] = 0.0;
		}
		for l241 in 0..16384 {
			self.fVec45[(l241) as usize] = 0.0;
		}
		for l242 in 0..2 {
			self.fVec46[(l242) as usize] = 0.0;
		}
		for l243 in 0..2 {
			self.fRec322[(l243) as usize] = 0.0;
		}
		for l244 in 0..2 {
			self.fRec320[(l244) as usize] = 0.0;
		}
		for l245 in 0..2 {
			self.fRec325[(l245) as usize] = 0.0;
		}
		for l246 in 0..16384 {
			self.fVec47[(l246) as usize] = 0.0;
		}
		for l247 in 0..2 {
			self.fVec48[(l247) as usize] = 0.0;
		}
		for l248 in 0..2 {
			self.fRec324[(l248) as usize] = 0.0;
		}
		for l249 in 0..2 {
			self.fRec321[(l249) as usize] = 0.0;
		}
		for l250 in 0..1024 {
			self.fVec49[(l250) as usize] = 0.0;
		}
		for l251 in 0..2 {
			self.fRec326[(l251) as usize] = 0.0;
		}
		for l252 in 0..2 {
			self.fRec327[(l252) as usize] = 0.0;
		}
		for l253 in 0..16384 {
			self.fVec50[(l253) as usize] = 0.0;
		}
		for l254 in 0..2 {
			self.fVec51[(l254) as usize] = 0.0;
		}
		for l255 in 0..2 {
			self.fRec294[(l255) as usize] = 0.0;
		}
		for l256 in 0..2 {
			self.fRec331[(l256) as usize] = 0.0;
		}
		for l257 in 0..2 {
			self.fRec333[(l257) as usize] = 0.0;
		}
		for l258 in 0..1024 {
			self.fVec52[(l258) as usize] = 0.0;
		}
		for l259 in 0..16384 {
			self.fVec53[(l259) as usize] = 0.0;
		}
		for l260 in 0..2 {
			self.fVec54[(l260) as usize] = 0.0;
		}
		for l261 in 0..2 {
			self.fRec332[(l261) as usize] = 0.0;
		}
		for l262 in 0..16384 {
			self.fVec55[(l262) as usize] = 0.0;
		}
		for l263 in 0..2 {
			self.fVec56[(l263) as usize] = 0.0;
		}
		for l264 in 0..2 {
			self.fRec330[(l264) as usize] = 0.0;
		}
		for l265 in 0..2 {
			self.fRec328[(l265) as usize] = 0.0;
		}
		for l266 in 0..2 {
			self.fRec335[(l266) as usize] = 0.0;
		}
		for l267 in 0..16384 {
			self.fVec57[(l267) as usize] = 0.0;
		}
		for l268 in 0..2 {
			self.fVec58[(l268) as usize] = 0.0;
		}
		for l269 in 0..2 {
			self.fRec334[(l269) as usize] = 0.0;
		}
		for l270 in 0..2 {
			self.fRec329[(l270) as usize] = 0.0;
		}
		for l271 in 0..2 {
			self.fRec339[(l271) as usize] = 0.0;
		}
		for l272 in 0..16384 {
			self.fVec59[(l272) as usize] = 0.0;
		}
		for l273 in 0..2 {
			self.fVec60[(l273) as usize] = 0.0;
		}
		for l274 in 0..2 {
			self.fRec338[(l274) as usize] = 0.0;
		}
		for l275 in 0..2 {
			self.fRec336[(l275) as usize] = 0.0;
		}
		for l276 in 0..2 {
			self.fRec341[(l276) as usize] = 0.0;
		}
		for l277 in 0..16384 {
			self.fVec61[(l277) as usize] = 0.0;
		}
		for l278 in 0..2 {
			self.fVec62[(l278) as usize] = 0.0;
		}
		for l279 in 0..2 {
			self.fRec340[(l279) as usize] = 0.0;
		}
		for l280 in 0..2 {
			self.fRec337[(l280) as usize] = 0.0;
		}
		for l281 in 0..2 {
			self.fRec345[(l281) as usize] = 0.0;
		}
		for l282 in 0..16384 {
			self.fVec63[(l282) as usize] = 0.0;
		}
		for l283 in 0..2 {
			self.fVec64[(l283) as usize] = 0.0;
		}
		for l284 in 0..2 {
			self.fRec344[(l284) as usize] = 0.0;
		}
		for l285 in 0..2 {
			self.fRec342[(l285) as usize] = 0.0;
		}
		for l286 in 0..2 {
			self.fRec347[(l286) as usize] = 0.0;
		}
		for l287 in 0..16384 {
			self.fVec65[(l287) as usize] = 0.0;
		}
		for l288 in 0..2 {
			self.fVec66[(l288) as usize] = 0.0;
		}
		for l289 in 0..2 {
			self.fRec346[(l289) as usize] = 0.0;
		}
		for l290 in 0..2 {
			self.fRec343[(l290) as usize] = 0.0;
		}
		for l291 in 0..2 {
			self.fRec351[(l291) as usize] = 0.0;
		}
		for l292 in 0..16384 {
			self.fVec67[(l292) as usize] = 0.0;
		}
		for l293 in 0..2 {
			self.fVec68[(l293) as usize] = 0.0;
		}
		for l294 in 0..2 {
			self.fRec350[(l294) as usize] = 0.0;
		}
		for l295 in 0..2 {
			self.fRec348[(l295) as usize] = 0.0;
		}
		for l296 in 0..2 {
			self.fRec353[(l296) as usize] = 0.0;
		}
		for l297 in 0..16384 {
			self.fVec69[(l297) as usize] = 0.0;
		}
		for l298 in 0..2 {
			self.fVec70[(l298) as usize] = 0.0;
		}
		for l299 in 0..2 {
			self.fRec352[(l299) as usize] = 0.0;
		}
		for l300 in 0..2 {
			self.fRec349[(l300) as usize] = 0.0;
		}
		for l301 in 0..2 {
			self.fRec357[(l301) as usize] = 0.0;
		}
		for l302 in 0..16384 {
			self.fVec71[(l302) as usize] = 0.0;
		}
		for l303 in 0..2 {
			self.fVec72[(l303) as usize] = 0.0;
		}
		for l304 in 0..2 {
			self.fRec356[(l304) as usize] = 0.0;
		}
		for l305 in 0..2 {
			self.fRec354[(l305) as usize] = 0.0;
		}
		for l306 in 0..2 {
			self.fRec359[(l306) as usize] = 0.0;
		}
		for l307 in 0..16384 {
			self.fVec73[(l307) as usize] = 0.0;
		}
		for l308 in 0..2 {
			self.fVec74[(l308) as usize] = 0.0;
		}
		for l309 in 0..2 {
			self.fRec358[(l309) as usize] = 0.0;
		}
		for l310 in 0..2 {
			self.fRec355[(l310) as usize] = 0.0;
		}
		for l311 in 0..16384 {
			self.fVec75[(l311) as usize] = 0.0;
		}
		for l312 in 0..16384 {
			self.fVec76[(l312) as usize] = 0.0;
		}
		for l313 in 0..2 {
			self.fVec77[(l313) as usize] = 0.0;
		}
		for l314 in 0..2 {
			self.fRec292[(l314) as usize] = 0.0;
		}
		for l315 in 0..2 {
			self.fRec291[(l315) as usize] = 0.0;
		}
		for l316 in 0..3 {
			self.fRec290[(l316) as usize] = 0.0;
		}
		for l317 in 0..3 {
			self.fRec289[(l317) as usize] = 0.0;
		}
		for l318 in 0..2 {
			self.fVec78[(l318) as usize] = 0.0;
		}
		for l319 in 0..2 {
			self.fRec288[(l319) as usize] = 0.0;
		}
		for l320 in 0..3 {
			self.fRec287[(l320) as usize] = 0.0;
		}
		for l321 in 0..3 {
			self.fRec286[(l321) as usize] = 0.0;
		}
		for l322 in 0..2 {
			self.fRec362[(l322) as usize] = 0.0;
		}
		for l323 in 0..3 {
			self.fRec361[(l323) as usize] = 0.0;
		}
		for l324 in 0..3 {
			self.fRec360[(l324) as usize] = 0.0;
		}
		for l325 in 0..2 {
			self.fRec366[(l325) as usize] = 0.0;
		}
		for l326 in 0..3 {
			self.fRec365[(l326) as usize] = 0.0;
		}
		for l327 in 0..3 {
			self.fRec364[(l327) as usize] = 0.0;
		}
		for l328 in 0..3 {
			self.fRec363[(l328) as usize] = 0.0;
		}
		for l329 in 0..1024 {
			self.fVec79[(l329) as usize] = 0.0;
		}
		for l330 in 0..2 {
			self.fRec285[(l330) as usize] = 0.0;
		}
		for l331 in 0..2 {
			self.fRec378[(l331) as usize] = 0.0;
		}
		for l332 in 0..16384 {
			self.fVec80[(l332) as usize] = 0.0;
		}
		for l333 in 0..16384 {
			self.fVec81[(l333) as usize] = 0.0;
		}
		for l334 in 0..2 {
			self.fVec82[(l334) as usize] = 0.0;
		}
		for l335 in 0..2 {
			self.fRec377[(l335) as usize] = 0.0;
		}
		for l336 in 0..2 {
			self.fRec376[(l336) as usize] = 0.0;
		}
		for l337 in 0..3 {
			self.fRec375[(l337) as usize] = 0.0;
		}
		for l338 in 0..3 {
			self.fRec374[(l338) as usize] = 0.0;
		}
		for l339 in 0..2 {
			self.fVec83[(l339) as usize] = 0.0;
		}
		for l340 in 0..2 {
			self.fRec373[(l340) as usize] = 0.0;
		}
		for l341 in 0..3 {
			self.fRec372[(l341) as usize] = 0.0;
		}
		for l342 in 0..3 {
			self.fRec371[(l342) as usize] = 0.0;
		}
		for l343 in 0..2 {
			self.fRec381[(l343) as usize] = 0.0;
		}
		for l344 in 0..3 {
			self.fRec380[(l344) as usize] = 0.0;
		}
		for l345 in 0..3 {
			self.fRec379[(l345) as usize] = 0.0;
		}
		for l346 in 0..2 {
			self.fRec385[(l346) as usize] = 0.0;
		}
		for l347 in 0..3 {
			self.fRec384[(l347) as usize] = 0.0;
		}
		for l348 in 0..3 {
			self.fRec383[(l348) as usize] = 0.0;
		}
		for l349 in 0..3 {
			self.fRec382[(l349) as usize] = 0.0;
		}
		for l350 in 0..1024 {
			self.fVec84[(l350) as usize] = 0.0;
		}
		for l351 in 0..2 {
			self.fRec370[(l351) as usize] = 0.0;
		}
		for l352 in 0..16384 {
			self.fVec85[(l352) as usize] = 0.0;
		}
		for l353 in 0..2 {
			self.fVec86[(l353) as usize] = 0.0;
		}
		for l354 in 0..2 {
			self.fRec369[(l354) as usize] = 0.0;
		}
		for l355 in 0..2 {
			self.fRec367[(l355) as usize] = 0.0;
		}
		for l356 in 0..2 {
			self.fRec387[(l356) as usize] = 0.0;
		}
		for l357 in 0..16384 {
			self.fVec87[(l357) as usize] = 0.0;
		}
		for l358 in 0..2 {
			self.fVec88[(l358) as usize] = 0.0;
		}
		for l359 in 0..2 {
			self.fRec386[(l359) as usize] = 0.0;
		}
		for l360 in 0..2 {
			self.fRec368[(l360) as usize] = 0.0;
		}
		for l361 in 0..16384 {
			self.fVec89[(l361) as usize] = 0.0;
		}
		for l362 in 0..2 {
			self.fVec90[(l362) as usize] = 0.0;
		}
		for l363 in 0..2 {
			self.fRec390[(l363) as usize] = 0.0;
		}
		for l364 in 0..2 {
			self.fRec388[(l364) as usize] = 0.0;
		}
		for l365 in 0..16384 {
			self.fVec91[(l365) as usize] = 0.0;
		}
		for l366 in 0..2 {
			self.fVec92[(l366) as usize] = 0.0;
		}
		for l367 in 0..2 {
			self.fRec391[(l367) as usize] = 0.0;
		}
		for l368 in 0..2 {
			self.fRec389[(l368) as usize] = 0.0;
		}
		for l369 in 0..16384 {
			self.fVec93[(l369) as usize] = 0.0;
		}
		for l370 in 0..2 {
			self.fVec94[(l370) as usize] = 0.0;
		}
		for l371 in 0..2 {
			self.fRec394[(l371) as usize] = 0.0;
		}
		for l372 in 0..2 {
			self.fRec392[(l372) as usize] = 0.0;
		}
		for l373 in 0..2 {
			self.fRec396[(l373) as usize] = 0.0;
		}
		for l374 in 0..16384 {
			self.fVec95[(l374) as usize] = 0.0;
		}
		for l375 in 0..2 {
			self.fVec96[(l375) as usize] = 0.0;
		}
		for l376 in 0..2 {
			self.fRec395[(l376) as usize] = 0.0;
		}
		for l377 in 0..2 {
			self.fRec393[(l377) as usize] = 0.0;
		}
		for l378 in 0..2 {
			self.fRec400[(l378) as usize] = 0.0;
		}
		for l379 in 0..16384 {
			self.fVec97[(l379) as usize] = 0.0;
		}
		for l380 in 0..2 {
			self.fVec98[(l380) as usize] = 0.0;
		}
		for l381 in 0..2 {
			self.fRec399[(l381) as usize] = 0.0;
		}
		for l382 in 0..2 {
			self.fRec397[(l382) as usize] = 0.0;
		}
		for l383 in 0..16384 {
			self.fVec99[(l383) as usize] = 0.0;
		}
		for l384 in 0..2 {
			self.fVec100[(l384) as usize] = 0.0;
		}
		for l385 in 0..2 {
			self.fRec401[(l385) as usize] = 0.0;
		}
		for l386 in 0..2 {
			self.fRec398[(l386) as usize] = 0.0;
		}
		for l387 in 0..2 {
			self.fRec283[(l387) as usize] = 0.0;
		}
		for l388 in 0..2 {
			self.fRec284[(l388) as usize] = 0.0;
		}
		for l389 in 0..2 {
			self.fRec402[(l389) as usize] = 0.0;
		}
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		self.fConst0 = F32::min(1.92e+05, F32::max(1.0, ((self.fSampleRate) as F32)));
		self.fConst1 = 44.1 / self.fConst0;
		self.fConst2 = 1.0 - self.fConst1;
		self.fConst3 = 2764.6016 / self.fConst0;
		self.fConst4 = 0.00882353 * self.fConst0;
		self.fConst5 = 0.00073529413 * self.fConst0;
		self.fConst6 = 6911.504 / self.fConst0;
		self.fConst7 = 0.002 * self.fConst0;
		self.fConst8 = F32::exp(0.0 - 1e+04 / self.fConst0);
		self.fConst9 = 1.0 - self.fConst8;
		self.iConst10 = ((0.1 * self.fConst0) as i32);
		self.fConst11 = F32::exp(0.0 - 5e+01 / self.fConst0);
		self.fConst12 = F32::exp(0.0 - 1e+01 / self.fConst0);
		self.fConst13 = 19404.0 / self.fConst0;
		self.fConst14 = 3.1415927 / self.fConst0;
		self.fConst15 = 1.0 / self.fConst0;
		self.fConst16 = 0.5 * self.fConst0;
		self.fConst17 = 0.25 * self.fConst0;
		self.fConst18 = 352.0 / self.fConst0;
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
		ui_interface.declare(Some(ParamIndex(11)), "0", "");
		ui_interface.add_button("gate", ParamIndex(11));
		ui_interface.declare(Some(ParamIndex(12)), "1", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(12), 8e+01, 0.0, 127.0, 0.001);
		ui_interface.declare(Some(ParamIndex(13)), "2", "");
		ui_interface.add_horizontal_slider("mute", ParamIndex(13), 1.0, 0.9, 1.0, 0.001);
		ui_interface.declare(None, "3", "");
		ui_interface.open_vertical_box("0");
		ui_interface.declare(Some(ParamIndex(14)), "0", "");
		ui_interface.add_button("gate", ParamIndex(14));
		ui_interface.declare(Some(ParamIndex(15)), "1", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(15), 8e+01, 0.0, 127.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "3", "");
		ui_interface.open_vertical_box("1");
		ui_interface.declare(Some(ParamIndex(16)), "0", "");
		ui_interface.add_button("gate", ParamIndex(16));
		ui_interface.declare(Some(ParamIndex(17)), "1", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(17), 8e+01, 0.0, 127.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "3", "");
		ui_interface.open_vertical_box("2");
		ui_interface.declare(Some(ParamIndex(18)), "0", "");
		ui_interface.add_button("gate", ParamIndex(18));
		ui_interface.declare(Some(ParamIndex(19)), "1", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(19), 8e+01, 0.0, 127.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "3", "");
		ui_interface.open_vertical_box("3");
		ui_interface.declare(Some(ParamIndex(20)), "0", "");
		ui_interface.add_button("gate", ParamIndex(20));
		ui_interface.declare(Some(ParamIndex(21)), "1", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(21), 8e+01, 0.0, 127.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(Some(ParamIndex(22)), "3", "");
		ui_interface.add_horizontal_slider("pitchBend", ParamIndex(22), 0.0, -1.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "2", "");
		ui_interface.open_horizontal_box("drone");
		ui_interface.declare(Some(ParamIndex(23)), "0", "");
		ui_interface.add_horizontal_slider("volume", ParamIndex(23), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(24)), "1", "");
		ui_interface.add_horizontal_slider("note", ParamIndex(24), 6e+01, 0.0, 127.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "2", "");
		ui_interface.open_horizontal_box("fx");
		ui_interface.declare(None, "0", "");
		ui_interface.open_vertical_box("echo");
		ui_interface.declare(Some(ParamIndex(25)), "0", "");
		ui_interface.declare(Some(ParamIndex(25)), "scale", "log");
		ui_interface.add_horizontal_slider("duration", ParamIndex(25), 0.3, 0.01, 3.0, 0.001);
		ui_interface.declare(Some(ParamIndex(26)), "0", "");
		ui_interface.add_horizontal_slider("mix", ParamIndex(26), 1.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(27)), "1", "");
		ui_interface.add_horizontal_slider("feedback", ParamIndex(27), 0.3, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.declare(None, "1", "");
		ui_interface.open_vertical_box("reverb");
		ui_interface.declare(Some(ParamIndex(28)), "0", "");
		ui_interface.add_horizontal_slider("mix", ParamIndex(28), 0.11, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(29)), "1", "");
		ui_interface.add_horizontal_slider("time", ParamIndex(29), 3.5, 0.1, 6e+01, 0.001);
		ui_interface.declare(Some(ParamIndex(30)), "2", "");
		ui_interface.add_horizontal_slider("damp", ParamIndex(30), 0.88, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(31)), "3", "");
		ui_interface.add_horizontal_slider("size", ParamIndex(31), 5.0, 0.5, 5.0, 0.001);
		ui_interface.declare(Some(ParamIndex(32)), "4", "");
		ui_interface.add_horizontal_slider("early_diff", ParamIndex(32), 0.75, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(33)), "5", "");
		ui_interface.add_horizontal_slider("mod_depth", ParamIndex(33), 0.98, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(34)), "6", "");
		ui_interface.add_horizontal_slider("mod_freq", ParamIndex(34), 0.6, 0.0, 1e+01, 0.001);
		ui_interface.close_box();
		ui_interface.close_box();
		ui_interface.declare(None, "3", "");
		ui_interface.open_vertical_box("mix");
		ui_interface.declare(Some(ParamIndex(35)), "0", "");
		ui_interface.add_horizontal_slider("master", ParamIndex(35), 1.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(36)), "1", "");
		ui_interface.add_horizontal_slider("drone", ParamIndex(36), 1.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(37)), "2", "");
		ui_interface.add_horizontal_slider("lead", ParamIndex(37), 1.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(38)), "3", "");
		ui_interface.add_horizontal_slider("pluck", ParamIndex(38), 1.0, 0.0, 1.0, 0.001);
		ui_interface.close_box();
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			14 => Some(self.fButton0),
			11 => Some(self.fButton1),
			16 => Some(self.fButton2),
			18 => Some(self.fButton3),
			20 => Some(self.fButton4),
			15 => Some(self.fHslider0),
			22 => Some(self.fHslider1),
			9 => Some(self.fHslider10),
			10 => Some(self.fHslider11),
			7 => Some(self.fHslider12),
			8 => Some(self.fHslider13),
			5 => Some(self.fHslider14),
			6 => Some(self.fHslider15),
			3 => Some(self.fHslider16),
			4 => Some(self.fHslider17),
			37 => Some(self.fHslider18),
			0 => Some(self.fHslider19),
			13 => Some(self.fHslider2),
			24 => Some(self.fHslider20),
			36 => Some(self.fHslider21),
			23 => Some(self.fHslider22),
			26 => Some(self.fHslider23),
			25 => Some(self.fHslider24),
			27 => Some(self.fHslider25),
			28 => Some(self.fHslider26),
			31 => Some(self.fHslider27),
			34 => Some(self.fHslider28),
			33 => Some(self.fHslider29),
			12 => Some(self.fHslider3),
			29 => Some(self.fHslider30),
			30 => Some(self.fHslider31),
			32 => Some(self.fHslider32),
			35 => Some(self.fHslider33),
			17 => Some(self.fHslider4),
			19 => Some(self.fHslider5),
			21 => Some(self.fHslider6),
			38 => Some(self.fHslider7),
			2 => Some(self.fHslider8),
			1 => Some(self.fHslider9),
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			14 => { self.fButton0 = value }
			11 => { self.fButton1 = value }
			16 => { self.fButton2 = value }
			18 => { self.fButton3 = value }
			20 => { self.fButton4 = value }
			15 => { self.fHslider0 = value }
			22 => { self.fHslider1 = value }
			9 => { self.fHslider10 = value }
			10 => { self.fHslider11 = value }
			7 => { self.fHslider12 = value }
			8 => { self.fHslider13 = value }
			5 => { self.fHslider14 = value }
			6 => { self.fHslider15 = value }
			3 => { self.fHslider16 = value }
			4 => { self.fHslider17 = value }
			37 => { self.fHslider18 = value }
			0 => { self.fHslider19 = value }
			13 => { self.fHslider2 = value }
			24 => { self.fHslider20 = value }
			36 => { self.fHslider21 = value }
			23 => { self.fHslider22 = value }
			26 => { self.fHslider23 = value }
			25 => { self.fHslider24 = value }
			27 => { self.fHslider25 = value }
			28 => { self.fHslider26 = value }
			31 => { self.fHslider27 = value }
			34 => { self.fHslider28 = value }
			33 => { self.fHslider29 = value }
			12 => { self.fHslider3 = value }
			29 => { self.fHslider30 = value }
			30 => { self.fHslider31 = value }
			32 => { self.fHslider32 = value }
			35 => { self.fHslider33 = value }
			17 => { self.fHslider4 = value }
			19 => { self.fHslider5 = value }
			21 => { self.fHslider6 = value }
			38 => { self.fHslider7 = value }
			2 => { self.fHslider8 = value }
			1 => { self.fHslider9 = value }
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
		let mut fSlow0: F32 = self.fButton0;
		let mut iSlow1: i32 = ((1.0 - fSlow0) as i32);
		let mut fSlow2: F32 = self.fHslider0;
		let mut fSlow3: F32 = self.fConst1 * self.fHslider1;
		let mut fSlow4: F32 = self.fHslider2;
		let mut fSlow5: F32 = self.fConst1 * self.fHslider3;
		let mut fSlow6: F32 = self.fButton1;
		let mut fSlow7: F32 = self.fButton2;
		let mut iSlow8: i32 = ((1.0 - fSlow7) as i32);
		let mut fSlow9: F32 = self.fHslider4;
		let mut fSlow10: F32 = self.fButton3;
		let mut iSlow11: i32 = ((1.0 - fSlow10) as i32);
		let mut fSlow12: F32 = self.fHslider5;
		let mut fSlow13: F32 = self.fButton4;
		let mut iSlow14: i32 = ((1.0 - fSlow13) as i32);
		let mut fSlow15: F32 = self.fHslider6;
		let mut fSlow16: F32 = self.fConst1 * self.fHslider7;
		let mut fSlow17: F32 = self.fConst1 * self.fHslider8;
		let mut fSlow18: F32 = self.fConst1 * self.fHslider9;
		let mut fSlow19: F32 = self.fHslider10;
		let mut fSlow20: F32 = self.fConst13 * F32::powf(2.0, 0.083333336 * (fSlow19 + -69.0));
		let mut fSlow21: F32 = self.fConst1 * self.fHslider11;
		let mut fSlow22: F32 = self.fHslider12;
		let mut fSlow23: F32 = self.fConst13 * F32::powf(2.0, 0.083333336 * (fSlow22 + -69.0));
		let mut fSlow24: F32 = self.fConst1 * self.fHslider13;
		let mut fSlow25: F32 = self.fHslider14;
		let mut fSlow26: F32 = self.fConst13 * F32::powf(2.0, 0.083333336 * (fSlow25 + -69.0));
		let mut fSlow27: F32 = self.fConst1 * self.fHslider15;
		let mut fSlow28: F32 = self.fHslider16;
		let mut fSlow29: F32 = self.fConst13 * F32::powf(2.0, 0.083333336 * (fSlow28 + -69.0));
		let mut fSlow30: F32 = self.fConst1 * self.fHslider17;
		let mut fSlow31: F32 = self.fConst1 * self.fHslider18;
		let mut fSlow32: F32 = self.fConst1 * self.fHslider19;
		let mut fSlow33: F32 = self.fConst1 * self.fHslider20;
		let mut fSlow34: F32 = self.fConst1 * self.fHslider21;
		let mut fSlow35: F32 = self.fConst1 * self.fHslider22;
		let mut fSlow36: F32 = self.fConst1 * self.fHslider23;
		let mut fSlow37: F32 = self.fConst1 * self.fHslider24;
		let mut fSlow38: F32 = self.fHslider25;
		let mut fSlow39: F32 = self.fHslider26;
		let mut fSlow40: F32 = 1.0 - fSlow39;
		let mut fSlow41: F32 = self.fHslider27;
		let mut iSlow42: i32 = unsafe { itbl0mydspSIG0[(((134.0 * fSlow41) as i32)) as usize] };
		let mut fSlow43: F32 = 0.005 * ((iSlow42) as F32);
		let mut iSlow44: i32 = unsafe { itbl0mydspSIG0[(((54.0 * fSlow41) as i32)) as usize] };
		let mut fSlow45: F32 = 0.005 * ((iSlow44) as F32);
		let mut iSlow46: i32 = unsafe { itbl0mydspSIG0[(((1e+01 * fSlow41) as i32)) as usize] };
		let mut fSlow47: F32 = 0.0001 * ((iSlow46) as F32);
		let mut iSlow48: i32 = unsafe { itbl0mydspSIG0[(((1.1e+02 * fSlow41) as i32)) as usize] };
		let mut fSlow49: F32 = 0.0001 * ((iSlow48) as F32);
		let mut iSlow50: i32 = unsafe { itbl0mydspSIG0[(((4e+01 * fSlow41) as i32)) as usize] };
		let mut fSlow51: F32 = 0.0001 * ((iSlow50) as F32);
		let mut iSlow52: i32 = unsafe { itbl0mydspSIG0[(((1.4e+02 * fSlow41) as i32)) as usize] };
		let mut fSlow53: F32 = 0.0001 * ((iSlow52) as F32);
		let mut iSlow54: i32 = unsafe { itbl0mydspSIG0[(((7e+01 * fSlow41) as i32)) as usize] };
		let mut fSlow55: F32 = 0.0001 * ((iSlow54) as F32);
		let mut iSlow56: i32 = unsafe { itbl0mydspSIG0[(((1.7e+02 * fSlow41) as i32)) as usize] };
		let mut fSlow57: F32 = 0.0001 * ((iSlow56) as F32);
		let mut iSlow58: i32 = unsafe { itbl0mydspSIG0[(((1e+02 * fSlow41) as i32)) as usize] };
		let mut fSlow59: F32 = 0.0001 * ((iSlow58) as F32);
		let mut iSlow60: i32 = unsafe { itbl0mydspSIG0[(((2e+02 * fSlow41) as i32)) as usize] };
		let mut fSlow61: F32 = 0.0001 * ((iSlow60) as F32);
		let mut iSlow62: i32 = unsafe { itbl0mydspSIG0[(((1.3e+02 * fSlow41) as i32)) as usize] };
		let mut fSlow63: F32 = 0.0001 * ((iSlow62) as F32);
		let mut iSlow64: i32 = unsafe { itbl0mydspSIG0[(((2.3e+02 * fSlow41) as i32)) as usize] };
		let mut fSlow65: F32 = 0.0001 * ((iSlow64) as F32);
		let mut fSlow66: F32 = self.fConst38 * self.fHslider28;
		let mut fSlow67: F32 = F32::cos(fSlow66);
		let mut fSlow68: F32 = F32::sin(fSlow66);
		let mut fSlow69: F32 = 5e+01 * self.fHslider29;
		let mut iSlow70: i32 = unsafe { itbl0mydspSIG0[(((125.0 * fSlow41) as i32)) as usize] };
		let mut fSlow71: F32 = 0.0001 * ((iSlow70) as F32);
		let mut iSlow72: i32 = unsafe { itbl0mydspSIG0[(((204.0 * fSlow41) as i32)) as usize] };
		let mut fSlow73: F32 = 0.005 * ((iSlow72) as F32);
		let mut fSlow74: F32 = 0.0 - fSlow69;
		let mut iSlow75: i32 = unsafe { itbl0mydspSIG0[(((25.0 * fSlow41) as i32)) as usize] };
		let mut fSlow76: F32 = 0.0001 * ((iSlow75) as F32);
		let mut iSlow77: i32 = unsafe { itbl0mydspSIG0[(((155.0 * fSlow41) as i32)) as usize] };
		let mut fSlow78: F32 = 0.0001 * ((iSlow77) as F32);
		let mut iSlow79: i32 = unsafe { itbl0mydspSIG0[(((55.0 * fSlow41) as i32)) as usize] };
		let mut fSlow80: F32 = 0.0001 * ((iSlow79) as F32);
		let mut iSlow81: i32 = unsafe { itbl0mydspSIG0[(((185.0 * fSlow41) as i32)) as usize] };
		let mut fSlow82: F32 = 0.0001 * ((iSlow81) as F32);
		let mut iSlow83: i32 = unsafe { itbl0mydspSIG0[(((85.0 * fSlow41) as i32)) as usize] };
		let mut fSlow84: F32 = 0.0001 * ((iSlow83) as F32);
		let mut iSlow85: i32 = unsafe { itbl0mydspSIG0[(((215.0 * fSlow41) as i32)) as usize] };
		let mut fSlow86: F32 = 0.0001 * ((iSlow85) as F32);
		let mut iSlow87: i32 = unsafe { itbl0mydspSIG0[(((115.0 * fSlow41) as i32)) as usize] };
		let mut fSlow88: F32 = 0.0001 * ((iSlow87) as F32);
		let mut iSlow89: i32 = unsafe { itbl0mydspSIG0[(((245.0 * fSlow41) as i32)) as usize] };
		let mut fSlow90: F32 = 0.0001 * ((iSlow89) as F32);
		let mut iSlow91: i32 = unsafe { itbl0mydspSIG0[(((145.0 * fSlow41) as i32)) as usize] };
		let mut fSlow92: F32 = 0.0001 * ((iSlow91) as F32);
		let mut fSlow93: F32 = F32::powf(1e+01, 0.0 - 0.51 * ((1.25 * fSlow41 + -0.25) / self.fHslider30));
		let mut fSlow94: F32 = self.fHslider31;
		let mut fSlow95: F32 = 1.0 - fSlow94;
		let mut fSlow96: F32 = self.fHslider32;
		let mut fSlow97: F32 = F32::sin(fSlow96);
		let mut iSlow98: i32 = unsafe { itbl0mydspSIG0[(((34.0 * fSlow41) as i32)) as usize] };
		let mut fSlow99: F32 = 0.005 * ((iSlow98) as F32);
		let mut fSlow100: F32 = F32::cos(fSlow96);
		let mut iSlow101: i32 = unsafe { itbl0mydspSIG0[(((2.4e+02 * fSlow41) as i32)) as usize] };
		let mut fSlow102: F32 = 0.0001 * ((iSlow101) as F32);
		let mut iSlow103: i32 = unsafe { itbl0mydspSIG0[(((1.9e+02 * fSlow41) as i32)) as usize] };
		let mut fSlow104: F32 = 0.0001 * ((iSlow103) as F32);
		let mut iSlow105: i32 = unsafe { itbl0mydspSIG0[(((175.0 * fSlow41) as i32)) as usize] };
		let mut fSlow106: F32 = 0.0001 * ((iSlow105) as F32);
		let mut fSlow107: F32 = self.fConst1 * self.fHslider33;
		let zipped_iterators = outputs0.zip(outputs1);
		for (output0, output1) in zipped_iterators {
			self.fVec0[0] = fSlow0;
			self.iVec1[0] = 1;
			self.fRec2[0] = if (iSlow1 as i32 != 0) { fSlow2 } else { self.fRec2[1] };
			self.fRec1[0] = self.fConst2 * self.fRec1[1] + self.fConst1 * self.fRec2[0];
			self.fRec3[0] = fSlow3 + self.fConst2 * self.fRec3[1];
			let mut fTemp0: F32 = F32::powf(2.0, 0.083333336 * (self.fRec3[0] + self.fRec1[0] + -69.0));
			let mut fTemp1: F32 = 1.0 / F32::tan(self.fConst3 * fTemp0);
			let mut fRec18: F32 = -1.0 * 0.9973053 * (0.9 * self.fRec19[2] + 0.05 * (self.fRec19[1] + self.fRec19[3]));
			let mut fTemp2: F32 = self.fConst5 * (0.77272725 / fTemp0 + -0.11);
			let mut fTemp3: F32 = fTemp2 + -1.499995;
			let mut iTemp4: i32 = ((fTemp3) as i32);
			let mut iTemp5: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp4, 4))) as F32))) as i32);
			let mut iTemp6: i32 = i32::wrapping_add(iTemp5, 1);
			let mut fTemp7: F32 = F32::floor(fTemp3);
			let mut fTemp8: F32 = fTemp2 + (-3.0 - fTemp7);
			let mut fTemp9: F32 = fTemp2 + (-2.0 - fTemp7);
			let mut fTemp10: F32 = fTemp2 + (-1.0 - fTemp7);
			let mut fTemp11: F32 = fTemp10 * fTemp9;
			let mut fTemp12: F32 = fTemp11 * fTemp8;
			let mut fTemp13: F32 = fTemp2 + (-4.0 - fTemp7);
			let mut fTemp14: F32 = 0.0 - fTemp13;
			let mut iTemp15: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp4, 3))) as F32))) as i32);
			let mut iTemp16: i32 = i32::wrapping_add(iTemp15, 1);
			let mut fTemp17: F32 = 0.0 - 0.5 * fTemp13;
			let mut fTemp18: F32 = 0.0 - fTemp8;
			let mut iTemp19: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp4, 2))) as F32))) as i32);
			let mut iTemp20: i32 = i32::wrapping_add(iTemp19, 1);
			let mut fTemp21: F32 = 0.0 - 0.33333334 * fTemp13;
			let mut fTemp22: F32 = 0.0 - 0.5 * fTemp8;
			let mut fTemp23: F32 = 0.0 - fTemp9;
			let mut iTemp24: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp4, 1))) as F32))) as i32);
			let mut iTemp25: i32 = i32::wrapping_add(iTemp24, 1);
			let mut fTemp26: F32 = fTemp2 - fTemp7;
			let mut fTemp27: F32 = 0.0 - 0.25 * fTemp13;
			let mut fTemp28: F32 = 0.0 - 0.33333334 * fTemp8;
			let mut fTemp29: F32 = 0.0 - 0.5 * fTemp9;
			let mut fTemp30: F32 = 0.0 - fTemp10;
			let mut iTemp31: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, iTemp4)) as F32))) as i32);
			let mut iTemp32: i32 = i32::wrapping_add(iTemp31, 1);
			self.fRec33[0] = self.fRec10[((i32::wrapping_sub(self.IOTA0, iTemp32)) & 2047) as usize] * fTemp30 * fTemp29 * fTemp28 * fTemp27 + fTemp26 * (self.fRec10[((i32::wrapping_sub(self.IOTA0, iTemp25)) & 2047) as usize] * fTemp23 * fTemp22 * fTemp21 + 0.5 * fTemp10 * self.fRec10[((i32::wrapping_sub(self.IOTA0, iTemp20)) & 2047) as usize] * fTemp18 * fTemp17 + 0.16666667 * fTemp11 * self.fRec10[((i32::wrapping_sub(self.IOTA0, iTemp16)) & 2047) as usize] * fTemp14 + 0.041666668 * fTemp12 * self.fRec10[((i32::wrapping_sub(self.IOTA0, iTemp6)) & 2047) as usize]);
			self.fRec37[0] = 0.05 * self.fRec37[1] + 0.95 * self.fRec33[1];
			let mut fRec34: F32 = self.fRec37[0];
			let mut fTemp33: F32 = F32::tan(self.fConst6 * fTemp0);
			let mut fTemp34: F32 = 1.0 / fTemp33;
			let mut fTemp35: F32 = (fTemp34 + 1.4142135) / fTemp33 + 1.0;
			self.iRec39[0] = i32::wrapping_add(i32::wrapping_mul(i32::wrapping_add(self.iRec39[1], ((self.iRec39[1] > 0) as i32)), ((fSlow0 <= self.fVec0[1]) as i32)), ((fSlow0 > self.fVec0[1]) as i32));
			let mut fTemp36: F32 = ((self.iRec39[0]) as F32) / F32::max(1.0, self.fConst7 * mydsp_faustpower2_f(1.0 - 0.22 * fTemp0));
			self.iRec41[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, self.iRec41[1]), 12345);
			let mut fTemp37: F32 = 4.656613e-10 * ((self.iRec41[0]) as F32);
			self.fRec40[0] = fTemp37 - (self.fRec40[2] * ((fTemp34 + -1.4142135) / fTemp33 + 1.0) + 2.0 * self.fRec40[1] * (1.0 - 1.0 / mydsp_faustpower2_f(fTemp33))) / fTemp35;
			self.fRec45[0] = self.fConst8 * self.fRec45[1] + self.fConst9 * F32::abs(self.fRec4[1]);
			let mut fRec44: F32 = self.fRec45[0];
			let mut iTemp38: i32 = ((fRec44 > 0.1) as i32);
			self.iVec2[0] = iTemp38;
			self.iRec46[0] = std::cmp::max(i32::wrapping_mul(self.iConst10, ((iTemp38 < self.iVec2[1]) as i32)), i32::wrapping_add(self.iRec46[1], -1));
			let mut fTemp39: F32 = F32::abs(F32::max(((iTemp38) as F32), ((((self.iRec46[0] > 0) as i32)) as F32)));
			let mut fTemp40: F32 = if (((self.fRec42[1] > fTemp39) as i32) as i32 != 0) { self.fConst11 } else { self.fConst8 };
			self.fRec43[0] = self.fRec43[1] * fTemp40 + fTemp39 * (1.0 - fTemp40);
			self.fRec42[0] = self.fRec43[0];
			let mut fTemp41: F32 = 0.005 * self.fRec42[0] * self.fRec4[1] + 0.5 * ((self.fRec40[2] + self.fRec40[0] + 2.0 * self.fRec40[1]) * F32::max(0.0, F32::min(fTemp36, 2.0 - fTemp36)) / fTemp35);
			self.fRec47[0] = self.fRec8[1];
			self.fRec48[(self.IOTA0 & 2047) as usize] = -1.0 * 0.9973053 * (0.9 * self.fRec47[2] + 0.05 * (self.fRec47[1] + self.fRec47[3]));
			let mut fTemp42: F32 = fTemp11 * fTemp14;
			let mut fTemp43: F32 = fTemp10 * fTemp18 * fTemp17;
			let mut fTemp44: F32 = fTemp23 * fTemp22 * fTemp21;
			let mut fTemp45: F32 = fTemp30 * fTemp29 * fTemp28 * fTemp27;
			self.fVec3[0] = fTemp45 * self.fRec48[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp31, 2))) & 2047) as usize] + fTemp26 * (fTemp44 * self.fRec48[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp24, 2))) & 2047) as usize] + 0.5 * fTemp43 * self.fRec48[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp19, 2))) & 2047) as usize] + 0.16666667 * fTemp42 * self.fRec48[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp15, 2))) & 2047) as usize] + 0.041666668 * fTemp12 * self.fRec48[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp5, 2))) & 2047) as usize]);
			let mut fTemp46: F32 = self.fVec3[1] + fTemp41;
			self.fVec4[0] = fTemp46;
			self.fRec38[(self.IOTA0 & 2047) as usize] = 0.05 * self.fRec38[((i32::wrapping_sub(self.IOTA0, 1)) & 2047) as usize] + 0.95 * self.fVec4[2];
			let mut fRec35: F32 = fTemp45 * self.fRec38[((i32::wrapping_sub(self.IOTA0, iTemp31)) & 2047) as usize] + fTemp26 * (fTemp44 * self.fRec38[((i32::wrapping_sub(self.IOTA0, iTemp24)) & 2047) as usize] + 0.5 * fTemp43 * self.fRec38[((i32::wrapping_sub(self.IOTA0, iTemp19)) & 2047) as usize] + 0.16666667 * fTemp42 * self.fRec38[((i32::wrapping_sub(self.IOTA0, iTemp15)) & 2047) as usize] + 0.041666668 * fTemp12 * self.fRec38[((i32::wrapping_sub(self.IOTA0, iTemp5)) & 2047) as usize]);
			let mut fRec36: F32 = self.fVec4[1] + self.fRec29[1];
			self.fRec29[0] = fRec34;
			let mut fRec30: F32 = self.fRec29[1];
			let mut fRec31: F32 = fRec35;
			let mut fRec32: F32 = fRec36;
			self.fRec25[0] = fRec30;
			let mut fRec26: F32 = fTemp41 + self.fRec25[1];
			let mut fRec27: F32 = fRec31;
			let mut fRec28: F32 = fRec32;
			self.fRec21[(self.IOTA0 & 2047) as usize] = fRec26;
			let mut fRec22: F32 = fTemp45 * self.fRec21[((i32::wrapping_sub(self.IOTA0, iTemp32)) & 2047) as usize] + fTemp26 * (fTemp44 * self.fRec21[((i32::wrapping_sub(self.IOTA0, iTemp25)) & 2047) as usize] + 0.5 * fTemp43 * self.fRec21[((i32::wrapping_sub(self.IOTA0, iTemp20)) & 2047) as usize] + 0.16666667 * fTemp42 * self.fRec21[((i32::wrapping_sub(self.IOTA0, iTemp16)) & 2047) as usize] + 0.041666668 * fTemp12 * self.fRec21[((i32::wrapping_sub(self.IOTA0, iTemp6)) & 2047) as usize]);
			self.fRec23[0] = fRec27;
			let mut fRec24: F32 = fRec28;
			self.fRec19[0] = fSlow4 * self.fRec23[1];
			let mut fRec20: F32 = fRec24;
			self.fRec14[0] = fRec18;
			let mut fRec15: F32 = fSlow4 * self.fRec14[1];
			let mut fRec16: F32 = self.fRec19[0];
			let mut fRec17: F32 = fRec20;
			self.fRec10[(self.IOTA0 & 2047) as usize] = fRec15;
			let mut fRec11: F32 = fRec22;
			let mut fRec12: F32 = fRec16;
			let mut fRec13: F32 = fRec17;
			self.fRec8[0] = fRec11;
			let mut fRec9: F32 = fRec13;
			let mut fTemp47: F32 = F32::abs(fRec9);
			let mut fTemp48: F32 = if (((self.fRec6[1] > fTemp47) as i32) as i32 != 0) { self.fConst12 } else { 0.0 };
			self.fRec7[0] = self.fRec7[1] * fTemp48 + fTemp47 * (1.0 - fTemp48);
			self.fRec6[0] = self.fRec7[0];
			let mut fRec5: F32 = 0.0 - 0.95 * F32::max(2e+01 * F32::log10(self.fRec6[0]) + 1e+01, 0.0);
			self.fRec4[0] = fRec9 * F32::powf(1e+01, 0.05 * fRec5);
			self.fRec0[0] = 0.0 - (self.fRec0[1] * (1.0 - fTemp1) - (self.fRec4[0] + self.fRec4[1])) / (fTemp1 + 1.0);
			self.fRec50[0] = fSlow5 + self.fConst2 * self.fRec50[1];
			let mut fTemp49: F32 = F32::powf(2.0, 0.083333336 * (self.fRec50[0] + self.fRec3[0] + -69.0));
			let mut fTemp50: F32 = 1.0 / F32::tan(self.fConst3 * fTemp49);
			let mut fRec65: F32 = -1.0 * 0.9973053 * (0.9 * self.fRec66[2] + 0.05 * (self.fRec66[1] + self.fRec66[3]));
			let mut fTemp51: F32 = self.fConst5 * (0.77272725 / fTemp49 + -0.11);
			let mut fTemp52: F32 = fTemp51 + -1.499995;
			let mut iTemp53: i32 = ((fTemp52) as i32);
			let mut iTemp54: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp53, 4))) as F32))) as i32);
			let mut iTemp55: i32 = i32::wrapping_add(iTemp54, 1);
			let mut fTemp56: F32 = F32::floor(fTemp52);
			let mut fTemp57: F32 = fTemp51 + (-3.0 - fTemp56);
			let mut fTemp58: F32 = fTemp51 + (-2.0 - fTemp56);
			let mut fTemp59: F32 = fTemp51 + (-1.0 - fTemp56);
			let mut fTemp60: F32 = fTemp59 * fTemp58;
			let mut fTemp61: F32 = fTemp60 * fTemp57;
			let mut fTemp62: F32 = fTemp51 + (-4.0 - fTemp56);
			let mut fTemp63: F32 = 0.0 - fTemp62;
			let mut iTemp64: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp53, 3))) as F32))) as i32);
			let mut iTemp65: i32 = i32::wrapping_add(iTemp64, 1);
			let mut fTemp66: F32 = 0.0 - 0.5 * fTemp62;
			let mut fTemp67: F32 = 0.0 - fTemp57;
			let mut iTemp68: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp53, 2))) as F32))) as i32);
			let mut iTemp69: i32 = i32::wrapping_add(iTemp68, 1);
			let mut fTemp70: F32 = 0.0 - 0.33333334 * fTemp62;
			let mut fTemp71: F32 = 0.0 - 0.5 * fTemp57;
			let mut fTemp72: F32 = 0.0 - fTemp58;
			let mut iTemp73: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp53, 1))) as F32))) as i32);
			let mut iTemp74: i32 = i32::wrapping_add(iTemp73, 1);
			let mut fTemp75: F32 = fTemp51 - fTemp56;
			let mut fTemp76: F32 = 0.0 - 0.25 * fTemp62;
			let mut fTemp77: F32 = 0.0 - 0.33333334 * fTemp57;
			let mut fTemp78: F32 = 0.0 - 0.5 * fTemp58;
			let mut fTemp79: F32 = 0.0 - fTemp59;
			let mut iTemp80: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, iTemp53)) as F32))) as i32);
			let mut iTemp81: i32 = i32::wrapping_add(iTemp80, 1);
			self.fRec80[0] = self.fRec57[((i32::wrapping_sub(self.IOTA0, iTemp81)) & 2047) as usize] * fTemp79 * fTemp78 * fTemp77 * fTemp76 + fTemp75 * (self.fRec57[((i32::wrapping_sub(self.IOTA0, iTemp74)) & 2047) as usize] * fTemp72 * fTemp71 * fTemp70 + 0.5 * fTemp59 * self.fRec57[((i32::wrapping_sub(self.IOTA0, iTemp69)) & 2047) as usize] * fTemp67 * fTemp66 + 0.16666667 * fTemp60 * self.fRec57[((i32::wrapping_sub(self.IOTA0, iTemp65)) & 2047) as usize] * fTemp63 + 0.041666668 * fTemp61 * self.fRec57[((i32::wrapping_sub(self.IOTA0, iTemp55)) & 2047) as usize]);
			self.fRec84[0] = 0.05 * self.fRec84[1] + 0.95 * self.fRec80[1];
			let mut fRec81: F32 = self.fRec84[0];
			self.fRec89[0] = self.fConst8 * self.fRec89[1] + self.fConst9 * F32::abs(self.fRec51[1]);
			let mut fRec88: F32 = self.fRec89[0];
			let mut iTemp82: i32 = ((fRec88 > 0.1) as i32);
			self.iVec5[0] = iTemp82;
			self.iRec90[0] = std::cmp::max(i32::wrapping_mul(self.iConst10, ((iTemp82 < self.iVec5[1]) as i32)), i32::wrapping_add(self.iRec90[1], -1));
			let mut fTemp83: F32 = F32::abs(F32::max(((iTemp82) as F32), ((((self.iRec90[0] > 0) as i32)) as F32)));
			let mut fTemp84: F32 = if (((self.fRec86[1] > fTemp83) as i32) as i32 != 0) { self.fConst11 } else { self.fConst8 };
			self.fRec87[0] = self.fRec87[1] * fTemp84 + fTemp83 * (1.0 - fTemp84);
			self.fRec86[0] = self.fRec87[0];
			let mut fTemp85: F32 = 0.005 * self.fRec86[0] * self.fRec51[1];
			self.fRec91[0] = self.fRec55[1];
			self.fRec92[(self.IOTA0 & 2047) as usize] = -1.0 * 0.9973053 * (0.9 * self.fRec91[2] + 0.05 * (self.fRec91[1] + self.fRec91[3]));
			let mut fTemp86: F32 = fTemp60 * fTemp63;
			let mut fTemp87: F32 = fTemp59 * fTemp67 * fTemp66;
			let mut fTemp88: F32 = fTemp72 * fTemp71 * fTemp70;
			let mut fTemp89: F32 = fTemp79 * fTemp78 * fTemp77 * fTemp76;
			self.fVec6[0] = fTemp89 * self.fRec92[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp80, 2))) & 2047) as usize] + fTemp75 * (fTemp88 * self.fRec92[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp73, 2))) & 2047) as usize] + 0.5 * fTemp87 * self.fRec92[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp68, 2))) & 2047) as usize] + 0.16666667 * fTemp86 * self.fRec92[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp64, 2))) & 2047) as usize] + 0.041666668 * fTemp61 * self.fRec92[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp54, 2))) & 2047) as usize]);
			let mut fTemp90: F32 = F32::tan(self.fConst6 * fTemp49);
			let mut fTemp91: F32 = 1.0 / fTemp90;
			let mut fTemp92: F32 = (fTemp91 + 1.4142135) / fTemp90 + 1.0;
			self.fVec7[0] = fSlow6;
			self.iRec93[0] = i32::wrapping_add(i32::wrapping_mul(i32::wrapping_add(self.iRec93[1], ((self.iRec93[1] > 0) as i32)), ((fSlow6 <= self.fVec7[1]) as i32)), ((fSlow6 > self.fVec7[1]) as i32));
			let mut fTemp93: F32 = ((self.iRec93[0]) as F32) / F32::max(1.0, self.fConst7 * mydsp_faustpower2_f(1.0 - 0.22 * fTemp49));
			self.fRec94[0] = fTemp37 - (self.fRec94[2] * ((fTemp91 + -1.4142135) / fTemp90 + 1.0) + 2.0 * self.fRec94[1] * (1.0 - 1.0 / mydsp_faustpower2_f(fTemp90))) / fTemp92;
			let mut fTemp94: F32 = 0.5 * ((self.fRec94[2] + self.fRec94[0] + 2.0 * self.fRec94[1]) * F32::max(0.0, F32::min(fTemp93, 2.0 - fTemp93)) / fTemp92);
			let mut fTemp95: F32 = fTemp94 + self.fVec6[1] + fTemp85;
			self.fVec8[0] = fTemp95;
			self.fRec85[(self.IOTA0 & 2047) as usize] = 0.05 * self.fRec85[((i32::wrapping_sub(self.IOTA0, 1)) & 2047) as usize] + 0.95 * self.fVec8[2];
			let mut fRec82: F32 = fTemp89 * self.fRec85[((i32::wrapping_sub(self.IOTA0, iTemp80)) & 2047) as usize] + fTemp75 * (fTemp88 * self.fRec85[((i32::wrapping_sub(self.IOTA0, iTemp73)) & 2047) as usize] + 0.5 * fTemp87 * self.fRec85[((i32::wrapping_sub(self.IOTA0, iTemp68)) & 2047) as usize] + 0.16666667 * fTemp86 * self.fRec85[((i32::wrapping_sub(self.IOTA0, iTemp64)) & 2047) as usize] + 0.041666668 * fTemp61 * self.fRec85[((i32::wrapping_sub(self.IOTA0, iTemp54)) & 2047) as usize]);
			let mut fRec83: F32 = self.fVec8[1] + self.fRec76[1];
			self.fRec76[0] = fRec81;
			let mut fRec77: F32 = self.fRec76[1];
			let mut fRec78: F32 = fRec82;
			let mut fRec79: F32 = fRec83;
			self.fRec72[0] = fRec77;
			let mut fRec73: F32 = fTemp85 + fTemp94 + self.fRec72[1];
			let mut fRec74: F32 = fRec78;
			let mut fRec75: F32 = fRec79;
			self.fRec68[(self.IOTA0 & 2047) as usize] = fRec73;
			let mut fRec69: F32 = fTemp89 * self.fRec68[((i32::wrapping_sub(self.IOTA0, iTemp81)) & 2047) as usize] + fTemp75 * (fTemp88 * self.fRec68[((i32::wrapping_sub(self.IOTA0, iTemp74)) & 2047) as usize] + 0.5 * fTemp87 * self.fRec68[((i32::wrapping_sub(self.IOTA0, iTemp69)) & 2047) as usize] + 0.16666667 * fTemp86 * self.fRec68[((i32::wrapping_sub(self.IOTA0, iTemp65)) & 2047) as usize] + 0.041666668 * fTemp61 * self.fRec68[((i32::wrapping_sub(self.IOTA0, iTemp55)) & 2047) as usize]);
			self.fRec70[0] = fRec74;
			let mut fRec71: F32 = fRec75;
			self.fRec66[0] = fSlow4 * self.fRec70[1];
			let mut fRec67: F32 = fRec71;
			self.fRec61[0] = fRec65;
			let mut fRec62: F32 = fSlow4 * self.fRec61[1];
			let mut fRec63: F32 = self.fRec66[0];
			let mut fRec64: F32 = fRec67;
			self.fRec57[(self.IOTA0 & 2047) as usize] = fRec62;
			let mut fRec58: F32 = fRec69;
			let mut fRec59: F32 = fRec63;
			let mut fRec60: F32 = fRec64;
			self.fRec55[0] = fRec58;
			let mut fRec56: F32 = fRec60;
			let mut fTemp96: F32 = F32::abs(fRec56);
			let mut fTemp97: F32 = if (((self.fRec53[1] > fTemp96) as i32) as i32 != 0) { self.fConst12 } else { 0.0 };
			self.fRec54[0] = self.fRec54[1] * fTemp97 + fTemp96 * (1.0 - fTemp97);
			self.fRec53[0] = self.fRec54[0];
			let mut fRec52: F32 = 0.0 - 0.95 * F32::max(2e+01 * F32::log10(self.fRec53[0]) + 1e+01, 0.0);
			self.fRec51[0] = fRec56 * F32::powf(1e+01, 0.05 * fRec52);
			self.fRec49[0] = 0.0 - (self.fRec49[1] * (1.0 - fTemp50) - (self.fRec51[0] + self.fRec51[1])) / (fTemp50 + 1.0);
			self.fVec9[0] = fSlow7;
			self.fRec97[0] = if (iSlow8 as i32 != 0) { fSlow9 } else { self.fRec97[1] };
			self.fRec96[0] = self.fConst2 * self.fRec96[1] + self.fConst1 * self.fRec97[0];
			let mut fTemp98: F32 = F32::powf(2.0, 0.083333336 * (self.fRec3[0] + self.fRec96[0] + -69.0));
			let mut fTemp99: F32 = 1.0 / F32::tan(self.fConst3 * fTemp98);
			let mut fRec112: F32 = -1.0 * 0.9973053 * (0.9 * self.fRec113[2] + 0.05 * (self.fRec113[1] + self.fRec113[3]));
			let mut fTemp100: F32 = self.fConst5 * (0.77272725 / fTemp98 + -0.11);
			let mut fTemp101: F32 = fTemp100 + -1.499995;
			let mut iTemp102: i32 = ((fTemp101) as i32);
			let mut iTemp103: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp102, 4))) as F32))) as i32);
			let mut iTemp104: i32 = i32::wrapping_add(iTemp103, 1);
			let mut fTemp105: F32 = F32::floor(fTemp101);
			let mut fTemp106: F32 = fTemp100 + (-3.0 - fTemp105);
			let mut fTemp107: F32 = fTemp100 + (-2.0 - fTemp105);
			let mut fTemp108: F32 = fTemp100 + (-1.0 - fTemp105);
			let mut fTemp109: F32 = fTemp108 * fTemp107;
			let mut fTemp110: F32 = fTemp109 * fTemp106;
			let mut fTemp111: F32 = fTemp100 + (-4.0 - fTemp105);
			let mut fTemp112: F32 = 0.0 - fTemp111;
			let mut iTemp113: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp102, 3))) as F32))) as i32);
			let mut iTemp114: i32 = i32::wrapping_add(iTemp113, 1);
			let mut fTemp115: F32 = 0.0 - 0.5 * fTemp111;
			let mut fTemp116: F32 = 0.0 - fTemp106;
			let mut iTemp117: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp102, 2))) as F32))) as i32);
			let mut iTemp118: i32 = i32::wrapping_add(iTemp117, 1);
			let mut fTemp119: F32 = 0.0 - 0.33333334 * fTemp111;
			let mut fTemp120: F32 = 0.0 - 0.5 * fTemp106;
			let mut fTemp121: F32 = 0.0 - fTemp107;
			let mut iTemp122: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp102, 1))) as F32))) as i32);
			let mut iTemp123: i32 = i32::wrapping_add(iTemp122, 1);
			let mut fTemp124: F32 = fTemp100 - fTemp105;
			let mut fTemp125: F32 = 0.0 - 0.25 * fTemp111;
			let mut fTemp126: F32 = 0.0 - 0.33333334 * fTemp106;
			let mut fTemp127: F32 = 0.0 - 0.5 * fTemp107;
			let mut fTemp128: F32 = 0.0 - fTemp108;
			let mut iTemp129: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, iTemp102)) as F32))) as i32);
			let mut iTemp130: i32 = i32::wrapping_add(iTemp129, 1);
			self.fRec127[0] = self.fRec104[((i32::wrapping_sub(self.IOTA0, iTemp130)) & 2047) as usize] * fTemp128 * fTemp127 * fTemp126 * fTemp125 + fTemp124 * (self.fRec104[((i32::wrapping_sub(self.IOTA0, iTemp123)) & 2047) as usize] * fTemp121 * fTemp120 * fTemp119 + 0.5 * fTemp108 * self.fRec104[((i32::wrapping_sub(self.IOTA0, iTemp118)) & 2047) as usize] * fTemp116 * fTemp115 + 0.16666667 * fTemp109 * self.fRec104[((i32::wrapping_sub(self.IOTA0, iTemp114)) & 2047) as usize] * fTemp112 + 0.041666668 * fTemp110 * self.fRec104[((i32::wrapping_sub(self.IOTA0, iTemp104)) & 2047) as usize]);
			self.fRec131[0] = 0.05 * self.fRec131[1] + 0.95 * self.fRec127[1];
			let mut fRec128: F32 = self.fRec131[0];
			self.fRec136[0] = self.fConst8 * self.fRec136[1] + self.fConst9 * F32::abs(self.fRec98[1]);
			let mut fRec135: F32 = self.fRec136[0];
			let mut iTemp131: i32 = ((fRec135 > 0.1) as i32);
			self.iVec10[0] = iTemp131;
			self.iRec137[0] = std::cmp::max(i32::wrapping_mul(self.iConst10, ((iTemp131 < self.iVec10[1]) as i32)), i32::wrapping_add(self.iRec137[1], -1));
			let mut fTemp132: F32 = F32::abs(F32::max(((iTemp131) as F32), ((((self.iRec137[0] > 0) as i32)) as F32)));
			let mut fTemp133: F32 = if (((self.fRec133[1] > fTemp132) as i32) as i32 != 0) { self.fConst11 } else { self.fConst8 };
			self.fRec134[0] = self.fRec134[1] * fTemp133 + fTemp132 * (1.0 - fTemp133);
			self.fRec133[0] = self.fRec134[0];
			let mut fTemp134: F32 = 0.005 * self.fRec133[0] * self.fRec98[1];
			self.fRec138[0] = self.fRec102[1];
			self.fRec139[(self.IOTA0 & 2047) as usize] = -1.0 * 0.9973053 * (0.9 * self.fRec138[2] + 0.05 * (self.fRec138[1] + self.fRec138[3]));
			let mut fTemp135: F32 = fTemp109 * fTemp112;
			let mut fTemp136: F32 = fTemp108 * fTemp116 * fTemp115;
			let mut fTemp137: F32 = fTemp121 * fTemp120 * fTemp119;
			let mut fTemp138: F32 = fTemp128 * fTemp127 * fTemp126 * fTemp125;
			self.fVec11[0] = fTemp138 * self.fRec139[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp129, 2))) & 2047) as usize] + fTemp124 * (fTemp137 * self.fRec139[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp122, 2))) & 2047) as usize] + 0.5 * fTemp136 * self.fRec139[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp117, 2))) & 2047) as usize] + 0.16666667 * fTemp135 * self.fRec139[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp113, 2))) & 2047) as usize] + 0.041666668 * fTemp110 * self.fRec139[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp103, 2))) & 2047) as usize]);
			let mut fTemp139: F32 = F32::tan(self.fConst6 * fTemp98);
			let mut fTemp140: F32 = 1.0 / fTemp139;
			let mut fTemp141: F32 = (fTemp140 + 1.4142135) / fTemp139 + 1.0;
			self.iRec140[0] = i32::wrapping_add(i32::wrapping_mul(i32::wrapping_add(self.iRec140[1], ((self.iRec140[1] > 0) as i32)), ((fSlow7 <= self.fVec9[1]) as i32)), ((fSlow7 > self.fVec9[1]) as i32));
			let mut fTemp142: F32 = ((self.iRec140[0]) as F32) / F32::max(1.0, self.fConst7 * mydsp_faustpower2_f(1.0 - 0.22 * fTemp98));
			self.fRec141[0] = fTemp37 - (self.fRec141[2] * ((fTemp140 + -1.4142135) / fTemp139 + 1.0) + 2.0 * self.fRec141[1] * (1.0 - 1.0 / mydsp_faustpower2_f(fTemp139))) / fTemp141;
			let mut fTemp143: F32 = 0.5 * ((self.fRec141[2] + self.fRec141[0] + 2.0 * self.fRec141[1]) * F32::max(0.0, F32::min(fTemp142, 2.0 - fTemp142)) / fTemp141);
			let mut fTemp144: F32 = fTemp143 + self.fVec11[1] + fTemp134;
			self.fVec12[0] = fTemp144;
			self.fRec132[(self.IOTA0 & 2047) as usize] = 0.05 * self.fRec132[((i32::wrapping_sub(self.IOTA0, 1)) & 2047) as usize] + 0.95 * self.fVec12[2];
			let mut fRec129: F32 = fTemp138 * self.fRec132[((i32::wrapping_sub(self.IOTA0, iTemp129)) & 2047) as usize] + fTemp124 * (fTemp137 * self.fRec132[((i32::wrapping_sub(self.IOTA0, iTemp122)) & 2047) as usize] + 0.5 * fTemp136 * self.fRec132[((i32::wrapping_sub(self.IOTA0, iTemp117)) & 2047) as usize] + 0.16666667 * fTemp135 * self.fRec132[((i32::wrapping_sub(self.IOTA0, iTemp113)) & 2047) as usize] + 0.041666668 * fTemp110 * self.fRec132[((i32::wrapping_sub(self.IOTA0, iTemp103)) & 2047) as usize]);
			let mut fRec130: F32 = self.fVec12[1] + self.fRec123[1];
			self.fRec123[0] = fRec128;
			let mut fRec124: F32 = self.fRec123[1];
			let mut fRec125: F32 = fRec129;
			let mut fRec126: F32 = fRec130;
			self.fRec119[0] = fRec124;
			let mut fRec120: F32 = fTemp134 + fTemp143 + self.fRec119[1];
			let mut fRec121: F32 = fRec125;
			let mut fRec122: F32 = fRec126;
			self.fRec115[(self.IOTA0 & 2047) as usize] = fRec120;
			let mut fRec116: F32 = fTemp138 * self.fRec115[((i32::wrapping_sub(self.IOTA0, iTemp130)) & 2047) as usize] + fTemp124 * (fTemp137 * self.fRec115[((i32::wrapping_sub(self.IOTA0, iTemp123)) & 2047) as usize] + 0.5 * fTemp136 * self.fRec115[((i32::wrapping_sub(self.IOTA0, iTemp118)) & 2047) as usize] + 0.16666667 * fTemp135 * self.fRec115[((i32::wrapping_sub(self.IOTA0, iTemp114)) & 2047) as usize] + 0.041666668 * fTemp110 * self.fRec115[((i32::wrapping_sub(self.IOTA0, iTemp104)) & 2047) as usize]);
			self.fRec117[0] = fRec121;
			let mut fRec118: F32 = fRec122;
			self.fRec113[0] = fSlow4 * self.fRec117[1];
			let mut fRec114: F32 = fRec118;
			self.fRec108[0] = fRec112;
			let mut fRec109: F32 = fSlow4 * self.fRec108[1];
			let mut fRec110: F32 = self.fRec113[0];
			let mut fRec111: F32 = fRec114;
			self.fRec104[(self.IOTA0 & 2047) as usize] = fRec109;
			let mut fRec105: F32 = fRec116;
			let mut fRec106: F32 = fRec110;
			let mut fRec107: F32 = fRec111;
			self.fRec102[0] = fRec105;
			let mut fRec103: F32 = fRec107;
			let mut fTemp145: F32 = F32::abs(fRec103);
			let mut fTemp146: F32 = if (((self.fRec100[1] > fTemp145) as i32) as i32 != 0) { self.fConst12 } else { 0.0 };
			self.fRec101[0] = self.fRec101[1] * fTemp146 + fTemp145 * (1.0 - fTemp146);
			self.fRec100[0] = self.fRec101[0];
			let mut fRec99: F32 = 0.0 - 0.95 * F32::max(2e+01 * F32::log10(self.fRec100[0]) + 1e+01, 0.0);
			self.fRec98[0] = fRec103 * F32::powf(1e+01, 0.05 * fRec99);
			self.fRec95[0] = 0.0 - (self.fRec95[1] * (1.0 - fTemp99) - (self.fRec98[0] + self.fRec98[1])) / (fTemp99 + 1.0);
			self.fVec13[0] = fSlow10;
			self.fRec144[0] = if (iSlow11 as i32 != 0) { fSlow12 } else { self.fRec144[1] };
			self.fRec143[0] = self.fConst2 * self.fRec143[1] + self.fConst1 * self.fRec144[0];
			let mut fTemp147: F32 = F32::powf(2.0, 0.083333336 * (self.fRec3[0] + self.fRec143[0] + -69.0));
			let mut fTemp148: F32 = 1.0 / F32::tan(self.fConst3 * fTemp147);
			let mut fRec159: F32 = -1.0 * 0.9973053 * (0.9 * self.fRec160[2] + 0.05 * (self.fRec160[1] + self.fRec160[3]));
			let mut fTemp149: F32 = self.fConst5 * (0.77272725 / fTemp147 + -0.11);
			let mut fTemp150: F32 = fTemp149 + -1.499995;
			let mut iTemp151: i32 = ((fTemp150) as i32);
			let mut iTemp152: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp151, 4))) as F32))) as i32);
			let mut iTemp153: i32 = i32::wrapping_add(iTemp152, 1);
			let mut fTemp154: F32 = F32::floor(fTemp150);
			let mut fTemp155: F32 = fTemp149 + (-3.0 - fTemp154);
			let mut fTemp156: F32 = fTemp149 + (-2.0 - fTemp154);
			let mut fTemp157: F32 = fTemp149 + (-1.0 - fTemp154);
			let mut fTemp158: F32 = fTemp157 * fTemp156;
			let mut fTemp159: F32 = fTemp158 * fTemp155;
			let mut fTemp160: F32 = fTemp149 + (-4.0 - fTemp154);
			let mut fTemp161: F32 = 0.0 - fTemp160;
			let mut iTemp162: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp151, 3))) as F32))) as i32);
			let mut iTemp163: i32 = i32::wrapping_add(iTemp162, 1);
			let mut fTemp164: F32 = 0.0 - 0.5 * fTemp160;
			let mut fTemp165: F32 = 0.0 - fTemp155;
			let mut iTemp166: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp151, 2))) as F32))) as i32);
			let mut iTemp167: i32 = i32::wrapping_add(iTemp166, 1);
			let mut fTemp168: F32 = 0.0 - 0.33333334 * fTemp160;
			let mut fTemp169: F32 = 0.0 - 0.5 * fTemp155;
			let mut fTemp170: F32 = 0.0 - fTemp156;
			let mut iTemp171: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp151, 1))) as F32))) as i32);
			let mut iTemp172: i32 = i32::wrapping_add(iTemp171, 1);
			let mut fTemp173: F32 = fTemp149 - fTemp154;
			let mut fTemp174: F32 = 0.0 - 0.25 * fTemp160;
			let mut fTemp175: F32 = 0.0 - 0.33333334 * fTemp155;
			let mut fTemp176: F32 = 0.0 - 0.5 * fTemp156;
			let mut fTemp177: F32 = 0.0 - fTemp157;
			let mut iTemp178: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, iTemp151)) as F32))) as i32);
			let mut iTemp179: i32 = i32::wrapping_add(iTemp178, 1);
			self.fRec174[0] = self.fRec151[((i32::wrapping_sub(self.IOTA0, iTemp179)) & 2047) as usize] * fTemp177 * fTemp176 * fTemp175 * fTemp174 + fTemp173 * (self.fRec151[((i32::wrapping_sub(self.IOTA0, iTemp172)) & 2047) as usize] * fTemp170 * fTemp169 * fTemp168 + 0.5 * fTemp157 * self.fRec151[((i32::wrapping_sub(self.IOTA0, iTemp167)) & 2047) as usize] * fTemp165 * fTemp164 + 0.16666667 * fTemp158 * self.fRec151[((i32::wrapping_sub(self.IOTA0, iTemp163)) & 2047) as usize] * fTemp161 + 0.041666668 * fTemp159 * self.fRec151[((i32::wrapping_sub(self.IOTA0, iTemp153)) & 2047) as usize]);
			self.fRec178[0] = 0.05 * self.fRec178[1] + 0.95 * self.fRec174[1];
			let mut fRec175: F32 = self.fRec178[0];
			self.fRec183[0] = self.fConst8 * self.fRec183[1] + self.fConst9 * F32::abs(self.fRec145[1]);
			let mut fRec182: F32 = self.fRec183[0];
			let mut iTemp180: i32 = ((fRec182 > 0.1) as i32);
			self.iVec14[0] = iTemp180;
			self.iRec184[0] = std::cmp::max(i32::wrapping_mul(self.iConst10, ((iTemp180 < self.iVec14[1]) as i32)), i32::wrapping_add(self.iRec184[1], -1));
			let mut fTemp181: F32 = F32::abs(F32::max(((iTemp180) as F32), ((((self.iRec184[0] > 0) as i32)) as F32)));
			let mut fTemp182: F32 = if (((self.fRec180[1] > fTemp181) as i32) as i32 != 0) { self.fConst11 } else { self.fConst8 };
			self.fRec181[0] = self.fRec181[1] * fTemp182 + fTemp181 * (1.0 - fTemp182);
			self.fRec180[0] = self.fRec181[0];
			let mut fTemp183: F32 = 0.005 * self.fRec180[0] * self.fRec145[1];
			self.fRec185[0] = self.fRec149[1];
			self.fRec186[(self.IOTA0 & 2047) as usize] = -1.0 * 0.9973053 * (0.9 * self.fRec185[2] + 0.05 * (self.fRec185[1] + self.fRec185[3]));
			let mut fTemp184: F32 = fTemp158 * fTemp161;
			let mut fTemp185: F32 = fTemp157 * fTemp165 * fTemp164;
			let mut fTemp186: F32 = fTemp170 * fTemp169 * fTemp168;
			let mut fTemp187: F32 = fTemp177 * fTemp176 * fTemp175 * fTemp174;
			self.fVec15[0] = fTemp187 * self.fRec186[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp178, 2))) & 2047) as usize] + fTemp173 * (fTemp186 * self.fRec186[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp171, 2))) & 2047) as usize] + 0.5 * fTemp185 * self.fRec186[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp166, 2))) & 2047) as usize] + 0.16666667 * fTemp184 * self.fRec186[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp162, 2))) & 2047) as usize] + 0.041666668 * fTemp159 * self.fRec186[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp152, 2))) & 2047) as usize]);
			let mut fTemp188: F32 = F32::tan(self.fConst6 * fTemp147);
			let mut fTemp189: F32 = 1.0 / fTemp188;
			let mut fTemp190: F32 = (fTemp189 + 1.4142135) / fTemp188 + 1.0;
			self.iRec187[0] = i32::wrapping_add(i32::wrapping_mul(i32::wrapping_add(self.iRec187[1], ((self.iRec187[1] > 0) as i32)), ((fSlow10 <= self.fVec13[1]) as i32)), ((fSlow10 > self.fVec13[1]) as i32));
			let mut fTemp191: F32 = ((self.iRec187[0]) as F32) / F32::max(1.0, self.fConst7 * mydsp_faustpower2_f(1.0 - 0.22 * fTemp147));
			self.fRec188[0] = fTemp37 - (self.fRec188[2] * ((fTemp189 + -1.4142135) / fTemp188 + 1.0) + 2.0 * self.fRec188[1] * (1.0 - 1.0 / mydsp_faustpower2_f(fTemp188))) / fTemp190;
			let mut fTemp192: F32 = 0.5 * ((self.fRec188[2] + self.fRec188[0] + 2.0 * self.fRec188[1]) * F32::max(0.0, F32::min(fTemp191, 2.0 - fTemp191)) / fTemp190);
			let mut fTemp193: F32 = fTemp192 + self.fVec15[1] + fTemp183;
			self.fVec16[0] = fTemp193;
			self.fRec179[(self.IOTA0 & 2047) as usize] = 0.05 * self.fRec179[((i32::wrapping_sub(self.IOTA0, 1)) & 2047) as usize] + 0.95 * self.fVec16[2];
			let mut fRec176: F32 = fTemp187 * self.fRec179[((i32::wrapping_sub(self.IOTA0, iTemp178)) & 2047) as usize] + fTemp173 * (fTemp186 * self.fRec179[((i32::wrapping_sub(self.IOTA0, iTemp171)) & 2047) as usize] + 0.5 * fTemp185 * self.fRec179[((i32::wrapping_sub(self.IOTA0, iTemp166)) & 2047) as usize] + 0.16666667 * fTemp184 * self.fRec179[((i32::wrapping_sub(self.IOTA0, iTemp162)) & 2047) as usize] + 0.041666668 * fTemp159 * self.fRec179[((i32::wrapping_sub(self.IOTA0, iTemp152)) & 2047) as usize]);
			let mut fRec177: F32 = self.fVec16[1] + self.fRec170[1];
			self.fRec170[0] = fRec175;
			let mut fRec171: F32 = self.fRec170[1];
			let mut fRec172: F32 = fRec176;
			let mut fRec173: F32 = fRec177;
			self.fRec166[0] = fRec171;
			let mut fRec167: F32 = fTemp183 + fTemp192 + self.fRec166[1];
			let mut fRec168: F32 = fRec172;
			let mut fRec169: F32 = fRec173;
			self.fRec162[(self.IOTA0 & 2047) as usize] = fRec167;
			let mut fRec163: F32 = fTemp187 * self.fRec162[((i32::wrapping_sub(self.IOTA0, iTemp179)) & 2047) as usize] + fTemp173 * (fTemp186 * self.fRec162[((i32::wrapping_sub(self.IOTA0, iTemp172)) & 2047) as usize] + 0.5 * fTemp185 * self.fRec162[((i32::wrapping_sub(self.IOTA0, iTemp167)) & 2047) as usize] + 0.16666667 * fTemp184 * self.fRec162[((i32::wrapping_sub(self.IOTA0, iTemp163)) & 2047) as usize] + 0.041666668 * fTemp159 * self.fRec162[((i32::wrapping_sub(self.IOTA0, iTemp153)) & 2047) as usize]);
			self.fRec164[0] = fRec168;
			let mut fRec165: F32 = fRec169;
			self.fRec160[0] = fSlow4 * self.fRec164[1];
			let mut fRec161: F32 = fRec165;
			self.fRec155[0] = fRec159;
			let mut fRec156: F32 = fSlow4 * self.fRec155[1];
			let mut fRec157: F32 = self.fRec160[0];
			let mut fRec158: F32 = fRec161;
			self.fRec151[(self.IOTA0 & 2047) as usize] = fRec156;
			let mut fRec152: F32 = fRec163;
			let mut fRec153: F32 = fRec157;
			let mut fRec154: F32 = fRec158;
			self.fRec149[0] = fRec152;
			let mut fRec150: F32 = fRec154;
			let mut fTemp194: F32 = F32::abs(fRec150);
			let mut fTemp195: F32 = if (((self.fRec147[1] > fTemp194) as i32) as i32 != 0) { self.fConst12 } else { 0.0 };
			self.fRec148[0] = self.fRec148[1] * fTemp195 + fTemp194 * (1.0 - fTemp195);
			self.fRec147[0] = self.fRec148[0];
			let mut fRec146: F32 = 0.0 - 0.95 * F32::max(2e+01 * F32::log10(self.fRec147[0]) + 1e+01, 0.0);
			self.fRec145[0] = fRec150 * F32::powf(1e+01, 0.05 * fRec146);
			self.fRec142[0] = 0.0 - (self.fRec142[1] * (1.0 - fTemp148) - (self.fRec145[0] + self.fRec145[1])) / (fTemp148 + 1.0);
			self.fVec17[0] = fSlow13;
			self.fRec191[0] = if (iSlow14 as i32 != 0) { fSlow15 } else { self.fRec191[1] };
			self.fRec190[0] = self.fConst2 * self.fRec190[1] + self.fConst1 * self.fRec191[0];
			let mut fTemp196: F32 = F32::powf(2.0, 0.083333336 * (self.fRec3[0] + self.fRec190[0] + -69.0));
			let mut fTemp197: F32 = 1.0 / F32::tan(self.fConst3 * fTemp196);
			let mut fRec206: F32 = -1.0 * 0.9973053 * (0.9 * self.fRec207[2] + 0.05 * (self.fRec207[1] + self.fRec207[3]));
			let mut fTemp198: F32 = self.fConst5 * (0.77272725 / fTemp196 + -0.11);
			let mut fTemp199: F32 = fTemp198 + -1.499995;
			let mut iTemp200: i32 = ((fTemp199) as i32);
			let mut iTemp201: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp200, 4))) as F32))) as i32);
			let mut iTemp202: i32 = i32::wrapping_add(iTemp201, 1);
			let mut fTemp203: F32 = F32::floor(fTemp199);
			let mut fTemp204: F32 = fTemp198 + (-3.0 - fTemp203);
			let mut fTemp205: F32 = fTemp198 + (-2.0 - fTemp203);
			let mut fTemp206: F32 = fTemp198 + (-1.0 - fTemp203);
			let mut fTemp207: F32 = fTemp206 * fTemp205;
			let mut fTemp208: F32 = fTemp207 * fTemp204;
			let mut fTemp209: F32 = fTemp198 + (-4.0 - fTemp203);
			let mut fTemp210: F32 = 0.0 - fTemp209;
			let mut iTemp211: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp200, 3))) as F32))) as i32);
			let mut iTemp212: i32 = i32::wrapping_add(iTemp211, 1);
			let mut fTemp213: F32 = 0.0 - 0.5 * fTemp209;
			let mut fTemp214: F32 = 0.0 - fTemp204;
			let mut iTemp215: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp200, 2))) as F32))) as i32);
			let mut iTemp216: i32 = i32::wrapping_add(iTemp215, 1);
			let mut fTemp217: F32 = 0.0 - 0.33333334 * fTemp209;
			let mut fTemp218: F32 = 0.0 - 0.5 * fTemp204;
			let mut fTemp219: F32 = 0.0 - fTemp205;
			let mut iTemp220: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, i32::wrapping_add(iTemp200, 1))) as F32))) as i32);
			let mut iTemp221: i32 = i32::wrapping_add(iTemp220, 1);
			let mut fTemp222: F32 = fTemp198 - fTemp203;
			let mut fTemp223: F32 = 0.0 - 0.25 * fTemp209;
			let mut fTemp224: F32 = 0.0 - 0.33333334 * fTemp204;
			let mut fTemp225: F32 = 0.0 - 0.5 * fTemp205;
			let mut fTemp226: F32 = 0.0 - fTemp206;
			let mut iTemp227: i32 = ((F32::min(self.fConst4, ((std::cmp::max(0, iTemp200)) as F32))) as i32);
			let mut iTemp228: i32 = i32::wrapping_add(iTemp227, 1);
			self.fRec221[0] = self.fRec198[((i32::wrapping_sub(self.IOTA0, iTemp228)) & 2047) as usize] * fTemp226 * fTemp225 * fTemp224 * fTemp223 + fTemp222 * (self.fRec198[((i32::wrapping_sub(self.IOTA0, iTemp221)) & 2047) as usize] * fTemp219 * fTemp218 * fTemp217 + 0.5 * fTemp206 * self.fRec198[((i32::wrapping_sub(self.IOTA0, iTemp216)) & 2047) as usize] * fTemp214 * fTemp213 + 0.16666667 * fTemp207 * self.fRec198[((i32::wrapping_sub(self.IOTA0, iTemp212)) & 2047) as usize] * fTemp210 + 0.041666668 * fTemp208 * self.fRec198[((i32::wrapping_sub(self.IOTA0, iTemp202)) & 2047) as usize]);
			self.fRec225[0] = 0.05 * self.fRec225[1] + 0.95 * self.fRec221[1];
			let mut fRec222: F32 = self.fRec225[0];
			self.fRec230[0] = self.fConst8 * self.fRec230[1] + self.fConst9 * F32::abs(self.fRec192[1]);
			let mut fRec229: F32 = self.fRec230[0];
			let mut iTemp229: i32 = ((fRec229 > 0.1) as i32);
			self.iVec18[0] = iTemp229;
			self.iRec231[0] = std::cmp::max(i32::wrapping_mul(self.iConst10, ((iTemp229 < self.iVec18[1]) as i32)), i32::wrapping_add(self.iRec231[1], -1));
			let mut fTemp230: F32 = F32::abs(F32::max(((iTemp229) as F32), ((((self.iRec231[0] > 0) as i32)) as F32)));
			let mut fTemp231: F32 = if (((self.fRec227[1] > fTemp230) as i32) as i32 != 0) { self.fConst11 } else { self.fConst8 };
			self.fRec228[0] = self.fRec228[1] * fTemp231 + fTemp230 * (1.0 - fTemp231);
			self.fRec227[0] = self.fRec228[0];
			let mut fTemp232: F32 = 0.005 * self.fRec227[0] * self.fRec192[1];
			self.fRec232[0] = self.fRec196[1];
			self.fRec233[(self.IOTA0 & 2047) as usize] = -1.0 * 0.9973053 * (0.9 * self.fRec232[2] + 0.05 * (self.fRec232[1] + self.fRec232[3]));
			let mut fTemp233: F32 = fTemp207 * fTemp210;
			let mut fTemp234: F32 = fTemp206 * fTemp214 * fTemp213;
			let mut fTemp235: F32 = fTemp219 * fTemp218 * fTemp217;
			let mut fTemp236: F32 = fTemp226 * fTemp225 * fTemp224 * fTemp223;
			self.fVec19[0] = fTemp236 * self.fRec233[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp227, 2))) & 2047) as usize] + fTemp222 * (fTemp235 * self.fRec233[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp220, 2))) & 2047) as usize] + 0.5 * fTemp234 * self.fRec233[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp215, 2))) & 2047) as usize] + 0.16666667 * fTemp233 * self.fRec233[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp211, 2))) & 2047) as usize] + 0.041666668 * fTemp208 * self.fRec233[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp201, 2))) & 2047) as usize]);
			let mut fTemp237: F32 = F32::tan(self.fConst6 * fTemp196);
			let mut fTemp238: F32 = 1.0 / fTemp237;
			let mut fTemp239: F32 = (fTemp238 + 1.4142135) / fTemp237 + 1.0;
			self.iRec234[0] = i32::wrapping_add(i32::wrapping_mul(i32::wrapping_add(self.iRec234[1], ((self.iRec234[1] > 0) as i32)), ((fSlow13 <= self.fVec17[1]) as i32)), ((fSlow13 > self.fVec17[1]) as i32));
			let mut fTemp240: F32 = ((self.iRec234[0]) as F32) / F32::max(1.0, self.fConst7 * mydsp_faustpower2_f(1.0 - 0.22 * fTemp196));
			self.fRec235[0] = fTemp37 - (self.fRec235[2] * ((fTemp238 + -1.4142135) / fTemp237 + 1.0) + 2.0 * self.fRec235[1] * (1.0 - 1.0 / mydsp_faustpower2_f(fTemp237))) / fTemp239;
			let mut fTemp241: F32 = 0.5 * ((self.fRec235[2] + self.fRec235[0] + 2.0 * self.fRec235[1]) * F32::max(0.0, F32::min(fTemp240, 2.0 - fTemp240)) / fTemp239);
			let mut fTemp242: F32 = fTemp241 + self.fVec19[1] + fTemp232;
			self.fVec20[0] = fTemp242;
			self.fRec226[(self.IOTA0 & 2047) as usize] = 0.05 * self.fRec226[((i32::wrapping_sub(self.IOTA0, 1)) & 2047) as usize] + 0.95 * self.fVec20[2];
			let mut fRec223: F32 = fTemp236 * self.fRec226[((i32::wrapping_sub(self.IOTA0, iTemp227)) & 2047) as usize] + fTemp222 * (fTemp235 * self.fRec226[((i32::wrapping_sub(self.IOTA0, iTemp220)) & 2047) as usize] + 0.5 * fTemp234 * self.fRec226[((i32::wrapping_sub(self.IOTA0, iTemp215)) & 2047) as usize] + 0.16666667 * fTemp233 * self.fRec226[((i32::wrapping_sub(self.IOTA0, iTemp211)) & 2047) as usize] + 0.041666668 * fTemp208 * self.fRec226[((i32::wrapping_sub(self.IOTA0, iTemp201)) & 2047) as usize]);
			let mut fRec224: F32 = self.fVec20[1] + self.fRec217[1];
			self.fRec217[0] = fRec222;
			let mut fRec218: F32 = self.fRec217[1];
			let mut fRec219: F32 = fRec223;
			let mut fRec220: F32 = fRec224;
			self.fRec213[0] = fRec218;
			let mut fRec214: F32 = fTemp232 + fTemp241 + self.fRec213[1];
			let mut fRec215: F32 = fRec219;
			let mut fRec216: F32 = fRec220;
			self.fRec209[(self.IOTA0 & 2047) as usize] = fRec214;
			let mut fRec210: F32 = fTemp236 * self.fRec209[((i32::wrapping_sub(self.IOTA0, iTemp228)) & 2047) as usize] + fTemp222 * (fTemp235 * self.fRec209[((i32::wrapping_sub(self.IOTA0, iTemp221)) & 2047) as usize] + 0.5 * fTemp234 * self.fRec209[((i32::wrapping_sub(self.IOTA0, iTemp216)) & 2047) as usize] + 0.16666667 * fTemp233 * self.fRec209[((i32::wrapping_sub(self.IOTA0, iTemp212)) & 2047) as usize] + 0.041666668 * fTemp208 * self.fRec209[((i32::wrapping_sub(self.IOTA0, iTemp202)) & 2047) as usize]);
			self.fRec211[0] = fRec215;
			let mut fRec212: F32 = fRec216;
			self.fRec207[0] = fSlow4 * self.fRec211[1];
			let mut fRec208: F32 = fRec212;
			self.fRec202[0] = fRec206;
			let mut fRec203: F32 = fSlow4 * self.fRec202[1];
			let mut fRec204: F32 = self.fRec207[0];
			let mut fRec205: F32 = fRec208;
			self.fRec198[(self.IOTA0 & 2047) as usize] = fRec203;
			let mut fRec199: F32 = fRec210;
			let mut fRec200: F32 = fRec204;
			let mut fRec201: F32 = fRec205;
			self.fRec196[0] = fRec199;
			let mut fRec197: F32 = fRec201;
			let mut fTemp243: F32 = F32::abs(fRec197);
			let mut fTemp244: F32 = if (((self.fRec194[1] > fTemp243) as i32) as i32 != 0) { self.fConst12 } else { 0.0 };
			self.fRec195[0] = self.fRec195[1] * fTemp244 + fTemp243 * (1.0 - fTemp244);
			self.fRec194[0] = self.fRec195[0];
			let mut fRec193: F32 = 0.0 - 0.95 * F32::max(2e+01 * F32::log10(self.fRec194[0]) + 1e+01, 0.0);
			self.fRec192[0] = fRec197 * F32::powf(1e+01, 0.05 * fRec193);
			self.fRec189[0] = 0.0 - (self.fRec189[1] * (1.0 - fTemp197) - (self.fRec192[0] + self.fRec192[1])) / (fTemp197 + 1.0);
			self.fRec236[0] = fSlow16 + self.fConst2 * self.fRec236[1];
			let mut fTemp245: F32 = self.fRec236[0] * (self.fRec189[0] + self.fRec142[0] + self.fRec95[0] + self.fRec49[0] + self.fRec0[0]);
			self.fRec237[0] = fSlow17 + self.fConst2 * self.fRec237[1];
			let mut fTemp246: F32 = F32::min(1.4141995, 1.4142135 * self.fRec237[0]);
			let mut fTemp247: F32 = 1.4142135 * fTemp246;
			let mut fTemp248: F32 = 1.0 - fTemp247;
			self.fRec239[0] = fSlow18 + self.fConst2 * self.fRec239[1];
			let mut fTemp249: F32 = self.fRec239[0] + -69.0;
			self.fRec238[0] = self.fConst2 * self.fRec238[1] + self.fConst13 * F32::powf(2.0, 0.083333336 * (fSlow19 + fTemp249));
			let mut fTemp250: F32 = F32::tan(self.fConst14 * F32::max(2e+01, F32::min(1e+04, self.fRec238[0])));
			let mut fTemp251: F32 = 1.0 / fTemp250;
			let mut fTemp252: F32 = 2.0 - fTemp247;
			let mut fTemp253: F32 = mydsp_faustpower2_f(fTemp246);
			let mut fTemp254: F32 = fTemp253 + (fTemp252 + fTemp251) / fTemp250 + fTemp248;
			let mut fTemp255: F32 = 1.0 / mydsp_faustpower2_f(fTemp250);
			let mut fTemp256: F32 = fTemp247 + 2.0;
			let mut fTemp257: F32 = fTemp247 + fTemp253;
			let mut fTemp258: F32 = fTemp257 + (fTemp256 + fTemp251) / fTemp250 + 1.0;
			self.fRec244[0] = fSlow20 + self.fConst2 * self.fRec244[1];
			let mut fTemp259: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec244[0]));
			let mut fTemp260: F32 = self.fRec242[1] + self.fConst15 * fTemp259;
			let mut fTemp261: F32 = fTemp260 + -1.0;
			let mut iTemp262: i32 = ((fTemp261 < 0.0) as i32);
			self.fRec242[0] = if (iTemp262 as i32 != 0) { fTemp260 } else { fTemp261 };
			let mut fThen15: F32 = fTemp260 + (1.0 - self.fConst0 / fTemp259) * fTemp261;
			let mut fRec243: F32 = if (iTemp262 as i32 != 0) { fTemp260 } else { fThen15 };
			self.fRec245[0] = fSlow21 + self.fConst2 * self.fRec245[1];
			self.fRec241[0] = self.fRec245[0] * (2.0 * fRec243 + -1.0) - (self.fRec241[2] * (fTemp257 + (fTemp251 - fTemp256) / fTemp250 + 1.0) + 2.0 * self.fRec241[1] * (fTemp257 + (1.0 - fTemp255))) / fTemp258;
			self.fRec240[0] = (self.fRec241[2] + self.fRec241[0] + 2.0 * self.fRec241[1]) / fTemp258 - (self.fRec240[2] * (fTemp253 + (fTemp251 - fTemp252) / fTemp250 + fTemp248) + 2.0 * self.fRec240[1] * (fTemp253 + (1.0 - (fTemp247 + fTemp255)))) / fTemp254;
			self.fRec246[0] = self.fConst2 * self.fRec246[1] + self.fConst13 * F32::powf(2.0, 0.083333336 * (fSlow22 + fTemp249));
			let mut fTemp263: F32 = F32::tan(self.fConst14 * F32::max(2e+01, F32::min(1e+04, self.fRec246[0])));
			let mut fTemp264: F32 = 1.0 / fTemp263;
			let mut fTemp265: F32 = fTemp253 + (fTemp252 + fTemp264) / fTemp263 + fTemp248;
			let mut fTemp266: F32 = 1.0 / mydsp_faustpower2_f(fTemp263);
			let mut fTemp267: F32 = fTemp257 + (fTemp256 + fTemp264) / fTemp263 + 1.0;
			self.fRec251[0] = fSlow23 + self.fConst2 * self.fRec251[1];
			let mut fTemp268: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec251[0]));
			let mut fTemp269: F32 = self.fRec249[1] + self.fConst15 * fTemp268;
			let mut fTemp270: F32 = fTemp269 + -1.0;
			let mut iTemp271: i32 = ((fTemp270 < 0.0) as i32);
			self.fRec249[0] = if (iTemp271 as i32 != 0) { fTemp269 } else { fTemp270 };
			let mut fThen17: F32 = fTemp269 + (1.0 - self.fConst0 / fTemp268) * fTemp270;
			let mut fRec250: F32 = if (iTemp271 as i32 != 0) { fTemp269 } else { fThen17 };
			self.fRec252[0] = fSlow24 + self.fConst2 * self.fRec252[1];
			self.fRec248[0] = self.fRec252[0] * (2.0 * fRec250 + -1.0) - (self.fRec248[2] * (fTemp257 + (fTemp264 - fTemp256) / fTemp263 + 1.0) + 2.0 * self.fRec248[1] * (fTemp257 + (1.0 - fTemp266))) / fTemp267;
			self.fRec247[0] = (self.fRec248[2] + self.fRec248[0] + 2.0 * self.fRec248[1]) / fTemp267 - (self.fRec247[2] * (fTemp253 + (fTemp264 - fTemp252) / fTemp263 + fTemp248) + 2.0 * self.fRec247[1] * (fTemp253 + (1.0 - (fTemp247 + fTemp266)))) / fTemp265;
			self.fRec253[0] = self.fConst2 * self.fRec253[1] + self.fConst13 * F32::powf(2.0, 0.083333336 * (fSlow25 + fTemp249));
			let mut fTemp272: F32 = F32::tan(self.fConst14 * F32::max(2e+01, F32::min(1e+04, self.fRec253[0])));
			let mut fTemp273: F32 = 1.0 / fTemp272;
			let mut fTemp274: F32 = fTemp253 + (fTemp252 + fTemp273) / fTemp272 + fTemp248;
			let mut fTemp275: F32 = 1.0 / mydsp_faustpower2_f(fTemp272);
			let mut fTemp276: F32 = fTemp257 + (fTemp256 + fTemp273) / fTemp272 + 1.0;
			self.fRec258[0] = fSlow26 + self.fConst2 * self.fRec258[1];
			let mut fTemp277: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec258[0]));
			let mut fTemp278: F32 = self.fConst15 * fTemp277;
			let mut fTemp279: F32 = self.fRec256[1] + fTemp278;
			let mut fTemp280: F32 = fTemp279 + -1.0;
			let mut iTemp281: i32 = ((fTemp280 < 0.0) as i32);
			self.fRec256[0] = if (iTemp281 as i32 != 0) { fTemp279 } else { fTemp280 };
			let mut fThen19: F32 = self.fRec256[1] + fTemp278 + (1.0 - self.fConst0 / fTemp277) * fTemp280;
			let mut fRec257: F32 = if (iTemp281 as i32 != 0) { fTemp279 } else { fThen19 };
			self.fRec259[0] = fSlow27 + self.fConst2 * self.fRec259[1];
			self.fRec255[0] = self.fRec259[0] * (2.0 * fRec257 + -1.0) - (self.fRec255[2] * (fTemp257 + (fTemp273 - fTemp256) / fTemp272 + 1.0) + 2.0 * self.fRec255[1] * (fTemp257 + (1.0 - fTemp275))) / fTemp276;
			self.fRec254[0] = (self.fRec255[2] + self.fRec255[0] + 2.0 * self.fRec255[1]) / fTemp276 - (self.fRec254[2] * (fTemp253 + (fTemp273 - fTemp252) / fTemp272 + fTemp248) + 2.0 * self.fRec254[1] * (fTemp253 + (1.0 - (fTemp247 + fTemp275)))) / fTemp274;
			self.fRec260[0] = self.fConst2 * self.fRec260[1] + self.fConst13 * F32::powf(2.0, 0.083333336 * (fSlow28 + fTemp249));
			let mut fTemp282: F32 = F32::tan(self.fConst14 * F32::max(2e+01, F32::min(1e+04, self.fRec260[0])));
			let mut fTemp283: F32 = 1.0 / fTemp282;
			let mut fTemp284: F32 = fTemp253 + (fTemp252 + fTemp283) / fTemp282 + fTemp248;
			let mut fTemp285: F32 = 1.0 / mydsp_faustpower2_f(fTemp282);
			let mut fTemp286: F32 = fTemp257 + (fTemp283 + fTemp256) / fTemp282 + 1.0;
			self.fRec265[0] = fSlow29 + self.fConst2 * self.fRec265[1];
			let mut fTemp287: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec265[0]));
			let mut fTemp288: F32 = self.fConst15 * fTemp287;
			let mut fTemp289: F32 = self.fRec263[1] + fTemp288;
			let mut fTemp290: F32 = fTemp289 + -1.0;
			let mut iTemp291: i32 = ((fTemp290 < 0.0) as i32);
			self.fRec263[0] = if (iTemp291 as i32 != 0) { fTemp289 } else { fTemp290 };
			let mut fThen21: F32 = self.fRec263[1] + fTemp288 + (1.0 - self.fConst0 / fTemp287) * fTemp290;
			let mut fRec264: F32 = if (iTemp291 as i32 != 0) { fTemp289 } else { fThen21 };
			self.fRec266[0] = fSlow30 + self.fConst2 * self.fRec266[1];
			self.fRec262[0] = self.fRec266[0] * (2.0 * fRec264 + -1.0) - (self.fRec262[2] * (fTemp257 + (1.0 - (fTemp256 - fTemp283) / fTemp282)) + 2.0 * self.fRec262[1] * (fTemp257 + (1.0 - fTemp285))) / fTemp286;
			self.fRec261[0] = (self.fRec262[2] + self.fRec262[0] + 2.0 * self.fRec262[1]) / fTemp286 - (self.fRec261[2] * (fTemp253 + (fTemp283 - fTemp252) / fTemp282 + fTemp248) + 2.0 * self.fRec261[1] * (fTemp253 + (1.0 - (fTemp247 + fTemp285)))) / fTemp284;
			self.fRec267[0] = fSlow31 + self.fConst2 * self.fRec267[1];
			self.fRec268[0] = fSlow32 + self.fConst2 * self.fRec268[1];
			let mut fTemp292: F32 = self.fRec268[0] * self.fRec267[0] * ((self.fRec261[2] + self.fRec261[0] + 2.0 * self.fRec261[1]) / fTemp284 + (self.fRec254[2] + self.fRec254[0] + 2.0 * self.fRec254[1]) / fTemp274 + (self.fRec247[2] + self.fRec247[0] + 2.0 * self.fRec247[1]) / fTemp265 + (self.fRec240[2] + self.fRec240[0] + 2.0 * self.fRec240[1]) / fTemp254);
			self.fRec269[0] = fSlow33 + self.fConst2 * self.fRec269[1];
			let mut fTemp293: F32 = F32::powf(2.0, 0.083333336 * (self.fRec269[0] + -61.88));
			let mut fTemp294: F32 = F32::max(4.4e+02 * fTemp293, 23.44895);
			let mut fTemp295: F32 = F32::max(2e+01, F32::abs(fTemp294));
			let mut fTemp296: F32 = self.fRec271[1] + self.fConst15 * fTemp295;
			self.fRec271[0] = fTemp296 - F32::floor(fTemp296);
			let mut fTemp297: F32 = mydsp_faustpower2_f(2.0 * self.fRec271[0] + -1.0);
			self.fVec21[0] = fTemp297;
			let mut fTemp298: F32 = ((self.iVec1[1]) as F32);
			let mut fTemp299: F32 = fTemp298 * (fTemp297 - self.fVec21[1]) / fTemp295;
			self.fVec22[(self.IOTA0 & 4095) as usize] = fTemp299;
			let mut fTemp300: F32 = F32::max(0.0, F32::min(2047.0, self.fConst16 / fTemp294));
			let mut iTemp301: i32 = ((fTemp300) as i32);
			let mut fTemp302: F32 = F32::floor(fTemp300);
			self.fRec270[0] = 0.999 * self.fRec270[1] - self.fConst17 * (self.fVec22[((i32::wrapping_sub(self.IOTA0, iTemp301)) & 4095) as usize] * (fTemp302 + (1.0 - fTemp300)) - fTemp299 + (fTemp300 - fTemp302) * self.fVec22[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp301, 1))) & 4095) as usize]);
			let mut fTemp303: F32 = F32::powf(2.0, 0.083333336 * (self.fRec269[0] + -81.11));
			let mut fTemp304: F32 = F32::max(4.4e+02 * fTemp303, 23.44895);
			let mut fTemp305: F32 = F32::max(2e+01, F32::abs(fTemp304));
			let mut fTemp306: F32 = self.fRec273[1] + self.fConst15 * fTemp305;
			self.fRec273[0] = fTemp306 - F32::floor(fTemp306);
			let mut fTemp307: F32 = mydsp_faustpower2_f(2.0 * self.fRec273[0] + -1.0);
			self.fVec23[0] = fTemp307;
			let mut fTemp308: F32 = fTemp298 * (fTemp307 - self.fVec23[1]) / fTemp305;
			self.fVec24[(self.IOTA0 & 4095) as usize] = fTemp308;
			let mut fTemp309: F32 = F32::max(0.0, F32::min(2047.0, self.fConst16 / fTemp304));
			let mut iTemp310: i32 = ((fTemp309) as i32);
			let mut fTemp311: F32 = F32::floor(fTemp309);
			self.fRec272[0] = 0.999 * self.fRec272[1] - self.fConst17 * (self.fVec24[((i32::wrapping_sub(self.IOTA0, iTemp310)) & 4095) as usize] * (fTemp311 + (1.0 - fTemp309)) - fTemp308 + (fTemp309 - fTemp311) * self.fVec24[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp310, 1))) & 4095) as usize]);
			let mut fTemp312: F32 = F32::powf(2.0, 0.083333336 * (self.fRec269[0] + -56.9));
			let mut fTemp313: F32 = F32::max(4.4e+02 * fTemp312, 23.44895);
			let mut fTemp314: F32 = F32::max(0.0, F32::min(2047.0, self.fConst16 / fTemp313));
			let mut fTemp315: F32 = F32::floor(fTemp314);
			let mut fTemp316: F32 = F32::max(2e+01, F32::abs(fTemp313));
			let mut fTemp317: F32 = self.fRec275[1] + self.fConst15 * fTemp316;
			self.fRec275[0] = fTemp317 - F32::floor(fTemp317);
			let mut fTemp318: F32 = mydsp_faustpower2_f(2.0 * self.fRec275[0] + -1.0);
			self.fVec25[0] = fTemp318;
			let mut fTemp319: F32 = fTemp298 * (fTemp318 - self.fVec25[1]) / fTemp316;
			self.fVec26[(self.IOTA0 & 4095) as usize] = fTemp319;
			let mut iTemp320: i32 = ((fTemp314) as i32);
			self.fRec274[0] = 0.999 * self.fRec274[1] - self.fConst17 * ((fTemp314 - fTemp315) * self.fVec26[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp320, 1))) & 4095) as usize] - (fTemp319 - self.fVec26[((i32::wrapping_sub(self.IOTA0, iTemp320)) & 4095) as usize] * (fTemp315 + (1.0 - fTemp314))));
			let mut fTemp321: F32 = F32::powf(2.0, 0.083333336 * (self.fRec269[0] + -69.0));
			let mut fTemp322: F32 = F32::max(4.4e+02 * fTemp321, 23.44895);
			let mut fTemp323: F32 = F32::max(2e+01, F32::abs(fTemp322));
			let mut fTemp324: F32 = self.fRec277[1] + self.fConst15 * fTemp323;
			self.fRec277[0] = fTemp324 - F32::floor(fTemp324);
			let mut fTemp325: F32 = mydsp_faustpower2_f(2.0 * self.fRec277[0] + -1.0);
			self.fVec27[0] = fTemp325;
			let mut fTemp326: F32 = fTemp298 * (fTemp325 - self.fVec27[1]) / fTemp323;
			self.fVec28[(self.IOTA0 & 4095) as usize] = fTemp326;
			let mut fTemp327: F32 = F32::max(0.0, F32::min(2047.0, self.fConst16 / fTemp322));
			let mut iTemp328: i32 = ((fTemp327) as i32);
			let mut fTemp329: F32 = F32::floor(fTemp327);
			self.fRec276[0] = 0.999 * self.fRec276[1] - self.fConst17 * (self.fVec28[((i32::wrapping_sub(self.IOTA0, iTemp328)) & 4095) as usize] * (fTemp329 + (1.0 - fTemp327)) - fTemp326 + (fTemp327 - fTemp329) * self.fVec28[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp328, 1))) & 4095) as usize]);
			self.fRec278[0] = fSlow34 + self.fConst2 * self.fRec278[1];
			self.fRec279[0] = fSlow35 + self.fConst2 * self.fRec279[1];
			let mut fTemp330: F32 = self.fConst18 * self.fRec279[0] * self.fRec278[0] * (self.fRec276[0] * fTemp321 + self.fRec274[0] * fTemp312 + self.fRec272[0] * fTemp303 + self.fRec270[0] * fTemp293);
			self.fRec280[0] = fSlow36 + self.fConst2 * self.fRec280[1];
			let mut fTemp331: F32 = (1.0 - self.fRec280[0]) * (fTemp330 + fTemp292 + fTemp245);
			self.fRec282[0] = fSlow37 + self.fConst2 * self.fRec282[1];
			self.fRec281[(self.IOTA0 & 2097151) as usize] = fTemp245 + fTemp292 + fSlow38 * self.fRec281[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(((F32::min(self.fConst19, F32::max(0.0, self.fConst0 * self.fRec282[0]))) as i32), 1))) & 2097151) as usize] + fTemp330;
			let mut fTemp332: F32 = self.fRec281[(self.IOTA0 & 2097151) as usize] * self.fRec280[0];
			let mut fTemp333: F32 = fTemp332 + fTemp331;
			let mut iTemp334: i32 = i32::wrapping_sub(1, self.iVec1[1]);
			self.fRec293[0] = 0.995 * (self.fRec293[1] + ((i32::wrapping_mul(iTemp334, iSlow42)) as F32)) + fSlow43;
			let mut fTemp335: F32 = self.fRec293[0] + -1.49999;
			let mut fTemp336: F32 = F32::floor(fTemp335);
			self.fRec295[0] = 0.995 * (self.fRec295[1] + ((i32::wrapping_mul(iTemp334, iSlow44)) as F32)) + fSlow45;
			let mut fTemp337: F32 = self.fRec295[0] + -1.49999;
			let mut fTemp338: F32 = F32::floor(fTemp337);
			self.fRec299[0] = 0.9999 * (self.fRec299[1] + ((i32::wrapping_mul(iTemp334, iSlow46)) as F32)) + fSlow47;
			let mut fTemp339: F32 = self.fRec299[0] + -1.49999;
			let mut fTemp340: F32 = F32::floor(fTemp339);
			let mut fTemp341: F32 = 0.760314 * self.fRec283[1] - 0.64955574 * self.fRec296[1];
			let mut fTemp342: F32 = 0.760314 * self.fRec284[1] - 0.64955574 * self.fRec297[1];
			self.fVec29[(self.IOTA0 & 16383) as usize] = 0.0 - 0.70710677 * (fTemp342 - fTemp341);
			let mut fTemp343: F32 = self.fVec29[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp339) as i32))))) & 16383) as usize];
			self.fVec30[0] = fTemp343;
			self.fRec298[0] = self.fVec30[1] + (fTemp340 + (2.0 - self.fRec299[0])) * (fTemp343 - self.fRec298[1]) / (self.fRec299[0] - fTemp340);
			self.fRec296[0] = self.fRec298[0];
			self.fRec301[0] = 0.9999 * (self.fRec301[1] + ((i32::wrapping_mul(iTemp334, iSlow48)) as F32)) + fSlow49;
			let mut fTemp344: F32 = self.fRec301[0] + -1.49999;
			let mut fTemp345: F32 = F32::floor(fTemp344);
			let mut fTemp346: F32 = self.fRec301[0] - fTemp345;
			let mut fTemp347: F32 = fTemp345 + (2.0 - self.fRec301[0]);
			self.fVec31[(self.IOTA0 & 16383) as usize] = fTemp341 + fTemp342;
			let mut fTemp348: F32 = self.fVec31[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp344) as i32))))) & 16383) as usize];
			self.fVec32[0] = fTemp348;
			self.fRec300[0] = 0.70710677 * (fTemp347 * fTemp348 / fTemp346 + self.fVec32[1]) - self.fRec300[1] * fTemp347 / fTemp346;
			self.fRec297[0] = self.fRec300[0];
			let mut fTemp349: F32 = 0.760314 * self.fRec296[1] + 0.64955574 * self.fRec283[1];
			self.fRec305[0] = 0.9999 * (self.fRec305[1] + ((i32::wrapping_mul(iTemp334, iSlow50)) as F32)) + fSlow51;
			let mut fTemp350: F32 = self.fRec305[0] + -1.49999;
			let mut fTemp351: F32 = F32::floor(fTemp350);
			let mut fTemp352: F32 = self.fRec305[0] - fTemp351;
			let mut fTemp353: F32 = fTemp351 + (2.0 - self.fRec305[0]);
			let mut fTemp354: F32 = 0.760314 * self.fRec297[1] + 0.64955574 * self.fRec284[1];
			let mut fTemp355: F32 = 0.760314 * fTemp354 - 0.64955574 * self.fRec303[1];
			let mut fTemp356: F32 = 0.760314 * fTemp349 - 0.64955574 * self.fRec302[1];
			self.fVec33[(self.IOTA0 & 16383) as usize] = fTemp356 - fTemp355;
			let mut fTemp357: F32 = self.fVec33[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp350) as i32))))) & 16383) as usize];
			self.fVec34[0] = fTemp357;
			self.fRec304[0] = 0.70710677 * (fTemp353 * fTemp357 / fTemp352 + self.fVec34[1]) - self.fRec304[1] * fTemp353 / fTemp352;
			self.fRec302[0] = self.fRec304[0];
			self.fRec307[0] = 0.9999 * (self.fRec307[1] + ((i32::wrapping_mul(iTemp334, iSlow52)) as F32)) + fSlow53;
			let mut fTemp358: F32 = self.fRec307[0] + -1.49999;
			let mut fTemp359: F32 = F32::floor(fTemp358);
			let mut fTemp360: F32 = self.fRec307[0] - fTemp359;
			let mut fTemp361: F32 = fTemp359 + (2.0 - self.fRec307[0]);
			self.fVec35[(self.IOTA0 & 16383) as usize] = fTemp356 + fTemp355;
			let mut fTemp362: F32 = self.fVec35[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp358) as i32))))) & 16383) as usize];
			self.fVec36[0] = fTemp362;
			self.fRec306[0] = 0.70710677 * (fTemp361 * fTemp362 / fTemp360 + self.fVec36[1]) - self.fRec306[1] * fTemp361 / fTemp360;
			self.fRec303[0] = self.fRec306[0];
			let mut fTemp363: F32 = 0.760314 * self.fRec302[1] + 0.64955574 * fTemp349;
			self.fRec311[0] = 0.9999 * (self.fRec311[1] + ((i32::wrapping_mul(iTemp334, iSlow54)) as F32)) + fSlow55;
			let mut fTemp364: F32 = self.fRec311[0] + -1.49999;
			let mut fTemp365: F32 = F32::floor(fTemp364);
			let mut fTemp366: F32 = 0.760314 * fTemp363 - 0.64955574 * self.fRec308[1];
			let mut fTemp367: F32 = 0.760314 * self.fRec303[1] + 0.64955574 * fTemp354;
			let mut fTemp368: F32 = 0.760314 * fTemp367 - 0.64955574 * self.fRec309[1];
			self.fVec37[(self.IOTA0 & 16383) as usize] = 0.0 - 0.70710677 * (fTemp368 - fTemp366);
			let mut fTemp369: F32 = self.fVec37[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp364) as i32))))) & 16383) as usize];
			self.fVec38[0] = fTemp369;
			self.fRec310[0] = self.fVec38[1] - (fTemp365 + (2.0 - self.fRec311[0])) * (self.fRec310[1] - fTemp369) / (self.fRec311[0] - fTemp365);
			self.fRec308[0] = self.fRec310[0];
			self.fRec313[0] = 0.9999 * (self.fRec313[1] + ((i32::wrapping_mul(iTemp334, iSlow56)) as F32)) + fSlow57;
			let mut fTemp370: F32 = self.fRec313[0] + -1.49999;
			let mut fTemp371: F32 = F32::floor(fTemp370);
			let mut fTemp372: F32 = self.fRec313[0] - fTemp371;
			let mut fTemp373: F32 = fTemp371 + (2.0 - self.fRec313[0]);
			self.fVec39[(self.IOTA0 & 16383) as usize] = fTemp366 + fTemp368;
			let mut fTemp374: F32 = self.fVec39[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp370) as i32))))) & 16383) as usize];
			self.fVec40[0] = fTemp374;
			self.fRec312[0] = 0.70710677 * (fTemp373 * fTemp374 / fTemp372 + self.fVec40[1]) - self.fRec312[1] * fTemp373 / fTemp372;
			self.fRec309[0] = self.fRec312[0];
			let mut fTemp375: F32 = 0.760314 * self.fRec308[1] + 0.64955574 * fTemp363;
			self.fRec317[0] = 0.9999 * (self.fRec317[1] + ((i32::wrapping_mul(iTemp334, iSlow58)) as F32)) + fSlow59;
			let mut fTemp376: F32 = self.fRec317[0] + -1.49999;
			let mut fTemp377: F32 = F32::floor(fTemp376);
			let mut fTemp378: F32 = 0.760314 * fTemp375 - 0.64955574 * self.fRec314[1];
			let mut fTemp379: F32 = 0.760314 * self.fRec309[1] + 0.64955574 * fTemp367;
			let mut fTemp380: F32 = 0.760314 * fTemp379 - 0.64955574 * self.fRec315[1];
			self.fVec41[(self.IOTA0 & 16383) as usize] = 0.0 - 0.70710677 * (fTemp380 - fTemp378);
			let mut fTemp381: F32 = self.fVec41[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp376) as i32))))) & 16383) as usize];
			self.fVec42[0] = fTemp381;
			self.fRec316[0] = self.fVec42[1] - (fTemp377 + (2.0 - self.fRec317[0])) * (self.fRec316[1] - fTemp381) / (self.fRec317[0] - fTemp377);
			self.fRec314[0] = self.fRec316[0];
			self.fRec319[0] = 0.9999 * (self.fRec319[1] + ((i32::wrapping_mul(iTemp334, iSlow60)) as F32)) + fSlow61;
			let mut fTemp382: F32 = self.fRec319[0] + -1.49999;
			let mut fTemp383: F32 = F32::floor(fTemp382);
			let mut fTemp384: F32 = self.fRec319[0] - fTemp383;
			let mut fTemp385: F32 = fTemp383 + (2.0 - self.fRec319[0]);
			self.fVec43[(self.IOTA0 & 16383) as usize] = fTemp378 + fTemp380;
			let mut fTemp386: F32 = self.fVec43[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp382) as i32))))) & 16383) as usize];
			self.fVec44[0] = fTemp386;
			self.fRec318[0] = 0.70710677 * (fTemp385 * fTemp386 / fTemp384 + self.fVec44[1]) - self.fRec318[1] * fTemp385 / fTemp384;
			self.fRec315[0] = self.fRec318[0];
			let mut fTemp387: F32 = 0.760314 * self.fRec314[1] + 0.64955574 * fTemp375;
			self.fRec323[0] = 0.9999 * (self.fRec323[1] + ((i32::wrapping_mul(iTemp334, iSlow62)) as F32)) + fSlow63;
			let mut fTemp388: F32 = self.fRec323[0] + -1.49999;
			let mut fTemp389: F32 = F32::floor(fTemp388);
			let mut fTemp390: F32 = 0.760314 * fTemp387 - 0.64955574 * self.fRec320[1];
			let mut fTemp391: F32 = 0.760314 * self.fRec315[1] + 0.64955574 * fTemp379;
			let mut fTemp392: F32 = 0.760314 * fTemp391 - 0.64955574 * self.fRec321[1];
			self.fVec45[(self.IOTA0 & 16383) as usize] = 0.0 - 0.70710677 * (fTemp392 - fTemp390);
			let mut fTemp393: F32 = self.fVec45[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp388) as i32))))) & 16383) as usize];
			self.fVec46[0] = fTemp393;
			self.fRec322[0] = self.fVec46[1] - (fTemp389 + (2.0 - self.fRec323[0])) * (self.fRec322[1] - fTemp393) / (self.fRec323[0] - fTemp389);
			self.fRec320[0] = self.fRec322[0];
			self.fRec325[0] = 0.9999 * (self.fRec325[1] + ((i32::wrapping_mul(iTemp334, iSlow64)) as F32)) + fSlow65;
			let mut fTemp394: F32 = self.fRec325[0] + -1.49999;
			let mut fTemp395: F32 = F32::floor(fTemp394);
			let mut fTemp396: F32 = self.fRec325[0] - fTemp395;
			let mut fTemp397: F32 = fTemp395 + (2.0 - self.fRec325[0]);
			self.fVec47[(self.IOTA0 & 16383) as usize] = fTemp390 + fTemp392;
			let mut fTemp398: F32 = self.fVec47[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp394) as i32))))) & 16383) as usize];
			self.fVec48[0] = fTemp398;
			self.fRec324[0] = 0.70710677 * (fTemp397 * fTemp398 / fTemp396 + self.fVec48[1]) - self.fRec324[1] * fTemp397 / fTemp396;
			self.fRec321[0] = self.fRec324[0];
			let mut fTemp399: F32 = 0.760314 * self.fRec320[1] + 0.64955574 * fTemp387;
			self.fVec49[(self.IOTA0 & 1023) as usize] = fTemp399;
			self.fRec326[0] = fSlow68 * self.fRec327[1] + fSlow67 * self.fRec326[1];
			self.fRec327[0] = ((iTemp334) as F32) + fSlow67 * self.fRec327[1] - fSlow68 * self.fRec326[1];
			let mut fTemp400: F32 = fSlow69 * (self.fRec327[0] + 1.0);
			let mut fTemp401: F32 = fTemp400 + 3.500005;
			let mut iTemp402: i32 = ((fTemp401) as i32);
			let mut iTemp403: i32 = std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp402, 4)));
			let mut fTemp404: F32 = F32::floor(fTemp401);
			let mut fTemp405: F32 = fTemp400 + (2.0 - fTemp404);
			let mut fTemp406: F32 = fTemp400 + (3.0 - fTemp404);
			let mut fTemp407: F32 = fTemp400 + (4.0 - fTemp404);
			let mut fTemp408: F32 = fTemp407 * fTemp406;
			let mut fTemp409: F32 = fTemp408 * fTemp405;
			let mut fTemp410: F32 = fTemp400 + (1.0 - fTemp404);
			let mut fTemp411: F32 = 0.0 - fTemp410;
			let mut iTemp412: i32 = std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp402, 3)));
			let mut fTemp413: F32 = 0.0 - 0.5 * fTemp410;
			let mut fTemp414: F32 = 0.0 - fTemp405;
			let mut iTemp415: i32 = std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp402, 2)));
			let mut fTemp416: F32 = 0.0 - 0.33333334 * fTemp410;
			let mut fTemp417: F32 = 0.0 - 0.5 * fTemp405;
			let mut fTemp418: F32 = 0.0 - fTemp406;
			let mut iTemp419: i32 = std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp402, 1)));
			let mut fTemp420: F32 = fTemp400 + (5.0 - fTemp404);
			let mut fTemp421: F32 = 0.0 - 0.25 * fTemp410;
			let mut fTemp422: F32 = 0.0 - 0.33333334 * fTemp405;
			let mut fTemp423: F32 = 0.0 - 0.5 * fTemp406;
			let mut fTemp424: F32 = 0.0 - fTemp407;
			let mut iTemp425: i32 = std::cmp::min(512, std::cmp::max(0, iTemp402));
			self.fVec50[(self.IOTA0 & 16383) as usize] = self.fVec49[((i32::wrapping_sub(self.IOTA0, iTemp425)) & 1023) as usize] * fTemp424 * fTemp423 * fTemp422 * fTemp421 + fTemp420 * (self.fVec49[((i32::wrapping_sub(self.IOTA0, iTemp419)) & 1023) as usize] * fTemp418 * fTemp417 * fTemp416 + 0.5 * fTemp407 * self.fVec49[((i32::wrapping_sub(self.IOTA0, iTemp415)) & 1023) as usize] * fTemp414 * fTemp413 + 0.16666667 * fTemp408 * self.fVec49[((i32::wrapping_sub(self.IOTA0, iTemp412)) & 1023) as usize] * fTemp411 + 0.041666668 * fTemp409 * self.fVec49[((i32::wrapping_sub(self.IOTA0, iTemp403)) & 1023) as usize]);
			let mut fTemp426: F32 = self.fVec50[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp337) as i32))))) & 16383) as usize];
			self.fVec51[0] = fTemp426;
			self.fRec294[0] = self.fVec51[1] - (fTemp338 + (2.0 - self.fRec295[0])) * (self.fRec294[1] - fTemp426) / (self.fRec295[0] - fTemp338);
			self.fRec331[0] = 0.9999 * (self.fRec331[1] + ((i32::wrapping_mul(iTemp334, iSlow70)) as F32)) + fSlow71;
			let mut fTemp427: F32 = self.fRec331[0] + -1.49999;
			let mut fTemp428: F32 = F32::floor(fTemp427);
			let mut fTemp429: F32 = self.fRec331[0] - fTemp428;
			let mut fTemp430: F32 = fTemp428 + (2.0 - self.fRec331[0]);
			self.fRec333[0] = 0.995 * (self.fRec333[1] + ((i32::wrapping_mul(iTemp334, iSlow72)) as F32)) + fSlow73;
			let mut fTemp431: F32 = self.fRec333[0] + -1.49999;
			let mut fTemp432: F32 = F32::floor(fTemp431);
			let mut fTemp433: F32 = 0.760314 * self.fRec321[1] + 0.64955574 * fTemp391;
			self.fVec52[(self.IOTA0 & 1023) as usize] = fTemp433;
			let mut fTemp434: F32 = fSlow74 * self.fRec327[0];
			let mut fTemp435: F32 = fSlow69 + fTemp434 + 3.500005;
			let mut iTemp436: i32 = ((fTemp435) as i32);
			let mut fTemp437: F32 = F32::floor(fTemp435);
			let mut fTemp438: F32 = fSlow69 + fTemp434 + (2.0 - fTemp437);
			let mut fTemp439: F32 = fSlow69 + fTemp434 + (3.0 - fTemp437);
			let mut fTemp440: F32 = fSlow69 + fTemp434 + (4.0 - fTemp437);
			let mut fTemp441: F32 = fTemp440 * fTemp439;
			let mut fTemp442: F32 = fSlow69 + fTemp434 + (1.0 - fTemp437);
			self.fVec53[(self.IOTA0 & 16383) as usize] = self.fVec52[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, std::cmp::max(0, iTemp436)))) & 1023) as usize] * (0.0 - fTemp440) * (0.0 - 0.5 * fTemp439) * (0.0 - 0.33333334 * fTemp438) * (0.0 - 0.25 * fTemp442) + (fSlow69 + fTemp434 + (5.0 - fTemp437)) * (self.fVec52[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp436, 1))))) & 1023) as usize] * (0.0 - fTemp439) * (0.0 - 0.5 * fTemp438) * (0.0 - 0.33333334 * fTemp442) + 0.5 * fTemp440 * self.fVec52[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp436, 2))))) & 1023) as usize] * (0.0 - fTemp438) * (0.0 - 0.5 * fTemp442) + 0.16666667 * fTemp441 * self.fVec52[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp436, 3))))) & 1023) as usize] * (0.0 - fTemp442) + 0.041666668 * fTemp441 * fTemp438 * self.fVec52[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp436, 4))))) & 1023) as usize]);
			let mut fTemp443: F32 = self.fVec53[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp431) as i32))))) & 16383) as usize];
			self.fVec54[0] = fTemp443;
			self.fRec332[0] = self.fVec54[1] - (fTemp432 + (2.0 - self.fRec333[0])) * (self.fRec332[1] - fTemp443) / (self.fRec333[0] - fTemp432);
			let mut fTemp444: F32 = 0.760314 * self.fRec332[0] - 0.64955574 * self.fRec329[1];
			let mut fTemp445: F32 = 0.760314 * self.fRec294[0] - 0.64955574 * self.fRec328[1];
			self.fVec55[(self.IOTA0 & 16383) as usize] = fTemp445 - fTemp444;
			let mut fTemp446: F32 = self.fVec55[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp427) as i32))))) & 16383) as usize];
			self.fVec56[0] = fTemp446;
			self.fRec330[0] = 0.70710677 * (fTemp430 * fTemp446 / fTemp429 + self.fVec56[1]) - self.fRec330[1] * fTemp430 / fTemp429;
			self.fRec328[0] = self.fRec330[0];
			self.fRec335[0] = 0.9999 * (self.fRec335[1] + ((i32::wrapping_mul(iTemp334, iSlow75)) as F32)) + fSlow76;
			let mut fTemp447: F32 = self.fRec335[0] + -1.49999;
			let mut fTemp448: F32 = F32::floor(fTemp447);
			let mut fTemp449: F32 = self.fRec335[0] - fTemp448;
			let mut fTemp450: F32 = fTemp448 + (2.0 - self.fRec335[0]);
			self.fVec57[(self.IOTA0 & 16383) as usize] = fTemp445 + fTemp444;
			let mut fTemp451: F32 = self.fVec57[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp447) as i32))))) & 16383) as usize];
			self.fVec58[0] = fTemp451;
			self.fRec334[0] = 0.70710677 * (fTemp450 * fTemp451 / fTemp449 + self.fVec58[1]) - self.fRec334[1] * fTemp450 / fTemp449;
			self.fRec329[0] = self.fRec334[0];
			let mut fTemp452: F32 = 0.760314 * self.fRec328[1] + 0.64955574 * self.fRec294[0];
			self.fRec339[0] = 0.9999 * (self.fRec339[1] + ((i32::wrapping_mul(iTemp334, iSlow77)) as F32)) + fSlow78;
			let mut fTemp453: F32 = self.fRec339[0] + -1.49999;
			let mut fTemp454: F32 = F32::floor(fTemp453);
			let mut fTemp455: F32 = 0.760314 * fTemp452 - 0.64955574 * self.fRec336[1];
			let mut fTemp456: F32 = 0.760314 * self.fRec329[1] + 0.64955574 * self.fRec332[0];
			let mut fTemp457: F32 = 0.760314 * fTemp456 - 0.64955574 * self.fRec337[1];
			self.fVec59[(self.IOTA0 & 16383) as usize] = 0.0 - 0.70710677 * (fTemp457 - fTemp455);
			let mut fTemp458: F32 = self.fVec59[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp453) as i32))))) & 16383) as usize];
			self.fVec60[0] = fTemp458;
			self.fRec338[0] = self.fVec60[1] - (fTemp454 + (2.0 - self.fRec339[0])) * (self.fRec338[1] - fTemp458) / (self.fRec339[0] - fTemp454);
			self.fRec336[0] = self.fRec338[0];
			self.fRec341[0] = 0.9999 * (self.fRec341[1] + ((i32::wrapping_mul(iTemp334, iSlow79)) as F32)) + fSlow80;
			let mut fTemp459: F32 = self.fRec341[0] + -1.49999;
			let mut fTemp460: F32 = F32::floor(fTemp459);
			let mut fTemp461: F32 = self.fRec341[0] - fTemp460;
			let mut fTemp462: F32 = fTemp460 + (2.0 - self.fRec341[0]);
			self.fVec61[(self.IOTA0 & 16383) as usize] = fTemp455 + fTemp457;
			let mut iTemp463: i32 = std::cmp::min(8192, std::cmp::max(0, ((fTemp459) as i32)));
			let mut fTemp464: F32 = self.fVec61[((i32::wrapping_sub(self.IOTA0, iTemp463)) & 16383) as usize];
			self.fVec62[0] = fTemp464;
			self.fRec340[0] = 0.70710677 * (fTemp462 * fTemp464 / fTemp461 + self.fVec62[1]) - fTemp462 * self.fRec340[1] / fTemp461;
			self.fRec337[0] = self.fRec340[0];
			let mut fTemp465: F32 = 0.760314 * self.fRec336[1] + 0.64955574 * fTemp452;
			self.fRec345[0] = 0.9999 * (self.fRec345[1] + ((i32::wrapping_mul(iTemp334, iSlow81)) as F32)) + fSlow82;
			let mut fTemp466: F32 = self.fRec345[0] + -1.49999;
			let mut fTemp467: F32 = F32::floor(fTemp466);
			let mut fTemp468: F32 = self.fRec345[0] - fTemp467;
			let mut fTemp469: F32 = fTemp467 + (2.0 - self.fRec345[0]);
			let mut fTemp470: F32 = 0.760314 * self.fRec337[1] + 0.64955574 * fTemp456;
			let mut fTemp471: F32 = 0.760314 * fTemp470 - 0.64955574 * self.fRec343[1];
			let mut fTemp472: F32 = 0.760314 * fTemp465 - 0.64955574 * self.fRec342[1];
			self.fVec63[(self.IOTA0 & 16383) as usize] = fTemp472 - fTemp471;
			let mut fTemp473: F32 = self.fVec63[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp466) as i32))))) & 16383) as usize];
			self.fVec64[0] = fTemp473;
			self.fRec344[0] = 0.70710677 * (fTemp469 * fTemp473 / fTemp468 + self.fVec64[1]) - self.fRec344[1] * fTemp469 / fTemp468;
			self.fRec342[0] = self.fRec344[0];
			self.fRec347[0] = 0.9999 * (self.fRec347[1] + ((i32::wrapping_mul(iTemp334, iSlow83)) as F32)) + fSlow84;
			let mut fTemp474: F32 = self.fRec347[0] + -1.49999;
			let mut fTemp475: F32 = F32::floor(fTemp474);
			let mut fTemp476: F32 = self.fRec347[0] - fTemp475;
			let mut fTemp477: F32 = fTemp475 + (2.0 - self.fRec347[0]);
			self.fVec65[(self.IOTA0 & 16383) as usize] = fTemp472 + fTemp471;
			let mut iTemp478: i32 = std::cmp::min(8192, std::cmp::max(0, ((fTemp474) as i32)));
			let mut fTemp479: F32 = self.fVec65[((i32::wrapping_sub(self.IOTA0, iTemp478)) & 16383) as usize];
			self.fVec66[0] = fTemp479;
			self.fRec346[0] = 0.70710677 * (fTemp477 * fTemp479 / fTemp476 + self.fVec66[1]) - self.fRec346[1] * fTemp477 / fTemp476;
			self.fRec343[0] = self.fRec346[0];
			let mut fTemp480: F32 = 0.760314 * self.fRec342[1] + 0.64955574 * fTemp465;
			self.fRec351[0] = 0.9999 * (self.fRec351[1] + ((i32::wrapping_mul(iTemp334, iSlow85)) as F32)) + fSlow86;
			let mut fTemp481: F32 = self.fRec351[0] + -1.49999;
			let mut fTemp482: F32 = F32::floor(fTemp481);
			let mut fTemp483: F32 = self.fRec351[0] - fTemp482;
			let mut fTemp484: F32 = 0.760314 * fTemp480 - 0.64955574 * self.fRec348[1];
			let mut fTemp485: F32 = 0.760314 * self.fRec343[1] + 0.64955574 * fTemp470;
			let mut fTemp486: F32 = 0.760314 * fTemp485 - 0.64955574 * self.fRec349[1];
			self.fVec67[(self.IOTA0 & 16383) as usize] = 0.0 - 0.70710677 * (fTemp486 - fTemp484);
			let mut iTemp487: i32 = std::cmp::min(8192, std::cmp::max(0, ((fTemp481) as i32)));
			let mut fTemp488: F32 = self.fVec67[((i32::wrapping_sub(self.IOTA0, iTemp487)) & 16383) as usize];
			self.fVec68[0] = fTemp488;
			let mut fTemp489: F32 = fTemp482 + (2.0 - self.fRec351[0]);
			self.fRec350[0] = self.fVec68[1] - fTemp489 * (self.fRec350[1] - fTemp488) / fTemp483;
			self.fRec348[0] = self.fRec350[0];
			self.fRec353[0] = 0.9999 * (self.fRec353[1] + ((i32::wrapping_mul(iTemp334, iSlow87)) as F32)) + fSlow88;
			let mut fTemp490: F32 = self.fRec353[0] + -1.49999;
			let mut fTemp491: F32 = F32::floor(fTemp490);
			let mut fTemp492: F32 = self.fRec353[0] - fTemp491;
			let mut fTemp493: F32 = fTemp491 + (2.0 - self.fRec353[0]);
			self.fVec69[(self.IOTA0 & 16383) as usize] = fTemp484 + fTemp486;
			let mut iTemp494: i32 = std::cmp::min(8192, std::cmp::max(0, ((fTemp490) as i32)));
			let mut fTemp495: F32 = self.fVec69[((i32::wrapping_sub(self.IOTA0, iTemp494)) & 16383) as usize];
			self.fVec70[0] = fTemp495;
			self.fRec352[0] = 0.70710677 * (fTemp493 * fTemp495 / fTemp492 + self.fVec70[1]) - fTemp493 * self.fRec352[1] / fTemp492;
			self.fRec349[0] = self.fRec352[0];
			let mut fTemp496: F32 = 0.760314 * self.fRec348[1] + 0.64955574 * fTemp480;
			self.fRec357[0] = 0.9999 * (self.fRec357[1] + ((i32::wrapping_mul(iTemp334, iSlow89)) as F32)) + fSlow90;
			let mut fTemp497: F32 = self.fRec357[0] + -1.49999;
			let mut fTemp498: F32 = F32::floor(fTemp497);
			let mut fTemp499: F32 = 0.760314 * fTemp496 - 0.64955574 * self.fRec354[1];
			let mut fTemp500: F32 = 0.760314 * self.fRec349[1] + 0.64955574 * fTemp485;
			let mut fTemp501: F32 = 0.760314 * fTemp500 - 0.64955574 * self.fRec355[1];
			self.fVec71[(self.IOTA0 & 16383) as usize] = 0.0 - 0.70710677 * (fTemp501 - fTemp499);
			let mut fTemp502: F32 = self.fVec71[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp497) as i32))))) & 16383) as usize];
			self.fVec72[0] = fTemp502;
			self.fRec356[0] = self.fVec72[1] - (fTemp498 + (2.0 - self.fRec357[0])) * (self.fRec356[1] - fTemp502) / (self.fRec357[0] - fTemp498);
			self.fRec354[0] = self.fRec356[0];
			self.fRec359[0] = 0.9999 * (self.fRec359[1] + ((i32::wrapping_mul(iTemp334, iSlow91)) as F32)) + fSlow92;
			let mut fTemp503: F32 = self.fRec359[0] + -1.49999;
			let mut fTemp504: F32 = F32::floor(fTemp503);
			let mut fTemp505: F32 = self.fRec359[0] - fTemp504;
			let mut fTemp506: F32 = fTemp504 + (2.0 - self.fRec359[0]);
			self.fVec73[(self.IOTA0 & 16383) as usize] = fTemp499 + fTemp501;
			let mut iTemp507: i32 = std::cmp::min(8192, std::cmp::max(0, ((fTemp503) as i32)));
			let mut fTemp508: F32 = self.fVec73[((i32::wrapping_sub(self.IOTA0, iTemp507)) & 16383) as usize];
			self.fVec74[0] = fTemp508;
			self.fRec358[0] = 0.70710677 * (fTemp506 * fTemp508 / fTemp505 + self.fVec74[1]) - self.fRec358[1] * fTemp506 / fTemp505;
			self.fRec355[0] = self.fRec358[0];
			let mut fTemp509: F32 = 0.760314 * self.fRec354[1] + 0.64955574 * fTemp496;
			self.fVec75[(self.IOTA0 & 16383) as usize] = fTemp509;
			let mut fTemp510: F32 = fSlow69 * (self.fRec326[0] + 1.0);
			let mut fTemp511: F32 = fTemp510 + 3.500005;
			let mut iTemp512: i32 = ((fTemp511) as i32);
			let mut iTemp513: i32 = std::cmp::max(0, i32::wrapping_add(iTemp512, 4));
			let mut fTemp514: F32 = F32::floor(fTemp511);
			let mut fTemp515: F32 = fTemp510 + (2.0 - fTemp514);
			let mut fTemp516: F32 = fTemp510 + (3.0 - fTemp514);
			let mut fTemp517: F32 = fTemp510 + (4.0 - fTemp514);
			let mut fTemp518: F32 = fTemp517 * fTemp516;
			let mut fTemp519: F32 = fTemp518 * fTemp515;
			let mut fTemp520: F32 = fTemp510 + (1.0 - fTemp514);
			let mut fTemp521: F32 = 0.0 - fTemp520;
			let mut iTemp522: i32 = std::cmp::max(0, i32::wrapping_add(iTemp512, 3));
			let mut fTemp523: F32 = 0.0 - 0.5 * fTemp520;
			let mut fTemp524: F32 = 0.0 - fTemp515;
			let mut iTemp525: i32 = std::cmp::max(0, i32::wrapping_add(iTemp512, 2));
			let mut fTemp526: F32 = 0.0 - 0.33333334 * fTemp520;
			let mut fTemp527: F32 = 0.0 - 0.5 * fTemp515;
			let mut fTemp528: F32 = 0.0 - fTemp516;
			let mut iTemp529: i32 = std::cmp::max(0, i32::wrapping_add(iTemp512, 1));
			let mut fTemp530: F32 = fTemp510 + (5.0 - fTemp514);
			let mut fTemp531: F32 = 0.0 - 0.25 * fTemp520;
			let mut fTemp532: F32 = 0.0 - 0.33333334 * fTemp515;
			let mut fTemp533: F32 = 0.0 - 0.5 * fTemp516;
			let mut fTemp534: F32 = 0.0 - fTemp517;
			let mut iTemp535: i32 = std::cmp::max(0, iTemp512);
			self.fVec76[(self.IOTA0 & 16383) as usize] = self.fVec75[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, iTemp535))) & 16383) as usize] * fTemp534 * fTemp533 * fTemp532 * fTemp531 + fTemp530 * (self.fVec75[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, iTemp529))) & 16383) as usize] * fTemp528 * fTemp527 * fTemp526 + 0.5 * fTemp517 * self.fVec75[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, iTemp525))) & 16383) as usize] * fTemp524 * fTemp523 + 0.16666667 * fTemp518 * self.fVec75[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, iTemp522))) & 16383) as usize] * fTemp521 + 0.041666668 * fTemp519 * self.fVec75[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, iTemp513))) & 16383) as usize]);
			let mut fTemp536: F32 = self.fVec76[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp335) as i32))))) & 16383) as usize];
			self.fVec77[0] = fTemp536;
			self.fRec292[0] = self.fVec77[1] - (fTemp336 + (2.0 - self.fRec293[0])) * (self.fRec292[1] - fTemp536) / (self.fRec293[0] - fTemp336);
			self.fRec291[0] = 0.0 - self.fConst41 * (self.fConst39 * self.fRec291[1] - (self.fRec292[0] + self.fRec292[1]));
			self.fRec290[0] = self.fRec291[0] - self.fConst37 * (self.fConst36 * self.fRec290[2] + self.fConst32 * self.fRec290[1]);
			self.fRec289[0] = self.fConst37 * (self.fRec290[2] + self.fRec290[0] + 2.0 * self.fRec290[1]) - self.fConst35 * (self.fConst34 * self.fRec289[2] + self.fConst32 * self.fRec289[1]);
			let mut fTemp537: F32 = self.fRec289[2] + self.fRec289[0] + 2.0 * self.fRec289[1];
			self.fVec78[0] = fTemp537;
			self.fRec288[0] = 0.0 - self.fConst44 * (self.fConst42 * self.fRec288[1] - self.fConst35 * (fTemp537 + self.fVec78[1]));
			self.fRec287[0] = self.fRec288[0] - self.fConst28 * (self.fConst27 * self.fRec287[2] + self.fConst23 * self.fRec287[1]);
			self.fRec286[0] = self.fConst28 * (self.fRec287[2] + self.fRec287[0] + 2.0 * self.fRec287[1]) - self.fConst26 * (self.fConst25 * self.fRec286[2] + self.fConst23 * self.fRec286[1]);
			self.fRec362[0] = self.fConst35 * (self.fConst47 * self.fVec78[1] + self.fConst46 * fTemp537) - self.fConst45 * self.fRec362[1];
			self.fRec361[0] = self.fRec362[0] - self.fConst28 * (self.fConst27 * self.fRec361[2] + self.fConst23 * self.fRec361[1]);
			self.fRec360[0] = self.fConst28 * (self.fConst48 * self.fRec361[1] + self.fConst22 * self.fRec361[0] + self.fConst22 * self.fRec361[2]) - self.fConst26 * (self.fConst25 * self.fRec360[2] + self.fConst23 * self.fRec360[1]);
			let mut fTemp538: F32 = self.fConst23 * self.fRec363[1];
			self.fRec366[0] = self.fConst51 * self.fRec292[1] - self.fConst41 * (self.fConst39 * self.fRec366[1] - self.fConst33 * self.fRec292[0]);
			self.fRec365[0] = self.fRec366[0] - self.fConst37 * (self.fConst36 * self.fRec365[2] + self.fConst32 * self.fRec365[1]);
			self.fRec364[0] = self.fConst37 * (self.fConst31 * self.fRec365[0] + self.fConst52 * self.fRec365[1] + self.fConst31 * self.fRec365[2]) - self.fConst35 * (self.fConst34 * self.fRec364[2] + self.fConst32 * self.fRec364[1]);
			self.fRec363[0] = self.fConst35 * (self.fConst31 * self.fRec364[0] + self.fConst52 * self.fRec364[1] + self.fConst31 * self.fRec364[2]) - self.fConst50 * (self.fConst49 * self.fRec363[2] + fTemp538);
			let mut fTemp539: F32 = fTemp331 + fTemp332 + fSlow93 * (self.fRec363[2] + self.fConst50 * (fTemp538 + self.fConst49 * self.fRec363[0]) + self.fConst26 * (self.fConst48 * self.fRec360[1] + self.fConst22 * self.fRec360[0] + self.fConst22 * self.fRec360[2] + self.fRec286[2] + self.fRec286[0] + 2.0 * self.fRec286[1]));
			self.fVec79[(self.IOTA0 & 1023) as usize] = fTemp539;
			self.fRec285[0] = fSlow94 * self.fRec285[1] + fSlow95 * (fTemp534 * fTemp533 * fTemp532 * fTemp531 * self.fVec79[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, iTemp535))) & 1023) as usize] + fTemp530 * (fTemp528 * fTemp527 * fTemp526 * self.fVec79[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, iTemp529))) & 1023) as usize] + 0.5 * fTemp517 * fTemp524 * fTemp523 * self.fVec79[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, iTemp525))) & 1023) as usize] + 0.16666667 * fTemp518 * fTemp521 * self.fVec79[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, iTemp522))) & 1023) as usize] + 0.041666668 * fTemp519 * self.fVec79[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, iTemp513))) & 1023) as usize]));
			self.fRec378[0] = 0.995 * (self.fRec378[1] + ((i32::wrapping_mul(iTemp334, iSlow98)) as F32)) + fSlow99;
			let mut fTemp540: F32 = self.fRec378[0] + -1.49999;
			let mut fTemp541: F32 = F32::floor(fTemp540);
			let mut fTemp542: F32 = 0.760314 * self.fRec355[1] + 0.64955574 * fTemp500;
			self.fVec80[(self.IOTA0 & 16383) as usize] = fTemp542;
			let mut fTemp543: F32 = fSlow74 * self.fRec326[0];
			let mut fTemp544: F32 = fSlow69 + fTemp543 + 3.500005;
			let mut iTemp545: i32 = ((fTemp544) as i32);
			let mut fTemp546: F32 = F32::floor(fTemp544);
			let mut fTemp547: F32 = fSlow69 + fTemp543 + (2.0 - fTemp546);
			let mut fTemp548: F32 = fSlow69 + fTemp543 + (3.0 - fTemp546);
			let mut fTemp549: F32 = fSlow69 + fTemp543 + (4.0 - fTemp546);
			let mut fTemp550: F32 = fTemp549 * fTemp548;
			let mut fTemp551: F32 = fSlow69 + fTemp543 + (1.0 - fTemp546);
			self.fVec81[(self.IOTA0 & 16383) as usize] = self.fVec80[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, iTemp545)))) & 16383) as usize] * (0.0 - fTemp549) * (0.0 - 0.5 * fTemp548) * (0.0 - 0.33333334 * fTemp547) * (0.0 - 0.25 * fTemp551) + (fSlow69 + fTemp543 + (5.0 - fTemp546)) * (self.fVec80[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, i32::wrapping_add(iTemp545, 1))))) & 16383) as usize] * (0.0 - fTemp548) * (0.0 - 0.5 * fTemp547) * (0.0 - 0.33333334 * fTemp551) + 0.5 * fTemp549 * self.fVec80[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, i32::wrapping_add(iTemp545, 2))))) & 16383) as usize] * (0.0 - fTemp547) * (0.0 - 0.5 * fTemp551) + 0.16666667 * fTemp550 * self.fVec80[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, i32::wrapping_add(iTemp545, 3))))) & 16383) as usize] * (0.0 - fTemp551) + 0.041666668 * fTemp550 * fTemp547 * self.fVec80[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, i32::wrapping_add(iTemp545, 4))))) & 16383) as usize]);
			let mut fTemp552: F32 = self.fVec81[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp540) as i32))))) & 16383) as usize];
			self.fVec82[0] = fTemp552;
			self.fRec377[0] = self.fVec82[1] - (fTemp541 + (2.0 - self.fRec378[0])) * (self.fRec377[1] - fTemp552) / (self.fRec378[0] - fTemp541);
			self.fRec376[0] = 0.0 - self.fConst41 * (self.fConst39 * self.fRec376[1] - (self.fRec377[0] + self.fRec377[1]));
			self.fRec375[0] = self.fRec376[0] - self.fConst37 * (self.fConst36 * self.fRec375[2] + self.fConst32 * self.fRec375[1]);
			self.fRec374[0] = self.fConst37 * (self.fRec375[2] + self.fRec375[0] + 2.0 * self.fRec375[1]) - self.fConst35 * (self.fConst34 * self.fRec374[2] + self.fConst32 * self.fRec374[1]);
			let mut fTemp553: F32 = self.fRec374[2] + self.fRec374[0] + 2.0 * self.fRec374[1];
			self.fVec83[0] = fTemp553;
			self.fRec373[0] = 0.0 - self.fConst44 * (self.fConst42 * self.fRec373[1] - self.fConst35 * (fTemp553 + self.fVec83[1]));
			self.fRec372[0] = self.fRec373[0] - self.fConst28 * (self.fConst27 * self.fRec372[2] + self.fConst23 * self.fRec372[1]);
			self.fRec371[0] = self.fConst28 * (self.fRec372[2] + self.fRec372[0] + 2.0 * self.fRec372[1]) - self.fConst26 * (self.fConst25 * self.fRec371[2] + self.fConst23 * self.fRec371[1]);
			self.fRec381[0] = self.fConst35 * (self.fConst46 * fTemp553 + self.fConst47 * self.fVec83[1]) - self.fConst45 * self.fRec381[1];
			self.fRec380[0] = self.fRec381[0] - self.fConst28 * (self.fConst27 * self.fRec380[2] + self.fConst23 * self.fRec380[1]);
			self.fRec379[0] = self.fConst28 * (self.fConst22 * self.fRec380[0] + self.fConst48 * self.fRec380[1] + self.fConst22 * self.fRec380[2]) - self.fConst26 * (self.fConst25 * self.fRec379[2] + self.fConst23 * self.fRec379[1]);
			let mut fTemp554: F32 = self.fConst23 * self.fRec382[1];
			self.fRec385[0] = self.fConst51 * self.fRec377[1] - self.fConst41 * (self.fConst39 * self.fRec385[1] - self.fConst33 * self.fRec377[0]);
			self.fRec384[0] = self.fRec385[0] - self.fConst37 * (self.fConst36 * self.fRec384[2] + self.fConst32 * self.fRec384[1]);
			self.fRec383[0] = self.fConst37 * (self.fConst31 * self.fRec384[0] + self.fConst52 * self.fRec384[1] + self.fConst31 * self.fRec384[2]) - self.fConst35 * (self.fConst34 * self.fRec383[2] + self.fConst32 * self.fRec383[1]);
			self.fRec382[0] = self.fConst35 * (self.fConst31 * self.fRec383[0] + self.fConst52 * self.fRec383[1] + self.fConst31 * self.fRec383[2]) - self.fConst50 * (self.fConst49 * self.fRec382[2] + fTemp554);
			let mut fTemp555: F32 = fTemp333 + fSlow93 * (self.fRec382[2] + self.fConst50 * (fTemp554 + self.fConst49 * self.fRec382[0]) + self.fConst26 * (self.fConst48 * self.fRec379[1] + self.fConst22 * self.fRec379[0] + self.fConst22 * self.fRec379[2] + self.fRec371[2] + self.fRec371[0] + 2.0 * self.fRec371[1]));
			self.fVec84[(self.IOTA0 & 1023) as usize] = fTemp555;
			self.fRec370[0] = fSlow94 * self.fRec370[1] + fSlow95 * (fTemp424 * fTemp423 * fTemp422 * fTemp421 * self.fVec84[((i32::wrapping_sub(self.IOTA0, iTemp425)) & 1023) as usize] + fTemp420 * (fTemp418 * fTemp417 * fTemp416 * self.fVec84[((i32::wrapping_sub(self.IOTA0, iTemp419)) & 1023) as usize] + 0.5 * fTemp407 * fTemp414 * fTemp413 * self.fVec84[((i32::wrapping_sub(self.IOTA0, iTemp415)) & 1023) as usize] + 0.16666667 * fTemp408 * fTemp411 * self.fVec84[((i32::wrapping_sub(self.IOTA0, iTemp412)) & 1023) as usize] + 0.041666668 * fTemp409 * self.fVec84[((i32::wrapping_sub(self.IOTA0, iTemp403)) & 1023) as usize]));
			let mut fTemp556: F32 = fSlow100 * self.fRec370[0] - fSlow97 * self.fRec368[1];
			let mut fTemp557: F32 = fSlow100 * self.fRec285[0] - fSlow97 * self.fRec367[1];
			self.fVec85[(self.IOTA0 & 16383) as usize] = fTemp557 - fTemp556;
			let mut fTemp558: F32 = self.fVec85[((i32::wrapping_sub(self.IOTA0, iTemp463)) & 16383) as usize];
			self.fVec86[0] = fTemp558;
			self.fRec369[0] = 0.70710677 * (fTemp462 * fTemp558 / fTemp461 + self.fVec86[1]) - self.fRec369[1] * fTemp462 / fTemp461;
			self.fRec367[0] = self.fRec369[0];
			self.fRec387[0] = 0.9999 * (self.fRec387[1] + ((i32::wrapping_mul(iTemp334, iSlow101)) as F32)) + fSlow102;
			let mut fTemp559: F32 = self.fRec387[0] + -1.49999;
			let mut fTemp560: F32 = F32::floor(fTemp559);
			let mut fTemp561: F32 = self.fRec387[0] - fTemp560;
			let mut fTemp562: F32 = fTemp560 + (2.0 - self.fRec387[0]);
			self.fVec87[(self.IOTA0 & 16383) as usize] = fTemp557 + fTemp556;
			let mut fTemp563: F32 = self.fVec87[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp559) as i32))))) & 16383) as usize];
			self.fVec88[0] = fTemp563;
			self.fRec386[0] = 0.70710677 * (fTemp562 * fTemp563 / fTemp561 + self.fVec88[1]) - self.fRec386[1] * fTemp562 / fTemp561;
			self.fRec368[0] = self.fRec386[0];
			let mut fTemp564: F32 = fSlow100 * self.fRec367[1] + fSlow97 * self.fRec285[0];
			let mut fTemp565: F32 = fSlow100 * self.fRec368[1] + fSlow97 * self.fRec370[0];
			let mut fTemp566: F32 = fSlow100 * fTemp565 - fSlow97 * self.fRec389[1];
			let mut fTemp567: F32 = fSlow100 * fTemp564 - fSlow97 * self.fRec388[1];
			self.fVec89[(self.IOTA0 & 16383) as usize] = fTemp567 - fTemp566;
			let mut fTemp568: F32 = self.fVec89[((i32::wrapping_sub(self.IOTA0, iTemp487)) & 16383) as usize];
			self.fVec90[0] = fTemp568;
			self.fRec390[0] = 0.70710677 * (fTemp489 * fTemp568 / fTemp483 + self.fVec90[1]) - self.fRec390[1] * fTemp489 / fTemp483;
			self.fRec388[0] = self.fRec390[0];
			self.fVec91[(self.IOTA0 & 16383) as usize] = fTemp567 + fTemp566;
			let mut fTemp569: F32 = self.fVec91[((i32::wrapping_sub(self.IOTA0, iTemp478)) & 16383) as usize];
			self.fVec92[0] = fTemp569;
			self.fRec391[0] = 0.70710677 * (fTemp477 * fTemp569 / fTemp476 + self.fVec92[1]) - fTemp477 * self.fRec391[1] / fTemp476;
			self.fRec389[0] = self.fRec391[0];
			let mut fTemp570: F32 = fSlow100 * self.fRec388[1] + fSlow97 * fTemp564;
			let mut fTemp571: F32 = fSlow100 * self.fRec389[1] + fSlow97 * fTemp565;
			let mut fTemp572: F32 = fSlow100 * fTemp571 - fSlow97 * self.fRec393[1];
			let mut fTemp573: F32 = fSlow100 * fTemp570 - fSlow97 * self.fRec392[1];
			self.fVec93[(self.IOTA0 & 16383) as usize] = fTemp573 - fTemp572;
			let mut fTemp574: F32 = self.fVec93[((i32::wrapping_sub(self.IOTA0, iTemp494)) & 16383) as usize];
			self.fVec94[0] = fTemp574;
			self.fRec394[0] = 0.70710677 * (fTemp493 * fTemp574 / fTemp492 + self.fVec94[1]) - self.fRec394[1] * fTemp493 / fTemp492;
			self.fRec392[0] = self.fRec394[0];
			self.fRec396[0] = 0.9999 * (self.fRec396[1] + ((i32::wrapping_mul(iTemp334, iSlow103)) as F32)) + fSlow104;
			let mut fTemp575: F32 = self.fRec396[0] + -1.49999;
			let mut fTemp576: F32 = F32::floor(fTemp575);
			let mut fTemp577: F32 = self.fRec396[0] - fTemp576;
			let mut fTemp578: F32 = fTemp576 + (2.0 - self.fRec396[0]);
			self.fVec95[(self.IOTA0 & 16383) as usize] = fTemp573 + fTemp572;
			let mut fTemp579: F32 = self.fVec95[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp575) as i32))))) & 16383) as usize];
			self.fVec96[0] = fTemp579;
			self.fRec395[0] = 0.70710677 * (fTemp578 * fTemp579 / fTemp577 + self.fVec96[1]) - self.fRec395[1] * fTemp578 / fTemp577;
			self.fRec393[0] = self.fRec395[0];
			let mut fTemp580: F32 = fSlow100 * self.fRec392[1] + fSlow97 * fTemp570;
			self.fRec400[0] = 0.9999 * (self.fRec400[1] + ((i32::wrapping_mul(iTemp334, iSlow105)) as F32)) + fSlow106;
			let mut fTemp581: F32 = self.fRec400[0] + -1.49999;
			let mut fTemp582: F32 = F32::floor(fTemp581);
			let mut fTemp583: F32 = self.fRec400[0] - fTemp582;
			let mut fTemp584: F32 = fTemp582 + (2.0 - self.fRec400[0]);
			let mut fTemp585: F32 = fSlow100 * self.fRec393[1] + fSlow97 * fTemp571;
			let mut fTemp586: F32 = fSlow100 * fTemp585 - fSlow97 * self.fRec398[1];
			let mut fTemp587: F32 = fSlow100 * fTemp580 - fSlow97 * self.fRec397[1];
			self.fVec97[(self.IOTA0 & 16383) as usize] = fTemp587 - fTemp586;
			let mut fTemp588: F32 = self.fVec97[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp581) as i32))))) & 16383) as usize];
			self.fVec98[0] = fTemp588;
			self.fRec399[0] = 0.70710677 * (fTemp584 * fTemp588 / fTemp583 + self.fVec98[1]) - self.fRec399[1] * fTemp584 / fTemp583;
			self.fRec397[0] = self.fRec399[0];
			self.fVec99[(self.IOTA0 & 16383) as usize] = fTemp587 + fTemp586;
			let mut fTemp589: F32 = self.fVec99[((i32::wrapping_sub(self.IOTA0, iTemp507)) & 16383) as usize];
			self.fVec100[0] = fTemp589;
			self.fRec401[0] = 0.70710677 * (fTemp506 * fTemp589 / fTemp505 + self.fVec100[1]) - fTemp506 * self.fRec401[1] / fTemp505;
			self.fRec398[0] = self.fRec401[0];
			self.fRec283[0] = fSlow100 * self.fRec397[1] + fSlow97 * fTemp580;
			self.fRec284[0] = fSlow100 * self.fRec398[1] + fSlow97 * fTemp585;
			self.fRec402[0] = fSlow107 + self.fConst2 * self.fRec402[1];
			let mut fTemp590: F32 = self.fRec402[0] * (fSlow39 * (self.fRec283[0] + self.fRec284[0]) + fSlow40 * fTemp333);
			*output0 = fTemp590;
			*output1 = fTemp590;
			self.fVec0[1] = self.fVec0[0];
			self.iVec1[1] = self.iVec1[0];
			self.fRec2[1] = self.fRec2[0];
			self.fRec1[1] = self.fRec1[0];
			self.fRec3[1] = self.fRec3[0];
			self.IOTA0 = i32::wrapping_add(self.IOTA0, 1);
			self.fRec33[1] = self.fRec33[0];
			self.fRec37[1] = self.fRec37[0];
			self.iRec39[1] = self.iRec39[0];
			self.iRec41[1] = self.iRec41[0];
			self.fRec40[2] = self.fRec40[1];
			self.fRec40[1] = self.fRec40[0];
			self.fRec45[1] = self.fRec45[0];
			self.iVec2[1] = self.iVec2[0];
			self.iRec46[1] = self.iRec46[0];
			self.fRec43[1] = self.fRec43[0];
			self.fRec42[1] = self.fRec42[0];
			for j0 in (1..=3).rev() {
				self.fRec47[(j0) as usize] = self.fRec47[(i32::wrapping_sub(j0, 1)) as usize];
			}
			self.fVec3[1] = self.fVec3[0];
			self.fVec4[2] = self.fVec4[1];
			self.fVec4[1] = self.fVec4[0];
			self.fRec29[1] = self.fRec29[0];
			self.fRec25[1] = self.fRec25[0];
			self.fRec23[1] = self.fRec23[0];
			for j1 in (1..=3).rev() {
				self.fRec19[(j1) as usize] = self.fRec19[(i32::wrapping_sub(j1, 1)) as usize];
			}
			self.fRec14[1] = self.fRec14[0];
			self.fRec8[1] = self.fRec8[0];
			self.fRec7[1] = self.fRec7[0];
			self.fRec6[1] = self.fRec6[0];
			self.fRec4[1] = self.fRec4[0];
			self.fRec0[1] = self.fRec0[0];
			self.fRec50[1] = self.fRec50[0];
			self.fRec80[1] = self.fRec80[0];
			self.fRec84[1] = self.fRec84[0];
			self.fRec89[1] = self.fRec89[0];
			self.iVec5[1] = self.iVec5[0];
			self.iRec90[1] = self.iRec90[0];
			self.fRec87[1] = self.fRec87[0];
			self.fRec86[1] = self.fRec86[0];
			for j2 in (1..=3).rev() {
				self.fRec91[(j2) as usize] = self.fRec91[(i32::wrapping_sub(j2, 1)) as usize];
			}
			self.fVec6[1] = self.fVec6[0];
			self.fVec7[1] = self.fVec7[0];
			self.iRec93[1] = self.iRec93[0];
			self.fRec94[2] = self.fRec94[1];
			self.fRec94[1] = self.fRec94[0];
			self.fVec8[2] = self.fVec8[1];
			self.fVec8[1] = self.fVec8[0];
			self.fRec76[1] = self.fRec76[0];
			self.fRec72[1] = self.fRec72[0];
			self.fRec70[1] = self.fRec70[0];
			for j3 in (1..=3).rev() {
				self.fRec66[(j3) as usize] = self.fRec66[(i32::wrapping_sub(j3, 1)) as usize];
			}
			self.fRec61[1] = self.fRec61[0];
			self.fRec55[1] = self.fRec55[0];
			self.fRec54[1] = self.fRec54[0];
			self.fRec53[1] = self.fRec53[0];
			self.fRec51[1] = self.fRec51[0];
			self.fRec49[1] = self.fRec49[0];
			self.fVec9[1] = self.fVec9[0];
			self.fRec97[1] = self.fRec97[0];
			self.fRec96[1] = self.fRec96[0];
			self.fRec127[1] = self.fRec127[0];
			self.fRec131[1] = self.fRec131[0];
			self.fRec136[1] = self.fRec136[0];
			self.iVec10[1] = self.iVec10[0];
			self.iRec137[1] = self.iRec137[0];
			self.fRec134[1] = self.fRec134[0];
			self.fRec133[1] = self.fRec133[0];
			for j4 in (1..=3).rev() {
				self.fRec138[(j4) as usize] = self.fRec138[(i32::wrapping_sub(j4, 1)) as usize];
			}
			self.fVec11[1] = self.fVec11[0];
			self.iRec140[1] = self.iRec140[0];
			self.fRec141[2] = self.fRec141[1];
			self.fRec141[1] = self.fRec141[0];
			self.fVec12[2] = self.fVec12[1];
			self.fVec12[1] = self.fVec12[0];
			self.fRec123[1] = self.fRec123[0];
			self.fRec119[1] = self.fRec119[0];
			self.fRec117[1] = self.fRec117[0];
			for j5 in (1..=3).rev() {
				self.fRec113[(j5) as usize] = self.fRec113[(i32::wrapping_sub(j5, 1)) as usize];
			}
			self.fRec108[1] = self.fRec108[0];
			self.fRec102[1] = self.fRec102[0];
			self.fRec101[1] = self.fRec101[0];
			self.fRec100[1] = self.fRec100[0];
			self.fRec98[1] = self.fRec98[0];
			self.fRec95[1] = self.fRec95[0];
			self.fVec13[1] = self.fVec13[0];
			self.fRec144[1] = self.fRec144[0];
			self.fRec143[1] = self.fRec143[0];
			self.fRec174[1] = self.fRec174[0];
			self.fRec178[1] = self.fRec178[0];
			self.fRec183[1] = self.fRec183[0];
			self.iVec14[1] = self.iVec14[0];
			self.iRec184[1] = self.iRec184[0];
			self.fRec181[1] = self.fRec181[0];
			self.fRec180[1] = self.fRec180[0];
			for j6 in (1..=3).rev() {
				self.fRec185[(j6) as usize] = self.fRec185[(i32::wrapping_sub(j6, 1)) as usize];
			}
			self.fVec15[1] = self.fVec15[0];
			self.iRec187[1] = self.iRec187[0];
			self.fRec188[2] = self.fRec188[1];
			self.fRec188[1] = self.fRec188[0];
			self.fVec16[2] = self.fVec16[1];
			self.fVec16[1] = self.fVec16[0];
			self.fRec170[1] = self.fRec170[0];
			self.fRec166[1] = self.fRec166[0];
			self.fRec164[1] = self.fRec164[0];
			for j7 in (1..=3).rev() {
				self.fRec160[(j7) as usize] = self.fRec160[(i32::wrapping_sub(j7, 1)) as usize];
			}
			self.fRec155[1] = self.fRec155[0];
			self.fRec149[1] = self.fRec149[0];
			self.fRec148[1] = self.fRec148[0];
			self.fRec147[1] = self.fRec147[0];
			self.fRec145[1] = self.fRec145[0];
			self.fRec142[1] = self.fRec142[0];
			self.fVec17[1] = self.fVec17[0];
			self.fRec191[1] = self.fRec191[0];
			self.fRec190[1] = self.fRec190[0];
			self.fRec221[1] = self.fRec221[0];
			self.fRec225[1] = self.fRec225[0];
			self.fRec230[1] = self.fRec230[0];
			self.iVec18[1] = self.iVec18[0];
			self.iRec231[1] = self.iRec231[0];
			self.fRec228[1] = self.fRec228[0];
			self.fRec227[1] = self.fRec227[0];
			for j8 in (1..=3).rev() {
				self.fRec232[(j8) as usize] = self.fRec232[(i32::wrapping_sub(j8, 1)) as usize];
			}
			self.fVec19[1] = self.fVec19[0];
			self.iRec234[1] = self.iRec234[0];
			self.fRec235[2] = self.fRec235[1];
			self.fRec235[1] = self.fRec235[0];
			self.fVec20[2] = self.fVec20[1];
			self.fVec20[1] = self.fVec20[0];
			self.fRec217[1] = self.fRec217[0];
			self.fRec213[1] = self.fRec213[0];
			self.fRec211[1] = self.fRec211[0];
			for j9 in (1..=3).rev() {
				self.fRec207[(j9) as usize] = self.fRec207[(i32::wrapping_sub(j9, 1)) as usize];
			}
			self.fRec202[1] = self.fRec202[0];
			self.fRec196[1] = self.fRec196[0];
			self.fRec195[1] = self.fRec195[0];
			self.fRec194[1] = self.fRec194[0];
			self.fRec192[1] = self.fRec192[0];
			self.fRec189[1] = self.fRec189[0];
			self.fRec236[1] = self.fRec236[0];
			self.fRec237[1] = self.fRec237[0];
			self.fRec239[1] = self.fRec239[0];
			self.fRec238[1] = self.fRec238[0];
			self.fRec244[1] = self.fRec244[0];
			self.fRec242[1] = self.fRec242[0];
			self.fRec245[1] = self.fRec245[0];
			self.fRec241[2] = self.fRec241[1];
			self.fRec241[1] = self.fRec241[0];
			self.fRec240[2] = self.fRec240[1];
			self.fRec240[1] = self.fRec240[0];
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
			self.fRec268[1] = self.fRec268[0];
			self.fRec269[1] = self.fRec269[0];
			self.fRec271[1] = self.fRec271[0];
			self.fVec21[1] = self.fVec21[0];
			self.fRec270[1] = self.fRec270[0];
			self.fRec273[1] = self.fRec273[0];
			self.fVec23[1] = self.fVec23[0];
			self.fRec272[1] = self.fRec272[0];
			self.fRec275[1] = self.fRec275[0];
			self.fVec25[1] = self.fVec25[0];
			self.fRec274[1] = self.fRec274[0];
			self.fRec277[1] = self.fRec277[0];
			self.fVec27[1] = self.fVec27[0];
			self.fRec276[1] = self.fRec276[0];
			self.fRec278[1] = self.fRec278[0];
			self.fRec279[1] = self.fRec279[0];
			self.fRec280[1] = self.fRec280[0];
			self.fRec282[1] = self.fRec282[0];
			self.fRec293[1] = self.fRec293[0];
			self.fRec295[1] = self.fRec295[0];
			self.fRec299[1] = self.fRec299[0];
			self.fVec30[1] = self.fVec30[0];
			self.fRec298[1] = self.fRec298[0];
			self.fRec296[1] = self.fRec296[0];
			self.fRec301[1] = self.fRec301[0];
			self.fVec32[1] = self.fVec32[0];
			self.fRec300[1] = self.fRec300[0];
			self.fRec297[1] = self.fRec297[0];
			self.fRec305[1] = self.fRec305[0];
			self.fVec34[1] = self.fVec34[0];
			self.fRec304[1] = self.fRec304[0];
			self.fRec302[1] = self.fRec302[0];
			self.fRec307[1] = self.fRec307[0];
			self.fVec36[1] = self.fVec36[0];
			self.fRec306[1] = self.fRec306[0];
			self.fRec303[1] = self.fRec303[0];
			self.fRec311[1] = self.fRec311[0];
			self.fVec38[1] = self.fVec38[0];
			self.fRec310[1] = self.fRec310[0];
			self.fRec308[1] = self.fRec308[0];
			self.fRec313[1] = self.fRec313[0];
			self.fVec40[1] = self.fVec40[0];
			self.fRec312[1] = self.fRec312[0];
			self.fRec309[1] = self.fRec309[0];
			self.fRec317[1] = self.fRec317[0];
			self.fVec42[1] = self.fVec42[0];
			self.fRec316[1] = self.fRec316[0];
			self.fRec314[1] = self.fRec314[0];
			self.fRec319[1] = self.fRec319[0];
			self.fVec44[1] = self.fVec44[0];
			self.fRec318[1] = self.fRec318[0];
			self.fRec315[1] = self.fRec315[0];
			self.fRec323[1] = self.fRec323[0];
			self.fVec46[1] = self.fVec46[0];
			self.fRec322[1] = self.fRec322[0];
			self.fRec320[1] = self.fRec320[0];
			self.fRec325[1] = self.fRec325[0];
			self.fVec48[1] = self.fVec48[0];
			self.fRec324[1] = self.fRec324[0];
			self.fRec321[1] = self.fRec321[0];
			self.fRec326[1] = self.fRec326[0];
			self.fRec327[1] = self.fRec327[0];
			self.fVec51[1] = self.fVec51[0];
			self.fRec294[1] = self.fRec294[0];
			self.fRec331[1] = self.fRec331[0];
			self.fRec333[1] = self.fRec333[0];
			self.fVec54[1] = self.fVec54[0];
			self.fRec332[1] = self.fRec332[0];
			self.fVec56[1] = self.fVec56[0];
			self.fRec330[1] = self.fRec330[0];
			self.fRec328[1] = self.fRec328[0];
			self.fRec335[1] = self.fRec335[0];
			self.fVec58[1] = self.fVec58[0];
			self.fRec334[1] = self.fRec334[0];
			self.fRec329[1] = self.fRec329[0];
			self.fRec339[1] = self.fRec339[0];
			self.fVec60[1] = self.fVec60[0];
			self.fRec338[1] = self.fRec338[0];
			self.fRec336[1] = self.fRec336[0];
			self.fRec341[1] = self.fRec341[0];
			self.fVec62[1] = self.fVec62[0];
			self.fRec340[1] = self.fRec340[0];
			self.fRec337[1] = self.fRec337[0];
			self.fRec345[1] = self.fRec345[0];
			self.fVec64[1] = self.fVec64[0];
			self.fRec344[1] = self.fRec344[0];
			self.fRec342[1] = self.fRec342[0];
			self.fRec347[1] = self.fRec347[0];
			self.fVec66[1] = self.fVec66[0];
			self.fRec346[1] = self.fRec346[0];
			self.fRec343[1] = self.fRec343[0];
			self.fRec351[1] = self.fRec351[0];
			self.fVec68[1] = self.fVec68[0];
			self.fRec350[1] = self.fRec350[0];
			self.fRec348[1] = self.fRec348[0];
			self.fRec353[1] = self.fRec353[0];
			self.fVec70[1] = self.fVec70[0];
			self.fRec352[1] = self.fRec352[0];
			self.fRec349[1] = self.fRec349[0];
			self.fRec357[1] = self.fRec357[0];
			self.fVec72[1] = self.fVec72[0];
			self.fRec356[1] = self.fRec356[0];
			self.fRec354[1] = self.fRec354[0];
			self.fRec359[1] = self.fRec359[0];
			self.fVec74[1] = self.fVec74[0];
			self.fRec358[1] = self.fRec358[0];
			self.fRec355[1] = self.fRec355[0];
			self.fVec77[1] = self.fVec77[0];
			self.fRec292[1] = self.fRec292[0];
			self.fRec291[1] = self.fRec291[0];
			self.fRec290[2] = self.fRec290[1];
			self.fRec290[1] = self.fRec290[0];
			self.fRec289[2] = self.fRec289[1];
			self.fRec289[1] = self.fRec289[0];
			self.fVec78[1] = self.fVec78[0];
			self.fRec288[1] = self.fRec288[0];
			self.fRec287[2] = self.fRec287[1];
			self.fRec287[1] = self.fRec287[0];
			self.fRec286[2] = self.fRec286[1];
			self.fRec286[1] = self.fRec286[0];
			self.fRec362[1] = self.fRec362[0];
			self.fRec361[2] = self.fRec361[1];
			self.fRec361[1] = self.fRec361[0];
			self.fRec360[2] = self.fRec360[1];
			self.fRec360[1] = self.fRec360[0];
			self.fRec366[1] = self.fRec366[0];
			self.fRec365[2] = self.fRec365[1];
			self.fRec365[1] = self.fRec365[0];
			self.fRec364[2] = self.fRec364[1];
			self.fRec364[1] = self.fRec364[0];
			self.fRec363[2] = self.fRec363[1];
			self.fRec363[1] = self.fRec363[0];
			self.fRec285[1] = self.fRec285[0];
			self.fRec378[1] = self.fRec378[0];
			self.fVec82[1] = self.fVec82[0];
			self.fRec377[1] = self.fRec377[0];
			self.fRec376[1] = self.fRec376[0];
			self.fRec375[2] = self.fRec375[1];
			self.fRec375[1] = self.fRec375[0];
			self.fRec374[2] = self.fRec374[1];
			self.fRec374[1] = self.fRec374[0];
			self.fVec83[1] = self.fVec83[0];
			self.fRec373[1] = self.fRec373[0];
			self.fRec372[2] = self.fRec372[1];
			self.fRec372[1] = self.fRec372[0];
			self.fRec371[2] = self.fRec371[1];
			self.fRec371[1] = self.fRec371[0];
			self.fRec381[1] = self.fRec381[0];
			self.fRec380[2] = self.fRec380[1];
			self.fRec380[1] = self.fRec380[0];
			self.fRec379[2] = self.fRec379[1];
			self.fRec379[1] = self.fRec379[0];
			self.fRec385[1] = self.fRec385[0];
			self.fRec384[2] = self.fRec384[1];
			self.fRec384[1] = self.fRec384[0];
			self.fRec383[2] = self.fRec383[1];
			self.fRec383[1] = self.fRec383[0];
			self.fRec382[2] = self.fRec382[1];
			self.fRec382[1] = self.fRec382[0];
			self.fRec370[1] = self.fRec370[0];
			self.fVec86[1] = self.fVec86[0];
			self.fRec369[1] = self.fRec369[0];
			self.fRec367[1] = self.fRec367[0];
			self.fRec387[1] = self.fRec387[0];
			self.fVec88[1] = self.fVec88[0];
			self.fRec386[1] = self.fRec386[0];
			self.fRec368[1] = self.fRec368[0];
			self.fVec90[1] = self.fVec90[0];
			self.fRec390[1] = self.fRec390[0];
			self.fRec388[1] = self.fRec388[0];
			self.fVec92[1] = self.fVec92[0];
			self.fRec391[1] = self.fRec391[0];
			self.fRec389[1] = self.fRec389[0];
			self.fVec94[1] = self.fVec94[0];
			self.fRec394[1] = self.fRec394[0];
			self.fRec392[1] = self.fRec392[0];
			self.fRec396[1] = self.fRec396[0];
			self.fVec96[1] = self.fVec96[0];
			self.fRec395[1] = self.fRec395[0];
			self.fRec393[1] = self.fRec393[0];
			self.fRec400[1] = self.fRec400[0];
			self.fVec98[1] = self.fVec98[0];
			self.fRec399[1] = self.fRec399[0];
			self.fRec397[1] = self.fRec397[0];
			self.fVec100[1] = self.fVec100[0];
			self.fRec401[1] = self.fRec401[0];
			self.fRec398[1] = self.fRec398[0];
			self.fRec283[1] = self.fRec283[0];
			self.fRec284[1] = self.fRec284[0];
			self.fRec402[1] = self.fRec402[0];
		}
	}

}


}
pub use dsp::mydsp as Instrument;
