use enumflags2::{bitflags, BitFlags};
use serde::de::Error;
use serde::de::{SeqAccess, Unexpected, Visitor};
use serde::{Deserialize, Deserializer};
use std::fmt::{Formatter, Write};

#[bitflags]
#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
pub enum Color { W, U, B, R, G }

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Colors(pub BitFlags<Color>);

impl<'de> Deserialize<'de> for Colors {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        deserializer.deserialize_seq(ColorArrayToSet).map(Colors)
    }
}

pub struct ColorArrayToSet;


impl<'de> Visitor<'de> for ColorArrayToSet {
    type Value = BitFlags<Color>;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("optional color array")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut colors = BitFlags::<Color>::empty();

        while let Some(next) = seq.next_element::<char>()? {
            match next {
                'W' => colors |= Color::W,
                'U' => colors |= Color::U,
                'B' => colors |= Color::B,
                'R' => colors |= Color::R,
                'G' => colors |= Color::G,
                _ => return Err(Error::invalid_value(Unexpected::Char(next), &self))
            }
        }

        Ok(colors)
    }
}
