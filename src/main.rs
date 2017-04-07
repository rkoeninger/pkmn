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
	name: &'static str
}

struct Room {
	paths: Vec<Path>,
	things: Vec<Thing>
}

struct Path {
	from: Box<Room>,
	to: Box<Room>
}

fn main() {
	// user inputs either:
	//   go <path>
	//   look <thing>
	println!("You awake in your room.");
	println!("You see a PC, an NES a door.");
	println!("What do you want to do?");
	let line: String = read!("{}\r\n");
	println!("You want to go to the {}?", line);
}
