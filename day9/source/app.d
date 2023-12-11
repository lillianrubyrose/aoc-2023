import std.stdio;
import std.array;
import std.algorithm;
import std.conv;
import std.file;
import std.path;
import std.string;

int extrapolate(int[] line, bool part2) {
	int[][] diffs;
	diffs ~= line.dup;

	while (any(diffs.back.map!(x => x != 0))) {
		auto nextLine = diffs.back;
		auto newLine = new int[](nextLine.length - 1);
		foreach (i; 0 .. nextLine.length - 1) {
			newLine[i] = nextLine[i + 1] - nextLine[i];
		}
		diffs ~= newLine;
	}

	auto last = 0;
	foreach_reverse (int[] l; diffs[0 .. $ - 1]) {
		last = part2 ? l[0] - last : l[$ - 1] + last;
	}
	return last;
}

void main() {
	auto lines = readText("input.txt").splitLines();
	auto nums = lines.map!(l => l.split.map!(to!int).array).array;

	writeln("p1: ", nums.map!(line => extrapolate(line, false)).sum);
	writeln("p2: ", nums.map!(line => extrapolate(line, true)).sum);
}
