use std::io::{stdin, stdout, Result};

use flate2::{bufread::GzEncoder, Compression};

fn main() -> Result<()> {
    let read = stdin().lock();
    let mut write = stdout();

    let mut encoder = GzEncoder::new(read, Compression::best());
    std::io::copy(&mut encoder, &mut write)?;
    Ok(())
}
