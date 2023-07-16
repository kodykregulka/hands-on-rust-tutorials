use std::io::stdin;
use std::collections::HashMap;

#[derive(Debug)]
enum VisitorAction {
	Accept,
	AcceptWithNote {note: String},
	Refuse,
	Probation,
}

#[derive(Debug)]
struct Visitor {
	name: String,
	action: VisitorAction,
	age: i8,
}

impl Visitor {
	fn new(name: &str, action: VisitorAction, age: i8) -> Self {
		Self {
			name: name.to_lowercase(),
			action,
			age,
		}
	}

	fn greet(&self){
		match &self.action{
			VisitorAction::Accept=> println!("Welcome to the treehouse, {}", self.name),
			VisitorAction::AcceptWithNote { note } => {
				println!("Welcome to the treehouse, {}", self.name);
				println!("{}", note);
				if self.age < 21 {
					println!("Do not serve alcohol to {}", self.name);
				}
			},
			VisitorAction::Probation => println!("{} is now a probational member", self.name),
			VisitorAction::Refuse => println!("Do not let in {}", self.name),
		}
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
	let mut visitor_list = vec![
		Visitor::new("bert", VisitorAction::Accept, 45),
		Visitor::new("rob", VisitorAction::AcceptWithNote { note: String::from("nut allergy") }, 15),
		Visitor::new("kody", VisitorAction::Refuse, 30),
	];

	loop{
		println!("Yo, whats your name?");
		let name = what_is_your_name();
		if(name.is_empty()){
			break;
		}

		let known_visitor = visitor_list
			.iter()
			.find(|visitor| visitor.name == name);

		match known_visitor {
			Some(visitor) => visitor.greet(),
			None => {
				println!("You are not welcome into the cool kids club this time");
				visitor_list.push(Visitor::new(&name, VisitorAction::Probation, 0))
			}
		}
	}

	/*println!("Hello, {:?}", name);
	if list_contains(&visitor_list, &name) {
		println!("welcome in {}", name);
	}
	else{
		println!("You are not on the list, {}", name);
	}*/
}
