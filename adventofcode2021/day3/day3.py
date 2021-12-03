#!/usr/bin/python

def part1(values: [[str]]):
    gamma = int(''.join(map(lambda lst: max(set(lst), key=lst.count), zip(*values))), 2)
    return gamma * (gamma ^ int(f"0b{'1' * gamma.bit_length()}", 2))


def get_sub_vals(values: [[str]], is_oxy: bool) -> int:
    max_l = lambda lst: '1' if lst.count('1') >= lst.count('0') else '0'
    min_l = lambda lst: '0' if lst.count('0') <= lst.count('1') else '1'

    if len(values) == 1:
        return int(values[0][0], 2)

    first_bits = [v[1][0] for v in values]
    bit = max_l(first_bits) if is_oxy else min_l(first_bits)
    new_values = [(v[0], v[1][1:]) for v in values if v[1][0] == bit]
    return get_sub_vals(new_values, is_oxy)


def part2(values: [[str]]):
    input = list(zip([''.join(v) for v in values], [list(v) for v in values]))
    return get_sub_vals(input, True) * get_sub_vals(input, False)


def main():
    values = [list(line[:-1]) for line in open("input3", 'r').readlines()]
    print(f"Part 1: {part1(values)}")
    print(f"Part 2: {part2(values)}")


if __name__ == "__main__":
    main()
