#!/usr/bin/python

def get_adjacent(coord, cavern):
    x, y = coord
    coords = [(x, y - 1), (x - 1, y), (x, y + 1), (x + 1, y)]
    return list(filter(lambda t: 0 <= t[0] < len(cavern[0]) and 0 <= t[1] < len(cavern[0]), [(x1, y1) for x1, y1 in coords]))


def navigate(current_coord, cavern):
    adjs = get_adjacent(current_coord, cavern)
    cx, cy = current_coord
    for (x, y) in adjs:
        val, cost = cavern[y][x]
        if val is None or val > cavern[cy][cx][0] + cost:
            cavern[y][x][0] = cavern[cy][cx][0] + cost
        else:
            continue
        print(*cavern, sep='\n', end='\n\n')
        navigate((x, y), cavern)


def part1(cavern):
    navigate((0, 0), cavern)
    return cavern[-1][-1][0] - cavern[0][0][1]


def main():
    cavern = [[[None, int(x)] for x in l.strip()] for l in open("inputs/input15", "r").readlines()]
    cavern[0][0][0] = cavern[0][0][1]

    print("Part 1:", part1(cavern))


if __name__ == "__main__":
    main()
