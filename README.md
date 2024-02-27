# wall-util
(in rust obv.)

# Requriement
swww (*1)
https://github.com/LGFae/swww

# How to install
> git clone https://github.com/TootiFruti/wall-util

> cd wall-util

> cargo build --release

And now the binary is ready, "target/release/wall-util"

# How to use:
0. -h: for help.
1. -t: for specifying the time interval (seconds), default is 0 seconds, which might cause lag and the difference will be unnoticable.
2. -d: for specifying the path to the directory
3. -m: for specifying which mode to use
   
       1. wall-show: it'll go thru the directory and set the wallpaper, randomly.
       2. wallhaven: it'll be fetching wallpapers from https://wallhaven.cc

# Why?
I wanted to make a thing which will change wallpaper after some time, kind of like the slide show, so i started.
Orginally i was making this in python. But then rust said to me, NO. So, here i am. It is my first major project in rust, you can say im noob too.
So this way i'll also learn rust too. (hehe..) 

# What's next?
My next goal will be to make one of those live wallpaper typa things which changes acc. to the time and weather.


# *1
swww is not mandatory, you can actually use anything you want. 
In the main.rs file, the last function,
set_wall, you can replace swww with whatever you want and then can change the cli-args too, in the array, ["arg1", "arg2", ...]
Just use the wall variable when you have to specify the path to the wallpaper. And all set.

# UPDATE 27 FEB 2024
Added new mode called "wallhaven"
wallhaven: it'll be fetching wallpapers from https://wallhaven.cc
You can provide additional informations, tags, resolutions or sorting way, and all can be left blank.
