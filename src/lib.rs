#![no_std]
use core::fmt::Debug;

use embedded_hal::blocking::spi::Transfer;
use embedded_hal::digital::v2::OutputPin;
use packed_struct::prelude::*;
use registers::Registers;

pub mod interrupt;
pub mod port_control;
pub mod registers;
pub mod reset;
pub mod sample_rate_converter;

pub use registers::ReadWrite;
use reset::Reset;

pub struct Src4392<P>
where
    P: OutputPin,
    <P as embedded_hal::digital::v2::OutputPin>::Error: Debug,
{
    chip_select: P,
}

impl<P> Src4392<P>
where
    P: OutputPin,
    <P as embedded_hal::digital::v2::OutputPin>::Error: Debug,
{
    pub fn new<SPI, E>(chip_select: P, spi: &mut SPI) -> Self
    where
        SPI: Transfer<u8, Error = E>,
        E: Debug,
    {
        let mut s = Self { chip_select };
        s.modify_register(spi, Reset::ADDRESS, |reg: &mut Reset| {
            reg.reset = true;
        })
        .unwrap();

        s
    }

    pub fn read_registers<'a, SPI, E>(
        &mut self,
        spi: &mut SPI,
        start_address: Registers,
        buffer: &'a mut [u8],
    ) -> Result<&'a [u8], E>
    where
        SPI: Transfer<u8, Error = E>,
        E: Debug,
    {
        self.register_transfer(spi, start_address, buffer, ReadWrite::Read)
    }

    pub fn write_registers<'a, SPI, E>(
        &mut self,
        spi: &mut SPI,
        start_address: Registers,
        buffer: &'a mut [u8],
    ) -> Result<&'a [u8], E>
    where
        SPI: Transfer<u8, Error = E>,
        E: Debug,
    {
        self.register_transfer(spi, start_address, buffer, ReadWrite::Write)
    }

    pub fn register_transfer<'a, SPI, E>(
        &mut self,
        spi: &mut SPI,
        start_address: Registers,
        buffer: &'a mut [u8],
        read_or_write: ReadWrite,
    ) -> Result<&'a [u8], E>
    where
        SPI: Transfer<u8, Error = E>,
        E: Debug,
    {
        self.chip_select.set_low().unwrap();
        let mut cmd_bytes = [start_address.as_address_byte(read_or_write), 0u8];
        spi.transfer(&mut cmd_bytes)?;
        let result = spi.transfer(buffer);
        self.chip_select.set_high().unwrap();
        result
    }

    pub fn modify_register<SPI, E, F, R, const RSIZE: usize>(
        &mut self,
        spi: &mut SPI,
        register_address: Registers,
        mut f: F,
    ) -> Result<(), E>
    where
        E: Debug,
        SPI: Transfer<u8, Error = E>,
        F: FnMut(&mut R),
        R: PackedStruct<ByteArray = [u8; RSIZE]>,
    {
        let mut buffer = [0u8; RSIZE];
        self.read_registers(spi, register_address, &mut buffer)?;
        let mut unpacked = R::unpack(&buffer).unwrap();
        f(&mut unpacked);
        let mut packed = unpacked.pack().unwrap();

        self.write_registers(spi, register_address, &mut packed)?;

        Ok(())
    }
}

trait Register {
    fn addr() -> u8;
    fn get_addr(&self) -> u8 {
        Self::addr()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
