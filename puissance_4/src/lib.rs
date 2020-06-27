use std::io::{self, Write};
use termcolor::{StandardStream, Color, ColorChoice, ColorSpec, WriteColor};
use std::cell::Cell;                    //  enables mutation inside an immutable struct

pub enum Case {
    Pleine(Couleur),
    Vide,
}

impl Copy for Case {}
impl Clone for Case {
    fn clone(&self) -> Case {
        *self
    }
}

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
pub struct Joueur {
    couleur : Couleur,
    nbre_jetons : Cell<u8>,
}


impl Joueur {
    pub fn nouveau_joueur(coul: Couleur) -> Joueur {
        Joueur {
            couleur: coul,
            nbre_jetons: Cell::new(21),
        }
    }

    pub fn moins_un_jeton(&self) {
        self.nbre_jetons.set(self.nbre_jetons.get() - 1);
    }

}


struct Participants {
    joueur_jaune: Joueur,
    joueur_rouge: Joueur,
    compte_tour: Cell<u8>,
}


impl Participants {
    pub fn new_participants() -> Participants {
            Participants {
                joueur_jaune: Joueur::nouveau_joueur(Couleur::Jaune),
                joueur_rouge: Joueur::nouveau_joueur(Couleur::Rouge),
                compte_tour: Cell::new(0),
            }
        }
    
    pub fn inc_tour(&self) {
        self.compte_tour.set(self.compte_tour.get() + 1);
    }
}







/* 
crée joueurs
affiche tableau -- colonnes -- à qui le tour
input
put_jeton
check: gagnant -- stalemate

*/

pub fn run() {
    let mut tableau = [[Case::Vide; 7]; 6];
    let participants = Participants::new_participants();


    // iplementer function de saisie utilisateur 
    while participants.compte_tour.get() < 42 {

        if participants.compte_tour.get() % 2 == 0 {
        put_jeton(&mut tableau, saisie_colonne(), &participants.joueur_jaune);
        } else {
            put_jeton(&mut tableau, saisie_colonne(), &participants.joueur_rouge);
        }
        participants.inc_tour();
        println!("Tour N°: {:?}", participants.compte_tour);
            
        aff_tableau(&mut tableau);
        println!("{:?}", &participants.joueur_jaune.nbre_jetons);
    }
    println!("Fin de partie");
}

pub fn saisie_colonne() -> usize {
    loop {
        let mut saisie = String::new();
        io::stdin().read_line(&mut saisie)
        .expect("Problème mémoire.");
        let colonne: usize = match saisie.trim().parse() {
            Ok(chiffre) => chiffre,
            Err(_) => continue
        };

        match colonne {
            1..=7 => return colonne - 1,
            _ => {println!("Erreur : la colonne doit être comprise entre 1 et 7"); continue},
        }

    }
    

}

pub fn aff_tableau(&mut tableau: &mut[[Case;7];6]) {
    let lignes = tableau.iter();

    for ligne in lignes {
        let cases = ligne.iter();
        for case in cases {
        match case {
            Case::Vide => print!("[ ]"),
            Case::Pleine(Couleur::Jaune) => print_jeton_jaune(),
            Case::Pleine(Couleur::Rouge) => print_jeton_rouge(),
            }
        }
        println!();
    }
    println!("[1][2][3][4][5][6][7]");
}



fn write_yellow() -> io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
    write!(&mut stdout, "O")?;

    stdout.reset()
}

fn print_jeton_jaune() {
    print!("[");
    write_yellow()
    .unwrap_or_else(|err| {println!("{}", err);});
    print!("]");
}

fn write_red() -> io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
    write!(&mut stdout, "O")?;
    stdout.reset()
}

fn print_jeton_rouge() {
    print!("[");
    write_red()
    .unwrap_or_else(|err| {println!("{}", err);});
    print!("]");
}

fn check_partie_continue(joueur_jaune: &Joueur, joueur_rouge: &Joueur) -> bool {
    if joueur_jaune.nbre_jetons.get() > 1 && joueur_rouge.nbre_jetons.get() > 1 {
        true
    } else {
        false
    }
}

/// Place un jeton sur le tableau et le retranche au stock du joueur.
fn put_jeton(tableau: &mut[[Case;7]; 6], col: usize, joueur: &Joueur) {
    // let test = tableau;
    let ligne = glisse_jeton(tableau, col).expect("erreur");
    tableau[ligne][col] = Case::Pleine(joueur.couleur);

    match joueur.couleur {
        Couleur::Jaune => {joueur.moins_un_jeton();},
        Couleur::Rouge => joueur.moins_un_jeton(),
    }
}



/// vérifie si on peut mettre un jeton dans la colonne. Si c'est le cas, renvoit la position du jeton ou une erreur.
fn glisse_jeton(tableau: &[[Case;7]; 6], col: usize) -> Result<usize, i8> {
    let mut resultat: Result<usize, i8> = Err(0);
    let nbre_lignes = tableau[0].len() - 1;

    match tableau[0][col] {
        Case::Pleine(_) => return Err(-1),
        Case::Vide => (),
    }

        for i in (0..nbre_lignes).rev() {
            match tableau[i][col] {
            Case::Pleine(_) =>  (),
            Case::Vide => {resultat = Ok(i); return resultat;},
        
        }
    }
    // println!("dernier renvoi, i = {:?}", resultat);
    resultat
}

