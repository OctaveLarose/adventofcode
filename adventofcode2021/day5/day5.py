#!/usr/bin/python

from collections import Counter


def part1(values: [((int, int), (int, int))]) -> int:
    c_list = []
    for (a, b), (c, d) in values:
        if not (a == c or b == d):
            continue
        if a == c:
            for i in range(min(b, d), max(b, d) + 1):
                c_list.append((a, i))
        if b == d:
            for i in range(min(a, c), max(a, c) + 1):
                c_list.append((i, b))

    return sum(map(lambda x: x > 1, Counter(c_list).values()))


def main():
    values = [(tuple(tuple(int(v) for v in bll.split(",")) for bll in line.strip().split('->'))) for line in open("input5", 'r').readlines()]

    print("Part 1:", part1(values))


if __name__ == "__main__":
    main()
