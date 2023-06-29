package aoc

fun day01() {
    val input = readFile("day01.txt")
    println(partOne(input))
    println(partTwo(input))
}

private fun partOne(input: List<String>): Int {
    return input
            .map(String::toInt)
            .foldIndexed(Pair(0, 0)) { i, acc, cur -> compareListValues(i, acc, cur) }
            .first
}

private fun partTwo(input: List<String>): Int {
    return input
            .map(String::toInt)
            .windowed(3)
            .map { it.sum() }
            .foldIndexed(Pair(0, 0)) { i, acc, cur -> compareListValues(i, acc, cur) }
            .first
}

private fun compareListValues(i: Int, acc: Pair<Int, Int>, cur: Int): Pair<Int, Int> {
    return when {
        i == 0 -> Pair(0, cur)
        cur > acc.second -> Pair(acc.first + 1, cur)
        else -> Pair(acc.first, cur)
    }
}