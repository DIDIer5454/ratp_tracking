//#[derive(Debug)]
use api::ClientAPI; 
use std::fs;
use reqwest::blocking::Client;


fn main() {
	let contents = fs::read_to_string("config")
	 .expect("Probleme pour lire le ficheir config");
	let conf: Vec<&str>=contents.split('"').collect();
	let url = &conf[1];
	let api_key = &conf[3];
	let g = ClientAPI::new(url.to_string(),api_key.to_string()); 
	g.get_trajet();
	println!( "url:{url};; api_key:{api_key}");
	let client = Client::new();	
	let resp =reqwest::blocking::get("https://www.example.com").expect("Failed").text();
	println!("body = {resp:?}");

}
