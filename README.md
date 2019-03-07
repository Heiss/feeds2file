Feed2File - F2F

-------------------------

Write given feed-urls in file *feeds* to a file *latest-news*. Simple script.

# Dependencies for linux
```
minimum: rustc 1.31.0
		 cargo 1.31.0
```
		 
(Windows don't need this, because you can find the latest release in this project. But you can compile this project on your own, too.)

# Installation
## Windows 10
Go to the release site of this github project and download the latest archive.

## Linux / older Windows
First you have to clone this project with git. Then you have to compile the project with the following command on your own. 

```
cargo build --release
```

The executable can be found in the target-folder.

## Configure
In the scripts folder, you can find two files. In there you can define the sleeping-time (default: 3600 seconds = 1 hour). 

In the main folder, you have to create a file with the name: *feeds*. Open it with your favourite texteditor. In this file you have to place your rss-feeds one per line.

## How to run
Place the executable in the same folder, where your feeds-file is located and you want to find your newsfeed later. The scripts folder must be there, too. Now you can execute the linux.sh oder win.bat in the scripts folder.

### Little explanation
This little scripts contain a while-loop, so the feed2file executable will be run forever. This is a convenient workaround, so the executable don't need the while-loop. If you want to run the program on your own and don't need the while-loop, you can run the feed2file program. Otherwise run the script for your OS with the while-loop.

# Troubleshooting
```
The linux.sh cannot be executed.
```

Then you have forget to make it executable with the following command:
```
chmod +x linux.sh
```

