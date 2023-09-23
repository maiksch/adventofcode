package main

import (
	"fmt"
	"regexp"
	"strings"

	"github.com/maiksch/adventofcode/2020/util"
)

func main() {
	input := util.ReadInput("./input.txt")

	fmt.Printf("Part One: %d\n", partOne(input))
	fmt.Printf("Part Two: %d\n", partTwo(input))
}

func partOne(input string) int {
	ans := 0
	lines := util.GetLines(input)
	passports := getPassports(lines)

	for _, passport := range passports {
		if checkPresent(passport) {
			ans += 1
		}
	}

	return ans
}

func partTwo(input string) int {
	ans := 0
	lines := util.GetLines(input)
	passports := getPassports(lines)

	for _, passport := range passports {
		if checkPresent(passport) && checkValid(passport) {
			ans += 1
		}
	}

	return ans
}

func getPassports(lines []string) []string {
	ans := []string{}
	passport := []string{}

	for _, line := range lines {
		if len(line) == 0 {
			ans = append(ans, strings.Join(passport, " "))
			passport = []string{}
		} else {
			passport = append(passport, line)
		}
	}
	ans = append(ans, strings.Join(passport, " "))

	return ans
}

func checkPresent(passport string) bool {
	requiredFields := []string {"byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"}
	for _, field := range requiredFields {
		if !strings.Contains(passport, fmt.Sprintf("%s:", field)) {
			return false
		}
	}
	return true
}

func checkValid(passport string) bool {
	byrValid, err := regexp.MatchString(`byr:(19[2-9][0-9]|200[0-2])\b`, passport)
	if err != nil || !byrValid {
		return false
	}

	iyrValid, err := regexp.MatchString(`iyr:(201[0-9]|2020)\b`, passport)
	if err != nil || !iyrValid {
		return false
	}

	eyrValid, err := regexp.MatchString(`eyr:(202[0-9]|2030)\b`, passport)
	if err != nil || !eyrValid {
		return false
	}

	hgtValid, err := regexp.MatchString(`hgt:((1[5-8][0-9]|19[0-3])cm|(59|6[0-9]|7[0-6])in)\b`, passport)
	if err != nil || !hgtValid {
		return false
	}

	hclValid, err := regexp.MatchString(`hcl:#([0-9]|[a-f]){6}\b`, passport)
	if err != nil || !hclValid {
		return false
	}

	eclValid, err := regexp.MatchString(`ecl:(amb|blu|brn|gry|grn|hzl|oth)\b`, passport)
	if err != nil || !eclValid{
		return false
	}

	pidValid, err := regexp.MatchString(`pid:\d{9}\b`, passport)
	if err != nil || !pidValid {
		return false
	}

	return true
}
