// [USER INPUT]
	loop{
		println!("Entrez le premier nombre la séquence de Collatz");
		let mut user_input = String::new();
		io::stdin().read_line(&mut user_input)
		.expect("Problème mémoire.");
		let chiffre: u32 = match user_input.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};


// [MULTIDIMENSIONNAL ARRAYS]
for (i, row) in my_int_matrix.iter().enumerate() {
    for (j, col) in row.iter().enumerate() {
        println!("[row={}][col={}]={}", i, j, col);
    }
}

const LIGNES: usize = 9;
	const COLS: usize = 6;

	let heart: [[char; COLS]; LIGNES] = [
		['.', '.', '.', '.', '.', '.'],
        ['.', 'O', 'O', '.', '.', '.'],
        ['O', 'O', 'O', 'O', '.', '.'],
        ['O', 'O', 'O', 'O', 'O', '.'],
        ['.', 'O', 'O', 'O', 'O', 'O'],
        ['O', 'O', 'O', 'O', 'O', '.'],
        ['O', 'O', 'O', 'O', '.', '.'],
        ['.', 'O', 'O', '.', '.', '.'],
        ['.', '.', '.', '.', '.', '.'],
        ];

    for i in 0..COLS{
    	for j in 0..LIGNES{
    		print!("{}", heart[j][i]);
    	}
    	println!();
    }


// STRUCTS & TUPLES STRUCTS

* On ne peut pas faire d'itération sur des structs ou tuples, car leurs éléments ont des tailles différentes.


impl Rectangle {
    // Méthodes (prennent en argument &self)
    // S'appelle sur une instance avec un .
    fn area(&self) -> u32 {
        self.longueur * self.largeur
    }

    fn can_hold(&self, autre_rectangle: &Rectangle) -> bool {
        self.longueur > autre_rectangle.longueur && 
        self.largeur > autre_rectangle.largeur
    }

    // Fonction associée (ex : un constructeur)
    // S'appelle sur le type avec :: ( ex: let mon_carre = Rectangle::cree_carre(3); )
    fn cree_carre(cote: u32) -> Rectangle {
        Rectangle { longueur: cote, largeur: cote }
    }
}

fn main() {
    let rect1 = Rectangle { longueur: 30, largeur: 50};
    let rect2 = Rectangle { longueur: 10, largeur: 40};
    let rect3 = Rectangle { longueur: 60, largeur: 45};

    println!("Est ce que rect1 peut contenir rect2 ? \t {}", rect1.can_hold(&rect2));
    println!("Est ce que rect1 peut contenir rect3 ? \t {}", rect1.can_hold(&rect3));

    println!("L'aire du rectangle est {} pixels carrés.", rect1.area());


    // DEPENDENCIES
    /* Members of the Rust community have made many packages available at crates.io, 
    and pulling any of them into your package involves these same steps: listing them 
    in your package’s Cargo.toml file and using use to bring items into scope. 

    Note that the standard library (std) is also a crate that’s external to our package. 
    Because the standard library is shipped with the Rust language, we don’t need to 
    change Cargo.toml to include std. But we do need to refer to it with use to bring 
    items from there into our package’s scope. For example, with HashMap we would use 
    this line: */

use std::collections::HashMap;
// This is an absolute path starting with std, the name of the standard library crate.

// Using Nested Paths to Clean Up Large use Lists
use std::{cmp::Ordering, io};

use std::io;
use std::io::Write; /* est égal à */ use std::io::{self, Write};

// The Glob Operator
use std::collections::*;


// VECTORS
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("The third element is {}", third);

match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
}