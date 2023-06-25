fun day_01() {
  var input = readFile("day01.txt")
  println(part_one(input))
  println(part_two(input))
}

fun part_one(input: List<String>): Int {
  return input
      .map { it.toInt() }
      .foldIndexed(
          Pair(0, 0),
          { i, acc, cur ->
            if (i == 0) Pair(0, cur)
            else if (cur > acc.second) Pair(acc.first + 1, cur) else Pair(acc.first, cur)
          }
      )
      .first
}

fun part_two(input: List<String>): Int {
  return input
      .map { it.toInt() }
      .windowed(3)
      .map { it.sum() }
      .foldIndexed(
          Pair(0, 0),
          { i, acc, cur ->
            if (i == 0) Pair(0, cur)
            else if (cur > acc.second) Pair(acc.first + 1, cur) else Pair(acc.first, cur)
          }
      )
      .first
}
