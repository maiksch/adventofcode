package main

import (
	"fmt"

	"github.com/maiksch/adventofcode/2020/util"
)

func main() {
	input := util.ReadInput("./input.txt")

	fmt.Printf("Part One: %d\n", partOne(input))
	fmt.Printf("Part Two: %d\n", partTwo(input))
}

func partOne(input string) int {
	lines := util.GetLines(input)

	return traverse(lines, 3, 1)	
}

func partTwo(input string) int {
	lines := util.GetLines(input)
	slopes := []struct {right int; down int} {
		{ right: 1, down: 1 },
		{ right: 3, down: 1 },
		{ right: 5, down: 1 },
		{ right: 7, down: 1 },
		{ right: 1, down: 2 },
	}

	ans := 1
	for _, slope := range slopes {
		ans *= traverse(lines, slope.right, slope.down)
	}

	return ans
}

func traverse(lines []string, right int, down int) int {
	var ans, x, y int

	for y < len(lines) {
		line := lines[y]
		char := string(line[x])
		if char == "#" {
			ans += 1
		}
		x = (x + right) % len(line)
		y += down
	}

	return ans
}
