use std::io::{Result, stdin, stdout};

use flate2::bufread::GzDecoder;

fn main() -> Result<()> {
    let read = stdin().lock();
    let mut write = stdout();

    let mut decoder = GzDecoder::new(read);
    std::io::copy(&mut decoder, &mut write)?;
    Ok(())
}