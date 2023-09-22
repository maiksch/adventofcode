package main

import "testing"

func TestPartOne(t *testing.T) {
	input := `1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc`

	want := 2
	have := PartOne(input)
	
	if want != have {
		t.Fail()
	}
}

func TestPartTwo(t *testing.T) {
	input := `1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc`

	want := 1
	have := PartTwo(input)

	if want != have {
		t.Fail()
	}
}
