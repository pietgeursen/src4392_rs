// Unused 
pub struct TransmitterControlRegister {
    pub bssl: u8,
    pub valid: u8,
    pub blsm: u8,
    pub txis0: u8,
    pub txis1: u8,
    pub txd: u8,
    pub txdiv0: u8,
    pub txdiv1: u8,
}

pub struct TransmitterControl1Register {
    pub txoff: u8,
    pub txmute: u8,
    pub aesoff: u8,
    pub txbtd: u8,
    pub ldmux: u8,
    pub aesmux: u8,
    pub bypmux0: u8,
    pub bypmux1: u8,
}

pub struct TransmitterControl2Register {
    pub txcus0: u8,
    pub txcus1: u8,
    pub valsel: u8,
    pub transmitter_control2_reserved: u8,
    pub transmitter_control2_reserved2: u8,
    pub transmitter_control2_reserved3: u8,
    pub transmitter_control2_reserved4: u8,
    pub transmitter_control2_reserved5: u8,
}

pub struct SRCAndDITStatusRegister {
    pub src_and_dit_status_reserved: u8,
    pub tbt_i: u8,
    pub tslip: u8,
    pub src_ready: u8,
    pub ratio_ready: u8,
    pub src_and_dit_status_reserved2: u8,
    pub src_and_dit_status_reserved3: u8,
    pub src_and_dit_status_reserved4: u8,
}

pub struct SRCAndDITInterruptMaskRegister {
    pub src_and_dit_interrupt_mask_reserved: u8,
    pub mtbt_i: u8,
    pub mtslip: u8,
    pub m_ready: u8,
    pub m_ratio: u8,
    pub src_and_dit_interrupt_mask_reserved2: u8,
    pub src_and_dit_interrupt_mask_reserved3: u8,
    pub src_and_dit_interrupt_mask_reserved4: u8,
}

pub struct SrcAndDitInterruptMode {
    pub tbtim0: u8,
    pub tbtim1: u8,
    pub tslipm0: u8,
    pub tslipm1: u8,
    pub readym0: u8,
    pub readym1: u8,
    pub ratiom0: u8,
    pub ratiom1: u8,
}

pub struct ReceiverControl {
    pub rxmux: u8,
    pub rxckoe: u8,
    pub rxckod0: u8,
    pub rxckod1: u8,
    pub rxamll: u8,
    pub rxclk: u8,
    pub rxbtd: u8,
    pub rxd: u8,
}

pub struct ReceiverClockAndMuteControl {
    pub txoff: u8,
    pub txmute: u8,
    pub aesoff: u8,
    pub txbtd: u8,
    pub ldmux: u8,
    pub aesmux: u8,
    pub bypmux0: u8,
    pub bypmux1: u8,
}

pub struct SrcControl4 {
    pub al0: u8,
    pub al1: u8,
    pub al2: u8,
    pub al3: u8,
    pub al4: u8,
    pub al5: u8,
    pub al6: u8,
    pub al7: u8,
}

pub struct SrcControl5 {
    pub ar0: u8,
    pub ar1: u8,
    pub ar2: u8,
    pub ar3: u8,
    pub ar4: u8,
    pub ar5: u8,
    pub ar6: u8,
    pub ar7: u8,
}


