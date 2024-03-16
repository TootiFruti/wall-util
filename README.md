# wall-util
wall-util is a wrapper around a (any) wallpaper engine (pre-installed on the system). With wall-util you can do things like slideshow of wallpaper or say wallpaper directly from internet (wallhaven.cc in wall-util), able to specify time interval and many more.

# Requriement
Any wallpaper engine.

curl
https://github.com/curl/curl

# How to install
> git clone https://github.com/TootiFruti/wall-util

> cd wall-util

> cargo build --release

And now the binary is ready, "target/release/wall-util"

# How to use:
0. -h: for help.
1. -t: for specifying the time interval (seconds), default is 0 seconds, which might cause lag and the difference will be unnoticable.
2. -d: for specifying the path to the directory
3. -w: for specifying wallpaper engine. (eg: -w swww)
3. -m: for specifying which mode to use
   
       1. wall-show: it'll go thru the directory and set the wallpaper, randomly.
       2. wallhaven: it'll be fetching wallpapers from https://wallhaven.cc
4. -log_lvl

# Supported wallpaper engine
1. swww 

# wallhaven mode
> -m wallhaven
1. You can also use "-save" flag, with this all the downloaded wallpapers will be saved in the specified wallpaper directory.
2. You can use "-default" flag, with this you will not need to input anything, and defaults will be used which is blank for tag, resolution and random for sorting.

Example: wall-util -t 60 -d path/to/wall_dir/ -m wallhaven -save -default -w swww 

# logging
> -log_lvl \<value\>
For value 0, It will only be logging important informations.
For value 1, It will be logging a bit more, which be less important, informations.
For value 3, It will not be logging anything except ERRORs.

# Why?
I wanted to make a thing which will change wallpaper after some time, kind of like the slide show, so i started.
Orginally i was making this in python. But then rust said to me, NO. So, here i am. It is my first major project in rust, you can say im noob too.
So this way i'll also learn rust too. (hehe..) 

# What's next?
My next goal will be to make one of those live wallpaper typa things which changes acc. to the time and weather.

# UPDATE 27 FEB 2024
Added new mode called "wallhaven"
wallhaven: it'll be fetching wallpapers from https://wallhaven.cc
You can provide additional informations, tags, resolutions or sorting way, and all can be left blank.
