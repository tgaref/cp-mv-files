extern crate clap;
use clap::{App, Arg, Values};
use std::path::Path;
use std::fs;
use std::ffi::{OsStr,OsString};
use std::io::{self, Result};
use std::collections::HashSet;

#[derive (Debug)]
pub struct Config {
    src  : String,
    dst  : String,
    ext  : HashSet<OsString>,
    rec  : bool,
}

impl Config {
    pub fn new(src: String, dst: String, ext: HashSet<OsString>, rec: bool) -> Config {
        Config {src, dst, ext, rec}
    }
}

pub fn parse_arguments() -> Config {
        let matches = App::new("My Copy Program")
        .version("0.1")
        .author("Theo G. <tgaref@gmail.com>")
        .about("Copies files with given extensions (recursively)")
        .arg(Arg::with_name("from_dir")
             .help("Sets a source dir")
             .required(true)
             .index(1))
        .arg(Arg::with_name("to_dir")
             .help("Sets target dir")
             .required(true)
             .index(2))
        .arg(Arg::with_name("extension")
             .short("x")
             .long("ext")
             .multiple(true)
             .required(false)
             .value_name("EXT")
             .help("Sets the extension(s) of files to copy"))
        .arg(Arg::with_name("recur")
             .short("r")
             .long("rec")
             .required(false)
             .help("Set if you want to visit subdirectories recursively"))
        .get_matches();

    let frm = matches.value_of("from_dir").unwrap();
    let to = matches.value_of("to_dir").unwrap();
    let ext_values = matches.values_of("extension").unwrap_or(Values::default());
    let rec = matches.is_present("recur");
    let mut ext_set = HashSet::new();
    for item in ext_values {
        ext_set.insert(OsString::from(item));
    };

    return Config::new(frm.to_string(), to.to_string(), ext_set, rec)
}


pub fn run(config: Config, cp: bool) -> Result<()> {
    let dst = Path::new(&config.dst);
    let src = Path::new(&config.src);
    if !dst.exists() {
        fs::create_dir(dst)?;
    } else if !dst.is_dir() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput,
                                  format!("Destination {} already exists and is not a directory", Path::display(dst))));
    }
    cp_or_mv(src, dst, &config.ext, config.rec,cp)?;
    Ok(())
}

fn cp_or_mv(src: &Path, dst: &Path, ext: &HashSet<OsString>, rec: bool, cp: bool) -> Result<()> {
    for entry_result in src.read_dir()? {
        let entry = entry_result?;
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            if rec {
                cp_or_mv(&src.join(entry.file_name()), dst, ext, rec, cp)?
            }
        } else if file_type.is_file() {
            let entry_path = entry.path();
            let default = OsString::from("");
            let entry_ext = entry_path.as_path().extension().unwrap_or_else(|| &default);
            if ext.contains(OsStr::new("*")) || ext.contains(entry_ext) {
                if cp {
                    fs::copy(entry_path.as_path(), dst.join(entry.file_name()))?;
                } else {
                    fs::rename(entry_path.as_path(), dst.join(entry.file_name()))?;
                }
            }
        }
    }
    Ok(())
}
