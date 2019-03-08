Feeds2File - F2F

-------------------------

Write given feed-urls in file *feeds* to a file *latest-news*. Simple script for simple users. No webservice needed.
This tool can be used by Streamer (ex. on Twitch) with OBS ([Open Broadcaster Software](https://obsproject.com/)) to get a newsticker from their favourite feeds. The tool writes all items from given feeds into one file one news per line ascending sorted by the published dates. So the latest news have the highest line number.

# (minimal version) Dependencies for building
```
rustc 1.31.0
cargo 1.31.0
```

(Windows don't need this, because you can find the [latest release in this project](https://github.com/iPhysicX/feeds2file/releases). But you can compile this project on your own, too. Follow the linux instructions then.)

# Installation
## Windows 10
Go to the release site of this github project and download the latest archive. Extract this to an empty folder. This folder will be used to store your feeds and generated program files.

## Linux / older Windows
First you have to clone this project with git. Then you have to compile the project with the following command on your own. 

```
cargo build --release
```

The executable can be found in the new target-folder. You have to move the feeds2file program to the folder, where you want to store the *feeds* and *latest-news* files. Also copy the scripts folder from your git-folder in the same folder.

After this, your folder should be like this (one folder, one executable):

```
feeds2file
linux.sh
win.bat
```

## Configure
The two files linux.sh and win.bat are script files for your OS. In there you can define the sleeping-time (default: 3600 seconds = 1 hour). 

In the main folder, you have to create a file with the name: *feeds*. Open it with your favourite texteditor. In this file you have to place your rss-feeds one per line.

Now your folder should be like this (one folder, one executable, one text file):
```
feeds
feeds2file
linux.sh
win.bat
```

## How to run
Place the executable in the same folder, where your feeds-file is located and you want to find your newsfeed later. The script files must be there, too. Now you can execute the linux.sh (for linux) oder win.bat (for windows).

After running the script or the executable one time, your folder should be like this (one folder, one executable, two text files):
```
feeds
feeds2file
latest-news
latest-news.html
linux.sh
win.bat
```

In the latest-news file, you can find all your news-items appended with the domain, where the news was posted. Now you can use it further.

The latest-news.html file is a small file, which you can include in your scene with the browser source. If you don't know how to configure this little program, you can find a lot of ressources in the internet about html and css. Please check them. This is only an easy script for fast use in simple stream-settings.

### Little explanation
This little scripts contain a while-loop, so the feed2file executable will be run forever. This is a convenient workaround, so the executable don't need the while-loop. If you want to run the program on your own and don't need the while-loop, you can run the feed2file program. Otherwise run the script for your OS with the while-loop.

# Troubleshooting
## The linux.sh cannot be executed.

Then you have forgotten to make it executable with the following command:
```
chmod +x linux.sh
```

### How i can get a newsticker in my obs-stream?

You have to add your scene and Text (GDI+) source. There you have to set the new *latest-news* file as a text file. Now you can enable the chatlog mode and set the maximum number of news, which will be shown in your newsticker. Now you can scroll them with the scrolling-filter.

