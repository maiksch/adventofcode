package main

import "testing"

func TestPartOne(t *testing.T) {
	input := `FBFBBFFRLR
BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL`	

	want := 820
	have := partOne(input)

	if want != have {
		t.Fail()
	}
}
