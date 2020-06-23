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
fn put_jeton(&mut tableau: &mut[[Case;7]; 6], col: usize) {

}

fn glisse_jeton(&mut tableau: &mut[[Case;7]; 6], col: usize) -> (usize, usize) {
    let mut resultat: (usize, usize) = (0, 0);
    let nbre_lignes = tableau[0].len();
    for i in 0..nbre_lignes {
        match tableau[i][col] {
            Case::Pleine(Couleur) => {resultat = (i - 1, col)},
            Case::Vide => {resultat = (4, 4)},
        }
    }
    resultat
}


            // match tableau[i][col] {
            // Case::Pleine(Couleur) => return (i, col),
            // Case::Vide => return (i, col),