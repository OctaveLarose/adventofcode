#!/usr/bin/python
import queue


def part1():
    counter = 0
    prev = None

    with open("input1", 'r') as f:
        for line in f.readlines():
            nbr = int(line[:-1])
            if prev is None:
                prev = nbr
            else:
                if nbr > prev:
                    counter += 1
                prev = nbr

    return counter


class Window:
    values = [None] * 3

    def print(self):
        print(self.values)

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


def part2():
    counter = 0

    window = Window()
    prev = None

    with open("input1", 'r') as f:
        for line in f.readlines():
            nbr = int(line[:-1])
            window.add(nbr)
            window.print()
            if window.has_none():
                continue
            else:
                if prev is not None:
                    print(window.get_sum(), " VS ", prev)
                    if window.get_sum() > prev:
                        counter += 1
                prev = window.get_sum()

    return counter


def main():
    print(part1())
    print(part2())


if __name__ == "__main__":
    main()
