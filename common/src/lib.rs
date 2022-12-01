use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader, Lines},
    path::Path,
};

pub fn read_input_as_string<P>(input: P) -> String
where
    P: AsRef<Path>,
{
    fs::read_to_string(input).unwrap()
}


pub fn read_input_as_lines<P>(input: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(input)?;
    let reader = BufReader::new(file);
    Ok(reader.lines())
}

/// Macro for some boiler plate code to read in the Input.txt file for the given package
#[macro_export]
macro_rules! read_input_file_for_project {
    () => {
        common::read_input_as_lines(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("Input.txt"),
        )
        .expect("Failed to load input file")
    };
}
