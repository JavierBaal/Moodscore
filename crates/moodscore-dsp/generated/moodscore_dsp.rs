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
	fConst3: F32,
	fConst4: F32,
	fConst5: F32,
	fRec1: [F32;2],
	fRec2: [F32;2],
	fRec3: [F32;2],
	fRec4: [F32;2],
	fRec5: [F32;2],
	fRec6: [F32;2],
	fRec7: [F32;2],
	fRec8: [F32;2],
	fConst6: F32,
	fConst7: F32,
	fConst8: F32,
	fConst9: F32,
	fConst10: F32,
	fConst11: F32,
	fConst12: F32,
	fConst13: F32,
	fConst14: F32,
	fConst15: F32,
	fConst16: F32,
	fConst17: F32,
	fRec9: [F32;2],
	fConst18: F32,
	fConst19: F32,
	fConst20: F32,
	fConst21: F32,
	fConst22: F32,
	fConst23: F32,
	fConst24: F32,
	fConst25: F32,
	fConst26: F32,
	fConst27: F32,
	fConst28: F32,
	fRec10: [F32;2],
	fHslider1: FaustFloat,
	fRec41: [F32;2],
	fConst29: F32,
	fHslider2: FaustFloat,
	fRec42: [F32;2],
	fHslider3: FaustFloat,
	fRec47: [F32;2],
	fHslider4: FaustFloat,
	fRec48: [F32;2],
	fHslider5: FaustFloat,
	fRec49: [F32;2],
	fHslider6: FaustFloat,
	fRec50: [F32;2],
	fHslider7: FaustFloat,
	fRec51: [F32;2],
	fHslider8: FaustFloat,
	fRec52: [F32;2],
	fHslider9: FaustFloat,
	fRec53: [F32;2],
	fConst30: F32,
	fConst31: F32,
	iRec55: [i32;2],
	fVec1: [F32;2],
	fConst32: F32,
	fRec54: [F32;2],
	fHslider10: FaustFloat,
	fRec56: [F32;2],
	fHslider11: FaustFloat,
	fRec57: [F32;2],
	fConst33: F32,
	fRec58: [F32;2],
	fVec2: [F32;2],
	fRec59: [F32;2],
	fRec60: [F32;2],
	fHslider12: FaustFloat,
	fRec61: [F32;2],
	fVec3: [F32;2],
	fRec62: [F32;2],
	fHslider13: FaustFloat,
	fRec63: [F32;2],
	fHslider14: FaustFloat,
	fRec64: [F32;2],
	fRec65: [F32;2],
	fVec4: [F32;2],
	fRec66: [F32;2],
	fRec67: [F32;2],
	fHslider15: FaustFloat,
	fRec68: [F32;2],
	fVec5: [F32;2],
	fRec69: [F32;2],
	fHslider16: FaustFloat,
	fRec70: [F32;2],
	fHslider17: FaustFloat,
	fRec71: [F32;2],
	fRec72: [F32;2],
	fVec6: [F32;2],
	fRec73: [F32;2],
	fRec74: [F32;2],
	fHslider18: FaustFloat,
	fRec75: [F32;2],
	fVec7: [F32;2],
	fRec76: [F32;2],
	fHslider19: FaustFloat,
	fRec77: [F32;2],
	fHslider20: FaustFloat,
	fRec78: [F32;2],
	fRec79: [F32;2],
	fVec8: [F32;2],
	fRec80: [F32;2],
	fRec81: [F32;2],
	fHslider21: FaustFloat,
	fRec82: [F32;2],
	fVec9: [F32;2],
	fRec83: [F32;2],
	fHslider22: FaustFloat,
	fRec84: [F32;2],
	fHslider23: FaustFloat,
	fRec85: [F32;2],
	fRec86: [F32;2],
	fVec10: [F32;2],
	fRec87: [F32;2],
	fRec88: [F32;2],
	fHslider24: FaustFloat,
	fRec89: [F32;2],
	fVec11: [F32;2],
	fRec90: [F32;2],
	fHslider25: FaustFloat,
	fRec91: [F32;2],
	fHslider26: FaustFloat,
	fRec92: [F32;2],
	fRec93: [F32;2],
	fVec12: [F32;2],
	fRec94: [F32;2],
	fRec95: [F32;2],
	fHslider27: FaustFloat,
	fRec96: [F32;2],
	fVec13: [F32;2],
	fRec99: [F32;2],
	fRec97: [F32;2],
	fRec98: [F32;2],
	fRec45: [F32;2],
	fRec46: [F32;2],
	fRec43: [F32;2],
	fRec44: [F32;2],
	IOTA0: i32,
	fVec14: [F32;16384],
	fConst34: F32,
	iConst35: i32,
	fVec15: [F32;2048],
	fVec16: [F32;16384],
	fConst36: F32,
	iConst37: i32,
	fVec17: [F32;2048],
	iConst38: i32,
	fRec13: [F32;2],
	fConst39: F32,
	fConst40: F32,
	fConst41: F32,
	fConst42: F32,
	fConst43: F32,
	fConst44: F32,
	fConst45: F32,
	fConst46: F32,
	fConst47: F32,
	fConst48: F32,
	fConst49: F32,
	fRec14: [F32;2],
	fConst50: F32,
	fConst51: F32,
	fConst52: F32,
	fConst53: F32,
	fConst54: F32,
	fConst55: F32,
	fConst56: F32,
	fConst57: F32,
	fConst58: F32,
	fConst59: F32,
	fConst60: F32,
	fRec15: [F32;2],
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
	fConst71: F32,
	fRec16: [F32;2],
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
	fConst82: F32,
	fRec17: [F32;2],
	iConst83: i32,
	fRec18: [F32;2],
	fVec18: [F32;32768],
	fConst84: F32,
	iConst85: i32,
	fRec102: [F32;2],
	fRec103: [F32;2],
	fRec100: [F32;2],
	fRec101: [F32;2],
	fVec19: [F32;32],
	fVec20: [F32;16384],
	fConst86: F32,
	iConst87: i32,
	fVec21: [F32;4096],
	iConst88: i32,
	fRec20: [F32;2],
	fConst89: F32,
	fConst90: F32,
	fConst91: F32,
	fConst92: F32,
	fConst93: F32,
	fConst94: F32,
	fConst95: F32,
	fConst96: F32,
	fConst97: F32,
	fConst98: F32,
	fConst99: F32,
	fRec21: [F32;2],
	fVec22: [F32;16384],
	fConst100: F32,
	iConst101: i32,
	fVec23: [F32;4096],
	fVec24: [F32;16384],
	fConst102: F32,
	iConst103: i32,
	fVec25: [F32;4096],
	fConst104: F32,
	fConst105: F32,
	fConst106: F32,
	fConst107: F32,
	fConst108: F32,
	fConst109: F32,
	fConst110: F32,
	fConst111: F32,
	fConst112: F32,
	fConst113: F32,
	fConst114: F32,
	fRec24: [F32;2],
	fVec26: [F32;32768],
	fConst115: F32,
	iConst116: i32,
	fVec27: [F32;4096],
	iConst117: i32,
	fRec25: [F32;2],
	iConst118: i32,
	fRec27: [F32;2],
	iConst119: i32,
	fRec28: [F32;2],
	fVec28: [F32;2048],
	iConst120: i32,
	fRec30: [F32;2],
	fVec29: [F32;32768],
	fConst121: F32,
	iConst122: i32,
	fVec30: [F32;4096],
	iConst123: i32,
	fRec32: [F32;2],
	fRec33: [F32;3],
	fRec34: [F32;3],
	fRec35: [F32;3],
	fRec36: [F32;3],
	fRec37: [F32;3],
	fRec38: [F32;3],
	fRec39: [F32;3],
	fRec40: [F32;3],
	fHslider28: FaustFloat,
	fRec104: [F32;2],
}


fn MoodscoreDsp_faustpower2_f(value: F32) -> F32 {
	return value * value;
}
pub const FAUST_INPUTS: usize = 0;
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
			fConst3: 0.0,
			fConst4: 0.0,
			fConst5: 0.0,
			fRec1: [0.0;2],
			fRec2: [0.0;2],
			fRec3: [0.0;2],
			fRec4: [0.0;2],
			fRec5: [0.0;2],
			fRec6: [0.0;2],
			fRec7: [0.0;2],
			fRec8: [0.0;2],
			fConst6: 0.0,
			fConst7: 0.0,
			fConst8: 0.0,
			fConst9: 0.0,
			fConst10: 0.0,
			fConst11: 0.0,
			fConst12: 0.0,
			fConst13: 0.0,
			fConst14: 0.0,
			fConst15: 0.0,
			fConst16: 0.0,
			fConst17: 0.0,
			fRec9: [0.0;2],
			fConst18: 0.0,
			fConst19: 0.0,
			fConst20: 0.0,
			fConst21: 0.0,
			fConst22: 0.0,
			fConst23: 0.0,
			fConst24: 0.0,
			fConst25: 0.0,
			fConst26: 0.0,
			fConst27: 0.0,
			fConst28: 0.0,
			fRec10: [0.0;2],
			fHslider1: 0.0,
			fRec41: [0.0;2],
			fConst29: 0.0,
			fHslider2: 0.0,
			fRec42: [0.0;2],
			fHslider3: 0.0,
			fRec47: [0.0;2],
			fHslider4: 0.0,
			fRec48: [0.0;2],
			fHslider5: 0.0,
			fRec49: [0.0;2],
			fHslider6: 0.0,
			fRec50: [0.0;2],
			fHslider7: 0.0,
			fRec51: [0.0;2],
			fHslider8: 0.0,
			fRec52: [0.0;2],
			fHslider9: 0.0,
			fRec53: [0.0;2],
			fConst30: 0.0,
			fConst31: 0.0,
			iRec55: [0;2],
			fVec1: [0.0;2],
			fConst32: 0.0,
			fRec54: [0.0;2],
			fHslider10: 0.0,
			fRec56: [0.0;2],
			fHslider11: 0.0,
			fRec57: [0.0;2],
			fConst33: 0.0,
			fRec58: [0.0;2],
			fVec2: [0.0;2],
			fRec59: [0.0;2],
			fRec60: [0.0;2],
			fHslider12: 0.0,
			fRec61: [0.0;2],
			fVec3: [0.0;2],
			fRec62: [0.0;2],
			fHslider13: 0.0,
			fRec63: [0.0;2],
			fHslider14: 0.0,
			fRec64: [0.0;2],
			fRec65: [0.0;2],
			fVec4: [0.0;2],
			fRec66: [0.0;2],
			fRec67: [0.0;2],
			fHslider15: 0.0,
			fRec68: [0.0;2],
			fVec5: [0.0;2],
			fRec69: [0.0;2],
			fHslider16: 0.0,
			fRec70: [0.0;2],
			fHslider17: 0.0,
			fRec71: [0.0;2],
			fRec72: [0.0;2],
			fVec6: [0.0;2],
			fRec73: [0.0;2],
			fRec74: [0.0;2],
			fHslider18: 0.0,
			fRec75: [0.0;2],
			fVec7: [0.0;2],
			fRec76: [0.0;2],
			fHslider19: 0.0,
			fRec77: [0.0;2],
			fHslider20: 0.0,
			fRec78: [0.0;2],
			fRec79: [0.0;2],
			fVec8: [0.0;2],
			fRec80: [0.0;2],
			fRec81: [0.0;2],
			fHslider21: 0.0,
			fRec82: [0.0;2],
			fVec9: [0.0;2],
			fRec83: [0.0;2],
			fHslider22: 0.0,
			fRec84: [0.0;2],
			fHslider23: 0.0,
			fRec85: [0.0;2],
			fRec86: [0.0;2],
			fVec10: [0.0;2],
			fRec87: [0.0;2],
			fRec88: [0.0;2],
			fHslider24: 0.0,
			fRec89: [0.0;2],
			fVec11: [0.0;2],
			fRec90: [0.0;2],
			fHslider25: 0.0,
			fRec91: [0.0;2],
			fHslider26: 0.0,
			fRec92: [0.0;2],
			fRec93: [0.0;2],
			fVec12: [0.0;2],
			fRec94: [0.0;2],
			fRec95: [0.0;2],
			fHslider27: 0.0,
			fRec96: [0.0;2],
			fVec13: [0.0;2],
			fRec99: [0.0;2],
			fRec97: [0.0;2],
			fRec98: [0.0;2],
			fRec45: [0.0;2],
			fRec46: [0.0;2],
			fRec43: [0.0;2],
			fRec44: [0.0;2],
			IOTA0: 0,
			fVec14: [0.0;16384],
			fConst34: 0.0,
			iConst35: 0,
			fVec15: [0.0;2048],
			fVec16: [0.0;16384],
			fConst36: 0.0,
			iConst37: 0,
			fVec17: [0.0;2048],
			iConst38: 0,
			fRec13: [0.0;2],
			fConst39: 0.0,
			fConst40: 0.0,
			fConst41: 0.0,
			fConst42: 0.0,
			fConst43: 0.0,
			fConst44: 0.0,
			fConst45: 0.0,
			fConst46: 0.0,
			fConst47: 0.0,
			fConst48: 0.0,
			fConst49: 0.0,
			fRec14: [0.0;2],
			fConst50: 0.0,
			fConst51: 0.0,
			fConst52: 0.0,
			fConst53: 0.0,
			fConst54: 0.0,
			fConst55: 0.0,
			fConst56: 0.0,
			fConst57: 0.0,
			fConst58: 0.0,
			fConst59: 0.0,
			fConst60: 0.0,
			fRec15: [0.0;2],
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
			fConst71: 0.0,
			fRec16: [0.0;2],
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
			fConst82: 0.0,
			fRec17: [0.0;2],
			iConst83: 0,
			fRec18: [0.0;2],
			fVec18: [0.0;32768],
			fConst84: 0.0,
			iConst85: 0,
			fRec102: [0.0;2],
			fRec103: [0.0;2],
			fRec100: [0.0;2],
			fRec101: [0.0;2],
			fVec19: [0.0;32],
			fVec20: [0.0;16384],
			fConst86: 0.0,
			iConst87: 0,
			fVec21: [0.0;4096],
			iConst88: 0,
			fRec20: [0.0;2],
			fConst89: 0.0,
			fConst90: 0.0,
			fConst91: 0.0,
			fConst92: 0.0,
			fConst93: 0.0,
			fConst94: 0.0,
			fConst95: 0.0,
			fConst96: 0.0,
			fConst97: 0.0,
			fConst98: 0.0,
			fConst99: 0.0,
			fRec21: [0.0;2],
			fVec22: [0.0;16384],
			fConst100: 0.0,
			iConst101: 0,
			fVec23: [0.0;4096],
			fVec24: [0.0;16384],
			fConst102: 0.0,
			iConst103: 0,
			fVec25: [0.0;4096],
			fConst104: 0.0,
			fConst105: 0.0,
			fConst106: 0.0,
			fConst107: 0.0,
			fConst108: 0.0,
			fConst109: 0.0,
			fConst110: 0.0,
			fConst111: 0.0,
			fConst112: 0.0,
			fConst113: 0.0,
			fConst114: 0.0,
			fRec24: [0.0;2],
			fVec26: [0.0;32768],
			fConst115: 0.0,
			iConst116: 0,
			fVec27: [0.0;4096],
			iConst117: 0,
			fRec25: [0.0;2],
			iConst118: 0,
			fRec27: [0.0;2],
			iConst119: 0,
			fRec28: [0.0;2],
			fVec28: [0.0;2048],
			iConst120: 0,
			fRec30: [0.0;2],
			fVec29: [0.0;32768],
			fConst121: 0.0,
			iConst122: 0,
			fVec30: [0.0;4096],
			iConst123: 0,
			fRec32: [0.0;2],
			fRec33: [0.0;3],
			fRec34: [0.0;3],
			fRec35: [0.0;3],
			fRec36: [0.0;3],
			fRec37: [0.0;3],
			fRec38: [0.0;3],
			fRec39: [0.0;3],
			fRec40: [0.0;3],
			fHslider28: 0.0,
			fRec104: [0.0;2],
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
		m.declare("filters.lib/lowpass0_highpass1:author", r"Julius O. Smith III");
		m.declare("filters.lib/lowpass0_highpass1:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/lowpass0_highpass1:license", r"LicenseRef-STK-4.3");
		m.declare("filters.lib/lowpass:author", r"Julius O. Smith III");
		m.declare("filters.lib/lowpass:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/lowpass:license", r"LicenseRef-STK-4.3");
		m.declare("filters.lib/name", r"Faust Filters Library");
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
		self.fHslider11 = (164.81) as FaustFloat;
		self.fHslider12 = (7.0) as FaustFloat;
		self.fHslider13 = (0.3) as FaustFloat;
		self.fHslider14 = (130.81) as FaustFloat;
		self.fHslider15 = (7.0) as FaustFloat;
		self.fHslider16 = (0.3) as FaustFloat;
		self.fHslider17 = (1.1e+02) as FaustFloat;
		self.fHslider18 = (7.0) as FaustFloat;
		self.fHslider19 = (0.3) as FaustFloat;
		self.fHslider20 = (82.41) as FaustFloat;
		self.fHslider21 = (7.0) as FaustFloat;
		self.fHslider22 = (0.3) as FaustFloat;
		self.fHslider23 = (2.2e+02) as FaustFloat;
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
			self.fRec1[l2 as usize] = 0.0;
		}
		for l3 in 0..2 {
			self.fRec2[l3 as usize] = 0.0;
		}
		for l4 in 0..2 {
			self.fRec3[l4 as usize] = 0.0;
		}
		for l5 in 0..2 {
			self.fRec4[l5 as usize] = 0.0;
		}
		for l6 in 0..2 {
			self.fRec5[l6 as usize] = 0.0;
		}
		for l7 in 0..2 {
			self.fRec6[l7 as usize] = 0.0;
		}
		for l8 in 0..2 {
			self.fRec7[l8 as usize] = 0.0;
		}
		for l9 in 0..2 {
			self.fRec8[l9 as usize] = 0.0;
		}
		for l10 in 0..2 {
			self.fRec9[l10 as usize] = 0.0;
		}
		for l11 in 0..2 {
			self.fRec10[l11 as usize] = 0.0;
		}
		for l12 in 0..2 {
			self.fRec41[l12 as usize] = 0.0;
		}
		for l13 in 0..2 {
			self.fRec42[l13 as usize] = 0.0;
		}
		for l14 in 0..2 {
			self.fRec47[l14 as usize] = 0.0;
		}
		for l15 in 0..2 {
			self.fRec48[l15 as usize] = 0.0;
		}
		for l16 in 0..2 {
			self.fRec49[l16 as usize] = 0.0;
		}
		for l17 in 0..2 {
			self.fRec50[l17 as usize] = 0.0;
		}
		for l18 in 0..2 {
			self.fRec51[l18 as usize] = 0.0;
		}
		for l19 in 0..2 {
			self.fRec52[l19 as usize] = 0.0;
		}
		for l20 in 0..2 {
			self.fRec53[l20 as usize] = 0.0;
		}
		for l21 in 0..2 {
			self.iRec55[l21 as usize] = 0;
		}
		for l22 in 0..2 {
			self.fVec1[l22 as usize] = 0.0;
		}
		for l23 in 0..2 {
			self.fRec54[l23 as usize] = 0.0;
		}
		for l24 in 0..2 {
			self.fRec56[l24 as usize] = 0.0;
		}
		for l25 in 0..2 {
			self.fRec57[l25 as usize] = 0.0;
		}
		for l26 in 0..2 {
			self.fRec58[l26 as usize] = 0.0;
		}
		for l27 in 0..2 {
			self.fVec2[l27 as usize] = 0.0;
		}
		for l28 in 0..2 {
			self.fRec59[l28 as usize] = 0.0;
		}
		for l29 in 0..2 {
			self.fRec60[l29 as usize] = 0.0;
		}
		for l30 in 0..2 {
			self.fRec61[l30 as usize] = 0.0;
		}
		for l31 in 0..2 {
			self.fVec3[l31 as usize] = 0.0;
		}
		for l32 in 0..2 {
			self.fRec62[l32 as usize] = 0.0;
		}
		for l33 in 0..2 {
			self.fRec63[l33 as usize] = 0.0;
		}
		for l34 in 0..2 {
			self.fRec64[l34 as usize] = 0.0;
		}
		for l35 in 0..2 {
			self.fRec65[l35 as usize] = 0.0;
		}
		for l36 in 0..2 {
			self.fVec4[l36 as usize] = 0.0;
		}
		for l37 in 0..2 {
			self.fRec66[l37 as usize] = 0.0;
		}
		for l38 in 0..2 {
			self.fRec67[l38 as usize] = 0.0;
		}
		for l39 in 0..2 {
			self.fRec68[l39 as usize] = 0.0;
		}
		for l40 in 0..2 {
			self.fVec5[l40 as usize] = 0.0;
		}
		for l41 in 0..2 {
			self.fRec69[l41 as usize] = 0.0;
		}
		for l42 in 0..2 {
			self.fRec70[l42 as usize] = 0.0;
		}
		for l43 in 0..2 {
			self.fRec71[l43 as usize] = 0.0;
		}
		for l44 in 0..2 {
			self.fRec72[l44 as usize] = 0.0;
		}
		for l45 in 0..2 {
			self.fVec6[l45 as usize] = 0.0;
		}
		for l46 in 0..2 {
			self.fRec73[l46 as usize] = 0.0;
		}
		for l47 in 0..2 {
			self.fRec74[l47 as usize] = 0.0;
		}
		for l48 in 0..2 {
			self.fRec75[l48 as usize] = 0.0;
		}
		for l49 in 0..2 {
			self.fVec7[l49 as usize] = 0.0;
		}
		for l50 in 0..2 {
			self.fRec76[l50 as usize] = 0.0;
		}
		for l51 in 0..2 {
			self.fRec77[l51 as usize] = 0.0;
		}
		for l52 in 0..2 {
			self.fRec78[l52 as usize] = 0.0;
		}
		for l53 in 0..2 {
			self.fRec79[l53 as usize] = 0.0;
		}
		for l54 in 0..2 {
			self.fVec8[l54 as usize] = 0.0;
		}
		for l55 in 0..2 {
			self.fRec80[l55 as usize] = 0.0;
		}
		for l56 in 0..2 {
			self.fRec81[l56 as usize] = 0.0;
		}
		for l57 in 0..2 {
			self.fRec82[l57 as usize] = 0.0;
		}
		for l58 in 0..2 {
			self.fVec9[l58 as usize] = 0.0;
		}
		for l59 in 0..2 {
			self.fRec83[l59 as usize] = 0.0;
		}
		for l60 in 0..2 {
			self.fRec84[l60 as usize] = 0.0;
		}
		for l61 in 0..2 {
			self.fRec85[l61 as usize] = 0.0;
		}
		for l62 in 0..2 {
			self.fRec86[l62 as usize] = 0.0;
		}
		for l63 in 0..2 {
			self.fVec10[l63 as usize] = 0.0;
		}
		for l64 in 0..2 {
			self.fRec87[l64 as usize] = 0.0;
		}
		for l65 in 0..2 {
			self.fRec88[l65 as usize] = 0.0;
		}
		for l66 in 0..2 {
			self.fRec89[l66 as usize] = 0.0;
		}
		for l67 in 0..2 {
			self.fVec11[l67 as usize] = 0.0;
		}
		for l68 in 0..2 {
			self.fRec90[l68 as usize] = 0.0;
		}
		for l69 in 0..2 {
			self.fRec91[l69 as usize] = 0.0;
		}
		for l70 in 0..2 {
			self.fRec92[l70 as usize] = 0.0;
		}
		for l71 in 0..2 {
			self.fRec93[l71 as usize] = 0.0;
		}
		for l72 in 0..2 {
			self.fVec12[l72 as usize] = 0.0;
		}
		for l73 in 0..2 {
			self.fRec94[l73 as usize] = 0.0;
		}
		for l74 in 0..2 {
			self.fRec95[l74 as usize] = 0.0;
		}
		for l75 in 0..2 {
			self.fRec96[l75 as usize] = 0.0;
		}
		for l76 in 0..2 {
			self.fVec13[l76 as usize] = 0.0;
		}
		for l77 in 0..2 {
			self.fRec99[l77 as usize] = 0.0;
		}
		for l78 in 0..2 {
			self.fRec97[l78 as usize] = 0.0;
		}
		for l79 in 0..2 {
			self.fRec98[l79 as usize] = 0.0;
		}
		for l80 in 0..2 {
			self.fRec45[l80 as usize] = 0.0;
		}
		for l81 in 0..2 {
			self.fRec46[l81 as usize] = 0.0;
		}
		for l82 in 0..2 {
			self.fRec43[l82 as usize] = 0.0;
		}
		for l83 in 0..2 {
			self.fRec44[l83 as usize] = 0.0;
		}
		self.IOTA0 = 0;
		for l84 in 0..16384 {
			self.fVec14[l84 as usize] = 0.0;
		}
		for l85 in 0..2048 {
			self.fVec15[l85 as usize] = 0.0;
		}
		for l86 in 0..16384 {
			self.fVec16[l86 as usize] = 0.0;
		}
		for l87 in 0..2048 {
			self.fVec17[l87 as usize] = 0.0;
		}
		for l88 in 0..2 {
			self.fRec13[l88 as usize] = 0.0;
		}
		for l89 in 0..2 {
			self.fRec14[l89 as usize] = 0.0;
		}
		for l90 in 0..2 {
			self.fRec15[l90 as usize] = 0.0;
		}
		for l91 in 0..2 {
			self.fRec16[l91 as usize] = 0.0;
		}
		for l92 in 0..2 {
			self.fRec17[l92 as usize] = 0.0;
		}
		for l93 in 0..2 {
			self.fRec18[l93 as usize] = 0.0;
		}
		for l94 in 0..32768 {
			self.fVec18[l94 as usize] = 0.0;
		}
		for l95 in 0..2 {
			self.fRec102[l95 as usize] = 0.0;
		}
		for l96 in 0..2 {
			self.fRec103[l96 as usize] = 0.0;
		}
		for l97 in 0..2 {
			self.fRec100[l97 as usize] = 0.0;
		}
		for l98 in 0..2 {
			self.fRec101[l98 as usize] = 0.0;
		}
		for l99 in 0..32 {
			self.fVec19[l99 as usize] = 0.0;
		}
		for l100 in 0..16384 {
			self.fVec20[l100 as usize] = 0.0;
		}
		for l101 in 0..4096 {
			self.fVec21[l101 as usize] = 0.0;
		}
		for l102 in 0..2 {
			self.fRec20[l102 as usize] = 0.0;
		}
		for l103 in 0..2 {
			self.fRec21[l103 as usize] = 0.0;
		}
		for l104 in 0..16384 {
			self.fVec22[l104 as usize] = 0.0;
		}
		for l105 in 0..4096 {
			self.fVec23[l105 as usize] = 0.0;
		}
		for l106 in 0..16384 {
			self.fVec24[l106 as usize] = 0.0;
		}
		for l107 in 0..4096 {
			self.fVec25[l107 as usize] = 0.0;
		}
		for l108 in 0..2 {
			self.fRec24[l108 as usize] = 0.0;
		}
		for l109 in 0..32768 {
			self.fVec26[l109 as usize] = 0.0;
		}
		for l110 in 0..4096 {
			self.fVec27[l110 as usize] = 0.0;
		}
		for l111 in 0..2 {
			self.fRec25[l111 as usize] = 0.0;
		}
		for l112 in 0..2 {
			self.fRec27[l112 as usize] = 0.0;
		}
		for l113 in 0..2 {
			self.fRec28[l113 as usize] = 0.0;
		}
		for l114 in 0..2048 {
			self.fVec28[l114 as usize] = 0.0;
		}
		for l115 in 0..2 {
			self.fRec30[l115 as usize] = 0.0;
		}
		for l116 in 0..32768 {
			self.fVec29[l116 as usize] = 0.0;
		}
		for l117 in 0..4096 {
			self.fVec30[l117 as usize] = 0.0;
		}
		for l118 in 0..2 {
			self.fRec32[l118 as usize] = 0.0;
		}
		for l119 in 0..3 {
			self.fRec33[l119 as usize] = 0.0;
		}
		for l120 in 0..3 {
			self.fRec34[l120 as usize] = 0.0;
		}
		for l121 in 0..3 {
			self.fRec35[l121 as usize] = 0.0;
		}
		for l122 in 0..3 {
			self.fRec36[l122 as usize] = 0.0;
		}
		for l123 in 0..3 {
			self.fRec37[l123 as usize] = 0.0;
		}
		for l124 in 0..3 {
			self.fRec38[l124 as usize] = 0.0;
		}
		for l125 in 0..3 {
			self.fRec39[l125 as usize] = 0.0;
		}
		for l126 in 0..3 {
			self.fRec40[l126 as usize] = 0.0;
		}
		for l127 in 0..2 {
			self.fRec104[l127 as usize] = 0.0;
		}
	}
	pub fn instance_constants(&mut self, sample_rate: i32) {
		// Obtaining locks on 0 static var(s)
		self.fSampleRate = sample_rate;
		self.fConst0 = F32::min(1.92e+05, F32::max(1.0, (self.fSampleRate) as F32));
		self.fConst1 = 44.1 / self.fConst0;
		self.fConst2 = 1.0 - self.fConst1;
		self.fConst3 = 1.0 / F32::tan(628.31854 / self.fConst0);
		self.fConst4 = 1.0 - self.fConst3;
		self.fConst5 = 1.0 / (self.fConst3 + 1.0);
		self.fConst6 = F32::floor(0.125 * self.fConst0 + 0.5);
		self.fConst7 = self.fConst6 / self.fConst0;
		self.fConst8 = F32::exp(-(0.8634694 * self.fConst7));
		self.fConst9 = MoodscoreDsp_faustpower2_f(self.fConst8);
		self.fConst10 = 1.0 - self.fConst9;
		self.fConst11 = F32::cos(37699.113 / self.fConst0);
		self.fConst12 = 1.0 - self.fConst11 * self.fConst9;
		self.fConst13 = F32::sqrt(F32::max(0.0, MoodscoreDsp_faustpower2_f(self.fConst12) / MoodscoreDsp_faustpower2_f(self.fConst10) + -1.0));
		self.fConst14 = self.fConst12 / self.fConst10;
		self.fConst15 = self.fConst14 - self.fConst13;
		self.fConst16 = F32::exp(-(1.1512926 * self.fConst7)) / self.fConst8 + -1.0;
		self.fConst17 = self.fConst8 * (self.fConst13 + (1.0 - self.fConst14));
		self.fConst18 = F32::floor(0.153129 * self.fConst0 + 0.5);
		self.fConst19 = self.fConst18 / self.fConst0;
		self.fConst20 = F32::exp(-(0.8634694 * self.fConst19));
		self.fConst21 = MoodscoreDsp_faustpower2_f(self.fConst20);
		self.fConst22 = 1.0 - self.fConst21;
		self.fConst23 = 1.0 - self.fConst11 * self.fConst21;
		self.fConst24 = F32::sqrt(F32::max(0.0, MoodscoreDsp_faustpower2_f(self.fConst23) / MoodscoreDsp_faustpower2_f(self.fConst22) + -1.0));
		self.fConst25 = self.fConst23 / self.fConst22;
		self.fConst26 = self.fConst25 - self.fConst24;
		self.fConst27 = F32::exp(-(1.1512926 * self.fConst19)) / self.fConst20 + -1.0;
		self.fConst28 = self.fConst20 * (self.fConst24 + (1.0 - self.fConst25));
		self.fConst29 = 3.1415927 / self.fConst0;
		self.fConst30 = 1.0 / F32::tan(0.4712389 / self.fConst0);
		self.fConst31 = 1.0 - self.fConst30;
		self.fConst32 = 1.0 / (self.fConst30 + 1.0);
		self.fConst33 = 1.0 / self.fConst0;
		self.fConst34 = F32::floor(0.013458 * self.fConst0 + 0.5);
		self.iConst35 = (F32::min(8192.0, F32::max(0.0, self.fConst6 - self.fConst34))) as i32;
		self.fConst36 = F32::floor(0.020346 * self.fConst0 + 0.5);
		self.iConst37 = (F32::min(8192.0, F32::max(0.0, self.fConst18 - self.fConst36))) as i32;
		self.iConst38 = (F32::min(1024.0, F32::max(0.0, self.fConst36 + -1.0))) as i32;
		self.fConst39 = F32::floor(0.127837 * self.fConst0 + 0.5);
		self.fConst40 = self.fConst39 / self.fConst0;
		self.fConst41 = F32::exp(-(0.8634694 * self.fConst40));
		self.fConst42 = MoodscoreDsp_faustpower2_f(self.fConst41);
		self.fConst43 = 1.0 - self.fConst42;
		self.fConst44 = 1.0 - self.fConst11 * self.fConst42;
		self.fConst45 = F32::sqrt(F32::max(0.0, MoodscoreDsp_faustpower2_f(self.fConst44) / MoodscoreDsp_faustpower2_f(self.fConst43) + -1.0));
		self.fConst46 = self.fConst44 / self.fConst43;
		self.fConst47 = self.fConst46 - self.fConst45;
		self.fConst48 = F32::exp(-(1.1512926 * self.fConst40)) / self.fConst41 + -1.0;
		self.fConst49 = self.fConst41 * (self.fConst45 + (1.0 - self.fConst46));
		self.fConst50 = F32::floor(0.192303 * self.fConst0 + 0.5);
		self.fConst51 = self.fConst50 / self.fConst0;
		self.fConst52 = F32::exp(-(0.8634694 * self.fConst51));
		self.fConst53 = MoodscoreDsp_faustpower2_f(self.fConst52);
		self.fConst54 = 1.0 - self.fConst53;
		self.fConst55 = 1.0 - self.fConst11 * self.fConst53;
		self.fConst56 = F32::sqrt(F32::max(0.0, MoodscoreDsp_faustpower2_f(self.fConst55) / MoodscoreDsp_faustpower2_f(self.fConst54) + -1.0));
		self.fConst57 = self.fConst55 / self.fConst54;
		self.fConst58 = self.fConst57 - self.fConst56;
		self.fConst59 = F32::exp(-(1.1512926 * self.fConst51)) / self.fConst52 + -1.0;
		self.fConst60 = self.fConst52 * (self.fConst56 + (1.0 - self.fConst57));
		self.fConst61 = F32::floor(0.219991 * self.fConst0 + 0.5);
		self.fConst62 = self.fConst61 / self.fConst0;
		self.fConst63 = F32::exp(-(0.8634694 * self.fConst62));
		self.fConst64 = MoodscoreDsp_faustpower2_f(self.fConst63);
		self.fConst65 = 1.0 - self.fConst64;
		self.fConst66 = 1.0 - self.fConst11 * self.fConst64;
		self.fConst67 = F32::sqrt(F32::max(0.0, MoodscoreDsp_faustpower2_f(self.fConst66) / MoodscoreDsp_faustpower2_f(self.fConst65) + -1.0));
		self.fConst68 = self.fConst66 / self.fConst65;
		self.fConst69 = self.fConst68 - self.fConst67;
		self.fConst70 = F32::exp(-(1.1512926 * self.fConst62)) / self.fConst63 + -1.0;
		self.fConst71 = self.fConst63 * (self.fConst67 + (1.0 - self.fConst68));
		self.fConst72 = F32::floor(0.174713 * self.fConst0 + 0.5);
		self.fConst73 = self.fConst72 / self.fConst0;
		self.fConst74 = F32::exp(-(0.8634694 * self.fConst73));
		self.fConst75 = MoodscoreDsp_faustpower2_f(self.fConst74);
		self.fConst76 = 1.0 - self.fConst75;
		self.fConst77 = 1.0 - self.fConst11 * self.fConst75;
		self.fConst78 = F32::sqrt(F32::max(0.0, MoodscoreDsp_faustpower2_f(self.fConst77) / MoodscoreDsp_faustpower2_f(self.fConst76) + -1.0));
		self.fConst79 = self.fConst77 / self.fConst76;
		self.fConst80 = self.fConst79 - self.fConst78;
		self.fConst81 = F32::exp(-(1.1512926 * self.fConst73)) / self.fConst74 + -1.0;
		self.fConst82 = self.fConst74 * (self.fConst78 + (1.0 - self.fConst79));
		self.iConst83 = (F32::min(1024.0, F32::max(0.0, self.fConst34 + -1.0))) as i32;
		self.fConst84 = F32::floor(0.019123 * self.fConst0 + 0.5);
		self.iConst85 = (F32::min(16384.0, F32::max(0.0, self.fConst61 - self.fConst84))) as i32;
		self.fConst86 = F32::floor(0.031604 * self.fConst0 + 0.5);
		self.iConst87 = (F32::min(8192.0, F32::max(0.0, self.fConst39 - self.fConst86))) as i32;
		self.iConst88 = (F32::min(2048.0, F32::max(0.0, self.fConst86 + -1.0))) as i32;
		self.fConst89 = F32::floor(0.210389 * self.fConst0 + 0.5);
		self.fConst90 = self.fConst89 / self.fConst0;
		self.fConst91 = F32::exp(-(0.8634694 * self.fConst90));
		self.fConst92 = MoodscoreDsp_faustpower2_f(self.fConst91);
		self.fConst93 = 1.0 - self.fConst92;
		self.fConst94 = 1.0 - self.fConst11 * self.fConst92;
		self.fConst95 = F32::sqrt(F32::max(0.0, MoodscoreDsp_faustpower2_f(self.fConst94) / MoodscoreDsp_faustpower2_f(self.fConst93) + -1.0));
		self.fConst96 = self.fConst94 / self.fConst93;
		self.fConst97 = self.fConst96 - self.fConst95;
		self.fConst98 = F32::exp(-(1.1512926 * self.fConst90)) / self.fConst91 + -1.0;
		self.fConst99 = self.fConst91 * (self.fConst95 + (1.0 - self.fConst96));
		self.fConst100 = F32::floor(0.022904 * self.fConst0 + 0.5);
		self.iConst101 = (F32::min(8192.0, F32::max(0.0, self.fConst72 - self.fConst100))) as i32;
		self.fConst102 = F32::floor(0.029291 * self.fConst0 + 0.5);
		self.iConst103 = (F32::min(8192.0, F32::max(0.0, self.fConst50 - self.fConst102))) as i32;
		self.fConst104 = F32::floor(0.256891 * self.fConst0 + 0.5);
		self.fConst105 = self.fConst104 / self.fConst0;
		self.fConst106 = F32::exp(-(0.8634694 * self.fConst105));
		self.fConst107 = MoodscoreDsp_faustpower2_f(self.fConst106);
		self.fConst108 = 1.0 - self.fConst107;
		self.fConst109 = 1.0 - self.fConst11 * self.fConst107;
		self.fConst110 = F32::sqrt(F32::max(0.0, MoodscoreDsp_faustpower2_f(self.fConst109) / MoodscoreDsp_faustpower2_f(self.fConst108) + -1.0));
		self.fConst111 = self.fConst109 / self.fConst108;
		self.fConst112 = self.fConst111 - self.fConst110;
		self.fConst113 = F32::exp(-(1.1512926 * self.fConst105)) / self.fConst106 + -1.0;
		self.fConst114 = self.fConst106 * (self.fConst110 + (1.0 - self.fConst111));
		self.fConst115 = F32::floor(0.024421 * self.fConst0 + 0.5);
		self.iConst116 = (F32::min(16384.0, F32::max(0.0, self.fConst89 - self.fConst115))) as i32;
		self.iConst117 = (F32::min(2048.0, F32::max(0.0, self.fConst115 + -1.0))) as i32;
		self.iConst118 = (F32::min(2048.0, F32::max(0.0, self.fConst102 + -1.0))) as i32;
		self.iConst119 = (F32::min(2048.0, F32::max(0.0, self.fConst100 + -1.0))) as i32;
		self.iConst120 = (F32::min(1024.0, F32::max(0.0, self.fConst84 + -1.0))) as i32;
		self.fConst121 = F32::floor(0.027333 * self.fConst0 + 0.5);
		self.iConst122 = (F32::min(16384.0, F32::max(0.0, self.fConst104 - self.fConst121))) as i32;
		self.iConst123 = (F32::min(2048.0, F32::max(0.0, self.fConst121 + -1.0))) as i32;
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
			18 => Some(self.fHslider10),
			20 => Some(self.fHslider11),
			13 => Some(self.fHslider12),
			14 => Some(self.fHslider13),
			16 => Some(self.fHslider14),
			9 => Some(self.fHslider15),
			10 => Some(self.fHslider16),
			12 => Some(self.fHslider17),
			5 => Some(self.fHslider18),
			6 => Some(self.fHslider19),
			26 => Some(self.fHslider2),
			8 => Some(self.fHslider20),
			21 => Some(self.fHslider21),
			22 => Some(self.fHslider22),
			24 => Some(self.fHslider23),
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
			17 => Some(self.fHslider9),
			_ => None,
		}
	}
	
	pub fn set_param(&mut self, param: ParamIndex, value: FaustFloat) {
		match param.0 {
			27 => { self.fHslider0 = value }
			25 => { self.fHslider1 = value }
			18 => { self.fHslider10 = value }
			20 => { self.fHslider11 = value }
			13 => { self.fHslider12 = value }
			14 => { self.fHslider13 = value }
			16 => { self.fHslider14 = value }
			9 => { self.fHslider15 = value }
			10 => { self.fHslider16 = value }
			12 => { self.fHslider17 = value }
			5 => { self.fHslider18 = value }
			6 => { self.fHslider19 = value }
			26 => { self.fHslider2 = value }
			8 => { self.fHslider20 = value }
			21 => { self.fHslider21 = value }
			22 => { self.fHslider22 = value }
			24 => { self.fHslider23 = value }
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
			17 => { self.fHslider9 = value }
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
		let zipped_iterators = outputs0.zip(outputs1);
		for (output0, output1) in zipped_iterators {
			self.iVec0[0] = 1;
			self.fRec0[0] = fSlow0 + self.fConst2 * self.fRec0[1];
			self.fRec1[0] = -(self.fConst5 * (self.fConst4 * self.fRec1[1] - (self.fRec40[1] + self.fRec40[2])));
			self.fRec2[0] = -(self.fConst5 * (self.fConst4 * self.fRec2[1] - (self.fRec37[1] + self.fRec37[2])));
			self.fRec3[0] = -(self.fConst5 * (self.fConst4 * self.fRec3[1] - (self.fRec39[1] + self.fRec39[2])));
			self.fRec4[0] = -(self.fConst5 * (self.fConst4 * self.fRec4[1] - (self.fRec35[1] + self.fRec35[2])));
			self.fRec5[0] = -(self.fConst5 * (self.fConst4 * self.fRec5[1] - (self.fRec36[1] + self.fRec36[2])));
			self.fRec6[0] = -(self.fConst5 * (self.fConst4 * self.fRec6[1] - (self.fRec34[1] + self.fRec34[2])));
			self.fRec7[0] = -(self.fConst5 * (self.fConst4 * self.fRec7[1] - (self.fRec38[1] + self.fRec38[2])));
			self.fRec8[0] = -(self.fConst5 * (self.fConst4 * self.fRec8[1] - (self.fRec33[1] + self.fRec33[2])));
			self.fRec9[0] = self.fConst17 * (self.fRec35[1] + self.fConst16 * self.fRec4[0]) + self.fConst15 * self.fRec9[1];
			self.fRec10[0] = self.fConst28 * (self.fRec40[1] + self.fConst27 * self.fRec1[0]) + self.fConst26 * self.fRec10[1];
			self.fRec41[0] = fSlow1 + self.fConst2 * self.fRec41[1];
			let mut fTemp0: F32 = F32::tan(self.fConst29 * F32::max(self.fRec41[0], 2e+01));
			self.fRec42[0] = fSlow2 + self.fConst2 * self.fRec42[1];
			let mut fTemp1: F32 = 1.99998 * self.fRec42[0];
			let mut fTemp2: F32 = 2.0 - fTemp1;
			let mut fTemp3: F32 = 1.0 / fTemp0;
			let mut fTemp4: F32 = MoodscoreDsp_faustpower2_f(1.4141995 * self.fRec42[0]);
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
			let mut fTemp16: F32 = 1.0 - fTemp9 / fTemp6;
			let mut fTemp17: F32 = 1.0 - fTemp7 - 2.0 * fTemp10 * fTemp16;
			let mut fTemp18: F32 = F32::sqrt(F32::max(0.0, fTemp11));
			let mut fTemp19: F32 = 2.0 + fTemp1;
			let mut fTemp20: F32 = fTemp1 + fTemp4;
			let mut fTemp21: F32 = 1.0 + fTemp20 + (fTemp19 + fTemp3) / fTemp0;
			let mut fTemp22: F32 = F32::sqrt(F32::max(0.0, fTemp13));
			let mut fTemp23: F32 = (1.0 - (fTemp19 - fTemp3) / fTemp0 + fTemp20) / fTemp21;
			let mut fTemp24: F32 = 1.0 - fTemp8 + fTemp20;
			let mut fTemp25: F32 = F32::max(-0.9999999, F32::min(0.9999999, 2.0 * (fTemp24 / (fTemp21 * (1.0 + fTemp23)))));
			let mut fTemp26: F32 = 1.0 - MoodscoreDsp_faustpower2_f(fTemp25);
			let mut fTemp27: F32 = F32::max(-0.9999999, F32::min(0.9999999, fTemp23));
			let mut fTemp28: F32 = 1.0 - MoodscoreDsp_faustpower2_f(fTemp27);
			let mut fTemp29: F32 = F32::sqrt(fTemp28);
			let mut fTemp30: F32 = fTemp29 * F32::sqrt(fTemp26);
			let mut fTemp31: F32 = 1.0 - fTemp24 / fTemp21;
			let mut fTemp32: F32 = 1.0 - fTemp23 - 2.0 * fTemp25 * fTemp31;
			let mut fTemp33: F32 = F32::sqrt(F32::max(0.0, fTemp26));
			self.fRec47[0] = fSlow3 + self.fConst2 * self.fRec47[1];
			self.fRec48[0] = fSlow4 + self.fConst2 * self.fRec48[1];
			self.fRec49[0] = fSlow5 + self.fConst2 * self.fRec49[1];
			self.fRec50[0] = fSlow6 + self.fConst2 * self.fRec50[1];
			self.fRec51[0] = fSlow7 + self.fConst2 * self.fRec51[1];
			self.fRec52[0] = fSlow8 + self.fConst2 * self.fRec52[1];
			let mut fTemp34: F32 = F32::max(1.0, 0.6666667 * F32::sqrt(MoodscoreDsp_faustpower2_f(self.fRec52[0]) + MoodscoreDsp_faustpower2_f(self.fRec51[0]) + MoodscoreDsp_faustpower2_f(self.fRec50[0]) + MoodscoreDsp_faustpower2_f(self.fRec49[0]) + MoodscoreDsp_faustpower2_f(self.fRec48[0]) + MoodscoreDsp_faustpower2_f(self.fRec47[0])));
			let mut fTemp35: F32 = F32::sqrt(F32::max(0.0, fTemp28));
			self.fRec53[0] = fSlow9 + self.fConst2 * self.fRec53[1];
			let mut iTemp36: i32 = i32::wrapping_mul(1103515245, i32::wrapping_add(12345, i32::wrapping_mul(1103515245, i32::wrapping_add(12345, self.iRec55[1]))));
			let mut iTemp37: i32 = i32::wrapping_mul(1103515245, i32::wrapping_add(12345, iTemp36));
			let mut iTemp38: i32 = i32::wrapping_mul(1103515245, i32::wrapping_add(12345, i32::wrapping_mul(1103515245, i32::wrapping_add(12345, iTemp37))));
			let mut iTemp39: i32 = i32::wrapping_mul(1103515245, i32::wrapping_add(12345, iTemp38));
			let mut iTemp40: i32 = i32::wrapping_mul(1103515245, i32::wrapping_add(12345, i32::wrapping_mul(1103515245, i32::wrapping_add(12345, iTemp39))));
			let mut iTemp41: i32 = i32::wrapping_mul(1103515245, i32::wrapping_add(12345, iTemp40));
			let mut iTemp42: i32 = i32::wrapping_mul(1103515245, i32::wrapping_add(12345, i32::wrapping_mul(1103515245, i32::wrapping_add(12345, iTemp41))));
			let mut iTemp43: i32 = i32::wrapping_mul(1103515245, i32::wrapping_add(12345, iTemp42));
			let mut iTemp44: i32 = i32::wrapping_mul(1103515245, i32::wrapping_add(12345, i32::wrapping_mul(1103515245, i32::wrapping_add(12345, iTemp43))));
			let mut iTemp45: i32 = i32::wrapping_mul(1103515245, i32::wrapping_add(12345, iTemp44));
			let mut iTemp46: i32 = i32::wrapping_mul(1103515245, i32::wrapping_add(12345, iTemp45));
			let mut iTemp47: i32 = i32::wrapping_mul(1103515245, i32::wrapping_add(12345, iTemp46));
			self.iRec55[0] = i32::wrapping_mul(1103515245, i32::wrapping_add(12345, iTemp47));
			let mut fTemp48: F32 = (iTemp38) as F32;
			self.fVec1[0] = fTemp48;
			self.fRec54[0] = self.fConst32 * (4.656613e-10 * (fTemp48 + self.fVec1[1]) - self.fConst31 * self.fRec54[1]);
			self.fRec56[0] = fSlow10 + self.fConst2 * self.fRec56[1];
			self.fRec57[0] = fSlow11 + self.fConst2 * self.fRec57[1];
			let mut fTemp49: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec57[0] * F32::powf(2.0, 0.00083333335 * (4.5e+03 * self.fRec56[0] * self.fRec54[0] - self.fRec53[0]))));
			let mut fTemp50: F32 = self.fRec58[1] + self.fConst33 * fTemp49;
			let mut fTemp51: F32 = -1.0 + fTemp50;
			let mut iTemp52: i32 = (fTemp51 < 0.0) as i32;
			self.fRec58[0] = (if iTemp52 != 0 {fTemp50} else {fTemp51});
			let mut fTemp53: F32 = (iTemp39) as F32;
			self.fVec2[0] = fTemp53;
			self.fRec59[0] = self.fConst32 * (4.656613e-10 * (fTemp53 + self.fVec2[1]) - self.fConst31 * self.fRec59[1]);
			let mut fTemp54: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec57[0] * F32::powf(2.0, 0.00083333335 * (4.5e+03 * self.fRec59[0] * self.fRec56[0] + self.fRec53[0]))));
			let mut fTemp55: F32 = self.fRec60[1] + self.fConst33 * fTemp54;
			let mut fTemp56: F32 = -1.0 + fTemp55;
			let mut iTemp57: i32 = (fTemp56 < 0.0) as i32;
			self.fRec60[0] = (if iTemp57 != 0 {fTemp55} else {fTemp56});
			let mut fTemp58: F32 = (1.0 - ((if iTemp57 != 0 {fTemp55} else {fTemp55 + fTemp56 * (1.0 - self.fConst0 / fTemp54)}) + (if iTemp52 != 0 {fTemp50} else {fTemp50 + fTemp51 * (1.0 - self.fConst0 / fTemp49)}))) * self.fRec48[0];
			self.fRec61[0] = fSlow12 + self.fConst2 * self.fRec61[1];
			let mut fTemp59: F32 = (iTemp40) as F32;
			self.fVec3[0] = fTemp59;
			self.fRec62[0] = self.fConst32 * (4.656613e-10 * (fTemp59 + self.fVec3[1]) - self.fConst31 * self.fRec62[1]);
			self.fRec63[0] = fSlow13 + self.fConst2 * self.fRec63[1];
			self.fRec64[0] = fSlow14 + self.fConst2 * self.fRec64[1];
			let mut fTemp60: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec64[0] * F32::powf(2.0, 0.00083333335 * (4.5e+03 * self.fRec63[0] * self.fRec62[0] - self.fRec61[0]))));
			let mut fTemp61: F32 = self.fRec65[1] + self.fConst33 * fTemp60;
			let mut fTemp62: F32 = -1.0 + fTemp61;
			let mut iTemp63: i32 = (fTemp62 < 0.0) as i32;
			self.fRec65[0] = (if iTemp63 != 0 {fTemp61} else {fTemp62});
			let mut fTemp64: F32 = (iTemp41) as F32;
			self.fVec4[0] = fTemp64;
			self.fRec66[0] = self.fConst32 * (4.656613e-10 * (fTemp64 + self.fVec4[1]) - self.fConst31 * self.fRec66[1]);
			let mut fTemp65: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec64[0] * F32::powf(2.0, 0.00083333335 * (4.5e+03 * self.fRec66[0] * self.fRec63[0] + self.fRec61[0]))));
			let mut fTemp66: F32 = self.fRec67[1] + self.fConst33 * fTemp65;
			let mut fTemp67: F32 = -1.0 + fTemp66;
			let mut iTemp68: i32 = (fTemp67 < 0.0) as i32;
			self.fRec67[0] = (if iTemp68 != 0 {fTemp66} else {fTemp67});
			let mut fTemp69: F32 = (1.0 - ((if iTemp68 != 0 {fTemp66} else {fTemp66 + fTemp67 * (1.0 - self.fConst0 / fTemp65)}) + (if iTemp63 != 0 {fTemp61} else {fTemp61 + fTemp62 * (1.0 - self.fConst0 / fTemp60)}))) * self.fRec49[0];
			self.fRec68[0] = fSlow15 + self.fConst2 * self.fRec68[1];
			let mut fTemp70: F32 = (iTemp42) as F32;
			self.fVec5[0] = fTemp70;
			self.fRec69[0] = self.fConst32 * (4.656613e-10 * (fTemp70 + self.fVec5[1]) - self.fConst31 * self.fRec69[1]);
			self.fRec70[0] = fSlow16 + self.fConst2 * self.fRec70[1];
			self.fRec71[0] = fSlow17 + self.fConst2 * self.fRec71[1];
			let mut fTemp71: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec71[0] * F32::powf(2.0, 0.00083333335 * (4.5e+03 * self.fRec70[0] * self.fRec69[0] - self.fRec68[0]))));
			let mut fTemp72: F32 = self.fRec72[1] + self.fConst33 * fTemp71;
			let mut fTemp73: F32 = -1.0 + fTemp72;
			let mut iTemp74: i32 = (fTemp73 < 0.0) as i32;
			self.fRec72[0] = (if iTemp74 != 0 {fTemp72} else {fTemp73});
			let mut fTemp75: F32 = (iTemp43) as F32;
			self.fVec6[0] = fTemp75;
			self.fRec73[0] = self.fConst32 * (4.656613e-10 * (fTemp75 + self.fVec6[1]) - self.fConst31 * self.fRec73[1]);
			let mut fTemp76: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec71[0] * F32::powf(2.0, 0.00083333335 * (4.5e+03 * self.fRec73[0] * self.fRec70[0] + self.fRec68[0]))));
			let mut fTemp77: F32 = self.fRec74[1] + self.fConst33 * fTemp76;
			let mut fTemp78: F32 = -1.0 + fTemp77;
			let mut iTemp79: i32 = (fTemp78 < 0.0) as i32;
			self.fRec74[0] = (if iTemp79 != 0 {fTemp77} else {fTemp78});
			let mut fTemp80: F32 = (1.0 - ((if iTemp79 != 0 {fTemp77} else {fTemp77 + fTemp78 * (1.0 - self.fConst0 / fTemp76)}) + (if iTemp74 != 0 {fTemp72} else {fTemp72 + fTemp73 * (1.0 - self.fConst0 / fTemp71)}))) * self.fRec50[0];
			self.fRec75[0] = fSlow18 + self.fConst2 * self.fRec75[1];
			let mut fTemp81: F32 = (iTemp44) as F32;
			self.fVec7[0] = fTemp81;
			self.fRec76[0] = self.fConst32 * (4.656613e-10 * (fTemp81 + self.fVec7[1]) - self.fConst31 * self.fRec76[1]);
			self.fRec77[0] = fSlow19 + self.fConst2 * self.fRec77[1];
			self.fRec78[0] = fSlow20 + self.fConst2 * self.fRec78[1];
			let mut fTemp82: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec78[0] * F32::powf(2.0, 0.00083333335 * (4.5e+03 * self.fRec77[0] * self.fRec76[0] - self.fRec75[0]))));
			let mut fTemp83: F32 = self.fRec79[1] + self.fConst33 * fTemp82;
			let mut fTemp84: F32 = -1.0 + fTemp83;
			let mut iTemp85: i32 = (fTemp84 < 0.0) as i32;
			self.fRec79[0] = (if iTemp85 != 0 {fTemp83} else {fTemp84});
			let mut fTemp86: F32 = (iTemp45) as F32;
			self.fVec8[0] = fTemp86;
			self.fRec80[0] = self.fConst32 * (4.656613e-10 * (fTemp86 + self.fVec8[1]) - self.fConst31 * self.fRec80[1]);
			let mut fTemp87: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec78[0] * F32::powf(2.0, 0.00083333335 * (4.5e+03 * self.fRec80[0] * self.fRec77[0] + self.fRec75[0]))));
			let mut fTemp88: F32 = self.fRec81[1] + self.fConst33 * fTemp87;
			let mut fTemp89: F32 = -1.0 + fTemp88;
			let mut iTemp90: i32 = (fTemp89 < 0.0) as i32;
			self.fRec81[0] = (if iTemp90 != 0 {fTemp88} else {fTemp89});
			let mut fTemp91: F32 = (1.0 - ((if iTemp90 != 0 {fTemp88} else {fTemp88 + fTemp89 * (1.0 - self.fConst0 / fTemp87)}) + (if iTemp85 != 0 {fTemp83} else {fTemp83 + fTemp84 * (1.0 - self.fConst0 / fTemp82)}))) * self.fRec51[0];
			self.fRec82[0] = fSlow21 + self.fConst2 * self.fRec82[1];
			let mut fTemp92: F32 = (iTemp36) as F32;
			self.fVec9[0] = fTemp92;
			self.fRec83[0] = self.fConst32 * (4.656613e-10 * (fTemp92 + self.fVec9[1]) - self.fConst31 * self.fRec83[1]);
			self.fRec84[0] = fSlow22 + self.fConst2 * self.fRec84[1];
			self.fRec85[0] = fSlow23 + self.fConst2 * self.fRec85[1];
			let mut fTemp93: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec85[0] * F32::powf(2.0, 0.00083333335 * (4.5e+03 * self.fRec84[0] * self.fRec83[0] - self.fRec82[0]))));
			let mut fTemp94: F32 = self.fRec86[1] + self.fConst33 * fTemp93;
			let mut fTemp95: F32 = -1.0 + fTemp94;
			let mut iTemp96: i32 = (fTemp95 < 0.0) as i32;
			self.fRec86[0] = (if iTemp96 != 0 {fTemp94} else {fTemp95});
			let mut fTemp97: F32 = (iTemp37) as F32;
			self.fVec10[0] = fTemp97;
			self.fRec87[0] = self.fConst32 * (4.656613e-10 * (fTemp97 + self.fVec10[1]) - self.fConst31 * self.fRec87[1]);
			let mut fTemp98: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec85[0] * F32::powf(2.0, 0.00083333335 * (4.5e+03 * self.fRec87[0] * self.fRec84[0] + self.fRec82[0]))));
			let mut fTemp99: F32 = self.fRec88[1] + self.fConst33 * fTemp98;
			let mut fTemp100: F32 = -1.0 + fTemp99;
			let mut iTemp101: i32 = (fTemp100 < 0.0) as i32;
			self.fRec88[0] = (if iTemp101 != 0 {fTemp99} else {fTemp100});
			let mut fTemp102: F32 = (1.0 - ((if iTemp101 != 0 {fTemp99} else {fTemp99 + fTemp100 * (1.0 - self.fConst0 / fTemp98)}) + (if iTemp96 != 0 {fTemp94} else {fTemp94 + fTemp95 * (1.0 - self.fConst0 / fTemp93)}))) * self.fRec47[0];
			self.fRec89[0] = fSlow24 + self.fConst2 * self.fRec89[1];
			let mut fTemp103: F32 = (iTemp47) as F32;
			self.fVec11[0] = fTemp103;
			self.fRec90[0] = self.fConst32 * (4.656613e-10 * (fTemp103 + self.fVec11[1]) - self.fConst31 * self.fRec90[1]);
			self.fRec91[0] = fSlow25 + self.fConst2 * self.fRec91[1];
			self.fRec92[0] = fSlow26 + self.fConst2 * self.fRec92[1];
			let mut fTemp104: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec92[0] * F32::powf(2.0, 0.00083333335 * (4.5e+03 * self.fRec91[0] * self.fRec90[0] - self.fRec89[0]))));
			let mut fTemp105: F32 = self.fRec93[1] + self.fConst33 * fTemp104;
			let mut fTemp106: F32 = -1.0 + fTemp105;
			let mut iTemp107: i32 = (fTemp106 < 0.0) as i32;
			self.fRec93[0] = (if iTemp107 != 0 {fTemp105} else {fTemp106});
			let mut fTemp108: F32 = (self.iRec55[0]) as F32;
			self.fVec12[0] = fTemp108;
			self.fRec94[0] = self.fConst32 * (4.656613e-10 * (fTemp108 + self.fVec12[1]) - self.fConst31 * self.fRec94[1]);
			let mut fTemp109: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec92[0] * F32::powf(2.0, 0.00083333335 * (4.5e+03 * self.fRec94[0] * self.fRec91[0] + self.fRec89[0]))));
			let mut fTemp110: F32 = self.fRec95[1] + self.fConst33 * fTemp109;
			let mut fTemp111: F32 = -1.0 + fTemp110;
			let mut iTemp112: i32 = (fTemp111 < 0.0) as i32;
			self.fRec95[0] = (if iTemp112 != 0 {fTemp110} else {fTemp111});
			self.fRec96[0] = fSlow27 + self.fConst2 * self.fRec96[1];
			let mut fTemp113: F32 = (iTemp46) as F32;
			self.fVec13[0] = fTemp113;
			self.fRec99[0] = self.fConst32 * (4.656613e-10 * (fTemp113 + self.fVec13[1]) - self.fConst31 * self.fRec99[1]);
			let mut fTemp114: F32 = F32::powf(2.0, 3.75 * self.fRec91[0] * self.fRec99[0]);
			self.fRec97[0] = self.fRec97[1] - self.fConst29 * self.fRec92[0] * self.fRec98[1] * fTemp114;
			self.fRec98[0] = (i32::wrapping_sub(1, self.iVec0[1])) as F32 + self.fRec98[1] + self.fConst29 * self.fRec92[0] * fTemp114 * self.fRec97[0];
			let mut fTemp115: F32 = (self.fRec98[0] * self.fRec96[0] - (1.0 - ((if iTemp112 != 0 {fTemp110} else {fTemp110 + fTemp111 * (1.0 - self.fConst0 / fTemp109)}) + (if iTemp107 != 0 {fTemp105} else {fTemp105 + fTemp106 * (1.0 - self.fConst0 / fTemp104)})))) * self.fRec52[0];
			let mut fTemp116: F32 = 1.0 * (fTemp115 - fTemp102) - (1.2338957 * fTemp91 + 0.69101477 * fTemp80 + 1.3751388 * fTemp69 + 0.3301416 * fTemp58);
			let mut fTemp117: F32 = fTemp116 * fTemp35 / fTemp34 - fTemp27 * self.fRec46[1];
			self.fRec45[0] = fTemp117 * fTemp33 - self.fRec45[1] * fTemp25;
			self.fRec46[0] = fTemp25 * fTemp117 + self.fRec45[1] * fTemp33;
			let mut fTemp118: F32 = fTemp116 * fTemp27 / fTemp34 + self.fRec46[1] * fTemp35 + 2.0 * (self.fRec46[0] * fTemp31 / fTemp29) + self.fRec45[0] * fTemp32 / fTemp30;
			let mut fTemp119: F32 = fTemp118 * fTemp22 / fTemp21 - fTemp12 * self.fRec44[1];
			self.fRec43[0] = fTemp119 * fTemp18 - self.fRec43[1] * fTemp10;
			self.fRec44[0] = fTemp10 * fTemp119 + self.fRec43[1] * fTemp18;
			let mut fTemp120: F32 = fTemp118 * fTemp12 / fTemp21 + self.fRec44[1] * fTemp22 + 2.0 * (self.fRec44[0] * fTemp16 / fTemp14) + self.fRec43[0] * fTemp17 / fTemp15;
			let mut fTemp121: F32 = 0.27 * (fTemp120 / fTemp6);
			self.fVec14[(self.IOTA0 & 16383) as usize] = 1e-20 + 0.35355338 * self.fRec9[0];
			let mut fTemp122: F32 = self.fVec14[((i32::wrapping_sub(self.IOTA0, self.iConst35)) & 16383) as usize] - (0.6 * self.fRec18[1] + fTemp121);
			self.fVec15[(self.IOTA0 & 2047) as usize] = fTemp122;
			let mut fRec11: F32 = 0.6 * fTemp122;
			self.fVec16[(self.IOTA0 & 16383) as usize] = 1e-20 + 0.35355338 * self.fRec10[0];
			let mut fTemp123: F32 = fTemp121 + self.fVec16[((i32::wrapping_sub(self.IOTA0, self.iConst37)) & 16383) as usize] - 0.6 * self.fRec13[1];
			self.fVec17[(self.IOTA0 & 2047) as usize] = fTemp123;
			let mut fRec12: F32 = 0.6 * fTemp123;
			self.fRec13[0] = self.fVec17[((i32::wrapping_sub(self.IOTA0, self.iConst38)) & 2047) as usize];
			self.fRec14[0] = self.fConst49 * (self.fRec37[1] + self.fConst48 * self.fRec2[0]) + self.fConst47 * self.fRec14[1];
			self.fRec15[0] = self.fConst60 * (self.fRec34[1] + self.fConst59 * self.fRec6[0]) + self.fConst58 * self.fRec15[1];
			self.fRec16[0] = self.fConst71 * (self.fRec36[1] + self.fConst70 * self.fRec5[0]) + self.fConst69 * self.fRec16[1];
			self.fRec17[0] = self.fConst82 * (self.fRec33[1] + self.fConst81 * self.fRec8[0]) + self.fConst80 * self.fRec17[1];
			self.fRec18[0] = self.fVec15[((i32::wrapping_sub(self.IOTA0, self.iConst83)) & 2047) as usize];
			self.fVec18[(self.IOTA0 & 32767) as usize] = 1e-20 + 0.35355338 * self.fRec16[0];
			let mut fTemp124: F32 = 0.6 * self.fRec30[1] + self.fVec18[((i32::wrapping_sub(self.IOTA0, self.iConst85)) & 32767) as usize];
			let mut fTemp125: F32 = fTemp115 - (fTemp102 + 0.69101477 * fTemp91 + 1.2338957 * fTemp80 + 0.3301416 * fTemp69 + 1.3751388 * fTemp58);
			let mut fTemp126: F32 = fTemp35 * fTemp125 / fTemp34 - fTemp27 * self.fRec103[1];
			self.fRec102[0] = fTemp33 * fTemp126 - fTemp25 * self.fRec102[1];
			self.fRec103[0] = fTemp25 * fTemp126 + fTemp33 * self.fRec102[1];
			let mut fTemp127: F32 = fTemp27 * fTemp125 / fTemp34 + fTemp35 * self.fRec103[1] + 2.0 * (fTemp31 * self.fRec103[0] / fTemp29) + fTemp32 * self.fRec102[0] / fTemp30;
			let mut fTemp128: F32 = fTemp22 * fTemp127 / fTemp21 - fTemp12 * self.fRec101[1];
			self.fRec100[0] = fTemp18 * fTemp128 - fTemp10 * self.fRec100[1];
			self.fRec101[0] = fTemp10 * fTemp128 + fTemp18 * self.fRec100[1];
			self.fVec19[(self.IOTA0 & 31) as usize] = (fTemp12 * fTemp127 / fTemp21 + fTemp22 * self.fRec101[1] + 2.0 * (fTemp16 * self.fRec101[0] / fTemp14) + fTemp17 * self.fRec100[0] / fTemp15) / fTemp6;
			let mut fTemp129: F32 = self.fVec19[((i32::wrapping_sub(self.IOTA0, 23)) & 31) as usize];
			let mut fTemp130: F32 = 0.27 * fTemp129;
			let mut fRec19: F32 = 0.6 * (fTemp130 - fTemp124);
			self.fVec20[(self.IOTA0 & 16383) as usize] = 1e-20 + 0.35355338 * self.fRec14[0];
			let mut fTemp131: F32 = self.fVec20[((i32::wrapping_sub(self.IOTA0, self.iConst87)) & 16383) as usize] - (fTemp121 + 0.6 * self.fRec20[1]);
			self.fVec21[(self.IOTA0 & 4095) as usize] = fTemp131;
			self.fRec20[0] = self.fVec21[((i32::wrapping_sub(self.IOTA0, self.iConst88)) & 4095) as usize];
			self.fRec21[0] = self.fConst99 * (self.fRec39[1] + self.fConst98 * self.fRec3[0]) + self.fConst97 * self.fRec21[1];
			self.fVec22[(self.IOTA0 & 16383) as usize] = 1e-20 + 0.35355338 * self.fRec17[0];
			let mut fTemp132: F32 = fTemp121 + self.fVec22[((i32::wrapping_sub(self.IOTA0, self.iConst101)) & 16383) as usize] - 0.6 * self.fRec28[1];
			self.fVec23[(self.IOTA0 & 4095) as usize] = fTemp132;
			let mut fRec22: F32 = 0.6 * fTemp132;
			self.fVec24[(self.IOTA0 & 16383) as usize] = 1e-20 + 0.35355338 * self.fRec15[0];
			let mut fTemp133: F32 = self.fVec24[((i32::wrapping_sub(self.IOTA0, self.iConst103)) & 16383) as usize] + fTemp130 + 0.6 * self.fRec27[1];
			self.fVec25[(self.IOTA0 & 4095) as usize] = fTemp133;
			let mut fRec23: F32 = -(0.6 * fTemp133);
			self.fRec24[0] = self.fConst114 * (self.fRec38[1] + self.fConst113 * self.fRec7[0]) + self.fConst112 * self.fRec24[1];
			self.fVec26[(self.IOTA0 & 32767) as usize] = 1e-20 + 0.35355338 * self.fRec21[0];
			let mut fTemp134: F32 = self.fVec26[((i32::wrapping_sub(self.IOTA0, self.iConst116)) & 32767) as usize] + fTemp130 + 0.6 * self.fRec25[1];
			self.fVec27[(self.IOTA0 & 4095) as usize] = fTemp134;
			self.fRec25[0] = self.fVec27[((i32::wrapping_sub(self.IOTA0, self.iConst117)) & 4095) as usize];
			let mut fRec26: F32 = 0.6 * fTemp131;
			self.fRec27[0] = self.fVec25[((i32::wrapping_sub(self.IOTA0, self.iConst118)) & 4095) as usize];
			self.fRec28[0] = self.fVec23[((i32::wrapping_sub(self.IOTA0, self.iConst119)) & 4095) as usize];
			let mut fRec29: F32 = -(0.6 * fTemp134);
			self.fVec28[(self.IOTA0 & 2047) as usize] = fTemp124 - fTemp130;
			self.fRec30[0] = self.fVec28[((i32::wrapping_sub(self.IOTA0, self.iConst120)) & 2047) as usize];
			self.fVec29[(self.IOTA0 & 32767) as usize] = 1e-20 + 0.35355338 * self.fRec24[0];
			let mut fTemp135: F32 = 0.6 * self.fRec32[1] + self.fVec29[((i32::wrapping_sub(self.IOTA0, self.iConst122)) & 32767) as usize];
			let mut fRec31: F32 = 0.6 * (fTemp130 - fTemp135);
			self.fVec30[(self.IOTA0 & 4095) as usize] = fTemp135 - fTemp130;
			self.fRec32[0] = self.fVec30[((i32::wrapping_sub(self.IOTA0, self.iConst123)) & 4095) as usize];
			let mut fTemp136: F32 = self.fRec18[1] + self.fRec30[1];
			self.fRec33[0] = fRec31 + fRec29 + fRec26 + fRec12 + self.fRec32[1] + self.fRec25[1] + self.fRec13[1] + self.fRec20[1] - (fRec19 + fRec23 + fRec11 + fRec22 + self.fRec27[1] + self.fRec28[1] + fTemp136);
			let mut fTemp137: F32 = self.fRec13[1] + self.fRec30[1];
			self.fRec34[0] = fRec19 + fRec23 + fRec26 + fRec12 + self.fRec27[1] + self.fRec20[1] + fTemp137 - (fRec31 + fRec29 + fRec11 + fRec22 + self.fRec32[1] + self.fRec25[1] + self.fRec18[1] + self.fRec28[1]);
			let mut fTemp138: F32 = self.fRec18[1] + self.fRec13[1];
			let mut fTemp139: F32 = self.fRec30[1] + fTemp138;
			self.fRec35[0] = fRec19 + fRec29 + fRec11 + fRec12 + self.fRec25[1] + fTemp139 - (fRec31 + fRec23 + fRec26 + fRec22 + self.fRec32[1] + self.fRec27[1] + self.fRec20[1] + self.fRec28[1]);
			self.fRec36[0] = fRec31 + fRec23 + fRec11 + fRec12 + self.fRec32[1] + self.fRec27[1] + fTemp138 - (fRec19 + fRec29 + fRec26 + fRec22 + self.fRec25[1] + self.fRec28[1] + self.fRec30[1] + self.fRec20[1]);
			self.fRec37[0] = fRec23 + fRec29 + fRec22 + fRec12 + self.fRec25[1] + self.fRec27[1] + self.fRec13[1] + self.fRec28[1] - (fRec19 + fRec31 + fRec11 + fRec26 + self.fRec32[1] + self.fRec20[1] + fTemp136);
			self.fRec38[0] = fRec19 + fRec31 + fRec22 + fRec12 + self.fRec32[1] + self.fRec28[1] + fTemp137 - (fRec23 + fRec29 + fRec11 + fRec26 + self.fRec25[1] + self.fRec27[1] + self.fRec18[1] + self.fRec20[1]);
			self.fRec39[0] = fRec11 + fRec26 + fRec22 + fRec12 + self.fRec28[1] + self.fRec20[1] + fTemp138 - (fRec19 + fRec31 + fRec23 + fRec29 + self.fRec32[1] + self.fRec25[1] + self.fRec30[1] + self.fRec27[1]);
			self.fRec40[0] = fRec19 + fRec31 + fRec23 + fRec29 + fRec11 + fRec26 + fRec22 + fRec12 + self.fRec32[1] + self.fRec25[1] + self.fRec27[1] + self.fRec28[1] + self.fRec20[1] + fTemp139;
			self.fRec104[0] = fSlow28 + self.fConst2 * self.fRec104[1];
			let mut fTemp140: F32 = 1.0 - self.fRec104[0];
			*output0 = (F32::tanh((0.9 * (fTemp120 * fTemp140 / fTemp6) + 0.37 * self.fRec104[0] * (self.fRec39[0] + self.fRec37[0])) * self.fRec0[0])) as FaustFloat;
			*output1 = (F32::tanh(self.fRec0[0] * (0.9 * fTemp140 * fTemp129 + 0.37 * self.fRec104[0] * (self.fRec39[0] - self.fRec37[0])))) as FaustFloat;
			self.iVec0[1] = self.iVec0[0];
			self.fRec0[1] = self.fRec0[0];
			self.fRec1[1] = self.fRec1[0];
			self.fRec2[1] = self.fRec2[0];
			self.fRec3[1] = self.fRec3[0];
			self.fRec4[1] = self.fRec4[0];
			self.fRec5[1] = self.fRec5[0];
			self.fRec6[1] = self.fRec6[0];
			self.fRec7[1] = self.fRec7[0];
			self.fRec8[1] = self.fRec8[0];
			self.fRec9[1] = self.fRec9[0];
			self.fRec10[1] = self.fRec10[0];
			self.fRec41[1] = self.fRec41[0];
			self.fRec42[1] = self.fRec42[0];
			self.fRec47[1] = self.fRec47[0];
			self.fRec48[1] = self.fRec48[0];
			self.fRec49[1] = self.fRec49[0];
			self.fRec50[1] = self.fRec50[0];
			self.fRec51[1] = self.fRec51[0];
			self.fRec52[1] = self.fRec52[0];
			self.fRec53[1] = self.fRec53[0];
			self.iRec55[1] = self.iRec55[0];
			self.fVec1[1] = self.fVec1[0];
			self.fRec54[1] = self.fRec54[0];
			self.fRec56[1] = self.fRec56[0];
			self.fRec57[1] = self.fRec57[0];
			self.fRec58[1] = self.fRec58[0];
			self.fVec2[1] = self.fVec2[0];
			self.fRec59[1] = self.fRec59[0];
			self.fRec60[1] = self.fRec60[0];
			self.fRec61[1] = self.fRec61[0];
			self.fVec3[1] = self.fVec3[0];
			self.fRec62[1] = self.fRec62[0];
			self.fRec63[1] = self.fRec63[0];
			self.fRec64[1] = self.fRec64[0];
			self.fRec65[1] = self.fRec65[0];
			self.fVec4[1] = self.fVec4[0];
			self.fRec66[1] = self.fRec66[0];
			self.fRec67[1] = self.fRec67[0];
			self.fRec68[1] = self.fRec68[0];
			self.fVec5[1] = self.fVec5[0];
			self.fRec69[1] = self.fRec69[0];
			self.fRec70[1] = self.fRec70[0];
			self.fRec71[1] = self.fRec71[0];
			self.fRec72[1] = self.fRec72[0];
			self.fVec6[1] = self.fVec6[0];
			self.fRec73[1] = self.fRec73[0];
			self.fRec74[1] = self.fRec74[0];
			self.fRec75[1] = self.fRec75[0];
			self.fVec7[1] = self.fVec7[0];
			self.fRec76[1] = self.fRec76[0];
			self.fRec77[1] = self.fRec77[0];
			self.fRec78[1] = self.fRec78[0];
			self.fRec79[1] = self.fRec79[0];
			self.fVec8[1] = self.fVec8[0];
			self.fRec80[1] = self.fRec80[0];
			self.fRec81[1] = self.fRec81[0];
			self.fRec82[1] = self.fRec82[0];
			self.fVec9[1] = self.fVec9[0];
			self.fRec83[1] = self.fRec83[0];
			self.fRec84[1] = self.fRec84[0];
			self.fRec85[1] = self.fRec85[0];
			self.fRec86[1] = self.fRec86[0];
			self.fVec10[1] = self.fVec10[0];
			self.fRec87[1] = self.fRec87[0];
			self.fRec88[1] = self.fRec88[0];
			self.fRec89[1] = self.fRec89[0];
			self.fVec11[1] = self.fVec11[0];
			self.fRec90[1] = self.fRec90[0];
			self.fRec91[1] = self.fRec91[0];
			self.fRec92[1] = self.fRec92[0];
			self.fRec93[1] = self.fRec93[0];
			self.fVec12[1] = self.fVec12[0];
			self.fRec94[1] = self.fRec94[0];
			self.fRec95[1] = self.fRec95[0];
			self.fRec96[1] = self.fRec96[0];
			self.fVec13[1] = self.fVec13[0];
			self.fRec99[1] = self.fRec99[0];
			self.fRec97[1] = self.fRec97[0];
			self.fRec98[1] = self.fRec98[0];
			self.fRec45[1] = self.fRec45[0];
			self.fRec46[1] = self.fRec46[0];
			self.fRec43[1] = self.fRec43[0];
			self.fRec44[1] = self.fRec44[0];
			self.IOTA0 = i32::wrapping_add(self.IOTA0, 1);
			self.fRec13[1] = self.fRec13[0];
			self.fRec14[1] = self.fRec14[0];
			self.fRec15[1] = self.fRec15[0];
			self.fRec16[1] = self.fRec16[0];
			self.fRec17[1] = self.fRec17[0];
			self.fRec18[1] = self.fRec18[0];
			self.fRec102[1] = self.fRec102[0];
			self.fRec103[1] = self.fRec103[0];
			self.fRec100[1] = self.fRec100[0];
			self.fRec101[1] = self.fRec101[0];
			self.fRec20[1] = self.fRec20[0];
			self.fRec21[1] = self.fRec21[0];
			self.fRec24[1] = self.fRec24[0];
			self.fRec25[1] = self.fRec25[0];
			self.fRec27[1] = self.fRec27[0];
			self.fRec28[1] = self.fRec28[0];
			self.fRec30[1] = self.fRec30[0];
			self.fRec32[1] = self.fRec32[0];
			self.fRec33[2] = self.fRec33[1];
			self.fRec33[1] = self.fRec33[0];
			self.fRec34[2] = self.fRec34[1];
			self.fRec34[1] = self.fRec34[0];
			self.fRec35[2] = self.fRec35[1];
			self.fRec35[1] = self.fRec35[0];
			self.fRec36[2] = self.fRec36[1];
			self.fRec36[1] = self.fRec36[0];
			self.fRec37[2] = self.fRec37[1];
			self.fRec37[1] = self.fRec37[0];
			self.fRec38[2] = self.fRec38[1];
			self.fRec38[1] = self.fRec38[0];
			self.fRec39[2] = self.fRec39[1];
			self.fRec39[1] = self.fRec39[0];
			self.fRec40[2] = self.fRec40[1];
			self.fRec40[1] = self.fRec40[0];
			self.fRec104[1] = self.fRec104[0];
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
