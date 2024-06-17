use std::io::{stdin, stdout, Result};

use flate2::bufread::MultiGzDecoder;

fn main() -> Result<()> {
    let read = stdin().lock();
    let mut write = stdout();

    let mut decoder = MultiGzDecoder::new(read);
    std::io::copy(&mut decoder, &mut write)?;
    Ok(())
}
