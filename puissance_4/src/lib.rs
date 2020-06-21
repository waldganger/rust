mod jeu {
    const COLONNES: usize = 7;
    const LIGNES: usize = 6;

// #[derive(Copy, Clone)]
    pub enum Couleur {
        Jaune,
        Rouge,
    }

    impl Copy for Couleur { }
    impl Clone for Couleur {
        fn clone(&self) -> Couleur {
            *self
        }
    }

    
    pub struct PlateauDeJeu {
        cases: usize,
        plateau: [[Option<Couleur>; COLONNES]; LIGNES],
    }

    fn build_plateau() -> PlateauDeJeu {

        PlateauDeJeu {
            cases: COLONNES * LIGNES,
            plateau: [[None; COLONNES]; LIGNES],
        }
    }

    pub struct Joueur {
        couleur: Couleur,
        nombre_jetons: u8,
    }


}

pub fn run() {

}