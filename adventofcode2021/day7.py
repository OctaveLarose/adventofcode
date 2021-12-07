#!/usr/bin/python


def part1(crabs) -> int:
    fuel_costs = []
    for i in range(min(crabs), max(crabs)):
        fuel_costs.append(sum([abs(c - i) for c in crabs]))
    return min(fuel_costs)


def main():
    crabs = [int(v) for v in open("inputs/input7", 'r').readline().split(",")]

    print("Part 1:", part1(crabs))


if __name__ == "__main__":
    main()
