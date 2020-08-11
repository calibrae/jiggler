use clap::{Clap};
use rand::Rng;
use enigo::*;
use std::{thread, time::Duration};


#[derive(Clap)]
#[clap(version = "1.0", author = "Nicolas Fargier Bousquet")]
struct Opts {
    /// Sets a custom config file. Could have been an Option<T> with no default too
    #[clap(short = "d", long = "discrete", about = "move mouse without moving cursor")]
    discrete: bool,
    /// Some input. Because this isn't an Option<T> it's required to be used
    #[clap(short = "t", long = "time", default_value = "300", about = "amount of time between mouse move in seconds")]
    time: u64,
    #[clap(short = "m", long = "delta", default_value = "1", about = "number of pixel the mouse will move, ignored if discrete is used")]
    delta : i32,
    #[clap(short = "r", long = "random", about = "random delta")]
    random : bool
}

fn main() {
    let opts: Opts = Opts::parse();

    println!("discrete {:0} time {:1} delta {:2}", opts.discrete, opts.time, opts.delta);

    let mut _enigo = Enigo::new();

    let mut delta: i32;
    if opts.discrete {
        delta = 0;
    } else {
        delta = opts.delta;
    }
    let mut rng = rand::thread_rng();

    let mut direction: bool = false;
    loop {
        if opts.random {
            delta = rng.gen_range(0,1000);
        }
        if direction {
            _enigo.mouse_move_relative(delta, delta);
        } else {
            _enigo.mouse_move_relative(-delta, -delta);
        }
        println!("Moved {:0} px", &delta);
        direction = !direction;
        thread::sleep(Duration::from_secs(opts.time));
    }
}
