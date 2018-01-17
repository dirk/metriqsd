#[macro_use]
extern crate serde_derive;

extern crate metriqs;
extern crate serde;
extern crate toml;

use std::fs::File;
use std::io::Read;

mod builder;
mod config;
mod runner;
mod util;

use self::config::Config;
use self::builder::RootBuilder;
use self::runner::Runnable;

fn main() {
    let mut file = File::open("config.toml").unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let config: Config = toml::from_str(&contents).unwrap();
    // println!("{:#?}", config);

    let builder: RootBuilder = config.into();
    println!("{:#?}", builder);

    let mut runner = builder.build();
    println!("{:#?}", runner);

    runner.run();
}
