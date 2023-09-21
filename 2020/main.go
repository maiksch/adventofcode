package main

import (
	"log"
)

func main() {
	input, err := readInput("day_01_part_two.txt")
	if err != nil {
		log.Fatal(err)
	}

	result, err := Day01PartTwo(input)
	if err != nil {
		log.Fatal(err)
	}

	println(result)
}

