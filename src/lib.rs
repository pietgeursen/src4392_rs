#![no_std]
use core::{fmt::Debug, marker::PhantomData};

use embedded_hal::digital::v2::OutputPin;
use embedded_hal::spi::MODE_3;
use embedded_hal::{
    blocking::{delay::DelayMs, spi::Transfer},
    spi::Mode,
};
use packed_struct::prelude::*;
pub use port_control::{
    AudioFormat, OutputDataSource, PortAControl1Register, PortAControl2Register,
    PortBControl1Register, PortBControl2Register, PortClockSource, PortMasterClockDivider,
};
use registers::Registers;
pub use sample_rate_converter::{
    Deemphasis, InterpolationFilterGroupDelay, SrcClockSource, SrcControl1, SrcControl2, SrcSource, SrcRatio,
};

pub mod interrupt;
pub mod port_control;
pub mod registers;
pub mod reset;
pub mod sample_rate_converter;

use reset::Reset;

#[derive(Copy, Clone, Debug)]
pub enum Port {
    A,
    B,
}

pub struct Src4392<P, SPI, E, D, DT>
where
    P: OutputPin,
    <P as embedded_hal::digital::v2::OutputPin>::Error: Debug,
    SPI: Transfer<u8, Error = E>,
    D: DelayMs<DT>,
    DT: From<u8>
{
    chip_select: P,
    spi: PhantomData<SPI>,
    delay: D,
    delay_type: PhantomData<DT>
}

impl<P, SPI, E, D, DT> Src4392<P, SPI, E, D, DT>
where
    P: OutputPin,
    <P as embedded_hal::digital::v2::OutputPin>::Error: Debug,
    SPI: Transfer<u8, Error = E>,
    D: DelayMs<DT>,
    DT: From<u8>
{
    pub const SPI_MODE: Mode = MODE_3;

    pub fn new(chip_select: P, delay: D) -> Self
    where
        SPI: Transfer<u8, Error = E>,
        E: Debug,
    {
        Self {
            chip_select,
            spi: PhantomData,
            delay,
            delay_type: PhantomData
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
        is_master: bool,
    ) -> Result<(), E> {
        match port {
            Port::A => {
                self.modify_register(spi, |reg: &mut PortAControl1Register| {
                    reg.am_slave = is_master;
                    reg.afmt = audio_format;
                    reg.aout = output_data_source;
                    reg.amute = false;
                })?;

                self.modify_register(spi, |reg: &mut PortAControl2Register| {
                    reg.adiv = clock_divider;
                    reg.aclk = clock_source;
                })
            }
            Port::B => {
                self.modify_register(spi, |reg: &mut PortBControl1Register| {
                    reg.am_slave = is_master;
                    reg.afmt = audio_format;
                    reg.aout = output_data_source;
                    reg.amute = false;
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
            reg.mute = false;
            reg.track = true;
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
}

impl<P, SPI, E, D, DT> ReadModifyWriteSpiRegister<SPI, E, Registers> for Src4392<P, SPI, E, D, DT>
where
    P: OutputPin,
    <P as embedded_hal::digital::v2::OutputPin>::Error: Debug,
    SPI: Transfer<u8, Error = E>,
    D: DelayMs<DT>,
    DT: From<u8>
{
    fn assert_cs(&mut self) {
        self.chip_select.set_low().unwrap();
    }

    fn deassert_cs(&mut self) {
        self.chip_select.set_high().unwrap();
        self.delay.delay_ms(1.into());
    }
}

#[derive(Copy, Clone)]
pub enum ReadWrite {
    Read,
    Write,
}

/// Each packed struct should implement this trait and return the specific Registers variant that
/// it corresponds to.
pub trait RegisterAddress<R> {
    fn register_address() -> R;
}

/// Implement this trait on your Registers enum. Often you will set a bit in part of the address to
/// denote a read or write.
pub trait AsAddressByte {
    fn as_address_byte(&self, rw: ReadWrite) -> u8;
}

pub trait ReadModifyWriteSpiRegister<SPI, SPIERROR, REGISTERS>
where
    SPI: Transfer<u8, Error = SPIERROR>,
    REGISTERS: AsAddressByte,
{
    fn read_registers<'a>(
        &mut self,
        spi: &mut SPI,
        start_address: REGISTERS,
        buffer: &'a mut [u8],
    ) -> Result<&'a [u8], SPIERROR> {
        self.register_transfer(spi, start_address, buffer, ReadWrite::Read)
    }

    fn write_registers<'a>(
        &mut self,
        spi: &mut SPI,
        start_address: REGISTERS,
        buffer: &'a mut [u8],
    ) -> Result<&'a [u8], SPIERROR> {
        self.register_transfer(spi, start_address, buffer, ReadWrite::Write)
    }

    fn register_transfer<'a>(
        &mut self,
        spi: &mut SPI,
        start_address: REGISTERS,
        buffer: &'a mut [u8],
        read_or_write: ReadWrite,
    ) -> Result<&'a [u8], SPIERROR> {
        self.assert_cs();
        let mut cmd_bytes = [start_address.as_address_byte(read_or_write), 0u8];
        spi.transfer(&mut cmd_bytes)?;
        let result = spi.transfer(buffer);
        self.deassert_cs();
        result
    }

    fn modify_register<F, R, const RSIZE: usize>(
        &mut self,
        spi: &mut SPI,
        mut f: F,
    ) -> Result<(), SPIERROR>
    where
        F: FnMut(&mut R),
        R: PackedStruct<ByteArray = [u8; RSIZE]> + RegisterAddress<REGISTERS>,
    {
        let mut buffer = [0u8; RSIZE];
        self.read_registers(spi, R::register_address(), &mut buffer)?;
        let mut unpacked = R::unpack(&buffer).unwrap();
        f(&mut unpacked);
        let mut packed = unpacked.pack().unwrap();

        self.write_registers(spi, R::register_address(), &mut packed)?;

        Ok(())
    }

    fn assert_cs(&mut self);
    fn deassert_cs(&mut self);
}
