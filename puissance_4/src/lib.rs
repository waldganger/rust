use std::io::{self, Write};
use termcolor::{StandardStream, Color, ColorChoice, ColorSpec, WriteColor};
use std::cell::Cell;                    //  enables mutation inside an immutable struct

const LIGNES: usize = 6;
const COLONNES: usize = 7;

#[derive(PartialEq)]
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

#[derive(Debug)]
#[derive(PartialEq)]
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







/* 
crée joueurs -- OK
affiche tableau -- colonnes -- à qui le tour
input -- OK
put_jeton -- OK
check: gagnant -- stalemate OK

*/

pub fn run() {
    let mut tableau = [[Case::Vide; COLONNES]; LIGNES];
    let participants = Participants::new_participants();


    // Le compte-tour peut aller plus haut que 42, en cas d'erreur
    // while participants.joueur_jaune.nbre_jetons.get() != 0
    // && participants.joueur_rouge.nbre_jetons.get() != 0 {
    loop {
        if participants.compte_tour.get() % 2 == 0 {
        put_jeton(&mut tableau, saisie_colonne(), &participants.joueur_jaune);
        } else {
            put_jeton(&mut tableau, saisie_colonne(), &participants.joueur_rouge);
        }
        participants.inc_tour();
        println!("Tour N°: {:?}", participants.compte_tour);
            
        aff_tableau(&mut tableau);
        println!("Joueur jaune a {:?} jetons", &participants.joueur_jaune.nbre_jetons);
        println!("Joueur rouge a {:?} jetons", &participants.joueur_rouge.nbre_jetons);
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


fn check_partie_continue(joueur_jaune: &Joueur, joueur_rouge: &Joueur) -> bool {
    if joueur_jaune.nbre_jetons.get() > 1 && joueur_rouge.nbre_jetons.get() > 1 {
        true
    } else {
        false
    }
}

/// Place un jeton sur le tableau et le retranche au stock du joueur.
fn put_jeton(tableau: &mut[[Case;COLONNES]; LIGNES], col: usize, joueur: &Joueur) -> u8 {
    // let test = tableau;

    let ligne = match glisse_jeton(tableau, col) {
        Ok(colonne) => colonne,
        Err(erreur) => {
            println!("{}", erreur);
            return 0},
    };


    // let ligne = glisse_jeton(tableau, col).unwrap_or_else(|erreur| {
    //     eprintln!("Erreur : {}", erreur);
    //     process::exit(1);
    // });
    // match ligne {
    //     8 => false,
    //     _ => (),
    // }
    
    tableau[ligne][col] = Case::Pleine(joueur.couleur);
    let couleur_du_joueur = joueur.couleur;
    if check_horizontal(tableau, &couleur_du_joueur) {
        println!("VICTOIRE DU JOUEUR {:?}", joueur.couleur);
        std::process::exit(0);
    }

    if check_vertical(tableau, &couleur_du_joueur, col) {
        println!("VICTOIRE DU JOUEUR {:?}", joueur.couleur);
        std::process::exit(0);
    }

    if check_diagonal_top_left_bottom_right(tableau, &couleur_du_joueur) {
        println!("VICTOIRE DU JOUEUR {:?}", joueur.couleur);
        std::process::exit(0);
    }

    if check_diagonal_bottom_left_top_right(tableau, &couleur_du_joueur) {
        println!("VICTOIRE DU JOUEUR {:?}", joueur.couleur);
        std::process::exit(0);
    }

    /*
    ICI INSERER FONCTION QUI VERIFIE SI 4 JETONS DE MEME COULEUR SONT ALIGNES
    - test sur le dernier jeton
    Si nombre de jeton < 19, on commence le test
    Pour tester :
    si x >= 3 
        x-1 y
        x-2 y
        ...
    Pas de limite de test pour y car o
        xy-1
        ...
    Test de la couleur du joueur uniquement
    HORIZONTAL
    Pour chaque ligne 0..6
        si case mm couleur
            alors + 1
        si case pleine ou autre couleur
            alors compteur à 0
        si compteur = 4
            WIN

    VERTICAL
    Pour chaque colonne (0..5).rev()        // en sens inverse, car on remonte
        si case vide                        // case vide == que des cases vides après.
            break
        si case mm couleur
            alors compteur + 1
        si case vide ou autre couleur
            alors compteur à 0

    DIAGONALE HAUT-GAUCHE >> BAS-DROIT
    lignes--
    colonnes++
    range départ : [2][0] -- [2][6]
                    |           |
                    V           V
    range arrivée: [0][3] -- [6][3]


    check_diagonal_top_left_bottom_right

    [5][0]

    [4][0]
    [5][1]

    [3][0]
    [4][1]
    [5][2]

    [2][0]
    [3][1]
    [4][2]
    [5][3]

    [1][0]
    [2][1]
    [3][2]
    [4][3]
    [5][4]

    [0][0]
    [1][1]
    [2][2]
    [3][3]
    [4][4]
    [5][5]


    check_diagonal_bottom_left_top_right
    lignes++
    col++ (départ à un)
    [0][1]
    [1][2]
    [2][3]
    [3][4]
    [4][5]
    [5][6]

    [0][2]
    [1][3]
    [2][4]
    [3][5]
    [4][6]
    
    etc

    DIAGONALE HAUT-DROIT << BAS-GAUCHE
    lignes++
    colonnes--
    range départ : [0][3] -- [2][6]
                    |           |
                    V           V
    range arrivée: [3][0] -- [6][3]

    */

    match joueur.couleur {
        Couleur::Jaune => {joueur.moins_un_jeton();},
        Couleur::Rouge => joueur.moins_un_jeton(),
    }
    1
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
    // println!("dernier renvoi, i = {:?}", resultat);
    resultat
}

fn check_horizontal(tableau: &mut[[Case; COLONNES]; LIGNES], jeton: &Couleur) -> bool {

    for ligne in tableau.iter(){
        let mut compteur: u8 = 0;
        for case in ligne.iter(){
            match case {
                Case::Pleine(Couleur::Jaune) => {
                    match jeton {
                        Couleur::Jaune => compteur +=1,
                        _ => compteur = 0,
                    }
                }
                Case::Pleine(Couleur::Rouge) => {
                    match jeton {
                        Couleur::Rouge => compteur += 1,
                        _ => compteur = 0,
                    }
                }
                _ => compteur = 0,
            }
            if let 4 = compteur {
                return true;
            }
        }
    }
    false
}

fn check_vertical(tableau: &mut [[Case; COLONNES];LIGNES], jeton: &Couleur, colonne: usize) -> bool { 
    let mut compteur: u8 = 0;
    for ligne in (0..LIGNES).rev() {
        match tableau[ligne][colonne] {
            Case::Pleine(Couleur::Jaune) => {
                match jeton {
                    Couleur::Jaune => compteur += 1,
                    Couleur::Rouge => compteur = 0,
                }
            }
            Case::Pleine(Couleur::Rouge) => {
                match jeton {
                    Couleur::Rouge => compteur += 1,
                    Couleur::Jaune => compteur = 0,
                }
            }
            Case::Vide => compteur = 0,
        }
        if let 4 = compteur {
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
        

        match tableau[j][k] {
                Case::Pleine(Couleur::Jaune) => {
                    match jeton {
                        Couleur::Jaune => compteur +=1,
                        _ => compteur = 0,
                    }
                }
                Case::Pleine(Couleur::Rouge) => {
                    match jeton {
                        Couleur::Rouge => compteur += 1,
                        _ => compteur = 0,
                    }
                }
                _ => compteur = 0,
            }
            if let 4 = compteur {
                return true;
            }

        }
        // println!();
    }
    false
}

fn check_diagonal_bottom_left_top_right
(tableau: &mut [[Case; COLONNES]; LIGNES], jeton: &Couleur) -> bool {
    for i in 1..COLONNES {
        let mut compteur: u8 = 0;
        for (j, k) in (0..COLONNES - i + 1).zip(i..COLONNES){
        

        match tableau[j][k] {
                Case::Pleine(Couleur::Jaune) => {
                    match jeton {
                        Couleur::Jaune => compteur +=1,
                        _ => compteur = 0,
                    }
                }
                Case::Pleine(Couleur::Rouge) => {
                    match jeton {
                        Couleur::Rouge => compteur += 1,
                        _ => compteur = 0,
                    }
                }
                _ => compteur = 0,
            }
            if let 4 = compteur {
                return true;
            }

        }
    }
    false
}

