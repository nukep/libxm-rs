#![feature(os, collections, core, io, path)]

extern crate getopts;
extern crate libxm;
extern crate sdl2;

use getopts::Options;
use libxm::XMContext;
use sdl2::audio::{AudioCallback, AudioSpecDesired};
use std::old_io::File;
use std::os;
use std::sync::mpsc::Sender;

struct MyCallback {
    xm: XMContext,
    last_loop_count: u8,
    loop_tx: Sender<()>
}

impl AudioCallback<f32> for MyCallback {
    fn callback(&mut self, out: &mut [f32]) {
        self.xm.generate_samples(out);

        let loop_count = self.xm.get_loop_count();

        if loop_count != self.last_loop_count {
            self.last_loop_count = loop_count;

            // Signal the driver that a loop has occured
            self.loop_tx.send(()).unwrap();
        }
    }
}

fn play_audio(xm: XMContext, rate: u32, max_loops: u8) {
    use std::sync::mpsc::channel;

    sdl2::init(sdl2::INIT_AUDIO);

    let (loop_tx, loop_rx) = channel();

    let desired_spec = AudioSpecDesired {
        freq: rate as i32,
        channels: 2,
        samples: 0,
        callback: MyCallback {
            xm: xm,
            last_loop_count: 0,
            loop_tx: loop_tx
        }
    };

    let device = desired_spec.open_audio_device(None, false).unwrap();

    device.resume();

    for _ in 0..max_loops {
        // Block until the song has looped
        loop_rx.recv().unwrap();
    }
}


fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] FILE", program);
    print!("{}", opts.usage(brief.as_slice()));
}

fn main() {

    let args = os::args();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("r", "rate", "Set the output rate", "RATE");
    opts.optopt("l", "loops", "Set the maximum number of loops", "LOOPS");

    let matches = match opts.parse(args.tail()) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(program.as_slice(), opts);
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

    let contents = File::open(&Path::new(input)).read_to_end().unwrap();

    let mut xm = XMContext::new(contents.as_slice(), rate).unwrap();
    xm.set_max_loop_count(max_loops);

    println!("Module name: {}", String::from_utf8_lossy(xm.get_module_name()));
    println!("Tracker: {}", String::from_utf8_lossy(xm.get_tracker_name()));
    println!("Channels: {}", xm.get_number_of_channels());
    println!("Module length: {}", xm.get_module_length());
    println!("Patterns: {}", xm.get_number_of_patterns());
    println!("Instruments: {}", xm.get_number_of_instruments());

    play_audio(xm, rate, max_loops);
}
