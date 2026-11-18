/* ------------------------------------------------------------
license: "MIT OR Apache-2.0"
name: "moodscore"
version: "0.0.1"
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
	fHslider1: FaustFloat,
	fRec41: [F32;2],
	fConst18: F32,
	fHslider2: FaustFloat,
	fRec42: [F32;2],
	fHslider3: FaustFloat,
	fRec43: [F32;2],
	fHslider4: FaustFloat,
	fRec48: [F32;2],
	fConst19: F32,
	fConst20: F32,
	iRec50: [i32;2],
	fVec1: [F32;2],
	fConst21: F32,
	fRec49: [F32;2],
	fHslider5: FaustFloat,
	fRec51: [F32;2],
	fHslider6: FaustFloat,
	fRec52: [F32;2],
	fConst22: F32,
	fRec53: [F32;2],
	fVec2: [F32;2],
	fRec54: [F32;2],
	fRec55: [F32;2],
	fHslider7: FaustFloat,
	fRec56: [F32;2],
	fVec3: [F32;2],
	fRec59: [F32;2],
	fRec57: [F32;2],
	fRec58: [F32;2],
	fRec46: [F32;2],
	fRec47: [F32;2],
	fRec44: [F32;2],
	fRec45: [F32;2],
	IOTA0: i32,
	fVec4: [F32;32],
	fVec5: [F32;16384],
	fConst23: F32,
	iConst24: i32,
	fVec6: [F32;2048],
	fConst25: F32,
	fConst26: F32,
	fConst27: F32,
	fConst28: F32,
	fConst29: F32,
	fConst30: F32,
	fConst31: F32,
	fConst32: F32,
	fConst33: F32,
	fConst34: F32,
	fConst35: F32,
	fRec11: [F32;2],
	fVec7: [F32;32768],
	fConst36: F32,
	iConst37: i32,
	fVec8: [F32;4096],
	iConst38: i32,
	fRec12: [F32;2],
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
	fRec13: [F32;2],
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
	fRec14: [F32;2],
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
	fRec15: [F32;2],
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
	fRec16: [F32;2],
	fConst83: F32,
	fConst84: F32,
	fConst85: F32,
	fConst86: F32,
	fConst87: F32,
	fConst88: F32,
	fConst89: F32,
	fConst90: F32,
	fConst91: F32,
	fConst92: F32,
	fConst93: F32,
	fRec17: [F32;2],
	fVec9: [F32;16384],
	fConst94: F32,
	iConst95: i32,
	fVec10: [F32;2048],
	iConst96: i32,
	fRec19: [F32;2],
	fConst97: F32,
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
	fRec20: [F32;2],
	fVec11: [F32;16384],
	fConst108: F32,
	iConst109: i32,
	fVec12: [F32;4096],
	iConst110: i32,
	fRec21: [F32;2],
	iConst111: i32,
	fRec22: [F32;2],
	fVec13: [F32;32768],
	fConst112: F32,
	iConst113: i32,
	fVec14: [F32;4096],
	iConst114: i32,
	fRec23: [F32;2],
	fVec15: [F32;32768],
	fConst115: F32,
	iConst116: i32,
	fVec16: [F32;2048],
	iConst117: i32,
	fRec24: [F32;2],
	fVec17: [F32;16384],
	fConst118: F32,
	iConst119: i32,
	fVec18: [F32;4096],
	fVec19: [F32;16384],
	fConst120: F32,
	iConst121: i32,
	fVec20: [F32;4096],
	iConst122: i32,
	fRec27: [F32;2],
	iConst123: i32,
	fRec31: [F32;2],
	fRec33: [F32;3],
	fRec34: [F32;3],
	fRec35: [F32;3],
	fRec36: [F32;3],
	fRec37: [F32;3],
	fRec38: [F32;3],
	fRec39: [F32;3],
	fRec40: [F32;3],
	fHslider8: FaustFloat,
	fRec60: [F32;2],
}


fn MoodscoreDsp_faustpower2_f(value: F32) -> F32 {
	return value * value;
}
pub const FAUST_INPUTS: usize = 0;
pub const FAUST_OUTPUTS: usize = 2;
pub const FAUST_ACTIVES: usize = 9;
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
			fHslider1: 0.0,
			fRec41: [0.0;2],
			fConst18: 0.0,
			fHslider2: 0.0,
			fRec42: [0.0;2],
			fHslider3: 0.0,
			fRec43: [0.0;2],
			fHslider4: 0.0,
			fRec48: [0.0;2],
			fConst19: 0.0,
			fConst20: 0.0,
			iRec50: [0;2],
			fVec1: [0.0;2],
			fConst21: 0.0,
			fRec49: [0.0;2],
			fHslider5: 0.0,
			fRec51: [0.0;2],
			fHslider6: 0.0,
			fRec52: [0.0;2],
			fConst22: 0.0,
			fRec53: [0.0;2],
			fVec2: [0.0;2],
			fRec54: [0.0;2],
			fRec55: [0.0;2],
			fHslider7: 0.0,
			fRec56: [0.0;2],
			fVec3: [0.0;2],
			fRec59: [0.0;2],
			fRec57: [0.0;2],
			fRec58: [0.0;2],
			fRec46: [0.0;2],
			fRec47: [0.0;2],
			fRec44: [0.0;2],
			fRec45: [0.0;2],
			IOTA0: 0,
			fVec4: [0.0;32],
			fVec5: [0.0;16384],
			fConst23: 0.0,
			iConst24: 0,
			fVec6: [0.0;2048],
			fConst25: 0.0,
			fConst26: 0.0,
			fConst27: 0.0,
			fConst28: 0.0,
			fConst29: 0.0,
			fConst30: 0.0,
			fConst31: 0.0,
			fConst32: 0.0,
			fConst33: 0.0,
			fConst34: 0.0,
			fConst35: 0.0,
			fRec11: [0.0;2],
			fVec7: [0.0;32768],
			fConst36: 0.0,
			iConst37: 0,
			fVec8: [0.0;4096],
			iConst38: 0,
			fRec12: [0.0;2],
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
			fRec13: [0.0;2],
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
			fRec14: [0.0;2],
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
			fRec15: [0.0;2],
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
			fRec16: [0.0;2],
			fConst83: 0.0,
			fConst84: 0.0,
			fConst85: 0.0,
			fConst86: 0.0,
			fConst87: 0.0,
			fConst88: 0.0,
			fConst89: 0.0,
			fConst90: 0.0,
			fConst91: 0.0,
			fConst92: 0.0,
			fConst93: 0.0,
			fRec17: [0.0;2],
			fVec9: [0.0;16384],
			fConst94: 0.0,
			iConst95: 0,
			fVec10: [0.0;2048],
			iConst96: 0,
			fRec19: [0.0;2],
			fConst97: 0.0,
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
			fRec20: [0.0;2],
			fVec11: [0.0;16384],
			fConst108: 0.0,
			iConst109: 0,
			fVec12: [0.0;4096],
			iConst110: 0,
			fRec21: [0.0;2],
			iConst111: 0,
			fRec22: [0.0;2],
			fVec13: [0.0;32768],
			fConst112: 0.0,
			iConst113: 0,
			fVec14: [0.0;4096],
			iConst114: 0,
			fRec23: [0.0;2],
			fVec15: [0.0;32768],
			fConst115: 0.0,
			iConst116: 0,
			fVec16: [0.0;2048],
			iConst117: 0,
			fRec24: [0.0;2],
			fVec17: [0.0;16384],
			fConst118: 0.0,
			iConst119: 0,
			fVec18: [0.0;4096],
			fVec19: [0.0;16384],
			fConst120: 0.0,
			iConst121: 0,
			fVec20: [0.0;4096],
			iConst122: 0,
			fRec27: [0.0;2],
			iConst123: 0,
			fRec31: [0.0;2],
			fRec33: [0.0;3],
			fRec34: [0.0;3],
			fRec35: [0.0;3],
			fRec36: [0.0;3],
			fRec37: [0.0;3],
			fRec38: [0.0;3],
			fRec39: [0.0;3],
			fRec40: [0.0;3],
			fHslider8: 0.0,
			fRec60: [0.0;2],
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
		m.declare("version", r"0.0.1");
	}

	pub fn get_sample_rate(&self) -> i32 { self.fSampleRate as i32}
	
	pub fn class_init(sample_rate: i32) {
		// Obtaining locks on 0 static var(s)
	}
	pub fn instance_reset_params(&mut self) {
		self.fHslider0 = (0.5) as FaustFloat;
		self.fHslider1 = (9e+02) as FaustFloat;
		self.fHslider2 = (0.3) as FaustFloat;
		self.fHslider3 = (0.7) as FaustFloat;
		self.fHslider4 = (7.0) as FaustFloat;
		self.fHslider5 = (0.3) as FaustFloat;
		self.fHslider6 = (55.0) as FaustFloat;
		self.fHslider7 = (0.5) as FaustFloat;
		self.fHslider8 = (0.35) as FaustFloat;
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
			self.fRec41[l11 as usize] = 0.0;
		}
		for l12 in 0..2 {
			self.fRec42[l12 as usize] = 0.0;
		}
		for l13 in 0..2 {
			self.fRec43[l13 as usize] = 0.0;
		}
		for l14 in 0..2 {
			self.fRec48[l14 as usize] = 0.0;
		}
		for l15 in 0..2 {
			self.iRec50[l15 as usize] = 0;
		}
		for l16 in 0..2 {
			self.fVec1[l16 as usize] = 0.0;
		}
		for l17 in 0..2 {
			self.fRec49[l17 as usize] = 0.0;
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
			self.fVec2[l21 as usize] = 0.0;
		}
		for l22 in 0..2 {
			self.fRec54[l22 as usize] = 0.0;
		}
		for l23 in 0..2 {
			self.fRec55[l23 as usize] = 0.0;
		}
		for l24 in 0..2 {
			self.fRec56[l24 as usize] = 0.0;
		}
		for l25 in 0..2 {
			self.fVec3[l25 as usize] = 0.0;
		}
		for l26 in 0..2 {
			self.fRec59[l26 as usize] = 0.0;
		}
		for l27 in 0..2 {
			self.fRec57[l27 as usize] = 0.0;
		}
		for l28 in 0..2 {
			self.fRec58[l28 as usize] = 0.0;
		}
		for l29 in 0..2 {
			self.fRec46[l29 as usize] = 0.0;
		}
		for l30 in 0..2 {
			self.fRec47[l30 as usize] = 0.0;
		}
		for l31 in 0..2 {
			self.fRec44[l31 as usize] = 0.0;
		}
		for l32 in 0..2 {
			self.fRec45[l32 as usize] = 0.0;
		}
		self.IOTA0 = 0;
		for l33 in 0..32 {
			self.fVec4[l33 as usize] = 0.0;
		}
		for l34 in 0..16384 {
			self.fVec5[l34 as usize] = 0.0;
		}
		for l35 in 0..2048 {
			self.fVec6[l35 as usize] = 0.0;
		}
		for l36 in 0..2 {
			self.fRec11[l36 as usize] = 0.0;
		}
		for l37 in 0..32768 {
			self.fVec7[l37 as usize] = 0.0;
		}
		for l38 in 0..4096 {
			self.fVec8[l38 as usize] = 0.0;
		}
		for l39 in 0..2 {
			self.fRec12[l39 as usize] = 0.0;
		}
		for l40 in 0..2 {
			self.fRec13[l40 as usize] = 0.0;
		}
		for l41 in 0..2 {
			self.fRec14[l41 as usize] = 0.0;
		}
		for l42 in 0..2 {
			self.fRec15[l42 as usize] = 0.0;
		}
		for l43 in 0..2 {
			self.fRec16[l43 as usize] = 0.0;
		}
		for l44 in 0..2 {
			self.fRec17[l44 as usize] = 0.0;
		}
		for l45 in 0..16384 {
			self.fVec9[l45 as usize] = 0.0;
		}
		for l46 in 0..2048 {
			self.fVec10[l46 as usize] = 0.0;
		}
		for l47 in 0..2 {
			self.fRec19[l47 as usize] = 0.0;
		}
		for l48 in 0..2 {
			self.fRec20[l48 as usize] = 0.0;
		}
		for l49 in 0..16384 {
			self.fVec11[l49 as usize] = 0.0;
		}
		for l50 in 0..4096 {
			self.fVec12[l50 as usize] = 0.0;
		}
		for l51 in 0..2 {
			self.fRec21[l51 as usize] = 0.0;
		}
		for l52 in 0..2 {
			self.fRec22[l52 as usize] = 0.0;
		}
		for l53 in 0..32768 {
			self.fVec13[l53 as usize] = 0.0;
		}
		for l54 in 0..4096 {
			self.fVec14[l54 as usize] = 0.0;
		}
		for l55 in 0..2 {
			self.fRec23[l55 as usize] = 0.0;
		}
		for l56 in 0..32768 {
			self.fVec15[l56 as usize] = 0.0;
		}
		for l57 in 0..2048 {
			self.fVec16[l57 as usize] = 0.0;
		}
		for l58 in 0..2 {
			self.fRec24[l58 as usize] = 0.0;
		}
		for l59 in 0..16384 {
			self.fVec17[l59 as usize] = 0.0;
		}
		for l60 in 0..4096 {
			self.fVec18[l60 as usize] = 0.0;
		}
		for l61 in 0..16384 {
			self.fVec19[l61 as usize] = 0.0;
		}
		for l62 in 0..4096 {
			self.fVec20[l62 as usize] = 0.0;
		}
		for l63 in 0..2 {
			self.fRec27[l63 as usize] = 0.0;
		}
		for l64 in 0..2 {
			self.fRec31[l64 as usize] = 0.0;
		}
		for l65 in 0..3 {
			self.fRec33[l65 as usize] = 0.0;
		}
		for l66 in 0..3 {
			self.fRec34[l66 as usize] = 0.0;
		}
		for l67 in 0..3 {
			self.fRec35[l67 as usize] = 0.0;
		}
		for l68 in 0..3 {
			self.fRec36[l68 as usize] = 0.0;
		}
		for l69 in 0..3 {
			self.fRec37[l69 as usize] = 0.0;
		}
		for l70 in 0..3 {
			self.fRec38[l70 as usize] = 0.0;
		}
		for l71 in 0..3 {
			self.fRec39[l71 as usize] = 0.0;
		}
		for l72 in 0..3 {
			self.fRec40[l72 as usize] = 0.0;
		}
		for l73 in 0..2 {
			self.fRec60[l73 as usize] = 0.0;
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
		self.fConst18 = 3.1415927 / self.fConst0;
		self.fConst19 = 1.0 / F32::tan(0.4712389 / self.fConst0);
		self.fConst20 = 1.0 - self.fConst19;
		self.fConst21 = 1.0 / (self.fConst19 + 1.0);
		self.fConst22 = 1.0 / self.fConst0;
		self.fConst23 = F32::floor(0.013458 * self.fConst0 + 0.5);
		self.iConst24 = (F32::min(8192.0, F32::max(0.0, self.fConst6 - self.fConst23))) as i32;
		self.fConst25 = F32::floor(0.256891 * self.fConst0 + 0.5);
		self.fConst26 = self.fConst25 / self.fConst0;
		self.fConst27 = F32::exp(-(0.8634694 * self.fConst26));
		self.fConst28 = MoodscoreDsp_faustpower2_f(self.fConst27);
		self.fConst29 = 1.0 - self.fConst28;
		self.fConst30 = 1.0 - self.fConst11 * self.fConst28;
		self.fConst31 = F32::sqrt(F32::max(0.0, MoodscoreDsp_faustpower2_f(self.fConst30) / MoodscoreDsp_faustpower2_f(self.fConst29) + -1.0));
		self.fConst32 = self.fConst30 / self.fConst29;
		self.fConst33 = self.fConst32 - self.fConst31;
		self.fConst34 = F32::exp(-(1.1512926 * self.fConst26)) / self.fConst27 + -1.0;
		self.fConst35 = self.fConst27 * (self.fConst31 + (1.0 - self.fConst32));
		self.fConst36 = F32::floor(0.027333 * self.fConst0 + 0.5);
		self.iConst37 = (F32::min(16384.0, F32::max(0.0, self.fConst25 - self.fConst36))) as i32;
		self.iConst38 = (F32::min(2048.0, F32::max(0.0, self.fConst36 + -1.0))) as i32;
		self.fConst39 = F32::floor(0.174713 * self.fConst0 + 0.5);
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
		self.fConst50 = F32::floor(0.219991 * self.fConst0 + 0.5);
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
		self.fConst61 = F32::floor(0.153129 * self.fConst0 + 0.5);
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
		self.fConst72 = F32::floor(0.192303 * self.fConst0 + 0.5);
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
		self.fConst83 = F32::floor(0.127837 * self.fConst0 + 0.5);
		self.fConst84 = self.fConst83 / self.fConst0;
		self.fConst85 = F32::exp(-(0.8634694 * self.fConst84));
		self.fConst86 = MoodscoreDsp_faustpower2_f(self.fConst85);
		self.fConst87 = 1.0 - self.fConst86;
		self.fConst88 = 1.0 - self.fConst11 * self.fConst86;
		self.fConst89 = F32::sqrt(F32::max(0.0, MoodscoreDsp_faustpower2_f(self.fConst88) / MoodscoreDsp_faustpower2_f(self.fConst87) + -1.0));
		self.fConst90 = self.fConst88 / self.fConst87;
		self.fConst91 = self.fConst90 - self.fConst89;
		self.fConst92 = F32::exp(-(1.1512926 * self.fConst84)) / self.fConst85 + -1.0;
		self.fConst93 = self.fConst85 * (self.fConst89 + (1.0 - self.fConst90));
		self.fConst94 = F32::floor(0.020346 * self.fConst0 + 0.5);
		self.iConst95 = (F32::min(8192.0, F32::max(0.0, self.fConst61 - self.fConst94))) as i32;
		self.iConst96 = (F32::min(1024.0, F32::max(0.0, self.fConst23 + -1.0))) as i32;
		self.fConst97 = F32::floor(0.210389 * self.fConst0 + 0.5);
		self.fConst98 = self.fConst97 / self.fConst0;
		self.fConst99 = F32::exp(-(0.8634694 * self.fConst98));
		self.fConst100 = MoodscoreDsp_faustpower2_f(self.fConst99);
		self.fConst101 = 1.0 - self.fConst100;
		self.fConst102 = 1.0 - self.fConst11 * self.fConst100;
		self.fConst103 = F32::sqrt(F32::max(0.0, MoodscoreDsp_faustpower2_f(self.fConst102) / MoodscoreDsp_faustpower2_f(self.fConst101) + -1.0));
		self.fConst104 = self.fConst102 / self.fConst101;
		self.fConst105 = self.fConst104 - self.fConst103;
		self.fConst106 = F32::exp(-(1.1512926 * self.fConst98)) / self.fConst99 + -1.0;
		self.fConst107 = self.fConst99 * (self.fConst103 + (1.0 - self.fConst104));
		self.fConst108 = F32::floor(0.022904 * self.fConst0 + 0.5);
		self.iConst109 = (F32::min(8192.0, F32::max(0.0, self.fConst39 - self.fConst108))) as i32;
		self.iConst110 = (F32::min(2048.0, F32::max(0.0, self.fConst108 + -1.0))) as i32;
		self.iConst111 = (F32::min(1024.0, F32::max(0.0, self.fConst94 + -1.0))) as i32;
		self.fConst112 = F32::floor(0.024421 * self.fConst0 + 0.5);
		self.iConst113 = (F32::min(16384.0, F32::max(0.0, self.fConst97 - self.fConst112))) as i32;
		self.iConst114 = (F32::min(2048.0, F32::max(0.0, self.fConst112 + -1.0))) as i32;
		self.fConst115 = F32::floor(0.019123 * self.fConst0 + 0.5);
		self.iConst116 = (F32::min(16384.0, F32::max(0.0, self.fConst50 - self.fConst115))) as i32;
		self.iConst117 = (F32::min(1024.0, F32::max(0.0, self.fConst115 + -1.0))) as i32;
		self.fConst118 = F32::floor(0.029291 * self.fConst0 + 0.5);
		self.iConst119 = (F32::min(8192.0, F32::max(0.0, self.fConst72 - self.fConst118))) as i32;
		self.fConst120 = F32::floor(0.031604 * self.fConst0 + 0.5);
		self.iConst121 = (F32::min(8192.0, F32::max(0.0, self.fConst83 - self.fConst120))) as i32;
		self.iConst122 = (F32::min(2048.0, F32::max(0.0, self.fConst120 + -1.0))) as i32;
		self.iConst123 = (F32::min(2048.0, F32::max(0.0, self.fConst118 + -1.0))) as i32;
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
		ui_interface.declare(Some(ParamIndex(3)), "unit", "Hz");
		ui_interface.add_horizontal_slider("drone_1/pitch", ParamIndex(3), 55.0, 2e+01, 4.4e+02, 0.01);
		ui_interface.add_horizontal_slider("drone_1/sub", ParamIndex(4), 0.5, 0.0, 1.0, 0.001);
		ui_interface.declare(Some(ParamIndex(5)), "scale", "log");
		ui_interface.declare(Some(ParamIndex(5)), "unit", "Hz");
		ui_interface.add_horizontal_slider("filter/cutoff", ParamIndex(5), 9e+02, 4e+01, 1.2e+04, 0.1);
		ui_interface.add_horizontal_slider("filter/resonance", ParamIndex(6), 0.3, 0.0, 0.95, 0.001);
		ui_interface.add_horizontal_slider("master/gain", ParamIndex(7), 0.5, 0.0, 1.0, 0.001);
		ui_interface.add_horizontal_slider("space/reverb_mix", ParamIndex(8), 0.35, 0.0, 1.0, 0.001);
		ui_interface.close_box();
	}
	
	pub fn get_param(&self, param: ParamIndex) -> Option<FaustFloat> {
		match param.0 {
			7 => Some(self.fHslider0),
			5 => Some(self.fHslider1),
			6 => Some(self.fHslider2),
			2 => Some(self.fHslider3),
			0 => Some(self.fHslider4),
			1 => Some(self.fHslider5),
			3 => Some(self.fHslider6),
			4 => Some(self.fHslider7),
			8 => Some(self.fHslider8),
			_ => None,
		}
	}
	
	pub fn set_param(&mut self, param: ParamIndex, value: FaustFloat) {
		match param.0 {
			7 => { self.fHslider0 = value }
			5 => { self.fHslider1 = value }
			6 => { self.fHslider2 = value }
			2 => { self.fHslider3 = value }
			0 => { self.fHslider4 = value }
			1 => { self.fHslider5 = value }
			3 => { self.fHslider6 = value }
			4 => { self.fHslider7 = value }
			8 => { self.fHslider8 = value }
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
		let zipped_iterators = outputs0.zip(outputs1);
		for (output0, output1) in zipped_iterators {
			self.iVec0[0] = 1;
			self.fRec0[0] = fSlow0 + self.fConst2 * self.fRec0[1];
			self.fRec1[0] = -(self.fConst5 * (self.fConst4 * self.fRec1[1] - (self.fRec40[1] + self.fRec40[2])));
			self.fRec2[0] = -(self.fConst5 * (self.fConst4 * self.fRec2[1] - (self.fRec36[1] + self.fRec36[2])));
			self.fRec3[0] = -(self.fConst5 * (self.fConst4 * self.fRec3[1] - (self.fRec35[1] + self.fRec35[2])));
			self.fRec4[0] = -(self.fConst5 * (self.fConst4 * self.fRec4[1] - (self.fRec33[1] + self.fRec33[2])));
			self.fRec5[0] = -(self.fConst5 * (self.fConst4 * self.fRec5[1] - (self.fRec34[1] + self.fRec34[2])));
			self.fRec6[0] = -(self.fConst5 * (self.fConst4 * self.fRec6[1] - (self.fRec38[1] + self.fRec38[2])));
			self.fRec7[0] = -(self.fConst5 * (self.fConst4 * self.fRec7[1] - (self.fRec37[1] + self.fRec37[2])));
			self.fRec8[0] = -(self.fConst5 * (self.fConst4 * self.fRec8[1] - (self.fRec39[1] + self.fRec39[2])));
			self.fRec9[0] = self.fConst17 * (self.fRec36[1] + self.fConst16 * self.fRec2[0]) + self.fConst15 * self.fRec9[1];
			self.fRec41[0] = fSlow1 + self.fConst2 * self.fRec41[1];
			let mut fTemp0: F32 = F32::tan(self.fConst18 * F32::max(self.fRec41[0], 2e+01));
			self.fRec42[0] = fSlow2 + self.fConst2 * self.fRec42[1];
			let mut fTemp1: F32 = 1.99998 * self.fRec42[0];
			let mut fTemp2: F32 = 2.0 - fTemp1;
			let mut fTemp3: F32 = 1.0 / fTemp0;
			let mut fTemp4: F32 = MoodscoreDsp_faustpower2_f(1.4141995 * self.fRec42[0]);
			let mut fTemp5: F32 = 1.0 - fTemp1;
			let mut fTemp6: F32 = fTemp5 + fTemp4 + (fTemp3 + fTemp2) / fTemp0;
			self.fRec43[0] = fSlow3 + self.fConst2 * self.fRec43[1];
			let mut fTemp7: F32 = (fTemp5 + fTemp4 + (fTemp3 - fTemp2) / fTemp0) / fTemp6;
			let mut fTemp8: F32 = 1.0 / MoodscoreDsp_faustpower2_f(fTemp0);
			let mut fTemp9: F32 = 1.0 - (fTemp1 + fTemp8) + fTemp4;
			let mut fTemp10: F32 = F32::max(-0.9999999, F32::min(0.9999999, 2.0 * (fTemp9 / (fTemp6 * (1.0 + fTemp7)))));
			let mut fTemp11: F32 = 1.0 - MoodscoreDsp_faustpower2_f(fTemp10);
			let mut fTemp12: F32 = F32::max(-0.9999999, F32::min(0.9999999, fTemp7));
			let mut fTemp13: F32 = 1.0 - MoodscoreDsp_faustpower2_f(fTemp12);
			let mut fTemp14: F32 = F32::sqrt(fTemp13);
			let mut fTemp15: F32 = 1.0 - fTemp9 / fTemp6;
			let mut fTemp16: F32 = F32::sqrt(F32::max(0.0, fTemp11));
			let mut fTemp17: F32 = 2.0 + fTemp1;
			let mut fTemp18: F32 = fTemp1 + fTemp4;
			let mut fTemp19: F32 = 1.0 + fTemp18 + (fTemp17 + fTemp3) / fTemp0;
			let mut fTemp20: F32 = F32::sqrt(F32::max(0.0, fTemp13));
			let mut fTemp21: F32 = (1.0 - (fTemp17 - fTemp3) / fTemp0 + fTemp18) / fTemp19;
			let mut fTemp22: F32 = 1.0 - fTemp8 + fTemp18;
			let mut fTemp23: F32 = F32::max(-0.9999999, F32::min(0.9999999, 2.0 * (fTemp22 / (fTemp19 * (1.0 + fTemp21)))));
			let mut fTemp24: F32 = 1.0 - MoodscoreDsp_faustpower2_f(fTemp23);
			let mut fTemp25: F32 = F32::max(-0.9999999, F32::min(0.9999999, fTemp21));
			let mut fTemp26: F32 = 1.0 - MoodscoreDsp_faustpower2_f(fTemp25);
			let mut fTemp27: F32 = F32::sqrt(fTemp26);
			let mut fTemp28: F32 = 1.0 - fTemp22 / fTemp19;
			let mut fTemp29: F32 = F32::sqrt(F32::max(0.0, fTemp24));
			let mut fTemp30: F32 = F32::sqrt(F32::max(0.0, fTemp26));
			self.fRec48[0] = fSlow4 + self.fConst2 * self.fRec48[1];
			let mut iTemp31: i32 = i32::wrapping_mul(1103515245, i32::wrapping_add(12345, self.iRec50[1]));
			let mut iTemp32: i32 = i32::wrapping_mul(1103515245, i32::wrapping_add(12345, iTemp31));
			self.iRec50[0] = i32::wrapping_mul(1103515245, i32::wrapping_add(12345, iTemp32));
			let mut fTemp33: F32 = (iTemp32) as F32;
			self.fVec1[0] = fTemp33;
			self.fRec49[0] = self.fConst21 * (4.656613e-10 * (fTemp33 + self.fVec1[1]) - self.fConst20 * self.fRec49[1]);
			self.fRec51[0] = fSlow5 + self.fConst2 * self.fRec51[1];
			self.fRec52[0] = fSlow6 + self.fConst2 * self.fRec52[1];
			let mut fTemp34: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec52[0] * F32::powf(2.0, 0.00083333335 * (4.5e+03 * self.fRec51[0] * self.fRec49[0] - self.fRec48[0]))));
			let mut fTemp35: F32 = self.fRec53[1] + self.fConst22 * fTemp34;
			let mut fTemp36: F32 = -1.0 + fTemp35;
			let mut iTemp37: i32 = (fTemp36 < 0.0) as i32;
			self.fRec53[0] = (if iTemp37 != 0 {fTemp35} else {fTemp36});
			let mut fTemp38: F32 = (self.iRec50[0]) as F32;
			self.fVec2[0] = fTemp38;
			self.fRec54[0] = self.fConst21 * (4.656613e-10 * (fTemp38 + self.fVec2[1]) - self.fConst20 * self.fRec54[1]);
			let mut fTemp39: F32 = F32::max(1.1920929e-07, F32::abs(self.fRec52[0] * F32::powf(2.0, 0.00083333335 * (4.5e+03 * self.fRec54[0] * self.fRec51[0] + self.fRec48[0]))));
			let mut fTemp40: F32 = self.fRec55[1] + self.fConst22 * fTemp39;
			let mut fTemp41: F32 = -1.0 + fTemp40;
			let mut iTemp42: i32 = (fTemp41 < 0.0) as i32;
			self.fRec55[0] = (if iTemp42 != 0 {fTemp40} else {fTemp41});
			self.fRec56[0] = fSlow7 + self.fConst2 * self.fRec56[1];
			let mut fTemp43: F32 = (iTemp31) as F32;
			self.fVec3[0] = fTemp43;
			self.fRec59[0] = self.fConst21 * (4.656613e-10 * (fTemp43 + self.fVec3[1]) - self.fConst20 * self.fRec59[1]);
			let mut fTemp44: F32 = F32::powf(2.0, 3.75 * self.fRec51[0] * self.fRec59[0]);
			self.fRec57[0] = self.fRec57[1] - self.fConst18 * self.fRec52[0] * self.fRec58[1] * fTemp44;
			self.fRec58[0] = (i32::wrapping_sub(1, self.iVec0[1])) as F32 + self.fRec58[1] + self.fConst18 * self.fRec52[0] * fTemp44 * self.fRec57[0];
			let mut fTemp45: F32 = self.fRec58[0] * self.fRec56[0] - (1.0 - ((if iTemp42 != 0 {fTemp40} else {fTemp40 + fTemp41 * (1.0 - self.fConst0 / fTemp39)}) + (if iTemp37 != 0 {fTemp35} else {fTemp35 + fTemp36 * (1.0 - self.fConst0 / fTemp34)})));
			let mut fTemp46: F32 = fTemp45 * fTemp30 - fTemp25 * self.fRec47[1];
			self.fRec46[0] = fTemp46 * fTemp29 - self.fRec46[1] * fTemp23;
			self.fRec47[0] = fTemp23 * fTemp46 + self.fRec46[1] * fTemp29;
			let mut fTemp47: F32 = fTemp45 * fTemp25 + self.fRec47[1] * fTemp30 + 2.0 * (self.fRec47[0] * fTemp28 / fTemp27) + self.fRec46[0] * (1.0 - fTemp21 - 2.0 * fTemp23 * fTemp28) / (fTemp27 * F32::sqrt(fTemp24));
			let mut fTemp48: F32 = fTemp47 * fTemp20 / fTemp19 - fTemp12 * self.fRec45[1];
			self.fRec44[0] = fTemp48 * fTemp16 - self.fRec44[1] * fTemp10;
			self.fRec45[0] = fTemp10 * fTemp48 + self.fRec44[1] * fTemp16;
			let mut fTemp49: F32 = (fTemp47 * fTemp12 / fTemp19 + self.fRec45[1] * fTemp20 + 2.0 * (self.fRec45[0] * fTemp15 / fTemp14) + self.fRec44[0] * (1.0 - fTemp7 - 2.0 * fTemp10 * fTemp15) / (fTemp14 * F32::sqrt(fTemp11))) * self.fRec43[0];
			let mut fTemp50: F32 = fTemp49 / fTemp6;
			self.fVec4[(self.IOTA0 & 31) as usize] = fTemp50;
			let mut fTemp51: F32 = self.fVec4[((i32::wrapping_sub(self.IOTA0, 23)) & 31) as usize];
			let mut fTemp52: F32 = 0.27 * fTemp51;
			self.fVec5[(self.IOTA0 & 16383) as usize] = 1e-20 + 0.35355338 * self.fRec9[0];
			let mut fTemp53: F32 = self.fVec5[((i32::wrapping_sub(self.IOTA0, self.iConst24)) & 16383) as usize] - (0.6 * self.fRec19[1] + fTemp52);
			self.fVec6[(self.IOTA0 & 2047) as usize] = fTemp53;
			let mut fRec10: F32 = 0.6 * fTemp53;
			self.fRec11[0] = self.fConst35 * (self.fRec38[1] + self.fConst34 * self.fRec6[0]) + self.fConst33 * self.fRec11[1];
			let mut fTemp54: F32 = 0.27 * fTemp50;
			self.fVec7[(self.IOTA0 & 32767) as usize] = 1e-20 + 0.35355338 * self.fRec11[0];
			let mut fTemp55: F32 = 0.6 * self.fRec12[1] + self.fVec7[((i32::wrapping_sub(self.IOTA0, self.iConst37)) & 32767) as usize];
			self.fVec8[(self.IOTA0 & 4095) as usize] = fTemp55 - fTemp54;
			self.fRec12[0] = self.fVec8[((i32::wrapping_sub(self.IOTA0, self.iConst38)) & 4095) as usize];
			self.fRec13[0] = self.fConst49 * (self.fRec33[1] + self.fConst48 * self.fRec4[0]) + self.fConst47 * self.fRec13[1];
			self.fRec14[0] = self.fConst60 * (self.fRec35[1] + self.fConst59 * self.fRec3[0]) + self.fConst58 * self.fRec14[1];
			self.fRec15[0] = self.fConst71 * (self.fRec40[1] + self.fConst70 * self.fRec1[0]) + self.fConst69 * self.fRec15[1];
			self.fRec16[0] = self.fConst82 * (self.fRec34[1] + self.fConst81 * self.fRec5[0]) + self.fConst80 * self.fRec16[1];
			self.fRec17[0] = self.fConst93 * (self.fRec37[1] + self.fConst92 * self.fRec7[0]) + self.fConst91 * self.fRec17[1];
			self.fVec9[(self.IOTA0 & 16383) as usize] = 1e-20 + 0.35355338 * self.fRec15[0];
			let mut fTemp56: F32 = fTemp52 + self.fVec9[((i32::wrapping_sub(self.IOTA0, self.iConst95)) & 16383) as usize] - 0.6 * self.fRec22[1];
			self.fVec10[(self.IOTA0 & 2047) as usize] = fTemp56;
			let mut fRec18: F32 = 0.6 * fTemp56;
			self.fRec19[0] = self.fVec6[((i32::wrapping_sub(self.IOTA0, self.iConst96)) & 2047) as usize];
			self.fRec20[0] = self.fConst107 * (self.fRec39[1] + self.fConst106 * self.fRec8[0]) + self.fConst105 * self.fRec20[1];
			self.fVec11[(self.IOTA0 & 16383) as usize] = 1e-20 + 0.35355338 * self.fRec13[0];
			let mut fTemp57: F32 = fTemp52 + self.fVec11[((i32::wrapping_sub(self.IOTA0, self.iConst109)) & 16383) as usize] - 0.6 * self.fRec21[1];
			self.fVec12[(self.IOTA0 & 4095) as usize] = fTemp57;
			self.fRec21[0] = self.fVec12[((i32::wrapping_sub(self.IOTA0, self.iConst110)) & 4095) as usize];
			self.fRec22[0] = self.fVec10[((i32::wrapping_sub(self.IOTA0, self.iConst111)) & 2047) as usize];
			self.fVec13[(self.IOTA0 & 32767) as usize] = 1e-20 + 0.35355338 * self.fRec20[0];
			let mut fTemp58: F32 = self.fVec13[((i32::wrapping_sub(self.IOTA0, self.iConst113)) & 32767) as usize] + fTemp54 + 0.6 * self.fRec23[1];
			self.fVec14[(self.IOTA0 & 4095) as usize] = fTemp58;
			self.fRec23[0] = self.fVec14[((i32::wrapping_sub(self.IOTA0, self.iConst114)) & 4095) as usize];
			self.fVec15[(self.IOTA0 & 32767) as usize] = 1e-20 + 0.35355338 * self.fRec14[0];
			let mut fTemp59: F32 = 0.6 * self.fRec24[1] + self.fVec15[((i32::wrapping_sub(self.IOTA0, self.iConst116)) & 32767) as usize];
			self.fVec16[(self.IOTA0 & 2047) as usize] = fTemp59 - fTemp54;
			self.fRec24[0] = self.fVec16[((i32::wrapping_sub(self.IOTA0, self.iConst117)) & 2047) as usize];
			self.fVec17[(self.IOTA0 & 16383) as usize] = 1e-20 + 0.35355338 * self.fRec16[0];
			let mut fTemp60: F32 = self.fVec17[((i32::wrapping_sub(self.IOTA0, self.iConst119)) & 16383) as usize] + fTemp54 + 0.6 * self.fRec31[1];
			self.fVec18[(self.IOTA0 & 4095) as usize] = fTemp60;
			let mut fRec25: F32 = -(0.6 * fTemp60);
			let mut fRec26: F32 = 0.6 * fTemp57;
			self.fVec19[(self.IOTA0 & 16383) as usize] = 1e-20 + 0.35355338 * self.fRec17[0];
			let mut fTemp61: F32 = self.fVec19[((i32::wrapping_sub(self.IOTA0, self.iConst121)) & 16383) as usize] - (fTemp52 + 0.6 * self.fRec27[1]);
			self.fVec20[(self.IOTA0 & 4095) as usize] = fTemp61;
			self.fRec27[0] = self.fVec20[((i32::wrapping_sub(self.IOTA0, self.iConst122)) & 4095) as usize];
			let mut fRec28: F32 = 0.6 * (fTemp54 - fTemp55);
			let mut fRec29: F32 = 0.6 * (fTemp54 - fTemp59);
			let mut fRec30: F32 = -(0.6 * fTemp58);
			self.fRec31[0] = self.fVec18[((i32::wrapping_sub(self.IOTA0, self.iConst123)) & 4095) as usize];
			let mut fRec32: F32 = 0.6 * fTemp61;
			let mut fTemp62: F32 = self.fRec12[1] + self.fRec22[1];
			self.fRec33[0] = fRec28 + fRec30 + fRec32 + fRec18 + self.fRec27[1] + self.fRec23[1] + fTemp62 - (fRec29 + fRec25 + fRec10 + fRec26 + self.fRec31[1] + self.fRec24[1] + self.fRec19[1] + self.fRec21[1]);
			let mut fTemp63: F32 = self.fRec19[1] + self.fRec12[1];
			self.fRec34[0] = fRec29 + fRec25 + fRec32 + fRec18 + self.fRec27[1] + self.fRec31[1] + self.fRec22[1] + self.fRec24[1] - (fRec28 + fRec30 + fRec10 + fRec26 + self.fRec23[1] + self.fRec21[1] + fTemp63);
			let mut fTemp64: F32 = self.fRec22[1] + fTemp63;
			self.fRec35[0] = fRec28 + fRec25 + fRec10 + fRec18 + self.fRec31[1] + fTemp64 - (fRec29 + fRec30 + fRec32 + fRec26 + self.fRec27[1] + self.fRec24[1] + self.fRec21[1] + self.fRec23[1]);
			let mut fTemp65: F32 = self.fRec19[1] + self.fRec22[1];
			self.fRec36[0] = fRec29 + fRec30 + fRec10 + fRec18 + self.fRec24[1] + self.fRec23[1] + fTemp65 - (fRec28 + fRec25 + fRec32 + fRec26 + self.fRec27[1] + self.fRec31[1] + self.fRec12[1] + self.fRec21[1]);
			self.fRec37[0] = fRec25 + fRec30 + fRec26 + fRec18 + self.fRec31[1] + self.fRec23[1] + self.fRec22[1] + self.fRec21[1] - (fRec29 + fRec28 + fRec10 + fRec32 + self.fRec27[1] + self.fRec24[1] + fTemp63);
			self.fRec38[0] = fRec29 + fRec28 + fRec26 + fRec18 + self.fRec24[1] + self.fRec21[1] + fTemp62 - (fRec25 + fRec30 + fRec10 + fRec32 + self.fRec27[1] + self.fRec31[1] + self.fRec19[1] + self.fRec23[1]);
			self.fRec39[0] = fRec10 + fRec32 + fRec26 + fRec18 + self.fRec27[1] + self.fRec21[1] + fTemp65 - (fRec29 + fRec28 + fRec25 + fRec30 + self.fRec31[1] + self.fRec24[1] + self.fRec12[1] + self.fRec23[1]);
			self.fRec40[0] = fRec29 + fRec28 + fRec25 + fRec30 + fRec10 + fRec32 + fRec26 + fRec18 + self.fRec27[1] + self.fRec31[1] + self.fRec24[1] + self.fRec23[1] + self.fRec21[1] + fTemp64;
			self.fRec60[0] = fSlow8 + self.fConst2 * self.fRec60[1];
			let mut fTemp66: F32 = 1.0 - self.fRec60[0];
			*output0 = (F32::tanh((0.9 * fTemp51 * fTemp66 + 0.37 * self.fRec60[0] * (self.fRec39[0] + self.fRec37[0])) * self.fRec0[0])) as FaustFloat;
			*output1 = (F32::tanh(self.fRec0[0] * (0.9 * (fTemp49 * fTemp66 / fTemp6) + 0.37 * self.fRec60[0] * (self.fRec39[0] - self.fRec37[0])))) as FaustFloat;
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
			self.fRec41[1] = self.fRec41[0];
			self.fRec42[1] = self.fRec42[0];
			self.fRec43[1] = self.fRec43[0];
			self.fRec48[1] = self.fRec48[0];
			self.iRec50[1] = self.iRec50[0];
			self.fVec1[1] = self.fVec1[0];
			self.fRec49[1] = self.fRec49[0];
			self.fRec51[1] = self.fRec51[0];
			self.fRec52[1] = self.fRec52[0];
			self.fRec53[1] = self.fRec53[0];
			self.fVec2[1] = self.fVec2[0];
			self.fRec54[1] = self.fRec54[0];
			self.fRec55[1] = self.fRec55[0];
			self.fRec56[1] = self.fRec56[0];
			self.fVec3[1] = self.fVec3[0];
			self.fRec59[1] = self.fRec59[0];
			self.fRec57[1] = self.fRec57[0];
			self.fRec58[1] = self.fRec58[0];
			self.fRec46[1] = self.fRec46[0];
			self.fRec47[1] = self.fRec47[0];
			self.fRec44[1] = self.fRec44[0];
			self.fRec45[1] = self.fRec45[0];
			self.IOTA0 = i32::wrapping_add(self.IOTA0, 1);
			self.fRec11[1] = self.fRec11[0];
			self.fRec12[1] = self.fRec12[0];
			self.fRec13[1] = self.fRec13[0];
			self.fRec14[1] = self.fRec14[0];
			self.fRec15[1] = self.fRec15[0];
			self.fRec16[1] = self.fRec16[0];
			self.fRec17[1] = self.fRec17[0];
			self.fRec19[1] = self.fRec19[0];
			self.fRec20[1] = self.fRec20[0];
			self.fRec21[1] = self.fRec21[0];
			self.fRec22[1] = self.fRec22[0];
			self.fRec23[1] = self.fRec23[0];
			self.fRec24[1] = self.fRec24[0];
			self.fRec27[1] = self.fRec27[0];
			self.fRec31[1] = self.fRec31[0];
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
			self.fRec60[1] = self.fRec60[0];
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
