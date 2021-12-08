#!/usr/bin/python


def part1(vals) -> int:
    return len([x for x in [y for x in [v[1] for v in vals] for y in x] if len(x) in [2, 3, 4, 7]])


def part2(vals) -> int:
    pass


def main():
    vals = [tuple([c.strip().split(" ") for c in l.split("|")]) for l in open("inputs/input8", 'r').readlines()]

    print("Part 1:", part1(vals))
    print("Part 2:", part2(vals))


if __name__ == "__main__":
    main()
