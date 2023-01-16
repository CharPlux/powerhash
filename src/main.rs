
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
