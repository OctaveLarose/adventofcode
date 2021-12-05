#!/usr/bin/python

from collections import Counter


def get_overlaps(values: [((int, int), (int, int))], handle_diags) -> int:
    c_list = []
    for (a, b), (c, d) in values:
        if handle_diags and not (a == c or b == d):
            c_list.extend([(a + (i if a < c else -i), b + (i if b < d else -i)) for i in range(abs(a - c) + 1)])
        if a == c:
            c_list.extend([(a, i) for i in range(min(b, d), max(b, d) + 1)])
        if b == d:
            c_list.extend([(i, b) for i in range(min(a, c), max(a, c) + 1)])

    return sum(map(lambda x: x > 1, Counter(c_list).values()))


def main():
    values = [(tuple(tuple(int(v) for v in bll.split(",")) for bll in line.strip().split('->'))) for line in open("input5", 'r').readlines()]

    print("Part 1:", get_overlaps(values, False))
    print("Part 2:", get_overlaps(values, True))


if __name__ == "__main__":
    main()
