mod dsp {
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
	iRec0: [i32;2],
}

impl mydspSIG0 {
	
	fn get_num_inputsmydspSIG0(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsmydspSIG0(&self) -> i32 {
		return 1;
	}
	
	fn instance_initmydspSIG0(&mut self, sample_rate: i32) {
		for l0 in 0..2 {
			self.iRec0[(l0) as usize] = 0;
		}
	}
	
	fn fillmydspSIG0(&mut self, count: i32, table: &mut[F32]) {
		for i1 in 0..count {
			self.iRec0[0] = i32::wrapping_add(self.iRec0[1], 1);
			table[(i1) as usize] = 4.4e+02 * F32::powf(2.0, 0.083333336 * (0.062042013 * ((i32::wrapping_add(self.iRec0[0], -1)) as F32) + -69.0));
			self.iRec0[1] = self.iRec0[0];
		}
	}

}


pub fn newmydspSIG0() -> mydspSIG0 { 
	mydspSIG0 {
		iRec0: [0;2],
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
	fHslider0: F32,
	iVec0: [i32;2],
	fSampleRate: i32,
	fConst0: F32,
	fConst1: F32,
	fConst2: F32,
	fHslider1: F32,
	fRec1: [F32;2],
	fConst3: F32,
	fRec3: [F32;2],
	fVec1: [F32;2],
	IOTA0: i32,
	fVec2: [F32;4096],
	fConst4: F32,
	fConst5: F32,
	fRec2: [F32;2],
	fRec5: [F32;2],
	fVec3: [F32;2],
	fVec4: [F32;4096],
	fRec4: [F32;2],
	fRec7: [F32;2],
	fVec5: [F32;2],
	fVec6: [F32;4096],
	fRec6: [F32;2],
	fHslider2: F32,
	fRec8: [F32;2],
	fHslider3: F32,
	fRec9: [F32;2],
	fRec11: [F32;2],
	fVec7: [F32;2],
	fVec8: [F32;4096],
	fRec10: [F32;2],
	fRec13: [F32;2],
	fVec9: [F32;2],
	fVec10: [F32;4096],
	fRec12: [F32;2],
	fRec15: [F32;2],
	fVec11: [F32;2],
	fVec12: [F32;4096],
	fRec14: [F32;2],
	fHslider4: F32,
	fRec16: [F32;2],
	fHslider5: F32,
	fRec17: [F32;2],
	fRec19: [F32;2],
	fVec13: [F32;2],
	fVec14: [F32;4096],
	fRec18: [F32;2],
	fRec21: [F32;2],
	fVec15: [F32;2],
	fVec16: [F32;4096],
	fRec20: [F32;2],
	fRec23: [F32;2],
	fVec17: [F32;2],
	fVec18: [F32;4096],
	fRec22: [F32;2],
	fHslider6: F32,
	fRec24: [F32;2],
	fHslider7: F32,
	fRec25: [F32;2],
	fRec27: [F32;2],
	fVec19: [F32;2],
	fVec20: [F32;4096],
	fRec26: [F32;2],
	fRec29: [F32;2],
	fVec21: [F32;2],
	fVec22: [F32;4096],
	fRec28: [F32;2],
	fRec31: [F32;2],
	fVec23: [F32;2],
	fVec24: [F32;4096],
	fRec30: [F32;2],
	fHslider8: F32,
	fRec32: [F32;2],
	fHslider9: F32,
	fRec33: [F32;2],
	fConst6: F32,
	fHslider10: F32,
	fRec34: [F32;2],
	fHslider11: F32,
	fRec36: [F32;2],
	fHslider12: F32,
	fRec37: [F32;2],
	fConst7: F32,
	fConst8: F32,
	fConst9: F32,
	fRec67: [F32;2],
	fRec71: [F32;2],
	fConst10: F32,
	fConst11: F32,
	fRec76: [F32;2],
	iVec25: [i32;2],
	iConst12: i32,
	iRec77: [i32;2],
	fConst13: F32,
	fRec74: [F32;2],
	fRec73: [F32;2],
	fRec78: [F32;4],
	fRec79: [F32;2048],
	fVec26: [F32;2],
	fConst14: F32,
	fConst15: F32,
	fButton0: F32,
	fVec27: [F32;2],
	iRec80: [i32;2],
	iRec82: [i32;2],
	fRec81: [F32;3],
	fVec28: [F32;3],
	fRec72: [F32;2048],
	fRec63: [F32;2],
	fRec59: [F32;2],
	fRec55: [F32;2048],
	fRec57: [F32;2],
	fHslider13: F32,
	fRec53: [F32;4],
	fRec48: [F32;2],
	fRec44: [F32;2048],
	fRec42: [F32;2],
	fConst16: F32,
	fRec41: [F32;2],
	fRec40: [F32;2],
	fRec38: [F32;2],
	fRec35: [F32;2],
	fHslider14: F32,
	fRec84: [F32;2],
	fRec114: [F32;2],
	fRec118: [F32;2],
	fRec123: [F32;2],
	iVec29: [i32;2],
	iRec124: [i32;2],
	fRec121: [F32;2],
	fRec120: [F32;2],
	fRec125: [F32;4],
	fRec126: [F32;2048],
	fVec30: [F32;2],
	fButton1: F32,
	fVec31: [F32;2],
	iRec127: [i32;2],
	fRec128: [F32;3],
	fVec32: [F32;3],
	fRec119: [F32;2048],
	fRec110: [F32;2],
	fRec106: [F32;2],
	fRec102: [F32;2048],
	fRec104: [F32;2],
	fRec100: [F32;4],
	fRec95: [F32;2],
	fRec91: [F32;2048],
	fRec89: [F32;2],
	fRec88: [F32;2],
	fRec87: [F32;2],
	fRec85: [F32;2],
	fRec83: [F32;2],
	fHslider15: F32,
	fRec130: [F32;2],
	fRec160: [F32;2],
	fRec164: [F32;2],
	fRec169: [F32;2],
	iVec33: [i32;2],
	iRec170: [i32;2],
	fRec167: [F32;2],
	fRec166: [F32;2],
	fRec171: [F32;4],
	fRec172: [F32;2048],
	fVec34: [F32;2],
	fButton2: F32,
	fVec35: [F32;2],
	iRec173: [i32;2],
	fRec174: [F32;3],
	fVec36: [F32;3],
	fRec165: [F32;2048],
	fRec156: [F32;2],
	fRec152: [F32;2],
	fRec148: [F32;2048],
	fRec150: [F32;2],
	fRec146: [F32;4],
	fRec141: [F32;2],
	fRec137: [F32;2048],
	fRec135: [F32;2],
	fRec134: [F32;2],
	fRec133: [F32;2],
	fRec131: [F32;2],
	fRec129: [F32;2],
	fHslider16: F32,
	fRec176: [F32;2],
	fRec206: [F32;2],
	fRec210: [F32;2],
	fRec215: [F32;2],
	iVec37: [i32;2],
	iRec216: [i32;2],
	fRec213: [F32;2],
	fRec212: [F32;2],
	fRec217: [F32;4],
	fRec218: [F32;2048],
	fVec38: [F32;2],
	fButton3: F32,
	fVec39: [F32;2],
	iRec219: [i32;2],
	fRec220: [F32;3],
	fVec40: [F32;3],
	fRec211: [F32;2048],
	fRec202: [F32;2],
	fRec198: [F32;2],
	fRec194: [F32;2048],
	fRec196: [F32;2],
	fRec192: [F32;4],
	fRec187: [F32;2],
	fRec183: [F32;2048],
	fRec181: [F32;2],
	fRec180: [F32;2],
	fRec179: [F32;2],
	fRec177: [F32;2],
	fRec175: [F32;2],
	fHslider17: F32,
	fRec221: [F32;2],
	fHslider18: F32,
	fRec223: [F32;2],
	fHslider19: F32,
	fRec222: [F32;2],
	fConst17: F32,
	fRec228: [F32;2],
	fRec226: [F32;2],
	fHslider20: F32,
	fRec229: [F32;2],
	fRec225: [F32;3],
	fRec224: [F32;3],
	fHslider21: F32,
	fRec230: [F32;2],
	fRec235: [F32;2],
	fRec233: [F32;2],
	fHslider22: F32,
	fRec236: [F32;2],
	fRec232: [F32;3],
	fRec231: [F32;3],
	fHslider23: F32,
	fRec237: [F32;2],
	fRec242: [F32;2],
	fRec240: [F32;2],
	fHslider24: F32,
	fRec243: [F32;2],
	fRec239: [F32;3],
	fRec238: [F32;3],
	fHslider25: F32,
	fRec244: [F32;2],
	fRec249: [F32;2],
	fRec247: [F32;2],
	fHslider26: F32,
	fRec250: [F32;2],
	fRec246: [F32;3],
	fRec245: [F32;3],
	fHslider27: F32,
	fRec251: [F32;2],
	fHslider28: F32,
	fRec252: [F32;2],
	fHslider29: F32,
	fRec253: [F32;2],
	fConst18: F32,
	fHslider30: F32,
	fRec255: [F32;2],
	fHslider31: F32,
	fRec254: [F32;2097152],
	fHslider32: F32,
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
	fConst37: F32,
	fHslider33: F32,
	fRec266: [F32;2],
	fRec268: [F32;2],
	fRec272: [F32;2],
	fVec41: [F32;16384],
	fVec42: [F32;2],
	fRec271: [F32;2],
	fRec269: [F32;2],
	fRec274: [F32;2],
	fVec43: [F32;16384],
	fVec44: [F32;2],
	fRec273: [F32;2],
	fRec270: [F32;2],
	fRec278: [F32;2],
	fVec45: [F32;16384],
	fVec46: [F32;2],
	fRec277: [F32;2],
	fRec275: [F32;2],
	fRec280: [F32;2],
	fVec47: [F32;16384],
	fVec48: [F32;2],
	fRec279: [F32;2],
	fRec276: [F32;2],
	fRec284: [F32;2],
	fVec49: [F32;16384],
	fVec50: [F32;2],
	fRec283: [F32;2],
	fRec281: [F32;2],
	fRec286: [F32;2],
	fVec51: [F32;16384],
	fVec52: [F32;2],
	fRec285: [F32;2],
	fRec282: [F32;2],
	fRec290: [F32;2],
	fVec53: [F32;16384],
	fVec54: [F32;2],
	fRec289: [F32;2],
	fRec287: [F32;2],
	fRec292: [F32;2],
	fVec55: [F32;16384],
	fVec56: [F32;2],
	fRec291: [F32;2],
	fRec288: [F32;2],
	fRec296: [F32;2],
	fVec57: [F32;16384],
	fVec58: [F32;2],
	fRec295: [F32;2],
	fRec293: [F32;2],
	fRec298: [F32;2],
	fVec59: [F32;16384],
	fVec60: [F32;2],
	fRec297: [F32;2],
	fRec294: [F32;2],
	fVec61: [F32;1024],
	fHslider34: F32,
	fRec299: [F32;2],
	fRec300: [F32;2],
	fHslider35: F32,
	fVec62: [F32;16384],
	fVec63: [F32;2],
	fRec267: [F32;2],
	fRec304: [F32;2],
	fRec306: [F32;2],
	fVec64: [F32;1024],
	fVec65: [F32;16384],
	fVec66: [F32;2],
	fRec305: [F32;2],
	fVec67: [F32;16384],
	fVec68: [F32;2],
	fRec303: [F32;2],
	fRec301: [F32;2],
	fRec308: [F32;2],
	fVec69: [F32;16384],
	fVec70: [F32;2],
	fRec307: [F32;2],
	fRec302: [F32;2],
	fRec312: [F32;2],
	fVec71: [F32;16384],
	fVec72: [F32;2],
	fRec311: [F32;2],
	fRec309: [F32;2],
	fRec314: [F32;2],
	fVec73: [F32;16384],
	fVec74: [F32;2],
	fRec313: [F32;2],
	fRec310: [F32;2],
	fRec318: [F32;2],
	fVec75: [F32;16384],
	fVec76: [F32;2],
	fRec317: [F32;2],
	fRec315: [F32;2],
	fRec320: [F32;2],
	fVec77: [F32;16384],
	fVec78: [F32;2],
	fRec319: [F32;2],
	fRec316: [F32;2],
	fRec324: [F32;2],
	fVec79: [F32;16384],
	fVec80: [F32;2],
	fRec323: [F32;2],
	fRec321: [F32;2],
	fRec326: [F32;2],
	fVec81: [F32;16384],
	fVec82: [F32;2],
	fRec325: [F32;2],
	fRec322: [F32;2],
	fRec330: [F32;2],
	fVec83: [F32;16384],
	fVec84: [F32;2],
	fRec329: [F32;2],
	fRec327: [F32;2],
	fRec332: [F32;2],
	fVec85: [F32;16384],
	fVec86: [F32;2],
	fRec331: [F32;2],
	fRec328: [F32;2],
	fVec87: [F32;16384],
	fVec88: [F32;16384],
	fVec89: [F32;2],
	fRec265: [F32;2],
	fConst39: F32,
	fRec264: [F32;2],
	fRec263: [F32;3],
	fRec262: [F32;3],
	fVec90: [F32;2],
	fConst40: F32,
	fConst42: F32,
	fRec261: [F32;2],
	fRec260: [F32;3],
	fRec259: [F32;3],
	fConst43: F32,
	fConst44: F32,
	fConst45: F32,
	fRec335: [F32;2],
	fRec334: [F32;3],
	fConst46: F32,
	fRec333: [F32;3],
	fConst47: F32,
	fConst48: F32,
	fConst49: F32,
	fRec339: [F32;2],
	fRec338: [F32;3],
	fConst50: F32,
	fRec337: [F32;3],
	fRec336: [F32;3],
	fHslider36: F32,
	fVec91: [F32;1024],
	fHslider37: F32,
	fRec258: [F32;2],
	fHslider38: F32,
	fRec351: [F32;2],
	fVec92: [F32;16384],
	fVec93: [F32;16384],
	fVec94: [F32;2],
	fRec350: [F32;2],
	fRec349: [F32;2],
	fRec348: [F32;3],
	fRec347: [F32;3],
	fVec95: [F32;2],
	fRec346: [F32;2],
	fRec345: [F32;3],
	fRec344: [F32;3],
	fRec354: [F32;2],
	fRec353: [F32;3],
	fRec352: [F32;3],
	fRec358: [F32;2],
	fRec357: [F32;3],
	fRec356: [F32;3],
	fRec355: [F32;3],
	fVec96: [F32;1024],
	fRec343: [F32;2],
	fVec97: [F32;16384],
	fVec98: [F32;2],
	fRec342: [F32;2],
	fRec340: [F32;2],
	fRec360: [F32;2],
	fVec99: [F32;16384],
	fVec100: [F32;2],
	fRec359: [F32;2],
	fRec341: [F32;2],
	fVec101: [F32;16384],
	fVec102: [F32;2],
	fRec363: [F32;2],
	fRec361: [F32;2],
	fVec103: [F32;16384],
	fVec104: [F32;2],
	fRec364: [F32;2],
	fRec362: [F32;2],
	fVec105: [F32;16384],
	fVec106: [F32;2],
	fRec367: [F32;2],
	fRec365: [F32;2],
	fRec369: [F32;2],
	fVec107: [F32;16384],
	fVec108: [F32;2],
	fRec368: [F32;2],
	fRec366: [F32;2],
	fRec373: [F32;2],
	fVec109: [F32;16384],
	fVec110: [F32;2],
	fRec372: [F32;2],
	fRec370: [F32;2],
	fVec111: [F32;16384],
	fVec112: [F32;2],
	fRec374: [F32;2],
	fRec371: [F32;2],
	fRec256: [F32;2],
	fRec257: [F32;2],
	fHslider39: F32,
	fRec375: [F32;2],
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
			fRec1: [0.0;2],
			fConst3: 0.0,
			fRec3: [0.0;2],
			fVec1: [0.0;2],
			IOTA0: 0,
			fVec2: [0.0;4096],
			fConst4: 0.0,
			fConst5: 0.0,
			fRec2: [0.0;2],
			fRec5: [0.0;2],
			fVec3: [0.0;2],
			fVec4: [0.0;4096],
			fRec4: [0.0;2],
			fRec7: [0.0;2],
			fVec5: [0.0;2],
			fVec6: [0.0;4096],
			fRec6: [0.0;2],
			fHslider2: 0.0,
			fRec8: [0.0;2],
			fHslider3: 0.0,
			fRec9: [0.0;2],
			fRec11: [0.0;2],
			fVec7: [0.0;2],
			fVec8: [0.0;4096],
			fRec10: [0.0;2],
			fRec13: [0.0;2],
			fVec9: [0.0;2],
			fVec10: [0.0;4096],
			fRec12: [0.0;2],
			fRec15: [0.0;2],
			fVec11: [0.0;2],
			fVec12: [0.0;4096],
			fRec14: [0.0;2],
			fHslider4: 0.0,
			fRec16: [0.0;2],
			fHslider5: 0.0,
			fRec17: [0.0;2],
			fRec19: [0.0;2],
			fVec13: [0.0;2],
			fVec14: [0.0;4096],
			fRec18: [0.0;2],
			fRec21: [0.0;2],
			fVec15: [0.0;2],
			fVec16: [0.0;4096],
			fRec20: [0.0;2],
			fRec23: [0.0;2],
			fVec17: [0.0;2],
			fVec18: [0.0;4096],
			fRec22: [0.0;2],
			fHslider6: 0.0,
			fRec24: [0.0;2],
			fHslider7: 0.0,
			fRec25: [0.0;2],
			fRec27: [0.0;2],
			fVec19: [0.0;2],
			fVec20: [0.0;4096],
			fRec26: [0.0;2],
			fRec29: [0.0;2],
			fVec21: [0.0;2],
			fVec22: [0.0;4096],
			fRec28: [0.0;2],
			fRec31: [0.0;2],
			fVec23: [0.0;2],
			fVec24: [0.0;4096],
			fRec30: [0.0;2],
			fHslider8: 0.0,
			fRec32: [0.0;2],
			fHslider9: 0.0,
			fRec33: [0.0;2],
			fConst6: 0.0,
			fHslider10: 0.0,
			fRec34: [0.0;2],
			fHslider11: 0.0,
			fRec36: [0.0;2],
			fHslider12: 0.0,
			fRec37: [0.0;2],
			fConst7: 0.0,
			fConst8: 0.0,
			fConst9: 0.0,
			fRec67: [0.0;2],
			fRec71: [0.0;2],
			fConst10: 0.0,
			fConst11: 0.0,
			fRec76: [0.0;2],
			iVec25: [0;2],
			iConst12: 0,
			iRec77: [0;2],
			fConst13: 0.0,
			fRec74: [0.0;2],
			fRec73: [0.0;2],
			fRec78: [0.0;4],
			fRec79: [0.0;2048],
			fVec26: [0.0;2],
			fConst14: 0.0,
			fConst15: 0.0,
			fButton0: 0.0,
			fVec27: [0.0;2],
			iRec80: [0;2],
			iRec82: [0;2],
			fRec81: [0.0;3],
			fVec28: [0.0;3],
			fRec72: [0.0;2048],
			fRec63: [0.0;2],
			fRec59: [0.0;2],
			fRec55: [0.0;2048],
			fRec57: [0.0;2],
			fHslider13: 0.0,
			fRec53: [0.0;4],
			fRec48: [0.0;2],
			fRec44: [0.0;2048],
			fRec42: [0.0;2],
			fConst16: 0.0,
			fRec41: [0.0;2],
			fRec40: [0.0;2],
			fRec38: [0.0;2],
			fRec35: [0.0;2],
			fHslider14: 0.0,
			fRec84: [0.0;2],
			fRec114: [0.0;2],
			fRec118: [0.0;2],
			fRec123: [0.0;2],
			iVec29: [0;2],
			iRec124: [0;2],
			fRec121: [0.0;2],
			fRec120: [0.0;2],
			fRec125: [0.0;4],
			fRec126: [0.0;2048],
			fVec30: [0.0;2],
			fButton1: 0.0,
			fVec31: [0.0;2],
			iRec127: [0;2],
			fRec128: [0.0;3],
			fVec32: [0.0;3],
			fRec119: [0.0;2048],
			fRec110: [0.0;2],
			fRec106: [0.0;2],
			fRec102: [0.0;2048],
			fRec104: [0.0;2],
			fRec100: [0.0;4],
			fRec95: [0.0;2],
			fRec91: [0.0;2048],
			fRec89: [0.0;2],
			fRec88: [0.0;2],
			fRec87: [0.0;2],
			fRec85: [0.0;2],
			fRec83: [0.0;2],
			fHslider15: 0.0,
			fRec130: [0.0;2],
			fRec160: [0.0;2],
			fRec164: [0.0;2],
			fRec169: [0.0;2],
			iVec33: [0;2],
			iRec170: [0;2],
			fRec167: [0.0;2],
			fRec166: [0.0;2],
			fRec171: [0.0;4],
			fRec172: [0.0;2048],
			fVec34: [0.0;2],
			fButton2: 0.0,
			fVec35: [0.0;2],
			iRec173: [0;2],
			fRec174: [0.0;3],
			fVec36: [0.0;3],
			fRec165: [0.0;2048],
			fRec156: [0.0;2],
			fRec152: [0.0;2],
			fRec148: [0.0;2048],
			fRec150: [0.0;2],
			fRec146: [0.0;4],
			fRec141: [0.0;2],
			fRec137: [0.0;2048],
			fRec135: [0.0;2],
			fRec134: [0.0;2],
			fRec133: [0.0;2],
			fRec131: [0.0;2],
			fRec129: [0.0;2],
			fHslider16: 0.0,
			fRec176: [0.0;2],
			fRec206: [0.0;2],
			fRec210: [0.0;2],
			fRec215: [0.0;2],
			iVec37: [0;2],
			iRec216: [0;2],
			fRec213: [0.0;2],
			fRec212: [0.0;2],
			fRec217: [0.0;4],
			fRec218: [0.0;2048],
			fVec38: [0.0;2],
			fButton3: 0.0,
			fVec39: [0.0;2],
			iRec219: [0;2],
			fRec220: [0.0;3],
			fVec40: [0.0;3],
			fRec211: [0.0;2048],
			fRec202: [0.0;2],
			fRec198: [0.0;2],
			fRec194: [0.0;2048],
			fRec196: [0.0;2],
			fRec192: [0.0;4],
			fRec187: [0.0;2],
			fRec183: [0.0;2048],
			fRec181: [0.0;2],
			fRec180: [0.0;2],
			fRec179: [0.0;2],
			fRec177: [0.0;2],
			fRec175: [0.0;2],
			fHslider17: 0.0,
			fRec221: [0.0;2],
			fHslider18: 0.0,
			fRec223: [0.0;2],
			fHslider19: 0.0,
			fRec222: [0.0;2],
			fConst17: 0.0,
			fRec228: [0.0;2],
			fRec226: [0.0;2],
			fHslider20: 0.0,
			fRec229: [0.0;2],
			fRec225: [0.0;3],
			fRec224: [0.0;3],
			fHslider21: 0.0,
			fRec230: [0.0;2],
			fRec235: [0.0;2],
			fRec233: [0.0;2],
			fHslider22: 0.0,
			fRec236: [0.0;2],
			fRec232: [0.0;3],
			fRec231: [0.0;3],
			fHslider23: 0.0,
			fRec237: [0.0;2],
			fRec242: [0.0;2],
			fRec240: [0.0;2],
			fHslider24: 0.0,
			fRec243: [0.0;2],
			fRec239: [0.0;3],
			fRec238: [0.0;3],
			fHslider25: 0.0,
			fRec244: [0.0;2],
			fRec249: [0.0;2],
			fRec247: [0.0;2],
			fHslider26: 0.0,
			fRec250: [0.0;2],
			fRec246: [0.0;3],
			fRec245: [0.0;3],
			fHslider27: 0.0,
			fRec251: [0.0;2],
			fHslider28: 0.0,
			fRec252: [0.0;2],
			fHslider29: 0.0,
			fRec253: [0.0;2],
			fConst18: 0.0,
			fHslider30: 0.0,
			fRec255: [0.0;2],
			fHslider31: 0.0,
			fRec254: [0.0;2097152],
			fHslider32: 0.0,
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
			fConst37: 0.0,
			fHslider33: 0.0,
			fRec266: [0.0;2],
			fRec268: [0.0;2],
			fRec272: [0.0;2],
			fVec41: [0.0;16384],
			fVec42: [0.0;2],
			fRec271: [0.0;2],
			fRec269: [0.0;2],
			fRec274: [0.0;2],
			fVec43: [0.0;16384],
			fVec44: [0.0;2],
			fRec273: [0.0;2],
			fRec270: [0.0;2],
			fRec278: [0.0;2],
			fVec45: [0.0;16384],
			fVec46: [0.0;2],
			fRec277: [0.0;2],
			fRec275: [0.0;2],
			fRec280: [0.0;2],
			fVec47: [0.0;16384],
			fVec48: [0.0;2],
			fRec279: [0.0;2],
			fRec276: [0.0;2],
			fRec284: [0.0;2],
			fVec49: [0.0;16384],
			fVec50: [0.0;2],
			fRec283: [0.0;2],
			fRec281: [0.0;2],
			fRec286: [0.0;2],
			fVec51: [0.0;16384],
			fVec52: [0.0;2],
			fRec285: [0.0;2],
			fRec282: [0.0;2],
			fRec290: [0.0;2],
			fVec53: [0.0;16384],
			fVec54: [0.0;2],
			fRec289: [0.0;2],
			fRec287: [0.0;2],
			fRec292: [0.0;2],
			fVec55: [0.0;16384],
			fVec56: [0.0;2],
			fRec291: [0.0;2],
			fRec288: [0.0;2],
			fRec296: [0.0;2],
			fVec57: [0.0;16384],
			fVec58: [0.0;2],
			fRec295: [0.0;2],
			fRec293: [0.0;2],
			fRec298: [0.0;2],
			fVec59: [0.0;16384],
			fVec60: [0.0;2],
			fRec297: [0.0;2],
			fRec294: [0.0;2],
			fVec61: [0.0;1024],
			fHslider34: 0.0,
			fRec299: [0.0;2],
			fRec300: [0.0;2],
			fHslider35: 0.0,
			fVec62: [0.0;16384],
			fVec63: [0.0;2],
			fRec267: [0.0;2],
			fRec304: [0.0;2],
			fRec306: [0.0;2],
			fVec64: [0.0;1024],
			fVec65: [0.0;16384],
			fVec66: [0.0;2],
			fRec305: [0.0;2],
			fVec67: [0.0;16384],
			fVec68: [0.0;2],
			fRec303: [0.0;2],
			fRec301: [0.0;2],
			fRec308: [0.0;2],
			fVec69: [0.0;16384],
			fVec70: [0.0;2],
			fRec307: [0.0;2],
			fRec302: [0.0;2],
			fRec312: [0.0;2],
			fVec71: [0.0;16384],
			fVec72: [0.0;2],
			fRec311: [0.0;2],
			fRec309: [0.0;2],
			fRec314: [0.0;2],
			fVec73: [0.0;16384],
			fVec74: [0.0;2],
			fRec313: [0.0;2],
			fRec310: [0.0;2],
			fRec318: [0.0;2],
			fVec75: [0.0;16384],
			fVec76: [0.0;2],
			fRec317: [0.0;2],
			fRec315: [0.0;2],
			fRec320: [0.0;2],
			fVec77: [0.0;16384],
			fVec78: [0.0;2],
			fRec319: [0.0;2],
			fRec316: [0.0;2],
			fRec324: [0.0;2],
			fVec79: [0.0;16384],
			fVec80: [0.0;2],
			fRec323: [0.0;2],
			fRec321: [0.0;2],
			fRec326: [0.0;2],
			fVec81: [0.0;16384],
			fVec82: [0.0;2],
			fRec325: [0.0;2],
			fRec322: [0.0;2],
			fRec330: [0.0;2],
			fVec83: [0.0;16384],
			fVec84: [0.0;2],
			fRec329: [0.0;2],
			fRec327: [0.0;2],
			fRec332: [0.0;2],
			fVec85: [0.0;16384],
			fVec86: [0.0;2],
			fRec331: [0.0;2],
			fRec328: [0.0;2],
			fVec87: [0.0;16384],
			fVec88: [0.0;16384],
			fVec89: [0.0;2],
			fRec265: [0.0;2],
			fConst39: 0.0,
			fRec264: [0.0;2],
			fRec263: [0.0;3],
			fRec262: [0.0;3],
			fVec90: [0.0;2],
			fConst40: 0.0,
			fConst42: 0.0,
			fRec261: [0.0;2],
			fRec260: [0.0;3],
			fRec259: [0.0;3],
			fConst43: 0.0,
			fConst44: 0.0,
			fConst45: 0.0,
			fRec335: [0.0;2],
			fRec334: [0.0;3],
			fConst46: 0.0,
			fRec333: [0.0;3],
			fConst47: 0.0,
			fConst48: 0.0,
			fConst49: 0.0,
			fRec339: [0.0;2],
			fRec338: [0.0;3],
			fConst50: 0.0,
			fRec337: [0.0;3],
			fRec336: [0.0;3],
			fHslider36: 0.0,
			fVec91: [0.0;1024],
			fHslider37: 0.0,
			fRec258: [0.0;2],
			fHslider38: 0.0,
			fRec351: [0.0;2],
			fVec92: [0.0;16384],
			fVec93: [0.0;16384],
			fVec94: [0.0;2],
			fRec350: [0.0;2],
			fRec349: [0.0;2],
			fRec348: [0.0;3],
			fRec347: [0.0;3],
			fVec95: [0.0;2],
			fRec346: [0.0;2],
			fRec345: [0.0;3],
			fRec344: [0.0;3],
			fRec354: [0.0;2],
			fRec353: [0.0;3],
			fRec352: [0.0;3],
			fRec358: [0.0;2],
			fRec357: [0.0;3],
			fRec356: [0.0;3],
			fRec355: [0.0;3],
			fVec96: [0.0;1024],
			fRec343: [0.0;2],
			fVec97: [0.0;16384],
			fVec98: [0.0;2],
			fRec342: [0.0;2],
			fRec340: [0.0;2],
			fRec360: [0.0;2],
			fVec99: [0.0;16384],
			fVec100: [0.0;2],
			fRec359: [0.0;2],
			fRec341: [0.0;2],
			fVec101: [0.0;16384],
			fVec102: [0.0;2],
			fRec363: [0.0;2],
			fRec361: [0.0;2],
			fVec103: [0.0;16384],
			fVec104: [0.0;2],
			fRec364: [0.0;2],
			fRec362: [0.0;2],
			fVec105: [0.0;16384],
			fVec106: [0.0;2],
			fRec367: [0.0;2],
			fRec365: [0.0;2],
			fRec369: [0.0;2],
			fVec107: [0.0;16384],
			fVec108: [0.0;2],
			fRec368: [0.0;2],
			fRec366: [0.0;2],
			fRec373: [0.0;2],
			fVec109: [0.0;16384],
			fVec110: [0.0;2],
			fRec372: [0.0;2],
			fRec370: [0.0;2],
			fVec111: [0.0;16384],
			fVec112: [0.0;2],
			fRec374: [0.0;2],
			fRec371: [0.0;2],
			fRec256: [0.0;2],
			fRec257: [0.0;2],
			fHslider39: 0.0,
			fRec375: [0.0;2],
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
		for l1 in 0..2 {
			self.iVec0[(l1) as usize] = 0;
		}
		for l2 in 0..2 {
			self.fRec1[(l2) as usize] = 0.0;
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
			self.fRec5[(l7) as usize] = 0.0;
		}
		for l8 in 0..2 {
			self.fVec3[(l8) as usize] = 0.0;
		}
		for l9 in 0..4096 {
			self.fVec4[(l9) as usize] = 0.0;
		}
		for l10 in 0..2 {
			self.fRec4[(l10) as usize] = 0.0;
		}
		for l11 in 0..2 {
			self.fRec7[(l11) as usize] = 0.0;
		}
		for l12 in 0..2 {
			self.fVec5[(l12) as usize] = 0.0;
		}
		for l13 in 0..4096 {
			self.fVec6[(l13) as usize] = 0.0;
		}
		for l14 in 0..2 {
			self.fRec6[(l14) as usize] = 0.0;
		}
		for l15 in 0..2 {
			self.fRec8[(l15) as usize] = 0.0;
		}
		for l16 in 0..2 {
			self.fRec9[(l16) as usize] = 0.0;
		}
		for l17 in 0..2 {
			self.fRec11[(l17) as usize] = 0.0;
		}
		for l18 in 0..2 {
			self.fVec7[(l18) as usize] = 0.0;
		}
		for l19 in 0..4096 {
			self.fVec8[(l19) as usize] = 0.0;
		}
		for l20 in 0..2 {
			self.fRec10[(l20) as usize] = 0.0;
		}
		for l21 in 0..2 {
			self.fRec13[(l21) as usize] = 0.0;
		}
		for l22 in 0..2 {
			self.fVec9[(l22) as usize] = 0.0;
		}
		for l23 in 0..4096 {
			self.fVec10[(l23) as usize] = 0.0;
		}
		for l24 in 0..2 {
			self.fRec12[(l24) as usize] = 0.0;
		}
		for l25 in 0..2 {
			self.fRec15[(l25) as usize] = 0.0;
		}
		for l26 in 0..2 {
			self.fVec11[(l26) as usize] = 0.0;
		}
		for l27 in 0..4096 {
			self.fVec12[(l27) as usize] = 0.0;
		}
		for l28 in 0..2 {
			self.fRec14[(l28) as usize] = 0.0;
		}
		for l29 in 0..2 {
			self.fRec16[(l29) as usize] = 0.0;
		}
		for l30 in 0..2 {
			self.fRec17[(l30) as usize] = 0.0;
		}
		for l31 in 0..2 {
			self.fRec19[(l31) as usize] = 0.0;
		}
		for l32 in 0..2 {
			self.fVec13[(l32) as usize] = 0.0;
		}
		for l33 in 0..4096 {
			self.fVec14[(l33) as usize] = 0.0;
		}
		for l34 in 0..2 {
			self.fRec18[(l34) as usize] = 0.0;
		}
		for l35 in 0..2 {
			self.fRec21[(l35) as usize] = 0.0;
		}
		for l36 in 0..2 {
			self.fVec15[(l36) as usize] = 0.0;
		}
		for l37 in 0..4096 {
			self.fVec16[(l37) as usize] = 0.0;
		}
		for l38 in 0..2 {
			self.fRec20[(l38) as usize] = 0.0;
		}
		for l39 in 0..2 {
			self.fRec23[(l39) as usize] = 0.0;
		}
		for l40 in 0..2 {
			self.fVec17[(l40) as usize] = 0.0;
		}
		for l41 in 0..4096 {
			self.fVec18[(l41) as usize] = 0.0;
		}
		for l42 in 0..2 {
			self.fRec22[(l42) as usize] = 0.0;
		}
		for l43 in 0..2 {
			self.fRec24[(l43) as usize] = 0.0;
		}
		for l44 in 0..2 {
			self.fRec25[(l44) as usize] = 0.0;
		}
		for l45 in 0..2 {
			self.fRec27[(l45) as usize] = 0.0;
		}
		for l46 in 0..2 {
			self.fVec19[(l46) as usize] = 0.0;
		}
		for l47 in 0..4096 {
			self.fVec20[(l47) as usize] = 0.0;
		}
		for l48 in 0..2 {
			self.fRec26[(l48) as usize] = 0.0;
		}
		for l49 in 0..2 {
			self.fRec29[(l49) as usize] = 0.0;
		}
		for l50 in 0..2 {
			self.fVec21[(l50) as usize] = 0.0;
		}
		for l51 in 0..4096 {
			self.fVec22[(l51) as usize] = 0.0;
		}
		for l52 in 0..2 {
			self.fRec28[(l52) as usize] = 0.0;
		}
		for l53 in 0..2 {
			self.fRec31[(l53) as usize] = 0.0;
		}
		for l54 in 0..2 {
			self.fVec23[(l54) as usize] = 0.0;
		}
		for l55 in 0..4096 {
			self.fVec24[(l55) as usize] = 0.0;
		}
		for l56 in 0..2 {
			self.fRec30[(l56) as usize] = 0.0;
		}
		for l57 in 0..2 {
			self.fRec32[(l57) as usize] = 0.0;
		}
		for l58 in 0..2 {
			self.fRec33[(l58) as usize] = 0.0;
		}
		for l59 in 0..2 {
			self.fRec34[(l59) as usize] = 0.0;
		}
		for l60 in 0..2 {
			self.fRec36[(l60) as usize] = 0.0;
		}
		for l61 in 0..2 {
			self.fRec37[(l61) as usize] = 0.0;
		}
		for l62 in 0..2 {
			self.fRec67[(l62) as usize] = 0.0;
		}
		for l63 in 0..2 {
			self.fRec71[(l63) as usize] = 0.0;
		}
		for l64 in 0..2 {
			self.fRec76[(l64) as usize] = 0.0;
		}
		for l65 in 0..2 {
			self.iVec25[(l65) as usize] = 0;
		}
		for l66 in 0..2 {
			self.iRec77[(l66) as usize] = 0;
		}
		for l67 in 0..2 {
			self.fRec74[(l67) as usize] = 0.0;
		}
		for l68 in 0..2 {
			self.fRec73[(l68) as usize] = 0.0;
		}
		for l69 in 0..4 {
			self.fRec78[(l69) as usize] = 0.0;
		}
		for l70 in 0..2048 {
			self.fRec79[(l70) as usize] = 0.0;
		}
		for l71 in 0..2 {
			self.fVec26[(l71) as usize] = 0.0;
		}
		for l72 in 0..2 {
			self.fVec27[(l72) as usize] = 0.0;
		}
		for l73 in 0..2 {
			self.iRec80[(l73) as usize] = 0;
		}
		for l74 in 0..2 {
			self.iRec82[(l74) as usize] = 0;
		}
		for l75 in 0..3 {
			self.fRec81[(l75) as usize] = 0.0;
		}
		for l76 in 0..3 {
			self.fVec28[(l76) as usize] = 0.0;
		}
		for l77 in 0..2048 {
			self.fRec72[(l77) as usize] = 0.0;
		}
		for l78 in 0..2 {
			self.fRec63[(l78) as usize] = 0.0;
		}
		for l79 in 0..2 {
			self.fRec59[(l79) as usize] = 0.0;
		}
		for l80 in 0..2048 {
			self.fRec55[(l80) as usize] = 0.0;
		}
		for l81 in 0..2 {
			self.fRec57[(l81) as usize] = 0.0;
		}
		for l82 in 0..4 {
			self.fRec53[(l82) as usize] = 0.0;
		}
		for l83 in 0..2 {
			self.fRec48[(l83) as usize] = 0.0;
		}
		for l84 in 0..2048 {
			self.fRec44[(l84) as usize] = 0.0;
		}
		for l85 in 0..2 {
			self.fRec42[(l85) as usize] = 0.0;
		}
		for l86 in 0..2 {
			self.fRec41[(l86) as usize] = 0.0;
		}
		for l87 in 0..2 {
			self.fRec40[(l87) as usize] = 0.0;
		}
		for l88 in 0..2 {
			self.fRec38[(l88) as usize] = 0.0;
		}
		for l89 in 0..2 {
			self.fRec35[(l89) as usize] = 0.0;
		}
		for l90 in 0..2 {
			self.fRec84[(l90) as usize] = 0.0;
		}
		for l91 in 0..2 {
			self.fRec114[(l91) as usize] = 0.0;
		}
		for l92 in 0..2 {
			self.fRec118[(l92) as usize] = 0.0;
		}
		for l93 in 0..2 {
			self.fRec123[(l93) as usize] = 0.0;
		}
		for l94 in 0..2 {
			self.iVec29[(l94) as usize] = 0;
		}
		for l95 in 0..2 {
			self.iRec124[(l95) as usize] = 0;
		}
		for l96 in 0..2 {
			self.fRec121[(l96) as usize] = 0.0;
		}
		for l97 in 0..2 {
			self.fRec120[(l97) as usize] = 0.0;
		}
		for l98 in 0..4 {
			self.fRec125[(l98) as usize] = 0.0;
		}
		for l99 in 0..2048 {
			self.fRec126[(l99) as usize] = 0.0;
		}
		for l100 in 0..2 {
			self.fVec30[(l100) as usize] = 0.0;
		}
		for l101 in 0..2 {
			self.fVec31[(l101) as usize] = 0.0;
		}
		for l102 in 0..2 {
			self.iRec127[(l102) as usize] = 0;
		}
		for l103 in 0..3 {
			self.fRec128[(l103) as usize] = 0.0;
		}
		for l104 in 0..3 {
			self.fVec32[(l104) as usize] = 0.0;
		}
		for l105 in 0..2048 {
			self.fRec119[(l105) as usize] = 0.0;
		}
		for l106 in 0..2 {
			self.fRec110[(l106) as usize] = 0.0;
		}
		for l107 in 0..2 {
			self.fRec106[(l107) as usize] = 0.0;
		}
		for l108 in 0..2048 {
			self.fRec102[(l108) as usize] = 0.0;
		}
		for l109 in 0..2 {
			self.fRec104[(l109) as usize] = 0.0;
		}
		for l110 in 0..4 {
			self.fRec100[(l110) as usize] = 0.0;
		}
		for l111 in 0..2 {
			self.fRec95[(l111) as usize] = 0.0;
		}
		for l112 in 0..2048 {
			self.fRec91[(l112) as usize] = 0.0;
		}
		for l113 in 0..2 {
			self.fRec89[(l113) as usize] = 0.0;
		}
		for l114 in 0..2 {
			self.fRec88[(l114) as usize] = 0.0;
		}
		for l115 in 0..2 {
			self.fRec87[(l115) as usize] = 0.0;
		}
		for l116 in 0..2 {
			self.fRec85[(l116) as usize] = 0.0;
		}
		for l117 in 0..2 {
			self.fRec83[(l117) as usize] = 0.0;
		}
		for l118 in 0..2 {
			self.fRec130[(l118) as usize] = 0.0;
		}
		for l119 in 0..2 {
			self.fRec160[(l119) as usize] = 0.0;
		}
		for l120 in 0..2 {
			self.fRec164[(l120) as usize] = 0.0;
		}
		for l121 in 0..2 {
			self.fRec169[(l121) as usize] = 0.0;
		}
		for l122 in 0..2 {
			self.iVec33[(l122) as usize] = 0;
		}
		for l123 in 0..2 {
			self.iRec170[(l123) as usize] = 0;
		}
		for l124 in 0..2 {
			self.fRec167[(l124) as usize] = 0.0;
		}
		for l125 in 0..2 {
			self.fRec166[(l125) as usize] = 0.0;
		}
		for l126 in 0..4 {
			self.fRec171[(l126) as usize] = 0.0;
		}
		for l127 in 0..2048 {
			self.fRec172[(l127) as usize] = 0.0;
		}
		for l128 in 0..2 {
			self.fVec34[(l128) as usize] = 0.0;
		}
		for l129 in 0..2 {
			self.fVec35[(l129) as usize] = 0.0;
		}
		for l130 in 0..2 {
			self.iRec173[(l130) as usize] = 0;
		}
		for l131 in 0..3 {
			self.fRec174[(l131) as usize] = 0.0;
		}
		for l132 in 0..3 {
			self.fVec36[(l132) as usize] = 0.0;
		}
		for l133 in 0..2048 {
			self.fRec165[(l133) as usize] = 0.0;
		}
		for l134 in 0..2 {
			self.fRec156[(l134) as usize] = 0.0;
		}
		for l135 in 0..2 {
			self.fRec152[(l135) as usize] = 0.0;
		}
		for l136 in 0..2048 {
			self.fRec148[(l136) as usize] = 0.0;
		}
		for l137 in 0..2 {
			self.fRec150[(l137) as usize] = 0.0;
		}
		for l138 in 0..4 {
			self.fRec146[(l138) as usize] = 0.0;
		}
		for l139 in 0..2 {
			self.fRec141[(l139) as usize] = 0.0;
		}
		for l140 in 0..2048 {
			self.fRec137[(l140) as usize] = 0.0;
		}
		for l141 in 0..2 {
			self.fRec135[(l141) as usize] = 0.0;
		}
		for l142 in 0..2 {
			self.fRec134[(l142) as usize] = 0.0;
		}
		for l143 in 0..2 {
			self.fRec133[(l143) as usize] = 0.0;
		}
		for l144 in 0..2 {
			self.fRec131[(l144) as usize] = 0.0;
		}
		for l145 in 0..2 {
			self.fRec129[(l145) as usize] = 0.0;
		}
		for l146 in 0..2 {
			self.fRec176[(l146) as usize] = 0.0;
		}
		for l147 in 0..2 {
			self.fRec206[(l147) as usize] = 0.0;
		}
		for l148 in 0..2 {
			self.fRec210[(l148) as usize] = 0.0;
		}
		for l149 in 0..2 {
			self.fRec215[(l149) as usize] = 0.0;
		}
		for l150 in 0..2 {
			self.iVec37[(l150) as usize] = 0;
		}
		for l151 in 0..2 {
			self.iRec216[(l151) as usize] = 0;
		}
		for l152 in 0..2 {
			self.fRec213[(l152) as usize] = 0.0;
		}
		for l153 in 0..2 {
			self.fRec212[(l153) as usize] = 0.0;
		}
		for l154 in 0..4 {
			self.fRec217[(l154) as usize] = 0.0;
		}
		for l155 in 0..2048 {
			self.fRec218[(l155) as usize] = 0.0;
		}
		for l156 in 0..2 {
			self.fVec38[(l156) as usize] = 0.0;
		}
		for l157 in 0..2 {
			self.fVec39[(l157) as usize] = 0.0;
		}
		for l158 in 0..2 {
			self.iRec219[(l158) as usize] = 0;
		}
		for l159 in 0..3 {
			self.fRec220[(l159) as usize] = 0.0;
		}
		for l160 in 0..3 {
			self.fVec40[(l160) as usize] = 0.0;
		}
		for l161 in 0..2048 {
			self.fRec211[(l161) as usize] = 0.0;
		}
		for l162 in 0..2 {
			self.fRec202[(l162) as usize] = 0.0;
		}
		for l163 in 0..2 {
			self.fRec198[(l163) as usize] = 0.0;
		}
		for l164 in 0..2048 {
			self.fRec194[(l164) as usize] = 0.0;
		}
		for l165 in 0..2 {
			self.fRec196[(l165) as usize] = 0.0;
		}
		for l166 in 0..4 {
			self.fRec192[(l166) as usize] = 0.0;
		}
		for l167 in 0..2 {
			self.fRec187[(l167) as usize] = 0.0;
		}
		for l168 in 0..2048 {
			self.fRec183[(l168) as usize] = 0.0;
		}
		for l169 in 0..2 {
			self.fRec181[(l169) as usize] = 0.0;
		}
		for l170 in 0..2 {
			self.fRec180[(l170) as usize] = 0.0;
		}
		for l171 in 0..2 {
			self.fRec179[(l171) as usize] = 0.0;
		}
		for l172 in 0..2 {
			self.fRec177[(l172) as usize] = 0.0;
		}
		for l173 in 0..2 {
			self.fRec175[(l173) as usize] = 0.0;
		}
		for l174 in 0..2 {
			self.fRec221[(l174) as usize] = 0.0;
		}
		for l175 in 0..2 {
			self.fRec223[(l175) as usize] = 0.0;
		}
		for l176 in 0..2 {
			self.fRec222[(l176) as usize] = 0.0;
		}
		for l177 in 0..2 {
			self.fRec228[(l177) as usize] = 0.0;
		}
		for l178 in 0..2 {
			self.fRec226[(l178) as usize] = 0.0;
		}
		for l179 in 0..2 {
			self.fRec229[(l179) as usize] = 0.0;
		}
		for l180 in 0..3 {
			self.fRec225[(l180) as usize] = 0.0;
		}
		for l181 in 0..3 {
			self.fRec224[(l181) as usize] = 0.0;
		}
		for l182 in 0..2 {
			self.fRec230[(l182) as usize] = 0.0;
		}
		for l183 in 0..2 {
			self.fRec235[(l183) as usize] = 0.0;
		}
		for l184 in 0..2 {
			self.fRec233[(l184) as usize] = 0.0;
		}
		for l185 in 0..2 {
			self.fRec236[(l185) as usize] = 0.0;
		}
		for l186 in 0..3 {
			self.fRec232[(l186) as usize] = 0.0;
		}
		for l187 in 0..3 {
			self.fRec231[(l187) as usize] = 0.0;
		}
		for l188 in 0..2 {
			self.fRec237[(l188) as usize] = 0.0;
		}
		for l189 in 0..2 {
			self.fRec242[(l189) as usize] = 0.0;
		}
		for l190 in 0..2 {
			self.fRec240[(l190) as usize] = 0.0;
		}
		for l191 in 0..2 {
			self.fRec243[(l191) as usize] = 0.0;
		}
		for l192 in 0..3 {
			self.fRec239[(l192) as usize] = 0.0;
		}
		for l193 in 0..3 {
			self.fRec238[(l193) as usize] = 0.0;
		}
		for l194 in 0..2 {
			self.fRec244[(l194) as usize] = 0.0;
		}
		for l195 in 0..2 {
			self.fRec249[(l195) as usize] = 0.0;
		}
		for l196 in 0..2 {
			self.fRec247[(l196) as usize] = 0.0;
		}
		for l197 in 0..2 {
			self.fRec250[(l197) as usize] = 0.0;
		}
		for l198 in 0..3 {
			self.fRec246[(l198) as usize] = 0.0;
		}
		for l199 in 0..3 {
			self.fRec245[(l199) as usize] = 0.0;
		}
		for l200 in 0..2 {
			self.fRec251[(l200) as usize] = 0.0;
		}
		for l201 in 0..2 {
			self.fRec252[(l201) as usize] = 0.0;
		}
		for l202 in 0..2 {
			self.fRec253[(l202) as usize] = 0.0;
		}
		for l203 in 0..2 {
			self.fRec255[(l203) as usize] = 0.0;
		}
		for l204 in 0..2097152 {
			self.fRec254[(l204) as usize] = 0.0;
		}
		for l205 in 0..2 {
			self.fRec266[(l205) as usize] = 0.0;
		}
		for l206 in 0..2 {
			self.fRec268[(l206) as usize] = 0.0;
		}
		for l207 in 0..2 {
			self.fRec272[(l207) as usize] = 0.0;
		}
		for l208 in 0..16384 {
			self.fVec41[(l208) as usize] = 0.0;
		}
		for l209 in 0..2 {
			self.fVec42[(l209) as usize] = 0.0;
		}
		for l210 in 0..2 {
			self.fRec271[(l210) as usize] = 0.0;
		}
		for l211 in 0..2 {
			self.fRec269[(l211) as usize] = 0.0;
		}
		for l212 in 0..2 {
			self.fRec274[(l212) as usize] = 0.0;
		}
		for l213 in 0..16384 {
			self.fVec43[(l213) as usize] = 0.0;
		}
		for l214 in 0..2 {
			self.fVec44[(l214) as usize] = 0.0;
		}
		for l215 in 0..2 {
			self.fRec273[(l215) as usize] = 0.0;
		}
		for l216 in 0..2 {
			self.fRec270[(l216) as usize] = 0.0;
		}
		for l217 in 0..2 {
			self.fRec278[(l217) as usize] = 0.0;
		}
		for l218 in 0..16384 {
			self.fVec45[(l218) as usize] = 0.0;
		}
		for l219 in 0..2 {
			self.fVec46[(l219) as usize] = 0.0;
		}
		for l220 in 0..2 {
			self.fRec277[(l220) as usize] = 0.0;
		}
		for l221 in 0..2 {
			self.fRec275[(l221) as usize] = 0.0;
		}
		for l222 in 0..2 {
			self.fRec280[(l222) as usize] = 0.0;
		}
		for l223 in 0..16384 {
			self.fVec47[(l223) as usize] = 0.0;
		}
		for l224 in 0..2 {
			self.fVec48[(l224) as usize] = 0.0;
		}
		for l225 in 0..2 {
			self.fRec279[(l225) as usize] = 0.0;
		}
		for l226 in 0..2 {
			self.fRec276[(l226) as usize] = 0.0;
		}
		for l227 in 0..2 {
			self.fRec284[(l227) as usize] = 0.0;
		}
		for l228 in 0..16384 {
			self.fVec49[(l228) as usize] = 0.0;
		}
		for l229 in 0..2 {
			self.fVec50[(l229) as usize] = 0.0;
		}
		for l230 in 0..2 {
			self.fRec283[(l230) as usize] = 0.0;
		}
		for l231 in 0..2 {
			self.fRec281[(l231) as usize] = 0.0;
		}
		for l232 in 0..2 {
			self.fRec286[(l232) as usize] = 0.0;
		}
		for l233 in 0..16384 {
			self.fVec51[(l233) as usize] = 0.0;
		}
		for l234 in 0..2 {
			self.fVec52[(l234) as usize] = 0.0;
		}
		for l235 in 0..2 {
			self.fRec285[(l235) as usize] = 0.0;
		}
		for l236 in 0..2 {
			self.fRec282[(l236) as usize] = 0.0;
		}
		for l237 in 0..2 {
			self.fRec290[(l237) as usize] = 0.0;
		}
		for l238 in 0..16384 {
			self.fVec53[(l238) as usize] = 0.0;
		}
		for l239 in 0..2 {
			self.fVec54[(l239) as usize] = 0.0;
		}
		for l240 in 0..2 {
			self.fRec289[(l240) as usize] = 0.0;
		}
		for l241 in 0..2 {
			self.fRec287[(l241) as usize] = 0.0;
		}
		for l242 in 0..2 {
			self.fRec292[(l242) as usize] = 0.0;
		}
		for l243 in 0..16384 {
			self.fVec55[(l243) as usize] = 0.0;
		}
		for l244 in 0..2 {
			self.fVec56[(l244) as usize] = 0.0;
		}
		for l245 in 0..2 {
			self.fRec291[(l245) as usize] = 0.0;
		}
		for l246 in 0..2 {
			self.fRec288[(l246) as usize] = 0.0;
		}
		for l247 in 0..2 {
			self.fRec296[(l247) as usize] = 0.0;
		}
		for l248 in 0..16384 {
			self.fVec57[(l248) as usize] = 0.0;
		}
		for l249 in 0..2 {
			self.fVec58[(l249) as usize] = 0.0;
		}
		for l250 in 0..2 {
			self.fRec295[(l250) as usize] = 0.0;
		}
		for l251 in 0..2 {
			self.fRec293[(l251) as usize] = 0.0;
		}
		for l252 in 0..2 {
			self.fRec298[(l252) as usize] = 0.0;
		}
		for l253 in 0..16384 {
			self.fVec59[(l253) as usize] = 0.0;
		}
		for l254 in 0..2 {
			self.fVec60[(l254) as usize] = 0.0;
		}
		for l255 in 0..2 {
			self.fRec297[(l255) as usize] = 0.0;
		}
		for l256 in 0..2 {
			self.fRec294[(l256) as usize] = 0.0;
		}
		for l257 in 0..1024 {
			self.fVec61[(l257) as usize] = 0.0;
		}
		for l258 in 0..2 {
			self.fRec299[(l258) as usize] = 0.0;
		}
		for l259 in 0..2 {
			self.fRec300[(l259) as usize] = 0.0;
		}
		for l260 in 0..16384 {
			self.fVec62[(l260) as usize] = 0.0;
		}
		for l261 in 0..2 {
			self.fVec63[(l261) as usize] = 0.0;
		}
		for l262 in 0..2 {
			self.fRec267[(l262) as usize] = 0.0;
		}
		for l263 in 0..2 {
			self.fRec304[(l263) as usize] = 0.0;
		}
		for l264 in 0..2 {
			self.fRec306[(l264) as usize] = 0.0;
		}
		for l265 in 0..1024 {
			self.fVec64[(l265) as usize] = 0.0;
		}
		for l266 in 0..16384 {
			self.fVec65[(l266) as usize] = 0.0;
		}
		for l267 in 0..2 {
			self.fVec66[(l267) as usize] = 0.0;
		}
		for l268 in 0..2 {
			self.fRec305[(l268) as usize] = 0.0;
		}
		for l269 in 0..16384 {
			self.fVec67[(l269) as usize] = 0.0;
		}
		for l270 in 0..2 {
			self.fVec68[(l270) as usize] = 0.0;
		}
		for l271 in 0..2 {
			self.fRec303[(l271) as usize] = 0.0;
		}
		for l272 in 0..2 {
			self.fRec301[(l272) as usize] = 0.0;
		}
		for l273 in 0..2 {
			self.fRec308[(l273) as usize] = 0.0;
		}
		for l274 in 0..16384 {
			self.fVec69[(l274) as usize] = 0.0;
		}
		for l275 in 0..2 {
			self.fVec70[(l275) as usize] = 0.0;
		}
		for l276 in 0..2 {
			self.fRec307[(l276) as usize] = 0.0;
		}
		for l277 in 0..2 {
			self.fRec302[(l277) as usize] = 0.0;
		}
		for l278 in 0..2 {
			self.fRec312[(l278) as usize] = 0.0;
		}
		for l279 in 0..16384 {
			self.fVec71[(l279) as usize] = 0.0;
		}
		for l280 in 0..2 {
			self.fVec72[(l280) as usize] = 0.0;
		}
		for l281 in 0..2 {
			self.fRec311[(l281) as usize] = 0.0;
		}
		for l282 in 0..2 {
			self.fRec309[(l282) as usize] = 0.0;
		}
		for l283 in 0..2 {
			self.fRec314[(l283) as usize] = 0.0;
		}
		for l284 in 0..16384 {
			self.fVec73[(l284) as usize] = 0.0;
		}
		for l285 in 0..2 {
			self.fVec74[(l285) as usize] = 0.0;
		}
		for l286 in 0..2 {
			self.fRec313[(l286) as usize] = 0.0;
		}
		for l287 in 0..2 {
			self.fRec310[(l287) as usize] = 0.0;
		}
		for l288 in 0..2 {
			self.fRec318[(l288) as usize] = 0.0;
		}
		for l289 in 0..16384 {
			self.fVec75[(l289) as usize] = 0.0;
		}
		for l290 in 0..2 {
			self.fVec76[(l290) as usize] = 0.0;
		}
		for l291 in 0..2 {
			self.fRec317[(l291) as usize] = 0.0;
		}
		for l292 in 0..2 {
			self.fRec315[(l292) as usize] = 0.0;
		}
		for l293 in 0..2 {
			self.fRec320[(l293) as usize] = 0.0;
		}
		for l294 in 0..16384 {
			self.fVec77[(l294) as usize] = 0.0;
		}
		for l295 in 0..2 {
			self.fVec78[(l295) as usize] = 0.0;
		}
		for l296 in 0..2 {
			self.fRec319[(l296) as usize] = 0.0;
		}
		for l297 in 0..2 {
			self.fRec316[(l297) as usize] = 0.0;
		}
		for l298 in 0..2 {
			self.fRec324[(l298) as usize] = 0.0;
		}
		for l299 in 0..16384 {
			self.fVec79[(l299) as usize] = 0.0;
		}
		for l300 in 0..2 {
			self.fVec80[(l300) as usize] = 0.0;
		}
		for l301 in 0..2 {
			self.fRec323[(l301) as usize] = 0.0;
		}
		for l302 in 0..2 {
			self.fRec321[(l302) as usize] = 0.0;
		}
		for l303 in 0..2 {
			self.fRec326[(l303) as usize] = 0.0;
		}
		for l304 in 0..16384 {
			self.fVec81[(l304) as usize] = 0.0;
		}
		for l305 in 0..2 {
			self.fVec82[(l305) as usize] = 0.0;
		}
		for l306 in 0..2 {
			self.fRec325[(l306) as usize] = 0.0;
		}
		for l307 in 0..2 {
			self.fRec322[(l307) as usize] = 0.0;
		}
		for l308 in 0..2 {
			self.fRec330[(l308) as usize] = 0.0;
		}
		for l309 in 0..16384 {
			self.fVec83[(l309) as usize] = 0.0;
		}
		for l310 in 0..2 {
			self.fVec84[(l310) as usize] = 0.0;
		}
		for l311 in 0..2 {
			self.fRec329[(l311) as usize] = 0.0;
		}
		for l312 in 0..2 {
			self.fRec327[(l312) as usize] = 0.0;
		}
		for l313 in 0..2 {
			self.fRec332[(l313) as usize] = 0.0;
		}
		for l314 in 0..16384 {
			self.fVec85[(l314) as usize] = 0.0;
		}
		for l315 in 0..2 {
			self.fVec86[(l315) as usize] = 0.0;
		}
		for l316 in 0..2 {
			self.fRec331[(l316) as usize] = 0.0;
		}
		for l317 in 0..2 {
			self.fRec328[(l317) as usize] = 0.0;
		}
		for l318 in 0..16384 {
			self.fVec87[(l318) as usize] = 0.0;
		}
		for l319 in 0..16384 {
			self.fVec88[(l319) as usize] = 0.0;
		}
		for l320 in 0..2 {
			self.fVec89[(l320) as usize] = 0.0;
		}
		for l321 in 0..2 {
			self.fRec265[(l321) as usize] = 0.0;
		}
		for l322 in 0..2 {
			self.fRec264[(l322) as usize] = 0.0;
		}
		for l323 in 0..3 {
			self.fRec263[(l323) as usize] = 0.0;
		}
		for l324 in 0..3 {
			self.fRec262[(l324) as usize] = 0.0;
		}
		for l325 in 0..2 {
			self.fVec90[(l325) as usize] = 0.0;
		}
		for l326 in 0..2 {
			self.fRec261[(l326) as usize] = 0.0;
		}
		for l327 in 0..3 {
			self.fRec260[(l327) as usize] = 0.0;
		}
		for l328 in 0..3 {
			self.fRec259[(l328) as usize] = 0.0;
		}
		for l329 in 0..2 {
			self.fRec335[(l329) as usize] = 0.0;
		}
		for l330 in 0..3 {
			self.fRec334[(l330) as usize] = 0.0;
		}
		for l331 in 0..3 {
			self.fRec333[(l331) as usize] = 0.0;
		}
		for l332 in 0..2 {
			self.fRec339[(l332) as usize] = 0.0;
		}
		for l333 in 0..3 {
			self.fRec338[(l333) as usize] = 0.0;
		}
		for l334 in 0..3 {
			self.fRec337[(l334) as usize] = 0.0;
		}
		for l335 in 0..3 {
			self.fRec336[(l335) as usize] = 0.0;
		}
		for l336 in 0..1024 {
			self.fVec91[(l336) as usize] = 0.0;
		}
		for l337 in 0..2 {
			self.fRec258[(l337) as usize] = 0.0;
		}
		for l338 in 0..2 {
			self.fRec351[(l338) as usize] = 0.0;
		}
		for l339 in 0..16384 {
			self.fVec92[(l339) as usize] = 0.0;
		}
		for l340 in 0..16384 {
			self.fVec93[(l340) as usize] = 0.0;
		}
		for l341 in 0..2 {
			self.fVec94[(l341) as usize] = 0.0;
		}
		for l342 in 0..2 {
			self.fRec350[(l342) as usize] = 0.0;
		}
		for l343 in 0..2 {
			self.fRec349[(l343) as usize] = 0.0;
		}
		for l344 in 0..3 {
			self.fRec348[(l344) as usize] = 0.0;
		}
		for l345 in 0..3 {
			self.fRec347[(l345) as usize] = 0.0;
		}
		for l346 in 0..2 {
			self.fVec95[(l346) as usize] = 0.0;
		}
		for l347 in 0..2 {
			self.fRec346[(l347) as usize] = 0.0;
		}
		for l348 in 0..3 {
			self.fRec345[(l348) as usize] = 0.0;
		}
		for l349 in 0..3 {
			self.fRec344[(l349) as usize] = 0.0;
		}
		for l350 in 0..2 {
			self.fRec354[(l350) as usize] = 0.0;
		}
		for l351 in 0..3 {
			self.fRec353[(l351) as usize] = 0.0;
		}
		for l352 in 0..3 {
			self.fRec352[(l352) as usize] = 0.0;
		}
		for l353 in 0..2 {
			self.fRec358[(l353) as usize] = 0.0;
		}
		for l354 in 0..3 {
			self.fRec357[(l354) as usize] = 0.0;
		}
		for l355 in 0..3 {
			self.fRec356[(l355) as usize] = 0.0;
		}
		for l356 in 0..3 {
			self.fRec355[(l356) as usize] = 0.0;
		}
		for l357 in 0..1024 {
			self.fVec96[(l357) as usize] = 0.0;
		}
		for l358 in 0..2 {
			self.fRec343[(l358) as usize] = 0.0;
		}
		for l359 in 0..16384 {
			self.fVec97[(l359) as usize] = 0.0;
		}
		for l360 in 0..2 {
			self.fVec98[(l360) as usize] = 0.0;
		}
		for l361 in 0..2 {
			self.fRec342[(l361) as usize] = 0.0;
		}
		for l362 in 0..2 {
			self.fRec340[(l362) as usize] = 0.0;
		}
		for l363 in 0..2 {
			self.fRec360[(l363) as usize] = 0.0;
		}
		for l364 in 0..16384 {
			self.fVec99[(l364) as usize] = 0.0;
		}
		for l365 in 0..2 {
			self.fVec100[(l365) as usize] = 0.0;
		}
		for l366 in 0..2 {
			self.fRec359[(l366) as usize] = 0.0;
		}
		for l367 in 0..2 {
			self.fRec341[(l367) as usize] = 0.0;
		}
		for l368 in 0..16384 {
			self.fVec101[(l368) as usize] = 0.0;
		}
		for l369 in 0..2 {
			self.fVec102[(l369) as usize] = 0.0;
		}
		for l370 in 0..2 {
			self.fRec363[(l370) as usize] = 0.0;
		}
		for l371 in 0..2 {
			self.fRec361[(l371) as usize] = 0.0;
		}
		for l372 in 0..16384 {
			self.fVec103[(l372) as usize] = 0.0;
		}
		for l373 in 0..2 {
			self.fVec104[(l373) as usize] = 0.0;
		}
		for l374 in 0..2 {
			self.fRec364[(l374) as usize] = 0.0;
		}
		for l375 in 0..2 {
			self.fRec362[(l375) as usize] = 0.0;
		}
		for l376 in 0..16384 {
			self.fVec105[(l376) as usize] = 0.0;
		}
		for l377 in 0..2 {
			self.fVec106[(l377) as usize] = 0.0;
		}
		for l378 in 0..2 {
			self.fRec367[(l378) as usize] = 0.0;
		}
		for l379 in 0..2 {
			self.fRec365[(l379) as usize] = 0.0;
		}
		for l380 in 0..2 {
			self.fRec369[(l380) as usize] = 0.0;
		}
		for l381 in 0..16384 {
			self.fVec107[(l381) as usize] = 0.0;
		}
		for l382 in 0..2 {
			self.fVec108[(l382) as usize] = 0.0;
		}
		for l383 in 0..2 {
			self.fRec368[(l383) as usize] = 0.0;
		}
		for l384 in 0..2 {
			self.fRec366[(l384) as usize] = 0.0;
		}
		for l385 in 0..2 {
			self.fRec373[(l385) as usize] = 0.0;
		}
		for l386 in 0..16384 {
			self.fVec109[(l386) as usize] = 0.0;
		}
		for l387 in 0..2 {
			self.fVec110[(l387) as usize] = 0.0;
		}
		for l388 in 0..2 {
			self.fRec372[(l388) as usize] = 0.0;
		}
		for l389 in 0..2 {
			self.fRec370[(l389) as usize] = 0.0;
		}
		for l390 in 0..16384 {
			self.fVec111[(l390) as usize] = 0.0;
		}
		for l391 in 0..2 {
			self.fVec112[(l391) as usize] = 0.0;
		}
		for l392 in 0..2 {
			self.fRec374[(l392) as usize] = 0.0;
		}
		for l393 in 0..2 {
			self.fRec371[(l393) as usize] = 0.0;
		}
		for l394 in 0..2 {
			self.fRec256[(l394) as usize] = 0.0;
		}
		for l395 in 0..2 {
			self.fRec257[(l395) as usize] = 0.0;
		}
		for l396 in 0..2 {
			self.fRec375[(l396) as usize] = 0.0;
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
		self.fConst6 = 0.8 / self.fConst0;
		self.fConst7 = 6.2831855 / self.fConst0;
		self.fConst8 = 0.00882353 * self.fConst0;
		self.fConst9 = 0.00073529413 * self.fConst0;
		self.fConst10 = F32::exp(0.0 - 1e+04 / self.fConst0);
		self.fConst11 = 1.0 - self.fConst10;
		self.iConst12 = ((0.1 * self.fConst0) as i32);
		self.fConst13 = F32::exp(0.0 - 5e+01 / self.fConst0);
		self.fConst14 = 15.707963 / self.fConst0;
		self.fConst15 = 0.002 * self.fConst0;
		self.fConst16 = F32::exp(0.0 - 1e+01 / self.fConst0);
		self.fConst17 = 3.1415927 / self.fConst0;
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
		self.fConst37 = 1.0 - self.fConst32;
		let mut fConst38: F32 = self.fConst32 + 1.0;
		self.fConst39 = 1.0 / fConst38;
		self.fConst40 = 1.0 - fConst23;
		let mut fConst41: F32 = fConst23 + 1.0;
		self.fConst42 = 1.0 / fConst41;
		self.fConst43 = self.fConst40 / fConst41;
		self.fConst44 = 1.0 / (fConst19 * fConst41);
		self.fConst45 = 0.0 - self.fConst44;
		self.fConst46 = 0.0 - 2.0 / fConst20;
		self.fConst47 = (fConst23 + -1.618034) / fConst19 + 1.0;
		self.fConst48 = 1.0 / ((fConst23 + 1.618034) / fConst19 + 1.0);
		self.fConst49 = 0.0 - 1.0 / (fConst28 * fConst38);
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
			3 => Some(self.fHslider21),
			4 => Some(self.fHslider22),
			5 => Some(self.fHslider23),
			6 => Some(self.fHslider24),
			7 => Some(self.fHslider25),
			8 => Some(self.fHslider26),
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
			3 => { self.fHslider21 = value }
			4 => { self.fHslider22 = value }
			5 => { self.fHslider23 = value }
			6 => { self.fHslider24 = value }
			7 => { self.fHslider25 = value }
			8 => { self.fHslider26 = value }
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
		let mut fSlow24: F32 = self.fConst1 * unsafe { ftbl0mydspSIG0[(((16.11811 * fSlow23) as i32)) as usize] };
		let mut fSlow25: F32 = self.fConst1 * self.fHslider20;
		let mut fSlow26: F32 = self.fHslider21;
		let mut fSlow27: F32 = self.fConst1 * unsafe { ftbl0mydspSIG0[(((16.11811 * fSlow26) as i32)) as usize] };
		let mut fSlow28: F32 = self.fConst1 * self.fHslider22;
		let mut fSlow29: F32 = self.fHslider23;
		let mut fSlow30: F32 = self.fConst1 * unsafe { ftbl0mydspSIG0[(((16.11811 * fSlow29) as i32)) as usize] };
		let mut fSlow31: F32 = self.fConst1 * self.fHslider24;
		let mut fSlow32: F32 = self.fHslider25;
		let mut fSlow33: F32 = self.fConst1 * unsafe { ftbl0mydspSIG0[(((16.11811 * fSlow32) as i32)) as usize] };
		let mut fSlow34: F32 = self.fConst1 * self.fHslider26;
		let mut fSlow35: F32 = self.fConst1 * self.fHslider27;
		let mut fSlow36: F32 = self.fConst1 * self.fHslider28;
		let mut fSlow37: F32 = self.fConst1 * self.fHslider29;
		let mut fSlow38: F32 = self.fConst1 * self.fHslider30;
		let mut fSlow39: F32 = self.fHslider31;
		let mut fSlow40: F32 = self.fHslider32;
		let mut fSlow41: F32 = 1.0 - fSlow40;
		let mut fSlow42: F32 = self.fHslider33;
		let mut iSlow43: i32 = unsafe { itbl1mydspSIG1[(((134.0 * fSlow42) as i32)) as usize] };
		let mut fSlow44: F32 = 0.005 * ((iSlow43) as F32);
		let mut iSlow45: i32 = unsafe { itbl1mydspSIG1[(((54.0 * fSlow42) as i32)) as usize] };
		let mut fSlow46: F32 = 0.005 * ((iSlow45) as F32);
		let mut iSlow47: i32 = unsafe { itbl1mydspSIG1[(((1e+01 * fSlow42) as i32)) as usize] };
		let mut fSlow48: F32 = 0.0001 * ((iSlow47) as F32);
		let mut iSlow49: i32 = unsafe { itbl1mydspSIG1[(((1.1e+02 * fSlow42) as i32)) as usize] };
		let mut fSlow50: F32 = 0.0001 * ((iSlow49) as F32);
		let mut iSlow51: i32 = unsafe { itbl1mydspSIG1[(((4e+01 * fSlow42) as i32)) as usize] };
		let mut fSlow52: F32 = 0.0001 * ((iSlow51) as F32);
		let mut iSlow53: i32 = unsafe { itbl1mydspSIG1[(((1.4e+02 * fSlow42) as i32)) as usize] };
		let mut fSlow54: F32 = 0.0001 * ((iSlow53) as F32);
		let mut iSlow55: i32 = unsafe { itbl1mydspSIG1[(((7e+01 * fSlow42) as i32)) as usize] };
		let mut fSlow56: F32 = 0.0001 * ((iSlow55) as F32);
		let mut iSlow57: i32 = unsafe { itbl1mydspSIG1[(((1.7e+02 * fSlow42) as i32)) as usize] };
		let mut fSlow58: F32 = 0.0001 * ((iSlow57) as F32);
		let mut iSlow59: i32 = unsafe { itbl1mydspSIG1[(((1e+02 * fSlow42) as i32)) as usize] };
		let mut fSlow60: F32 = 0.0001 * ((iSlow59) as F32);
		let mut iSlow61: i32 = unsafe { itbl1mydspSIG1[(((2e+02 * fSlow42) as i32)) as usize] };
		let mut fSlow62: F32 = 0.0001 * ((iSlow61) as F32);
		let mut iSlow63: i32 = unsafe { itbl1mydspSIG1[(((1.3e+02 * fSlow42) as i32)) as usize] };
		let mut fSlow64: F32 = 0.0001 * ((iSlow63) as F32);
		let mut iSlow65: i32 = unsafe { itbl1mydspSIG1[(((2.3e+02 * fSlow42) as i32)) as usize] };
		let mut fSlow66: F32 = 0.0001 * ((iSlow65) as F32);
		let mut fSlow67: F32 = self.fConst7 * self.fHslider34;
		let mut fSlow68: F32 = F32::cos(fSlow67);
		let mut fSlow69: F32 = F32::sin(fSlow67);
		let mut fSlow70: F32 = 5e+01 * self.fHslider35;
		let mut iSlow71: i32 = unsafe { itbl1mydspSIG1[(((125.0 * fSlow42) as i32)) as usize] };
		let mut fSlow72: F32 = 0.0001 * ((iSlow71) as F32);
		let mut iSlow73: i32 = unsafe { itbl1mydspSIG1[(((204.0 * fSlow42) as i32)) as usize] };
		let mut fSlow74: F32 = 0.005 * ((iSlow73) as F32);
		let mut fSlow75: F32 = 0.0 - fSlow70;
		let mut iSlow76: i32 = unsafe { itbl1mydspSIG1[(((25.0 * fSlow42) as i32)) as usize] };
		let mut fSlow77: F32 = 0.0001 * ((iSlow76) as F32);
		let mut iSlow78: i32 = unsafe { itbl1mydspSIG1[(((155.0 * fSlow42) as i32)) as usize] };
		let mut fSlow79: F32 = 0.0001 * ((iSlow78) as F32);
		let mut iSlow80: i32 = unsafe { itbl1mydspSIG1[(((55.0 * fSlow42) as i32)) as usize] };
		let mut fSlow81: F32 = 0.0001 * ((iSlow80) as F32);
		let mut iSlow82: i32 = unsafe { itbl1mydspSIG1[(((185.0 * fSlow42) as i32)) as usize] };
		let mut fSlow83: F32 = 0.0001 * ((iSlow82) as F32);
		let mut iSlow84: i32 = unsafe { itbl1mydspSIG1[(((85.0 * fSlow42) as i32)) as usize] };
		let mut fSlow85: F32 = 0.0001 * ((iSlow84) as F32);
		let mut iSlow86: i32 = unsafe { itbl1mydspSIG1[(((215.0 * fSlow42) as i32)) as usize] };
		let mut fSlow87: F32 = 0.0001 * ((iSlow86) as F32);
		let mut iSlow88: i32 = unsafe { itbl1mydspSIG1[(((115.0 * fSlow42) as i32)) as usize] };
		let mut fSlow89: F32 = 0.0001 * ((iSlow88) as F32);
		let mut iSlow90: i32 = unsafe { itbl1mydspSIG1[(((245.0 * fSlow42) as i32)) as usize] };
		let mut fSlow91: F32 = 0.0001 * ((iSlow90) as F32);
		let mut iSlow92: i32 = unsafe { itbl1mydspSIG1[(((145.0 * fSlow42) as i32)) as usize] };
		let mut fSlow93: F32 = 0.0001 * ((iSlow92) as F32);
		let mut fSlow94: F32 = F32::powf(1e+01, 0.0 - 0.51 * ((1.25 * fSlow42 + -0.25) / self.fHslider36));
		let mut fSlow95: F32 = self.fHslider37;
		let mut fSlow96: F32 = 1.0 - fSlow95;
		let mut fSlow97: F32 = self.fHslider38;
		let mut fSlow98: F32 = F32::sin(fSlow97);
		let mut iSlow99: i32 = unsafe { itbl1mydspSIG1[(((34.0 * fSlow42) as i32)) as usize] };
		let mut fSlow100: F32 = 0.005 * ((iSlow99) as F32);
		let mut fSlow101: F32 = F32::cos(fSlow97);
		let mut iSlow102: i32 = unsafe { itbl1mydspSIG1[(((2.4e+02 * fSlow42) as i32)) as usize] };
		let mut fSlow103: F32 = 0.0001 * ((iSlow102) as F32);
		let mut iSlow104: i32 = unsafe { itbl1mydspSIG1[(((1.9e+02 * fSlow42) as i32)) as usize] };
		let mut fSlow105: F32 = 0.0001 * ((iSlow104) as F32);
		let mut iSlow106: i32 = unsafe { itbl1mydspSIG1[(((175.0 * fSlow42) as i32)) as usize] };
		let mut fSlow107: F32 = 0.0001 * ((iSlow106) as F32);
		let mut fSlow108: F32 = self.fConst1 * self.fHslider39;
		let zipped_iterators = outputs0.zip(outputs1);
		for (output0, output1) in zipped_iterators {
			self.iVec0[0] = 1;
			self.fRec1[0] = fSlow1 + self.fConst2 * self.fRec1[1];
			let mut fTemp0: F32 = unsafe { ftbl0mydspSIG0[(((16.11811 * (self.fRec1[0] - fSlow0)) as i32)) as usize] };
			let mut fTemp1: F32 = F32::max(fTemp0, 23.44895);
			let mut fTemp2: F32 = F32::max(2e+01, F32::abs(fTemp1));
			let mut fTemp3: F32 = self.fRec3[1] + self.fConst3 * fTemp2;
			self.fRec3[0] = fTemp3 - F32::floor(fTemp3);
			let mut fTemp4: F32 = mydsp_faustpower2_f(2.0 * self.fRec3[0] + -1.0);
			self.fVec1[0] = fTemp4;
			let mut fTemp5: F32 = ((self.iVec0[1]) as F32);
			let mut fTemp6: F32 = fTemp5 * (fTemp4 - self.fVec1[1]) / fTemp2;
			self.fVec2[(self.IOTA0 & 4095) as usize] = fTemp6;
			let mut fTemp7: F32 = F32::max(0.0, F32::min(2047.0, self.fConst4 / fTemp1));
			let mut iTemp8: i32 = ((fTemp7) as i32);
			let mut fTemp9: F32 = F32::floor(fTemp7);
			self.fRec2[0] = 0.999 * self.fRec2[1] - self.fConst5 * (self.fVec2[((i32::wrapping_sub(self.IOTA0, iTemp8)) & 4095) as usize] * (fTemp9 + (1.0 - fTemp7)) - fTemp6 + (fTemp7 - fTemp9) * self.fVec2[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp8, 1))) & 4095) as usize]);
			let mut fTemp10: F32 = unsafe { ftbl0mydspSIG0[(((16.11811 * (fSlow0 + self.fRec1[0])) as i32)) as usize] };
			let mut fTemp11: F32 = F32::max(fTemp10, 23.44895);
			let mut fTemp12: F32 = F32::max(2e+01, F32::abs(fTemp11));
			let mut fTemp13: F32 = self.fRec5[1] + self.fConst3 * fTemp12;
			self.fRec5[0] = fTemp13 - F32::floor(fTemp13);
			let mut fTemp14: F32 = mydsp_faustpower2_f(2.0 * self.fRec5[0] + -1.0);
			self.fVec3[0] = fTemp14;
			let mut fTemp15: F32 = fTemp5 * (fTemp14 - self.fVec3[1]) / fTemp12;
			self.fVec4[(self.IOTA0 & 4095) as usize] = fTemp15;
			let mut fTemp16: F32 = F32::max(0.0, F32::min(2047.0, self.fConst4 / fTemp11));
			let mut iTemp17: i32 = ((fTemp16) as i32);
			let mut fTemp18: F32 = F32::floor(fTemp16);
			self.fRec4[0] = 0.999 * self.fRec4[1] - self.fConst5 * (self.fVec4[((i32::wrapping_sub(self.IOTA0, iTemp17)) & 4095) as usize] * (fTemp18 + (1.0 - fTemp16)) - fTemp15 + (fTemp16 - fTemp18) * self.fVec4[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp17, 1))) & 4095) as usize]);
			let mut fTemp19: F32 = unsafe { ftbl0mydspSIG0[(((16.11811 * self.fRec1[0]) as i32)) as usize] };
			let mut fTemp20: F32 = F32::max(fTemp19, 23.44895);
			let mut fTemp21: F32 = F32::max(0.0, F32::min(2047.0, self.fConst4 / fTemp20));
			let mut fTemp22: F32 = F32::floor(fTemp21);
			let mut fTemp23: F32 = F32::max(2e+01, F32::abs(fTemp20));
			let mut fTemp24: F32 = self.fRec7[1] + self.fConst3 * fTemp23;
			self.fRec7[0] = fTemp24 - F32::floor(fTemp24);
			let mut fTemp25: F32 = mydsp_faustpower2_f(2.0 * self.fRec7[0] + -1.0);
			self.fVec5[0] = fTemp25;
			let mut fTemp26: F32 = fTemp5 * (fTemp25 - self.fVec5[1]) / fTemp23;
			self.fVec6[(self.IOTA0 & 4095) as usize] = fTemp26;
			let mut iTemp27: i32 = ((fTemp21) as i32);
			self.fRec6[0] = 0.999 * self.fRec6[1] - self.fConst5 * ((fTemp21 - fTemp22) * self.fVec6[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp27, 1))) & 4095) as usize] - (fTemp26 - self.fVec6[((i32::wrapping_sub(self.IOTA0, iTemp27)) & 4095) as usize] * (fTemp22 + (1.0 - fTemp21))));
			self.fRec8[0] = fSlow2 + self.fConst2 * self.fRec8[1];
			self.fRec9[0] = fSlow3 + self.fConst2 * self.fRec9[1];
			let mut fTemp28: F32 = unsafe { ftbl0mydspSIG0[(((16.11811 * (self.fRec9[0] - fSlow0)) as i32)) as usize] };
			let mut fTemp29: F32 = F32::max(fTemp28, 23.44895);
			let mut fTemp30: F32 = F32::max(2e+01, F32::abs(fTemp29));
			let mut fTemp31: F32 = self.fRec11[1] + self.fConst3 * fTemp30;
			self.fRec11[0] = fTemp31 - F32::floor(fTemp31);
			let mut fTemp32: F32 = mydsp_faustpower2_f(2.0 * self.fRec11[0] + -1.0);
			self.fVec7[0] = fTemp32;
			let mut fTemp33: F32 = fTemp5 * (fTemp32 - self.fVec7[1]) / fTemp30;
			self.fVec8[(self.IOTA0 & 4095) as usize] = fTemp33;
			let mut fTemp34: F32 = F32::max(0.0, F32::min(2047.0, self.fConst4 / fTemp29));
			let mut iTemp35: i32 = ((fTemp34) as i32);
			let mut fTemp36: F32 = F32::floor(fTemp34);
			self.fRec10[0] = 0.999 * self.fRec10[1] + self.fConst5 * (fTemp33 - self.fVec8[((i32::wrapping_sub(self.IOTA0, iTemp35)) & 4095) as usize] * (fTemp36 + (1.0 - fTemp34)) - (fTemp34 - fTemp36) * self.fVec8[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp35, 1))) & 4095) as usize]);
			let mut fTemp37: F32 = unsafe { ftbl0mydspSIG0[(((16.11811 * (fSlow0 + self.fRec9[0])) as i32)) as usize] };
			let mut fTemp38: F32 = F32::max(fTemp37, 23.44895);
			let mut fTemp39: F32 = F32::max(2e+01, F32::abs(fTemp38));
			let mut fTemp40: F32 = self.fRec13[1] + self.fConst3 * fTemp39;
			self.fRec13[0] = fTemp40 - F32::floor(fTemp40);
			let mut fTemp41: F32 = mydsp_faustpower2_f(2.0 * self.fRec13[0] + -1.0);
			self.fVec9[0] = fTemp41;
			let mut fTemp42: F32 = fTemp5 * (fTemp41 - self.fVec9[1]) / fTemp39;
			self.fVec10[(self.IOTA0 & 4095) as usize] = fTemp42;
			let mut fTemp43: F32 = F32::max(0.0, F32::min(2047.0, self.fConst4 / fTemp38));
			let mut iTemp44: i32 = ((fTemp43) as i32);
			let mut fTemp45: F32 = F32::floor(fTemp43);
			self.fRec12[0] = 0.999 * self.fRec12[1] + self.fConst5 * (fTemp42 - self.fVec10[((i32::wrapping_sub(self.IOTA0, iTemp44)) & 4095) as usize] * (fTemp45 + (1.0 - fTemp43)) - (fTemp43 - fTemp45) * self.fVec10[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp44, 1))) & 4095) as usize]);
			let mut fTemp46: F32 = unsafe { ftbl0mydspSIG0[(((16.11811 * self.fRec9[0]) as i32)) as usize] };
			let mut fTemp47: F32 = F32::max(fTemp46, 23.44895);
			let mut fTemp48: F32 = F32::max(2e+01, F32::abs(fTemp47));
			let mut fTemp49: F32 = self.fRec15[1] + self.fConst3 * fTemp48;
			self.fRec15[0] = fTemp49 - F32::floor(fTemp49);
			let mut fTemp50: F32 = mydsp_faustpower2_f(2.0 * self.fRec15[0] + -1.0);
			self.fVec11[0] = fTemp50;
			let mut fTemp51: F32 = fTemp5 * (fTemp50 - self.fVec11[1]) / fTemp48;
			self.fVec12[(self.IOTA0 & 4095) as usize] = fTemp51;
			let mut fTemp52: F32 = F32::max(0.0, F32::min(2047.0, self.fConst4 / fTemp47));
			let mut iTemp53: i32 = ((fTemp52) as i32);
			let mut fTemp54: F32 = F32::floor(fTemp52);
			self.fRec14[0] = 0.999 * self.fRec14[1] + self.fConst5 * (fTemp51 - self.fVec12[((i32::wrapping_sub(self.IOTA0, iTemp53)) & 4095) as usize] * (fTemp54 + (1.0 - fTemp52)) - (fTemp52 - fTemp54) * self.fVec12[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp53, 1))) & 4095) as usize]);
			self.fRec16[0] = fSlow4 + self.fConst2 * self.fRec16[1];
			self.fRec17[0] = fSlow5 + self.fConst2 * self.fRec17[1];
			let mut fTemp55: F32 = unsafe { ftbl0mydspSIG0[(((16.11811 * (self.fRec17[0] - fSlow0)) as i32)) as usize] };
			let mut fTemp56: F32 = F32::max(fTemp55, 23.44895);
			let mut fTemp57: F32 = F32::max(0.0, F32::min(2047.0, self.fConst4 / fTemp56));
			let mut fTemp58: F32 = F32::floor(fTemp57);
			let mut fTemp59: F32 = F32::max(2e+01, F32::abs(fTemp56));
			let mut fTemp60: F32 = self.fRec19[1] + self.fConst3 * fTemp59;
			self.fRec19[0] = fTemp60 - F32::floor(fTemp60);
			let mut fTemp61: F32 = mydsp_faustpower2_f(2.0 * self.fRec19[0] + -1.0);
			self.fVec13[0] = fTemp61;
			let mut fTemp62: F32 = fTemp5 * (fTemp61 - self.fVec13[1]) / fTemp59;
			self.fVec14[(self.IOTA0 & 4095) as usize] = fTemp62;
			let mut iTemp63: i32 = ((fTemp57) as i32);
			self.fRec18[0] = 0.999 * self.fRec18[1] - self.fConst5 * ((fTemp57 - fTemp58) * self.fVec14[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp63, 1))) & 4095) as usize] - (fTemp62 - self.fVec14[((i32::wrapping_sub(self.IOTA0, iTemp63)) & 4095) as usize] * (fTemp58 + (1.0 - fTemp57))));
			let mut fTemp64: F32 = unsafe { ftbl0mydspSIG0[(((16.11811 * (fSlow0 + self.fRec17[0])) as i32)) as usize] };
			let mut fTemp65: F32 = F32::max(fTemp64, 23.44895);
			let mut fTemp66: F32 = F32::max(2e+01, F32::abs(fTemp65));
			let mut fTemp67: F32 = self.fRec21[1] + self.fConst3 * fTemp66;
			self.fRec21[0] = fTemp67 - F32::floor(fTemp67);
			let mut fTemp68: F32 = mydsp_faustpower2_f(2.0 * self.fRec21[0] + -1.0);
			self.fVec15[0] = fTemp68;
			let mut fTemp69: F32 = fTemp5 * (fTemp68 - self.fVec15[1]) / fTemp66;
			self.fVec16[(self.IOTA0 & 4095) as usize] = fTemp69;
			let mut fTemp70: F32 = F32::max(0.0, F32::min(2047.0, self.fConst4 / fTemp65));
			let mut iTemp71: i32 = ((fTemp70) as i32);
			let mut fTemp72: F32 = F32::floor(fTemp70);
			self.fRec20[0] = 0.999 * self.fRec20[1] + self.fConst5 * (fTemp69 - self.fVec16[((i32::wrapping_sub(self.IOTA0, iTemp71)) & 4095) as usize] * (fTemp72 + (1.0 - fTemp70)) - (fTemp70 - fTemp72) * self.fVec16[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp71, 1))) & 4095) as usize]);
			let mut fTemp73: F32 = unsafe { ftbl0mydspSIG0[(((16.11811 * self.fRec17[0]) as i32)) as usize] };
			let mut fTemp74: F32 = F32::max(fTemp73, 23.44895);
			let mut fTemp75: F32 = F32::max(2e+01, F32::abs(fTemp74));
			let mut fTemp76: F32 = self.fRec23[1] + self.fConst3 * fTemp75;
			self.fRec23[0] = fTemp76 - F32::floor(fTemp76);
			let mut fTemp77: F32 = mydsp_faustpower2_f(2.0 * self.fRec23[0] + -1.0);
			self.fVec17[0] = fTemp77;
			let mut fTemp78: F32 = fTemp5 * (fTemp77 - self.fVec17[1]) / fTemp75;
			self.fVec18[(self.IOTA0 & 4095) as usize] = fTemp78;
			let mut fTemp79: F32 = F32::max(0.0, F32::min(2047.0, self.fConst4 / fTemp74));
			let mut iTemp80: i32 = ((fTemp79) as i32);
			let mut fTemp81: F32 = F32::floor(fTemp79);
			self.fRec22[0] = 0.999 * self.fRec22[1] - self.fConst5 * (self.fVec18[((i32::wrapping_sub(self.IOTA0, iTemp80)) & 4095) as usize] * (fTemp81 + (1.0 - fTemp79)) - fTemp78 + (fTemp79 - fTemp81) * self.fVec18[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp80, 1))) & 4095) as usize]);
			self.fRec24[0] = fSlow6 + self.fConst2 * self.fRec24[1];
			self.fRec25[0] = fSlow7 + self.fConst2 * self.fRec25[1];
			let mut fTemp82: F32 = unsafe { ftbl0mydspSIG0[(((16.11811 * (self.fRec25[0] - fSlow0)) as i32)) as usize] };
			let mut fTemp83: F32 = F32::max(fTemp82, 23.44895);
			let mut fTemp84: F32 = F32::max(2e+01, F32::abs(fTemp83));
			let mut fTemp85: F32 = self.fRec27[1] + self.fConst3 * fTemp84;
			self.fRec27[0] = fTemp85 - F32::floor(fTemp85);
			let mut fTemp86: F32 = mydsp_faustpower2_f(2.0 * self.fRec27[0] + -1.0);
			self.fVec19[0] = fTemp86;
			let mut fTemp87: F32 = fTemp5 * (fTemp86 - self.fVec19[1]) / fTemp84;
			self.fVec20[(self.IOTA0 & 4095) as usize] = fTemp87;
			let mut fTemp88: F32 = F32::max(0.0, F32::min(2047.0, self.fConst4 / fTemp83));
			let mut iTemp89: i32 = ((fTemp88) as i32);
			let mut fTemp90: F32 = F32::floor(fTemp88);
			self.fRec26[0] = 0.999 * self.fRec26[1] + self.fConst5 * (fTemp87 - self.fVec20[((i32::wrapping_sub(self.IOTA0, iTemp89)) & 4095) as usize] * (fTemp90 + (1.0 - fTemp88)) - (fTemp88 - fTemp90) * self.fVec20[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp89, 1))) & 4095) as usize]);
			let mut fTemp91: F32 = unsafe { ftbl0mydspSIG0[(((16.11811 * (fSlow0 + self.fRec25[0])) as i32)) as usize] };
			let mut fTemp92: F32 = F32::max(fTemp91, 23.44895);
			let mut fTemp93: F32 = F32::max(2e+01, F32::abs(fTemp92));
			let mut fTemp94: F32 = self.fRec29[1] + self.fConst3 * fTemp93;
			self.fRec29[0] = fTemp94 - F32::floor(fTemp94);
			let mut fTemp95: F32 = mydsp_faustpower2_f(2.0 * self.fRec29[0] + -1.0);
			self.fVec21[0] = fTemp95;
			let mut fTemp96: F32 = fTemp5 * (fTemp95 - self.fVec21[1]) / fTemp93;
			self.fVec22[(self.IOTA0 & 4095) as usize] = fTemp96;
			let mut fTemp97: F32 = F32::max(0.0, F32::min(2047.0, self.fConst4 / fTemp92));
			let mut iTemp98: i32 = ((fTemp97) as i32);
			let mut fTemp99: F32 = F32::floor(fTemp97);
			self.fRec28[0] = 0.999 * self.fRec28[1] - self.fConst5 * (self.fVec22[((i32::wrapping_sub(self.IOTA0, iTemp98)) & 4095) as usize] * (fTemp99 + (1.0 - fTemp97)) - fTemp96 + (fTemp97 - fTemp99) * self.fVec22[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp98, 1))) & 4095) as usize]);
			let mut fTemp100: F32 = unsafe { ftbl0mydspSIG0[(((16.11811 * self.fRec25[0]) as i32)) as usize] };
			let mut fTemp101: F32 = F32::max(fTemp100, 23.44895);
			let mut fTemp102: F32 = F32::max(2e+01, F32::abs(fTemp101));
			let mut fTemp103: F32 = self.fRec31[1] + self.fConst3 * fTemp102;
			self.fRec31[0] = fTemp103 - F32::floor(fTemp103);
			let mut fTemp104: F32 = mydsp_faustpower2_f(2.0 * self.fRec31[0] + -1.0);
			self.fVec23[0] = fTemp104;
			let mut fTemp105: F32 = fTemp5 * (fTemp104 - self.fVec23[1]) / fTemp102;
			self.fVec24[(self.IOTA0 & 4095) as usize] = fTemp105;
			let mut fTemp106: F32 = F32::max(0.0, F32::min(2047.0, self.fConst4 / fTemp101));
			let mut iTemp107: i32 = ((fTemp106) as i32);
			let mut fTemp108: F32 = F32::floor(fTemp106);
			self.fRec30[0] = 0.999 * self.fRec30[1] + self.fConst5 * (fTemp105 - self.fVec24[((i32::wrapping_sub(self.IOTA0, iTemp107)) & 4095) as usize] * (fTemp108 + (1.0 - fTemp106)) - (fTemp106 - fTemp108) * self.fVec24[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp107, 1))) & 4095) as usize]);
			self.fRec32[0] = fSlow8 + self.fConst2 * self.fRec32[1];
			self.fRec33[0] = fSlow9 + self.fConst2 * self.fRec33[1];
			let mut fTemp109: F32 = self.fConst6 * self.fRec33[0] * (self.fRec32[0] * (self.fRec30[0] * fTemp100 + self.fRec28[0] * fTemp91 + self.fRec26[0] * fTemp82) + self.fRec24[0] * (self.fRec22[0] * fTemp73 + self.fRec20[0] * fTemp64 + self.fRec18[0] * fTemp55) + self.fRec16[0] * (self.fRec14[0] * fTemp46 + self.fRec12[0] * fTemp37 + self.fRec10[0] * fTemp28) + self.fRec8[0] * (self.fRec6[0] * fTemp19 + self.fRec4[0] * fTemp10 + self.fRec2[0] * fTemp0));
			self.fRec34[0] = fSlow10 + self.fConst2 * self.fRec34[1];
			self.fRec36[0] = fSlow11 + self.fConst2 * self.fRec36[1];
			self.fRec37[0] = fSlow12 + self.fConst2 * self.fRec37[1];
			let mut fTemp110: F32 = unsafe { ftbl0mydspSIG0[(((16.11811 * (self.fRec37[0] + self.fRec36[0])) as i32)) as usize] };
			let mut fTemp111: F32 = 1.0 / F32::tan(self.fConst7 * fTemp110);
			let mut fRec52: F32 = -1.0 * 0.9973053 * (0.9 * self.fRec53[2] + 0.05 * (self.fRec53[1] + self.fRec53[3]));
			let mut fTemp112: F32 = self.fConst9 * (3.4e+02 / fTemp110 + -0.11);
			let mut fTemp113: F32 = fTemp112 + -1.499995;
			let mut iTemp114: i32 = ((fTemp113) as i32);
			let mut iTemp115: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, i32::wrapping_add(iTemp114, 4))) as F32))) as i32);
			let mut iTemp116: i32 = i32::wrapping_add(iTemp115, 1);
			let mut fTemp117: F32 = F32::floor(fTemp113);
			let mut fTemp118: F32 = fTemp112 + (-3.0 - fTemp117);
			let mut fTemp119: F32 = fTemp112 + (-2.0 - fTemp117);
			let mut fTemp120: F32 = fTemp112 + (-1.0 - fTemp117);
			let mut fTemp121: F32 = fTemp120 * fTemp119;
			let mut fTemp122: F32 = fTemp121 * fTemp118;
			let mut fTemp123: F32 = fTemp112 + (-4.0 - fTemp117);
			let mut fTemp124: F32 = 0.0 - fTemp123;
			let mut iTemp125: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, i32::wrapping_add(iTemp114, 3))) as F32))) as i32);
			let mut iTemp126: i32 = i32::wrapping_add(iTemp125, 1);
			let mut fTemp127: F32 = 0.0 - 0.5 * fTemp123;
			let mut fTemp128: F32 = 0.0 - fTemp118;
			let mut iTemp129: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, i32::wrapping_add(iTemp114, 2))) as F32))) as i32);
			let mut iTemp130: i32 = i32::wrapping_add(iTemp129, 1);
			let mut fTemp131: F32 = 0.0 - 0.33333334 * fTemp123;
			let mut fTemp132: F32 = 0.0 - 0.5 * fTemp118;
			let mut fTemp133: F32 = 0.0 - fTemp119;
			let mut iTemp134: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, i32::wrapping_add(iTemp114, 1))) as F32))) as i32);
			let mut iTemp135: i32 = i32::wrapping_add(iTemp134, 1);
			let mut fTemp136: F32 = fTemp112 - fTemp117;
			let mut fTemp137: F32 = 0.0 - 0.25 * fTemp123;
			let mut fTemp138: F32 = 0.0 - 0.33333334 * fTemp118;
			let mut fTemp139: F32 = 0.0 - 0.5 * fTemp119;
			let mut fTemp140: F32 = 0.0 - fTemp120;
			let mut iTemp141: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, iTemp114)) as F32))) as i32);
			let mut iTemp142: i32 = i32::wrapping_add(iTemp141, 1);
			self.fRec67[0] = self.fRec44[((i32::wrapping_sub(self.IOTA0, iTemp142)) & 2047) as usize] * fTemp140 * fTemp139 * fTemp138 * fTemp137 + fTemp136 * (self.fRec44[((i32::wrapping_sub(self.IOTA0, iTemp135)) & 2047) as usize] * fTemp133 * fTemp132 * fTemp131 + 0.5 * fTemp120 * self.fRec44[((i32::wrapping_sub(self.IOTA0, iTemp130)) & 2047) as usize] * fTemp128 * fTemp127 + 0.16666667 * fTemp121 * self.fRec44[((i32::wrapping_sub(self.IOTA0, iTemp126)) & 2047) as usize] * fTemp124 + 0.041666668 * fTemp122 * self.fRec44[((i32::wrapping_sub(self.IOTA0, iTemp116)) & 2047) as usize]);
			self.fRec71[0] = 0.05 * self.fRec71[1] + 0.95 * self.fRec67[1];
			let mut fRec68: F32 = self.fRec71[0];
			self.fRec76[0] = self.fConst10 * self.fRec76[1] + self.fConst11 * F32::abs(self.fRec38[1]);
			let mut fRec75: F32 = self.fRec76[0];
			let mut iTemp143: i32 = ((fRec75 > 0.1) as i32);
			self.iVec25[0] = iTemp143;
			self.iRec77[0] = std::cmp::max(i32::wrapping_mul(self.iConst12, ((iTemp143 < self.iVec25[1]) as i32)), i32::wrapping_add(self.iRec77[1], -1));
			let mut fTemp144: F32 = F32::abs(F32::max(((iTemp143) as F32), ((((self.iRec77[0] > 0) as i32)) as F32)));
			let mut fTemp145: F32 = if (((self.fRec73[1] > fTemp144) as i32) as i32 != 0) { self.fConst13 } else { self.fConst10 };
			self.fRec74[0] = self.fRec74[1] * fTemp145 + fTemp144 * (1.0 - fTemp145);
			self.fRec73[0] = self.fRec74[0];
			let mut fTemp146: F32 = 0.005 * self.fRec73[0] * self.fRec38[1];
			self.fRec78[0] = self.fRec42[1];
			self.fRec79[(self.IOTA0 & 2047) as usize] = -1.0 * 0.9973053 * (0.9 * self.fRec78[2] + 0.05 * (self.fRec78[1] + self.fRec78[3]));
			let mut fTemp147: F32 = fTemp121 * fTemp124;
			let mut fTemp148: F32 = fTemp120 * fTemp128 * fTemp127;
			let mut fTemp149: F32 = fTemp133 * fTemp132 * fTemp131;
			let mut fTemp150: F32 = fTemp140 * fTemp139 * fTemp138 * fTemp137;
			self.fVec26[0] = fTemp150 * self.fRec79[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp141, 2))) & 2047) as usize] + fTemp136 * (fTemp149 * self.fRec79[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp134, 2))) & 2047) as usize] + 0.5 * fTemp148 * self.fRec79[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp129, 2))) & 2047) as usize] + 0.16666667 * fTemp147 * self.fRec79[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp125, 2))) & 2047) as usize] + 0.041666668 * fTemp122 * self.fRec79[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp115, 2))) & 2047) as usize]);
			let mut fTemp151: F32 = F32::tan(self.fConst14 * fTemp110);
			let mut fTemp152: F32 = 1.0 / fTemp151;
			let mut fTemp153: F32 = (fTemp152 + 1.4142135) / fTemp151 + 1.0;
			self.fVec27[0] = fSlow13;
			self.iRec80[0] = i32::wrapping_add(i32::wrapping_mul(i32::wrapping_add(self.iRec80[1], ((self.iRec80[1] > 0) as i32)), ((fSlow13 <= self.fVec27[1]) as i32)), ((fSlow13 > self.fVec27[1]) as i32));
			let mut fTemp154: F32 = ((self.iRec80[0]) as F32) / F32::max(1.0, self.fConst15 * mydsp_faustpower2_f(1.0 - 0.0005 * fTemp110));
			self.iRec82[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, self.iRec82[1]), 12345);
			let mut fTemp155: F32 = 4.656613e-10 * ((self.iRec82[0]) as F32);
			self.fRec81[0] = fTemp155 - (self.fRec81[2] * ((fTemp152 + -1.4142135) / fTemp151 + 1.0) + 2.0 * self.fRec81[1] * (1.0 - 1.0 / mydsp_faustpower2_f(fTemp151))) / fTemp153;
			let mut fTemp156: F32 = 0.5 * ((self.fRec81[2] + self.fRec81[0] + 2.0 * self.fRec81[1]) * F32::max(0.0, F32::min(fTemp154, 2.0 - fTemp154)) / fTemp153);
			let mut fTemp157: F32 = fTemp156 + self.fVec26[1] + fTemp146;
			self.fVec28[0] = fTemp157;
			self.fRec72[(self.IOTA0 & 2047) as usize] = 0.05 * self.fRec72[((i32::wrapping_sub(self.IOTA0, 1)) & 2047) as usize] + 0.95 * self.fVec28[2];
			let mut fRec69: F32 = fTemp150 * self.fRec72[((i32::wrapping_sub(self.IOTA0, iTemp141)) & 2047) as usize] + fTemp136 * (fTemp149 * self.fRec72[((i32::wrapping_sub(self.IOTA0, iTemp134)) & 2047) as usize] + 0.5 * fTemp148 * self.fRec72[((i32::wrapping_sub(self.IOTA0, iTemp129)) & 2047) as usize] + 0.16666667 * fTemp147 * self.fRec72[((i32::wrapping_sub(self.IOTA0, iTemp125)) & 2047) as usize] + 0.041666668 * fTemp122 * self.fRec72[((i32::wrapping_sub(self.IOTA0, iTemp115)) & 2047) as usize]);
			let mut fRec70: F32 = self.fVec28[1] + self.fRec63[1];
			self.fRec63[0] = fRec68;
			let mut fRec64: F32 = self.fRec63[1];
			let mut fRec65: F32 = fRec69;
			let mut fRec66: F32 = fRec70;
			self.fRec59[0] = fRec64;
			let mut fRec60: F32 = fTemp146 + fTemp156 + self.fRec59[1];
			let mut fRec61: F32 = fRec65;
			let mut fRec62: F32 = fRec66;
			self.fRec55[(self.IOTA0 & 2047) as usize] = fRec60;
			let mut fRec56: F32 = fTemp150 * self.fRec55[((i32::wrapping_sub(self.IOTA0, iTemp142)) & 2047) as usize] + fTemp136 * (fTemp149 * self.fRec55[((i32::wrapping_sub(self.IOTA0, iTemp135)) & 2047) as usize] + 0.5 * fTemp148 * self.fRec55[((i32::wrapping_sub(self.IOTA0, iTemp130)) & 2047) as usize] + 0.16666667 * fTemp147 * self.fRec55[((i32::wrapping_sub(self.IOTA0, iTemp126)) & 2047) as usize] + 0.041666668 * fTemp122 * self.fRec55[((i32::wrapping_sub(self.IOTA0, iTemp116)) & 2047) as usize]);
			self.fRec57[0] = fRec61;
			let mut fRec58: F32 = fRec62;
			self.fRec53[0] = fSlow14 * self.fRec57[1];
			let mut fRec54: F32 = fRec58;
			self.fRec48[0] = fRec52;
			let mut fRec49: F32 = fSlow14 * self.fRec48[1];
			let mut fRec50: F32 = self.fRec53[0];
			let mut fRec51: F32 = fRec54;
			self.fRec44[(self.IOTA0 & 2047) as usize] = fRec49;
			let mut fRec45: F32 = fRec56;
			let mut fRec46: F32 = fRec50;
			let mut fRec47: F32 = fRec51;
			self.fRec42[0] = fRec45;
			let mut fRec43: F32 = fRec47;
			let mut fTemp158: F32 = F32::abs(fRec43);
			let mut fTemp159: F32 = if (((self.fRec40[1] > fTemp158) as i32) as i32 != 0) { self.fConst16 } else { 0.0 };
			self.fRec41[0] = self.fRec41[1] * fTemp159 + fTemp158 * (1.0 - fTemp159);
			self.fRec40[0] = self.fRec41[0];
			let mut fRec39: F32 = 0.0 - 0.95 * F32::max(2e+01 * F32::log10(self.fRec40[0]) + 1e+01, 0.0);
			self.fRec38[0] = fRec43 * F32::powf(1e+01, 0.05 * fRec39);
			self.fRec35[0] = 0.0 - (self.fRec35[1] * (1.0 - fTemp111) - (self.fRec38[0] + self.fRec38[1])) / (fTemp111 + 1.0);
			self.fRec84[0] = fSlow15 + self.fConst2 * self.fRec84[1];
			let mut fTemp160: F32 = unsafe { ftbl0mydspSIG0[(((16.11811 * (self.fRec37[0] + self.fRec84[0])) as i32)) as usize] };
			let mut fTemp161: F32 = 1.0 / F32::tan(self.fConst7 * fTemp160);
			let mut fRec99: F32 = -1.0 * 0.9973053 * (0.9 * self.fRec100[2] + 0.05 * (self.fRec100[1] + self.fRec100[3]));
			let mut fTemp162: F32 = self.fConst9 * (3.4e+02 / fTemp160 + -0.11);
			let mut fTemp163: F32 = fTemp162 + -1.499995;
			let mut iTemp164: i32 = ((fTemp163) as i32);
			let mut iTemp165: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, i32::wrapping_add(iTemp164, 4))) as F32))) as i32);
			let mut iTemp166: i32 = i32::wrapping_add(iTemp165, 1);
			let mut fTemp167: F32 = F32::floor(fTemp163);
			let mut fTemp168: F32 = fTemp162 + (-3.0 - fTemp167);
			let mut fTemp169: F32 = fTemp162 + (-2.0 - fTemp167);
			let mut fTemp170: F32 = fTemp162 + (-1.0 - fTemp167);
			let mut fTemp171: F32 = fTemp170 * fTemp169;
			let mut fTemp172: F32 = fTemp171 * fTemp168;
			let mut fTemp173: F32 = fTemp162 + (-4.0 - fTemp167);
			let mut fTemp174: F32 = 0.0 - fTemp173;
			let mut iTemp175: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, i32::wrapping_add(iTemp164, 3))) as F32))) as i32);
			let mut iTemp176: i32 = i32::wrapping_add(iTemp175, 1);
			let mut fTemp177: F32 = 0.0 - 0.5 * fTemp173;
			let mut fTemp178: F32 = 0.0 - fTemp168;
			let mut iTemp179: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, i32::wrapping_add(iTemp164, 2))) as F32))) as i32);
			let mut iTemp180: i32 = i32::wrapping_add(iTemp179, 1);
			let mut fTemp181: F32 = 0.0 - 0.33333334 * fTemp173;
			let mut fTemp182: F32 = 0.0 - 0.5 * fTemp168;
			let mut fTemp183: F32 = 0.0 - fTemp169;
			let mut iTemp184: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, i32::wrapping_add(iTemp164, 1))) as F32))) as i32);
			let mut iTemp185: i32 = i32::wrapping_add(iTemp184, 1);
			let mut fTemp186: F32 = fTemp162 - fTemp167;
			let mut fTemp187: F32 = 0.0 - 0.25 * fTemp173;
			let mut fTemp188: F32 = 0.0 - 0.33333334 * fTemp168;
			let mut fTemp189: F32 = 0.0 - 0.5 * fTemp169;
			let mut fTemp190: F32 = 0.0 - fTemp170;
			let mut iTemp191: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, iTemp164)) as F32))) as i32);
			let mut iTemp192: i32 = i32::wrapping_add(iTemp191, 1);
			self.fRec114[0] = self.fRec91[((i32::wrapping_sub(self.IOTA0, iTemp192)) & 2047) as usize] * fTemp190 * fTemp189 * fTemp188 * fTemp187 + fTemp186 * (self.fRec91[((i32::wrapping_sub(self.IOTA0, iTemp185)) & 2047) as usize] * fTemp183 * fTemp182 * fTemp181 + 0.5 * fTemp170 * self.fRec91[((i32::wrapping_sub(self.IOTA0, iTemp180)) & 2047) as usize] * fTemp178 * fTemp177 + 0.16666667 * fTemp171 * self.fRec91[((i32::wrapping_sub(self.IOTA0, iTemp176)) & 2047) as usize] * fTemp174 + 0.041666668 * fTemp172 * self.fRec91[((i32::wrapping_sub(self.IOTA0, iTemp166)) & 2047) as usize]);
			self.fRec118[0] = 0.05 * self.fRec118[1] + 0.95 * self.fRec114[1];
			let mut fRec115: F32 = self.fRec118[0];
			self.fRec123[0] = self.fConst10 * self.fRec123[1] + self.fConst11 * F32::abs(self.fRec85[1]);
			let mut fRec122: F32 = self.fRec123[0];
			let mut iTemp193: i32 = ((fRec122 > 0.1) as i32);
			self.iVec29[0] = iTemp193;
			self.iRec124[0] = std::cmp::max(i32::wrapping_mul(self.iConst12, ((iTemp193 < self.iVec29[1]) as i32)), i32::wrapping_add(self.iRec124[1], -1));
			let mut fTemp194: F32 = F32::abs(F32::max(((iTemp193) as F32), ((((self.iRec124[0] > 0) as i32)) as F32)));
			let mut fTemp195: F32 = if (((self.fRec120[1] > fTemp194) as i32) as i32 != 0) { self.fConst13 } else { self.fConst10 };
			self.fRec121[0] = self.fRec121[1] * fTemp195 + fTemp194 * (1.0 - fTemp195);
			self.fRec120[0] = self.fRec121[0];
			let mut fTemp196: F32 = 0.005 * self.fRec120[0] * self.fRec85[1];
			self.fRec125[0] = self.fRec89[1];
			self.fRec126[(self.IOTA0 & 2047) as usize] = -1.0 * 0.9973053 * (0.9 * self.fRec125[2] + 0.05 * (self.fRec125[1] + self.fRec125[3]));
			let mut fTemp197: F32 = fTemp171 * fTemp174;
			let mut fTemp198: F32 = fTemp170 * fTemp178 * fTemp177;
			let mut fTemp199: F32 = fTemp183 * fTemp182 * fTemp181;
			let mut fTemp200: F32 = fTemp190 * fTemp189 * fTemp188 * fTemp187;
			self.fVec30[0] = fTemp200 * self.fRec126[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp191, 2))) & 2047) as usize] + fTemp186 * (fTemp199 * self.fRec126[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp184, 2))) & 2047) as usize] + 0.5 * fTemp198 * self.fRec126[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp179, 2))) & 2047) as usize] + 0.16666667 * fTemp197 * self.fRec126[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp175, 2))) & 2047) as usize] + 0.041666668 * fTemp172 * self.fRec126[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp165, 2))) & 2047) as usize]);
			let mut fTemp201: F32 = F32::tan(self.fConst14 * fTemp160);
			let mut fTemp202: F32 = 1.0 / fTemp201;
			let mut fTemp203: F32 = (fTemp202 + 1.4142135) / fTemp201 + 1.0;
			self.fVec31[0] = fSlow16;
			self.iRec127[0] = i32::wrapping_add(i32::wrapping_mul(i32::wrapping_add(self.iRec127[1], ((self.iRec127[1] > 0) as i32)), ((fSlow16 <= self.fVec31[1]) as i32)), ((fSlow16 > self.fVec31[1]) as i32));
			let mut fTemp204: F32 = ((self.iRec127[0]) as F32) / F32::max(1.0, self.fConst15 * mydsp_faustpower2_f(1.0 - 0.0005 * fTemp160));
			self.fRec128[0] = fTemp155 - (self.fRec128[2] * ((fTemp202 + -1.4142135) / fTemp201 + 1.0) + 2.0 * self.fRec128[1] * (1.0 - 1.0 / mydsp_faustpower2_f(fTemp201))) / fTemp203;
			let mut fTemp205: F32 = 0.5 * ((self.fRec128[2] + self.fRec128[0] + 2.0 * self.fRec128[1]) * F32::max(0.0, F32::min(fTemp204, 2.0 - fTemp204)) / fTemp203);
			let mut fTemp206: F32 = fTemp205 + self.fVec30[1] + fTemp196;
			self.fVec32[0] = fTemp206;
			self.fRec119[(self.IOTA0 & 2047) as usize] = 0.05 * self.fRec119[((i32::wrapping_sub(self.IOTA0, 1)) & 2047) as usize] + 0.95 * self.fVec32[2];
			let mut fRec116: F32 = fTemp200 * self.fRec119[((i32::wrapping_sub(self.IOTA0, iTemp191)) & 2047) as usize] + fTemp186 * (fTemp199 * self.fRec119[((i32::wrapping_sub(self.IOTA0, iTemp184)) & 2047) as usize] + 0.5 * fTemp198 * self.fRec119[((i32::wrapping_sub(self.IOTA0, iTemp179)) & 2047) as usize] + 0.16666667 * fTemp197 * self.fRec119[((i32::wrapping_sub(self.IOTA0, iTemp175)) & 2047) as usize] + 0.041666668 * fTemp172 * self.fRec119[((i32::wrapping_sub(self.IOTA0, iTemp165)) & 2047) as usize]);
			let mut fRec117: F32 = self.fVec32[1] + self.fRec110[1];
			self.fRec110[0] = fRec115;
			let mut fRec111: F32 = self.fRec110[1];
			let mut fRec112: F32 = fRec116;
			let mut fRec113: F32 = fRec117;
			self.fRec106[0] = fRec111;
			let mut fRec107: F32 = fTemp196 + fTemp205 + self.fRec106[1];
			let mut fRec108: F32 = fRec112;
			let mut fRec109: F32 = fRec113;
			self.fRec102[(self.IOTA0 & 2047) as usize] = fRec107;
			let mut fRec103: F32 = fTemp200 * self.fRec102[((i32::wrapping_sub(self.IOTA0, iTemp192)) & 2047) as usize] + fTemp186 * (fTemp199 * self.fRec102[((i32::wrapping_sub(self.IOTA0, iTemp185)) & 2047) as usize] + 0.5 * fTemp198 * self.fRec102[((i32::wrapping_sub(self.IOTA0, iTemp180)) & 2047) as usize] + 0.16666667 * fTemp197 * self.fRec102[((i32::wrapping_sub(self.IOTA0, iTemp176)) & 2047) as usize] + 0.041666668 * fTemp172 * self.fRec102[((i32::wrapping_sub(self.IOTA0, iTemp166)) & 2047) as usize]);
			self.fRec104[0] = fRec108;
			let mut fRec105: F32 = fRec109;
			self.fRec100[0] = fSlow14 * self.fRec104[1];
			let mut fRec101: F32 = fRec105;
			self.fRec95[0] = fRec99;
			let mut fRec96: F32 = fSlow14 * self.fRec95[1];
			let mut fRec97: F32 = self.fRec100[0];
			let mut fRec98: F32 = fRec101;
			self.fRec91[(self.IOTA0 & 2047) as usize] = fRec96;
			let mut fRec92: F32 = fRec103;
			let mut fRec93: F32 = fRec97;
			let mut fRec94: F32 = fRec98;
			self.fRec89[0] = fRec92;
			let mut fRec90: F32 = fRec94;
			let mut fTemp207: F32 = F32::abs(fRec90);
			let mut fTemp208: F32 = if (((self.fRec87[1] > fTemp207) as i32) as i32 != 0) { self.fConst16 } else { 0.0 };
			self.fRec88[0] = self.fRec88[1] * fTemp208 + fTemp207 * (1.0 - fTemp208);
			self.fRec87[0] = self.fRec88[0];
			let mut fRec86: F32 = 0.0 - 0.95 * F32::max(2e+01 * F32::log10(self.fRec87[0]) + 1e+01, 0.0);
			self.fRec85[0] = fRec90 * F32::powf(1e+01, 0.05 * fRec86);
			self.fRec83[0] = 0.0 - (self.fRec83[1] * (1.0 - fTemp161) - (self.fRec85[0] + self.fRec85[1])) / (fTemp161 + 1.0);
			self.fRec130[0] = fSlow17 + self.fConst2 * self.fRec130[1];
			let mut fTemp209: F32 = unsafe { ftbl0mydspSIG0[(((16.11811 * (self.fRec37[0] + self.fRec130[0])) as i32)) as usize] };
			let mut fTemp210: F32 = 1.0 / F32::tan(self.fConst7 * fTemp209);
			let mut fRec145: F32 = -1.0 * 0.9973053 * (0.9 * self.fRec146[2] + 0.05 * (self.fRec146[1] + self.fRec146[3]));
			let mut fTemp211: F32 = self.fConst9 * (3.4e+02 / fTemp209 + -0.11);
			let mut fTemp212: F32 = fTemp211 + -1.499995;
			let mut iTemp213: i32 = ((fTemp212) as i32);
			let mut iTemp214: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, i32::wrapping_add(iTemp213, 4))) as F32))) as i32);
			let mut iTemp215: i32 = i32::wrapping_add(iTemp214, 1);
			let mut fTemp216: F32 = F32::floor(fTemp212);
			let mut fTemp217: F32 = fTemp211 + (-3.0 - fTemp216);
			let mut fTemp218: F32 = fTemp211 + (-2.0 - fTemp216);
			let mut fTemp219: F32 = fTemp211 + (-1.0 - fTemp216);
			let mut fTemp220: F32 = fTemp219 * fTemp218;
			let mut fTemp221: F32 = fTemp220 * fTemp217;
			let mut fTemp222: F32 = fTemp211 + (-4.0 - fTemp216);
			let mut fTemp223: F32 = 0.0 - fTemp222;
			let mut iTemp224: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, i32::wrapping_add(iTemp213, 3))) as F32))) as i32);
			let mut iTemp225: i32 = i32::wrapping_add(iTemp224, 1);
			let mut fTemp226: F32 = 0.0 - 0.5 * fTemp222;
			let mut fTemp227: F32 = 0.0 - fTemp217;
			let mut iTemp228: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, i32::wrapping_add(iTemp213, 2))) as F32))) as i32);
			let mut iTemp229: i32 = i32::wrapping_add(iTemp228, 1);
			let mut fTemp230: F32 = 0.0 - 0.33333334 * fTemp222;
			let mut fTemp231: F32 = 0.0 - 0.5 * fTemp217;
			let mut fTemp232: F32 = 0.0 - fTemp218;
			let mut iTemp233: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, i32::wrapping_add(iTemp213, 1))) as F32))) as i32);
			let mut iTemp234: i32 = i32::wrapping_add(iTemp233, 1);
			let mut fTemp235: F32 = fTemp211 - fTemp216;
			let mut fTemp236: F32 = 0.0 - 0.25 * fTemp222;
			let mut fTemp237: F32 = 0.0 - 0.33333334 * fTemp217;
			let mut fTemp238: F32 = 0.0 - 0.5 * fTemp218;
			let mut fTemp239: F32 = 0.0 - fTemp219;
			let mut iTemp240: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, iTemp213)) as F32))) as i32);
			let mut iTemp241: i32 = i32::wrapping_add(iTemp240, 1);
			self.fRec160[0] = self.fRec137[((i32::wrapping_sub(self.IOTA0, iTemp241)) & 2047) as usize] * fTemp239 * fTemp238 * fTemp237 * fTemp236 + fTemp235 * (self.fRec137[((i32::wrapping_sub(self.IOTA0, iTemp234)) & 2047) as usize] * fTemp232 * fTemp231 * fTemp230 + 0.5 * fTemp219 * self.fRec137[((i32::wrapping_sub(self.IOTA0, iTemp229)) & 2047) as usize] * fTemp227 * fTemp226 + 0.16666667 * fTemp220 * self.fRec137[((i32::wrapping_sub(self.IOTA0, iTemp225)) & 2047) as usize] * fTemp223 + 0.041666668 * fTemp221 * self.fRec137[((i32::wrapping_sub(self.IOTA0, iTemp215)) & 2047) as usize]);
			self.fRec164[0] = 0.05 * self.fRec164[1] + 0.95 * self.fRec160[1];
			let mut fRec161: F32 = self.fRec164[0];
			self.fRec169[0] = self.fConst10 * self.fRec169[1] + self.fConst11 * F32::abs(self.fRec131[1]);
			let mut fRec168: F32 = self.fRec169[0];
			let mut iTemp242: i32 = ((fRec168 > 0.1) as i32);
			self.iVec33[0] = iTemp242;
			self.iRec170[0] = std::cmp::max(i32::wrapping_mul(self.iConst12, ((iTemp242 < self.iVec33[1]) as i32)), i32::wrapping_add(self.iRec170[1], -1));
			let mut fTemp243: F32 = F32::abs(F32::max(((iTemp242) as F32), ((((self.iRec170[0] > 0) as i32)) as F32)));
			let mut fTemp244: F32 = if (((self.fRec166[1] > fTemp243) as i32) as i32 != 0) { self.fConst13 } else { self.fConst10 };
			self.fRec167[0] = self.fRec167[1] * fTemp244 + fTemp243 * (1.0 - fTemp244);
			self.fRec166[0] = self.fRec167[0];
			let mut fTemp245: F32 = 0.005 * self.fRec166[0] * self.fRec131[1];
			self.fRec171[0] = self.fRec135[1];
			self.fRec172[(self.IOTA0 & 2047) as usize] = -1.0 * 0.9973053 * (0.9 * self.fRec171[2] + 0.05 * (self.fRec171[1] + self.fRec171[3]));
			let mut fTemp246: F32 = fTemp220 * fTemp223;
			let mut fTemp247: F32 = fTemp219 * fTemp227 * fTemp226;
			let mut fTemp248: F32 = fTemp232 * fTemp231 * fTemp230;
			let mut fTemp249: F32 = fTemp239 * fTemp238 * fTemp237 * fTemp236;
			self.fVec34[0] = fTemp249 * self.fRec172[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp240, 2))) & 2047) as usize] + fTemp235 * (fTemp248 * self.fRec172[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp233, 2))) & 2047) as usize] + 0.5 * fTemp247 * self.fRec172[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp228, 2))) & 2047) as usize] + 0.16666667 * fTemp246 * self.fRec172[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp224, 2))) & 2047) as usize] + 0.041666668 * fTemp221 * self.fRec172[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp214, 2))) & 2047) as usize]);
			let mut fTemp250: F32 = F32::tan(self.fConst14 * fTemp209);
			let mut fTemp251: F32 = 1.0 / fTemp250;
			let mut fTemp252: F32 = (fTemp251 + 1.4142135) / fTemp250 + 1.0;
			self.fVec35[0] = fSlow18;
			self.iRec173[0] = i32::wrapping_add(i32::wrapping_mul(i32::wrapping_add(self.iRec173[1], ((self.iRec173[1] > 0) as i32)), ((fSlow18 <= self.fVec35[1]) as i32)), ((fSlow18 > self.fVec35[1]) as i32));
			let mut fTemp253: F32 = ((self.iRec173[0]) as F32) / F32::max(1.0, self.fConst15 * mydsp_faustpower2_f(1.0 - 0.0005 * fTemp209));
			self.fRec174[0] = fTemp155 - (self.fRec174[2] * ((fTemp251 + -1.4142135) / fTemp250 + 1.0) + 2.0 * self.fRec174[1] * (1.0 - 1.0 / mydsp_faustpower2_f(fTemp250))) / fTemp252;
			let mut fTemp254: F32 = 0.5 * ((self.fRec174[2] + self.fRec174[0] + 2.0 * self.fRec174[1]) * F32::max(0.0, F32::min(fTemp253, 2.0 - fTemp253)) / fTemp252);
			let mut fTemp255: F32 = fTemp254 + self.fVec34[1] + fTemp245;
			self.fVec36[0] = fTemp255;
			self.fRec165[(self.IOTA0 & 2047) as usize] = 0.05 * self.fRec165[((i32::wrapping_sub(self.IOTA0, 1)) & 2047) as usize] + 0.95 * self.fVec36[2];
			let mut fRec162: F32 = fTemp249 * self.fRec165[((i32::wrapping_sub(self.IOTA0, iTemp240)) & 2047) as usize] + fTemp235 * (fTemp248 * self.fRec165[((i32::wrapping_sub(self.IOTA0, iTemp233)) & 2047) as usize] + 0.5 * fTemp247 * self.fRec165[((i32::wrapping_sub(self.IOTA0, iTemp228)) & 2047) as usize] + 0.16666667 * fTemp246 * self.fRec165[((i32::wrapping_sub(self.IOTA0, iTemp224)) & 2047) as usize] + 0.041666668 * fTemp221 * self.fRec165[((i32::wrapping_sub(self.IOTA0, iTemp214)) & 2047) as usize]);
			let mut fRec163: F32 = self.fVec36[1] + self.fRec156[1];
			self.fRec156[0] = fRec161;
			let mut fRec157: F32 = self.fRec156[1];
			let mut fRec158: F32 = fRec162;
			let mut fRec159: F32 = fRec163;
			self.fRec152[0] = fRec157;
			let mut fRec153: F32 = fTemp245 + fTemp254 + self.fRec152[1];
			let mut fRec154: F32 = fRec158;
			let mut fRec155: F32 = fRec159;
			self.fRec148[(self.IOTA0 & 2047) as usize] = fRec153;
			let mut fRec149: F32 = fTemp249 * self.fRec148[((i32::wrapping_sub(self.IOTA0, iTemp241)) & 2047) as usize] + fTemp235 * (fTemp248 * self.fRec148[((i32::wrapping_sub(self.IOTA0, iTemp234)) & 2047) as usize] + 0.5 * fTemp247 * self.fRec148[((i32::wrapping_sub(self.IOTA0, iTemp229)) & 2047) as usize] + 0.16666667 * fTemp246 * self.fRec148[((i32::wrapping_sub(self.IOTA0, iTemp225)) & 2047) as usize] + 0.041666668 * fTemp221 * self.fRec148[((i32::wrapping_sub(self.IOTA0, iTemp215)) & 2047) as usize]);
			self.fRec150[0] = fRec154;
			let mut fRec151: F32 = fRec155;
			self.fRec146[0] = fSlow14 * self.fRec150[1];
			let mut fRec147: F32 = fRec151;
			self.fRec141[0] = fRec145;
			let mut fRec142: F32 = fSlow14 * self.fRec141[1];
			let mut fRec143: F32 = self.fRec146[0];
			let mut fRec144: F32 = fRec147;
			self.fRec137[(self.IOTA0 & 2047) as usize] = fRec142;
			let mut fRec138: F32 = fRec149;
			let mut fRec139: F32 = fRec143;
			let mut fRec140: F32 = fRec144;
			self.fRec135[0] = fRec138;
			let mut fRec136: F32 = fRec140;
			let mut fTemp256: F32 = F32::abs(fRec136);
			let mut fTemp257: F32 = if (((self.fRec133[1] > fTemp256) as i32) as i32 != 0) { self.fConst16 } else { 0.0 };
			self.fRec134[0] = self.fRec134[1] * fTemp257 + fTemp256 * (1.0 - fTemp257);
			self.fRec133[0] = self.fRec134[0];
			let mut fRec132: F32 = 0.0 - 0.95 * F32::max(2e+01 * F32::log10(self.fRec133[0]) + 1e+01, 0.0);
			self.fRec131[0] = fRec136 * F32::powf(1e+01, 0.05 * fRec132);
			self.fRec129[0] = 0.0 - (self.fRec129[1] * (1.0 - fTemp210) - (self.fRec131[0] + self.fRec131[1])) / (fTemp210 + 1.0);
			self.fRec176[0] = fSlow19 + self.fConst2 * self.fRec176[1];
			let mut fTemp258: F32 = unsafe { ftbl0mydspSIG0[(((16.11811 * (self.fRec176[0] + self.fRec37[0])) as i32)) as usize] };
			let mut fTemp259: F32 = 1.0 / F32::tan(self.fConst7 * fTemp258);
			let mut fRec191: F32 = -1.0 * 0.9973053 * (0.9 * self.fRec192[2] + 0.05 * (self.fRec192[1] + self.fRec192[3]));
			let mut fTemp260: F32 = self.fConst9 * (3.4e+02 / fTemp258 + -0.11);
			let mut fTemp261: F32 = fTemp260 + -1.499995;
			let mut iTemp262: i32 = ((fTemp261) as i32);
			let mut iTemp263: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, i32::wrapping_add(iTemp262, 4))) as F32))) as i32);
			let mut iTemp264: i32 = i32::wrapping_add(iTemp263, 1);
			let mut fTemp265: F32 = F32::floor(fTemp261);
			let mut fTemp266: F32 = fTemp260 + (-3.0 - fTemp265);
			let mut fTemp267: F32 = fTemp260 + (-2.0 - fTemp265);
			let mut fTemp268: F32 = fTemp260 + (-1.0 - fTemp265);
			let mut fTemp269: F32 = fTemp268 * fTemp267;
			let mut fTemp270: F32 = fTemp269 * fTemp266;
			let mut fTemp271: F32 = fTemp260 + (-4.0 - fTemp265);
			let mut fTemp272: F32 = 0.0 - fTemp271;
			let mut iTemp273: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, i32::wrapping_add(iTemp262, 3))) as F32))) as i32);
			let mut iTemp274: i32 = i32::wrapping_add(iTemp273, 1);
			let mut fTemp275: F32 = 0.0 - 0.5 * fTemp271;
			let mut fTemp276: F32 = 0.0 - fTemp266;
			let mut iTemp277: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, i32::wrapping_add(iTemp262, 2))) as F32))) as i32);
			let mut iTemp278: i32 = i32::wrapping_add(iTemp277, 1);
			let mut fTemp279: F32 = 0.0 - 0.33333334 * fTemp271;
			let mut fTemp280: F32 = 0.0 - 0.5 * fTemp266;
			let mut fTemp281: F32 = 0.0 - fTemp267;
			let mut iTemp282: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, i32::wrapping_add(iTemp262, 1))) as F32))) as i32);
			let mut iTemp283: i32 = i32::wrapping_add(iTemp282, 1);
			let mut fTemp284: F32 = fTemp260 - fTemp265;
			let mut fTemp285: F32 = 0.0 - 0.25 * fTemp271;
			let mut fTemp286: F32 = 0.0 - 0.33333334 * fTemp266;
			let mut fTemp287: F32 = 0.0 - 0.5 * fTemp267;
			let mut fTemp288: F32 = 0.0 - fTemp268;
			let mut iTemp289: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, iTemp262)) as F32))) as i32);
			let mut iTemp290: i32 = i32::wrapping_add(iTemp289, 1);
			self.fRec206[0] = self.fRec183[((i32::wrapping_sub(self.IOTA0, iTemp290)) & 2047) as usize] * fTemp288 * fTemp287 * fTemp286 * fTemp285 + fTemp284 * (self.fRec183[((i32::wrapping_sub(self.IOTA0, iTemp283)) & 2047) as usize] * fTemp281 * fTemp280 * fTemp279 + 0.5 * fTemp268 * self.fRec183[((i32::wrapping_sub(self.IOTA0, iTemp278)) & 2047) as usize] * fTemp276 * fTemp275 + 0.16666667 * fTemp269 * self.fRec183[((i32::wrapping_sub(self.IOTA0, iTemp274)) & 2047) as usize] * fTemp272 + 0.041666668 * fTemp270 * self.fRec183[((i32::wrapping_sub(self.IOTA0, iTemp264)) & 2047) as usize]);
			self.fRec210[0] = 0.05 * self.fRec210[1] + 0.95 * self.fRec206[1];
			let mut fRec207: F32 = self.fRec210[0];
			self.fRec215[0] = self.fConst10 * self.fRec215[1] + self.fConst11 * F32::abs(self.fRec177[1]);
			let mut fRec214: F32 = self.fRec215[0];
			let mut iTemp291: i32 = ((fRec214 > 0.1) as i32);
			self.iVec37[0] = iTemp291;
			self.iRec216[0] = std::cmp::max(i32::wrapping_mul(self.iConst12, ((iTemp291 < self.iVec37[1]) as i32)), i32::wrapping_add(self.iRec216[1], -1));
			let mut fTemp292: F32 = F32::abs(F32::max(((iTemp291) as F32), ((((self.iRec216[0] > 0) as i32)) as F32)));
			let mut fTemp293: F32 = if (((self.fRec212[1] > fTemp292) as i32) as i32 != 0) { self.fConst13 } else { self.fConst10 };
			self.fRec213[0] = self.fRec213[1] * fTemp293 + fTemp292 * (1.0 - fTemp293);
			self.fRec212[0] = self.fRec213[0];
			let mut fTemp294: F32 = 0.005 * self.fRec212[0] * self.fRec177[1];
			self.fRec217[0] = self.fRec181[1];
			self.fRec218[(self.IOTA0 & 2047) as usize] = -1.0 * 0.9973053 * (0.9 * self.fRec217[2] + 0.05 * (self.fRec217[1] + self.fRec217[3]));
			let mut fTemp295: F32 = fTemp269 * fTemp272;
			let mut fTemp296: F32 = fTemp268 * fTemp276 * fTemp275;
			let mut fTemp297: F32 = fTemp281 * fTemp280 * fTemp279;
			let mut fTemp298: F32 = fTemp288 * fTemp287 * fTemp286 * fTemp285;
			self.fVec38[0] = fTemp298 * self.fRec218[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp289, 2))) & 2047) as usize] + fTemp284 * (fTemp297 * self.fRec218[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp282, 2))) & 2047) as usize] + 0.5 * fTemp296 * self.fRec218[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp277, 2))) & 2047) as usize] + 0.16666667 * fTemp295 * self.fRec218[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp273, 2))) & 2047) as usize] + 0.041666668 * fTemp270 * self.fRec218[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp263, 2))) & 2047) as usize]);
			let mut fTemp299: F32 = F32::tan(self.fConst14 * fTemp258);
			let mut fTemp300: F32 = 1.0 / fTemp299;
			let mut fTemp301: F32 = (fTemp300 + 1.4142135) / fTemp299 + 1.0;
			self.fVec39[0] = fSlow20;
			self.iRec219[0] = i32::wrapping_add(i32::wrapping_mul(i32::wrapping_add(self.iRec219[1], ((self.iRec219[1] > 0) as i32)), ((fSlow20 <= self.fVec39[1]) as i32)), ((fSlow20 > self.fVec39[1]) as i32));
			let mut fTemp302: F32 = ((self.iRec219[0]) as F32) / F32::max(1.0, self.fConst15 * mydsp_faustpower2_f(1.0 - 0.0005 * fTemp258));
			self.fRec220[0] = fTemp155 - (self.fRec220[2] * ((fTemp300 + -1.4142135) / fTemp299 + 1.0) + 2.0 * self.fRec220[1] * (1.0 - 1.0 / mydsp_faustpower2_f(fTemp299))) / fTemp301;
			let mut fTemp303: F32 = 0.5 * ((self.fRec220[2] + self.fRec220[0] + 2.0 * self.fRec220[1]) * F32::max(0.0, F32::min(fTemp302, 2.0 - fTemp302)) / fTemp301);
			let mut fTemp304: F32 = fTemp303 + self.fVec38[1] + fTemp294;
			self.fVec40[0] = fTemp304;
			self.fRec211[(self.IOTA0 & 2047) as usize] = 0.05 * self.fRec211[((i32::wrapping_sub(self.IOTA0, 1)) & 2047) as usize] + 0.95 * self.fVec40[2];
			let mut fRec208: F32 = fTemp298 * self.fRec211[((i32::wrapping_sub(self.IOTA0, iTemp289)) & 2047) as usize] + fTemp284 * (fTemp297 * self.fRec211[((i32::wrapping_sub(self.IOTA0, iTemp282)) & 2047) as usize] + 0.5 * fTemp296 * self.fRec211[((i32::wrapping_sub(self.IOTA0, iTemp277)) & 2047) as usize] + 0.16666667 * fTemp295 * self.fRec211[((i32::wrapping_sub(self.IOTA0, iTemp273)) & 2047) as usize] + 0.041666668 * fTemp270 * self.fRec211[((i32::wrapping_sub(self.IOTA0, iTemp263)) & 2047) as usize]);
			let mut fRec209: F32 = self.fVec40[1] + self.fRec202[1];
			self.fRec202[0] = fRec207;
			let mut fRec203: F32 = self.fRec202[1];
			let mut fRec204: F32 = fRec208;
			let mut fRec205: F32 = fRec209;
			self.fRec198[0] = fRec203;
			let mut fRec199: F32 = fTemp294 + fTemp303 + self.fRec198[1];
			let mut fRec200: F32 = fRec204;
			let mut fRec201: F32 = fRec205;
			self.fRec194[(self.IOTA0 & 2047) as usize] = fRec199;
			let mut fRec195: F32 = fTemp298 * self.fRec194[((i32::wrapping_sub(self.IOTA0, iTemp290)) & 2047) as usize] + fTemp284 * (fTemp297 * self.fRec194[((i32::wrapping_sub(self.IOTA0, iTemp283)) & 2047) as usize] + 0.5 * fTemp296 * self.fRec194[((i32::wrapping_sub(self.IOTA0, iTemp278)) & 2047) as usize] + 0.16666667 * fTemp295 * self.fRec194[((i32::wrapping_sub(self.IOTA0, iTemp274)) & 2047) as usize] + 0.041666668 * fTemp270 * self.fRec194[((i32::wrapping_sub(self.IOTA0, iTemp264)) & 2047) as usize]);
			self.fRec196[0] = fRec200;
			let mut fRec197: F32 = fRec201;
			self.fRec192[0] = fSlow14 * self.fRec196[1];
			let mut fRec193: F32 = fRec197;
			self.fRec187[0] = fRec191;
			let mut fRec188: F32 = fSlow14 * self.fRec187[1];
			let mut fRec189: F32 = self.fRec192[0];
			let mut fRec190: F32 = fRec193;
			self.fRec183[(self.IOTA0 & 2047) as usize] = fRec188;
			let mut fRec184: F32 = fRec195;
			let mut fRec185: F32 = fRec189;
			let mut fRec186: F32 = fRec190;
			self.fRec181[0] = fRec184;
			let mut fRec182: F32 = fRec186;
			let mut fTemp305: F32 = F32::abs(fRec182);
			let mut fTemp306: F32 = if (((self.fRec179[1] > fTemp305) as i32) as i32 != 0) { self.fConst16 } else { 0.0 };
			self.fRec180[0] = self.fRec180[1] * fTemp306 + fTemp305 * (1.0 - fTemp306);
			self.fRec179[0] = self.fRec180[0];
			let mut fRec178: F32 = 0.0 - 0.95 * F32::max(2e+01 * F32::log10(self.fRec179[0]) + 1e+01, 0.0);
			self.fRec177[0] = fRec182 * F32::powf(1e+01, 0.05 * fRec178);
			self.fRec175[0] = 0.0 - (self.fRec175[1] * (1.0 - fTemp259) - (self.fRec177[0] + self.fRec177[1])) / (fTemp259 + 1.0);
			let mut fTemp307: F32 = (self.fRec175[0] + self.fRec129[0] + self.fRec83[0] + self.fRec35[0]) * self.fRec34[0];
			self.fRec221[0] = fSlow21 + self.fConst2 * self.fRec221[1];
			let mut fTemp308: F32 = F32::min(1.4141995, 1.4142135 * self.fRec221[0]);
			let mut fTemp309: F32 = 1.4142135 * fTemp308;
			let mut fTemp310: F32 = 1.0 - fTemp309;
			self.fRec223[0] = fSlow22 + self.fConst2 * self.fRec223[1];
			self.fRec222[0] = self.fConst2 * self.fRec222[1] + self.fConst1 * unsafe { ftbl0mydspSIG0[(((16.11811 * (fSlow23 + self.fRec223[0])) as i32)) as usize] };
			let mut fTemp311: F32 = F32::tan(self.fConst17 * F32::max(2e+01, F32::min(1e+04, self.fRec222[0])));
			let mut fTemp312: F32 = 1.0 / fTemp311;
			let mut fTemp313: F32 = 2.0 - fTemp309;
			let mut fTemp314: F32 = mydsp_faustpower2_f(fTemp308);
			let mut fTemp315: F32 = fTemp314 + (fTemp313 + fTemp312) / fTemp311 + fTemp310;
			let mut fTemp316: F32 = 1.0 / mydsp_faustpower2_f(fTemp311);
			let mut fTemp317: F32 = fTemp309 + 2.0;
			let mut fTemp318: F32 = fTemp309 + fTemp314;
			let mut fTemp319: F32 = fTemp318 + (fTemp317 + fTemp312) / fTemp311 + 1.0;
			self.fRec228[0] = self.fConst2 * self.fRec228[1] + fSlow24;
			let mut fTemp320: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec228[0]));
			let mut fTemp321: F32 = self.fConst3 * fTemp320;
			let mut fTemp322: F32 = self.fRec226[1] + fTemp321;
			let mut fTemp323: F32 = fTemp322 + -1.0;
			let mut iTemp324: i32 = ((fTemp323 < 0.0) as i32);
			self.fRec226[0] = if (iTemp324 as i32 != 0) { fTemp322 } else { fTemp323 };
			let mut fThen9: F32 = self.fRec226[1] + fTemp321 + (1.0 - self.fConst0 / fTemp320) * fTemp323;
			let mut fRec227: F32 = if (iTemp324 as i32 != 0) { fTemp322 } else { fThen9 };
			self.fRec229[0] = fSlow25 + self.fConst2 * self.fRec229[1];
			self.fRec225[0] = self.fRec229[0] * (2.0 * fRec227 + -1.0) - (self.fRec225[2] * (fTemp318 + (fTemp312 - fTemp317) / fTemp311 + 1.0) + 2.0 * self.fRec225[1] * (fTemp318 + (1.0 - fTemp316))) / fTemp319;
			self.fRec224[0] = (self.fRec225[2] + self.fRec225[0] + 2.0 * self.fRec225[1]) / fTemp319 - (self.fRec224[2] * (fTemp314 + (fTemp312 - fTemp313) / fTemp311 + fTemp310) + 2.0 * self.fRec224[1] * (fTemp314 + (1.0 - (fTemp309 + fTemp316)))) / fTemp315;
			self.fRec230[0] = self.fConst2 * self.fRec230[1] + self.fConst1 * unsafe { ftbl0mydspSIG0[(((16.11811 * (fSlow26 + self.fRec223[0])) as i32)) as usize] };
			let mut fTemp325: F32 = F32::tan(self.fConst17 * F32::max(2e+01, F32::min(1e+04, self.fRec230[0])));
			let mut fTemp326: F32 = 1.0 / fTemp325;
			let mut fTemp327: F32 = fTemp314 + (fTemp313 + fTemp326) / fTemp325 + fTemp310;
			let mut fTemp328: F32 = 1.0 / mydsp_faustpower2_f(fTemp325);
			let mut fTemp329: F32 = fTemp318 + (fTemp326 + fTemp317) / fTemp325 + 1.0;
			self.fRec235[0] = self.fConst2 * self.fRec235[1] + fSlow27;
			let mut fTemp330: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec235[0]));
			let mut fTemp331: F32 = self.fRec233[1] + self.fConst3 * fTemp330;
			let mut fTemp332: F32 = fTemp331 + -1.0;
			let mut iTemp333: i32 = ((fTemp332 < 0.0) as i32);
			self.fRec233[0] = if (iTemp333 as i32 != 0) { fTemp331 } else { fTemp332 };
			let mut fThen11: F32 = fTemp331 + (1.0 - self.fConst0 / fTemp330) * fTemp332;
			let mut fRec234: F32 = if (iTemp333 as i32 != 0) { fTemp331 } else { fThen11 };
			self.fRec236[0] = fSlow28 + self.fConst2 * self.fRec236[1];
			self.fRec232[0] = self.fRec236[0] * (2.0 * fRec234 + -1.0) - (self.fRec232[2] * (fTemp318 + (fTemp326 - fTemp317) / fTemp325 + 1.0) + 2.0 * self.fRec232[1] * (fTemp318 + (1.0 - fTemp328))) / fTemp329;
			self.fRec231[0] = (self.fRec232[2] + self.fRec232[0] + 2.0 * self.fRec232[1]) / fTemp329 - (self.fRec231[2] * (fTemp314 + (fTemp326 - fTemp313) / fTemp325 + fTemp310) + 2.0 * self.fRec231[1] * (fTemp314 + (1.0 - (fTemp309 + fTemp328)))) / fTemp327;
			self.fRec237[0] = self.fConst2 * self.fRec237[1] + self.fConst1 * unsafe { ftbl0mydspSIG0[(((16.11811 * (fSlow29 + self.fRec223[0])) as i32)) as usize] };
			let mut fTemp334: F32 = F32::tan(self.fConst17 * F32::max(2e+01, F32::min(1e+04, self.fRec237[0])));
			let mut fTemp335: F32 = 1.0 / fTemp334;
			let mut fTemp336: F32 = fTemp314 + (fTemp313 + fTemp335) / fTemp334 + fTemp310;
			let mut fTemp337: F32 = 1.0 / mydsp_faustpower2_f(fTemp334);
			let mut fTemp338: F32 = fTemp318 + (fTemp317 + fTemp335) / fTemp334 + 1.0;
			self.fRec242[0] = self.fConst2 * self.fRec242[1] + fSlow30;
			let mut fTemp339: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec242[0]));
			let mut fTemp340: F32 = self.fConst3 * fTemp339;
			let mut fTemp341: F32 = self.fRec240[1] + fTemp340;
			let mut fTemp342: F32 = fTemp341 + -1.0;
			let mut iTemp343: i32 = ((fTemp342 < 0.0) as i32);
			self.fRec240[0] = if (iTemp343 as i32 != 0) { fTemp341 } else { fTemp342 };
			let mut fThen13: F32 = self.fRec240[1] + fTemp340 + (1.0 - self.fConst0 / fTemp339) * fTemp342;
			let mut fRec241: F32 = if (iTemp343 as i32 != 0) { fTemp341 } else { fThen13 };
			self.fRec243[0] = fSlow31 + self.fConst2 * self.fRec243[1];
			self.fRec239[0] = self.fRec243[0] * (2.0 * fRec241 + -1.0) - (self.fRec239[2] * (fTemp318 + (1.0 - (fTemp317 - fTemp335) / fTemp334)) + 2.0 * self.fRec239[1] * (fTemp318 + (1.0 - fTemp337))) / fTemp338;
			self.fRec238[0] = (self.fRec239[2] + self.fRec239[0] + 2.0 * self.fRec239[1]) / fTemp338 - (self.fRec238[2] * (fTemp314 + (fTemp335 - fTemp313) / fTemp334 + fTemp310) + 2.0 * self.fRec238[1] * (fTemp314 + (1.0 - (fTemp309 + fTemp337)))) / fTemp336;
			self.fRec244[0] = self.fConst2 * self.fRec244[1] + self.fConst1 * unsafe { ftbl0mydspSIG0[(((16.11811 * (fSlow32 + self.fRec223[0])) as i32)) as usize] };
			let mut fTemp344: F32 = F32::tan(self.fConst17 * F32::max(2e+01, F32::min(1e+04, self.fRec244[0])));
			let mut fTemp345: F32 = 1.0 / fTemp344;
			let mut fTemp346: F32 = fTemp314 + (fTemp313 + fTemp345) / fTemp344 + fTemp310;
			let mut fTemp347: F32 = 1.0 / mydsp_faustpower2_f(fTemp344);
			let mut fTemp348: F32 = fTemp318 + (fTemp317 + fTemp345) / fTemp344 + 1.0;
			self.fRec249[0] = self.fConst2 * self.fRec249[1] + fSlow33;
			let mut fTemp349: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec249[0]));
			let mut fTemp350: F32 = self.fRec247[1] + self.fConst3 * fTemp349;
			let mut fTemp351: F32 = fTemp350 + -1.0;
			let mut iTemp352: i32 = ((fTemp351 < 0.0) as i32);
			self.fRec247[0] = if (iTemp352 as i32 != 0) { fTemp350 } else { fTemp351 };
			let mut fThen15: F32 = fTemp350 + (1.0 - self.fConst0 / fTemp349) * fTemp351;
			let mut fRec248: F32 = if (iTemp352 as i32 != 0) { fTemp350 } else { fThen15 };
			self.fRec250[0] = fSlow34 + self.fConst2 * self.fRec250[1];
			self.fRec246[0] = self.fRec250[0] * (2.0 * fRec248 + -1.0) - (self.fRec246[2] * (fTemp318 + (fTemp345 - fTemp317) / fTemp344 + 1.0) + 2.0 * self.fRec246[1] * (fTemp318 + (1.0 - fTemp347))) / fTemp348;
			self.fRec245[0] = (self.fRec246[2] + self.fRec246[0] + 2.0 * self.fRec246[1]) / fTemp348 - (self.fRec245[2] * (fTemp314 + (fTemp345 - fTemp313) / fTemp344 + fTemp310) + 2.0 * self.fRec245[1] * (fTemp314 + (1.0 - (fTemp309 + fTemp347)))) / fTemp346;
			self.fRec251[0] = fSlow35 + self.fConst2 * self.fRec251[1];
			self.fRec252[0] = fSlow36 + self.fConst2 * self.fRec252[1];
			let mut fTemp353: F32 = self.fRec252[0] * self.fRec251[0] * ((self.fRec245[2] + self.fRec245[0] + 2.0 * self.fRec245[1]) / fTemp346 + (self.fRec238[2] + self.fRec238[0] + 2.0 * self.fRec238[1]) / fTemp336 + (self.fRec231[2] + self.fRec231[0] + 2.0 * self.fRec231[1]) / fTemp327 + (self.fRec224[2] + self.fRec224[0] + 2.0 * self.fRec224[1]) / fTemp315);
			self.fRec253[0] = fSlow37 + self.fConst2 * self.fRec253[1];
			let mut fTemp354: F32 = (1.0 - self.fRec253[0]) * (fTemp353 + fTemp307 + fTemp109);
			self.fRec255[0] = fSlow38 + self.fConst2 * self.fRec255[1];
			self.fRec254[(self.IOTA0 & 2097151) as usize] = fTemp353 + fTemp109 + fTemp307 + fSlow39 * self.fRec254[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(((F32::min(self.fConst18, F32::max(0.0, self.fConst0 * self.fRec255[0]))) as i32), 1))) & 2097151) as usize];
			let mut fTemp355: F32 = self.fRec254[(self.IOTA0 & 2097151) as usize] * self.fRec253[0];
			let mut fTemp356: F32 = fTemp355 + fTemp354;
			let mut iTemp357: i32 = i32::wrapping_sub(1, self.iVec0[1]);
			self.fRec266[0] = 0.995 * (self.fRec266[1] + ((i32::wrapping_mul(iTemp357, iSlow43)) as F32)) + fSlow44;
			let mut fTemp358: F32 = self.fRec266[0] + -1.49999;
			let mut fTemp359: F32 = F32::floor(fTemp358);
			self.fRec268[0] = 0.995 * (self.fRec268[1] + ((i32::wrapping_mul(iTemp357, iSlow45)) as F32)) + fSlow46;
			let mut fTemp360: F32 = self.fRec268[0] + -1.49999;
			let mut fTemp361: F32 = F32::floor(fTemp360);
			self.fRec272[0] = 0.9999 * (self.fRec272[1] + ((i32::wrapping_mul(iTemp357, iSlow47)) as F32)) + fSlow48;
			let mut fTemp362: F32 = self.fRec272[0] + -1.49999;
			let mut fTemp363: F32 = F32::floor(fTemp362);
			let mut fTemp364: F32 = 0.760314 * self.fRec256[1] - 0.64955574 * self.fRec269[1];
			let mut fTemp365: F32 = 0.760314 * self.fRec257[1] - 0.64955574 * self.fRec270[1];
			self.fVec41[(self.IOTA0 & 16383) as usize] = 0.0 - 0.70710677 * (fTemp365 - fTemp364);
			let mut fTemp366: F32 = self.fVec41[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp362) as i32))))) & 16383) as usize];
			self.fVec42[0] = fTemp366;
			self.fRec271[0] = self.fVec42[1] - (fTemp363 + (2.0 - self.fRec272[0])) * (self.fRec271[1] - fTemp366) / (self.fRec272[0] - fTemp363);
			self.fRec269[0] = self.fRec271[0];
			self.fRec274[0] = 0.9999 * (self.fRec274[1] + ((i32::wrapping_mul(iTemp357, iSlow49)) as F32)) + fSlow50;
			let mut fTemp367: F32 = self.fRec274[0] + -1.49999;
			let mut fTemp368: F32 = F32::floor(fTemp367);
			let mut fTemp369: F32 = self.fRec274[0] - fTemp368;
			let mut fTemp370: F32 = fTemp368 + (2.0 - self.fRec274[0]);
			self.fVec43[(self.IOTA0 & 16383) as usize] = fTemp364 + fTemp365;
			let mut fTemp371: F32 = self.fVec43[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp367) as i32))))) & 16383) as usize];
			self.fVec44[0] = fTemp371;
			self.fRec273[0] = 0.70710677 * (fTemp370 * fTemp371 / fTemp369 + self.fVec44[1]) - self.fRec273[1] * fTemp370 / fTemp369;
			self.fRec270[0] = self.fRec273[0];
			let mut fTemp372: F32 = 0.760314 * self.fRec269[1] + 0.64955574 * self.fRec256[1];
			self.fRec278[0] = 0.9999 * (self.fRec278[1] + ((i32::wrapping_mul(iTemp357, iSlow51)) as F32)) + fSlow52;
			let mut fTemp373: F32 = self.fRec278[0] + -1.49999;
			let mut fTemp374: F32 = F32::floor(fTemp373);
			let mut fTemp375: F32 = self.fRec278[0] - fTemp374;
			let mut fTemp376: F32 = fTemp374 + (2.0 - self.fRec278[0]);
			let mut fTemp377: F32 = 0.760314 * self.fRec270[1] + 0.64955574 * self.fRec257[1];
			let mut fTemp378: F32 = 0.760314 * fTemp377 - 0.64955574 * self.fRec276[1];
			let mut fTemp379: F32 = 0.760314 * fTemp372 - 0.64955574 * self.fRec275[1];
			self.fVec45[(self.IOTA0 & 16383) as usize] = fTemp379 - fTemp378;
			let mut fTemp380: F32 = self.fVec45[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp373) as i32))))) & 16383) as usize];
			self.fVec46[0] = fTemp380;
			self.fRec277[0] = 0.70710677 * (fTemp376 * fTemp380 / fTemp375 + self.fVec46[1]) - self.fRec277[1] * fTemp376 / fTemp375;
			self.fRec275[0] = self.fRec277[0];
			self.fRec280[0] = 0.9999 * (self.fRec280[1] + ((i32::wrapping_mul(iTemp357, iSlow53)) as F32)) + fSlow54;
			let mut fTemp381: F32 = self.fRec280[0] + -1.49999;
			let mut fTemp382: F32 = F32::floor(fTemp381);
			let mut fTemp383: F32 = self.fRec280[0] - fTemp382;
			let mut fTemp384: F32 = fTemp382 + (2.0 - self.fRec280[0]);
			self.fVec47[(self.IOTA0 & 16383) as usize] = fTemp379 + fTemp378;
			let mut fTemp385: F32 = self.fVec47[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp381) as i32))))) & 16383) as usize];
			self.fVec48[0] = fTemp385;
			self.fRec279[0] = 0.70710677 * (fTemp384 * fTemp385 / fTemp383 + self.fVec48[1]) - self.fRec279[1] * fTemp384 / fTemp383;
			self.fRec276[0] = self.fRec279[0];
			let mut fTemp386: F32 = 0.760314 * self.fRec275[1] + 0.64955574 * fTemp372;
			self.fRec284[0] = 0.9999 * (self.fRec284[1] + ((i32::wrapping_mul(iTemp357, iSlow55)) as F32)) + fSlow56;
			let mut fTemp387: F32 = self.fRec284[0] + -1.49999;
			let mut fTemp388: F32 = F32::floor(fTemp387);
			let mut fTemp389: F32 = 0.760314 * fTemp386 - 0.64955574 * self.fRec281[1];
			let mut fTemp390: F32 = 0.760314 * self.fRec276[1] + 0.64955574 * fTemp377;
			let mut fTemp391: F32 = 0.760314 * fTemp390 - 0.64955574 * self.fRec282[1];
			self.fVec49[(self.IOTA0 & 16383) as usize] = 0.0 - 0.70710677 * (fTemp391 - fTemp389);
			let mut fTemp392: F32 = self.fVec49[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp387) as i32))))) & 16383) as usize];
			self.fVec50[0] = fTemp392;
			self.fRec283[0] = self.fVec50[1] - (fTemp388 + (2.0 - self.fRec284[0])) * (self.fRec283[1] - fTemp392) / (self.fRec284[0] - fTemp388);
			self.fRec281[0] = self.fRec283[0];
			self.fRec286[0] = 0.9999 * (self.fRec286[1] + ((i32::wrapping_mul(iTemp357, iSlow57)) as F32)) + fSlow58;
			let mut fTemp393: F32 = self.fRec286[0] + -1.49999;
			let mut fTemp394: F32 = F32::floor(fTemp393);
			let mut fTemp395: F32 = self.fRec286[0] - fTemp394;
			let mut fTemp396: F32 = fTemp394 + (2.0 - self.fRec286[0]);
			self.fVec51[(self.IOTA0 & 16383) as usize] = fTemp389 + fTemp391;
			let mut fTemp397: F32 = self.fVec51[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp393) as i32))))) & 16383) as usize];
			self.fVec52[0] = fTemp397;
			self.fRec285[0] = 0.70710677 * (fTemp396 * fTemp397 / fTemp395 + self.fVec52[1]) - self.fRec285[1] * fTemp396 / fTemp395;
			self.fRec282[0] = self.fRec285[0];
			let mut fTemp398: F32 = 0.760314 * self.fRec281[1] + 0.64955574 * fTemp386;
			self.fRec290[0] = 0.9999 * (self.fRec290[1] + ((i32::wrapping_mul(iTemp357, iSlow59)) as F32)) + fSlow60;
			let mut fTemp399: F32 = self.fRec290[0] + -1.49999;
			let mut fTemp400: F32 = F32::floor(fTemp399);
			let mut fTemp401: F32 = self.fRec290[0] - fTemp400;
			let mut fTemp402: F32 = fTemp400 + (2.0 - self.fRec290[0]);
			let mut fTemp403: F32 = 0.760314 * self.fRec282[1] + 0.64955574 * fTemp390;
			let mut fTemp404: F32 = 0.760314 * fTemp403 - 0.64955574 * self.fRec288[1];
			let mut fTemp405: F32 = 0.760314 * fTemp398 - 0.64955574 * self.fRec287[1];
			self.fVec53[(self.IOTA0 & 16383) as usize] = fTemp405 - fTemp404;
			let mut fTemp406: F32 = self.fVec53[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp399) as i32))))) & 16383) as usize];
			self.fVec54[0] = fTemp406;
			self.fRec289[0] = 0.70710677 * (fTemp402 * fTemp406 / fTemp401 + self.fVec54[1]) - self.fRec289[1] * fTemp402 / fTemp401;
			self.fRec287[0] = self.fRec289[0];
			self.fRec292[0] = 0.9999 * (self.fRec292[1] + ((i32::wrapping_mul(iTemp357, iSlow61)) as F32)) + fSlow62;
			let mut fTemp407: F32 = self.fRec292[0] + -1.49999;
			let mut fTemp408: F32 = F32::floor(fTemp407);
			let mut fTemp409: F32 = self.fRec292[0] - fTemp408;
			let mut fTemp410: F32 = fTemp408 + (2.0 - self.fRec292[0]);
			self.fVec55[(self.IOTA0 & 16383) as usize] = fTemp405 + fTemp404;
			let mut fTemp411: F32 = self.fVec55[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp407) as i32))))) & 16383) as usize];
			self.fVec56[0] = fTemp411;
			self.fRec291[0] = 0.70710677 * (fTemp410 * fTemp411 / fTemp409 + self.fVec56[1]) - self.fRec291[1] * fTemp410 / fTemp409;
			self.fRec288[0] = self.fRec291[0];
			let mut fTemp412: F32 = 0.760314 * self.fRec287[1] + 0.64955574 * fTemp398;
			self.fRec296[0] = 0.9999 * (self.fRec296[1] + ((i32::wrapping_mul(iTemp357, iSlow63)) as F32)) + fSlow64;
			let mut fTemp413: F32 = self.fRec296[0] + -1.49999;
			let mut fTemp414: F32 = F32::floor(fTemp413);
			let mut fTemp415: F32 = self.fRec296[0] - fTemp414;
			let mut fTemp416: F32 = fTemp414 + (2.0 - self.fRec296[0]);
			let mut fTemp417: F32 = 0.760314 * self.fRec288[1] + 0.64955574 * fTemp403;
			let mut fTemp418: F32 = 0.760314 * fTemp417 - 0.64955574 * self.fRec294[1];
			let mut fTemp419: F32 = 0.760314 * fTemp412 - 0.64955574 * self.fRec293[1];
			self.fVec57[(self.IOTA0 & 16383) as usize] = fTemp419 - fTemp418;
			let mut fTemp420: F32 = self.fVec57[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp413) as i32))))) & 16383) as usize];
			self.fVec58[0] = fTemp420;
			self.fRec295[0] = 0.70710677 * (fTemp416 * fTemp420 / fTemp415 + self.fVec58[1]) - self.fRec295[1] * fTemp416 / fTemp415;
			self.fRec293[0] = self.fRec295[0];
			self.fRec298[0] = 0.9999 * (self.fRec298[1] + ((i32::wrapping_mul(iTemp357, iSlow65)) as F32)) + fSlow66;
			let mut fTemp421: F32 = self.fRec298[0] + -1.49999;
			let mut fTemp422: F32 = F32::floor(fTemp421);
			let mut fTemp423: F32 = self.fRec298[0] - fTemp422;
			let mut fTemp424: F32 = fTemp422 + (2.0 - self.fRec298[0]);
			self.fVec59[(self.IOTA0 & 16383) as usize] = fTemp419 + fTemp418;
			let mut fTemp425: F32 = self.fVec59[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp421) as i32))))) & 16383) as usize];
			self.fVec60[0] = fTemp425;
			self.fRec297[0] = 0.70710677 * (fTemp424 * fTemp425 / fTemp423 + self.fVec60[1]) - self.fRec297[1] * fTemp424 / fTemp423;
			self.fRec294[0] = self.fRec297[0];
			let mut fTemp426: F32 = 0.760314 * self.fRec293[1] + 0.64955574 * fTemp412;
			self.fVec61[(self.IOTA0 & 1023) as usize] = fTemp426;
			self.fRec299[0] = fSlow69 * self.fRec300[1] + fSlow68 * self.fRec299[1];
			self.fRec300[0] = ((iTemp357) as F32) + fSlow68 * self.fRec300[1] - fSlow69 * self.fRec299[1];
			let mut fTemp427: F32 = fSlow70 * (self.fRec300[0] + 1.0);
			let mut fTemp428: F32 = fTemp427 + 3.500005;
			let mut iTemp429: i32 = ((fTemp428) as i32);
			let mut iTemp430: i32 = std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp429, 4)));
			let mut fTemp431: F32 = F32::floor(fTemp428);
			let mut fTemp432: F32 = fTemp427 + (2.0 - fTemp431);
			let mut fTemp433: F32 = fTemp427 + (3.0 - fTemp431);
			let mut fTemp434: F32 = fTemp427 + (4.0 - fTemp431);
			let mut fTemp435: F32 = fTemp434 * fTemp433;
			let mut fTemp436: F32 = fTemp435 * fTemp432;
			let mut fTemp437: F32 = fTemp427 + (1.0 - fTemp431);
			let mut fTemp438: F32 = 0.0 - fTemp437;
			let mut iTemp439: i32 = std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp429, 3)));
			let mut fTemp440: F32 = 0.0 - 0.5 * fTemp437;
			let mut fTemp441: F32 = 0.0 - fTemp432;
			let mut iTemp442: i32 = std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp429, 2)));
			let mut fTemp443: F32 = 0.0 - 0.33333334 * fTemp437;
			let mut fTemp444: F32 = 0.0 - 0.5 * fTemp432;
			let mut fTemp445: F32 = 0.0 - fTemp433;
			let mut iTemp446: i32 = std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp429, 1)));
			let mut fTemp447: F32 = fTemp427 + (5.0 - fTemp431);
			let mut fTemp448: F32 = 0.0 - 0.25 * fTemp437;
			let mut fTemp449: F32 = 0.0 - 0.33333334 * fTemp432;
			let mut fTemp450: F32 = 0.0 - 0.5 * fTemp433;
			let mut fTemp451: F32 = 0.0 - fTemp434;
			let mut iTemp452: i32 = std::cmp::min(512, std::cmp::max(0, iTemp429));
			self.fVec62[(self.IOTA0 & 16383) as usize] = self.fVec61[((i32::wrapping_sub(self.IOTA0, iTemp452)) & 1023) as usize] * fTemp451 * fTemp450 * fTemp449 * fTemp448 + fTemp447 * (self.fVec61[((i32::wrapping_sub(self.IOTA0, iTemp446)) & 1023) as usize] * fTemp445 * fTemp444 * fTemp443 + 0.5 * fTemp434 * self.fVec61[((i32::wrapping_sub(self.IOTA0, iTemp442)) & 1023) as usize] * fTemp441 * fTemp440 + 0.16666667 * fTemp435 * self.fVec61[((i32::wrapping_sub(self.IOTA0, iTemp439)) & 1023) as usize] * fTemp438 + 0.041666668 * fTemp436 * self.fVec61[((i32::wrapping_sub(self.IOTA0, iTemp430)) & 1023) as usize]);
			let mut fTemp453: F32 = self.fVec62[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp360) as i32))))) & 16383) as usize];
			self.fVec63[0] = fTemp453;
			self.fRec267[0] = self.fVec63[1] - (fTemp361 + (2.0 - self.fRec268[0])) * (self.fRec267[1] - fTemp453) / (self.fRec268[0] - fTemp361);
			self.fRec304[0] = 0.9999 * (self.fRec304[1] + ((i32::wrapping_mul(iTemp357, iSlow71)) as F32)) + fSlow72;
			let mut fTemp454: F32 = self.fRec304[0] + -1.49999;
			let mut fTemp455: F32 = F32::floor(fTemp454);
			let mut fTemp456: F32 = self.fRec304[0] - fTemp455;
			let mut fTemp457: F32 = fTemp455 + (2.0 - self.fRec304[0]);
			self.fRec306[0] = 0.995 * (self.fRec306[1] + ((i32::wrapping_mul(iTemp357, iSlow73)) as F32)) + fSlow74;
			let mut fTemp458: F32 = self.fRec306[0] + -1.49999;
			let mut fTemp459: F32 = F32::floor(fTemp458);
			let mut fTemp460: F32 = 0.760314 * self.fRec294[1] + 0.64955574 * fTemp417;
			self.fVec64[(self.IOTA0 & 1023) as usize] = fTemp460;
			let mut fTemp461: F32 = fSlow75 * self.fRec300[0];
			let mut fTemp462: F32 = fSlow70 + fTemp461 + 3.500005;
			let mut iTemp463: i32 = ((fTemp462) as i32);
			let mut fTemp464: F32 = F32::floor(fTemp462);
			let mut fTemp465: F32 = fSlow70 + fTemp461 + (2.0 - fTemp464);
			let mut fTemp466: F32 = fSlow70 + fTemp461 + (3.0 - fTemp464);
			let mut fTemp467: F32 = fSlow70 + fTemp461 + (4.0 - fTemp464);
			let mut fTemp468: F32 = fTemp467 * fTemp466;
			let mut fTemp469: F32 = fSlow70 + fTemp461 + (1.0 - fTemp464);
			self.fVec65[(self.IOTA0 & 16383) as usize] = self.fVec64[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, std::cmp::max(0, iTemp463)))) & 1023) as usize] * (0.0 - fTemp467) * (0.0 - 0.5 * fTemp466) * (0.0 - 0.33333334 * fTemp465) * (0.0 - 0.25 * fTemp469) + (fSlow70 + fTemp461 + (5.0 - fTemp464)) * (self.fVec64[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp463, 1))))) & 1023) as usize] * (0.0 - fTemp466) * (0.0 - 0.5 * fTemp465) * (0.0 - 0.33333334 * fTemp469) + 0.5 * fTemp467 * self.fVec64[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp463, 2))))) & 1023) as usize] * (0.0 - fTemp465) * (0.0 - 0.5 * fTemp469) + 0.16666667 * fTemp468 * self.fVec64[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp463, 3))))) & 1023) as usize] * (0.0 - fTemp469) + 0.041666668 * fTemp468 * fTemp465 * self.fVec64[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp463, 4))))) & 1023) as usize]);
			let mut fTemp470: F32 = self.fVec65[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp458) as i32))))) & 16383) as usize];
			self.fVec66[0] = fTemp470;
			self.fRec305[0] = self.fVec66[1] - (fTemp459 + (2.0 - self.fRec306[0])) * (self.fRec305[1] - fTemp470) / (self.fRec306[0] - fTemp459);
			let mut fTemp471: F32 = 0.760314 * self.fRec305[0] - 0.64955574 * self.fRec302[1];
			let mut fTemp472: F32 = 0.760314 * self.fRec267[0] - 0.64955574 * self.fRec301[1];
			self.fVec67[(self.IOTA0 & 16383) as usize] = fTemp472 - fTemp471;
			let mut fTemp473: F32 = self.fVec67[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp454) as i32))))) & 16383) as usize];
			self.fVec68[0] = fTemp473;
			self.fRec303[0] = 0.70710677 * (fTemp457 * fTemp473 / fTemp456 + self.fVec68[1]) - self.fRec303[1] * fTemp457 / fTemp456;
			self.fRec301[0] = self.fRec303[0];
			self.fRec308[0] = 0.9999 * (self.fRec308[1] + ((i32::wrapping_mul(iTemp357, iSlow76)) as F32)) + fSlow77;
			let mut fTemp474: F32 = self.fRec308[0] + -1.49999;
			let mut fTemp475: F32 = F32::floor(fTemp474);
			let mut fTemp476: F32 = self.fRec308[0] - fTemp475;
			let mut fTemp477: F32 = fTemp475 + (2.0 - self.fRec308[0]);
			self.fVec69[(self.IOTA0 & 16383) as usize] = fTemp472 + fTemp471;
			let mut fTemp478: F32 = self.fVec69[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp474) as i32))))) & 16383) as usize];
			self.fVec70[0] = fTemp478;
			self.fRec307[0] = 0.70710677 * (fTemp477 * fTemp478 / fTemp476 + self.fVec70[1]) - self.fRec307[1] * fTemp477 / fTemp476;
			self.fRec302[0] = self.fRec307[0];
			let mut fTemp479: F32 = 0.760314 * self.fRec301[1] + 0.64955574 * self.fRec267[0];
			self.fRec312[0] = 0.9999 * (self.fRec312[1] + ((i32::wrapping_mul(iTemp357, iSlow78)) as F32)) + fSlow79;
			let mut fTemp480: F32 = self.fRec312[0] + -1.49999;
			let mut fTemp481: F32 = F32::floor(fTemp480);
			let mut fTemp482: F32 = self.fRec312[0] - fTemp481;
			let mut fTemp483: F32 = fTemp481 + (2.0 - self.fRec312[0]);
			let mut fTemp484: F32 = 0.760314 * self.fRec302[1] + 0.64955574 * self.fRec305[0];
			let mut fTemp485: F32 = 0.760314 * fTemp484 - 0.64955574 * self.fRec310[1];
			let mut fTemp486: F32 = 0.760314 * fTemp479 - 0.64955574 * self.fRec309[1];
			self.fVec71[(self.IOTA0 & 16383) as usize] = fTemp486 - fTemp485;
			let mut fTemp487: F32 = self.fVec71[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp480) as i32))))) & 16383) as usize];
			self.fVec72[0] = fTemp487;
			self.fRec311[0] = 0.70710677 * (fTemp483 * fTemp487 / fTemp482 + self.fVec72[1]) - self.fRec311[1] * fTemp483 / fTemp482;
			self.fRec309[0] = self.fRec311[0];
			self.fRec314[0] = 0.9999 * (self.fRec314[1] + ((i32::wrapping_mul(iTemp357, iSlow80)) as F32)) + fSlow81;
			let mut fTemp488: F32 = self.fRec314[0] + -1.49999;
			let mut fTemp489: F32 = F32::floor(fTemp488);
			let mut fTemp490: F32 = self.fRec314[0] - fTemp489;
			let mut fTemp491: F32 = fTemp489 + (2.0 - self.fRec314[0]);
			self.fVec73[(self.IOTA0 & 16383) as usize] = fTemp486 + fTemp485;
			let mut iTemp492: i32 = std::cmp::min(8192, std::cmp::max(0, ((fTemp488) as i32)));
			let mut fTemp493: F32 = self.fVec73[((i32::wrapping_sub(self.IOTA0, iTemp492)) & 16383) as usize];
			self.fVec74[0] = fTemp493;
			self.fRec313[0] = 0.70710677 * (fTemp491 * fTemp493 / fTemp490 + self.fVec74[1]) - fTemp491 * self.fRec313[1] / fTemp490;
			self.fRec310[0] = self.fRec313[0];
			let mut fTemp494: F32 = 0.760314 * self.fRec309[1] + 0.64955574 * fTemp479;
			self.fRec318[0] = 0.9999 * (self.fRec318[1] + ((i32::wrapping_mul(iTemp357, iSlow82)) as F32)) + fSlow83;
			let mut fTemp495: F32 = self.fRec318[0] + -1.49999;
			let mut fTemp496: F32 = F32::floor(fTemp495);
			let mut fTemp497: F32 = self.fRec318[0] - fTemp496;
			let mut fTemp498: F32 = fTemp496 + (2.0 - self.fRec318[0]);
			let mut fTemp499: F32 = 0.760314 * self.fRec310[1] + 0.64955574 * fTemp484;
			let mut fTemp500: F32 = 0.760314 * fTemp499 - 0.64955574 * self.fRec316[1];
			let mut fTemp501: F32 = 0.760314 * fTemp494 - 0.64955574 * self.fRec315[1];
			self.fVec75[(self.IOTA0 & 16383) as usize] = fTemp501 - fTemp500;
			let mut fTemp502: F32 = self.fVec75[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp495) as i32))))) & 16383) as usize];
			self.fVec76[0] = fTemp502;
			self.fRec317[0] = 0.70710677 * (fTemp498 * fTemp502 / fTemp497 + self.fVec76[1]) - self.fRec317[1] * fTemp498 / fTemp497;
			self.fRec315[0] = self.fRec317[0];
			self.fRec320[0] = 0.9999 * (self.fRec320[1] + ((i32::wrapping_mul(iTemp357, iSlow84)) as F32)) + fSlow85;
			let mut fTemp503: F32 = self.fRec320[0] + -1.49999;
			let mut fTemp504: F32 = F32::floor(fTemp503);
			let mut fTemp505: F32 = self.fRec320[0] - fTemp504;
			let mut fTemp506: F32 = fTemp504 + (2.0 - self.fRec320[0]);
			self.fVec77[(self.IOTA0 & 16383) as usize] = fTemp501 + fTemp500;
			let mut iTemp507: i32 = std::cmp::min(8192, std::cmp::max(0, ((fTemp503) as i32)));
			let mut fTemp508: F32 = self.fVec77[((i32::wrapping_sub(self.IOTA0, iTemp507)) & 16383) as usize];
			self.fVec78[0] = fTemp508;
			self.fRec319[0] = 0.70710677 * (fTemp506 * fTemp508 / fTemp505 + self.fVec78[1]) - self.fRec319[1] * fTemp506 / fTemp505;
			self.fRec316[0] = self.fRec319[0];
			let mut fTemp509: F32 = 0.760314 * self.fRec315[1] + 0.64955574 * fTemp494;
			self.fRec324[0] = 0.9999 * (self.fRec324[1] + ((i32::wrapping_mul(iTemp357, iSlow86)) as F32)) + fSlow87;
			let mut fTemp510: F32 = self.fRec324[0] + -1.49999;
			let mut fTemp511: F32 = F32::floor(fTemp510);
			let mut fTemp512: F32 = self.fRec324[0] - fTemp511;
			let mut fTemp513: F32 = fTemp511 + (2.0 - self.fRec324[0]);
			let mut fTemp514: F32 = 0.760314 * self.fRec316[1] + 0.64955574 * fTemp499;
			let mut fTemp515: F32 = 0.760314 * fTemp514 - 0.64955574 * self.fRec322[1];
			let mut fTemp516: F32 = 0.760314 * fTemp509 - 0.64955574 * self.fRec321[1];
			self.fVec79[(self.IOTA0 & 16383) as usize] = fTemp516 - fTemp515;
			let mut iTemp517: i32 = std::cmp::min(8192, std::cmp::max(0, ((fTemp510) as i32)));
			let mut fTemp518: F32 = self.fVec79[((i32::wrapping_sub(self.IOTA0, iTemp517)) & 16383) as usize];
			self.fVec80[0] = fTemp518;
			self.fRec323[0] = 0.70710677 * (fTemp513 * fTemp518 / fTemp512 + self.fVec80[1]) - fTemp513 * self.fRec323[1] / fTemp512;
			self.fRec321[0] = self.fRec323[0];
			self.fRec326[0] = 0.9999 * (self.fRec326[1] + ((i32::wrapping_mul(iTemp357, iSlow88)) as F32)) + fSlow89;
			let mut fTemp519: F32 = self.fRec326[0] + -1.49999;
			let mut fTemp520: F32 = F32::floor(fTemp519);
			let mut fTemp521: F32 = self.fRec326[0] - fTemp520;
			let mut fTemp522: F32 = fTemp520 + (2.0 - self.fRec326[0]);
			self.fVec81[(self.IOTA0 & 16383) as usize] = fTemp516 + fTemp515;
			let mut iTemp523: i32 = std::cmp::min(8192, std::cmp::max(0, ((fTemp519) as i32)));
			let mut fTemp524: F32 = self.fVec81[((i32::wrapping_sub(self.IOTA0, iTemp523)) & 16383) as usize];
			self.fVec82[0] = fTemp524;
			self.fRec325[0] = 0.70710677 * (fTemp522 * fTemp524 / fTemp521 + self.fVec82[1]) - fTemp522 * self.fRec325[1] / fTemp521;
			self.fRec322[0] = self.fRec325[0];
			let mut fTemp525: F32 = 0.760314 * self.fRec321[1] + 0.64955574 * fTemp509;
			self.fRec330[0] = 0.9999 * (self.fRec330[1] + ((i32::wrapping_mul(iTemp357, iSlow90)) as F32)) + fSlow91;
			let mut fTemp526: F32 = self.fRec330[0] + -1.49999;
			let mut fTemp527: F32 = F32::floor(fTemp526);
			let mut fTemp528: F32 = self.fRec330[0] - fTemp527;
			let mut fTemp529: F32 = fTemp527 + (2.0 - self.fRec330[0]);
			let mut fTemp530: F32 = 0.760314 * self.fRec322[1] + 0.64955574 * fTemp514;
			let mut fTemp531: F32 = 0.760314 * fTemp530 - 0.64955574 * self.fRec328[1];
			let mut fTemp532: F32 = 0.760314 * fTemp525 - 0.64955574 * self.fRec327[1];
			self.fVec83[(self.IOTA0 & 16383) as usize] = fTemp532 - fTemp531;
			let mut fTemp533: F32 = self.fVec83[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp526) as i32))))) & 16383) as usize];
			self.fVec84[0] = fTemp533;
			self.fRec329[0] = 0.70710677 * (fTemp529 * fTemp533 / fTemp528 + self.fVec84[1]) - self.fRec329[1] * fTemp529 / fTemp528;
			self.fRec327[0] = self.fRec329[0];
			self.fRec332[0] = 0.9999 * (self.fRec332[1] + ((i32::wrapping_mul(iTemp357, iSlow92)) as F32)) + fSlow93;
			let mut fTemp534: F32 = self.fRec332[0] + -1.49999;
			let mut fTemp535: F32 = F32::floor(fTemp534);
			let mut fTemp536: F32 = self.fRec332[0] - fTemp535;
			let mut fTemp537: F32 = fTemp535 + (2.0 - self.fRec332[0]);
			self.fVec85[(self.IOTA0 & 16383) as usize] = fTemp532 + fTemp531;
			let mut iTemp538: i32 = std::cmp::min(8192, std::cmp::max(0, ((fTemp534) as i32)));
			let mut fTemp539: F32 = self.fVec85[((i32::wrapping_sub(self.IOTA0, iTemp538)) & 16383) as usize];
			self.fVec86[0] = fTemp539;
			self.fRec331[0] = 0.70710677 * (fTemp537 * fTemp539 / fTemp536 + self.fVec86[1]) - self.fRec331[1] * fTemp537 / fTemp536;
			self.fRec328[0] = self.fRec331[0];
			let mut fTemp540: F32 = 0.760314 * self.fRec327[1] + 0.64955574 * fTemp525;
			self.fVec87[(self.IOTA0 & 16383) as usize] = fTemp540;
			let mut fTemp541: F32 = fSlow70 * (self.fRec299[0] + 1.0);
			let mut fTemp542: F32 = fTemp541 + 3.500005;
			let mut iTemp543: i32 = ((fTemp542) as i32);
			let mut iTemp544: i32 = std::cmp::max(0, i32::wrapping_add(iTemp543, 4));
			let mut fTemp545: F32 = F32::floor(fTemp542);
			let mut fTemp546: F32 = fTemp541 + (2.0 - fTemp545);
			let mut fTemp547: F32 = fTemp541 + (3.0 - fTemp545);
			let mut fTemp548: F32 = fTemp541 + (4.0 - fTemp545);
			let mut fTemp549: F32 = fTemp548 * fTemp547;
			let mut fTemp550: F32 = fTemp549 * fTemp546;
			let mut fTemp551: F32 = fTemp541 + (1.0 - fTemp545);
			let mut fTemp552: F32 = 0.0 - fTemp551;
			let mut iTemp553: i32 = std::cmp::max(0, i32::wrapping_add(iTemp543, 3));
			let mut fTemp554: F32 = 0.0 - 0.5 * fTemp551;
			let mut fTemp555: F32 = 0.0 - fTemp546;
			let mut iTemp556: i32 = std::cmp::max(0, i32::wrapping_add(iTemp543, 2));
			let mut fTemp557: F32 = 0.0 - 0.33333334 * fTemp551;
			let mut fTemp558: F32 = 0.0 - 0.5 * fTemp546;
			let mut fTemp559: F32 = 0.0 - fTemp547;
			let mut iTemp560: i32 = std::cmp::max(0, i32::wrapping_add(iTemp543, 1));
			let mut fTemp561: F32 = fTemp541 + (5.0 - fTemp545);
			let mut fTemp562: F32 = 0.0 - 0.25 * fTemp551;
			let mut fTemp563: F32 = 0.0 - 0.33333334 * fTemp546;
			let mut fTemp564: F32 = 0.0 - 0.5 * fTemp547;
			let mut fTemp565: F32 = 0.0 - fTemp548;
			let mut iTemp566: i32 = std::cmp::max(0, iTemp543);
			self.fVec88[(self.IOTA0 & 16383) as usize] = self.fVec87[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, iTemp566))) & 16383) as usize] * fTemp565 * fTemp564 * fTemp563 * fTemp562 + fTemp561 * (self.fVec87[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, iTemp560))) & 16383) as usize] * fTemp559 * fTemp558 * fTemp557 + 0.5 * fTemp548 * self.fVec87[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, iTemp556))) & 16383) as usize] * fTemp555 * fTemp554 + 0.16666667 * fTemp549 * self.fVec87[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, iTemp553))) & 16383) as usize] * fTemp552 + 0.041666668 * fTemp550 * self.fVec87[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, iTemp544))) & 16383) as usize]);
			let mut fTemp567: F32 = self.fVec88[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp358) as i32))))) & 16383) as usize];
			self.fVec89[0] = fTemp567;
			self.fRec265[0] = self.fVec89[1] - (fTemp359 + (2.0 - self.fRec266[0])) * (self.fRec265[1] - fTemp567) / (self.fRec266[0] - fTemp359);
			self.fRec264[0] = self.fConst39 * (self.fRec265[0] + self.fRec265[1] - self.fConst37 * self.fRec264[1]);
			self.fRec263[0] = self.fRec264[0] - self.fConst36 * (self.fConst35 * self.fRec263[2] + self.fConst31 * self.fRec263[1]);
			self.fRec262[0] = self.fConst36 * (self.fRec263[2] + self.fRec263[0] + 2.0 * self.fRec263[1]) - self.fConst34 * (self.fConst33 * self.fRec262[2] + self.fConst31 * self.fRec262[1]);
			let mut fTemp568: F32 = self.fRec262[2] + self.fRec262[0] + 2.0 * self.fRec262[1];
			self.fVec90[0] = fTemp568;
			self.fRec261[0] = 0.0 - self.fConst42 * (self.fConst40 * self.fRec261[1] - self.fConst34 * (fTemp568 + self.fVec90[1]));
			self.fRec260[0] = self.fRec261[0] - self.fConst27 * (self.fConst26 * self.fRec260[2] + self.fConst22 * self.fRec260[1]);
			self.fRec259[0] = self.fConst27 * (self.fRec260[2] + self.fRec260[0] + 2.0 * self.fRec260[1]) - self.fConst25 * (self.fConst24 * self.fRec259[2] + self.fConst22 * self.fRec259[1]);
			self.fRec335[0] = self.fConst34 * (self.fConst44 * fTemp568 + self.fConst45 * self.fVec90[1]) - self.fConst43 * self.fRec335[1];
			self.fRec334[0] = self.fRec335[0] - self.fConst27 * (self.fConst26 * self.fRec334[2] + self.fConst22 * self.fRec334[1]);
			self.fRec333[0] = self.fConst27 * (self.fConst46 * self.fRec334[1] + self.fConst21 * self.fRec334[0] + self.fConst21 * self.fRec334[2]) - self.fConst25 * (self.fConst24 * self.fRec333[2] + self.fConst22 * self.fRec333[1]);
			let mut fTemp569: F32 = self.fConst22 * self.fRec336[1];
			self.fRec339[0] = self.fConst49 * self.fRec265[1] - self.fConst39 * (self.fConst37 * self.fRec339[1] - self.fConst32 * self.fRec265[0]);
			self.fRec338[0] = self.fRec339[0] - self.fConst36 * (self.fConst35 * self.fRec338[2] + self.fConst31 * self.fRec338[1]);
			self.fRec337[0] = self.fConst36 * (self.fConst30 * self.fRec338[0] + self.fConst50 * self.fRec338[1] + self.fConst30 * self.fRec338[2]) - self.fConst34 * (self.fConst33 * self.fRec337[2] + self.fConst31 * self.fRec337[1]);
			self.fRec336[0] = self.fConst34 * (self.fConst30 * self.fRec337[0] + self.fConst50 * self.fRec337[1] + self.fConst30 * self.fRec337[2]) - self.fConst48 * (self.fConst47 * self.fRec336[2] + fTemp569);
			let mut fTemp570: F32 = fTemp354 + fTemp355 + fSlow94 * (self.fRec336[2] + self.fConst48 * (fTemp569 + self.fConst47 * self.fRec336[0]) + self.fConst25 * (self.fConst46 * self.fRec333[1] + self.fConst21 * self.fRec333[0] + self.fConst21 * self.fRec333[2] + self.fRec259[2] + self.fRec259[0] + 2.0 * self.fRec259[1]));
			self.fVec91[(self.IOTA0 & 1023) as usize] = fTemp570;
			self.fRec258[0] = fSlow95 * self.fRec258[1] + fSlow96 * (fTemp565 * fTemp564 * fTemp563 * fTemp562 * self.fVec91[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, iTemp566))) & 1023) as usize] + fTemp561 * (fTemp559 * fTemp558 * fTemp557 * self.fVec91[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, iTemp560))) & 1023) as usize] + 0.5 * fTemp548 * fTemp555 * fTemp554 * self.fVec91[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, iTemp556))) & 1023) as usize] + 0.16666667 * fTemp549 * fTemp552 * self.fVec91[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, iTemp553))) & 1023) as usize] + 0.041666668 * fTemp550 * self.fVec91[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, iTemp544))) & 1023) as usize]));
			self.fRec351[0] = 0.995 * (self.fRec351[1] + ((i32::wrapping_mul(iTemp357, iSlow99)) as F32)) + fSlow100;
			let mut fTemp571: F32 = self.fRec351[0] + -1.49999;
			let mut fTemp572: F32 = F32::floor(fTemp571);
			let mut fTemp573: F32 = 0.760314 * self.fRec328[1] + 0.64955574 * fTemp530;
			self.fVec92[(self.IOTA0 & 16383) as usize] = fTemp573;
			let mut fTemp574: F32 = fSlow75 * self.fRec299[0];
			let mut fTemp575: F32 = fSlow70 + fTemp574 + 3.500005;
			let mut iTemp576: i32 = ((fTemp575) as i32);
			let mut fTemp577: F32 = F32::floor(fTemp575);
			let mut fTemp578: F32 = fSlow70 + fTemp574 + (2.0 - fTemp577);
			let mut fTemp579: F32 = fSlow70 + fTemp574 + (3.0 - fTemp577);
			let mut fTemp580: F32 = fSlow70 + fTemp574 + (4.0 - fTemp577);
			let mut fTemp581: F32 = fTemp580 * fTemp579;
			let mut fTemp582: F32 = fSlow70 + fTemp574 + (1.0 - fTemp577);
			self.fVec93[(self.IOTA0 & 16383) as usize] = self.fVec92[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, iTemp576)))) & 16383) as usize] * (0.0 - fTemp580) * (0.0 - 0.5 * fTemp579) * (0.0 - 0.33333334 * fTemp578) * (0.0 - 0.25 * fTemp582) + (fSlow70 + fTemp574 + (5.0 - fTemp577)) * (self.fVec92[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, i32::wrapping_add(iTemp576, 1))))) & 16383) as usize] * (0.0 - fTemp579) * (0.0 - 0.5 * fTemp578) * (0.0 - 0.33333334 * fTemp582) + 0.5 * fTemp580 * self.fVec92[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, i32::wrapping_add(iTemp576, 2))))) & 16383) as usize] * (0.0 - fTemp578) * (0.0 - 0.5 * fTemp582) + 0.16666667 * fTemp581 * self.fVec92[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, i32::wrapping_add(iTemp576, 3))))) & 16383) as usize] * (0.0 - fTemp582) + 0.041666668 * fTemp581 * fTemp578 * self.fVec92[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, i32::wrapping_add(iTemp576, 4))))) & 16383) as usize]);
			let mut fTemp583: F32 = self.fVec93[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp571) as i32))))) & 16383) as usize];
			self.fVec94[0] = fTemp583;
			self.fRec350[0] = self.fVec94[1] - (fTemp572 + (2.0 - self.fRec351[0])) * (self.fRec350[1] - fTemp583) / (self.fRec351[0] - fTemp572);
			self.fRec349[0] = 0.0 - self.fConst39 * (self.fConst37 * self.fRec349[1] - (self.fRec350[0] + self.fRec350[1]));
			self.fRec348[0] = self.fRec349[0] - self.fConst36 * (self.fConst35 * self.fRec348[2] + self.fConst31 * self.fRec348[1]);
			self.fRec347[0] = self.fConst36 * (self.fRec348[2] + self.fRec348[0] + 2.0 * self.fRec348[1]) - self.fConst34 * (self.fConst33 * self.fRec347[2] + self.fConst31 * self.fRec347[1]);
			let mut fTemp584: F32 = self.fRec347[2] + self.fRec347[0] + 2.0 * self.fRec347[1];
			self.fVec95[0] = fTemp584;
			self.fRec346[0] = 0.0 - self.fConst42 * (self.fConst40 * self.fRec346[1] - self.fConst34 * (fTemp584 + self.fVec95[1]));
			self.fRec345[0] = self.fRec346[0] - self.fConst27 * (self.fConst26 * self.fRec345[2] + self.fConst22 * self.fRec345[1]);
			self.fRec344[0] = self.fConst27 * (self.fRec345[2] + self.fRec345[0] + 2.0 * self.fRec345[1]) - self.fConst25 * (self.fConst24 * self.fRec344[2] + self.fConst22 * self.fRec344[1]);
			self.fRec354[0] = self.fConst34 * (self.fConst45 * self.fVec95[1] + self.fConst44 * fTemp584) - self.fConst43 * self.fRec354[1];
			self.fRec353[0] = self.fRec354[0] - self.fConst27 * (self.fConst26 * self.fRec353[2] + self.fConst22 * self.fRec353[1]);
			self.fRec352[0] = self.fConst27 * (self.fConst46 * self.fRec353[1] + self.fConst21 * self.fRec353[0] + self.fConst21 * self.fRec353[2]) - self.fConst25 * (self.fConst24 * self.fRec352[2] + self.fConst22 * self.fRec352[1]);
			let mut fTemp585: F32 = self.fConst22 * self.fRec355[1];
			self.fRec358[0] = self.fConst49 * self.fRec350[1] - self.fConst39 * (self.fConst37 * self.fRec358[1] - self.fConst32 * self.fRec350[0]);
			self.fRec357[0] = self.fRec358[0] - self.fConst36 * (self.fConst35 * self.fRec357[2] + self.fConst31 * self.fRec357[1]);
			self.fRec356[0] = self.fConst36 * (self.fConst50 * self.fRec357[1] + self.fConst30 * self.fRec357[0] + self.fConst30 * self.fRec357[2]) - self.fConst34 * (self.fConst33 * self.fRec356[2] + self.fConst31 * self.fRec356[1]);
			self.fRec355[0] = self.fConst34 * (self.fConst30 * self.fRec356[0] + self.fConst50 * self.fRec356[1] + self.fConst30 * self.fRec356[2]) - self.fConst48 * (self.fConst47 * self.fRec355[2] + fTemp585);
			let mut fTemp586: F32 = fTemp356 + fSlow94 * (self.fRec355[2] + self.fConst48 * (fTemp585 + self.fConst47 * self.fRec355[0]) + self.fConst25 * (self.fConst21 * self.fRec352[0] + self.fConst46 * self.fRec352[1] + self.fConst21 * self.fRec352[2] + self.fRec344[2] + self.fRec344[0] + 2.0 * self.fRec344[1]));
			self.fVec96[(self.IOTA0 & 1023) as usize] = fTemp586;
			self.fRec343[0] = fSlow95 * self.fRec343[1] + fSlow96 * (fTemp451 * fTemp450 * fTemp449 * fTemp448 * self.fVec96[((i32::wrapping_sub(self.IOTA0, iTemp452)) & 1023) as usize] + fTemp447 * (fTemp445 * fTemp444 * fTemp443 * self.fVec96[((i32::wrapping_sub(self.IOTA0, iTemp446)) & 1023) as usize] + 0.5 * fTemp434 * fTemp441 * fTemp440 * self.fVec96[((i32::wrapping_sub(self.IOTA0, iTemp442)) & 1023) as usize] + 0.16666667 * fTemp435 * fTemp438 * self.fVec96[((i32::wrapping_sub(self.IOTA0, iTemp439)) & 1023) as usize] + 0.041666668 * fTemp436 * self.fVec96[((i32::wrapping_sub(self.IOTA0, iTemp430)) & 1023) as usize]));
			let mut fTemp587: F32 = fSlow101 * self.fRec343[0] - fSlow98 * self.fRec341[1];
			let mut fTemp588: F32 = fSlow101 * self.fRec258[0] - fSlow98 * self.fRec340[1];
			self.fVec97[(self.IOTA0 & 16383) as usize] = fTemp588 - fTemp587;
			let mut fTemp589: F32 = self.fVec97[((i32::wrapping_sub(self.IOTA0, iTemp492)) & 16383) as usize];
			self.fVec98[0] = fTemp589;
			self.fRec342[0] = 0.70710677 * (fTemp491 * fTemp589 / fTemp490 + self.fVec98[1]) - self.fRec342[1] * fTemp491 / fTemp490;
			self.fRec340[0] = self.fRec342[0];
			self.fRec360[0] = 0.9999 * (self.fRec360[1] + ((i32::wrapping_mul(iTemp357, iSlow102)) as F32)) + fSlow103;
			let mut fTemp590: F32 = self.fRec360[0] + -1.49999;
			let mut fTemp591: F32 = F32::floor(fTemp590);
			let mut fTemp592: F32 = self.fRec360[0] - fTemp591;
			let mut fTemp593: F32 = fTemp591 + (2.0 - self.fRec360[0]);
			self.fVec99[(self.IOTA0 & 16383) as usize] = fTemp588 + fTemp587;
			let mut fTemp594: F32 = self.fVec99[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp590) as i32))))) & 16383) as usize];
			self.fVec100[0] = fTemp594;
			self.fRec359[0] = 0.70710677 * (fTemp593 * fTemp594 / fTemp592 + self.fVec100[1]) - self.fRec359[1] * fTemp593 / fTemp592;
			self.fRec341[0] = self.fRec359[0];
			let mut fTemp595: F32 = fSlow101 * self.fRec340[1] + fSlow98 * self.fRec258[0];
			let mut fTemp596: F32 = fSlow101 * self.fRec341[1] + fSlow98 * self.fRec343[0];
			let mut fTemp597: F32 = fSlow101 * fTemp596 - fSlow98 * self.fRec362[1];
			let mut fTemp598: F32 = fSlow101 * fTemp595 - fSlow98 * self.fRec361[1];
			self.fVec101[(self.IOTA0 & 16383) as usize] = fTemp598 - fTemp597;
			let mut fTemp599: F32 = self.fVec101[((i32::wrapping_sub(self.IOTA0, iTemp517)) & 16383) as usize];
			self.fVec102[0] = fTemp599;
			self.fRec363[0] = 0.70710677 * (fTemp513 * fTemp599 / fTemp512 + self.fVec102[1]) - self.fRec363[1] * fTemp513 / fTemp512;
			self.fRec361[0] = self.fRec363[0];
			self.fVec103[(self.IOTA0 & 16383) as usize] = fTemp598 + fTemp597;
			let mut fTemp600: F32 = self.fVec103[((i32::wrapping_sub(self.IOTA0, iTemp507)) & 16383) as usize];
			self.fVec104[0] = fTemp600;
			self.fRec364[0] = 0.70710677 * (fTemp506 * fTemp600 / fTemp505 + self.fVec104[1]) - fTemp506 * self.fRec364[1] / fTemp505;
			self.fRec362[0] = self.fRec364[0];
			let mut fTemp601: F32 = fSlow101 * self.fRec361[1] + fSlow98 * fTemp595;
			let mut fTemp602: F32 = fSlow101 * self.fRec362[1] + fSlow98 * fTemp596;
			let mut fTemp603: F32 = fSlow101 * fTemp602 - fSlow98 * self.fRec366[1];
			let mut fTemp604: F32 = fSlow101 * fTemp601 - fSlow98 * self.fRec365[1];
			self.fVec105[(self.IOTA0 & 16383) as usize] = fTemp604 - fTemp603;
			let mut fTemp605: F32 = self.fVec105[((i32::wrapping_sub(self.IOTA0, iTemp523)) & 16383) as usize];
			self.fVec106[0] = fTemp605;
			self.fRec367[0] = 0.70710677 * (fTemp522 * fTemp605 / fTemp521 + self.fVec106[1]) - self.fRec367[1] * fTemp522 / fTemp521;
			self.fRec365[0] = self.fRec367[0];
			self.fRec369[0] = 0.9999 * (self.fRec369[1] + ((i32::wrapping_mul(iTemp357, iSlow104)) as F32)) + fSlow105;
			let mut fTemp606: F32 = self.fRec369[0] + -1.49999;
			let mut fTemp607: F32 = F32::floor(fTemp606);
			let mut fTemp608: F32 = self.fRec369[0] - fTemp607;
			let mut fTemp609: F32 = fTemp607 + (2.0 - self.fRec369[0]);
			self.fVec107[(self.IOTA0 & 16383) as usize] = fTemp604 + fTemp603;
			let mut fTemp610: F32 = self.fVec107[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp606) as i32))))) & 16383) as usize];
			self.fVec108[0] = fTemp610;
			self.fRec368[0] = 0.70710677 * (fTemp609 * fTemp610 / fTemp608 + self.fVec108[1]) - self.fRec368[1] * fTemp609 / fTemp608;
			self.fRec366[0] = self.fRec368[0];
			let mut fTemp611: F32 = fSlow101 * self.fRec365[1] + fSlow98 * fTemp601;
			self.fRec373[0] = 0.9999 * (self.fRec373[1] + ((i32::wrapping_mul(iTemp357, iSlow106)) as F32)) + fSlow107;
			let mut fTemp612: F32 = self.fRec373[0] + -1.49999;
			let mut fTemp613: F32 = F32::floor(fTemp612);
			let mut fTemp614: F32 = self.fRec373[0] - fTemp613;
			let mut fTemp615: F32 = fTemp613 + (2.0 - self.fRec373[0]);
			let mut fTemp616: F32 = fSlow101 * self.fRec366[1] + fSlow98 * fTemp602;
			let mut fTemp617: F32 = fSlow101 * fTemp616 - fSlow98 * self.fRec371[1];
			let mut fTemp618: F32 = fSlow101 * fTemp611 - fSlow98 * self.fRec370[1];
			self.fVec109[(self.IOTA0 & 16383) as usize] = fTemp618 - fTemp617;
			let mut fTemp619: F32 = self.fVec109[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp612) as i32))))) & 16383) as usize];
			self.fVec110[0] = fTemp619;
			self.fRec372[0] = 0.70710677 * (fTemp615 * fTemp619 / fTemp614 + self.fVec110[1]) - self.fRec372[1] * fTemp615 / fTemp614;
			self.fRec370[0] = self.fRec372[0];
			self.fVec111[(self.IOTA0 & 16383) as usize] = fTemp618 + fTemp617;
			let mut fTemp620: F32 = self.fVec111[((i32::wrapping_sub(self.IOTA0, iTemp538)) & 16383) as usize];
			self.fVec112[0] = fTemp620;
			self.fRec374[0] = 0.70710677 * (fTemp537 * fTemp620 / fTemp536 + self.fVec112[1]) - fTemp537 * self.fRec374[1] / fTemp536;
			self.fRec371[0] = self.fRec374[0];
			self.fRec256[0] = fSlow101 * self.fRec370[1] + fSlow98 * fTemp611;
			self.fRec257[0] = fSlow101 * self.fRec371[1] + fSlow98 * fTemp616;
			self.fRec375[0] = fSlow108 + self.fConst2 * self.fRec375[1];
			let mut fTemp621: F32 = self.fRec375[0] * (fSlow40 * (self.fRec256[0] + self.fRec257[0]) + fSlow41 * fTemp356);
			*output0 = fTemp621;
			*output1 = fTemp621;
			self.iVec0[1] = self.iVec0[0];
			self.fRec1[1] = self.fRec1[0];
			self.fRec3[1] = self.fRec3[0];
			self.fVec1[1] = self.fVec1[0];
			self.IOTA0 = i32::wrapping_add(self.IOTA0, 1);
			self.fRec2[1] = self.fRec2[0];
			self.fRec5[1] = self.fRec5[0];
			self.fVec3[1] = self.fVec3[0];
			self.fRec4[1] = self.fRec4[0];
			self.fRec7[1] = self.fRec7[0];
			self.fVec5[1] = self.fVec5[0];
			self.fRec6[1] = self.fRec6[0];
			self.fRec8[1] = self.fRec8[0];
			self.fRec9[1] = self.fRec9[0];
			self.fRec11[1] = self.fRec11[0];
			self.fVec7[1] = self.fVec7[0];
			self.fRec10[1] = self.fRec10[0];
			self.fRec13[1] = self.fRec13[0];
			self.fVec9[1] = self.fVec9[0];
			self.fRec12[1] = self.fRec12[0];
			self.fRec15[1] = self.fRec15[0];
			self.fVec11[1] = self.fVec11[0];
			self.fRec14[1] = self.fRec14[0];
			self.fRec16[1] = self.fRec16[0];
			self.fRec17[1] = self.fRec17[0];
			self.fRec19[1] = self.fRec19[0];
			self.fVec13[1] = self.fVec13[0];
			self.fRec18[1] = self.fRec18[0];
			self.fRec21[1] = self.fRec21[0];
			self.fVec15[1] = self.fVec15[0];
			self.fRec20[1] = self.fRec20[0];
			self.fRec23[1] = self.fRec23[0];
			self.fVec17[1] = self.fVec17[0];
			self.fRec22[1] = self.fRec22[0];
			self.fRec24[1] = self.fRec24[0];
			self.fRec25[1] = self.fRec25[0];
			self.fRec27[1] = self.fRec27[0];
			self.fVec19[1] = self.fVec19[0];
			self.fRec26[1] = self.fRec26[0];
			self.fRec29[1] = self.fRec29[0];
			self.fVec21[1] = self.fVec21[0];
			self.fRec28[1] = self.fRec28[0];
			self.fRec31[1] = self.fRec31[0];
			self.fVec23[1] = self.fVec23[0];
			self.fRec30[1] = self.fRec30[0];
			self.fRec32[1] = self.fRec32[0];
			self.fRec33[1] = self.fRec33[0];
			self.fRec34[1] = self.fRec34[0];
			self.fRec36[1] = self.fRec36[0];
			self.fRec37[1] = self.fRec37[0];
			self.fRec67[1] = self.fRec67[0];
			self.fRec71[1] = self.fRec71[0];
			self.fRec76[1] = self.fRec76[0];
			self.iVec25[1] = self.iVec25[0];
			self.iRec77[1] = self.iRec77[0];
			self.fRec74[1] = self.fRec74[0];
			self.fRec73[1] = self.fRec73[0];
			for j0 in (1..=3).rev() {
				self.fRec78[(j0) as usize] = self.fRec78[(i32::wrapping_sub(j0, 1)) as usize];
			}
			self.fVec26[1] = self.fVec26[0];
			self.fVec27[1] = self.fVec27[0];
			self.iRec80[1] = self.iRec80[0];
			self.iRec82[1] = self.iRec82[0];
			self.fRec81[2] = self.fRec81[1];
			self.fRec81[1] = self.fRec81[0];
			self.fVec28[2] = self.fVec28[1];
			self.fVec28[1] = self.fVec28[0];
			self.fRec63[1] = self.fRec63[0];
			self.fRec59[1] = self.fRec59[0];
			self.fRec57[1] = self.fRec57[0];
			for j1 in (1..=3).rev() {
				self.fRec53[(j1) as usize] = self.fRec53[(i32::wrapping_sub(j1, 1)) as usize];
			}
			self.fRec48[1] = self.fRec48[0];
			self.fRec42[1] = self.fRec42[0];
			self.fRec41[1] = self.fRec41[0];
			self.fRec40[1] = self.fRec40[0];
			self.fRec38[1] = self.fRec38[0];
			self.fRec35[1] = self.fRec35[0];
			self.fRec84[1] = self.fRec84[0];
			self.fRec114[1] = self.fRec114[0];
			self.fRec118[1] = self.fRec118[0];
			self.fRec123[1] = self.fRec123[0];
			self.iVec29[1] = self.iVec29[0];
			self.iRec124[1] = self.iRec124[0];
			self.fRec121[1] = self.fRec121[0];
			self.fRec120[1] = self.fRec120[0];
			for j2 in (1..=3).rev() {
				self.fRec125[(j2) as usize] = self.fRec125[(i32::wrapping_sub(j2, 1)) as usize];
			}
			self.fVec30[1] = self.fVec30[0];
			self.fVec31[1] = self.fVec31[0];
			self.iRec127[1] = self.iRec127[0];
			self.fRec128[2] = self.fRec128[1];
			self.fRec128[1] = self.fRec128[0];
			self.fVec32[2] = self.fVec32[1];
			self.fVec32[1] = self.fVec32[0];
			self.fRec110[1] = self.fRec110[0];
			self.fRec106[1] = self.fRec106[0];
			self.fRec104[1] = self.fRec104[0];
			for j3 in (1..=3).rev() {
				self.fRec100[(j3) as usize] = self.fRec100[(i32::wrapping_sub(j3, 1)) as usize];
			}
			self.fRec95[1] = self.fRec95[0];
			self.fRec89[1] = self.fRec89[0];
			self.fRec88[1] = self.fRec88[0];
			self.fRec87[1] = self.fRec87[0];
			self.fRec85[1] = self.fRec85[0];
			self.fRec83[1] = self.fRec83[0];
			self.fRec130[1] = self.fRec130[0];
			self.fRec160[1] = self.fRec160[0];
			self.fRec164[1] = self.fRec164[0];
			self.fRec169[1] = self.fRec169[0];
			self.iVec33[1] = self.iVec33[0];
			self.iRec170[1] = self.iRec170[0];
			self.fRec167[1] = self.fRec167[0];
			self.fRec166[1] = self.fRec166[0];
			for j4 in (1..=3).rev() {
				self.fRec171[(j4) as usize] = self.fRec171[(i32::wrapping_sub(j4, 1)) as usize];
			}
			self.fVec34[1] = self.fVec34[0];
			self.fVec35[1] = self.fVec35[0];
			self.iRec173[1] = self.iRec173[0];
			self.fRec174[2] = self.fRec174[1];
			self.fRec174[1] = self.fRec174[0];
			self.fVec36[2] = self.fVec36[1];
			self.fVec36[1] = self.fVec36[0];
			self.fRec156[1] = self.fRec156[0];
			self.fRec152[1] = self.fRec152[0];
			self.fRec150[1] = self.fRec150[0];
			for j5 in (1..=3).rev() {
				self.fRec146[(j5) as usize] = self.fRec146[(i32::wrapping_sub(j5, 1)) as usize];
			}
			self.fRec141[1] = self.fRec141[0];
			self.fRec135[1] = self.fRec135[0];
			self.fRec134[1] = self.fRec134[0];
			self.fRec133[1] = self.fRec133[0];
			self.fRec131[1] = self.fRec131[0];
			self.fRec129[1] = self.fRec129[0];
			self.fRec176[1] = self.fRec176[0];
			self.fRec206[1] = self.fRec206[0];
			self.fRec210[1] = self.fRec210[0];
			self.fRec215[1] = self.fRec215[0];
			self.iVec37[1] = self.iVec37[0];
			self.iRec216[1] = self.iRec216[0];
			self.fRec213[1] = self.fRec213[0];
			self.fRec212[1] = self.fRec212[0];
			for j6 in (1..=3).rev() {
				self.fRec217[(j6) as usize] = self.fRec217[(i32::wrapping_sub(j6, 1)) as usize];
			}
			self.fVec38[1] = self.fVec38[0];
			self.fVec39[1] = self.fVec39[0];
			self.iRec219[1] = self.iRec219[0];
			self.fRec220[2] = self.fRec220[1];
			self.fRec220[1] = self.fRec220[0];
			self.fVec40[2] = self.fVec40[1];
			self.fVec40[1] = self.fVec40[0];
			self.fRec202[1] = self.fRec202[0];
			self.fRec198[1] = self.fRec198[0];
			self.fRec196[1] = self.fRec196[0];
			for j7 in (1..=3).rev() {
				self.fRec192[(j7) as usize] = self.fRec192[(i32::wrapping_sub(j7, 1)) as usize];
			}
			self.fRec187[1] = self.fRec187[0];
			self.fRec181[1] = self.fRec181[0];
			self.fRec180[1] = self.fRec180[0];
			self.fRec179[1] = self.fRec179[0];
			self.fRec177[1] = self.fRec177[0];
			self.fRec175[1] = self.fRec175[0];
			self.fRec221[1] = self.fRec221[0];
			self.fRec223[1] = self.fRec223[0];
			self.fRec222[1] = self.fRec222[0];
			self.fRec228[1] = self.fRec228[0];
			self.fRec226[1] = self.fRec226[0];
			self.fRec229[1] = self.fRec229[0];
			self.fRec225[2] = self.fRec225[1];
			self.fRec225[1] = self.fRec225[0];
			self.fRec224[2] = self.fRec224[1];
			self.fRec224[1] = self.fRec224[0];
			self.fRec230[1] = self.fRec230[0];
			self.fRec235[1] = self.fRec235[0];
			self.fRec233[1] = self.fRec233[0];
			self.fRec236[1] = self.fRec236[0];
			self.fRec232[2] = self.fRec232[1];
			self.fRec232[1] = self.fRec232[0];
			self.fRec231[2] = self.fRec231[1];
			self.fRec231[1] = self.fRec231[0];
			self.fRec237[1] = self.fRec237[0];
			self.fRec242[1] = self.fRec242[0];
			self.fRec240[1] = self.fRec240[0];
			self.fRec243[1] = self.fRec243[0];
			self.fRec239[2] = self.fRec239[1];
			self.fRec239[1] = self.fRec239[0];
			self.fRec238[2] = self.fRec238[1];
			self.fRec238[1] = self.fRec238[0];
			self.fRec244[1] = self.fRec244[0];
			self.fRec249[1] = self.fRec249[0];
			self.fRec247[1] = self.fRec247[0];
			self.fRec250[1] = self.fRec250[0];
			self.fRec246[2] = self.fRec246[1];
			self.fRec246[1] = self.fRec246[0];
			self.fRec245[2] = self.fRec245[1];
			self.fRec245[1] = self.fRec245[0];
			self.fRec251[1] = self.fRec251[0];
			self.fRec252[1] = self.fRec252[0];
			self.fRec253[1] = self.fRec253[0];
			self.fRec255[1] = self.fRec255[0];
			self.fRec266[1] = self.fRec266[0];
			self.fRec268[1] = self.fRec268[0];
			self.fRec272[1] = self.fRec272[0];
			self.fVec42[1] = self.fVec42[0];
			self.fRec271[1] = self.fRec271[0];
			self.fRec269[1] = self.fRec269[0];
			self.fRec274[1] = self.fRec274[0];
			self.fVec44[1] = self.fVec44[0];
			self.fRec273[1] = self.fRec273[0];
			self.fRec270[1] = self.fRec270[0];
			self.fRec278[1] = self.fRec278[0];
			self.fVec46[1] = self.fVec46[0];
			self.fRec277[1] = self.fRec277[0];
			self.fRec275[1] = self.fRec275[0];
			self.fRec280[1] = self.fRec280[0];
			self.fVec48[1] = self.fVec48[0];
			self.fRec279[1] = self.fRec279[0];
			self.fRec276[1] = self.fRec276[0];
			self.fRec284[1] = self.fRec284[0];
			self.fVec50[1] = self.fVec50[0];
			self.fRec283[1] = self.fRec283[0];
			self.fRec281[1] = self.fRec281[0];
			self.fRec286[1] = self.fRec286[0];
			self.fVec52[1] = self.fVec52[0];
			self.fRec285[1] = self.fRec285[0];
			self.fRec282[1] = self.fRec282[0];
			self.fRec290[1] = self.fRec290[0];
			self.fVec54[1] = self.fVec54[0];
			self.fRec289[1] = self.fRec289[0];
			self.fRec287[1] = self.fRec287[0];
			self.fRec292[1] = self.fRec292[0];
			self.fVec56[1] = self.fVec56[0];
			self.fRec291[1] = self.fRec291[0];
			self.fRec288[1] = self.fRec288[0];
			self.fRec296[1] = self.fRec296[0];
			self.fVec58[1] = self.fVec58[0];
			self.fRec295[1] = self.fRec295[0];
			self.fRec293[1] = self.fRec293[0];
			self.fRec298[1] = self.fRec298[0];
			self.fVec60[1] = self.fVec60[0];
			self.fRec297[1] = self.fRec297[0];
			self.fRec294[1] = self.fRec294[0];
			self.fRec299[1] = self.fRec299[0];
			self.fRec300[1] = self.fRec300[0];
			self.fVec63[1] = self.fVec63[0];
			self.fRec267[1] = self.fRec267[0];
			self.fRec304[1] = self.fRec304[0];
			self.fRec306[1] = self.fRec306[0];
			self.fVec66[1] = self.fVec66[0];
			self.fRec305[1] = self.fRec305[0];
			self.fVec68[1] = self.fVec68[0];
			self.fRec303[1] = self.fRec303[0];
			self.fRec301[1] = self.fRec301[0];
			self.fRec308[1] = self.fRec308[0];
			self.fVec70[1] = self.fVec70[0];
			self.fRec307[1] = self.fRec307[0];
			self.fRec302[1] = self.fRec302[0];
			self.fRec312[1] = self.fRec312[0];
			self.fVec72[1] = self.fVec72[0];
			self.fRec311[1] = self.fRec311[0];
			self.fRec309[1] = self.fRec309[0];
			self.fRec314[1] = self.fRec314[0];
			self.fVec74[1] = self.fVec74[0];
			self.fRec313[1] = self.fRec313[0];
			self.fRec310[1] = self.fRec310[0];
			self.fRec318[1] = self.fRec318[0];
			self.fVec76[1] = self.fVec76[0];
			self.fRec317[1] = self.fRec317[0];
			self.fRec315[1] = self.fRec315[0];
			self.fRec320[1] = self.fRec320[0];
			self.fVec78[1] = self.fVec78[0];
			self.fRec319[1] = self.fRec319[0];
			self.fRec316[1] = self.fRec316[0];
			self.fRec324[1] = self.fRec324[0];
			self.fVec80[1] = self.fVec80[0];
			self.fRec323[1] = self.fRec323[0];
			self.fRec321[1] = self.fRec321[0];
			self.fRec326[1] = self.fRec326[0];
			self.fVec82[1] = self.fVec82[0];
			self.fRec325[1] = self.fRec325[0];
			self.fRec322[1] = self.fRec322[0];
			self.fRec330[1] = self.fRec330[0];
			self.fVec84[1] = self.fVec84[0];
			self.fRec329[1] = self.fRec329[0];
			self.fRec327[1] = self.fRec327[0];
			self.fRec332[1] = self.fRec332[0];
			self.fVec86[1] = self.fVec86[0];
			self.fRec331[1] = self.fRec331[0];
			self.fRec328[1] = self.fRec328[0];
			self.fVec89[1] = self.fVec89[0];
			self.fRec265[1] = self.fRec265[0];
			self.fRec264[1] = self.fRec264[0];
			self.fRec263[2] = self.fRec263[1];
			self.fRec263[1] = self.fRec263[0];
			self.fRec262[2] = self.fRec262[1];
			self.fRec262[1] = self.fRec262[0];
			self.fVec90[1] = self.fVec90[0];
			self.fRec261[1] = self.fRec261[0];
			self.fRec260[2] = self.fRec260[1];
			self.fRec260[1] = self.fRec260[0];
			self.fRec259[2] = self.fRec259[1];
			self.fRec259[1] = self.fRec259[0];
			self.fRec335[1] = self.fRec335[0];
			self.fRec334[2] = self.fRec334[1];
			self.fRec334[1] = self.fRec334[0];
			self.fRec333[2] = self.fRec333[1];
			self.fRec333[1] = self.fRec333[0];
			self.fRec339[1] = self.fRec339[0];
			self.fRec338[2] = self.fRec338[1];
			self.fRec338[1] = self.fRec338[0];
			self.fRec337[2] = self.fRec337[1];
			self.fRec337[1] = self.fRec337[0];
			self.fRec336[2] = self.fRec336[1];
			self.fRec336[1] = self.fRec336[0];
			self.fRec258[1] = self.fRec258[0];
			self.fRec351[1] = self.fRec351[0];
			self.fVec94[1] = self.fVec94[0];
			self.fRec350[1] = self.fRec350[0];
			self.fRec349[1] = self.fRec349[0];
			self.fRec348[2] = self.fRec348[1];
			self.fRec348[1] = self.fRec348[0];
			self.fRec347[2] = self.fRec347[1];
			self.fRec347[1] = self.fRec347[0];
			self.fVec95[1] = self.fVec95[0];
			self.fRec346[1] = self.fRec346[0];
			self.fRec345[2] = self.fRec345[1];
			self.fRec345[1] = self.fRec345[0];
			self.fRec344[2] = self.fRec344[1];
			self.fRec344[1] = self.fRec344[0];
			self.fRec354[1] = self.fRec354[0];
			self.fRec353[2] = self.fRec353[1];
			self.fRec353[1] = self.fRec353[0];
			self.fRec352[2] = self.fRec352[1];
			self.fRec352[1] = self.fRec352[0];
			self.fRec358[1] = self.fRec358[0];
			self.fRec357[2] = self.fRec357[1];
			self.fRec357[1] = self.fRec357[0];
			self.fRec356[2] = self.fRec356[1];
			self.fRec356[1] = self.fRec356[0];
			self.fRec355[2] = self.fRec355[1];
			self.fRec355[1] = self.fRec355[0];
			self.fRec343[1] = self.fRec343[0];
			self.fVec98[1] = self.fVec98[0];
			self.fRec342[1] = self.fRec342[0];
			self.fRec340[1] = self.fRec340[0];
			self.fRec360[1] = self.fRec360[0];
			self.fVec100[1] = self.fVec100[0];
			self.fRec359[1] = self.fRec359[0];
			self.fRec341[1] = self.fRec341[0];
			self.fVec102[1] = self.fVec102[0];
			self.fRec363[1] = self.fRec363[0];
			self.fRec361[1] = self.fRec361[0];
			self.fVec104[1] = self.fVec104[0];
			self.fRec364[1] = self.fRec364[0];
			self.fRec362[1] = self.fRec362[0];
			self.fVec106[1] = self.fVec106[0];
			self.fRec367[1] = self.fRec367[0];
			self.fRec365[1] = self.fRec365[0];
			self.fRec369[1] = self.fRec369[0];
			self.fVec108[1] = self.fVec108[0];
			self.fRec368[1] = self.fRec368[0];
			self.fRec366[1] = self.fRec366[0];
			self.fRec373[1] = self.fRec373[0];
			self.fVec110[1] = self.fVec110[0];
			self.fRec372[1] = self.fRec372[0];
			self.fRec370[1] = self.fRec370[0];
			self.fVec112[1] = self.fVec112[0];
			self.fRec374[1] = self.fRec374[0];
			self.fRec371[1] = self.fRec371[0];
			self.fRec256[1] = self.fRec256[0];
			self.fRec257[1] = self.fRec257[0];
			self.fRec375[1] = self.fRec375[0];
		}
	}

}


}
pub use dsp::mydsp as Instrument;
