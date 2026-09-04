#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

#[allow(unused)]
use alloc::vec::Vec;

mod color;
mod dct;
mod error;
mod quant;
mod zigzag;

pub use color::{YCbCrImage, rgb_to_ycbcr};
pub use dct::{BlockI16, BlockU8, Dct, NaiveDct};
pub use quant::{QC, QY, quant, quant_mut};
pub use zigzag::zigzag_enc;
