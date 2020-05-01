// Exercices chapitre 8

fn main() {
	let voyelles: [char; 12] = ['A', 'a', 'E', 'e', 'I', 'i', 'O', 'o', 'U', 'u', 'Y', 'y'];

	let mut i: u8 = 5;
	while i > 0 {
		let mut user_input = String::new();
		std::io::stdin().read_line(&mut user_input).expect("Problème de mémoire.");
		println!("{}", pig_latin(&mut user_input, &voyelles));
		i -= 1;
	}
	}

// --------------------------------------------------------------

fn pig_latin(mot: &String, voyelles: &[char]) -> String {
	if tri(&mot, &voyelles) {
		voyelle_pig_latin(&mot)
	} else {
		consonne_pig_latin(&mot)
	}
}

// --------------------------------------------------------------

fn tri(mot: &String, voyelles: &[char]) -> bool {
	for c in mot[0..1].chars(){
		for voyelle in voyelles{
			if c == *voyelle{
				return true
			}
		}
	}
	false
}

// --------------------------------------------------------------

fn consonne_pig_latin(mot: &String) -> String {
	let mut c: char = ' ';
	for lettre in mot.chars() {
		c = lettre;
		break;
	}
	let mot_incomplet = &mot[1..].trim();
	let mut mot_complet: String = mot_incomplet.to_string();
	mot_complet.push('-');
	mot_complet.push(c);
	let suffixe: &str = "ay";
	mot_complet.push_str(suffixe);
	mot_complet
}

// --------------------------------------------------------------

fn voyelle_pig_latin(mot: &String) -> String {
	let suffixe: &str = "-way";
	let mut mot_final: String = mot.trim().to_string();
	mot_final.push_str(suffixe);
	mot_final
}


// Exercice 1
// use std::collections::HashMap;

// fn main() {
// 	let notes = vec![12, 15, 17, 17, 8, 6, 14, 16, 10, 9, 4 ,4, 4];
// 	let mut somme: u8 = 0;

// 	for note in &notes {
// 		somme += note;
// 	}

// 	let mut notes2 = notes;

// 	notes2.sort();


// 	let mut mode = HashMap::new();

// 	for el in &notes2 {
// 		let count = mode.entry(el).or_insert(0);
// 		*count += 1;
// 	}

// 	let mut max: u8 = 0;
// 	let mut cle: u8 = 0;

// 	for (key, value) in mode {
// 		if value > max {
// 			max = value;
// 			cle = *key;
// 		}
// 	}

// 	println!("{:?}", notes2);
// 	println!("La valeur médiane est :{:?}", notes2[notes2.len()/2]);
// 	println!("La valeur mode est : {:?}", cle);

	

	// let moyenne: f64 = somme * 1.0 / taille;
	// println!("{}", somme as f64 / notes.len() as f64);
// }


// use std::collections::HashMap;

// fn main() {


// 	let phrase = String::from("Hello world wonderful world");

// 	let mut count_word_map = HashMap::new();

// 	for mot in phrase.split_whitespace() {
// 		let compte = count_word_map.entry(mot).or_insert(0);
// 		*compte += 1;
// 	}

// 	println!("{:?}", count_word_map);

	// let mut map = HashMap::new();

	// map.insert(String::from("Bleus"), 20);

	// map.entry(String::from("Bleus")).or_insert(10);
	// map.entry(String::from("Jaunes")).or_insert(50);

	// for (key, value) in map {
	// 	println!("{}: {}", key, value);
	// }

	// let equipes = vec![String::from("Bleus"), String::from("Jaunes")];
	// let scores = vec![10, 50];

	// let map: HashMap<_, _> = equipes.iter().zip(scores.iter()).collect();


	// let mut scores = HashMap::new();
	// scores.insert(String::from("Bleus"), 10);
	// scores.insert(String::from("Jaunes"), 50);


	// for (key, value) in &scores {
	// 	println!("{}: {}", key, value);
	// }

	// let equipe = String::from("Bleus");
	// let score = scores.get(&equipe);

	// println!("L'équipe \"Les {}\" a {} points", equipe, score.unwrap());

// }

// fn main() {

// 	let mut mot = String::from("Antony");
// 	let manon: [char;5] = ['M', 'a', 'n', 'o', 'n'];

// 	println!("Mot avant : {}", mot);

// 	for lettre in &manon {
// 		mot.push(*lettre);
// 	}

// 	println!("Mot après : {} ", mot);

// 	let s2 = " et Basile";
// 	let s3 = mot + &s2;

// 	println!("{}", s3);

// 	let mut s4 = String::from("Pipé le loir");

// 	let l = s4.len();

// 	let s5 = &s4[0..l];
// 	println!("{}", s5);

// 	let finalement = format!("{} {} {} {}", s2, s3, s4, s5);
// 	println!("{}", finalement);

// 	let mut vecteur = Vec::new();

// 	vecteur.push(10);
// 	vecteur.push(15);
// 	vecteur.pop();

// 	for i in vecteur{
// 		println!("{}", i);
// 	}

	// let mut s1 = String::from("foo");
	// let s2 = "bar";

	// s1.push_str(s2);
	// println!("s2 est {}", s2);
// }

// #[derive(Debug)]
// enum SpreadsheetCell<'lifetime> {
// 	Int(i32),
// 	Float(f64),
// 	Text(&'lifetime str),
// }

// #[allow(unused_variables)]
// fn main() {
// 	let v: Vec<i32> = Vec::new();
// 	let v2 = vec![1, 2, 3];
// 	let mut v3 = Vec::new();

// 	v3.push(5);
// 	v3.push(10);
// 	v3.push(15);
// 	v3.push(20);

// let vingt: &i32 = &v3[2];

// let test = match v3.get(3) {
// 	Some(el) => v3[3] ,
// 	None => 0,//println!("Il n'y a rien ici."),
// };

// println!("{}", test);

// let mut vecteur = vec![1,2,3,4,5];

// let first = &vecteur[0];

// vecteur.push(6);

// for i in &mut vecteur{
// 	*i += 50;
// 	println!("{}", i);
// }



// let vecteur = vec![
// 	SpreadsheetCell::Int(3),
// 	SpreadsheetCell::Float(64.8),
// 	SpreadsheetCell::Text("vert"),
// ];

// let couleur = vert_ou_non(vecteur);

// println!("C'est {}", couleur);

// }

// fn vert_ou_non (vecteur: Vec<SpreadsheetCell>) -> &str {
// 	let mut retour: &str = "Il n'y a rien.";
// 	for i in &vecteur {
// 	match i {
// 		SpreadsheetCell::Text("vert") => retour = "vert",
// 		_ => (),
// 	};
// }
// retour
// }

// for i in &vecteur {
// 	match i {
// 		SpreadsheetCell::Text("vert") => println!("C'est vert !"),
// 		_ => println!("Rien du tout"),
// 	}
// }

// let moi = "Antony";

// println!("{}", &moi[0]);
// for c in moi.chars(){
// 	println!("{}", c);
// }




// fn main() {
// 	let some_u8_value = Some(4u8);

// 	if let Some(4) = some_u8_value {
// 		println!("quatre");
// 	}

// match some_u8_value {
//     1 => println!("one"),
//     3 => println!("three"),
//     5 => println!("five"),
//     7 => println!("seven"),
//     _ => (),
// }
// }

// fn main() {

// 	println!("Résultat : {}", unwrap_dividend(10, 2));

// 	// let a = Some(5);
// 	// let b: Option<i32> = None;
// 	// println!("{:?}", plus_one(a));
// 	// println!("{:?}", plus_one(b));
// }

// fn plus_one(x: Option<i32>) -> i32 {
// 	match x {
// 		Some(i) => i,
// 		None => 0,
// 	}
// }

// fn safe_division(dividend: i32, divisor: i32) -> Option<i32> {
// 	if divisor == 0 {
// 		None
// 	}
// 	else {
// 		Some(dividend / divisor)
// 	}
// }

// fn unwrap_dividend(dividend: i32, divisor: i32) -> i32 {
// 	match safe_division(dividend, divisor) {
// 		Some(quotient) => quotient,
// 		None => 0,
// 	}
// }

// #[derive(Debug)]
// #[allow(dead_code)]

// enum PaysUe {
// 	France,
// 	Allemagne,
// 	Italie,
// 	Espagne,
// 	Portugal,
// 	PaysBas,
// 	Luxembourg,
// 	Autriche,

// }

// #[allow(dead_code)]
// enum Pieces {
// 	Cent1,
// 	Cent2,
// 	Cent5,
// 	Cent10,
// 	Cent20,
// 	Cent50,
// 	Euro1,
// 	Euros2(PaysUe),
// }


// fn main() {
// 	println!("{}", coin_values(Pieces::Euros2(PaysUe::France)));
// }

// fn coin_values (piece: Pieces) -> u8 {
// 	match piece {
// 		Pieces::Cent1 => 1,
// 		Pieces::Cent2 => 2,
// 		Pieces::Cent5 => 5,
// 		Pieces::Cent10 => 10,
// 		Pieces::Cent20 => 20,
// 		Pieces::Cent50 => 50,
// 		Pieces::Euro1 => 100,
// 		Pieces::Euros2(PaysUe::France) => { 
// 			println!("Cocorico");
// 			200},
// 		Pieces::Euros2(_) => 200,
// }
// 	}

// 	let some_number = Some(5);
// 	let some_string = Some("a string");

// 	let absent_number: Option<i32> = None;
// }



// #[derive(Debug)]
// enum IpAdrr {
// 	V4(u8,u8,u8,u8),
// 	V6(String),
// }

// impl IpAdrr {

// 	fn rec_ip_v4(ip1: u8, ip2: u8, ip3: u8, ip4: u8) -> IpAdrr {
// 		IpAdrr::V4(ip1, ip2, ip3, ip4)
// 	}

// 	fn prnt_ip(&self) {
// 		println!("{:?}", self);
// 	}

// }

// fn main() {
// 	let t430 = IpAdrr::V4(192,168,1,12);

// 	println!("t430 : {:?}", t430.prnt_ip());

// 	let lain = IpAdrr::rec_ip_v4(192,168,1,17);

// 	println!("lain : {:?}", lain.prnt_ip());
// }

// struct Tel(u8, u8, u8, u8, u8);

// fn main() {
// 	loop{
// 	println!("Entrez votre numéro de téléphone.");
// 	let mut user_input = String::new();
// 	std::io::stdin().read_line(&mut user_input)
// 	.expect("Problème mémoire.");
// 	// let numero_telephone: u32 = match user_input.trim().parse() {
// 	// 	Ok(num) => num,
// 	// 	Err(_) => continue,
// 	// };
// 	let mon_numero = build_tel(&user_input);
// 	// println!("{:?}", mon_numero.0);
// 	// for el in mon_numero.iter(){
// 	// 	println!("{:?}", el);
// 	// }
// 	display_telephone(&mon_numero);
// }
// }
// fn build_tel(numero: &str) -> Tel {
// 	let un_deux: u8 = match numero[..2].parse(){
// 		Ok(num) => num,
// 		Err(_) => panic!(),
// 	};
// 	let trois_quatre: u8 = match numero[2..4].parse(){
// 		Ok(num) => num,
// 		Err(_) => panic!(),
// 	};
// 	let cinq_six: u8 = match  numero[4..6].parse(){
// 		Ok(num) => num,
// 		Err(_) => panic!(),
// 	};
// 	let sept_huit: u8 = match numero[6..8].parse(){
// 		Ok(num) => num,
// 		Err(_) => panic!(),
// 	};
// 	let neuf_dix: u8 = match numero[8..10].parse(){
// 		Ok(num) => num,
// 		Err(_) => panic!(),
// 	};
// 	Tel(un_deux, trois_quatre, cinq_six, sept_huit, neuf_dix)
// }

// fn display_telephone(numero: &Tel) {
// 	println!("{}.{}.{}.{}.{}", numero.0, numero.1, numero.2, numero.3, numero.4);
// }

// 	struct User {
// 		nom: String,
// 		prenom: String,
// 		adresse: String,
// 		courriel: String,
// 		telephone: String,
// 	}

// fn main() {
// 	let user1 = build_user(String::from("Merle"), String::from("Antony"), 
// 		String::from("8 allée de la corderie"), String::from("anto@anto"), 
// 			String::from("06 84 25 99 61"));
// 	display_user(&user1);

// 	let user2 = User {
// 		prenom: String::from("Manon"),
// 		telephone: String::from("06 69 28 20 81"),
// 		..user1
// 	};
// 	println!();
// 	display_user(&user2);
// }

// fn build_user(nom: String, prenom: String, adresse: String, courriel: String, telephone: String) -> User {
// 	User{
// 	nom,
// 	prenom,
// 	adresse,
// 	courriel,
// 	telephone,
// }
// }

// fn display_user(fiche: &User) {
// 	println!("{}", fiche.nom);
// 	println!("{}", fiche.prenom);
// 	println!("{}", fiche.adresse);
// 	println!("{}", fiche.courriel);
// 	println!("{}", fiche.telephone);
// }

// fn main() {

// 	const LIGNES: usize = 9;
// 	const COLS: usize = 6;

// 	let heart: [[char; COLS]; LIGNES] = [
// 		['.', '.', '.', '.', '.', '.'],
//         ['.', 'O', 'O', '.', '.', '.'],
//         ['O', 'O', 'O', 'O', '.', '.'],
//         ['O', 'O', 'O', 'O', 'O', '.'],
//         ['.', 'O', 'O', 'O', 'O', 'O'],
//         ['O', 'O', 'O', 'O', 'O', '.'],
//         ['O', 'O', 'O', 'O', '.', '.'],
//         ['.', 'O', 'O', '.', '.', '.'],
//         ['.', '.', '.', '.', '.', '.'],
//         ];

//     for i in 0..COLS{
//     	for j in 0..LIGNES{
//     		print!("{}", heart[j][i]);
//     	}
//     	println!();
//     }
// }





// fn main(){
// 	let string_literal: &str = "Bonjour tout le monde !";

// 	println!("{}", &string_literal[8..string_literal.len()]);
// }


// use std::io;

// fn main() {
	// loop{
	// 	println!("Entrez le premier nombre la séquence de Collatz");
	// 	let mut user_input = String::new();
	// 	io::stdin().read_line(&mut user_input)
	// 	.expect("Problème mémoire.");
	// 	let chiffre: u32 = match user_input.trim().parse() {
	// 		Ok(num) => num,
	// 		Err(_) => continue,
	// 	};
// 		collatz(chiffre);
// 		break;
// 	}
	
// }

// fn collatz(nombre: u32) -> u32{
// 	if nombre == 1 {
// 		return 1
// 	}
// 	else if nombre % 2 != 0 {
// 		// println!("Impair");
// 		println!("{}", (3 * nombre) + 1);
// 		return collatz((3 * nombre)  +1)
// 	}
// 	else {
// 		// println!("pair");
// 		println!("{}", nombre / 2);
// 		return collatz(nombre / 2)
// 	}
// }

// fn main() {
// 	// let prenom = String::from("Antony Manon");
// 	let prenom = "Antony Manon";

// 	let antony = &prenom[..6];
// 	let manon = &prenom[7..];

// 	println!("{} et {}", antony, manon);

// 	println!("Le premier mot de la phrase est {}", first_word(&prenom));
// }

// fn first_word(some_string: &str) -> &str {
// 	let tableau = some_string.as_bytes();

// 	for (i, &element) in tableau.iter().enumerate() {
// 		if element == b' '{
// 			return &some_string[..i]
// 		}
// 	}
// 	&some_string[..]
// }

// fn main() {
// 	let some_string = no_dangle();
// 	println!("{}", some_string);
// }

// fn no_dangle() -> String {
// 	let s = String::from("Hello");
// 	s
// }


// fn main(){
// 	let mut mot = String::from("Hello");
// 	complete_mot(&mut mot);
// 	println!("{}", mot);
// }

// fn complete_mot(some_string: &mut String) {
// 	some_string.push_str(", world !");
// }

// fn main() {
// 	let s = String::from("Hello");
// 	let lenght = return_lenght(&s);
// 	println!("{:?} a une longueur de {}.", s, lenght);
// }

// fn return_lenght(some_string: &String) -> usize {
// 	let lenght: usize = some_string.len();
// 	lenght
// }


// fn main() {
// 	let s1 = String::from("Hello");

// 	let (s2, len) = calculate_length(s1);
// 	println!("String {:?} is {} characters long.", s2, len);
// }

// fn calculate_length(s: String) -> (String, usize) {
// 	let lenght = s.len();
// 	(s, lenght)
// }

// fn main() {
// 	let s1 = gives_ownership();

// 	let s2 = String::from("Hello");

// 	let s3 = takes_and_gives_back(s2);
// }

// fn gives_ownership() -> String {
// 	let some_string = String::from("Hello");
// 	some_string
// }

// fn takes_and_gives_back(some_string: String) -> String {
// 	some_string
// }

// fn main() {
	

// 	let ms = String::from("Je suis mutable.");
	
// 	let x = 5;

// 	println!("x est toujours valable est vaut {:?}", x);



// 	takes_ownership(ms);


// 	makes_copy(x);
// }

// fn takes_ownership(some_string: String) {
// 	println!("{}", some_string);
// }

// fn makes_copy(some_integer: i32) {
// 	println!("{}", some_integer);
// }

// fn main() {
// 	let day_nth = [ 
// 	"first", "second", "third",
// 	"fourth", "fifth", "sixth",
// 	"seventh", "eighth", "ninth",
// 	"tenth", "eleventh", "twelth"];

// 	let numbers = [
// 	"One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Eleven", "Twelve"];

// 	let cadeaux = [
// 	"partridge in a pear-tree",
// 	"Turtle doves",
// 	"French hens",
// 	"colly birds",
// 	"gold rings",
// 	"geese a laying",
// 	"swans a swimming",
// 	"maids a milking",
// 	"drummers drumming",
// 	"pipers piping",
// 	"ladies dancing",
// 	"lords a leaping"];

// 	for jour in 1..13{
// 	println!("On the {} of Christmas my love sent to me", day_nth[jour - 1]);
// 	println!("{} {}", numbers[jour - 1], cadeaux[jour - 1] );
// 	if (jour - 1) >= 1{
// 		for repetition in (0..jour - 1).rev(){
// 		// println!("{:?}", jour);
// 		println!("{} {}", numbers[repetition], cadeaux[repetition]);
// 	}}
// }
// }


// fn main(){
//    let mut line = String::new();
//    println!("Enter your name :");
//    let b1 = std::io::stdin().read_line(&mut line).unwrap();
//    println!("Hello , {}", line);
//    println!("no of bytes read , {}", b1);
// }


// fn main() {
// 	for n in 1..17{
// 	println!("Fibonnaci de {} = {}", n, fibo(n));
// }
// }

// fn fibo(n: u32) -> u32 {
// 	let n = n + 1;
// 	let mut i = 0;
// 	let mut j = 1;
// 	let mut k;
// 	for chiffre in 1..n{
// 		k = i + j;
// 		i = j;
// 		j = k;
// 	}
// 	i
// }


// fn main() {
// 	let celsius = 30.0;
// 	let farenheit = 78.0;
// 	println!("{}° celsius = {}° farenheit", celsius, cel_to_far(celsius));
// 	println!("{}° farenheit = {}° celsius", farenheit, far_to_cel(farenheit));
// }

// fn cel_to_far(cel: f32) -> f32{
// 	(cel * 1.8) + 32.0
// }

// fn far_to_cel(far: f32) -> f32{
// 	(far - 32.0) / 1.8
// }

// fn main() {
	// let a: [u32; 10] = [0;10];

	// let mut index = 0;
	// while index < 10 {
	// 	print!("[{:?}] ", a[index]);
	// 	index += 1;
	// }

	// for element in a.iter(){
	// 	print!("[{:?}] ", element);
	// }

// 	for number in (1..4){
// 		println!("{:?}", number);
// 	}
// 	println!("DECOLLAGE");
// }

// fn main(){
// 	loop {
// 		println!("Encore !");
// 	}
// }

// fn main(){
// 	let nombre = 7;
// 	if nombre != 0 {
// 		println!("Le nombre n'est pas 0");
// 	}
// }

// fn main() {
//     println!("Hello, world!");

    
//     let x = five();
//     println!("x = {:?}", x);
//     let x = plus_one(x);
//     println!("x = {:?}", x);
// }

// fn five() -> u32 {
// 	5
// }

// fn plus_one(x: u32) -> u32 {
// 	x + 1
// }