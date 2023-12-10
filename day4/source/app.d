import std.stdio;
import std.array;
import std.regex;
import std.string;
import std.path;
import std.conv;
import std.algorithm;
import std.file;
import std.math;

int[] ints(string s) {
	auto matches = s.matchAll(regex("-?[0-9]+"));
	int[] result;
	foreach (m; matches) {
		result ~= to!int(m.hit);
	}
	return result;
}

void main() {
	auto lines = readText("input.txt").splitLines();
	int part1 = 0;
	int[] count = new int[](lines.length).map!(a => 1).array;

	foreach (i, line; lines) {
		auto parts = line.split(": ")[1].split(" | ");
		auto win = ints(parts[0]).to!(int[]);
		auto my = ints(parts[1]).to!(int[]);
		auto c = 0;
		foreach (int key; my) {
			if (win.canFind(key)) {
				c += 1;
			}
		}

		if (c > 0) {
			part1 += pow(2, c - 1);
		}

		foreach (j; 0 .. c) {
			count[i + j + 1] += count[i];
		}
	}

	writeln("p1: ", part1);
	writeln("p2: ", sum(count));
}
