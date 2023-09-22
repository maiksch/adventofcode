package main

import (
	"fmt"

	"github.com/maiksch/adventofcode/2020/util"
)

func main() {
	input := util.ReadInput("./part_one.txt")
	fmt.Printf("Part One: %d\n", partOne(input))

	input = util.ReadInput("./part_two.txt")
	fmt.Printf("Part Two: %d\n", partTwo(input))
}


func partOne(input string) int {
	numbers := util.MapLinesToNumbers(input)

	// look for numbers whose sum equals 2020
	for i := 0; i < len(numbers); i++ {
		for j := i + 1; j < len(numbers); j++ {
			if numbers[i] + numbers[j] == 2020 {
				return numbers[i] * numbers[j]
			}
		}
	}

	return 0
}

func partTwo(input string) int {
	numbers := util.MapLinesToNumbers(input)

	// look for 3 numbers whose sum equals 2020
	for i := 0; i < len(numbers); i++ {
		for j := i + 1; j < len(numbers); j++ {
			for k := i + 2; k < len(numbers); k++ {
				sum := numbers[i] + numbers[j] + numbers[k]
				if sum == 2020 {
					return numbers[i] * numbers[j] * numbers[k]
				}
			}
		}
	}

	return 0
}
