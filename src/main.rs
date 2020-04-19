fn main() {
	let day_nth = [ 
	"first", "second", "third",
	"fourth", "fifth", "sixth",
	"seventh", "eighth", "ninth",
	"tenth", "eleventh", "twelth"];

	let numbers = [
	"One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Eleven", "Twelve"];

	let cadeaux = [
	"partridge in a pear-tree",
	"Turtle doves",
	"French hens",
	"colly birds",
	"gold rings",
	"geese a laying",
	"swans a swimming",
	"maids a milking",
	"drummers drumming",
	"pipers piping",
	"ladies dancing",
	"lords a leaping"];

	for jour in 1..13{
	println!("On the {} of Christmas my love sent to me", day_nth[jour - 1]);
	println!("{} {}", numbers[jour - 1], cadeaux[jour - 1] );
}
}


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