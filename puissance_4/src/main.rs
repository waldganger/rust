mod lib;

const LIGNES: u8 = 6;
//const COLONNES: u8 = 7;

fn main() {
    

    for i in (0..LIGNES).rev() {
        for (j, k) in (i..LIGNES).zip(0..LIGNES - i + 1){
        println!("{}\t{}", j, k);
        }
        println!();
    }
    lib::run();
}
