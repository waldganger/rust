fn main() {
	loop{
	let mut user_input = String::new();
	std::io::stdin().read_line(&mut user_input).expect("FAILURE");

	println!("{}", romanize(user_input.trim().parse().unwrap()));
	}
}

fn romanize(c: u32) -> String {
	let mut result = String::new();
	let mut x = c;
	while x > 0 { // on enlève M par M, D puis, C, puis L, X par X,puis V, puis I par I
		if x >= 1000 {
			result.push('M');
			x -= 1000;
		} else if x >= 900 {
			result.push_str("CM");
			x %= 900;
		}
		else if x >= 500 {
			result.push('D');
			x -= 500;
		} else if x >= 400 {
			result.push_str("CD");
			x %= 400;
		}
		else if x >= 100 {
			result.push('C');
			x -= 100;
		} else if x >= 90 {
			result.push_str("XC");
			x %= 90;
		}
		else if x >= 50 {
			result.push('L');
			x -= 50;
		} else if x > 39 {
			result.push_str("XL");
			x %= 40;
		}
		else if x >= 10 {
			result.push('X');
			x -= 10;
		} else if x > 8 {
			result.push_str("IX");
			x %= 9;
		}
		else if x >= 5 {
			result.push('V');
			x -= 5;
		} else if x > 3 {
			result.push_str("IV");
			x %= 4;
		}
		else {
			result.push('I');
			x -= 1;
		}
	}
	result
}