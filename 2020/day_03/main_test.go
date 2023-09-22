package main

import "testing"

func TestPartOne(t *testing.T) {
	input := `..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#`

	want := 7
	have := partOne(input)

	if want != have {
		t.Fail()
	}
}

func TestPartTwo(t *testing.T) {
	input := `..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#`

	want := 336
	have := partTwo(input)

	if want != have {
		t.Fail()
	}
}
