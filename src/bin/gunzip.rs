use std::io::{stdin, stdout, BufWriter, Result};

use flate2::bufread::MultiGzDecoder;

fn main() -> Result<()> {
    let read = stdin().lock();
    let mut write = BufWriter::new(stdout());

    let mut decoder = MultiGzDecoder::new(read);
    std::io::copy(&mut decoder, &mut write)?;
    Ok(())
}
