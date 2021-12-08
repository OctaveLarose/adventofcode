#!/usr/bin/python
from collections import Counter


def part1(vals) -> int:
    return len([x for x in [y for x in [v[1] for v in vals] for y in x] if len(x) in [2, 3, 4, 7]])


def part2(vals) -> int:
    for patterns in [v[0] for v in vals]:
        mapped_vals = {i: None for i in range(10)}
        for a, b in [(2, 1), (3, 7), (4, 4), (7, 8)]:
            mapped_vals[b] = next(filter(lambda val: len(val) == a, patterns))

        f_letter = next((letter for letter, nbr in Counter(''.join([p for p in patterns])).items() if nbr == 9), None)
        mapped_vals[2] = next(filter(lambda val: f_letter not in val, patterns))

        # Len 5: 2, 3, 5
        # Len 6: 0, 6, 9. 8 minus dddd, cccc and eeee respectively
        # 2 missing A but known, 4 missing B, 2 C, 3 D, 6 E, 1 F!!!, 3 G (all known!)

        print(mapped_vals)
    return 42


def main():
    vals = [tuple([c.strip().split(" ") for c in l.split("|")]) for l in open("inputs/tests/testinput8-2", 'r').readlines()]

    print("Part 1:", part1(vals))
    print("Part 2:", part2(vals))


if __name__ == "__main__":
    main()
