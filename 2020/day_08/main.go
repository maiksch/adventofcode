package main

import (
	"fmt"
	"strconv"
	"strings"

	"github.com/maiksch/adventofcode/2020/util"
)

func main() {
	input := util.ReadInput("./input.txt")

	fmt.Printf("Part One: %d\n", partOne(input))
	fmt.Printf("Part Two: %d\n", partTwo(input))
}

func partOne(input string) int {
	program := strings.Split(input, "\n")

	acc, _ := runProgram(program)

	return acc
}

func partTwo(input string) int {
	originalProgram := strings.Split(input, "\n")

	for i := 0; i < len(originalProgram); i++ {
		program := make([]string, len(originalProgram))
		copy(program, originalProgram)

		op, arg := readInstruction(program[i])

		if op == "acc" {
			continue
		}
		if op == "nop" {
			program[i] = fmt.Sprintf("jmp %+d", arg)
		}
		if op == "jmp" {
			program[i] = fmt.Sprintf("nop %+d", arg)
		}

		acc, terminates := runProgram(program)
		if terminates {
			return acc
		}
	}

	return 0
}

func runProgram(program []string) (int, bool) {
	acc := 0 // accumulator
	pc := 0 // program counter
	ic := map[int]bool{} // instruction counter
	terminates := false

	for isNewInstruction(pc, ic) {
		if pc >= len(program) {
			terminates = true
			break
		}

		ic[pc] = true
		op, arg := readInstruction(program[pc])

		if op == "nop" {
			pc += 1
		}
		if op == "acc" {
			pc += 1
			acc += arg
		}
		if op == "jmp" {
			pc += arg
		}
	}

	return acc, terminates
}

func readInstruction(line string) (string, int) {
	instruction := strings.Split(line, " ")
	op := instruction[0]
	arg, _ := strconv.Atoi(instruction[1]) 
	return op, arg
}

func isNewInstruction(pc int, ic map[int]bool) bool {
	_, ok := ic[pc]
	return !ok
}
