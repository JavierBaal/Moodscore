/* ------------------------------------------------------------
license: "MIT OR Apache-2.0"
name: "moodscore"
version: "0.1.0"
Code generated with Faust 2.88.0 (https://faust.grame.fr)
Compilation options: -lang rust -fpga-mem-th 4 -ct 1 -cn MoodscoreDsp -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0
------------------------------------------------------------ */

#[repr(C)]
pub struct MoodscoreDsp {
	iVec0: [i32;2],
	fSampleRate: i32,
	fConst0: F32,
	fConst1: F32,
	fConst2: F32,
	fHslider0: FaustFloat,
	fRec0: [F32;2],
	fHslider1: FaustFloat,
	fRec5: [F32;2],
	fConst3: F32,
	fHslider2: FaustFloat,
	fRec6: [F32;2],
	fHslider3: FaustFloat,
	fRec11: [F32;2],
	fHslider4: FaustFloat,
	fRec12: [F32;2],
	fHslider5: FaustFloat,
	fRec13: [F32;2],
	fHslider6: FaustFloat,
	fRec14: [F32;2],
	fHslider7: FaustFloat,
	fRec15: [F32;2],
	fHslider8: FaustFloat,
	fRec16: [F32;2],
	fHslider9: FaustFloat,
	fRec17: [F32;2],
	fConst4: F32,
	fConst5: F32,
	iRec19: [i32;2],
	fVec1: [F32;2],
	fConst6: F32,
	fRec18: [F32;2],
	fHslider10: FaustFloat,
	fRec20: [F32;2],
	fHslider11: FaustFloat,
	fRec21: [F32;2],
	fConst7: F32,
	fRec22: [F32;2],
	fVec2: [F32;2],
	fRec23: [F32;2],
	fRec24: [F32;2],
	fHslider12: FaustFloat,
	fRec25: [F32;2],
	fVec3: [F32;2],
	fRec26: [F32;2],
	fHslider13: FaustFloat,
	fRec27: [F32;2],
	fHslider14: FaustFloat,
	fRec28: [F32;2],
	fRec29: [F32;2],
	fVec4: [F32;2],
	fRec30: [F32;2],
	fRec31: [F32;2],
	fHslider15: FaustFloat,
	fRec32: [F32;2],
	fVec5: [F32;2],
	fRec33: [F32;2],
	fHslider16: FaustFloat,
	fRec34: [F32;2],
	fHslider17: FaustFloat,
	fRec35: [F32;2],
	fRec36: [F32;2],
	fVec6: [F32;2],
	fRec37: [F32;2],
	fRec38: [F32;2],
	fHslider18: FaustFloat,
	fRec39: [F32;2],
	fVec7: [F32;2],
	fRec40: [F32;2],
	fHslider19: FaustFloat,
	fRec41: [F32;2],
	fHslider20: FaustFloat,
	fRec42: [F32;2],
	fRec43: [F32;2],
	fVec8: [F32;2],
	fRec44: [F32;2],
	fRec45: [F32;2],
	fHslider21: FaustFloat,
	fRec46: [F32;2],
	fVec9: [F32;2],
	fRec47: [F32;2],
	fHslider22: FaustFloat,
	fRec48: [F32;2],
	fHslider23: FaustFloat,
	fRec49: [F32;2],
	fRec50: [F32;2],
	fVec10: [F32;2],
	fRec51: [F32;2],
	fRec52: [F32;2],
	fHslider24: FaustFloat,
	fRec53: [F32;2],
	fVec11: [F32;2],
	fRec54: [F32;2],
	fHslider25: FaustFloat,
	fRec55: [F32;2],
	fHslider26: FaustFloat,
	fRec56: [F32;2],
	fRec57: [F32;2],
	fVec12: [F32;2],
	fRec58: [F32;2],
	fRec59: [F32;2],
	fHslider27: FaustFloat,
	fRec60: [F32;2],
	fVec13: [F32;2],
	fRec63: [F32;2],
	fRec61: [F32;2],
	fRec62: [F32;2],
	fRec9: [F32;2],
	fRec10: [F32;2],
	fRec7: [F32;2],
	fRec8: [F32;2],
	fRec66: [F32;2],
	fRec67: [F32;2],
	fRec64: [F32;2],
	fRec65: [F32;2],
	fHslider28: FaustFloat,
	fRec68: [F32;2],
	fConst8: F32,
	fConst9: F32,
	fConst10: F32,
	fRec69: [F32;2],
	fRec70: [F32;2],
	fRec71: [F32;2],
	fRec72: [F32;2],
	fRec73: [F32;2],
	fRec74: [F32;2],
	fRec75: [F32;2],
	fRec76: [F32;2],
	fConst11: F32,
	fConst12: F32,
	fConst13: F32,
	fConst14: F32,
	fConst15: F32,
	fConst16: F32,
	fConst17: F32,
	fConst18: F32,
	fConst19: F32,
	fConst20: F32,
	fConst21: F32,
	fConst22: F32,
	fRec77: [F32;2],
	fConst23: F32,
	fConst24: F32,
	fConst25: F32,
	fConst26: F32,
	fConst27: F32,
	fConst28: F32,
	fConst29: F32,
	fConst30: F32,
	fConst31: F32,
	fConst32: F32,
	fConst33: F32,
	fRec78: [F32;2],
	IOTA0: i32,
	fVec14: [F32;16384],
	fConst34: F32,
	iConst35: i32,
	fVec15: [F32;2048],
	fConst36: F32,
	fConst37: F32,
	fConst38: F32,
	fConst39: F32,
	fConst40: F32,
	fConst41: F32,
	fConst42: F32,
	fConst43: F32,
	fConst44: F32,
	fConst45: F32,
	fConst46: F32,
	fRec80: [F32;2],
	fConst47: F32,
	fConst48: F32,
	fConst49: F32,
	fConst50: F32,
	fConst51: F32,
	fConst52: F32,
	fConst53: F32,
	fConst54: F32,
	fConst55: F32,
	fConst56: F32,
	fConst57: F32,
	fRec81: [F32;2],
	fVec16: [F32;16384],
	fConst58: F32,
	iConst59: i32,
	fVec17: [F32;2048],
	fConst60: F32,
	fConst61: F32,
	fConst62: F32,
	fConst63: F32,
	fConst64: F32,
	fConst65: F32,
	fConst66: F32,
	fConst67: F32,
	fConst68: F32,
	fConst69: F32,
	fConst70: F32,
	fRec83: [F32;2],
	fConst71: F32,
	fConst72: F32,
	fConst73: F32,
	fConst74: F32,
	fConst75: F32,
	fConst76: F32,
	fConst77: F32,
	fConst78: F32,
	fConst79: F32,
	fConst80: F32,
	fConst81: F32,
	fRec84: [F32;2],
	iConst82: i32,
	fRec85: [F32;2],
	fVec18: [F32;16384],
	fConst83: F32,
	iConst84: i32,
	fVec19: [F32;4096],
	iConst85: i32,
	fRec87: [F32;2],
	iConst86: i32,
	fRec88: [F32;2],
	fConst87: F32,
	fConst88: F32,
	fConst89: F32,
	fConst90: F32,
	fConst91: F32,
	fConst92: F32,
	fConst93: F32,
	fConst94: F32,
	fConst95: F32,
	fConst96: F32,
	fConst97: F32,
	fRec89: [F32;2],
	fConst98: F32,
	fConst99: F32,
	fConst100: F32,
	fConst101: F32,
	fConst102: F32,
	fConst103: F32,
	fConst104: F32,
	fConst105: F32,
	fConst106: F32,
	fConst107: F32,
	fConst108: F32,
	fRec90: [F32;2],
	fVec20: [F32;32768],
	fConst109: F32,
	iConst110: i32,
	fVec21: [F32;2048],
	iConst111: i32,
	fRec91: [F32;2],
	fVec22: [F32;16384],
	fConst112: F32,
	iConst113: i32,
	fVec23: [F32;4096],
	iConst114: i32,
	fRec93: [F32;2],
	fVec24: [F32;16384],
	fConst115: F32,
	iConst116: i32,
	fVec25: [F32;4096],
	iConst117: i32,
	fRec95: [F32;2],
	fVec26: [F32;32768],
	fConst118: F32,
	iConst119: i32,
	fVec27: [F32;4096],
	iConst120: i32,
	fRec96: [F32;2],
	fVec28: [F32;32768],
	fConst121: F32,
	iConst122: i32,
	fVec29: [F32;4096],
	iConst123: i32,
	fRec97: [F32;2],
	fRec101: [F32;3],
	fRec102: [F32;3],
	fRec103: [F32;3],
	fRec104: [F32;3],
	fRec105: [F32;3],
	fRec106: [F32;3],
	fRec107: [F32;3],
	fRec108: [F32;3],
	fConst124: F32,
	fConst125: F32,
	fConst126: F32,
	fRec3: [F32;2],
	fConst127: F32,
	fRec4: [F32;2],
	fRec1: [F32;2],
	fRec2: [F32;2],
	fRec109: [F32;2],
	fRec110: [F32;2],
	fConst128: F32,
	fConst129: F32,
	fRec111: [F32;2],
	fRec112: [F32;2],
	fConst130: F32,
	fRec113: [F32;2],
	fRec114: [F32;2],
	fRec115: [F32;2],
	fRec116: [F32;2],
}


fn MoodscoreDsp_faustpower2_f(value: F32) -> F32 {
	return value * value;
}
pub const FAUST_INPUTS: usize = 4;
pub const FAUST_OUTPUTS: usize = 2;
pub const FAUST_ACTIVES: usize = 29;
pub const FAUST_PASSIVES: usize = 0;

impl MoodscoreDsp {
		
	pub fn new() -> MoodscoreDsp { 
		MoodscoreDsp {
			iVec0: [0;2],
			fSampleRate: 0,
			fConst0: 0.0,
			fConst1: 0.0,
			fConst2: 0.0,
			fHslider0: 0.0,
			fRec0: [0.0;2],
			fHslider1: 0.0,
			fRec5: [0.0;2],
			fConst3: 0.0,
			fHslider2: 0.0,
			fRec6: [0.0;2],
			fHslider3: 0.0,
			fRec11: [0.0;2],
			fHslider4: 0.0,
			fRec12: [0.0;2],
			fHslider5: 0.0,
			fRec13: [0.0;2],
			fHslider6: 0.0,
			fRec14: [0.0;2],
			fHslider7: 0.0,
			fRec15: [0.0;2],
			fHslider8: 0.0,
			fRec16: [0.0;2],
			fHslider9: 0.0,
			fRec17: [0.0;2],
			fConst4: 0.0,
			fConst5: 0.0,
			iRec19: [0;2],
			fVec1: [0.0;2],
			fConst6: 0.0,
			fRec18: [0.0;2],
			fHslider10: 0.0,
			fRec20: [0.0;2],
			fHslider11: 0.0,
			fRec21: [0.0;2],
			fConst7: 0.0,
			fRec22: [0.0;2],
			fVec2: [0.0;2],
			fRec23: [0.0;2],
			fRec24: [0.0;2],
			fHslider12: 0.0,
			fRec25: [0.0;2],
			fVec3: [0.0;2],
			fRec26: [0.0;2],
			fHslider13: 0.0,
			fRec27: [0.0;2],
			fHslider14: 0.0,
			fRec28: [0.0;2],
			fRec29: [0.0;2],
			fVec4: [0.0;2],
			fRec30: [0.0;2],
			fRec31: [0.0;2],
			fHslider15: 0.0,
			fRec32: [0.0;2],
			fVec5: [0.0;2],
			fRec33: [0.0;2],
			fHslider16: 0.0,
			fRec34: [0.0;2],
			fHslider17: 0.0,
			fRec35: [0.0;2],
			fRec36: [0.0;2],
			fVec6: [0.0;2],
			fRec37: [0.0;2],
			fRec38: [0.0;2],
			fHslider18: 0.0,
			fRec39: [0.0;2],
			fVec7: [0.0;2],
			fRec40: [0.0;2],
			fHslider19: 0.0,
			fRec41: [0.0;2],
			fHslider20: 0.0,
			fRec42: [0.0;2],
			fRec43: [0.0;2],
			fVec8: [0.0;2],
			fRec44: [0.0;2],
			fRec45: [0.0;2],
			fHslider21: 0.0,
			fRec46: [0.0;2],
			fVec9: [0.0;2],
			fRec47: [0.0;2],
			fHslider22: 0.0,
			fRec48: [0.0;2],
			fHslider23: 0.0,
			fRec49: [0.0;2],
			fRec50: [0.0;2],
			fVec10: [0.0;2],
			fRec51: [0.0;2],
			fRec52: [0.0;2],
			fHslider24: 0.0,
			fRec53: [0.0;2],
			fVec11: [0.0;2],
			fRec54: [0.0;2],
			fHslider25: 0.0,
			fRec55: [0.0;2],
			fHslider26: 0.0,
			fRec56: [0.0;2],
			fRec57: [0.0;2],
			fVec12: [0.0;2],
			fRec58: [0.0;2],
			fRec59: [0.0;2],
			fHslider27: 0.0,
			fRec60: [0.0;2],
			fVec13: [0.0;2],
			fRec63: [0.0;2],
			fRec61: [0.0;2],
			fRec62: [0.0;2],
			fRec9: [0.0;2],
			fRec10: [0.0;2],
			fRec7: [0.0;2],
			fRec8: [0.0;2],
			fRec66: [0.0;2],
			fRec67: [0.0;2],
			fRec64: [0.0;2],
			fRec65: [0.0;2],
			fHslider28: 0.0,
			fRec68: [0.0;2],
			fConst8: 0.0,
			fConst9: 0.0,
			fConst10: 0.0,
			fRec69: [0.0;2],
			fRec70: [0.0;2],
			fRec71: [0.0;2],
			fRec72: [0.0;2],
			fRec73: [0.0;2],
			fRec74: [0.0;2],
			fRec75: [0.0;2],
			fRec76: [0.0;2],
			fConst11: 0.0,
			fConst12: 0.0,
			fConst13: 0.0,
			fConst14: 0.0,
			fConst15: 0.0,
			fConst16: 0.0,
			fConst17: 0.0,
			fConst18: 0.0,
			fConst19: 0.0,
			fConst20: 0.0,
			fConst21: 0.0,
			fConst22: 0.0,
			fRec77: [0.0;2],
			fConst23: 0.0,
			fConst24: 0.0,
			fConst25: 0.0,
			fConst26: 0.0,
			fConst27: 0.0,
			fConst28: 0.0,
			fConst29: 0.0,
			fConst30: 0.0,
			fConst31: 0.0,
			fConst32: 0.0,
			fConst33: 0.0,
			fRec78: [0.0;2],
			IOTA0: 0,
			fVec14: [0.0;16384],
			fConst34: 0.0,
			iConst35: 0,
			fVec15: [0.0;2048],
			fConst36: 0.0,
			fConst37: 0.0,
			fConst38: 0.0,
			fConst39: 0.0,
			fConst40: 0.0,
			fConst41: 0.0,
			fConst42: 0.0,
			fConst43: 0.0,
			fConst44: 0.0,
			fConst45: 0.0,
			fConst46: 0.0,
			fRec80: [0.0;2],
			fConst47: 0.0,
			fConst48: 0.0,
			fConst49: 0.0,
			fConst50: 0.0,
			fConst51: 0.0,
			fConst52: 0.0,
			fConst53: 0.0,
			fConst54: 0.0,
			fConst55: 0.0,
			fConst56: 0.0,
			fConst57: 0.0,
			fRec81: [0.0;2],
			fVec16: [0.0;16384],
			fConst58: 0.0,
			iConst59: 0,
			fVec17: [0.0;2048],
			fConst60: 0.0,
			fConst61: 0.0,
			fConst62: 0.0,
			fConst63: 0.0,
			fConst64: 0.0,
			fConst65: 0.0,
			fConst66: 0.0,
			fConst67: 0.0,
			fConst68: 0.0,
			fConst69: 0.0,
			fConst70: 0.0,
			fRec83: [0.0;2],
			fConst71: 0.0,
			fConst72: 0.0,
			fConst73: 0.0,
			fConst74: 0.0,
			fConst75: 0.0,
			fConst76: 0.0,
			fConst77: 0.0,
			fConst78: 0.0,
			fConst79: 0.0,
			fConst80: 0.0,
			fConst81: 0.0,
			fRec84: [0.0;2],
			iConst82: 0,
			fRec85: [0.0;2],
			fVec18: [0.0;16384],
			fConst83: 0.0,
			iConst84: 0,
			fVec19: [0.0;4096],
			iConst85: 0,
			fRec87: [0.0;2],
			iConst86: 0,
			fRec88: [0.0;2],
			fConst87: 0.0,
			fConst88: 0.0,
			fConst89: 0.0,
			fConst90: 0.0,
			fConst91: 0.0,
			fConst92: 0.0,
			fConst93: 0.0,
			fConst94: 0.0,
			fConst95: 0.0,
			fConst96: 0.0,
			fConst97: 0.0,
			fRec89: [0.0;2],
			fConst98: 0.0,
			fConst99: 0.0,
			fConst100: 0.0,
			fConst101: 0.0,
			fConst102: 0.0,
			fConst103: 0.0,
			fConst104: 0.0,
			fConst105: 0.0,
			fConst106: 0.0,
			fConst107: 0.0,
			fConst108: 0.0,
			fRec90: [0.0;2],
			fVec20: [0.0;32768],
			fConst109: 0.0,
			iConst110: 0,
			fVec21: [0.0;2048],
			iConst111: 0,
			fRec91: [0.0;2],
			fVec22: [0.0;16384],
			fConst112: 0.0,
			iConst113: 0,
			fVec23: [0.0;4096],
			iConst114: 0,
			fRec93: [0.0;2],
			fVec24: [0.0;16384],
			fConst115: 0.0,
			iConst116: 0,
			fVec25: [0.0;4096],
			iConst117: 0,
			fRec95: [0.0;2],
			fVec26: [0.0;32768],
			fConst118: 0.0,
			iConst119: 0,
			fVec27: [0.0;4096],
			iConst120: 0,
			fRec96: [0.0;2],
			fVec28: [0.0;32768],
			fConst121: 0.0,
			iConst122: 0,
			fVec29: [0.0;4096],
			iConst123: 0,
			fRec97: [0.0;2],
			fRec101: [0.0;3],
			fRec102: [0.0;3],
			fRec103: [0.0;3],
			fRec104: [0.0;3],
			fRec105: [0.0;3],
			fRec106: [0.0;3],
			fRec107: [0.0;3],
			fRec108: [0.0;3],
			fConst124: 0.0,
			fConst125: 0.0,
			fConst126: 0.0,
			fRec3: [0.0;2],
			fConst127: 0.0,
			fRec4: [0.0;2],
			fRec1: [0.0;2],
			fRec2: [0.0;2],
			fRec109: [0.0;2],
			fRec110: [0.0;2],
			fConst128: 0.0,
			fConst129: 0.0,
			fRec111: [0.0;2],
			fRec112: [0.0;2],
			fConst130: 0.0,
			fRec113: [0.0;2],
			fRec114: [0.0;2],
			fRec115: [0.0;2],
			fRec116: [0.0;2],
		}
	}
	pub fn metadata(&self, m: &mut dyn Meta) { 
		m.declare("basics.lib/name", r"Faust Basic Element Library");
		m.declare("basics.lib/version", r"1.23.0");
		m.declare("compile_options", r"-lang rust -fpga-mem-th 4 -ct 1 -cn MoodscoreDsp -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0");
		m.declare("delays.lib/name", r"Faust Delay Library");
		m.declare("delays.lib/version", r"1.2.0");
		m.declare("filename", r"moodscore.dsp");
		m.declare("filters.lib/allpass_comb:author", r"Julius O. Smith III");
		m.declare("filters.lib/allpass_comb:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/allpass_comb:license", r"LicenseRef-STK-4.3");
		m.declare("filters.lib/allpassnnlt:author", r"Julius O. Smith III");
		m.declare("filters.lib/allpassnnlt:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/allpassnnlt:license", r"LicenseRef-STK-4.3");
		m.declare("filters.lib/highpassLR4:author", r"Dario Sanfilippo");
		m.declare("filters.lib/highpassLR4:copyright", r"Copyright (C) 2022 Dario Sanfilippo <sanfilippo.dario@gmail.com>");
		m.declare("filters.lib/highpassLR4:license", r"LicenseRef-STK-4.3");
		m.declare("filters.lib/lowpass0_highpass1:author", r"Julius O. Smith III");
		m.declare("filters.lib/lowpass0_highpass1:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/lowpass0_highpass1:license", r"LicenseRef-STK-4.3");
		m.declare("filters.lib/lowpass:author", r"Julius O. Smith III");
		m.declare("filters.lib/lowpass:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/lowpass:license", r"LicenseRef-STK-4.3");
		m.declare("filters.lib/lowpassLR4:author", r"Dario Sanfilippo");
		m.declare("filters.lib/lowpassLR4:copyright", r"Copyright (C) 2022 Dario Sanfilippo <sanfilippo.dario@gmail.com>");
		m.declare("filters.lib/lowpassLR4:license", r"LicenseRef-STK-4.3");
		m.declare("filters.lib/name", r"Faust Filters Library");
		m.declare("filters.lib/svf:author", r"Oleg Nesterov");
		m.declare("filters.lib/svf:copyright", r"Copyright (C) 2020 Oleg Nesterov <oleg@redhat.com>");
		m.declare("filters.lib/svf:license", r"LicenseRef-STK-4.3");
		m.declare("filters.lib/tf1:author", r"Julius O. Smith III");
		m.declare("filters.lib/tf1:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf1:license", r"LicenseRef-STK-4.3");
		m.declare("filters.lib/tf1s:author", r"Julius O. Smith III");
		m.declare("filters.lib/tf1s:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf1s:license", r"LicenseRef-STK-4.3");
		m.declare("filters.lib/tf2np:author", r"Julius O. Smith III");
		m.declare("filters.lib/tf2np:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf2np:license", r"LicenseRef-STK-4.3");
		m.declare("filters.lib/version", r"1.9.0");
		m.declare("license", r"MIT OR Apache-2.0");
		m.declare("maths.lib/author", r"GRAME");
		m.declare("maths.lib/copyright", r"GRAME");
		m.declare("maths.lib/license", r"LicenseRef-LGPL-2.1-or-later-with-Faust-exception");
		m.declare("maths.lib/name", r"Faust Math Library");
		m.declare("maths.lib/version", r"2.9.0");
		m.declare("name", r"moodscore");
		m.declare("noises.lib/name", r"Faust Noise Generator Library");
		m.declare("noises.lib/version", r"1.6.0");
		m.declare("oscillators.lib/name", r"Faust Oscillator Library");
		m.declare("oscillators.lib/saw2ptr:author", r"Julius O. Smith III");
		m.declare("oscillators.lib/saw2ptr:license", r"LicenseRef-STK-4.3");
		m.declare("oscillators.lib/version", r"1.8.0");
		m.declare("platform.lib/name", r"Generic Platform Library");
		m.declare("platform.lib/version", r"1.3.0");
		m.declare("reverbs.lib/name", r"Faust Reverb Library");
		m.declare("reverbs.lib/version", r"1.5.1");
		m.declare("routes.lib/hadamard:author", r"Remy Muller, revised by Romain Michon");
		m.declare("routes.lib/name", r"Faust Signal Routing Library");
		m.declare("routes.lib/version", r"1.4.0");
		m.declare("signals.lib/name", r"Faust Routing Library");
		m.declare("signals.lib/version", r"1.7.0");
		m.declare("vaeffects.lib/moog_vcf_2bn:author", r"Julius O. Smith III");
		m.declare("vaeffects.lib/moog_vcf_2bn:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("vaeffects.lib/moog_vcf_2bn:license", r"LicenseRef-STK-4.3");
		m.declare("vaeffects.lib/name", r"Faust Virtual Analog Filter Effect Library");
		m.declare("vaeffects.lib/version", r"1.6.0");
		m.declare("version", r"0.1.0");
	}

	pub fn get_sample_rate(&self) -> i32 { self.fSampleRate as i32}
	
	pub fn class_init(sample_rate: i32) {
		// Obtaining locks on 0 static var(s)
	}
	pub fn instance_reset_params(&mut self) {
		self.fHslider0 = (0.5) as FaustFloat;
		self.fHslider1 = (9e+02) as FaustFloat;
		self.fHslider2 = (0.3) as FaustFloat;
		self.fHslider3 = (0.0) as FaustFloat;
		self.fHslider4 = (0.0) as FaustFloat;
		self.fHslider5 = (0.0) as FaustFloat;
		self.fHslider6 = (0.0) as FaustFloat;
		self.fHslider7 = (0.3) as FaustFloat;
		self.fHslider8 = (0.7) as FaustFloat;
		self.fHslider9 = (7.0) as FaustFloat;
		self.fHslider10 = (0.3) as FaustFloat;
		self.fHslider11 = (2.2e+02) as FaustFloat;
		self.fHslider12 = (7.0) as FaustFloat;
		self.fHslider13 = (0.3) as FaustFloat;
		self.fHslider14 = (164.81) as FaustFloat;
		self.fHslider15 = (7.0) as FaustFloat;
		self.fHslider16 = (0.3) as FaustFloat;
		self.fHslider17 = (130.81) as FaustFloat;
		self.fHslider18 = (7.0) as FaustFloat;
		self.fHslider19 = (0.3) as FaustFloat;
		self.fHslider20 = (1.1e+02) as FaustFloat;
		self.fHslider21 = (7.0) as FaustFloat;
		self.fHslider22 = (0.3) as FaustFloat;
		self.fHslider23 = (82.41) as FaustFloat;
		self.fHslider24 = (7.0) as FaustFloat;
		self.fHslider25 = (0.3) as FaustFloat;
		self.fHslider26 = (55.0) as FaustFloat;
		self.fHslider27 = (0.5) as FaustFloat;
		self.fHslider28 = (0.35) as FaustFloat;
	}
	pub fn instance_clear(&mut self) {
		for l0 in 0..2 {
			self.iVec0[l0 as usize] = 0;
		}
		for l1 in 0..2 {
			self.fRec0[l1 as usize] = 0.0;
		}
		for l2 in 0..2 {
			self.fRec5[l2 as usize] = 0.0;
		}
		for l3 in 0..2 {
			self.fRec6[l3 as usize] = 0.0;
		}
		for l4 in 0..2 {
			self.fRec11[l4 as usize] = 0.0;
		}
		for l5 in 0..2 {
			self.fRec12[l5 as usize] = 0.0;
		}
		for l6 in 0..2 {
			self.fRec13[l6 as usize] = 0.0;
		}
		for l7 in 0..2 {
			self.fRec14[l7 as usize] = 0.0;
		}
		for l8 in 0..2 {
			self.fRec15[l8 as usize] = 0.0;
		}
		for l9 in 0..2 {
			self.fRec16[l9 as usize] = 0.0;
		}
		for l10 in 0..2 {
			self.fRec17[l10 as usize] = 0.0;
		}
		for l11 in 0..2 {
			self.iRec19[l11 as usize] = 0;
		}
		for l12 in 0..2 {
			self.fVec1[l12 as usize] = 0.0;
		}
		for l13 in 0..2 {
			self.fRec18[l13 as usize] = 0.0;
		}
		for l14 in 0..2 {
			self.fRec20[l14 as usize] = 0.0;
		}
		for l15 in 0..2 {
			self.fRec21[l15 as usize] = 0.0;
		}
		for l16 in 0..2 {
			self.fRec22[l16 as usize] = 0.0;
		}
		for l17 in 0..2 {
			self.fVec2[l17 as usize] = 0.0;
		}
		for l18 in 0..2 {
			self.fRec23[l18 as usize] = 0.0;
		}
		for l19 in 0..2 {
			self.fRec24[l19 as usize] = 0.0;
		}
		for l20 in 0..2 {
			self.fRec25[l20 as usize] = 0.0;
		}
		for l21 in 0..2 {
			self.fVec3[l21 as usize] = 0.0;
		}
		for l22 in 0..2 {
			self.fRec26[l22 as usize] = 0.0;
		}
		for l23 in 0..2 {
			self.fRec27[l23 as usize] = 0.0;
		}
		for l24 in 0..2 {
			self.fRec28[l24 as usize] = 0.0;
		}
		for l25 in 0..2 {
			self.fRec29[l25 as usize] = 0.0;
		}
		for l26 in 0..2 {
			self.fVec4[l26 as usize] = 0.0;
		}
		for l27 in 0..2 {
			self.fRec30[l27 as usize] = 0.0;
		}
		for l28 in 0..2 {
			self.fRec31[l28 as usize] = 0.0;
		}
		for l29 in 0..2 {
			self.fRec32[l29 as usize] = 0.0;
		}
		for l30 in 0..2 {
			self.fVec5[l30 as usize] = 0.0;
		}
		for l31 in 0..2 {
			self.fRec33[l31 as usize] = 0.0;
		}
		for l32 in 0..2 {
			self.fRec34[l32 as usize] = 0.0;
		}
		for l33 in 0..2 {
			self.fRec35[l33 as usize] = 0.0;
		}
		for l34 in 0..2 {
			self.fRec36[l34 as usize] = 0.0;
		}
		for l35 in 0..2 {
			self.fVec6[l35 as usize] = 0.0;
		}
		for l36 in 0..2 {
			self.fRec37[l36 as usize] = 0.0;
		}
		for l37 in 0..2 {
			self.fRec38[l37 as usize] = 0.0;
		}
		for l38 in 0..2 {
			self.fRec39[l38 as usize] = 0.0;
		}
		for l39 in 0..2 {
			self.fVec7[l39 as usize] = 0.0;
		}
		for l40 in 0..2 {
			self.fRec40[l40 as usize] = 0.0;
		}
		for l41 in 0..2 {
			self.fRec41[l41 as usize] = 0.0;
		}
		for l42 in 0..2 {
			self.fRec42[l42 as usize] = 0.0;
		}
		for l43 in 0..2 {
			self.fRec43[l43 as usize] = 0.0;
		}
		for l44 in 0..2 {
			self.fVec8[l44 as usize] = 0.0;
		}
		for l45 in 0..2 {
			self.fRec44[l45 as usize] = 0.0;
		}
		for l46 in 0..2 {
			self.fRec45[l46 as usize] = 0.0;
		}
		for l47 in 0..2 {
			self.fRec46[l47 as usize] = 0.0;
		}
		for l48 in 0..2 {
			self.fVec9[l48 as usize] = 0.0;
		}
		for l49 in 0..2 {
			self.fRec47[l49 as usize] = 0.0;
		}
		for l50 in 0..2 {
			self.fRec48[l50 as usize] = 0.0;
		}
		for l51 in 0..2 {
			self.fRec49[l51 as usize] = 0.0;
		}
		for l52 in 0..2 {
			self.fRec50[l52 as usize] = 0.0;
		}
		for l53 in 0..2 {
			self.fVec10[l53 as usize] = 0.0;
		}
		for l54 in 0..2 {
			self.fRec51[l54 as usize] = 0.0;
		}
		for l55 in 0..2 {
			self.fRec52[l55 as usize] = 0.0;
		}
		for l56 in 0..2 {
			self.fRec53[l56 as usize] = 0.0;
		}
		for l57 in 0..2 {
			self.fVec11[l57 as usize] = 0.0;
		}
		for l58 in 0..2 {
			self.fRec54[l58 as usize] = 0.0;
		}
		for l59 in 0..2 {
			self.fRec55[l59 as usize] = 0.0;
		}
		for l60 in 0..2 {
			self.fRec56[l60 as usize] = 0.0;
		}
		for l61 in 0..2 {
			self.fRec57[l61 as usize] = 0.0;
		}
		for l62 in 0..2 {
			self.fVec12[l62 as usize] = 0.0;
		}
		for l63 in 0..2 {
			self.fRec58[l63 as usize] = 0.0;
		}
		for l64 in 0..2 {
			self.fRec59[l64 as usize] = 0.0;
		}
		for l65 in 0..2 {
			self.fRec60[l65 as usize] = 0.0;
		}
		for l66 in 0..2 {
			self.fVec13[l66 as usize] = 0.0;
		}
		for l67 in 0..2 {
			self.fRec63[l67 as usize] = 0.0;
		}
		for l68 in 0..2 {
			self.fRec61[l68 as usize] = 0.0;
		}
		for l69 in 0..2 {
			self.fRec62[l69 as usize] = 0.0;
		}
		for l70 in 0..2 {
			self.fRec9[l70 as usize] = 0.0;
		}
		for l71 in 0..2 {
			self.fRec10[l71 as usize] = 0.0;
		}
		for l72 in 0..2 {
			self.fRec7[l72 as usize] = 0.0;
		}
		for l73 in 0..2 {
			self.fRec8[l73 as usize] = 0.0;
		}
		for l74 in 0..2 {
			self.fRec66[l74 as usize] = 0.0;
		}
		for l75 in 0..2 {
			self.fRec67[l75 as usize] = 0.0;
		}
		for l76 in 0..2 {
			self.fRec64[l76 as usize] = 0.0;
		}
		for l77 in 0..2 {
			self.fRec65[l77 as usize] = 0.0;
		}
		for l78 in 0..2 {
			self.fRec68[l78 as usize] = 0.0;
		}
		for l79 in 0..2 {
			self.fRec69[l79 as usize] = 0.0;
		}
		for l80 in 0..2 {
			self.fRec70[l80 as usize] = 0.0;
		}
		for l81 in 0..2 {
			self.fRec71[l81 as usize] = 0.0;
		}
		for l82 in 0..2 {
			self.fRec72[l82 as usize] = 0.0;
		}
		for l83 in 0..2 {
			self.fRec73[l83 as usize] = 0.0;
		}
		for l84 in 0..2 {
			self.fRec74[l84 as usize] = 0.0;
		}
		for l85 in 0..2 {
			self.fRec75[l85 as usize] = 0.0;
		}
		for l86 in 0..2 {
			self.fRec76[l86 as usize] = 0.0;
		}
		for l87 in 0..2 {
			self.fRec77[l87 as usize] = 0.0;
		}
		for l88 in 0..2 {
			self.fRec78[l88 as usize] = 0.0;
		}
		self.IOTA0 = 0;
		for l89 in 0..16384 {
			self.fVec14[l89 as usize] = 0.0;
		}
		for l90 in 0..2048 {
			self.fVec15[l90 as usize] = 0.0;
		}
		for l91 in 0..2 {
			self.fRec80[l91 as usize] = 0.0;
		}
		for l92 in 0..2 {
			self.fRec81[l92 as usize] = 0.0;
		}
		for l93 in 0..16384 {
			self.fVec16[l93 as usize] = 0.0;
		}
		for l94 in 0..2048 {
			self.fVec17[l94 as usize] = 0.0;
		}
		for l95 in 0..2 {
			self.fRec83[l95 as usize] = 0.0;
		}
		for l96 in 0..2 {
			self.fRec84[l96 as usize] = 0.0;
		}
		for l97 in 0..2 {
			self.fRec85[l97 as usize] = 0.0;
		}
		for l98 in 0..16384 {
			self.fVec18[l98 as usize] = 0.0;
		}
		for l99 in 0..4096 {
			self.fVec19[l99 as usize] = 0.0;
		}
		for l100 in 0..2 {
			self.fRec87[l100 as usize] = 0.0;
		}
		for l101 in 0..2 {
			self.fRec88[l101 as usize] = 0.0;
		}
		for l102 in 0..2 {
			self.fRec89[l102 as usize] = 0.0;
		}
		for l103 in 0..2 {
			self.fRec90[l103 as usize] = 0.0;
		}
		for l104 in 0..32768 {
			self.fVec20[l104 as usize] = 0.0;
		}
		for l105 in 0..2048 {
			self.fVec21[l105 as usize] = 0.0;
		}
		for l106 in 0..2 {
			self.fRec91[l106 as usize] = 0.0;
		}
		for l107 in 0..16384 {
			self.fVec22[l107 as usize] = 0.0;
		}
		for l108 in 0..4096 {
			self.fVec23[l108 as usize] = 0.0;
		}
		for l109 in 0..2 {
			self.fRec93[l109 as usize] = 0.0;
		}
		for l110 in 0..16384 {
			self.fVec24[l110 as usize] = 0.0;
		}
		for l111 in 0..4096 {
			self.fVec25[l111 as usize] = 0.0;
		}
		for l112 in 0..2 {
			self.fRec95[l112 as usize] = 0.0;
		}
		for l113 in 0..32768 {
			self.fVec26[l113 as usize] = 0.0;
		}
		for l114 in 0..4096 {
			self.fVec27[l114 as usize] = 0.0;
		}
		for l115 in 0..2 {
			self.fRec96[l115 as usize] = 0.0;
		}
		for l116 in 0..32768 {
			self.fVec28[l116 as usize] = 0.0;
		}
		for l117 in 0..4096 {
			self.fVec29[l117 as usize] = 0.0;
		}
		for l118 in 0..2 {
			self.fRec97[l118 as usize] = 0.0;
		}
		for l119 in 0..3 {
			self.fRec101[l119 as usize] = 0.0;
		}
		for l120 in 0..3 {
			self.fRec102[l120 as usize] = 0.0;
		}
		for l121 in 0..3 {
			self.fRec103[l121 as usize] = 0.0;
		}
		for l122 in 0..3 {
			self.fRec104[l122 as usize] = 0.0;
		}
		for l123 in 0..3 {
			self.fRec105[l123 as usize] = 0.0;
		}
		for l124 in 0..3 {
			self.fRec106[l124 as usize] = 0.0;
		}
		for l125 in 0..3 {
			self.fRec107[l125 as usize] = 0.0;
		}
		for l126 in 0..3 {
			self.fRec108[l126 as usize] = 0.0;
		}
		for l127 in 0..2 {
			self.fRec3[l127 as usize] = 0.0;
		}
		for l128 in 0..2 {
			self.fRec4[l128 as usize] = 0.0;
		}
		for l129 in 0..2 {
			self.fRec1[l129 as usize] = 0.0;
		}
		for l130 in 0..2 {
			self.fRec2[l130 as usize] = 0.0;
		}
		for l131 in 0..2 {
			self.fRec109[l131 as usize] = 0.0;
		}
		for l132 in 0..2 {
			self.fRec110[l132 as usize] = 0.0;
		}
		for l133 in 0..2 {
			self.fRec111[l133 as usize] = 0.0;
		}
		for l134 in 0..2 {
			self.fRec112[l134 as usize] = 0.0;
		}
		for l135 in 0..2 {
			self.fRec113[l135 as usize] = 0.0;
		}
		for l136 in 0..2 {
			self.fRec114[l136 as usize] = 0.0;
		}
		for l137 in 0..2 {
			self.fRec115[l137 as usize] = 0.0;
		}
		for l138 in 0..2 {
			self.fRec116[l138 as usize] = 0.0;
		}
	}
	pub fn instance_constants(&mut self, sample_rate: i32) {
		// Obtaining locks on 0 static var(s)
		self.fSampleRate = sample_rate;
		self.fConst0 = F32::min(1.92e+05, F32::max(1.0, (self.fSampleRate) as F32));
		self.fConst1 = 44.1 / self.fConst0;
		self.fConst2 = 1.0 - self.fConst1;
		self.fConst3 = 3.1415927 / self.fConst0;
		self.fConst4 = 1.0 / F32::tan(0.4712389 / self.fConst0);
		self.fConst5 = 1.0 - self.fConst4;
		self.fConst6 = 1.0 / (self.fConst4 + 1.0);
		self.fConst7 = 1.0 / self.fConst0;
		self.fConst8 = 1.0 / F32::tan(628.31854 / self.fConst0);
		self.fConst9 = 1.0 - self.fConst8;
		self.fConst10 = 1.0 / (self.fConst8 + 1.0);
		self.fConst11 = F32::floor(0.125 * self.fConst0 + 0.5);
		self.fConst12 = self.fConst11 / self.fConst0;
		self.fConst13 = F32::exp(-(0.8634694 * self.fConst12));
		self.fConst14 = MoodscoreDsp_faustpower2_f(self.fConst13);
		self.fConst15 = 1.0 - self.fConst14;
		self.fConst16 = F32::cos(37699.113 / self.fConst0);
		self.fConst17 = 1.0 - self.fConst16 * self.fConst14;
		self.fConst18 = F32::sqrt(F32::max(0.0, MoodscoreDsp_faustpower2_f(self.fConst17) / MoodscoreDsp_faustpower2_f(self.fConst15) + -1.0));
		self.fConst19 = self.fConst17 / self.fConst15;
		self.fConst20 = self.fConst19 - self.fConst18;
		self.fConst21 = F32::exp(-(1.1512926 * self.fConst12)) / self.fConst13 + -1.0;
		self.fConst22 = self.fConst13 * (self.fConst18 + (1.0 - self.fConst19));
		self.fConst23 = F32::floor(0.153129 * self.fConst0 + 0.5);
		self.fConst24 = self.fConst23 / self.fConst0;
		self.fConst25 = F32::exp(-(0.8634694 * self.fConst24));
		self.fConst26 = MoodscoreDsp_faustpower2_f(self.fConst25);
		self.fConst27 = 1.0 - self.fConst26;
		self.fConst28 = 1.0 - self.fConst16 * self.fConst26;
		self.fConst29 = F32::sqrt(F32::max(0.0, MoodscoreDsp_faustpower2_f(self.fConst28) / MoodscoreDsp_faustpower2_f(self.fConst27) + -1.0));
		self.fConst30 = self.fConst28 / self.fConst27;
		self.fConst31 = self.fConst30 - self.fConst29;
		self.fConst32 = F32::exp(-(1.1512926 * self.fConst24)) / self.fConst25 + -1.0;
		self.fConst33 = self.fConst25 * (self.fConst29 + (1.0 - self.fConst30));
		self.fConst34 = F32::floor(0.013458 * self.fConst0 + 0.5);
		self.iConst35 = (F32::min(8192.0, F32::max(0.0, self.fConst11 - self.fConst34))) as i32;
		self.fConst36 = F32::floor(0.127837 * self.fConst0 + 0.5);
		self.fConst37 = self.fConst36 / self.fConst0;
		self.fConst38 = F32::exp(-(0.8634694 * self.fConst37));
		self.fConst39 = MoodscoreDsp_faustpower2_f(self.fConst38);
		self.fConst40 = 1.0 - self.fConst39;
		self.fConst41 = 1.0 - self.fConst16 * self.fConst39;
		self.fConst42 = F32::sqrt(F32::max(0.0, MoodscoreDsp_faustpower2_f(self.fConst41) / MoodscoreDsp_faustpower2_f(self.fConst40) + -1.0));
		self.fConst43 = self.fConst41 / self.fConst40;
		self.fConst44 = self.fConst43 - self.fConst42;
		self.fConst45 = F32::exp(-(1.1512926 * self.fConst37)) / self.fConst38 + -1.0;
		self.fConst46 = self.fConst38 * (self.fConst42 + (1.0 - self.fConst43));
		self.fConst47 = F32::floor(0.192303 * self.fConst0 + 0.5);
		self.fConst48 = self.fConst47 / self.fConst0;
		self.fConst49 = F32::exp(-(0.8634694 * self.fConst48));
		self.fConst50 = MoodscoreDsp_faustpower2_f(self.fConst49);
		self.fConst51 = 1.0 - self.fConst50;
		self.fConst52 = 1.0 - self.fConst16 * self.fConst50;
		self.fConst53 = F32::sqrt(F32::max(0.0, MoodscoreDsp_faustpower2_f(self.fConst52) / MoodscoreDsp_faustpower2_f(self.fConst51) + -1.0));
		self.fConst54 = self.fConst52 / self.fConst51;
		self.fConst55 = self.fConst54 - self.fConst53;
		self.fConst56 = F32::exp(-(1.1512926 * self.fConst48)) / self.fConst49 + -1.0;
		self.fConst57 = self.fConst49 * (self.fConst53 + (1.0 - self.fConst54));
		self.fConst58 = F32::floor(0.020346 * self.fConst0 + 0.5);
		self.iConst59 = (F32::min(8192.0, F32::max(0.0, self.fConst23 - self.fConst58))) as i32;
		self.fConst60 = F32::floor(0.219991 * self.fConst0 + 0.5);
		self.fConst61 = self.fConst60 / self.fConst0;
		self.fConst62 = F32::exp(-(0.8634694 * self.fConst61));
		self.fConst63 = MoodscoreDsp_faustpower2_f(self.fConst62);
		self.fConst64 = 1.0 - self.fConst63;
		self.fConst65 = 1.0 - self.fConst16 * self.fConst63;
		self.fConst66 = F32::sqrt(F32::max(0.0, MoodscoreDsp_faustpower2_f(self.fConst65) / MoodscoreDsp_faustpower2_f(self.fConst64) + -1.0));
		self.fConst67 = self.fConst65 / self.fConst64;
		self.fConst68 = self.fConst67 - self.fConst66;
		self.fConst69 = F32::exp(-(1.1512926 * self.fConst61)) / self.fConst62 + -1.0;
		self.fConst70 = self.fConst62 * (self.fConst66 + (1.0 - self.fConst67));
		self.fConst71 = F32::floor(0.174713 * self.fConst0 + 0.5);
		self.fConst72 = self.fConst71 / self.fConst0;
		self.fConst73 = F32::exp(-(0.8634694 * self.fConst72));
		self.fConst74 = MoodscoreDsp_faustpower2_f(self.fConst73);
		self.fConst75 = 1.0 - self.fConst74;
		self.fConst76 = 1.0 - self.fConst16 * self.fConst74;
		self.fConst77 = F32::sqrt(F32::max(0.0, MoodscoreDsp_faustpower2_f(self.fConst76) / MoodscoreDsp_faustpower2_f(self.fConst75) + -1.0));
		self.fConst78 = self.fConst76 / self.fConst75;
		self.fConst79 = self.fConst78 - self.fConst77;
		self.fConst80 = F32::exp(-(1.1512926 * self.fConst72)) / self.fConst73 + -1.0;
		self.fConst81 = self.fConst73 * (self.fConst77 + (1.0 - self.fConst78));
		self.iConst82 = (F32::min(1024.0, F32::max(0.0, self.fConst58 + -1.0))) as i32;
		self.fConst83 = F32::floor(0.031604 * self.fConst0 + 0.5);
		self.iConst84 = (F32::min(8192.0, F32::max(0.0, self.fConst36 - self.fConst83))) as i32;
		self.iConst85 = (F32::min(1024.0, F32::max(0.0, self.fConst34 + -1.0))) as i32;
		self.iConst86 = (F32::min(2048.0, F32::max(0.0, self.fConst83 + -1.0))) as i32;
		self.fConst87 = F32::floor(0.210389 * self.fConst0 + 0.5);
		self.fConst88 = self.fConst87 / self.fConst0;
		self.fConst89 = F32::exp(-(0.8634694 * self.fConst88));
		self.fConst90 = MoodscoreDsp_faustpower2_f(self.fConst89);
		self.fConst91 = 1.0 - self.fConst90;
		self.fConst92 = 1.0 - self.fConst16 * self.fConst90;
		self.fConst93 = F32::sqrt(F32::max(0.0, MoodscoreDsp_faustpower2_f(self.fConst92) / MoodscoreDsp_faustpower2_f(self.fConst91) + -1.0));
		self.fConst94 = self.fConst92 / self.fConst91;
		self.fConst95 = self.fConst94 - self.fConst93;
		self.fConst96 = F32::exp(-(1.1512926 * self.fConst88)) / self.fConst89 + -1.0;
		self.fConst97 = self.fConst89 * (self.fConst93 + (1.0 - self.fConst94));
		self.fConst98 = F32::floor(0.256891 * self.fConst0 + 0.5);
		self.fConst99 = self.fConst98 / self.fConst0;
		self.fConst100 = F32::exp(-(0.8634694 * self.fConst99));
		self.fConst101 = MoodscoreDsp_faustpower2_f(self.fConst100);
		self.fConst102 = 1.0 - self.fConst101;
		self.fConst103 = 1.0 - self.fConst16 * self.fConst101;
		self.fConst104 = F32::sqrt(F32::max(0.0, MoodscoreDsp_faustpower2_f(self.fConst103) / MoodscoreDsp_faustpower2_f(self.fConst102) + -1.0));
		self.fConst105 = self.fConst103 / self.fConst102;
		self.fConst106 = self.fConst105 - self.fConst104;
		self.fConst107 = F32::exp(-(1.1512926 * self.fConst99)) / self.fConst100 + -1.0;
		self.fConst108 = self.fConst100 * (self.fConst104 + (1.0 - self.fConst105));
		self.fConst109 = F32::floor(0.019123 * self.fConst0 + 0.5);
		self.iConst110 = (F32::min(16384.0, F32::max(0.0, self.fConst60 - self.fConst109))) as i32;
		self.iConst111 = (F32::min(1024.0, F32::max(0.0, self.fConst109 + -1.0))) as i32;
		self.fConst112 = F32::floor(0.029291 * self.fConst0 + 0.5);
		self.iConst113 = (F32::min(8192.0, F32::max(0.0, self.fConst47 - self.fConst112))) as i32;
		self.iConst114 = (F32::min(2048.0, F32::max(0.0, self.fConst112 + -1.0))) as i32;
		self.fConst115 = F32::floor(0.022904 * self.fConst0 + 0.5);
		self.iConst116 = (F32::min(8192.0, F32::max(0.0, self.fConst71 - self.fConst115))) as i32;
		self.iConst117 = (F32::min(2048.0, F32::max(0.0, self.fConst115 + -1.0))) as i32;
		self.fConst118 = F32::floor(0.027333 * self.fConst0 + 0.5);
		self.iConst119 = (F32::min(16384.0, F32::max(0.0, self.fConst98 - self.fConst118))) as i32;
		self.iConst120 = (F32::min(2048.0, F32::max(0.0, self.fConst118 + -1.0))) as i32;
		self.fConst121 = F32::floor(0.024421 * self.fConst0 + 0.5);
		self.iConst122 = (F32::min(16384.0, F32::max(0.0, self.fConst87 - self.fConst121))) as i32;
		self.iConst123 = (F32::min(2048.0, F32::max(0.0, self.fConst121 + -1.0))) as i32;
		self.fConst124 = F32::tan(471.2389 / self.fConst0);
		self.fConst125 = self.fConst124 * (self.fConst124 + 1.4142135) + 1.0;
		self.fConst126 = self.fConst124 / self.fConst125;
		self.fConst127 = 2.0 / self.fConst125;
		self.fConst128 = -1.4142135 - self.fConst124;
		self.fConst129 = self.fConst128 / self.fConst125;
		self.fConst130 = 1.0 / self.fConst125;
	}
	pub fn instance_init(&mut self, sample_rate: i32) {
		self.instance_constants(sample_rate);
		self.instance_reset_params();
		self.instance_clear();
	}
	pub fn init(&mut self, sample_rate: i32) {
		MoodscoreDsp::class_init(sample_rate);
		self.instance_init(sample_rate);
	}
	
	pub fn build_user_interface(&self, ui_interface: &mut dyn UI<FaustFloat>) {
		Self::build_user_interface_static(ui_interface);
	}
	
	pub fn build_user_interface_static(ui_interface: &mut dyn UI<FaustFloat>) {
		ui_interface.open_vertical_box("moodscore");
		ui_interface.declare(Some(ParamIndex(0)), "unit", "cents");
		ui_interface.add_horizontal_slider("drone_1/detune", ParamIndex(0), 7.0, 0.0, 5e+01, 0.01);
		ui_interface.add_horizontal_slider("drone_1/drift", ParamIndex(1), 0.3, 0.0, 1.0, 0.001);
		ui_interface.add_horizontal_slider("drone_1/level", ParamIndex(2), 0.7, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(3)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(3)), "unit", "Hz");
		ui_interface.add_horizontal_slider("drone_1/pitch", ParamIndex(3), 55.0, 2e+01, 8.8e+02, 0.01);
		ui_interface.add_horizontal_slider("drone_1/sub", ParamIndex(4), 0.5, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(5)), "unit", "cents");
		ui_interface.add_horizontal_slider("drone_2/detune", ParamIndex(5), 7.0, 0.0, 5e+01, 0.01);
		ui_interface.add_horizontal_slider("drone_2/drift", ParamIndex(6), 0.3, 0.0, 1.0, 0.001);
		ui_interface.add_horizontal_slider("drone_2/level", ParamIndex(7), 0.3, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(8)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(8)), "unit", "Hz");
		ui_interface.add_horizontal_slider("drone_2/pitch", ParamIndex(8), 82.41, 2e+01, 8.8e+02, 0.01);
		ui_interface.declare(Some(ParamIndex(9)), "unit", "cents");
		ui_interface.add_horizontal_slider("drone_3/detune", ParamIndex(9), 7.0, 0.0, 5e+01, 0.01);
		ui_interface.add_horizontal_slider("drone_3/drift", ParamIndex(10), 0.3, 0.0, 1.0, 0.001);
		ui_interface.add_horizontal_slider("drone_3/level", ParamIndex(11), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(12)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(12)), "unit", "Hz");
		ui_interface.add_horizontal_slider("drone_3/pitch", ParamIndex(12), 1.1e+02, 2e+01, 8.8e+02, 0.01);
		ui_interface.declare(Some(ParamIndex(13)), "unit", "cents");
		ui_interface.add_horizontal_slider("drone_4/detune", ParamIndex(13), 7.0, 0.0, 5e+01, 0.01);
		ui_interface.add_horizontal_slider("drone_4/drift", ParamIndex(14), 0.3, 0.0, 1.0, 0.001);
		ui_interface.add_horizontal_slider("drone_4/level", ParamIndex(15), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(16)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(16)), "unit", "Hz");
		ui_interface.add_horizontal_slider("drone_4/pitch", ParamIndex(16), 130.81, 2e+01, 8.8e+02, 0.01);
		ui_interface.declare(Some(ParamIndex(17)), "unit", "cents");
		ui_interface.add_horizontal_slider("drone_5/detune", ParamIndex(17), 7.0, 0.0, 5e+01, 0.01);
		ui_interface.add_horizontal_slider("drone_5/drift", ParamIndex(18), 0.3, 0.0, 1.0, 0.001);
		ui_interface.add_horizontal_slider("drone_5/level", ParamIndex(19), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(20)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(20)), "unit", "Hz");
		ui_interface.add_horizontal_slider("drone_5/pitch", ParamIndex(20), 164.81, 2e+01, 8.8e+02, 0.01);
		ui_interface.declare(Some(ParamIndex(21)), "unit", "cents");
		ui_interface.add_horizontal_slider("drone_6/detune", ParamIndex(21), 7.0, 0.0, 5e+01, 0.01);
		ui_interface.add_horizontal_slider("drone_6/drift", ParamIndex(22), 0.3, 0.0, 1.0, 0.001);
		ui_interface.add_horizontal_slider("drone_6/level", ParamIndex(23), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(24)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(24)), "unit", "Hz");
		ui_interface.add_horizontal_slider("drone_6/pitch", ParamIndex(24), 2.2e+02, 2e+01, 8.8e+02, 0.01);
		ui_interface.declare(Some(ParamIndex(25)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(25)), "unit", "Hz");
		ui_interface.add_horizontal_slider("filter/cutoff", ParamIndex(25), 9e+02, 4e+01, 1.2e+04, 0.1);
		ui_interface.add_horizontal_slider("filter/resonance", ParamIndex(26), 0.3, 0.0, 0.95, 0.001);
		ui_interface.add_horizontal_slider("master/gain", ParamIndex(27), 0.5, 0.0, 1.0, 0.001);
		ui_interface.add_horizontal_slider("space/reverb_mix", ParamIndex(28), 0.35, 0.0, 1.0, 0.001);
		ui_interface.close_box();
	}
	
	pub fn get_param(&self, param: ParamIndex) -> Option<FaustFloat> {
		match param.0 {
			27 => Some(self.fHslider0),
			25 => Some(self.fHslider1),
			22 => Some(self.fHslider10),
			24 => Some(self.fHslider11),
			17 => Some(self.fHslider12),
			18 => Some(self.fHslider13),
			20 => Some(self.fHslider14),
			13 => Some(self.fHslider15),
			14 => Some(self.fHslider16),
			16 => Some(self.fHslider17),
			9 => Some(self.fHslider18),
			10 => Some(self.fHslider19),
			26 => Some(self.fHslider2),
			12 => Some(self.fHslider20),
			5 => Some(self.fHslider21),
			6 => Some(self.fHslider22),
			8 => Some(self.fHslider23),
			0 => Some(self.fHslider24),
			1 => Some(self.fHslider25),
			3 => Some(self.fHslider26),
			4 => Some(self.fHslider27),
			28 => Some(self.fHslider28),
			23 => Some(self.fHslider3),
			19 => Some(self.fHslider4),
			15 => Some(self.fHslider5),
			11 => Some(self.fHslider6),
			7 => Some(self.fHslider7),
			2 => Some(self.fHslider8),
			21 => Some(self.fHslider9),
			_ => None,
		}
	}
	
	pub fn set_param(&mut self, param: ParamIndex, value: FaustFloat) {
		match param.0 {
			27 => { self.fHslider0 = value }
			25 => { self.fHslider1 = value }
			22 => { self.fHslider10 = value }
			24 => { self.fHslider11 = value }
			17 => { self.fHslider12 = value }
			18 => { self.fHslider13 = value }
			20 => { self.fHslider14 = value }
			13 => { self.fHslider15 = value }
			14 => { self.fHslider16 = value }
			16 => { self.fHslider17 = value }
			9 => { self.fHslider18 = value }
			10 => { self.fHslider19 = value }
			26 => { self.fHslider2 = value }
			12 => { self.fHslider20 = value }
			5 => { self.fHslider21 = value }
			6 => { self.fHslider22 = value }
			8 => { self.fHslider23 = value }
			0 => { self.fHslider24 = value }
			1 => { self.fHslider25 = value }
			3 => { self.fHslider26 = value }
			4 => { self.fHslider27 = value }
			28 => { self.fHslider28 = value }
			23 => { self.fHslider3 = value }
			19 => { self.fHslider4 = value }
			15 => { self.fHslider5 = value }
			11 => { self.fHslider6 = value }
			7 => { self.fHslider7 = value }
			2 => { self.fHslider8 = value }
			21 => { self.fHslider9 = value }
			_ => {}
		}
	}
	
	pub fn compute(
		&mut self,
		count: usize,
		inputs: &[impl AsRef<[FaustFloat]>],
		outputs: &mut[impl AsMut<[FaustFloat]>],
	) {
		
		// Obtaining locks on 0 static var(s)
		let [inputs0, inputs1, inputs2, inputs3, .. ] = inputs.as_ref() else { panic!("wrong number of input buffers"); };
		let inputs0 = inputs0.as_ref()[..count].iter();
		let inputs1 = inputs1.as_ref()[..count].iter();
		let inputs2 = inputs2.as_ref()[..count].iter();
		let inputs3 = inputs3.as_ref()[..count].iter();
		let [outputs0, outputs1, .. ] = outputs.as_mut() else { panic!("wrong number of output buffers"); };
		let outputs0 = outputs0.as_mut()[..count].iter_mut();
		let outputs1 = outputs1.as_mut()[..count].iter_mut();
		let mut fSlow0: F32 = self.fConst1 * (self.fHslider0) as F32;
		let mut fSlow1: F32 = self.fConst1 * (self.fHslider1) as F32;
		let mut fSlow2: F32 = self.fConst1 * (self.fHslider2) as F32;
		let mut fSlow3: F32 = self.fConst1 * (self.fHslider3) as F32;
		let mut fSlow4: F32 = self.fConst1 * (self.fHslider4) as F32;
		let mut fSlow5: F32 = self.fConst1 * (self.fHslider5) as F32;
		let mut fSlow6: F32 = self.fConst1 * (self.fHslider6) as F32;
		let mut fSlow7: F32 = self.fConst1 * (self.fHslider7) as F32;
		let mut fSlow8: F32 = self.fConst1 * (self.fHslider8) as F32;
		let mut fSlow9: F32 = self.fConst1 * (self.fHslider9) as F32;
		let mut fSlow10: F32 = self.fConst1 * (self.fHslider10) as F32;
		let mut fSlow11: F32 = self.fConst1 * (self.fHslider11) as F32;
		let mut fSlow12: F32 = self.fConst1 * (self.fHslider12) as F32;
		let mut fSlow13: F32 = self.fConst1 * (self.fHslider13) as F32;
		let mut fSlow14: F32 = self.fConst1 * (self.fHslider14) as F32;
		let mut fSlow15: F32 = self.fConst1 * (self.fHslider15) as F32;
		let mut fSlow16: F32 = self.fConst1 * (self.fHslider16) as F32;
		let mut fSlow17: F32 = self.fConst1 * (self.fHslider17) as F32;
		let mut fSlow18: F32 = self.fConst1 * (self.fHslider18) as F32;
		let mut fSlow19: F32 = self.fConst1 * (self.fHslider19) as F32;
		let mut fSlow20: F32 = self.fConst1 * (self.fHslider20) as F32;
		let mut fSlow21: F32 = self.fConst1 * (self.fHslider21) as F32;
		let mut fSlow22: F32 = self.fConst1 * (self.fHslider22) as F32;
		let mut fSlow23: F32 = self.fConst1 * (self.fHslider23) as F32;
		let mut fSlow24: F32 = self.fConst1 * (self.fHslider24) as F32;
		let mut fSlow25: F32 = self.fConst1 * (self.fHslider25) as F32;
		let mut fSlow26: F32 = self.fConst1 * (self.fHslider26) as F32;
		let mut fSlow27: F32 = self.fConst1 * (self.fHslider27) as F32;
		let mut fSlow28: F32 = self.fConst1 * (self.fHslider28) as F32;
		let zipped_iterators = inputs0.zip(inputs1).zip(inputs2).zip(inputs3).zip(outputs0).zip(outputs1);
		for (((((input0, input1), input2), input3), output0), output1) in zipped_iterators {
			self.iVec0[0] = 1;
			self.fRec0[0] = fSlow0 + self.fConst2 * self.fRec0[1];
			self.fRec5[0] = fSlow1 + self.fConst2 * self.fRec5[1];
			let mut fTemp0: F32 = F32::tan(self.fConst3 * F32::max(self.fRec5[0], 2e+01));
			self.fRec6[0] = fSlow2 + self.fConst2 * self.fRec6[1];
			let mut fTemp1: F32 = 1.99998 * self.fRec6[0];
			let mut fTemp2: F32 = 2.0 - fTemp1;
			let mut fTemp3: F32 = 1.0 / fTemp0;
			let mut fTemp4: F32 = MoodscoreDsp_faustpower2_f(1.4141995 * self.fRec6[0]);
			let mut fTemp5: F32 = 1.0 - fTemp1;
			let mut fTemp6: F32 = fTemp5 + fTemp4 + (fTemp3 + fTemp2) / fTemp0;
			let mut fTemp7: F32 = (fTemp5 + fTemp4 + (fTemp3 - fTemp2) / fTemp0) / fTemp6;
			let mut fTemp8: F32 = 1.0 / MoodscoreDsp_faustpower2_f(fTemp0);
			let mut fTemp9: F32 = 1.0 - (fTemp1 + fTemp8) + fTemp4;
			let mut fTemp10: F32 = F32::max(-0.9999999, F32::min(0.9999999, 2.0 * (fTemp9 / (fTemp6 * (1.0 + fTemp7)))));
			let mut fTemp11: F32 = 1.0 - MoodscoreDsp_faustpower2_f(fTemp10);
			let mut fTemp12: F32 = F32::max(-0.9999999, F32::min(0.9999999, fTemp7));
			let mut fTemp13: F32 = 1.0 - MoodscoreDsp_faustpower2_f(fTemp12);
			let mut fTemp14: F32 = F32::sqrt(fTemp13);
			let mut fTemp15: F32 = fTemp14 * F32::sqrt(fTemp11);
			let mut fTemp16: F32 = 2.0 + fTemp1;
			let mut fTemp17: F32 = fTemp1 + fTemp4;
			let mut fTemp18: F32 = 1.0 + fTemp17 + (fTemp16 + fTemp3) / fTemp0;
			let mut fTemp19: F32 = (1.0 - (fTemp16 - fTemp3) / fTemp0 + fTemp17) / fTemp18;
			let mut fTemp20: F32 = 1.0 - fTemp8 + fTemp17;
			let mut fTemp21: F32 = F32::max(-0.9999999, F32::min(0.9999999, 2.0 * (fTemp20 / (fTemp18 * (1.0 + fTemp19)))));
			let mut fTemp22: F32 = 1.0 - MoodscoreDsp_faustpower2_f(fTemp21);
			let mut fTemp23: F32 = F32::max(-0.9999999, F32::min(0.9999999, fTemp19));
			let mut fTemp24: F32 = 1.0 - MoodscoreDsp_faustpower2_f(fTemp23);
			let mut fTemp25: F32 = F32::sqrt(fTemp24);
			let mut fTemp26: F32 = fTemp25 * F32::sqrt(fTemp22);
			self.fRec11[0] = fSlow3 + self.fConst2 * self.fRec11[1];
			self.fRec12[0] = fSlow4 + self.fConst2 * self.fRec12[1];
			self.fRec13[0] = fSlow5 + self.fConst2 * self.fRec13[1];
			self.fRec14[0] = fSlow6 + self.fConst2 * self.fRec14[1];
			self.fRec15[0] = fSlow7 + self.fConst2 * self.fRec15[1];
			self.fRec16[0] = fSlow8 + self.fConst2 * self.fRec16[1];
			let mut fTemp27: F32 = F32::max(1.0, 0.6666667 * F32::sqrt(MoodscoreDsp_faustpower2_f(self.fRec16[0]) + MoodscoreDsp_faustpower2_f(self.fRec15[0]) + MoodscoreDsp_faustpower2_f(self.fRec14[0]) + MoodscoreDsp_faustpower2_f(self.fRec13[0]) + MoodscoreDsp_faustpower2_f(self.fRec12[0]) + MoodscoreDsp_faustpower2_f(self.fRec11[0])));
			self.fRec17[0] = fSlow9 + self.fConst2 * self.fRec17[1];
			let mut iTemp28: i32 = i32::wrapping_mul(1103515245, i32::wrapping_add(12345, i32::wrapping_mul(1103515245, i32::wrapping_add(12345, self.iRec19[1]))));
			let mut iTemp29: i32 = i32::wrapping_mul(1103515245, i32::wrapping_add(12345, iTemp28));
			let mut iTemp30: i32 = i32::wrapping_mul(1103515245, i32::wrapping_add(12345, i32::wrapping_mul(1103515245, i32::wrapping_add(12345, iTemp29))));
			let mut iTemp31: i32 = i32::wrapping_mul(1103515245, i32::wrapping_add(12345, iTemp30));
			let mut iTemp32: i32 = i32::wrapping_mul(1103515245, i32::wrapping_add(12345, i32::wrapping_mul(1103515245, i32::wrapping_add(12345, iTemp31))));
			let mut iTemp33: i32 = i32::wrapping_mul(1103515245, i32::wrapping_add(12345, iTemp32));
			let mut iTemp34: i32 = i32::wrapping_mul(1103515245, i32::wrapping_add(12345, i32::wrapping_mul(1103515245, i32::wrapping_add(12345, iTemp33))));
			let mut iTemp35: i32 = i32::wrapping_mul(1103515245, i32::wrapping_add(12345, iTemp34));
			let mut iTemp36: i32 = i32::wrapping_mul(1103515245, i32::wrapping_add(12345, i32::wrapping_mul(1103515245, i32::wrapping_add(12345, iTemp35))));
			let mut iTemp37: i32 = i32::wrapping_mul(1103515245, i32::wrapping_add(12345, iTemp36));
			let mut iTemp38: i32 = i32::wrapping_mul(1103515245, i32::wrapping_add(12345, iTemp37));
			let mut iTemp39: i32 = i32::wrapping_mul(1103515245, i32::wrapping_add(12345, iTemp38));
			self.iRec19[0] = i32::wrapping_mul(1103515245, i32::wrapping_add(12345, iTemp39));
			let mut fTemp40: F32 = (iTemp28) as F32;
			self.fVec1[0] = fTemp40;
			self.fRec18[0] = self.fConst6 * (4.656613e-10 * (fTemp40 + self.fVec1[1]) - self.fConst5 * self.fRec18[1]);
			self.fRec20[0] = fSlow10 + self.fConst2 * self.fRec20[1];
			self.fRec21[0] = fSlow11 + self.fConst2 * self.fRec21[1];
			let mut fTemp41: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec21[0] * F32::powf(2.0, 0.00083333335 * (4.5e+03 * self.fRec20[0] * self.fRec18[0] - self.fRec17[0]))));
			let mut fTemp42: F32 = self.fRec22[1] + self.fConst7 * fTemp41;
			let mut fTemp43: F32 = -1.0 + fTemp42;
			let mut iTemp44: i32 = (fTemp43 < 0.0) as i32;
			self.fRec22[0] = (if iTemp44 != 0 {fTemp42} else {fTemp43});
			let mut fTemp45: F32 = -1.0 + 2.0 * (if iTemp44 != 0 {fTemp42} else {fTemp42 + fTemp43 * (1.0 - self.fConst0 / fTemp41)});
			let mut fTemp46: F32 = (iTemp29) as F32;
			self.fVec2[0] = fTemp46;
			self.fRec23[0] = self.fConst6 * (4.656613e-10 * (fTemp46 + self.fVec2[1]) - self.fConst5 * self.fRec23[1]);
			let mut fTemp47: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec21[0] * F32::powf(2.0, 0.00083333335 * (4.5e+03 * self.fRec23[0] * self.fRec20[0] + self.fRec17[0]))));
			let mut fTemp48: F32 = self.fRec24[1] + self.fConst7 * fTemp47;
			let mut fTemp49: F32 = -1.0 + fTemp48;
			let mut iTemp50: i32 = (fTemp49 < 0.0) as i32;
			self.fRec24[0] = (if iTemp50 != 0 {fTemp48} else {fTemp49});
			let mut fTemp51: F32 = -1.0 + 2.0 * (if iTemp50 != 0 {fTemp48} else {fTemp48 + fTemp49 * (1.0 - self.fConst0 / fTemp47)});
			self.fRec25[0] = fSlow12 + self.fConst2 * self.fRec25[1];
			let mut fTemp52: F32 = (iTemp30) as F32;
			self.fVec3[0] = fTemp52;
			self.fRec26[0] = self.fConst6 * (4.656613e-10 * (fTemp52 + self.fVec3[1]) - self.fConst5 * self.fRec26[1]);
			self.fRec27[0] = fSlow13 + self.fConst2 * self.fRec27[1];
			self.fRec28[0] = fSlow14 + self.fConst2 * self.fRec28[1];
			let mut fTemp53: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec28[0] * F32::powf(2.0, 0.00083333335 * (4.5e+03 * self.fRec27[0] * self.fRec26[0] - self.fRec25[0]))));
			let mut fTemp54: F32 = self.fRec29[1] + self.fConst7 * fTemp53;
			let mut fTemp55: F32 = -1.0 + fTemp54;
			let mut iTemp56: i32 = (fTemp55 < 0.0) as i32;
			self.fRec29[0] = (if iTemp56 != 0 {fTemp54} else {fTemp55});
			let mut fTemp57: F32 = -1.0 + 2.0 * (if iTemp56 != 0 {fTemp54} else {fTemp54 + fTemp55 * (1.0 - self.fConst0 / fTemp53)});
			let mut fTemp58: F32 = (iTemp31) as F32;
			self.fVec4[0] = fTemp58;
			self.fRec30[0] = self.fConst6 * (4.656613e-10 * (fTemp58 + self.fVec4[1]) - self.fConst5 * self.fRec30[1]);
			let mut fTemp59: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec28[0] * F32::powf(2.0, 0.00083333335 * (4.5e+03 * self.fRec30[0] * self.fRec27[0] + self.fRec25[0]))));
			let mut fTemp60: F32 = self.fRec31[1] + self.fConst7 * fTemp59;
			let mut fTemp61: F32 = -1.0 + fTemp60;
			let mut iTemp62: i32 = (fTemp61 < 0.0) as i32;
			self.fRec31[0] = (if iTemp62 != 0 {fTemp60} else {fTemp61});
			let mut fTemp63: F32 = -1.0 + 2.0 * (if iTemp62 != 0 {fTemp60} else {fTemp60 + fTemp61 * (1.0 - self.fConst0 / fTemp59)});
			self.fRec32[0] = fSlow15 + self.fConst2 * self.fRec32[1];
			let mut fTemp64: F32 = (iTemp32) as F32;
			self.fVec5[0] = fTemp64;
			self.fRec33[0] = self.fConst6 * (4.656613e-10 * (fTemp64 + self.fVec5[1]) - self.fConst5 * self.fRec33[1]);
			self.fRec34[0] = fSlow16 + self.fConst2 * self.fRec34[1];
			self.fRec35[0] = fSlow17 + self.fConst2 * self.fRec35[1];
			let mut fTemp65: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec35[0] * F32::powf(2.0, 0.00083333335 * (4.5e+03 * self.fRec34[0] * self.fRec33[0] - self.fRec32[0]))));
			let mut fTemp66: F32 = self.fRec36[1] + self.fConst7 * fTemp65;
			let mut fTemp67: F32 = -1.0 + fTemp66;
			let mut iTemp68: i32 = (fTemp67 < 0.0) as i32;
			self.fRec36[0] = (if iTemp68 != 0 {fTemp66} else {fTemp67});
			let mut fTemp69: F32 = -1.0 + 2.0 * (if iTemp68 != 0 {fTemp66} else {fTemp66 + fTemp67 * (1.0 - self.fConst0 / fTemp65)});
			let mut fTemp70: F32 = (iTemp33) as F32;
			self.fVec6[0] = fTemp70;
			self.fRec37[0] = self.fConst6 * (4.656613e-10 * (fTemp70 + self.fVec6[1]) - self.fConst5 * self.fRec37[1]);
			let mut fTemp71: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec35[0] * F32::powf(2.0, 0.00083333335 * (4.5e+03 * self.fRec37[0] * self.fRec34[0] + self.fRec32[0]))));
			let mut fTemp72: F32 = self.fRec38[1] + self.fConst7 * fTemp71;
			let mut fTemp73: F32 = -1.0 + fTemp72;
			let mut iTemp74: i32 = (fTemp73 < 0.0) as i32;
			self.fRec38[0] = (if iTemp74 != 0 {fTemp72} else {fTemp73});
			let mut fTemp75: F32 = -1.0 + 2.0 * (if iTemp74 != 0 {fTemp72} else {fTemp72 + fTemp73 * (1.0 - self.fConst0 / fTemp71)});
			self.fRec39[0] = fSlow18 + self.fConst2 * self.fRec39[1];
			let mut fTemp76: F32 = (iTemp34) as F32;
			self.fVec7[0] = fTemp76;
			self.fRec40[0] = self.fConst6 * (4.656613e-10 * (fTemp76 + self.fVec7[1]) - self.fConst5 * self.fRec40[1]);
			self.fRec41[0] = fSlow19 + self.fConst2 * self.fRec41[1];
			self.fRec42[0] = fSlow20 + self.fConst2 * self.fRec42[1];
			let mut fTemp77: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec42[0] * F32::powf(2.0, 0.00083333335 * (4.5e+03 * self.fRec41[0] * self.fRec40[0] - self.fRec39[0]))));
			let mut fTemp78: F32 = self.fRec43[1] + self.fConst7 * fTemp77;
			let mut fTemp79: F32 = -1.0 + fTemp78;
			let mut iTemp80: i32 = (fTemp79 < 0.0) as i32;
			self.fRec43[0] = (if iTemp80 != 0 {fTemp78} else {fTemp79});
			let mut fTemp81: F32 = -1.0 + 2.0 * (if iTemp80 != 0 {fTemp78} else {fTemp78 + fTemp79 * (1.0 - self.fConst0 / fTemp77)});
			let mut fTemp82: F32 = (iTemp35) as F32;
			self.fVec8[0] = fTemp82;
			self.fRec44[0] = self.fConst6 * (4.656613e-10 * (fTemp82 + self.fVec8[1]) - self.fConst5 * self.fRec44[1]);
			let mut fTemp83: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec42[0] * F32::powf(2.0, 0.00083333335 * (4.5e+03 * self.fRec44[0] * self.fRec41[0] + self.fRec39[0]))));
			let mut fTemp84: F32 = self.fRec45[1] + self.fConst7 * fTemp83;
			let mut fTemp85: F32 = -1.0 + fTemp84;
			let mut iTemp86: i32 = (fTemp85 < 0.0) as i32;
			self.fRec45[0] = (if iTemp86 != 0 {fTemp84} else {fTemp85});
			let mut fTemp87: F32 = -1.0 + 2.0 * (if iTemp86 != 0 {fTemp84} else {fTemp84 + fTemp85 * (1.0 - self.fConst0 / fTemp83)});
			self.fRec46[0] = fSlow21 + self.fConst2 * self.fRec46[1];
			let mut fTemp88: F32 = (iTemp36) as F32;
			self.fVec9[0] = fTemp88;
			self.fRec47[0] = self.fConst6 * (4.656613e-10 * (fTemp88 + self.fVec9[1]) - self.fConst5 * self.fRec47[1]);
			self.fRec48[0] = fSlow22 + self.fConst2 * self.fRec48[1];
			self.fRec49[0] = fSlow23 + self.fConst2 * self.fRec49[1];
			let mut fTemp89: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec49[0] * F32::powf(2.0, 0.00083333335 * (4.5e+03 * self.fRec48[0] * self.fRec47[0] - self.fRec46[0]))));
			let mut fTemp90: F32 = self.fRec50[1] + self.fConst7 * fTemp89;
			let mut fTemp91: F32 = -1.0 + fTemp90;
			let mut iTemp92: i32 = (fTemp91 < 0.0) as i32;
			self.fRec50[0] = (if iTemp92 != 0 {fTemp90} else {fTemp91});
			let mut fTemp93: F32 = -1.0 + 2.0 * (if iTemp92 != 0 {fTemp90} else {fTemp90 + fTemp91 * (1.0 - self.fConst0 / fTemp89)});
			let mut fTemp94: F32 = (iTemp37) as F32;
			self.fVec10[0] = fTemp94;
			self.fRec51[0] = self.fConst6 * (4.656613e-10 * (fTemp94 + self.fVec10[1]) - self.fConst5 * self.fRec51[1]);
			let mut fTemp95: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec49[0] * F32::powf(2.0, 0.00083333335 * (4.5e+03 * self.fRec51[0] * self.fRec48[0] + self.fRec46[0]))));
			let mut fTemp96: F32 = self.fRec52[1] + self.fConst7 * fTemp95;
			let mut fTemp97: F32 = -1.0 + fTemp96;
			let mut iTemp98: i32 = (fTemp97 < 0.0) as i32;
			self.fRec52[0] = (if iTemp98 != 0 {fTemp96} else {fTemp97});
			let mut fTemp99: F32 = -1.0 + 2.0 * (if iTemp98 != 0 {fTemp96} else {fTemp96 + fTemp97 * (1.0 - self.fConst0 / fTemp95)});
			self.fRec53[0] = fSlow24 + self.fConst2 * self.fRec53[1];
			let mut fTemp100: F32 = (iTemp39) as F32;
			self.fVec11[0] = fTemp100;
			self.fRec54[0] = self.fConst6 * (4.656613e-10 * (fTemp100 + self.fVec11[1]) - self.fConst5 * self.fRec54[1]);
			self.fRec55[0] = fSlow25 + self.fConst2 * self.fRec55[1];
			self.fRec56[0] = fSlow26 + self.fConst2 * self.fRec56[1];
			let mut fTemp101: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec56[0] * F32::powf(2.0, 0.00083333335 * (4.5e+03 * self.fRec55[0] * self.fRec54[0] - self.fRec53[0]))));
			let mut fTemp102: F32 = self.fRec57[1] + self.fConst7 * fTemp101;
			let mut fTemp103: F32 = -1.0 + fTemp102;
			let mut iTemp104: i32 = (fTemp103 < 0.0) as i32;
			self.fRec57[0] = (if iTemp104 != 0 {fTemp102} else {fTemp103});
			let mut fTemp105: F32 = -1.0 + 2.0 * (if iTemp104 != 0 {fTemp102} else {fTemp102 + fTemp103 * (1.0 - self.fConst0 / fTemp101)});
			let mut fTemp106: F32 = (self.iRec19[0]) as F32;
			self.fVec12[0] = fTemp106;
			self.fRec58[0] = self.fConst6 * (4.656613e-10 * (fTemp106 + self.fVec12[1]) - self.fConst5 * self.fRec58[1]);
			let mut fTemp107: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec56[0] * F32::powf(2.0, 0.00083333335 * (4.5e+03 * self.fRec58[0] * self.fRec55[0] + self.fRec53[0]))));
			let mut fTemp108: F32 = self.fRec59[1] + self.fConst7 * fTemp107;
			let mut fTemp109: F32 = -1.0 + fTemp108;
			let mut iTemp110: i32 = (fTemp109 < 0.0) as i32;
			self.fRec59[0] = (if iTemp110 != 0 {fTemp108} else {fTemp109});
			let mut fTemp111: F32 = -1.0 + 2.0 * (if iTemp110 != 0 {fTemp108} else {fTemp108 + fTemp109 * (1.0 - self.fConst0 / fTemp107)});
			self.fRec60[0] = fSlow27 + self.fConst2 * self.fRec60[1];
			let mut fTemp112: F32 = (iTemp38) as F32;
			self.fVec13[0] = fTemp112;
			self.fRec63[0] = self.fConst6 * (4.656613e-10 * (fTemp112 + self.fVec13[1]) - self.fConst5 * self.fRec63[1]);
			let mut fTemp113: F32 = F32::powf(2.0, 3.75 * self.fRec55[0] * self.fRec63[0]);
			self.fRec61[0] = self.fRec61[1] - self.fConst3 * self.fRec56[0] * self.fRec62[1] * fTemp113;
			self.fRec62[0] = (i32::wrapping_sub(1, self.iVec0[1])) as F32 + self.fRec62[1] + self.fConst3 * self.fRec56[0] * fTemp113 * self.fRec61[0];
			let mut fTemp114: F32 = self.fRec62[0] * self.fRec60[0];
			let mut fTemp115: F32 = self.fRec16[0] * (fTemp114 + 0.39284748 * fTemp111 + 0.5879378 * fTemp105) + self.fRec15[0] * (0.39284748 * fTemp99 + 0.5879378 * fTemp93) + self.fRec14[0] * (0.19193739 * fTemp87 + 0.43776554 * fTemp81) + self.fRec13[0] * (0.5553029 * fTemp75 + 0.6805586 * fTemp69) + self.fRec12[0] * (0.05547896 * fTemp63 + 0.32101977 * fTemp57) + self.fRec11[0] * (0.6300368 * fTemp51 + 0.704927 * fTemp45);
			let mut fTemp116: F32 = F32::sqrt(F32::max(0.0, fTemp24));
			let mut fTemp117: F32 = fTemp116 * fTemp115 / fTemp27 - fTemp23 * self.fRec10[1];
			let mut fTemp118: F32 = F32::sqrt(F32::max(0.0, fTemp22));
			self.fRec9[0] = fTemp118 * fTemp117 - fTemp21 * self.fRec9[1];
			self.fRec10[0] = fTemp21 * fTemp117 + fTemp118 * self.fRec9[1];
			let mut fTemp119: F32 = 1.0 - fTemp20 / fTemp18;
			let mut fTemp120: F32 = 1.0 - fTemp19 - 2.0 * fTemp21 * fTemp119;
			let mut fTemp121: F32 = fTemp23 * fTemp115 / fTemp27 + fTemp116 * self.fRec10[1] + 2.0 * (fTemp119 * self.fRec10[0] / fTemp25) + fTemp120 * self.fRec9[0] / fTemp26;
			let mut fTemp122: F32 = F32::sqrt(F32::max(0.0, fTemp13));
			let mut fTemp123: F32 = fTemp122 * fTemp121 / fTemp18 - fTemp12 * self.fRec8[1];
			let mut fTemp124: F32 = F32::sqrt(F32::max(0.0, fTemp11));
			self.fRec7[0] = fTemp124 * fTemp123 - fTemp10 * self.fRec7[1];
			self.fRec8[0] = fTemp10 * fTemp123 + fTemp124 * self.fRec7[1];
			let mut fTemp125: F32 = 1.0 - fTemp9 / fTemp6;
			let mut fTemp126: F32 = 1.0 - fTemp7 - 2.0 * fTemp10 * fTemp125;
			let mut fTemp127: F32 = fTemp12 * fTemp121 / fTemp18 + fTemp122 * self.fRec8[1] + 2.0 * (fTemp125 * self.fRec8[0] / fTemp14) + fTemp126 * self.fRec7[0] / fTemp15;
			let mut fTemp128: F32 = self.fRec16[0] * (0.5879378 * fTemp111 + 0.39284748 * fTemp105 + 1.0 * fTemp114) + self.fRec15[0] * (0.5879378 * fTemp99 + 0.39284748 * fTemp93) + self.fRec14[0] * (0.6805586 * fTemp87 + 0.5553029 * fTemp81) + self.fRec13[0] * (0.43776554 * fTemp75 + 0.19193739 * fTemp69) + self.fRec12[0] * (0.704927 * fTemp63 + 0.6300368 * fTemp57) + self.fRec11[0] * (0.32101977 * fTemp51 + 0.05547896 * fTemp45);
			let mut fTemp129: F32 = fTemp128 * fTemp116 / fTemp27 - fTemp23 * self.fRec67[1];
			self.fRec66[0] = fTemp129 * fTemp118 - self.fRec66[1] * fTemp21;
			self.fRec67[0] = fTemp21 * fTemp129 + self.fRec66[1] * fTemp118;
			let mut fTemp130: F32 = fTemp128 * fTemp23 / fTemp27 + self.fRec67[1] * fTemp116 + 2.0 * (self.fRec67[0] * fTemp119 / fTemp25) + self.fRec66[0] * fTemp120 / fTemp26;
			let mut fTemp131: F32 = fTemp130 * fTemp122 / fTemp18 - fTemp12 * self.fRec65[1];
			self.fRec64[0] = fTemp131 * fTemp124 - self.fRec64[1] * fTemp10;
			self.fRec65[0] = fTemp10 * fTemp131 + self.fRec64[1] * fTemp124;
			let mut fTemp132: F32 = fTemp130 * fTemp12 / fTemp18 + self.fRec65[1] * fTemp122 + 2.0 * (self.fRec65[0] * fTemp125 / fTemp14) + self.fRec64[0] * fTemp126 / fTemp15;
			self.fRec68[0] = fSlow28 + self.fConst2 * self.fRec68[1];
			let mut fTemp133: F32 = 1.0 - self.fRec68[0];
			self.fRec69[0] = -(self.fConst10 * (self.fConst9 * self.fRec69[1] - (self.fRec106[1] + self.fRec106[2])));
			self.fRec70[0] = -(self.fConst10 * (self.fConst9 * self.fRec70[1] - (self.fRec107[1] + self.fRec107[2])));
			self.fRec71[0] = -(self.fConst10 * (self.fConst9 * self.fRec71[1] - (self.fRec108[1] + self.fRec108[2])));
			self.fRec72[0] = -(self.fConst10 * (self.fConst9 * self.fRec72[1] - (self.fRec105[1] + self.fRec105[2])));
			self.fRec73[0] = -(self.fConst10 * (self.fConst9 * self.fRec73[1] - (self.fRec104[1] + self.fRec104[2])));
			self.fRec74[0] = -(self.fConst10 * (self.fConst9 * self.fRec74[1] - (self.fRec101[1] + self.fRec101[2])));
			self.fRec75[0] = -(self.fConst10 * (self.fConst9 * self.fRec75[1] - (self.fRec103[1] + self.fRec103[2])));
			self.fRec76[0] = -(self.fConst10 * (self.fConst9 * self.fRec76[1] - (self.fRec102[1] + self.fRec102[2])));
			self.fRec77[0] = self.fConst22 * (self.fRec102[1] + self.fConst21 * self.fRec76[0]) + self.fConst20 * self.fRec77[1];
			self.fRec78[0] = self.fConst33 * (self.fRec107[1] + self.fConst32 * self.fRec70[0]) + self.fConst31 * self.fRec78[1];
			let mut fTemp134: F32 = 0.3 * ((*input2) as F32 + 0.9 * (fTemp132 * self.fRec68[0] / fTemp6));
			self.fVec14[(self.IOTA0 & 16383) as usize] = 1e-20 + 0.35355338 * self.fRec77[0];
			let mut fTemp135: F32 = self.fVec14[((i32::wrapping_sub(self.IOTA0, self.iConst35)) & 16383) as usize] - (0.6 * self.fRec87[1] + fTemp134);
			self.fVec15[(self.IOTA0 & 2047) as usize] = fTemp135;
			let mut fRec79: F32 = 0.6 * fTemp135;
			self.fRec80[0] = self.fConst46 * (self.fRec106[1] + self.fConst45 * self.fRec69[0]) + self.fConst44 * self.fRec80[1];
			self.fRec81[0] = self.fConst57 * (self.fRec104[1] + self.fConst56 * self.fRec73[0]) + self.fConst55 * self.fRec81[1];
			self.fVec16[(self.IOTA0 & 16383) as usize] = 1e-20 + 0.35355338 * self.fRec78[0];
			let mut fTemp136: F32 = fTemp134 + self.fVec16[((i32::wrapping_sub(self.IOTA0, self.iConst59)) & 16383) as usize] - 0.6 * self.fRec85[1];
			self.fVec17[(self.IOTA0 & 2047) as usize] = fTemp136;
			let mut fRec82: F32 = 0.6 * fTemp136;
			self.fRec83[0] = self.fConst70 * (self.fRec101[1] + self.fConst69 * self.fRec74[0]) + self.fConst68 * self.fRec83[1];
			self.fRec84[0] = self.fConst81 * (self.fRec103[1] + self.fConst80 * self.fRec75[0]) + self.fConst79 * self.fRec84[1];
			self.fRec85[0] = self.fVec17[((i32::wrapping_sub(self.IOTA0, self.iConst82)) & 2047) as usize];
			self.fVec18[(self.IOTA0 & 16383) as usize] = 1e-20 + 0.35355338 * self.fRec80[0];
			let mut fTemp137: F32 = self.fVec18[((i32::wrapping_sub(self.IOTA0, self.iConst84)) & 16383) as usize] - (fTemp134 + 0.6 * self.fRec88[1]);
			self.fVec19[(self.IOTA0 & 4095) as usize] = fTemp137;
			let mut fRec86: F32 = 0.6 * fTemp137;
			self.fRec87[0] = self.fVec15[((i32::wrapping_sub(self.IOTA0, self.iConst85)) & 2047) as usize];
			self.fRec88[0] = self.fVec19[((i32::wrapping_sub(self.IOTA0, self.iConst86)) & 4095) as usize];
			self.fRec89[0] = self.fConst97 * (self.fRec108[1] + self.fConst96 * self.fRec71[0]) + self.fConst95 * self.fRec89[1];
			self.fRec90[0] = self.fConst108 * (self.fRec105[1] + self.fConst107 * self.fRec72[0]) + self.fConst106 * self.fRec90[1];
			let mut fTemp138: F32 = 0.3 * ((*input3) as F32 + 0.9 * (self.fRec68[0] * fTemp127 / fTemp6));
			self.fVec20[(self.IOTA0 & 32767) as usize] = 1e-20 + 0.35355338 * self.fRec83[0];
			let mut fTemp139: F32 = 0.6 * self.fRec91[1] + self.fVec20[((i32::wrapping_sub(self.IOTA0, self.iConst110)) & 32767) as usize];
			self.fVec21[(self.IOTA0 & 2047) as usize] = fTemp139 - fTemp138;
			self.fRec91[0] = self.fVec21[((i32::wrapping_sub(self.IOTA0, self.iConst111)) & 2047) as usize];
			self.fVec22[(self.IOTA0 & 16383) as usize] = 1e-20 + 0.35355338 * self.fRec81[0];
			let mut fTemp140: F32 = self.fVec22[((i32::wrapping_sub(self.IOTA0, self.iConst113)) & 16383) as usize] + fTemp138 + 0.6 * self.fRec93[1];
			self.fVec23[(self.IOTA0 & 4095) as usize] = fTemp140;
			let mut fRec92: F32 = -(0.6 * fTemp140);
			self.fRec93[0] = self.fVec23[((i32::wrapping_sub(self.IOTA0, self.iConst114)) & 4095) as usize];
			let mut fRec94: F32 = 0.6 * (fTemp138 - fTemp139);
			self.fVec24[(self.IOTA0 & 16383) as usize] = 1e-20 + 0.35355338 * self.fRec84[0];
			let mut fTemp141: F32 = fTemp134 + self.fVec24[((i32::wrapping_sub(self.IOTA0, self.iConst116)) & 16383) as usize] - 0.6 * self.fRec95[1];
			self.fVec25[(self.IOTA0 & 4095) as usize] = fTemp141;
			self.fRec95[0] = self.fVec25[((i32::wrapping_sub(self.IOTA0, self.iConst117)) & 4095) as usize];
			self.fVec26[(self.IOTA0 & 32767) as usize] = 1e-20 + 0.35355338 * self.fRec90[0];
			let mut fTemp142: F32 = 0.6 * self.fRec96[1] + self.fVec26[((i32::wrapping_sub(self.IOTA0, self.iConst119)) & 32767) as usize];
			self.fVec27[(self.IOTA0 & 4095) as usize] = fTemp142 - fTemp138;
			self.fRec96[0] = self.fVec27[((i32::wrapping_sub(self.IOTA0, self.iConst120)) & 4095) as usize];
			self.fVec28[(self.IOTA0 & 32767) as usize] = 1e-20 + 0.35355338 * self.fRec89[0];
			let mut fTemp143: F32 = self.fVec28[((i32::wrapping_sub(self.IOTA0, self.iConst122)) & 32767) as usize] + fTemp138 + 0.6 * self.fRec97[1];
			self.fVec29[(self.IOTA0 & 4095) as usize] = fTemp143;
			self.fRec97[0] = self.fVec29[((i32::wrapping_sub(self.IOTA0, self.iConst123)) & 4095) as usize];
			let mut fRec98: F32 = 0.6 * fTemp141;
			let mut fRec99: F32 = 0.6 * (fTemp138 - fTemp142);
			let mut fRec100: F32 = -(0.6 * fTemp143);
			let mut fTemp144: F32 = self.fRec87[1] + self.fRec85[1];
			self.fRec101[0] = fRec99 + fRec92 + fRec79 + fRec82 + self.fRec96[1] + self.fRec93[1] + fTemp144 - (fRec94 + fRec100 + fRec86 + fRec98 + self.fRec97[1] + self.fRec95[1] + self.fRec88[1] + self.fRec91[1]);
			self.fRec102[0] = fRec94 + fRec100 + fRec79 + fRec82 + self.fRec97[1] + self.fRec91[1] + fTemp144 - (fRec99 + fRec92 + fRec86 + fRec98 + self.fRec96[1] + self.fRec95[1] + self.fRec88[1] + self.fRec93[1]);
			let mut fTemp145: F32 = self.fRec85[1] + self.fRec88[1];
			self.fRec103[0] = fRec99 + fRec100 + fRec86 + fRec82 + self.fRec97[1] + self.fRec96[1] + fTemp145 - (fRec94 + fRec92 + fRec79 + fRec98 + self.fRec95[1] + self.fRec93[1] + self.fRec87[1] + self.fRec91[1]);
			self.fRec104[0] = fRec94 + fRec92 + fRec86 + fRec82 + self.fRec93[1] + self.fRec91[1] + fTemp145 - (fRec99 + fRec100 + fRec79 + fRec98 + self.fRec97[1] + self.fRec96[1] + self.fRec87[1] + self.fRec95[1]);
			let mut fTemp146: F32 = self.fRec87[1] + self.fRec88[1];
			self.fRec105[0] = fRec94 + fRec99 + fRec98 + fRec82 + self.fRec96[1] + self.fRec95[1] + self.fRec85[1] + self.fRec91[1] - (fRec92 + fRec100 + fRec79 + fRec86 + self.fRec97[1] + self.fRec93[1] + fTemp146);
			self.fRec106[0] = fRec92 + fRec100 + fRec98 + fRec82 + self.fRec97[1] + self.fRec95[1] + self.fRec85[1] + self.fRec93[1] - (fRec94 + fRec99 + fRec79 + fRec86 + self.fRec96[1] + self.fRec91[1] + fTemp146);
			let mut fTemp147: F32 = self.fRec88[1] + fTemp144;
			self.fRec107[0] = fRec94 + fRec99 + fRec92 + fRec100 + fRec79 + fRec86 + fRec98 + fRec82 + self.fRec97[1] + self.fRec96[1] + self.fRec95[1] + self.fRec93[1] + self.fRec91[1] + fTemp147;
			self.fRec108[0] = fRec79 + fRec86 + fRec98 + fRec82 + self.fRec95[1] + fTemp147 - (fRec94 + fRec99 + fRec92 + fRec100 + self.fRec97[1] + self.fRec96[1] + self.fRec91[1] + self.fRec93[1]);
			let mut fTemp148: F32 = self.fRec108[0] - self.fRec106[0];
			let mut fTemp149: F32 = self.fRec108[0] + self.fRec106[0];
			let mut fTemp150: F32 = (*input1) as F32;
			let mut fTemp151: F32 = (*input0) as F32;
			let mut fTemp152: F32 = self.fRec4[1] + self.fConst124 * (0.5 * (fTemp151 + fTemp150 + 0.37 * (fTemp149 + fTemp148) + 0.9 * (fTemp133 * (fTemp132 + fTemp127) / fTemp6)) - self.fRec3[1]);
			let mut fTemp153: F32 = self.fRec3[1] + self.fConst126 * fTemp152;
			self.fRec3[0] = 2.0 * fTemp153 - self.fRec3[1];
			self.fRec4[0] = self.fConst127 * fTemp152 - self.fRec4[1];
			let mut fTemp154: F32 = self.fRec2[1] + self.fConst124 * (fTemp153 - self.fRec1[1]);
			self.fRec1[0] = 2.0 * (self.fRec1[1] + self.fConst126 * fTemp154) - self.fRec1[1];
			self.fRec2[0] = self.fConst127 * fTemp154 - self.fRec2[1];
			let mut fTemp155: F32 = 0.9 * (fTemp132 * fTemp133 / fTemp6) + 0.37 * fTemp149;
			let mut fTemp156: F32 = self.fRec109[1] + self.fConst124 * (fTemp151 - self.fRec110[1] + fTemp155);
			self.fRec109[0] = self.fConst127 * fTemp156 - self.fRec109[1];
			self.fRec110[0] = 2.0 * (self.fRec110[1] + self.fConst126 * fTemp156) - self.fRec110[1];
			let mut fTemp157: F32 = fTemp151 - (self.fRec110[1] + self.fRec111[1]);
			let mut fTemp158: F32 = self.fRec112[1] + self.fConst124 * (fTemp157 + fTemp155 + self.fConst129 * fTemp156);
			self.fRec111[0] = 2.0 * (self.fRec111[1] + self.fConst126 * fTemp158) - self.fRec111[1];
			self.fRec112[0] = self.fConst127 * fTemp158 - self.fRec112[1];
			*output0 = (F32::tanh((fTemp157 + self.fConst130 * (self.fConst128 * fTemp156 - 1.4142135 * fTemp158 - self.fConst124 * (fTemp158 - fTemp154)) + fTemp155 + self.fRec1[1]) * self.fRec0[0])) as FaustFloat;
			let mut fTemp159: F32 = 0.9 * (fTemp133 * fTemp127 / fTemp6) + 0.37 * fTemp148;
			let mut fTemp160: F32 = self.fRec114[1] + self.fConst124 * (fTemp150 - self.fRec113[1] + fTemp159);
			self.fRec113[0] = 2.0 * (self.fRec113[1] + self.fConst126 * fTemp160) - self.fRec113[1];
			self.fRec114[0] = self.fConst127 * fTemp160 - self.fRec114[1];
			let mut fTemp161: F32 = fTemp150 - (self.fRec113[1] + self.fRec115[1]);
			let mut fTemp162: F32 = self.fRec116[1] + self.fConst124 * (fTemp161 + fTemp159 + self.fConst129 * fTemp160);
			self.fRec115[0] = 2.0 * (self.fRec115[1] + self.fConst126 * fTemp162) - self.fRec115[1];
			self.fRec116[0] = self.fConst127 * fTemp162 - self.fRec116[1];
			*output1 = (F32::tanh(self.fRec0[0] * (fTemp161 + self.fConst130 * (self.fConst128 * fTemp160 - 1.4142135 * fTemp162 + self.fConst124 * (fTemp154 - fTemp162)) + fTemp159 + self.fRec1[1]))) as FaustFloat;
			self.iVec0[1] = self.iVec0[0];
			self.fRec0[1] = self.fRec0[0];
			self.fRec5[1] = self.fRec5[0];
			self.fRec6[1] = self.fRec6[0];
			self.fRec11[1] = self.fRec11[0];
			self.fRec12[1] = self.fRec12[0];
			self.fRec13[1] = self.fRec13[0];
			self.fRec14[1] = self.fRec14[0];
			self.fRec15[1] = self.fRec15[0];
			self.fRec16[1] = self.fRec16[0];
			self.fRec17[1] = self.fRec17[0];
			self.iRec19[1] = self.iRec19[0];
			self.fVec1[1] = self.fVec1[0];
			self.fRec18[1] = self.fRec18[0];
			self.fRec20[1] = self.fRec20[0];
			self.fRec21[1] = self.fRec21[0];
			self.fRec22[1] = self.fRec22[0];
			self.fVec2[1] = self.fVec2[0];
			self.fRec23[1] = self.fRec23[0];
			self.fRec24[1] = self.fRec24[0];
			self.fRec25[1] = self.fRec25[0];
			self.fVec3[1] = self.fVec3[0];
			self.fRec26[1] = self.fRec26[0];
			self.fRec27[1] = self.fRec27[0];
			self.fRec28[1] = self.fRec28[0];
			self.fRec29[1] = self.fRec29[0];
			self.fVec4[1] = self.fVec4[0];
			self.fRec30[1] = self.fRec30[0];
			self.fRec31[1] = self.fRec31[0];
			self.fRec32[1] = self.fRec32[0];
			self.fVec5[1] = self.fVec5[0];
			self.fRec33[1] = self.fRec33[0];
			self.fRec34[1] = self.fRec34[0];
			self.fRec35[1] = self.fRec35[0];
			self.fRec36[1] = self.fRec36[0];
			self.fVec6[1] = self.fVec6[0];
			self.fRec37[1] = self.fRec37[0];
			self.fRec38[1] = self.fRec38[0];
			self.fRec39[1] = self.fRec39[0];
			self.fVec7[1] = self.fVec7[0];
			self.fRec40[1] = self.fRec40[0];
			self.fRec41[1] = self.fRec41[0];
			self.fRec42[1] = self.fRec42[0];
			self.fRec43[1] = self.fRec43[0];
			self.fVec8[1] = self.fVec8[0];
			self.fRec44[1] = self.fRec44[0];
			self.fRec45[1] = self.fRec45[0];
			self.fRec46[1] = self.fRec46[0];
			self.fVec9[1] = self.fVec9[0];
			self.fRec47[1] = self.fRec47[0];
			self.fRec48[1] = self.fRec48[0];
			self.fRec49[1] = self.fRec49[0];
			self.fRec50[1] = self.fRec50[0];
			self.fVec10[1] = self.fVec10[0];
			self.fRec51[1] = self.fRec51[0];
			self.fRec52[1] = self.fRec52[0];
			self.fRec53[1] = self.fRec53[0];
			self.fVec11[1] = self.fVec11[0];
			self.fRec54[1] = self.fRec54[0];
			self.fRec55[1] = self.fRec55[0];
			self.fRec56[1] = self.fRec56[0];
			self.fRec57[1] = self.fRec57[0];
			self.fVec12[1] = self.fVec12[0];
			self.fRec58[1] = self.fRec58[0];
			self.fRec59[1] = self.fRec59[0];
			self.fRec60[1] = self.fRec60[0];
			self.fVec13[1] = self.fVec13[0];
			self.fRec63[1] = self.fRec63[0];
			self.fRec61[1] = self.fRec61[0];
			self.fRec62[1] = self.fRec62[0];
			self.fRec9[1] = self.fRec9[0];
			self.fRec10[1] = self.fRec10[0];
			self.fRec7[1] = self.fRec7[0];
			self.fRec8[1] = self.fRec8[0];
			self.fRec66[1] = self.fRec66[0];
			self.fRec67[1] = self.fRec67[0];
			self.fRec64[1] = self.fRec64[0];
			self.fRec65[1] = self.fRec65[0];
			self.fRec68[1] = self.fRec68[0];
			self.fRec69[1] = self.fRec69[0];
			self.fRec70[1] = self.fRec70[0];
			self.fRec71[1] = self.fRec71[0];
			self.fRec72[1] = self.fRec72[0];
			self.fRec73[1] = self.fRec73[0];
			self.fRec74[1] = self.fRec74[0];
			self.fRec75[1] = self.fRec75[0];
			self.fRec76[1] = self.fRec76[0];
			self.fRec77[1] = self.fRec77[0];
			self.fRec78[1] = self.fRec78[0];
			self.IOTA0 = i32::wrapping_add(self.IOTA0, 1);
			self.fRec80[1] = self.fRec80[0];
			self.fRec81[1] = self.fRec81[0];
			self.fRec83[1] = self.fRec83[0];
			self.fRec84[1] = self.fRec84[0];
			self.fRec85[1] = self.fRec85[0];
			self.fRec87[1] = self.fRec87[0];
			self.fRec88[1] = self.fRec88[0];
			self.fRec89[1] = self.fRec89[0];
			self.fRec90[1] = self.fRec90[0];
			self.fRec91[1] = self.fRec91[0];
			self.fRec93[1] = self.fRec93[0];
			self.fRec95[1] = self.fRec95[0];
			self.fRec96[1] = self.fRec96[0];
			self.fRec97[1] = self.fRec97[0];
			self.fRec101[2] = self.fRec101[1];
			self.fRec101[1] = self.fRec101[0];
			self.fRec102[2] = self.fRec102[1];
			self.fRec102[1] = self.fRec102[0];
			self.fRec103[2] = self.fRec103[1];
			self.fRec103[1] = self.fRec103[0];
			self.fRec104[2] = self.fRec104[1];
			self.fRec104[1] = self.fRec104[0];
			self.fRec105[2] = self.fRec105[1];
			self.fRec105[1] = self.fRec105[0];
			self.fRec106[2] = self.fRec106[1];
			self.fRec106[1] = self.fRec106[0];
			self.fRec107[2] = self.fRec107[1];
			self.fRec107[1] = self.fRec107[0];
			self.fRec108[2] = self.fRec108[1];
			self.fRec108[1] = self.fRec108[0];
			self.fRec3[1] = self.fRec3[0];
			self.fRec4[1] = self.fRec4[0];
			self.fRec1[1] = self.fRec1[0];
			self.fRec2[1] = self.fRec2[0];
			self.fRec109[1] = self.fRec109[0];
			self.fRec110[1] = self.fRec110[0];
			self.fRec111[1] = self.fRec111[0];
			self.fRec112[1] = self.fRec112[0];
			self.fRec113[1] = self.fRec113[0];
			self.fRec114[1] = self.fRec114[0];
			self.fRec115[1] = self.fRec115[0];
			self.fRec116[1] = self.fRec116[0];
		}
		
	}

}

#[cfg(not(target_arch = "wasm32"))] // Compile ffi bindings only on non-wasm targets
mod ffi {
	use core::ffi::c_float;
	// Conditionally compile the link attribute only on non-Windows platforms
	#[cfg_attr(not(target_os = "windows"), link(name = "m"))]
	unsafe extern "C" {
		pub fn remainderf(from: c_float, to: c_float) -> c_float;
		pub fn rintf(val: c_float) -> c_float;
	}
}
fn remainderf(from: f32, to: f32) -> f32 {
	#[cfg(not(target_arch = "wasm32"))] // non-wasm targets use ffi bindings
	unsafe { ffi::remainderf(from, to) }
	#[cfg(target_arch = "wasm32")] // wasm relies on libm
	libm::remainderf(from, to)
}
fn rintf(val: f32) -> f32 {
	#[cfg(not(target_arch = "wasm32"))] // non-wasm targets use ffi bindings
	unsafe { ffi::rintf(val) }
	#[cfg(target_arch = "wasm32")] // wasm relies on libm
	libm::rintf(val)
}

impl FaustDsp for MoodscoreDsp {
	type T = FaustFloat;
	fn new() -> Self where Self: Sized {
		Self::new()
	}
	fn metadata(&self, m: &mut dyn Meta) {
		self.metadata(m)
	}
	fn get_sample_rate(&self) -> i32 {
		self.get_sample_rate()
	}
	fn get_num_inputs(&self) -> i32 {
		FAUST_INPUTS as i32
	}
	fn get_num_outputs(&self) -> i32 {
		FAUST_OUTPUTS as i32
	}
	fn class_init(sample_rate: i32) where Self: Sized {
		Self::class_init(sample_rate);
	}
	fn instance_reset_params(&mut self) {
		self.instance_reset_params()
	}
	fn instance_clear(&mut self) {
		self.instance_clear()
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.instance_constants(sample_rate)
	}
	fn instance_init(&mut self, sample_rate: i32) {
		self.instance_init(sample_rate)
	}
	fn init(&mut self, sample_rate: i32) {
		self.init(sample_rate)
	}
	fn build_user_interface(&self, ui_interface: &mut dyn UI<Self::T>) {
		self.build_user_interface(ui_interface)
	}
	fn build_user_interface_static(ui_interface: &mut dyn UI<Self::T>) where Self: Sized {
		Self::build_user_interface_static(ui_interface);
	}
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		self.get_param(param)
	}
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		self.set_param(param, value)
	}
	fn compute(&mut self, count: i32, inputs: &[&[Self::T]], outputs: &mut [&mut [Self::T]]) {
		self.compute(count as usize, inputs, outputs)
	}
}
