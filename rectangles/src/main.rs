#[derive(Debug)]
struct Rectangle {
	longueur: u32,
	largeur: u32,
}

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
}

// fn area(rectangle: &Rectangle) -> u32 {
// 	rectangle.longueur * rectangle.largeur
// }

// fn main() {
//     let rect1 = (30, 50);

//     println!("L'aire du rectangle est {} pixels carrés.", area(rect1));
// }

// fn area(dimensions: (u32, u32)) -> u32 {
// 	dimensions.0 * dimensions.1
// }