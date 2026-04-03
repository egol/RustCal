/// Extension trait for the `Cursive` root to simplify initialization.
///
/// It brings backend-specific methods to initialize a `Cursive` root.
///
/// # Examples
///
/// ```rust,no_run
/// use cursive::{Cursive, CursiveExt};
///
/// let mut siv = Cursive::new();
///
/// // Use `CursiveExt::run()` to pick one of the enabled backends,
/// // depending on cargo features.
/// siv.run();
///
/// // Or explicitly use a specific backend
/// #[cfg(feature = "ncurses-backend")]
/// siv.run_ncurses().unwrap();
/// #[cfg(feature = "panncurses-backend")]
/// siv.run_pancurses().unwrap();
/// #[cfg(feature = "termion-backend")]
/// siv.run_termion().unwrap();
/// #[cfg(feature = "crossterm-backend")]
/// siv.run_crossterm().unwrap();
/// #[cfg(feature = "blt-backend")]
/// siv.run_blt();
/// #[cfg(feature="wasm-backend")]
/// siv.run_wasm().unwrap();
/// ```
pub trait CursiveExt {
    /// Tries to use one of the enabled backends.
    ///
    /// Will fallback to the dummy backend if no other backend feature is enabled.
    ///
    /// # Panics
    ///
    /// If the backend initialization fails.
    fn run(&mut self);

    /// Creates a new Cursive root using a ncurses backend.
    #[cfg(all(feature = "ncurses-backend", not(target_arch = "wasm32")))]
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "ncurses-backend")))]
    fn run_ncurses(&mut self) -> std::io::Result<()>;

    /// Creates a new Cursive root using a pancurses backend.
    #[cfg(all(feature = "pancurses-backend", not(target_arch = "wasm32")))]
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "pancurses-backend")))]
    fn run_pancurses(&mut self) -> std::io::Result<()>;

    /// Creates a new Cursive root using a termion backend.
    #[cfg(all(feature = "termion-backend", not(target_arch = "wasm32")))]
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "termion-backend")))]
    fn run_termion(&mut self) -> std::io::Result<()>;

    /// Creates a new Cursive root using a crossterm backend.
    #[cfg(all(feature = "crossterm-backend", not(target_arch = "wasm32")))]
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "crossterm-backend")))]
    fn run_crossterm(&mut self) -> std::io::Result<()>;

    /// Creates a new Cursive root using a bear-lib-terminal backend.
    #[cfg(all(feature = "blt-backend", not(target_arch = "wasm32")))]
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "blt-backend")))]
    fn run_blt(&mut self);
}

impl CursiveExt for cursive_core::Cursive {
    fn run(&mut self) {
        cfg_if::cfg_if! {
            if #[cfg(all(feature = "blt-backend", not(target_arch = "wasm32")))] {
                self.run_blt()
            } else if #[cfg(all(feature = "termion-backend", not(target_arch = "wasm32")))] {
                self.run_termion().unwrap()
            } else if #[cfg(all(feature = "crossterm-backend", not(target_arch = "wasm32")))] {
                self.run_crossterm().unwrap()
            } else if #[cfg(all(feature = "pancurses-backend", not(target_arch = "wasm32")))] {
                self.run_pancurses().unwrap()
            } else if #[cfg(all(feature = "ncurses-backend", not(target_arch = "wasm32")))] {
                self.run_ncurses().unwrap()
            } else {
                log::warn!("No built-it backend, falling back to Cursive::dummy().");
                futures::executor::block_on(
                    self.run_with(cursive_core::backend::Dummy::init),
                )
            }
        }
    }

    #[cfg(all(feature = "ncurses-backend", not(target_arch = "wasm32")))]
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "curses-backend")))]
    fn run_ncurses(&mut self) -> std::io::Result<()> {
        futures::executor::block_on(self.try_run_with(crate::backends::curses::n::Backend::init))
    }

    #[cfg(all(feature = "pancurses-backend", not(target_arch = "wasm32")))]
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "pancurses-backend")))]
    fn run_pancurses(&mut self) -> std::io::Result<()> {
        futures::executor::block_on(self.try_run_with(crate::backends::curses::pan::Backend::init))
    }

    #[cfg(all(feature = "termion-backend", not(target_arch = "wasm32")))]
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "termion-backend")))]
    fn run_termion(&mut self) -> std::io::Result<()> {
        futures::executor::block_on(self.try_run_with(crate::backends::termion::Backend::init))
    }

    #[cfg(all(feature = "crossterm-backend", not(target_arch = "wasm32")))]
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "crossterm-backend")))]
    fn run_crossterm(&mut self) -> std::io::Result<()> {
        futures::executor::block_on(self.try_run_with(crate::backends::crossterm::Backend::init))
    }

    #[cfg(all(feature = "blt-backend", not(target_arch = "wasm32")))]
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "blt-backend")))]
    fn run_blt(&mut self) {
        futures::executor::block_on(self.run_with(crate::backends::blt::Backend::init))
    }
}
