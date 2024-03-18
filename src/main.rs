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
    wallhaven_default_args: bool,

    log_lvl: u8,
    active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct WallheavenObj {
    #[serde(rename = "path")]
    path: String,
}

fn main() {
    let mut options: CliOptions = parse_args();
    if options.wall_engine.is_empty() {
        log("No wallpaper engine specified.", 2, 0, &options);
        exit(2)
    }
    if !options.active {
        return;
    }
    log(
        format!("Time interval: {} seconds", options.time_interval).as_str(),
        0,
        0,
        &options,
    );
    if options.time_interval.is_empty() {
        options.time_interval = String::from("0");
        log("Time interval is set to 0 seconds. It might cause lag and difference might be unnoticable", 1, 0, &options);
    }
    if options.mode.is_empty() {
        log("No was mode chosen.", 0, 0, &options);
    }
    match options.mode.as_str() {
        "wall-show" => wall_show(options),
        "wallhaven" => wall_from_wallheaven(options),
        _ => {
            log(
                format!("{}: Invaild mode.", options.mode).as_str(),
                2,
                0,
                &options,
            );
        }
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
        wallhaven_default_args: false,

        log_lvl: 0,
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
                "-log_lvl" => {
                    let log_lvl = String::from(&args[i + 1]);
                    let log_lvl = log_lvl.parse::<u8>().unwrap_or_else(|_| {
                        log(
                            format!("{}: Invaild log_lvl value.", log_lvl).as_str(),
                            2,
                            0,
                            &options,
                        );
                        exit(0);
                    });
                    options.log_lvl = log_lvl;
                }
                "-d" => options.path_to_dir = String::from(&args[i + 1]),
                "-t" => {
                    options.time_interval = String::from(&args[i + 1]);
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
                "-default" => options.wallhaven_default_args = true,
                _ => {
                    if arg.starts_with("-") {
                        log(
                            format!(": {}: Invaild argument.", &arg).as_str(),
                            2,
                            0,
                            &options,
                        );
                        exit(1);
                    } else {
                        continue;
                    }
                }
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


# wallhaven mode
> -m wallhaven
1. You can also use \"-save\" flag, with this all the downloaded wallpapers will be saved in the specified wallpaper directory.
2. You can use \"-default\" flag, with this you will not need to input anything, and defaults will be used which is blank for tag, resolution and random for sorting.

Example: wall-util -t 60 -d path/to/wall_dir/ -m wallhaven -save -default -w swww 

# Supported wallpaper engine
1. swww             (-w swww)
2. The Gnome DE     (-w gnome)
"
;
    println!("{help}");
    exit(0)
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
        log("path to wallpaper directory not given.", 2, 0, &options);
        options.active = false;
    }
    if !(Path::new(options.path_to_dir.as_str()).is_dir()) {
        log("{options.path_to_dir}: Invaild path.", 2, 0, &options);
        options.active = false;
    }
    if options.active {
        let mut walls = walls_from_dir(options.path_to_dir.as_str()).unwrap();
        let time_interval: u64 = options.time_interval.parse::<u64>().unwrap_or_else(|_e| {
            println!();
            log(
                "Invaild time interval: {options.time_interval}",
                2,
                0,
                &options,
            );
            options.active = false;
            0
        });
        loop {
            let mut rng = rand::thread_rng();
            walls.shuffle(&mut rng);
            let mut i = 1;
            log(
                format!("Total wallpaper: {}", walls.len()).as_str(),
                0,
                0,
                &options,
            );
            for wall in &walls {
                let wall = format!("{}", wall.display());
                let wall = wall.as_str();
                if options.log_lvl == 0 || options.log_lvl == 1 {
                    print!("{i}: {wall}\r");
                    io::stdout().flush().unwrap();
                }
                i = i + 1;
                set_wall(wall, options.wall_engine.as_str(), &options);
                thread::sleep(Duration::from_secs(time_interval));
            }
        }
    }
}

fn wall_from_wallheaven(options: CliOptions) {
    let time_interval: u64 = options.time_interval.parse::<u64>().unwrap_or_else(|_e| {
        log(
            format!("Invaild time interval: {}", options.time_interval).as_str(),
            2,
            0,
            &options,
        );
        exit(2)
    });

    let mut _save_dir_path = String::new();
    if options.path_to_dir.is_empty() {
        log(
            "No path to directory received. Current directory will be used.",
            1,
            0,
            &options,
        );
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
            log(
                format!("{}: Invaild path", &options.path_to_dir).as_str(),
                2,
                0,
                &options,
            );
            exit(2);
        }
        log(
            format!("Path to directory: {}", dir_path).as_str(),
            0,
            0,
            &options,
        );
    }
    let dir_tpl = String::from(&_save_dir_path);

    let mut tagnames = String::new();
    let mut resolution = String::new();
    let mut sorting = String::new();

    if !options.wallhaven_default_args {
        print!("[OPTIONAL] Type the tags, seperated by spaces: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut tagnames).unwrap();
        let tagnames_t = tagnames.trim();
        let tagnames_t = if tagnames_t.is_empty() {
            String::new()
        } else {
            tagnames_t.replace(" ", "+")
        };
        tagnames = String::from(tagnames_t);

        print!("[OPTIONAL] Enter the resolution, (example: 1920x1080): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut resolution).unwrap();
        let reso_t = resolution.trim();
        let reso_t = if resolution.is_empty() {
            String::new()
        } else {
            reso_t.replace(" ", "+")
        };
        resolution = String::from(reso_t);

        print!("[OPTIONAL] Enter the sorting, toplist or random (default is random): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut sorting).unwrap();
        let sorting_t = sorting.trim();
        let sorting_t = if sorting.is_empty() {
            String::from("random")
        } else {
            String::from(sorting_t)
        };
        sorting = String::from(sorting_t);
    }
    let link = format!(
        "https://wallhaven.cc/api/v1/search?&q={}&categories=100&purity=100&resolution={}&sorting={}",
        tagnames, resolution, sorting
    );
    if options.active {
        let mut j = 1;
        loop {
            let repsonse = wallheaven_request(String::from(&link), &options);
            if repsonse.len() == 0 {
                log(
                    format!("Did not got any wallpapers for current options from wallheaven.")
                        .as_str(),
                    0,
                    0,
                    &options,
                );
                exit(0);
            }
            for i in &repsonse {
                if options.wallhaven_save {
                    let date = Command::new("date").args(["+%c"]).output().unwrap();
                    let mut date = String::from_utf8(date.stdout).unwrap();
                    date = date[0..(date.len() - 5)].to_string();
                    _save_dir_path = format!("{} {}", _save_dir_path, date);
                }

                _save_dir_path = format!("{}.jpg", _save_dir_path);

                Command::new("curl")
                    .args([i.as_str(), "--output", _save_dir_path.as_str()])
                    .output()
                    .unwrap();
                set_wall(
                    _save_dir_path.as_str(),
                    options.wall_engine.as_str(),
                    &options,
                );
                _save_dir_path = String::from(&dir_tpl);
                if options.log_lvl == 0 || options.log_lvl == 1 {
                    print!("[{}]: {i}\r", j);
                    io::stdout().flush().unwrap();
                }
                j = j + 1;

                thread::sleep(Duration::from_secs(time_interval));
            }
        }
    }
}

fn wallheaven_request(link: String, options: &CliOptions) -> Vec<String> {
    let query_url = link.as_str();
    let response = Command::new("curl")
        .arg(query_url)
        .output()
        .unwrap_or_else(|err| {
            log(
                format!("ERROR: Failed to use curl.\n{}", err).as_str(),
                2,
                0,
                &options,
            );
            exit(2);
        });
    let response = String::from_utf8(response.stdout).unwrap();
    log(
        format!("curl reponse:\n{}", response).as_str(),
        0,
        1,
        &options,
    );
    let parse_json: serde_json::Value = serde_json::from_str(response.as_str()).unwrap();
    let mut response: Vec<String> = Vec::new();
    if let Some(data_array) = parse_json["data"].as_array() {
        for i in data_array {
            if let Some(path) = i.get("path").and_then(|p| p.as_str()) {
                response.push(path.to_string());
            }
        }
    }
    log(
        format!("response vector:\n{:?}", response).as_str(),
        0,
        1,
        &options,
    );
    response
}

fn log(msg: &str, msg_type: u8, msg_lvl: u8, options: &CliOptions) {
    const RED: &str = "\x1b[31m";
    const GREEN: &str = "\x1b[32m";
    const YELLOW: &str = "\x1b[33m";
    const RESET: &str = "\x1b[0m";

    // msg_type 1 is for Warning
    // msg_type 2 is for Error
    // msg_type 0 is for Info

    let mut _text: String = String::new();

    match msg_type {
        0 => {
            _text = format!("[{}INFO{}] {}", GREEN, RESET, msg);
        }
        1 => {
            _text = format!("[{}WARN{}] {}", YELLOW, RESET, msg);
        }
        2 => {
            _text = format!("[{}ERROR{}] {}", RED, RESET, msg);
        }
        _ => {
            println!("[LOG] {}: Invaild msg_type", msg_type)
        }
    }

    if msg_lvl == 0 && options.log_lvl == 0 {
        println!("{}", &_text);
    } else if options.log_lvl == 1 {
        println!("{}", &_text);
    } else if options.log_lvl == 3 {
        if msg_lvl == 2 {
            println!("{}", &_text);
        }
    }
}

fn set_wall(wall: &str, wall_engine: &str, options: &CliOptions) {
    match wall_engine {
        "swww" => {
            let args = ["img", wall, "--transition-type", "any"];
            Command::new("swww")
                .args(args)
                .output()
                .unwrap_or_else(|_e| {
                    log(
                        format!("Failed to use swww.\n{}", _e).as_str(),
                        2,
                        0,
                        &options,
                    );
                    exit(2);
                });
        }
        "gnome" => {
            let t = format!("file:///{}", wall);
            let args = [
                "set",
                "org.gnome.desktop.background",
                "picture-uri",
                &t.as_str(),
            ];
            Command::new("gsettings")
                .args(args)
                .output()
                .unwrap_or_else(|e| {
                    log(
                        format!("Failed to use gsettings.\n{}", e).as_str(),
                        2,
                        0,
                        &options,
                    );
                    exit(2);
                });
        }
        _ => {}
    }
}
