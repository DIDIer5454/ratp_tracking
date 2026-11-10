//#[derive(Debug)]
use api::ClientAPI; 
use std::fs;
use std::env;
use reqwest::blocking::Client;

fn main() {
	let args: Vec<String> = env::args().collect();
	let contents = fs::read_to_string(&args[1])
	 .expect("Probleme pour lire le fichier configuration");
	let conf: Vec<&str>=contents.split('"').collect();
	let url = &conf[1];
	let api_key = &conf[3];
	let g = ClientAPI::new(url.to_string(),api_key.to_string()); 
	g.get_trajet();
//	let client = Client::new();	let resp =reqwest::blocking::get("https://www.example.com").expect("Failed").text();
//	println!("body = {resp:?}");

}
