use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{exit, Command};
use std::time::Duration;
use std::{env, fs, io, thread};

struct CliOptions {
    path_to_dir: String,
    time_interval: String,
    mode: String,
    _optional_args: String,
    wallhaven_save: bool,
    wall_engine: String,
    active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct WallheavenObj {
    #[serde(rename = "path")]
    path: String,
}

fn main() {
    let mut options: CliOptions = parse_args();
    if !options.active {
        return;
    }
    if options.time_interval.is_empty() {
        options.time_interval = String::from("0");
        println!("WARNING: Time interval is set to 0 seconds. It might cause lag and difference might be unnoticable");
    }
    match options.mode.as_str() {
        "wall-show" => wall_show(options),
        "wallhaven" => wall_from_wallheaven(options),
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
        wallhaven_save: false,
        wall_engine: String::new(),
        active: true,
    };
    let mut _t = String::new();
    if args.len() == 1 {
        print_help();
        options.active = false;
    } else {
        for i in 0..args.len() {
            let arg = args[i].as_str();
            match arg {
                "-d" => options.path_to_dir = String::from(&args[i + 1]),
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
                "-w" => options.wall_engine = String::from(&args[i + 1]),
                "-save" => {
                    options.wallhaven_save = true;
                }
                _ => continue,
            }
        }
    }
    options._optional_args = _t;
    options
}

fn print_help() {
    let help = "
Usage:      wall-util [OPTIONS] 

-d      for setting path to wallpaper directory
        -d /path/to/dir 
-t      for setting time interval in seconds (default is 0) 
        -t 10
-w      for specifying which wallpaper engine to use.
        -w swww
        (Currently supported swww)
-m      for setting mode.
        wall-show   it will go thru all the wallpaper from the directory randomly.
        wallhaven   it'll be fetching wallpapers from https://wallhaven.cc
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
    if options.path_to_dir.is_empty() {
        println!("path to wallpaper directory not given.");
        options.active = false;
    }
    if !(Path::new(options.path_to_dir.as_str()).is_dir()) {
        println!("{}: Invaild path.", options.path_to_dir);
        options.active = false;
    }
    if options.active {
        let mut walls = walls_from_dir(options.path_to_dir.as_str()).unwrap();
        let time_interval: u64 = options.time_interval.parse::<u64>().unwrap_or_else(|_e| {
            println!("Invaild time interval: {}", options.time_interval);
            options.active = false;
            0
        });
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
                set_wall(wall, options.wall_engine.as_str());
                thread::sleep(Duration::from_secs(time_interval));
            }
        }
    }
}

fn wall_from_wallheaven(mut options: CliOptions) {
    let time_interval: u64 = options.time_interval.parse::<u64>().unwrap_or_else(|_e| {
        println!("Invaild time interval: {}", options.time_interval);
        options.active = false;
        0
    });

    let mut _save_dir_path = String::new();
    if options.path_to_dir.is_empty() {
        println!("No path to directory received. Current directory will be used.");
        _save_dir_path = String::from("wall-util-save.jpg");
    } else {
        let dir_path = &options.path_to_dir;
        if Path::new(dir_path.as_str()).is_dir() {
            if dir_path.ends_with('/') {
                _save_dir_path = format!("{dir_path}wall-util-save");
            } else {
                _save_dir_path = format!("{dir_path}/wall-util-save");
            }
        } else {
            println!("ERROR: {}: Invaild path", &options.path_to_dir);
            exit(1);
        }
        println!("Path to directory: {dir_path}");
    }
    let dir_tpl = String::from(&_save_dir_path);

    let mut tagnames = String::new();
    let mut resolution = String::new();

    print!("[OPTIONAL] Type the tags, seperated by spaces: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut tagnames).unwrap();
    let tagnames = tagnames.trim();
    let tagnames = if tagnames.is_empty() {
        String::new()
    } else {
        tagnames.replace(" ", "+")
    };

    print!("[OPTIONAL] Enter the resolution, (example: 1920x1080): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut resolution).unwrap();
    let resolution = resolution.trim();
    let resolution = if resolution.is_empty() {
        String::new()
    } else {
        resolution.replace(" ", "+")
    };

    let mut sorting = String::new();
    print!("[OPTIONAL] Enter the sorting, toplist or random (default is random): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut sorting).unwrap();
    let sorting = sorting.trim();
    let sorting = if sorting.is_empty() {
        String::from("random")
    } else {
        String::from(sorting)
    };
    let link = format!(
        "https://wallhaven.cc/api/v1/search?&q={}&categories=100&purity=100&resolution={}&sorting={}",
        tagnames, resolution, sorting
    );
    if options.active {
        let mut j = 1;
        loop {
            let repsonse = wallheaven_request(String::from(&link));
            if repsonse.len() == 0 {
                println!("Did not got any wallpapers for current options from wallheaven.");
                exit(0);
            }
            for i in &repsonse {
                print!("[{}]: {i}\r", j);
                io::stdout().flush().unwrap();
                j = j + 1;

                if options.wallhaven_save {
                    let date = Command::new("date").args(["+%c"]).output().unwrap();
                    let mut date = String::from_utf8(date.stdout).unwrap();
                    date = date[0..(date.len() - 5)].to_string();
                    _save_dir_path = format!("{} {}", _save_dir_path, date);
                }

                _save_dir_path = format!("{}.png", _save_dir_path);

                Command::new("curl")
                    .args([i.as_str(), "--output", _save_dir_path.as_str()])
                    .output()
                    .unwrap();
                set_wall(_save_dir_path.as_str(), options.wall_engine.as_str());
                _save_dir_path = String::from(&dir_tpl);
                thread::sleep(Duration::from_secs(time_interval));
            }
        }
    }
}

fn wallheaven_request(link: String) -> Vec<String> {
    let query_url = link.as_str();
    let response = Command::new("curl")
        .arg(query_url)
        .output()
        .unwrap_or_else(|err| {
            eprintln!("ERROR: Failed to use curl.\n{err}");
            exit(1);
        });
    let response = String::from_utf8(response.stdout).unwrap();
    let parse_json: serde_json::Value = serde_json::from_str(response.as_str()).unwrap();
    let mut response: Vec<String> = Vec::new();
    if let Some(data_array) = parse_json["data"].as_array() {
        for i in data_array {
            if let Some(path) = i.get("path").and_then(|p| p.as_str()) {
                response.push(path.to_string());
            }
        }
    }
    response
}

fn set_wall(wall: &str, wall_engine: &str) {
    match wall_engine {
        "swww" => {
            let args = ["img", wall, "--transition-type", "any"];
            Command::new("swww")
                .args(args)
                .output()
                .unwrap_or_else(|_e| {
                    println!("[Error] Problem using swww");
                    exit(1);
                });
        }
        _ => {}
    }
}
