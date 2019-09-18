pub struct FpsCounter {
    frame_count: i32,
    accum_time: f32,
}

impl FpsCounter {
    pub fn new() -> FpsCounter {
        FpsCounter {
            frame_count: 0,
            accum_time: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) -> Option<String> {
        self.frame_count += 1;
        self.accum_time += dt;

        if self.accum_time > 1.0 {
            let ret = format!("FPS: {}", self.frame_count);
            self.frame_count = 0;
            self.accum_time -= 1.0;
            return Some(ret);
        }
        None
    }
}
