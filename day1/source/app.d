import std.stdio;
import std.string;
import std.conv;
import std.file;

int tryParse(string line, int[string] numberMap) {
	foreach (num, value; numberMap) {
		if (line.startsWith(num)) {
			return value;
		}
	}
	return -1;
}

int solve(string[] lines, int[string] numberMap) {
	int result = 0;
	foreach (line; lines) {
		foreach (i; 0 .. line.length) {
			int value = tryParse(line[i .. $], numberMap);
			if (value != -1) {
				result += value * 10;
				break;
			}
		}

		foreach_reverse (i; 0 .. line.length) {
			int value = tryParse(line[i .. $], numberMap);
			if (value != -1) {
				result += value;
				break;
			}
		}
	}
	return result;
}

void main() {
	auto numberMapShort = [
		"0": 0,
		"1": 1,
		"2": 2,
		"3": 3,
		"4": 4,
		"5": 5,
		"6": 6,
		"7": 7,
		"8": 8,
		"9": 9
	];

	auto numberMapLong = [
		"zero": 0,
		"one": 1,
		"two": 2,
		"three": 3,
		"four": 4,
		"five": 5,
		"six": 6,
		"seven": 7,
		"eight": 8,
		"nine": 9
	];

	string[] lines = readText("input.txt").splitLines();
	writeln("p1: ", solve(lines, numberMapShort));
	writeln("p2: ", solve(lines, merge(numberMapShort, numberMapLong)));
}

int[string] merge(int[string] a, int[string] b) {
	int[string] result;
	foreach (key, value; a) {
		result[key] = value;
	}

	foreach (key, value; b) {
		result[key] = value;
	}

	return result;
}
