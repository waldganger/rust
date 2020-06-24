// use std::io;
// use std::io::Write;
// use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

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

pub fn run() {
    let mut tableau = [[Case::Vide; 7]; 6];
    tableau[3][4] = Case::Pleine(Couleur::Rouge);
    tableau[3][5] = Case::Pleine(Couleur::Jaune);
    put_jeton(&mut tableau, 0);
    put_jeton(&mut tableau, 0);
    put_jeton(&mut tableau, 0);
    put_jeton(&mut tableau, 0);
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
fn put_jeton(tableau: &mut[[Case;7]; 6], col: usize) {
    // let test = tableau;
    let ligne = glisse_jeton(tableau, 0).expect("erreur");
    tableau[ligne][col] = Case::Pleine(Couleur::Rouge);
}



/// vérifie si on peut mettre un jeton dans la colonne. Si c'est le cas, renvoit la position du jeton ou une erreur.
fn glisse_jeton(tableau: &[[Case;7]; 6], col: usize) -> Result<usize, i8> {
    let mut resultat: Result<usize, i8> = Err(0);
    let nbre_lignes = tableau[0].len() - 1;

    // D'abord, on vérifie qu'il y a de la place pour mettre un jeton
    // Si la colonne est pleine, on retourne une erreur à put_jeton
    // if let Case::Pleine(Couleur) = tableau[0][col] {
    //     return Err(-1);
    // }
    // if tableau[0][col] == Case::Pleine(Couleur) {
    //     return Err(-1);
    // }
    match tableau[0][col] {
        Case::Pleine(Couleur::Rouge) => {println!("pleine Rouge !"); return Err(-1)},
        Case::Pleine(Couleur::Jaune) => {println!("pleine Jaune !"); return Err(-1)},
        Case::Vide => (),
    }
    println!("exec");
    
        for i in (0..nbre_lignes).rev() {
            match tableau[i][col] {
            Case::Pleine(Couleur) => {println!("Plein, je passe"); ()},
            Case::Vide => {println!("{}", i); resultat = Ok(i); return resultat;},
        
    }

    
    }
    println!("dernier renvoi, i = {:?}", resultat);
    resultat
}

