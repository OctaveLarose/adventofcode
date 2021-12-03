#!/usr/bin/python

def part1(values: [[str]]):
    gamma_str = ''.join(list(map(lambda lst: max(set(lst), key=lst.count), list(zip(*values)))))
    epsilon_str = ''.join('0' if c == '1' else '1' for c in gamma_str)
    return int(gamma_str, 2) * int(epsilon_str, 2)


def main():
    values = [list(line[:-1]) for line in open("input3", 'r').readlines()]
    print(f"Part 1: {part1(values)}")


if __name__ == "__main__":
    main()
