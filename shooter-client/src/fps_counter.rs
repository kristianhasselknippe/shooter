pub struct FpsCounter {
    frame_count: i32,
    accum_time: f64,
}

impl FpsCounter {
    pub fn new() -> FpsCounter {
        FpsCounter {
            frame_count: 0,
            accum_time: 0.0,
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.frame_count += 1;
        self.accum_time += dt;

        if self.accum_time > 1.0 {
            println!("FPS: {}", self.frame_count);
            self.frame_count = 0;
            self.accum_time -= 1.0;
        }
    }
}
