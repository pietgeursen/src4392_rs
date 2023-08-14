#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Registers {
    Reset = 0x01,
    GlobalInterruptStatus = 0x02,
    PortAControl = 0x03,
    PortAControl2 = 0x04,
    PortBControl = 0x05,
    PortBControl2 = 0x06,
    TransmitterControl = 0x07,
    TransmitterControl2 = 0x08,
    TransmitterControl3 = 0x09,
    SrcAndDitStatus = 0x0A,
    SrcAndDitInterruptMask = 0x0B,
    SrcAndDitInterruptMode = 0x0C,
    ReceiverControl = 0x0D,
    ReceiverControl2 = 0x0E,
    ReceiverPllConfiguration = 0x0F,
    ReceiverPllConfiguration2 = 0x10,
    ReceiverPllConfiguration3 = 0x11,
    NonPcmAudioDetection = 0x12,
    ReceiverStatus = 0x13,
    ReceiverStatus2 = 0x14,
    ReceiverStatus3 = 0x15,
    ReceiverInterruptMask = 0x16,
    ReceiverInterruptMask2 = 0x17,
    ReceiverInterruptMode = 0x18,
    ReceiverInterruptMode2 = 0x19,
    ReceiverInterruptMode3 = 0x1A,
    Gpo1 = 0x1B,
    Gpo2 = 0x1C,
    Gpo3 = 0x1D,
    Gpo4 = 0x1E,
    AudioCdQChannelSubCode1 = 0x1F,
    AudioCdQChannelSubCode2 = 0x20,
    AudioCdQChannelSubCode3 = 0x21,
    AudioCdQChannelSubCode4 = 0x22,
    AudioCdQChannelSubCode5 = 0x23,
    AudioCdQChannelSubCode6 = 0x24,
    AudioCdQChannelSubCode7 = 0x25,
    AudioCdQChannelSubCode8 = 0x26,
    AudioCdQChannelSubCode9 = 0x27,
    AudioCdQChannelSubCode10 = 0x28,
    PcBurstPreambleHighByte = 0x29,
    PcBurstPreambleLowByte = 0x2A,
    PdBurstPreambleHighByte = 0x2B,
    PdBurstPreambleLowByte = 0x2C,
    SrcControl = 0x2D,
    SrcControl2 = 0x2E,
    SrcControl3 = 0x2F,
    SrcControl4 = 0x30,
    SrcControl5 = 0x31,
    SrcInputOutputRatio1 = 0x32,
    SrcInputOutputRatio2 = 0x33,
    PageSelection = 0x7F,
}

impl Registers {
    pub fn as_address_byte(&self, rw: ReadWrite) -> u8 {
        match rw {
            ReadWrite::Read => *self as u8 | (1u8 << 8),
            ReadWrite::Write => *self as u8,
        }
    }
}

#[derive(Copy, Clone)]
pub enum ReadWrite {
    Read,
    Write,
}
