//! A module for a struct that holds dmx-address information.
//!
//! **dmx-struct is in pre-release state. Any breaking changes may be implemented without further notice!**
//!
//! ## Description
//!
//! This is a simple crate that contains a struct `DMXAddress`. The struct implements the trait `TryFrom` that understands notation with dot (eg. 1.234, 5.231) and absolute dmx addresses to parse.
//!
//! The module is designed to never panic but return `DMXParseError` instead
//!
//! The module holds both, the absolute address and the address separated by universe and address so no calculation needed for further oparations
//!
//! ## Usage
//! The main struct `DMXAddress` implements the trait `TryFrom<&str>` so usage is straight forward:
//!
//! ```crate
//! [dependencies]
//! dmx-struct = "0.1.0"
//! ```
//!
//! ### Example try from
//!
//!```rust
//! use std::convert::TryFrom;
//!
//! use dmx_struct::{DMXAddress, DMXParseError};
//!
//! fn test() {
//!     let dmx_address: Result<DMXAddress, DMXParseError> = DMXAddress::try_from("1.511");
//!     let dmx_address: Result<DMXAddress, DMXParseError> = DMXAddress::try_from("1024");
//! }
//! ```
//!
//! ### Example try into
//!
//! ```rust
//! use std::convert::TryInto;
//!
//! use dmx_struct::{DMXAddress, DMXParseError};
//!
//! fn test() {
//!     let dmx_address: Result<DMXAddress, DMXParseError> = "1.511".try_into();
//!     let dmx_address: Result<DMXAddress, DMXParseError> = "1024".try_into();
//! }
//! ```

use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[cfg(test)]
mod doc_test;

///This Error is return if an invalid &str is tried to be deparsed as dmx-address instead of panicing
#[derive(Debug)]
pub struct DMXParseError;

impl std::fmt::Display for DMXParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "something went terribly wrong")
    }
}

impl Error for DMXParseError {}

///DMXAddress color representation used in GDTF
#[derive(Debug)]
pub struct DMXAddress {
    ///Universe of the DMXAddress starting from 1
    pub universe: u16,
    ///The address in the dmx universe (1-512)
    pub address: u16,
    ///The absolute dmx address including the universe (1-32767488)
    pub absolute: u32,
}

impl TryFrom<&str> for DMXAddress {
    type Error = DMXParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let universe;
        let address;
        let absolute;

        if value.contains(".") {
            //The input is of format 1.234
            //Splitting the input by .
            let value: Vec<&str> = value.split(".").collect();
            //Only one . allowed in this format
            if value.len() != 2 { return Err(DMXParseError {}); }
            //Value before . is universe
            universe = u32::from_str(value[0]).or_else(|_| Err(DMXParseError {}))?;
            //If the universe is 0, the input was not valid
            if universe == 0 { return Err(DMXParseError {}); }
            //Value after . is address
            address = u32::from_str(value[1]).or_else(|_| Err(DMXParseError {}))?;
            //calculating the absolute address from universe and address
            absolute = address + ((universe - 1) * 512);
        } else {
            //The input holds the absolute address
            absolute = u32::from_str(value).or_else(|_| { Err(DMXParseError {}) })?;
            //Calculating the address from the absolute address
            let x = absolute % 512;
            //Special case if the address is 512 the % operator will return 0 but should return 512 because dmx starts counting at 1
            address = if x > 0 { x } else { 512 };
            if x > 0 {
                //If address was not 512 adding one to the universe because dmx starts counting at 1
                universe = (absolute / 512) + 1;
            } else {
                //If address was 512 not adding one to the universe because dmx starts counting at 1
                universe = absolute / 512;
            }
        }
        //Some dmx validity checks.
        //63'999 is max number of universes supported by sACN
        //dmx address is max 512 by definition
        //address 0 and universe 0 are not valid. Start count at 1
        if universe > 63_999 || address > 512 || address == 0 || universe == 0 {
            return Err(DMXParseError {});
        }
        Ok(DMXAddress {
            universe: universe.try_into().unwrap(),
            address: address.try_into().unwrap(),
            absolute: absolute,
        })
    }
}

///Dmx addresses can be compared with ==
impl PartialEq for DMXAddress {
    fn eq(&self, other: &Self) -> bool {
        self.universe == other.universe && self.address == other.address && self.absolute == other.absolute
    }
}

///Dmx addresses can be used in format with {}. It will return the format 'universe.address'
impl Display for DMXAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{:03}", self.universe, self.address)
    }
}

///Some tests
#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use crate::DMXAddress;

    #[test]
    fn test_valid_separated() {
        assert_eq!(
            DMXAddress { universe: 4, address: 465, absolute: 2001 },
            DMXAddress::try_from("4.465").unwrap()
        );
    }

    #[test]
    fn test_valid_separated_2() {
        assert_eq!(
            DMXAddress { universe: 5, address: 1, absolute: 2049 },
            DMXAddress::try_from("5.1").unwrap()
        );
    }

    #[test]
    fn test_valid_separated_3() {
        assert_eq!(
            DMXAddress { universe: 4, address: 512, absolute: 2048 },
            DMXAddress::try_from("4.512").unwrap()
        );
    }

    #[test]
    fn test_valid_separated_4() {
        assert_eq!(
            DMXAddress { universe: 2, address: 4, absolute: 516 },
            DMXAddress::try_from("2.4").unwrap()
        );
    }

    #[test]
    fn test_valid_separated_5() {
        assert_eq!(
            DMXAddress { universe: 1, address: 1, absolute: 1 },
            DMXAddress::try_from("1.1").unwrap()
        );
    }

    #[test]
    fn test_valid_separated_6() {
        assert_eq!(
            DMXAddress { universe: 2, address: 1, absolute: 513 },
            DMXAddress::try_from("2.1").unwrap()
        );
    }

    #[test]
    fn test_valid_separated_7() {
        assert_eq!(
            DMXAddress { universe: 1, address: 512, absolute: 512 },
            DMXAddress::try_from("1.512").unwrap()
        );
    }

    #[test]
    fn test_valid_single() {
        assert_eq!(
            DMXAddress { universe: 1, address: 224, absolute: 224 },
            DMXAddress::try_from("224").unwrap()
        );
    }

    #[test]
    fn test_valid_single_2() {
        assert_eq!(
            DMXAddress { universe: 3, address: 210, absolute: 1234 },
            DMXAddress::try_from("1234").unwrap()
        );
    }

    #[test]
    fn test_valid_single_3() {
        assert_eq!(
            DMXAddress { universe: 4, address: 1, absolute: 1537 },
            DMXAddress::try_from("1537").unwrap()
        );
    }

    #[test]
    fn test_valid_single_4() {
        assert_eq!(
            DMXAddress { universe: 3, address: 512, absolute: 1536 },
            DMXAddress::try_from("1536").unwrap()
        );
    }


    #[test]
    fn test_valid_single_5() {
        assert_eq!(
            DMXAddress { universe: 2, address: 1, absolute: 513 },
            DMXAddress::try_from("513").unwrap()
        );
    }

    #[test]
    fn test_valid_single_6() {
        assert_eq!(
            DMXAddress { universe: 1, address: 512, absolute: 512 },
            DMXAddress::try_from("512").unwrap()
        );
    }

    #[test]
    fn test_valid_single_7() {
        assert_eq!(
            DMXAddress { universe: 256, address: 512, absolute: 131072 },
            DMXAddress::try_from("131072").unwrap()
        );
    }

    #[test]
    fn test_invalid_1() {
        match DMXAddress::try_from("something invalid") {
            Ok(_) => { panic!("test_invalid should return an error"); }
            Err(_) => {}
        }
    }

    #[test]
    fn test_invalid_2() {
        match DMXAddress::try_from("2.") {
            Ok(_) => { panic!("test_invalid should return an error"); }
            Err(_) => {}
        }
    }

    #[test]
    fn test_invalid_3() {
        match DMXAddress::try_from(".2") {
            Ok(_) => { panic!("test_invalid should return an error"); }
            Err(_) => {}
        }
    }

    #[test]
    fn test_invalid_4() {
        match DMXAddress::try_from(".") {
            Ok(_) => { panic!("test_invalid should return an error"); }
            Err(_) => {}
        }
    }

    #[test]
    fn test_invalid_5() {
        match DMXAddress::try_from("0.1") {
            Ok(_) => { panic!("test_invalid should return an error"); }
            Err(_) => {}
        }
    }

    #[test]
    fn test_invalid_6() {
        match DMXAddress::try_from("2.0") {
            Ok(_) => { panic!("test_invalid should return an error"); }
            Err(_) => {}
        }
    }

    #[test]
    fn test_invalid_7() {
        match DMXAddress::try_from("0.0") {
            Ok(_) => { panic!("test_invalid should return an error"); }
            Err(_) => {}
        }
    }

    #[test]
    fn test_invalid_8() {
        match DMXAddress::try_from("2.513") {
            Ok(_) => { panic!("test_invalid should return an error"); }
            Err(_) => {}
        }
    }

    #[test]
    fn test_invalid_9() {
        match DMXAddress::try_from("63999.513") {
            Ok(_) => { panic!("test_invalid should return an error"); }
            Err(_) => {}
        }
    }

    #[test]
    fn test_invalid_10() {
        match DMXAddress::try_from("0") {
            Ok(_) => { panic!("test_invalid should return an error"); }
            Err(_) => {}
        }
    }

    #[test]
    fn test_invalid_11() {
        match DMXAddress::try_from("98981265123519681981681514984984984464984984") {
            Ok(_) => { panic!("test_invalid should return an error"); }
            Err(_) => {}
        }
    }

    #[test]
    fn test_invalid_12() {
        match DMXAddress::try_from("-3") {
            Ok(_) => { panic!("test_invalid should return an error"); }
            Err(_) => {}
        }
    }

    #[test]
    fn test_invalid_13() {
        match DMXAddress::try_from("-1.3") {
            Ok(_) => { panic!("test_invalid should return an error"); }
            Err(_) => {}
        }
    }

    #[test]
    fn test_invalid_14() {
        match DMXAddress::try_from("1.-3") {
            Ok(_) => { panic!("test_invalid should return an error"); }
            Err(_) => {}
        }
    }

    #[test]
    fn test_invalid_15() {
        match DMXAddress::try_from("-1.-4") {
            Ok(_) => { panic!("test_invalid should return an error"); }
            Err(_) => {}
        }
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", DMXAddress { universe: 1, address: 342, absolute: 342 }), "1.342");
    }

    #[test]
    fn test_display_2() {
        assert_eq!(format!("{}", DMXAddress { universe: 1, address: 12, absolute: 12 }), "1.012");
    }

    #[test]
    fn test_display_3() {
        assert_eq!(format!("{}", DMXAddress { universe: 1, address: 9, absolute: 9 }), "1.009");
    }
}