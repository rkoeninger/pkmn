#![allow(dead_code)]
#![allow(unused_variables)]

use std::fmt;
#[macro_use] extern crate text_io;

enum Element {
	Normal,
	Flying,
	Grass,
	Bug,
	Poison,
	Fire,
	Rock,
	Ground,
	Fighting,
	Water,
	Ice,
	Electric,
	Psychic,
	Ghost,
	Dragon
}

// TODO: have a separate CombatStatus for status's that only apply during combat
enum Status {
	Poisoned,
	Paralyzed,
	Asleep,
	Confused,
	Burned,
	Frozen
}

enum TransformMode {
	ByLevel(u32),
	ByStone(StoneType),
	ByTrade
}

enum StoneType {
	FireStone,
	WaterStone,
	ThunderStone,
	LeafStone,
	MoonStone
}

struct Move {
	name: &'static str,
	type1: Element
	//ppMax:    u32
	//accuracy: u32 // 0 - 100
	//power:    u32
	//effect: EffectType
	//effectPropibility: u32 // 0 - 100
}

struct Transform<'a> {
	mode: TransformMode,
	into: &'a Species<'a>
}

struct Species<'a> {
	index: u32,
	name: &'static str,
	type1: Element,
	type2: Option<Element>,
	transforms: Vec<Transform<'a>>,
	schedule: Vec<(u32, Move)>
}

impl<'a> fmt::Display for Species<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let _ = writeln!(f, "#{}", self.index);
		let _ = writeln!(f, "{}", self.name);
		let _ = write!(f, "{}", self.type1);
		let _ = match self.type2 {
			Some(ref t) => writeln!(f, "/{}", t),
			None => writeln!(f, "")
		};
		for trans in self.transforms.iter() {
			let _ = write!(f, "transforms by ");
			let _ = match trans.mode {
				TransformMode::ByLevel(ref x) => write!(f, "level {} ", x),
				TransformMode::ByStone(ref x) => write!(f, "{} ", x),
				TransformMode::ByTrade => write!(f, "trade ")
			};
			let _ = writeln!(f, "into {}", trans.into.name);
		}
		write!(f, "")
	}
}

impl fmt::Display for Element {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let s = match *self {
			Element::Normal => "Normal",
			Element::Flying => "Flying",
			Element::Grass => "Grass",
			Element::Bug => "Bug",
			Element::Poison => "Poison",
			Element::Fire => "Fire",
			Element::Rock => "Rock",
			Element::Ground => "Ground",
			Element::Fighting => "Fighting",
			Element::Water => "Water",
			Element::Ice => "Ice",
			Element::Electric => "Electric",
			Element::Psychic => "Psychic",
			Element::Ghost => "Ghost",
			Element::Dragon => "Dragon"
		};
		write!(f, "{}", s)
	}
}

impl fmt::Display for StoneType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let s = match *self {
			StoneType::FireStone => "Fire Stone",
			StoneType::WaterStone => "Water Stone",
			StoneType::ThunderStone => "Thunder Stone",
			StoneType::LeafStone => "Leaf Stone",
			StoneType::MoonStone => "Moon Stone"
		};
		write!(f, "{}", s)
	}
}

struct Specimen<'a> {
	name: Option<&'static str>,
	species: &'a Species<'a>,
	level: u32,
	status: Option<Status>,
	max_hp: u32,
	hp: u32,
	exp: u32,
	defense: u32,
	attack: u32
}

struct Thing {
	name: &'static str,
	info: &'static str
}

struct Room {
	name: &'static str,
	paths: Vec<Box<Room>>,
	things: Vec<Thing>
}

enum ParseResult {
	Quit,
	Fail,
	Go(String),
	Look(String),
}

fn parse_input(mut line: String) -> ParseResult {
	if line == "quit".to_string() {
		ParseResult::Quit
	} else if line.starts_with("go ") {
		line = line["go ".len()..].to_string();

		if line.starts_with("to ") {
			line = line["to ".len()..].to_string();
		}

		ParseResult::Go(line.to_string())
	} else if line.starts_with("look ") {
		line = line["look ".len()..].to_string();

		if line.starts_with("at ") {
			line = line["at ".len()..].to_string();
		}

		ParseResult::Look(line.to_string())
	} else {
		ParseResult::Fail
	}
}

fn explore(current: &Room) {
	println!("You awake in {}.", current.name);
	println!("Paths:");
	for room in current.paths.iter() {
		println!("\t{}", room.name);
	}
	println!("Things:");
	for thing in current.things.iter() {
		println!("\t{}", thing.name);
	}
	println!("What do you want to do?");
	match parse_input(read!("{}\r\n")) { // TODO: this won't work on non-windows?
		ParseResult::Go(room_name) => {
			println!("Going to {}...", room_name);
			let index = current
				.paths
				.iter()
				.position(|r| r.name.to_string() == room_name)
				.unwrap();
			let room = current.paths.iter().nth(index).unwrap();
			explore(room);
		},
		ParseResult::Look(thing_name) => {
			println!("Looking at the {}...", thing_name);
			let index = current
				.things
				.iter()
				.position(|r| r.name.to_string() == thing_name)
				.unwrap();
			let thing = current.things.iter().nth(index).unwrap();
			println!("{}", thing.info);
			explore(current);
		},
		ParseResult::Fail => {
			println!("That didn't make any sense.");
			println!("Please respond in the form of");
			println!("\t\"go $ROOM_NAME\"");
			println!("\t\"look $THING_NAME\"");
			explore(current);
		},
		ParseResult::Quit => {
			println!("Goodbye!");
		}
	}
}

fn main() {
	let mut your_room = Room {
		name: "your room",
		paths: vec![],
		things: vec![
			Thing { name: "PC", info: "Looks like it's off." },
			Thing { name: "NES", info: "These were really popular back in the day." }
		]
	};
	let your_house = Room {
		name: "your house",
		paths: vec![],
		things: vec![
			Thing { name: "Mom", info: "Good luck out there!" }
		]
	};
	your_room.paths.push(Box::new(your_house));
	// TODO: make circular reference
	// your_house.paths.push(Box::new(your_room));
	explore(&your_room);
}
