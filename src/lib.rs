//!
//! Rust FFI bindings for the [KORG SYRO](https://github.com/korginc/volcasample) library for the Volca Sample.
//!
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

impl Default for VolcaSample_Part_Data {
    fn default() -> Self {
        Self {
            SampleNum: 0, // the sample number (0~99) to be used
            StepOn: 0, // a bit map to set step on/off steps 1-16 correspond to bits 0-15, 0=off and 1=on
            Accent: 0, // this parameter cannot be operated on the volca sample. Must be set to 0.
            Reserved: 0,
            Level: 127, // this parameter cannot be operated on the volca sample. Must be set to 127.
            Param: [
                127, // VOLCASAMPLE_PARAM_LEVEL 0~127, (127)
                64,  // VOLCASAMPLE_PARAM_PAN   1~127, 64=Center (64)
                64, // VOLCASAMPLE_PARAM_SPEED 40~88, 64=Center (64) *changes speed in semitones (FUNC+SPEED operation)
                //                             129~255, 192=Centre   *changes speed continuously
                0,   // VOLCASAMPLE_PARAM_AMPEG_ATTACK   0~127 (0)
                127, // VOLCASAMPLE_PARAM_AMPEG_DECAY    0~127 (127)
                64,  // VOLCASAMPLE_PARAM_PITCHEG_INT    1~127, 64=Center (64)
                0,   // VOLCASAMPLE_PARAM_PITCHEG_ATTACK 0~127 (0)
                127, // VOLCASAMPLE_PARAM_PITCHEG_DECAY  0~127 (127)
                0,   // VOLCASAMPLE_PARAM_START_POINT    0~127 (0)
                127, // VOLCASAMPLE_PARAM_LENGTH         0~127 (127)
                127, // VOLCASAMPLE_PARAM_HICUT          0~127 (127)
            ],
            // on/off for each part parameters
            //     bit0 : Motion On/Off   VOLCASAMPLE_FUNC_MOTION
            //     bit1 : Loop On/Off     VOLCASAMPLE_FUNC_LOOP
            //     bit2 : Reverb On/Off   VOLCASAMPLE_FUNC_REVERB
            //     bit3 : Reverse On/Off  VOLCASAMPLE_FUNC_REVERSE
            //     bit4 : Mute On/Off (1=mute off) VOLCASAMPLE_FUNC_MUTE
            FuncMemoryPart: VOLCASAMPLE_FUNC_MUTE as u8,
            Padding1: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            Motion: [
                // The values for the motion sequence are as follows:
                // SPEED                   knob value (0~127)
                // all other parameters    knob value +128
                // for all parameters, 0=no motion data
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // LEVEL (start)
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // LEVEL (end)
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // PAN (start)
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // PAN (end)
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // SPEED (start)
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // SPEED (end)
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // AMP EG ATTACk
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // AMP EG DECAY
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // PITCH EG INT
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // PITCH EG ATTACK
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // PITCH EG DECAY
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // START POINT
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // LENGTH
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // HI CUT
            ],
        }
    }
}

impl Default for VolcaSample_Pattern_Data {
    fn default() -> Self {
        Self {
            Header: VOLCASAMPLE_PATTERN_HEADER,
            DevCode: VOLCASAMPLE_PATTERN_DEVCODE as u16,
            Reserved: [0, 0],
            ActiveStep: 0xffff,
            Padding1: [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            Part: [
                VolcaSample_Part_Data::default(),
                VolcaSample_Part_Data::default(),
                VolcaSample_Part_Data::default(),
                VolcaSample_Part_Data::default(),
                VolcaSample_Part_Data::default(),
                VolcaSample_Part_Data::default(),
                VolcaSample_Part_Data::default(),
                VolcaSample_Part_Data::default(),
                VolcaSample_Part_Data::default(),
                VolcaSample_Part_Data::default(),
            ],
            Padding2: [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            Footer: VOLCASAMPLE_PATTERN_FOOTER,
        }
    }
}
