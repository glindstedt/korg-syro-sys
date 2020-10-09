use std::fs::File;
use std::io::BufWriter;
use std::mem;

use anyhow::Context;
use clap::{App, Arg};
use korg_syro_sys;
use wav;

fn handle_syro_status(
    status: korg_syro_sys::SyroStatus,
    handle: &korg_syro_sys::SyroHandle,
) -> anyhow::Result<()> {
    match status {
        korg_syro_sys::SyroStatus::Status_Success => Ok(()),
        // Status_IllegalDataType => {}
        // Status_IllegalData => {}
        // Status_IllegalParameter => {}
        // Status_OutOfRange_Number => {}
        // Status_OutOfRange_Quality => {}
        // Status_NotEnoughMemory => {}
        // Status_InvalidHandle => {}
        // Status_NoData => {}
        not_success => {
            let status = unsafe { korg_syro_sys::SyroVolcaSample_End(*handle) };
            Err(anyhow::anyhow!(
                "{:?}; end with status '{:?}'",
                not_success,
                status
            ))
        }
    }
}

fn main() -> anyhow::Result<()> {
    let matches = App::new("foobar")
        .arg(Arg::with_name("output").required(true).index(1))
        .arg(Arg::with_name("input").required(true).index(2))
        .arg(
            Arg::with_name("sample_index")
                .short("i")
                .long("sample_index")
                .takes_value(true)
                .value_name("index")
                .help("Index of the sample in the range [0-99] (Default 0)"),
        )
        .get_matches();
    let input_file = matches.value_of("input").unwrap();
    let output_file = matches.value_of("output").unwrap();
    let sample_index: u32 = matches
        .value_of("sample_index")
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(0);

    let mut file = File::open(input_file).with_context(|| "Cannot open input file")?;
    let (header, data) = wav::read(&mut file).with_context(|| "Cannot read input file")?;
    println!("Input: {:?}", header);

    let mut data_vec = match data {
        wav::BitDepth::Eight(_) => Err(anyhow::anyhow!("8 bit stream not supported yet")),
        wav::BitDepth::Sixteen(d) => Ok(d),
        wav::BitDepth::TwentyFour(_) => Err(anyhow::anyhow!("24 bit stream not supported yet")),
        wav::BitDepth::Empty => Err(anyhow::anyhow!("empty?? bit stream not supported yet")),
    }?;

    println!("Data length: {}", data_vec.len());
    println!("Sample index: {}", sample_index);

    let syro_data = korg_syro_sys::SyroData {
        DataType: korg_syro_sys::SyroDataType::DataType_Sample_Liner,
        pData: data_vec.as_mut_ptr() as *mut u8,
        // the sample (0-99) or sequence pattern (0-9) number
        Number: sample_index,
        // size of data to be converted (in bytes), Vec<i16> -> length * 2 bytes
        Size: 2 * data_vec.len() as u32,
        // The conversion bit depth. It can be set to 8-16. Seems unused when DataType = Sample_liner
        Quality: 0,
        Fs: header.sampling_rate,
        SampleEndian: korg_syro_sys::Endian::LittleEndian,
    };

    let mut num_frames = 0;

    // Array
    let num_data = 1;
    let mut data: Vec<korg_syro_sys::SyroData> = vec![syro_data];

    let handle: korg_syro_sys::SyroHandle = unsafe {
        let mut handle = mem::MaybeUninit::<korg_syro_sys::SyroHandle>::uninit();

        let status = korg_syro_sys::SyroVolcaSample_Start(
            handle.as_mut_ptr(),
            data.as_mut_ptr(),
            num_data,
            0,
            &mut num_frames,
        );
        println!("Init status: {:?}", status);
        handle.assume_init()
    };

    println!("frames: {:?}", num_frames);

    let mut left: i16 = 0;
    let mut right: i16 = 0;
    let mut buffer = Vec::with_capacity(num_frames as usize * 2);
    while num_frames > 0 {
        unsafe {
            #[allow(unused)]
            let status = korg_syro_sys::SyroVolcaSample_GetSample(handle, &mut left, &mut right);
            // TODO at index 54040 and all indexes following Status_NoData is returned
            // for kick.wav, seems to be the same with the example code in the C library,
            // but they ignore the status there.
            // handle_syro_status(status, &handle)
            //     .with_context(|| format!("GetSample: index = {}", num_frames))?;
        }
        buffer.push(left);
        buffer.push(right);
        num_frames -= 1;
    }
    println!("buffer length: {}", buffer.len());

    unsafe {
        let status = korg_syro_sys::SyroVolcaSample_End(handle);
        handle_syro_status(status, &handle)?;
    }
    let output_header = wav::Header::new(1, 2, 44100, 16);
    println!("Output: {:?}", output_header);

    let output = File::create(output_file)?;

    wav::write(
        output_header,
        wav::BitDepth::Sixteen(buffer),
        &mut BufWriter::new(output),
    )?;

    Ok(())
}
