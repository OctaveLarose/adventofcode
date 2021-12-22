#!/usr/bin/python
import ast


def get_magnitude(pair):
    if isinstance(pair, int):
        return pair
    return get_magnitude(pair[0]) * 3 + get_magnitude(pair[1]) * 2


def main():
    pairs = [ast.literal_eval(expr) for expr in open("inputs/input18").read().splitlines()]
    # print(*pairs, sep='\n')

    pair = [[[[8, 7], [7, 7]], [[8, 6], [7, 7]]], [[[0, 7], [6, 6]], [8, 7]]]
    print(get_magnitude(pair))


if __name__ == "__main__":
    main()
