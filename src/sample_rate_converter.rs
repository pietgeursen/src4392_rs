use packed_struct::prelude::*;

use crate::RegisterAddress;

use crate::registers::Registers;

#[derive(PrimitiveEnum, Clone, Copy, PartialEq, Debug, Default)]
pub enum SrcSource {
    #[default]
    PortA = 0b00,
    PortB = 0b01,
    DIR = 0b10,
}
#[derive(PrimitiveEnum, Clone, Copy, PartialEq, Debug, Default)]
pub enum SrcClockSource {
    #[default]
    Mclk = 0b00,
    Rxcki = 0b01,
    Rxcko = 0b10,
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering = "msb0", endian = "lsb")]
pub struct SrcControl1 {
    #[packed_field(bits = "0..2", ty = "enum")]
    pub source: SrcSource,
    #[packed_field(bits = "2..4", ty = "enum")]
    pub clock_source: SrcClockSource,
    #[packed_field(bits = "4")]
    pub mute: bool,
    #[packed_field(bits = "6")]
    pub track: bool,
}
impl SrcControl1 {
    pub const REGISTER_ADDRESS: Registers = Registers::SrcControl;
}
impl RegisterAddress for SrcControl1 {
    fn register_address() -> crate::registers::Registers {
        Self::REGISTER_ADDRESS
    }
}

#[derive(PrimitiveEnum, Clone, Copy, PartialEq, Debug, Default)]
pub enum InterpolationFilterGroupDelay {
    #[default]
    _64 = 0b00,
    _32 = 0b01,
    _16 = 0b10,
    _8 = 0b11,
}

#[derive(PrimitiveEnum, Clone, Copy, PartialEq, Debug, Default)]
pub enum Deemphasis {
    #[default]
    None = 0b00,
    _48000hz = 0b01,
    _44100Hz = 0b10,
    _32000Hz = 0b11,
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering = "msb0", endian = "lsb")]
pub struct SrcControl2 {
    #[packed_field(bits = "0..2", ty = "enum")]
    pub interpolation_group_delay: InterpolationFilterGroupDelay,
    /// This bit selects the mode of the decimation function, either true decimation filter or direct downsampling without filtering.
    /// DDN Decimation Function
    /// 0 Decimation Filter (Default)
    /// 1 Direct Down Sampling
    /// Note: Direct down-sampling should only be used when the output sampling rate is higher than
    /// the input sampling rate. When the output sampling rate is equal to or lower than the input
    /// sampling rate, the Decimation Filter must be used in order to avoid aliasing.
    #[packed_field(bits = "2")]
    pub dd_n: bool,
    #[packed_field(bits = "3..5", ty = "enum")]
    pub deemphasis: Deemphasis,
    #[packed_field(bits = "5")]
    pub autodem: bool,
    #[packed_field(bits = "6..8")]
    _reserved: ReservedZeroes<packed_bits::Bits<2>>,
}
impl SrcControl2 {
    pub const REGISTER_ADDRESS: Registers = Registers::SrcControl2;
}
impl RegisterAddress for SrcControl2 {
    fn register_address() -> crate::registers::Registers {
        Self::REGISTER_ADDRESS
    }
}

#[derive(PrimitiveEnum, Clone, Copy, PartialEq, Debug, Default)]
pub enum SrcOutputWordLength {
    #[default]
    _24bits = 0b00,
    _20bits = 0b01,
    _18bits = 0b10,
    _16bits = 0b11,
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering = "msb0", endian = "lsb")]
pub struct SrcControl3 {
    #[packed_field(bits = "0..6")]
    _reserved: ReservedZeroes<packed_bits::Bits<6>>,
    #[packed_field(bits = "6..8", ty = "enum")]
    pub output_word_length: SrcOutputWordLength,
}
impl SrcControl3 {
    pub const REGISTER_ADDRESS: Registers = Registers::SrcControl3;
}
impl RegisterAddress for SrcControl3 {
    fn register_address() -> crate::registers::Registers {
        Self::REGISTER_ADDRESS
    }
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering = "msb0", endian = "lsb", size_bytes = "2")]
pub struct SrcRatio {
    #[packed_field(bits = "11..16")]
    integer: Integer<u8, packed_bits::Bits<5>>,
    #[packed_field(bits = "0..11")]
    fraction: Integer<u16, packed_bits::Bits<11>>,
}
impl SrcRatio {
    pub const REGISTER_ADDRESS: Registers = Registers::SrcInputOutputRatio1;
}
impl RegisterAddress for SrcRatio {
    fn register_address() -> crate::registers::Registers {
        Self::REGISTER_ADDRESS
    }
}
