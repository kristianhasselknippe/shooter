#![allow(dead_code)]
use std::thread::sleep;
use std::time::Duration;
use std::time::{UNIX_EPOCH, SystemTime};

pub struct Time {
    epoc: f64,

    last_time: f64,
}

const MILLIS_IN_60_FPS: f64 = 16.66;

impl Time {
    pub fn new() -> Time {
        let e = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64();
        Time {
            epoc: e,
            last_time: e,
        }
    }

    pub fn delta_time(&mut self) -> f64 {
        let t = Time::new().epoc;
        let diff = t - self.last_time;
        self.last_time = t;
        diff
    }

    pub fn wait_until_frame_target(&self) {
        let t =  Time::new().epoc;
        let dt = t - self.last_time;

        let diff = MILLIS_IN_60_FPS - dt;

        if diff > 0.0 {
            sleep(Duration::from_millis(diff as u64));
        }
    }
}
