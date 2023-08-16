use packed_struct::prelude::*;

use crate::{RegisterAddress, Registers};

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering = "msb0", endian = "lsb")]
pub struct Reset {
    #[packed_field(bits = "7")]
    pub n_pdsrc: bool,
    #[packed_field(bits = "6")]
    pub n_pdrx: bool,
    #[packed_field(bits = "5")]
    pub n_pdtx: bool,
    #[packed_field(bits = "4")]
    pub n_pdpb: bool,
    #[packed_field(bits = "3")]
    pub n_pdpa: bool,
    #[packed_field(bits = "2")]
    pub pdall: bool,
    #[packed_field(bits = "1")]
    pub _reserved: ReservedZero<packed_bits::Bits<1>>,
    #[packed_field(bits = "0")]
    pub reset: bool,
}
impl Reset {
    pub const ADDRESS: Registers = Registers::Reset;
}
impl RegisterAddress for Reset {
    fn register_address() -> Registers {
        Self::ADDRESS
    }
}
