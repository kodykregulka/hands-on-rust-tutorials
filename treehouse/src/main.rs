use std::io::stdin;
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Visitor {
	name: String,
	greeting: String,
}

impl Visitor {
	fn new(name: &str, greeting: &str) -> Visitor {
		Visitor {name: name.to_string(), greeting: greeting.to_string()}
	}

	fn greet(&self) {
		println!("{}", self.greeting);
	}
}



fn what_is_your_name() -> String{
	let mut your_name = String::new();

	stdin()
		.read_line(&mut your_name)
		.expect("failed to read line");

	your_name
		.trim()
		.to_lowercase()
}


fn list_contains(list:&[Visitor], name: &String) -> bool{
	for visitor in list {
		if &visitor.name == name {
			return true;
		}
	}
	return false;
}

fn main() {
	//let visitor_list = ["bert", "steve", "fred"];
	/*let visitor_list = HashMap::from([
		("kody", Visitor::new("kody", "yo yo yo")),
	]);*/
	let visitor_list = [
		Visitor::new("bert", "hi"),
		Visitor::new("rob", "come in"),
		Visitor::new("kody", "Welcome"),
	];

	println!("Yo, whats your name?");
	let name = what_is_your_name();

	let known_visitor = visitor_list
		.iter()
		.find(|visitor| visitor.name == name);

	match known_visitor {
		Some(visitor) => visitor.greet(),
		None => println!("You are not welcome into the cool kids club")
	}

	/*println!("Hello, {:?}", name);
	if list_contains(&visitor_list, &name) {
		println!("welcome in {}", name);
	}
	else{
		println!("You are not on the list, {}", name);
	}*/
}
