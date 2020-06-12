fn main() {
	let s = ' ';
	let r = '\u{2588}';
	let f = '\u{2500}';
	let d = '\u{2571}';
	let v = '\u{2502}';

	let mut l1: [(i32, i32, i32, char, bool); 4] = [(2, 8, 1, s, false), (1, 1, 1, r, false), (4, 28, 4, f, false), (1, 1, 1, r, false)];
	let mut l2: [(i32, i32, i32, char, bool); 5] = [(1, 7, 1, s, false), (1, 1, 1, d, false), (4, 28, 4, s, false), (1, 1, 1, d, false), (1, 1, 0, v, false)];
	let mut l3: [(i32, i32, i32, char, bool); 6] = [(1, 0, 1, s, false), (1, 1, 1, d, false), (8, 28, 4, s, false), (1, 1, 1, d, false), (0, 0, 0, s, false), (1, 1, 1, v, false)];
	let l4 = [(1, r), (4, f), (1, r), (1, s), (1, v)];
	let l5_fantome = [(0, v), (0, s), (0, v), (0, s), (0, v)];
	let l6 = [(1, v), (4, s), (1, v), (1, s), (1, r)];
	let l7_fantome = [(1, v), (4, s), (1, v), (0, s), (1, d)];
	let l8 = [(1, r), (4, f), (1, r), (2, s)];
	

	let mut vecteur: Vec<String> = Vec::new();

	fn pl2(arr: &mut [(i32, i32, i32, char, bool)], vecteur: &mut Vec<String>) {
		let mut line = String::new();
		for tuple in arr.iter_mut() {	
			// println!("{:?}", tuple);

			let (mut mouvement, plafond, step, c, starter) = *tuple; 
			if starter {
				if mouvement >= plafond {
				tuple.0 = plafond - step;
				mouvement = tuple.1;
				// println!("Limitation du mouvement à {}", mouvement);
				}

				c_push(mouvement, c, &mut line);
				tuple.0 += step;

			} else {
				c_push(mouvement, c, &mut line);
				tuple.4 = true;
				tuple.0 += step;
			}
			

		}
		vecteur.push(line.clone());

	}

	fn c_push(mouvement: i32, character: char, s: &mut String) {
		for i in 0..mouvement {
			s.push(character)
		}
	}

	// boucle finale d'affichage

	// pour chaque carré
	for n in 0..7 {
		pl2(&mut l1, &mut vecteur);
		pl2(&mut l2, &mut vecteur);

		// pour chaque ligne diag de chaque carré
		for i in (0..n).rev() {
			// espace
			l3[0].1 = i + 1; 						// plafond descendant
			l3[0].0 = n;						// mouvement montant

			// espaces centraux
			l3[2].0 = (n+1) * l3[2].2;

			// espaces finaux

			l3[4].0 += 1 ;
			l3[4].1 = n ;
			
			

			// l3[4].0 = n - 2 ;
			println!("{:?}", l3[4]);
			pl2(&mut l3, &mut vecteur);
			
			// l3[0].4 = false;
			
		}
			// réinitialisation du tuple l3[4]
			l3[4].0 = 0;
			l3[4].1 = 0;

	}
	for s in vecteur {
		println!("{}", s);
	}

	// println!("{:?}", l1[0]);

	// for n in 0..7 {
	// 	// let s = pl2(&l1);
	// 	stock.push(pl2(&l1));
	// 	// println!("{}", pl2(&l1));
	// 	for ligne in &stock {
	// 		println!("{}", ligne);
	// 	}
	// }

	// assert_eq!(l1[0].0, 2);

	// for x in 1..8 {
	// 	println!("{}", pl(&l1, x));
	// 	for y in 0..x  {
	// 		println!("{}", pl(&l2, x - y)); 
			
	// 	}
		
	// 	println!("{}", pl(&l3, x));
	// }

	// fn pl(arr: &[(i32, char)], x: i32) -> String {
	// 	let mut line = String::new();
	// 	for i in arr.iter() {
	// 		if i.1 == '\u{2588}' || i.1 == '\u{2571}' || i.1 == '\u{2502}' {
	// 			for j in 0..i.0 {
	// 				line.push(i.1);
	// 			}
	// 		} else if i.1 == ' ' && i.0 < 4 {
	// 			for k in 0..i.0 + (x - 1) {
	// 				line.push(i.1);
	// 				}
	// 		} else {
	// 			for l in 0..i.0 * x {
	// 				line.push(i.1);
	// 			}
	// 		}	
	// 	}
	// 	line
	// }

	// idée 3 : faire chaque ligne correctement 7 fois et les stocker dans un array
	// pour affichage final.

	// let pre_space = 2;
	// let mut flat_line = 0;
	// for i in 0..7 {
	// 	flat_line += 4;
	// 	let mut l1 = String::new();
	// 	for j in 0..i+pre_space {
	// 		l1.push(' ');
	// 	}
	// 	l1.push('\u{2588}');
	// 	for k in 0..flat_line {
	// 		l1.push('\u{2500}');
	// 	}
	// 	l1.push('\u{2588}');
	// 	println!("{}", l1);
		// l1 imprimée

	// 	let mut l2 = String::new();
	// }
	// assert_eq!(flat_line, 28);
}

// fn main() {
// 	for i in 0..10 {
// 		l(i);
// 		if i > 0 && i < 9 {
// 			println!();
// 		}
// 	}
// }

// fn p(x: i32) -> String {
// 	let mut ligne = String::from(" ");
// 	for i in 1..x {
// 		let c = i.to_string();
// 		ligne.push_str(&c[..]);
// 	}
// 	for i in (1..x - 1).rev() {
// 		let c = i.to_string();
// 		ligne.push_str(&c[..]);
// 	}
// 	ligne
// }

// fn l(x:i32) {
// 	let mut v: Vec<String> = Vec::new();
// 	for i in 0..x{
// 		v.push(p(i+2));
// 		println!("{:^18}", p(i+2));
// 	}
// 	if x > 1 {
// 		v.reverse();
// 		for e in &v[1..] {
// 			println!("{:^18}", e);
// 		}
// 	}
// 	print!("yes");
// }

// fn main() {for i in 3..10{d(i);}}
// fn d(l: usize) {
// 	let w = l*2-1;
// 	let mut r = String::new();
// 	let  a = '*';
// 	for i in 1..l+1{
// 		if i==1{
// 			r.push(a);
// 		}else{
// 			r.push(a);
// 			r.push(a);
// 		}
// 		println!(" {:^lar$}", r, lar = w);
// 	}
// 	println!(" {:^lar$}\n", a, lar = w);
// }

// fn main() {
// 	let ln = 9;
// 	let w = ln * 2 - 1;

// 	let mut ligne = String::new();
// 	let  a = '*';
// 	for i in 1..ln+1 {
// 		if i == 1 {
// 			ligne.push(a);
// 		} else {
// 			ligne.push(a);
// 			ligne.push(a);
// 		}
// 		println!("{:^lar$}", ligne, lar = w);
// 	}
// 	println!("{:^lar$}\n", a, lar = w);
// }

// fn main() {
// 	for i in 3..10 {
// 		draw_tree(i);
// 	}
// }

// fn draw_tree(l: usize) {
// 	let w: usize = l * 2 - 1;

// 	let mut ligne = String::new();
// 	let  a = '*';
// 	for i in 1..l+1 {
// 		if i == 1 {
// 			ligne.push(a);
// 		} else {
// 			ligne.push(a);
// 			ligne.push(a);
// 		}
// 		println!(" {:^lar$}", ligne, lar = w);
// 	}
// 	println!(" {:^lar$}\n", a, lar = w);
// }
	
	// let mut l: usize = 1;
	// for x in 1..4 {
	// 	let mut y = x;
	// 	let mut s = String::new();
	// 	while y > 0 {
	// 		// print!("{:^w$}", '*', w = x);

	// 		y -= 1;
	// 	}
	// println!();
// 	}
// }
// fn main() {
// 	// let taille = 3;
// 	for i in 0..3 {
// 		for s in (0..i).rev() {
// 			print!(" ");
// 		}
// 		for x in 0..i {
// 			print!("*");
// 		}
// 		for s in (0..i).rev() {
// 			print!(" ");
// 		}
// 		println!();
// }
	
// }

// fn main() {
// 	let s1 = String::from("abcd");
// 	let s2 = "xyz";

// 	let result = longest(s1.as_str(), s2);
// 	println!("The longest string is {}", result);
// 	next();
// }

// fn longest<'a>(s: &'a str, t: &'a str) -> &'a str {
// 	if s.len() > t.len() {
// 		s
// 	} else {
// 		t
// 	}
// }
// fn main() {
// 	let v = vec![12, 34, 78, 4, 78, 38];
// 	let a:[i32;5] = [1, 2, 5, 4, 3];
// 	println!("{}", generic_largest(&v));
// 	println!("{}", generic_largest(&a));
// 	// println!("{}", big_num(&v));
// 	// println!("{}", big_num(&a));
// 	let prenom = ['a', 'n', 't', 'o', 'n'];
// 	print!("{}", generic_largest(&prenom));
	
// }

// fn big_num(liste: &[i32]) -> i32 {
// 	let mut maximum: i32 = liste[0];
// 	for &n in liste {
// 		if n > maximum {
// 			maximum = n;
// 		}
// 	}
// 	maximum
// }

// fn big_char(liste: &[char]) -> char {
// 	let mut max_char = liste[0];
// 	for &c in liste {
// 		if c > max_char {
// 			max_char = c;
// 		}
// 	}
// 	max_char
// }

// fn generic_largest<T: PartialOrd + Copy>(liste: &[T]) -> T {
// 	let mut largest = liste[0];
// 	for &el in liste {
// 		if el > largest {
// 			largest = el;
// 		}
// 	}
// 	largest
// }

// use std::env;
// fn main() {
// 	let args: Vec<String> = env::args().collect();
// 	for c in &args[1..]{
// 		let d = c.parse::<u32>().unwrap();
// 		println!("{}", romanize(d));
// 	}
// }


// fn main() {
// 	loop{
// 	let mut user_input = String::new();
// 	std::io::stdin().read_line(&mut user_input).expect("FAILURE");

// 	println!("{}", romanize(user_input.trim().parse().unwrap()));
// 	}
// }

// fn romanize(c: u32) -> String {
// 	let mut result = String::new();
// 	let mut x = c;
// 	while x > 0 { // on enlève M par M, D puis, C, puis L, X par X,puis V, puis I par I
// 		if x >= 1000 {
// 			result.push('M');
// 			x -= 1000;
// 		} else if x >= 900 {
// 			result.push_str("CM");
// 			x %= 900;
// 		}
// 		else if x >= 500 {
// 			result.push('D');
// 			x -= 500;
// 		} else if x >= 400 {
// 			result.push_str("CD");
// 			x %= 400;
// 		}
// 		else if x >= 100 {
// 			result.push('C');
// 			x -= 100;
// 		} else if x >= 90 {
// 			result.push_str("XC");
// 			x %= 90;
// 		}
// 		else if x >= 50 {
// 			result.push('L');
// 			x -= 50;
// 		} else if x > 39 {
// 			result.push_str("XL");
// 			x %= 40;
// 		}
// 		else if x >= 10 {
// 			result.push('X');
// 			x -= 10;
// 		} else if x > 8 {
// 			result.push_str("IX");
// 			x %= 9;
// 		}
// 		else if x >= 5 {
// 			result.push('V');
// 			x -= 5;
// 		} else if x > 3 {
// 			result.push_str("IV");
// 			x %= 4;
// 		}
// 		else {
// 			result.push('I');
// 			x -= 1;
// 		}
// 	}
// 	result
// }

// pub trait Summary {
// 	fn summarize_author(&self) -> String;

// 	fn summarize(&self) -> String {
// 		format!("Read more from {}", self.summarize_author())
// 	}
// }

// pub struct NewsArticle {
// 	pub headline: String,
// 	pub location: String,
// 	pub author: String,
// 	pub content: String,
// }

// pub struct Tweet {
// 	pub username: String,
// 	pub content: String,
// 	pub reply: bool,
// 	pub retweet: bool,
// }

// impl Summary for NewsArticle {
// 	fn summarize(&self) -> String {
// 		format!("{} by {}. ({})", self.headline, self.author, self.location)
// 	}
// }

// impl Summary for NewsArticle {
// 	fn summarize_author(&self) -> String {
// 		format!("{}", self.author)
// 	}
// }

// impl Summary for Tweet {
// 	fn summarize_author(&self) -> String {
// 		format!("@{}", self.username)
// 	}

	// fn summarize(&self) -> String {
	// 	format!("{} : {}", self.username, self.content)
	// }
// }


// fn main() {
// let tweet = Tweet {
// 	username: String::from("antony"),
// 	content: String::from("J'apprends Rust depuis avril, c'est très intéressant."),
// 	reply: false,
// 	retweet: true,
// };

// println!("One new tweet : {}", tweet.summarize());

// let article = NewsArticle {
// 	headline: String::from("La fête du miassou bat son plein à Fillou-les-Rillettes !"),
// 	location: String::from("Fillou-les-Rillettes"),
// 	author: String::from("Anne-Sophie Lapix"),
// 	content: String::from("La 28ème fête du miassou a battu des records de fréquentation cette année, amenant des millers de personnes dans le petit village de Fillou-les-Rillettes"),
// };
// // println!("{}", article.summarize());
// 	notify(article);

// }

// pub fn notify(item: impl Summary) {
// 	println!("Breaking news : {}", item.summarize());
// }

// fn main() {
// 	let b = "bottle";
// 	let c = 's';
// 	let d = "of beer";
// 	let w = " on the wall";
// 	let t = "Take one down and pass it around";
// 	let n ="no more";
// 	let m = "No more";
// 	let g = "Go to the store and buy some more";

// 	for i in (0..100).rev() {
// 		match i {
// 			2 => println!("{} {}{} {}{}, {} {}{} {}.\n{}, {} {} {}{}.\n",i,b,c,d,w,i,b,c,d, t,i-1,b,d,w),
// 			1 => println!("{} {} {}{}, {} {} {}.\n{}, {} {}{} {}{}.\n",i,b,d,w,i,b,d, t,n,b,c,d,w),
// 			0 =>println!("{} {}{} {}{}, {} {}{} {}.\n{}, 99 {}{} {}{}.\n",m,b,c,d,w,n,b,c,d ,g,b,c,d,w),
// 			_ => println!("{} {}{} {}{}, {} {}{} {}.\n{}, {} {}{} {}{}.\n",i,b,c,d,w,i,b,c,d, t,i-1,b,c,d,w),
// 		}
// 	}
// }

// struct Point<T, U> {
// 	x: T,
// 	y: U,
// }

// impl<T,U> Point<T, U> {
// 	fn melange<V, W>(self, other: Point<V, W>) -> Point<T, W> {
// 		Point {
// 			x: self.x,
// 			y: other.y,
// 		}
// 	}
// }

// fn main() {
// 	let p1 = Point {x: 5, y: 10.34};
// 	let p2 = Point {x: "Hello", y: 'c'};
// 	let p3 = p1.melange(p2);
// 	println!("p3 = x: {} y: {}", p3.x, p3.y);
// }

// fn main() {
// 	let v = vec![12, 34, 78, 4, 78, 38];
// 	let a:[i32;5] = [1, 2, 5, 4, 3];
// 	println!("{}", big_num(&v));
// 	println!("{}", big_num(&a));
// 	let prenom = ['a', 'n', 't', 'o', 'n'];
// 	print!("{}", big_char(&prenom));
	
// }

// fn big_num(liste: &[i32]) -> i32 {
// 	let mut maximum: i32 = liste[0];
// 	for &n in liste {
// 		if n > maximum {
// 			maximum = n;
// 		}
// 	}
// 	maximum
// }

// fn big_char(liste: &[char]) -> char {
// 	let mut max_char = liste[0];
// 	for &c in liste {
// 		if c > max_char {
// 			max_char = c;
// 		}
// 	}
// 	max_char
// }

// fn generic_largest<T>(liste: &[T]) -> T {
// 	let mut largest = liste[0];
// 	for &el in liste {
// 		if el > largest {
// 			largest = el;
// 		}
// 	}
// 	largest
// }



// fn main() {
// 	let mut a = [1; 1001];
// 	let mut v: Vec<usize> = vec![];
// 	for i in 2..1001 {
// 		for j in 2..i {
// 			if i%j == 0 {
// 				a[i] = 0;
// 			}
// 		}
// 		}
// 	for i in 1..1001{
// 		if a[i] == 1 {

// 			let r = i.to_string().chars().rev().collect::<String>().parse::<usize>().unwrap();
// 			if a[r] == 1 && r != i {
// 				v.push(r);}
// 		}
// 	}
// 	v.sort();
// 	for n in v{
// 		println!("{}", n);
// 	}
// }

// use std::fs::File;
// use std::io;
// use std::io::Read;

// fn main() {
// 	match read_username_from_file() {
// 		Ok(s) => println!("{}", s),
// 		Err(e) => println!("{:?}", e),
// 	};
// }

// fn read_username_from_file() -> Result<String, io::Error> {
// 	let f = File::open("Hello.txt");
// 	let mut f = match f {
// 		Ok(file) => file,
// 		Err(e) => return Err(e),
// 	};
// 	let mut s = String::new();
// 	match f.read_to_string(&mut s) {
// 		Ok(_) => Ok(s),
// 		Err(e) => Err(e),
// 	}
// }

// fn read_court() -> Result<String, io::Error> {
// 	let mut s = String::new();
// 	File::open("Hello.txt")?.read_to_string(&mut s)?;
// 	Ok(s)
// }

// fn main() {
	// let f = std::fs::File::open("Hello.txt").expect("Fichier introuvable.");
// 	let f = std::fs::File::open("Hello.txt").unwrap_or_else(|error| {
// 		panic!("BIG PB : {:?}", error)});
// }

// fn main() {
// 	let mut user_input = String::new();
// 	std::io::stdin().read_line(&mut user_input).expect("Ca marche pas");
// 	println!("{}", user_input);
// }

// fn main() {
// 	let a = String::from("Antony");
// 	println!("{}", conc(a));

// }

// fn conc(mut debut: String) -> String {
// 	debut.push_str( "et Manon");
// 	debut
// }

// use std::fs::File;
// use std::io::ErrorKind;

// fn main() {
// 	let f = File::open("hello.txt").unwrap_or_else(|error| {
// 		if error.kind() == ErrorKind::NotFound {
// 			File::create("hello.txt").unwrap_or_else(|error| {
// 				panic!("Problème à la création du fichier : {:?}", error);
// 			})} else {
// 				panic!("Problème à l'ouverture du fichier : {:?}", error);
// 			}
// 	});


// }

// fn main() {
// 	for i in 1..201{
// 		let c = 0;
// 		if powify(i, c) == true {
// 			println!("{}", i);
// 		}
// 	}
// }
// fn powify(n: u32, mut c: u32) -> bool {
// 		let mut sum = 0;
// 		for d in n.to_string().chars(){			
// 			sum += d.to_digit(10).unwrap().pow(2);
// 		}
// 		c += 1; 
// 		if sum == 1 {return true;}
// 		if c > 10 {return false;}		
// 		powify(sum, c)
// }


// fn main() {
// 	for i in 1..201{
// 		let mut c=0;
// 		for j in 1..i{
// 			if c>i{
// 				println!("{}", i);
// 				break;
// 			}
// 			if i%j==0{
// 				c+=j;
// 			}
// 		}
// 	}
// }

// use std::collections::HashMap;

// fn main() {
// 	let mut user_input = String::new();
// 	std::io::stdin().read_line(&mut user_input)
// 	.ok()
// 	.expect("Erreur de mémoire");

// 	let mut v: Vec<&str> = vec![];
// 	for mot in user_input.split_whitespace() {
// 		v.push(mot);
// 	}

// 	let mut hr_registry: HashMap<&str, &str> = HashMap::new();
	
// 	if (v[0] == "Add" || v[0] == "add") && v[2] == "to" {
// 		hr_registry.entry(v[1]).or_insert(v[3]);
// 	} else {println!("Saisie invalide. Add xxx to dpt.")};

// 	for (key, value) in hr_registry {
// 		println!("{} est affecté au département {} ", key, value);
// 	}

// }
// fn main() {

// 	let equipes = vec![String::from("Bleus"), String::from("Verts")];
// 	let scores = vec![24, 9];

// 	let mut regroup_map: HashMap<_,_> = equipes.into_iter().zip(scores.into_iter()).collect();

// 	regroup_map.insert(String::from("Jaunes"), 13);

// 	let j = String::from("Jaunes");
// 	let score = regroup_map.get(&j).unwrap();

// 	print!("{}", score);

// 	for (key, value) in &regroup_map {
// 		println!("{}: {}", key, value);
// 	}
// 	println!();

// 	regroup_map.entry(j).or_insert(55);
// 	regroup_map.entry(String::from("Rouges")).or_insert(17);

// 	for (k,v) in regroup_map {
// 		println!("{}: {} ", k, v);
// 	}
// 	println!();

// 	let texte = "Il était un petit navire, il était un petit navire, qui n'avait jamais jamais navigué ohé ohé !";

// 	let mut compte_mot = HashMap::new();

// 	for word in texte.split_whitespace() {
// 		let count = compte_mot.entry(word).or_insert(0);
// 		*count += 1;
// 	}

// 	for (w,n) in compte_mot {
// 		println!("{} : {}", w, n);
// 	}


	// let s1 = String::from("Antony ");
	// let s2 = String::from("Manon ");
	// let s3 = String::from("et Basile");

	// let s4 = format!("{} {} {}", s1, s2, s3);
	// println!("{}", s4);
	// println!("{}", s1);

	// for c in 0..8366 {
	// 	println!("{}", char::from_u32(c).unwrap()); //(c + b'0') as char)
	// }
	// (0..200).for_each(|c| println!("{}", std::char::from_u32(c).unwrap_or(' ')));
	// for chr in (0..200).filter_map(|c| std::char::from_u32(c + u32::from(b'0'))) {
	// 	println!("{}", chr);
	// }
// }

// fn main() {
// 	let mut v: Vec<i32> = vec![];
// 	for x in 0..100 {
// 		v.push(x);
// 	}
// 	for i in &mut v {
// 		*i = *i * *i;
// 		println!("[{}]", i);
// 	}
// }


// enum Classeur {
// 	Int(i32),
// 	Float(f64),
// 	Text(String),
// }

// fn main() {
// 	let ligne = vec![
// 		Classeur::Int(456),
// 		Classeur::Text(String::from("Ventes")),
// 		Classeur::Float(987367.78987),
// 	];

// 	for case in &ligne{
// 		match case {
// 			Classeur::Int(num) => println!("{}", num),
// 			Classeur::Float(num) => println!("{}", num),
// 			Classeur::Text(mot) => println!("{}", mot),
// 		}
// 	}
// }

// fn main() {
// 	let d = [ 
// 	"First", "Second", "Third",
// 	"Fourth", "Fifth", "Sixth",
// 	"Seventh", "Eighth", "Ninth",
// 	"Tenth", "Eleventh", "Twelfth"];

// 	let n = [
// 	"A", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Eleven", "Twelve"];

// 	let c = [
// 	"Partridge in a Pear Tree",
// 	"Turtle Doves",
// 	"French Hens",
// 	"Calling Birds",
// 	"Gold Rings",
// 	"Geese-a-Laying",
// 	"Swans-a-Swimming",
// 	"Maids-a-Milking",
// 	"Ladies Dancing",
// 	"Lords-a-Leaping",
// 	"Pipers Piping",
// 	"Drummers Drumming"];

// 	for j in 0..12{
// 		println!("On the {} day of Christmas\nMy true love sent to me", d[j]);
// 		print!("{} {}", n[j], c[j] );
// 		if j == 0 {
// 			println!(".");
// 		} else if j == 1 {
// 			println!(", and");
// 		} else {
// 			println!(",");
// 		}

// 		if j > 0{
// 			for r in (0..j).rev(){
// 				if r == 1 {
// 					println!("{} {}, and", n[r], c[r]);
// 				} else if r == 0 {
// 					println!("{} {}.", n[r], c[r]);
// 				}
// 				else {println!("{} {},", n[r], c[r]);
// 			}
// 	}
// }
// println!();
// }
// }


// for jour in 1..13{
// 		println!("On the {} day of Christmas\nMy true love sent to me", day_nth[jour - 1]);
// 		print!("{} {}", numbers[jour - 1], cadeaux[jour - 1] );
// 		if jour == 1 {
// 			println!(".");
// 		} else {
// 			println!(",");
// 		}

// 		if (jour - 1) >= 1{
// 			for repetition in (0..jour -1).rev(){
// 				println!("{}", repetition);
// 				if repetition == 0 {
// 					println!("{} {}.", numbers[repetition], cadeaux[repetition]);
// 				} else if repetition == 1 {
// 					println!("{} {}, and", numbers[repetition], cadeaux[repetition]);
// 				}
// 				else {println!("{} {},", numbers[repetition], cadeaux[repetition]);
// 			}
// 	}
// }
// println!();
// }
// }

// fn main () {
// 	let mut a: [i32; 101] = [0;101];
// 	let mut j = 0;
// 	for i in 0..101 {
// 		a[i] = j;
// 		j += 1;
// 	}
// 	for n in 2..101 {
// 		for j in 2..n {
// 			if n % j == 0 {
// 				a[n] = 0;
// 			}
// 		}
// 	}

// 	for i in 2..101 {
// 		if a[i] != 0 {
// 			println!("{}", a[i]);
// 		}
// 	}
// }
	// let mut i: usize = 0;
	// while i < 101 {
	// 	let j: i32 = i.try_into().unwrap();
	// 	a[i] = j;
	// 	print!("[{}]", a[i]);
	// 	i += 1;
	// }
	
	// for i in 1..101 {
	// 	for j in 2..i {
	// 		if i % j == 0 {
	// 			break;} /* pas un nombre premier */
				
	// 	}
	// }


// fn main() {
// 	for i in 0..51 {
// 		let b = format!("{:b}", i);
// 		if count_one(&b) {
// 			println!("{}", i);
// 		}
// 	}
// }

// fn count_one(n: &str) -> bool {
// 	let mut i:usize = 0;
// 	for c in n.chars(){
// 		if c == '1' {
// 			i += 1;
// 			}
// 		}
// 	if i % 2 == 0 {
// 		true
// 	} else {
// 		false
// 	}
// }
	

// fn main() {
// 	for i in 1..101 {
// 		for j in 1..i {
// 			if i%j == 0 {
// 				print!("{} ", j);
// 			}
// 		}
// 		println!("{}", i);
// 	}
// }

// fn main() {
// 	for i in 0..31 {
// 		println!("{}", f(i));
// 	}
// }
// fn f(n: u32) -> u32 {
// 	match n {
// 		0 => 0,
// 		1 => 1,
// 		_ => f(n - 1) + f(n - 2),
// 	}
// }

// fn main() {
// 	let mut v = vec![0, 1];

// 	while v.len() < 31 {
// 		let i = v[v.len() - 1];
// 		let j = v[v.len() - 2];
// 		v.push(i + j);
// 	}
// 	for n in v.iter() {
// 		println!("{}", n);
// 	}
// }

// fn main() {
// 	let mut i = 0;
// 	let mut j = 1;
// 	let mut k = i;
// 	for n in 2..31 {
// 		if i == 0 && j == 1 {
// 			println!("{}", i);
// 			println!("{}", j);
// 		}
// 		println!("{}", i + j);
// 		i = j;
// 		j += k;
// 		k = i;
// 	}
// }

// fn main() {
// 	for y in 1800..2401 {
// 		if y % 4 == 0 && y % 100 != 0 || y % 400 == 0 {
// 			println!("{}", y);
// 		}
// 	}
// }

// mod enums;

// enum Pieces {
// 	Un,
// 	Cinq,
// 	Dix,
// 	Vingt,
// 	Cinquante(Pays),
// }

// #[derive(Debug)]
// enum Pays {
// 	France,
// 	Espagne,
// 	Italie,
// 	Allemagne,
// }

// fn main() {
// 	let maison = enums::Couleurs::Brillant(String::from("Bleu"));
// 	maison.prnt();

// 	let x = Some(2);
// 	assert_eq!(x.is_some(), true);

// 	let y = 4;
// 	let sum = x.unwrap() + y;
// 	println!("{}", sum);

// 	valeur_en_centimes(Pieces::Cinquante(Pays::France));

// 	let five = Some(5);
// 	let six = add_one(five);
// 	let none = add_one(None);
	
// }

// fn valeur_en_centimes(piece: Pieces) -> u8 {
// 	match piece {
// 		Pieces::Un => 1,
// 		Pieces::Cinq => 5,
// 		Pieces::Dix => 10,
// 		Pieces::Vingt => 20,
// 		Pieces::Cinquante(pays) => {
// 			println!("Pièce de 50 cts venant de {:?} !", pays);
// 			50
// 		}
// 	}
// }

// fn add_one(x: Option<i32>) -> Option<i32> {
// 	match x {
// 		None => None,
// 		Some(i) => Some(i + 1),
// 	}
// }



// #[derive(Debug)]
// struct Maison {
// 	peinture: enums::Couleurs,
// 	adresse: String,
// }

// fn main() {
// 	let ma_couleur = enums::Couleurs::Vert;
// 	println!("{}", coul(ma_couleur));

// 	let corderie: Maison = Maison {
// 	peinture: enums::Couleurs::Vert,
// 	adresse: String::from("Anglet"),
// 	};
// 	println!("{:?}", corderie);
// }

// fn coul(c: enums::Couleurs) -> u8 {
// 	match c {
// 		enums::Couleurs::Vert => 1,
// 		enums::Couleurs::Bleu => 2,
// 		enums::Couleurs::Rouge => 3,
// 	}
// }



// mod structs;

// fn main() {
// 	let mon_rectangle = structs::Rectangle {
// 		longueur: 30,
// 		largeur: 5,
// 	};
// 	println!("L'aire du rectangle qui a pour longueur {} et largeur {} est : {}", mon_rectangle.longueur, mon_rectangle.largeur, mon_rectangle.aire());

// 	let mon_rectangle2 = structs::Rectangle {
// 		longueur: 14,
// 		largeur: 18,
// 	};

// 	println!("Rectangle 1 rentre dans rectangle 2 ? {}", mon_rectangle.rentre_dans(&mon_rectangle2));

// 	let mon_rectangle3 = structs::Rectangle::new_rectangle(20, 13);

// 	let moi = "Antony et Manon";
// 	let manon = &moi[10..];
// 	println!("{}", manon);
// }



// mod structs;
// fn main() {
	
// 	let nom = String::from("Merle");
// 	let prenom = String::from("Antony");
// 	let antony = structs::build_user(nom, prenom);
// 	println!("{:?}", antony);
// 	let manon = structs::Fiche {
// 		prenom: String::from("Manon"),
// 		..antony
// 	};
// 	println!("{:?}", manon);
// 	let basile = structs::build_user(String::from("Merle"), String::from("Basile"));
// 	println!("{:?}", basile);
// }

// mod slices;
// fn main() {
// 	let phrase = String::from("Antony arrive");
// 	println!("{}", slices::first_word(&phrase));
// 	let mot = slices::first_word(&phrase[..]);
// 	println!("{}", mot);
// 	println!("{}", std::i32::MAX);
// 	let mut mot = String::with_capacity(3);
// 	// mot.push_str("antony M.!y");
// 	let tableau: [char; 3] = ['a', 'b', 'c'];
// 	for lettre in tableau.iter(){
// 		mot.push(*lettre);
// 	}
// 	println!("{}", mot);
// }

// mod reference;

// fn main() {
// 	let s1 = String::from("Antony");
// 	println!("{}", reference::calculate_length(&s1));
// 	for c in s1.chars(){
// 		println!("{}", c);
// 	}
// 	let x = 2;
// 	let tuple = (s1, x);
// 	let (s2, y) = tuple;

// 	println!("{:?}, {:?}", s2, y);
	
// }

// mod ownership;

// fn main(){
// 	let moi = ownership::gives_ownership();
// 	print!("{} et ", moi);
// 	let manon = String::from("Manon");

// 	let resultat = ownership::takes_and_gives_back_ownership(manon);
// 	println!("{}", resultat);

// }

// mod push;
// fn main() {
// 	let mut phrase = String::from("Antony");
// 	// phrase.push_str(" et Manon");
// 	push::my_push_str(&mut phrase, " et Manon");
// 	println!("{}", phrase);
// }

// fn main() {
// 	let s1 = String::from("Antony");
// 	let taille = calculate_length(&s1);
// 	println!("La longueur du mot est : {}", taille);
// }

// fn calculate_length(s: &String) -> usize {
// 	s.len()
// }


// fn main () {
// 	for i in 1..101 {
// 		if i % 15 == 0 {
// 			println!("FizzBuzz");
// 		} else if i % 3 == 0 {
// 			println!("Fizz"); } 
// 			else if i % 5 == 0 {
// 			println!("Buzz");
// 		} else {
// 			println!("{}", i);
// 		}
// 	}
// }


// fn main() {
// 	let mois: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July",
// 	"August", "September", "October", "November", "December"];

// 	for month in mois.iter() {
// 		if month == &"May" {
// 			println!("C'est le printemps !");
// 		}
// 	}
// }

// Exercices chapitre 8

// use::std::collections::HashMap;

// HashMap<"dept", vecteur[nom 1, nom2, nom3]>

// fn main() {
// 	let mut i: u8 = 3;
// 	let mut people: HashMap<&str, &str> = HashMap::new();
	// let mut people: HashMap<String, String> = HashMap::new();
	
	
// 	while i > 0 {
// 	let mut user_input = String::new();
// 	std::io::stdin().read_line(&mut user_input).ok().expect("Erreur de lecture.");
// 	println!("{}", user_input);

// 	// let vecteur: Vec<&str> = user_input.split_whitespace().collect();
// 	let vecteur: Vec<String> = user_input
// 	.trim()
// 	.split_whitespace()
// 	.map(|t| t.to_string())
// 	.collect();
// 	println!("{:?}", user_input);

// 	if vecteur[0] == "Add" && vecteur[2] == "to" {
// 		let copy1: &str = &*vecteur[0];
// 		let copy2: &str = &*vecteur[2];
// 		people.insert(copy1, copy2);
// 	} else {
// 		println!("Je n'ai pas compris.");
// 	}
// 	println!("{:?}", people);
// 	user_input.clear();
// 	i += 1;
// }

// }


// fn main() {
// 	let voyelles: [char; 12] = ['A', 'a', 'E', 'e', 'I', 'i', 'O', 'o', 'U', 'u', 'Y', 'y'];

// 	let mut i: u8 = 5;
// 	while i > 0 {
// 		let mut user_input = String::new();
// 		std::io::stdin().read_line(&mut user_input).expect("Problème de mémoire.");
// 		println!("{}", pig_latin(&mut user_input, &voyelles));
// 		i -= 1;
// 	}
// 	}

// // --------------------------------------------------------------

// fn pig_latin(mot: &String, voyelles: &[char]) -> String {
// 	if tri(&mot, &voyelles) {
// 		voyelle_pig_latin(&mot)
// 	} else {
// 		consonne_pig_latin(&mot)
// 	}
// }

// // --------------------------------------------------------------

// fn tri(mot: &String, voyelles: &[char]) -> bool {
// 	for c in mot[0..1].chars(){
// 		for voyelle in voyelles{
// 			if c == *voyelle{
// 				return true
// 			}
// 		}
// 	}
// 	false
// }

// // --------------------------------------------------------------

// fn consonne_pig_latin(mot: &String) -> String {
// 	let mut c: char = ' ';
// 	for lettre in mot.chars() {
// 		c = lettre;
// 		break;
// 	}
// 	let mot_incomplet = &mot[1..].trim();
// 	let mut mot_complet: String = mot_incomplet.to_string();
// 	mot_complet.push('-');
// 	mot_complet.push(c);
// 	let suffixe: &str = "ay";
// 	mot_complet.push_str(suffixe);
// 	mot_complet
// }

// // --------------------------------------------------------------

// fn voyelle_pig_latin(mot: &String) -> String {
// 	let suffixe: &str = "-way";
// 	let mut mot_final: String = mot.trim().to_string();
// 	mot_final.push_str(suffixe);
// 	mot_final
// }


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