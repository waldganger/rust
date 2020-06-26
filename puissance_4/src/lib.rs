use std::io::{self, Write};
use termcolor::{StandardStream, Color, ColorChoice, ColorSpec, WriteColor};

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
#[derive(Clone, Copy)]
pub struct Joueur {
    couleur : Couleur,
    coups_restants : u8,
}

pub fn nouveau_joueur(coul: Couleur) -> Joueur {
    Joueur {
        couleur: coul,
        coups_restants: 21,
    }
}
#[derive(Clone, Copy)]
struct Participants {
    joueur_jaune: Joueur,
    joueur_rouge: Joueur,
}

fn new_participants() -> Participants {
    Participants {
        joueur_jaune: nouveau_joueur(Couleur::Jaune),
        joueur_rouge: nouveau_joueur(Couleur::Rouge),
    }
}



pub fn run() {
    let mut tableau = [[Case::Vide; 7]; 6];
    let mut participants = new_participants();
    // let mut participants: [Joueur; 2] = [nouveau_joueur(Couleur::Jaune), nouveau_joueur(Couleur::Rouge)];
    // tableau[3][4] = Case::Pleine(Couleur::Rouge);
    // tableau[3][5] = Case::Pleine(Couleur::Jaune);
    // let mut joueur_jaune = nouveau_joueur(Couleur::Jaune);
    put_jeton(&mut tableau, 0, participants.joueur_jaune);
    put_jeton(&mut tableau, 0, participants.joueur_rouge);
    put_jeton(&mut tableau, 0, participants.joueur_jaune);
    put_jeton(&mut tableau, 0, participants.joueur_rouge);
    put_jeton(&mut tableau, 0, participants.joueur_jaune);
    put_jeton(&mut tableau, 0, participants.joueur_rouge);
    put_jeton(&mut tableau, 1, participants.joueur_jaune);
    put_jeton(&mut tableau, 6, participants.joueur_rouge);
    
    aff_tableau(&mut tableau);
    println!("{}", tableau.len());
    
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

// penser à ajouter player en arg, pour décompter le nombre de jetons
/// Place un jeton sur le tableau et le retranche au stock du joueur.
fn put_jeton(tableau: &mut[[Case;7]; 6], col: usize, joueur: Joueur) {
    // let test = tableau;
    let ligne = glisse_jeton(tableau, col).expect("erreur");
    tableau[ligne][col] = Case::Pleine(joueur.couleur);
    // match coul {
    //     Couleur::Jaune => parti
    // }
}



/// vérifie si on peut mettre un jeton dans la colonne. Si c'est le cas, renvoit la position du jeton ou une erreur.
fn glisse_jeton(tableau: &[[Case;7]; 6], col: usize) -> Result<usize, i8> {
    let mut resultat: Result<usize, i8> = Err(0);
    let nbre_lignes = tableau[0].len() - 1;

    match tableau[0][col] {
        Case::Pleine(_) => {println!("pleine Rouge !"); return Err(-1)},
        Case::Vide => (),
    }

        for i in (0..nbre_lignes).rev() {
            match tableau[i][col] {
            Case::Pleine(_) => {println!("Plein, je passe"); ()},
            Case::Vide => {println!("{}", i); resultat = Ok(i); return resultat;},
        
        }
    }
    println!("dernier renvoi, i = {:?}", resultat);
    resultat
}

