
use anyhow::Result;
use csv::Reader;
use std::io::Read;
use csv::StringRecord;
use std::fs::File;
use std::io::prelude::*;

pub fn csv_md<R: Read>(album: &mut Reader<R>) -> Result<()> {
    for result in album.records() {
        let record: ::csv::StringRecord = result?;
        print!("importing scrobble: {} from  ", &record[0]);
        print!("{}", &record[1]);
        println!(" scrobbled on {}", &record[2]);
        write_md(&record);
    }
    Ok(())
}

pub fn write_md(data: &StringRecord) -> Result<(), Box<dyn std::error::Error>> {
    println!("writing file....");
    print!("test {}", &data[0]);
    let path = Path::new("/vault/Artist/Album");
    if let Some(parent) = str.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut file = File::create("example.md")?;
    Ok(())

}
