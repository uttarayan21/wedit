pub trait Parse {
    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self>
    where
        Self: Sized;
}

