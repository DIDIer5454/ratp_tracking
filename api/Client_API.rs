//#[derive(Debug)]
//pub mod Gestionnaire_de_services;

pub struct ClientAPI{
	url: String, //url de l'api
	api_key: String, //clef api 
}
//Constructeur du Client_API
impl ClientAPI{ 
	pub fn new(url: String, api_key: String) -> Self { 
		Self {
			url, 
			api_key,
		}
	}
	pub fn get_trajet(&self) {
		//Ici on va implementer la fonction qui permet de recuperer le trajet final via l'api 	
		println!("yo I'm here : url= {} and api_key={}",self.url,self.api_key);	
	}
}
