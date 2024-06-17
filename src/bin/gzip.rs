use std::io::{stdin, stdout, BufWriter, Result};

use flate2::{bufread::GzEncoder, Compression};

fn main() -> Result<()> {
    let read = stdin().lock();
    let mut write = BufWriter::new(stdout());

    let mut encoder = GzEncoder::new(read, Compression::best());
    std::io::copy(&mut encoder, &mut write)?;
    Ok(())
}
