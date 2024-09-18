from pathlib import Path


def main():
    print("PART 1")
    print("------")
    part_1("example.txt")
    part_1("input.txt")
    print()
    print("PART 2")
    print("------")
    part_2("example.txt")
    part_2("input.txt")


def part_1(path: str):
    histories = Path(path).read_text().splitlines()
    next_values = [get_next_value(h, last=True) for h in histories]
    result = sum(next_values)
    print(f"{path}: {result}")


def get_next_value(history: str, *, last: bool) -> int:
    values = [int(value) for value in history.split()]
    levels = [values]
    while any(levels[-1]):
        pairs = [(a, b) for (a, b) in zip(values, values[1:])]
        values = [(b - a) for (a, b) in pairs]
        levels.append(values)
    levels = list(reversed(levels))
    for bottom, top in zip(levels, levels[1:]):
        if last:
            new_value = bottom[-1] + top[-1]
            top.append(new_value)
        else:
            new_value = top[0] - bottom[0]
            top.insert(0, new_value)
    result = levels[-1][-1] if last else levels[-1][0]
    return result


def part_2(path: str):
    histories = Path(path).read_text().splitlines()
    next_values = [get_next_value(h, last=False) for h in histories]
    result = sum(next_values)
    print(f"{path}: {result}")


if __name__ == "__main__":
    main()
