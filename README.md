Feed2File - F2F

-------------------------

Write given feed-urls in file *feeds* to a file *latest-news*. Simple script.

# Dependencies for linux
minimum: rustc 1.31.0
		 cargo 1.31.0
		 
(Windows doesnt need the compiler, because you can find the latest release in this project. But you can compile this project on your own, too.)

#Installation guide
## Windows 10
Go to the release site of this github project and download the latest archive.

## Linux / older Windows
First you have to clone this project with git. Then you have to compile the project with **cargo build --release** on your own. The executable can be found in the target-folder.

## Configure
In the scripts folder, you can find two files. In there you can define the sleeping-time (default: 3600 seconds = 1 hour). 

In the main folder, you have to create a file with the name: *feeds*. There you have to place your rss-feeds one per line, you want to track.

## How to run
Place the executable in the same folder, where your feeds-file is located and you want to find your newsfeed later. The scripts folder must be there, too. Now you can execute the linux.sh oder win.bat in the scripts folder.

