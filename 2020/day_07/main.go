package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"regexp"
	"strconv"
)

var bags map[string][]string
var checked map[string]bool

func main() {
	file, err := os.Open("./input.txt")
	defer file.Close()
	if err != nil {
		panic(err)
	}

	fmt.Printf("Part One: %d\n", partOne(file))
	
	file.Seek(0, 0)
	fmt.Printf("Part Two: %d\n", partTwo(file))
}

func partOne(reader io.Reader) int {
	ans := 0
	bags = map[string][]string{}
	checked = map[string]bool{}

	scanner := bufio.NewScanner(reader)
	for scanner.Scan() {
		text := scanner.Text()

		regex := regexp.MustCompile(`(\w+\s\w+) bags contain`)
		bag := regex.FindStringSubmatch(text)[1]

		regex = regexp.MustCompile(`\d+\s(\w+\s\w+)+`)
		rules := regex.FindAllStringSubmatch(text, -1)

		nestedBags := []string{}

		for _, rule := range rules {
			nestedBags = append(nestedBags, rule[1])
		}

		bags[bag] = nestedBags
	}

	for name := range bags {
		if name == "shiny gold" {
			continue
		}
		if check(name) {
			ans += 1
		}
	}

	return ans
}

func check(name string) bool {
	if name == "shiny gold" {
		return true
	}

	if isChecked, ok := checked[name]; ok {
		return isChecked
	}

	nestedBags := bags[name]

	if len(nestedBags) == 0 {
		return false
	}

	for _, nestedBag := range nestedBags {
		canHoldShinyGold := check(nestedBag)
		if canHoldShinyGold {
			checked[nestedBag] = true
			return true
		}
	}

	return false
}

type NestedBag struct {
	name string
	amount int
}

func partTwo(reader io.Reader) int {
	bags := map[string][]NestedBag{}

	scanner := bufio.NewScanner(reader)
	for scanner.Scan() {
		text := scanner.Text()

		regex := regexp.MustCompile(`(\w+\s\w+) bags contain`)
		bag := regex.FindStringSubmatch(text)[1]

		regex = regexp.MustCompile(`(\d+)\s(\w+\s\w+)+`)
		rules := regex.FindAllStringSubmatch(text, -1)

		nestedBags := []NestedBag{}

		for _, rule := range rules {
			amount, _ := strconv.Atoi(rule[1])
			nestedBags = append(nestedBags, NestedBag{
				amount: amount,
				name: rule[2],
			})
		}

		bags[bag] = nestedBags
	}

	return countNestedBags("shiny gold", bags)
}

func countNestedBags(name string, bags map[string][]NestedBag) int {
	bag := bags[name]

	amount := 0

	for _, nested := range bag {
		amount += (countNestedBags(nested.name, bags) * nested.amount) + nested.amount
	}

	return amount
}
