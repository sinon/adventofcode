package main

import (
	_ "embed"
	"fmt"
	"strings"
)

//go:embed input.txt
var input string

var dirs = [][2]int{
	{-1, 0}, // Move north
	{0, 1},  // Move east
	{1, 0},  // Move south
	{0, -1}, // Move west
}

func Abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func TaxiCabDistance(x1, y1, x2, y2 int) int {
	return Abs(x1-x2) + Abs(y1-y2)
}

func main() {
	xCoOrd, yCoOrd := 0, 0
	input = strings.TrimSpace(input)
	var dirFacing int = 0

	fmt.Println("Part 1")
	for _, command := range strings.Split(input, ", ") {
		var turn string
		var steps int
		fmt.Sscanf(command, "%1s%d", &turn, &steps)

		if turn == "R" {
			dirFacing = (dirFacing + 1) % 4
		} else if turn == "L" {
			dirFacing = (dirFacing + 3) % 4
		} else {
			panic("unhandled turning direction " + turn)
		}

		for step := 0; step < steps; step++ {
			xCoOrd += dirs[dirFacing][0]
			yCoOrd += dirs[dirFacing][1]
		}
	}
	fmt.Printf("X: %v, Y: %v \n", xCoOrd, yCoOrd)
	fmt.Printf("Distance: %v \n", TaxiCabDistance(0, 0, xCoOrd, yCoOrd))

	fmt.Println("Part 2")
	xCoOrd = 0
	yCoOrd = 0
	dirFacing = 0
	visitedCoOrds := map[[2]int]bool{
		{0, 0}: true,
	}
	for _, command := range strings.Split(input, ", ") {
		var turn string
		var steps int
		fmt.Sscanf(command, "%1s%d", &turn, &steps)

		if turn == "R" {
			dirFacing = (dirFacing + 1) % 4
		} else if turn == "L" {
			dirFacing = (dirFacing + 3) % 4
		} else {
			panic("unhandled turning direction " + turn)
		}

		for step := 0; step < steps; step++ {
			xCoOrd += dirs[dirFacing][0]
			yCoOrd += dirs[dirFacing][1]
			if visitedCoOrds[[2]int{xCoOrd, yCoOrd}] {
				distance := TaxiCabDistance(0, 0, xCoOrd, yCoOrd)
				fmt.Printf("P2 Distance: %v \n", distance)
				return
			}
			visitedCoOrds[[2]int{xCoOrd, yCoOrd}] = true
		}
	}
}
