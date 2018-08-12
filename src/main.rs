#[macro_use]
extern crate structopt;
extern crate rand;

use rand::Rng;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "mkpass")]
struct Options {
    /// Length of the password
    #[structopt(short = "l", long = "length", default_value = "4")]
    length: usize,

    /// Number of passwords
    #[structopt(short = "n", long = "number", default_value = "1")]
    number: usize,

    /// Word separator
    #[structopt(short = "s", long = "separator", default_value = " ")]
    separator: String,

    /// Dictionary to use
    #[structopt(
        short = "d",
        long = "dict",
        default_value = "/usr/share/dict/words",
        parse(from_os_str)
    )]
    dict: std::path::PathBuf,
}

fn main() -> Result<(), String> {
    let opts = Options::from_args();

    let dict = std::fs::read_to_string(&opts.dict)
        .map_err(|e| format!("{}: {}", &opts.dict.display(), e))?;

    let mut dict: Vec<&str> = dict
        .lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect();

    dict.sort_unstable();
    dict.dedup();

    let mut rng = rand::EntropyRng::new();

    let mut sampler = std::iter::repeat_with(|| rng.choose(&dict).unwrap());

    for _ in 0..opts.number {
        let password = sampler
            .by_ref()
            .take(opts.length)
            .map(|s| *s)
            .collect::<Vec<&str>>()
            .join(&opts.separator);
        println!("{}", password);
    }

    Ok(())
}
