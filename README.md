# Dmx-Struct

A rust- module for a struct that holds dmx-address information.

**dmx-struct is in pre-release state. Any breaking changes may be implemented without further notice!**

## Description

This is a simple crate that contains a struct `DMXAddress`. The struct implements the trait `TryFrom` that understands
notation with dot (eg. 1.234, 5.231) and absolute dmx addresses to parse.

The module is designed to never panic but return `DMXParseError` instead

The module holds both, the absolute address and the address separated by universe and address so no calculation needed
for further oparations

## Usage

The main struct `DMXAddress` implements the trait `TryFrom<&str>` so usage is straight forward:

 ```crate
 [dependencies]
 dmx-struct = "0.1.0"
 ```

### Example try from

```rust
 use std::convert::TryFrom;

use dmx_struct::{DMXAddress, DMXParseError};

fn test() {
    let dmx_address: Result<DMXAddress, DMXParseError> = DMXAddress::try_from("1.511");
    let dmx_address: Result<DMXAddress, DMXParseError> = DMXAddress::try_from("1024");
}
 ```

### Example try into

 ```rust
 use std::convert::TryInto;

use dmx_struct::{DMXAddress, DMXParseError};

fn test() {
    let dmx_address: Result<DMXAddress, DMXParseError> = "1.511".try_into();
    let dmx_address: Result<DMXAddress, DMXParseError> = "1024".try_into();
}
 ```