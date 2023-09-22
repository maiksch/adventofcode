package util

import (
	"os"
	"strconv"
	"strings"
)

func ReadInput(filename string) string {
	file, err := os.ReadFile(filename)
	if err != nil {
		panic(err)
	}

	// Remove trailing line breaks and whitespace
	return strings.TrimSpace(string(file))
}

func GetLines(content string) []string {
	return strings.Split(content, "\n")
}

func MapLinesToNumbers(content string) []int {
	lines := GetLines(content)
	numbers := make([]int, len(lines))

	// convert strings to numbers
	for i := 0; i < len(lines); i++ {
		numbers[i], _ = strconv.Atoi(lines[i])
	}

	return numbers
}
