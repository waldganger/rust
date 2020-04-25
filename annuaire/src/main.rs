use std::io;

#[derive(Debug)]
struct Contact {
	nom: String,
	prenom: String,
	adresse: String,
	courriel: String,
	telephone: String,
}

impl Contact {
	// Fonction associée : constructeur
	fn cree_contact(nom: String, prenom: String, adresse: String, 
		courriel: String, telephone: String) -> Contact {
		Contact {
			nom,
			prenom,
			adresse,
			courriel,
			telephone,
		}
	}
	
	// Methodes de modification.
	// A faire : assembler ces méthodes en une metamethode de modif de toute la fiche
	fn modif_nom(&mut self, nouveau_nom: String)  {
		self.nom = nouveau_nom;
	}

	fn modif_prenom(&mut self, nouveau_prenom: String)  {
		self.prenom = nouveau_prenom;
	}

	fn modif_adresse(&mut self, nouvelle_adresse: String)  {
		self.adresse = nouvelle_adresse;
	}

	fn modif_courriel(&mut self, nouveau_courriel: String)  {
		self.courriel = nouveau_courriel;
	}

	fn modif_telephone(&mut self, nouveau_telephone: String)  {
		self.telephone = nouveau_telephone;
	}
}



fn main() {
	let mut input_nom = String::new();
	io::stdin().read_line(&mut input_nom)
	.expect("Problème de mémoire.");
	
	let mut input_prenom = String::new();
	io::stdin().read_line(&mut input_prenom)
	.expect("Problème de mémoire.");

	let mut input_adresse = String::new();
	io::stdin().read_line(&mut input_adresse)
	.expect("Problème de mémoire.");

	let mut input_courriel = String::new();
	io::stdin().read_line(&mut input_courriel)
	.expect("Problème de mémoire.");

	let mut input_telephone = String::new();
	io::stdin().read_line(&mut input_telephone)
	.expect("Problème de mémoire.");

	let mut antony = Contact::cree_contact(input_nom, input_prenom, input_adresse, input_courriel, input_telephone);



    
    println!("{}", antony.nom);

    antony.modif_nom(String::from("MERLE"));
    antony.modif_prenom(String::from("ANTONY"));
    antony.modif_adresse(String::from("chez lui"));
    antony.modif_courriel(String::from("anto@yahoo.fr"));
    antony.modif_telephone(String::from("07897647836"));

    println!("{:?}", antony);
}

// legacy code

	// let champs = ["nom", "prénom", "adresse", "courriel", "téléphone"];
	// let user_inputs: [String;NOMBRE_CHAMPS];

	// for i in 0..NOMBRE_CHAMPS{
	// 	println!("Entrez l'information suivante : {}", champs[i]);

	// 	let mut user_input = String::new();
	// 	io::stdin().read_line(&mut user_input)
	// 	.expect("Problème de mémoire.");
	// 	user_inputs[i] = String::from(user_input)
		
	// }

	

	// loop {
	// 	println!("Entrez le nom du contact");
	// 	io::stdin().read_line(&mut user_input)
	// 	.expect("Problème de mémoire.");
	// 	break;
	// }
	// println!("{}", user_input);

	// let mut antony = Contact::cree_contact(
	// 	String::from("Merle"), 
	// 	String::from("antony"), 
	// 	String::from("8 allée de la corderie 64600 ANGLET"), 
	// 	String::from("antony.merle@protonmail.com"), 
	// 	String::from("0684259961"));