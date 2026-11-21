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
	fConst4: F32,
	fConst5: F32,
	fConst6: F32,
	fRec6: [F32;2],
	fRec7: [F32;2],
	fRec8: [F32;2],
	fRec9: [F32;2],
	fRec10: [F32;2],
	fRec11: [F32;2],
	fRec12: [F32;2],
	fRec13: [F32;2],
	fHslider2: FaustFloat,
	fRec46: [F32;2],
	fConst7: F32,
	fConst8: F32,
	fConst9: F32,
	fHslider3: FaustFloat,
	fRec47: [F32;2],
	fConst10: F32,
	fConst11: F32,
	fRec14: [F32;2],
	fConst12: F32,
	fConst13: F32,
	fConst14: F32,
	fConst15: F32,
	fRec15: [F32;2],
	fConst16: F32,
	fConst17: F32,
	fConst18: F32,
	fConst19: F32,
	fRec16: [F32;2],
	IOTA0: i32,
	fVec1: [F32;16384],
	fConst20: F32,
	iConst21: i32,
	fConst22: F32,
	fConst23: F32,
	fConst24: F32,
	fHslider4: FaustFloat,
	fRec49: [F32;2],
	fConst25: F32,
	fHslider5: FaustFloat,
	fRec50: [F32;2],
	fHslider6: FaustFloat,
	fRec55: [F32;2],
	fHslider7: FaustFloat,
	fRec56: [F32;2],
	fHslider8: FaustFloat,
	fRec57: [F32;2],
	fHslider9: FaustFloat,
	fRec58: [F32;2],
	fHslider10: FaustFloat,
	fRec59: [F32;2],
	fHslider11: FaustFloat,
	fRec60: [F32;2],
	fHslider12: FaustFloat,
	fRec61: [F32;2],
	fConst26: F32,
	fConst27: F32,
	iRec63: [i32;2],
	fVec2: [F32;2],
	fConst28: F32,
	fRec62: [F32;2],
	fHslider13: FaustFloat,
	fRec64: [F32;2],
	fHslider14: FaustFloat,
	fRec65: [F32;2],
	fConst29: F32,
	fRec66: [F32;2],
	fVec3: [F32;2],
	fRec67: [F32;2],
	fRec68: [F32;2],
	fHslider15: FaustFloat,
	fRec69: [F32;2],
	fVec4: [F32;2],
	fRec70: [F32;2],
	fHslider16: FaustFloat,
	fRec71: [F32;2],
	fHslider17: FaustFloat,
	fRec72: [F32;2],
	fRec73: [F32;2],
	fVec5: [F32;2],
	fRec74: [F32;2],
	fRec75: [F32;2],
	fHslider18: FaustFloat,
	fRec76: [F32;2],
	fVec6: [F32;2],
	fRec77: [F32;2],
	fHslider19: FaustFloat,
	fRec78: [F32;2],
	fHslider20: FaustFloat,
	fRec79: [F32;2],
	fRec80: [F32;2],
	fVec7: [F32;2],
	fRec81: [F32;2],
	fRec82: [F32;2],
	fHslider21: FaustFloat,
	fRec83: [F32;2],
	fVec8: [F32;2],
	fRec84: [F32;2],
	fHslider22: FaustFloat,
	fRec85: [F32;2],
	fHslider23: FaustFloat,
	fRec86: [F32;2],
	fRec87: [F32;2],
	fVec9: [F32;2],
	fRec88: [F32;2],
	fRec89: [F32;2],
	fHslider24: FaustFloat,
	fRec90: [F32;2],
	fVec10: [F32;2],
	fRec91: [F32;2],
	fHslider25: FaustFloat,
	fRec92: [F32;2],
	fHslider26: FaustFloat,
	fRec93: [F32;2],
	fRec94: [F32;2],
	fVec11: [F32;2],
	fRec95: [F32;2],
	fRec96: [F32;2],
	fHslider27: FaustFloat,
	fRec97: [F32;2],
	fVec12: [F32;2],
	fRec98: [F32;2],
	fHslider28: FaustFloat,
	fRec99: [F32;2],
	fHslider29: FaustFloat,
	fRec100: [F32;2],
	fRec101: [F32;2],
	fVec13: [F32;2],
	fRec102: [F32;2],
	fRec103: [F32;2],
	fHslider30: FaustFloat,
	fRec104: [F32;2],
	fVec14: [F32;2],
	fRec107: [F32;2],
	fRec105: [F32;2],
	fRec106: [F32;2],
	fRec53: [F32;2],
	fRec54: [F32;2],
	fRec51: [F32;2],
	fRec52: [F32;2],
	fRec48: [F32;3],
	fConst30: F32,
	fConst31: F32,
	fConst32: F32,
	fConst33: F32,
	fConst34: F32,
	fRec108: [F32;3],
	fRec110: [F32;3],
	fRec111: [F32;3],
	fConst35: F32,
	fConst36: F32,
	fRec109: [F32;2],
	fHslider31: FaustFloat,
	fRec112: [F32;2],
	fRec113: [F32;2],
	fHslider32: FaustFloat,
	fRec114: [F32;2],
	fRec115: [F32;3],
	fHslider33: FaustFloat,
	fRec116: [F32;2],
	fVec15: [F32;16384],
	fHslider34: FaustFloat,
	fRec117: [F32;2],
	fConst37: F32,
	fVec16: [F32;4096],
	iConst38: i32,
	fRec17: [F32;2],
	fConst39: F32,
	fConst40: F32,
	fConst41: F32,
	fConst42: F32,
	fRec18: [F32;2],
	fVec17: [F32;32768],
	fConst43: F32,
	iConst44: i32,
	fVec18: [F32;2048],
	iConst45: i32,
	fRec19: [F32;2],
	fConst46: F32,
	fConst47: F32,
	fConst48: F32,
	fConst49: F32,
	fRec20: [F32;2],
	fVec19: [F32;32768],
	fConst50: F32,
	iConst51: i32,
	fRec121: [F32;2],
	fRec122: [F32;2],
	fRec119: [F32;2],
	fRec120: [F32;2],
	fRec118: [F32;3],
	fRec123: [F32;3],
	fRec124: [F32;3],
	fVec20: [F32;16384],
	fVec21: [F32;16384],
	fConst52: F32,
	iConst53: i32,
	fVec22: [F32;2048],
	iConst54: i32,
	fRec22: [F32;2],
	fConst55: F32,
	fConst56: F32,
	fConst57: F32,
	fConst58: F32,
	fRec23: [F32;2],
	fConst59: F32,
	fConst60: F32,
	fConst61: F32,
	fConst62: F32,
	fRec24: [F32;2],
	fVec23: [F32;32768],
	fConst63: F32,
	iConst64: i32,
	fVec24: [F32;4096],
	iConst65: i32,
	fRec25: [F32;2],
	fConst66: F32,
	fConst67: F32,
	fConst68: F32,
	fConst69: F32,
	fRec26: [F32;2],
	fVec25: [F32;16384],
	fConst70: F32,
	iConst71: i32,
	fVec26: [F32;2048],
	iConst72: i32,
	fRec30: [F32;2],
	fVec27: [F32;16384],
	fConst73: F32,
	iConst74: i32,
	fVec28: [F32;4096],
	fVec29: [F32;16384],
	fConst75: F32,
	iConst76: i32,
	fVec30: [F32;4096],
	fVec31: [F32;4096],
	iConst77: i32,
	fRec35: [F32;2],
	iConst78: i32,
	fRec36: [F32;2],
	iConst79: i32,
	fRec37: [F32;2],
	fRec38: [F32;3],
	fRec39: [F32;3],
	fRec40: [F32;3],
	fRec41: [F32;3],
	fRec42: [F32;3],
	fRec43: [F32;3],
	fRec44: [F32;3],
	fRec45: [F32;3],
	fHslider35: FaustFloat,
	fRec125: [F32;2],
	fHslider36: FaustFloat,
	fRec128: [F32;2],
	fConst80: F32,
	fRec129: [F32;2],
	fRec127: [F32;2],
	fRec126: [F32;3],
	fConst81: F32,
	fConst82: F32,
	fConst83: F32,
	fRec3: [F32;2],
	fConst84: F32,
	fRec4: [F32;2],
	fConst85: F32,
	fConst86: F32,
	fRec1: [F32;2],
	fRec2: [F32;2],
	fRec136: [F32;2],
	fRec135: [F32;2],
	fRec134: [F32;3],
	fRec132: [F32;2],
	fRec133: [F32;2],
	fRec130: [F32;2],
	fRec131: [F32;2],
	fConst87: F32,
	fRec139: [F32;2],
	fRec140: [F32;2],
	fRec137: [F32;2],
	fRec138: [F32;2],
}


fn MoodscoreDsp_faustpower2_f(value: F32) -> F32 {
	return value * value;
}
pub const FAUST_INPUTS: usize = 4;
pub const FAUST_OUTPUTS: usize = 3;
pub const FAUST_ACTIVES: usize = 37;
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
			fConst4: 0.0,
			fConst5: 0.0,
			fConst6: 0.0,
			fRec6: [0.0;2],
			fRec7: [0.0;2],
			fRec8: [0.0;2],
			fRec9: [0.0;2],
			fRec10: [0.0;2],
			fRec11: [0.0;2],
			fRec12: [0.0;2],
			fRec13: [0.0;2],
			fHslider2: 0.0,
			fRec46: [0.0;2],
			fConst7: 0.0,
			fConst8: 0.0,
			fConst9: 0.0,
			fHslider3: 0.0,
			fRec47: [0.0;2],
			fConst10: 0.0,
			fConst11: 0.0,
			fRec14: [0.0;2],
			fConst12: 0.0,
			fConst13: 0.0,
			fConst14: 0.0,
			fConst15: 0.0,
			fRec15: [0.0;2],
			fConst16: 0.0,
			fConst17: 0.0,
			fConst18: 0.0,
			fConst19: 0.0,
			fRec16: [0.0;2],
			IOTA0: 0,
			fVec1: [0.0;16384],
			fConst20: 0.0,
			iConst21: 0,
			fConst22: 0.0,
			fConst23: 0.0,
			fConst24: 0.0,
			fHslider4: 0.0,
			fRec49: [0.0;2],
			fConst25: 0.0,
			fHslider5: 0.0,
			fRec50: [0.0;2],
			fHslider6: 0.0,
			fRec55: [0.0;2],
			fHslider7: 0.0,
			fRec56: [0.0;2],
			fHslider8: 0.0,
			fRec57: [0.0;2],
			fHslider9: 0.0,
			fRec58: [0.0;2],
			fHslider10: 0.0,
			fRec59: [0.0;2],
			fHslider11: 0.0,
			fRec60: [0.0;2],
			fHslider12: 0.0,
			fRec61: [0.0;2],
			fConst26: 0.0,
			fConst27: 0.0,
			iRec63: [0;2],
			fVec2: [0.0;2],
			fConst28: 0.0,
			fRec62: [0.0;2],
			fHslider13: 0.0,
			fRec64: [0.0;2],
			fHslider14: 0.0,
			fRec65: [0.0;2],
			fConst29: 0.0,
			fRec66: [0.0;2],
			fVec3: [0.0;2],
			fRec67: [0.0;2],
			fRec68: [0.0;2],
			fHslider15: 0.0,
			fRec69: [0.0;2],
			fVec4: [0.0;2],
			fRec70: [0.0;2],
			fHslider16: 0.0,
			fRec71: [0.0;2],
			fHslider17: 0.0,
			fRec72: [0.0;2],
			fRec73: [0.0;2],
			fVec5: [0.0;2],
			fRec74: [0.0;2],
			fRec75: [0.0;2],
			fHslider18: 0.0,
			fRec76: [0.0;2],
			fVec6: [0.0;2],
			fRec77: [0.0;2],
			fHslider19: 0.0,
			fRec78: [0.0;2],
			fHslider20: 0.0,
			fRec79: [0.0;2],
			fRec80: [0.0;2],
			fVec7: [0.0;2],
			fRec81: [0.0;2],
			fRec82: [0.0;2],
			fHslider21: 0.0,
			fRec83: [0.0;2],
			fVec8: [0.0;2],
			fRec84: [0.0;2],
			fHslider22: 0.0,
			fRec85: [0.0;2],
			fHslider23: 0.0,
			fRec86: [0.0;2],
			fRec87: [0.0;2],
			fVec9: [0.0;2],
			fRec88: [0.0;2],
			fRec89: [0.0;2],
			fHslider24: 0.0,
			fRec90: [0.0;2],
			fVec10: [0.0;2],
			fRec91: [0.0;2],
			fHslider25: 0.0,
			fRec92: [0.0;2],
			fHslider26: 0.0,
			fRec93: [0.0;2],
			fRec94: [0.0;2],
			fVec11: [0.0;2],
			fRec95: [0.0;2],
			fRec96: [0.0;2],
			fHslider27: 0.0,
			fRec97: [0.0;2],
			fVec12: [0.0;2],
			fRec98: [0.0;2],
			fHslider28: 0.0,
			fRec99: [0.0;2],
			fHslider29: 0.0,
			fRec100: [0.0;2],
			fRec101: [0.0;2],
			fVec13: [0.0;2],
			fRec102: [0.0;2],
			fRec103: [0.0;2],
			fHslider30: 0.0,
			fRec104: [0.0;2],
			fVec14: [0.0;2],
			fRec107: [0.0;2],
			fRec105: [0.0;2],
			fRec106: [0.0;2],
			fRec53: [0.0;2],
			fRec54: [0.0;2],
			fRec51: [0.0;2],
			fRec52: [0.0;2],
			fRec48: [0.0;3],
			fConst30: 0.0,
			fConst31: 0.0,
			fConst32: 0.0,
			fConst33: 0.0,
			fConst34: 0.0,
			fRec108: [0.0;3],
			fRec110: [0.0;3],
			fRec111: [0.0;3],
			fConst35: 0.0,
			fConst36: 0.0,
			fRec109: [0.0;2],
			fHslider31: 0.0,
			fRec112: [0.0;2],
			fRec113: [0.0;2],
			fHslider32: 0.0,
			fRec114: [0.0;2],
			fRec115: [0.0;3],
			fHslider33: 0.0,
			fRec116: [0.0;2],
			fVec15: [0.0;16384],
			fHslider34: 0.0,
			fRec117: [0.0;2],
			fConst37: 0.0,
			fVec16: [0.0;4096],
			iConst38: 0,
			fRec17: [0.0;2],
			fConst39: 0.0,
			fConst40: 0.0,
			fConst41: 0.0,
			fConst42: 0.0,
			fRec18: [0.0;2],
			fVec17: [0.0;32768],
			fConst43: 0.0,
			iConst44: 0,
			fVec18: [0.0;2048],
			iConst45: 0,
			fRec19: [0.0;2],
			fConst46: 0.0,
			fConst47: 0.0,
			fConst48: 0.0,
			fConst49: 0.0,
			fRec20: [0.0;2],
			fVec19: [0.0;32768],
			fConst50: 0.0,
			iConst51: 0,
			fRec121: [0.0;2],
			fRec122: [0.0;2],
			fRec119: [0.0;2],
			fRec120: [0.0;2],
			fRec118: [0.0;3],
			fRec123: [0.0;3],
			fRec124: [0.0;3],
			fVec20: [0.0;16384],
			fVec21: [0.0;16384],
			fConst52: 0.0,
			iConst53: 0,
			fVec22: [0.0;2048],
			iConst54: 0,
			fRec22: [0.0;2],
			fConst55: 0.0,
			fConst56: 0.0,
			fConst57: 0.0,
			fConst58: 0.0,
			fRec23: [0.0;2],
			fConst59: 0.0,
			fConst60: 0.0,
			fConst61: 0.0,
			fConst62: 0.0,
			fRec24: [0.0;2],
			fVec23: [0.0;32768],
			fConst63: 0.0,
			iConst64: 0,
			fVec24: [0.0;4096],
			iConst65: 0,
			fRec25: [0.0;2],
			fConst66: 0.0,
			fConst67: 0.0,
			fConst68: 0.0,
			fConst69: 0.0,
			fRec26: [0.0;2],
			fVec25: [0.0;16384],
			fConst70: 0.0,
			iConst71: 0,
			fVec26: [0.0;2048],
			iConst72: 0,
			fRec30: [0.0;2],
			fVec27: [0.0;16384],
			fConst73: 0.0,
			iConst74: 0,
			fVec28: [0.0;4096],
			fVec29: [0.0;16384],
			fConst75: 0.0,
			iConst76: 0,
			fVec30: [0.0;4096],
			fVec31: [0.0;4096],
			iConst77: 0,
			fRec35: [0.0;2],
			iConst78: 0,
			fRec36: [0.0;2],
			iConst79: 0,
			fRec37: [0.0;2],
			fRec38: [0.0;3],
			fRec39: [0.0;3],
			fRec40: [0.0;3],
			fRec41: [0.0;3],
			fRec42: [0.0;3],
			fRec43: [0.0;3],
			fRec44: [0.0;3],
			fRec45: [0.0;3],
			fHslider35: 0.0,
			fRec125: [0.0;2],
			fHslider36: 0.0,
			fRec128: [0.0;2],
			fConst80: 0.0,
			fRec129: [0.0;2],
			fRec127: [0.0;2],
			fRec126: [0.0;3],
			fConst81: 0.0,
			fConst82: 0.0,
			fConst83: 0.0,
			fRec3: [0.0;2],
			fConst84: 0.0,
			fRec4: [0.0;2],
			fConst85: 0.0,
			fConst86: 0.0,
			fRec1: [0.0;2],
			fRec2: [0.0;2],
			fRec136: [0.0;2],
			fRec135: [0.0;2],
			fRec134: [0.0;3],
			fRec132: [0.0;2],
			fRec133: [0.0;2],
			fRec130: [0.0;2],
			fRec131: [0.0;2],
			fConst87: 0.0,
			fRec139: [0.0;2],
			fRec140: [0.0;2],
			fRec137: [0.0;2],
			fRec138: [0.0;2],
		}
	}
	pub fn metadata(&self, m: &mut dyn Meta) { 
		m.declare("analyzers.lib/amp_follower_ar:author", r"Jonatan Liljedahl, revised by Romain Michon");
		m.declare("analyzers.lib/name", r"Faust Analyzer Library");
		m.declare("analyzers.lib/version", r"1.4.0");
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
		m.declare("filters.lib/fir:author", r"Julius O. Smith III");
		m.declare("filters.lib/fir:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/fir:license", r"LicenseRef-STK-4.3");
		m.declare("filters.lib/highpassLR4:author", r"Dario Sanfilippo");
		m.declare("filters.lib/highpassLR4:copyright", r"Copyright (C) 2022 Dario Sanfilippo <sanfilippo.dario@gmail.com>");
		m.declare("filters.lib/highpassLR4:license", r"LicenseRef-STK-4.3");
		m.declare("filters.lib/iir:author", r"Julius O. Smith III");
		m.declare("filters.lib/iir:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/iir:license", r"LicenseRef-STK-4.3");
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
		m.declare("filters.lib/pole:author", r"Julius O. Smith III");
		m.declare("filters.lib/pole:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/pole:license", r"LicenseRef-STK-4.3");
		m.declare("filters.lib/svf:author", r"Oleg Nesterov");
		m.declare("filters.lib/svf:copyright", r"Copyright (C) 2020 Oleg Nesterov <oleg@redhat.com>");
		m.declare("filters.lib/svf:license", r"LicenseRef-STK-4.3");
		m.declare("filters.lib/tf1:author", r"Julius O. Smith III");
		m.declare("filters.lib/tf1:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf1:license", r"LicenseRef-STK-4.3");
		m.declare("filters.lib/tf1s:author", r"Julius O. Smith III");
		m.declare("filters.lib/tf1s:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf1s:license", r"LicenseRef-STK-4.3");
		m.declare("filters.lib/tf2:author", r"Julius O. Smith III");
		m.declare("filters.lib/tf2:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf2:license", r"LicenseRef-STK-4.3");
		m.declare("filters.lib/tf2np:author", r"Julius O. Smith III");
		m.declare("filters.lib/tf2np:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf2np:license", r"LicenseRef-STK-4.3");
		m.declare("filters.lib/tf2s:author", r"Julius O. Smith III");
		m.declare("filters.lib/tf2s:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf2s:license", r"LicenseRef-STK-4.3");
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
		m.declare("signals.lib/onePoleSwitching:author", r"Jonatan Liljedahl, revised by Dario Sanfilippo");
		m.declare("signals.lib/onePoleSwitching:licence", r"LicenseRef-STK-4.3");
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
		self.fHslider1 = (1.0) as FaustFloat;
		self.fHslider2 = (8.0) as FaustFloat;
		self.fHslider3 = (6e+03) as FaustFloat;
		self.fHslider4 = (9e+02) as FaustFloat;
		self.fHslider5 = (0.3) as FaustFloat;
		self.fHslider6 = (0.0) as FaustFloat;
		self.fHslider7 = (0.0) as FaustFloat;
		self.fHslider8 = (0.0) as FaustFloat;
		self.fHslider9 = (0.0) as FaustFloat;
		self.fHslider10 = (0.3) as FaustFloat;
		self.fHslider11 = (0.7) as FaustFloat;
		self.fHslider12 = (7.0) as FaustFloat;
		self.fHslider13 = (0.3) as FaustFloat;
		self.fHslider14 = (2.2e+02) as FaustFloat;
		self.fHslider15 = (7.0) as FaustFloat;
		self.fHslider16 = (0.3) as FaustFloat;
		self.fHslider17 = (164.81) as FaustFloat;
		self.fHslider18 = (7.0) as FaustFloat;
		self.fHslider19 = (0.3) as FaustFloat;
		self.fHslider20 = (130.81) as FaustFloat;
		self.fHslider21 = (7.0) as FaustFloat;
		self.fHslider22 = (0.3) as FaustFloat;
		self.fHslider23 = (1.1e+02) as FaustFloat;
		self.fHslider24 = (7.0) as FaustFloat;
		self.fHslider25 = (0.3) as FaustFloat;
		self.fHslider26 = (82.41) as FaustFloat;
		self.fHslider27 = (7.0) as FaustFloat;
		self.fHslider28 = (0.3) as FaustFloat;
		self.fHslider29 = (55.0) as FaustFloat;
		self.fHslider30 = (0.5) as FaustFloat;
		self.fHslider31 = (0.5) as FaustFloat;
		self.fHslider32 = (0.35) as FaustFloat;
		self.fHslider33 = (0.0) as FaustFloat;
		self.fHslider34 = (0.0) as FaustFloat;
		self.fHslider35 = (1.0) as FaustFloat;
		self.fHslider36 = (0.0) as FaustFloat;
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
			self.fRec7[l4 as usize] = 0.0;
		}
		for l5 in 0..2 {
			self.fRec8[l5 as usize] = 0.0;
		}
		for l6 in 0..2 {
			self.fRec9[l6 as usize] = 0.0;
		}
		for l7 in 0..2 {
			self.fRec10[l7 as usize] = 0.0;
		}
		for l8 in 0..2 {
			self.fRec11[l8 as usize] = 0.0;
		}
		for l9 in 0..2 {
			self.fRec12[l9 as usize] = 0.0;
		}
		for l10 in 0..2 {
			self.fRec13[l10 as usize] = 0.0;
		}
		for l11 in 0..2 {
			self.fRec46[l11 as usize] = 0.0;
		}
		for l12 in 0..2 {
			self.fRec47[l12 as usize] = 0.0;
		}
		for l13 in 0..2 {
			self.fRec14[l13 as usize] = 0.0;
		}
		for l14 in 0..2 {
			self.fRec15[l14 as usize] = 0.0;
		}
		for l15 in 0..2 {
			self.fRec16[l15 as usize] = 0.0;
		}
		self.IOTA0 = 0;
		for l16 in 0..16384 {
			self.fVec1[l16 as usize] = 0.0;
		}
		for l17 in 0..2 {
			self.fRec49[l17 as usize] = 0.0;
		}
		for l18 in 0..2 {
			self.fRec50[l18 as usize] = 0.0;
		}
		for l19 in 0..2 {
			self.fRec55[l19 as usize] = 0.0;
		}
		for l20 in 0..2 {
			self.fRec56[l20 as usize] = 0.0;
		}
		for l21 in 0..2 {
			self.fRec57[l21 as usize] = 0.0;
		}
		for l22 in 0..2 {
			self.fRec58[l22 as usize] = 0.0;
		}
		for l23 in 0..2 {
			self.fRec59[l23 as usize] = 0.0;
		}
		for l24 in 0..2 {
			self.fRec60[l24 as usize] = 0.0;
		}
		for l25 in 0..2 {
			self.fRec61[l25 as usize] = 0.0;
		}
		for l26 in 0..2 {
			self.iRec63[l26 as usize] = 0;
		}
		for l27 in 0..2 {
			self.fVec2[l27 as usize] = 0.0;
		}
		for l28 in 0..2 {
			self.fRec62[l28 as usize] = 0.0;
		}
		for l29 in 0..2 {
			self.fRec64[l29 as usize] = 0.0;
		}
		for l30 in 0..2 {
			self.fRec65[l30 as usize] = 0.0;
		}
		for l31 in 0..2 {
			self.fRec66[l31 as usize] = 0.0;
		}
		for l32 in 0..2 {
			self.fVec3[l32 as usize] = 0.0;
		}
		for l33 in 0..2 {
			self.fRec67[l33 as usize] = 0.0;
		}
		for l34 in 0..2 {
			self.fRec68[l34 as usize] = 0.0;
		}
		for l35 in 0..2 {
			self.fRec69[l35 as usize] = 0.0;
		}
		for l36 in 0..2 {
			self.fVec4[l36 as usize] = 0.0;
		}
		for l37 in 0..2 {
			self.fRec70[l37 as usize] = 0.0;
		}
		for l38 in 0..2 {
			self.fRec71[l38 as usize] = 0.0;
		}
		for l39 in 0..2 {
			self.fRec72[l39 as usize] = 0.0;
		}
		for l40 in 0..2 {
			self.fRec73[l40 as usize] = 0.0;
		}
		for l41 in 0..2 {
			self.fVec5[l41 as usize] = 0.0;
		}
		for l42 in 0..2 {
			self.fRec74[l42 as usize] = 0.0;
		}
		for l43 in 0..2 {
			self.fRec75[l43 as usize] = 0.0;
		}
		for l44 in 0..2 {
			self.fRec76[l44 as usize] = 0.0;
		}
		for l45 in 0..2 {
			self.fVec6[l45 as usize] = 0.0;
		}
		for l46 in 0..2 {
			self.fRec77[l46 as usize] = 0.0;
		}
		for l47 in 0..2 {
			self.fRec78[l47 as usize] = 0.0;
		}
		for l48 in 0..2 {
			self.fRec79[l48 as usize] = 0.0;
		}
		for l49 in 0..2 {
			self.fRec80[l49 as usize] = 0.0;
		}
		for l50 in 0..2 {
			self.fVec7[l50 as usize] = 0.0;
		}
		for l51 in 0..2 {
			self.fRec81[l51 as usize] = 0.0;
		}
		for l52 in 0..2 {
			self.fRec82[l52 as usize] = 0.0;
		}
		for l53 in 0..2 {
			self.fRec83[l53 as usize] = 0.0;
		}
		for l54 in 0..2 {
			self.fVec8[l54 as usize] = 0.0;
		}
		for l55 in 0..2 {
			self.fRec84[l55 as usize] = 0.0;
		}
		for l56 in 0..2 {
			self.fRec85[l56 as usize] = 0.0;
		}
		for l57 in 0..2 {
			self.fRec86[l57 as usize] = 0.0;
		}
		for l58 in 0..2 {
			self.fRec87[l58 as usize] = 0.0;
		}
		for l59 in 0..2 {
			self.fVec9[l59 as usize] = 0.0;
		}
		for l60 in 0..2 {
			self.fRec88[l60 as usize] = 0.0;
		}
		for l61 in 0..2 {
			self.fRec89[l61 as usize] = 0.0;
		}
		for l62 in 0..2 {
			self.fRec90[l62 as usize] = 0.0;
		}
		for l63 in 0..2 {
			self.fVec10[l63 as usize] = 0.0;
		}
		for l64 in 0..2 {
			self.fRec91[l64 as usize] = 0.0;
		}
		for l65 in 0..2 {
			self.fRec92[l65 as usize] = 0.0;
		}
		for l66 in 0..2 {
			self.fRec93[l66 as usize] = 0.0;
		}
		for l67 in 0..2 {
			self.fRec94[l67 as usize] = 0.0;
		}
		for l68 in 0..2 {
			self.fVec11[l68 as usize] = 0.0;
		}
		for l69 in 0..2 {
			self.fRec95[l69 as usize] = 0.0;
		}
		for l70 in 0..2 {
			self.fRec96[l70 as usize] = 0.0;
		}
		for l71 in 0..2 {
			self.fRec97[l71 as usize] = 0.0;
		}
		for l72 in 0..2 {
			self.fVec12[l72 as usize] = 0.0;
		}
		for l73 in 0..2 {
			self.fRec98[l73 as usize] = 0.0;
		}
		for l74 in 0..2 {
			self.fRec99[l74 as usize] = 0.0;
		}
		for l75 in 0..2 {
			self.fRec100[l75 as usize] = 0.0;
		}
		for l76 in 0..2 {
			self.fRec101[l76 as usize] = 0.0;
		}
		for l77 in 0..2 {
			self.fVec13[l77 as usize] = 0.0;
		}
		for l78 in 0..2 {
			self.fRec102[l78 as usize] = 0.0;
		}
		for l79 in 0..2 {
			self.fRec103[l79 as usize] = 0.0;
		}
		for l80 in 0..2 {
			self.fRec104[l80 as usize] = 0.0;
		}
		for l81 in 0..2 {
			self.fVec14[l81 as usize] = 0.0;
		}
		for l82 in 0..2 {
			self.fRec107[l82 as usize] = 0.0;
		}
		for l83 in 0..2 {
			self.fRec105[l83 as usize] = 0.0;
		}
		for l84 in 0..2 {
			self.fRec106[l84 as usize] = 0.0;
		}
		for l85 in 0..2 {
			self.fRec53[l85 as usize] = 0.0;
		}
		for l86 in 0..2 {
			self.fRec54[l86 as usize] = 0.0;
		}
		for l87 in 0..2 {
			self.fRec51[l87 as usize] = 0.0;
		}
		for l88 in 0..2 {
			self.fRec52[l88 as usize] = 0.0;
		}
		for l89 in 0..3 {
			self.fRec48[l89 as usize] = 0.0;
		}
		for l90 in 0..3 {
			self.fRec108[l90 as usize] = 0.0;
		}
		for l91 in 0..3 {
			self.fRec110[l91 as usize] = 0.0;
		}
		for l92 in 0..3 {
			self.fRec111[l92 as usize] = 0.0;
		}
		for l93 in 0..2 {
			self.fRec109[l93 as usize] = 0.0;
		}
		for l94 in 0..2 {
			self.fRec112[l94 as usize] = 0.0;
		}
		for l95 in 0..2 {
			self.fRec113[l95 as usize] = 0.0;
		}
		for l96 in 0..2 {
			self.fRec114[l96 as usize] = 0.0;
		}
		for l97 in 0..3 {
			self.fRec115[l97 as usize] = 0.0;
		}
		for l98 in 0..2 {
			self.fRec116[l98 as usize] = 0.0;
		}
		for l99 in 0..16384 {
			self.fVec15[l99 as usize] = 0.0;
		}
		for l100 in 0..2 {
			self.fRec117[l100 as usize] = 0.0;
		}
		for l101 in 0..4096 {
			self.fVec16[l101 as usize] = 0.0;
		}
		for l102 in 0..2 {
			self.fRec17[l102 as usize] = 0.0;
		}
		for l103 in 0..2 {
			self.fRec18[l103 as usize] = 0.0;
		}
		for l104 in 0..32768 {
			self.fVec17[l104 as usize] = 0.0;
		}
		for l105 in 0..2048 {
			self.fVec18[l105 as usize] = 0.0;
		}
		for l106 in 0..2 {
			self.fRec19[l106 as usize] = 0.0;
		}
		for l107 in 0..2 {
			self.fRec20[l107 as usize] = 0.0;
		}
		for l108 in 0..32768 {
			self.fVec19[l108 as usize] = 0.0;
		}
		for l109 in 0..2 {
			self.fRec121[l109 as usize] = 0.0;
		}
		for l110 in 0..2 {
			self.fRec122[l110 as usize] = 0.0;
		}
		for l111 in 0..2 {
			self.fRec119[l111 as usize] = 0.0;
		}
		for l112 in 0..2 {
			self.fRec120[l112 as usize] = 0.0;
		}
		for l113 in 0..3 {
			self.fRec118[l113 as usize] = 0.0;
		}
		for l114 in 0..3 {
			self.fRec123[l114 as usize] = 0.0;
		}
		for l115 in 0..3 {
			self.fRec124[l115 as usize] = 0.0;
		}
		for l116 in 0..16384 {
			self.fVec20[l116 as usize] = 0.0;
		}
		for l117 in 0..16384 {
			self.fVec21[l117 as usize] = 0.0;
		}
		for l118 in 0..2048 {
			self.fVec22[l118 as usize] = 0.0;
		}
		for l119 in 0..2 {
			self.fRec22[l119 as usize] = 0.0;
		}
		for l120 in 0..2 {
			self.fRec23[l120 as usize] = 0.0;
		}
		for l121 in 0..2 {
			self.fRec24[l121 as usize] = 0.0;
		}
		for l122 in 0..32768 {
			self.fVec23[l122 as usize] = 0.0;
		}
		for l123 in 0..4096 {
			self.fVec24[l123 as usize] = 0.0;
		}
		for l124 in 0..2 {
			self.fRec25[l124 as usize] = 0.0;
		}
		for l125 in 0..2 {
			self.fRec26[l125 as usize] = 0.0;
		}
		for l126 in 0..16384 {
			self.fVec25[l126 as usize] = 0.0;
		}
		for l127 in 0..2048 {
			self.fVec26[l127 as usize] = 0.0;
		}
		for l128 in 0..2 {
			self.fRec30[l128 as usize] = 0.0;
		}
		for l129 in 0..16384 {
			self.fVec27[l129 as usize] = 0.0;
		}
		for l130 in 0..4096 {
			self.fVec28[l130 as usize] = 0.0;
		}
		for l131 in 0..16384 {
			self.fVec29[l131 as usize] = 0.0;
		}
		for l132 in 0..4096 {
			self.fVec30[l132 as usize] = 0.0;
		}
		for l133 in 0..4096 {
			self.fVec31[l133 as usize] = 0.0;
		}
		for l134 in 0..2 {
			self.fRec35[l134 as usize] = 0.0;
		}
		for l135 in 0..2 {
			self.fRec36[l135 as usize] = 0.0;
		}
		for l136 in 0..2 {
			self.fRec37[l136 as usize] = 0.0;
		}
		for l137 in 0..3 {
			self.fRec38[l137 as usize] = 0.0;
		}
		for l138 in 0..3 {
			self.fRec39[l138 as usize] = 0.0;
		}
		for l139 in 0..3 {
			self.fRec40[l139 as usize] = 0.0;
		}
		for l140 in 0..3 {
			self.fRec41[l140 as usize] = 0.0;
		}
		for l141 in 0..3 {
			self.fRec42[l141 as usize] = 0.0;
		}
		for l142 in 0..3 {
			self.fRec43[l142 as usize] = 0.0;
		}
		for l143 in 0..3 {
			self.fRec44[l143 as usize] = 0.0;
		}
		for l144 in 0..3 {
			self.fRec45[l144 as usize] = 0.0;
		}
		for l145 in 0..2 {
			self.fRec125[l145 as usize] = 0.0;
		}
		for l146 in 0..2 {
			self.fRec128[l146 as usize] = 0.0;
		}
		for l147 in 0..2 {
			self.fRec129[l147 as usize] = 0.0;
		}
		for l148 in 0..2 {
			self.fRec127[l148 as usize] = 0.0;
		}
		for l149 in 0..3 {
			self.fRec126[l149 as usize] = 0.0;
		}
		for l150 in 0..2 {
			self.fRec3[l150 as usize] = 0.0;
		}
		for l151 in 0..2 {
			self.fRec4[l151 as usize] = 0.0;
		}
		for l152 in 0..2 {
			self.fRec1[l152 as usize] = 0.0;
		}
		for l153 in 0..2 {
			self.fRec2[l153 as usize] = 0.0;
		}
		for l154 in 0..2 {
			self.fRec136[l154 as usize] = 0.0;
		}
		for l155 in 0..2 {
			self.fRec135[l155 as usize] = 0.0;
		}
		for l156 in 0..3 {
			self.fRec134[l156 as usize] = 0.0;
		}
		for l157 in 0..2 {
			self.fRec132[l157 as usize] = 0.0;
		}
		for l158 in 0..2 {
			self.fRec133[l158 as usize] = 0.0;
		}
		for l159 in 0..2 {
			self.fRec130[l159 as usize] = 0.0;
		}
		for l160 in 0..2 {
			self.fRec131[l160 as usize] = 0.0;
		}
		for l161 in 0..2 {
			self.fRec139[l161 as usize] = 0.0;
		}
		for l162 in 0..2 {
			self.fRec140[l162 as usize] = 0.0;
		}
		for l163 in 0..2 {
			self.fRec137[l163 as usize] = 0.0;
		}
		for l164 in 0..2 {
			self.fRec138[l164 as usize] = 0.0;
		}
	}
	pub fn instance_constants(&mut self, sample_rate: i32) {
		// Obtaining locks on 0 static var(s)
		self.fSampleRate = sample_rate;
		self.fConst0 = F32::min(1.92e+05, F32::max(1.0, (self.fSampleRate) as F32));
		self.fConst1 = 44.1 / self.fConst0;
		self.fConst2 = 1.0 - self.fConst1;
		self.fConst3 = F32::tan(628.31854 / self.fConst0);
		self.fConst4 = 1.0 / self.fConst3;
		self.fConst5 = 1.0 - self.fConst4;
		self.fConst6 = 1.0 / (self.fConst4 + 1.0);
		self.fConst7 = F32::floor(0.192303 * self.fConst0 + 0.5);
		self.fConst8 = self.fConst7 / self.fConst0;
		self.fConst9 = 6.9077554 * self.fConst8;
		self.fConst10 = 6.2831855 / self.fConst0;
		self.fConst11 = 9.2103405 * self.fConst8;
		self.fConst12 = F32::floor(0.256891 * self.fConst0 + 0.5);
		self.fConst13 = self.fConst12 / self.fConst0;
		self.fConst14 = 6.9077554 * self.fConst13;
		self.fConst15 = 9.2103405 * self.fConst13;
		self.fConst16 = F32::floor(0.219991 * self.fConst0 + 0.5);
		self.fConst17 = self.fConst16 / self.fConst0;
		self.fConst18 = 6.9077554 * self.fConst17;
		self.fConst19 = 9.2103405 * self.fConst17;
		self.fConst20 = F32::floor(0.029291 * self.fConst0 + 0.5);
		self.iConst21 = (F32::min(8192.0, F32::max(0.0, self.fConst7 - self.fConst20))) as i32;
		self.fConst22 = 2.0 * (1.0 - 1.0 / MoodscoreDsp_faustpower2_f(self.fConst3));
		self.fConst23 = (self.fConst4 + -1.4142135) / self.fConst3 + 1.0;
		self.fConst24 = 1.0 / ((self.fConst4 + 1.4142135) / self.fConst3 + 1.0);
		self.fConst25 = 3.1415927 / self.fConst0;
		self.fConst26 = 1.0 / F32::tan(0.4712389 / self.fConst0);
		self.fConst27 = 1.0 - self.fConst26;
		self.fConst28 = 1.0 / (self.fConst26 + 1.0);
		self.fConst29 = 1.0 / self.fConst0;
		self.fConst30 = F32::tan(12566.371 / self.fConst0);
		self.fConst31 = 2.0 * (1.0 - 1.0 / MoodscoreDsp_faustpower2_f(self.fConst30));
		self.fConst32 = 1.0 / self.fConst30;
		self.fConst33 = (self.fConst32 + -1.4142135) / self.fConst30 + 1.0;
		self.fConst34 = 1.0 / ((self.fConst32 + 1.4142135) / self.fConst30 + 1.0);
		self.fConst35 = F32::exp(-(0.6666667 / self.fConst0));
		self.fConst36 = F32::exp(-(3.3333333 / self.fConst0));
		self.fConst37 = 0.001 * self.fConst0;
		self.iConst38 = (F32::min(2048.0, F32::max(0.0, self.fConst20 + -1.0))) as i32;
		self.fConst39 = F32::floor(0.125 * self.fConst0 + 0.5);
		self.fConst40 = self.fConst39 / self.fConst0;
		self.fConst41 = 6.9077554 * self.fConst40;
		self.fConst42 = 9.2103405 * self.fConst40;
		self.fConst43 = F32::floor(0.019123 * self.fConst0 + 0.5);
		self.iConst44 = (F32::min(16384.0, F32::max(0.0, self.fConst16 - self.fConst43))) as i32;
		self.iConst45 = (F32::min(1024.0, F32::max(0.0, self.fConst43 + -1.0))) as i32;
		self.fConst46 = F32::floor(0.153129 * self.fConst0 + 0.5);
		self.fConst47 = self.fConst46 / self.fConst0;
		self.fConst48 = 6.9077554 * self.fConst47;
		self.fConst49 = 9.2103405 * self.fConst47;
		self.fConst50 = F32::floor(0.027333 * self.fConst0 + 0.5);
		self.iConst51 = (F32::min(16384.0, F32::max(0.0, self.fConst12 - self.fConst50))) as i32;
		self.fConst52 = F32::floor(0.013458 * self.fConst0 + 0.5);
		self.iConst53 = (F32::min(8192.0, F32::max(0.0, self.fConst39 - self.fConst52))) as i32;
		self.iConst54 = (F32::min(1024.0, F32::max(0.0, self.fConst52 + -1.0))) as i32;
		self.fConst55 = F32::floor(0.210389 * self.fConst0 + 0.5);
		self.fConst56 = self.fConst55 / self.fConst0;
		self.fConst57 = 6.9077554 * self.fConst56;
		self.fConst58 = 9.2103405 * self.fConst56;
		self.fConst59 = F32::floor(0.174713 * self.fConst0 + 0.5);
		self.fConst60 = self.fConst59 / self.fConst0;
		self.fConst61 = 6.9077554 * self.fConst60;
		self.fConst62 = 9.2103405 * self.fConst60;
		self.fConst63 = F32::floor(0.024421 * self.fConst0 + 0.5);
		self.iConst64 = (F32::min(16384.0, F32::max(0.0, self.fConst55 - self.fConst63))) as i32;
		self.iConst65 = (F32::min(2048.0, F32::max(0.0, self.fConst63 + -1.0))) as i32;
		self.fConst66 = F32::floor(0.127837 * self.fConst0 + 0.5);
		self.fConst67 = self.fConst66 / self.fConst0;
		self.fConst68 = 6.9077554 * self.fConst67;
		self.fConst69 = 9.2103405 * self.fConst67;
		self.fConst70 = F32::floor(0.020346 * self.fConst0 + 0.5);
		self.iConst71 = (F32::min(8192.0, F32::max(0.0, self.fConst46 - self.fConst70))) as i32;
		self.iConst72 = (F32::min(1024.0, F32::max(0.0, self.fConst70 + -1.0))) as i32;
		self.fConst73 = F32::floor(0.031604 * self.fConst0 + 0.5);
		self.iConst74 = (F32::min(8192.0, F32::max(0.0, self.fConst66 - self.fConst73))) as i32;
		self.fConst75 = F32::floor(0.022904 * self.fConst0 + 0.5);
		self.iConst76 = (F32::min(8192.0, F32::max(0.0, self.fConst59 - self.fConst75))) as i32;
		self.iConst77 = (F32::min(2048.0, F32::max(0.0, self.fConst50 + -1.0))) as i32;
		self.iConst78 = (F32::min(2048.0, F32::max(0.0, self.fConst75 + -1.0))) as i32;
		self.iConst79 = (F32::min(2048.0, F32::max(0.0, self.fConst73 + -1.0))) as i32;
		self.fConst80 = F32::exp(-(18849.557 / self.fConst0));
		self.fConst81 = F32::tan(471.2389 / self.fConst0);
		self.fConst82 = self.fConst81 * (self.fConst81 + 1.4142135) + 1.0;
		self.fConst83 = 2.0 / self.fConst82;
		self.fConst84 = self.fConst81 / self.fConst82;
		self.fConst85 = -1.4142135 - self.fConst81;
		self.fConst86 = self.fConst85 / self.fConst82;
		self.fConst87 = 1.0 / self.fConst82;
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
		ui_interface.add_horizontal_slider("space/air", ParamIndex(28), 0.0, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(29)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(29)), "unit", "Hz");
		ui_interface.add_horizontal_slider("space/damping", ParamIndex(29), 6e+03, 1.5e+03, 1.2e+04, 1.0);
		ui_interface.add_horizontal_slider("space/direct", ParamIndex(30), 1.0, 0.0, 1.0, 0.001);
		ui_interface.add_horizontal_slider("space/notes_room", ParamIndex(31), 0.5, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(32)), "unit", "ms");
		ui_interface.add_horizontal_slider("space/predelay", ParamIndex(32), 0.0, 0.0, 8e+01, 0.01);
		ui_interface.add_horizontal_slider("space/reverb_mix", ParamIndex(33), 0.35, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(34)), "unit", "s");
		ui_interface.add_horizontal_slider("space/size", ParamIndex(34), 8.0, 1.0, 2e+01, 0.01);
		ui_interface.add_horizontal_slider("space/thin", ParamIndex(35), 0.0, 0.0, 1.0, 0.001);
		ui_interface.add_horizontal_slider("space/wet", ParamIndex(36), 1.0, 0.0, 1.5, 0.001);
		ui_interface.close_box();
	}
	
	pub fn get_param(&self, param: ParamIndex) -> Option<FaustFloat> {
		match param.0 {
			27 => Some(self.fHslider0),
			36 => Some(self.fHslider1),
			7 => Some(self.fHslider10),
			2 => Some(self.fHslider11),
			21 => Some(self.fHslider12),
			22 => Some(self.fHslider13),
			24 => Some(self.fHslider14),
			17 => Some(self.fHslider15),
			18 => Some(self.fHslider16),
			20 => Some(self.fHslider17),
			13 => Some(self.fHslider18),
			14 => Some(self.fHslider19),
			34 => Some(self.fHslider2),
			16 => Some(self.fHslider20),
			9 => Some(self.fHslider21),
			10 => Some(self.fHslider22),
			12 => Some(self.fHslider23),
			5 => Some(self.fHslider24),
			6 => Some(self.fHslider25),
			8 => Some(self.fHslider26),
			0 => Some(self.fHslider27),
			1 => Some(self.fHslider28),
			3 => Some(self.fHslider29),
			29 => Some(self.fHslider3),
			4 => Some(self.fHslider30),
			31 => Some(self.fHslider31),
			33 => Some(self.fHslider32),
			35 => Some(self.fHslider33),
			32 => Some(self.fHslider34),
			30 => Some(self.fHslider35),
			28 => Some(self.fHslider36),
			25 => Some(self.fHslider4),
			26 => Some(self.fHslider5),
			23 => Some(self.fHslider6),
			19 => Some(self.fHslider7),
			15 => Some(self.fHslider8),
			11 => Some(self.fHslider9),
			_ => None,
		}
	}
	
	pub fn set_param(&mut self, param: ParamIndex, value: FaustFloat) {
		match param.0 {
			27 => { self.fHslider0 = value }
			36 => { self.fHslider1 = value }
			7 => { self.fHslider10 = value }
			2 => { self.fHslider11 = value }
			21 => { self.fHslider12 = value }
			22 => { self.fHslider13 = value }
			24 => { self.fHslider14 = value }
			17 => { self.fHslider15 = value }
			18 => { self.fHslider16 = value }
			20 => { self.fHslider17 = value }
			13 => { self.fHslider18 = value }
			14 => { self.fHslider19 = value }
			34 => { self.fHslider2 = value }
			16 => { self.fHslider20 = value }
			9 => { self.fHslider21 = value }
			10 => { self.fHslider22 = value }
			12 => { self.fHslider23 = value }
			5 => { self.fHslider24 = value }
			6 => { self.fHslider25 = value }
			8 => { self.fHslider26 = value }
			0 => { self.fHslider27 = value }
			1 => { self.fHslider28 = value }
			3 => { self.fHslider29 = value }
			29 => { self.fHslider3 = value }
			4 => { self.fHslider30 = value }
			31 => { self.fHslider31 = value }
			33 => { self.fHslider32 = value }
			35 => { self.fHslider33 = value }
			32 => { self.fHslider34 = value }
			30 => { self.fHslider35 = value }
			28 => { self.fHslider36 = value }
			25 => { self.fHslider4 = value }
			26 => { self.fHslider5 = value }
			23 => { self.fHslider6 = value }
			19 => { self.fHslider7 = value }
			15 => { self.fHslider8 = value }
			11 => { self.fHslider9 = value }
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
		let [outputs0, outputs1, outputs2, .. ] = outputs.as_mut() else { panic!("wrong number of output buffers"); };
		let outputs0 = outputs0.as_mut()[..count].iter_mut();
		let outputs1 = outputs1.as_mut()[..count].iter_mut();
		let outputs2 = outputs2.as_mut()[..count].iter_mut();
		let mut fSlow0: F32 = self.fConst1 * (self.fHslider0) as F32;
		let mut fSlow1: F32 = self.fConst1 * ((self.fHslider1) as F32 + -1.0);
		let mut fSlow2: F32 = self.fConst1 * ((self.fHslider2) as F32 + -8.0);
		let mut fSlow3: F32 = self.fConst1 * ((self.fHslider3) as F32 + -6e+03);
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
		let mut fSlow29: F32 = self.fConst1 * (self.fHslider29) as F32;
		let mut fSlow30: F32 = self.fConst1 * (self.fHslider30) as F32;
		let mut fSlow31: F32 = self.fConst1 * (self.fHslider31) as F32;
		let mut fSlow32: F32 = self.fConst1 * (self.fHslider32) as F32;
		let mut fSlow33: F32 = self.fConst1 * (self.fHslider33) as F32;
		let mut fSlow34: F32 = self.fConst1 * (self.fHslider34) as F32;
		let mut fSlow35: F32 = self.fConst1 * ((self.fHslider35) as F32 + -1.0);
		let mut fSlow36: F32 = self.fConst1 * (self.fHslider36) as F32;
		let zipped_iterators = inputs0.zip(inputs1).zip(inputs2).zip(inputs3).zip(outputs0).zip(outputs1).zip(outputs2);
		for ((((((input0, input1), input2), input3), output0), output1), output2) in zipped_iterators {
			self.iVec0[0] = 1;
			self.fRec0[0] = fSlow0 + self.fConst2 * self.fRec0[1];
			self.fRec5[0] = fSlow1 + self.fConst2 * self.fRec5[1];
			let mut fTemp0: F32 = 1.0 + self.fRec5[0];
			self.fRec6[0] = -(self.fConst6 * (self.fConst5 * self.fRec6[1] - (self.fRec45[1] + self.fRec45[2])));
			self.fRec7[0] = -(self.fConst6 * (self.fConst5 * self.fRec7[1] - (self.fRec42[1] + self.fRec42[2])));
			self.fRec8[0] = -(self.fConst6 * (self.fConst5 * self.fRec8[1] - (self.fRec38[1] + self.fRec38[2])));
			self.fRec9[0] = -(self.fConst6 * (self.fConst5 * self.fRec9[1] - (self.fRec44[1] + self.fRec44[2])));
			self.fRec10[0] = -(self.fConst6 * (self.fConst5 * self.fRec10[1] - (self.fRec40[1] + self.fRec40[2])));
			self.fRec11[0] = -(self.fConst6 * (self.fConst5 * self.fRec11[1] - (self.fRec39[1] + self.fRec39[2])));
			self.fRec12[0] = -(self.fConst6 * (self.fConst5 * self.fRec12[1] - (self.fRec41[1] + self.fRec41[2])));
			self.fRec13[0] = -(self.fConst6 * (self.fConst5 * self.fRec13[1] - (self.fRec43[1] + self.fRec43[2])));
			self.fRec46[0] = fSlow2 + self.fConst2 * self.fRec46[1];
			let mut fTemp1: F32 = 8.0 + self.fRec46[0];
			let mut fTemp2: F32 = F32::exp(-(self.fConst9 / fTemp1));
			let mut fTemp3: F32 = MoodscoreDsp_faustpower2_f(fTemp2);
			let mut fTemp4: F32 = 1.0 - fTemp3;
			self.fRec47[0] = fSlow3 + self.fConst2 * self.fRec47[1];
			let mut fTemp5: F32 = F32::cos(self.fConst10 * (6e+03 + self.fRec47[0]));
			let mut fTemp6: F32 = 1.0 - fTemp3 * fTemp5;
			let mut fTemp7: F32 = F32::sqrt(F32::max(0.0, -1.0 + MoodscoreDsp_faustpower2_f(fTemp6) / MoodscoreDsp_faustpower2_f(fTemp4)));
			let mut fTemp8: F32 = fTemp6 / fTemp4;
			self.fRec14[0] = fTemp2 * (1.0 - fTemp8 + fTemp7) * (self.fRec38[1] + (-1.0 + F32::exp(-(self.fConst11 / fTemp1)) / fTemp2) * self.fRec8[0]) + (fTemp8 - fTemp7) * self.fRec14[1];
			let mut fTemp9: F32 = F32::exp(-(self.fConst14 / fTemp1));
			let mut fTemp10: F32 = MoodscoreDsp_faustpower2_f(fTemp9);
			let mut fTemp11: F32 = 1.0 - fTemp10;
			let mut fTemp12: F32 = 1.0 - fTemp5 * fTemp10;
			let mut fTemp13: F32 = F32::sqrt(F32::max(0.0, -1.0 + MoodscoreDsp_faustpower2_f(fTemp12) / MoodscoreDsp_faustpower2_f(fTemp11)));
			let mut fTemp14: F32 = fTemp12 / fTemp11;
			self.fRec15[0] = fTemp9 * (1.0 - fTemp14 + fTemp13) * (self.fRec45[1] + (-1.0 + F32::exp(-(self.fConst15 / fTemp1)) / fTemp9) * self.fRec6[0]) + (fTemp14 - fTemp13) * self.fRec15[1];
			let mut fTemp15: F32 = F32::exp(-(self.fConst18 / fTemp1));
			let mut fTemp16: F32 = MoodscoreDsp_faustpower2_f(fTemp15);
			let mut fTemp17: F32 = 1.0 - fTemp16;
			let mut fTemp18: F32 = 1.0 - fTemp5 * fTemp16;
			let mut fTemp19: F32 = F32::sqrt(F32::max(0.0, -1.0 + MoodscoreDsp_faustpower2_f(fTemp18) / MoodscoreDsp_faustpower2_f(fTemp17)));
			let mut fTemp20: F32 = fTemp18 / fTemp17;
			self.fRec16[0] = fTemp15 * (1.0 - fTemp20 + fTemp19) * (self.fRec41[1] + (-1.0 + F32::exp(-(self.fConst19 / fTemp1)) / fTemp15) * self.fRec12[0]) + (fTemp20 - fTemp19) * self.fRec16[1];
			self.fVec1[(self.IOTA0 & 16383) as usize] = 1e-20 + 0.35355338 * self.fRec14[0];
			self.fRec49[0] = fSlow4 + self.fConst2 * self.fRec49[1];
			let mut fTemp21: F32 = F32::tan(self.fConst25 * F32::max(self.fRec49[0], 2e+01));
			self.fRec50[0] = fSlow5 + self.fConst2 * self.fRec50[1];
			let mut fTemp22: F32 = 1.99998 * self.fRec50[0];
			let mut fTemp23: F32 = 2.0 - fTemp22;
			let mut fTemp24: F32 = 1.0 / fTemp21;
			let mut fTemp25: F32 = MoodscoreDsp_faustpower2_f(1.4141995 * self.fRec50[0]);
			let mut fTemp26: F32 = 1.0 - fTemp22;
			let mut fTemp27: F32 = fTemp26 + fTemp25 + (fTemp24 + fTemp23) / fTemp21;
			let mut fTemp28: F32 = (fTemp26 + fTemp25 + (fTemp24 - fTemp23) / fTemp21) / fTemp27;
			let mut fTemp29: F32 = 1.0 / MoodscoreDsp_faustpower2_f(fTemp21);
			let mut fTemp30: F32 = 1.0 - (fTemp22 + fTemp29) + fTemp25;
			let mut fTemp31: F32 = F32::max(-0.9999999, F32::min(0.9999999, 2.0 * (fTemp30 / (fTemp27 * (1.0 + fTemp28)))));
			let mut fTemp32: F32 = 1.0 - MoodscoreDsp_faustpower2_f(fTemp31);
			let mut fTemp33: F32 = F32::max(-0.9999999, F32::min(0.9999999, fTemp28));
			let mut fTemp34: F32 = 1.0 - MoodscoreDsp_faustpower2_f(fTemp33);
			let mut fTemp35: F32 = F32::sqrt(fTemp34);
			let mut fTemp36: F32 = fTemp35 * F32::sqrt(fTemp32);
			let mut fTemp37: F32 = 2.0 + fTemp22;
			let mut fTemp38: F32 = fTemp22 + fTemp25;
			let mut fTemp39: F32 = 1.0 + fTemp38 + (fTemp37 + fTemp24) / fTemp21;
			let mut fTemp40: F32 = (1.0 - (fTemp37 - fTemp24) / fTemp21 + fTemp38) / fTemp39;
			let mut fTemp41: F32 = 1.0 - fTemp29 + fTemp38;
			let mut fTemp42: F32 = F32::max(-0.9999999, F32::min(0.9999999, 2.0 * (fTemp41 / (fTemp39 * (1.0 + fTemp40)))));
			let mut fTemp43: F32 = 1.0 - MoodscoreDsp_faustpower2_f(fTemp42);
			let mut fTemp44: F32 = F32::max(-0.9999999, F32::min(0.9999999, fTemp40));
			let mut fTemp45: F32 = 1.0 - MoodscoreDsp_faustpower2_f(fTemp44);
			let mut fTemp46: F32 = F32::sqrt(fTemp45);
			let mut fTemp47: F32 = fTemp46 * F32::sqrt(fTemp43);
			self.fRec55[0] = fSlow6 + self.fConst2 * self.fRec55[1];
			self.fRec56[0] = fSlow7 + self.fConst2 * self.fRec56[1];
			self.fRec57[0] = fSlow8 + self.fConst2 * self.fRec57[1];
			self.fRec58[0] = fSlow9 + self.fConst2 * self.fRec58[1];
			self.fRec59[0] = fSlow10 + self.fConst2 * self.fRec59[1];
			self.fRec60[0] = fSlow11 + self.fConst2 * self.fRec60[1];
			let mut fTemp48: F32 = F32::max(1.0, 0.6666667 * F32::sqrt(MoodscoreDsp_faustpower2_f(self.fRec60[0]) + MoodscoreDsp_faustpower2_f(self.fRec59[0]) + MoodscoreDsp_faustpower2_f(self.fRec58[0]) + MoodscoreDsp_faustpower2_f(self.fRec57[0]) + MoodscoreDsp_faustpower2_f(self.fRec56[0]) + MoodscoreDsp_faustpower2_f(self.fRec55[0])));
			self.fRec61[0] = fSlow12 + self.fConst2 * self.fRec61[1];
			let mut iTemp49: i32 = i32::wrapping_mul(1103515245, i32::wrapping_add(12345, i32::wrapping_mul(1103515245, i32::wrapping_add(12345, self.iRec63[1]))));
			let mut iTemp50: i32 = i32::wrapping_mul(1103515245, i32::wrapping_add(12345, iTemp49));
			let mut iTemp51: i32 = i32::wrapping_mul(1103515245, i32::wrapping_add(12345, i32::wrapping_mul(1103515245, i32::wrapping_add(12345, iTemp50))));
			let mut iTemp52: i32 = i32::wrapping_mul(1103515245, i32::wrapping_add(12345, iTemp51));
			let mut iTemp53: i32 = i32::wrapping_mul(1103515245, i32::wrapping_add(12345, i32::wrapping_mul(1103515245, i32::wrapping_add(12345, iTemp52))));
			let mut iTemp54: i32 = i32::wrapping_mul(1103515245, i32::wrapping_add(12345, iTemp53));
			let mut iTemp55: i32 = i32::wrapping_mul(1103515245, i32::wrapping_add(12345, i32::wrapping_mul(1103515245, i32::wrapping_add(12345, iTemp54))));
			let mut iTemp56: i32 = i32::wrapping_mul(1103515245, i32::wrapping_add(12345, iTemp55));
			let mut iTemp57: i32 = i32::wrapping_mul(1103515245, i32::wrapping_add(12345, i32::wrapping_mul(1103515245, i32::wrapping_add(12345, iTemp56))));
			let mut iTemp58: i32 = i32::wrapping_mul(1103515245, i32::wrapping_add(12345, iTemp57));
			let mut iTemp59: i32 = i32::wrapping_mul(1103515245, i32::wrapping_add(12345, iTemp58));
			let mut iTemp60: i32 = i32::wrapping_mul(1103515245, i32::wrapping_add(12345, iTemp59));
			self.iRec63[0] = i32::wrapping_mul(1103515245, i32::wrapping_add(12345, iTemp60));
			let mut fTemp61: F32 = (iTemp49) as F32;
			self.fVec2[0] = fTemp61;
			self.fRec62[0] = self.fConst28 * (4.656613e-10 * (fTemp61 + self.fVec2[1]) - self.fConst27 * self.fRec62[1]);
			self.fRec64[0] = fSlow13 + self.fConst2 * self.fRec64[1];
			self.fRec65[0] = fSlow14 + self.fConst2 * self.fRec65[1];
			let mut fTemp62: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec65[0] * F32::powf(2.0, 0.00083333335 * (4.5e+03 * self.fRec64[0] * self.fRec62[0] - self.fRec61[0]))));
			let mut fTemp63: F32 = self.fRec66[1] + self.fConst29 * fTemp62;
			let mut fTemp64: F32 = -1.0 + fTemp63;
			let mut iTemp65: i32 = (fTemp64 < 0.0) as i32;
			self.fRec66[0] = (if iTemp65 != 0 {fTemp63} else {fTemp64});
			let mut fTemp66: F32 = -1.0 + 2.0 * (if iTemp65 != 0 {fTemp63} else {fTemp63 + fTemp64 * (1.0 - self.fConst0 / fTemp62)});
			let mut fTemp67: F32 = (iTemp50) as F32;
			self.fVec3[0] = fTemp67;
			self.fRec67[0] = self.fConst28 * (4.656613e-10 * (fTemp67 + self.fVec3[1]) - self.fConst27 * self.fRec67[1]);
			let mut fTemp68: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec65[0] * F32::powf(2.0, 0.00083333335 * (4.5e+03 * self.fRec67[0] * self.fRec64[0] + self.fRec61[0]))));
			let mut fTemp69: F32 = self.fRec68[1] + self.fConst29 * fTemp68;
			let mut fTemp70: F32 = -1.0 + fTemp69;
			let mut iTemp71: i32 = (fTemp70 < 0.0) as i32;
			self.fRec68[0] = (if iTemp71 != 0 {fTemp69} else {fTemp70});
			let mut fTemp72: F32 = -1.0 + 2.0 * (if iTemp71 != 0 {fTemp69} else {fTemp69 + fTemp70 * (1.0 - self.fConst0 / fTemp68)});
			self.fRec69[0] = fSlow15 + self.fConst2 * self.fRec69[1];
			let mut fTemp73: F32 = (iTemp51) as F32;
			self.fVec4[0] = fTemp73;
			self.fRec70[0] = self.fConst28 * (4.656613e-10 * (fTemp73 + self.fVec4[1]) - self.fConst27 * self.fRec70[1]);
			self.fRec71[0] = fSlow16 + self.fConst2 * self.fRec71[1];
			self.fRec72[0] = fSlow17 + self.fConst2 * self.fRec72[1];
			let mut fTemp74: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec72[0] * F32::powf(2.0, 0.00083333335 * (4.5e+03 * self.fRec71[0] * self.fRec70[0] - self.fRec69[0]))));
			let mut fTemp75: F32 = self.fRec73[1] + self.fConst29 * fTemp74;
			let mut fTemp76: F32 = -1.0 + fTemp75;
			let mut iTemp77: i32 = (fTemp76 < 0.0) as i32;
			self.fRec73[0] = (if iTemp77 != 0 {fTemp75} else {fTemp76});
			let mut fTemp78: F32 = -1.0 + 2.0 * (if iTemp77 != 0 {fTemp75} else {fTemp75 + fTemp76 * (1.0 - self.fConst0 / fTemp74)});
			let mut fTemp79: F32 = (iTemp52) as F32;
			self.fVec5[0] = fTemp79;
			self.fRec74[0] = self.fConst28 * (4.656613e-10 * (fTemp79 + self.fVec5[1]) - self.fConst27 * self.fRec74[1]);
			let mut fTemp80: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec72[0] * F32::powf(2.0, 0.00083333335 * (4.5e+03 * self.fRec74[0] * self.fRec71[0] + self.fRec69[0]))));
			let mut fTemp81: F32 = self.fRec75[1] + self.fConst29 * fTemp80;
			let mut fTemp82: F32 = -1.0 + fTemp81;
			let mut iTemp83: i32 = (fTemp82 < 0.0) as i32;
			self.fRec75[0] = (if iTemp83 != 0 {fTemp81} else {fTemp82});
			let mut fTemp84: F32 = -1.0 + 2.0 * (if iTemp83 != 0 {fTemp81} else {fTemp81 + fTemp82 * (1.0 - self.fConst0 / fTemp80)});
			self.fRec76[0] = fSlow18 + self.fConst2 * self.fRec76[1];
			let mut fTemp85: F32 = (iTemp53) as F32;
			self.fVec6[0] = fTemp85;
			self.fRec77[0] = self.fConst28 * (4.656613e-10 * (fTemp85 + self.fVec6[1]) - self.fConst27 * self.fRec77[1]);
			self.fRec78[0] = fSlow19 + self.fConst2 * self.fRec78[1];
			self.fRec79[0] = fSlow20 + self.fConst2 * self.fRec79[1];
			let mut fTemp86: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec79[0] * F32::powf(2.0, 0.00083333335 * (4.5e+03 * self.fRec78[0] * self.fRec77[0] - self.fRec76[0]))));
			let mut fTemp87: F32 = self.fRec80[1] + self.fConst29 * fTemp86;
			let mut fTemp88: F32 = -1.0 + fTemp87;
			let mut iTemp89: i32 = (fTemp88 < 0.0) as i32;
			self.fRec80[0] = (if iTemp89 != 0 {fTemp87} else {fTemp88});
			let mut fTemp90: F32 = -1.0 + 2.0 * (if iTemp89 != 0 {fTemp87} else {fTemp87 + fTemp88 * (1.0 - self.fConst0 / fTemp86)});
			let mut fTemp91: F32 = (iTemp54) as F32;
			self.fVec7[0] = fTemp91;
			self.fRec81[0] = self.fConst28 * (4.656613e-10 * (fTemp91 + self.fVec7[1]) - self.fConst27 * self.fRec81[1]);
			let mut fTemp92: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec79[0] * F32::powf(2.0, 0.00083333335 * (4.5e+03 * self.fRec81[0] * self.fRec78[0] + self.fRec76[0]))));
			let mut fTemp93: F32 = self.fRec82[1] + self.fConst29 * fTemp92;
			let mut fTemp94: F32 = -1.0 + fTemp93;
			let mut iTemp95: i32 = (fTemp94 < 0.0) as i32;
			self.fRec82[0] = (if iTemp95 != 0 {fTemp93} else {fTemp94});
			let mut fTemp96: F32 = -1.0 + 2.0 * (if iTemp95 != 0 {fTemp93} else {fTemp93 + fTemp94 * (1.0 - self.fConst0 / fTemp92)});
			self.fRec83[0] = fSlow21 + self.fConst2 * self.fRec83[1];
			let mut fTemp97: F32 = (iTemp55) as F32;
			self.fVec8[0] = fTemp97;
			self.fRec84[0] = self.fConst28 * (4.656613e-10 * (fTemp97 + self.fVec8[1]) - self.fConst27 * self.fRec84[1]);
			self.fRec85[0] = fSlow22 + self.fConst2 * self.fRec85[1];
			self.fRec86[0] = fSlow23 + self.fConst2 * self.fRec86[1];
			let mut fTemp98: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec86[0] * F32::powf(2.0, 0.00083333335 * (4.5e+03 * self.fRec85[0] * self.fRec84[0] - self.fRec83[0]))));
			let mut fTemp99: F32 = self.fRec87[1] + self.fConst29 * fTemp98;
			let mut fTemp100: F32 = -1.0 + fTemp99;
			let mut iTemp101: i32 = (fTemp100 < 0.0) as i32;
			self.fRec87[0] = (if iTemp101 != 0 {fTemp99} else {fTemp100});
			let mut fTemp102: F32 = -1.0 + 2.0 * (if iTemp101 != 0 {fTemp99} else {fTemp99 + fTemp100 * (1.0 - self.fConst0 / fTemp98)});
			let mut fTemp103: F32 = (iTemp56) as F32;
			self.fVec9[0] = fTemp103;
			self.fRec88[0] = self.fConst28 * (4.656613e-10 * (fTemp103 + self.fVec9[1]) - self.fConst27 * self.fRec88[1]);
			let mut fTemp104: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec86[0] * F32::powf(2.0, 0.00083333335 * (4.5e+03 * self.fRec88[0] * self.fRec85[0] + self.fRec83[0]))));
			let mut fTemp105: F32 = self.fRec89[1] + self.fConst29 * fTemp104;
			let mut fTemp106: F32 = -1.0 + fTemp105;
			let mut iTemp107: i32 = (fTemp106 < 0.0) as i32;
			self.fRec89[0] = (if iTemp107 != 0 {fTemp105} else {fTemp106});
			let mut fTemp108: F32 = -1.0 + 2.0 * (if iTemp107 != 0 {fTemp105} else {fTemp105 + fTemp106 * (1.0 - self.fConst0 / fTemp104)});
			self.fRec90[0] = fSlow24 + self.fConst2 * self.fRec90[1];
			let mut fTemp109: F32 = (iTemp57) as F32;
			self.fVec10[0] = fTemp109;
			self.fRec91[0] = self.fConst28 * (4.656613e-10 * (fTemp109 + self.fVec10[1]) - self.fConst27 * self.fRec91[1]);
			self.fRec92[0] = fSlow25 + self.fConst2 * self.fRec92[1];
			self.fRec93[0] = fSlow26 + self.fConst2 * self.fRec93[1];
			let mut fTemp110: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec93[0] * F32::powf(2.0, 0.00083333335 * (4.5e+03 * self.fRec92[0] * self.fRec91[0] - self.fRec90[0]))));
			let mut fTemp111: F32 = self.fRec94[1] + self.fConst29 * fTemp110;
			let mut fTemp112: F32 = -1.0 + fTemp111;
			let mut iTemp113: i32 = (fTemp112 < 0.0) as i32;
			self.fRec94[0] = (if iTemp113 != 0 {fTemp111} else {fTemp112});
			let mut fTemp114: F32 = -1.0 + 2.0 * (if iTemp113 != 0 {fTemp111} else {fTemp111 + fTemp112 * (1.0 - self.fConst0 / fTemp110)});
			let mut fTemp115: F32 = (iTemp58) as F32;
			self.fVec11[0] = fTemp115;
			self.fRec95[0] = self.fConst28 * (4.656613e-10 * (fTemp115 + self.fVec11[1]) - self.fConst27 * self.fRec95[1]);
			let mut fTemp116: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec93[0] * F32::powf(2.0, 0.00083333335 * (4.5e+03 * self.fRec95[0] * self.fRec92[0] + self.fRec90[0]))));
			let mut fTemp117: F32 = self.fRec96[1] + self.fConst29 * fTemp116;
			let mut fTemp118: F32 = -1.0 + fTemp117;
			let mut iTemp119: i32 = (fTemp118 < 0.0) as i32;
			self.fRec96[0] = (if iTemp119 != 0 {fTemp117} else {fTemp118});
			let mut fTemp120: F32 = -1.0 + 2.0 * (if iTemp119 != 0 {fTemp117} else {fTemp117 + fTemp118 * (1.0 - self.fConst0 / fTemp116)});
			self.fRec97[0] = fSlow27 + self.fConst2 * self.fRec97[1];
			let mut fTemp121: F32 = (iTemp60) as F32;
			self.fVec12[0] = fTemp121;
			self.fRec98[0] = self.fConst28 * (4.656613e-10 * (fTemp121 + self.fVec12[1]) - self.fConst27 * self.fRec98[1]);
			self.fRec99[0] = fSlow28 + self.fConst2 * self.fRec99[1];
			self.fRec100[0] = fSlow29 + self.fConst2 * self.fRec100[1];
			let mut fTemp122: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec100[0] * F32::powf(2.0, 0.00083333335 * (4.5e+03 * self.fRec99[0] * self.fRec98[0] - self.fRec97[0]))));
			let mut fTemp123: F32 = self.fRec101[1] + self.fConst29 * fTemp122;
			let mut fTemp124: F32 = -1.0 + fTemp123;
			let mut iTemp125: i32 = (fTemp124 < 0.0) as i32;
			self.fRec101[0] = (if iTemp125 != 0 {fTemp123} else {fTemp124});
			let mut fTemp126: F32 = -1.0 + 2.0 * (if iTemp125 != 0 {fTemp123} else {fTemp123 + fTemp124 * (1.0 - self.fConst0 / fTemp122)});
			let mut fTemp127: F32 = (self.iRec63[0]) as F32;
			self.fVec13[0] = fTemp127;
			self.fRec102[0] = self.fConst28 * (4.656613e-10 * (fTemp127 + self.fVec13[1]) - self.fConst27 * self.fRec102[1]);
			let mut fTemp128: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec100[0] * F32::powf(2.0, 0.00083333335 * (4.5e+03 * self.fRec102[0] * self.fRec99[0] + self.fRec97[0]))));
			let mut fTemp129: F32 = self.fRec103[1] + self.fConst29 * fTemp128;
			let mut fTemp130: F32 = -1.0 + fTemp129;
			let mut iTemp131: i32 = (fTemp130 < 0.0) as i32;
			self.fRec103[0] = (if iTemp131 != 0 {fTemp129} else {fTemp130});
			let mut fTemp132: F32 = -1.0 + 2.0 * (if iTemp131 != 0 {fTemp129} else {fTemp129 + fTemp130 * (1.0 - self.fConst0 / fTemp128)});
			self.fRec104[0] = fSlow30 + self.fConst2 * self.fRec104[1];
			let mut fTemp133: F32 = (iTemp59) as F32;
			self.fVec14[0] = fTemp133;
			self.fRec107[0] = self.fConst28 * (4.656613e-10 * (fTemp133 + self.fVec14[1]) - self.fConst27 * self.fRec107[1]);
			let mut fTemp134: F32 = F32::powf(2.0, 3.75 * self.fRec99[0] * self.fRec107[0]);
			self.fRec105[0] = self.fRec105[1] - self.fConst25 * self.fRec100[0] * self.fRec106[1] * fTemp134;
			self.fRec106[0] = (i32::wrapping_sub(1, self.iVec0[1])) as F32 + self.fRec106[1] + self.fConst25 * self.fRec100[0] * fTemp134 * self.fRec105[0];
			let mut fTemp135: F32 = self.fRec106[0] * self.fRec104[0];
			let mut fTemp136: F32 = self.fRec60[0] * (fTemp135 + 0.39284748 * fTemp132 + 0.5879378 * fTemp126) + self.fRec59[0] * (0.39284748 * fTemp120 + 0.5879378 * fTemp114) + self.fRec58[0] * (0.19193739 * fTemp108 + 0.43776554 * fTemp102) + self.fRec57[0] * (0.5553029 * fTemp96 + 0.6805586 * fTemp90) + self.fRec56[0] * (0.05547896 * fTemp84 + 0.32101977 * fTemp78) + self.fRec55[0] * (0.6300368 * fTemp72 + 0.704927 * fTemp66);
			let mut fTemp137: F32 = F32::sqrt(F32::max(0.0, fTemp45));
			let mut fTemp138: F32 = fTemp137 * fTemp136 / fTemp48 - fTemp44 * self.fRec54[1];
			let mut fTemp139: F32 = F32::sqrt(F32::max(0.0, fTemp43));
			self.fRec53[0] = fTemp139 * fTemp138 - fTemp42 * self.fRec53[1];
			self.fRec54[0] = fTemp42 * fTemp138 + fTemp139 * self.fRec53[1];
			let mut fTemp140: F32 = 1.0 - fTemp41 / fTemp39;
			let mut fTemp141: F32 = 1.0 - fTemp40 - 2.0 * fTemp42 * fTemp140;
			let mut fTemp142: F32 = fTemp44 * fTemp136 / fTemp48 + fTemp137 * self.fRec54[1] + 2.0 * (fTemp140 * self.fRec54[0] / fTemp46) + fTemp141 * self.fRec53[0] / fTemp47;
			let mut fTemp143: F32 = F32::sqrt(F32::max(0.0, fTemp34));
			let mut fTemp144: F32 = fTemp143 * fTemp142 / fTemp39 - fTemp33 * self.fRec52[1];
			let mut fTemp145: F32 = F32::sqrt(F32::max(0.0, fTemp32));
			self.fRec51[0] = fTemp145 * fTemp144 - fTemp31 * self.fRec51[1];
			self.fRec52[0] = fTemp31 * fTemp144 + fTemp145 * self.fRec51[1];
			let mut fTemp146: F32 = 1.0 - fTemp30 / fTemp27;
			let mut fTemp147: F32 = 1.0 - fTemp28 - 2.0 * fTemp31 * fTemp146;
			let mut fTemp148: F32 = fTemp33 * fTemp142 / fTemp39 + fTemp143 * self.fRec52[1] + 2.0 * (fTemp146 * self.fRec52[0] / fTemp35) + fTemp147 * self.fRec51[0] / fTemp36;
			let mut fTemp149: F32 = 0.9 * (fTemp148 / fTemp27);
			self.fRec48[0] = fTemp149 - self.fConst24 * (self.fConst23 * self.fRec48[2] + self.fConst22 * self.fRec48[1]);
			let mut fTemp150: F32 = self.fRec48[2] + self.fRec48[0] + 2.0 * self.fRec48[1];
			self.fRec108[0] = fTemp149 - self.fConst34 * (self.fConst33 * self.fRec108[2] + self.fConst31 * self.fRec108[1]);
			let mut fTemp151: F32 = self.fConst34 * (self.fRec108[2] + self.fRec108[0] + 2.0 * self.fRec108[1]) - self.fConst24 * fTemp150;
			let mut fTemp152: F32 = (*input1) as F32;
			let mut fTemp153: F32 = (*input0) as F32;
			let mut fTemp154: F32 = 0.5 * (fTemp153 + fTemp152);
			self.fRec110[0] = fTemp154 - self.fConst24 * (self.fConst23 * self.fRec110[2] + self.fConst22 * self.fRec110[1]);
			let mut fTemp155: F32 = self.fConst24 * (self.fRec110[2] + self.fRec110[0] + 2.0 * self.fRec110[1]);
			self.fRec111[0] = fTemp154 - self.fConst34 * (self.fConst33 * self.fRec111[2] + self.fConst31 * self.fRec111[1]);
			let mut fTemp156: F32 = F32::abs(self.fConst34 * (self.fRec111[2] + self.fRec111[0] + 2.0 * self.fRec111[1]) - fTemp155);
			let mut fTemp157: F32 = (if (fTemp156 > self.fRec109[1]) as i32 != 0 {self.fConst36} else {self.fConst35});
			self.fRec109[0] = fTemp156 * (1.0 - fTemp157) + self.fRec109[1] * fTemp157;
			self.fRec112[0] = fSlow31 + self.fConst2 * self.fRec112[1];
			let mut fTemp158: F32 = 1.0 - F32::powf(1e+01, -(0.45 * self.fRec112[0] * F32::min(1.0, F32::max(0.0, 0.05 * (5e+01 + 2e+01 * F32::log10(F32::max(1.1754944e-38, F32::max(1e-10, self.fRec109[0]))))))));
			let mut fTemp159: F32 = F32::abs(fTemp155);
			let mut fTemp160: F32 = (if (fTemp159 > self.fRec113[1]) as i32 != 0 {self.fConst36} else {self.fConst35});
			self.fRec113[0] = fTemp159 * (1.0 - fTemp160) + self.fRec113[1] * fTemp160;
			let mut fTemp161: F32 = 1.0 - F32::powf(1e+01, -(0.45 * self.fRec112[0] * F32::min(1.0, F32::max(0.0, 0.05 * (5e+01 + 2e+01 * F32::log10(F32::max(1.1754944e-38, F32::max(1e-10, self.fRec113[0]))))))));
			let mut fTemp162: F32 = fTemp149 - (self.fConst24 * fTemp161 * fTemp150 + fTemp158 * fTemp151);
			self.fRec114[0] = fSlow32 + self.fConst2 * self.fRec114[1];
			let mut fTemp163: F32 = self.fRec114[0] * fTemp162;
			let mut fTemp164: F32 = (*input3) as F32;
			self.fRec115[0] = fTemp164 - self.fConst24 * (self.fConst23 * self.fRec115[2] + self.fConst22 * self.fRec115[1]) + fTemp163;
			self.fRec116[0] = fSlow33 + self.fConst2 * self.fRec116[1];
			let mut fTemp165: F32 = fTemp164 - self.fConst24 * self.fRec116[0] * (self.fRec115[2] + self.fRec115[0] + 2.0 * self.fRec115[1]) + fTemp163;
			self.fVec15[(self.IOTA0 & 16383) as usize] = fTemp165;
			self.fRec117[0] = fSlow34 + self.fConst2 * self.fRec117[1];
			let mut fTemp166: F32 = self.fConst37 * self.fRec117[0];
			let mut iTemp167: i32 = (fTemp166) as i32;
			let mut iTemp168: i32 = core::cmp::min(8193, core::cmp::max(0, i32::wrapping_add(1, iTemp167)));
			let mut fTemp169: F32 = F32::floor(fTemp166);
			let mut fTemp170: F32 = fTemp166 - fTemp169;
			let mut fTemp171: F32 = 1.0 - fTemp166 + fTemp169;
			let mut iTemp172: i32 = core::cmp::min(8193, core::cmp::max(0, iTemp167));
			let mut fTemp173: F32 = 0.3 * (self.fVec15[((i32::wrapping_sub(self.IOTA0, iTemp172)) & 16383) as usize] * fTemp171 + fTemp170 * self.fVec15[((i32::wrapping_sub(self.IOTA0, iTemp168)) & 16383) as usize]);
			let mut fTemp174: F32 = fTemp173 + 0.6 * self.fRec17[1] + self.fVec1[((i32::wrapping_sub(self.IOTA0, self.iConst21)) & 16383) as usize];
			self.fVec16[(self.IOTA0 & 4095) as usize] = fTemp174;
			self.fRec17[0] = self.fVec16[((i32::wrapping_sub(self.IOTA0, self.iConst38)) & 4095) as usize];
			let mut fTemp175: F32 = F32::exp(-(self.fConst41 / fTemp1));
			let mut fTemp176: F32 = MoodscoreDsp_faustpower2_f(fTemp175);
			let mut fTemp177: F32 = 1.0 - fTemp176;
			let mut fTemp178: F32 = 1.0 - fTemp5 * fTemp176;
			let mut fTemp179: F32 = F32::sqrt(F32::max(0.0, -1.0 + MoodscoreDsp_faustpower2_f(fTemp178) / MoodscoreDsp_faustpower2_f(fTemp177)));
			let mut fTemp180: F32 = fTemp178 / fTemp177;
			self.fRec18[0] = fTemp175 * (1.0 - fTemp180 + fTemp179) * (self.fRec40[1] + (-1.0 + F32::exp(-(self.fConst42 / fTemp1)) / fTemp175) * self.fRec10[0]) + (fTemp180 - fTemp179) * self.fRec18[1];
			self.fVec17[(self.IOTA0 & 32767) as usize] = 1e-20 + 0.35355338 * self.fRec16[0];
			let mut fTemp181: F32 = 0.6 * self.fRec19[1] + self.fVec17[((i32::wrapping_sub(self.IOTA0, self.iConst44)) & 32767) as usize];
			self.fVec18[(self.IOTA0 & 2047) as usize] = fTemp181 - fTemp173;
			self.fRec19[0] = self.fVec18[((i32::wrapping_sub(self.IOTA0, self.iConst45)) & 2047) as usize];
			let mut fTemp182: F32 = F32::exp(-(self.fConst48 / fTemp1));
			let mut fTemp183: F32 = MoodscoreDsp_faustpower2_f(fTemp182);
			let mut fTemp184: F32 = 1.0 - fTemp183;
			let mut fTemp185: F32 = 1.0 - fTemp5 * fTemp183;
			let mut fTemp186: F32 = F32::sqrt(F32::max(0.0, -1.0 + MoodscoreDsp_faustpower2_f(fTemp185) / MoodscoreDsp_faustpower2_f(fTemp184)));
			let mut fTemp187: F32 = fTemp185 / fTemp184;
			self.fRec20[0] = fTemp182 * (1.0 - fTemp187 + fTemp186) * (self.fRec42[1] + (-1.0 + F32::exp(-(self.fConst49 / fTemp1)) / fTemp182) * self.fRec7[0]) + (fTemp187 - fTemp186) * self.fRec20[1];
			self.fVec19[(self.IOTA0 & 32767) as usize] = 1e-20 + 0.35355338 * self.fRec15[0];
			let mut fTemp188: F32 = 0.6 * self.fRec35[1] + self.fVec19[((i32::wrapping_sub(self.IOTA0, self.iConst51)) & 32767) as usize];
			let mut fRec21: F32 = 0.6 * (fTemp173 - fTemp188);
			let mut fTemp189: F32 = self.fRec60[0] * (0.5879378 * fTemp132 + 0.39284748 * fTemp126 + 1.0 * fTemp135) + self.fRec59[0] * (0.5879378 * fTemp120 + 0.39284748 * fTemp114) + self.fRec58[0] * (0.6805586 * fTemp108 + 0.5553029 * fTemp102) + self.fRec57[0] * (0.43776554 * fTemp96 + 0.19193739 * fTemp90) + self.fRec56[0] * (0.704927 * fTemp84 + 0.6300368 * fTemp78) + self.fRec55[0] * (0.32101977 * fTemp72 + 0.05547896 * fTemp66);
			let mut fTemp190: F32 = fTemp189 * fTemp137 / fTemp48 - fTemp44 * self.fRec122[1];
			self.fRec121[0] = fTemp190 * fTemp139 - self.fRec121[1] * fTemp42;
			self.fRec122[0] = fTemp42 * fTemp190 + self.fRec121[1] * fTemp139;
			let mut fTemp191: F32 = fTemp189 * fTemp44 / fTemp48 + self.fRec122[1] * fTemp137 + 2.0 * (self.fRec122[0] * fTemp140 / fTemp46) + self.fRec121[0] * fTemp141 / fTemp47;
			let mut fTemp192: F32 = fTemp191 * fTemp143 / fTemp39 - fTemp33 * self.fRec120[1];
			self.fRec119[0] = fTemp192 * fTemp145 - self.fRec119[1] * fTemp31;
			self.fRec120[0] = fTemp31 * fTemp192 + self.fRec119[1] * fTemp145;
			let mut fTemp193: F32 = fTemp191 * fTemp33 / fTemp39 + self.fRec120[1] * fTemp143 + 2.0 * (self.fRec120[0] * fTemp146 / fTemp35) + self.fRec119[0] * fTemp147 / fTemp36;
			let mut fTemp194: F32 = 0.9 * (fTemp193 / fTemp27);
			self.fRec118[0] = fTemp194 - self.fConst24 * (self.fConst23 * self.fRec118[2] + self.fConst22 * self.fRec118[1]);
			let mut fTemp195: F32 = self.fRec118[2] + self.fRec118[0] + 2.0 * self.fRec118[1];
			self.fRec123[0] = fTemp194 - self.fConst34 * (self.fConst33 * self.fRec123[2] + self.fConst31 * self.fRec123[1]);
			let mut fTemp196: F32 = self.fConst34 * (self.fRec123[2] + self.fRec123[0] + 2.0 * self.fRec123[1]) - self.fConst24 * fTemp195;
			let mut fTemp197: F32 = fTemp194 - (self.fConst24 * fTemp161 * fTemp195 + fTemp158 * fTemp196);
			let mut fTemp198: F32 = fTemp197 * self.fRec114[0];
			let mut fTemp199: F32 = (*input2) as F32;
			self.fRec124[0] = fTemp199 - self.fConst24 * (self.fConst23 * self.fRec124[2] + self.fConst22 * self.fRec124[1]) + fTemp198;
			let mut fTemp200: F32 = fTemp199 - self.fConst24 * self.fRec116[0] * (self.fRec124[2] + self.fRec124[0] + 2.0 * self.fRec124[1]) + fTemp198;
			self.fVec20[(self.IOTA0 & 16383) as usize] = fTemp200;
			let mut fTemp201: F32 = 0.3 * (fTemp171 * self.fVec20[((i32::wrapping_sub(self.IOTA0, iTemp172)) & 16383) as usize] + fTemp170 * self.fVec20[((i32::wrapping_sub(self.IOTA0, iTemp168)) & 16383) as usize]);
			self.fVec21[(self.IOTA0 & 16383) as usize] = 1e-20 + 0.35355338 * self.fRec18[0];
			let mut fTemp202: F32 = self.fVec21[((i32::wrapping_sub(self.IOTA0, self.iConst53)) & 16383) as usize] - (0.6 * self.fRec22[1] + fTemp201);
			self.fVec22[(self.IOTA0 & 2047) as usize] = fTemp202;
			self.fRec22[0] = self.fVec22[((i32::wrapping_sub(self.IOTA0, self.iConst54)) & 2047) as usize];
			let mut fTemp203: F32 = F32::exp(-(self.fConst57 / fTemp1));
			let mut fTemp204: F32 = MoodscoreDsp_faustpower2_f(fTemp203);
			let mut fTemp205: F32 = 1.0 - fTemp204;
			let mut fTemp206: F32 = 1.0 - fTemp5 * fTemp204;
			let mut fTemp207: F32 = F32::sqrt(F32::max(0.0, -1.0 + MoodscoreDsp_faustpower2_f(fTemp206) / MoodscoreDsp_faustpower2_f(fTemp205)));
			let mut fTemp208: F32 = fTemp206 / fTemp205;
			self.fRec23[0] = fTemp203 * (1.0 - fTemp208 + fTemp207) * (self.fRec43[1] + (-1.0 + F32::exp(-(self.fConst58 / fTemp1)) / fTemp203) * self.fRec13[0]) + (fTemp208 - fTemp207) * self.fRec23[1];
			let mut fTemp209: F32 = F32::exp(-(self.fConst61 / fTemp1));
			let mut fTemp210: F32 = MoodscoreDsp_faustpower2_f(fTemp209);
			let mut fTemp211: F32 = 1.0 - fTemp210;
			let mut fTemp212: F32 = 1.0 - fTemp5 * fTemp210;
			let mut fTemp213: F32 = F32::sqrt(F32::max(0.0, -1.0 + MoodscoreDsp_faustpower2_f(fTemp212) / MoodscoreDsp_faustpower2_f(fTemp211)));
			let mut fTemp214: F32 = fTemp212 / fTemp211;
			self.fRec24[0] = fTemp209 * (1.0 - fTemp214 + fTemp213) * (self.fRec39[1] + (-1.0 + F32::exp(-(self.fConst62 / fTemp1)) / fTemp209) * self.fRec11[0]) + (fTemp214 - fTemp213) * self.fRec24[1];
			self.fVec23[(self.IOTA0 & 32767) as usize] = 1e-20 + 0.35355338 * self.fRec23[0];
			let mut fTemp215: F32 = self.fVec23[((i32::wrapping_sub(self.IOTA0, self.iConst64)) & 32767) as usize] + fTemp173 + 0.6 * self.fRec25[1];
			self.fVec24[(self.IOTA0 & 4095) as usize] = fTemp215;
			self.fRec25[0] = self.fVec24[((i32::wrapping_sub(self.IOTA0, self.iConst65)) & 4095) as usize];
			let mut fTemp216: F32 = F32::exp(-(self.fConst68 / fTemp1));
			let mut fTemp217: F32 = MoodscoreDsp_faustpower2_f(fTemp216);
			let mut fTemp218: F32 = 1.0 - fTemp217;
			let mut fTemp219: F32 = 1.0 - fTemp5 * fTemp217;
			let mut fTemp220: F32 = F32::sqrt(F32::max(0.0, -1.0 + MoodscoreDsp_faustpower2_f(fTemp219) / MoodscoreDsp_faustpower2_f(fTemp218)));
			let mut fTemp221: F32 = fTemp219 / fTemp218;
			self.fRec26[0] = fTemp216 * (1.0 - fTemp221 + fTemp220) * (self.fRec44[1] + (-1.0 + F32::exp(-(self.fConst69 / fTemp1)) / fTemp216) * self.fRec9[0]) + (fTemp221 - fTemp220) * self.fRec26[1];
			let mut fRec27: F32 = 0.6 * (fTemp173 - fTemp181);
			let mut fRec28: F32 = -(0.6 * fTemp215);
			let mut fRec29: F32 = -(0.6 * fTemp174);
			self.fVec25[(self.IOTA0 & 16383) as usize] = 1e-20 + 0.35355338 * self.fRec20[0];
			let mut fTemp222: F32 = fTemp201 + self.fVec25[((i32::wrapping_sub(self.IOTA0, self.iConst71)) & 16383) as usize] - 0.6 * self.fRec30[1];
			self.fVec26[(self.IOTA0 & 2047) as usize] = fTemp222;
			self.fRec30[0] = self.fVec26[((i32::wrapping_sub(self.IOTA0, self.iConst72)) & 2047) as usize];
			self.fVec27[(self.IOTA0 & 16383) as usize] = 1e-20 + 0.35355338 * self.fRec26[0];
			let mut fTemp223: F32 = self.fVec27[((i32::wrapping_sub(self.IOTA0, self.iConst74)) & 16383) as usize] - (fTemp201 + 0.6 * self.fRec37[1]);
			self.fVec28[(self.IOTA0 & 4095) as usize] = fTemp223;
			let mut fRec31: F32 = 0.6 * fTemp223;
			let mut fRec32: F32 = 0.6 * fTemp222;
			self.fVec29[(self.IOTA0 & 16383) as usize] = 1e-20 + 0.35355338 * self.fRec24[0];
			let mut fTemp224: F32 = fTemp201 + self.fVec29[((i32::wrapping_sub(self.IOTA0, self.iConst76)) & 16383) as usize] - 0.6 * self.fRec36[1];
			self.fVec30[(self.IOTA0 & 4095) as usize] = fTemp224;
			let mut fRec33: F32 = 0.6 * fTemp224;
			let mut fRec34: F32 = 0.6 * fTemp202;
			self.fVec31[(self.IOTA0 & 4095) as usize] = fTemp188 - fTemp173;
			self.fRec35[0] = self.fVec31[((i32::wrapping_sub(self.IOTA0, self.iConst77)) & 4095) as usize];
			self.fRec36[0] = self.fVec30[((i32::wrapping_sub(self.IOTA0, self.iConst78)) & 4095) as usize];
			self.fRec37[0] = self.fVec28[((i32::wrapping_sub(self.IOTA0, self.iConst79)) & 4095) as usize];
			let mut fTemp225: F32 = self.fRec17[1] + self.fRec19[1];
			self.fRec38[0] = fRec27 + fRec29 + fRec31 + fRec32 + self.fRec37[1] + self.fRec30[1] + fTemp225 - (fRec21 + fRec28 + fRec34 + fRec33 + self.fRec36[1] + self.fRec25[1] + self.fRec35[1] + self.fRec22[1]);
			self.fRec39[0] = fRec21 + fRec28 + fRec31 + fRec32 + self.fRec37[1] + self.fRec30[1] + self.fRec35[1] + self.fRec25[1] - (fRec27 + fRec29 + fRec34 + fRec33 + self.fRec36[1] + self.fRec22[1] + fTemp225);
			let mut fTemp226: F32 = self.fRec17[1] + self.fRec35[1];
			self.fRec40[0] = fRec27 + fRec28 + fRec34 + fRec32 + self.fRec30[1] + self.fRec25[1] + self.fRec19[1] + self.fRec22[1] - (fRec21 + fRec29 + fRec31 + fRec33 + self.fRec36[1] + self.fRec37[1] + fTemp226);
			self.fRec41[0] = fRec21 + fRec29 + fRec34 + fRec32 + self.fRec30[1] + self.fRec22[1] + fTemp226 - (fRec27 + fRec28 + fRec31 + fRec33 + self.fRec36[1] + self.fRec37[1] + self.fRec19[1] + self.fRec25[1]);
			let mut fTemp227: F32 = self.fRec35[1] + fTemp225;
			self.fRec42[0] = fRec27 + fRec21 + fRec29 + fRec28 + fRec34 + fRec31 + fRec33 + fRec32 + self.fRec36[1] + self.fRec37[1] + self.fRec30[1] + self.fRec25[1] + self.fRec22[1] + fTemp227;
			self.fRec43[0] = fRec34 + fRec31 + fRec33 + fRec32 + self.fRec36[1] + self.fRec37[1] + self.fRec22[1] + self.fRec30[1] - (fRec27 + fRec21 + fRec29 + fRec28 + self.fRec25[1] + fTemp227);
			let mut fTemp228: F32 = self.fRec19[1] + self.fRec35[1];
			self.fRec44[0] = fRec29 + fRec28 + fRec33 + fRec32 + self.fRec36[1] + self.fRec30[1] + self.fRec17[1] + self.fRec25[1] - (fRec27 + fRec21 + fRec34 + fRec31 + self.fRec37[1] + self.fRec22[1] + fTemp228);
			self.fRec45[0] = fRec27 + fRec21 + fRec33 + fRec32 + self.fRec36[1] + self.fRec30[1] + fTemp228 - (fRec29 + fRec28 + fRec34 + fRec31 + self.fRec37[1] + self.fRec25[1] + self.fRec17[1] + self.fRec22[1]);
			let mut fTemp229: F32 = self.fRec43[0] + self.fRec44[0];
			self.fRec125[0] = fSlow35 + self.fConst2 * self.fRec125[1];
			let mut fTemp230: F32 = 1.0 + self.fRec125[0];
			self.fRec128[0] = fSlow36 + self.fConst2 * self.fRec128[1];
			let mut fTemp231: F32 = 1.0 - self.fConst80 * self.fRec128[0];
			let mut fTemp232: F32 = 1.0 - self.fRec114[0];
			self.fRec129[0] = fTemp153 + self.fConst80 * self.fRec128[0] * self.fRec129[1] + fTemp197 * fTemp232;
			self.fRec127[0] = self.fConst80 * self.fRec127[1] * self.fRec128[0] + self.fRec129[0] * fTemp231;
			let mut fTemp233: F32 = fTemp231 * self.fRec127[0];
			self.fRec126[0] = fTemp233 - self.fConst24 * (self.fConst23 * self.fRec126[2] + self.fConst22 * self.fRec126[1]);
			let mut fTemp234: F32 = fTemp233 - self.fConst24 * (self.fRec126[2] + self.fRec126[0] + 2.0 * self.fRec126[1]) * self.fRec116[0];
			let mut fTemp235: F32 = fTemp234 * fTemp230 + 0.37 * fTemp229 * fTemp0;
			let mut fTemp236: F32 = self.fRec3[1] + self.fConst81 * (fTemp235 - self.fRec4[1]);
			self.fRec3[0] = self.fConst83 * fTemp236 - self.fRec3[1];
			self.fRec4[0] = 2.0 * (self.fRec4[1] + self.fConst84 * fTemp236) - self.fRec4[1];
			let mut fTemp237: F32 = self.fRec4[1] + self.fRec2[1];
			let mut fTemp238: F32 = self.fRec1[1] + self.fConst81 * (fTemp235 + self.fConst86 * fTemp236 - fTemp237);
			self.fRec1[0] = self.fConst83 * fTemp238 - self.fRec1[1];
			self.fRec2[0] = 2.0 * (self.fRec2[1] + self.fConst84 * fTemp238) - self.fRec2[1];
			let mut fTemp239: F32 = self.fRec43[0] - self.fRec44[0];
			self.fRec136[0] = fTemp152 + self.fConst80 * self.fRec128[0] * self.fRec136[1] + fTemp232 * fTemp162;
			self.fRec135[0] = self.fConst80 * self.fRec128[0] * self.fRec135[1] + fTemp231 * self.fRec136[0];
			let mut fTemp240: F32 = fTemp231 * self.fRec135[0];
			self.fRec134[0] = fTemp240 - self.fConst24 * (self.fConst23 * self.fRec134[2] + self.fConst22 * self.fRec134[1]);
			let mut fTemp241: F32 = fTemp240 - self.fConst24 * self.fRec116[0] * (self.fRec134[2] + self.fRec134[0] + 2.0 * self.fRec134[1]);
			let mut fTemp242: F32 = self.fRec132[1] + self.fConst81 * (0.5 * (fTemp230 * (fTemp234 + fTemp241) + 0.37 * fTemp0 * (fTemp229 + fTemp239)) - self.fRec133[1]);
			self.fRec132[0] = self.fConst83 * fTemp242 - self.fRec132[1];
			let mut fTemp243: F32 = self.fRec133[1] + self.fConst84 * fTemp242;
			self.fRec133[0] = 2.0 * fTemp243 - self.fRec133[1];
			let mut fTemp244: F32 = self.fRec130[1] + self.fConst81 * (fTemp243 - self.fRec131[1]);
			self.fRec130[0] = self.fConst83 * fTemp244 - self.fRec130[1];
			self.fRec131[0] = 2.0 * (self.fRec131[1] + self.fConst84 * fTemp244) - self.fRec131[1];
			*output0 = (F32::tanh((self.fConst87 * (self.fConst85 * fTemp236 - 1.4142135 * fTemp238 - self.fConst81 * (fTemp238 - fTemp244)) + fTemp235 + self.fRec131[1] - fTemp237) * self.fRec0[0])) as FaustFloat;
			let mut fTemp245: F32 = fTemp230 * fTemp241 + 0.37 * fTemp0 * fTemp239;
			let mut fTemp246: F32 = self.fRec139[1] + self.fConst81 * (fTemp245 - self.fRec140[1]);
			self.fRec139[0] = self.fConst83 * fTemp246 - self.fRec139[1];
			self.fRec140[0] = 2.0 * (self.fRec140[1] + self.fConst84 * fTemp246) - self.fRec140[1];
			let mut fTemp247: F32 = self.fRec140[1] + self.fRec138[1];
			let mut fTemp248: F32 = self.fRec137[1] + self.fConst81 * (fTemp245 + self.fConst86 * fTemp246 - fTemp247);
			self.fRec137[0] = self.fConst83 * fTemp248 - self.fRec137[1];
			self.fRec138[0] = 2.0 * (self.fRec138[1] + self.fConst84 * fTemp248) - self.fRec138[1];
			*output1 = (F32::tanh(self.fRec0[0] * (self.fConst87 * (self.fConst85 * fTemp246 - 1.4142135 * fTemp248 + self.fConst81 * (fTemp244 - fTemp248)) + fTemp245 + self.fRec131[1] - fTemp247))) as FaustFloat;
			*output2 = (0.5 * (0.9 * ((fTemp193 + fTemp148) / fTemp27) - (self.fConst24 * fTemp161 * (fTemp195 + fTemp150) + fTemp158 * (fTemp196 + fTemp151)))) as FaustFloat;
			self.iVec0[1] = self.iVec0[0];
			self.fRec0[1] = self.fRec0[0];
			self.fRec5[1] = self.fRec5[0];
			self.fRec6[1] = self.fRec6[0];
			self.fRec7[1] = self.fRec7[0];
			self.fRec8[1] = self.fRec8[0];
			self.fRec9[1] = self.fRec9[0];
			self.fRec10[1] = self.fRec10[0];
			self.fRec11[1] = self.fRec11[0];
			self.fRec12[1] = self.fRec12[0];
			self.fRec13[1] = self.fRec13[0];
			self.fRec46[1] = self.fRec46[0];
			self.fRec47[1] = self.fRec47[0];
			self.fRec14[1] = self.fRec14[0];
			self.fRec15[1] = self.fRec15[0];
			self.fRec16[1] = self.fRec16[0];
			self.IOTA0 = i32::wrapping_add(self.IOTA0, 1);
			self.fRec49[1] = self.fRec49[0];
			self.fRec50[1] = self.fRec50[0];
			self.fRec55[1] = self.fRec55[0];
			self.fRec56[1] = self.fRec56[0];
			self.fRec57[1] = self.fRec57[0];
			self.fRec58[1] = self.fRec58[0];
			self.fRec59[1] = self.fRec59[0];
			self.fRec60[1] = self.fRec60[0];
			self.fRec61[1] = self.fRec61[0];
			self.iRec63[1] = self.iRec63[0];
			self.fVec2[1] = self.fVec2[0];
			self.fRec62[1] = self.fRec62[0];
			self.fRec64[1] = self.fRec64[0];
			self.fRec65[1] = self.fRec65[0];
			self.fRec66[1] = self.fRec66[0];
			self.fVec3[1] = self.fVec3[0];
			self.fRec67[1] = self.fRec67[0];
			self.fRec68[1] = self.fRec68[0];
			self.fRec69[1] = self.fRec69[0];
			self.fVec4[1] = self.fVec4[0];
			self.fRec70[1] = self.fRec70[0];
			self.fRec71[1] = self.fRec71[0];
			self.fRec72[1] = self.fRec72[0];
			self.fRec73[1] = self.fRec73[0];
			self.fVec5[1] = self.fVec5[0];
			self.fRec74[1] = self.fRec74[0];
			self.fRec75[1] = self.fRec75[0];
			self.fRec76[1] = self.fRec76[0];
			self.fVec6[1] = self.fVec6[0];
			self.fRec77[1] = self.fRec77[0];
			self.fRec78[1] = self.fRec78[0];
			self.fRec79[1] = self.fRec79[0];
			self.fRec80[1] = self.fRec80[0];
			self.fVec7[1] = self.fVec7[0];
			self.fRec81[1] = self.fRec81[0];
			self.fRec82[1] = self.fRec82[0];
			self.fRec83[1] = self.fRec83[0];
			self.fVec8[1] = self.fVec8[0];
			self.fRec84[1] = self.fRec84[0];
			self.fRec85[1] = self.fRec85[0];
			self.fRec86[1] = self.fRec86[0];
			self.fRec87[1] = self.fRec87[0];
			self.fVec9[1] = self.fVec9[0];
			self.fRec88[1] = self.fRec88[0];
			self.fRec89[1] = self.fRec89[0];
			self.fRec90[1] = self.fRec90[0];
			self.fVec10[1] = self.fVec10[0];
			self.fRec91[1] = self.fRec91[0];
			self.fRec92[1] = self.fRec92[0];
			self.fRec93[1] = self.fRec93[0];
			self.fRec94[1] = self.fRec94[0];
			self.fVec11[1] = self.fVec11[0];
			self.fRec95[1] = self.fRec95[0];
			self.fRec96[1] = self.fRec96[0];
			self.fRec97[1] = self.fRec97[0];
			self.fVec12[1] = self.fVec12[0];
			self.fRec98[1] = self.fRec98[0];
			self.fRec99[1] = self.fRec99[0];
			self.fRec100[1] = self.fRec100[0];
			self.fRec101[1] = self.fRec101[0];
			self.fVec13[1] = self.fVec13[0];
			self.fRec102[1] = self.fRec102[0];
			self.fRec103[1] = self.fRec103[0];
			self.fRec104[1] = self.fRec104[0];
			self.fVec14[1] = self.fVec14[0];
			self.fRec107[1] = self.fRec107[0];
			self.fRec105[1] = self.fRec105[0];
			self.fRec106[1] = self.fRec106[0];
			self.fRec53[1] = self.fRec53[0];
			self.fRec54[1] = self.fRec54[0];
			self.fRec51[1] = self.fRec51[0];
			self.fRec52[1] = self.fRec52[0];
			self.fRec48[2] = self.fRec48[1];
			self.fRec48[1] = self.fRec48[0];
			self.fRec108[2] = self.fRec108[1];
			self.fRec108[1] = self.fRec108[0];
			self.fRec110[2] = self.fRec110[1];
			self.fRec110[1] = self.fRec110[0];
			self.fRec111[2] = self.fRec111[1];
			self.fRec111[1] = self.fRec111[0];
			self.fRec109[1] = self.fRec109[0];
			self.fRec112[1] = self.fRec112[0];
			self.fRec113[1] = self.fRec113[0];
			self.fRec114[1] = self.fRec114[0];
			self.fRec115[2] = self.fRec115[1];
			self.fRec115[1] = self.fRec115[0];
			self.fRec116[1] = self.fRec116[0];
			self.fRec117[1] = self.fRec117[0];
			self.fRec17[1] = self.fRec17[0];
			self.fRec18[1] = self.fRec18[0];
			self.fRec19[1] = self.fRec19[0];
			self.fRec20[1] = self.fRec20[0];
			self.fRec121[1] = self.fRec121[0];
			self.fRec122[1] = self.fRec122[0];
			self.fRec119[1] = self.fRec119[0];
			self.fRec120[1] = self.fRec120[0];
			self.fRec118[2] = self.fRec118[1];
			self.fRec118[1] = self.fRec118[0];
			self.fRec123[2] = self.fRec123[1];
			self.fRec123[1] = self.fRec123[0];
			self.fRec124[2] = self.fRec124[1];
			self.fRec124[1] = self.fRec124[0];
			self.fRec22[1] = self.fRec22[0];
			self.fRec23[1] = self.fRec23[0];
			self.fRec24[1] = self.fRec24[0];
			self.fRec25[1] = self.fRec25[0];
			self.fRec26[1] = self.fRec26[0];
			self.fRec30[1] = self.fRec30[0];
			self.fRec35[1] = self.fRec35[0];
			self.fRec36[1] = self.fRec36[0];
			self.fRec37[1] = self.fRec37[0];
			self.fRec38[2] = self.fRec38[1];
			self.fRec38[1] = self.fRec38[0];
			self.fRec39[2] = self.fRec39[1];
			self.fRec39[1] = self.fRec39[0];
			self.fRec40[2] = self.fRec40[1];
			self.fRec40[1] = self.fRec40[0];
			self.fRec41[2] = self.fRec41[1];
			self.fRec41[1] = self.fRec41[0];
			self.fRec42[2] = self.fRec42[1];
			self.fRec42[1] = self.fRec42[0];
			self.fRec43[2] = self.fRec43[1];
			self.fRec43[1] = self.fRec43[0];
			self.fRec44[2] = self.fRec44[1];
			self.fRec44[1] = self.fRec44[0];
			self.fRec45[2] = self.fRec45[1];
			self.fRec45[1] = self.fRec45[0];
			self.fRec125[1] = self.fRec125[0];
			self.fRec128[1] = self.fRec128[0];
			self.fRec129[1] = self.fRec129[0];
			self.fRec127[1] = self.fRec127[0];
			self.fRec126[2] = self.fRec126[1];
			self.fRec126[1] = self.fRec126[0];
			self.fRec3[1] = self.fRec3[0];
			self.fRec4[1] = self.fRec4[0];
			self.fRec1[1] = self.fRec1[0];
			self.fRec2[1] = self.fRec2[0];
			self.fRec136[1] = self.fRec136[0];
			self.fRec135[1] = self.fRec135[0];
			self.fRec134[2] = self.fRec134[1];
			self.fRec134[1] = self.fRec134[0];
			self.fRec132[1] = self.fRec132[0];
			self.fRec133[1] = self.fRec133[0];
			self.fRec130[1] = self.fRec130[0];
			self.fRec131[1] = self.fRec131[0];
			self.fRec139[1] = self.fRec139[0];
			self.fRec140[1] = self.fRec140[0];
			self.fRec137[1] = self.fRec137[0];
			self.fRec138[1] = self.fRec138[0];
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
