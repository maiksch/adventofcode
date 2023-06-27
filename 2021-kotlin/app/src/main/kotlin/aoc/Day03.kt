package aoc

fun day03() {
    val input = readFile("day03.txt")
    println(partOne(input))
    println(partTwo(input))
}

private fun partOne(input: List<String>): Int {
    var gammaRate = ""
    var epsilonRate = ""

    for (i in (0 until input.first().length)) {
        var count0 = 0
        var count1 = 0
        for (line in input) {
            val digit = line[i].digitToInt()
            if (digit == 0) {
                count0 += 1
            } else {
                count1 += 1
            }
        }
        gammaRate += if (count0 > count1) 0 else 1
        epsilonRate += if (count0 < count1) 0 else 1
    }

    return gammaRate.toInt(2) * epsilonRate.toInt(2)
}

private fun partTwo(input: List<String>): Int {
    val oxygenRating = findRating(input) { count0, count1 -> count1 >= count0 }
    val co2Rating = findRating(input) { count0, count1 -> count1 < count0 }
    
    return oxygenRating * co2Rating
}

private fun findRating(input: List<String>, criteria: (count0: Int, count1: Int) -> Boolean): Int {
    var filteredInput = input

    for (i in (0 until input.first().length)) {
        var count0 = 0
        var count1 = 0

        for (line in filteredInput) {
            val digit = line[i].digitToInt()
            if (digit == 0) {
                count0 += 1
            } else {
                count1 += 1
            }
        }

        filteredInput = filteredInput.filter {
            it[i].digitToInt() == if (criteria(count0, count1)) 1 else 0
        }

        if (filteredInput.size == 1) {
            return filteredInput[0].toInt(2)
        }
    }
    
    return 0
}