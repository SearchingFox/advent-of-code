import strutils, sequtils

var example = "3,4,3,1,2".split(",").mapIt(it.parseInt)
var part1 = readFile("day6.txt").split(",").mapIt(it.parseInt)

for i in 0..<256:
    var t = 0
    for f in 0..<part1.len:
        if part1[f] == 0:
            t += 1
            part1[f] = 6
        else:
            part1[f] -= 1

    for i in 0..<t:
        part1.add(8)

echo part1.len
