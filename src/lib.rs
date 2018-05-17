extern crate nom;

use std::io::{self, Read};


pub fn read<P, O, E, R>(parser: P, mut rdr: R) -> Result<O, E>
    where R: Read,
          E: From<nom::ErrorKind> + From<io::Error>,
          P: Fn(&[u8]) -> nom::IResult<&[u8], O>,
{
    let mut input: Vec<u8> = Vec::new();
    loop {
        let to_read = match parser(&input) {
            Ok((_, parsed)) => return Ok(parsed),
            Err(nom::Err::Incomplete(needed)) => {
                match needed {
                    nom::Needed::Unknown => 1,     // read one byte
                    nom::Needed::Size(len) => len,
                }
            },
            Err(e) => return Err(e.into_error_kind().into()),
        };

        (&mut rdr).take(to_read as u64).read_to_end(&mut input)?;
    }
}
