package main

import (
	"fmt"
)

func main() {

	mewtwo := species{"Mewtwo", psychic, 150, []transform{}}

	fmt.Println(mewtwo)

	flareon := species{"Flareon", fire, 136, []transform{}}
	jolteon := species{"Jolteon", electric, 135, []transform{}}
	vaporeon := species{"Vaporeon", water, 134, []transform{}}
	eevee := species{"Eevee", normal, 133, []transform{
		transformByStone(waterStone, &vaporeon),
		transformByStone(thunderStone, &jolteon),
		transformByStone(fireStone, &flareon)}}

	fmt.Println(eevee)

	weezing := species{"Weezing", poison, 110, []transform{}}
	koffing := species{"Koffing", poison, 109, []transform{
		transformByLevel(35, &weezing)}}

	fmt.Println(koffing)

	cloyster := species{"Cloyster", water | ice, 91, []transform{}}
	shelder := species{"Shelder", water, 90, []transform{
		transformByStone(waterStone, &cloyster)}}

	fmt.Println(shelder)

	alakazam := species{"Alakazam", psychic, 65, []transform{}}
	kadabra := species{"Kadabra", psychic, 64, []transform{
		transformByTrade(&alakazam)}}
	abra := species{"Abra", psychic, 63, []transform{
		transformByLevel(16, &kadabra)}}

	fmt.Println(abra)

	vileplume := species{"Vileplume", poison | grass, 45, []transform{}}
	gloom := species{"Gloom", poison | grass, 44, []transform{
		transformByStone(leafStone, &vileplume)}}
	oddish := species{"Oddish", poison | grass, 43, []transform{
		transformByLevel(21, &gloom)}}
	nuptup := specimen{"Nuptup", &vileplume, 35, noStatus, 90, 90, 143, 40, 20}

	fmt.Println(gloom)
	fmt.Println(oddish)
	fmt.Println(nuptup.species)
	fmt.Println(nuptup)

	venusaur := species{"Venusaur", grass | poison, 3, []transform{}}
	ivysaur := species{"Ivysaur", grass | poison, 2, []transform{
		transformByLevel(32, &venusaur)}}
	blubasaur := species{"Bulbasaur", grass | poison, 1, []transform{
		transformByLevel(16, &ivysaur)}}

	fmt.Println(blubasaur)
}

type speciesType uint

const (
	normal   speciesType = 1 << iota
	flying               = 1 << iota
	grass                = 1 << iota
	bug                  = 1 << iota
	poison               = 1 << iota
	fire                 = 1 << iota
	rock                 = 1 << iota
	ground               = 1 << iota
	fighting             = 1 << iota
	water                = 1 << iota
	ice                  = 1 << iota
	electric             = 1 << iota
	psychic              = 1 << iota
	ghost                = 1 << iota
	dragon               = 1 << iota
)

type status uint

const (
	// noStatus could mean healthy or fainted if hp == 0
	noStatus  status = 0
	poisoned         = 1 << (iota - 1)
	paralyzed        = 1 << (iota - 1)
	asleep           = 1 << (iota - 1)
	confused         = 1 << (iota - 1)
	burned           = 1 << (iota - 1)
	frozen           = 1 << (iota - 1)
)

type transformMode uint

const (
	byLevel transformMode = iota
	byStone
	byTrade
)

type stoneType uint

const (
	fireStone stoneType = iota
	waterStone
	thunderStone
	leafStone
	moonStone
)

type transform struct {
	mode           transformMode
	requiredLevel  uint
	stoneType      stoneType
	transformsInto *species
}

func transformByLevel(requiredLevel uint, transformsInto *species) transform {
	return transform{byLevel, requiredLevel, fireStone, transformsInto}
}

func transformByStone(stoneType stoneType, transformsInto *species) transform {
	return transform{byStone, 0, stoneType, transformsInto}
}

func transformByTrade(transformsInto *species) transform {
	return transform{byTrade, 0, fireStone, transformsInto}
}

type species struct {
	name       string
	types      speciesType
	number     uint
	transforms []transform
}

type specimen struct {
	name    string
	species *species
	level   uint
	status  status
	maxHp   uint
	hp      uint
	exp     uint
	defense uint
	attack  uint
}