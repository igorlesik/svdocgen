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
//! SvDocGen is primarily used as a command line tool,
//! even though it exposes all its functionality as a Rust crate
//! for integration in other projects.
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
pub mod fsnode;
pub mod mdbook;

// TODO static_assert!
const _: () = assert!(std::mem::size_of::<u64>() == 8);
