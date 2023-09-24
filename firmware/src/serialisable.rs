use embedded_hal::serial::{Read, Write};
use nb::block;



//pub trait SerialWriteDevice<E>: Write<u8, Error = E> {
//    fn serialize_to_serial<T: BinarySerde>(&mut self, val: &T) -> Result<(), E>
//    where
//        [(); T::MAX_SERIALIZED_SIZE]:,
//    {
//        let buffer = [0; T::MAX_SERIALIZED_SIZE];
//        val.binary_serialize(&mut buffer, binary_serde::Endianness::Big);
//        for b in buffer {
//            block!(self.write(b))?;
//        }
//        Ok(())
//    }
//}
//impl<E, T: Write<u8, Error = E>> SerialWriteDevice<E> for T {}
//
//pub trait SerialReadDevice<E>: Read<u8, Error = E> {
//    fn deserialize_from_serial<T: BinarySerde>(&mut self) -> Result<T, E>
//    where
//        [(); T::MAX_SERIALIZED_SIZE]:,
//    {
//        let buffer = [0; T::MAX_SERIALIZED_SIZE];
//        for (i, b) in buffer.iter().enumerate() {
//            buffer[i] = block!(self.read())?;
//        }
//        T::binary_deserialize(&buffer, binary_serde::Endianness::Big)
//    }
//}
//impl<E, T: Read<u8, Error = E>> SerialReadDevice<E> for T {}
