import unittest
from collections import deque


class Node:
    def __init__(
        self,
        val: int = 0,
        left: "Node | None" = None,
        right: "Node | None" = None,
        next: "Node | None" = None,
    ):
        self.val = val
        self.left = left
        self.right = right
        self.next = next


class Solution:
    def dfs(
        self, node: "Node", depth: int, leftmost_per_depth: list["Node | None"]
    ) -> None:
        node.next = leftmost_per_depth[depth]

        if node.right:
            self.dfs(node.right, depth + 1, leftmost_per_depth)
        if node.left:
            self.dfs(node.left, depth + 1, leftmost_per_depth)

        leftmost_per_depth[depth] = node

    def connect(self, root: "Node | None") -> "Node | None":
        if root:
            self.dfs(root, depth=0, leftmost_per_depth=[None] * 6_000)
        return root


def create_tree(vals: list[int | None]) -> Node | None:
    if not vals:
        return
    assert vals[0] is not None
    root = Node(vals[0])
    que = deque()
    que.append(root)

    it = iter(vals[1:])
    while 1:
        curr = que.popleft()
        try:
            left_val = next(it)
            if left_val:
                left_node = Node(left_val)
                que.append(left_node)
                curr.left = left_node
        except StopIteration:
            break

        try:
            right_val = next(it)
            if right_val:
                right_node = Node(right_val)
                que.append(right_node)
                curr.right = right_node
        except StopIteration:
            break

    return root


def serialise(root: Node | None) -> list[Node | None]:
    res = []
    if root:
        res.append(root)
        res.extend(serialise(root.left))
        res.extend(serialise(root.right))

    return res


class Tests(unittest.TestCase):
    def test(self):
        vals = [1, 2, 3, 4, 5, None, 7]
        root = create_tree(vals)
        Solution().connect(root)
        out = serialise(root)

        assert [x.next.val if x and x.next else None for x in out] == [
            None,
            3,
            5,
            7,
            None,
            None,
        ]
