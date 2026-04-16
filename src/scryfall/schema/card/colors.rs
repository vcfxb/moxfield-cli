use enumflags2::{bitflags, BitFlags};
use serde::de::Error;
use serde::de::{SeqAccess, Unexpected, Visitor};
use serde::{Deserialize, Deserializer};
use std::fmt::{Formatter, Write};

#[bitflags]
#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
pub enum Color { W, U, B, R, G }

pub struct ColorArrayToSet;

pub struct OptionalColorArrayToSet;


pub fn deserialize_with_color_array_to_set<'de, D>(d: D) -> Result<BitFlags<Color>, D::Error>
where D: Deserializer<'de>
{
    d.deserialize_seq(ColorArrayToSet)
}

pub fn deserialize_optional_color_array_to_set<'de, D>(d: D) -> Result<Option<BitFlags<Color>>, D::Error>
where D: Deserializer<'de>
{
    d.deserialize_option(OptionalColorArrayToSet)
}

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

impl<'de> Visitor<'de> for OptionalColorArrayToSet {
    type Value = Option<<ColorArrayToSet as Visitor<'de>>::Value>;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("optional color array")
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(None)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(ColorArrayToSet).map(Some)
    }
}
