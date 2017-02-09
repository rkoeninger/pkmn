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

fn main() {
	let tackle = Move { name: "Tackle", type1: Element::Normal };
	let growl = Move { name: "Growl", type1: Element::Normal };
	let leech_seed = Move { name: "Leech Seed", type1: Element::Grass };
	let vine_whip = Move { name: "Vine Whip", type1: Element::Grass };
	let poison_powder = Move { name: "Poison Powder", type1: Element::Poison };
	let razor_leaf = Move { name: "Razor Leaf", type1: Element::Grass };
	let growth = Move { name: "Growth", type1: Element::Normal };
	let sleep_powder = Move { name: "Sleep Powder", type1: Element::Grass };
	let solar_beam = Move { name: "Solar Beam", type1: Element::Grass };
	let mewtwo = Species {
		index: 150,
		name: "Mewtwo",
		type1: Element::Psychic,
		type2: None,
		transforms: vec![],
		schedule: vec![]
	};
	let flareon = Species {
		index: 136,
		name: "Flareon",
		type1: Element::Fire,
		type2: None,
		transforms: vec![],
		schedule: vec![]
	};
	let jolteon = Species {
		index: 135,
		name: "Jolteon",
		type1: Element::Electric,
		type2: None,
		transforms: vec![],
		schedule: vec![]
	};
	let vaporeon = Species {
		index: 134,
		name: "Vaporeon",
		type1: Element::Water,
		type2: None,
		transforms: vec![],
		schedule: vec![]
	};
	let eevee = Species {
		index: 133,
		name: "Eevee",
		type1: Element::Normal,
		type2: None,
		transforms: vec![
			Transform { mode: TransformMode::ByStone(StoneType::FireStone), into: &flareon },
			Transform { mode: TransformMode::ByStone(StoneType::ThunderStone), into: &jolteon },
			Transform { mode: TransformMode::ByStone(StoneType::WaterStone), into: &vaporeon }
		],
		schedule: vec![]
	};
	let venusaur = Species {
		index: 3,
		name: "Venusaur",
		type1: Element::Grass,
		type2: Some(Element::Poison),
		transforms: vec![],
		schedule: vec![]
	};
	let ivysaur = Species {
		index: 2,
		name: "Ivysaur",
		type1: Element::Grass,
		type2: Some(Element::Poison),
		transforms: vec![
			Transform { mode: TransformMode::ByLevel(32), into: &venusaur }
		],
		schedule: vec![]
	};
	let bulbasaur = Species {
		index: 1,
		name: "Bulbasaur",
		type1: Element::Grass,
		type2: Some(Element::Poison),
		transforms: vec![
			Transform { mode: TransformMode::ByLevel(16), into: &ivysaur }
		],
		schedule: vec![
			(1, tackle),
			(1, growl),
			(7, leech_seed),
			(13, vine_whip),
			(20, poison_powder),
			(27, razor_leaf),
			(34, growth),
			(41, sleep_powder),
			(48, solar_beam)
		]
	};
	let vileplume = Species {
		index: 45,
		name: "Vileplume",
		type1: Element::Grass,
		type2: Some(Element::Poison),
		transforms: vec![],
		schedule: vec![]
	};
	let nuptup = Specimen {
		name: Some("Nuptup"),
		species: &vileplume,
		level: 35,
		status: None,
		max_hp: 100,
		hp: 80,
		exp: 365,
		defense: 50,
		attack: 30
	};
	println!("{}", bulbasaur);
	println!("{}", ivysaur);
	println!("{}", venusaur);
	println!("{}", vileplume);
	println!("{}", eevee);
	println!("{}", vaporeon);
	println!("{}", jolteon);
	println!("{}", flareon);
	println!("{}", mewtwo);

	println!("You awake in your room.");
	println!("You see a PC, an NES a door.");
	println!("What do you want to do?");
	let line: String = read!("{}\r\n");
	println!("You want to go to the {}?", line);
}
