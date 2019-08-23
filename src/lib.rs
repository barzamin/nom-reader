extern crate nom;

use std::io::{self, Read};

pub fn read<P, T, E, R, Any>(parser: P, mut rdr: R) -> Result<T, E>
where
    R: Read,
    E: From<io::Error>,
    P: Fn(&[u8]) -> Result<T, nom::Err<E>>,
{
    let mut input = Vec::new();
    loop {
        let to_read = match parser(&input) {
            Ok(parsed) => return Ok(parsed),
            Err(nom::Err::Incomplete(needed)) => {
                match needed {
                    nom::Needed::Unknown => 1, // read one byte
                    nom::Needed::Size(len) => len,
                }
            }
            Err(nom::Err::Failure(e)) | Err(nom::Err::Error(e)) => return Err(e),
        };

        (&mut rdr).take(to_read as u64).read_to_end(&mut input)?;
    }
}
