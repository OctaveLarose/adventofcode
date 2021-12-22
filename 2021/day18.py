#!/usr/bin/python
import ast

SHOULD_EXPLODE = 1
SHOULD_SPLIT = 2
NOTHING_NEEDED = 2


def get_magnitude(pair):
    if isinstance(pair, int):
        return pair
    return get_magnitude(pair[0]) * 3 + get_magnitude(pair[1]) * 2


def visit_list(l, coords):
    if isinstance(l, int):
        return NOTHING_NEEDED

    # if isinstance(l[0], int) or isinstance(l[1], int):
    #     print(*coords, ',', len(coords), ':', l)

    if len(coords) >= 4:
        return coords, SHOULD_EXPLODE
    if any([isinstance(sl, int) and sl >= 10 for sl in l]):
        return coords, SHOULD_SPLIT

    v0 = visit_list(l[0], coords + [0])
    if v0 != NOTHING_NEEDED:
        return v0
    v1 = visit_list(l[1], coords + [1])
    if v1 != NOTHING_NEEDED:
        return v1
    return NOTHING_NEEDED


def main():
    # pairs = [ast.literal_eval(expr) for expr in open("inputs/input18").read().splitlines()]
    # print(*pairs, sep='\n')

    # pair = [[[[8, 7], [7, 7]], [[8, 6], [7, 7]]], [[[0, 7], [6, 6]], [8, 7]]]
    # print(get_magnitude(pair))

    # pair = [[[[[9, 8], 1], 2], 3], 4]
    # pair = [[3, [2, [1, [7, 3]]]], [6, [5, [4, [3, 2]]]]]
    pair = [[[[0, 7], 4], [15, [0, 13]]], [1, 1]]
    print(visit_list(pair, []))


if __name__ == "__main__":
    main()
