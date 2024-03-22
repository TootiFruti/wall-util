# wall-util
wall-util is a wrapper around a (any) wallpaper engine (pre-installed on the system). With wall-util you can do things like slideshow of wallpaper or say wallpaper directly from internet (wallhaven.cc in wall-util), able to specify time interval and many more. Basically with wall-util i am hoping to achieve things which are outside the scope of a wallpaper engine.

# Requriement
Any wallpaper engine.

curl
https://github.com/curl/curl

# How to install
> git clone https://github.com/TootiFruti/wall-util

> cd wall-util

> cargo build --release

And now the binary is ready, "target/release/wall-util"

# First time use.
For you to use wall-util, required directory and file must be present/exist, for example, one of them being ~/.local/share/wall-util. For the required file to setup, just use the wall-util (without any arguments), then it will ask you whether you want to set these files up or not. Enter "y" as choice. And the next time you are all ready.

# How to use:
0. -h: for help.
1. -t: for specifying the time interval (seconds), default is 0 seconds, which might cause lag and the difference will be unnoticable.
2. -d: for specifying the path to the directory
3. -w: for specifying wallpaper engine. (eg: -w swww)
3. -m: for specifying which mode to use
   
       1. wall-show: it'll go thru the directory and set the wallpaper, randomly.
       2. wallhaven: it'll be fetching wallpapers from https://wallhaven.cc
4. -restore: with this flag, wall-util will be using arguments from the last time.
5. -log_lvl

# Supported wallpaper engine
> -w <wallpaper engine>

1. swww           ->  For using swww.
2. gnome          ->  For the Gnome DE.
3. gsettings      ->  For using gsettings.
4. xwallpaper     ->  For using xwallpaper.

# wallhaven mode
> -m wallhaven

1. You can also use "-save" flag, with this all the downloaded wallpapers will be saved in the specified wallpaper directory.
2. You can use "-default" flag, with this you will not need to input anything, and defaults will be used which is blank for tag, resolution and random for sorting.

Example: wall-util -t 60 -d path/to/wall_dir/ -m wallhaven -save -default -w swww 

# restore 
> -restore 

With this flag, wall-util will be using arguments from the last time. But you can also add/edit arguments to it, for example if you want to change the mode and want to use all the arguments as the last time, you will only need to mention the flag you want to change.

> wall-util -restore -log_lvl 1

(log_lvl value last time was 0)

(Extra information, the file for restore feature is stored in ~/.local/share/wall-util/last_cmd.txt, this is the file from where wall-util will be taking arguments from last time.)

# logging
> -log_lvl \<value\>

For value 0, It will only be logging important informations.
For value 1, It will be logging a bit more, which be less important, informations.
For value 3, It will not be logging anything except ERRORs.

And all the logs are stored in ~/.local/share/wall-util/logs/. Only two files are stored, the current and the log file of last time.

# Why?
I wanted to make a thing which will change wallpaper after some time, kind of like the slide show, so i started.
Orginally i was making this in python. But then rust said to me, NO. So, here i am. It is my first major project in rust, you can say im noob too.
So this way i'll also learn rust too. (hehe..) 

# What's next?
My next goal will be to make one of those live wallpaper typa things which changes acc. to the time and weather.
