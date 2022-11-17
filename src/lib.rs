use std::error::Error;
use clap::App;

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config)-> MyResult<()>{
    dbg!(config);
    Ok(())
}

#[derive(Debug)]
pub struct Config{
    files:Vec<String>,
    number_lines: bool,
    number_nonblank_lines:bool,
}

pub fn get_args()-> MyResult<Config>{
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Evgen")
        .about("Rust cat")
        .get_matches();

    Ok(Config{
        files,
        number_lines,
        number_nonblank_lines,
    })
}