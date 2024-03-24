import java.io.File
import java.io.BufferedReader

fun main() {
	println("PART ONE:")
	var input = """
		1abc2
		pqr3stu8vwx
		a1b2c3d4e5f
		treb7uchet""".trimIndent()
	var bufferedReader = BufferedReader(input.reader())
	println(partOne(bufferedReader))

	bufferedReader = File("day_01_part_1.txt").bufferedReader()
	println(partOne(bufferedReader))

	println("PART TWO:")

	input = """
		two1nine
		eightwothree
		abcone2threexyz
		xtwone3four
		4nineeightseven2
		zoneight234
		7pqrstsixteen
		""".trimIndent()
	bufferedReader = BufferedReader(input.reader())
	println(partTwo(bufferedReader))

	bufferedReader = File("day_01_part_2.txt").bufferedReader()
	println(partTwo(bufferedReader))
}

fun partOne(reader: BufferedReader): Int {
	var numbers = mutableListOf<Int>()

	reader.forEachLine { line ->
		val first = line.fold<Int?>(null) { result, char ->
			when {
				result != null -> result
				else -> char.toString().toIntOrNull()
			}
		}
		
		val last = line.foldRight<Int?>(null) { char, result ->
			when {
				result != null -> result
				else -> char.toString().toIntOrNull()
			}
		}

		numbers.add("$first$last".toInt())
	}

	return numbers.sum()
}

fun partTwo(reader: BufferedReader): Int {
	var numbers = mutableListOf<Int>()

	fun containsNumber(string: String): Int? {
		return when {
			string.contains("one") -> 1
			string.contains("two") -> 2
			string.contains("three") -> 3
			string.contains("four") -> 4
			string.contains("five") -> 5
			string.contains("six") -> 6
			string.contains("seven") -> 7
			string.contains("eight") -> 8
			string.contains("nine") -> 9
			else -> null
		}
	}

	reader.forEachLine { line ->
		var first: Int? = null
		var string = ""
		var last: Int? = null

		for (char in line) {
			var number = char.toString().toIntOrNull()
			if (number != null) {
				first = number
				break
			}
			string += char
			number = containsNumber(string)
			if (number != null) {
				first = number
				break
			}
		}

		string = ""

		for (char in line.reversed()) {
			var number = char.toString().toIntOrNull()
			if (number != null) {
				last = number
				break
			}
			string = "$char$string"
			number = containsNumber(string)
			if (number != null) {
				last = number
				break
			}
		}

		numbers.add("$first$last".toInt())
	}

	return numbers.sum()
}