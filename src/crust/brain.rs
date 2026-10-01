//! VEX V5 Brain, Screen (LCD), and SD card abstractions.

pub use crate::mantle::display::{Color, Display};
use std::sync::{Arc, Mutex};

/// Touch state on the V5 Brain LCD screen.
#[derive(Debug, Clone, Copy, Default)]
pub struct TouchState {
    pub pressing: bool,
    pub x: i32,
    pub y: i32,
}

/// Brain LCD Screen abstraction (480 x 272 pixels).
#[derive(Debug, Clone, Default)]
pub struct BrainScreen {
    touch: Arc<Mutex<TouchState>>,
    pen_color: Arc<Mutex<u32>>,
    fill_color: Arc<Mutex<u32>>,
}

impl BrainScreen {
    /// Creates a new BrainScreen instance.
    pub fn new() -> Self {
        Self {
            touch: Arc::new(Mutex::new(TouchState::default())),
            pen_color: Arc::new(Mutex::new(0xFFFFFF)),
            fill_color: Arc::new(Mutex::new(0x000000)),
        }
    }

    /// Returns whether the screen is currently being pressed.
    pub fn pressing(&self) -> bool {
        self.touch.lock().unwrap().pressing
    }

    /// Returns the X coordinate of the current touch (0..480).
    pub fn x_position(&self) -> i32 {
        self.touch.lock().unwrap().x
    }

    /// Returns the Y coordinate of the current touch (0..272).
    pub fn y_position(&self) -> i32 {
        self.touch.lock().unwrap().y
    }

    /// Sets the touch state (for simulation / testing).
    pub fn set_touch(&self, pressing: bool, x: i32, y: i32) {
        let mut t = self.touch.lock().unwrap();
        t.pressing = pressing;
        t.x = x;
        t.y = y;
    }

    /// Clears the screen.
    pub fn clear_screen(&self) {}

    /// Clears a specific row.
    pub fn clear_row(&self, _row: i32) {}

    /// Sets pen color (RGB hex or Color struct).
    pub fn set_pen_color<C: Into<u32>>(&self, color: C) {
        *self.pen_color.lock().unwrap() = color.into();
    }

    /// Sets fill color (RGB hex or Color struct).
    pub fn set_fill_color<C: Into<u32>>(&self, color: C) {
        *self.fill_color.lock().unwrap() = color.into();
    }

    /// Sets pen drawing width.
    pub fn set_pen_width(&self, _width: u32) {}

    /// Draws a filled or outlined rectangle.
    pub fn draw_rectangle(&self, _x: i32, _y: i32, _width: i32, _height: i32) {}

    /// Draws a filled or outlined circle.
    pub fn draw_circle(&self, _x: i32, _y: i32, _radius: i32) {}

    /// Draws a line between two points.
    pub fn draw_line(&self, _x1: i32, _y1: i32, _x2: i32, _y2: i32) {}

    /// Draws an image buffer onto the screen.
    pub fn draw_image_from_buffer(
        &self,
        _buffer: &[u32],
        _x: i32,
        _y: i32,
        _width: i32,
        _height: i32,
    ) {
    }

    /// Prints text at cursor position.
    pub fn print(&self, _s: &str) {}

    /// Prints formatted text at a specific coordinate.
    pub fn print_at(&self, _x: i32, _y: i32, _s: &str) {}

    /// Sets print cursor.
    pub fn set_cursor(&self, _row: i32, _col: i32) {}
}

impl Display for BrainScreen {
    fn pressing(&self) -> bool {
        self.pressing()
    }

    fn x_position(&self) -> i32 {
        self.x_position()
    }

    fn y_position(&self) -> i32 {
        self.y_position()
    }

    fn clear_screen(&mut self) {
        BrainScreen::clear_screen(self);
    }

    fn set_pen_color(&mut self, color: Color) {
        BrainScreen::set_pen_color(self, color);
    }

    fn set_fill_color(&mut self, color: Color) {
        BrainScreen::set_fill_color(self, color);
    }

    fn draw_rectangle(&mut self, x: i32, y: i32, width: i32, height: i32) {
        BrainScreen::draw_rectangle(self, x, y, width, height);
    }

    fn fill_rectangle(&mut self, x: i32, y: i32, width: i32, height: i32) {
        BrainScreen::draw_rectangle(self, x, y, width, height);
    }

    fn draw_circle(&mut self, x: i32, y: i32, radius: i32) {
        BrainScreen::draw_circle(self, x, y, radius);
    }

    fn fill_circle(&mut self, x: i32, y: i32, radius: i32) {
        BrainScreen::draw_circle(self, x, y, radius);
    }

    fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        BrainScreen::draw_line(self, x1, y1, x2, y2);
    }

    fn draw_pixel(&mut self, _x: i32, _y: i32) {}

    fn print_at(&mut self, x: i32, y: i32, text: &str) {
        BrainScreen::print_at(self, x, y, text);
    }

    fn render(&mut self) {}
}

/// VEX V5 Brain abstraction.
#[derive(Debug, Clone, Default)]
pub struct Brain {
    pub screen: BrainScreen,
}

impl Brain {
    /// Creates a new Brain instance.
    pub fn new() -> Self {
        Self {
            screen: BrainScreen::new(),
        }
    }
}
