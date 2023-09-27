package main

import "testing"

func TestPartOne(t *testing.T) {
	input := `35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576`

	want := 127
	have := partOne(input, 5)

	if want != have {
		t.FailNow()
	}
}

func TestPartTwo(t *testing.T) {
	input := `35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576`

	want := 62
	have := partTwo(input, 5)

	if want != have {
		t.FailNow()
	}
}
