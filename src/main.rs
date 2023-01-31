
// copyright 2017 Kaz Wesley

use std::fs::File;
use std::io::BufRead;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use cn_stratum::client::{
    ErrorReply, Job, JobAssignment, MessageHandler, PoolClient, PoolClientWriter, RequestId,
};
use yellowsun::{Algo, AllocPolicy, Hasher};

use byteorder::{ByteOrder, LE};
use core_affinity::CoreId;
use log::*;
use serde_derive::{Deserialize, Serialize};

const AGENT: &str = "pow#er/0.2.0";

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClientConfig {
    pub address: String,
    pub login: String,
    pub pass: String,
    pub keepalive_s: Option<u64>,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(deny_unknown_fields)]
struct Config {
    pub pool: ClientConfig,
    pub cores: Vec<u32>,
}

fn main() {
    env_logger::init();

    let panicker = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        eprintln!("panicked");
        panicker(info);
        std::process::exit(1);
    }));

    let args = clap::App::new("Pow#er")
        .author("Kaz Wesley <kaz@lambdaverse.org>")
        .arg(
            clap::Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .required(true)
                .takes_value(true),
        ).arg(
            clap::Arg::with_name("allow-slow-mem")
                .long("allow-slow-mem")
                .help("Continue even if hugepages are not available (SLOW!)"),
        ).get_matches();

    let cfg: Config = File::open(args.value_of("config").unwrap())
        .map(serde_json::from_reader)