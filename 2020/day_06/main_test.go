package main

import "testing"

func TestPartOne(t *testing.T) {
	input := `abc

a
b
c

ab
ac

a
a
a
a

b`

	want := 11
	have := partOne(input)

	if want != have {
		t.Fail()
	}
}

func TestPartTwo(t *testing.T) {
	input := `abc

a
b
c

ab
ac

a
a
a
a

b`

	want := 6
	have := partTwo(input)

	if want != have {
		t.Fail()
	}
}
