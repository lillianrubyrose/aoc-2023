import math, re, strutils

proc ints(line: string): seq[int] =
  var values = newSeq[int]()
  for match in re.findAll(line, re"\d+"):
    values.add(parseInt(match))
  return values

proc solve(input: string): int =
  var value = 1
  let lines = input.splitLines()
  let times = ints(lines[0])
  let dists = ints(lines[1])

  for i in 0 ..< min(times.len, dists.len):
    let time = float(times[i])
    let dist = float(dists[i])

    let disc = sqrt(time * time - 4 * dist)
    let roota = (-time - disc) / -2
    let rootb = (-time + disc) / -2

    let upper = int(floor(min(roota, rootb) + 1))
    let lower = int(ceil(max(roota, rootb) - 1))

    value *= (lower - upper + 1)

  return value

when isMainModule:
  let input = readFile("input.txt")
  echo "p1: ", solve(input)
  echo "p2: ", solve(replace(input, " ", ""))
