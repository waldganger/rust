// #[derive(Debug)]
// pub enum Couleurs {
//     Bleu,
//     Vert,
//     Rouge,
// }



pub enum Couleurs {
    Mat(String),
    Brillant(String),
}

impl Couleurs {
    pub fn prnt(&self) {
        match self {
            Couleurs::Mat(_) => println!("C'est mat !"),
            Couleurs::Brillant(_) => println!("C'est brillant"),
        }
    }
}