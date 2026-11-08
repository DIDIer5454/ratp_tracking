
#[derive(Debug)]
//pub mod Gestionnaire_de_services;

pub struct Gestionnaire_de_services {
	parameters: String,
}

impl Gestionnaire_de_services {
	pub fn new() -> Gestionnaire_de_services {
		Gestionnaire_de_services {
			parameters: String::from("hey"),		
		}
	}
}
