use rand::seq::SliceRandom;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;
use std::{env, fs, io, thread};

struct CliOptions {
    path_to_dir: String,
    time_interval: String,
    mode: String,
    _optional_args: String,
    active: bool,
}

fn main() {
    let mut options: CliOptions = parse_args();
    if !options.active {
        return;
    }
    if options.path_to_dir.is_empty() {
        println!("path to wallpaper directory not given.");
        return;
    }
    if !(Path::new(options.path_to_dir.as_str()).is_dir()) {
        println!("{}: Invaild path.", options.path_to_dir);
        return;
    }

    if options.mode.is_empty() {
        println!("No mode was chosen");
        return;
    }
    if options.time_interval.is_empty() {
        options.time_interval = String::from("0");
        println!("WARNING: Time interval is set to 0 seconds. It might cause lag and difference might be unnoticable");
    }
    match options.mode.as_str() {
        "wall-show" => wall_show(options),
        _ => println!("{}: Invaild mode.", options.mode),
    }
}

fn parse_args() -> CliOptions {
    let args: Vec<_> = env::args().collect();
    let mut options = CliOptions {
        path_to_dir: String::new(),
        time_interval: String::new(),
        mode: String::new(),
        _optional_args: String::new(),
        active: true,
    };
    if args.len() == 1 {
        print_help();
        options.active = false;
    } else {
        for i in 0..args.len() {
            let arg = args[i].as_str();
            match arg {
                "-d" => {
                    println!("Path of directory : {}", args[i + 1]);
                    options.path_to_dir = String::from(&args[i + 1])
                }
                "-t" => {
                    options.time_interval = String::from(&args[i + 1]);
                    println!("Time interval: {} seconds", options.time_interval);
                }
                "-m" => {
                    options.mode = String::from(&args[i + 1]);
                }
                "-h" => {
                    print_help();
                }
                _ => continue,
            }
        }
    }
    options
}

fn print_help() {
    let help = "
Usage:      wall-util [OPTIONS] 

-d      for setting path to wallpaper directory
        -d /path/to/dir 
-t      for setting time interval in seconds (default is 0) 
        -t 10
-m      for setting mode.
        wall-show   it will go thru all the wallpaper from the directory randomly.
        -m wall-show
";
    println!("{help}");
}

fn walls_from_dir(path_of_dir: &str) -> io::Result<Vec<PathBuf>> {
    let wall_dir = fs::read_dir(path_of_dir).unwrap();
    let walls: Vec<_> = wall_dir
        .filter_map(|wall_dir| {
            wall_dir.ok().and_then(|e| {
                let full_path = e.path();
                Some(full_path)
            })
        })
        .collect();

    Ok(walls)
}

fn wall_show(mut options: CliOptions) {
    let mut walls = walls_from_dir(options.path_to_dir.as_str()).unwrap();
    let time_interval: u64 = options.time_interval.parse::<u64>().unwrap_or_else(|_e| {
        println!("Invaild time interval: {}", options.time_interval);
        options.active = false;
        0
    });
    if options.active {
        loop {
            let mut rng = rand::thread_rng();
            walls.shuffle(&mut rng);
            let mut i = 1;
            println!("Total wallpaper: {}", walls.len());
            for wall in &walls {
                let wall = format!("{}", wall.display());
                let wall = wall.as_str();
                print!("{i}: {wall}\r");
                i = i + 1;
                io::stdout().flush().unwrap();
                set_wall(wall);
                thread::sleep(Duration::from_secs(time_interval));
            }
        }
    }
}

fn set_wall(wall: &str) {
    Command::new("swww")
        .args(["img", wall, "--transition-type", "any"])
        .output()
        .unwrap();
}
