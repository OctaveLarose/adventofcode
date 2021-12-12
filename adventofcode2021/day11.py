#!/usr/bin/python
from pprint import pprint

GRID_SIZE = 10
NBR_STEPS = 100
THRESHOLD = 9


def get_adjacent_coords(idx):
    x, y = idx % GRID_SIZE, idx // GRID_SIZE
    adjs = [(x - 1, y - 1), (x, y - 1), (x + 1, y - 1), (x - 1, y), (x + 1, y), (x - 1, y + 1), (x, y + 1), (x + 1, y + 1)]
    adjs = filter(None, [adj if 0 <= adj[0] < GRID_SIZE and 0 <= adj[1] < GRID_SIZE else None for adj in adjs])
    return list(map(lambda c: c[1] * GRID_SIZE + c[0], adjs))


def handle_flashes(idx, flash_map):
    flash_map[idx][1] = True
    adj_c = get_adjacent_coords(idx)
    for c_idx in adj_c:
        flash_map[c_idx][0] += 1
        if not flash_map[c_idx][1] and flash_map[c_idx][0] > THRESHOLD:
            handle_flashes(c_idx, flash_map)


def flash_calculator(octopuses, first_sync_flash=False) -> int:
    nbr_flashes, i = 0, 0
    flash_map = [[o, False] for o in octopuses]

    while True:
        i += 1
        if not first_sync_flash:
            if i > NBR_STEPS:
                break

        flash_map = [[o + 1, False] for o, hf in flash_map]
        for idx in range(len(flash_map)):
            if flash_map[idx][0] > THRESHOLD and not flash_map[idx][1]:
                handle_flashes(idx, flash_map)

        step_nbr_flashes = [f[1] for f in flash_map].count(True)
        if first_sync_flash:
            if step_nbr_flashes == GRID_SIZE * GRID_SIZE:
                return i
        else:
            nbr_flashes += [f[1] for f in flash_map].count(True)

        flash_map = [(0, False) if o > THRESHOLD else (o, False) for o, hf in flash_map]

    return nbr_flashes


def main():
    octopuses = [int(v) for v in open("inputs/input11", 'r').read().replace("\n", "")]

    print("Part 1:", flash_calculator(octopuses))
    print("Part 2:", flash_calculator(octopuses, first_sync_flash=True))


if __name__ == "__main__":
    main()
