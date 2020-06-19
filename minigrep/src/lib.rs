use std::error::Error;
use std::fs;
// use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: std::env::Args) -> Result<Config, &'static str> {
        
        let mut args = args.peekable();
        args.next();
                
        if args.peek() == Some(&String::from("-i")) {
            
            let case_sensitive = false;
            args.next();

            let query = match args.next() {
                Some(arg) => arg,
                None => return Err("Requête absente")
            };

            let filename = match args.next() {
                Some(arg) => arg,
                None => return Err("Fichier non spécifié"),
            };

            return Ok(Config {
            query,
            filename,
            case_sensitive,
        })

        } else {
            println!("bon");
            let query = match args.next() {
                Some(arg) => arg,
                None => return Err("Requête absente")
            };

            let filename = match args.next() {
                Some(arg) => arg,
                None => return Err("Fichier non spécifié"),
            };

            // let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
            let case_sensitive = true;

            Ok(Config {
            query,
            filename,
            case_sensitive,
            })
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        println!("Case insensitive active");
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    contents.lines()
    .filter(|line| line.contains(query))
    .collect()

}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    let query = query.to_lowercase();
    contents.lines()
    .filter(|line| line.to_lowercase().contains(&query))
    .collect()

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
        Rust:
safe, fast, productive.
Pick three.
Duct tape.";
    

    assert_eq!(vec!["safe, fast, productive."],
    search(query, contents)
);
}

#[test]
fn case_insensitive() {
    let query = "rUST";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

//     let contents = "
// Rust:
// safe, fast, productive.
// Pick three.
// Duct tape.";

    assert_eq!(vec!["Rust:", "Trust me."],
    search_case_insensitive(query, contents)
);
}

}