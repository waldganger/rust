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