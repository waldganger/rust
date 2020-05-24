pub trait Summary {
	fn summarize_author(&self) -> String;

	fn summarize(&self) -> String {
		format!("Read more from {}", self.summarize_author())
	}
}

pub struct NewsArticle {
	pub headline: String,
	pub location: String,
	pub author: String,
	pub content: String,
}

pub struct Tweet {
	pub username: String,
	pub content: String,
	pub reply: bool,
	pub retweet: bool,
}

// impl Summary for NewsArticle {
// 	fn summarize(&self) -> String {
// 		format!("{} by {}. ({})", self.headline, self.author, self.location)
// 	}
// }

impl Summary for NewsArticle {
	fn summarize_author(&self) -> String {
		format!("{}", self.author)
	}
}

impl Summary for Tweet {
	fn summarize_author(&self) -> String {
		format!("@{}", self.username)
	}

	// fn summarize(&self) -> String {
	// 	format!("{} : {}", self.username, self.content)
	// }
}


fn main() {
let tweet = Tweet {
	username: String::from("antony"),
	content: String::from("J'apprends Rust depuis avril, c'est très intéressant."),
	reply: false,
	retweet: true,
};

println!("One new tweet : {}", tweet.summarize());

let article = NewsArticle {
	headline: String::from("La fête du miassou bat son plein à Fillou-les-Rillettes !"),
	location: String::from("Fillou-les-Rillettes"),
	author: String::from("Anne-Sophie Lapix"),
	content: String::from("La 28ème fête du miassou a battu des records de fréquentation cette année, amenant des millers de personnes dans le petit village de Fillou-les-Rillettes"),
};
println!("{}", article.summarize());
}