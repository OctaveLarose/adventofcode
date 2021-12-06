#!/usr/bin/python


def part1(lfs: [int], nbr_days: int) -> int:
    for i in range(nbr_days):
        for idx, f in enumerate(lfs):
            lfs[idx] = lfs[idx] - 1
            if lfs[idx] == -1:
                lfs[idx] = 6
                lfs.append(9)
    return len(lfs)


def main():
    lfs = [int(v) for v in open("inputs/tests/testinput6", 'r').readline().split(",")]

    print("Part 1:", part1(lfs, 80))


if __name__ == "__main__":
    main()
