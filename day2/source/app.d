import std.stdio;
import std.file;
import std.string;
import std.conv;
import std.math;
import std.algorithm;

void main() {
	ulong part1 = 0, part2 = 0;
	auto lines = readText("input.txt").splitLines();

	foreach (i, line; lines) {
		auto rounds = line.split(": ")[1].split(";");
		auto mins = ["red": 0, "green": 0, "blue": 0];
		auto maxs = ["red": 12, "green": 13, "blue": 14];
		auto wrong = false;

		foreach (r; rounds) {
			foreach (cubes; r.split(", ")) {
				auto parts = cubes.split();
				int n = to!int(parts[0]);
				string color = parts[1];

				mins[color] = max(mins[color], n);
				if (n > maxs[color]) {
					wrong = true;
				}
			}
		}

		if (!wrong) {
			part1 += i + 1;
		}
		part2 += product(mins.values);
	}

	writeln("p1: ", part1);
	writeln("p2: ", part2);
}

ulong product(int[] values) {
	ulong result = 1;
	foreach (value; values) {
		result *= value;
	}
	return result;
}
