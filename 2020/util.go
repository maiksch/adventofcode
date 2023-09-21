package main

import (
	"os"
	"strconv"
	"strings"
)

func readInput(filename string) (string, error) {
	file, err := os.ReadFile("inputs/"+filename)
	if err != nil {
		return "", err
	}

	return string(file), nil
}

func mapLinesToNumbers(input string) ([]int, error) {
	var numbers []int
	lines := strings.Split(input, "\n")

	// convert strings to numbers
	for i := 0; i < len(lines); i++ {
		line := strings.TrimSpace(lines[i])
		if line == "" {
			continue
		}
		number, err := strconv.Atoi(line)
		if err != nil {
			return nil, err
		}
		numbers = append(numbers, number)
	}

	return numbers, nil
}
