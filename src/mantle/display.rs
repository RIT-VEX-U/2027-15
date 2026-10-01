//! Display abstractions: Color, Display trait, GraphDrawer, ScreenController, and legacy widgets.

use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use crate::core::geometry::{Point2d, Rect, Translation2d};
use crate::core::time::Timer;
use crate::mantle::motor::Motor;

/// RGB color structure for drawing on graphical displays.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Color {
    pub rgb: u32,
}

impl Color {
    pub const WHITE: Color = Color { rgb: 0xFFFFFF };
    pub const BLACK: Color = Color { rgb: 0x000000 };
    pub const RED: Color = Color { rgb: 0xFF0000 };
    pub const GREEN: Color = Color { rgb: 0x00FF00 };
    pub const BLUE: Color = Color { rgb: 0x0000FF };
    pub const YELLOW: Color = Color { rgb: 0xFFFF00 };
    pub const CYAN: Color = Color { rgb: 0x00FFFF };
    pub const MAGENTA: Color = Color { rgb: 0xFF00FF };
    pub const ORANGE: Color = Color { rgb: 0xFFA500 };
    pub const PURPLE: Color = Color { rgb: 0x800080 };
    pub const TRANSPARENT: Color = Color { rgb: 0x00000000 };

    /// Creates an RGB color from 8-bit red, green, and blue components.
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self {
            rgb: ((r as u32) << 16) | ((g as u32) << 8) | (b as u32),
        }
    }
}

impl From<Color> for u32 {
    fn from(c: Color) -> Self {
        c.rgb
    }
}

impl From<u32> for Color {
    fn from(rgb: u32) -> Self {
        Color { rgb }
    }
}

/// Abstract 2D graphical display / LCD touch screen interface.
pub trait Display: Send + Sync + Debug {
    /// Returns true if the touch screen is currently pressed.
    fn pressing(&self) -> bool;

    /// Returns current touch X coordinate (0..480).
    fn x_position(&self) -> i32;

    /// Returns current touch Y coordinate (0..272).
    fn y_position(&self) -> i32;

    /// Clears the display screen.
    fn clear_screen(&mut self);

    /// Sets the pen/stroke color.
    fn set_pen_color(&mut self, color: Color);

    /// Sets the interior fill color.
    fn set_fill_color(&mut self, color: Color);

    /// Draws an outline rectangle.
    fn draw_rectangle(&mut self, x: i32, y: i32, width: i32, height: i32);

    /// Draws a filled rectangle.
    fn fill_rectangle(&mut self, x: i32, y: i32, width: i32, height: i32);

    /// Draws an outline circle.
    fn draw_circle(&mut self, x: i32, y: i32, radius: i32);

    /// Draws a filled circle.
    fn fill_circle(&mut self, x: i32, y: i32, radius: i32);

    /// Draws a straight line between two points.
    fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32);

    /// Draws a single pixel.
    fn draw_pixel(&mut self, x: i32, y: i32);

    /// Prints text at specified screen coordinates.
    fn print_at(&mut self, x: i32, y: i32, text: &str);

    /// Flushes render buffer if double-buffered.
    fn render(&mut self);
}

/// Simulated in-memory display for unit testing and headless execution.
#[derive(Debug, Clone, Default)]
pub struct MockDisplay {
    pub touch_pressed: bool,
    pub touch_x: i32,
    pub touch_y: i32,
    pub pen: Color,
    pub fill: Color,
}

impl MockDisplay {
    pub fn new() -> Self {
        Self {
            touch_pressed: false,
            touch_x: 0,
            touch_y: 0,
            pen: Color::WHITE,
            fill: Color::BLACK,
        }
    }
}

impl Display for MockDisplay {
    fn pressing(&self) -> bool { self.touch_pressed }
    fn x_position(&self) -> i32 { self.touch_x }
    fn y_position(&self) -> i32 { self.touch_y }
    fn clear_screen(&mut self) {}
    fn set_pen_color(&mut self, color: Color) { self.pen = color; }
    fn set_fill_color(&mut self, color: Color) { self.fill = color; }
    fn draw_rectangle(&mut self, _x: i32, _y: i32, _width: i32, _height: i32) {}
    fn fill_rectangle(&mut self, _x: i32, _y: i32, _width: i32, _height: i32) {}
    fn draw_circle(&mut self, _x: i32, _y: i32, _radius: i32) {}
    fn fill_circle(&mut self, _x: i32, _y: i32, _radius: i32) {}
    fn draw_line(&mut self, _x1: i32, _y1: i32, _x2: i32, _y2: i32) {}
    fn draw_pixel(&mut self, _x: i32, _y: i32) {}
    fn print_at(&mut self, _x: i32, _y: i32, _text: &str) {}
    fn render(&mut self) {}
}

/// Manages multi-series real-time data plotting on graphical displays.
#[derive(Debug, Clone)]
pub struct GraphDrawer {
    series: Vec<Vec<Translation2d>>,
    sample_index: usize,
    colors: Vec<Color>,
    bg_color: Color,
    border: bool,
    upper: f64,
    lower: f64,
    auto_fit: bool,
}

impl GraphDrawer {
    /// Creates a new graph drawer with specified parameters.
    pub fn new(
        num_samples: usize,
        lower_bound: f64,
        upper_bound: f64,
        colors: Vec<Color>,
        num_series: usize,
    ) -> Self {
        let auto_fit = (lower_bound - upper_bound).abs() < 1e-12;
        let mut series = Vec::with_capacity(num_series);
        for _ in 0..num_series {
            series.push(vec![Translation2d::new(0.0, 0.0); num_samples]);
        }

        let (upper, lower) = if auto_fit {
            (-1000.0, 1000.0)
        } else {
            (upper_bound, lower_bound)
        };

        Self {
            series,
            sample_index: 0,
            colors,
            bg_color: Color::TRANSPARENT,
            border: true,
            upper,
            lower,
            auto_fit,
        }
    }

    /// Adds a coordinate sample for each series.
    pub fn add_samples(&mut self, new_samples: &[Translation2d]) {
        for (i, sample) in new_samples.iter().enumerate() {
            if i < self.series.len() && !self.series[i].is_empty() {
                self.series[i][self.sample_index] = *sample;
            }
        }
        if !self.series.is_empty() && !self.series[0].is_empty() {
            self.sample_index = (self.sample_index + 1) % self.series[0].len();
        }
    }

    /// Draws the graph onto the provided display.
    pub fn draw(&mut self, screen: &mut dyn Display, rect: Rect) {
        if self.border {
            screen.set_pen_color(Color::WHITE);
            screen.draw_rectangle(rect.x as i32, rect.y as i32, rect.width as i32, rect.height as i32);
        }

        let n = if !self.series.is_empty() { self.series[0].len() } else { return; };
        if n < 2 { return; }

        for (s_idx, s) in self.series.iter().enumerate() {
            let color = self.colors.get(s_idx).copied().unwrap_or(Color::WHITE);
            screen.set_pen_color(color);

            for i in 0..(n - 1) {
                let p1 = s[i];
                let p2 = s[i + 1];

                let x1 = rect.x + (i as f64 / (n - 1) as f64) * rect.width;
                let x2 = rect.x + ((i + 1) as f64 / (n - 1) as f64) * rect.width;

                let range = (self.upper - self.lower).max(1e-6);
                let y1 = rect.y + rect.height - ((p1.y() - self.lower) / range) * rect.height;
                let y2 = rect.y + rect.height - ((p2.y() - self.lower) / range) * rect.height;

                screen.draw_line(x1 as i32, y1 as i32, x2 as i32, y2 as i32);
            }
        }
    }
}

/// Interactive button widget for touchscreen interfaces.
pub struct ButtonWidget {
    on_press: Box<dyn FnMut() + Send + Sync>,
    rect: Rect,
    name: String,
    was_pressed_last: bool,
}

impl ButtonWidget {
    pub fn new(rect: Rect, name: impl Into<String>, on_press: impl FnMut() + Send + Sync + 'static) -> Self {
        Self {
            on_press: Box::new(on_press),
            rect,
            name: name.into(),
            was_pressed_last: false,
        }
    }

    pub fn update(&mut self, was_pressed: bool, x: i32, y: i32) -> bool {
        let inside = self.rect.contains(x as f64, y as f64);
        let clicked = was_pressed && !self.was_pressed_last && inside;
        if clicked {
            (self.on_press)();
        }
        self.was_pressed_last = was_pressed && inside;
        clicked
    }

    pub fn draw(&self, screen: &mut dyn Display, _draw_border: bool, _offset: i32) {
        screen.set_pen_color(Color::WHITE);
        screen.set_fill_color(Color::new(40, 40, 40));
        screen.fill_rectangle(self.rect.x as i32, self.rect.y as i32, self.rect.width as i32, self.rect.height as i32);
        screen.draw_rectangle(self.rect.x as i32, self.rect.y as i32, self.rect.width as i32, self.rect.height as i32);
        screen.print_at(self.rect.x as i32 + 10, self.rect.y as i32 + 20, &self.name);
    }
}

/// A page that can be displayed on the screen.
pub trait Page: Send + Sync {
    fn update(&mut self, was_pressed: bool, x: i32, y: i32);
    fn draw(&mut self, screen: &mut dyn Display, draw_border: bool, offset: i32);
}

/// Diagnostic stats page displaying motor temperatures and battery stats.
pub struct StatsPage {
    motors: Vec<(&'static str, Arc<Mutex<Box<dyn Motor>>>)>,
}

impl StatsPage {
    pub fn new(motors: Vec<(&'static str, Arc<Mutex<Box<dyn Motor>>>)>) -> Self {
        Self { motors }
    }
}

impl Page for StatsPage {
    fn update(&mut self, _was_pressed: bool, _x: i32, _y: i32) {}

    fn draw(&mut self, screen: &mut dyn Display, _draw_border: bool, _offset: i32) {
        screen.set_pen_color(Color::WHITE);
        screen.print_at(10, 20, "Motor Diagnostics:");
        for (i, (name, m)) in self.motors.iter().enumerate() {
            let temp = if let Ok(lock) = m.lock() { lock.temperature() } else { 0.0 };
            let text = format!("{}: {:.1} C", name, temp);
            let y = 45 + (i as i32 * 20);
            screen.print_at(10, y, &text);
        }
    }
}

/// Tabbed multi-page display manager.
pub struct LegacyPage {
    pages: Vec<Box<dyn Page>>,
    current_page: usize,
    prev_btn: ButtonWidget,
    next_btn: ButtonWidget,
}

impl LegacyPage {
    pub fn new(pages: Vec<Box<dyn Page>>) -> Self {
        Self {
            pages,
            current_page: 0,
            prev_btn: ButtonWidget::new(Rect::new(10.0, 230.0, 80.0, 35.0), "< Prev", || {}),
            next_btn: ButtonWidget::new(Rect::new(390.0, 230.0, 80.0, 35.0), "Next >", || {}),
        }
    }

    pub fn update(&mut self, was_pressed: bool, x: i32, y: i32) {
        let prev_clicked = self.prev_btn.update(was_pressed, x, y);
        let next_clicked = self.next_btn.update(was_pressed, x, y);

        if prev_clicked && self.current_page > 0 {
            self.current_page -= 1;
        }
        if next_clicked && !self.pages.is_empty() && self.current_page + 1 < self.pages.len() {
            self.current_page += 1;
        }

        if let Some(page) = self.pages.get_mut(self.current_page) {
            page.update(was_pressed, x, y);
        }
    }

    pub fn draw(&mut self, screen: &mut dyn Display, draw_border: bool, offset: i32) {
        if let Some(page) = self.pages.get_mut(self.current_page) {
            page.draw(screen, draw_border, offset);
        }
        self.prev_btn.draw(screen, draw_border, offset);
        self.next_btn.draw(screen, draw_border, offset);
    }
}

/// Coordinates periodic screen updating and touch callbacks in a background thread.
pub struct ScreenController {
    handler: Arc<Mutex<Option<Box<dyn Fn() + Send + Sync>>>>,
    timer: Arc<Mutex<Timer>>,
    running: Arc<std::sync::atomic::AtomicBool>,
}

impl Default for ScreenController {
    fn default() -> Self {
        Self::new()
    }
}

impl ScreenController {
    pub fn new() -> Self {
        Self {
            handler: Arc::new(Mutex::new(None)),
            timer: Arc::new(Mutex::new(Timer::new())),
            running: Arc::new(std::sync::atomic::AtomicBool::new(true)),
        }
    }

    pub fn set(
        &mut self,
        handle: impl Fn() + Send + Sync + 'static,
        _press_fn: Option<Box<dyn Fn(Point2d) + Send + Sync>>,
        _release_fn: bool,
    ) {
        *self.handler.lock().unwrap() = Some(Box::new(handle));
    }

    pub fn update(&self) {
        if let Some(ref handler) = *self.handler.lock().unwrap() {
            handler();
        }
    }
}
