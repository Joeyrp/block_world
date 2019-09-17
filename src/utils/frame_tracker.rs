
use std::{time::Instant};
use crate::utils;


pub struct FrameTracker
{
    start_time: Instant,
    prev_time: Instant,
    frame_counter: u32,
    second_counter: u128,
    total_frames: u128,
    delta_time: u128,
    current_fps: u32,
}

impl FrameTracker
{
    /// Create and initialize a new FrameTracker
    pub fn new() -> FrameTracker
    {
        let now = Instant::now();
        FrameTracker { 
            start_time: now, 
            prev_time: now, 
            frame_counter: 0, 
            second_counter: 0, 
            total_frames: 0,
            delta_time: 0,
            current_fps: 0,
            }
    }

    /// This method must be called every frame in order to keep the
    /// tracker updated correctly. Call this method ONCE at the beginning
    /// of your frame loop.
    pub fn update(self: &mut FrameTracker)
    {
        let now = Instant::now();
        self.delta_time = (now - self.prev_time).as_micros();
        self.prev_time = now;

        self.total_frames += 1;
        self.frame_counter += 1;
        self.second_counter += self.delta_time;

        if self.second_counter >= utils::ONE_MILLION
        {
            self.second_counter = 0;
            self.current_fps = self.frame_counter;
            self.frame_counter = 0;
        }
    }

    /// Returns the total elapsed time (in microseconds) since the
    /// FrameTracker was created.
    pub fn get_elapsed_time(self: &mut FrameTracker) -> u128
    {
        (Instant::now() - self.start_time).as_micros()
    }

    /// Returns the current frames per second. This value only
    /// updates once every second.
    pub fn get_current_fps(self: &mut FrameTracker) -> u32
    {
        self.current_fps
    }

    /// Returns the delta time (the time between update calls) as 
    /// microseconds. 
    /// 
    /// If you call update once at the start of every frame loop then 
    /// this will tell you how long your previous frame ran for.
    pub fn get_delta_time(self: &mut FrameTracker) -> u128
    {
        self.delta_time
    }

    /// Returns the average time between update calls.
    /// 
    /// If you call update once at the start of each frame loop
    /// this will tell you on average how long each frame loop is taking.
    pub fn get_average_frame_time(self: &FrameTracker) -> u128
    {
        (Instant::now() - self.start_time).as_micros() / self.total_frames
    }
}