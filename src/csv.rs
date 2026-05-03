
use anyhow::Result;
use csv::Reader;
use std::io::Read;

pub fn csv_md<R: Read>(album: &mut Reader<R>) -> Result<()> {
    for result in album.records() {
        let record = result?;
        println!("col 0 = {}", &record[0]);
    }
    Ok(())
}
