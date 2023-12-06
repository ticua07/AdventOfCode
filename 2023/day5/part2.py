inputs, * \
    blocks = open(
        r"K:\rs_projects\advent_of_code\2023\day5\src\bin\input.txt").read().split("\n\n")

inputs = list(map(int, inputs.split(":")[1].split()))

seeds = []

for i in range(0, len(inputs), 2):
    seeds.append((inputs[i], inputs[i] + inputs[i + 1]))

for block in blocks:
    ranges = []
    for line in block.splitlines()[1:]:
        ranges.append(list(map(int, line.split())))
    new = []
    while len(seeds) > 0:
        start, end = seeds.pop()
        for dest, source, range in ranges:
            os = max(start, source)  # Overlapping start
            oe = min(end, source + range)  # Overlapping end
            if os < oe:
                new.append((os - source + dest, oe - source + dest))
                if os > start:
                    seeds.append((start, os))
                if end > oe:
                    seeds.append((oe, end))
                break
        else:

            new.append((start, end))
    seeds = new

print(min(seeds)[0])
