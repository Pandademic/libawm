//! libawm - power's awm
//! **NOTE**: in order to use the xcb implementation of penrose, you will need to install the C
//! libraries that are dependencies (namely xcb, Cairo and Pango).
//!
//! [1]: https://dwm.suckless.org/
//! [2]: https://xmonad.org/
//! [3]: http://www.qtile.org/
//! [4]: https://www.rust-lang.org/learn
//! [5]: https://doc.rust-lang.org/book/
//! [6]: https://github.com/sminez/penrose/tree/develop/examples
//! [7]: crate::gen_keybindings
//! [8]: crate::core::xconnection::XConn
//! [9]: crate::core::config::Config
//! [10]: crate::core::manager::WindowManager
//! [11]: crate::core::hooks
//! [12]: crate::draw::bar
//! [13]: https://crates.io/crates/simplelog
//! [14]: https://xcb.freedesktop.org/
//! [15]: https://www.rust-lang.org
#![warn(
    clippy::complexity,
    clippy::correctness,
    clippy::style,
    future_incompatible,
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms
)]
#![allow(clippy::too_many_arguments, clippy::borrowed_box, clippy::or_fun_call)]

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate tracing;

#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

#[macro_use]
pub mod core;

pub mod contrib;
pub mod draw;

#[cfg(feature = "xcb")]
pub mod xcb;

#[cfg(feature = "x11rb")]
pub mod x11rb;

#[doc(hidden)]
pub mod __test_helpers;

#[doc(hidden)]
pub use penrose_proc::validate_user_bindings;

// top level re-exports
#[doc(inline)]
pub use crate::core::{
    config::Config,
    data_types::Change::*,
    helpers::logging_error_handler,
    manager::WindowManager,
    ring::{Direction::*, InsertPoint, Selector},
    xconnection::Xid,
};

#[cfg(feature = "xcb")]
#[doc(inline)]
pub use crate::xcb::{new_xcb_backed_window_manager, XcbConnection};

/// Enum to store the various ways that operations can fail in Penrose
#[derive(thiserror::Error, Debug)]
pub enum PenroseError {
    /// Something went wrong using the [draw] module.
    ///
    /// See [DrawError][crate::draw::DrawError] for variants.
    #[error(transparent)]
    Draw(#[from] crate::draw::DrawError),

    /// Something was inconsistant when attempting to re-create a serialised [WindowManager]
    #[error("unable to rehydrate from serialized state: {0}")]
    HydrationState(String),

    /// Something was inconsistant when attempting to re-create a serialised [WindowManager]
    #[error("the following serialized client IDs were not known to the X server: {0:?}")]
    MissingClientIds(Vec<Xid>),

    /// A conversion to utf-8 failed
    #[error("UTF-8 error")]
    NonUtf8Prop(#[from] std::string::FromUtf8Error),

    #[doc(hidden)]
    #[error(transparent)]
    Infallible(#[from] std::convert::Infallible),

    /// An [IO Error][std::io::Error] was encountered
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// Wm(Normal)Hints received from the X server were invalid
    #[error("Invalid window hints property: {0}")]
    InvalidHints(String),

    /// No elements match the given selector
    #[error("No elements match the given selector")]
    NoMatchingElement,

    /// Attempting to construct a penrose data type from an int failed.
    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),

    /// A generic error type for use in user code when needing to construct
    /// a simple [PenroseError].
    #[error("Unhandled error: {0}")]
    Raw(String),

    /// An attempt to spawn an external process failed
    #[error("unable to get stdout handle for child process: {0}")]
    SpawnProc(String),

    /// Parsing an [Atom][core::xconnection::Atom] from a str failed.
    ///
    /// This happens when the atom name being requested is not a known atom.
    #[error(transparent)]
    Strum(#[from] strum::ParseError),

    /// An attempt was made to reference a client that is not known to penrose
    #[error("{0} is not a known client")]
    UnknownClient(Xid),

    /// A user specified key binding contained an invalid modifier key
    #[error("Could not find modkey {0}.Does it exist?")]
    UnknownModifier(String),

    /// Something went wrong using the [xcb] module.
    ///
    /// See [XcbError][crate::xcb::XcbError] for variants.
    #[cfg(feature = "xcb")]
    #[error(transparent)]
    Xcb(#[from] crate::xcb::XcbError),

    /// Something went wrong using the [x11rb] module.
    ///
    /// See [X11rbError][crate::x11rb::X11rbError] for variants.
    #[cfg(feature = "x11rb")]
    #[error(transparent)]
    X11rb(#[from] crate::x11rb::X11rbError),

    /// Something went wrong when communicating with the X server
    #[error(transparent)]
    X(#[from] crate::core::xconnection::XError),
}

/// Top level penrose Result type
pub type Result<T> = std::result::Result<T, PenroseError>;

/// A function that can be registered to handle errors that occur during [WindowManager] operation
pub type ErrorHandler = Box<dyn FnMut(PenroseError)>;
