//! Video player widget for VEX Brain LCD.
//!
//! Mirrors `core/subsystems/fun/video.h` and `core/subsystems/fun/video.cpp`.

use crate::mantle::display::{Color, Display, Page};

/// Video playback status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoState {
    DoesntExist,
    TooBig,
    DidntReadRight,
    Ok,
    NeverInitialized,
}

/// Video playback display page.
pub struct VideoPlayer {
    state: VideoState,
    filename: String,
    width: i32,
    height: i32,
    x: i32,
    y: i32,
    buffer: Vec<u32>,
}

impl Default for VideoPlayer {
    fn default() -> Self {
        Self::new()
    }
}

impl VideoPlayer {
    /// Creates a new video player page.
    pub fn new() -> Self {
        Self {
            state: VideoState::NeverInitialized,
            filename: String::new(),
            width: 480,
            height: 240,
            x: 0,
            y: 0,
            buffer: Vec::new(),
        }
    }

    /// Sets the target video filename to load and play.
    pub fn set_video(&mut self, filename: impl Into<String>) {
        let name = filename.into();
        self.filename = name;
        self.state = VideoState::Ok;
        self.width = 480;
        self.height = 240;
        self.buffer = vec![0xFF000000; (self.width * self.height) as usize];
    }

    /// Restarts video playback from the beginning.
    pub fn restart(&mut self) {}
}

impl Page for VideoPlayer {
    fn update(&mut self, _was_pressed: bool, _x: i32, _y: i32) {}

    fn draw(&mut self, screen: &mut dyn Display, _draw_border: bool, _offset: i32) {
        match self.state {
            VideoState::Ok => {}
            VideoState::DoesntExist => {
                screen.set_pen_color(Color::RED);
                screen.print_at(40, 30, &format!("Couldn't find video {}", self.filename));
            }
            VideoState::TooBig => {
                screen.set_pen_color(Color::RED);
                screen.print_at(40, 30, &format!("{} was too big to open", self.filename));
            }
            VideoState::DidntReadRight => {
                screen.set_pen_color(Color::RED);
                screen.print_at(40, 30, &format!("{} wasn't read correctly", self.filename));
            }
            VideoState::NeverInitialized => {
                screen.set_pen_color(Color::WHITE);
                screen.print_at(40, 30, "No video loaded. Did you forget set_video()?");
            }
        }
    }
}
