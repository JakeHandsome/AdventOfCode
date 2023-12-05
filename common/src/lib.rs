use std::{
    env,
    error::Error,
    fs::{self, File},
    io::{self, BufRead, BufReader, Lines},
    path::Path,
    time,
};

// Short hand result type for a dyn Error
pub type R<T> = Result<T, Box<dyn Error>>;

pub fn read_input_as_string<P>(input: P) -> String
where
    P: AsRef<Path>,
{
    match download_file_if_doesnt_exist(input.as_ref()) {
        Ok(_) => fs::read_to_string(input).unwrap(),
        Err(err) => panic!("{}", err),
    }
}

pub fn read_input_as_lines<P>(input: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    match download_file_if_doesnt_exist(input.as_ref()) {
        Ok(_) => {
            let file = File::open(input)?;
            let reader = BufReader::new(file);
            Ok(reader.lines())
        }
        Err(err) => panic!("{}", err),
    }
}

#[derive(Clone, Debug)]
pub struct AdventOfCodeError(String);
const DAY_ERR_MESSAGE: &str = "Day folder not in correct format. Folder should end with 2 digits for the date Ex: 01";
const YEAR_ERR_MESSAGE: &str =
    "Year folder not in correct format. Folder should end with 4 digits for the Year Ex: 2022";
impl std::fmt::Display for AdventOfCodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Error for AdventOfCodeError {}
impl AdventOfCodeError {
    pub fn new<S>(s: S) -> AdventOfCodeError
    where
        S: Into<String>,
    {
        AdventOfCodeError(s.into())
    }
}

fn download_file_if_doesnt_exist(input: &Path) -> Result<(), Box<dyn Error>> {
    if input.is_file() {
        // file already exists, no need to do anthing
        Ok(())
    } else if input.file_name().expect("Input should be a file") == "Input.txt" {
        // We can only download Input.txt, there is no API for sample
        // Get the folder name to determine year and day
        let mut it = input.iter().rev();
        let _ = it.next(); // This is the file name it can be ignored
        let day = it
            .next()
            .ok_or_else(|| Box::new(AdventOfCodeError::new(DAY_ERR_MESSAGE)))?
            .to_str()
            .ok_or_else(|| Box::new(AdventOfCodeError::new(DAY_ERR_MESSAGE)))?; //This is the folder with the day in the form (day##)
        let year = it
            .next()
            .ok_or_else(|| Box::new(AdventOfCodeError::new(YEAR_ERR_MESSAGE)))?
            .to_str()
            .ok_or_else(|| Box::new(AdventOfCodeError::new(YEAR_ERR_MESSAGE)))?;
        let day = &day[day.len() - 2..].parse::<usize>()?;
        let year = &year[year.len() - 4..];
        // Session token to come from envvar
        let session = env::var("AOC_SESSION")?;
        let client = reqwest::blocking::Client::new();
        let res = client
            .get(format!("https://adventofcode.com/{}/day/{}/input", year, day))
            .header("Cookie", format!("session={}", session))
            .send()?;
        fs::write(input, res.text()?)?;
        Ok(())
    } else {
        Err(Box::new(AdventOfCodeError::new(
            "Cannot download files other than 'Input.txt'",
        )))
    }
}

pub struct Timer {
    message: String,
    start_time: time::SystemTime,
}

impl Timer {
    pub fn new<S>(message: S) -> Self
    where
        S: Into<String>,
    {
        Timer {
            message: message.into(),
            start_time: time::SystemTime::now(),
        }
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        println!("Time of {}, {:?}", self.message, self.start_time.elapsed().unwrap())
    }
}

/// Macro for some boiler plate code to read in the Input.txt file for the given package
#[macro_export]
macro_rules! read_input_file_for_project {
    () => {
        common::read_input_as_lines(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("Input.txt"))
            .expect("Failed to load input file")
    };
}
#[macro_export]
macro_rules! read_input_file_for_project_as_string {
    () => {
        common::read_input_as_string(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("Input.txt"))
    };
}

