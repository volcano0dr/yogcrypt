//! **A fast, general purpose crypto library in pure Rust.**
//!
//! YogCrypt is designed to be a high-performance, general purpose crypto library.
//!
//! YogCrypt currently provides three cryptographic algorithms in Chinese National
//! Standard, namely the SM2 cryptographic asymmetric algorithm, the SM3
//! cryptographic hash algorithm, and the SM4 block cipher algorithm.
//!
//! ## License
//!
//! YogCrypt is distributed under the terms of both the MIT license and the Apache
//! License (Version 2.0). See LICENSE-APACHE and LICENSE-MIT for details.

#![no_std]
#![feature(core_intrinsics)]
#![feature(global_asm)]
#![allow(unused_doc_comments)]
#![allow(dead_code)]
// make the linter allow the following usage

// literals declartion are used in S boxes which are
// not intended for human reading
#![cfg_attr(feature = "cargo-clippy", allow(clippy::unreadable_literal))]
// Or (`|`) operators are used in overflowing addition
// which is not a mistake
#![cfg_attr(feature = "cargo-clippy", allow(clippy::suspicious_arithmetic_impl))]
// single characters names are used in accordance to
// documentation of cryptographic schemes
#![cfg_attr(feature = "cargo-clippy", allow(clippy::many_single_char_names))]
// Some expressions are too long and are necessarily split into lines
// No commas are needed in these cases
#![cfg_attr(feature = "cargo-clippy", allow(clippy::possible_missing_comma))]

#[macro_use]
extern crate alloc;

#[macro_use]
extern crate lazy_static;

mod basic;
pub mod sm2;
pub mod sm3;
pub mod sm4;
