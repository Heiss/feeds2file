use rss::{Channel};
use chrono::{DateTime};
use std::io::{BufReader, BufRead, Write};
use std::fs::File;

	
fn main() {
    println!("Hello, world!");
	read_in();
}

fn read_in() {

	let mut newsfeed = Vec::new();
	
	let f = File::open("feeds").unwrap();
	let file = BufReader::new(&f);
	for (_num, line) in file.lines().enumerate() {
		let l = line.unwrap();
		
		let chars: String = l.chars().collect(); 
		println!("{}: ReadIn: >{:}", _num, chars);
		
		let channel = Channel::from_url(&chars).unwrap();

		let mut items = channel.into_items().to_vec();
		newsfeed.append(&mut items);
	}

	newsfeed.sort_by(|a, b| DateTime::parse_from_rfc2822(a.pub_date().unwrap()).unwrap().timestamp_millis().cmp(&DateTime::parse_from_rfc2822(b.pub_date().unwrap()).unwrap().timestamp_millis()));
	
	let mut fw = File::create("latest-news").expect("Unable to create file");                                                                                                          
	for news in &newsfeed {
		println!("{:?}", news.title());                                                                                                                                             
		write!(fw, "{}: {} - {}\n", news.title().unwrap());                                                                                                                            
	}
	
}