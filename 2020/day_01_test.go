package main

import "testing"

func TestDay01Part1(t *testing.T) {
	input := `1721
979
366
299
675
1456`

	want := 514579
	have, err := Day01PartOne(input)
	if err != nil {
		t.Fatal(err)
	}

	if want != have  {
		t.Fail()
	}
}

func TestDay01Part2(t *testing.T) {
	input := `1721
979
366
299
675
1456`

	want := 241861950
	have, err := Day01PartTwo(input)
	if err != nil {
		t.Fatal(err)
	}

	if want != have  {
		t.Fail()
	}
}
