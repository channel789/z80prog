use std::path::PathBuf;

use clap::Parser;
use probe_rs::{Probe, Permissions, MemoryInterface};

#[derive(Parser)]
struct Opts {
    #[clap(short, long, default_value_t = 0)]
    offset: usize,
    data: PathBuf,
}

const RAM_ADDR: u64 = 0x20000000;
const RUN_ADDR: u64 = 0x20010000 - 4;

fn main() {
    let opts = Opts::parse();
    let data = std::fs::read(opts.data).unwrap();
    let probes = Probe::list_all();
    let probe = probes[0].open().unwrap();
    let mut session = probe.attach("RP2040", Permissions::default()).unwrap();
    let mut core = session.core(0).unwrap();
    core.write_word_32(RUN_ADDR, 0x00).unwrap();
    core.write_8(RAM_ADDR, &data).unwrap();
    core.write_word_32(RUN_ADDR, 0x10).unwrap();
}
