//! Implementation of [`Serial`](https://docs.rs/embedded-hal/0.2.1/embedded_hal/serial/index.html)

use hal::serial::{Read, Write};
use nb;
use serial;

/// Newtype around [`serial::SystemPort`] that implements the `embedded-hal` traits
pub struct Serial(pub serial::SystemPort);

impl Read<u8> for Serial {
    type Error = serial::Error;

    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        use std::io::Read;
        let mut buffer = [0; 1];
        let bytes_read = self
            .0
            .read(&mut buffer)
            .map_err(|err| nb::Error::Other(Self::Error::from(err)))?;
        if bytes_read == 1 {
            Ok(buffer[0])
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

impl Write<u8> for Serial {
    type Error = serial::Error;

    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        use std::io::Write;
        self.0
            .write(&[word])
            .map_err(|err| nb::Error::Other(Self::Error::from(err)))?;
        Ok(())
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        use std::io::Write;
        self.0
            .flush()
            .map_err(|err| nb::Error::Other(Self::Error::from(err)))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty() {
        let mut port: Box<Read<u8, Error = serial::Error>> =
            Box::new(Serial(serial::open("/dev/tty1").unwrap()));

        port.read().unwrap();
    }
}
