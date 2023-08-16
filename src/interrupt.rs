use packed_struct::prelude::*;

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering = "msb0", endian = "lsb")]
pub struct GlobalInterruptStatus {
    #[packed_field(bits = "7:3")]
    pub _reserved: ReservedZeroes<packed_bits::Bits<5>>,
    #[packed_field(bits = "2")]
    pub tx: bool,
    #[packed_field(bits = "1")]
    pub rx: bool,
    #[packed_field(bits = "0")]
    pub src: bool,
}
