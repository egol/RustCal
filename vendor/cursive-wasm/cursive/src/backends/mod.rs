//! Define backends using common libraries.
//!
//! Cursive doesn't print anything by itself: it delegates this job to a
//! backend library, which handles all actual input and output.
//!
//! This module defines the [`Backend`] trait, as well as a few implementations
//! using some common libraries. Each of those included backends needs a
//! corresponding feature to be enabled.
//!
//! [`Backend`]: ../backend/trait.Backend.html
#[cfg(all(unix, not(target_arch = "wasm32")))]
mod resize;

#[cfg(all(not(target_arch = "wasm32"), feature = "blt-backend"))]
pub mod blt;
#[cfg(all(not(target_arch = "wasm32"), feature = "crossterm-backend"))]
pub mod crossterm;
#[cfg(any(feature = "ncurses-backend", feature = "pancurses-backend"))]
pub mod curses;
pub mod puppet;
#[cfg(all(not(target_arch = "wasm32"), feature = "termion-backend"))]
pub mod termion;
/// Provides a backend using the `wasm`.
#[cfg(feature = "wasm-backend")]
pub mod wasm;

#[allow(dead_code)]
fn boxed(e: impl std::error::Error + 'static) -> Box<dyn std::error::Error> {
    Box::new(e)
}

/// Tries to initialize the default backend.
///
/// Will use the first backend enabled from the list:
/// * BearLibTerminal
/// * Termion
/// * Crossterm
/// * Pancurses
/// * Ncurses
/// * wasm
/// * Dummy
pub fn try_default() -> Result<Box<dyn cursive_core::backend::Backend>, Box<dyn std::error::Error>>
{
    cfg_if::cfg_if! {
        if #[cfg(all(feature = "blt-backend", not(target_arch = "wasm32")))] {
            Ok(blt::Backend::init())
        } else if #[cfg(all(feature = "termion-backend", not(target_arch = "wasm32")))] {
            termion::Backend::init().map_err(boxed)
        } else if #[cfg(all(feature = "crossterm-backend", not(target_arch = "wasm32")))] {
            crossterm::Backend::init().map_err(boxed)
        } else if #[cfg(all(feature = "pancurses-backend", not(target_arch = "wasm32")))] {
            curses::pan::Backend::init().map_err(boxed)
        } else if #[cfg(all(feature = "ncurses-backend", not(target_arch = "wasm32")))] {
            curses::n::Backend::init().map_err(boxed)
        } else if #[cfg(feature = "wasm-backend")] {
            wasm::Backend::init().map_err(boxed)
        } else {
            log::warn!("No built-it backend, falling back to Dummy backend.");
            Ok(cursive_core::backend::Dummy::init())
        }
    }
}
