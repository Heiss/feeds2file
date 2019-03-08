use rss::{Channel};
use chrono::{DateTime};
use std::io::{BufReader, BufRead, Write};
use std::fs::File;
use url::Url;

// Small Welcome-Text.
fn main() {
    println!("Thanks for using this tool. Any contributions are welcome. \nGithub-Project can be find here: https://github.com/iPhysicX/feeds2file");
	read_in();
}

// This function reads in all given RSS-Feeds and 
fn read_in() {
	// a vector for all news items.
	let mut newsfeed = Vec::new();
	
	// open the feeds file
	let f = File::open("feeds").expect("No \"feeds\" file found. Please create one and place your feeds in there one per line.");
	let file = BufReader::new(&f);
	
	// loop over all given feeds.
	for (_num, line) in file.lines().enumerate() {
		// read in the given feeds and print them for debug.
		let l = line.unwrap();
		let chars: String = l.chars().collect(); 
		println!("{}: ReadIn: >{:}", _num, chars);
		
		// parse the given rss feed from url.
		let channel = Channel::from_url(&chars).expect(&format!("{} is not an rss-feed. Please remove it from feeds.", chars));

		// append the new items into the news vector for later sorting.
		let mut items = channel.into_items().to_vec();
		newsfeed.append(&mut items);
	}

	// Sorting the vector by publishing datetime. This need DateTime crate, because it is a string and can't be compared.
	newsfeed.sort_by(|a, b| DateTime::parse_from_rfc2822(a.pub_date().unwrap()).unwrap().timestamp_millis().cmp(&DateTime::parse_from_rfc2822(b.pub_date().unwrap()).unwrap().timestamp_millis()));
	
	// create the latest-news, so older entries will removed.
	let mut fw_simple = File::create("latest-news").expect("Unable to create \"latest-news\" file in the current folder.");     
	let mut fw = File::create("latest-news.html").expect("Unable to create \"latest-news.html\" file in the current folder.");     
	
	write!(fw, "<html>
		<head>
		<meta charset=\"utf-8\">

		<style>
		html, body {{
			background-color: black;
		}}
		body {{

		}}
		#wrap {{
			width: 100%;
			margin: 0 auto;
		}}
		h1 {{
			color: #333;
			text-shadow: 1px 1px 0 #fff;
		}}
		a, a:active, a:visited {{
			color: #ccc;
		}}
		.demo {{
			font-size:35px;
			color: #fff;
			width: 100%;
			height: 40px;
			overflow: hidden;
		}}
		</style>

		</head>
		<body>

		<ul id=\"ticker1\" class=\"demo\">"
	).expect("Error while writing to \"latest-news\" file.");

	// loop over all entries. Possible multi threading here
	for news in &newsfeed {
		// print news items for debug and parse them into an url.
		println!("{:?}", news.title().unwrap());
		let url = Url::parse(news.link().unwrap()).expect(&format!("Not an url link given from {}. Maybe you should delete it from the feeds.", news.link().unwrap()));
		
		// wrte items to "latest-news "file one by one.
		write!(fw, "<li>{1} - {0}</li>\n", news.title().unwrap(), url.domain().unwrap()).expect("Error while writing to \"latest-news.html\" file.");           
		write!(fw_simple, "{1} - {0}\n", news.title().unwrap(), url.domain().unwrap()).expect("Error while writing to \"latest-news\" file.");                                                  
	}
	
	write!(fw, "</ul>
		<p>

		<script src=\"https://ajax.googleapis.com/ajax/libs/jquery/1.10.2/jquery.min.js\"></script>
		<script src=\"jquery.ticker.js\"></script>

		<script>
			$(document).ready(function(){{
				$(\"#ticker1\").ticker({{ effect: \"slideUp\"}});
			}});
		</script>

		</body>
		</html>"
	).expect("Error while writing to \"latest-news\" file.");
}