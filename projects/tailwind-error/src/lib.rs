#![forbid(missing_docs)]
#![doc = include_str!("../readme.md")]
#![allow(clippy::needless_return)]

mod error;
mod error_3rd;

pub use self::error::{MaybeRanged, Result, TailwindError, TailwindErrorKind};
pub use url::Url;
pub use yggdrasil_shared::DiagnosticLevel;

#[cfg(feature = "git2")]
pub extern crate git2;
#[cfg(feature = "globset")]
pub extern crate globset;
#[cfg(feature = "num")]
pub extern crate num;
