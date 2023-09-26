package main

import "testing"

func TestPartOne(t *testing.T) {
	input := `nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6`

	want := 5
	have := partOne(input)

	if have != want {
		t.Fail()
	}
}

func TestPartTwo(t *testing.T) {
	input := `nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6`

	want := 8
	have := partTwo(input)

	if have != want {
		t.Fail()
	}
}
