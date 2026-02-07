//! Breenix-specific definitions.

#![stable(feature = "raw_ext", since = "1.1.0")]
#![doc(cfg(target_os = "breenix"))]

pub mod fs;
pub mod raw;
