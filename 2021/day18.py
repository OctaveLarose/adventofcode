#!/usr/bin/python
import ast
import math

SHOULD_EXPLODE = 1
SHOULD_SPLIT = 2
NOTHING_NEEDED = 3


def get_magnitude(pair):
    if isinstance(pair, int):
        return pair
    return get_magnitude(pair[0]) * 3 + get_magnitude(pair[1]) * 2


def analyse_list(l, coords):
    if isinstance(l, int):
        return None, NOTHING_NEEDED

    if len(coords) >= 4:
        return coords, SHOULD_EXPLODE
    if isinstance(l[0], int) and l[0] >= 10:
        return coords + [0], SHOULD_SPLIT
    if isinstance(l[1], int) and l[1] >= 10:
        return coords + [1], SHOULD_SPLIT

    v0 = analyse_list(l[0], coords + [0])
    if v0 != (None, NOTHING_NEEDED):
        return v0
    v1 = analyse_list(l[1], coords + [1])
    if v1 != (None, NOTHING_NEEDED):
        return v1
    return None, NOTHING_NEEDED


def get_sublist_elem(pair, coord):
    for c in coord:
        pair = pair[c]
    return pair


def replace_sublist_elem(pair, coord, new_val):
    for c in coord[:-1]:
        pair = pair[c]
    pair[coord[-1]] = new_val


def handle_explode(pair, coord):
    idx_left = coord.copy() if not all([c == 0 for c in coord]) else None
    idx_right = coord.copy() if not all([c == 1 for c in coord]) else None

    if idx_left is not None:
        idx_left = idx_left[:len(idx_left) - idx_left[::-1].index(1) - 1] + [0]
        while isinstance(get_sublist_elem(pair, idx_left), list):
            idx_left += [1]

    if idx_right is not None:
        idx_right = idx_right[:len(idx_right) - idx_right[::-1].index(0) - 1] + [1]
        while isinstance(get_sublist_elem(pair, idx_right), list):
            idx_right += [0]

    x, y = get_sublist_elem(pair, coord)

    if idx_left is not None:
        replace_sublist_elem(pair, idx_left, x + get_sublist_elem(pair, idx_left))
    if idx_right is not None:
        replace_sublist_elem(pair, idx_right, y + get_sublist_elem(pair, idx_right))

    replace_sublist_elem(pair, coord, 0)

    return pair


def handle_split(pair, coord):
    target_nbr = get_sublist_elem(pair, coord)
    new_val = [target_nbr // 2, math.ceil(target_nbr / 2)]
    replace_sublist_elem(pair, coord, new_val)
    return pair


def add(p1, p2):
    p = [p1] + [p2]
    res = None
    while res != NOTHING_NEEDED:
        c, res = analyse_list(p, [])
        if res == SHOULD_EXPLODE:
            p = handle_explode(p, c)
        elif res == SHOULD_SPLIT:
            p = handle_split(p, c)
    return p


def run_tests():
    # assert handle_explode([[[[[9, 8], 1], 2], 3], 4], [0, 0, 0, 0]) == [[[[0, 9], 2], 3], 4]
    # assert handle_explode([7, [6, [5, [4, [3, 2]]]]], [1, 1, 1, 1]) == [7, [6, [5, [7, 0]]]]
    # assert handle_explode([[3, [2, [1, [7, 3]]]], [6, [5, [4, [3, 2]]]]], [0, 1, 1, 1]) == [[3, [2, [8, 0]]], [9, [5, [4, [3, 2]]]]]
    # assert handle_explode([[3, [2, [8, 0]]], [9, [5, [4, [3, 2]]]]], [1, 1, 1, 1]) == [[3, [2, [8, 0]]], [9, [5, [7, 0]]]]
    # assert handle_explode([[[[0, 7], 4], [7, [[8, 4], 9]]], [1, 1]], [0, 1, 1, 0]) == [[[[0, 7], 4], [15, [0, 13]]], [1, 1]]
    #
    # assert handle_split([[[[0, 7], 4], [15, [0, 13]]], [1, 1]], [0, 1, 0]) == [[[[0, 7], 4], [[7, 8], [0, 13]]], [1, 1]]
    # assert handle_split([[[[0, 7], 4], [[7, 8], [0, 13]]], [1, 1]], [0, 1, 1, 1]) == [[[[0, 7], 4], [[7, 8], [0, [6, 7]]]], [1, 1]]
    #
    # assert add([[[[4, 3], 4], 4], [7, [[8, 4], 9]]], [1, 1]) == [[[[0, 7], 4], [[7, 8], [6, 0]]], [8, 1]]
    assert add([[[0, [4, 5]], [0, 0]], [[[4, 5], [2, 6]], [9, 5]]], [7, [[[3, 7], [4, 3]], [[6, 3], [8, 8]]]]) == [
        [[[4, 0], [5, 4]], [[7, 7], [6, 0]]], [[8, [7, 7]], [[7, 9], [5, 0]]]]


def main():
    # pairs = [ast.literal_eval(expr) for expr in open("inputs/input18").read().splitlines()]
    # pairs = [ast.literal_eval(expr) for expr in open("inputs/tests/testinput18").read().splitlines()]

    run_tests()

    # s = pairs[0]
    # for p in pairs[1:]:
    #     print(s, '+++', p)
    #     s = add(s, p)
    #     print('=', s)
    #
    # print("Part 1:", get_magnitude(s))


if __name__ == "__main__":
    main()
