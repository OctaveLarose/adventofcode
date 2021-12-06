#!/usr/bin/python
from collections import Counter, OrderedDict


def count_lanternfish(lfs: {int: int}, nbr_days: int) -> int:
    for i in range(nbr_days):
        for k, v in list(lfs.items())[1:]:
            lfs[k - 1] += v
            lfs[k] = 0
        lfs[8] = lfs[-1]
        lfs[6] += lfs[-1]
        lfs[-1] = 0

    return sum(v for v in lfs.values())


def main():
    lfs = OrderedDict((i, 0) for i in range(-1, 9))
    lfs.update(Counter([int(v) for v in open("inputs/input6", 'r').readline().split(",")]))

    print("Part 1:", count_lanternfish(lfs.copy(), 80))
    print("Part 2:", count_lanternfish(lfs.copy(), 256))


if __name__ == "__main__":
    main()
