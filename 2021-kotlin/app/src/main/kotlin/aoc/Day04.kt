package aoc

fun day04() {
    val input = readFile("day04.txt")

    // Read first line
    val numbers = input[0]
        .split(",")
        .map(String::toInt)

    // Parse Input and create Boards
    val boards = input
        .slice(2 until input.size)
        .joinToString("\n")
        .split("\n\n")
        .map { Board(it) }

    println(partOne(numbers, boards))
    println(partTwo(numbers, boards))
}

private fun partOne(numbers: List<Int>, boards: List<Board>): Int {
    for (number in numbers) {
        for (board in boards) {
            board.markAndCheckBingo(number)

            if (board.hasBingo()) {
                return board.score
            }
        }
    }

    return 0
}

private fun partTwo(numbers: List<Int>, boards: List<Board>): Int {
    var notBingoBoards = boards.toMutableList()
    for (number in numbers) {
        for (board in notBingoBoards) {
            board.markAndCheckBingo(number)

            // The last board finally won
            if (board.hasBingo() && notBingoBoards.size == 1) {
                return board.score
            }
        }
        notBingoBoards = notBingoBoards.filter { !it.hasBingo() }.toMutableList()
    }

    return 0

}

class Board(input: String) {
    private val matrix: List<List<Int>>
    private val marks: List<MutableList<Boolean>>
    private val width: Int
    private val height: Int
    var score: Int = 0

    init {
        matrix = input
            .split("\n")
            .map { row ->
                row
                    .split(" ")
                    .map { it.trim() }
                    .filter { it.isNotBlank() }
                    .map { it.toInt() }
            }

        height = matrix.size
        width = matrix[0].size
        marks = List(height) { MutableList(width) { false } }
    }

    fun hasBingo(): Boolean {
        return score > 0
    }

    fun markAndCheckBingo(number: Int) {
        // mark
        for (row in (0 until height)) {
            for (column in (0 until width)) {
                if (matrix[row][column] == number) {
                    marks[row][column] = true
                }
            }
        }

        // check for horizontal bingo
        val horizontalBingo = marks.any { row -> row.all { it } }
        if (horizontalBingo) {
            calculateScore(number)
            return
        }

        // check for vertical bingo
        val verticalBingo = (0 until width).any { column ->
            (0 until height).all { row ->
                marks[row][column]
            }
        }
        if (verticalBingo) {
            calculateScore(number)
        }
    }

    private fun calculateScore(number: Int) {
        var sum = 0
        for (row in (0 until height)) {
            for (column in (0 until width)) {
                if (!marks[row][column]) {
                    sum += matrix[row][column]
                }
            }
        }
        score = sum * number
    }
}