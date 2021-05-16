//!contains a testcase for documentation. If this code is rewritten it must be changed in readme and the documentation of lib.rs also.
#![cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use crate::{DMXAddress, DMXParseError};

    #[test]
    fn test() {
        let dmx_address: Result<DMXAddress, DMXParseError> = DMXAddress::try_from("1.511");
        println!("{:?}", dmx_address);
        let dmx_address: Result<DMXAddress, DMXParseError> = DMXAddress::try_from("1024");
        println!("{:?}", dmx_address);
    }
}