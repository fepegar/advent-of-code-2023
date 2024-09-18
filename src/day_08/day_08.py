from __future__ import annotations

import re
from enum import Enum
from itertools import cycle
from pathlib import Path


def main():
    part_1("example.txt")
    part_1("example2.txt")
    part_1("input.txt")


def part_1(path: str):
    lines = Path(path).read_text().splitlines()
    instructions_line, _, *network_lines = lines
    instructions = cycle(instructions_line)
    name_to_node = read_nodes(network_lines)

    current_node = name_to_node["AAA"]
    steps = 0
    for instruction in instructions:
        match instruction:
            case Instruction.LEFT:
                current_node = current_node.left
            case Instruction.RIGHT:
                current_node = current_node.right
            case _:
                raise ValueError(f"Unknown instruction {instruction}")
        steps += 1
        if current_node.name == "ZZZ":
            break
    print(f"{path}: {steps}")


def read_nodes(lines: list[str]) -> dict[str, Node]:
    name_to_node = {}
    pattern = re.compile(r"(\w+) = \((\w+), (\w+)\)")
    for line in lines:
        match = pattern.match(line)
        assert match is not None
        parent_name, left_name, right_name = match.groups()

        parent = get_or_add_node(parent_name, name_to_node)
        left = get_or_add_node(left_name, name_to_node)
        right = get_or_add_node(right_name, name_to_node)
        parent.left = left
        parent.right = right
    return name_to_node


def get_or_add_node(name: str, name_to_node: dict[str, Node]) -> Node:
    node = name_to_node.get(name)
    if node is None:
        node = Node(name)
        name_to_node[name] = node
    return node


class Node:
    def __init__(self, name: str):
        self.name = name
        self.left: Node
        self.right: Node

    def __repr__(self):
        return f"Node({self.name}, {self.left.name}, {self.right.name})"


class Instruction(str, Enum):
    LEFT = "L"
    RIGHT = "R"


if __name__ == "__main__":
    main()
