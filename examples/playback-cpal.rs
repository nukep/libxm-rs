use getopts::Options;
use libxm::XMContext;
use std::fs::File;
use std::io::Read;
use std::env;
use std::sync::mpsc::Sender;

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    StreamConfig,
};

struct MyCallback {
    xm: XMContext,
    last_loop_count: u8,
    loop_tx: Sender<()>
}

impl MyCallback {
    fn callback(&mut self, out: &mut [f32]) {
        self.xm.generate_samples(out);

        let loop_count = self.xm.loop_count();

        if loop_count != self.last_loop_count {
            self.last_loop_count = loop_count;

            // Signal the driver that a loop has occured
            self.loop_tx.send(()).unwrap();
        }
    }
}

fn play_audio(contents: &[u8], rate: u32, max_loops: u8) {
    use std::sync::mpsc::channel;

    let (loop_tx, loop_rx) = channel();

    let mut xm = XMContext::new(&contents, rate).unwrap();
    xm.set_max_loop_count(max_loops);

    if let Some(module_name) = xm.module_name() {
        println!("Module name: {}", String::from_utf8_lossy(module_name));
    }
    if let Some(tracker_name) = xm.tracker_name() {
        println!("Tracker: {}", String::from_utf8_lossy(tracker_name));
    }
    println!("Channels: {}", xm.number_of_channels());
    println!("Module length: {}", xm.module_length());
    println!("Patterns: {}", xm.number_of_patterns());
    println!("Instruments: {}", xm.number_of_instruments());

    let host = cpal::default_host();
    let device = host.default_output_device().unwrap();
    let config = StreamConfig {
        channels: 2,
        sample_rate: cpal::SampleRate(rate),
        buffer_size: cpal::BufferSize::Default
    };

    let mut my_callback = MyCallback {
        xm,
        last_loop_count: 0,
        loop_tx
    };

    let cb = move |output: &mut [f32], _: &cpal::OutputCallbackInfo| {
        my_callback.callback(output);
    };
    let ecb = move |err| {
        println!("ERROR: {:?}", err);
    };

    let stream = device.build_output_stream(&config, cb, ecb, None).unwrap();
    stream.play().unwrap();

    for _ in 0..max_loops {
        // Block until the song has looped
        loop_rx.recv().unwrap();
    }
}


fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] FILE", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("r", "rate", "Set the output rate", "RATE");
    opts.optopt("l", "loops", "Set the maximum number of loops", "LOOPS");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!("{:?}", f) }
    };

    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };

    let rate = match matches.opt_str("r") {
        Some(s) => s.parse().unwrap(),
        None => 48000
    };

    let max_loops = match matches.opt_str("l") {
        Some(s) => s.parse().unwrap(),
        None => 1
    };

    let mut contents = Vec::new();
    File::open(&input).unwrap().read_to_end(&mut contents).unwrap();

    play_audio(&contents, rate, max_loops);
}
