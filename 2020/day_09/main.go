package main

import (
	"fmt"
	"math"
	"strconv"

	"github.com/maiksch/adventofcode/2020/util"
)

func main() {
	input := util.ReadInput("./input.txt")

	fmt.Printf("Part One: %d\n", partOne(input, 25))
	fmt.Printf("Part Two: %d\n", partTwo(input, 25))
}

func partOne(input string, preambleSize int) int {
	lines := util.GetLines(input)
	preamble := make([]int, preambleSize)
	numbers := make([]int, len(lines))

	// transform lines to numbers
	for i, line := range lines {
		numbers[i], _ = strconv.Atoi(line)
	}

	// fill preamble
	for i := 0; i < preambleSize; i++ {
		preamble[i] = numbers[i]
	}

	// go over the lines and start after preamble offset
	for i := preambleSize; i < len(numbers); i++ {
		if !isSum(numbers[i], preamble) {
			return numbers[i]
		}
		// override the "oldest" number in the preamble
		preamble[i % preambleSize] = numbers[i]
	}

	return 0
}

func isSum(sum int, preamble []int) bool {
	for i := 0; i < len(preamble); i++ {
		summand1 := preamble[i]

		for j := i + 1; j < len(preamble); j++ {
			summand2 := preamble[j]

			if (summand1 + summand2 == sum) {
				return true
			}
		}
	}

	return false
}

func partTwo(input string, preambleSize int) int {
	lines := util.GetLines(input)
	numbers := make([]int, len(lines))

	// transform lines to numbers
	for i, line := range lines {
		numbers[i], _ = strconv.Atoi(line)
	}

	// get the invalid number from part one
	invalidNumber := partOne(input, preambleSize)

	// find the window that sums to the invalid number
	var from, to int
	for i, num := range numbers {
		sum := num

		j := i
		for sum < invalidNumber {
			j += 1
			sum += numbers[j]
		}

		if sum == invalidNumber {
			from = i
			to = j
			break
		}
	}

	// find the smallest and largest number between the two indices
	largest := math.MinInt
	smallest := math.MaxInt
	for i := from; i <= to; i++ {
		largest = max(largest, numbers[i])
		smallest = min(smallest, numbers[i])
	}

	return largest + smallest
}
