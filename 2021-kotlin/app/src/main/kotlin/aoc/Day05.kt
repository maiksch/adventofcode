package aoc

import kotlin.math.max
import kotlin.math.min

private data class Point(val x: Int, val y: Int) {
    companion object {
        fun parse(input: String): Point {
            val (x, y) = input.split(",").map(String::toInt)
            return Point(x, y)
        }
    }
}

private class Line(input: String, withDiagonal: Boolean) {
    var points: MutableList<Point> = mutableListOf()

    init {
        val parts = input.split(" ")
        val from = Point.parse(parts.first().trim())
        val to = Point.parse(parts.last().trim())

        val isHorizontalLine = from.x != to.x && from.y == to.y
        if (isHorizontalLine) {
            val lower = min(from.x, to.x)
            val upper = max(from.x, to.x)
            for (x in lower..upper) {
                points.add(Point(x, from.y))
            }
        }

        val isVerticalLine = from.x == to.x && from.y != to.y
        if (isVerticalLine) {
            val lower = min(from.y, to.y)
            val upper = max(from.y, to.y)
            for (y in lower..upper) {
                points.add(Point(from.x, y))
            }
        }

        val isDiagonalLine = from.x != to.x && from.y != to.y
        if (withDiagonal && isDiagonalLine) {
            val toTopRight = from.x < to.x && from.y < to.y
            val toBottomLeft = from.x > to.x && from.y > to.y
            if (toTopRight || toBottomLeft) {
                val start = if (toTopRight) from else to
                val end = if (toTopRight) to else from
                var x = start.x
                var y = start.y

                while (x <= end.x && y <= end.y) {
                    points.add(Point(x, y))
                    x += 1
                    y += 1
                }
            }
            val toBottomRight = from.x < to.x && from.y > to.y
            val toTopLeft = from.x > to.x && from.y < to.y
            if (toBottomRight || toTopLeft) {
                val start = if (toBottomRight) from else to
                val end = if (toBottomRight) to else from
                var x = start.x
                var y = start.y

                while (x <= end.x && y >= end.y) {
                    points.add(Point(x, y))
                    x += 1
                    y -= 1
                }
            }
        }
    }
}

fun day05() {
    val input = readFile("day05.txt")

    println(partOne(input))
    println(partTwo(input))
}

private fun partOne(input: List<String>): Int {
    return calculate(input, false)
}

private fun partTwo(input: List<String>): Int {
    return calculate(input, true)
}

private fun calculate(input: List<String>, withDiagonal: Boolean): Int {
    val lines = input.map { Line(it, withDiagonal) }.filter { it.points.any() }

    val width = lines.flatMap { it.points }.maxOf { it.x } + 1
    val height = lines.flatMap { it.points }.maxOf { it.y } + 1

    val coordinateSystem = MutableList(width) { MutableList(height) { 0 } }

    for (line in lines) {
        for (point in line.points) {
            coordinateSystem[point.x][point.y] += 1
        }
    }

    var overlappingCounter = 0
    for (y in 0 until height) {
        for (x in 0 until width) {
            val value = coordinateSystem[x][y]
            if (value > 1) {
                overlappingCounter += 1
            }
        }
    }

    return overlappingCounter
}