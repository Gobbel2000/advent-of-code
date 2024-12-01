use std::path::PathBuf;
use std::{io, env, fs};

fn find_input_dir() -> io::Result<PathBuf> {
    let cwd = env::current_dir()?;
    cwd.ancestors()
        .map(|p| p.join("input"))
        .find(|p| p.is_dir())
        .ok_or(io::Error::new(io::ErrorKind::NotFound,
                              "No input directory found on any parent"))
}

fn input_file(fname: &str) -> io::Result<PathBuf> {
    Ok(find_input_dir()?.join(fname))
}

pub fn get_input(fname: &str) -> io::Result<(String, bool)> {
    let mut part_2 = false;
    let mut input_path = None;
    for arg in env::args_os().skip(1) {
        if arg == "2" {
            part_2 = true;
        } else if arg != "1" {
            input_path = Some(arg);
        }
    }
    let path = match input_path {
        Some(p) => p.into(),
        None => input_file(fname)?,
    };
    let input = fs::read_to_string(path)?;
    Ok((input, part_2))
}

#[macro_export]
macro_rules! aoc_main {
    () => {
        fn input_name() -> String {
            let this_file = ::std::file!();
            let path = ::std::path::Path::new(this_file);
            let txt_path = path.with_extension("txt");
            let fname = txt_path.file_name().unwrap();
            fname.to_str().unwrap().to_owned()
        }
        util::aoc_main!(&input_name());
    };
    ( $x:expr ) => {
        fn main() -> std::io::Result<()> {
            let (input, p2) = util::get_input( $x )?;
            if p2 {
                part2(input);
            } else {
                part1(input);
            }
            Ok(())
        }
    };
}
