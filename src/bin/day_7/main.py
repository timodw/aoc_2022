def create_new_file(name, size):
    return {
        "name": name,
        "size": size,
        "type": "FILE"
    }


def create_new_dir(name, parent):
    return {
        "name": name,
        "contents": [],
        "size": 0,
        "parent": parent,
        "type": "DIR"
    }


def calculate_sizes(dir_tree):
    for c in dir_tree["contents"]:
        dir_tree["size"] += c["size"]
        if c["type"] == "DIR":
            dir_tree["size"] += calculate_sizes(c)
    return dir_tree["size"]


def get_size_sum(dir, size=0, threshold=100_000):
    if dir["size"] <= threshold:
        size += dir["size"]
    for d in dir["contents"]:
        if d["type"] == "DIR":
            size += get_size_sum(d)
    return size


def build_fs(input_list):
    root_dir = create_new_dir("/", None)
    cur_dir = root_dir
    for stdout in input_list[1:]:
        if stdout == "$ cd ..":
            cur_dir = cur_dir["parent"]
        elif stdout == "$ cd /":
            cur_dir = root_dir
        elif stdout.startswith("$ cd"):
            new_dir = create_new_dir(stdout[4:], parent=cur_dir)
            cur_dir["contents"].append(new_dir)
            cur_dir = new_dir
        elif stdout != "$ ls":
            if not stdout.startswith("dir"):
                size, name = stdout.split(" ")
                cur_dir["contents"].append(create_new_file(name, int(size)))
    calculate_sizes(root_dir)
    return root_dir


def get_dir_sizes_to_remove(dir, threshold, sizes=None):
    if sizes is None:
        sizes = []
    if dir["size"] >= threshold:
        sizes.append(dir["size"])
    for d in dir["contents"]:
        if d["type"] == "DIR":
            get_dir_sizes_to_remove(d, threshold=threshold, sizes=sizes)
    return sizes


def part_1(input_list):
    root_dir = build_fs(input_list)
    return get_size_sum(root_dir)


def part_2(input_list):
    root_dir = build_fs(input_list)
    free_space = 70_000_000 - root_dir["size"]
    space_needed = 30_000_000 - free_space
    return sorted(get_dir_sizes_to_remove(root_dir, space_needed))[0]


if __name__ == "__main__":
    input_data = open("src/bin/day_7/full.input", "r")
    input_list = [l.strip() for l in input_data.readlines()]
    print("Part 1:", part_1(input_list))
    print("Part 2:", part_2(input_list))