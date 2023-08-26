#![no_std]
use core::{fmt::Debug, marker::PhantomData};

use embedded_hal::blocking::spi::Transfer;
use embedded_hal::digital::v2::OutputPin;
use packed_struct::prelude::*;
pub use port_control::{
    AudioFormat, OutputDataSource, PortAControl1Register, PortAControl2Register,
    PortBControl1Register, PortBControl2Register, PortClockSource, PortMasterClockDivider,
};
use registers::Registers;
pub use sample_rate_converter::{
    Deemphasis, InterpolationFilterGroupDelay, SrcClockSource, SrcControl1, SrcControl2, SrcSource,
};

pub mod interrupt;
pub mod port_control;
pub mod registers;
pub mod reset;
pub mod sample_rate_converter;

pub use registers::ReadWrite;
use reset::Reset;

#[derive(Copy, Clone, Debug)]
pub enum Port {
    A,
    B,
}

pub struct Src4392<P, SPI, E>
where
    P: OutputPin,
    <P as embedded_hal::digital::v2::OutputPin>::Error: Debug,
    SPI: Transfer<u8, Error = E>,
{
    chip_select: P,
    spi: PhantomData<SPI>,
}

impl<P, SPI, E> Src4392<P, SPI, E>
where
    P: OutputPin,
    <P as embedded_hal::digital::v2::OutputPin>::Error: Debug,
    SPI: Transfer<u8, Error = E>,
{
    pub fn new(chip_select: P) -> Self
    where
        SPI: Transfer<u8, Error = E>,
        E: Debug,
    {
        Self {
            chip_select,
            spi: PhantomData,
        }
    }
    pub fn reset(&mut self, spi: &mut SPI) -> Result<(), E> {
        self.modify_register(spi, |reg: &mut Reset| {
            reg.reset = true;
        })
    }

    pub fn configure_port(
        &mut self,
        spi: &mut SPI,
        port: Port,
        audio_format: AudioFormat,
        output_data_source: OutputDataSource,
        clock_divider: PortMasterClockDivider,
        clock_source: PortClockSource,
        is_slave: bool,
    ) -> Result<(), E> {
        match port {
            Port::A => {
                self.modify_register(spi, |reg: &mut PortAControl1Register| {
                    reg.am_slave = is_slave;
                    reg.afmt = audio_format;
                    reg.aout = output_data_source;
                })?;

                self.modify_register(spi, |reg: &mut PortAControl2Register| {
                    reg.adiv = clock_divider;
                    reg.aclk = clock_source;
                })
            }
            Port::B => {
                self.modify_register(spi, |reg: &mut PortBControl1Register| {
                    reg.am_slave = is_slave;
                    reg.afmt = audio_format;
                    reg.aout = output_data_source;
                })?;

                self.modify_register(spi, |reg: &mut PortBControl2Register| {
                    reg.adiv = clock_divider;
                    reg.aclk = clock_source;
                })
            }
        }
    }

    pub fn set_src(
        &mut self,
        spi: &mut SPI,
        src_source: SrcSource,
        clock_source: SrcClockSource,
        interpolation_group_delay: InterpolationFilterGroupDelay,
        deemphasis: Deemphasis,
        is_direct_down_sampling: bool,
    ) -> Result<(), E> {
        self.modify_register(spi, |reg: &mut SrcControl1| {
            reg.source = src_source;
            reg.clock_source = clock_source;
        })?;

        self.modify_register(spi, |reg: &mut SrcControl2| {
            reg.deemphasis = deemphasis;
            reg.interpolation_group_delay = interpolation_group_delay;
            reg.dd_n = is_direct_down_sampling;
        })
    }

    pub fn set_port_audio_format(
        &mut self,
        spi: &mut SPI,
        port: Port,
        audio_format: AudioFormat,
    ) -> Result<(), E> {
        match port {
            Port::A => self.modify_register(spi, |reg: &mut PortAControl1Register| {
                reg.afmt = audio_format
            }),
            Port::B => self.modify_register(spi, |reg: &mut PortBControl1Register| {
                reg.afmt = audio_format
            }),
        }
    }

    pub fn read_registers<'a>(
        &mut self,
        spi: &mut SPI,
        start_address: Registers,
        buffer: &'a mut [u8],
    ) -> Result<&'a [u8], E> {
        self.register_transfer(spi, start_address, buffer, ReadWrite::Read)
    }

    pub fn write_registers<'a>(
        &mut self,
        spi: &mut SPI,
        start_address: Registers,
        buffer: &'a mut [u8],
    ) -> Result<&'a [u8], E> {
        self.register_transfer(spi, start_address, buffer, ReadWrite::Write)
    }

    pub fn register_transfer<'a>(
        &mut self,
        spi: &mut SPI,
        start_address: Registers,
        buffer: &'a mut [u8],
        read_or_write: ReadWrite,
    ) -> Result<&'a [u8], E> {
        self.chip_select.set_low().unwrap();
        let mut cmd_bytes = [start_address.as_address_byte(read_or_write), 0u8];
        spi.transfer(&mut cmd_bytes)?;
        let result = spi.transfer(buffer);
        self.chip_select.set_high().unwrap();
        result
    }

    pub fn modify_register<F, R, const RSIZE: usize>(
        &mut self,
        spi: &mut SPI,
        mut f: F,
    ) -> Result<(), E>
    where
        F: FnMut(&mut R),
        R: PackedStruct<ByteArray = [u8; RSIZE]> + RegisterAddress,
    {
        let mut buffer = [0u8; RSIZE];
        self.read_registers(spi, R::register_address(), &mut buffer)?;
        let mut unpacked = R::unpack(&buffer).unwrap();
        f(&mut unpacked);
        let mut packed = unpacked.pack().unwrap();

        self.write_registers(spi, R::register_address(), &mut packed)?;

        Ok(())
    }
}

pub trait RegisterAddress {
    fn register_address() -> Registers;
}

//pub trait ReadModifyWriteSpiRegister<SPI>
//
//{
//    fn read_registers<'a>(
//        &mut self,
//        spi: &mut SPI,
//        start_address: Registers,
//        buffer: &'a mut [u8],
//    ) -> Result<&'a [u8], E> {
//        self.register_transfer(spi, start_address, buffer, ReadWrite::Read)
//    }
//
//
//}
