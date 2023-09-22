package main

import (
	"fmt"
	"regexp"
	"strconv"
	"strings"

	"github.com/maiksch/adventofcode/2020/util"
)

func main() {
	input := util.ReadInput("./part_one.txt")
	fmt.Printf("Part One: %d\n", PartOne(input))

	input = util.ReadInput("./part_two.txt")
	fmt.Printf("Part Two: %d\n", PartTwo(input))
}

func PartOne(input string) int {
	lines := util.GetLines(input)
	result := 0

	for _, l := range lines {
		line := parseLine(l)
		count := strings.Count(line.pw, line.char)

		if line.first <= count && count <= line.second {
			result += 1
		}
	}

	return result
}

func PartTwo(input string) int {
	lines := util.GetLines(input)
	result := 0

	for _, l := range lines {
		line := parseLine(l)
		count := 0
		if string(line.pw[line.first - 1]) == line.char {
			count += 1
		}
		if string(line.pw[line.second - 1]) == line.char {
			count += 1
		}
		if count == 1 {
			result += 1
		}
	}

	return result
}

type line struct {
	first int
	second int
	char string
	pw string
}
func parseLine(l string) line {
	regex, _ := regexp.Compile(`^(\d+)-(\d+)\s(\w+):\s(\w+)$`)
	groups := regex.FindStringSubmatch(l)

	first, _ := strconv.Atoi(groups[1])
	second, _ := strconv.Atoi(groups[2])

	return line{
		first: first,
		second: second,
		char: groups[3],
		pw: groups[4],
	}
}
