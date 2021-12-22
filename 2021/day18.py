#!/usr/bin/python
import ast

SHOULD_EXPLODE = 1
SHOULD_SPLIT = 2
NOTHING_NEEDED = 2


def get_magnitude(pair):
    if isinstance(pair, int):
        return pair
    return get_magnitude(pair[0]) * 3 + get_magnitude(pair[1]) * 2


def analyse_list(l, coords):
    if isinstance(l, int):
        return None, NOTHING_NEEDED

    # if isinstance(l[0], int) or isinstance(l[1], int):
    #     print(*coords, ',', len(coords), ':', l)

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


def handle_explode(pair, coord):
    target_nbr = pair
    for c in coord:
        target_nbr = target_nbr[c]
    return pair  # TODO


def handle_split(pair, coord):
    target_nbr = pair
    for c in coord[:-1]:
        target_nbr = target_nbr[c]
    if isinstance(target_nbr[0], int) and target_nbr[0] >= 10:
        target_nbr[0] = [target_nbr[0] // 2, target_nbr[0] // 2 if target_nbr[0] % 2 == 0 else target_nbr[0] // 2 + 1]
    else:
        target_nbr[1] = [target_nbr[1] // 2, target_nbr[1] // 2 if target_nbr[1] % 2 == 0 else target_nbr[1] // 2 + 1]
    return pair


def add(p1, p2):
    p = [p1] + [p2]
    c, res = analyse_list(p, [])
    while res != NOTHING_NEEDED:
        print(p)
        if res == SHOULD_EXPLODE:
            p = handle_explode(p, c)
        elif res == SHOULD_SPLIT:
            p = handle_split(p, c)
        c, res = analyse_list(p, [])
    return p


def tests():
    assert handle_explode([[[[[9, 8], 1], 2], 3], 4], [0, 0, 0, 0]) == [[[[0, 9], 2], 3], 4]
    assert handle_explode([7, [6, [5, [4, [3, 2]]]]], [1, 1, 1, 1]) == [7, [6, [5, [7, 0]]]]
    assert handle_explode([[3, [2, [1, [7, 3]]]], [6, [5, [4, [3, 2]]]]], [0, 1, 1, 1]) == [[3, [2, [8, 0]]], [9, [5, [4, [3, 2]]]]]
    assert handle_explode([[3, [2, [8, 0]]], [9, [5, [4, [3, 2]]]]], [1, 1, 1, 1]) == [[3, [2, [8, 0]]], [9, [5, [7, 0]]]]

    assert handle_split([[[[0, 7], 4], [15, [0, 13]]], [1, 1]], [0, 1, 0]) == [[[[0, 7], 4], [[7, 8], [0, 13]]], [1, 1]]
    assert handle_split([[[[0, 7], 4], [[7, 8], [0, 13]]], [1, 1]], [0, 1, 1, 1]) == [[[[0, 7], 4], [[7, 8], [0, [6, 7]]]], [1, 1]]


def main():
    # pairs = [ast.literal_eval(expr) for expr in open("inputs/input18").read().splitlines()]
    # print(*pairs, sep='\n')

    # pair = [[[[8, 7], [7, 7]], [[8, 6], [7, 7]]], [[[0, 7], [6, 6]], [8, 7]]]
    # print(get_magnitude(pair))

    tests()

    # pair = [[[[[9, 8], 1], 2], 3], 4]
    # pair = [[3, [2, [1, [7, 3]]]], [6, [5, [4, [3, 2]]]]]
    # pair = [[[[0, 7], 4], [15, [0, 13]]], [1, 1]]
    # print(analyse_list(pair, []))
    # print(add([[[[4, 3], 4], 4], [7, [[8, 4], 9]]], [1, 1]))


if __name__ == "__main__":
    main()
