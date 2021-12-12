//! Generate documentation for SystemVerilog project.
//!
//! ```svgbob
//!     .--.---.
//! SV  |#  \_ | DOC
//! o-->||__(_)|*-->
//!     |   \ \|
//!     '----'-'
//! ```
//!
//! # Binary `svdocgen`
//!
//! ```terminal
//! $svdocgen [INPUT]
//! ```
//!
//! # Use as library
//!
//! TODO: #[doc = svgbobdoc::transform_mdstr!(

pub mod args;
pub mod mdbook;
