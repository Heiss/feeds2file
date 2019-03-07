use rss::{Channel};
use chrono::{DateTime};
use std::io::{BufReader, BufRead, Write};
use std::fs::File;
use url::Url;

// Small Welcome-Text.
fn main() {
    println!("Thanks for using this tool. Any contributions are welcome. \nGithub-Project can be find here: https://github.com/iPhysicX/feed2file");
	read_in();
}

// This function reads in all given RSS-Feeds and 
fn read_in() {

	let mut newsfeed = Vec::new();
	
	let f = File::open("feeds").unwrap();
	let file = BufReader::new(&f);
	for (_num, line) in file.lines().enumerate() {
		let l = line.unwrap();
		
		let chars: String = l.chars().collect(); 
		println!("{}: ReadIn: >{:}", _num, chars);
		
		let channel = Channel::from_url(&chars).expect(&format!("{} is not an rss-feed. Please remove it from feeds.", chars));

		let mut items = channel.into_items().to_vec();
		newsfeed.append(&mut items);
	}

	// Sorting the vector by publishing datetime. This need DateTime crate, because it is a string and can't be compared.
	newsfeed.sort_by(|a, b| DateTime::parse_from_rfc2822(a.pub_date().unwrap()).unwrap().timestamp_millis().cmp(&DateTime::parse_from_rfc2822(b.pub_date().unwrap()).unwrap().timestamp_millis()));
	
	let mut fw = File::create("latest-news").expect("Unable to create \"latest-news\" file in the current folder.");                                                                                                          
	for news in &newsfeed {
		println!("{:?}", news.title().unwrap());
		let url = Url::parse(news.link().unwrap()).expect(&format!("Not an url link given from {}. Maybe you should delete it from the feeds.", news.link().unwrap()));
		
		write!(fw, "{} - {}\n", news.title().unwrap(), url.domain().unwrap()).expect("Error while writing to \"latest-news\" file.");                                                                                                                            
	}
	
}