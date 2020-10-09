use std::env;
use std::path::PathBuf;

use bindgen;

const TYPES: [&'static str; 6] = [
    // korg_syro_func.h
    "SyroChannel",
    // korg_syro_volcasample.h
    "Endian",
    "SyroData",
    "SyroDataType",
    "SyroStatus",
    "SyroHandle",
];

const FUNCTIONS: [&'static str; 12] = [
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
];

const CONSTANTS: [&'static str; 9] = [
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
];

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");

    cc::Build::new()
        .file("volcasample/syro/korg_syro_volcasample.c")
        .file("volcasample/syro/korg_syro_comp.c")
        .file("volcasample/syro/korg_syro_func.c")
        .compile("korg_syro");

    let mut builder = bindgen::Builder::default()
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
