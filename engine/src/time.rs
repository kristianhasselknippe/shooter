use std::thread::sleep;
use std::time::Duration;
use t::precise_time_s;

pub struct Time {
    target_fps: i32,
    epoc: f64,

    last_time: f64,
}

const MILLIS_IN_60_FPS: f64 = 16.66;

impl Time {
    pub fn new(target_fps: i32) -> Time {
        let e = precise_time_s();
        Time {
            target_fps: target_fps,

            epoc: e,
            last_time: e,
        }
    }

    pub fn delta_time(&mut self) -> f64 {
        let t = precise_time_s();
        let diff = t - self.last_time;
        self.last_time = t;
        diff
    }

    pub fn wait_until_frame_target(&self) {
        let t = precise_time_s();
        let dt = t - self.last_time;

        let diff = MILLIS_IN_60_FPS - dt;

        if diff > 0.0 {
            sleep(Duration::from_millis(diff as u64));
        }
    }
}
