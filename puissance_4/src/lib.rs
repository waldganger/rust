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
    
}

pub fn aff_tableau(&mut tableau: &mut[[Case;7];6]) {
    let lignes = tableau.iter();

    for ligne in lignes {
        let cases = ligne.iter();
        for case in cases {
        match case {
            Case::Vide => print!("Vide "),
            Case::Pleine(Couleur::Jaune) => write_yellow()
            .unwrap_or_else(|err| {println!("{}", err);
                }),
            Case::Pleine(Couleur::Rouge) => write_red()
            .unwrap_or_else(|err| {println!("{}", err)}),
        }
        }
        println!();
    }
}


fn write_yellow() -> io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
    write!(&mut stdout, "jaune")?;
    stdout.reset()
}

fn write_red() -> io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
    write!(&mut stdout, "rouge")?;
    stdout.reset()
}

// fn reset_stdout() -> io::Result<()> {
//     let mut stdout = StandardStream::stdout(ColorChoice::Always);
//     stdout.reset()
// }

// mod jeu {
//     const COLONNES: usize = 7;
//     const LIGNES: usize = 6;

// // #[derive(Copy, Clone)]
// #[derive(Debug)]
//     pub enum Case {
//         Vide,
//         Pleine(Couleur),
//     }
// #[derive(Debug)]
//     pub enum Couleur {
//         Jaune,
//         Rouge,
//     }

//     impl Copy for Case { }
//     impl Clone for Case {
//         fn clone(&self) -> Case {
//             *self
//         }
//     }

//      impl Copy for Couleur { }
//     impl Clone for Couleur {
//         fn clone(&self) -> Couleur {
//             *self
//         }
//     }

//     use std::fmt;
//     impl fmt::Display for Case {
//         fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//             match *self {
//                 Vide => write!(f, ""),
//                 Case::Pleine(Couleur::Jaune) => write!(f, "Jaune"),
//                 Case::Pleine(Couleur::Rouge) => write!(f, "Rouge"),
//             }
//         }
//     }

    
//     // pub struct PlateauDeJeu {
//     //     nombre_cases: usize,
//     //     pub plateau: [[Couleur; COLONNES]; LIGNES],
//     // }

//     // pub fn build_plateau() -> PlateauDeJeu {

//     //     PlateauDeJeu {
//     //         nombre_cases: COLONNES * LIGNES,
//     //         plateau: [[Couleur::Jaune; COLONNES]; LIGNES],
//     //     }
//     // }

//     pub struct Joueur {
//         couleur: Case,
//         nombre_jetons: u8,
//     }


// }

// pub fn run() {
//     // let mut board = jeu::build_plateau();
//     // let plateau = board.plateau;

//     // let cases = plateau.iter();
//     // for case in cases {
//     //     println!("{}", case);
//     const COLONNES: usize = 7;
//     const LIGNES: usize = 6;
 
//     let mut board = [[jeu::Case::Pleine(jeu::Couleur::Jaune); COLONNES]; LIGNES];

//     let cases = board.iter();
//     for case in cases{
//         println!("{:?}", case);
//     }
//     println!("{}", jeu::Case::Pleine(jeu::Couleur::Rouge));
//     }
