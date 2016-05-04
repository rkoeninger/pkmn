#![allow(dead_code)]
#![allow(unused_variables)]

use std::fmt;

enum SpeciesType {
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
	name: String,
	type1: SpeciesType
	//ppMax:    u32
	//accuracy: u32 // 0 - 100
	//power:    u32
	//effect: EffectType
	//effectPropibility: u32 // 0 - 100
}

struct Transform {
	mode: TransformMode,
	into: Box<Species>
}

struct Species {
	index: u32,
	name: String,
	type1: SpeciesType,
	type2: Option<SpeciesType>,
	transforms: Vec<Transform>,
	schedule: Vec<(u32, Move)>
}

impl fmt::Display for Species {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "#{} {}", self.index, self.name)
	}
}

struct Specimen {
	name: Option<String>,
	species: Box<Species>,
	level: u32,
	status: Status,
	max_hp: u32,
	hp: u32,
	exp: u32,
	defense: u32,
	attack: u32
}

fn main() {
	let mewtwo = Species {
		index: 150,
		name: "Mewtwo".to_string(),
		type1: SpeciesType::Psychic,
		type2: None,
		transforms: vec![],
		schedule: vec![]
	};
	println!("{}", mewtwo);
}
