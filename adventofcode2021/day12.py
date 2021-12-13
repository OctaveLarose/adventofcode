#!/usr/bin/python
from collections import Counter


def get_connected_rooms(room, prev_path, conns, max_small_visits):
    connected_rooms = []
    for c in conns:
        if room not in c:
            continue

        conn_room = c[0] if c[1] == room else c[1]

        if conn_room == "start":
            continue

        if conn_room.islower():
            if max_small_visits == 1 and conn_room in prev_path:
                continue
            else:
                if conn_room in prev_path:
                    c = Counter(prev_path)
                    if any([a.islower() and b >= max_small_visits for a, b in c.items()]):
                        continue

        connected_rooms.append(conn_room)
    return connected_rooms


def navigate(room, prev_path, conns, paths, max_small_visits):
    if room == "end":
        return prev_path + ["end"]
    for cr in get_connected_rooms(room, prev_path, conns, max_small_visits):
        n = navigate(cr, prev_path + [room], conns, paths, max_small_visits)
        # if not any([isinstance(v, list) for v in n]):
        paths.append(n)
    return paths


def navigator(conns, max_small_visits=1):
    paths = navigate("start", [], conns, [], max_small_visits)
    paths = list(filter(None, map(lambda l: None if any([type(x) is list for x in l]) else l, paths)))
    # print(paths)
    return len(paths)


def main():
    conns = [tuple(v.strip().split("-")) for v in open("inputs/input12", 'r').readlines()]

    print("Part 1:", navigator(conns))
    print("Part 2:", navigator(conns, 2))


if __name__ == "__main__":
    main()
