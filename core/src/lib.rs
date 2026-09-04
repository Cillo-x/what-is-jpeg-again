#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

#[allow(unused)]
use alloc::vec::Vec;

mod bitwriter;
mod color;
mod dct;
mod error;
mod quant;
mod rle;
mod zigzag;

pub use bitwriter::BitWriter;
pub use color::{YCbCrImage, rgb_to_ycbcr};
pub use dct::{BlockI16, BlockU8, Dct, NaiveDct};
pub use quant::{QC, QY, quant, quant_mut};
pub use rle::{AcPair, rle};
pub use zigzag::zigzag_scan;
