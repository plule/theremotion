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
		for l0 in 0..2 {
			self.iRec2[(l0) as usize] = 0;
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
	fHslider0: F32,
	fHslider1: F32,
	fSampleRate: i32,
	fConst0: F32,
	fConst1: F32,
	iVec0: [i32;2],
	fConst2: F32,
	fRec1: [F32;2],
	fConst3: F32,
	fRec3: [F32;2],
	fVec1: [F32;2],
	IOTA0: i32,
	fVec2: [F32;4096],
	fConst4: F32,
	fConst5: F32,
	fRec0: [F32;2],
	fRec5: [F32;2],
	fRec6: [F32;2],
	fVec3: [F32;2],
	fVec4: [F32;4096],
	fRec4: [F32;2],
	fRec8: [F32;2],
	fRec9: [F32;2],
	fVec5: [F32;2],
	fVec6: [F32;4096],
	fRec7: [F32;2],
	fHslider2: F32,
	fRec10: [F32;2],
	fHslider3: F32,
	fRec12: [F32;2],
	fRec13: [F32;2],
	fVec7: [F32;2],
	fVec8: [F32;4096],
	fRec11: [F32;2],
	fRec15: [F32;2],
	fRec16: [F32;2],
	fVec9: [F32;2],
	fVec10: [F32;4096],
	fRec14: [F32;2],
	fRec18: [F32;2],
	fRec19: [F32;2],
	fVec11: [F32;2],
	fVec12: [F32;4096],
	fRec17: [F32;2],
	fHslider4: F32,
	fRec20: [F32;2],
	fHslider5: F32,
	fRec22: [F32;2],
	fRec23: [F32;2],
	fVec13: [F32;2],
	fVec14: [F32;4096],
	fRec21: [F32;2],
	fRec25: [F32;2],
	fRec26: [F32;2],
	fVec15: [F32;2],
	fVec16: [F32;4096],
	fRec24: [F32;2],
	fRec28: [F32;2],
	fRec29: [F32;2],
	fVec17: [F32;2],
	fVec18: [F32;4096],
	fRec27: [F32;2],
	fHslider6: F32,
	fRec30: [F32;2],
	fHslider7: F32,
	fRec32: [F32;2],
	fRec33: [F32;2],
	fVec19: [F32;2],
	fVec20: [F32;4096],
	fRec31: [F32;2],
	fRec35: [F32;2],
	fRec36: [F32;2],
	fVec21: [F32;2],
	fVec22: [F32;4096],
	fRec34: [F32;2],
	fRec38: [F32;2],
	fRec39: [F32;2],
	fVec23: [F32;2],
	fVec24: [F32;4096],
	fRec37: [F32;2],
	fHslider8: F32,
	fRec40: [F32;2],
	fHslider9: F32,
	fRec41: [F32;2],
	fConst6: F32,
	fHslider10: F32,
	fRec42: [F32;2],
	fHslider11: F32,
	fRec45: [F32;2],
	fHslider12: F32,
	fRec44: [F32;2],
	fConst7: F32,
	fConst8: F32,
	fConst9: F32,
	fRec75: [F32;2],
	fRec79: [F32;2],
	fConst10: F32,
	fConst11: F32,
	fRec84: [F32;2],
	iVec25: [i32;2],
	iConst12: i32,
	iRec85: [i32;2],
	fConst13: F32,
	fRec82: [F32;2],
	fRec81: [F32;2],
	fRec86: [F32;4],
	fRec87: [F32;2048],
	fVec26: [F32;2],
	fConst14: F32,
	fConst15: F32,
	fButton0: F32,
	fVec27: [F32;2],
	iRec88: [i32;2],
	iRec90: [i32;2],
	fRec89: [F32;3],
	fVec28: [F32;3],
	fRec80: [F32;2048],
	fRec71: [F32;2],
	fRec67: [F32;2],
	fRec63: [F32;2048],
	fRec65: [F32;2],
	fHslider13: F32,
	fRec61: [F32;4],
	fRec56: [F32;2],
	fRec52: [F32;2048],
	fRec50: [F32;2],
	fConst16: F32,
	fRec49: [F32;2],
	fRec48: [F32;2],
	fRec46: [F32;2],
	fRec43: [F32;2],
	fHslider14: F32,
	fRec92: [F32;2],
	fRec122: [F32;2],
	fRec126: [F32;2],
	fRec131: [F32;2],
	iVec29: [i32;2],
	iRec132: [i32;2],
	fRec129: [F32;2],
	fRec128: [F32;2],
	fRec133: [F32;4],
	fRec134: [F32;2048],
	fVec30: [F32;2],
	fButton1: F32,
	fVec31: [F32;2],
	iRec135: [i32;2],
	fRec136: [F32;3],
	fVec32: [F32;3],
	fRec127: [F32;2048],
	fRec118: [F32;2],
	fRec114: [F32;2],
	fRec110: [F32;2048],
	fRec112: [F32;2],
	fRec108: [F32;4],
	fRec103: [F32;2],
	fRec99: [F32;2048],
	fRec97: [F32;2],
	fRec96: [F32;2],
	fRec95: [F32;2],
	fRec93: [F32;2],
	fRec91: [F32;2],
	fHslider15: F32,
	fRec138: [F32;2],
	fRec168: [F32;2],
	fRec172: [F32;2],
	fRec177: [F32;2],
	iVec33: [i32;2],
	iRec178: [i32;2],
	fRec175: [F32;2],
	fRec174: [F32;2],
	fRec179: [F32;4],
	fRec180: [F32;2048],
	fVec34: [F32;2],
	fButton2: F32,
	fVec35: [F32;2],
	iRec181: [i32;2],
	fRec182: [F32;3],
	fVec36: [F32;3],
	fRec173: [F32;2048],
	fRec164: [F32;2],
	fRec160: [F32;2],
	fRec156: [F32;2048],
	fRec158: [F32;2],
	fRec154: [F32;4],
	fRec149: [F32;2],
	fRec145: [F32;2048],
	fRec143: [F32;2],
	fRec142: [F32;2],
	fRec141: [F32;2],
	fRec139: [F32;2],
	fRec137: [F32;2],
	fHslider16: F32,
	fRec184: [F32;2],
	fRec214: [F32;2],
	fRec218: [F32;2],
	fRec223: [F32;2],
	iVec37: [i32;2],
	iRec224: [i32;2],
	fRec221: [F32;2],
	fRec220: [F32;2],
	fRec225: [F32;4],
	fRec226: [F32;2048],
	fVec38: [F32;2],
	fButton3: F32,
	fVec39: [F32;2],
	iRec227: [i32;2],
	fRec228: [F32;3],
	fVec40: [F32;3],
	fRec219: [F32;2048],
	fRec210: [F32;2],
	fRec206: [F32;2],
	fRec202: [F32;2048],
	fRec204: [F32;2],
	fRec200: [F32;4],
	fRec195: [F32;2],
	fRec191: [F32;2048],
	fRec189: [F32;2],
	fRec188: [F32;2],
	fRec187: [F32;2],
	fRec185: [F32;2],
	fRec183: [F32;2],
	fHslider17: F32,
	fRec229: [F32;2],
	fHslider18: F32,
	fRec231: [F32;2],
	fHslider19: F32,
	fRec230: [F32;2],
	fConst17: F32,
	fRec236: [F32;2],
	fRec234: [F32;2],
	fHslider20: F32,
	fRec237: [F32;2],
	fRec233: [F32;3],
	fRec232: [F32;3],
	fHslider21: F32,
	fRec238: [F32;2],
	fRec243: [F32;2],
	fRec241: [F32;2],
	fHslider22: F32,
	fRec244: [F32;2],
	fRec240: [F32;3],
	fRec239: [F32;3],
	fHslider23: F32,
	fRec245: [F32;2],
	fRec250: [F32;2],
	fRec248: [F32;2],
	fHslider24: F32,
	fRec251: [F32;2],
	fRec247: [F32;3],
	fRec246: [F32;3],
	fHslider25: F32,
	fRec252: [F32;2],
	fRec257: [F32;2],
	fRec255: [F32;2],
	fHslider26: F32,
	fRec258: [F32;2],
	fRec254: [F32;3],
	fRec253: [F32;3],
	fHslider27: F32,
	fRec259: [F32;2],
	fHslider28: F32,
	fRec260: [F32;2],
	fHslider29: F32,
	fRec261: [F32;2],
	fConst18: F32,
	fHslider30: F32,
	fRec263: [F32;2],
	fHslider31: F32,
	fRec262: [F32;2097152],
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
	fRec274: [F32;2],
	fRec276: [F32;2],
	fRec280: [F32;2],
	fVec41: [F32;16384],
	fVec42: [F32;2],
	fRec279: [F32;2],
	fRec277: [F32;2],
	fRec282: [F32;2],
	fVec43: [F32;16384],
	fVec44: [F32;2],
	fRec281: [F32;2],
	fRec278: [F32;2],
	fRec286: [F32;2],
	fVec45: [F32;16384],
	fVec46: [F32;2],
	fRec285: [F32;2],
	fRec283: [F32;2],
	fRec288: [F32;2],
	fVec47: [F32;16384],
	fVec48: [F32;2],
	fRec287: [F32;2],
	fRec284: [F32;2],
	fRec292: [F32;2],
	fVec49: [F32;16384],
	fVec50: [F32;2],
	fRec291: [F32;2],
	fRec289: [F32;2],
	fRec294: [F32;2],
	fVec51: [F32;16384],
	fVec52: [F32;2],
	fRec293: [F32;2],
	fRec290: [F32;2],
	fRec298: [F32;2],
	fVec53: [F32;16384],
	fVec54: [F32;2],
	fRec297: [F32;2],
	fRec295: [F32;2],
	fRec300: [F32;2],
	fVec55: [F32;16384],
	fVec56: [F32;2],
	fRec299: [F32;2],
	fRec296: [F32;2],
	fRec304: [F32;2],
	fVec57: [F32;16384],
	fVec58: [F32;2],
	fRec303: [F32;2],
	fRec301: [F32;2],
	fRec306: [F32;2],
	fVec59: [F32;16384],
	fVec60: [F32;2],
	fRec305: [F32;2],
	fRec302: [F32;2],
	fVec61: [F32;1024],
	fHslider34: F32,
	fRec307: [F32;2],
	fRec308: [F32;2],
	fHslider35: F32,
	fVec62: [F32;16384],
	fVec63: [F32;2],
	fRec275: [F32;2],
	fRec312: [F32;2],
	fRec314: [F32;2],
	fVec64: [F32;1024],
	fVec65: [F32;16384],
	fVec66: [F32;2],
	fRec313: [F32;2],
	fVec67: [F32;16384],
	fVec68: [F32;2],
	fRec311: [F32;2],
	fRec309: [F32;2],
	fRec316: [F32;2],
	fVec69: [F32;16384],
	fVec70: [F32;2],
	fRec315: [F32;2],
	fRec310: [F32;2],
	fRec320: [F32;2],
	fVec71: [F32;16384],
	fVec72: [F32;2],
	fRec319: [F32;2],
	fRec317: [F32;2],
	fRec322: [F32;2],
	fVec73: [F32;16384],
	fVec74: [F32;2],
	fRec321: [F32;2],
	fRec318: [F32;2],
	fRec326: [F32;2],
	fVec75: [F32;16384],
	fVec76: [F32;2],
	fRec325: [F32;2],
	fRec323: [F32;2],
	fRec328: [F32;2],
	fVec77: [F32;16384],
	fVec78: [F32;2],
	fRec327: [F32;2],
	fRec324: [F32;2],
	fRec332: [F32;2],
	fVec79: [F32;16384],
	fVec80: [F32;2],
	fRec331: [F32;2],
	fRec329: [F32;2],
	fRec334: [F32;2],
	fVec81: [F32;16384],
	fVec82: [F32;2],
	fRec333: [F32;2],
	fRec330: [F32;2],
	fRec338: [F32;2],
	fVec83: [F32;16384],
	fVec84: [F32;2],
	fRec337: [F32;2],
	fRec335: [F32;2],
	fRec340: [F32;2],
	fVec85: [F32;16384],
	fVec86: [F32;2],
	fRec339: [F32;2],
	fRec336: [F32;2],
	fVec87: [F32;16384],
	fVec88: [F32;16384],
	fVec89: [F32;2],
	fRec273: [F32;2],
	fConst39: F32,
	fRec272: [F32;2],
	fRec271: [F32;3],
	fRec270: [F32;3],
	fVec90: [F32;2],
	fConst40: F32,
	fConst42: F32,
	fRec269: [F32;2],
	fRec268: [F32;3],
	fRec267: [F32;3],
	fConst43: F32,
	fConst44: F32,
	fConst45: F32,
	fRec343: [F32;2],
	fRec342: [F32;3],
	fConst46: F32,
	fRec341: [F32;3],
	fConst47: F32,
	fConst48: F32,
	fConst49: F32,
	fRec347: [F32;2],
	fRec346: [F32;3],
	fConst50: F32,
	fRec345: [F32;3],
	fRec344: [F32;3],
	fHslider36: F32,
	fVec91: [F32;1024],
	fHslider37: F32,
	fRec266: [F32;2],
	fHslider38: F32,
	fRec359: [F32;2],
	fVec92: [F32;16384],
	fVec93: [F32;16384],
	fVec94: [F32;2],
	fRec358: [F32;2],
	fRec357: [F32;2],
	fRec356: [F32;3],
	fRec355: [F32;3],
	fVec95: [F32;2],
	fRec354: [F32;2],
	fRec353: [F32;3],
	fRec352: [F32;3],
	fRec362: [F32;2],
	fRec361: [F32;3],
	fRec360: [F32;3],
	fRec366: [F32;2],
	fRec365: [F32;3],
	fRec364: [F32;3],
	fRec363: [F32;3],
	fVec96: [F32;1024],
	fRec351: [F32;2],
	fVec97: [F32;16384],
	fVec98: [F32;2],
	fRec350: [F32;2],
	fRec348: [F32;2],
	fRec368: [F32;2],
	fVec99: [F32;16384],
	fVec100: [F32;2],
	fRec367: [F32;2],
	fRec349: [F32;2],
	fVec101: [F32;16384],
	fVec102: [F32;2],
	fRec371: [F32;2],
	fRec369: [F32;2],
	fVec103: [F32;16384],
	fVec104: [F32;2],
	fRec372: [F32;2],
	fRec370: [F32;2],
	fVec105: [F32;16384],
	fVec106: [F32;2],
	fRec375: [F32;2],
	fRec373: [F32;2],
	fRec377: [F32;2],
	fVec107: [F32;16384],
	fVec108: [F32;2],
	fRec376: [F32;2],
	fRec374: [F32;2],
	fRec381: [F32;2],
	fVec109: [F32;16384],
	fVec110: [F32;2],
	fRec380: [F32;2],
	fRec378: [F32;2],
	fVec111: [F32;16384],
	fVec112: [F32;2],
	fRec382: [F32;2],
	fRec379: [F32;2],
	fRec264: [F32;2],
	fRec265: [F32;2],
	fHslider39: F32,
	fRec383: [F32;2],
}

impl FaustDsp for mydsp {
	type T = F32;
		
	fn new() -> mydsp { 
		mydsp {
			fHslider0: 0.0,
			fHslider1: 0.0,
			fSampleRate: 0,
			fConst0: 0.0,
			fConst1: 0.0,
			iVec0: [0;2],
			fConst2: 0.0,
			fRec1: [0.0;2],
			fConst3: 0.0,
			fRec3: [0.0;2],
			fVec1: [0.0;2],
			IOTA0: 0,
			fVec2: [0.0;4096],
			fConst4: 0.0,
			fConst5: 0.0,
			fRec0: [0.0;2],
			fRec5: [0.0;2],
			fRec6: [0.0;2],
			fVec3: [0.0;2],
			fVec4: [0.0;4096],
			fRec4: [0.0;2],
			fRec8: [0.0;2],
			fRec9: [0.0;2],
			fVec5: [0.0;2],
			fVec6: [0.0;4096],
			fRec7: [0.0;2],
			fHslider2: 0.0,
			fRec10: [0.0;2],
			fHslider3: 0.0,
			fRec12: [0.0;2],
			fRec13: [0.0;2],
			fVec7: [0.0;2],
			fVec8: [0.0;4096],
			fRec11: [0.0;2],
			fRec15: [0.0;2],
			fRec16: [0.0;2],
			fVec9: [0.0;2],
			fVec10: [0.0;4096],
			fRec14: [0.0;2],
			fRec18: [0.0;2],
			fRec19: [0.0;2],
			fVec11: [0.0;2],
			fVec12: [0.0;4096],
			fRec17: [0.0;2],
			fHslider4: 0.0,
			fRec20: [0.0;2],
			fHslider5: 0.0,
			fRec22: [0.0;2],
			fRec23: [0.0;2],
			fVec13: [0.0;2],
			fVec14: [0.0;4096],
			fRec21: [0.0;2],
			fRec25: [0.0;2],
			fRec26: [0.0;2],
			fVec15: [0.0;2],
			fVec16: [0.0;4096],
			fRec24: [0.0;2],
			fRec28: [0.0;2],
			fRec29: [0.0;2],
			fVec17: [0.0;2],
			fVec18: [0.0;4096],
			fRec27: [0.0;2],
			fHslider6: 0.0,
			fRec30: [0.0;2],
			fHslider7: 0.0,
			fRec32: [0.0;2],
			fRec33: [0.0;2],
			fVec19: [0.0;2],
			fVec20: [0.0;4096],
			fRec31: [0.0;2],
			fRec35: [0.0;2],
			fRec36: [0.0;2],
			fVec21: [0.0;2],
			fVec22: [0.0;4096],
			fRec34: [0.0;2],
			fRec38: [0.0;2],
			fRec39: [0.0;2],
			fVec23: [0.0;2],
			fVec24: [0.0;4096],
			fRec37: [0.0;2],
			fHslider8: 0.0,
			fRec40: [0.0;2],
			fHslider9: 0.0,
			fRec41: [0.0;2],
			fConst6: 0.0,
			fHslider10: 0.0,
			fRec42: [0.0;2],
			fHslider11: 0.0,
			fRec45: [0.0;2],
			fHslider12: 0.0,
			fRec44: [0.0;2],
			fConst7: 0.0,
			fConst8: 0.0,
			fConst9: 0.0,
			fRec75: [0.0;2],
			fRec79: [0.0;2],
			fConst10: 0.0,
			fConst11: 0.0,
			fRec84: [0.0;2],
			iVec25: [0;2],
			iConst12: 0,
			iRec85: [0;2],
			fConst13: 0.0,
			fRec82: [0.0;2],
			fRec81: [0.0;2],
			fRec86: [0.0;4],
			fRec87: [0.0;2048],
			fVec26: [0.0;2],
			fConst14: 0.0,
			fConst15: 0.0,
			fButton0: 0.0,
			fVec27: [0.0;2],
			iRec88: [0;2],
			iRec90: [0;2],
			fRec89: [0.0;3],
			fVec28: [0.0;3],
			fRec80: [0.0;2048],
			fRec71: [0.0;2],
			fRec67: [0.0;2],
			fRec63: [0.0;2048],
			fRec65: [0.0;2],
			fHslider13: 0.0,
			fRec61: [0.0;4],
			fRec56: [0.0;2],
			fRec52: [0.0;2048],
			fRec50: [0.0;2],
			fConst16: 0.0,
			fRec49: [0.0;2],
			fRec48: [0.0;2],
			fRec46: [0.0;2],
			fRec43: [0.0;2],
			fHslider14: 0.0,
			fRec92: [0.0;2],
			fRec122: [0.0;2],
			fRec126: [0.0;2],
			fRec131: [0.0;2],
			iVec29: [0;2],
			iRec132: [0;2],
			fRec129: [0.0;2],
			fRec128: [0.0;2],
			fRec133: [0.0;4],
			fRec134: [0.0;2048],
			fVec30: [0.0;2],
			fButton1: 0.0,
			fVec31: [0.0;2],
			iRec135: [0;2],
			fRec136: [0.0;3],
			fVec32: [0.0;3],
			fRec127: [0.0;2048],
			fRec118: [0.0;2],
			fRec114: [0.0;2],
			fRec110: [0.0;2048],
			fRec112: [0.0;2],
			fRec108: [0.0;4],
			fRec103: [0.0;2],
			fRec99: [0.0;2048],
			fRec97: [0.0;2],
			fRec96: [0.0;2],
			fRec95: [0.0;2],
			fRec93: [0.0;2],
			fRec91: [0.0;2],
			fHslider15: 0.0,
			fRec138: [0.0;2],
			fRec168: [0.0;2],
			fRec172: [0.0;2],
			fRec177: [0.0;2],
			iVec33: [0;2],
			iRec178: [0;2],
			fRec175: [0.0;2],
			fRec174: [0.0;2],
			fRec179: [0.0;4],
			fRec180: [0.0;2048],
			fVec34: [0.0;2],
			fButton2: 0.0,
			fVec35: [0.0;2],
			iRec181: [0;2],
			fRec182: [0.0;3],
			fVec36: [0.0;3],
			fRec173: [0.0;2048],
			fRec164: [0.0;2],
			fRec160: [0.0;2],
			fRec156: [0.0;2048],
			fRec158: [0.0;2],
			fRec154: [0.0;4],
			fRec149: [0.0;2],
			fRec145: [0.0;2048],
			fRec143: [0.0;2],
			fRec142: [0.0;2],
			fRec141: [0.0;2],
			fRec139: [0.0;2],
			fRec137: [0.0;2],
			fHslider16: 0.0,
			fRec184: [0.0;2],
			fRec214: [0.0;2],
			fRec218: [0.0;2],
			fRec223: [0.0;2],
			iVec37: [0;2],
			iRec224: [0;2],
			fRec221: [0.0;2],
			fRec220: [0.0;2],
			fRec225: [0.0;4],
			fRec226: [0.0;2048],
			fVec38: [0.0;2],
			fButton3: 0.0,
			fVec39: [0.0;2],
			iRec227: [0;2],
			fRec228: [0.0;3],
			fVec40: [0.0;3],
			fRec219: [0.0;2048],
			fRec210: [0.0;2],
			fRec206: [0.0;2],
			fRec202: [0.0;2048],
			fRec204: [0.0;2],
			fRec200: [0.0;4],
			fRec195: [0.0;2],
			fRec191: [0.0;2048],
			fRec189: [0.0;2],
			fRec188: [0.0;2],
			fRec187: [0.0;2],
			fRec185: [0.0;2],
			fRec183: [0.0;2],
			fHslider17: 0.0,
			fRec229: [0.0;2],
			fHslider18: 0.0,
			fRec231: [0.0;2],
			fHslider19: 0.0,
			fRec230: [0.0;2],
			fConst17: 0.0,
			fRec236: [0.0;2],
			fRec234: [0.0;2],
			fHslider20: 0.0,
			fRec237: [0.0;2],
			fRec233: [0.0;3],
			fRec232: [0.0;3],
			fHslider21: 0.0,
			fRec238: [0.0;2],
			fRec243: [0.0;2],
			fRec241: [0.0;2],
			fHslider22: 0.0,
			fRec244: [0.0;2],
			fRec240: [0.0;3],
			fRec239: [0.0;3],
			fHslider23: 0.0,
			fRec245: [0.0;2],
			fRec250: [0.0;2],
			fRec248: [0.0;2],
			fHslider24: 0.0,
			fRec251: [0.0;2],
			fRec247: [0.0;3],
			fRec246: [0.0;3],
			fHslider25: 0.0,
			fRec252: [0.0;2],
			fRec257: [0.0;2],
			fRec255: [0.0;2],
			fHslider26: 0.0,
			fRec258: [0.0;2],
			fRec254: [0.0;3],
			fRec253: [0.0;3],
			fHslider27: 0.0,
			fRec259: [0.0;2],
			fHslider28: 0.0,
			fRec260: [0.0;2],
			fHslider29: 0.0,
			fRec261: [0.0;2],
			fConst18: 0.0,
			fHslider30: 0.0,
			fRec263: [0.0;2],
			fHslider31: 0.0,
			fRec262: [0.0;2097152],
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
			fRec274: [0.0;2],
			fRec276: [0.0;2],
			fRec280: [0.0;2],
			fVec41: [0.0;16384],
			fVec42: [0.0;2],
			fRec279: [0.0;2],
			fRec277: [0.0;2],
			fRec282: [0.0;2],
			fVec43: [0.0;16384],
			fVec44: [0.0;2],
			fRec281: [0.0;2],
			fRec278: [0.0;2],
			fRec286: [0.0;2],
			fVec45: [0.0;16384],
			fVec46: [0.0;2],
			fRec285: [0.0;2],
			fRec283: [0.0;2],
			fRec288: [0.0;2],
			fVec47: [0.0;16384],
			fVec48: [0.0;2],
			fRec287: [0.0;2],
			fRec284: [0.0;2],
			fRec292: [0.0;2],
			fVec49: [0.0;16384],
			fVec50: [0.0;2],
			fRec291: [0.0;2],
			fRec289: [0.0;2],
			fRec294: [0.0;2],
			fVec51: [0.0;16384],
			fVec52: [0.0;2],
			fRec293: [0.0;2],
			fRec290: [0.0;2],
			fRec298: [0.0;2],
			fVec53: [0.0;16384],
			fVec54: [0.0;2],
			fRec297: [0.0;2],
			fRec295: [0.0;2],
			fRec300: [0.0;2],
			fVec55: [0.0;16384],
			fVec56: [0.0;2],
			fRec299: [0.0;2],
			fRec296: [0.0;2],
			fRec304: [0.0;2],
			fVec57: [0.0;16384],
			fVec58: [0.0;2],
			fRec303: [0.0;2],
			fRec301: [0.0;2],
			fRec306: [0.0;2],
			fVec59: [0.0;16384],
			fVec60: [0.0;2],
			fRec305: [0.0;2],
			fRec302: [0.0;2],
			fVec61: [0.0;1024],
			fHslider34: 0.0,
			fRec307: [0.0;2],
			fRec308: [0.0;2],
			fHslider35: 0.0,
			fVec62: [0.0;16384],
			fVec63: [0.0;2],
			fRec275: [0.0;2],
			fRec312: [0.0;2],
			fRec314: [0.0;2],
			fVec64: [0.0;1024],
			fVec65: [0.0;16384],
			fVec66: [0.0;2],
			fRec313: [0.0;2],
			fVec67: [0.0;16384],
			fVec68: [0.0;2],
			fRec311: [0.0;2],
			fRec309: [0.0;2],
			fRec316: [0.0;2],
			fVec69: [0.0;16384],
			fVec70: [0.0;2],
			fRec315: [0.0;2],
			fRec310: [0.0;2],
			fRec320: [0.0;2],
			fVec71: [0.0;16384],
			fVec72: [0.0;2],
			fRec319: [0.0;2],
			fRec317: [0.0;2],
			fRec322: [0.0;2],
			fVec73: [0.0;16384],
			fVec74: [0.0;2],
			fRec321: [0.0;2],
			fRec318: [0.0;2],
			fRec326: [0.0;2],
			fVec75: [0.0;16384],
			fVec76: [0.0;2],
			fRec325: [0.0;2],
			fRec323: [0.0;2],
			fRec328: [0.0;2],
			fVec77: [0.0;16384],
			fVec78: [0.0;2],
			fRec327: [0.0;2],
			fRec324: [0.0;2],
			fRec332: [0.0;2],
			fVec79: [0.0;16384],
			fVec80: [0.0;2],
			fRec331: [0.0;2],
			fRec329: [0.0;2],
			fRec334: [0.0;2],
			fVec81: [0.0;16384],
			fVec82: [0.0;2],
			fRec333: [0.0;2],
			fRec330: [0.0;2],
			fRec338: [0.0;2],
			fVec83: [0.0;16384],
			fVec84: [0.0;2],
			fRec337: [0.0;2],
			fRec335: [0.0;2],
			fRec340: [0.0;2],
			fVec85: [0.0;16384],
			fVec86: [0.0;2],
			fRec339: [0.0;2],
			fRec336: [0.0;2],
			fVec87: [0.0;16384],
			fVec88: [0.0;16384],
			fVec89: [0.0;2],
			fRec273: [0.0;2],
			fConst39: 0.0,
			fRec272: [0.0;2],
			fRec271: [0.0;3],
			fRec270: [0.0;3],
			fVec90: [0.0;2],
			fConst40: 0.0,
			fConst42: 0.0,
			fRec269: [0.0;2],
			fRec268: [0.0;3],
			fRec267: [0.0;3],
			fConst43: 0.0,
			fConst44: 0.0,
			fConst45: 0.0,
			fRec343: [0.0;2],
			fRec342: [0.0;3],
			fConst46: 0.0,
			fRec341: [0.0;3],
			fConst47: 0.0,
			fConst48: 0.0,
			fConst49: 0.0,
			fRec347: [0.0;2],
			fRec346: [0.0;3],
			fConst50: 0.0,
			fRec345: [0.0;3],
			fRec344: [0.0;3],
			fHslider36: 0.0,
			fVec91: [0.0;1024],
			fHslider37: 0.0,
			fRec266: [0.0;2],
			fHslider38: 0.0,
			fRec359: [0.0;2],
			fVec92: [0.0;16384],
			fVec93: [0.0;16384],
			fVec94: [0.0;2],
			fRec358: [0.0;2],
			fRec357: [0.0;2],
			fRec356: [0.0;3],
			fRec355: [0.0;3],
			fVec95: [0.0;2],
			fRec354: [0.0;2],
			fRec353: [0.0;3],
			fRec352: [0.0;3],
			fRec362: [0.0;2],
			fRec361: [0.0;3],
			fRec360: [0.0;3],
			fRec366: [0.0;2],
			fRec365: [0.0;3],
			fRec364: [0.0;3],
			fRec363: [0.0;3],
			fVec96: [0.0;1024],
			fRec351: [0.0;2],
			fVec97: [0.0;16384],
			fVec98: [0.0;2],
			fRec350: [0.0;2],
			fRec348: [0.0;2],
			fRec368: [0.0;2],
			fVec99: [0.0;16384],
			fVec100: [0.0;2],
			fRec367: [0.0;2],
			fRec349: [0.0;2],
			fVec101: [0.0;16384],
			fVec102: [0.0;2],
			fRec371: [0.0;2],
			fRec369: [0.0;2],
			fVec103: [0.0;16384],
			fVec104: [0.0;2],
			fRec372: [0.0;2],
			fRec370: [0.0;2],
			fVec105: [0.0;16384],
			fVec106: [0.0;2],
			fRec375: [0.0;2],
			fRec373: [0.0;2],
			fRec377: [0.0;2],
			fVec107: [0.0;16384],
			fVec108: [0.0;2],
			fRec376: [0.0;2],
			fRec374: [0.0;2],
			fRec381: [0.0;2],
			fVec109: [0.0;16384],
			fVec110: [0.0;2],
			fRec380: [0.0;2],
			fRec378: [0.0;2],
			fVec111: [0.0;16384],
			fVec112: [0.0;2],
			fRec382: [0.0;2],
			fRec379: [0.0;2],
			fRec264: [0.0;2],
			fRec265: [0.0;2],
			fHslider39: 0.0,
			fRec383: [0.0;2],
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
		self.fHslider11 = 0.0;
		self.fHslider12 = 8e+01;
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
			self.fRec0[(l6) as usize] = 0.0;
		}
		for l7 in 0..2 {
			self.fRec5[(l7) as usize] = 0.0;
		}
		for l8 in 0..2 {
			self.fRec6[(l8) as usize] = 0.0;
		}
		for l9 in 0..2 {
			self.fVec3[(l9) as usize] = 0.0;
		}
		for l10 in 0..4096 {
			self.fVec4[(l10) as usize] = 0.0;
		}
		for l11 in 0..2 {
			self.fRec4[(l11) as usize] = 0.0;
		}
		for l12 in 0..2 {
			self.fRec8[(l12) as usize] = 0.0;
		}
		for l13 in 0..2 {
			self.fRec9[(l13) as usize] = 0.0;
		}
		for l14 in 0..2 {
			self.fVec5[(l14) as usize] = 0.0;
		}
		for l15 in 0..4096 {
			self.fVec6[(l15) as usize] = 0.0;
		}
		for l16 in 0..2 {
			self.fRec7[(l16) as usize] = 0.0;
		}
		for l17 in 0..2 {
			self.fRec10[(l17) as usize] = 0.0;
		}
		for l18 in 0..2 {
			self.fRec12[(l18) as usize] = 0.0;
		}
		for l19 in 0..2 {
			self.fRec13[(l19) as usize] = 0.0;
		}
		for l20 in 0..2 {
			self.fVec7[(l20) as usize] = 0.0;
		}
		for l21 in 0..4096 {
			self.fVec8[(l21) as usize] = 0.0;
		}
		for l22 in 0..2 {
			self.fRec11[(l22) as usize] = 0.0;
		}
		for l23 in 0..2 {
			self.fRec15[(l23) as usize] = 0.0;
		}
		for l24 in 0..2 {
			self.fRec16[(l24) as usize] = 0.0;
		}
		for l25 in 0..2 {
			self.fVec9[(l25) as usize] = 0.0;
		}
		for l26 in 0..4096 {
			self.fVec10[(l26) as usize] = 0.0;
		}
		for l27 in 0..2 {
			self.fRec14[(l27) as usize] = 0.0;
		}
		for l28 in 0..2 {
			self.fRec18[(l28) as usize] = 0.0;
		}
		for l29 in 0..2 {
			self.fRec19[(l29) as usize] = 0.0;
		}
		for l30 in 0..2 {
			self.fVec11[(l30) as usize] = 0.0;
		}
		for l31 in 0..4096 {
			self.fVec12[(l31) as usize] = 0.0;
		}
		for l32 in 0..2 {
			self.fRec17[(l32) as usize] = 0.0;
		}
		for l33 in 0..2 {
			self.fRec20[(l33) as usize] = 0.0;
		}
		for l34 in 0..2 {
			self.fRec22[(l34) as usize] = 0.0;
		}
		for l35 in 0..2 {
			self.fRec23[(l35) as usize] = 0.0;
		}
		for l36 in 0..2 {
			self.fVec13[(l36) as usize] = 0.0;
		}
		for l37 in 0..4096 {
			self.fVec14[(l37) as usize] = 0.0;
		}
		for l38 in 0..2 {
			self.fRec21[(l38) as usize] = 0.0;
		}
		for l39 in 0..2 {
			self.fRec25[(l39) as usize] = 0.0;
		}
		for l40 in 0..2 {
			self.fRec26[(l40) as usize] = 0.0;
		}
		for l41 in 0..2 {
			self.fVec15[(l41) as usize] = 0.0;
		}
		for l42 in 0..4096 {
			self.fVec16[(l42) as usize] = 0.0;
		}
		for l43 in 0..2 {
			self.fRec24[(l43) as usize] = 0.0;
		}
		for l44 in 0..2 {
			self.fRec28[(l44) as usize] = 0.0;
		}
		for l45 in 0..2 {
			self.fRec29[(l45) as usize] = 0.0;
		}
		for l46 in 0..2 {
			self.fVec17[(l46) as usize] = 0.0;
		}
		for l47 in 0..4096 {
			self.fVec18[(l47) as usize] = 0.0;
		}
		for l48 in 0..2 {
			self.fRec27[(l48) as usize] = 0.0;
		}
		for l49 in 0..2 {
			self.fRec30[(l49) as usize] = 0.0;
		}
		for l50 in 0..2 {
			self.fRec32[(l50) as usize] = 0.0;
		}
		for l51 in 0..2 {
			self.fRec33[(l51) as usize] = 0.0;
		}
		for l52 in 0..2 {
			self.fVec19[(l52) as usize] = 0.0;
		}
		for l53 in 0..4096 {
			self.fVec20[(l53) as usize] = 0.0;
		}
		for l54 in 0..2 {
			self.fRec31[(l54) as usize] = 0.0;
		}
		for l55 in 0..2 {
			self.fRec35[(l55) as usize] = 0.0;
		}
		for l56 in 0..2 {
			self.fRec36[(l56) as usize] = 0.0;
		}
		for l57 in 0..2 {
			self.fVec21[(l57) as usize] = 0.0;
		}
		for l58 in 0..4096 {
			self.fVec22[(l58) as usize] = 0.0;
		}
		for l59 in 0..2 {
			self.fRec34[(l59) as usize] = 0.0;
		}
		for l60 in 0..2 {
			self.fRec38[(l60) as usize] = 0.0;
		}
		for l61 in 0..2 {
			self.fRec39[(l61) as usize] = 0.0;
		}
		for l62 in 0..2 {
			self.fVec23[(l62) as usize] = 0.0;
		}
		for l63 in 0..4096 {
			self.fVec24[(l63) as usize] = 0.0;
		}
		for l64 in 0..2 {
			self.fRec37[(l64) as usize] = 0.0;
		}
		for l65 in 0..2 {
			self.fRec40[(l65) as usize] = 0.0;
		}
		for l66 in 0..2 {
			self.fRec41[(l66) as usize] = 0.0;
		}
		for l67 in 0..2 {
			self.fRec42[(l67) as usize] = 0.0;
		}
		for l68 in 0..2 {
			self.fRec45[(l68) as usize] = 0.0;
		}
		for l69 in 0..2 {
			self.fRec44[(l69) as usize] = 0.0;
		}
		for l70 in 0..2 {
			self.fRec75[(l70) as usize] = 0.0;
		}
		for l71 in 0..2 {
			self.fRec79[(l71) as usize] = 0.0;
		}
		for l72 in 0..2 {
			self.fRec84[(l72) as usize] = 0.0;
		}
		for l73 in 0..2 {
			self.iVec25[(l73) as usize] = 0;
		}
		for l74 in 0..2 {
			self.iRec85[(l74) as usize] = 0;
		}
		for l75 in 0..2 {
			self.fRec82[(l75) as usize] = 0.0;
		}
		for l76 in 0..2 {
			self.fRec81[(l76) as usize] = 0.0;
		}
		for l77 in 0..4 {
			self.fRec86[(l77) as usize] = 0.0;
		}
		for l78 in 0..2048 {
			self.fRec87[(l78) as usize] = 0.0;
		}
		for l79 in 0..2 {
			self.fVec26[(l79) as usize] = 0.0;
		}
		for l80 in 0..2 {
			self.fVec27[(l80) as usize] = 0.0;
		}
		for l81 in 0..2 {
			self.iRec88[(l81) as usize] = 0;
		}
		for l82 in 0..2 {
			self.iRec90[(l82) as usize] = 0;
		}
		for l83 in 0..3 {
			self.fRec89[(l83) as usize] = 0.0;
		}
		for l84 in 0..3 {
			self.fVec28[(l84) as usize] = 0.0;
		}
		for l85 in 0..2048 {
			self.fRec80[(l85) as usize] = 0.0;
		}
		for l86 in 0..2 {
			self.fRec71[(l86) as usize] = 0.0;
		}
		for l87 in 0..2 {
			self.fRec67[(l87) as usize] = 0.0;
		}
		for l88 in 0..2048 {
			self.fRec63[(l88) as usize] = 0.0;
		}
		for l89 in 0..2 {
			self.fRec65[(l89) as usize] = 0.0;
		}
		for l90 in 0..4 {
			self.fRec61[(l90) as usize] = 0.0;
		}
		for l91 in 0..2 {
			self.fRec56[(l91) as usize] = 0.0;
		}
		for l92 in 0..2048 {
			self.fRec52[(l92) as usize] = 0.0;
		}
		for l93 in 0..2 {
			self.fRec50[(l93) as usize] = 0.0;
		}
		for l94 in 0..2 {
			self.fRec49[(l94) as usize] = 0.0;
		}
		for l95 in 0..2 {
			self.fRec48[(l95) as usize] = 0.0;
		}
		for l96 in 0..2 {
			self.fRec46[(l96) as usize] = 0.0;
		}
		for l97 in 0..2 {
			self.fRec43[(l97) as usize] = 0.0;
		}
		for l98 in 0..2 {
			self.fRec92[(l98) as usize] = 0.0;
		}
		for l99 in 0..2 {
			self.fRec122[(l99) as usize] = 0.0;
		}
		for l100 in 0..2 {
			self.fRec126[(l100) as usize] = 0.0;
		}
		for l101 in 0..2 {
			self.fRec131[(l101) as usize] = 0.0;
		}
		for l102 in 0..2 {
			self.iVec29[(l102) as usize] = 0;
		}
		for l103 in 0..2 {
			self.iRec132[(l103) as usize] = 0;
		}
		for l104 in 0..2 {
			self.fRec129[(l104) as usize] = 0.0;
		}
		for l105 in 0..2 {
			self.fRec128[(l105) as usize] = 0.0;
		}
		for l106 in 0..4 {
			self.fRec133[(l106) as usize] = 0.0;
		}
		for l107 in 0..2048 {
			self.fRec134[(l107) as usize] = 0.0;
		}
		for l108 in 0..2 {
			self.fVec30[(l108) as usize] = 0.0;
		}
		for l109 in 0..2 {
			self.fVec31[(l109) as usize] = 0.0;
		}
		for l110 in 0..2 {
			self.iRec135[(l110) as usize] = 0;
		}
		for l111 in 0..3 {
			self.fRec136[(l111) as usize] = 0.0;
		}
		for l112 in 0..3 {
			self.fVec32[(l112) as usize] = 0.0;
		}
		for l113 in 0..2048 {
			self.fRec127[(l113) as usize] = 0.0;
		}
		for l114 in 0..2 {
			self.fRec118[(l114) as usize] = 0.0;
		}
		for l115 in 0..2 {
			self.fRec114[(l115) as usize] = 0.0;
		}
		for l116 in 0..2048 {
			self.fRec110[(l116) as usize] = 0.0;
		}
		for l117 in 0..2 {
			self.fRec112[(l117) as usize] = 0.0;
		}
		for l118 in 0..4 {
			self.fRec108[(l118) as usize] = 0.0;
		}
		for l119 in 0..2 {
			self.fRec103[(l119) as usize] = 0.0;
		}
		for l120 in 0..2048 {
			self.fRec99[(l120) as usize] = 0.0;
		}
		for l121 in 0..2 {
			self.fRec97[(l121) as usize] = 0.0;
		}
		for l122 in 0..2 {
			self.fRec96[(l122) as usize] = 0.0;
		}
		for l123 in 0..2 {
			self.fRec95[(l123) as usize] = 0.0;
		}
		for l124 in 0..2 {
			self.fRec93[(l124) as usize] = 0.0;
		}
		for l125 in 0..2 {
			self.fRec91[(l125) as usize] = 0.0;
		}
		for l126 in 0..2 {
			self.fRec138[(l126) as usize] = 0.0;
		}
		for l127 in 0..2 {
			self.fRec168[(l127) as usize] = 0.0;
		}
		for l128 in 0..2 {
			self.fRec172[(l128) as usize] = 0.0;
		}
		for l129 in 0..2 {
			self.fRec177[(l129) as usize] = 0.0;
		}
		for l130 in 0..2 {
			self.iVec33[(l130) as usize] = 0;
		}
		for l131 in 0..2 {
			self.iRec178[(l131) as usize] = 0;
		}
		for l132 in 0..2 {
			self.fRec175[(l132) as usize] = 0.0;
		}
		for l133 in 0..2 {
			self.fRec174[(l133) as usize] = 0.0;
		}
		for l134 in 0..4 {
			self.fRec179[(l134) as usize] = 0.0;
		}
		for l135 in 0..2048 {
			self.fRec180[(l135) as usize] = 0.0;
		}
		for l136 in 0..2 {
			self.fVec34[(l136) as usize] = 0.0;
		}
		for l137 in 0..2 {
			self.fVec35[(l137) as usize] = 0.0;
		}
		for l138 in 0..2 {
			self.iRec181[(l138) as usize] = 0;
		}
		for l139 in 0..3 {
			self.fRec182[(l139) as usize] = 0.0;
		}
		for l140 in 0..3 {
			self.fVec36[(l140) as usize] = 0.0;
		}
		for l141 in 0..2048 {
			self.fRec173[(l141) as usize] = 0.0;
		}
		for l142 in 0..2 {
			self.fRec164[(l142) as usize] = 0.0;
		}
		for l143 in 0..2 {
			self.fRec160[(l143) as usize] = 0.0;
		}
		for l144 in 0..2048 {
			self.fRec156[(l144) as usize] = 0.0;
		}
		for l145 in 0..2 {
			self.fRec158[(l145) as usize] = 0.0;
		}
		for l146 in 0..4 {
			self.fRec154[(l146) as usize] = 0.0;
		}
		for l147 in 0..2 {
			self.fRec149[(l147) as usize] = 0.0;
		}
		for l148 in 0..2048 {
			self.fRec145[(l148) as usize] = 0.0;
		}
		for l149 in 0..2 {
			self.fRec143[(l149) as usize] = 0.0;
		}
		for l150 in 0..2 {
			self.fRec142[(l150) as usize] = 0.0;
		}
		for l151 in 0..2 {
			self.fRec141[(l151) as usize] = 0.0;
		}
		for l152 in 0..2 {
			self.fRec139[(l152) as usize] = 0.0;
		}
		for l153 in 0..2 {
			self.fRec137[(l153) as usize] = 0.0;
		}
		for l154 in 0..2 {
			self.fRec184[(l154) as usize] = 0.0;
		}
		for l155 in 0..2 {
			self.fRec214[(l155) as usize] = 0.0;
		}
		for l156 in 0..2 {
			self.fRec218[(l156) as usize] = 0.0;
		}
		for l157 in 0..2 {
			self.fRec223[(l157) as usize] = 0.0;
		}
		for l158 in 0..2 {
			self.iVec37[(l158) as usize] = 0;
		}
		for l159 in 0..2 {
			self.iRec224[(l159) as usize] = 0;
		}
		for l160 in 0..2 {
			self.fRec221[(l160) as usize] = 0.0;
		}
		for l161 in 0..2 {
			self.fRec220[(l161) as usize] = 0.0;
		}
		for l162 in 0..4 {
			self.fRec225[(l162) as usize] = 0.0;
		}
		for l163 in 0..2048 {
			self.fRec226[(l163) as usize] = 0.0;
		}
		for l164 in 0..2 {
			self.fVec38[(l164) as usize] = 0.0;
		}
		for l165 in 0..2 {
			self.fVec39[(l165) as usize] = 0.0;
		}
		for l166 in 0..2 {
			self.iRec227[(l166) as usize] = 0;
		}
		for l167 in 0..3 {
			self.fRec228[(l167) as usize] = 0.0;
		}
		for l168 in 0..3 {
			self.fVec40[(l168) as usize] = 0.0;
		}
		for l169 in 0..2048 {
			self.fRec219[(l169) as usize] = 0.0;
		}
		for l170 in 0..2 {
			self.fRec210[(l170) as usize] = 0.0;
		}
		for l171 in 0..2 {
			self.fRec206[(l171) as usize] = 0.0;
		}
		for l172 in 0..2048 {
			self.fRec202[(l172) as usize] = 0.0;
		}
		for l173 in 0..2 {
			self.fRec204[(l173) as usize] = 0.0;
		}
		for l174 in 0..4 {
			self.fRec200[(l174) as usize] = 0.0;
		}
		for l175 in 0..2 {
			self.fRec195[(l175) as usize] = 0.0;
		}
		for l176 in 0..2048 {
			self.fRec191[(l176) as usize] = 0.0;
		}
		for l177 in 0..2 {
			self.fRec189[(l177) as usize] = 0.0;
		}
		for l178 in 0..2 {
			self.fRec188[(l178) as usize] = 0.0;
		}
		for l179 in 0..2 {
			self.fRec187[(l179) as usize] = 0.0;
		}
		for l180 in 0..2 {
			self.fRec185[(l180) as usize] = 0.0;
		}
		for l181 in 0..2 {
			self.fRec183[(l181) as usize] = 0.0;
		}
		for l182 in 0..2 {
			self.fRec229[(l182) as usize] = 0.0;
		}
		for l183 in 0..2 {
			self.fRec231[(l183) as usize] = 0.0;
		}
		for l184 in 0..2 {
			self.fRec230[(l184) as usize] = 0.0;
		}
		for l185 in 0..2 {
			self.fRec236[(l185) as usize] = 0.0;
		}
		for l186 in 0..2 {
			self.fRec234[(l186) as usize] = 0.0;
		}
		for l187 in 0..2 {
			self.fRec237[(l187) as usize] = 0.0;
		}
		for l188 in 0..3 {
			self.fRec233[(l188) as usize] = 0.0;
		}
		for l189 in 0..3 {
			self.fRec232[(l189) as usize] = 0.0;
		}
		for l190 in 0..2 {
			self.fRec238[(l190) as usize] = 0.0;
		}
		for l191 in 0..2 {
			self.fRec243[(l191) as usize] = 0.0;
		}
		for l192 in 0..2 {
			self.fRec241[(l192) as usize] = 0.0;
		}
		for l193 in 0..2 {
			self.fRec244[(l193) as usize] = 0.0;
		}
		for l194 in 0..3 {
			self.fRec240[(l194) as usize] = 0.0;
		}
		for l195 in 0..3 {
			self.fRec239[(l195) as usize] = 0.0;
		}
		for l196 in 0..2 {
			self.fRec245[(l196) as usize] = 0.0;
		}
		for l197 in 0..2 {
			self.fRec250[(l197) as usize] = 0.0;
		}
		for l198 in 0..2 {
			self.fRec248[(l198) as usize] = 0.0;
		}
		for l199 in 0..2 {
			self.fRec251[(l199) as usize] = 0.0;
		}
		for l200 in 0..3 {
			self.fRec247[(l200) as usize] = 0.0;
		}
		for l201 in 0..3 {
			self.fRec246[(l201) as usize] = 0.0;
		}
		for l202 in 0..2 {
			self.fRec252[(l202) as usize] = 0.0;
		}
		for l203 in 0..2 {
			self.fRec257[(l203) as usize] = 0.0;
		}
		for l204 in 0..2 {
			self.fRec255[(l204) as usize] = 0.0;
		}
		for l205 in 0..2 {
			self.fRec258[(l205) as usize] = 0.0;
		}
		for l206 in 0..3 {
			self.fRec254[(l206) as usize] = 0.0;
		}
		for l207 in 0..3 {
			self.fRec253[(l207) as usize] = 0.0;
		}
		for l208 in 0..2 {
			self.fRec259[(l208) as usize] = 0.0;
		}
		for l209 in 0..2 {
			self.fRec260[(l209) as usize] = 0.0;
		}
		for l210 in 0..2 {
			self.fRec261[(l210) as usize] = 0.0;
		}
		for l211 in 0..2 {
			self.fRec263[(l211) as usize] = 0.0;
		}
		for l212 in 0..2097152 {
			self.fRec262[(l212) as usize] = 0.0;
		}
		for l213 in 0..2 {
			self.fRec274[(l213) as usize] = 0.0;
		}
		for l214 in 0..2 {
			self.fRec276[(l214) as usize] = 0.0;
		}
		for l215 in 0..2 {
			self.fRec280[(l215) as usize] = 0.0;
		}
		for l216 in 0..16384 {
			self.fVec41[(l216) as usize] = 0.0;
		}
		for l217 in 0..2 {
			self.fVec42[(l217) as usize] = 0.0;
		}
		for l218 in 0..2 {
			self.fRec279[(l218) as usize] = 0.0;
		}
		for l219 in 0..2 {
			self.fRec277[(l219) as usize] = 0.0;
		}
		for l220 in 0..2 {
			self.fRec282[(l220) as usize] = 0.0;
		}
		for l221 in 0..16384 {
			self.fVec43[(l221) as usize] = 0.0;
		}
		for l222 in 0..2 {
			self.fVec44[(l222) as usize] = 0.0;
		}
		for l223 in 0..2 {
			self.fRec281[(l223) as usize] = 0.0;
		}
		for l224 in 0..2 {
			self.fRec278[(l224) as usize] = 0.0;
		}
		for l225 in 0..2 {
			self.fRec286[(l225) as usize] = 0.0;
		}
		for l226 in 0..16384 {
			self.fVec45[(l226) as usize] = 0.0;
		}
		for l227 in 0..2 {
			self.fVec46[(l227) as usize] = 0.0;
		}
		for l228 in 0..2 {
			self.fRec285[(l228) as usize] = 0.0;
		}
		for l229 in 0..2 {
			self.fRec283[(l229) as usize] = 0.0;
		}
		for l230 in 0..2 {
			self.fRec288[(l230) as usize] = 0.0;
		}
		for l231 in 0..16384 {
			self.fVec47[(l231) as usize] = 0.0;
		}
		for l232 in 0..2 {
			self.fVec48[(l232) as usize] = 0.0;
		}
		for l233 in 0..2 {
			self.fRec287[(l233) as usize] = 0.0;
		}
		for l234 in 0..2 {
			self.fRec284[(l234) as usize] = 0.0;
		}
		for l235 in 0..2 {
			self.fRec292[(l235) as usize] = 0.0;
		}
		for l236 in 0..16384 {
			self.fVec49[(l236) as usize] = 0.0;
		}
		for l237 in 0..2 {
			self.fVec50[(l237) as usize] = 0.0;
		}
		for l238 in 0..2 {
			self.fRec291[(l238) as usize] = 0.0;
		}
		for l239 in 0..2 {
			self.fRec289[(l239) as usize] = 0.0;
		}
		for l240 in 0..2 {
			self.fRec294[(l240) as usize] = 0.0;
		}
		for l241 in 0..16384 {
			self.fVec51[(l241) as usize] = 0.0;
		}
		for l242 in 0..2 {
			self.fVec52[(l242) as usize] = 0.0;
		}
		for l243 in 0..2 {
			self.fRec293[(l243) as usize] = 0.0;
		}
		for l244 in 0..2 {
			self.fRec290[(l244) as usize] = 0.0;
		}
		for l245 in 0..2 {
			self.fRec298[(l245) as usize] = 0.0;
		}
		for l246 in 0..16384 {
			self.fVec53[(l246) as usize] = 0.0;
		}
		for l247 in 0..2 {
			self.fVec54[(l247) as usize] = 0.0;
		}
		for l248 in 0..2 {
			self.fRec297[(l248) as usize] = 0.0;
		}
		for l249 in 0..2 {
			self.fRec295[(l249) as usize] = 0.0;
		}
		for l250 in 0..2 {
			self.fRec300[(l250) as usize] = 0.0;
		}
		for l251 in 0..16384 {
			self.fVec55[(l251) as usize] = 0.0;
		}
		for l252 in 0..2 {
			self.fVec56[(l252) as usize] = 0.0;
		}
		for l253 in 0..2 {
			self.fRec299[(l253) as usize] = 0.0;
		}
		for l254 in 0..2 {
			self.fRec296[(l254) as usize] = 0.0;
		}
		for l255 in 0..2 {
			self.fRec304[(l255) as usize] = 0.0;
		}
		for l256 in 0..16384 {
			self.fVec57[(l256) as usize] = 0.0;
		}
		for l257 in 0..2 {
			self.fVec58[(l257) as usize] = 0.0;
		}
		for l258 in 0..2 {
			self.fRec303[(l258) as usize] = 0.0;
		}
		for l259 in 0..2 {
			self.fRec301[(l259) as usize] = 0.0;
		}
		for l260 in 0..2 {
			self.fRec306[(l260) as usize] = 0.0;
		}
		for l261 in 0..16384 {
			self.fVec59[(l261) as usize] = 0.0;
		}
		for l262 in 0..2 {
			self.fVec60[(l262) as usize] = 0.0;
		}
		for l263 in 0..2 {
			self.fRec305[(l263) as usize] = 0.0;
		}
		for l264 in 0..2 {
			self.fRec302[(l264) as usize] = 0.0;
		}
		for l265 in 0..1024 {
			self.fVec61[(l265) as usize] = 0.0;
		}
		for l266 in 0..2 {
			self.fRec307[(l266) as usize] = 0.0;
		}
		for l267 in 0..2 {
			self.fRec308[(l267) as usize] = 0.0;
		}
		for l268 in 0..16384 {
			self.fVec62[(l268) as usize] = 0.0;
		}
		for l269 in 0..2 {
			self.fVec63[(l269) as usize] = 0.0;
		}
		for l270 in 0..2 {
			self.fRec275[(l270) as usize] = 0.0;
		}
		for l271 in 0..2 {
			self.fRec312[(l271) as usize] = 0.0;
		}
		for l272 in 0..2 {
			self.fRec314[(l272) as usize] = 0.0;
		}
		for l273 in 0..1024 {
			self.fVec64[(l273) as usize] = 0.0;
		}
		for l274 in 0..16384 {
			self.fVec65[(l274) as usize] = 0.0;
		}
		for l275 in 0..2 {
			self.fVec66[(l275) as usize] = 0.0;
		}
		for l276 in 0..2 {
			self.fRec313[(l276) as usize] = 0.0;
		}
		for l277 in 0..16384 {
			self.fVec67[(l277) as usize] = 0.0;
		}
		for l278 in 0..2 {
			self.fVec68[(l278) as usize] = 0.0;
		}
		for l279 in 0..2 {
			self.fRec311[(l279) as usize] = 0.0;
		}
		for l280 in 0..2 {
			self.fRec309[(l280) as usize] = 0.0;
		}
		for l281 in 0..2 {
			self.fRec316[(l281) as usize] = 0.0;
		}
		for l282 in 0..16384 {
			self.fVec69[(l282) as usize] = 0.0;
		}
		for l283 in 0..2 {
			self.fVec70[(l283) as usize] = 0.0;
		}
		for l284 in 0..2 {
			self.fRec315[(l284) as usize] = 0.0;
		}
		for l285 in 0..2 {
			self.fRec310[(l285) as usize] = 0.0;
		}
		for l286 in 0..2 {
			self.fRec320[(l286) as usize] = 0.0;
		}
		for l287 in 0..16384 {
			self.fVec71[(l287) as usize] = 0.0;
		}
		for l288 in 0..2 {
			self.fVec72[(l288) as usize] = 0.0;
		}
		for l289 in 0..2 {
			self.fRec319[(l289) as usize] = 0.0;
		}
		for l290 in 0..2 {
			self.fRec317[(l290) as usize] = 0.0;
		}
		for l291 in 0..2 {
			self.fRec322[(l291) as usize] = 0.0;
		}
		for l292 in 0..16384 {
			self.fVec73[(l292) as usize] = 0.0;
		}
		for l293 in 0..2 {
			self.fVec74[(l293) as usize] = 0.0;
		}
		for l294 in 0..2 {
			self.fRec321[(l294) as usize] = 0.0;
		}
		for l295 in 0..2 {
			self.fRec318[(l295) as usize] = 0.0;
		}
		for l296 in 0..2 {
			self.fRec326[(l296) as usize] = 0.0;
		}
		for l297 in 0..16384 {
			self.fVec75[(l297) as usize] = 0.0;
		}
		for l298 in 0..2 {
			self.fVec76[(l298) as usize] = 0.0;
		}
		for l299 in 0..2 {
			self.fRec325[(l299) as usize] = 0.0;
		}
		for l300 in 0..2 {
			self.fRec323[(l300) as usize] = 0.0;
		}
		for l301 in 0..2 {
			self.fRec328[(l301) as usize] = 0.0;
		}
		for l302 in 0..16384 {
			self.fVec77[(l302) as usize] = 0.0;
		}
		for l303 in 0..2 {
			self.fVec78[(l303) as usize] = 0.0;
		}
		for l304 in 0..2 {
			self.fRec327[(l304) as usize] = 0.0;
		}
		for l305 in 0..2 {
			self.fRec324[(l305) as usize] = 0.0;
		}
		for l306 in 0..2 {
			self.fRec332[(l306) as usize] = 0.0;
		}
		for l307 in 0..16384 {
			self.fVec79[(l307) as usize] = 0.0;
		}
		for l308 in 0..2 {
			self.fVec80[(l308) as usize] = 0.0;
		}
		for l309 in 0..2 {
			self.fRec331[(l309) as usize] = 0.0;
		}
		for l310 in 0..2 {
			self.fRec329[(l310) as usize] = 0.0;
		}
		for l311 in 0..2 {
			self.fRec334[(l311) as usize] = 0.0;
		}
		for l312 in 0..16384 {
			self.fVec81[(l312) as usize] = 0.0;
		}
		for l313 in 0..2 {
			self.fVec82[(l313) as usize] = 0.0;
		}
		for l314 in 0..2 {
			self.fRec333[(l314) as usize] = 0.0;
		}
		for l315 in 0..2 {
			self.fRec330[(l315) as usize] = 0.0;
		}
		for l316 in 0..2 {
			self.fRec338[(l316) as usize] = 0.0;
		}
		for l317 in 0..16384 {
			self.fVec83[(l317) as usize] = 0.0;
		}
		for l318 in 0..2 {
			self.fVec84[(l318) as usize] = 0.0;
		}
		for l319 in 0..2 {
			self.fRec337[(l319) as usize] = 0.0;
		}
		for l320 in 0..2 {
			self.fRec335[(l320) as usize] = 0.0;
		}
		for l321 in 0..2 {
			self.fRec340[(l321) as usize] = 0.0;
		}
		for l322 in 0..16384 {
			self.fVec85[(l322) as usize] = 0.0;
		}
		for l323 in 0..2 {
			self.fVec86[(l323) as usize] = 0.0;
		}
		for l324 in 0..2 {
			self.fRec339[(l324) as usize] = 0.0;
		}
		for l325 in 0..2 {
			self.fRec336[(l325) as usize] = 0.0;
		}
		for l326 in 0..16384 {
			self.fVec87[(l326) as usize] = 0.0;
		}
		for l327 in 0..16384 {
			self.fVec88[(l327) as usize] = 0.0;
		}
		for l328 in 0..2 {
			self.fVec89[(l328) as usize] = 0.0;
		}
		for l329 in 0..2 {
			self.fRec273[(l329) as usize] = 0.0;
		}
		for l330 in 0..2 {
			self.fRec272[(l330) as usize] = 0.0;
		}
		for l331 in 0..3 {
			self.fRec271[(l331) as usize] = 0.0;
		}
		for l332 in 0..3 {
			self.fRec270[(l332) as usize] = 0.0;
		}
		for l333 in 0..2 {
			self.fVec90[(l333) as usize] = 0.0;
		}
		for l334 in 0..2 {
			self.fRec269[(l334) as usize] = 0.0;
		}
		for l335 in 0..3 {
			self.fRec268[(l335) as usize] = 0.0;
		}
		for l336 in 0..3 {
			self.fRec267[(l336) as usize] = 0.0;
		}
		for l337 in 0..2 {
			self.fRec343[(l337) as usize] = 0.0;
		}
		for l338 in 0..3 {
			self.fRec342[(l338) as usize] = 0.0;
		}
		for l339 in 0..3 {
			self.fRec341[(l339) as usize] = 0.0;
		}
		for l340 in 0..2 {
			self.fRec347[(l340) as usize] = 0.0;
		}
		for l341 in 0..3 {
			self.fRec346[(l341) as usize] = 0.0;
		}
		for l342 in 0..3 {
			self.fRec345[(l342) as usize] = 0.0;
		}
		for l343 in 0..3 {
			self.fRec344[(l343) as usize] = 0.0;
		}
		for l344 in 0..1024 {
			self.fVec91[(l344) as usize] = 0.0;
		}
		for l345 in 0..2 {
			self.fRec266[(l345) as usize] = 0.0;
		}
		for l346 in 0..2 {
			self.fRec359[(l346) as usize] = 0.0;
		}
		for l347 in 0..16384 {
			self.fVec92[(l347) as usize] = 0.0;
		}
		for l348 in 0..16384 {
			self.fVec93[(l348) as usize] = 0.0;
		}
		for l349 in 0..2 {
			self.fVec94[(l349) as usize] = 0.0;
		}
		for l350 in 0..2 {
			self.fRec358[(l350) as usize] = 0.0;
		}
		for l351 in 0..2 {
			self.fRec357[(l351) as usize] = 0.0;
		}
		for l352 in 0..3 {
			self.fRec356[(l352) as usize] = 0.0;
		}
		for l353 in 0..3 {
			self.fRec355[(l353) as usize] = 0.0;
		}
		for l354 in 0..2 {
			self.fVec95[(l354) as usize] = 0.0;
		}
		for l355 in 0..2 {
			self.fRec354[(l355) as usize] = 0.0;
		}
		for l356 in 0..3 {
			self.fRec353[(l356) as usize] = 0.0;
		}
		for l357 in 0..3 {
			self.fRec352[(l357) as usize] = 0.0;
		}
		for l358 in 0..2 {
			self.fRec362[(l358) as usize] = 0.0;
		}
		for l359 in 0..3 {
			self.fRec361[(l359) as usize] = 0.0;
		}
		for l360 in 0..3 {
			self.fRec360[(l360) as usize] = 0.0;
		}
		for l361 in 0..2 {
			self.fRec366[(l361) as usize] = 0.0;
		}
		for l362 in 0..3 {
			self.fRec365[(l362) as usize] = 0.0;
		}
		for l363 in 0..3 {
			self.fRec364[(l363) as usize] = 0.0;
		}
		for l364 in 0..3 {
			self.fRec363[(l364) as usize] = 0.0;
		}
		for l365 in 0..1024 {
			self.fVec96[(l365) as usize] = 0.0;
		}
		for l366 in 0..2 {
			self.fRec351[(l366) as usize] = 0.0;
		}
		for l367 in 0..16384 {
			self.fVec97[(l367) as usize] = 0.0;
		}
		for l368 in 0..2 {
			self.fVec98[(l368) as usize] = 0.0;
		}
		for l369 in 0..2 {
			self.fRec350[(l369) as usize] = 0.0;
		}
		for l370 in 0..2 {
			self.fRec348[(l370) as usize] = 0.0;
		}
		for l371 in 0..2 {
			self.fRec368[(l371) as usize] = 0.0;
		}
		for l372 in 0..16384 {
			self.fVec99[(l372) as usize] = 0.0;
		}
		for l373 in 0..2 {
			self.fVec100[(l373) as usize] = 0.0;
		}
		for l374 in 0..2 {
			self.fRec367[(l374) as usize] = 0.0;
		}
		for l375 in 0..2 {
			self.fRec349[(l375) as usize] = 0.0;
		}
		for l376 in 0..16384 {
			self.fVec101[(l376) as usize] = 0.0;
		}
		for l377 in 0..2 {
			self.fVec102[(l377) as usize] = 0.0;
		}
		for l378 in 0..2 {
			self.fRec371[(l378) as usize] = 0.0;
		}
		for l379 in 0..2 {
			self.fRec369[(l379) as usize] = 0.0;
		}
		for l380 in 0..16384 {
			self.fVec103[(l380) as usize] = 0.0;
		}
		for l381 in 0..2 {
			self.fVec104[(l381) as usize] = 0.0;
		}
		for l382 in 0..2 {
			self.fRec372[(l382) as usize] = 0.0;
		}
		for l383 in 0..2 {
			self.fRec370[(l383) as usize] = 0.0;
		}
		for l384 in 0..16384 {
			self.fVec105[(l384) as usize] = 0.0;
		}
		for l385 in 0..2 {
			self.fVec106[(l385) as usize] = 0.0;
		}
		for l386 in 0..2 {
			self.fRec375[(l386) as usize] = 0.0;
		}
		for l387 in 0..2 {
			self.fRec373[(l387) as usize] = 0.0;
		}
		for l388 in 0..2 {
			self.fRec377[(l388) as usize] = 0.0;
		}
		for l389 in 0..16384 {
			self.fVec107[(l389) as usize] = 0.0;
		}
		for l390 in 0..2 {
			self.fVec108[(l390) as usize] = 0.0;
		}
		for l391 in 0..2 {
			self.fRec376[(l391) as usize] = 0.0;
		}
		for l392 in 0..2 {
			self.fRec374[(l392) as usize] = 0.0;
		}
		for l393 in 0..2 {
			self.fRec381[(l393) as usize] = 0.0;
		}
		for l394 in 0..16384 {
			self.fVec109[(l394) as usize] = 0.0;
		}
		for l395 in 0..2 {
			self.fVec110[(l395) as usize] = 0.0;
		}
		for l396 in 0..2 {
			self.fRec380[(l396) as usize] = 0.0;
		}
		for l397 in 0..2 {
			self.fRec378[(l397) as usize] = 0.0;
		}
		for l398 in 0..16384 {
			self.fVec111[(l398) as usize] = 0.0;
		}
		for l399 in 0..2 {
			self.fVec112[(l399) as usize] = 0.0;
		}
		for l400 in 0..2 {
			self.fRec382[(l400) as usize] = 0.0;
		}
		for l401 in 0..2 {
			self.fRec379[(l401) as usize] = 0.0;
		}
		for l402 in 0..2 {
			self.fRec264[(l402) as usize] = 0.0;
		}
		for l403 in 0..2 {
			self.fRec265[(l403) as usize] = 0.0;
		}
		for l404 in 0..2 {
			self.fRec383[(l404) as usize] = 0.0;
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
		self.fConst6 = 1.3333334 / self.fConst0;
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
			20 => Some(self.fHslider11),
			19 => Some(self.fHslider12),
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
			20 => { self.fHslider11 = value }
			19 => { self.fHslider12 = value }
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
		let mut fSlow1: F32 = self.fHslider1;
		let mut fSlow2: F32 = self.fConst1 * unsafe { ftbl0mydspSIG0[(((16.11811 * F32::max(0.0, F32::min(127.0, fSlow1 - fSlow0))) as i32)) as usize] };
		let mut fSlow3: F32 = self.fConst1 * unsafe { ftbl0mydspSIG0[(((16.11811 * F32::max(0.0, F32::min(127.0, fSlow0 + fSlow1))) as i32)) as usize] };
		let mut fSlow4: F32 = self.fConst1 * unsafe { ftbl0mydspSIG0[(((16.11811 * F32::max(0.0, F32::min(127.0, fSlow1))) as i32)) as usize] };
		let mut fSlow5: F32 = self.fConst1 * self.fHslider2;
		let mut fSlow6: F32 = self.fHslider3;
		let mut fSlow7: F32 = self.fConst1 * unsafe { ftbl0mydspSIG0[(((16.11811 * F32::max(0.0, F32::min(127.0, fSlow6 - fSlow0))) as i32)) as usize] };
		let mut fSlow8: F32 = self.fConst1 * unsafe { ftbl0mydspSIG0[(((16.11811 * F32::max(0.0, F32::min(127.0, fSlow0 + fSlow6))) as i32)) as usize] };
		let mut fSlow9: F32 = self.fConst1 * unsafe { ftbl0mydspSIG0[(((16.11811 * F32::max(0.0, F32::min(127.0, fSlow6))) as i32)) as usize] };
		let mut fSlow10: F32 = self.fConst1 * self.fHslider4;
		let mut fSlow11: F32 = self.fHslider5;
		let mut fSlow12: F32 = self.fConst1 * unsafe { ftbl0mydspSIG0[(((16.11811 * F32::max(0.0, F32::min(127.0, fSlow11 - fSlow0))) as i32)) as usize] };
		let mut fSlow13: F32 = self.fConst1 * unsafe { ftbl0mydspSIG0[(((16.11811 * F32::max(0.0, F32::min(127.0, fSlow0 + fSlow11))) as i32)) as usize] };
		let mut fSlow14: F32 = self.fConst1 * unsafe { ftbl0mydspSIG0[(((16.11811 * F32::max(0.0, F32::min(127.0, fSlow11))) as i32)) as usize] };
		let mut fSlow15: F32 = self.fConst1 * self.fHslider6;
		let mut fSlow16: F32 = self.fHslider7;
		let mut fSlow17: F32 = self.fConst1 * unsafe { ftbl0mydspSIG0[(((16.11811 * F32::max(0.0, F32::min(127.0, fSlow16 - fSlow0))) as i32)) as usize] };
		let mut fSlow18: F32 = self.fConst1 * unsafe { ftbl0mydspSIG0[(((16.11811 * F32::max(0.0, F32::min(127.0, fSlow0 + fSlow16))) as i32)) as usize] };
		let mut fSlow19: F32 = self.fConst1 * unsafe { ftbl0mydspSIG0[(((16.11811 * F32::max(0.0, F32::min(127.0, fSlow16))) as i32)) as usize] };
		let mut fSlow20: F32 = self.fConst1 * self.fHslider8;
		let mut fSlow21: F32 = self.fConst1 * self.fHslider9;
		let mut fSlow22: F32 = self.fConst1 * self.fHslider10;
		let mut fSlow23: F32 = self.fConst1 * self.fHslider11;
		let mut fSlow24: F32 = self.fHslider12;
		let mut fSlow25: F32 = self.fButton0;
		let mut fSlow26: F32 = self.fHslider13;
		let mut fSlow27: F32 = self.fHslider14;
		let mut fSlow28: F32 = self.fButton1;
		let mut fSlow29: F32 = self.fHslider15;
		let mut fSlow30: F32 = self.fButton2;
		let mut fSlow31: F32 = self.fHslider16;
		let mut fSlow32: F32 = self.fButton3;
		let mut fSlow33: F32 = self.fConst1 * self.fHslider17;
		let mut fSlow34: F32 = self.fConst1 * self.fHslider18;
		let mut fSlow35: F32 = self.fHslider19;
		let mut fSlow36: F32 = self.fConst1 * unsafe { ftbl0mydspSIG0[(((16.11811 * F32::max(0.0, F32::min(127.0, fSlow35))) as i32)) as usize] };
		let mut fSlow37: F32 = self.fConst1 * self.fHslider20;
		let mut fSlow38: F32 = self.fHslider21;
		let mut fSlow39: F32 = self.fConst1 * unsafe { ftbl0mydspSIG0[(((16.11811 * F32::max(0.0, F32::min(127.0, fSlow38))) as i32)) as usize] };
		let mut fSlow40: F32 = self.fConst1 * self.fHslider22;
		let mut fSlow41: F32 = self.fHslider23;
		let mut fSlow42: F32 = self.fConst1 * unsafe { ftbl0mydspSIG0[(((16.11811 * F32::max(0.0, F32::min(127.0, fSlow41))) as i32)) as usize] };
		let mut fSlow43: F32 = self.fConst1 * self.fHslider24;
		let mut fSlow44: F32 = self.fHslider25;
		let mut fSlow45: F32 = self.fConst1 * unsafe { ftbl0mydspSIG0[(((16.11811 * F32::max(0.0, F32::min(127.0, fSlow44))) as i32)) as usize] };
		let mut fSlow46: F32 = self.fConst1 * self.fHslider26;
		let mut fSlow47: F32 = self.fConst1 * self.fHslider27;
		let mut fSlow48: F32 = self.fConst1 * self.fHslider28;
		let mut fSlow49: F32 = self.fConst1 * self.fHslider29;
		let mut fSlow50: F32 = self.fConst1 * self.fHslider30;
		let mut fSlow51: F32 = self.fHslider31;
		let mut fSlow52: F32 = self.fHslider32;
		let mut fSlow53: F32 = 1.0 - fSlow52;
		let mut fSlow54: F32 = self.fHslider33;
		let mut iSlow55: i32 = unsafe { itbl1mydspSIG1[(((134.0 * fSlow54) as i32)) as usize] };
		let mut fSlow56: F32 = 0.005 * ((iSlow55) as F32);
		let mut iSlow57: i32 = unsafe { itbl1mydspSIG1[(((54.0 * fSlow54) as i32)) as usize] };
		let mut fSlow58: F32 = 0.005 * ((iSlow57) as F32);
		let mut iSlow59: i32 = unsafe { itbl1mydspSIG1[(((1e+01 * fSlow54) as i32)) as usize] };
		let mut fSlow60: F32 = 0.0001 * ((iSlow59) as F32);
		let mut iSlow61: i32 = unsafe { itbl1mydspSIG1[(((1.1e+02 * fSlow54) as i32)) as usize] };
		let mut fSlow62: F32 = 0.0001 * ((iSlow61) as F32);
		let mut iSlow63: i32 = unsafe { itbl1mydspSIG1[(((4e+01 * fSlow54) as i32)) as usize] };
		let mut fSlow64: F32 = 0.0001 * ((iSlow63) as F32);
		let mut iSlow65: i32 = unsafe { itbl1mydspSIG1[(((1.4e+02 * fSlow54) as i32)) as usize] };
		let mut fSlow66: F32 = 0.0001 * ((iSlow65) as F32);
		let mut iSlow67: i32 = unsafe { itbl1mydspSIG1[(((7e+01 * fSlow54) as i32)) as usize] };
		let mut fSlow68: F32 = 0.0001 * ((iSlow67) as F32);
		let mut iSlow69: i32 = unsafe { itbl1mydspSIG1[(((1.7e+02 * fSlow54) as i32)) as usize] };
		let mut fSlow70: F32 = 0.0001 * ((iSlow69) as F32);
		let mut iSlow71: i32 = unsafe { itbl1mydspSIG1[(((1e+02 * fSlow54) as i32)) as usize] };
		let mut fSlow72: F32 = 0.0001 * ((iSlow71) as F32);
		let mut iSlow73: i32 = unsafe { itbl1mydspSIG1[(((2e+02 * fSlow54) as i32)) as usize] };
		let mut fSlow74: F32 = 0.0001 * ((iSlow73) as F32);
		let mut iSlow75: i32 = unsafe { itbl1mydspSIG1[(((1.3e+02 * fSlow54) as i32)) as usize] };
		let mut fSlow76: F32 = 0.0001 * ((iSlow75) as F32);
		let mut iSlow77: i32 = unsafe { itbl1mydspSIG1[(((2.3e+02 * fSlow54) as i32)) as usize] };
		let mut fSlow78: F32 = 0.0001 * ((iSlow77) as F32);
		let mut fSlow79: F32 = self.fConst7 * self.fHslider34;
		let mut fSlow80: F32 = F32::cos(fSlow79);
		let mut fSlow81: F32 = F32::sin(fSlow79);
		let mut fSlow82: F32 = 5e+01 * self.fHslider35;
		let mut iSlow83: i32 = unsafe { itbl1mydspSIG1[(((125.0 * fSlow54) as i32)) as usize] };
		let mut fSlow84: F32 = 0.0001 * ((iSlow83) as F32);
		let mut iSlow85: i32 = unsafe { itbl1mydspSIG1[(((204.0 * fSlow54) as i32)) as usize] };
		let mut fSlow86: F32 = 0.005 * ((iSlow85) as F32);
		let mut fSlow87: F32 = 0.0 - fSlow82;
		let mut iSlow88: i32 = unsafe { itbl1mydspSIG1[(((25.0 * fSlow54) as i32)) as usize] };
		let mut fSlow89: F32 = 0.0001 * ((iSlow88) as F32);
		let mut iSlow90: i32 = unsafe { itbl1mydspSIG1[(((155.0 * fSlow54) as i32)) as usize] };
		let mut fSlow91: F32 = 0.0001 * ((iSlow90) as F32);
		let mut iSlow92: i32 = unsafe { itbl1mydspSIG1[(((55.0 * fSlow54) as i32)) as usize] };
		let mut fSlow93: F32 = 0.0001 * ((iSlow92) as F32);
		let mut iSlow94: i32 = unsafe { itbl1mydspSIG1[(((185.0 * fSlow54) as i32)) as usize] };
		let mut fSlow95: F32 = 0.0001 * ((iSlow94) as F32);
		let mut iSlow96: i32 = unsafe { itbl1mydspSIG1[(((85.0 * fSlow54) as i32)) as usize] };
		let mut fSlow97: F32 = 0.0001 * ((iSlow96) as F32);
		let mut iSlow98: i32 = unsafe { itbl1mydspSIG1[(((215.0 * fSlow54) as i32)) as usize] };
		let mut fSlow99: F32 = 0.0001 * ((iSlow98) as F32);
		let mut iSlow100: i32 = unsafe { itbl1mydspSIG1[(((115.0 * fSlow54) as i32)) as usize] };
		let mut fSlow101: F32 = 0.0001 * ((iSlow100) as F32);
		let mut iSlow102: i32 = unsafe { itbl1mydspSIG1[(((245.0 * fSlow54) as i32)) as usize] };
		let mut fSlow103: F32 = 0.0001 * ((iSlow102) as F32);
		let mut iSlow104: i32 = unsafe { itbl1mydspSIG1[(((145.0 * fSlow54) as i32)) as usize] };
		let mut fSlow105: F32 = 0.0001 * ((iSlow104) as F32);
		let mut fSlow106: F32 = F32::powf(1e+01, 0.0 - 0.51 * ((1.25 * fSlow54 + -0.25) / self.fHslider36));
		let mut fSlow107: F32 = self.fHslider37;
		let mut fSlow108: F32 = 1.0 - fSlow107;
		let mut fSlow109: F32 = self.fHslider38;
		let mut fSlow110: F32 = F32::sin(fSlow109);
		let mut iSlow111: i32 = unsafe { itbl1mydspSIG1[(((34.0 * fSlow54) as i32)) as usize] };
		let mut fSlow112: F32 = 0.005 * ((iSlow111) as F32);
		let mut fSlow113: F32 = F32::cos(fSlow109);
		let mut iSlow114: i32 = unsafe { itbl1mydspSIG1[(((2.4e+02 * fSlow54) as i32)) as usize] };
		let mut fSlow115: F32 = 0.0001 * ((iSlow114) as F32);
		let mut iSlow116: i32 = unsafe { itbl1mydspSIG1[(((1.9e+02 * fSlow54) as i32)) as usize] };
		let mut fSlow117: F32 = 0.0001 * ((iSlow116) as F32);
		let mut iSlow118: i32 = unsafe { itbl1mydspSIG1[(((175.0 * fSlow54) as i32)) as usize] };
		let mut fSlow119: F32 = 0.0001 * ((iSlow118) as F32);
		let mut fSlow120: F32 = self.fConst1 * self.fHslider39;
		let zipped_iterators = outputs0.zip(outputs1);
		for (output0, output1) in zipped_iterators {
			self.iVec0[0] = 1;
			self.fRec1[0] = self.fConst2 * self.fRec1[1] + fSlow2;
			let mut fTemp0: F32 = F32::max(self.fRec1[0], 23.44895);
			let mut fTemp1: F32 = F32::max(2e+01, F32::abs(fTemp0));
			let mut fTemp2: F32 = self.fRec3[1] + self.fConst3 * fTemp1;
			self.fRec3[0] = fTemp2 - F32::floor(fTemp2);
			let mut fTemp3: F32 = mydsp_faustpower2_f(2.0 * self.fRec3[0] + -1.0);
			self.fVec1[0] = fTemp3;
			let mut fTemp4: F32 = ((self.iVec0[1]) as F32);
			let mut fTemp5: F32 = fTemp4 * (fTemp3 - self.fVec1[1]) / fTemp1;
			self.fVec2[(self.IOTA0 & 4095) as usize] = fTemp5;
			let mut fTemp6: F32 = F32::max(0.0, F32::min(2047.0, self.fConst4 / fTemp0));
			let mut iTemp7: i32 = ((fTemp6) as i32);
			let mut fTemp8: F32 = F32::floor(fTemp6);
			self.fRec0[0] = 0.999 * self.fRec0[1] + self.fConst5 * (fTemp5 - self.fVec2[((i32::wrapping_sub(self.IOTA0, iTemp7)) & 4095) as usize] * (fTemp8 + (1.0 - fTemp6)) - (fTemp6 - fTemp8) * self.fVec2[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp7, 1))) & 4095) as usize]);
			self.fRec5[0] = self.fConst2 * self.fRec5[1] + fSlow3;
			let mut fTemp9: F32 = F32::max(self.fRec5[0], 23.44895);
			let mut fTemp10: F32 = F32::max(2e+01, F32::abs(fTemp9));
			let mut fTemp11: F32 = self.fRec6[1] + self.fConst3 * fTemp10;
			self.fRec6[0] = fTemp11 - F32::floor(fTemp11);
			let mut fTemp12: F32 = mydsp_faustpower2_f(2.0 * self.fRec6[0] + -1.0);
			self.fVec3[0] = fTemp12;
			let mut fTemp13: F32 = fTemp4 * (fTemp12 - self.fVec3[1]) / fTemp10;
			self.fVec4[(self.IOTA0 & 4095) as usize] = fTemp13;
			let mut fTemp14: F32 = F32::max(0.0, F32::min(2047.0, self.fConst4 / fTemp9));
			let mut iTemp15: i32 = ((fTemp14) as i32);
			let mut fTemp16: F32 = F32::floor(fTemp14);
			self.fRec4[0] = 0.999 * self.fRec4[1] + self.fConst5 * (fTemp13 - self.fVec4[((i32::wrapping_sub(self.IOTA0, iTemp15)) & 4095) as usize] * (fTemp16 + (1.0 - fTemp14)) - (fTemp14 - fTemp16) * self.fVec4[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp15, 1))) & 4095) as usize]);
			self.fRec8[0] = self.fConst2 * self.fRec8[1] + fSlow4;
			let mut fTemp17: F32 = F32::max(self.fRec8[0], 23.44895);
			let mut fTemp18: F32 = F32::max(2e+01, F32::abs(fTemp17));
			let mut fTemp19: F32 = self.fRec9[1] + self.fConst3 * fTemp18;
			self.fRec9[0] = fTemp19 - F32::floor(fTemp19);
			let mut fTemp20: F32 = mydsp_faustpower2_f(2.0 * self.fRec9[0] + -1.0);
			self.fVec5[0] = fTemp20;
			let mut fTemp21: F32 = fTemp4 * (fTemp20 - self.fVec5[1]) / fTemp18;
			self.fVec6[(self.IOTA0 & 4095) as usize] = fTemp21;
			let mut fTemp22: F32 = F32::max(0.0, F32::min(2047.0, self.fConst4 / fTemp17));
			let mut iTemp23: i32 = ((fTemp22) as i32);
			let mut fTemp24: F32 = F32::floor(fTemp22);
			self.fRec7[0] = 0.999 * self.fRec7[1] - self.fConst5 * (self.fVec6[((i32::wrapping_sub(self.IOTA0, iTemp23)) & 4095) as usize] * (fTemp24 + (1.0 - fTemp22)) - fTemp21 + (fTemp22 - fTemp24) * self.fVec6[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp23, 1))) & 4095) as usize]);
			self.fRec10[0] = fSlow5 + self.fConst2 * self.fRec10[1];
			self.fRec12[0] = self.fConst2 * self.fRec12[1] + fSlow7;
			let mut fTemp25: F32 = F32::max(self.fRec12[0], 23.44895);
			let mut fTemp26: F32 = F32::max(2e+01, F32::abs(fTemp25));
			let mut fTemp27: F32 = self.fRec13[1] + self.fConst3 * fTemp26;
			self.fRec13[0] = fTemp27 - F32::floor(fTemp27);
			let mut fTemp28: F32 = mydsp_faustpower2_f(2.0 * self.fRec13[0] + -1.0);
			self.fVec7[0] = fTemp28;
			let mut fTemp29: F32 = fTemp4 * (fTemp28 - self.fVec7[1]) / fTemp26;
			self.fVec8[(self.IOTA0 & 4095) as usize] = fTemp29;
			let mut fTemp30: F32 = F32::max(0.0, F32::min(2047.0, self.fConst4 / fTemp25));
			let mut iTemp31: i32 = ((fTemp30) as i32);
			let mut fTemp32: F32 = F32::floor(fTemp30);
			self.fRec11[0] = 0.999 * self.fRec11[1] + self.fConst5 * (fTemp29 - self.fVec8[((i32::wrapping_sub(self.IOTA0, iTemp31)) & 4095) as usize] * (fTemp32 + (1.0 - fTemp30)) - (fTemp30 - fTemp32) * self.fVec8[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp31, 1))) & 4095) as usize]);
			self.fRec15[0] = self.fConst2 * self.fRec15[1] + fSlow8;
			let mut fTemp33: F32 = F32::max(self.fRec15[0], 23.44895);
			let mut fTemp34: F32 = F32::max(2e+01, F32::abs(fTemp33));
			let mut fTemp35: F32 = self.fRec16[1] + self.fConst3 * fTemp34;
			self.fRec16[0] = fTemp35 - F32::floor(fTemp35);
			let mut fTemp36: F32 = mydsp_faustpower2_f(2.0 * self.fRec16[0] + -1.0);
			self.fVec9[0] = fTemp36;
			let mut fTemp37: F32 = fTemp4 * (fTemp36 - self.fVec9[1]) / fTemp34;
			self.fVec10[(self.IOTA0 & 4095) as usize] = fTemp37;
			let mut fTemp38: F32 = F32::max(0.0, F32::min(2047.0, self.fConst4 / fTemp33));
			let mut iTemp39: i32 = ((fTemp38) as i32);
			let mut fTemp40: F32 = F32::floor(fTemp38);
			self.fRec14[0] = 0.999 * self.fRec14[1] - self.fConst5 * (self.fVec10[((i32::wrapping_sub(self.IOTA0, iTemp39)) & 4095) as usize] * (fTemp40 + (1.0 - fTemp38)) - fTemp37 + (fTemp38 - fTemp40) * self.fVec10[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp39, 1))) & 4095) as usize]);
			self.fRec18[0] = self.fConst2 * self.fRec18[1] + fSlow9;
			let mut fTemp41: F32 = F32::max(self.fRec18[0], 23.44895);
			let mut fTemp42: F32 = F32::max(2e+01, F32::abs(fTemp41));
			let mut fTemp43: F32 = self.fRec19[1] + self.fConst3 * fTemp42;
			self.fRec19[0] = fTemp43 - F32::floor(fTemp43);
			let mut fTemp44: F32 = mydsp_faustpower2_f(2.0 * self.fRec19[0] + -1.0);
			self.fVec11[0] = fTemp44;
			let mut fTemp45: F32 = fTemp4 * (fTemp44 - self.fVec11[1]) / fTemp42;
			self.fVec12[(self.IOTA0 & 4095) as usize] = fTemp45;
			let mut fTemp46: F32 = F32::max(0.0, F32::min(2047.0, self.fConst4 / fTemp41));
			let mut iTemp47: i32 = ((fTemp46) as i32);
			let mut fTemp48: F32 = F32::floor(fTemp46);
			self.fRec17[0] = 0.999 * self.fRec17[1] - self.fConst5 * (self.fVec12[((i32::wrapping_sub(self.IOTA0, iTemp47)) & 4095) as usize] * (fTemp48 + (1.0 - fTemp46)) - fTemp45 + (fTemp46 - fTemp48) * self.fVec12[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp47, 1))) & 4095) as usize]);
			self.fRec20[0] = fSlow10 + self.fConst2 * self.fRec20[1];
			self.fRec22[0] = self.fConst2 * self.fRec22[1] + fSlow12;
			let mut fTemp49: F32 = F32::max(self.fRec22[0], 23.44895);
			let mut fTemp50: F32 = F32::max(2e+01, F32::abs(fTemp49));
			let mut fTemp51: F32 = self.fRec23[1] + self.fConst3 * fTemp50;
			self.fRec23[0] = fTemp51 - F32::floor(fTemp51);
			let mut fTemp52: F32 = mydsp_faustpower2_f(2.0 * self.fRec23[0] + -1.0);
			self.fVec13[0] = fTemp52;
			let mut fTemp53: F32 = fTemp4 * (fTemp52 - self.fVec13[1]) / fTemp50;
			self.fVec14[(self.IOTA0 & 4095) as usize] = fTemp53;
			let mut fTemp54: F32 = F32::max(0.0, F32::min(2047.0, self.fConst4 / fTemp49));
			let mut iTemp55: i32 = ((fTemp54) as i32);
			let mut fTemp56: F32 = F32::floor(fTemp54);
			self.fRec21[0] = 0.999 * self.fRec21[1] - self.fConst5 * (self.fVec14[((i32::wrapping_sub(self.IOTA0, iTemp55)) & 4095) as usize] * (fTemp56 + (1.0 - fTemp54)) - fTemp53 + (fTemp54 - fTemp56) * self.fVec14[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp55, 1))) & 4095) as usize]);
			self.fRec25[0] = self.fConst2 * self.fRec25[1] + fSlow13;
			let mut fTemp57: F32 = F32::max(self.fRec25[0], 23.44895);
			let mut fTemp58: F32 = F32::max(2e+01, F32::abs(fTemp57));
			let mut fTemp59: F32 = self.fRec26[1] + self.fConst3 * fTemp58;
			self.fRec26[0] = fTemp59 - F32::floor(fTemp59);
			let mut fTemp60: F32 = mydsp_faustpower2_f(2.0 * self.fRec26[0] + -1.0);
			self.fVec15[0] = fTemp60;
			let mut fTemp61: F32 = fTemp4 * (fTemp60 - self.fVec15[1]) / fTemp58;
			self.fVec16[(self.IOTA0 & 4095) as usize] = fTemp61;
			let mut fTemp62: F32 = F32::max(0.0, F32::min(2047.0, self.fConst4 / fTemp57));
			let mut iTemp63: i32 = ((fTemp62) as i32);
			let mut fTemp64: F32 = F32::floor(fTemp62);
			self.fRec24[0] = 0.999 * self.fRec24[1] + self.fConst5 * (fTemp61 - self.fVec16[((i32::wrapping_sub(self.IOTA0, iTemp63)) & 4095) as usize] * (fTemp64 + (1.0 - fTemp62)) - (fTemp62 - fTemp64) * self.fVec16[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp63, 1))) & 4095) as usize]);
			self.fRec28[0] = self.fConst2 * self.fRec28[1] + fSlow14;
			let mut fTemp65: F32 = F32::max(self.fRec28[0], 23.44895);
			let mut fTemp66: F32 = F32::max(2e+01, F32::abs(fTemp65));
			let mut fTemp67: F32 = self.fRec29[1] + self.fConst3 * fTemp66;
			self.fRec29[0] = fTemp67 - F32::floor(fTemp67);
			let mut fTemp68: F32 = mydsp_faustpower2_f(2.0 * self.fRec29[0] + -1.0);
			self.fVec17[0] = fTemp68;
			let mut fTemp69: F32 = fTemp4 * (fTemp68 - self.fVec17[1]) / fTemp66;
			self.fVec18[(self.IOTA0 & 4095) as usize] = fTemp69;
			let mut fTemp70: F32 = F32::max(0.0, F32::min(2047.0, self.fConst4 / fTemp65));
			let mut iTemp71: i32 = ((fTemp70) as i32);
			let mut fTemp72: F32 = F32::floor(fTemp70);
			self.fRec27[0] = 0.999 * self.fRec27[1] + self.fConst5 * (fTemp69 - self.fVec18[((i32::wrapping_sub(self.IOTA0, iTemp71)) & 4095) as usize] * (fTemp72 + (1.0 - fTemp70)) - (fTemp70 - fTemp72) * self.fVec18[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp71, 1))) & 4095) as usize]);
			self.fRec30[0] = fSlow15 + self.fConst2 * self.fRec30[1];
			self.fRec32[0] = self.fConst2 * self.fRec32[1] + fSlow17;
			let mut fTemp73: F32 = F32::max(self.fRec32[0], 23.44895);
			let mut fTemp74: F32 = F32::max(2e+01, F32::abs(fTemp73));
			let mut fTemp75: F32 = self.fRec33[1] + self.fConst3 * fTemp74;
			self.fRec33[0] = fTemp75 - F32::floor(fTemp75);
			let mut fTemp76: F32 = mydsp_faustpower2_f(2.0 * self.fRec33[0] + -1.0);
			self.fVec19[0] = fTemp76;
			let mut fTemp77: F32 = fTemp4 * (fTemp76 - self.fVec19[1]) / fTemp74;
			self.fVec20[(self.IOTA0 & 4095) as usize] = fTemp77;
			let mut fTemp78: F32 = F32::max(0.0, F32::min(2047.0, self.fConst4 / fTemp73));
			let mut iTemp79: i32 = ((fTemp78) as i32);
			let mut fTemp80: F32 = F32::floor(fTemp78);
			self.fRec31[0] = 0.999 * self.fRec31[1] - self.fConst5 * (self.fVec20[((i32::wrapping_sub(self.IOTA0, iTemp79)) & 4095) as usize] * (fTemp80 + (1.0 - fTemp78)) - fTemp77 + (fTemp78 - fTemp80) * self.fVec20[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp79, 1))) & 4095) as usize]);
			self.fRec35[0] = self.fConst2 * self.fRec35[1] + fSlow18;
			let mut fTemp81: F32 = F32::max(self.fRec35[0], 23.44895);
			let mut fTemp82: F32 = F32::max(2e+01, F32::abs(fTemp81));
			let mut fTemp83: F32 = self.fRec36[1] + self.fConst3 * fTemp82;
			self.fRec36[0] = fTemp83 - F32::floor(fTemp83);
			let mut fTemp84: F32 = mydsp_faustpower2_f(2.0 * self.fRec36[0] + -1.0);
			self.fVec21[0] = fTemp84;
			let mut fTemp85: F32 = fTemp4 * (fTemp84 - self.fVec21[1]) / fTemp82;
			self.fVec22[(self.IOTA0 & 4095) as usize] = fTemp85;
			let mut fTemp86: F32 = F32::max(0.0, F32::min(2047.0, self.fConst4 / fTemp81));
			let mut iTemp87: i32 = ((fTemp86) as i32);
			let mut fTemp88: F32 = F32::floor(fTemp86);
			self.fRec34[0] = 0.999 * self.fRec34[1] - self.fConst5 * (self.fVec22[((i32::wrapping_sub(self.IOTA0, iTemp87)) & 4095) as usize] * (fTemp88 + (1.0 - fTemp86)) - fTemp85 + (fTemp86 - fTemp88) * self.fVec22[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp87, 1))) & 4095) as usize]);
			self.fRec38[0] = self.fConst2 * self.fRec38[1] + fSlow19;
			let mut fTemp89: F32 = F32::max(self.fRec38[0], 23.44895);
			let mut fTemp90: F32 = F32::max(0.0, F32::min(2047.0, self.fConst4 / fTemp89));
			let mut fTemp91: F32 = F32::floor(fTemp90);
			let mut fTemp92: F32 = F32::max(2e+01, F32::abs(fTemp89));
			let mut fTemp93: F32 = self.fRec39[1] + self.fConst3 * fTemp92;
			self.fRec39[0] = fTemp93 - F32::floor(fTemp93);
			let mut fTemp94: F32 = mydsp_faustpower2_f(2.0 * self.fRec39[0] + -1.0);
			self.fVec23[0] = fTemp94;
			let mut fTemp95: F32 = fTemp4 * (fTemp94 - self.fVec23[1]) / fTemp92;
			self.fVec24[(self.IOTA0 & 4095) as usize] = fTemp95;
			let mut iTemp96: i32 = ((fTemp90) as i32);
			self.fRec37[0] = 0.999 * self.fRec37[1] - self.fConst5 * ((fTemp90 - fTemp91) * self.fVec24[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp96, 1))) & 4095) as usize] - (fTemp95 - self.fVec24[((i32::wrapping_sub(self.IOTA0, iTemp96)) & 4095) as usize] * (fTemp91 + (1.0 - fTemp90))));
			self.fRec40[0] = fSlow20 + self.fConst2 * self.fRec40[1];
			self.fRec41[0] = fSlow21 + self.fConst2 * self.fRec41[1];
			let mut fTemp97: F32 = self.fConst6 * self.fRec41[0] * (self.fRec40[0] * (self.fRec38[0] * self.fRec37[0] + self.fRec35[0] * self.fRec34[0] + self.fRec32[0] * self.fRec31[0]) + self.fRec30[0] * (self.fRec28[0] * self.fRec27[0] + self.fRec25[0] * self.fRec24[0] + self.fRec22[0] * self.fRec21[0]) + self.fRec20[0] * (self.fRec18[0] * self.fRec17[0] + self.fRec15[0] * self.fRec14[0] + self.fRec12[0] * self.fRec11[0]) + self.fRec10[0] * (self.fRec8[0] * self.fRec7[0] + self.fRec5[0] * self.fRec4[0] + self.fRec1[0] * self.fRec0[0]));
			self.fRec42[0] = fSlow22 + self.fConst2 * self.fRec42[1];
			self.fRec45[0] = fSlow23 + self.fConst2 * self.fRec45[1];
			self.fRec44[0] = self.fConst2 * self.fRec44[1] + self.fConst1 * unsafe { ftbl0mydspSIG0[(((16.11811 * F32::max(0.0, F32::min(127.0, fSlow24 + self.fRec45[0]))) as i32)) as usize] };
			let mut fTemp98: F32 = 1.0 / F32::tan(self.fConst7 * self.fRec44[0]);
			let mut fRec60: F32 = -1.0 * 0.9973053 * (0.9 * self.fRec61[2] + 0.05 * (self.fRec61[1] + self.fRec61[3]));
			let mut fTemp99: F32 = self.fConst9 * (3.4e+02 / self.fRec44[0] + -0.11);
			let mut fTemp100: F32 = fTemp99 + -1.499995;
			let mut iTemp101: i32 = ((fTemp100) as i32);
			let mut iTemp102: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, i32::wrapping_add(iTemp101, 4))) as F32))) as i32);
			let mut iTemp103: i32 = i32::wrapping_add(iTemp102, 1);
			let mut fTemp104: F32 = F32::floor(fTemp100);
			let mut fTemp105: F32 = fTemp99 + (-3.0 - fTemp104);
			let mut fTemp106: F32 = fTemp99 + (-2.0 - fTemp104);
			let mut fTemp107: F32 = fTemp99 + (-1.0 - fTemp104);
			let mut fTemp108: F32 = fTemp107 * fTemp106;
			let mut fTemp109: F32 = fTemp108 * fTemp105;
			let mut fTemp110: F32 = fTemp99 + (-4.0 - fTemp104);
			let mut fTemp111: F32 = 0.0 - fTemp110;
			let mut iTemp112: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, i32::wrapping_add(iTemp101, 3))) as F32))) as i32);
			let mut iTemp113: i32 = i32::wrapping_add(iTemp112, 1);
			let mut fTemp114: F32 = 0.0 - 0.5 * fTemp110;
			let mut fTemp115: F32 = 0.0 - fTemp105;
			let mut iTemp116: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, i32::wrapping_add(iTemp101, 2))) as F32))) as i32);
			let mut iTemp117: i32 = i32::wrapping_add(iTemp116, 1);
			let mut fTemp118: F32 = 0.0 - 0.33333334 * fTemp110;
			let mut fTemp119: F32 = 0.0 - 0.5 * fTemp105;
			let mut fTemp120: F32 = 0.0 - fTemp106;
			let mut iTemp121: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, i32::wrapping_add(iTemp101, 1))) as F32))) as i32);
			let mut iTemp122: i32 = i32::wrapping_add(iTemp121, 1);
			let mut fTemp123: F32 = fTemp99 - fTemp104;
			let mut fTemp124: F32 = 0.0 - 0.25 * fTemp110;
			let mut fTemp125: F32 = 0.0 - 0.33333334 * fTemp105;
			let mut fTemp126: F32 = 0.0 - 0.5 * fTemp106;
			let mut fTemp127: F32 = 0.0 - fTemp107;
			let mut iTemp128: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, iTemp101)) as F32))) as i32);
			let mut iTemp129: i32 = i32::wrapping_add(iTemp128, 1);
			self.fRec75[0] = self.fRec52[((i32::wrapping_sub(self.IOTA0, iTemp129)) & 2047) as usize] * fTemp127 * fTemp126 * fTemp125 * fTemp124 + fTemp123 * (self.fRec52[((i32::wrapping_sub(self.IOTA0, iTemp122)) & 2047) as usize] * fTemp120 * fTemp119 * fTemp118 + 0.5 * fTemp107 * self.fRec52[((i32::wrapping_sub(self.IOTA0, iTemp117)) & 2047) as usize] * fTemp115 * fTemp114 + 0.16666667 * fTemp108 * self.fRec52[((i32::wrapping_sub(self.IOTA0, iTemp113)) & 2047) as usize] * fTemp111 + 0.041666668 * fTemp109 * self.fRec52[((i32::wrapping_sub(self.IOTA0, iTemp103)) & 2047) as usize]);
			self.fRec79[0] = 0.05 * self.fRec79[1] + 0.95 * self.fRec75[1];
			let mut fRec76: F32 = self.fRec79[0];
			self.fRec84[0] = self.fConst10 * self.fRec84[1] + self.fConst11 * F32::abs(self.fRec46[1]);
			let mut fRec83: F32 = self.fRec84[0];
			let mut iTemp130: i32 = ((fRec83 > 0.1) as i32);
			self.iVec25[0] = iTemp130;
			self.iRec85[0] = std::cmp::max(i32::wrapping_mul(self.iConst12, ((iTemp130 < self.iVec25[1]) as i32)), i32::wrapping_add(self.iRec85[1], -1));
			let mut fTemp131: F32 = F32::abs(F32::max(((iTemp130) as F32), ((((self.iRec85[0] > 0) as i32)) as F32)));
			let mut fTemp132: F32 = if (((self.fRec81[1] > fTemp131) as i32) as i32 != 0) { self.fConst13 } else { self.fConst10 };
			self.fRec82[0] = self.fRec82[1] * fTemp132 + fTemp131 * (1.0 - fTemp132);
			self.fRec81[0] = self.fRec82[0];
			let mut fTemp133: F32 = 0.005 * self.fRec81[0] * self.fRec46[1];
			self.fRec86[0] = self.fRec50[1];
			self.fRec87[(self.IOTA0 & 2047) as usize] = -1.0 * 0.9973053 * (0.9 * self.fRec86[2] + 0.05 * (self.fRec86[1] + self.fRec86[3]));
			let mut fTemp134: F32 = fTemp108 * fTemp111;
			let mut fTemp135: F32 = fTemp107 * fTemp115 * fTemp114;
			let mut fTemp136: F32 = fTemp120 * fTemp119 * fTemp118;
			let mut fTemp137: F32 = fTemp127 * fTemp126 * fTemp125 * fTemp124;
			self.fVec26[0] = fTemp137 * self.fRec87[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp128, 2))) & 2047) as usize] + fTemp123 * (fTemp136 * self.fRec87[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp121, 2))) & 2047) as usize] + 0.5 * fTemp135 * self.fRec87[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp116, 2))) & 2047) as usize] + 0.16666667 * fTemp134 * self.fRec87[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp112, 2))) & 2047) as usize] + 0.041666668 * fTemp109 * self.fRec87[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp102, 2))) & 2047) as usize]);
			let mut fTemp138: F32 = F32::tan(self.fConst14 * self.fRec44[0]);
			let mut fTemp139: F32 = 1.0 / fTemp138;
			let mut fTemp140: F32 = (fTemp139 + 1.4142135) / fTemp138 + 1.0;
			self.fVec27[0] = fSlow25;
			self.iRec88[0] = i32::wrapping_add(i32::wrapping_mul(i32::wrapping_add(self.iRec88[1], ((self.iRec88[1] > 0) as i32)), ((fSlow25 <= self.fVec27[1]) as i32)), ((fSlow25 > self.fVec27[1]) as i32));
			let mut fTemp141: F32 = ((self.iRec88[0]) as F32) / F32::max(1.0, self.fConst15 * mydsp_faustpower2_f(1.0 - 0.0005 * self.fRec44[0]));
			self.iRec90[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, self.iRec90[1]), 12345);
			let mut fTemp142: F32 = 4.656613e-10 * ((self.iRec90[0]) as F32);
			self.fRec89[0] = fTemp142 - (self.fRec89[2] * ((fTemp139 + -1.4142135) / fTemp138 + 1.0) + 2.0 * self.fRec89[1] * (1.0 - 1.0 / mydsp_faustpower2_f(fTemp138))) / fTemp140;
			let mut fTemp143: F32 = 0.5 * ((self.fRec89[2] + self.fRec89[0] + 2.0 * self.fRec89[1]) * F32::max(0.0, F32::min(fTemp141, 2.0 - fTemp141)) / fTemp140);
			let mut fTemp144: F32 = fTemp143 + self.fVec26[1] + fTemp133;
			self.fVec28[0] = fTemp144;
			self.fRec80[(self.IOTA0 & 2047) as usize] = 0.05 * self.fRec80[((i32::wrapping_sub(self.IOTA0, 1)) & 2047) as usize] + 0.95 * self.fVec28[2];
			let mut fRec77: F32 = fTemp137 * self.fRec80[((i32::wrapping_sub(self.IOTA0, iTemp128)) & 2047) as usize] + fTemp123 * (fTemp136 * self.fRec80[((i32::wrapping_sub(self.IOTA0, iTemp121)) & 2047) as usize] + 0.5 * fTemp135 * self.fRec80[((i32::wrapping_sub(self.IOTA0, iTemp116)) & 2047) as usize] + 0.16666667 * fTemp134 * self.fRec80[((i32::wrapping_sub(self.IOTA0, iTemp112)) & 2047) as usize] + 0.041666668 * fTemp109 * self.fRec80[((i32::wrapping_sub(self.IOTA0, iTemp102)) & 2047) as usize]);
			let mut fRec78: F32 = self.fVec28[1] + self.fRec71[1];
			self.fRec71[0] = fRec76;
			let mut fRec72: F32 = self.fRec71[1];
			let mut fRec73: F32 = fRec77;
			let mut fRec74: F32 = fRec78;
			self.fRec67[0] = fRec72;
			let mut fRec68: F32 = fTemp133 + fTemp143 + self.fRec67[1];
			let mut fRec69: F32 = fRec73;
			let mut fRec70: F32 = fRec74;
			self.fRec63[(self.IOTA0 & 2047) as usize] = fRec68;
			let mut fRec64: F32 = fTemp137 * self.fRec63[((i32::wrapping_sub(self.IOTA0, iTemp129)) & 2047) as usize] + fTemp123 * (fTemp136 * self.fRec63[((i32::wrapping_sub(self.IOTA0, iTemp122)) & 2047) as usize] + 0.5 * fTemp135 * self.fRec63[((i32::wrapping_sub(self.IOTA0, iTemp117)) & 2047) as usize] + 0.16666667 * fTemp134 * self.fRec63[((i32::wrapping_sub(self.IOTA0, iTemp113)) & 2047) as usize] + 0.041666668 * fTemp109 * self.fRec63[((i32::wrapping_sub(self.IOTA0, iTemp103)) & 2047) as usize]);
			self.fRec65[0] = fRec69;
			let mut fRec66: F32 = fRec70;
			self.fRec61[0] = fSlow26 * self.fRec65[1];
			let mut fRec62: F32 = fRec66;
			self.fRec56[0] = fRec60;
			let mut fRec57: F32 = fSlow26 * self.fRec56[1];
			let mut fRec58: F32 = self.fRec61[0];
			let mut fRec59: F32 = fRec62;
			self.fRec52[(self.IOTA0 & 2047) as usize] = fRec57;
			let mut fRec53: F32 = fRec64;
			let mut fRec54: F32 = fRec58;
			let mut fRec55: F32 = fRec59;
			self.fRec50[0] = fRec53;
			let mut fRec51: F32 = fRec55;
			let mut fTemp145: F32 = F32::abs(fRec51);
			let mut fTemp146: F32 = if (((self.fRec48[1] > fTemp145) as i32) as i32 != 0) { self.fConst16 } else { 0.0 };
			self.fRec49[0] = self.fRec49[1] * fTemp146 + fTemp145 * (1.0 - fTemp146);
			self.fRec48[0] = self.fRec49[0];
			let mut fRec47: F32 = 0.0 - 0.95 * F32::max(2e+01 * F32::log10(self.fRec48[0]) + 1e+01, 0.0);
			self.fRec46[0] = fRec51 * F32::powf(1e+01, 0.05 * fRec47);
			self.fRec43[0] = 0.0 - (self.fRec43[1] * (1.0 - fTemp98) - (self.fRec46[0] + self.fRec46[1])) / (fTemp98 + 1.0);
			self.fRec92[0] = self.fConst2 * self.fRec92[1] + self.fConst1 * unsafe { ftbl0mydspSIG0[(((16.11811 * F32::max(0.0, F32::min(127.0, fSlow27 + self.fRec45[0]))) as i32)) as usize] };
			let mut fTemp147: F32 = 1.0 / F32::tan(self.fConst7 * self.fRec92[0]);
			let mut fRec107: F32 = -1.0 * 0.9973053 * (0.9 * self.fRec108[2] + 0.05 * (self.fRec108[1] + self.fRec108[3]));
			let mut fTemp148: F32 = self.fConst9 * (3.4e+02 / self.fRec92[0] + -0.11);
			let mut fTemp149: F32 = fTemp148 + -1.499995;
			let mut iTemp150: i32 = ((fTemp149) as i32);
			let mut iTemp151: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, i32::wrapping_add(iTemp150, 4))) as F32))) as i32);
			let mut iTemp152: i32 = i32::wrapping_add(iTemp151, 1);
			let mut fTemp153: F32 = F32::floor(fTemp149);
			let mut fTemp154: F32 = fTemp148 + (-3.0 - fTemp153);
			let mut fTemp155: F32 = fTemp148 + (-2.0 - fTemp153);
			let mut fTemp156: F32 = fTemp148 + (-1.0 - fTemp153);
			let mut fTemp157: F32 = fTemp156 * fTemp155;
			let mut fTemp158: F32 = fTemp157 * fTemp154;
			let mut fTemp159: F32 = fTemp148 + (-4.0 - fTemp153);
			let mut fTemp160: F32 = 0.0 - fTemp159;
			let mut iTemp161: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, i32::wrapping_add(iTemp150, 3))) as F32))) as i32);
			let mut iTemp162: i32 = i32::wrapping_add(iTemp161, 1);
			let mut fTemp163: F32 = 0.0 - 0.5 * fTemp159;
			let mut fTemp164: F32 = 0.0 - fTemp154;
			let mut iTemp165: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, i32::wrapping_add(iTemp150, 2))) as F32))) as i32);
			let mut iTemp166: i32 = i32::wrapping_add(iTemp165, 1);
			let mut fTemp167: F32 = 0.0 - 0.33333334 * fTemp159;
			let mut fTemp168: F32 = 0.0 - 0.5 * fTemp154;
			let mut fTemp169: F32 = 0.0 - fTemp155;
			let mut iTemp170: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, i32::wrapping_add(iTemp150, 1))) as F32))) as i32);
			let mut iTemp171: i32 = i32::wrapping_add(iTemp170, 1);
			let mut fTemp172: F32 = fTemp148 - fTemp153;
			let mut fTemp173: F32 = 0.0 - 0.25 * fTemp159;
			let mut fTemp174: F32 = 0.0 - 0.33333334 * fTemp154;
			let mut fTemp175: F32 = 0.0 - 0.5 * fTemp155;
			let mut fTemp176: F32 = 0.0 - fTemp156;
			let mut iTemp177: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, iTemp150)) as F32))) as i32);
			let mut iTemp178: i32 = i32::wrapping_add(iTemp177, 1);
			self.fRec122[0] = self.fRec99[((i32::wrapping_sub(self.IOTA0, iTemp178)) & 2047) as usize] * fTemp176 * fTemp175 * fTemp174 * fTemp173 + fTemp172 * (self.fRec99[((i32::wrapping_sub(self.IOTA0, iTemp171)) & 2047) as usize] * fTemp169 * fTemp168 * fTemp167 + 0.5 * fTemp156 * self.fRec99[((i32::wrapping_sub(self.IOTA0, iTemp166)) & 2047) as usize] * fTemp164 * fTemp163 + 0.16666667 * fTemp157 * self.fRec99[((i32::wrapping_sub(self.IOTA0, iTemp162)) & 2047) as usize] * fTemp160 + 0.041666668 * fTemp158 * self.fRec99[((i32::wrapping_sub(self.IOTA0, iTemp152)) & 2047) as usize]);
			self.fRec126[0] = 0.05 * self.fRec126[1] + 0.95 * self.fRec122[1];
			let mut fRec123: F32 = self.fRec126[0];
			self.fRec131[0] = self.fConst10 * self.fRec131[1] + self.fConst11 * F32::abs(self.fRec93[1]);
			let mut fRec130: F32 = self.fRec131[0];
			let mut iTemp179: i32 = ((fRec130 > 0.1) as i32);
			self.iVec29[0] = iTemp179;
			self.iRec132[0] = std::cmp::max(i32::wrapping_mul(self.iConst12, ((iTemp179 < self.iVec29[1]) as i32)), i32::wrapping_add(self.iRec132[1], -1));
			let mut fTemp180: F32 = F32::abs(F32::max(((iTemp179) as F32), ((((self.iRec132[0] > 0) as i32)) as F32)));
			let mut fTemp181: F32 = if (((self.fRec128[1] > fTemp180) as i32) as i32 != 0) { self.fConst13 } else { self.fConst10 };
			self.fRec129[0] = self.fRec129[1] * fTemp181 + fTemp180 * (1.0 - fTemp181);
			self.fRec128[0] = self.fRec129[0];
			let mut fTemp182: F32 = 0.005 * self.fRec128[0] * self.fRec93[1];
			self.fRec133[0] = self.fRec97[1];
			self.fRec134[(self.IOTA0 & 2047) as usize] = -1.0 * 0.9973053 * (0.9 * self.fRec133[2] + 0.05 * (self.fRec133[1] + self.fRec133[3]));
			let mut fTemp183: F32 = fTemp157 * fTemp160;
			let mut fTemp184: F32 = fTemp156 * fTemp164 * fTemp163;
			let mut fTemp185: F32 = fTemp169 * fTemp168 * fTemp167;
			let mut fTemp186: F32 = fTemp176 * fTemp175 * fTemp174 * fTemp173;
			self.fVec30[0] = fTemp186 * self.fRec134[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp177, 2))) & 2047) as usize] + fTemp172 * (fTemp185 * self.fRec134[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp170, 2))) & 2047) as usize] + 0.5 * fTemp184 * self.fRec134[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp165, 2))) & 2047) as usize] + 0.16666667 * fTemp183 * self.fRec134[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp161, 2))) & 2047) as usize] + 0.041666668 * fTemp158 * self.fRec134[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp151, 2))) & 2047) as usize]);
			let mut fTemp187: F32 = F32::tan(self.fConst14 * self.fRec92[0]);
			let mut fTemp188: F32 = 1.0 / fTemp187;
			let mut fTemp189: F32 = (fTemp188 + 1.4142135) / fTemp187 + 1.0;
			self.fVec31[0] = fSlow28;
			self.iRec135[0] = i32::wrapping_add(i32::wrapping_mul(i32::wrapping_add(self.iRec135[1], ((self.iRec135[1] > 0) as i32)), ((fSlow28 <= self.fVec31[1]) as i32)), ((fSlow28 > self.fVec31[1]) as i32));
			let mut fTemp190: F32 = ((self.iRec135[0]) as F32) / F32::max(1.0, self.fConst15 * mydsp_faustpower2_f(1.0 - 0.0005 * self.fRec92[0]));
			self.fRec136[0] = fTemp142 - (self.fRec136[2] * ((fTemp188 + -1.4142135) / fTemp187 + 1.0) + 2.0 * self.fRec136[1] * (1.0 - 1.0 / mydsp_faustpower2_f(fTemp187))) / fTemp189;
			let mut fTemp191: F32 = 0.5 * ((self.fRec136[2] + self.fRec136[0] + 2.0 * self.fRec136[1]) * F32::max(0.0, F32::min(fTemp190, 2.0 - fTemp190)) / fTemp189);
			let mut fTemp192: F32 = fTemp191 + self.fVec30[1] + fTemp182;
			self.fVec32[0] = fTemp192;
			self.fRec127[(self.IOTA0 & 2047) as usize] = 0.05 * self.fRec127[((i32::wrapping_sub(self.IOTA0, 1)) & 2047) as usize] + 0.95 * self.fVec32[2];
			let mut fRec124: F32 = fTemp186 * self.fRec127[((i32::wrapping_sub(self.IOTA0, iTemp177)) & 2047) as usize] + fTemp172 * (fTemp185 * self.fRec127[((i32::wrapping_sub(self.IOTA0, iTemp170)) & 2047) as usize] + 0.5 * fTemp184 * self.fRec127[((i32::wrapping_sub(self.IOTA0, iTemp165)) & 2047) as usize] + 0.16666667 * fTemp183 * self.fRec127[((i32::wrapping_sub(self.IOTA0, iTemp161)) & 2047) as usize] + 0.041666668 * fTemp158 * self.fRec127[((i32::wrapping_sub(self.IOTA0, iTemp151)) & 2047) as usize]);
			let mut fRec125: F32 = self.fVec32[1] + self.fRec118[1];
			self.fRec118[0] = fRec123;
			let mut fRec119: F32 = self.fRec118[1];
			let mut fRec120: F32 = fRec124;
			let mut fRec121: F32 = fRec125;
			self.fRec114[0] = fRec119;
			let mut fRec115: F32 = fTemp182 + fTemp191 + self.fRec114[1];
			let mut fRec116: F32 = fRec120;
			let mut fRec117: F32 = fRec121;
			self.fRec110[(self.IOTA0 & 2047) as usize] = fRec115;
			let mut fRec111: F32 = fTemp186 * self.fRec110[((i32::wrapping_sub(self.IOTA0, iTemp178)) & 2047) as usize] + fTemp172 * (fTemp185 * self.fRec110[((i32::wrapping_sub(self.IOTA0, iTemp171)) & 2047) as usize] + 0.5 * fTemp184 * self.fRec110[((i32::wrapping_sub(self.IOTA0, iTemp166)) & 2047) as usize] + 0.16666667 * fTemp183 * self.fRec110[((i32::wrapping_sub(self.IOTA0, iTemp162)) & 2047) as usize] + 0.041666668 * fTemp158 * self.fRec110[((i32::wrapping_sub(self.IOTA0, iTemp152)) & 2047) as usize]);
			self.fRec112[0] = fRec116;
			let mut fRec113: F32 = fRec117;
			self.fRec108[0] = fSlow26 * self.fRec112[1];
			let mut fRec109: F32 = fRec113;
			self.fRec103[0] = fRec107;
			let mut fRec104: F32 = fSlow26 * self.fRec103[1];
			let mut fRec105: F32 = self.fRec108[0];
			let mut fRec106: F32 = fRec109;
			self.fRec99[(self.IOTA0 & 2047) as usize] = fRec104;
			let mut fRec100: F32 = fRec111;
			let mut fRec101: F32 = fRec105;
			let mut fRec102: F32 = fRec106;
			self.fRec97[0] = fRec100;
			let mut fRec98: F32 = fRec102;
			let mut fTemp193: F32 = F32::abs(fRec98);
			let mut fTemp194: F32 = if (((self.fRec95[1] > fTemp193) as i32) as i32 != 0) { self.fConst16 } else { 0.0 };
			self.fRec96[0] = self.fRec96[1] * fTemp194 + fTemp193 * (1.0 - fTemp194);
			self.fRec95[0] = self.fRec96[0];
			let mut fRec94: F32 = 0.0 - 0.95 * F32::max(2e+01 * F32::log10(self.fRec95[0]) + 1e+01, 0.0);
			self.fRec93[0] = fRec98 * F32::powf(1e+01, 0.05 * fRec94);
			self.fRec91[0] = 0.0 - (self.fRec91[1] * (1.0 - fTemp147) - (self.fRec93[0] + self.fRec93[1])) / (fTemp147 + 1.0);
			self.fRec138[0] = self.fConst2 * self.fRec138[1] + self.fConst1 * unsafe { ftbl0mydspSIG0[(((16.11811 * F32::max(0.0, F32::min(127.0, fSlow29 + self.fRec45[0]))) as i32)) as usize] };
			let mut fTemp195: F32 = 1.0 / F32::tan(self.fConst7 * self.fRec138[0]);
			let mut fRec153: F32 = -1.0 * 0.9973053 * (0.9 * self.fRec154[2] + 0.05 * (self.fRec154[1] + self.fRec154[3]));
			let mut fTemp196: F32 = self.fConst9 * (3.4e+02 / self.fRec138[0] + -0.11);
			let mut fTemp197: F32 = fTemp196 + -1.499995;
			let mut iTemp198: i32 = ((fTemp197) as i32);
			let mut iTemp199: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, i32::wrapping_add(iTemp198, 4))) as F32))) as i32);
			let mut iTemp200: i32 = i32::wrapping_add(iTemp199, 1);
			let mut fTemp201: F32 = F32::floor(fTemp197);
			let mut fTemp202: F32 = fTemp196 + (-3.0 - fTemp201);
			let mut fTemp203: F32 = fTemp196 + (-2.0 - fTemp201);
			let mut fTemp204: F32 = fTemp196 + (-1.0 - fTemp201);
			let mut fTemp205: F32 = fTemp204 * fTemp203;
			let mut fTemp206: F32 = fTemp205 * fTemp202;
			let mut fTemp207: F32 = fTemp196 + (-4.0 - fTemp201);
			let mut fTemp208: F32 = 0.0 - fTemp207;
			let mut iTemp209: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, i32::wrapping_add(iTemp198, 3))) as F32))) as i32);
			let mut iTemp210: i32 = i32::wrapping_add(iTemp209, 1);
			let mut fTemp211: F32 = 0.0 - 0.5 * fTemp207;
			let mut fTemp212: F32 = 0.0 - fTemp202;
			let mut iTemp213: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, i32::wrapping_add(iTemp198, 2))) as F32))) as i32);
			let mut iTemp214: i32 = i32::wrapping_add(iTemp213, 1);
			let mut fTemp215: F32 = 0.0 - 0.33333334 * fTemp207;
			let mut fTemp216: F32 = 0.0 - 0.5 * fTemp202;
			let mut fTemp217: F32 = 0.0 - fTemp203;
			let mut iTemp218: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, i32::wrapping_add(iTemp198, 1))) as F32))) as i32);
			let mut iTemp219: i32 = i32::wrapping_add(iTemp218, 1);
			let mut fTemp220: F32 = fTemp196 - fTemp201;
			let mut fTemp221: F32 = 0.0 - 0.25 * fTemp207;
			let mut fTemp222: F32 = 0.0 - 0.33333334 * fTemp202;
			let mut fTemp223: F32 = 0.0 - 0.5 * fTemp203;
			let mut fTemp224: F32 = 0.0 - fTemp204;
			let mut iTemp225: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, iTemp198)) as F32))) as i32);
			let mut iTemp226: i32 = i32::wrapping_add(iTemp225, 1);
			self.fRec168[0] = self.fRec145[((i32::wrapping_sub(self.IOTA0, iTemp226)) & 2047) as usize] * fTemp224 * fTemp223 * fTemp222 * fTemp221 + fTemp220 * (self.fRec145[((i32::wrapping_sub(self.IOTA0, iTemp219)) & 2047) as usize] * fTemp217 * fTemp216 * fTemp215 + 0.5 * fTemp204 * self.fRec145[((i32::wrapping_sub(self.IOTA0, iTemp214)) & 2047) as usize] * fTemp212 * fTemp211 + 0.16666667 * fTemp205 * self.fRec145[((i32::wrapping_sub(self.IOTA0, iTemp210)) & 2047) as usize] * fTemp208 + 0.041666668 * fTemp206 * self.fRec145[((i32::wrapping_sub(self.IOTA0, iTemp200)) & 2047) as usize]);
			self.fRec172[0] = 0.05 * self.fRec172[1] + 0.95 * self.fRec168[1];
			let mut fRec169: F32 = self.fRec172[0];
			self.fRec177[0] = self.fConst10 * self.fRec177[1] + self.fConst11 * F32::abs(self.fRec139[1]);
			let mut fRec176: F32 = self.fRec177[0];
			let mut iTemp227: i32 = ((fRec176 > 0.1) as i32);
			self.iVec33[0] = iTemp227;
			self.iRec178[0] = std::cmp::max(i32::wrapping_mul(self.iConst12, ((iTemp227 < self.iVec33[1]) as i32)), i32::wrapping_add(self.iRec178[1], -1));
			let mut fTemp228: F32 = F32::abs(F32::max(((iTemp227) as F32), ((((self.iRec178[0] > 0) as i32)) as F32)));
			let mut fTemp229: F32 = if (((self.fRec174[1] > fTemp228) as i32) as i32 != 0) { self.fConst13 } else { self.fConst10 };
			self.fRec175[0] = self.fRec175[1] * fTemp229 + fTemp228 * (1.0 - fTemp229);
			self.fRec174[0] = self.fRec175[0];
			let mut fTemp230: F32 = 0.005 * self.fRec174[0] * self.fRec139[1];
			self.fRec179[0] = self.fRec143[1];
			self.fRec180[(self.IOTA0 & 2047) as usize] = -1.0 * 0.9973053 * (0.9 * self.fRec179[2] + 0.05 * (self.fRec179[1] + self.fRec179[3]));
			let mut fTemp231: F32 = fTemp205 * fTemp208;
			let mut fTemp232: F32 = fTemp204 * fTemp212 * fTemp211;
			let mut fTemp233: F32 = fTemp217 * fTemp216 * fTemp215;
			let mut fTemp234: F32 = fTemp224 * fTemp223 * fTemp222 * fTemp221;
			self.fVec34[0] = fTemp234 * self.fRec180[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp225, 2))) & 2047) as usize] + fTemp220 * (fTemp233 * self.fRec180[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp218, 2))) & 2047) as usize] + 0.5 * fTemp232 * self.fRec180[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp213, 2))) & 2047) as usize] + 0.16666667 * fTemp231 * self.fRec180[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp209, 2))) & 2047) as usize] + 0.041666668 * fTemp206 * self.fRec180[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp199, 2))) & 2047) as usize]);
			let mut fTemp235: F32 = F32::tan(self.fConst14 * self.fRec138[0]);
			let mut fTemp236: F32 = 1.0 / fTemp235;
			let mut fTemp237: F32 = (fTemp236 + 1.4142135) / fTemp235 + 1.0;
			self.fVec35[0] = fSlow30;
			self.iRec181[0] = i32::wrapping_add(i32::wrapping_mul(i32::wrapping_add(self.iRec181[1], ((self.iRec181[1] > 0) as i32)), ((fSlow30 <= self.fVec35[1]) as i32)), ((fSlow30 > self.fVec35[1]) as i32));
			let mut fTemp238: F32 = ((self.iRec181[0]) as F32) / F32::max(1.0, self.fConst15 * mydsp_faustpower2_f(1.0 - 0.0005 * self.fRec138[0]));
			self.fRec182[0] = fTemp142 - (self.fRec182[2] * ((fTemp236 + -1.4142135) / fTemp235 + 1.0) + 2.0 * self.fRec182[1] * (1.0 - 1.0 / mydsp_faustpower2_f(fTemp235))) / fTemp237;
			let mut fTemp239: F32 = 0.5 * ((self.fRec182[2] + self.fRec182[0] + 2.0 * self.fRec182[1]) * F32::max(0.0, F32::min(fTemp238, 2.0 - fTemp238)) / fTemp237);
			let mut fTemp240: F32 = fTemp239 + self.fVec34[1] + fTemp230;
			self.fVec36[0] = fTemp240;
			self.fRec173[(self.IOTA0 & 2047) as usize] = 0.05 * self.fRec173[((i32::wrapping_sub(self.IOTA0, 1)) & 2047) as usize] + 0.95 * self.fVec36[2];
			let mut fRec170: F32 = fTemp234 * self.fRec173[((i32::wrapping_sub(self.IOTA0, iTemp225)) & 2047) as usize] + fTemp220 * (fTemp233 * self.fRec173[((i32::wrapping_sub(self.IOTA0, iTemp218)) & 2047) as usize] + 0.5 * fTemp232 * self.fRec173[((i32::wrapping_sub(self.IOTA0, iTemp213)) & 2047) as usize] + 0.16666667 * fTemp231 * self.fRec173[((i32::wrapping_sub(self.IOTA0, iTemp209)) & 2047) as usize] + 0.041666668 * fTemp206 * self.fRec173[((i32::wrapping_sub(self.IOTA0, iTemp199)) & 2047) as usize]);
			let mut fRec171: F32 = self.fVec36[1] + self.fRec164[1];
			self.fRec164[0] = fRec169;
			let mut fRec165: F32 = self.fRec164[1];
			let mut fRec166: F32 = fRec170;
			let mut fRec167: F32 = fRec171;
			self.fRec160[0] = fRec165;
			let mut fRec161: F32 = fTemp230 + fTemp239 + self.fRec160[1];
			let mut fRec162: F32 = fRec166;
			let mut fRec163: F32 = fRec167;
			self.fRec156[(self.IOTA0 & 2047) as usize] = fRec161;
			let mut fRec157: F32 = fTemp234 * self.fRec156[((i32::wrapping_sub(self.IOTA0, iTemp226)) & 2047) as usize] + fTemp220 * (fTemp233 * self.fRec156[((i32::wrapping_sub(self.IOTA0, iTemp219)) & 2047) as usize] + 0.5 * fTemp232 * self.fRec156[((i32::wrapping_sub(self.IOTA0, iTemp214)) & 2047) as usize] + 0.16666667 * fTemp231 * self.fRec156[((i32::wrapping_sub(self.IOTA0, iTemp210)) & 2047) as usize] + 0.041666668 * fTemp206 * self.fRec156[((i32::wrapping_sub(self.IOTA0, iTemp200)) & 2047) as usize]);
			self.fRec158[0] = fRec162;
			let mut fRec159: F32 = fRec163;
			self.fRec154[0] = fSlow26 * self.fRec158[1];
			let mut fRec155: F32 = fRec159;
			self.fRec149[0] = fRec153;
			let mut fRec150: F32 = fSlow26 * self.fRec149[1];
			let mut fRec151: F32 = self.fRec154[0];
			let mut fRec152: F32 = fRec155;
			self.fRec145[(self.IOTA0 & 2047) as usize] = fRec150;
			let mut fRec146: F32 = fRec157;
			let mut fRec147: F32 = fRec151;
			let mut fRec148: F32 = fRec152;
			self.fRec143[0] = fRec146;
			let mut fRec144: F32 = fRec148;
			let mut fTemp241: F32 = F32::abs(fRec144);
			let mut fTemp242: F32 = if (((self.fRec141[1] > fTemp241) as i32) as i32 != 0) { self.fConst16 } else { 0.0 };
			self.fRec142[0] = self.fRec142[1] * fTemp242 + fTemp241 * (1.0 - fTemp242);
			self.fRec141[0] = self.fRec142[0];
			let mut fRec140: F32 = 0.0 - 0.95 * F32::max(2e+01 * F32::log10(self.fRec141[0]) + 1e+01, 0.0);
			self.fRec139[0] = fRec144 * F32::powf(1e+01, 0.05 * fRec140);
			self.fRec137[0] = 0.0 - (self.fRec137[1] * (1.0 - fTemp195) - (self.fRec139[0] + self.fRec139[1])) / (fTemp195 + 1.0);
			self.fRec184[0] = self.fConst2 * self.fRec184[1] + self.fConst1 * unsafe { ftbl0mydspSIG0[(((16.11811 * F32::max(0.0, F32::min(127.0, fSlow31 + self.fRec45[0]))) as i32)) as usize] };
			let mut fTemp243: F32 = 1.0 / F32::tan(self.fConst7 * self.fRec184[0]);
			let mut fRec199: F32 = -1.0 * 0.9973053 * (0.9 * self.fRec200[2] + 0.05 * (self.fRec200[1] + self.fRec200[3]));
			let mut fTemp244: F32 = self.fConst9 * (3.4e+02 / self.fRec184[0] + -0.11);
			let mut fTemp245: F32 = fTemp244 + -1.499995;
			let mut iTemp246: i32 = ((fTemp245) as i32);
			let mut iTemp247: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, i32::wrapping_add(iTemp246, 4))) as F32))) as i32);
			let mut iTemp248: i32 = i32::wrapping_add(iTemp247, 1);
			let mut fTemp249: F32 = F32::floor(fTemp245);
			let mut fTemp250: F32 = fTemp244 + (-3.0 - fTemp249);
			let mut fTemp251: F32 = fTemp244 + (-2.0 - fTemp249);
			let mut fTemp252: F32 = fTemp244 + (-1.0 - fTemp249);
			let mut fTemp253: F32 = fTemp252 * fTemp251;
			let mut fTemp254: F32 = fTemp253 * fTemp250;
			let mut fTemp255: F32 = fTemp244 + (-4.0 - fTemp249);
			let mut fTemp256: F32 = 0.0 - fTemp255;
			let mut iTemp257: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, i32::wrapping_add(iTemp246, 3))) as F32))) as i32);
			let mut iTemp258: i32 = i32::wrapping_add(iTemp257, 1);
			let mut fTemp259: F32 = 0.0 - 0.5 * fTemp255;
			let mut fTemp260: F32 = 0.0 - fTemp250;
			let mut iTemp261: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, i32::wrapping_add(iTemp246, 2))) as F32))) as i32);
			let mut iTemp262: i32 = i32::wrapping_add(iTemp261, 1);
			let mut fTemp263: F32 = 0.0 - 0.33333334 * fTemp255;
			let mut fTemp264: F32 = 0.0 - 0.5 * fTemp250;
			let mut fTemp265: F32 = 0.0 - fTemp251;
			let mut iTemp266: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, i32::wrapping_add(iTemp246, 1))) as F32))) as i32);
			let mut iTemp267: i32 = i32::wrapping_add(iTemp266, 1);
			let mut fTemp268: F32 = fTemp244 - fTemp249;
			let mut fTemp269: F32 = 0.0 - 0.25 * fTemp255;
			let mut fTemp270: F32 = 0.0 - 0.33333334 * fTemp250;
			let mut fTemp271: F32 = 0.0 - 0.5 * fTemp251;
			let mut fTemp272: F32 = 0.0 - fTemp252;
			let mut iTemp273: i32 = ((F32::min(self.fConst8, ((std::cmp::max(0, iTemp246)) as F32))) as i32);
			let mut iTemp274: i32 = i32::wrapping_add(iTemp273, 1);
			self.fRec214[0] = self.fRec191[((i32::wrapping_sub(self.IOTA0, iTemp274)) & 2047) as usize] * fTemp272 * fTemp271 * fTemp270 * fTemp269 + fTemp268 * (self.fRec191[((i32::wrapping_sub(self.IOTA0, iTemp267)) & 2047) as usize] * fTemp265 * fTemp264 * fTemp263 + 0.5 * fTemp252 * self.fRec191[((i32::wrapping_sub(self.IOTA0, iTemp262)) & 2047) as usize] * fTemp260 * fTemp259 + 0.16666667 * fTemp253 * self.fRec191[((i32::wrapping_sub(self.IOTA0, iTemp258)) & 2047) as usize] * fTemp256 + 0.041666668 * fTemp254 * self.fRec191[((i32::wrapping_sub(self.IOTA0, iTemp248)) & 2047) as usize]);
			self.fRec218[0] = 0.05 * self.fRec218[1] + 0.95 * self.fRec214[1];
			let mut fRec215: F32 = self.fRec218[0];
			self.fRec223[0] = self.fConst10 * self.fRec223[1] + self.fConst11 * F32::abs(self.fRec185[1]);
			let mut fRec222: F32 = self.fRec223[0];
			let mut iTemp275: i32 = ((fRec222 > 0.1) as i32);
			self.iVec37[0] = iTemp275;
			self.iRec224[0] = std::cmp::max(i32::wrapping_mul(self.iConst12, ((iTemp275 < self.iVec37[1]) as i32)), i32::wrapping_add(self.iRec224[1], -1));
			let mut fTemp276: F32 = F32::abs(F32::max(((iTemp275) as F32), ((((self.iRec224[0] > 0) as i32)) as F32)));
			let mut fTemp277: F32 = if (((self.fRec220[1] > fTemp276) as i32) as i32 != 0) { self.fConst13 } else { self.fConst10 };
			self.fRec221[0] = self.fRec221[1] * fTemp277 + fTemp276 * (1.0 - fTemp277);
			self.fRec220[0] = self.fRec221[0];
			let mut fTemp278: F32 = 0.005 * self.fRec220[0] * self.fRec185[1];
			self.fRec225[0] = self.fRec189[1];
			self.fRec226[(self.IOTA0 & 2047) as usize] = -1.0 * 0.9973053 * (0.9 * self.fRec225[2] + 0.05 * (self.fRec225[1] + self.fRec225[3]));
			let mut fTemp279: F32 = fTemp253 * fTemp256;
			let mut fTemp280: F32 = fTemp252 * fTemp260 * fTemp259;
			let mut fTemp281: F32 = fTemp265 * fTemp264 * fTemp263;
			let mut fTemp282: F32 = fTemp272 * fTemp271 * fTemp270 * fTemp269;
			self.fVec38[0] = fTemp282 * self.fRec226[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp273, 2))) & 2047) as usize] + fTemp268 * (fTemp281 * self.fRec226[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp266, 2))) & 2047) as usize] + 0.5 * fTemp280 * self.fRec226[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp261, 2))) & 2047) as usize] + 0.16666667 * fTemp279 * self.fRec226[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp257, 2))) & 2047) as usize] + 0.041666668 * fTemp254 * self.fRec226[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp247, 2))) & 2047) as usize]);
			let mut fTemp283: F32 = F32::tan(self.fConst14 * self.fRec184[0]);
			let mut fTemp284: F32 = 1.0 / fTemp283;
			let mut fTemp285: F32 = (fTemp284 + 1.4142135) / fTemp283 + 1.0;
			self.fVec39[0] = fSlow32;
			self.iRec227[0] = i32::wrapping_add(i32::wrapping_mul(i32::wrapping_add(self.iRec227[1], ((self.iRec227[1] > 0) as i32)), ((fSlow32 <= self.fVec39[1]) as i32)), ((fSlow32 > self.fVec39[1]) as i32));
			let mut fTemp286: F32 = ((self.iRec227[0]) as F32) / F32::max(1.0, self.fConst15 * mydsp_faustpower2_f(1.0 - 0.0005 * self.fRec184[0]));
			self.fRec228[0] = fTemp142 - (self.fRec228[2] * ((fTemp284 + -1.4142135) / fTemp283 + 1.0) + 2.0 * self.fRec228[1] * (1.0 - 1.0 / mydsp_faustpower2_f(fTemp283))) / fTemp285;
			let mut fTemp287: F32 = 0.5 * ((self.fRec228[2] + self.fRec228[0] + 2.0 * self.fRec228[1]) * F32::max(0.0, F32::min(fTemp286, 2.0 - fTemp286)) / fTemp285);
			let mut fTemp288: F32 = fTemp287 + self.fVec38[1] + fTemp278;
			self.fVec40[0] = fTemp288;
			self.fRec219[(self.IOTA0 & 2047) as usize] = 0.05 * self.fRec219[((i32::wrapping_sub(self.IOTA0, 1)) & 2047) as usize] + 0.95 * self.fVec40[2];
			let mut fRec216: F32 = fTemp282 * self.fRec219[((i32::wrapping_sub(self.IOTA0, iTemp273)) & 2047) as usize] + fTemp268 * (fTemp281 * self.fRec219[((i32::wrapping_sub(self.IOTA0, iTemp266)) & 2047) as usize] + 0.5 * fTemp280 * self.fRec219[((i32::wrapping_sub(self.IOTA0, iTemp261)) & 2047) as usize] + 0.16666667 * fTemp279 * self.fRec219[((i32::wrapping_sub(self.IOTA0, iTemp257)) & 2047) as usize] + 0.041666668 * fTemp254 * self.fRec219[((i32::wrapping_sub(self.IOTA0, iTemp247)) & 2047) as usize]);
			let mut fRec217: F32 = self.fVec40[1] + self.fRec210[1];
			self.fRec210[0] = fRec215;
			let mut fRec211: F32 = self.fRec210[1];
			let mut fRec212: F32 = fRec216;
			let mut fRec213: F32 = fRec217;
			self.fRec206[0] = fRec211;
			let mut fRec207: F32 = fTemp278 + fTemp287 + self.fRec206[1];
			let mut fRec208: F32 = fRec212;
			let mut fRec209: F32 = fRec213;
			self.fRec202[(self.IOTA0 & 2047) as usize] = fRec207;
			let mut fRec203: F32 = fTemp282 * self.fRec202[((i32::wrapping_sub(self.IOTA0, iTemp274)) & 2047) as usize] + fTemp268 * (fTemp281 * self.fRec202[((i32::wrapping_sub(self.IOTA0, iTemp267)) & 2047) as usize] + 0.5 * fTemp280 * self.fRec202[((i32::wrapping_sub(self.IOTA0, iTemp262)) & 2047) as usize] + 0.16666667 * fTemp279 * self.fRec202[((i32::wrapping_sub(self.IOTA0, iTemp258)) & 2047) as usize] + 0.041666668 * fTemp254 * self.fRec202[((i32::wrapping_sub(self.IOTA0, iTemp248)) & 2047) as usize]);
			self.fRec204[0] = fRec208;
			let mut fRec205: F32 = fRec209;
			self.fRec200[0] = fSlow26 * self.fRec204[1];
			let mut fRec201: F32 = fRec205;
			self.fRec195[0] = fRec199;
			let mut fRec196: F32 = fSlow26 * self.fRec195[1];
			let mut fRec197: F32 = self.fRec200[0];
			let mut fRec198: F32 = fRec201;
			self.fRec191[(self.IOTA0 & 2047) as usize] = fRec196;
			let mut fRec192: F32 = fRec203;
			let mut fRec193: F32 = fRec197;
			let mut fRec194: F32 = fRec198;
			self.fRec189[0] = fRec192;
			let mut fRec190: F32 = fRec194;
			let mut fTemp289: F32 = F32::abs(fRec190);
			let mut fTemp290: F32 = if (((self.fRec187[1] > fTemp289) as i32) as i32 != 0) { self.fConst16 } else { 0.0 };
			self.fRec188[0] = self.fRec188[1] * fTemp290 + fTemp289 * (1.0 - fTemp290);
			self.fRec187[0] = self.fRec188[0];
			let mut fRec186: F32 = 0.0 - 0.95 * F32::max(2e+01 * F32::log10(self.fRec187[0]) + 1e+01, 0.0);
			self.fRec185[0] = fRec190 * F32::powf(1e+01, 0.05 * fRec186);
			self.fRec183[0] = 0.0 - (self.fRec183[1] * (1.0 - fTemp243) - (self.fRec185[0] + self.fRec185[1])) / (fTemp243 + 1.0);
			let mut fTemp291: F32 = (self.fRec183[0] + self.fRec137[0] + self.fRec91[0] + self.fRec43[0]) * self.fRec42[0];
			self.fRec229[0] = fSlow33 + self.fConst2 * self.fRec229[1];
			let mut fTemp292: F32 = F32::min(1.4141995, 1.4142135 * self.fRec229[0]);
			let mut fTemp293: F32 = 1.4142135 * fTemp292;
			let mut fTemp294: F32 = 1.0 - fTemp293;
			self.fRec231[0] = fSlow34 + self.fConst2 * self.fRec231[1];
			self.fRec230[0] = self.fConst2 * self.fRec230[1] + self.fConst1 * unsafe { ftbl0mydspSIG0[(((16.11811 * F32::max(0.0, F32::min(127.0, fSlow35 + self.fRec231[0]))) as i32)) as usize] };
			let mut fTemp295: F32 = F32::tan(self.fConst17 * F32::max(2e+01, F32::min(1e+04, self.fRec230[0])));
			let mut fTemp296: F32 = 1.0 / fTemp295;
			let mut fTemp297: F32 = 2.0 - fTemp293;
			let mut fTemp298: F32 = mydsp_faustpower2_f(fTemp292);
			let mut fTemp299: F32 = fTemp298 + (fTemp297 + fTemp296) / fTemp295 + fTemp294;
			let mut fTemp300: F32 = 1.0 / mydsp_faustpower2_f(fTemp295);
			let mut fTemp301: F32 = fTemp293 + 2.0;
			let mut fTemp302: F32 = fTemp293 + fTemp298;
			let mut fTemp303: F32 = fTemp302 + (fTemp301 + fTemp296) / fTemp295 + 1.0;
			self.fRec236[0] = self.fConst2 * self.fRec236[1] + fSlow36;
			let mut fTemp304: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec236[0]));
			let mut fTemp305: F32 = self.fConst3 * fTemp304;
			let mut fTemp306: F32 = self.fRec234[1] + fTemp305;
			let mut fTemp307: F32 = fTemp306 + -1.0;
			let mut iTemp308: i32 = ((fTemp307 < 0.0) as i32);
			self.fRec234[0] = if (iTemp308 as i32 != 0) { fTemp306 } else { fTemp307 };
			let mut fThen9: F32 = self.fRec234[1] + fTemp305 + (1.0 - self.fConst0 / fTemp304) * fTemp307;
			let mut fRec235: F32 = if (iTemp308 as i32 != 0) { fTemp306 } else { fThen9 };
			self.fRec237[0] = fSlow37 + self.fConst2 * self.fRec237[1];
			self.fRec233[0] = self.fRec237[0] * (2.0 * fRec235 + -1.0) - (self.fRec233[2] * (fTemp302 + (1.0 - (fTemp301 - fTemp296) / fTemp295)) + 2.0 * self.fRec233[1] * (fTemp302 + (1.0 - fTemp300))) / fTemp303;
			self.fRec232[0] = (self.fRec233[2] + self.fRec233[0] + 2.0 * self.fRec233[1]) / fTemp303 - (self.fRec232[2] * (fTemp298 + (1.0 - (fTemp293 + (fTemp297 - fTemp296) / fTemp295))) + 2.0 * self.fRec232[1] * (fTemp298 + (1.0 - (fTemp293 + fTemp300)))) / fTemp299;
			self.fRec238[0] = self.fConst2 * self.fRec238[1] + self.fConst1 * unsafe { ftbl0mydspSIG0[(((16.11811 * F32::max(0.0, F32::min(127.0, fSlow38 + self.fRec231[0]))) as i32)) as usize] };
			let mut fTemp309: F32 = F32::tan(self.fConst17 * F32::max(2e+01, F32::min(1e+04, self.fRec238[0])));
			let mut fTemp310: F32 = 1.0 / fTemp309;
			let mut fTemp311: F32 = fTemp298 + (fTemp297 + fTemp310) / fTemp309 + fTemp294;
			let mut fTemp312: F32 = 1.0 / mydsp_faustpower2_f(fTemp309);
			let mut fTemp313: F32 = fTemp302 + (fTemp301 + fTemp310) / fTemp309 + 1.0;
			self.fRec243[0] = self.fConst2 * self.fRec243[1] + fSlow39;
			let mut fTemp314: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec243[0]));
			let mut fTemp315: F32 = self.fConst3 * fTemp314;
			let mut fTemp316: F32 = self.fRec241[1] + fTemp315;
			let mut fTemp317: F32 = fTemp316 + -1.0;
			let mut iTemp318: i32 = ((fTemp317 < 0.0) as i32);
			self.fRec241[0] = if (iTemp318 as i32 != 0) { fTemp316 } else { fTemp317 };
			let mut fThen11: F32 = fTemp315 + self.fRec241[1] + (1.0 - self.fConst0 / fTemp314) * fTemp317;
			let mut fRec242: F32 = if (iTemp318 as i32 != 0) { fTemp316 } else { fThen11 };
			self.fRec244[0] = fSlow40 + self.fConst2 * self.fRec244[1];
			self.fRec240[0] = self.fRec244[0] * (2.0 * fRec242 + -1.0) - (self.fRec240[2] * (fTemp302 + (fTemp310 - fTemp301) / fTemp309 + 1.0) + 2.0 * self.fRec240[1] * (fTemp302 + (1.0 - fTemp312))) / fTemp313;
			self.fRec239[0] = (self.fRec240[2] + self.fRec240[0] + 2.0 * self.fRec240[1]) / fTemp313 - (self.fRec239[2] * (fTemp298 + (fTemp310 - fTemp297) / fTemp309 + fTemp294) + 2.0 * self.fRec239[1] * (fTemp298 + (1.0 - (fTemp293 + fTemp312)))) / fTemp311;
			self.fRec245[0] = self.fConst2 * self.fRec245[1] + self.fConst1 * unsafe { ftbl0mydspSIG0[(((16.11811 * F32::max(0.0, F32::min(127.0, fSlow41 + self.fRec231[0]))) as i32)) as usize] };
			let mut fTemp319: F32 = F32::tan(self.fConst17 * F32::max(2e+01, F32::min(1e+04, self.fRec245[0])));
			let mut fTemp320: F32 = 1.0 / fTemp319;
			let mut fTemp321: F32 = fTemp298 + (fTemp297 + fTemp320) / fTemp319 + fTemp294;
			let mut fTemp322: F32 = 1.0 / mydsp_faustpower2_f(fTemp319);
			let mut fTemp323: F32 = fTemp302 + (fTemp301 + fTemp320) / fTemp319 + 1.0;
			self.fRec250[0] = self.fConst2 * self.fRec250[1] + fSlow42;
			let mut fTemp324: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec250[0]));
			let mut fTemp325: F32 = self.fRec248[1] + self.fConst3 * fTemp324;
			let mut fTemp326: F32 = fTemp325 + -1.0;
			let mut iTemp327: i32 = ((fTemp326 < 0.0) as i32);
			self.fRec248[0] = if (iTemp327 as i32 != 0) { fTemp325 } else { fTemp326 };
			let mut fThen13: F32 = fTemp325 + (1.0 - self.fConst0 / fTemp324) * fTemp326;
			let mut fRec249: F32 = if (iTemp327 as i32 != 0) { fTemp325 } else { fThen13 };
			self.fRec251[0] = fSlow43 + self.fConst2 * self.fRec251[1];
			self.fRec247[0] = self.fRec251[0] * (2.0 * fRec249 + -1.0) - (self.fRec247[2] * (fTemp302 + (1.0 - (fTemp301 - fTemp320) / fTemp319)) + 2.0 * self.fRec247[1] * (fTemp302 + (1.0 - fTemp322))) / fTemp323;
			self.fRec246[0] = (self.fRec247[2] + self.fRec247[0] + 2.0 * self.fRec247[1]) / fTemp323 - (self.fRec246[2] * (fTemp298 + (fTemp320 - fTemp297) / fTemp319 + fTemp294) + 2.0 * self.fRec246[1] * (fTemp298 + (1.0 - (fTemp293 + fTemp322)))) / fTemp321;
			self.fRec252[0] = self.fConst2 * self.fRec252[1] + self.fConst1 * unsafe { ftbl0mydspSIG0[(((16.11811 * F32::max(0.0, F32::min(127.0, fSlow44 + self.fRec231[0]))) as i32)) as usize] };
			let mut fTemp328: F32 = F32::tan(self.fConst17 * F32::max(2e+01, F32::min(1e+04, self.fRec252[0])));
			let mut fTemp329: F32 = 1.0 / fTemp328;
			let mut fTemp330: F32 = fTemp298 + (fTemp297 + fTemp329) / fTemp328 + fTemp294;
			let mut fTemp331: F32 = 1.0 / mydsp_faustpower2_f(fTemp328);
			let mut fTemp332: F32 = fTemp302 + (fTemp329 + fTemp301) / fTemp328 + 1.0;
			self.fRec257[0] = self.fConst2 * self.fRec257[1] + fSlow45;
			let mut fTemp333: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec257[0]));
			let mut fTemp334: F32 = self.fRec255[1] + self.fConst3 * fTemp333;
			let mut fTemp335: F32 = fTemp334 + -1.0;
			let mut iTemp336: i32 = ((fTemp335 < 0.0) as i32);
			self.fRec255[0] = if (iTemp336 as i32 != 0) { fTemp334 } else { fTemp335 };
			let mut fThen15: F32 = fTemp334 + (1.0 - self.fConst0 / fTemp333) * fTemp335;
			let mut fRec256: F32 = if (iTemp336 as i32 != 0) { fTemp334 } else { fThen15 };
			self.fRec258[0] = fSlow46 + self.fConst2 * self.fRec258[1];
			self.fRec254[0] = self.fRec258[0] * (2.0 * fRec256 + -1.0) - (self.fRec254[2] * (fTemp302 + (fTemp329 - fTemp301) / fTemp328 + 1.0) + 2.0 * self.fRec254[1] * (fTemp302 + (1.0 - fTemp331))) / fTemp332;
			self.fRec253[0] = (self.fRec254[2] + self.fRec254[0] + 2.0 * self.fRec254[1]) / fTemp332 - (self.fRec253[2] * (fTemp298 + (fTemp329 - fTemp297) / fTemp328 + fTemp294) + 2.0 * self.fRec253[1] * (fTemp298 + (1.0 - (fTemp293 + fTemp331)))) / fTemp330;
			self.fRec259[0] = fSlow47 + self.fConst2 * self.fRec259[1];
			self.fRec260[0] = fSlow48 + self.fConst2 * self.fRec260[1];
			let mut fTemp337: F32 = self.fRec260[0] * self.fRec259[0] * ((self.fRec253[2] + self.fRec253[0] + 2.0 * self.fRec253[1]) / fTemp330 + (self.fRec246[2] + self.fRec246[0] + 2.0 * self.fRec246[1]) / fTemp321 + (self.fRec239[2] + self.fRec239[0] + 2.0 * self.fRec239[1]) / fTemp311 + (self.fRec232[2] + self.fRec232[0] + 2.0 * self.fRec232[1]) / fTemp299);
			self.fRec261[0] = fSlow49 + self.fConst2 * self.fRec261[1];
			let mut fTemp338: F32 = (1.0 - self.fRec261[0]) * (fTemp337 + fTemp291 + fTemp97);
			self.fRec263[0] = fSlow50 + self.fConst2 * self.fRec263[1];
			self.fRec262[(self.IOTA0 & 2097151) as usize] = fTemp337 + fTemp97 + fTemp291 + fSlow51 * self.fRec262[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(((F32::min(self.fConst18, F32::max(0.0, self.fConst0 * self.fRec263[0]))) as i32), 1))) & 2097151) as usize];
			let mut fTemp339: F32 = self.fRec262[(self.IOTA0 & 2097151) as usize] * self.fRec261[0];
			let mut fTemp340: F32 = fTemp339 + fTemp338;
			let mut iTemp341: i32 = i32::wrapping_sub(1, self.iVec0[1]);
			self.fRec274[0] = 0.995 * (self.fRec274[1] + ((i32::wrapping_mul(iTemp341, iSlow55)) as F32)) + fSlow56;
			let mut fTemp342: F32 = self.fRec274[0] + -1.49999;
			let mut fTemp343: F32 = F32::floor(fTemp342);
			self.fRec276[0] = 0.995 * (self.fRec276[1] + ((i32::wrapping_mul(iTemp341, iSlow57)) as F32)) + fSlow58;
			let mut fTemp344: F32 = self.fRec276[0] + -1.49999;
			let mut fTemp345: F32 = F32::floor(fTemp344);
			self.fRec280[0] = 0.9999 * (self.fRec280[1] + ((i32::wrapping_mul(iTemp341, iSlow59)) as F32)) + fSlow60;
			let mut fTemp346: F32 = self.fRec280[0] + -1.49999;
			let mut fTemp347: F32 = F32::floor(fTemp346);
			let mut fTemp348: F32 = self.fRec280[0] - fTemp347;
			let mut fTemp349: F32 = fTemp347 + (2.0 - self.fRec280[0]);
			let mut fTemp350: F32 = 0.760314 * self.fRec265[1] - 0.64955574 * self.fRec278[1];
			let mut fTemp351: F32 = 0.760314 * self.fRec264[1] - 0.64955574 * self.fRec277[1];
			self.fVec41[(self.IOTA0 & 16383) as usize] = fTemp351 - fTemp350;
			let mut fTemp352: F32 = self.fVec41[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp346) as i32))))) & 16383) as usize];
			self.fVec42[0] = fTemp352;
			self.fRec279[0] = 0.70710677 * (fTemp349 * fTemp352 / fTemp348 + self.fVec42[1]) - self.fRec279[1] * fTemp349 / fTemp348;
			self.fRec277[0] = self.fRec279[0];
			self.fRec282[0] = 0.9999 * (self.fRec282[1] + ((i32::wrapping_mul(iTemp341, iSlow61)) as F32)) + fSlow62;
			let mut fTemp353: F32 = self.fRec282[0] + -1.49999;
			let mut fTemp354: F32 = F32::floor(fTemp353);
			let mut fTemp355: F32 = self.fRec282[0] - fTemp354;
			let mut fTemp356: F32 = fTemp354 + (2.0 - self.fRec282[0]);
			self.fVec43[(self.IOTA0 & 16383) as usize] = fTemp351 + fTemp350;
			let mut fTemp357: F32 = self.fVec43[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp353) as i32))))) & 16383) as usize];
			self.fVec44[0] = fTemp357;
			self.fRec281[0] = 0.70710677 * (fTemp356 * fTemp357 / fTemp355 + self.fVec44[1]) - self.fRec281[1] * fTemp356 / fTemp355;
			self.fRec278[0] = self.fRec281[0];
			let mut fTemp358: F32 = 0.760314 * self.fRec277[1] + 0.64955574 * self.fRec264[1];
			self.fRec286[0] = 0.9999 * (self.fRec286[1] + ((i32::wrapping_mul(iTemp341, iSlow63)) as F32)) + fSlow64;
			let mut fTemp359: F32 = self.fRec286[0] + -1.49999;
			let mut fTemp360: F32 = F32::floor(fTemp359);
			let mut fTemp361: F32 = 0.760314 * fTemp358 - 0.64955574 * self.fRec283[1];
			let mut fTemp362: F32 = 0.760314 * self.fRec278[1] + 0.64955574 * self.fRec265[1];
			let mut fTemp363: F32 = 0.760314 * fTemp362 - 0.64955574 * self.fRec284[1];
			self.fVec45[(self.IOTA0 & 16383) as usize] = 0.0 - 0.70710677 * (fTemp363 - fTemp361);
			let mut fTemp364: F32 = self.fVec45[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp359) as i32))))) & 16383) as usize];
			self.fVec46[0] = fTemp364;
			self.fRec285[0] = self.fVec46[1] - (fTemp360 + (2.0 - self.fRec286[0])) * (self.fRec285[1] - fTemp364) / (self.fRec286[0] - fTemp360);
			self.fRec283[0] = self.fRec285[0];
			self.fRec288[0] = 0.9999 * (self.fRec288[1] + ((i32::wrapping_mul(iTemp341, iSlow65)) as F32)) + fSlow66;
			let mut fTemp365: F32 = self.fRec288[0] + -1.49999;
			let mut fTemp366: F32 = F32::floor(fTemp365);
			let mut fTemp367: F32 = self.fRec288[0] - fTemp366;
			let mut fTemp368: F32 = fTemp366 + (2.0 - self.fRec288[0]);
			self.fVec47[(self.IOTA0 & 16383) as usize] = fTemp361 + fTemp363;
			let mut fTemp369: F32 = self.fVec47[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp365) as i32))))) & 16383) as usize];
			self.fVec48[0] = fTemp369;
			self.fRec287[0] = 0.70710677 * (fTemp368 * fTemp369 / fTemp367 + self.fVec48[1]) - self.fRec287[1] * fTemp368 / fTemp367;
			self.fRec284[0] = self.fRec287[0];
			let mut fTemp370: F32 = 0.760314 * self.fRec283[1] + 0.64955574 * fTemp358;
			self.fRec292[0] = 0.9999 * (self.fRec292[1] + ((i32::wrapping_mul(iTemp341, iSlow67)) as F32)) + fSlow68;
			let mut fTemp371: F32 = self.fRec292[0] + -1.49999;
			let mut fTemp372: F32 = F32::floor(fTemp371);
			let mut fTemp373: F32 = 0.760314 * fTemp370 - 0.64955574 * self.fRec289[1];
			let mut fTemp374: F32 = 0.760314 * self.fRec284[1] + 0.64955574 * fTemp362;
			let mut fTemp375: F32 = 0.760314 * fTemp374 - 0.64955574 * self.fRec290[1];
			self.fVec49[(self.IOTA0 & 16383) as usize] = 0.0 - 0.70710677 * (fTemp375 - fTemp373);
			let mut fTemp376: F32 = self.fVec49[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp371) as i32))))) & 16383) as usize];
			self.fVec50[0] = fTemp376;
			self.fRec291[0] = self.fVec50[1] - (fTemp372 + (2.0 - self.fRec292[0])) * (self.fRec291[1] - fTemp376) / (self.fRec292[0] - fTemp372);
			self.fRec289[0] = self.fRec291[0];
			self.fRec294[0] = 0.9999 * (self.fRec294[1] + ((i32::wrapping_mul(iTemp341, iSlow69)) as F32)) + fSlow70;
			let mut fTemp377: F32 = self.fRec294[0] + -1.49999;
			let mut fTemp378: F32 = F32::floor(fTemp377);
			let mut fTemp379: F32 = self.fRec294[0] - fTemp378;
			let mut fTemp380: F32 = fTemp378 + (2.0 - self.fRec294[0]);
			self.fVec51[(self.IOTA0 & 16383) as usize] = fTemp373 + fTemp375;
			let mut fTemp381: F32 = self.fVec51[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp377) as i32))))) & 16383) as usize];
			self.fVec52[0] = fTemp381;
			self.fRec293[0] = 0.70710677 * (fTemp380 * fTemp381 / fTemp379 + self.fVec52[1]) - self.fRec293[1] * fTemp380 / fTemp379;
			self.fRec290[0] = self.fRec293[0];
			let mut fTemp382: F32 = 0.760314 * self.fRec289[1] + 0.64955574 * fTemp370;
			self.fRec298[0] = 0.9999 * (self.fRec298[1] + ((i32::wrapping_mul(iTemp341, iSlow71)) as F32)) + fSlow72;
			let mut fTemp383: F32 = self.fRec298[0] + -1.49999;
			let mut fTemp384: F32 = F32::floor(fTemp383);
			let mut fTemp385: F32 = 0.760314 * fTemp382 - 0.64955574 * self.fRec295[1];
			let mut fTemp386: F32 = 0.760314 * self.fRec290[1] + 0.64955574 * fTemp374;
			let mut fTemp387: F32 = 0.760314 * fTemp386 - 0.64955574 * self.fRec296[1];
			self.fVec53[(self.IOTA0 & 16383) as usize] = 0.0 - 0.70710677 * (fTemp387 - fTemp385);
			let mut fTemp388: F32 = self.fVec53[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp383) as i32))))) & 16383) as usize];
			self.fVec54[0] = fTemp388;
			self.fRec297[0] = self.fVec54[1] - (fTemp384 + (2.0 - self.fRec298[0])) * (self.fRec297[1] - fTemp388) / (self.fRec298[0] - fTemp384);
			self.fRec295[0] = self.fRec297[0];
			self.fRec300[0] = 0.9999 * (self.fRec300[1] + ((i32::wrapping_mul(iTemp341, iSlow73)) as F32)) + fSlow74;
			let mut fTemp389: F32 = self.fRec300[0] + -1.49999;
			let mut fTemp390: F32 = F32::floor(fTemp389);
			let mut fTemp391: F32 = self.fRec300[0] - fTemp390;
			let mut fTemp392: F32 = fTemp390 + (2.0 - self.fRec300[0]);
			self.fVec55[(self.IOTA0 & 16383) as usize] = fTemp385 + fTemp387;
			let mut fTemp393: F32 = self.fVec55[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp389) as i32))))) & 16383) as usize];
			self.fVec56[0] = fTemp393;
			self.fRec299[0] = 0.70710677 * (fTemp392 * fTemp393 / fTemp391 + self.fVec56[1]) - self.fRec299[1] * fTemp392 / fTemp391;
			self.fRec296[0] = self.fRec299[0];
			let mut fTemp394: F32 = 0.760314 * self.fRec295[1] + 0.64955574 * fTemp382;
			self.fRec304[0] = 0.9999 * (self.fRec304[1] + ((i32::wrapping_mul(iTemp341, iSlow75)) as F32)) + fSlow76;
			let mut fTemp395: F32 = self.fRec304[0] + -1.49999;
			let mut fTemp396: F32 = F32::floor(fTemp395);
			let mut fTemp397: F32 = self.fRec304[0] - fTemp396;
			let mut fTemp398: F32 = fTemp396 + (2.0 - self.fRec304[0]);
			let mut fTemp399: F32 = 0.760314 * self.fRec296[1] + 0.64955574 * fTemp386;
			let mut fTemp400: F32 = 0.760314 * fTemp399 - 0.64955574 * self.fRec302[1];
			let mut fTemp401: F32 = 0.760314 * fTemp394 - 0.64955574 * self.fRec301[1];
			self.fVec57[(self.IOTA0 & 16383) as usize] = fTemp401 - fTemp400;
			let mut fTemp402: F32 = self.fVec57[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp395) as i32))))) & 16383) as usize];
			self.fVec58[0] = fTemp402;
			self.fRec303[0] = 0.70710677 * (fTemp398 * fTemp402 / fTemp397 + self.fVec58[1]) - self.fRec303[1] * fTemp398 / fTemp397;
			self.fRec301[0] = self.fRec303[0];
			self.fRec306[0] = 0.9999 * (self.fRec306[1] + ((i32::wrapping_mul(iTemp341, iSlow77)) as F32)) + fSlow78;
			let mut fTemp403: F32 = self.fRec306[0] + -1.49999;
			let mut fTemp404: F32 = F32::floor(fTemp403);
			let mut fTemp405: F32 = self.fRec306[0] - fTemp404;
			let mut fTemp406: F32 = fTemp404 + (2.0 - self.fRec306[0]);
			self.fVec59[(self.IOTA0 & 16383) as usize] = fTemp401 + fTemp400;
			let mut fTemp407: F32 = self.fVec59[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp403) as i32))))) & 16383) as usize];
			self.fVec60[0] = fTemp407;
			self.fRec305[0] = 0.70710677 * (fTemp406 * fTemp407 / fTemp405 + self.fVec60[1]) - self.fRec305[1] * fTemp406 / fTemp405;
			self.fRec302[0] = self.fRec305[0];
			let mut fTemp408: F32 = 0.760314 * self.fRec301[1] + 0.64955574 * fTemp394;
			self.fVec61[(self.IOTA0 & 1023) as usize] = fTemp408;
			self.fRec307[0] = fSlow81 * self.fRec308[1] + fSlow80 * self.fRec307[1];
			self.fRec308[0] = ((iTemp341) as F32) + fSlow80 * self.fRec308[1] - fSlow81 * self.fRec307[1];
			let mut fTemp409: F32 = fSlow82 * (self.fRec308[0] + 1.0);
			let mut fTemp410: F32 = fTemp409 + 3.500005;
			let mut iTemp411: i32 = ((fTemp410) as i32);
			let mut iTemp412: i32 = std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp411, 4)));
			let mut fTemp413: F32 = F32::floor(fTemp410);
			let mut fTemp414: F32 = fTemp409 + (2.0 - fTemp413);
			let mut fTemp415: F32 = fTemp409 + (3.0 - fTemp413);
			let mut fTemp416: F32 = fTemp409 + (4.0 - fTemp413);
			let mut fTemp417: F32 = fTemp416 * fTemp415;
			let mut fTemp418: F32 = fTemp417 * fTemp414;
			let mut fTemp419: F32 = fTemp409 + (1.0 - fTemp413);
			let mut fTemp420: F32 = 0.0 - fTemp419;
			let mut iTemp421: i32 = std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp411, 3)));
			let mut fTemp422: F32 = 0.0 - 0.5 * fTemp419;
			let mut fTemp423: F32 = 0.0 - fTemp414;
			let mut iTemp424: i32 = std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp411, 2)));
			let mut fTemp425: F32 = 0.0 - 0.33333334 * fTemp419;
			let mut fTemp426: F32 = 0.0 - 0.5 * fTemp414;
			let mut fTemp427: F32 = 0.0 - fTemp415;
			let mut iTemp428: i32 = std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp411, 1)));
			let mut fTemp429: F32 = fTemp409 + (5.0 - fTemp413);
			let mut fTemp430: F32 = 0.0 - 0.25 * fTemp419;
			let mut fTemp431: F32 = 0.0 - 0.33333334 * fTemp414;
			let mut fTemp432: F32 = 0.0 - 0.5 * fTemp415;
			let mut fTemp433: F32 = 0.0 - fTemp416;
			let mut iTemp434: i32 = std::cmp::min(512, std::cmp::max(0, iTemp411));
			self.fVec62[(self.IOTA0 & 16383) as usize] = self.fVec61[((i32::wrapping_sub(self.IOTA0, iTemp434)) & 1023) as usize] * fTemp433 * fTemp432 * fTemp431 * fTemp430 + fTemp429 * (self.fVec61[((i32::wrapping_sub(self.IOTA0, iTemp428)) & 1023) as usize] * fTemp427 * fTemp426 * fTemp425 + 0.5 * fTemp416 * self.fVec61[((i32::wrapping_sub(self.IOTA0, iTemp424)) & 1023) as usize] * fTemp423 * fTemp422 + 0.16666667 * fTemp417 * self.fVec61[((i32::wrapping_sub(self.IOTA0, iTemp421)) & 1023) as usize] * fTemp420 + 0.041666668 * fTemp418 * self.fVec61[((i32::wrapping_sub(self.IOTA0, iTemp412)) & 1023) as usize]);
			let mut fTemp435: F32 = self.fVec62[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp344) as i32))))) & 16383) as usize];
			self.fVec63[0] = fTemp435;
			self.fRec275[0] = self.fVec63[1] - (fTemp345 + (2.0 - self.fRec276[0])) * (self.fRec275[1] - fTemp435) / (self.fRec276[0] - fTemp345);
			self.fRec312[0] = 0.9999 * (self.fRec312[1] + ((i32::wrapping_mul(iTemp341, iSlow83)) as F32)) + fSlow84;
			let mut fTemp436: F32 = self.fRec312[0] + -1.49999;
			let mut fTemp437: F32 = F32::floor(fTemp436);
			let mut fTemp438: F32 = self.fRec312[0] - fTemp437;
			let mut fTemp439: F32 = fTemp437 + (2.0 - self.fRec312[0]);
			self.fRec314[0] = 0.995 * (self.fRec314[1] + ((i32::wrapping_mul(iTemp341, iSlow85)) as F32)) + fSlow86;
			let mut fTemp440: F32 = self.fRec314[0] + -1.49999;
			let mut fTemp441: F32 = F32::floor(fTemp440);
			let mut fTemp442: F32 = 0.760314 * self.fRec302[1] + 0.64955574 * fTemp399;
			self.fVec64[(self.IOTA0 & 1023) as usize] = fTemp442;
			let mut fTemp443: F32 = fSlow87 * self.fRec308[0];
			let mut fTemp444: F32 = fSlow82 + fTemp443 + 3.500005;
			let mut iTemp445: i32 = ((fTemp444) as i32);
			let mut fTemp446: F32 = F32::floor(fTemp444);
			let mut fTemp447: F32 = fSlow82 + fTemp443 + (2.0 - fTemp446);
			let mut fTemp448: F32 = fSlow82 + fTemp443 + (3.0 - fTemp446);
			let mut fTemp449: F32 = fSlow82 + fTemp443 + (4.0 - fTemp446);
			let mut fTemp450: F32 = fTemp449 * fTemp448;
			let mut fTemp451: F32 = fSlow82 + fTemp443 + (1.0 - fTemp446);
			self.fVec65[(self.IOTA0 & 16383) as usize] = self.fVec64[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, std::cmp::max(0, iTemp445)))) & 1023) as usize] * (0.0 - fTemp449) * (0.0 - 0.5 * fTemp448) * (0.0 - 0.33333334 * fTemp447) * (0.0 - 0.25 * fTemp451) + (fSlow82 + fTemp443 + (5.0 - fTemp446)) * (self.fVec64[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp445, 1))))) & 1023) as usize] * (0.0 - fTemp448) * (0.0 - 0.5 * fTemp447) * (0.0 - 0.33333334 * fTemp451) + 0.5 * fTemp449 * self.fVec64[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp445, 2))))) & 1023) as usize] * (0.0 - fTemp447) * (0.0 - 0.5 * fTemp451) + 0.16666667 * fTemp450 * self.fVec64[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp445, 3))))) & 1023) as usize] * (0.0 - fTemp451) + 0.041666668 * fTemp450 * fTemp447 * self.fVec64[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, std::cmp::max(0, i32::wrapping_add(iTemp445, 4))))) & 1023) as usize]);
			let mut fTemp452: F32 = self.fVec65[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp440) as i32))))) & 16383) as usize];
			self.fVec66[0] = fTemp452;
			self.fRec313[0] = self.fVec66[1] - (fTemp441 + (2.0 - self.fRec314[0])) * (self.fRec313[1] - fTemp452) / (self.fRec314[0] - fTemp441);
			let mut fTemp453: F32 = 0.760314 * self.fRec313[0] - 0.64955574 * self.fRec310[1];
			let mut fTemp454: F32 = 0.760314 * self.fRec275[0] - 0.64955574 * self.fRec309[1];
			self.fVec67[(self.IOTA0 & 16383) as usize] = fTemp454 - fTemp453;
			let mut fTemp455: F32 = self.fVec67[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp436) as i32))))) & 16383) as usize];
			self.fVec68[0] = fTemp455;
			self.fRec311[0] = 0.70710677 * (fTemp439 * fTemp455 / fTemp438 + self.fVec68[1]) - self.fRec311[1] * fTemp439 / fTemp438;
			self.fRec309[0] = self.fRec311[0];
			self.fRec316[0] = 0.9999 * (self.fRec316[1] + ((i32::wrapping_mul(iTemp341, iSlow88)) as F32)) + fSlow89;
			let mut fTemp456: F32 = self.fRec316[0] + -1.49999;
			let mut fTemp457: F32 = F32::floor(fTemp456);
			let mut fTemp458: F32 = self.fRec316[0] - fTemp457;
			let mut fTemp459: F32 = fTemp457 + (2.0 - self.fRec316[0]);
			self.fVec69[(self.IOTA0 & 16383) as usize] = fTemp454 + fTemp453;
			let mut fTemp460: F32 = self.fVec69[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp456) as i32))))) & 16383) as usize];
			self.fVec70[0] = fTemp460;
			self.fRec315[0] = 0.70710677 * (fTemp459 * fTemp460 / fTemp458 + self.fVec70[1]) - self.fRec315[1] * fTemp459 / fTemp458;
			self.fRec310[0] = self.fRec315[0];
			let mut fTemp461: F32 = 0.760314 * self.fRec309[1] + 0.64955574 * self.fRec275[0];
			self.fRec320[0] = 0.9999 * (self.fRec320[1] + ((i32::wrapping_mul(iTemp341, iSlow90)) as F32)) + fSlow91;
			let mut fTemp462: F32 = self.fRec320[0] + -1.49999;
			let mut fTemp463: F32 = F32::floor(fTemp462);
			let mut fTemp464: F32 = 0.760314 * fTemp461 - 0.64955574 * self.fRec317[1];
			let mut fTemp465: F32 = 0.760314 * self.fRec310[1] + 0.64955574 * self.fRec313[0];
			let mut fTemp466: F32 = 0.760314 * fTemp465 - 0.64955574 * self.fRec318[1];
			self.fVec71[(self.IOTA0 & 16383) as usize] = 0.0 - 0.70710677 * (fTemp466 - fTemp464);
			let mut fTemp467: F32 = self.fVec71[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp462) as i32))))) & 16383) as usize];
			self.fVec72[0] = fTemp467;
			self.fRec319[0] = self.fVec72[1] - (fTemp463 + (2.0 - self.fRec320[0])) * (self.fRec319[1] - fTemp467) / (self.fRec320[0] - fTemp463);
			self.fRec317[0] = self.fRec319[0];
			self.fRec322[0] = 0.9999 * (self.fRec322[1] + ((i32::wrapping_mul(iTemp341, iSlow92)) as F32)) + fSlow93;
			let mut fTemp468: F32 = self.fRec322[0] + -1.49999;
			let mut fTemp469: F32 = F32::floor(fTemp468);
			let mut fTemp470: F32 = self.fRec322[0] - fTemp469;
			let mut fTemp471: F32 = fTemp469 + (2.0 - self.fRec322[0]);
			self.fVec73[(self.IOTA0 & 16383) as usize] = fTemp464 + fTemp466;
			let mut iTemp472: i32 = std::cmp::min(8192, std::cmp::max(0, ((fTemp468) as i32)));
			let mut fTemp473: F32 = self.fVec73[((i32::wrapping_sub(self.IOTA0, iTemp472)) & 16383) as usize];
			self.fVec74[0] = fTemp473;
			self.fRec321[0] = 0.70710677 * (fTemp471 * fTemp473 / fTemp470 + self.fVec74[1]) - fTemp471 * self.fRec321[1] / fTemp470;
			self.fRec318[0] = self.fRec321[0];
			let mut fTemp474: F32 = 0.760314 * self.fRec317[1] + 0.64955574 * fTemp461;
			self.fRec326[0] = 0.9999 * (self.fRec326[1] + ((i32::wrapping_mul(iTemp341, iSlow94)) as F32)) + fSlow95;
			let mut fTemp475: F32 = self.fRec326[0] + -1.49999;
			let mut fTemp476: F32 = F32::floor(fTemp475);
			let mut fTemp477: F32 = self.fRec326[0] - fTemp476;
			let mut fTemp478: F32 = fTemp476 + (2.0 - self.fRec326[0]);
			let mut fTemp479: F32 = 0.760314 * self.fRec318[1] + 0.64955574 * fTemp465;
			let mut fTemp480: F32 = 0.760314 * fTemp479 - 0.64955574 * self.fRec324[1];
			let mut fTemp481: F32 = 0.760314 * fTemp474 - 0.64955574 * self.fRec323[1];
			self.fVec75[(self.IOTA0 & 16383) as usize] = fTemp481 - fTemp480;
			let mut fTemp482: F32 = self.fVec75[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp475) as i32))))) & 16383) as usize];
			self.fVec76[0] = fTemp482;
			self.fRec325[0] = 0.70710677 * (fTemp478 * fTemp482 / fTemp477 + self.fVec76[1]) - self.fRec325[1] * fTemp478 / fTemp477;
			self.fRec323[0] = self.fRec325[0];
			self.fRec328[0] = 0.9999 * (self.fRec328[1] + ((i32::wrapping_mul(iTemp341, iSlow96)) as F32)) + fSlow97;
			let mut fTemp483: F32 = self.fRec328[0] + -1.49999;
			let mut fTemp484: F32 = F32::floor(fTemp483);
			let mut fTemp485: F32 = self.fRec328[0] - fTemp484;
			let mut fTemp486: F32 = fTemp484 + (2.0 - self.fRec328[0]);
			self.fVec77[(self.IOTA0 & 16383) as usize] = fTemp481 + fTemp480;
			let mut iTemp487: i32 = std::cmp::min(8192, std::cmp::max(0, ((fTemp483) as i32)));
			let mut fTemp488: F32 = self.fVec77[((i32::wrapping_sub(self.IOTA0, iTemp487)) & 16383) as usize];
			self.fVec78[0] = fTemp488;
			self.fRec327[0] = 0.70710677 * (fTemp486 * fTemp488 / fTemp485 + self.fVec78[1]) - self.fRec327[1] * fTemp486 / fTemp485;
			self.fRec324[0] = self.fRec327[0];
			let mut fTemp489: F32 = 0.760314 * self.fRec323[1] + 0.64955574 * fTemp474;
			self.fRec332[0] = 0.9999 * (self.fRec332[1] + ((i32::wrapping_mul(iTemp341, iSlow98)) as F32)) + fSlow99;
			let mut fTemp490: F32 = self.fRec332[0] + -1.49999;
			let mut fTemp491: F32 = F32::floor(fTemp490);
			let mut fTemp492: F32 = self.fRec332[0] - fTemp491;
			let mut fTemp493: F32 = fTemp491 + (2.0 - self.fRec332[0]);
			let mut fTemp494: F32 = 0.760314 * self.fRec324[1] + 0.64955574 * fTemp479;
			let mut fTemp495: F32 = 0.760314 * fTemp494 - 0.64955574 * self.fRec330[1];
			let mut fTemp496: F32 = 0.760314 * fTemp489 - 0.64955574 * self.fRec329[1];
			self.fVec79[(self.IOTA0 & 16383) as usize] = fTemp496 - fTemp495;
			let mut iTemp497: i32 = std::cmp::min(8192, std::cmp::max(0, ((fTemp490) as i32)));
			let mut fTemp498: F32 = self.fVec79[((i32::wrapping_sub(self.IOTA0, iTemp497)) & 16383) as usize];
			self.fVec80[0] = fTemp498;
			self.fRec331[0] = 0.70710677 * (fTemp493 * fTemp498 / fTemp492 + self.fVec80[1]) - fTemp493 * self.fRec331[1] / fTemp492;
			self.fRec329[0] = self.fRec331[0];
			self.fRec334[0] = 0.9999 * (self.fRec334[1] + ((i32::wrapping_mul(iTemp341, iSlow100)) as F32)) + fSlow101;
			let mut fTemp499: F32 = self.fRec334[0] + -1.49999;
			let mut fTemp500: F32 = F32::floor(fTemp499);
			let mut fTemp501: F32 = self.fRec334[0] - fTemp500;
			let mut fTemp502: F32 = fTemp500 + (2.0 - self.fRec334[0]);
			self.fVec81[(self.IOTA0 & 16383) as usize] = fTemp496 + fTemp495;
			let mut iTemp503: i32 = std::cmp::min(8192, std::cmp::max(0, ((fTemp499) as i32)));
			let mut fTemp504: F32 = self.fVec81[((i32::wrapping_sub(self.IOTA0, iTemp503)) & 16383) as usize];
			self.fVec82[0] = fTemp504;
			self.fRec333[0] = 0.70710677 * (fTemp502 * fTemp504 / fTemp501 + self.fVec82[1]) - fTemp502 * self.fRec333[1] / fTemp501;
			self.fRec330[0] = self.fRec333[0];
			let mut fTemp505: F32 = 0.760314 * self.fRec329[1] + 0.64955574 * fTemp489;
			self.fRec338[0] = 0.9999 * (self.fRec338[1] + ((i32::wrapping_mul(iTemp341, iSlow102)) as F32)) + fSlow103;
			let mut fTemp506: F32 = self.fRec338[0] + -1.49999;
			let mut fTemp507: F32 = F32::floor(fTemp506);
			let mut fTemp508: F32 = 0.760314 * fTemp505 - 0.64955574 * self.fRec335[1];
			let mut fTemp509: F32 = 0.760314 * self.fRec330[1] + 0.64955574 * fTemp494;
			let mut fTemp510: F32 = 0.760314 * fTemp509 - 0.64955574 * self.fRec336[1];
			self.fVec83[(self.IOTA0 & 16383) as usize] = 0.0 - 0.70710677 * (fTemp510 - fTemp508);
			let mut fTemp511: F32 = self.fVec83[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp506) as i32))))) & 16383) as usize];
			self.fVec84[0] = fTemp511;
			self.fRec337[0] = self.fVec84[1] - (fTemp507 + (2.0 - self.fRec338[0])) * (self.fRec337[1] - fTemp511) / (self.fRec338[0] - fTemp507);
			self.fRec335[0] = self.fRec337[0];
			self.fRec340[0] = 0.9999 * (self.fRec340[1] + ((i32::wrapping_mul(iTemp341, iSlow104)) as F32)) + fSlow105;
			let mut fTemp512: F32 = self.fRec340[0] + -1.49999;
			let mut fTemp513: F32 = F32::floor(fTemp512);
			let mut fTemp514: F32 = self.fRec340[0] - fTemp513;
			let mut fTemp515: F32 = fTemp513 + (2.0 - self.fRec340[0]);
			self.fVec85[(self.IOTA0 & 16383) as usize] = fTemp508 + fTemp510;
			let mut iTemp516: i32 = std::cmp::min(8192, std::cmp::max(0, ((fTemp512) as i32)));
			let mut fTemp517: F32 = self.fVec85[((i32::wrapping_sub(self.IOTA0, iTemp516)) & 16383) as usize];
			self.fVec86[0] = fTemp517;
			self.fRec339[0] = 0.70710677 * (fTemp515 * fTemp517 / fTemp514 + self.fVec86[1]) - self.fRec339[1] * fTemp515 / fTemp514;
			self.fRec336[0] = self.fRec339[0];
			let mut fTemp518: F32 = 0.760314 * self.fRec335[1] + 0.64955574 * fTemp505;
			self.fVec87[(self.IOTA0 & 16383) as usize] = fTemp518;
			let mut fTemp519: F32 = fSlow82 * (self.fRec307[0] + 1.0);
			let mut fTemp520: F32 = fTemp519 + 3.500005;
			let mut iTemp521: i32 = ((fTemp520) as i32);
			let mut iTemp522: i32 = std::cmp::max(0, i32::wrapping_add(iTemp521, 4));
			let mut fTemp523: F32 = F32::floor(fTemp520);
			let mut fTemp524: F32 = fTemp519 + (2.0 - fTemp523);
			let mut fTemp525: F32 = fTemp519 + (3.0 - fTemp523);
			let mut fTemp526: F32 = fTemp519 + (4.0 - fTemp523);
			let mut fTemp527: F32 = fTemp526 * fTemp525;
			let mut fTemp528: F32 = fTemp527 * fTemp524;
			let mut fTemp529: F32 = fTemp519 + (1.0 - fTemp523);
			let mut fTemp530: F32 = 0.0 - fTemp529;
			let mut iTemp531: i32 = std::cmp::max(0, i32::wrapping_add(iTemp521, 3));
			let mut fTemp532: F32 = 0.0 - 0.5 * fTemp529;
			let mut fTemp533: F32 = 0.0 - fTemp524;
			let mut iTemp534: i32 = std::cmp::max(0, i32::wrapping_add(iTemp521, 2));
			let mut fTemp535: F32 = 0.0 - 0.33333334 * fTemp529;
			let mut fTemp536: F32 = 0.0 - 0.5 * fTemp524;
			let mut fTemp537: F32 = 0.0 - fTemp525;
			let mut iTemp538: i32 = std::cmp::max(0, i32::wrapping_add(iTemp521, 1));
			let mut fTemp539: F32 = fTemp519 + (5.0 - fTemp523);
			let mut fTemp540: F32 = 0.0 - 0.25 * fTemp529;
			let mut fTemp541: F32 = 0.0 - 0.33333334 * fTemp524;
			let mut fTemp542: F32 = 0.0 - 0.5 * fTemp525;
			let mut fTemp543: F32 = 0.0 - fTemp526;
			let mut iTemp544: i32 = std::cmp::max(0, iTemp521);
			self.fVec88[(self.IOTA0 & 16383) as usize] = self.fVec87[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, iTemp544))) & 16383) as usize] * fTemp543 * fTemp542 * fTemp541 * fTemp540 + fTemp539 * (self.fVec87[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, iTemp538))) & 16383) as usize] * fTemp537 * fTemp536 * fTemp535 + 0.5 * fTemp526 * self.fVec87[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, iTemp534))) & 16383) as usize] * fTemp533 * fTemp532 + 0.16666667 * fTemp527 * self.fVec87[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, iTemp531))) & 16383) as usize] * fTemp530 + 0.041666668 * fTemp528 * self.fVec87[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, iTemp522))) & 16383) as usize]);
			let mut fTemp545: F32 = self.fVec88[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp342) as i32))))) & 16383) as usize];
			self.fVec89[0] = fTemp545;
			self.fRec273[0] = self.fVec89[1] - (fTemp343 + (2.0 - self.fRec274[0])) * (self.fRec273[1] - fTemp545) / (self.fRec274[0] - fTemp343);
			self.fRec272[0] = self.fConst39 * (self.fRec273[0] + self.fRec273[1] - self.fConst37 * self.fRec272[1]);
			self.fRec271[0] = self.fRec272[0] - self.fConst36 * (self.fConst35 * self.fRec271[2] + self.fConst31 * self.fRec271[1]);
			self.fRec270[0] = self.fConst36 * (self.fRec271[2] + self.fRec271[0] + 2.0 * self.fRec271[1]) - self.fConst34 * (self.fConst33 * self.fRec270[2] + self.fConst31 * self.fRec270[1]);
			let mut fTemp546: F32 = self.fRec270[2] + self.fRec270[0] + 2.0 * self.fRec270[1];
			self.fVec90[0] = fTemp546;
			self.fRec269[0] = 0.0 - self.fConst42 * (self.fConst40 * self.fRec269[1] - self.fConst34 * (fTemp546 + self.fVec90[1]));
			self.fRec268[0] = self.fRec269[0] - self.fConst27 * (self.fConst26 * self.fRec268[2] + self.fConst22 * self.fRec268[1]);
			self.fRec267[0] = self.fConst27 * (self.fRec268[2] + self.fRec268[0] + 2.0 * self.fRec268[1]) - self.fConst25 * (self.fConst24 * self.fRec267[2] + self.fConst22 * self.fRec267[1]);
			self.fRec343[0] = self.fConst34 * (self.fConst44 * fTemp546 + self.fConst45 * self.fVec90[1]) - self.fConst43 * self.fRec343[1];
			self.fRec342[0] = self.fRec343[0] - self.fConst27 * (self.fConst26 * self.fRec342[2] + self.fConst22 * self.fRec342[1]);
			self.fRec341[0] = self.fConst27 * (self.fConst21 * self.fRec342[0] + self.fConst46 * self.fRec342[1] + self.fConst21 * self.fRec342[2]) - self.fConst25 * (self.fConst24 * self.fRec341[2] + self.fConst22 * self.fRec341[1]);
			let mut fTemp547: F32 = self.fConst22 * self.fRec344[1];
			self.fRec347[0] = self.fConst49 * self.fRec273[1] - self.fConst39 * (self.fConst37 * self.fRec347[1] - self.fConst32 * self.fRec273[0]);
			self.fRec346[0] = self.fRec347[0] - self.fConst36 * (self.fConst35 * self.fRec346[2] + self.fConst31 * self.fRec346[1]);
			self.fRec345[0] = self.fConst36 * (self.fConst30 * self.fRec346[0] + self.fConst50 * self.fRec346[1] + self.fConst30 * self.fRec346[2]) - self.fConst34 * (self.fConst33 * self.fRec345[2] + self.fConst31 * self.fRec345[1]);
			self.fRec344[0] = self.fConst34 * (self.fConst30 * self.fRec345[0] + self.fConst50 * self.fRec345[1] + self.fConst30 * self.fRec345[2]) - self.fConst48 * (self.fConst47 * self.fRec344[2] + fTemp547);
			let mut fTemp548: F32 = fTemp338 + fTemp339 + fSlow106 * (self.fRec344[2] + self.fConst48 * (fTemp547 + self.fConst47 * self.fRec344[0]) + self.fConst25 * (self.fConst46 * self.fRec341[1] + self.fConst21 * self.fRec341[0] + self.fConst21 * self.fRec341[2] + self.fRec267[2] + self.fRec267[0] + 2.0 * self.fRec267[1]));
			self.fVec91[(self.IOTA0 & 1023) as usize] = fTemp548;
			self.fRec266[0] = fSlow107 * self.fRec266[1] + fSlow108 * (fTemp543 * fTemp542 * fTemp541 * fTemp540 * self.fVec91[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, iTemp544))) & 1023) as usize] + fTemp539 * (fTemp537 * fTemp536 * fTemp535 * self.fVec91[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, iTemp538))) & 1023) as usize] + 0.5 * fTemp526 * fTemp533 * fTemp532 * self.fVec91[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, iTemp534))) & 1023) as usize] + 0.16666667 * fTemp527 * fTemp530 * self.fVec91[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, iTemp531))) & 1023) as usize] + 0.041666668 * fTemp528 * self.fVec91[((i32::wrapping_sub(self.IOTA0, std::cmp::min(512, iTemp522))) & 1023) as usize]));
			self.fRec359[0] = 0.995 * (self.fRec359[1] + ((i32::wrapping_mul(iTemp341, iSlow111)) as F32)) + fSlow112;
			let mut fTemp549: F32 = self.fRec359[0] + -1.49999;
			let mut fTemp550: F32 = F32::floor(fTemp549);
			let mut fTemp551: F32 = 0.760314 * self.fRec336[1] + 0.64955574 * fTemp509;
			self.fVec92[(self.IOTA0 & 16383) as usize] = fTemp551;
			let mut fTemp552: F32 = fSlow87 * self.fRec307[0];
			let mut fTemp553: F32 = fSlow82 + fTemp552 + 3.500005;
			let mut iTemp554: i32 = ((fTemp553) as i32);
			let mut fTemp555: F32 = F32::floor(fTemp553);
			let mut fTemp556: F32 = fSlow82 + fTemp552 + (2.0 - fTemp555);
			let mut fTemp557: F32 = fSlow82 + fTemp552 + (3.0 - fTemp555);
			let mut fTemp558: F32 = fSlow82 + fTemp552 + (4.0 - fTemp555);
			let mut fTemp559: F32 = fTemp558 * fTemp557;
			let mut fTemp560: F32 = fSlow82 + fTemp552 + (1.0 - fTemp555);
			self.fVec93[(self.IOTA0 & 16383) as usize] = self.fVec92[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, iTemp554)))) & 16383) as usize] * (0.0 - fTemp558) * (0.0 - 0.5 * fTemp557) * (0.0 - 0.33333334 * fTemp556) * (0.0 - 0.25 * fTemp560) + (fSlow82 + fTemp552 + (5.0 - fTemp555)) * (self.fVec92[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, i32::wrapping_add(iTemp554, 1))))) & 16383) as usize] * (0.0 - fTemp557) * (0.0 - 0.5 * fTemp556) * (0.0 - 0.33333334 * fTemp560) + 0.5 * fTemp558 * self.fVec92[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, i32::wrapping_add(iTemp554, 2))))) & 16383) as usize] * (0.0 - fTemp556) * (0.0 - 0.5 * fTemp560) + 0.16666667 * fTemp559 * self.fVec92[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, i32::wrapping_add(iTemp554, 3))))) & 16383) as usize] * (0.0 - fTemp560) + 0.041666668 * fTemp559 * fTemp556 * self.fVec92[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, i32::wrapping_add(iTemp554, 4))))) & 16383) as usize]);
			let mut fTemp561: F32 = self.fVec93[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp549) as i32))))) & 16383) as usize];
			self.fVec94[0] = fTemp561;
			self.fRec358[0] = self.fVec94[1] - (fTemp550 + (2.0 - self.fRec359[0])) * (self.fRec358[1] - fTemp561) / (self.fRec359[0] - fTemp550);
			self.fRec357[0] = 0.0 - self.fConst39 * (self.fConst37 * self.fRec357[1] - (self.fRec358[0] + self.fRec358[1]));
			self.fRec356[0] = self.fRec357[0] - self.fConst36 * (self.fConst35 * self.fRec356[2] + self.fConst31 * self.fRec356[1]);
			self.fRec355[0] = self.fConst36 * (self.fRec356[2] + self.fRec356[0] + 2.0 * self.fRec356[1]) - self.fConst34 * (self.fConst33 * self.fRec355[2] + self.fConst31 * self.fRec355[1]);
			let mut fTemp562: F32 = self.fRec355[2] + self.fRec355[0] + 2.0 * self.fRec355[1];
			self.fVec95[0] = fTemp562;
			self.fRec354[0] = self.fConst42 * (self.fConst34 * (fTemp562 + self.fVec95[1]) - self.fConst40 * self.fRec354[1]);
			self.fRec353[0] = self.fRec354[0] - self.fConst27 * (self.fConst26 * self.fRec353[2] + self.fConst22 * self.fRec353[1]);
			self.fRec352[0] = self.fConst27 * (self.fRec353[2] + self.fRec353[0] + 2.0 * self.fRec353[1]) - self.fConst25 * (self.fConst24 * self.fRec352[2] + self.fConst22 * self.fRec352[1]);
			self.fRec362[0] = self.fConst34 * (self.fConst45 * self.fVec95[1] + self.fConst44 * fTemp562) - self.fConst43 * self.fRec362[1];
			self.fRec361[0] = self.fRec362[0] - self.fConst27 * (self.fConst26 * self.fRec361[2] + self.fConst22 * self.fRec361[1]);
			self.fRec360[0] = self.fConst27 * (self.fConst46 * self.fRec361[1] + self.fConst21 * self.fRec361[0] + self.fConst21 * self.fRec361[2]) - self.fConst25 * (self.fConst24 * self.fRec360[2] + self.fConst22 * self.fRec360[1]);
			let mut fTemp563: F32 = self.fConst22 * self.fRec363[1];
			self.fRec366[0] = self.fConst49 * self.fRec358[1] - self.fConst39 * (self.fConst37 * self.fRec366[1] - self.fConst32 * self.fRec358[0]);
			self.fRec365[0] = self.fRec366[0] - self.fConst36 * (self.fConst35 * self.fRec365[2] + self.fConst31 * self.fRec365[1]);
			self.fRec364[0] = self.fConst36 * (self.fConst30 * self.fRec365[0] + self.fConst50 * self.fRec365[1] + self.fConst30 * self.fRec365[2]) - self.fConst34 * (self.fConst33 * self.fRec364[2] + self.fConst31 * self.fRec364[1]);
			self.fRec363[0] = self.fConst34 * (self.fConst50 * self.fRec364[1] + self.fConst30 * self.fRec364[0] + self.fConst30 * self.fRec364[2]) - self.fConst48 * (self.fConst47 * self.fRec363[2] + fTemp563);
			let mut fTemp564: F32 = fTemp340 + fSlow106 * (self.fRec363[2] + self.fConst48 * (fTemp563 + self.fConst47 * self.fRec363[0]) + self.fConst25 * (self.fConst21 * self.fRec360[0] + self.fConst46 * self.fRec360[1] + self.fConst21 * self.fRec360[2] + self.fRec352[2] + self.fRec352[0] + 2.0 * self.fRec352[1]));
			self.fVec96[(self.IOTA0 & 1023) as usize] = fTemp564;
			self.fRec351[0] = fSlow107 * self.fRec351[1] + fSlow108 * (fTemp433 * fTemp432 * fTemp431 * fTemp430 * self.fVec96[((i32::wrapping_sub(self.IOTA0, iTemp434)) & 1023) as usize] + fTemp429 * (fTemp427 * fTemp426 * fTemp425 * self.fVec96[((i32::wrapping_sub(self.IOTA0, iTemp428)) & 1023) as usize] + 0.5 * fTemp416 * fTemp423 * fTemp422 * self.fVec96[((i32::wrapping_sub(self.IOTA0, iTemp424)) & 1023) as usize] + 0.16666667 * fTemp417 * fTemp420 * self.fVec96[((i32::wrapping_sub(self.IOTA0, iTemp421)) & 1023) as usize] + 0.041666668 * fTemp418 * self.fVec96[((i32::wrapping_sub(self.IOTA0, iTemp412)) & 1023) as usize]));
			let mut fTemp565: F32 = fSlow113 * self.fRec351[0] - fSlow110 * self.fRec349[1];
			let mut fTemp566: F32 = fSlow113 * self.fRec266[0] - fSlow110 * self.fRec348[1];
			self.fVec97[(self.IOTA0 & 16383) as usize] = fTemp566 - fTemp565;
			let mut fTemp567: F32 = self.fVec97[((i32::wrapping_sub(self.IOTA0, iTemp472)) & 16383) as usize];
			self.fVec98[0] = fTemp567;
			self.fRec350[0] = 0.70710677 * (fTemp471 * fTemp567 / fTemp470 + self.fVec98[1]) - self.fRec350[1] * fTemp471 / fTemp470;
			self.fRec348[0] = self.fRec350[0];
			self.fRec368[0] = 0.9999 * (self.fRec368[1] + ((i32::wrapping_mul(iTemp341, iSlow114)) as F32)) + fSlow115;
			let mut fTemp568: F32 = self.fRec368[0] + -1.49999;
			let mut fTemp569: F32 = F32::floor(fTemp568);
			let mut fTemp570: F32 = self.fRec368[0] - fTemp569;
			let mut fTemp571: F32 = fTemp569 + (2.0 - self.fRec368[0]);
			self.fVec99[(self.IOTA0 & 16383) as usize] = fTemp566 + fTemp565;
			let mut fTemp572: F32 = self.fVec99[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp568) as i32))))) & 16383) as usize];
			self.fVec100[0] = fTemp572;
			self.fRec367[0] = 0.70710677 * (fTemp571 * fTemp572 / fTemp570 + self.fVec100[1]) - self.fRec367[1] * fTemp571 / fTemp570;
			self.fRec349[0] = self.fRec367[0];
			let mut fTemp573: F32 = fSlow113 * self.fRec348[1] + fSlow110 * self.fRec266[0];
			let mut fTemp574: F32 = fSlow113 * self.fRec349[1] + fSlow110 * self.fRec351[0];
			let mut fTemp575: F32 = fSlow113 * fTemp574 - fSlow110 * self.fRec370[1];
			let mut fTemp576: F32 = fSlow113 * fTemp573 - fSlow110 * self.fRec369[1];
			self.fVec101[(self.IOTA0 & 16383) as usize] = fTemp576 - fTemp575;
			let mut fTemp577: F32 = self.fVec101[((i32::wrapping_sub(self.IOTA0, iTemp497)) & 16383) as usize];
			self.fVec102[0] = fTemp577;
			self.fRec371[0] = 0.70710677 * (fTemp493 * fTemp577 / fTemp492 + self.fVec102[1]) - self.fRec371[1] * fTemp493 / fTemp492;
			self.fRec369[0] = self.fRec371[0];
			self.fVec103[(self.IOTA0 & 16383) as usize] = fTemp576 + fTemp575;
			let mut fTemp578: F32 = self.fVec103[((i32::wrapping_sub(self.IOTA0, iTemp487)) & 16383) as usize];
			self.fVec104[0] = fTemp578;
			self.fRec372[0] = 0.70710677 * (fTemp486 * fTemp578 / fTemp485 + self.fVec104[1]) - fTemp486 * self.fRec372[1] / fTemp485;
			self.fRec370[0] = self.fRec372[0];
			let mut fTemp579: F32 = fSlow113 * self.fRec369[1] + fSlow110 * fTemp573;
			let mut fTemp580: F32 = fSlow113 * self.fRec370[1] + fSlow110 * fTemp574;
			let mut fTemp581: F32 = fSlow113 * fTemp580 - fSlow110 * self.fRec374[1];
			let mut fTemp582: F32 = fSlow113 * fTemp579 - fSlow110 * self.fRec373[1];
			self.fVec105[(self.IOTA0 & 16383) as usize] = fTemp582 - fTemp581;
			let mut fTemp583: F32 = self.fVec105[((i32::wrapping_sub(self.IOTA0, iTemp503)) & 16383) as usize];
			self.fVec106[0] = fTemp583;
			self.fRec375[0] = 0.70710677 * (fTemp502 * fTemp583 / fTemp501 + self.fVec106[1]) - self.fRec375[1] * fTemp502 / fTemp501;
			self.fRec373[0] = self.fRec375[0];
			self.fRec377[0] = 0.9999 * (self.fRec377[1] + ((i32::wrapping_mul(iTemp341, iSlow116)) as F32)) + fSlow117;
			let mut fTemp584: F32 = self.fRec377[0] + -1.49999;
			let mut fTemp585: F32 = F32::floor(fTemp584);
			let mut fTemp586: F32 = self.fRec377[0] - fTemp585;
			let mut fTemp587: F32 = fTemp585 + (2.0 - self.fRec377[0]);
			self.fVec107[(self.IOTA0 & 16383) as usize] = fTemp582 + fTemp581;
			let mut fTemp588: F32 = self.fVec107[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp584) as i32))))) & 16383) as usize];
			self.fVec108[0] = fTemp588;
			self.fRec376[0] = 0.70710677 * (fTemp587 * fTemp588 / fTemp586 + self.fVec108[1]) - self.fRec376[1] * fTemp587 / fTemp586;
			self.fRec374[0] = self.fRec376[0];
			let mut fTemp589: F32 = fSlow113 * self.fRec373[1] + fSlow110 * fTemp579;
			self.fRec381[0] = 0.9999 * (self.fRec381[1] + ((i32::wrapping_mul(iTemp341, iSlow118)) as F32)) + fSlow119;
			let mut fTemp590: F32 = self.fRec381[0] + -1.49999;
			let mut fTemp591: F32 = F32::floor(fTemp590);
			let mut fTemp592: F32 = self.fRec381[0] - fTemp591;
			let mut fTemp593: F32 = fTemp591 + (2.0 - self.fRec381[0]);
			let mut fTemp594: F32 = fSlow113 * self.fRec374[1] + fSlow110 * fTemp580;
			let mut fTemp595: F32 = fSlow113 * fTemp594 - fSlow110 * self.fRec379[1];
			let mut fTemp596: F32 = fSlow113 * fTemp589 - fSlow110 * self.fRec378[1];
			self.fVec109[(self.IOTA0 & 16383) as usize] = fTemp596 - fTemp595;
			let mut fTemp597: F32 = self.fVec109[((i32::wrapping_sub(self.IOTA0, std::cmp::min(8192, std::cmp::max(0, ((fTemp590) as i32))))) & 16383) as usize];
			self.fVec110[0] = fTemp597;
			self.fRec380[0] = 0.70710677 * (fTemp593 * fTemp597 / fTemp592 + self.fVec110[1]) - self.fRec380[1] * fTemp593 / fTemp592;
			self.fRec378[0] = self.fRec380[0];
			self.fVec111[(self.IOTA0 & 16383) as usize] = fTemp596 + fTemp595;
			let mut fTemp598: F32 = self.fVec111[((i32::wrapping_sub(self.IOTA0, iTemp516)) & 16383) as usize];
			self.fVec112[0] = fTemp598;
			self.fRec382[0] = 0.70710677 * (fTemp515 * fTemp598 / fTemp514 + self.fVec112[1]) - fTemp515 * self.fRec382[1] / fTemp514;
			self.fRec379[0] = self.fRec382[0];
			self.fRec264[0] = fSlow113 * self.fRec378[1] + fSlow110 * fTemp589;
			self.fRec265[0] = fSlow113 * self.fRec379[1] + fSlow110 * fTemp594;
			self.fRec383[0] = fSlow120 + self.fConst2 * self.fRec383[1];
			let mut fTemp599: F32 = self.fRec383[0] * (fSlow52 * (self.fRec264[0] + self.fRec265[0]) + fSlow53 * fTemp340);
			*output0 = fTemp599;
			*output1 = fTemp599;
			self.iVec0[1] = self.iVec0[0];
			self.fRec1[1] = self.fRec1[0];
			self.fRec3[1] = self.fRec3[0];
			self.fVec1[1] = self.fVec1[0];
			self.IOTA0 = i32::wrapping_add(self.IOTA0, 1);
			self.fRec0[1] = self.fRec0[0];
			self.fRec5[1] = self.fRec5[0];
			self.fRec6[1] = self.fRec6[0];
			self.fVec3[1] = self.fVec3[0];
			self.fRec4[1] = self.fRec4[0];
			self.fRec8[1] = self.fRec8[0];
			self.fRec9[1] = self.fRec9[0];
			self.fVec5[1] = self.fVec5[0];
			self.fRec7[1] = self.fRec7[0];
			self.fRec10[1] = self.fRec10[0];
			self.fRec12[1] = self.fRec12[0];
			self.fRec13[1] = self.fRec13[0];
			self.fVec7[1] = self.fVec7[0];
			self.fRec11[1] = self.fRec11[0];
			self.fRec15[1] = self.fRec15[0];
			self.fRec16[1] = self.fRec16[0];
			self.fVec9[1] = self.fVec9[0];
			self.fRec14[1] = self.fRec14[0];
			self.fRec18[1] = self.fRec18[0];
			self.fRec19[1] = self.fRec19[0];
			self.fVec11[1] = self.fVec11[0];
			self.fRec17[1] = self.fRec17[0];
			self.fRec20[1] = self.fRec20[0];
			self.fRec22[1] = self.fRec22[0];
			self.fRec23[1] = self.fRec23[0];
			self.fVec13[1] = self.fVec13[0];
			self.fRec21[1] = self.fRec21[0];
			self.fRec25[1] = self.fRec25[0];
			self.fRec26[1] = self.fRec26[0];
			self.fVec15[1] = self.fVec15[0];
			self.fRec24[1] = self.fRec24[0];
			self.fRec28[1] = self.fRec28[0];
			self.fRec29[1] = self.fRec29[0];
			self.fVec17[1] = self.fVec17[0];
			self.fRec27[1] = self.fRec27[0];
			self.fRec30[1] = self.fRec30[0];
			self.fRec32[1] = self.fRec32[0];
			self.fRec33[1] = self.fRec33[0];
			self.fVec19[1] = self.fVec19[0];
			self.fRec31[1] = self.fRec31[0];
			self.fRec35[1] = self.fRec35[0];
			self.fRec36[1] = self.fRec36[0];
			self.fVec21[1] = self.fVec21[0];
			self.fRec34[1] = self.fRec34[0];
			self.fRec38[1] = self.fRec38[0];
			self.fRec39[1] = self.fRec39[0];
			self.fVec23[1] = self.fVec23[0];
			self.fRec37[1] = self.fRec37[0];
			self.fRec40[1] = self.fRec40[0];
			self.fRec41[1] = self.fRec41[0];
			self.fRec42[1] = self.fRec42[0];
			self.fRec45[1] = self.fRec45[0];
			self.fRec44[1] = self.fRec44[0];
			self.fRec75[1] = self.fRec75[0];
			self.fRec79[1] = self.fRec79[0];
			self.fRec84[1] = self.fRec84[0];
			self.iVec25[1] = self.iVec25[0];
			self.iRec85[1] = self.iRec85[0];
			self.fRec82[1] = self.fRec82[0];
			self.fRec81[1] = self.fRec81[0];
			for j0 in (1..=3).rev() {
				self.fRec86[(j0) as usize] = self.fRec86[(i32::wrapping_sub(j0, 1)) as usize];
			}
			self.fVec26[1] = self.fVec26[0];
			self.fVec27[1] = self.fVec27[0];
			self.iRec88[1] = self.iRec88[0];
			self.iRec90[1] = self.iRec90[0];
			self.fRec89[2] = self.fRec89[1];
			self.fRec89[1] = self.fRec89[0];
			self.fVec28[2] = self.fVec28[1];
			self.fVec28[1] = self.fVec28[0];
			self.fRec71[1] = self.fRec71[0];
			self.fRec67[1] = self.fRec67[0];
			self.fRec65[1] = self.fRec65[0];
			for j1 in (1..=3).rev() {
				self.fRec61[(j1) as usize] = self.fRec61[(i32::wrapping_sub(j1, 1)) as usize];
			}
			self.fRec56[1] = self.fRec56[0];
			self.fRec50[1] = self.fRec50[0];
			self.fRec49[1] = self.fRec49[0];
			self.fRec48[1] = self.fRec48[0];
			self.fRec46[1] = self.fRec46[0];
			self.fRec43[1] = self.fRec43[0];
			self.fRec92[1] = self.fRec92[0];
			self.fRec122[1] = self.fRec122[0];
			self.fRec126[1] = self.fRec126[0];
			self.fRec131[1] = self.fRec131[0];
			self.iVec29[1] = self.iVec29[0];
			self.iRec132[1] = self.iRec132[0];
			self.fRec129[1] = self.fRec129[0];
			self.fRec128[1] = self.fRec128[0];
			for j2 in (1..=3).rev() {
				self.fRec133[(j2) as usize] = self.fRec133[(i32::wrapping_sub(j2, 1)) as usize];
			}
			self.fVec30[1] = self.fVec30[0];
			self.fVec31[1] = self.fVec31[0];
			self.iRec135[1] = self.iRec135[0];
			self.fRec136[2] = self.fRec136[1];
			self.fRec136[1] = self.fRec136[0];
			self.fVec32[2] = self.fVec32[1];
			self.fVec32[1] = self.fVec32[0];
			self.fRec118[1] = self.fRec118[0];
			self.fRec114[1] = self.fRec114[0];
			self.fRec112[1] = self.fRec112[0];
			for j3 in (1..=3).rev() {
				self.fRec108[(j3) as usize] = self.fRec108[(i32::wrapping_sub(j3, 1)) as usize];
			}
			self.fRec103[1] = self.fRec103[0];
			self.fRec97[1] = self.fRec97[0];
			self.fRec96[1] = self.fRec96[0];
			self.fRec95[1] = self.fRec95[0];
			self.fRec93[1] = self.fRec93[0];
			self.fRec91[1] = self.fRec91[0];
			self.fRec138[1] = self.fRec138[0];
			self.fRec168[1] = self.fRec168[0];
			self.fRec172[1] = self.fRec172[0];
			self.fRec177[1] = self.fRec177[0];
			self.iVec33[1] = self.iVec33[0];
			self.iRec178[1] = self.iRec178[0];
			self.fRec175[1] = self.fRec175[0];
			self.fRec174[1] = self.fRec174[0];
			for j4 in (1..=3).rev() {
				self.fRec179[(j4) as usize] = self.fRec179[(i32::wrapping_sub(j4, 1)) as usize];
			}
			self.fVec34[1] = self.fVec34[0];
			self.fVec35[1] = self.fVec35[0];
			self.iRec181[1] = self.iRec181[0];
			self.fRec182[2] = self.fRec182[1];
			self.fRec182[1] = self.fRec182[0];
			self.fVec36[2] = self.fVec36[1];
			self.fVec36[1] = self.fVec36[0];
			self.fRec164[1] = self.fRec164[0];
			self.fRec160[1] = self.fRec160[0];
			self.fRec158[1] = self.fRec158[0];
			for j5 in (1..=3).rev() {
				self.fRec154[(j5) as usize] = self.fRec154[(i32::wrapping_sub(j5, 1)) as usize];
			}
			self.fRec149[1] = self.fRec149[0];
			self.fRec143[1] = self.fRec143[0];
			self.fRec142[1] = self.fRec142[0];
			self.fRec141[1] = self.fRec141[0];
			self.fRec139[1] = self.fRec139[0];
			self.fRec137[1] = self.fRec137[0];
			self.fRec184[1] = self.fRec184[0];
			self.fRec214[1] = self.fRec214[0];
			self.fRec218[1] = self.fRec218[0];
			self.fRec223[1] = self.fRec223[0];
			self.iVec37[1] = self.iVec37[0];
			self.iRec224[1] = self.iRec224[0];
			self.fRec221[1] = self.fRec221[0];
			self.fRec220[1] = self.fRec220[0];
			for j6 in (1..=3).rev() {
				self.fRec225[(j6) as usize] = self.fRec225[(i32::wrapping_sub(j6, 1)) as usize];
			}
			self.fVec38[1] = self.fVec38[0];
			self.fVec39[1] = self.fVec39[0];
			self.iRec227[1] = self.iRec227[0];
			self.fRec228[2] = self.fRec228[1];
			self.fRec228[1] = self.fRec228[0];
			self.fVec40[2] = self.fVec40[1];
			self.fVec40[1] = self.fVec40[0];
			self.fRec210[1] = self.fRec210[0];
			self.fRec206[1] = self.fRec206[0];
			self.fRec204[1] = self.fRec204[0];
			for j7 in (1..=3).rev() {
				self.fRec200[(j7) as usize] = self.fRec200[(i32::wrapping_sub(j7, 1)) as usize];
			}
			self.fRec195[1] = self.fRec195[0];
			self.fRec189[1] = self.fRec189[0];
			self.fRec188[1] = self.fRec188[0];
			self.fRec187[1] = self.fRec187[0];
			self.fRec185[1] = self.fRec185[0];
			self.fRec183[1] = self.fRec183[0];
			self.fRec229[1] = self.fRec229[0];
			self.fRec231[1] = self.fRec231[0];
			self.fRec230[1] = self.fRec230[0];
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
			self.fRec274[1] = self.fRec274[0];
			self.fRec276[1] = self.fRec276[0];
			self.fRec280[1] = self.fRec280[0];
			self.fVec42[1] = self.fVec42[0];
			self.fRec279[1] = self.fRec279[0];
			self.fRec277[1] = self.fRec277[0];
			self.fRec282[1] = self.fRec282[0];
			self.fVec44[1] = self.fVec44[0];
			self.fRec281[1] = self.fRec281[0];
			self.fRec278[1] = self.fRec278[0];
			self.fRec286[1] = self.fRec286[0];
			self.fVec46[1] = self.fVec46[0];
			self.fRec285[1] = self.fRec285[0];
			self.fRec283[1] = self.fRec283[0];
			self.fRec288[1] = self.fRec288[0];
			self.fVec48[1] = self.fVec48[0];
			self.fRec287[1] = self.fRec287[0];
			self.fRec284[1] = self.fRec284[0];
			self.fRec292[1] = self.fRec292[0];
			self.fVec50[1] = self.fVec50[0];
			self.fRec291[1] = self.fRec291[0];
			self.fRec289[1] = self.fRec289[0];
			self.fRec294[1] = self.fRec294[0];
			self.fVec52[1] = self.fVec52[0];
			self.fRec293[1] = self.fRec293[0];
			self.fRec290[1] = self.fRec290[0];
			self.fRec298[1] = self.fRec298[0];
			self.fVec54[1] = self.fVec54[0];
			self.fRec297[1] = self.fRec297[0];
			self.fRec295[1] = self.fRec295[0];
			self.fRec300[1] = self.fRec300[0];
			self.fVec56[1] = self.fVec56[0];
			self.fRec299[1] = self.fRec299[0];
			self.fRec296[1] = self.fRec296[0];
			self.fRec304[1] = self.fRec304[0];
			self.fVec58[1] = self.fVec58[0];
			self.fRec303[1] = self.fRec303[0];
			self.fRec301[1] = self.fRec301[0];
			self.fRec306[1] = self.fRec306[0];
			self.fVec60[1] = self.fVec60[0];
			self.fRec305[1] = self.fRec305[0];
			self.fRec302[1] = self.fRec302[0];
			self.fRec307[1] = self.fRec307[0];
			self.fRec308[1] = self.fRec308[0];
			self.fVec63[1] = self.fVec63[0];
			self.fRec275[1] = self.fRec275[0];
			self.fRec312[1] = self.fRec312[0];
			self.fRec314[1] = self.fRec314[0];
			self.fVec66[1] = self.fVec66[0];
			self.fRec313[1] = self.fRec313[0];
			self.fVec68[1] = self.fVec68[0];
			self.fRec311[1] = self.fRec311[0];
			self.fRec309[1] = self.fRec309[0];
			self.fRec316[1] = self.fRec316[0];
			self.fVec70[1] = self.fVec70[0];
			self.fRec315[1] = self.fRec315[0];
			self.fRec310[1] = self.fRec310[0];
			self.fRec320[1] = self.fRec320[0];
			self.fVec72[1] = self.fVec72[0];
			self.fRec319[1] = self.fRec319[0];
			self.fRec317[1] = self.fRec317[0];
			self.fRec322[1] = self.fRec322[0];
			self.fVec74[1] = self.fVec74[0];
			self.fRec321[1] = self.fRec321[0];
			self.fRec318[1] = self.fRec318[0];
			self.fRec326[1] = self.fRec326[0];
			self.fVec76[1] = self.fVec76[0];
			self.fRec325[1] = self.fRec325[0];
			self.fRec323[1] = self.fRec323[0];
			self.fRec328[1] = self.fRec328[0];
			self.fVec78[1] = self.fVec78[0];
			self.fRec327[1] = self.fRec327[0];
			self.fRec324[1] = self.fRec324[0];
			self.fRec332[1] = self.fRec332[0];
			self.fVec80[1] = self.fVec80[0];
			self.fRec331[1] = self.fRec331[0];
			self.fRec329[1] = self.fRec329[0];
			self.fRec334[1] = self.fRec334[0];
			self.fVec82[1] = self.fVec82[0];
			self.fRec333[1] = self.fRec333[0];
			self.fRec330[1] = self.fRec330[0];
			self.fRec338[1] = self.fRec338[0];
			self.fVec84[1] = self.fVec84[0];
			self.fRec337[1] = self.fRec337[0];
			self.fRec335[1] = self.fRec335[0];
			self.fRec340[1] = self.fRec340[0];
			self.fVec86[1] = self.fVec86[0];
			self.fRec339[1] = self.fRec339[0];
			self.fRec336[1] = self.fRec336[0];
			self.fVec89[1] = self.fVec89[0];
			self.fRec273[1] = self.fRec273[0];
			self.fRec272[1] = self.fRec272[0];
			self.fRec271[2] = self.fRec271[1];
			self.fRec271[1] = self.fRec271[0];
			self.fRec270[2] = self.fRec270[1];
			self.fRec270[1] = self.fRec270[0];
			self.fVec90[1] = self.fVec90[0];
			self.fRec269[1] = self.fRec269[0];
			self.fRec268[2] = self.fRec268[1];
			self.fRec268[1] = self.fRec268[0];
			self.fRec267[2] = self.fRec267[1];
			self.fRec267[1] = self.fRec267[0];
			self.fRec343[1] = self.fRec343[0];
			self.fRec342[2] = self.fRec342[1];
			self.fRec342[1] = self.fRec342[0];
			self.fRec341[2] = self.fRec341[1];
			self.fRec341[1] = self.fRec341[0];
			self.fRec347[1] = self.fRec347[0];
			self.fRec346[2] = self.fRec346[1];
			self.fRec346[1] = self.fRec346[0];
			self.fRec345[2] = self.fRec345[1];
			self.fRec345[1] = self.fRec345[0];
			self.fRec344[2] = self.fRec344[1];
			self.fRec344[1] = self.fRec344[0];
			self.fRec266[1] = self.fRec266[0];
			self.fRec359[1] = self.fRec359[0];
			self.fVec94[1] = self.fVec94[0];
			self.fRec358[1] = self.fRec358[0];
			self.fRec357[1] = self.fRec357[0];
			self.fRec356[2] = self.fRec356[1];
			self.fRec356[1] = self.fRec356[0];
			self.fRec355[2] = self.fRec355[1];
			self.fRec355[1] = self.fRec355[0];
			self.fVec95[1] = self.fVec95[0];
			self.fRec354[1] = self.fRec354[0];
			self.fRec353[2] = self.fRec353[1];
			self.fRec353[1] = self.fRec353[0];
			self.fRec352[2] = self.fRec352[1];
			self.fRec352[1] = self.fRec352[0];
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
			self.fRec351[1] = self.fRec351[0];
			self.fVec98[1] = self.fVec98[0];
			self.fRec350[1] = self.fRec350[0];
			self.fRec348[1] = self.fRec348[0];
			self.fRec368[1] = self.fRec368[0];
			self.fVec100[1] = self.fVec100[0];
			self.fRec367[1] = self.fRec367[0];
			self.fRec349[1] = self.fRec349[0];
			self.fVec102[1] = self.fVec102[0];
			self.fRec371[1] = self.fRec371[0];
			self.fRec369[1] = self.fRec369[0];
			self.fVec104[1] = self.fVec104[0];
			self.fRec372[1] = self.fRec372[0];
			self.fRec370[1] = self.fRec370[0];
			self.fVec106[1] = self.fVec106[0];
			self.fRec375[1] = self.fRec375[0];
			self.fRec373[1] = self.fRec373[0];
			self.fRec377[1] = self.fRec377[0];
			self.fVec108[1] = self.fVec108[0];
			self.fRec376[1] = self.fRec376[0];
			self.fRec374[1] = self.fRec374[0];
			self.fRec381[1] = self.fRec381[0];
			self.fVec110[1] = self.fVec110[0];
			self.fRec380[1] = self.fRec380[0];
			self.fRec378[1] = self.fRec378[0];
			self.fVec112[1] = self.fVec112[0];
			self.fRec382[1] = self.fRec382[0];
			self.fRec379[1] = self.fRec379[0];
			self.fRec264[1] = self.fRec264[0];
			self.fRec265[1] = self.fRec265[0];
			self.fRec383[1] = self.fRec383[0];
		}
	}

}


}
pub use dsp::mydsp as Instrument;
