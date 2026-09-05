#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

#[allow(unused)]
use alloc::vec::Vec;

mod bitwriter;
mod color;
mod dct;
mod encoder;
mod error;
mod huffman;
mod quant;
mod rle;
mod zigzag;

pub use bitwriter::BitWriter;
pub use color::{YCbCrImage, rgb_to_ycbcr};
pub use dct::{BlockI16, BlockU8, Dct, NaiveDct};
pub use encoder::JpegEncoder;
pub use huffman::{
    AcHuffmanTable, C_AC_HFT, C_DC_HFT, DcHuffmanTable, Y_AC_HFT, Y_DC_HFT, calculate_size_and_amp,
};
pub use quant::{QC, QY, quant, quant_mut};
pub use rle::{AcPair, rle};
pub use zigzag::zigzag_scan;
