use rayon::prelude::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, BufWriter, Error, Write},
    path::Path,
    str,
};

extern crate linereader;
use linereader::LineReader;

pub fn filter_file<P>(file: P, cats: P, cats_column: usize, outfile: P) -> Result<bool, Error>
where
    P: AsRef<Path>,
{
    let mut keywords: Vec<&str> = Vec::new();

    let tmp = BufReader::new(File::open(cats)?)
        .lines()
        .map(|f| f.unwrap())
        .collect::<Vec<String>>();
    tmp.par_iter()
        .map(String::as_str)
        .collect_into_vec(&mut keywords);

    let mut out_file = BufWriter::new(
        OpenOptions::new()
            .truncate(true)
            .write(true)
            .create(true)
            .open(outfile)?,
    );

    let mut lines = LineReader::new(File::open(file)?);

    while let Some(line) = lines.next_line() {
        let line_unwrapped = match str::from_utf8(line?) {
            Ok(ln) => ln,
            Err(_) => {
                println!("Error while converting the string ti utf8");
                continue;
            }
        };

        let category = match line_unwrapped.split_whitespace().nth(cats_column) {
            Some(c) => c,
            None => continue,
        };

        if keywords.iter().all(|&key| !category.starts_with(key)) {
            out_file
                .write_all(line_unwrapped.as_bytes())
                .expect("Couldn't write to file");
        }
    }

    _ = out_file.flush();

    Ok(true)
}
