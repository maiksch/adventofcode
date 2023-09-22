package main

import "testing"

func TestPartOne(t *testing.T) {
	input := `1721
979
366
299
675
1456`

	want := 514579
	have := partOne(input)

	if want != have  {
		t.Fail()
	}
}

func TestPartTwo(t *testing.T) {
	input := `1721
979
366
299
675
1456`

	want := 241861950
	have := partTwo(input)

	if want != have  {
		t.Fail()
	}
}
