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
        idx_left[len(idx_left) - idx_left[::-1].index(1) - 1] = 0
    if idx_right is not None:
        idx_right[len(idx_right) - idx_right[::-1].index(0) - 1] = 1

    # print('Left:', idx_left, ':', get_sublist_elem(pair, idx_left) if idx_left is not None else 0)
    # print('Right:', idx_right, ':', get_sublist_elem(pair, idx_right) if idx_right is not None else 'N/A')

    x, y = get_sublist_elem(pair, coord)
    parent_pair = get_sublist_elem(pair, coord[:-1])
    # print(parent_pair)
    if coord[-1] == 0:
        replace_sublist_elem(pair, coord[:-1], [0, y + parent_pair[1]])
        if idx_left is not None:
            replace_sublist_elem(pair, idx_left, x + get_sublist_elem(pair, idx_left))
    else:
        replace_sublist_elem(pair, coord[:-1], [x + parent_pair[0], 0])
        if idx_right is not None:
            replace_sublist_elem(pair, idx_right, y + get_sublist_elem(pair, idx_right))

    return pair


def handle_split(pair, coord):
    target_nbr = get_sublist_elem(pair, coord)
    new_val = [target_nbr // 2, target_nbr // 2 if target_nbr % 2 == 0 else target_nbr // 2 + 1]
    replace_sublist_elem(pair, coord, new_val)
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
    # assert handle_explode([[3, [2, [1, [7, 3]]]], [6, [5, [4, [3, 2]]]]], [0, 1, 1, 1]) == [[3, [2, [8, 0]]], [9, [5, [4, [3, 2]]]]]
    # assert handle_explode([[3, [2, [8, 0]]], [9, [5, [4, [3, 2]]]]], [1, 1, 1, 1]) == [[3, [2, [8, 0]]], [9, [5, [7, 0]]]]

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
