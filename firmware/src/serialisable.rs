use core::fmt::{Debug, Display};

use embedded_hal::serial::{Read, Write};
use nb::block;
use thiserror_no_std::Error;

use binary_serde::BinarySerde;

#[derive(Error, Debug)]
pub enum Error<SerialError: Display> {
    #[error("Writing to serial device failed: {0}")]
    SerialWrite(SerialError),
    #[error("Reading from serial device failed: {0}")]
    SerialRead(SerialError),
    #[error("Deserialization failed: {0}")]
    Deserialize(#[from] binary_serde::DeserializeError),
}

pub trait SerialWriteDevice<E: Display>: Write<u8, Error = E> {
    fn serialize<T: BinarySerde>(&mut self, val: &T) -> Result<(), Error<E>>
    where
        [(); T::SERIALIZED_SIZE]:,
    {
        let mut buffer = [0; T::SERIALIZED_SIZE];
        val.binary_serialize(&mut buffer, binary_serde::Endianness::Big);
        for b in buffer {
            block!(self.write(b)).map_err(Error::SerialWrite)?;
        }
        Ok(())
    }
}
impl<E: Display, T: Write<u8, Error = E>> SerialWriteDevice<E> for T {}

pub trait SerialReadDevice<E: Display>: Read<u8, Error = E> {
    fn deserialize<T: BinarySerde>(&mut self) -> Result<T, Error<E>>
    where
        [(); T::SERIALIZED_SIZE]:,
    {
        let mut buffer = [0; T::SERIALIZED_SIZE];
        for i in buffer.iter_mut() {
            *i = block!(self.read()).map_err(Error::SerialRead)?;
        }
        let result = T::binary_deserialize(&buffer, binary_serde::Endianness::Big)?;
        Ok(result)
    }
}
impl<E: Display, T: Read<u8, Error = E>> SerialReadDevice<E> for T {}
