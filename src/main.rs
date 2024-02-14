use rand::seq::SliceRandom;
use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;
use std::time::Duration;

struct CliOptions {
    path_to_dir: String,
    time_interval: String,
    mode: String,
}

fn main() {
    let options: CliOptions = parse_args();
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
    if options.mode.as_str() == "wall-show" {
        wall_show(options);
    }
}

fn parse_args() -> CliOptions {
    let args: Vec<_> = env::args().collect();
    let mut options = CliOptions {
        path_to_dir: String::new(),
        time_interval: String::from("0"),
        mode: String::new(),
    };
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
            _ => continue,
        }
    }
    options
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

fn wall_show(options: CliOptions) {
    let mut walls = walls_from_dir(options.path_to_dir.as_str()).unwrap();
    let time_interval: u64 = options.time_interval.parse::<u64>().unwrap();

    loop {
        let mut rng = rand::thread_rng();
        walls.shuffle(&mut rng);
        for wall in &walls {
            // let wall = wall.display();
            let wall = format!("{}", wall.display());
            let wall = wall.as_str();
            println!("{wall}");
            let output = Command::new("swww")
                .arg("img")
                .arg(wall)
                .arg("--transition-type")
                .arg("any")
                .output();
            println!("{:?}", output);
            thread::sleep(Duration::from_secs(time_interval));
        }
    }
}
