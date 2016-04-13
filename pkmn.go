package main

import (
	"fmt"
)

func main() {
	fmt.Println("Hello, World!")
}

const (
	normal   = 1 << iota
	flying   = 1 << iota
	grass    = 1 << iota
	bug      = 1 << iota
	poison   = 1 << iota
	fire     = 1 << iota
	rock     = 1 << iota
	ground   = 1 << iota
	fighting = 1 << iota
	water    = 1 << iota
	ice      = 1 << iota
	electric = 1 << iota
	psychic  = 1 << iota
	ghost    = 1 << iota
	dragon   = 1 << iota
)

const (
	none      = 0
	poisoned  = 1 << iota
	paralyzed = 1 << iota
	asleep    = 1 << iota
	confused  = 1 << iota
	burned    = 1 << iota
	frozen    = 1 << iota
)

type Species struct {
	name         string
	types        uint
	number       uint
	evolves_from *Species
	evolves_into *Species
}

type Specimen struct {
	name    string
	species *Species
	level   uint
	hp      uint
	exp     uint
	defense uint
	attack  uint
}
