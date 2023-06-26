package aoc

data class Position(val horizontal: Int = 0, val depth: Int = 0, val aim: Int = 0)

data class Instruction(val direction: String, val distance: Int)

fun day_02() {
  var input = readFile("day02_example.txt")
  var instructions =
      input.map {
        var (direction, distance) = it.split(" ")
        Instruction(direction, distance.toInt())
      }

  println(part_one(instructions))
  println(part_two(instructions))
}

private fun part_one(instructions: List<Instruction>): Int {
  var pos = Position()

  for (instruction in instructions) {
    pos =
        when (instruction.direction) {
          "forward" -> Position(pos.horizontal + instruction.distance, pos.depth)
          "down" -> Position(pos.horizontal, pos.depth + instruction.distance)
          "up" -> Position(pos.horizontal, pos.depth - instruction.distance)
          else -> pos
        }
  }

  return pos.horizontal * pos.depth
}

private fun part_two(instructions: List<Instruction>): Int {
  var pos = Position()

  for (instruction in instructions) {
    pos =
        when (instruction.direction) {
          "forward" ->
              Position(
                  pos.horizontal + instruction.distance,
                  pos.depth + pos.aim * instruction.distance,
                  pos.aim
              )
          "down" -> Position(pos.horizontal, pos.depth, pos.aim + instruction.distance)
          "up" -> Position(pos.horizontal, pos.depth, pos.aim - instruction.distance)
          else -> pos
        }
  }

  return pos.horizontal * pos.depth
}
