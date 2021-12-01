#!/usr/bin/python

def part1(nbrs: [int]) -> int:
    counter = 0
    prev = None

    for nbr in nbrs:
        if prev is None:
            prev = nbr
        else:
            if nbr > prev:
                counter += 1
            prev = nbr

    return counter


class Window:
    values = [None] * 3

    def has_none(self):
        return None in self.values

    def add(self, nbr):
        for idx, val in enumerate(self.values):
            if val is None:
                self.values[idx] = nbr
                return

        tmp1 = self.values[0]
        self.values[0] = nbr
        for idx in range(1, len(self.values)):
            tmp2 = self.values[idx]
            self.values[idx] = tmp1
            tmp1 = tmp2

    def get_sum(self):
        sum = 0
        for val in self.values:
            sum += val if val is not None else 0
        return sum


def part2(nbrs: [int]) -> int:
    counter = 0

    window = Window()
    prev = None

    for nbr in nbrs:
        window.add(nbr)
        if window.has_none():
            continue
        else:
            if prev is not None:
                if window.get_sum() > prev:
                    counter += 1
            prev = window.get_sum()

    return counter


def main():
    nbrs = [int(line[:-1]) for line in open("input1", 'r').readlines()]

    print("Part 1:", part1(nbrs))
    print("Part 2:", part2(nbrs))


if __name__ == "__main__":
    main()
