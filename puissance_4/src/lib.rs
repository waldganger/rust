use std::io::{self, Write};
use termcolor::{StandardStream, Color, ColorChoice, ColorSpec, WriteColor};
use std::cell::Cell;                    //  enables mutation inside an immutable struct
use std::thread;
use std::time::Duration;

const LIGNES: usize = 6;
const COLONNES: usize = 7;

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
            nbre_jetons: Cell::new(21),       // Plus de jetons == fin de partie.
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
                compte_tour: Cell::new(0),      // Sert uniquement à déterminer à qui le tour
            }
        }
    
    pub fn inc_tour(&self) {
        self.compte_tour.set(self.compte_tour.get() + 1);  
    }
}

pub fn run() {
    let mut tableau = [[Case::Vide; COLONNES]; LIGNES];
    let participants = Participants::new_participants();

    loop {
        cls();
        println!("Tour {} : joueur {}", &participants.compte_tour.get() + 1, jaune_ou_rouge(&participants));
        aff_tableau(&mut tableau);
        // bloc de gestion du tour. A partir du compte tour, on détermine quel joueur doit agir.
        // en cas de colonne pleine, on reste au même tour.
        if &participants.compte_tour.get() % 2 == 0 {
            match put_jeton(&mut tableau, saisie_colonne(), &participants.joueur_jaune) {
                1 => (),
                _ => participants.inc_tour(),
            }
        } else {
            match put_jeton(&mut tableau, saisie_colonne(), &participants.joueur_rouge) {
                1 => (),
                _ => participants.inc_tour(),
            }
        }

        cls();

        if participants.joueur_jaune.nbre_jetons.get() == 0 && participants.joueur_rouge.nbre_jetons.get() == 0 {
            println!("Fin de partie");
            break;
        }
    }
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
            1..=COLONNES => return colonne - 1,
            _ => {println!("Erreur : la colonne doit être comprise entre 1 et 7"); continue},
        }

    }
}

pub fn aff_tableau(&mut tableau: &mut[[Case;COLONNES];LIGNES]) {
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

/// Place un jeton sur le tableau et le retranche au stock du joueur.
fn put_jeton(tableau: &mut[[Case;COLONNES]; LIGNES], col: usize, joueur: &Joueur) -> u8 {
    // let test = tableau;

    let ligne = match glisse_jeton(tableau, col) {
        Ok(colonne) => colonne,
        Err(erreur) => {
            println!("{}", erreur);
            thread::sleep(Duration::from_secs(2));
            return 1},
    };
    
    tableau[ligne][col] = Case::Pleine(joueur.couleur);
    let couleur_du_joueur = joueur.couleur;

    check_etoile(tableau, &couleur_du_joueur, joueur, col);

    match joueur.couleur {
        Couleur::Jaune => {joueur.moins_un_jeton();},
        Couleur::Rouge => joueur.moins_un_jeton(),
    }
    0
}

/// vérifie si on peut mettre un jeton dans la colonne. Si c'est le cas, renvoit la position du jeton ou une erreur.
fn glisse_jeton(tableau: &[[Case;COLONNES]; LIGNES], col: usize) -> Result<usize, &'static str> {
    let mut resultat: Result<usize, &'static str> = Err("Bug fn glisse_jeton : variable résultat initialisée, mais non changée");
    let nbre_lignes = tableau[0].len() - 1;

    match tableau[0][col] {
        Case::Pleine(_) => return Err("Colonne pleine !"),
        Case::Vide => (),
    }

    for i in (0..nbre_lignes).rev() {
        match tableau[i][col] {
        Case::Pleine(_) =>  (),
        Case::Vide => {resultat = Ok(i); return resultat;},
        }
    }
    resultat
}

fn check_horizontal(tableau: &mut[[Case; COLONNES]; LIGNES], jeton: &Couleur) -> bool {
    for i in 0..LIGNES {
        let mut compteur: u8 = 0;
        for j in 0..COLONNES {
            match_case(tableau, jeton, i, j, &mut compteur);
            if compteur == 4 {
                return true;
            }   
        }
    }
    false
}

fn check_vertical(tableau: &mut [[Case; COLONNES];LIGNES], jeton: &Couleur, colonne: usize) -> bool { 
    let mut compteur: u8 = 0;
    for ligne in (0..LIGNES).rev() {
        match_case(tableau, jeton, ligne, colonne, &mut compteur);
        if compteur == 4 {
            return true;
        }
    }
    false
}

fn check_diagonal_top_left_bottom_right
(tableau: &mut [[Case; COLONNES]; LIGNES], jeton: &Couleur) -> bool {
    for i in (0..LIGNES).rev() {
        let mut compteur: u8 = 0;
        for (j, k) in (i..LIGNES).zip(0..LIGNES - i + 1){
            match_case(tableau, jeton, j, k, &mut compteur);     
            if compteur == 4 {
                return true;
            }
        }
    }
    false
}

fn check_diagonal_top_left_bottom_right_2
(tableau: &mut [[Case; COLONNES]; LIGNES], jeton: &Couleur) -> bool {
    for i in 1..COLONNES {
        let mut compteur: u8 = 0;
        for (j, k) in (0..COLONNES - i + 1).zip(i..COLONNES){
            match_case(tableau, jeton, j, k, &mut compteur);
            if compteur == 4 {
                return true;
            }
        }
    }
    false
}

fn check_diagonal_top_right_bottom_left
(tableau: &mut [[Case; COLONNES]; LIGNES], jeton: &Couleur) -> bool {
    for i in (0..LIGNES).rev() {
        let mut compteur: u8 = 0;
        for (j, k) in (i..LIGNES).zip((0..COLONNES).rev()){
            match_case(tableau, jeton, j, k, &mut compteur);
            if compteur == 4 {
                return true;
            }
        }
    }
    false
}

fn check_diagonal_top_right_bottom_left_2
(tableau: &mut [[Case; COLONNES]; LIGNES], jeton: &Couleur) -> bool {
    for i in (0..LIGNES).rev() {
        let mut compteur: u8 = 0;
        for (j, k) in (0..i + 1).rev().zip(0..COLONNES){
            match_case(tableau, jeton, j, k, &mut compteur); 
            if compteur == 4 {
                return true;
            }
        }
    }
    false
}

fn cls() {
    for _i in 0..41 {
        println!();
    }
}

fn jaune_ou_rouge(participants: &Participants) -> String {
    if participants.compte_tour.get() % 2 == 0 {
        return String::from("jaune")
    } else {
        String::from("rouge")
    }
}

/// match_case : vérifie que le jeton dans la case du tableau reçu en argument est de la couleur du joueur
/// si oui, incrémente et renvoit le compteur à la fonction de vérification d'alignement.
fn match_case(tableau: &mut [[Case; COLONNES];LIGNES], jeton: &Couleur, 
    ligne: usize, col:usize, compteur: &mut u8) {
    match tableau[ligne][col] {
        Case::Pleine(Couleur::Jaune) => {
                match jeton {
                    Couleur::Jaune => *compteur +=1,
                    _ => *compteur = 0,
                }
            }
        Case::Pleine(Couleur::Rouge) => {
            match jeton {
                Couleur::Rouge => *compteur += 1,
                _ => *compteur = 0,
            }
        }
        _ => *compteur = 0,
    }
}

/// Vérifie les cases adjacentes en étoile pour vérifier si quatre jetons sont identiques.
fn check_etoile
(tableau: &mut [[Case; 7]; 6], jeton: &Couleur, joueur: &Joueur, colonne: usize) {

    if check_horizontal(tableau, jeton)
    || check_vertical(tableau, jeton, colonne)
    || check_diagonal_top_left_bottom_right(tableau, jeton)
    || check_diagonal_top_left_bottom_right_2(tableau, jeton)
    || check_diagonal_top_right_bottom_left(tableau, jeton)
    || check_diagonal_top_right_bottom_left_2(tableau, jeton) {
        end_game(tableau, joueur);
    };
}

/// Affiche le tableau et quitte proprement avec std::process::exit(0)
fn end_game(tableau: &mut [[Case; 7]; 6], joueur: &Joueur) {
    aff_tableau(tableau);
    println!("Victoire du joueur {} !", match joueur.couleur {
        Couleur::Jaune => "jaune",
        Couleur::Rouge => "rouge"
    });
    thread::sleep(Duration::from_secs(2));
    std::process::exit(0);
}