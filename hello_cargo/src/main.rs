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