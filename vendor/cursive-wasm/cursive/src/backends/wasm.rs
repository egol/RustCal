#![cfg(feature = "wasm-backend")]

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::backend;
use cursive_core::event::{Event, Key, MouseButton, MouseEvent};
use cursive_core::{theme, Vec2};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, KeyboardEvent, MouseEvent as WebMouseEvent};
use cursive_core::backend::Backend as _;

const DEFAULT_FONT_WIDTH: usize = 10;
const DEFAULT_FONT_HEIGHT: usize = 18;

/// Backend using wasm.
pub struct Backend {
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
    color: RefCell<ColorPair>,
    font_height: usize,
    font_width: usize,
    logical_width: usize,
    logical_height: usize,
    events: Rc<RefCell<VecDeque<Event>>>,
}

impl Backend {
    /// Creates a new Cursive root using a wasm backend.
    pub fn init() -> std::io::Result<Box<dyn backend::Backend>> {
        let document = web_sys::window()
            .ok_or(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to get window",
            ))?
            .document()
            .ok_or(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to get document",
            ))?;

        let canvas = document
            .get_element_by_id("cursive-wasm-canvas")
            .ok_or(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to find canvas with id cursive-wasm-canvas",
            ))?
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| {
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Failed to cast canvas element",
                )
            })?;

        let logical_width = if canvas.width() == 0 {
            1080
        } else {
            canvas.width()
        } as usize;
        let logical_height = if canvas.height() == 0 {
            920
        } else {
            canvas.height()
        } as usize;
        let dpr = web_sys::window()
            .map(|window| window.device_pixel_ratio())
            .unwrap_or(1.0)
            .max(1.0);

        canvas.set_width((logical_width as f64 * dpr).round() as u32);
        canvas.set_height((logical_height as f64 * dpr).round() as u32);
        let ctx: CanvasRenderingContext2d = canvas
            .get_context("2d")
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Failed to get canvas context"))?
            .ok_or(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to get canvas context",
            ))?
            .dyn_into::<CanvasRenderingContext2d>()
            .map_err(|_| {
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Failed to cast canvas context",
                )
            })?;

        let _ = ctx.scale(dpr, dpr);
        ctx.set_image_smoothing_enabled(false);
        ctx.set_font(&format!("{}px monospace", DEFAULT_FONT_HEIGHT));
        ctx.set_text_baseline("top");

        let color = RefCell::new(cursive_to_color_pair(theme::ColorPair {
            front: theme::Color::Dark(theme::BaseColor::White),
            back: theme::Color::Dark(theme::BaseColor::Black),
        }));
        let events = Rc::new(RefCell::new(VecDeque::new()));

        canvas
            .set_attribute("tabindex", "0")
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Failed to set tabindex"))?;

        register_keyboard_events(&canvas, Rc::clone(&events))?;
        register_mouse_events(&canvas, Rc::clone(&events), DEFAULT_FONT_WIDTH, DEFAULT_FONT_HEIGHT)?;
        register_resize_events(Rc::clone(&events))?;

        let backend = Backend {
            canvas,
            ctx,
            color,
            font_height: DEFAULT_FONT_HEIGHT,
            font_width: DEFAULT_FONT_WIDTH,
            logical_width,
            logical_height,
            events,
        };

        backend.clear(theme::Color::Dark(theme::BaseColor::Black));
        Ok(Box::new(backend))
    }
}

impl cursive_core::backend::Backend for Backend {
    fn poll_event(&mut self) -> Option<Event> {
        self.events.borrow_mut().pop_front()
    }

    fn set_title(&mut self, title: String) {
        self.canvas.set_title(&title);
    }

    fn refresh(&mut self) {}

    fn has_colors(&self) -> bool {
        true
    }

    fn screen_size(&self) -> Vec2 {
        Vec2::new(
            self.logical_width / self.font_width,
            self.logical_height / self.font_height,
        )
    }

    fn print_at(&self, pos: Vec2, text: &str) {
        let color = self.color.borrow();
        let x = (pos.x * self.font_width) as f64;
        let y = (pos.y * self.font_height) as f64;
        let width = (self.font_width * text.chars().count()) as f64;
        let height = self.font_height as f64;

        self.ctx.set_fill_style(&JsValue::from_str(&color.back));
        self.ctx.fill_rect(x, y, width.max(self.font_width as f64), height);
        self.ctx.set_fill_style(&JsValue::from_str(&color.front));
        let _ = self.ctx.fill_text(text, x, y);
    }

    fn clear(&self, color: cursive_core::theme::Color) {
        self.ctx
            .set_fill_style(&JsValue::from_str(&cursive_to_color(color)));
        self.ctx.fill_rect(
            0.0,
            0.0,
            self.logical_width as f64,
            self.logical_height as f64,
        );
    }

    fn set_color(
        &self,
        color_pair: cursive_core::theme::ColorPair,
    ) -> cursive_core::theme::ColorPair {
        *self.color.borrow_mut() = cursive_to_color_pair(color_pair);
        color_pair
    }

    fn set_effect(&self, _: cursive_core::theme::Effect) {}

    fn unset_effect(&self, _: cursive_core::theme::Effect) {}

    fn name(&self) -> &str {
        "cursive-wasm-backend"
    }
}

fn register_keyboard_events(
    canvas: &HtmlCanvasElement,
    events: Rc<RefCell<VecDeque<Event>>>,
) -> std::io::Result<()> {
    let closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
        if let Some(mapped) = map_keyboard_event(&event) {
            event.prevent_default();
            events.borrow_mut().push_back(mapped);
        }
    }) as Box<dyn FnMut(_)>);

    canvas
        .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Failed to add keydown listener"))?;
    closure.forget();
    Ok(())
}

fn register_mouse_events(
    canvas: &HtmlCanvasElement,
    events: Rc<RefCell<VecDeque<Event>>>,
    font_width: usize,
    font_height: usize,
) -> std::io::Result<()> {
    let press_events = Rc::clone(&events);
    let press_canvas = canvas.clone();
    let press = Closure::wrap(Box::new(move |event: WebMouseEvent| {
        event.prevent_default();
        let _ = press_canvas.focus();
        if let Some(button) = map_mouse_button(event.button()) {
            press_events.borrow_mut().push_back(Event::Mouse {
                offset: Vec2::new(0, 0),
                position: mouse_position(&event, font_width, font_height),
                event: MouseEvent::Press(button),
            });
        }
    }) as Box<dyn FnMut(_)>);
    canvas
        .add_event_listener_with_callback("mousedown", press.as_ref().unchecked_ref())
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Failed to add mousedown listener"))?;
    press.forget();

    let release_events = Rc::clone(&events);
    let release = Closure::wrap(Box::new(move |event: WebMouseEvent| {
        event.prevent_default();
        if let Some(button) = map_mouse_button(event.button()) {
            release_events.borrow_mut().push_back(Event::Mouse {
                offset: Vec2::new(0, 0),
                position: mouse_position(&event, font_width, font_height),
                event: MouseEvent::Release(button),
            });
        }
    }) as Box<dyn FnMut(_)>);
    canvas
        .add_event_listener_with_callback("mouseup", release.as_ref().unchecked_ref())
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Failed to add mouseup listener"))?;
    release.forget();

    let context = Closure::wrap(Box::new(move |event: WebMouseEvent| {
        event.prevent_default();
    }) as Box<dyn FnMut(_)>);
    canvas
        .add_event_listener_with_callback("contextmenu", context.as_ref().unchecked_ref())
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Failed to add contextmenu listener"))?;
    context.forget();

    Ok(())
}

fn register_resize_events(events: Rc<RefCell<VecDeque<Event>>>) -> std::io::Result<()> {
    let window = web_sys::window()
        .ok_or(std::io::Error::new(std::io::ErrorKind::Other, "Failed to get window"))?;

    let resize = Closure::wrap(Box::new(move || {
        events.borrow_mut().push_back(Event::WindowResize);
    }) as Box<dyn FnMut()>);

    window
        .add_event_listener_with_callback("resize", resize.as_ref().unchecked_ref())
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Failed to add resize listener"))?;
    resize.forget();
    Ok(())
}

fn map_keyboard_event(event: &KeyboardEvent) -> Option<Event> {
    let key = match event.key().as_str() {
        "ArrowLeft" => return Some(modified_key_event(event, Key::Left)),
        "ArrowRight" => return Some(modified_key_event(event, Key::Right)),
        "ArrowUp" => return Some(modified_key_event(event, Key::Up)),
        "ArrowDown" => return Some(modified_key_event(event, Key::Down)),
        "Enter" => return Some(modified_key_event(event, Key::Enter)),
        "Tab" => return Some(modified_key_event(event, Key::Tab)),
        "Backspace" => return Some(modified_key_event(event, Key::Backspace)),
        "Escape" => return Some(modified_key_event(event, Key::Esc)),
        _ => event.key(),
    };

    if key.chars().count() == 1 {
        let character = key.chars().next().unwrap();
        if event.ctrl_key() {
            Some(Event::CtrlChar(character))
        } else if event.alt_key() {
            Some(Event::AltChar(character))
        } else {
            Some(Event::Char(character))
        }
    } else {
        None
    }
}

fn modified_key_event(event: &KeyboardEvent, key: Key) -> Event {
    if event.ctrl_key() && event.shift_key() {
        Event::CtrlShift(key)
    } else if event.ctrl_key() && event.alt_key() {
        Event::CtrlAlt(key)
    } else if event.ctrl_key() {
        Event::Ctrl(key)
    } else if event.alt_key() && event.shift_key() {
        Event::AltShift(key)
    } else if event.alt_key() {
        Event::Alt(key)
    } else if event.shift_key() {
        Event::Shift(key)
    } else {
        Event::Key(key)
    }
}

fn mouse_position(
    event: &WebMouseEvent,
    font_width: usize,
    font_height: usize,
) -> Vec2 {
    let x = (event.offset_x() as f64 / font_width as f64).floor();
    let y = (event.offset_y() as f64 / font_height as f64).floor();
    Vec2::new(x.max(0.0) as usize, y.max(0.0) as usize)
}

fn map_mouse_button(button: i16) -> Option<MouseButton> {
    match button {
        0 => Some(MouseButton::Left),
        1 => Some(MouseButton::Middle),
        2 => Some(MouseButton::Right),
        3 => Some(MouseButton::Button4),
        4 => Some(MouseButton::Button5),
        _ => Some(MouseButton::Other),
    }
}

/// Type of hex color which starts with #.
pub type Color = String;

/// Type of color pair.
pub struct ColorPair {
    /// Foreground text color.
    pub front: Color,
    /// Background color.
    pub back: Color,
}

/// Convert cursive color to hex color.
pub fn cursive_to_color(color: theme::Color) -> Color {
    match color {
        theme::Color::Dark(theme::BaseColor::Black) => "#111827".to_string(),
        theme::Color::Dark(theme::BaseColor::Red) => "#7f1d1d".to_string(),
        theme::Color::Dark(theme::BaseColor::Green) => "#14532d".to_string(),
        theme::Color::Dark(theme::BaseColor::Yellow) => "#78350f".to_string(),
        theme::Color::Dark(theme::BaseColor::Blue) => "#1e3a8a".to_string(),
        theme::Color::Dark(theme::BaseColor::Magenta) => "#581c87".to_string(),
        theme::Color::Dark(theme::BaseColor::Cyan) => "#164e63".to_string(),
        theme::Color::Dark(theme::BaseColor::White) => "#d6d3d1".to_string(),
        theme::Color::Light(theme::BaseColor::Black) => "#1f2937".to_string(),
        theme::Color::Light(theme::BaseColor::Red) => "#ef4444".to_string(),
        theme::Color::Light(theme::BaseColor::Green) => "#22c55e".to_string(),
        theme::Color::Light(theme::BaseColor::Yellow) => "#facc15".to_string(),
        theme::Color::Light(theme::BaseColor::Blue) => "#60a5fa".to_string(),
        theme::Color::Light(theme::BaseColor::Magenta) => "#c084fc".to_string(),
        theme::Color::Light(theme::BaseColor::Cyan) => "#22d3ee".to_string(),
        theme::Color::Light(theme::BaseColor::White) => "#fafaf9".to_string(),
        theme::Color::Rgb(r, g, b) => format!("#{:02x}{:02x}{:02x}", r, g, b),
        theme::Color::RgbLowRes(r, g, b) => format!(
            "#{:02x}{:02x}{:02x}",
            r.saturating_mul(51),
            g.saturating_mul(51),
            b.saturating_mul(51)
        ),
        theme::Color::TerminalDefault => "#ece7de".to_string(),
    }
}

/// Convert cursive color pair to hex color pair.
pub fn cursive_to_color_pair(c: theme::ColorPair) -> ColorPair {
    ColorPair {
        front: cursive_to_color(c.front),
        back: cursive_to_color(c.back),
    }
}
