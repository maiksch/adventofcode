package aoc

fun readFile(path: String): List<String> =
                object {}.javaClass.getResource(path)?.readText()!!.split("\n").map { it.trim() }
