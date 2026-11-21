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
	fConst6: F32,
	fConst7: F32,
	fConst8: F32,
	fHslider3: FaustFloat,
	fRec47: [F32;2],
	fConst9: F32,
	fConst10: F32,
	fRec14: [F32;2],
	fHslider4: FaustFloat,
	fRec48: [F32;2],
	fConst11: F32,
	fHslider5: FaustFloat,
	fRec49: [F32;2],
	fHslider6: FaustFloat,
	fRec50: [F32;2],
	fHslider7: FaustFloat,
	fRec55: [F32;2],
	fHslider8: FaustFloat,
	fRec56: [F32;2],
	fHslider9: FaustFloat,
	fRec57: [F32;2],
	fHslider10: FaustFloat,
	fRec58: [F32;2],
	fHslider11: FaustFloat,
	fRec59: [F32;2],
	fHslider12: FaustFloat,
	fRec60: [F32;2],
	fHslider13: FaustFloat,
	fRec61: [F32;2],
	fConst12: F32,
	fConst13: F32,
	iRec63: [i32;2],
	fVec1: [F32;2],
	fConst14: F32,
	fRec62: [F32;2],
	fHslider14: FaustFloat,
	fRec64: [F32;2],
	fHslider15: FaustFloat,
	fRec65: [F32;2],
	fConst15: F32,
	fRec66: [F32;2],
	fVec2: [F32;2],
	fRec67: [F32;2],
	fRec68: [F32;2],
	fHslider16: FaustFloat,
	fRec69: [F32;2],
	fVec3: [F32;2],
	fRec70: [F32;2],
	fHslider17: FaustFloat,
	fRec71: [F32;2],
	fHslider18: FaustFloat,
	fRec72: [F32;2],
	fRec73: [F32;2],
	fVec4: [F32;2],
	fRec74: [F32;2],
	fRec75: [F32;2],
	fHslider19: FaustFloat,
	fRec76: [F32;2],
	fVec5: [F32;2],
	fRec77: [F32;2],
	fHslider20: FaustFloat,
	fRec78: [F32;2],
	fHslider21: FaustFloat,
	fRec79: [F32;2],
	fRec80: [F32;2],
	fVec6: [F32;2],
	fRec81: [F32;2],
	fRec82: [F32;2],
	fHslider22: FaustFloat,
	fRec83: [F32;2],
	fVec7: [F32;2],
	fRec84: [F32;2],
	fHslider23: FaustFloat,
	fRec85: [F32;2],
	fHslider24: FaustFloat,
	fRec86: [F32;2],
	fRec87: [F32;2],
	fVec8: [F32;2],
	fRec88: [F32;2],
	fRec89: [F32;2],
	fHslider25: FaustFloat,
	fRec90: [F32;2],
	fVec9: [F32;2],
	fRec91: [F32;2],
	fHslider26: FaustFloat,
	fRec92: [F32;2],
	fHslider27: FaustFloat,
	fRec93: [F32;2],
	fRec94: [F32;2],
	fVec10: [F32;2],
	fRec95: [F32;2],
	fRec96: [F32;2],
	fHslider28: FaustFloat,
	fRec97: [F32;2],
	fVec11: [F32;2],
	fRec100: [F32;2],
	fHslider29: FaustFloat,
	fRec101: [F32;2],
	fHslider30: FaustFloat,
	fRec102: [F32;2],
	fRec98: [F32;2],
	fRec99: [F32;2],
	fHslider31: FaustFloat,
	fRec103: [F32;2],
	fVec12: [F32;2],
	fRec104: [F32;2],
	fRec105: [F32;2],
	fVec13: [F32;2],
	fRec106: [F32;2],
	fRec107: [F32;2],
	fRec53: [F32;2],
	fRec54: [F32;2],
	fRec51: [F32;2],
	fRec52: [F32;2],
	IOTA0: i32,
	fVec14: [F32;16384],
	fHslider32: FaustFloat,
	fRec108: [F32;2],
	fConst16: F32,
	fVec15: [F32;16384],
	fConst17: F32,
	iConst18: i32,
	fVec16: [F32;4096],
	fConst19: F32,
	fConst20: F32,
	fConst21: F32,
	fConst22: F32,
	fRec16: [F32;2],
	fConst23: F32,
	fConst24: F32,
	fConst25: F32,
	fConst26: F32,
	fRec17: [F32;2],
	fConst27: F32,
	fConst28: F32,
	fConst29: F32,
	fConst30: F32,
	fRec18: [F32;2],
	fConst31: F32,
	fConst32: F32,
	fConst33: F32,
	fConst34: F32,
	fRec19: [F32;2],
	fConst35: F32,
	fConst36: F32,
	fConst37: F32,
	fConst38: F32,
	fRec20: [F32;2],
	fRec111: [F32;2],
	fRec112: [F32;2],
	fRec109: [F32;2],
	fRec110: [F32;2],
	fVec17: [F32;16384],
	fVec18: [F32;32768],
	fConst39: F32,
	iConst40: i32,
	fVec19: [F32;4096],
	iConst41: i32,
	fRec21: [F32;2],
	fConst42: F32,
	fConst43: F32,
	fConst44: F32,
	fConst45: F32,
	fRec22: [F32;2],
	fVec20: [F32;16384],
	fConst46: F32,
	iConst47: i32,
	fVec21: [F32;4096],
	iConst48: i32,
	fRec23: [F32;2],
	fVec22: [F32;16384],
	fConst49: F32,
	iConst50: i32,
	fVec23: [F32;2048],
	fConst51: F32,
	fConst52: F32,
	fConst53: F32,
	fConst54: F32,
	fRec25: [F32;2],
	fVec24: [F32;32768],
	fConst55: F32,
	iConst56: i32,
	fVec25: [F32;4096],
	iConst57: i32,
	fRec27: [F32;2],
	fVec26: [F32;16384],
	fConst58: F32,
	iConst59: i32,
	fVec27: [F32;2048],
	fVec28: [F32;32768],
	fConst60: F32,
	iConst61: i32,
	fVec29: [F32;2048],
	iConst62: i32,
	fRec31: [F32;2],
	iConst63: i32,
	fRec32: [F32;2],
	fVec30: [F32;16384],
	fConst64: F32,
	iConst65: i32,
	fVec31: [F32;4096],
	iConst66: i32,
	fRec34: [F32;2],
	iConst67: i32,
	fRec36: [F32;2],
	iConst68: i32,
	fRec37: [F32;2],
	fRec38: [F32;3],
	fRec39: [F32;3],
	fRec40: [F32;3],
	fRec41: [F32;3],
	fRec42: [F32;3],
	fRec43: [F32;3],
	fRec44: [F32;3],
	fRec45: [F32;3],
	fHslider33: FaustFloat,
	fRec113: [F32;2],
	fHslider34: FaustFloat,
	fRec115: [F32;2],
	fConst69: F32,
	fRec116: [F32;2],
	fRec114: [F32;2],
	fConst70: F32,
	fConst71: F32,
	fConst72: F32,
	fRec3: [F32;2],
	fConst73: F32,
	fRec4: [F32;2],
	fConst74: F32,
	fConst75: F32,
	fRec1: [F32;2],
	fRec2: [F32;2],
	fRec122: [F32;2],
	fRec121: [F32;2],
	fRec119: [F32;2],
	fRec120: [F32;2],
	fRec117: [F32;2],
	fRec118: [F32;2],
	fConst76: F32,
	fRec125: [F32;2],
	fRec126: [F32;2],
	fRec123: [F32;2],
	fRec124: [F32;2],
}


fn MoodscoreDsp_faustpower2_f(value: F32) -> F32 {
	return value * value;
}
pub const FAUST_INPUTS: usize = 4;
pub const FAUST_OUTPUTS: usize = 2;
pub const FAUST_ACTIVES: usize = 35;
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
			fConst6: 0.0,
			fConst7: 0.0,
			fConst8: 0.0,
			fHslider3: 0.0,
			fRec47: [0.0;2],
			fConst9: 0.0,
			fConst10: 0.0,
			fRec14: [0.0;2],
			fHslider4: 0.0,
			fRec48: [0.0;2],
			fConst11: 0.0,
			fHslider5: 0.0,
			fRec49: [0.0;2],
			fHslider6: 0.0,
			fRec50: [0.0;2],
			fHslider7: 0.0,
			fRec55: [0.0;2],
			fHslider8: 0.0,
			fRec56: [0.0;2],
			fHslider9: 0.0,
			fRec57: [0.0;2],
			fHslider10: 0.0,
			fRec58: [0.0;2],
			fHslider11: 0.0,
			fRec59: [0.0;2],
			fHslider12: 0.0,
			fRec60: [0.0;2],
			fHslider13: 0.0,
			fRec61: [0.0;2],
			fConst12: 0.0,
			fConst13: 0.0,
			iRec63: [0;2],
			fVec1: [0.0;2],
			fConst14: 0.0,
			fRec62: [0.0;2],
			fHslider14: 0.0,
			fRec64: [0.0;2],
			fHslider15: 0.0,
			fRec65: [0.0;2],
			fConst15: 0.0,
			fRec66: [0.0;2],
			fVec2: [0.0;2],
			fRec67: [0.0;2],
			fRec68: [0.0;2],
			fHslider16: 0.0,
			fRec69: [0.0;2],
			fVec3: [0.0;2],
			fRec70: [0.0;2],
			fHslider17: 0.0,
			fRec71: [0.0;2],
			fHslider18: 0.0,
			fRec72: [0.0;2],
			fRec73: [0.0;2],
			fVec4: [0.0;2],
			fRec74: [0.0;2],
			fRec75: [0.0;2],
			fHslider19: 0.0,
			fRec76: [0.0;2],
			fVec5: [0.0;2],
			fRec77: [0.0;2],
			fHslider20: 0.0,
			fRec78: [0.0;2],
			fHslider21: 0.0,
			fRec79: [0.0;2],
			fRec80: [0.0;2],
			fVec6: [0.0;2],
			fRec81: [0.0;2],
			fRec82: [0.0;2],
			fHslider22: 0.0,
			fRec83: [0.0;2],
			fVec7: [0.0;2],
			fRec84: [0.0;2],
			fHslider23: 0.0,
			fRec85: [0.0;2],
			fHslider24: 0.0,
			fRec86: [0.0;2],
			fRec87: [0.0;2],
			fVec8: [0.0;2],
			fRec88: [0.0;2],
			fRec89: [0.0;2],
			fHslider25: 0.0,
			fRec90: [0.0;2],
			fVec9: [0.0;2],
			fRec91: [0.0;2],
			fHslider26: 0.0,
			fRec92: [0.0;2],
			fHslider27: 0.0,
			fRec93: [0.0;2],
			fRec94: [0.0;2],
			fVec10: [0.0;2],
			fRec95: [0.0;2],
			fRec96: [0.0;2],
			fHslider28: 0.0,
			fRec97: [0.0;2],
			fVec11: [0.0;2],
			fRec100: [0.0;2],
			fHslider29: 0.0,
			fRec101: [0.0;2],
			fHslider30: 0.0,
			fRec102: [0.0;2],
			fRec98: [0.0;2],
			fRec99: [0.0;2],
			fHslider31: 0.0,
			fRec103: [0.0;2],
			fVec12: [0.0;2],
			fRec104: [0.0;2],
			fRec105: [0.0;2],
			fVec13: [0.0;2],
			fRec106: [0.0;2],
			fRec107: [0.0;2],
			fRec53: [0.0;2],
			fRec54: [0.0;2],
			fRec51: [0.0;2],
			fRec52: [0.0;2],
			IOTA0: 0,
			fVec14: [0.0;16384],
			fHslider32: 0.0,
			fRec108: [0.0;2],
			fConst16: 0.0,
			fVec15: [0.0;16384],
			fConst17: 0.0,
			iConst18: 0,
			fVec16: [0.0;4096],
			fConst19: 0.0,
			fConst20: 0.0,
			fConst21: 0.0,
			fConst22: 0.0,
			fRec16: [0.0;2],
			fConst23: 0.0,
			fConst24: 0.0,
			fConst25: 0.0,
			fConst26: 0.0,
			fRec17: [0.0;2],
			fConst27: 0.0,
			fConst28: 0.0,
			fConst29: 0.0,
			fConst30: 0.0,
			fRec18: [0.0;2],
			fConst31: 0.0,
			fConst32: 0.0,
			fConst33: 0.0,
			fConst34: 0.0,
			fRec19: [0.0;2],
			fConst35: 0.0,
			fConst36: 0.0,
			fConst37: 0.0,
			fConst38: 0.0,
			fRec20: [0.0;2],
			fRec111: [0.0;2],
			fRec112: [0.0;2],
			fRec109: [0.0;2],
			fRec110: [0.0;2],
			fVec17: [0.0;16384],
			fVec18: [0.0;32768],
			fConst39: 0.0,
			iConst40: 0,
			fVec19: [0.0;4096],
			iConst41: 0,
			fRec21: [0.0;2],
			fConst42: 0.0,
			fConst43: 0.0,
			fConst44: 0.0,
			fConst45: 0.0,
			fRec22: [0.0;2],
			fVec20: [0.0;16384],
			fConst46: 0.0,
			iConst47: 0,
			fVec21: [0.0;4096],
			iConst48: 0,
			fRec23: [0.0;2],
			fVec22: [0.0;16384],
			fConst49: 0.0,
			iConst50: 0,
			fVec23: [0.0;2048],
			fConst51: 0.0,
			fConst52: 0.0,
			fConst53: 0.0,
			fConst54: 0.0,
			fRec25: [0.0;2],
			fVec24: [0.0;32768],
			fConst55: 0.0,
			iConst56: 0,
			fVec25: [0.0;4096],
			iConst57: 0,
			fRec27: [0.0;2],
			fVec26: [0.0;16384],
			fConst58: 0.0,
			iConst59: 0,
			fVec27: [0.0;2048],
			fVec28: [0.0;32768],
			fConst60: 0.0,
			iConst61: 0,
			fVec29: [0.0;2048],
			iConst62: 0,
			fRec31: [0.0;2],
			iConst63: 0,
			fRec32: [0.0;2],
			fVec30: [0.0;16384],
			fConst64: 0.0,
			iConst65: 0,
			fVec31: [0.0;4096],
			iConst66: 0,
			fRec34: [0.0;2],
			iConst67: 0,
			fRec36: [0.0;2],
			iConst68: 0,
			fRec37: [0.0;2],
			fRec38: [0.0;3],
			fRec39: [0.0;3],
			fRec40: [0.0;3],
			fRec41: [0.0;3],
			fRec42: [0.0;3],
			fRec43: [0.0;3],
			fRec44: [0.0;3],
			fRec45: [0.0;3],
			fHslider33: 0.0,
			fRec113: [0.0;2],
			fHslider34: 0.0,
			fRec115: [0.0;2],
			fConst69: 0.0,
			fRec116: [0.0;2],
			fRec114: [0.0;2],
			fConst70: 0.0,
			fConst71: 0.0,
			fConst72: 0.0,
			fRec3: [0.0;2],
			fConst73: 0.0,
			fRec4: [0.0;2],
			fConst74: 0.0,
			fConst75: 0.0,
			fRec1: [0.0;2],
			fRec2: [0.0;2],
			fRec122: [0.0;2],
			fRec121: [0.0;2],
			fRec119: [0.0;2],
			fRec120: [0.0;2],
			fRec117: [0.0;2],
			fRec118: [0.0;2],
			fConst76: 0.0,
			fRec125: [0.0;2],
			fRec126: [0.0;2],
			fRec123: [0.0;2],
			fRec124: [0.0;2],
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
		self.fHslider1 = (1.0) as FaustFloat;
		self.fHslider2 = (8.0) as FaustFloat;
		self.fHslider3 = (6e+03) as FaustFloat;
		self.fHslider4 = (9e+02) as FaustFloat;
		self.fHslider5 = (0.3) as FaustFloat;
		self.fHslider6 = (0.35) as FaustFloat;
		self.fHslider7 = (0.0) as FaustFloat;
		self.fHslider8 = (0.0) as FaustFloat;
		self.fHslider9 = (0.0) as FaustFloat;
		self.fHslider10 = (0.0) as FaustFloat;
		self.fHslider11 = (0.3) as FaustFloat;
		self.fHslider12 = (0.7) as FaustFloat;
		self.fHslider13 = (7.0) as FaustFloat;
		self.fHslider14 = (0.3) as FaustFloat;
		self.fHslider15 = (2.2e+02) as FaustFloat;
		self.fHslider16 = (7.0) as FaustFloat;
		self.fHslider17 = (0.3) as FaustFloat;
		self.fHslider18 = (164.81) as FaustFloat;
		self.fHslider19 = (7.0) as FaustFloat;
		self.fHslider20 = (0.3) as FaustFloat;
		self.fHslider21 = (130.81) as FaustFloat;
		self.fHslider22 = (7.0) as FaustFloat;
		self.fHslider23 = (0.3) as FaustFloat;
		self.fHslider24 = (1.1e+02) as FaustFloat;
		self.fHslider25 = (7.0) as FaustFloat;
		self.fHslider26 = (0.3) as FaustFloat;
		self.fHslider27 = (82.41) as FaustFloat;
		self.fHslider28 = (0.5) as FaustFloat;
		self.fHslider29 = (0.3) as FaustFloat;
		self.fHslider30 = (55.0) as FaustFloat;
		self.fHslider31 = (7.0) as FaustFloat;
		self.fHslider32 = (0.0) as FaustFloat;
		self.fHslider33 = (1.0) as FaustFloat;
		self.fHslider34 = (0.0) as FaustFloat;
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
			self.fRec48[l14 as usize] = 0.0;
		}
		for l15 in 0..2 {
			self.fRec49[l15 as usize] = 0.0;
		}
		for l16 in 0..2 {
			self.fRec50[l16 as usize] = 0.0;
		}
		for l17 in 0..2 {
			self.fRec55[l17 as usize] = 0.0;
		}
		for l18 in 0..2 {
			self.fRec56[l18 as usize] = 0.0;
		}
		for l19 in 0..2 {
			self.fRec57[l19 as usize] = 0.0;
		}
		for l20 in 0..2 {
			self.fRec58[l20 as usize] = 0.0;
		}
		for l21 in 0..2 {
			self.fRec59[l21 as usize] = 0.0;
		}
		for l22 in 0..2 {
			self.fRec60[l22 as usize] = 0.0;
		}
		for l23 in 0..2 {
			self.fRec61[l23 as usize] = 0.0;
		}
		for l24 in 0..2 {
			self.iRec63[l24 as usize] = 0;
		}
		for l25 in 0..2 {
			self.fVec1[l25 as usize] = 0.0;
		}
		for l26 in 0..2 {
			self.fRec62[l26 as usize] = 0.0;
		}
		for l27 in 0..2 {
			self.fRec64[l27 as usize] = 0.0;
		}
		for l28 in 0..2 {
			self.fRec65[l28 as usize] = 0.0;
		}
		for l29 in 0..2 {
			self.fRec66[l29 as usize] = 0.0;
		}
		for l30 in 0..2 {
			self.fVec2[l30 as usize] = 0.0;
		}
		for l31 in 0..2 {
			self.fRec67[l31 as usize] = 0.0;
		}
		for l32 in 0..2 {
			self.fRec68[l32 as usize] = 0.0;
		}
		for l33 in 0..2 {
			self.fRec69[l33 as usize] = 0.0;
		}
		for l34 in 0..2 {
			self.fVec3[l34 as usize] = 0.0;
		}
		for l35 in 0..2 {
			self.fRec70[l35 as usize] = 0.0;
		}
		for l36 in 0..2 {
			self.fRec71[l36 as usize] = 0.0;
		}
		for l37 in 0..2 {
			self.fRec72[l37 as usize] = 0.0;
		}
		for l38 in 0..2 {
			self.fRec73[l38 as usize] = 0.0;
		}
		for l39 in 0..2 {
			self.fVec4[l39 as usize] = 0.0;
		}
		for l40 in 0..2 {
			self.fRec74[l40 as usize] = 0.0;
		}
		for l41 in 0..2 {
			self.fRec75[l41 as usize] = 0.0;
		}
		for l42 in 0..2 {
			self.fRec76[l42 as usize] = 0.0;
		}
		for l43 in 0..2 {
			self.fVec5[l43 as usize] = 0.0;
		}
		for l44 in 0..2 {
			self.fRec77[l44 as usize] = 0.0;
		}
		for l45 in 0..2 {
			self.fRec78[l45 as usize] = 0.0;
		}
		for l46 in 0..2 {
			self.fRec79[l46 as usize] = 0.0;
		}
		for l47 in 0..2 {
			self.fRec80[l47 as usize] = 0.0;
		}
		for l48 in 0..2 {
			self.fVec6[l48 as usize] = 0.0;
		}
		for l49 in 0..2 {
			self.fRec81[l49 as usize] = 0.0;
		}
		for l50 in 0..2 {
			self.fRec82[l50 as usize] = 0.0;
		}
		for l51 in 0..2 {
			self.fRec83[l51 as usize] = 0.0;
		}
		for l52 in 0..2 {
			self.fVec7[l52 as usize] = 0.0;
		}
		for l53 in 0..2 {
			self.fRec84[l53 as usize] = 0.0;
		}
		for l54 in 0..2 {
			self.fRec85[l54 as usize] = 0.0;
		}
		for l55 in 0..2 {
			self.fRec86[l55 as usize] = 0.0;
		}
		for l56 in 0..2 {
			self.fRec87[l56 as usize] = 0.0;
		}
		for l57 in 0..2 {
			self.fVec8[l57 as usize] = 0.0;
		}
		for l58 in 0..2 {
			self.fRec88[l58 as usize] = 0.0;
		}
		for l59 in 0..2 {
			self.fRec89[l59 as usize] = 0.0;
		}
		for l60 in 0..2 {
			self.fRec90[l60 as usize] = 0.0;
		}
		for l61 in 0..2 {
			self.fVec9[l61 as usize] = 0.0;
		}
		for l62 in 0..2 {
			self.fRec91[l62 as usize] = 0.0;
		}
		for l63 in 0..2 {
			self.fRec92[l63 as usize] = 0.0;
		}
		for l64 in 0..2 {
			self.fRec93[l64 as usize] = 0.0;
		}
		for l65 in 0..2 {
			self.fRec94[l65 as usize] = 0.0;
		}
		for l66 in 0..2 {
			self.fVec10[l66 as usize] = 0.0;
		}
		for l67 in 0..2 {
			self.fRec95[l67 as usize] = 0.0;
		}
		for l68 in 0..2 {
			self.fRec96[l68 as usize] = 0.0;
		}
		for l69 in 0..2 {
			self.fRec97[l69 as usize] = 0.0;
		}
		for l70 in 0..2 {
			self.fVec11[l70 as usize] = 0.0;
		}
		for l71 in 0..2 {
			self.fRec100[l71 as usize] = 0.0;
		}
		for l72 in 0..2 {
			self.fRec101[l72 as usize] = 0.0;
		}
		for l73 in 0..2 {
			self.fRec102[l73 as usize] = 0.0;
		}
		for l74 in 0..2 {
			self.fRec98[l74 as usize] = 0.0;
		}
		for l75 in 0..2 {
			self.fRec99[l75 as usize] = 0.0;
		}
		for l76 in 0..2 {
			self.fRec103[l76 as usize] = 0.0;
		}
		for l77 in 0..2 {
			self.fVec12[l77 as usize] = 0.0;
		}
		for l78 in 0..2 {
			self.fRec104[l78 as usize] = 0.0;
		}
		for l79 in 0..2 {
			self.fRec105[l79 as usize] = 0.0;
		}
		for l80 in 0..2 {
			self.fVec13[l80 as usize] = 0.0;
		}
		for l81 in 0..2 {
			self.fRec106[l81 as usize] = 0.0;
		}
		for l82 in 0..2 {
			self.fRec107[l82 as usize] = 0.0;
		}
		for l83 in 0..2 {
			self.fRec53[l83 as usize] = 0.0;
		}
		for l84 in 0..2 {
			self.fRec54[l84 as usize] = 0.0;
		}
		for l85 in 0..2 {
			self.fRec51[l85 as usize] = 0.0;
		}
		for l86 in 0..2 {
			self.fRec52[l86 as usize] = 0.0;
		}
		self.IOTA0 = 0;
		for l87 in 0..16384 {
			self.fVec14[l87 as usize] = 0.0;
		}
		for l88 in 0..2 {
			self.fRec108[l88 as usize] = 0.0;
		}
		for l89 in 0..16384 {
			self.fVec15[l89 as usize] = 0.0;
		}
		for l90 in 0..4096 {
			self.fVec16[l90 as usize] = 0.0;
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
		for l94 in 0..2 {
			self.fRec19[l94 as usize] = 0.0;
		}
		for l95 in 0..2 {
			self.fRec20[l95 as usize] = 0.0;
		}
		for l96 in 0..2 {
			self.fRec111[l96 as usize] = 0.0;
		}
		for l97 in 0..2 {
			self.fRec112[l97 as usize] = 0.0;
		}
		for l98 in 0..2 {
			self.fRec109[l98 as usize] = 0.0;
		}
		for l99 in 0..2 {
			self.fRec110[l99 as usize] = 0.0;
		}
		for l100 in 0..16384 {
			self.fVec17[l100 as usize] = 0.0;
		}
		for l101 in 0..32768 {
			self.fVec18[l101 as usize] = 0.0;
		}
		for l102 in 0..4096 {
			self.fVec19[l102 as usize] = 0.0;
		}
		for l103 in 0..2 {
			self.fRec21[l103 as usize] = 0.0;
		}
		for l104 in 0..2 {
			self.fRec22[l104 as usize] = 0.0;
		}
		for l105 in 0..16384 {
			self.fVec20[l105 as usize] = 0.0;
		}
		for l106 in 0..4096 {
			self.fVec21[l106 as usize] = 0.0;
		}
		for l107 in 0..2 {
			self.fRec23[l107 as usize] = 0.0;
		}
		for l108 in 0..16384 {
			self.fVec22[l108 as usize] = 0.0;
		}
		for l109 in 0..2048 {
			self.fVec23[l109 as usize] = 0.0;
		}
		for l110 in 0..2 {
			self.fRec25[l110 as usize] = 0.0;
		}
		for l111 in 0..32768 {
			self.fVec24[l111 as usize] = 0.0;
		}
		for l112 in 0..4096 {
			self.fVec25[l112 as usize] = 0.0;
		}
		for l113 in 0..2 {
			self.fRec27[l113 as usize] = 0.0;
		}
		for l114 in 0..16384 {
			self.fVec26[l114 as usize] = 0.0;
		}
		for l115 in 0..2048 {
			self.fVec27[l115 as usize] = 0.0;
		}
		for l116 in 0..32768 {
			self.fVec28[l116 as usize] = 0.0;
		}
		for l117 in 0..2048 {
			self.fVec29[l117 as usize] = 0.0;
		}
		for l118 in 0..2 {
			self.fRec31[l118 as usize] = 0.0;
		}
		for l119 in 0..2 {
			self.fRec32[l119 as usize] = 0.0;
		}
		for l120 in 0..16384 {
			self.fVec30[l120 as usize] = 0.0;
		}
		for l121 in 0..4096 {
			self.fVec31[l121 as usize] = 0.0;
		}
		for l122 in 0..2 {
			self.fRec34[l122 as usize] = 0.0;
		}
		for l123 in 0..2 {
			self.fRec36[l123 as usize] = 0.0;
		}
		for l124 in 0..2 {
			self.fRec37[l124 as usize] = 0.0;
		}
		for l125 in 0..3 {
			self.fRec38[l125 as usize] = 0.0;
		}
		for l126 in 0..3 {
			self.fRec39[l126 as usize] = 0.0;
		}
		for l127 in 0..3 {
			self.fRec40[l127 as usize] = 0.0;
		}
		for l128 in 0..3 {
			self.fRec41[l128 as usize] = 0.0;
		}
		for l129 in 0..3 {
			self.fRec42[l129 as usize] = 0.0;
		}
		for l130 in 0..3 {
			self.fRec43[l130 as usize] = 0.0;
		}
		for l131 in 0..3 {
			self.fRec44[l131 as usize] = 0.0;
		}
		for l132 in 0..3 {
			self.fRec45[l132 as usize] = 0.0;
		}
		for l133 in 0..2 {
			self.fRec113[l133 as usize] = 0.0;
		}
		for l134 in 0..2 {
			self.fRec115[l134 as usize] = 0.0;
		}
		for l135 in 0..2 {
			self.fRec116[l135 as usize] = 0.0;
		}
		for l136 in 0..2 {
			self.fRec114[l136 as usize] = 0.0;
		}
		for l137 in 0..2 {
			self.fRec3[l137 as usize] = 0.0;
		}
		for l138 in 0..2 {
			self.fRec4[l138 as usize] = 0.0;
		}
		for l139 in 0..2 {
			self.fRec1[l139 as usize] = 0.0;
		}
		for l140 in 0..2 {
			self.fRec2[l140 as usize] = 0.0;
		}
		for l141 in 0..2 {
			self.fRec122[l141 as usize] = 0.0;
		}
		for l142 in 0..2 {
			self.fRec121[l142 as usize] = 0.0;
		}
		for l143 in 0..2 {
			self.fRec119[l143 as usize] = 0.0;
		}
		for l144 in 0..2 {
			self.fRec120[l144 as usize] = 0.0;
		}
		for l145 in 0..2 {
			self.fRec117[l145 as usize] = 0.0;
		}
		for l146 in 0..2 {
			self.fRec118[l146 as usize] = 0.0;
		}
		for l147 in 0..2 {
			self.fRec125[l147 as usize] = 0.0;
		}
		for l148 in 0..2 {
			self.fRec126[l148 as usize] = 0.0;
		}
		for l149 in 0..2 {
			self.fRec123[l149 as usize] = 0.0;
		}
		for l150 in 0..2 {
			self.fRec124[l150 as usize] = 0.0;
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
		self.fConst6 = F32::floor(0.127837 * self.fConst0 + 0.5);
		self.fConst7 = self.fConst6 / self.fConst0;
		self.fConst8 = 6.9077554 * self.fConst7;
		self.fConst9 = 6.2831855 / self.fConst0;
		self.fConst10 = 9.2103405 * self.fConst7;
		self.fConst11 = 3.1415927 / self.fConst0;
		self.fConst12 = 1.0 / F32::tan(0.4712389 / self.fConst0);
		self.fConst13 = 1.0 - self.fConst12;
		self.fConst14 = 1.0 / (self.fConst12 + 1.0);
		self.fConst15 = 1.0 / self.fConst0;
		self.fConst16 = 0.001 * self.fConst0;
		self.fConst17 = F32::floor(0.031604 * self.fConst0 + 0.5);
		self.iConst18 = (F32::min(8192.0, F32::max(0.0, self.fConst6 - self.fConst17))) as i32;
		self.fConst19 = F32::floor(0.192303 * self.fConst0 + 0.5);
		self.fConst20 = self.fConst19 / self.fConst0;
		self.fConst21 = 6.9077554 * self.fConst20;
		self.fConst22 = 9.2103405 * self.fConst20;
		self.fConst23 = F32::floor(0.256891 * self.fConst0 + 0.5);
		self.fConst24 = self.fConst23 / self.fConst0;
		self.fConst25 = 6.9077554 * self.fConst24;
		self.fConst26 = 9.2103405 * self.fConst24;
		self.fConst27 = F32::floor(0.174713 * self.fConst0 + 0.5);
		self.fConst28 = self.fConst27 / self.fConst0;
		self.fConst29 = 6.9077554 * self.fConst28;
		self.fConst30 = 9.2103405 * self.fConst28;
		self.fConst31 = F32::floor(0.153129 * self.fConst0 + 0.5);
		self.fConst32 = self.fConst31 / self.fConst0;
		self.fConst33 = 6.9077554 * self.fConst32;
		self.fConst34 = 9.2103405 * self.fConst32;
		self.fConst35 = F32::floor(0.219991 * self.fConst0 + 0.5);
		self.fConst36 = self.fConst35 / self.fConst0;
		self.fConst37 = 6.9077554 * self.fConst36;
		self.fConst38 = 9.2103405 * self.fConst36;
		self.fConst39 = F32::floor(0.027333 * self.fConst0 + 0.5);
		self.iConst40 = (F32::min(16384.0, F32::max(0.0, self.fConst23 - self.fConst39))) as i32;
		self.iConst41 = (F32::min(2048.0, F32::max(0.0, self.fConst39 + -1.0))) as i32;
		self.fConst42 = F32::floor(0.210389 * self.fConst0 + 0.5);
		self.fConst43 = self.fConst42 / self.fConst0;
		self.fConst44 = 6.9077554 * self.fConst43;
		self.fConst45 = 9.2103405 * self.fConst43;
		self.fConst46 = F32::floor(0.029291 * self.fConst0 + 0.5);
		self.iConst47 = (F32::min(8192.0, F32::max(0.0, self.fConst19 - self.fConst46))) as i32;
		self.iConst48 = (F32::min(2048.0, F32::max(0.0, self.fConst46 + -1.0))) as i32;
		self.fConst49 = F32::floor(0.020346 * self.fConst0 + 0.5);
		self.iConst50 = (F32::min(8192.0, F32::max(0.0, self.fConst31 - self.fConst49))) as i32;
		self.fConst51 = F32::floor(0.125 * self.fConst0 + 0.5);
		self.fConst52 = self.fConst51 / self.fConst0;
		self.fConst53 = 6.9077554 * self.fConst52;
		self.fConst54 = 9.2103405 * self.fConst52;
		self.fConst55 = F32::floor(0.024421 * self.fConst0 + 0.5);
		self.iConst56 = (F32::min(16384.0, F32::max(0.0, self.fConst42 - self.fConst55))) as i32;
		self.iConst57 = (F32::min(2048.0, F32::max(0.0, self.fConst17 + -1.0))) as i32;
		self.fConst58 = F32::floor(0.013458 * self.fConst0 + 0.5);
		self.iConst59 = (F32::min(8192.0, F32::max(0.0, self.fConst51 - self.fConst58))) as i32;
		self.fConst60 = F32::floor(0.019123 * self.fConst0 + 0.5);
		self.iConst61 = (F32::min(16384.0, F32::max(0.0, self.fConst35 - self.fConst60))) as i32;
		self.iConst62 = (F32::min(1024.0, F32::max(0.0, self.fConst60 + -1.0))) as i32;
		self.iConst63 = (F32::min(1024.0, F32::max(0.0, self.fConst58 + -1.0))) as i32;
		self.fConst64 = F32::floor(0.022904 * self.fConst0 + 0.5);
		self.iConst65 = (F32::min(8192.0, F32::max(0.0, self.fConst27 - self.fConst64))) as i32;
		self.iConst66 = (F32::min(2048.0, F32::max(0.0, self.fConst64 + -1.0))) as i32;
		self.iConst67 = (F32::min(1024.0, F32::max(0.0, self.fConst49 + -1.0))) as i32;
		self.iConst68 = (F32::min(2048.0, F32::max(0.0, self.fConst55 + -1.0))) as i32;
		self.fConst69 = F32::exp(-(18849.557 / self.fConst0));
		self.fConst70 = F32::tan(471.2389 / self.fConst0);
		self.fConst71 = self.fConst70 * (self.fConst70 + 1.4142135) + 1.0;
		self.fConst72 = 2.0 / self.fConst71;
		self.fConst73 = self.fConst70 / self.fConst71;
		self.fConst74 = -1.4142135 - self.fConst70;
		self.fConst75 = self.fConst74 / self.fConst71;
		self.fConst76 = 1.0 / self.fConst71;
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
		ui_interface.declare(Some(ParamIndex(31)), "unit", "ms");
		ui_interface.add_horizontal_slider("space/predelay", ParamIndex(31), 0.0, 0.0, 8e+01, 0.01);
		ui_interface.add_horizontal_slider("space/reverb_mix", ParamIndex(32), 0.35, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(33)), "unit", "s");
		ui_interface.add_horizontal_slider("space/size", ParamIndex(33), 8.0, 1.0, 2e+01, 0.01);
		ui_interface.add_horizontal_slider("space/wet", ParamIndex(34), 1.0, 0.0, 1.5, 0.001);
		ui_interface.close_box();
	}
	
	pub fn get_param(&self, param: ParamIndex) -> Option<FaustFloat> {
		match param.0 {
			27 => Some(self.fHslider0),
			34 => Some(self.fHslider1),
			11 => Some(self.fHslider10),
			7 => Some(self.fHslider11),
			2 => Some(self.fHslider12),
			21 => Some(self.fHslider13),
			22 => Some(self.fHslider14),
			24 => Some(self.fHslider15),
			17 => Some(self.fHslider16),
			18 => Some(self.fHslider17),
			20 => Some(self.fHslider18),
			13 => Some(self.fHslider19),
			33 => Some(self.fHslider2),
			14 => Some(self.fHslider20),
			16 => Some(self.fHslider21),
			9 => Some(self.fHslider22),
			10 => Some(self.fHslider23),
			12 => Some(self.fHslider24),
			5 => Some(self.fHslider25),
			6 => Some(self.fHslider26),
			8 => Some(self.fHslider27),
			4 => Some(self.fHslider28),
			1 => Some(self.fHslider29),
			29 => Some(self.fHslider3),
			3 => Some(self.fHslider30),
			0 => Some(self.fHslider31),
			31 => Some(self.fHslider32),
			30 => Some(self.fHslider33),
			28 => Some(self.fHslider34),
			25 => Some(self.fHslider4),
			26 => Some(self.fHslider5),
			32 => Some(self.fHslider6),
			23 => Some(self.fHslider7),
			19 => Some(self.fHslider8),
			15 => Some(self.fHslider9),
			_ => None,
		}
	}
	
	pub fn set_param(&mut self, param: ParamIndex, value: FaustFloat) {
		match param.0 {
			27 => { self.fHslider0 = value }
			34 => { self.fHslider1 = value }
			11 => { self.fHslider10 = value }
			7 => { self.fHslider11 = value }
			2 => { self.fHslider12 = value }
			21 => { self.fHslider13 = value }
			22 => { self.fHslider14 = value }
			24 => { self.fHslider15 = value }
			17 => { self.fHslider16 = value }
			18 => { self.fHslider17 = value }
			20 => { self.fHslider18 = value }
			13 => { self.fHslider19 = value }
			33 => { self.fHslider2 = value }
			14 => { self.fHslider20 = value }
			16 => { self.fHslider21 = value }
			9 => { self.fHslider22 = value }
			10 => { self.fHslider23 = value }
			12 => { self.fHslider24 = value }
			5 => { self.fHslider25 = value }
			6 => { self.fHslider26 = value }
			8 => { self.fHslider27 = value }
			4 => { self.fHslider28 = value }
			1 => { self.fHslider29 = value }
			29 => { self.fHslider3 = value }
			3 => { self.fHslider30 = value }
			0 => { self.fHslider31 = value }
			31 => { self.fHslider32 = value }
			30 => { self.fHslider33 = value }
			28 => { self.fHslider34 = value }
			25 => { self.fHslider4 = value }
			26 => { self.fHslider5 = value }
			32 => { self.fHslider6 = value }
			23 => { self.fHslider7 = value }
			19 => { self.fHslider8 = value }
			15 => { self.fHslider9 = value }
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
		let mut fSlow33: F32 = self.fConst1 * ((self.fHslider33) as F32 + -1.0);
		let mut fSlow34: F32 = self.fConst1 * (self.fHslider34) as F32;
		let zipped_iterators = inputs0.zip(inputs1).zip(inputs2).zip(inputs3).zip(outputs0).zip(outputs1);
		for (((((input0, input1), input2), input3), output0), output1) in zipped_iterators {
			self.iVec0[0] = 1;
			self.fRec0[0] = fSlow0 + self.fConst2 * self.fRec0[1];
			self.fRec5[0] = fSlow1 + self.fConst2 * self.fRec5[1];
			let mut fTemp0: F32 = 1.0 + self.fRec5[0];
			self.fRec6[0] = -(self.fConst5 * (self.fConst4 * self.fRec6[1] - (self.fRec39[1] + self.fRec39[2])));
			self.fRec7[0] = -(self.fConst5 * (self.fConst4 * self.fRec7[1] - (self.fRec41[1] + self.fRec41[2])));
			self.fRec8[0] = -(self.fConst5 * (self.fConst4 * self.fRec8[1] - (self.fRec38[1] + self.fRec38[2])));
			self.fRec9[0] = -(self.fConst5 * (self.fConst4 * self.fRec9[1] - (self.fRec45[1] + self.fRec45[2])));
			self.fRec10[0] = -(self.fConst5 * (self.fConst4 * self.fRec10[1] - (self.fRec44[1] + self.fRec44[2])));
			self.fRec11[0] = -(self.fConst5 * (self.fConst4 * self.fRec11[1] - (self.fRec40[1] + self.fRec40[2])));
			self.fRec12[0] = -(self.fConst5 * (self.fConst4 * self.fRec12[1] - (self.fRec42[1] + self.fRec42[2])));
			self.fRec13[0] = -(self.fConst5 * (self.fConst4 * self.fRec13[1] - (self.fRec43[1] + self.fRec43[2])));
			self.fRec46[0] = fSlow2 + self.fConst2 * self.fRec46[1];
			let mut fTemp1: F32 = 8.0 + self.fRec46[0];
			let mut fTemp2: F32 = F32::exp(-(self.fConst8 / fTemp1));
			let mut fTemp3: F32 = MoodscoreDsp_faustpower2_f(fTemp2);
			let mut fTemp4: F32 = 1.0 - fTemp3;
			self.fRec47[0] = fSlow3 + self.fConst2 * self.fRec47[1];
			let mut fTemp5: F32 = F32::cos(self.fConst9 * (6e+03 + self.fRec47[0]));
			let mut fTemp6: F32 = 1.0 - fTemp3 * fTemp5;
			let mut fTemp7: F32 = F32::sqrt(F32::max(0.0, -1.0 + MoodscoreDsp_faustpower2_f(fTemp6) / MoodscoreDsp_faustpower2_f(fTemp4)));
			let mut fTemp8: F32 = fTemp6 / fTemp4;
			self.fRec14[0] = fTemp2 * (1.0 - fTemp8 + fTemp7) * (self.fRec41[1] + (-1.0 + F32::exp(-(self.fConst10 / fTemp1)) / fTemp2) * self.fRec7[0]) + (fTemp8 - fTemp7) * self.fRec14[1];
			self.fRec48[0] = fSlow4 + self.fConst2 * self.fRec48[1];
			let mut fTemp9: F32 = F32::tan(self.fConst11 * F32::max(self.fRec48[0], 2e+01));
			self.fRec49[0] = fSlow5 + self.fConst2 * self.fRec49[1];
			let mut fTemp10: F32 = 1.99998 * self.fRec49[0];
			let mut fTemp11: F32 = 2.0 - fTemp10;
			let mut fTemp12: F32 = 1.0 / fTemp9;
			let mut fTemp13: F32 = MoodscoreDsp_faustpower2_f(1.4141995 * self.fRec49[0]);
			let mut fTemp14: F32 = 1.0 - fTemp10;
			let mut fTemp15: F32 = fTemp14 + fTemp13 + (fTemp12 + fTemp11) / fTemp9;
			self.fRec50[0] = fSlow6 + self.fConst2 * self.fRec50[1];
			let mut fTemp16: F32 = (fTemp14 + fTemp13 + (fTemp12 - fTemp11) / fTemp9) / fTemp15;
			let mut fTemp17: F32 = 1.0 / MoodscoreDsp_faustpower2_f(fTemp9);
			let mut fTemp18: F32 = 1.0 - (fTemp10 + fTemp17) + fTemp13;
			let mut fTemp19: F32 = F32::max(-0.9999999, F32::min(0.9999999, 2.0 * (fTemp18 / (fTemp15 * (1.0 + fTemp16)))));
			let mut fTemp20: F32 = 1.0 - MoodscoreDsp_faustpower2_f(fTemp19);
			let mut fTemp21: F32 = F32::max(-0.9999999, F32::min(0.9999999, fTemp16));
			let mut fTemp22: F32 = 1.0 - MoodscoreDsp_faustpower2_f(fTemp21);
			let mut fTemp23: F32 = F32::sqrt(fTemp22);
			let mut fTemp24: F32 = fTemp23 * F32::sqrt(fTemp20);
			let mut fTemp25: F32 = 1.0 - fTemp18 / fTemp15;
			let mut fTemp26: F32 = 1.0 - fTemp16 - 2.0 * fTemp19 * fTemp25;
			let mut fTemp27: F32 = F32::sqrt(F32::max(0.0, fTemp20));
			let mut fTemp28: F32 = 2.0 + fTemp10;
			let mut fTemp29: F32 = fTemp10 + fTemp13;
			let mut fTemp30: F32 = 1.0 + fTemp29 + (fTemp28 + fTemp12) / fTemp9;
			let mut fTemp31: F32 = F32::sqrt(F32::max(0.0, fTemp22));
			let mut fTemp32: F32 = (1.0 - (fTemp28 - fTemp12) / fTemp9 + fTemp29) / fTemp30;
			let mut fTemp33: F32 = 1.0 - fTemp17 + fTemp29;
			let mut fTemp34: F32 = F32::max(-0.9999999, F32::min(0.9999999, 2.0 * (fTemp33 / (fTemp30 * (1.0 + fTemp32)))));
			let mut fTemp35: F32 = 1.0 - MoodscoreDsp_faustpower2_f(fTemp34);
			let mut fTemp36: F32 = F32::max(-0.9999999, F32::min(0.9999999, fTemp32));
			let mut fTemp37: F32 = 1.0 - MoodscoreDsp_faustpower2_f(fTemp36);
			let mut fTemp38: F32 = F32::sqrt(fTemp37);
			let mut fTemp39: F32 = fTemp38 * F32::sqrt(fTemp35);
			let mut fTemp40: F32 = 1.0 - fTemp33 / fTemp30;
			let mut fTemp41: F32 = 1.0 - fTemp32 - 2.0 * fTemp34 * fTemp40;
			let mut fTemp42: F32 = F32::sqrt(F32::max(0.0, fTemp35));
			self.fRec55[0] = fSlow7 + self.fConst2 * self.fRec55[1];
			self.fRec56[0] = fSlow8 + self.fConst2 * self.fRec56[1];
			self.fRec57[0] = fSlow9 + self.fConst2 * self.fRec57[1];
			self.fRec58[0] = fSlow10 + self.fConst2 * self.fRec58[1];
			self.fRec59[0] = fSlow11 + self.fConst2 * self.fRec59[1];
			self.fRec60[0] = fSlow12 + self.fConst2 * self.fRec60[1];
			let mut fTemp43: F32 = F32::max(1.0, 0.6666667 * F32::sqrt(MoodscoreDsp_faustpower2_f(self.fRec60[0]) + MoodscoreDsp_faustpower2_f(self.fRec59[0]) + MoodscoreDsp_faustpower2_f(self.fRec58[0]) + MoodscoreDsp_faustpower2_f(self.fRec57[0]) + MoodscoreDsp_faustpower2_f(self.fRec56[0]) + MoodscoreDsp_faustpower2_f(self.fRec55[0])));
			let mut fTemp44: F32 = F32::sqrt(F32::max(0.0, fTemp37));
			self.fRec61[0] = fSlow13 + self.fConst2 * self.fRec61[1];
			let mut iTemp45: i32 = i32::wrapping_mul(1103515245, i32::wrapping_add(12345, i32::wrapping_mul(1103515245, i32::wrapping_add(12345, self.iRec63[1]))));
			let mut iTemp46: i32 = i32::wrapping_mul(1103515245, i32::wrapping_add(12345, iTemp45));
			let mut iTemp47: i32 = i32::wrapping_mul(1103515245, i32::wrapping_add(12345, i32::wrapping_mul(1103515245, i32::wrapping_add(12345, iTemp46))));
			let mut iTemp48: i32 = i32::wrapping_mul(1103515245, i32::wrapping_add(12345, iTemp47));
			let mut iTemp49: i32 = i32::wrapping_mul(1103515245, i32::wrapping_add(12345, i32::wrapping_mul(1103515245, i32::wrapping_add(12345, iTemp48))));
			let mut iTemp50: i32 = i32::wrapping_mul(1103515245, i32::wrapping_add(12345, iTemp49));
			let mut iTemp51: i32 = i32::wrapping_mul(1103515245, i32::wrapping_add(12345, i32::wrapping_mul(1103515245, i32::wrapping_add(12345, iTemp50))));
			let mut iTemp52: i32 = i32::wrapping_mul(1103515245, i32::wrapping_add(12345, iTemp51));
			let mut iTemp53: i32 = i32::wrapping_mul(1103515245, i32::wrapping_add(12345, i32::wrapping_mul(1103515245, i32::wrapping_add(12345, iTemp52))));
			let mut iTemp54: i32 = i32::wrapping_mul(1103515245, i32::wrapping_add(12345, iTemp53));
			let mut iTemp55: i32 = i32::wrapping_mul(1103515245, i32::wrapping_add(12345, iTemp54));
			let mut iTemp56: i32 = i32::wrapping_mul(1103515245, i32::wrapping_add(12345, iTemp55));
			self.iRec63[0] = i32::wrapping_mul(1103515245, i32::wrapping_add(12345, iTemp56));
			let mut fTemp57: F32 = (iTemp45) as F32;
			self.fVec1[0] = fTemp57;
			self.fRec62[0] = self.fConst14 * (4.656613e-10 * (fTemp57 + self.fVec1[1]) - self.fConst13 * self.fRec62[1]);
			self.fRec64[0] = fSlow14 + self.fConst2 * self.fRec64[1];
			self.fRec65[0] = fSlow15 + self.fConst2 * self.fRec65[1];
			let mut fTemp58: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec65[0] * F32::powf(2.0, 0.00083333335 * (4.5e+03 * self.fRec64[0] * self.fRec62[0] - self.fRec61[0]))));
			let mut fTemp59: F32 = self.fRec66[1] + self.fConst15 * fTemp58;
			let mut fTemp60: F32 = -1.0 + fTemp59;
			let mut iTemp61: i32 = (fTemp60 < 0.0) as i32;
			self.fRec66[0] = (if iTemp61 != 0 {fTemp59} else {fTemp60});
			let mut fTemp62: F32 = -1.0 + 2.0 * (if iTemp61 != 0 {fTemp59} else {fTemp59 + fTemp60 * (1.0 - self.fConst0 / fTemp58)});
			let mut fTemp63: F32 = (iTemp46) as F32;
			self.fVec2[0] = fTemp63;
			self.fRec67[0] = self.fConst14 * (4.656613e-10 * (fTemp63 + self.fVec2[1]) - self.fConst13 * self.fRec67[1]);
			let mut fTemp64: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec65[0] * F32::powf(2.0, 0.00083333335 * (4.5e+03 * self.fRec67[0] * self.fRec64[0] + self.fRec61[0]))));
			let mut fTemp65: F32 = self.fRec68[1] + self.fConst15 * fTemp64;
			let mut fTemp66: F32 = -1.0 + fTemp65;
			let mut iTemp67: i32 = (fTemp66 < 0.0) as i32;
			self.fRec68[0] = (if iTemp67 != 0 {fTemp65} else {fTemp66});
			let mut fTemp68: F32 = -1.0 + 2.0 * (if iTemp67 != 0 {fTemp65} else {fTemp65 + fTemp66 * (1.0 - self.fConst0 / fTemp64)});
			self.fRec69[0] = fSlow16 + self.fConst2 * self.fRec69[1];
			let mut fTemp69: F32 = (iTemp47) as F32;
			self.fVec3[0] = fTemp69;
			self.fRec70[0] = self.fConst14 * (4.656613e-10 * (fTemp69 + self.fVec3[1]) - self.fConst13 * self.fRec70[1]);
			self.fRec71[0] = fSlow17 + self.fConst2 * self.fRec71[1];
			self.fRec72[0] = fSlow18 + self.fConst2 * self.fRec72[1];
			let mut fTemp70: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec72[0] * F32::powf(2.0, 0.00083333335 * (4.5e+03 * self.fRec71[0] * self.fRec70[0] - self.fRec69[0]))));
			let mut fTemp71: F32 = self.fRec73[1] + self.fConst15 * fTemp70;
			let mut fTemp72: F32 = -1.0 + fTemp71;
			let mut iTemp73: i32 = (fTemp72 < 0.0) as i32;
			self.fRec73[0] = (if iTemp73 != 0 {fTemp71} else {fTemp72});
			let mut fTemp74: F32 = -1.0 + 2.0 * (if iTemp73 != 0 {fTemp71} else {fTemp71 + fTemp72 * (1.0 - self.fConst0 / fTemp70)});
			let mut fTemp75: F32 = (iTemp48) as F32;
			self.fVec4[0] = fTemp75;
			self.fRec74[0] = self.fConst14 * (4.656613e-10 * (fTemp75 + self.fVec4[1]) - self.fConst13 * self.fRec74[1]);
			let mut fTemp76: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec72[0] * F32::powf(2.0, 0.00083333335 * (4.5e+03 * self.fRec74[0] * self.fRec71[0] + self.fRec69[0]))));
			let mut fTemp77: F32 = self.fRec75[1] + self.fConst15 * fTemp76;
			let mut fTemp78: F32 = -1.0 + fTemp77;
			let mut iTemp79: i32 = (fTemp78 < 0.0) as i32;
			self.fRec75[0] = (if iTemp79 != 0 {fTemp77} else {fTemp78});
			let mut fTemp80: F32 = -1.0 + 2.0 * (if iTemp79 != 0 {fTemp77} else {fTemp77 + fTemp78 * (1.0 - self.fConst0 / fTemp76)});
			self.fRec76[0] = fSlow19 + self.fConst2 * self.fRec76[1];
			let mut fTemp81: F32 = (iTemp49) as F32;
			self.fVec5[0] = fTemp81;
			self.fRec77[0] = self.fConst14 * (4.656613e-10 * (fTemp81 + self.fVec5[1]) - self.fConst13 * self.fRec77[1]);
			self.fRec78[0] = fSlow20 + self.fConst2 * self.fRec78[1];
			self.fRec79[0] = fSlow21 + self.fConst2 * self.fRec79[1];
			let mut fTemp82: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec79[0] * F32::powf(2.0, 0.00083333335 * (4.5e+03 * self.fRec78[0] * self.fRec77[0] - self.fRec76[0]))));
			let mut fTemp83: F32 = self.fRec80[1] + self.fConst15 * fTemp82;
			let mut fTemp84: F32 = -1.0 + fTemp83;
			let mut iTemp85: i32 = (fTemp84 < 0.0) as i32;
			self.fRec80[0] = (if iTemp85 != 0 {fTemp83} else {fTemp84});
			let mut fTemp86: F32 = -1.0 + 2.0 * (if iTemp85 != 0 {fTemp83} else {fTemp83 + fTemp84 * (1.0 - self.fConst0 / fTemp82)});
			let mut fTemp87: F32 = (iTemp50) as F32;
			self.fVec6[0] = fTemp87;
			self.fRec81[0] = self.fConst14 * (4.656613e-10 * (fTemp87 + self.fVec6[1]) - self.fConst13 * self.fRec81[1]);
			let mut fTemp88: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec79[0] * F32::powf(2.0, 0.00083333335 * (4.5e+03 * self.fRec81[0] * self.fRec78[0] + self.fRec76[0]))));
			let mut fTemp89: F32 = self.fRec82[1] + self.fConst15 * fTemp88;
			let mut fTemp90: F32 = -1.0 + fTemp89;
			let mut iTemp91: i32 = (fTemp90 < 0.0) as i32;
			self.fRec82[0] = (if iTemp91 != 0 {fTemp89} else {fTemp90});
			let mut fTemp92: F32 = -1.0 + 2.0 * (if iTemp91 != 0 {fTemp89} else {fTemp89 + fTemp90 * (1.0 - self.fConst0 / fTemp88)});
			self.fRec83[0] = fSlow22 + self.fConst2 * self.fRec83[1];
			let mut fTemp93: F32 = (iTemp51) as F32;
			self.fVec7[0] = fTemp93;
			self.fRec84[0] = self.fConst14 * (4.656613e-10 * (fTemp93 + self.fVec7[1]) - self.fConst13 * self.fRec84[1]);
			self.fRec85[0] = fSlow23 + self.fConst2 * self.fRec85[1];
			self.fRec86[0] = fSlow24 + self.fConst2 * self.fRec86[1];
			let mut fTemp94: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec86[0] * F32::powf(2.0, 0.00083333335 * (4.5e+03 * self.fRec85[0] * self.fRec84[0] - self.fRec83[0]))));
			let mut fTemp95: F32 = self.fRec87[1] + self.fConst15 * fTemp94;
			let mut fTemp96: F32 = -1.0 + fTemp95;
			let mut iTemp97: i32 = (fTemp96 < 0.0) as i32;
			self.fRec87[0] = (if iTemp97 != 0 {fTemp95} else {fTemp96});
			let mut fTemp98: F32 = -1.0 + 2.0 * (if iTemp97 != 0 {fTemp95} else {fTemp95 + fTemp96 * (1.0 - self.fConst0 / fTemp94)});
			let mut fTemp99: F32 = (iTemp52) as F32;
			self.fVec8[0] = fTemp99;
			self.fRec88[0] = self.fConst14 * (4.656613e-10 * (fTemp99 + self.fVec8[1]) - self.fConst13 * self.fRec88[1]);
			let mut fTemp100: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec86[0] * F32::powf(2.0, 0.00083333335 * (4.5e+03 * self.fRec88[0] * self.fRec85[0] + self.fRec83[0]))));
			let mut fTemp101: F32 = self.fRec89[1] + self.fConst15 * fTemp100;
			let mut fTemp102: F32 = -1.0 + fTemp101;
			let mut iTemp103: i32 = (fTemp102 < 0.0) as i32;
			self.fRec89[0] = (if iTemp103 != 0 {fTemp101} else {fTemp102});
			let mut fTemp104: F32 = -1.0 + 2.0 * (if iTemp103 != 0 {fTemp101} else {fTemp101 + fTemp102 * (1.0 - self.fConst0 / fTemp100)});
			self.fRec90[0] = fSlow25 + self.fConst2 * self.fRec90[1];
			let mut fTemp105: F32 = (iTemp53) as F32;
			self.fVec9[0] = fTemp105;
			self.fRec91[0] = self.fConst14 * (4.656613e-10 * (fTemp105 + self.fVec9[1]) - self.fConst13 * self.fRec91[1]);
			self.fRec92[0] = fSlow26 + self.fConst2 * self.fRec92[1];
			self.fRec93[0] = fSlow27 + self.fConst2 * self.fRec93[1];
			let mut fTemp106: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec93[0] * F32::powf(2.0, 0.00083333335 * (4.5e+03 * self.fRec92[0] * self.fRec91[0] - self.fRec90[0]))));
			let mut fTemp107: F32 = self.fRec94[1] + self.fConst15 * fTemp106;
			let mut fTemp108: F32 = -1.0 + fTemp107;
			let mut iTemp109: i32 = (fTemp108 < 0.0) as i32;
			self.fRec94[0] = (if iTemp109 != 0 {fTemp107} else {fTemp108});
			let mut fTemp110: F32 = -1.0 + 2.0 * (if iTemp109 != 0 {fTemp107} else {fTemp107 + fTemp108 * (1.0 - self.fConst0 / fTemp106)});
			let mut fTemp111: F32 = (iTemp54) as F32;
			self.fVec10[0] = fTemp111;
			self.fRec95[0] = self.fConst14 * (4.656613e-10 * (fTemp111 + self.fVec10[1]) - self.fConst13 * self.fRec95[1]);
			let mut fTemp112: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec93[0] * F32::powf(2.0, 0.00083333335 * (4.5e+03 * self.fRec95[0] * self.fRec92[0] + self.fRec90[0]))));
			let mut fTemp113: F32 = self.fRec96[1] + self.fConst15 * fTemp112;
			let mut fTemp114: F32 = -1.0 + fTemp113;
			let mut iTemp115: i32 = (fTemp114 < 0.0) as i32;
			self.fRec96[0] = (if iTemp115 != 0 {fTemp113} else {fTemp114});
			let mut fTemp116: F32 = -1.0 + 2.0 * (if iTemp115 != 0 {fTemp113} else {fTemp113 + fTemp114 * (1.0 - self.fConst0 / fTemp112)});
			self.fRec97[0] = fSlow28 + self.fConst2 * self.fRec97[1];
			let mut fTemp117: F32 = (iTemp55) as F32;
			self.fVec11[0] = fTemp117;
			self.fRec100[0] = self.fConst14 * (4.656613e-10 * (fTemp117 + self.fVec11[1]) - self.fConst13 * self.fRec100[1]);
			self.fRec101[0] = fSlow29 + self.fConst2 * self.fRec101[1];
			let mut fTemp118: F32 = F32::powf(2.0, 3.75 * self.fRec101[0] * self.fRec100[0]);
			self.fRec102[0] = fSlow30 + self.fConst2 * self.fRec102[1];
			self.fRec98[0] = self.fRec98[1] - self.fConst11 * self.fRec102[0] * self.fRec99[1] * fTemp118;
			self.fRec99[0] = (i32::wrapping_sub(1, self.iVec0[1])) as F32 + self.fRec99[1] + self.fConst11 * self.fRec102[0] * fTemp118 * self.fRec98[0];
			let mut fTemp119: F32 = self.fRec99[0] * self.fRec97[0];
			self.fRec103[0] = fSlow31 + self.fConst2 * self.fRec103[1];
			let mut fTemp120: F32 = (iTemp56) as F32;
			self.fVec12[0] = fTemp120;
			self.fRec104[0] = self.fConst14 * (4.656613e-10 * (fTemp120 + self.fVec12[1]) - self.fConst13 * self.fRec104[1]);
			let mut fTemp121: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec102[0] * F32::powf(2.0, 0.00083333335 * (4.5e+03 * self.fRec101[0] * self.fRec104[0] - self.fRec103[0]))));
			let mut fTemp122: F32 = self.fRec105[1] + self.fConst15 * fTemp121;
			let mut fTemp123: F32 = -1.0 + fTemp122;
			let mut iTemp124: i32 = (fTemp123 < 0.0) as i32;
			self.fRec105[0] = (if iTemp124 != 0 {fTemp122} else {fTemp123});
			let mut fTemp125: F32 = -1.0 + 2.0 * (if iTemp124 != 0 {fTemp122} else {fTemp122 + fTemp123 * (1.0 - self.fConst0 / fTemp121)});
			let mut fTemp126: F32 = (self.iRec63[0]) as F32;
			self.fVec13[0] = fTemp126;
			self.fRec106[0] = self.fConst14 * (4.656613e-10 * (fTemp126 + self.fVec13[1]) - self.fConst13 * self.fRec106[1]);
			let mut fTemp127: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec102[0] * F32::powf(2.0, 0.00083333335 * (4.5e+03 * self.fRec106[0] * self.fRec101[0] + self.fRec103[0]))));
			let mut fTemp128: F32 = self.fRec107[1] + self.fConst15 * fTemp127;
			let mut fTemp129: F32 = -1.0 + fTemp128;
			let mut iTemp130: i32 = (fTemp129 < 0.0) as i32;
			self.fRec107[0] = (if iTemp130 != 0 {fTemp128} else {fTemp129});
			let mut fTemp131: F32 = -1.0 + 2.0 * (if iTemp130 != 0 {fTemp128} else {fTemp128 + fTemp129 * (1.0 - self.fConst0 / fTemp127)});
			let mut fTemp132: F32 = self.fRec60[0] * (0.5879378 * fTemp131 + 0.39284748 * fTemp125 + 1.0 * fTemp119) + self.fRec59[0] * (0.5879378 * fTemp116 + 0.39284748 * fTemp110) + self.fRec58[0] * (0.6805586 * fTemp104 + 0.5553029 * fTemp98) + self.fRec57[0] * (0.43776554 * fTemp92 + 0.19193739 * fTemp86) + self.fRec56[0] * (0.704927 * fTemp80 + 0.6300368 * fTemp74) + self.fRec55[0] * (0.32101977 * fTemp68 + 0.05547896 * fTemp62);
			let mut fTemp133: F32 = fTemp132 * fTemp44 / fTemp43 - fTemp36 * self.fRec54[1];
			self.fRec53[0] = fTemp133 * fTemp42 - self.fRec53[1] * fTemp34;
			self.fRec54[0] = fTemp34 * fTemp133 + self.fRec53[1] * fTemp42;
			let mut fTemp134: F32 = fTemp132 * fTemp36 / fTemp43 + self.fRec54[1] * fTemp44 + 2.0 * (self.fRec54[0] * fTemp40 / fTemp38) + self.fRec53[0] * fTemp41 / fTemp39;
			let mut fTemp135: F32 = fTemp134 * fTemp31 / fTemp30 - fTemp21 * self.fRec52[1];
			self.fRec51[0] = fTemp135 * fTemp27 - self.fRec51[1] * fTemp19;
			self.fRec52[0] = fTemp19 * fTemp135 + self.fRec51[1] * fTemp27;
			let mut fTemp136: F32 = fTemp134 * fTemp21 / fTemp30 + self.fRec52[1] * fTemp31 + 2.0 * (self.fRec52[0] * fTemp25 / fTemp23) + self.fRec51[0] * fTemp26 / fTemp24;
			let mut fTemp137: F32 = (*input2) as F32 + 0.9 * (fTemp136 * self.fRec50[0] / fTemp15);
			self.fVec14[(self.IOTA0 & 16383) as usize] = fTemp137;
			self.fRec108[0] = fSlow32 + self.fConst2 * self.fRec108[1];
			let mut fTemp138: F32 = self.fConst16 * self.fRec108[0];
			let mut iTemp139: i32 = (fTemp138) as i32;
			let mut iTemp140: i32 = core::cmp::min(8193, core::cmp::max(0, i32::wrapping_add(1, iTemp139)));
			let mut fTemp141: F32 = F32::floor(fTemp138);
			let mut fTemp142: F32 = fTemp138 - fTemp141;
			let mut fTemp143: F32 = 1.0 - fTemp138 + fTemp141;
			let mut iTemp144: i32 = core::cmp::min(8193, core::cmp::max(0, iTemp139));
			let mut fTemp145: F32 = 0.3 * (self.fVec14[((i32::wrapping_sub(self.IOTA0, iTemp144)) & 16383) as usize] * fTemp143 + fTemp142 * self.fVec14[((i32::wrapping_sub(self.IOTA0, iTemp140)) & 16383) as usize]);
			self.fVec15[(self.IOTA0 & 16383) as usize] = 1e-20 + 0.35355338 * self.fRec14[0];
			let mut fTemp146: F32 = self.fVec15[((i32::wrapping_sub(self.IOTA0, self.iConst18)) & 16383) as usize] - (0.6 * self.fRec27[1] + fTemp145);
			self.fVec16[(self.IOTA0 & 4095) as usize] = fTemp146;
			let mut fRec15: F32 = 0.6 * fTemp146;
			let mut fTemp147: F32 = F32::exp(-(self.fConst21 / fTemp1));
			let mut fTemp148: F32 = MoodscoreDsp_faustpower2_f(fTemp147);
			let mut fTemp149: F32 = 1.0 - fTemp148;
			let mut fTemp150: F32 = 1.0 - fTemp5 * fTemp148;
			let mut fTemp151: F32 = F32::sqrt(F32::max(0.0, -1.0 + MoodscoreDsp_faustpower2_f(fTemp150) / MoodscoreDsp_faustpower2_f(fTemp149)));
			let mut fTemp152: F32 = fTemp150 / fTemp149;
			self.fRec16[0] = fTemp147 * (1.0 - fTemp152 + fTemp151) * (self.fRec44[1] + (-1.0 + F32::exp(-(self.fConst22 / fTemp1)) / fTemp147) * self.fRec10[0]) + (fTemp152 - fTemp151) * self.fRec16[1];
			let mut fTemp153: F32 = F32::exp(-(self.fConst25 / fTemp1));
			let mut fTemp154: F32 = MoodscoreDsp_faustpower2_f(fTemp153);
			let mut fTemp155: F32 = 1.0 - fTemp154;
			let mut fTemp156: F32 = 1.0 - fTemp5 * fTemp154;
			let mut fTemp157: F32 = F32::sqrt(F32::max(0.0, -1.0 + MoodscoreDsp_faustpower2_f(fTemp156) / MoodscoreDsp_faustpower2_f(fTemp155)));
			let mut fTemp158: F32 = fTemp156 / fTemp155;
			self.fRec17[0] = fTemp153 * (1.0 - fTemp158 + fTemp157) * (self.fRec40[1] + (-1.0 + F32::exp(-(self.fConst26 / fTemp1)) / fTemp153) * self.fRec11[0]) + (fTemp158 - fTemp157) * self.fRec17[1];
			let mut fTemp159: F32 = F32::exp(-(self.fConst29 / fTemp1));
			let mut fTemp160: F32 = MoodscoreDsp_faustpower2_f(fTemp159);
			let mut fTemp161: F32 = 1.0 - fTemp160;
			let mut fTemp162: F32 = 1.0 - fTemp5 * fTemp160;
			let mut fTemp163: F32 = F32::sqrt(F32::max(0.0, -1.0 + MoodscoreDsp_faustpower2_f(fTemp162) / MoodscoreDsp_faustpower2_f(fTemp161)));
			let mut fTemp164: F32 = fTemp162 / fTemp161;
			self.fRec18[0] = fTemp159 * (1.0 - fTemp164 + fTemp163) * (self.fRec45[1] + (-1.0 + F32::exp(-(self.fConst30 / fTemp1)) / fTemp159) * self.fRec9[0]) + (fTemp164 - fTemp163) * self.fRec18[1];
			let mut fTemp165: F32 = F32::exp(-(self.fConst33 / fTemp1));
			let mut fTemp166: F32 = MoodscoreDsp_faustpower2_f(fTemp165);
			let mut fTemp167: F32 = 1.0 - fTemp166;
			let mut fTemp168: F32 = 1.0 - fTemp5 * fTemp166;
			let mut fTemp169: F32 = F32::sqrt(F32::max(0.0, -1.0 + MoodscoreDsp_faustpower2_f(fTemp168) / MoodscoreDsp_faustpower2_f(fTemp167)));
			let mut fTemp170: F32 = fTemp168 / fTemp167;
			self.fRec19[0] = fTemp165 * (1.0 - fTemp170 + fTemp169) * (self.fRec38[1] + (-1.0 + F32::exp(-(self.fConst34 / fTemp1)) / fTemp165) * self.fRec8[0]) + (fTemp170 - fTemp169) * self.fRec19[1];
			let mut fTemp171: F32 = F32::exp(-(self.fConst37 / fTemp1));
			let mut fTemp172: F32 = MoodscoreDsp_faustpower2_f(fTemp171);
			let mut fTemp173: F32 = 1.0 - fTemp172;
			let mut fTemp174: F32 = 1.0 - fTemp5 * fTemp172;
			let mut fTemp175: F32 = F32::sqrt(F32::max(0.0, -1.0 + MoodscoreDsp_faustpower2_f(fTemp174) / MoodscoreDsp_faustpower2_f(fTemp173)));
			let mut fTemp176: F32 = fTemp174 / fTemp173;
			self.fRec20[0] = fTemp171 * (1.0 - fTemp176 + fTemp175) * (self.fRec43[1] + (-1.0 + F32::exp(-(self.fConst38 / fTemp1)) / fTemp171) * self.fRec13[0]) + (fTemp176 - fTemp175) * self.fRec20[1];
			let mut fTemp177: F32 = self.fRec60[0] * (fTemp119 + 0.39284748 * fTemp131 + 0.5879378 * fTemp125) + self.fRec59[0] * (0.39284748 * fTemp116 + 0.5879378 * fTemp110) + self.fRec58[0] * (0.19193739 * fTemp104 + 0.43776554 * fTemp98) + self.fRec57[0] * (0.5553029 * fTemp92 + 0.6805586 * fTemp86) + self.fRec56[0] * (0.05547896 * fTemp80 + 0.32101977 * fTemp74) + self.fRec55[0] * (0.6300368 * fTemp68 + 0.704927 * fTemp62);
			let mut fTemp178: F32 = fTemp44 * fTemp177 / fTemp43 - fTemp36 * self.fRec112[1];
			self.fRec111[0] = fTemp42 * fTemp178 - fTemp34 * self.fRec111[1];
			self.fRec112[0] = fTemp34 * fTemp178 + fTemp42 * self.fRec111[1];
			let mut fTemp179: F32 = fTemp36 * fTemp177 / fTemp43 + fTemp44 * self.fRec112[1] + 2.0 * (fTemp40 * self.fRec112[0] / fTemp38) + fTemp41 * self.fRec111[0] / fTemp39;
			let mut fTemp180: F32 = fTemp31 * fTemp179 / fTemp30 - fTemp21 * self.fRec110[1];
			self.fRec109[0] = fTemp27 * fTemp180 - fTemp19 * self.fRec109[1];
			self.fRec110[0] = fTemp19 * fTemp180 + fTemp27 * self.fRec109[1];
			let mut fTemp181: F32 = fTemp21 * fTemp179 / fTemp30 + fTemp31 * self.fRec110[1] + 2.0 * (fTemp25 * self.fRec110[0] / fTemp23) + fTemp26 * self.fRec109[0] / fTemp24;
			let mut fTemp182: F32 = (*input3) as F32 + 0.9 * (self.fRec50[0] * fTemp181 / fTemp15);
			self.fVec17[(self.IOTA0 & 16383) as usize] = fTemp182;
			let mut fTemp183: F32 = 0.3 * (fTemp143 * self.fVec17[((i32::wrapping_sub(self.IOTA0, iTemp144)) & 16383) as usize] + fTemp142 * self.fVec17[((i32::wrapping_sub(self.IOTA0, iTemp140)) & 16383) as usize]);
			self.fVec18[(self.IOTA0 & 32767) as usize] = 1e-20 + 0.35355338 * self.fRec17[0];
			let mut fTemp184: F32 = 0.6 * self.fRec21[1] + self.fVec18[((i32::wrapping_sub(self.IOTA0, self.iConst40)) & 32767) as usize];
			self.fVec19[(self.IOTA0 & 4095) as usize] = fTemp184 - fTemp183;
			self.fRec21[0] = self.fVec19[((i32::wrapping_sub(self.IOTA0, self.iConst41)) & 4095) as usize];
			let mut fTemp185: F32 = F32::exp(-(self.fConst44 / fTemp1));
			let mut fTemp186: F32 = MoodscoreDsp_faustpower2_f(fTemp185);
			let mut fTemp187: F32 = 1.0 - fTemp186;
			let mut fTemp188: F32 = 1.0 - fTemp5 * fTemp186;
			let mut fTemp189: F32 = F32::sqrt(F32::max(0.0, -1.0 + MoodscoreDsp_faustpower2_f(fTemp188) / MoodscoreDsp_faustpower2_f(fTemp187)));
			let mut fTemp190: F32 = fTemp188 / fTemp187;
			self.fRec22[0] = fTemp185 * (1.0 - fTemp190 + fTemp189) * (self.fRec39[1] + (-1.0 + F32::exp(-(self.fConst45 / fTemp1)) / fTemp185) * self.fRec6[0]) + (fTemp190 - fTemp189) * self.fRec22[1];
			self.fVec20[(self.IOTA0 & 16383) as usize] = 1e-20 + 0.35355338 * self.fRec16[0];
			let mut fTemp191: F32 = self.fVec20[((i32::wrapping_sub(self.IOTA0, self.iConst47)) & 16383) as usize] + fTemp183 + 0.6 * self.fRec23[1];
			self.fVec21[(self.IOTA0 & 4095) as usize] = fTemp191;
			self.fRec23[0] = self.fVec21[((i32::wrapping_sub(self.IOTA0, self.iConst48)) & 4095) as usize];
			self.fVec22[(self.IOTA0 & 16383) as usize] = 1e-20 + 0.35355338 * self.fRec19[0];
			let mut fTemp192: F32 = fTemp145 + self.fVec22[((i32::wrapping_sub(self.IOTA0, self.iConst50)) & 16383) as usize] - 0.6 * self.fRec36[1];
			self.fVec23[(self.IOTA0 & 2047) as usize] = fTemp192;
			let mut fRec24: F32 = 0.6 * fTemp192;
			let mut fTemp193: F32 = F32::exp(-(self.fConst53 / fTemp1));
			let mut fTemp194: F32 = MoodscoreDsp_faustpower2_f(fTemp193);
			let mut fTemp195: F32 = 1.0 - fTemp194;
			let mut fTemp196: F32 = 1.0 - fTemp5 * fTemp194;
			let mut fTemp197: F32 = F32::sqrt(F32::max(0.0, -1.0 + MoodscoreDsp_faustpower2_f(fTemp196) / MoodscoreDsp_faustpower2_f(fTemp195)));
			let mut fTemp198: F32 = fTemp196 / fTemp195;
			self.fRec25[0] = fTemp193 * (1.0 - fTemp198 + fTemp197) * (self.fRec42[1] + (-1.0 + F32::exp(-(self.fConst54 / fTemp1)) / fTemp193) * self.fRec12[0]) + (fTemp198 - fTemp197) * self.fRec25[1];
			self.fVec24[(self.IOTA0 & 32767) as usize] = 1e-20 + 0.35355338 * self.fRec22[0];
			let mut fTemp199: F32 = self.fVec24[((i32::wrapping_sub(self.IOTA0, self.iConst56)) & 32767) as usize] + fTemp183 + 0.6 * self.fRec37[1];
			self.fVec25[(self.IOTA0 & 4095) as usize] = fTemp199;
			let mut fRec26: F32 = -(0.6 * fTemp199);
			self.fRec27[0] = self.fVec16[((i32::wrapping_sub(self.IOTA0, self.iConst57)) & 4095) as usize];
			self.fVec26[(self.IOTA0 & 16383) as usize] = 1e-20 + 0.35355338 * self.fRec25[0];
			let mut fTemp200: F32 = self.fVec26[((i32::wrapping_sub(self.IOTA0, self.iConst59)) & 16383) as usize] - (fTemp145 + 0.6 * self.fRec32[1]);
			self.fVec27[(self.IOTA0 & 2047) as usize] = fTemp200;
			let mut fRec28: F32 = 0.6 * fTemp200;
			let mut fRec29: F32 = 0.6 * (fTemp183 - fTemp184);
			let mut fRec30: F32 = -(0.6 * fTemp191);
			self.fVec28[(self.IOTA0 & 32767) as usize] = 1e-20 + 0.35355338 * self.fRec20[0];
			let mut fTemp201: F32 = 0.6 * self.fRec31[1] + self.fVec28[((i32::wrapping_sub(self.IOTA0, self.iConst61)) & 32767) as usize];
			self.fVec29[(self.IOTA0 & 2047) as usize] = fTemp201 - fTemp183;
			self.fRec31[0] = self.fVec29[((i32::wrapping_sub(self.IOTA0, self.iConst62)) & 2047) as usize];
			self.fRec32[0] = self.fVec27[((i32::wrapping_sub(self.IOTA0, self.iConst63)) & 2047) as usize];
			self.fVec30[(self.IOTA0 & 16383) as usize] = 1e-20 + 0.35355338 * self.fRec18[0];
			let mut fTemp202: F32 = fTemp145 + self.fVec30[((i32::wrapping_sub(self.IOTA0, self.iConst65)) & 16383) as usize] - 0.6 * self.fRec34[1];
			self.fVec31[(self.IOTA0 & 4095) as usize] = fTemp202;
			let mut fRec33: F32 = 0.6 * fTemp202;
			self.fRec34[0] = self.fVec31[((i32::wrapping_sub(self.IOTA0, self.iConst66)) & 4095) as usize];
			let mut fRec35: F32 = 0.6 * (fTemp183 - fTemp201);
			self.fRec36[0] = self.fVec23[((i32::wrapping_sub(self.IOTA0, self.iConst67)) & 2047) as usize];
			self.fRec37[0] = self.fVec25[((i32::wrapping_sub(self.IOTA0, self.iConst68)) & 4095) as usize];
			let mut fTemp203: F32 = self.fRec27[1] + self.fRec21[1];
			let mut fTemp204: F32 = self.fRec23[1] + fTemp203;
			self.fRec38[0] = fRec35 + fRec29 + fRec30 + fRec26 + fRec28 + fRec15 + fRec33 + fRec24 + self.fRec34[1] + self.fRec31[1] + self.fRec32[1] + self.fRec37[1] + self.fRec36[1] + fTemp204;
			let mut fTemp205: F32 = self.fRec21[1] + self.fRec23[1];
			self.fRec39[0] = fRec28 + fRec15 + fRec33 + fRec24 + self.fRec34[1] + self.fRec32[1] + self.fRec27[1] + self.fRec36[1] - (fRec35 + fRec29 + fRec30 + fRec26 + self.fRec31[1] + self.fRec37[1] + fTemp205);
			let mut fTemp206: F32 = self.fRec27[1] + self.fRec23[1];
			self.fRec40[0] = fRec35 + fRec29 + fRec33 + fRec24 + self.fRec34[1] + self.fRec31[1] + self.fRec21[1] + self.fRec36[1] - (fRec30 + fRec26 + fRec28 + fRec15 + self.fRec32[1] + self.fRec37[1] + fTemp206);
			self.fRec41[0] = fRec30 + fRec26 + fRec33 + fRec24 + self.fRec34[1] + self.fRec37[1] + self.fRec23[1] + self.fRec36[1] - (fRec35 + fRec29 + fRec28 + fRec15 + self.fRec31[1] + self.fRec32[1] + fTemp203);
			self.fRec42[0] = fRec35 + fRec26 + fRec28 + fRec24 + self.fRec31[1] + self.fRec32[1] + self.fRec36[1] + self.fRec37[1] - (fRec29 + fRec30 + fRec15 + fRec33 + self.fRec34[1] + fTemp204);
			self.fRec43[0] = fRec29 + fRec30 + fRec28 + fRec24 + self.fRec32[1] + self.fRec36[1] + fTemp205 - (fRec35 + fRec26 + fRec15 + fRec33 + self.fRec34[1] + self.fRec31[1] + self.fRec27[1] + self.fRec37[1]);
			self.fRec44[0] = fRec35 + fRec30 + fRec15 + fRec24 + self.fRec31[1] + self.fRec36[1] + fTemp206 - (fRec29 + fRec26 + fRec28 + fRec33 + self.fRec34[1] + self.fRec32[1] + self.fRec21[1] + self.fRec37[1]);
			self.fRec45[0] = fRec29 + fRec26 + fRec15 + fRec24 + self.fRec37[1] + self.fRec36[1] + fTemp203 - (fRec35 + fRec30 + fRec28 + fRec33 + self.fRec34[1] + self.fRec31[1] + self.fRec23[1] + self.fRec32[1]);
			let mut fTemp207: F32 = self.fRec39[0] + self.fRec41[0];
			self.fRec113[0] = fSlow33 + self.fConst2 * self.fRec113[1];
			let mut fTemp208: F32 = 1.0 + self.fRec113[0];
			self.fRec115[0] = fSlow34 + self.fConst2 * self.fRec115[1];
			let mut fTemp209: F32 = 1.0 - self.fConst69 * self.fRec115[0];
			let mut fTemp210: F32 = 1.0 - self.fRec50[0];
			self.fRec116[0] = (*input0) as F32 + self.fConst69 * self.fRec115[0] * self.fRec116[1] + 0.9 * (fTemp136 * fTemp210 / fTemp15);
			self.fRec114[0] = self.fConst69 * self.fRec114[1] * self.fRec115[0] + self.fRec116[0] * fTemp209;
			let mut fTemp211: F32 = fTemp209 * self.fRec114[0] * fTemp208 + 0.37 * fTemp207 * fTemp0;
			let mut fTemp212: F32 = self.fRec3[1] + self.fConst70 * (fTemp211 - self.fRec4[1]);
			self.fRec3[0] = self.fConst72 * fTemp212 - self.fRec3[1];
			self.fRec4[0] = 2.0 * (self.fRec4[1] + self.fConst73 * fTemp212) - self.fRec4[1];
			let mut fTemp213: F32 = self.fRec4[1] + self.fRec2[1];
			let mut fTemp214: F32 = self.fRec1[1] + self.fConst70 * (fTemp211 + self.fConst75 * fTemp212 - fTemp213);
			self.fRec1[0] = self.fConst72 * fTemp214 - self.fRec1[1];
			self.fRec2[0] = 2.0 * (self.fRec2[1] + self.fConst73 * fTemp214) - self.fRec2[1];
			let mut fTemp215: F32 = self.fRec39[0] - self.fRec41[0];
			self.fRec122[0] = (*input1) as F32 + self.fConst69 * self.fRec115[0] * self.fRec122[1] + 0.9 * (fTemp210 * fTemp181 / fTemp15);
			self.fRec121[0] = self.fConst69 * self.fRec115[0] * self.fRec121[1] + fTemp209 * self.fRec122[0];
			let mut fTemp216: F32 = fTemp209 * fTemp208;
			let mut fTemp217: F32 = self.fRec119[1] + self.fConst70 * (0.5 * (fTemp216 * (self.fRec114[0] + self.fRec121[0]) + 0.37 * fTemp0 * (fTemp207 + fTemp215)) - self.fRec120[1]);
			self.fRec119[0] = self.fConst72 * fTemp217 - self.fRec119[1];
			let mut fTemp218: F32 = self.fRec120[1] + self.fConst73 * fTemp217;
			self.fRec120[0] = 2.0 * fTemp218 - self.fRec120[1];
			let mut fTemp219: F32 = self.fRec117[1] + self.fConst70 * (fTemp218 - self.fRec118[1]);
			self.fRec117[0] = self.fConst72 * fTemp219 - self.fRec117[1];
			self.fRec118[0] = 2.0 * (self.fRec118[1] + self.fConst73 * fTemp219) - self.fRec118[1];
			*output0 = (F32::tanh((self.fConst76 * (self.fConst74 * fTemp212 - 1.4142135 * fTemp214 - self.fConst70 * (fTemp214 - fTemp219)) + fTemp211 + self.fRec118[1] - fTemp213) * self.fRec0[0])) as FaustFloat;
			let mut fTemp220: F32 = fTemp216 * self.fRec121[0] + 0.37 * fTemp0 * fTemp215;
			let mut fTemp221: F32 = self.fRec126[1] + self.fConst70 * (fTemp220 - self.fRec125[1]);
			self.fRec125[0] = 2.0 * (self.fRec125[1] + self.fConst73 * fTemp221) - self.fRec125[1];
			self.fRec126[0] = self.fConst72 * fTemp221 - self.fRec126[1];
			let mut fTemp222: F32 = self.fRec125[1] + self.fRec123[1];
			let mut fTemp223: F32 = self.fRec124[1] + self.fConst70 * (fTemp220 + self.fConst75 * fTemp221 - fTemp222);
			self.fRec123[0] = 2.0 * (self.fRec123[1] + self.fConst73 * fTemp223) - self.fRec123[1];
			self.fRec124[0] = self.fConst72 * fTemp223 - self.fRec124[1];
			*output1 = (F32::tanh(self.fRec0[0] * (self.fConst76 * (self.fConst74 * fTemp221 - 1.4142135 * fTemp223 + self.fConst70 * (fTemp219 - fTemp223)) + fTemp220 + self.fRec118[1] - fTemp222))) as FaustFloat;
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
			self.fRec48[1] = self.fRec48[0];
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
			self.fVec1[1] = self.fVec1[0];
			self.fRec62[1] = self.fRec62[0];
			self.fRec64[1] = self.fRec64[0];
			self.fRec65[1] = self.fRec65[0];
			self.fRec66[1] = self.fRec66[0];
			self.fVec2[1] = self.fVec2[0];
			self.fRec67[1] = self.fRec67[0];
			self.fRec68[1] = self.fRec68[0];
			self.fRec69[1] = self.fRec69[0];
			self.fVec3[1] = self.fVec3[0];
			self.fRec70[1] = self.fRec70[0];
			self.fRec71[1] = self.fRec71[0];
			self.fRec72[1] = self.fRec72[0];
			self.fRec73[1] = self.fRec73[0];
			self.fVec4[1] = self.fVec4[0];
			self.fRec74[1] = self.fRec74[0];
			self.fRec75[1] = self.fRec75[0];
			self.fRec76[1] = self.fRec76[0];
			self.fVec5[1] = self.fVec5[0];
			self.fRec77[1] = self.fRec77[0];
			self.fRec78[1] = self.fRec78[0];
			self.fRec79[1] = self.fRec79[0];
			self.fRec80[1] = self.fRec80[0];
			self.fVec6[1] = self.fVec6[0];
			self.fRec81[1] = self.fRec81[0];
			self.fRec82[1] = self.fRec82[0];
			self.fRec83[1] = self.fRec83[0];
			self.fVec7[1] = self.fVec7[0];
			self.fRec84[1] = self.fRec84[0];
			self.fRec85[1] = self.fRec85[0];
			self.fRec86[1] = self.fRec86[0];
			self.fRec87[1] = self.fRec87[0];
			self.fVec8[1] = self.fVec8[0];
			self.fRec88[1] = self.fRec88[0];
			self.fRec89[1] = self.fRec89[0];
			self.fRec90[1] = self.fRec90[0];
			self.fVec9[1] = self.fVec9[0];
			self.fRec91[1] = self.fRec91[0];
			self.fRec92[1] = self.fRec92[0];
			self.fRec93[1] = self.fRec93[0];
			self.fRec94[1] = self.fRec94[0];
			self.fVec10[1] = self.fVec10[0];
			self.fRec95[1] = self.fRec95[0];
			self.fRec96[1] = self.fRec96[0];
			self.fRec97[1] = self.fRec97[0];
			self.fVec11[1] = self.fVec11[0];
			self.fRec100[1] = self.fRec100[0];
			self.fRec101[1] = self.fRec101[0];
			self.fRec102[1] = self.fRec102[0];
			self.fRec98[1] = self.fRec98[0];
			self.fRec99[1] = self.fRec99[0];
			self.fRec103[1] = self.fRec103[0];
			self.fVec12[1] = self.fVec12[0];
			self.fRec104[1] = self.fRec104[0];
			self.fRec105[1] = self.fRec105[0];
			self.fVec13[1] = self.fVec13[0];
			self.fRec106[1] = self.fRec106[0];
			self.fRec107[1] = self.fRec107[0];
			self.fRec53[1] = self.fRec53[0];
			self.fRec54[1] = self.fRec54[0];
			self.fRec51[1] = self.fRec51[0];
			self.fRec52[1] = self.fRec52[0];
			self.IOTA0 = i32::wrapping_add(self.IOTA0, 1);
			self.fRec108[1] = self.fRec108[0];
			self.fRec16[1] = self.fRec16[0];
			self.fRec17[1] = self.fRec17[0];
			self.fRec18[1] = self.fRec18[0];
			self.fRec19[1] = self.fRec19[0];
			self.fRec20[1] = self.fRec20[0];
			self.fRec111[1] = self.fRec111[0];
			self.fRec112[1] = self.fRec112[0];
			self.fRec109[1] = self.fRec109[0];
			self.fRec110[1] = self.fRec110[0];
			self.fRec21[1] = self.fRec21[0];
			self.fRec22[1] = self.fRec22[0];
			self.fRec23[1] = self.fRec23[0];
			self.fRec25[1] = self.fRec25[0];
			self.fRec27[1] = self.fRec27[0];
			self.fRec31[1] = self.fRec31[0];
			self.fRec32[1] = self.fRec32[0];
			self.fRec34[1] = self.fRec34[0];
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
			self.fRec113[1] = self.fRec113[0];
			self.fRec115[1] = self.fRec115[0];
			self.fRec116[1] = self.fRec116[0];
			self.fRec114[1] = self.fRec114[0];
			self.fRec3[1] = self.fRec3[0];
			self.fRec4[1] = self.fRec4[0];
			self.fRec1[1] = self.fRec1[0];
			self.fRec2[1] = self.fRec2[0];
			self.fRec122[1] = self.fRec122[0];
			self.fRec121[1] = self.fRec121[0];
			self.fRec119[1] = self.fRec119[0];
			self.fRec120[1] = self.fRec120[0];
			self.fRec117[1] = self.fRec117[0];
			self.fRec118[1] = self.fRec118[0];
			self.fRec125[1] = self.fRec125[0];
			self.fRec126[1] = self.fRec126[0];
			self.fRec123[1] = self.fRec123[0];
			self.fRec124[1] = self.fRec124[0];
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
