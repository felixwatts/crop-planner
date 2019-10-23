use std::convert::TryFrom;
use json::JsonValue;
use crate::common::*;

pub const BED_FLAG_NONE: u8 = 0b0;
pub const BED_FLAG_POLYTUNNEL: BedFlags = BedFlags(0b1);

#[derive(Clone, PartialEq, Debug)]
pub struct BedFlags(pub u8);

impl BedFlags {
    pub fn has_all(&self, flags: &BedFlags) -> bool {
        (flags.0 & self.0) == flags.0
    }
}

#[cfg(test)]
#[test]
fn bed_flag_has_all() {
    let test_flag_1: u8 = 0b01;
    let test_flag_2: u8 = 0b10;
    let flags_a = BedFlags(test_flag_1);
    let flags_b = BedFlags(test_flag_1 | test_flag_2);
    assert!(flags_b.has_all(&flags_a));
    assert!(!flags_a.has_all(&flags_b));
}

#[derive(Clone)]
pub struct Bed {
    pub name: String,
    pub flags: BedFlags,
}

impl TryFrom<&JsonValue> for BedFlags {
    type Error = &'static str;
    fn try_from(item: &JsonValue) -> Result<Self, Self::Error> {
        let arr = as_array(item)?;
        let mut flags = BED_FLAG_NONE;
        for flag_name in arr.iter() {
            if flag_name == "polytunnel" {
                flags |= BED_FLAG_POLYTUNNEL.0
            }
        }
        return Ok(BedFlags(flags));
    }
}

#[cfg(test)]
#[test]
fn bed_flags_from_json() {
    let js = json::parse(r#"[ "polytunnel" ]"#).expect("test is wrong");
    let bed_flags = BedFlags::try_from(&js).expect("failed to parse");
    assert_eq!(bed_flags, BED_FLAG_POLYTUNNEL);
}

impl TryFrom<&JsonValue> for Bed {
    type Error = &'static str;
    fn try_from(item: &JsonValue) -> Result<Self, Self::Error> {
        let name = as_string(&item["name"])?;
        let flags = BedFlags::try_from(&item["flags"])?;
        Ok(Bed {
            name: String::from(name),
            flags: flags,
        })
    }
}

#[cfg(test)]
#[test]
fn bed_from_json() {
    let js = json::parse(r#"
{
    "name": "~b00",
    "flags": [ "polytunnel" ]
}"#).expect("test is wrong");
    let bed = Bed::try_from(&js).expect("failed to parse");
    assert_eq!(bed.name, "~b00");
    assert_eq!(bed.flags, BED_FLAG_POLYTUNNEL);
}

