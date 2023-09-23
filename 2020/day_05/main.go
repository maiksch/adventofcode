package main

import (
	"fmt"
	"math"
	"slices"

	"github.com/maiksch/adventofcode/2020/util"
)

func main() {
	input := util.ReadInput("./input.txt")

	fmt.Printf("Part One: %d\n", partOne(input))
	fmt.Printf("Part Two: %d\n", partTwo(input))
}

func partOne(input string) int {
	lines := util.GetLines(input)

	seatIds := getSeatIds(lines)

	return slices.Max(seatIds)
}

func partTwo(input string) int {
	lines := util.GetLines(input)
	seatIds := getSeatIds(lines)
	slices.Sort(seatIds)

	for i, seat := range seatIds {
		if seatIds[i+1] != seat + 1 {
			return seat+1
		}
	}

	return 0 
}

func getSeatIds(lines []string) []int {
	seatIds := []int{}

	for _, line := range lines {
		lowerRow, upperRow := 0, 127
		lowerCol, upperCol := 0, 7

		for _, c := range line {
			char := string(c)
			deltaRow := int(math.Ceil(float64(upperRow - lowerRow) / 2))
			deltaCol := int(math.Ceil(float64(upperCol - lowerCol) / 2))
			if char == "F" {
				upperRow -= deltaRow
			}
			if char == "B" {
				lowerRow += deltaRow
			}
			if char == "L" {
				upperCol -= deltaCol
			}
			if char == "R" {
				lowerCol += deltaCol
			}
		}
		seatIds = append(seatIds, lowerRow * 8 + lowerCol)
	}

	return seatIds
}
