//!contains a testcase for documentation. If this code is rewritten it must be changed in readme and the documentation of lib.rs also.
#![cfg(test)]
mod tests {
    use std::convert::TryInto;

    use crate::{DMXAddress, DMXParseError};

    #[test]
    fn test() {
        let dmx_address: Result<DMXAddress, DMXParseError> = "1.511".try_into();
        println!("{:?}", dmx_address);
        let dmx_address: Result<DMXAddress, DMXParseError> = "1024".try_into();
        println!("{:?}", dmx_address);
    }
}
