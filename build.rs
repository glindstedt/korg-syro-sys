use std::env;
use std::path::PathBuf;

use bindgen;

const TYPES: [&'static str; 8] = [
    // korg_syro_func.h
    "SyroChannel",
    // korg_syro_volcasample.h
    "Endian",
    "SyroData",
    "SyroDataType",
    "SyroStatus",
    "SyroHandle",
    // volcasample_pattern.h
    "VolcaSample_Part_Data",
    "VolcaSample_Pattern_Data",
];

const FUNCTIONS: [&'static str; 13] = [
    // korg_syro_comp.h
    "SyroComp_Comp",
    "SyroComp_GetCompSize",
    // korg_syro_func.h
    "SyroFunc_CalculateCrc16",
    "SyroFunc_CalculateEcc",
    "SyroFunc_GenerateSingleCycle",
    "SyroFunc_MakeChannelInfo",
    "SyroFunc_MakeGap",
    "SyroFunc_MakeStartMark",
    "SyroFunc_SetTxSize",
    // korg_syro_volcasample.h
    "SyroVolcaSample_End",
    "SyroVolcaSample_GetSample",
    "SyroVolcaSample_Start",
    // volcasample_pattern.h
    "VolcaSample_Pattern_Init",
];

const CONSTANTS: [&'static str; 51] = [
    // korg_syro_comp.h
    "VOLCASAMPLE_COMP_BLOCK_LEN",
    "KORGSYRO_NUM_OF_CHANNEL",
    // korg_syro_func.h
    "KORGSYRO_QAM_CYCLE",
    "KORGSYRO_NUM_OF_CYCLE",
    "KORGSYRO_NUM_OF_CYCLE_BUF",
    // korg_syro_volcasample.h
    "SYRO_VOLCASAMPLE_VERSION",
    "VOLCASAMPLE_NUM_OF_SAMPLE",
    "VOLCASAMPLE_NUM_OF_PATTERN",
    "VOLCASAMPLE_PATTERN_SIZE",
    // volcasample_pattern.h
    "VOLCASAMPLE_NUM_OF_PART",
    "VOLCASAMPLE_NUM_OF_STEP",
    "VOLCASAMPLE_FUNC_BIT_NOTION",
    "VOLCASAMPLE_FUNC_BIT_LOOP",
    "VOLCASAMPLE_FUNC_BIT_REVERB",
    "VOLCASAMPLE_FUNC_BIT_REVERSE",
    "VOLCASAMPLE_FUNC_BIT_MUTE",
    "VOLCASAMPLE_FUNC_MOTION",
    "VOLCASAMPLE_FUNC_LOOP",
    "VOLCASAMPLE_FUNC_REVERB",
    "VOLCASAMPLE_FUNC_REVERSE",
    "VOLCASAMPLE_FUNC_MUTE",
    "VOLCASAMPLE_PARAM_LEVEL",
    "VOLCASAMPLE_PARAM_PAN",
    "VOLCASAMPLE_PARAM_SPEED",
    "VOLCASAMPLE_PARAM_AMPEG_ATTACK",
    "VOLCASAMPLE_PARAM_AMPEG_DECAY",
    "VOLCASAMPLE_PARAM_PITCHEG_INT",
    "VOLCASAMPLE_PARAM_PITCHEG_ATTACK",
    "VOLCASAMPLE_PARAM_PITCHEG_DECAY",
    "VOLCASAMPLE_PARAM_START_POINT",
    "VOLCASAMPLE_PARAM_LENGTH",
    "VOLCASAMPLE_PARAM_HICUT",
    "VOLCASAMPLE_NUM_OF_PARAM",
    "VOLCASAMPLE_MOTION_LEVEL_0",
    "VOLCASAMPLE_MOTION_LEVEL_1",
    "VOLCASAMPLE_MOTION_PAN_0",
    "VOLCASAMPLE_MOTION_PAN_1",
    "VOLCASAMPLE_MOTION_SPEED_0",
    "VOLCASAMPLE_MOTION_SPEED_1",
    "VOLCASAMPLE_MOTION_AMPEG_ATTACK",
    "VOLCASAMPLE_MOTION_AMPEG_DECAY",
    "VOLCASAMPLE_MOTION_PITCHEG_INT",
    "VOLCASAMPLE_MOTION_PITCHEG_ATTACK",
    "VOLCASAMPLE_MOTION_PITCHEG_DECAY",
    "VOLCASAMPLE_MOTION_START_POINT",
    "VOLCASAMPLE_MOTION_LENGTH",
    "VOLCASAMPLE_MOTION_HICUT",
    "VOLCASAMPLE_NUM_OF_MOTION",
    "VOLCASAMPLE_PATTERN_HEADER",
    "VOLCASAMPLE_PATTERN_FOOTER",
    "VOLCASAMPLE_PATTERN_DEVCODE",
];

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");

    cc::Build::new()
        .include("volcasample/syro")
        .file("volcasample/syro/korg_syro_volcasample.c")
        .file("volcasample/syro/korg_syro_comp.c")
        .file("volcasample/syro/korg_syro_func.c")
        .file("volcasample/pattern/volcasample_pattern.c")
        .compile("korg_syro");

    let mut builder = bindgen::Builder::default()
        .clang_arg("-Ivolcasample/syro")
        .header("wrapper.h")
        .rustified_enum("*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks));

    // Clean up exports using whitelisting
    for &t in TYPES.iter() {
        builder = builder.whitelist_type(t);
    }
    for &f in FUNCTIONS.iter() {
        builder = builder.whitelist_function(f);
    }
    for &c in CONSTANTS.iter() {
        builder = builder.whitelist_var(c);
    }

    let bindings = builder.generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}
