use url::Url;
use reqwest::blocking::Client;
use serde::Deserialize;  
use std::collections::HashMap;

pub struct ClientAPI{
	url: String, //url de l'api
	api_key: String, //clef api 
        connection: reqwest::blocking::Client,
}

#[derive(Deserialize,Debug)]
struct Test1{
    nom: String,
    code: String, 
    codeDepartement: String,
    siren: String,
    codeEpci: String,
    codeRegion: String,
    codesPostaux: Vec<String>,
    population: u32,
}

impl ClientAPI{ 
	pub fn new(url: String, api_key: String) -> Self { 
		Self {
			url, 
			api_key,
                        connection:  Client::new(),
		}
	}
        pub fn create_connection(&self) {
            
        }

	pub fn get_trajet(&self) {
		//on va implementer la fonction qui permet de recuperer le trajet final via l'api 	
                let api_url = Url::parse(&self.url).expect("Failed");
                let rep=self.connection.get(api_url).send().expect("request failed").text().expect("failed");
                let s= "r#\"".to_string()+  &rep[1..rep.len()-1] +"\"#";
                println!("nom::::{}",rep);
                let test1: Test1 = serde_json::from_str(&rep[1..rep.len()-1]).expect("Test");
                println!("nom::::{}",test1.nom);
                println!("code::::{}",test1.code);
                println!("codeDep::::{}",test1.codeDepartement);
                println!("siren::::{}",test1.siren);
                println!("codeEpci::::{}",test1.codeEpci);
                println!("codeP::::{:?}",test1.codesPostaux);
                println!("population::::{}",test1.population);
//                let test = deserializer.deserialize_str("Test1");
        }
}
