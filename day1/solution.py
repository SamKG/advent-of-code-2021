def part1():
    prev = 1E100
    f = open("input.txt")
    count = 0
    for line in f.readlines():
        val = int(line.strip())
        if val > prev:
            count += 1
        prev = val
    print(count)

def part2():
    f = open("input.txt")
    values = [int(l.strip()) for l in f.readlines()]
    windows = [sum(values[i:i+3]) for i in range(len(values)) if len(values[i:i+3]) == 3]
    count = 0
    prev = 1E100
    for w in windows:
        if w > prev:
            count += 1
        prev = w
    print(count)