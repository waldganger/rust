pub fn gives_ownership() -> String {
    let some_string = String::from("Antony");
    some_string
}

pub fn takes_and_gives_back_ownership(s: String) -> String {
    s
}