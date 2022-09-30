use crate::traits::*;
use crate::types::*;
use nom::bytes::complete::*;
use nom::multi::count;
use nom::IResult;

impl Parse for Rect {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, left) = i32::parse(input)?;
        let (input, right) = i32::parse(input)?;
        let (input, top) = i32::parse(input)?;
        let (input, bottom) = i32::parse(input)?;
        Ok((
            input,
            Self {
                left,
                right,
                top,
                bottom,
            },
        ))
    }
}

impl World {
    pub(crate) fn parse<'input>(input: &'input [u8]) -> nom::IResult<&'input [u8], Self> {
        let original = input;
        let (mut input, version) = i32::parse(input)?;
        if version > 135 {
            let (_input, _) = take(7usize)(input)?;
            input = _input;
        }
        let (input, filetype) = u8::parse(input)?;
        if filetype != 2 {
            // World files are type 2
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Tag,
            )));
        }

        let (input, _) = take(12usize)(input)?;
        let input = if version >= 88 {
            let (input, counts) = i16::parse(input)?;
            let (input, sections) = count(i32::parse, counts as usize)(input)?;
            dbg!(&sections);

            if original.len() < sections[0] as usize {
                return Err(nom::Err::Error(nom::error::Error::new(
                    input,
                    nom::error::ErrorKind::Tag,
                )));
            }

            let (input, header) = Header::parse(&original[(sections[0] as usize)..])?;

            dbg!((original.len() - input.len()) + sections[0] as usize);
            dbg!(header);

            input
        } else {
            input
        };

        let (input, rest) = take(input.len())(input)?;
        Ok((
            input,
            Self {
                version,
                filetype,
                sections: Default::default(),
                rest: rest.to_vec(),
            },
        ))
    }
}

impl Parse for String {
    fn parse(input: &[u8]) -> IResult<&[u8], String> {
        use nom::number::complete::u8;
        let (input, len) = u8(input)?;
        let (input, string) = take(len as usize)(input)?;
        Ok((input, String::from_utf8_lossy(string).to_string()))
    }
}

impl Parse for bool {
    fn parse(input: &[u8]) -> IResult<&[u8], bool> {
        use nom::number::complete::u8;
        let (input, byte) = u8(input)?;
        Ok((input, byte != 0))
    }
}

macro_rules! impl_parse_le_num {
    ($($t:ty),*) => {
        $(
            paste::paste! {
                impl Parse for $t {
                    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
                        use nom::number::complete::[<le_ $t>];
                        [<le_ $t>](input)
                    }
                }
            }
        )*
    };

}

impl_parse_le_num!(i16, i32, i64, f64, f32);

impl Parse for u8 {
    fn parse(input: &[u8]) -> IResult<&[u8], u8> {
        use nom::number::complete::u8;
        let (input, byte) = u8(input)?;
        Ok((input, byte))
    }
}

macro_rules! impl_parse_array {
    ($($t:expr),*) => {
        $(
            impl<T: Parse> Parse for [T; $t] {
                fn parse(input: &[u8]) -> IResult<&[u8], Self> {
                    let (input, array) = count(T::parse, $t)(input)?;
                    Ok((input, array.try_into().map_err(|_| nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Tag)))?))
                }
            }
        )*
    };
}

impl_parse_array!(3, 4, 16);
