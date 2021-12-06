#!/usr/bin/python


def part1(lfs: [int], nbr_days: int) -> int:
    print(lfs)
    return 42


def main():
    lfs = [int(v) for v in open("inputs/tests/testinput6", 'r').readline().split(",")]

    print("Part 1:", part1(lfs, 18))


if __name__ == "__main__":
    main()
