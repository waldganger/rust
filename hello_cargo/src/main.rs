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