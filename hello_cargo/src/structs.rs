pub struct Rectangle {
    pub longueur: u32,
    pub largeur: u32,
}

impl Rectangle {
    pub fn aire(&self) -> u32 {
        self.longueur * self.largeur
    }

    pub fn rentre_dans(&self, autre: &Rectangle) -> bool {
        self.longueur < autre.longueur && self.largeur < autre.largeur
    }

    pub fn new_rectangle(lon: u32, lar: u32) -> Rectangle {
        Rectangle {
            longueur: lon,
            largeur: lar,
        }
    }
}

// static mut COMPTEUR: u32 = 5;

// #[derive(Debug)]
// pub struct Fiche {
//     pub nom: String,
//     pub prenom: String,
//     pub pers_id: u32,
//     pub active: bool,
// }

// pub fn build_user (nom: String, prenom: String) -> Fiche {
//     Fiche {
//     nom,
//     prenom,
//     pers_id: unsafe {incremente(&mut COMPTEUR)}, 
//     active: true,
//     }
    
// }

// pub fn incremente(c: &mut u32) -> u32 {
//     *c += 1;
//     *c
// }
