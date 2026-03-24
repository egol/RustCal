#![cfg(feature = "wasm-backend")]

use std::cell::RefCell;
use std::collections::VecDeque;
use std::io::{Error, ErrorKind};
use std::rc::Rc;

use crate::backend;
use cursive_core::event::{Event, Key, MouseButton, MouseEvent};
use cursive_core::{theme, Vec2};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlElement, KeyboardEvent, MouseEvent as WebMouseEvent};

const ROOT_ID: &str = "cursive-wasm-root";
const DEFAULT_COLS: usize = 108;
const DEFAULT_ROWS: usize = 51;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
struct EffectState {
    reverse: bool,
    dim: bool,
    bold: bool,
    italic: bool,
    strikethrough: bool,
    underline: bool,
    blink: bool,
}

impl EffectState {
    fn enable(&mut self, effect: theme::Effect) {
        match effect {
            theme::Effect::Simple => *self = Self::default(),
            theme::Effect::Reverse => self.reverse = true,
            theme::Effect::Dim => self.dim = true,
            theme::Effect::Bold => self.bold = true,
            theme::Effect::Italic => self.italic = true,
            theme::Effect::Strikethrough => self.strikethrough = true,
            theme::Effect::Underline => self.underline = true,
            theme::Effect::Blink => self.blink = true,
        }
    }

    fn disable(&mut self, effect: theme::Effect) {
        match effect {
            theme::Effect::Simple => {}
            theme::Effect::Reverse => self.reverse = false,
            theme::Effect::Dim => self.dim = false,
            theme::Effect::Bold => self.bold = false,
            theme::Effect::Italic => self.italic = false,
            theme::Effect::Strikethrough => self.strikethrough = false,
            theme::Effect::Underline => self.underline = false,
            theme::Effect::Blink => self.blink = false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct CellStyle {
    front: String,
    back: String,
    effects: EffectState,
}

impl CellStyle {
    fn from_theme(colors: theme::ColorPair, effects: EffectState) -> Self {
        Self {
            front: cursive_to_color(colors.front),
            back: cursive_to_color(colors.back),
            effects,
        }
    }

    fn to_css(&self) -> String {
        let (front, back) = if self.effects.reverse {
            (&self.back, &self.front)
        } else {
            (&self.front, &self.back)
        };

        let font_weight = if self.effects.bold { "700" } else { "400" };
        let font_style = if self.effects.italic { "italic" } else { "normal" };
        let opacity = if self.effects.dim { "0.75" } else { "1" };

        let mut decorations = Vec::new();
        if self.effects.underline {
            decorations.push("underline");
        }
        if self.effects.strikethrough {
            decorations.push("line-through");
        }

        let text_decoration = if decorations.is_empty() {
            "none".to_string()
        } else {
            decorations.join(" ")
        };

        let animation = if self.effects.blink {
            "animation: rustcal-blink 1s steps(1) infinite;"
        } else {
            ""
        };

        format!(
            "color: {front}; background: {back}; font-weight: {font_weight}; font-style: {font_style}; opacity: {opacity}; text-decoration: {text_decoration};{animation}"
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Cell {
    ch: char,
    style: CellStyle,
}

impl Cell {
    fn blank(style: CellStyle) -> Self {
        Self { ch: ' ', style }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ScreenBuffer {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl ScreenBuffer {
    fn new(width: usize, height: usize, fill: Cell) -> Self {
        Self {
            width,
            height,
            cells: vec![fill; width * height],
        }
    }

    fn row(&self, row: usize) -> &[Cell] {
        let start = row * self.width;
        &self.cells[start..start + self.width]
    }

    fn fill_blank(&mut self, style: CellStyle) {
        let blank = Cell::blank(style);
        for cell in &mut self.cells {
            *cell = blank.clone();
        }
    }

    fn write_text(&mut self, pos: Vec2, text: &str, style: &CellStyle) {
        let y = pos.y;
        if y >= self.height {
            return;
        }

        for (offset, ch) in text.chars().enumerate() {
            let x = pos.x + offset;
            if x >= self.width {
                break;
            }

            let index = y * self.width + x;
            self.cells[index] = Cell {
                ch,
                style: style.clone(),
            };
        }
    }
}

struct TextRun {
    text: String,
    style: CellStyle,
}

/// Backend using a DOM text grid for crisp character rendering.
pub struct Backend {
    document: Document,
    root: HtmlElement,
    rows: Vec<HtmlElement>,
    current_colors: RefCell<theme::ColorPair>,
    current_effects: RefCell<EffectState>,
    pending: RefCell<ScreenBuffer>,
    rendered: ScreenBuffer,
    events: Rc<RefCell<VecDeque<Event>>>,
}

impl Backend {
    /// Creates a new Cursive root using a wasm backend.
    pub fn init() -> std::io::Result<Box<dyn backend::Backend>> {
        let document = web_sys::window()
            .ok_or_else(|| io_error("Failed to get window"))?
            .document()
            .ok_or_else(|| io_error("Failed to get document"))?;

        let root = document
            .get_element_by_id(ROOT_ID)
            .ok_or_else(|| io_error("Failed to find DOM root with id cursive-wasm-root"))?
            .dyn_into::<HtmlElement>()
            .map_err(|_| io_error("Failed to cast DOM root"))?;

        let initial_colors = theme::ColorPair {
            front: theme::Color::Dark(theme::BaseColor::White),
            back: theme::Color::Dark(theme::BaseColor::Black),
        };
        let initial_style = CellStyle::from_theme(initial_colors, EffectState::default());
        let buffer = ScreenBuffer::new(DEFAULT_COLS, DEFAULT_ROWS, Cell::blank(initial_style.clone()));
        let events = Rc::new(RefCell::new(VecDeque::new()));

        setup_root(&root, DEFAULT_COLS, &initial_style)?;
        let rows = build_rows(&document, &root, DEFAULT_ROWS)?;

        register_keyboard_events(&root, Rc::clone(&events))?;
        register_mouse_events(&root, Rc::clone(&events), DEFAULT_COLS, DEFAULT_ROWS)?;
        register_resize_events(Rc::clone(&events))?;

        let mut backend = Backend {
            document,
            root,
            rows,
            current_colors: RefCell::new(initial_colors),
            current_effects: RefCell::new(EffectState::default()),
            pending: RefCell::new(buffer.clone()),
            rendered: buffer,
            events,
        };

        backend.draw_all_rows()?;
        Ok(Box::new(backend))
    }

    fn active_style(&self) -> CellStyle {
        CellStyle::from_theme(
            *self.current_colors.borrow(),
            self.current_effects.borrow().clone(),
        )
    }

    fn draw_all_rows(&mut self) -> std::io::Result<()> {
        for row_index in 0..self.rendered.height {
            self.render_row(row_index, self.rendered.row(row_index))?;
        }
        Ok(())
    }

    fn render_row(&self, row_index: usize, row: &[Cell]) -> std::io::Result<()> {
        let row_element = self
            .rows
            .get(row_index)
            .ok_or_else(|| io_error("Failed to access row element"))?;

        row_element.set_inner_html("");

        for run in row_runs(row) {
            let span = self
                .document
                .create_element("span")
                .map_err(|_| io_error("Failed to create row span"))?;
            span.set_text_content(Some(&run.text));
            span.set_attribute("style", &run.style.to_css())
                .map_err(|_| io_error("Failed to style row span"))?;

            row_element
                .append_child(&span)
                .map_err(|_| io_error("Failed to append row span"))?;
        }

        Ok(())
    }
}

impl cursive_core::backend::Backend for Backend {
    fn poll_event(&mut self) -> Option<Event> {
        self.events.borrow_mut().pop_front()
    }

    fn set_title(&mut self, title: String) {
        self.root.set_title(&title);
    }

    fn refresh(&mut self) {
        let pending = self.pending.borrow();

        for row_index in 0..pending.height {
            if pending.row(row_index) != self.rendered.row(row_index) {
                let _ = self.render_row(row_index, pending.row(row_index));
            }
        }

        self.rendered = pending.clone();
    }

    fn has_colors(&self) -> bool {
        true
    }

    fn screen_size(&self) -> Vec2 {
        let pending = self.pending.borrow();
        Vec2::new(pending.width, pending.height)
    }

    fn print_at(&self, pos: Vec2, text: &str) {
        let style = self.active_style();
        self.pending.borrow_mut().write_text(pos, text, &style);
    }

    fn clear(&self, color: theme::Color) {
        let colors = theme::ColorPair {
            front: self.current_colors.borrow().front,
            back: color,
        };
        let style = CellStyle::from_theme(colors, EffectState::default());
        self.pending.borrow_mut().fill_blank(style);
    }

    fn set_color(&self, color_pair: theme::ColorPair) -> theme::ColorPair {
        self.current_colors.replace(color_pair)
    }

    fn set_effect(&self, effect: theme::Effect) {
        self.current_effects.borrow_mut().enable(effect);
    }

    fn unset_effect(&self, effect: theme::Effect) {
        self.current_effects.borrow_mut().disable(effect);
    }

    fn name(&self) -> &str {
        "cursive-wasm-dom-backend"
    }
}

fn io_error(message: &str) -> Error {
    Error::new(ErrorKind::Other, message)
}

fn setup_root(root: &HtmlElement, cols: usize, initial_style: &CellStyle) -> std::io::Result<()> {
    root.set_inner_html("");
    root.set_attribute("tabindex", "0")
        .map_err(|_| io_error("Failed to set tabindex"))?;
    root.set_attribute(
        "style",
        &format!(
            "display: inline-block; min-width: {}ch; padding: 12px 14px; white-space: pre; font-family: 'Iosevka Term', 'IBM Plex Mono', 'SFMono-Regular', Menlo, Monaco, Consolas, 'Liberation Mono', monospace; font-size: clamp(10px, 0.95vw, 13px); line-height: 1.15; letter-spacing: 0; user-select: none; cursor: default; background-color: {}; color: {};",
            cols, initial_style.back, initial_style.front
        ),
    )
    .map_err(|_| io_error("Failed to set root style"))?;

    let style_tag = root
        .owner_document()
        .and_then(|document| document.get_element_by_id("rustcal-dom-backend-style"));

    if style_tag.is_none() {
        let document = root
            .owner_document()
            .ok_or_else(|| io_error("Failed to get owner document"))?;
        let style = document
            .create_element("style")
            .map_err(|_| io_error("Failed to create style tag"))?;
        style
            .set_attribute("id", "rustcal-dom-backend-style")
            .map_err(|_| io_error("Failed to set style id"))?;
        style.set_text_content(Some(
            "@keyframes rustcal-blink { 0%, 49% { opacity: 1; } 50%, 100% { opacity: 0.2; } }",
        ));
        if let Some(parent) = root.parent_node() {
            parent
                .append_child(&style)
                .map_err(|_| io_error("Failed to append style tag"))?;
        } else {
            root.append_child(&style)
                .map_err(|_| io_error("Failed to append style tag"))?;
        }
    }

    Ok(())
}

fn build_rows(
    document: &Document,
    root: &HtmlElement,
    row_count: usize,
) -> std::io::Result<Vec<HtmlElement>> {
    let mut rows = Vec::with_capacity(row_count);

    for _ in 0..row_count {
        let row = document
            .create_element("div")
            .map_err(|_| io_error("Failed to create row"))?
            .dyn_into::<HtmlElement>()
            .map_err(|_| io_error("Failed to cast row"))?;
        row.set_attribute(
            "style",
            "display: block; min-height: 1.15em; white-space: pre;",
        )
        .map_err(|_| io_error("Failed to style row"))?;

        root.append_child(&row)
            .map_err(|_| io_error("Failed to append row"))?;
        rows.push(row);
    }

    Ok(rows)
}

fn row_runs(row: &[Cell]) -> Vec<TextRun> {
    if row.is_empty() {
        return Vec::new();
    }

    let mut runs = Vec::new();
    let mut current_style = row[0].style.clone();
    let mut current_text = String::new();

    for cell in row {
        if cell.style == current_style {
            current_text.push(cell.ch);
            continue;
        }

        runs.push(TextRun {
            text: current_text,
            style: current_style,
        });

        current_style = cell.style.clone();
        current_text = String::new();
        current_text.push(cell.ch);
    }

    runs.push(TextRun {
        text: current_text,
        style: current_style,
    });

    runs
}

fn register_keyboard_events(
    root: &HtmlElement,
    events: Rc<RefCell<VecDeque<Event>>>,
) -> std::io::Result<()> {
    let closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
        if let Some(mapped) = map_keyboard_event(&event) {
            event.prevent_default();
            events.borrow_mut().push_back(mapped);
        }
    }) as Box<dyn FnMut(_)>);

    root.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
        .map_err(|_| io_error("Failed to add keydown listener"))?;
    closure.forget();
    Ok(())
}

fn register_mouse_events(
    root: &HtmlElement,
    events: Rc<RefCell<VecDeque<Event>>>,
    cols: usize,
    rows: usize,
) -> std::io::Result<()> {
    let press_events = Rc::clone(&events);
    let press_root = root.clone();
    let press = Closure::wrap(Box::new(move |event: WebMouseEvent| {
        event.prevent_default();
        let _ = press_root.focus();
        if let Some(button) = map_mouse_button(event.button()) {
            press_events.borrow_mut().push_back(Event::Mouse {
                offset: Vec2::new(0, 0),
                position: mouse_position(&press_root, &event, cols, rows),
                event: MouseEvent::Press(button),
            });
        }
    }) as Box<dyn FnMut(_)>);
    root.add_event_listener_with_callback("mousedown", press.as_ref().unchecked_ref())
        .map_err(|_| io_error("Failed to add mousedown listener"))?;
    press.forget();

    let release_events = Rc::clone(&events);
    let release_root = root.clone();
    let release = Closure::wrap(Box::new(move |event: WebMouseEvent| {
        event.prevent_default();
        if let Some(button) = map_mouse_button(event.button()) {
            release_events.borrow_mut().push_back(Event::Mouse {
                offset: Vec2::new(0, 0),
                position: mouse_position(&release_root, &event, cols, rows),
                event: MouseEvent::Release(button),
            });
        }
    }) as Box<dyn FnMut(_)>);
    root.add_event_listener_with_callback("mouseup", release.as_ref().unchecked_ref())
        .map_err(|_| io_error("Failed to add mouseup listener"))?;
    release.forget();

    let context = Closure::wrap(Box::new(move |event: WebMouseEvent| {
        event.prevent_default();
    }) as Box<dyn FnMut(_)>);
    root.add_event_listener_with_callback("contextmenu", context.as_ref().unchecked_ref())
        .map_err(|_| io_error("Failed to add contextmenu listener"))?;
    context.forget();

    Ok(())
}

fn register_resize_events(events: Rc<RefCell<VecDeque<Event>>>) -> std::io::Result<()> {
    let window = web_sys::window().ok_or_else(|| io_error("Failed to get window"))?;

    let resize = Closure::wrap(Box::new(move || {
        events.borrow_mut().push_back(Event::WindowResize);
    }) as Box<dyn FnMut()>);

    window
        .add_event_listener_with_callback("resize", resize.as_ref().unchecked_ref())
        .map_err(|_| io_error("Failed to add resize listener"))?;
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
    root: &HtmlElement,
    event: &WebMouseEvent,
    cols: usize,
    rows: usize,
) -> Vec2 {
    let rect = root.get_bounding_client_rect();
    let cell_width = (rect.width() / cols as f64).max(1.0);
    let cell_height = (rect.height() / rows as f64).max(1.0);
    let x = ((event.client_x() as f64 - rect.left()) / cell_width).floor();
    let y = ((event.client_y() as f64 - rect.top()) / cell_height).floor();

    Vec2::new(
        x.max(0.0).min((cols.saturating_sub(1)) as f64) as usize,
        y.max(0.0).min((rows.saturating_sub(1)) as f64) as usize,
    )
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

/// Convert cursive color to hex color.
pub fn cursive_to_color(color: theme::Color) -> String {
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
        theme::Color::Light(theme::BaseColor::Red) => "#b91c1c".to_string(),
        theme::Color::Light(theme::BaseColor::Green) => "#15803d".to_string(),
        theme::Color::Light(theme::BaseColor::Yellow) => "#a16207".to_string(),
        theme::Color::Light(theme::BaseColor::Blue) => "#2563eb".to_string(),
        theme::Color::Light(theme::BaseColor::Magenta) => "#7c3aed".to_string(),
        theme::Color::Light(theme::BaseColor::Cyan) => "#0891b2".to_string(),
        theme::Color::Light(theme::BaseColor::White) => "#fafaf9".to_string(),
        theme::Color::Rgb(r, g, b) => format!("#{r:02x}{g:02x}{b:02x}"),
        theme::Color::RgbLowRes(r, g, b) => {
            let r = (u16::from(r) * 255 / 5) as u8;
            let g = (u16::from(g) * 255 / 5) as u8;
            let b = (u16::from(b) * 255 / 5) as u8;
            format!("#{r:02x}{g:02x}{b:02x}")
        }
        theme::Color::TerminalDefault => "#ece7de".to_string(),
    }
}
