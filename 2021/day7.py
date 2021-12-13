#!/usr/bin/python


def part1(crabs) -> int:
    return min([sum([abs(c - i) for c in crabs]) for i in range(min(crabs), max(crabs))])


def part2(crabs) -> int:
    return min([sum([abs(c - i) * (abs(c - i) + 1) // 2 for c in crabs]) for i in range(min(crabs), max(crabs))])


def main():
    crabs = [int(v) for v in open("inputs/input7", 'r').readline().split(",")]

    print("Part 1:", part1(crabs))
    print("Part 2:", part2(crabs))


if __name__ == "__main__":
    main()
