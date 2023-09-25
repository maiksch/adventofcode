package main

import (
	"fmt"
	"math/bits"
	"strings"

	"github.com/maiksch/adventofcode/2020/util"
)

func main() {
	input := util.ReadInput("./input.txt")

	fmt.Printf("Part One: %d\n", partOne(input))
	fmt.Printf("Part Two: %d\n", partTwo(input))
}

func partOne(input string) int {
	groups := strings.Split(input, "\n\n")
	ans := 0

	for _, group := range groups {
		persons := strings.Split(group, "\n")
		var answers uint32

		for _, person := range persons {
			for _, question := range person {
				// use a 26 (1 for every letter in the alphabet) bit number to count the unique answers.
				// every question corresponds to one specific bit (e.g. "a" = 0001, "d" = 1000).

				// make it start at 0 instead of 97
				zeroBased := int(question - rune('a'))

				// use bitshifting to get the bit that belongs to the letter
				// e.g. "c" is the 3rd letter, so the 3rd bit becomes one (100)
				answer := uint32(1 << zeroBased)

				// use the bitwise OR to mark the questions, that ANY person answered YES to
				answers |= answer
			}
		}

		ans += bits.OnesCount32(answers)
	}

	return ans
}

func partTwo(input string) int {
	groups := strings.Split(input, "\n\n")
	ans := 0

	for _, group := range groups {
		persons := strings.Split(group, "\n")
		// set group groupAnswers to all 1s, so that 
		groupAnswers := uint32(1 << 27) - 1

		for _, person := range persons {
			var personAnswers uint32

			for _, question := range person {
				// use a 26 (1 for every letter in the alphabet) bit number to count the unique answers.
				// every question corresponds to one specific bit (e.g. "a" = 0001, "d" = 1000).

				// make it start at 0 instead of 97
				zeroBased := int(question - rune('a'))

				// use bitshifting to get the bit that belongs to the letter
				// e.g. "c" is the 3rd letter, so the 3rd bit becomes one (100)
				answer := uint32(1 << zeroBased)

				// use the bitwise OR to add the bit to the persons answers
				personAnswers |= answer
			}

			// use the bitwise AND to mark questions that EVERY person answered YES to
			groupAnswers &= personAnswers
		}

		ans += bits.OnesCount32(groupAnswers)
	}

	return ans
}
