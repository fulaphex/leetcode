class Node:
    val: str
    children: dict[str, "Node"]

    def __init__(self, val):
        self.val = val
        self.children = {}
        self._hash = None
        self.terminal = False

    def add(self, path: list[str]) -> None:
        self._hash = None
        if not path:
            self.terminal = True
            return

        el = path[0]
        if el not in self.children:
            self.children[el] = Node(el)
        child = self.children[el]
        child.add(path[1:])

    def remove(self, nodes) -> None:
        # print(f"calling remove in {self.val}")
        # remove path and all subpaths
        # invalidating the hash
        self._hash = None
        for key, val in list(self.children.items()):
            if val in nodes:
                # print(f"removing {key}: {val.val}")
                self.children.pop(key)
            else:
                val.remove(nodes)

    def _get_paths(self, cur_path: list[str], paths: list[list[str]]):
        cur_path.append(self.val)
        if self.terminal:
            paths.append(list(cur_path))

        for c in self.children.values():
            c._get_paths(cur_path, paths)

        cur_path.pop()

    def get_all_paths(self):
        res = []
        self._get_paths([], res)
        res = [x[1:] for x in res if x[1:]]
        return res

    def get_hash(self) -> int:
        if self._hash:
            return self._hash

        if not self.children:
            self._hash = hash(self.val)
        else:
            self._hash = hash(
                tuple((k, c.get_hash()) for k, c in sorted(self.children.items()))
            )

        return self._hash

    def __repr__(self):
        return f"Node: {self.val}"


def find_dups(node: Node, hashes: dict[int, list[Node]], ind=0):
    hashval = node.get_hash()
    if hashval not in hashes:
        hashes[hashval] = [node]
        for k, c in sorted(node.children.items()):
            find_dups(c, hashes, ind + 1)
    else:
        hashes[hashval].append(node)


class Solution:
    def deleteDuplicateFolder(self, paths: list[list[str]]) -> list[list[str]]:
        root = Node(val="__root__")
        node = root
        for path in paths:
            root.add(path)

        hashes = {}
        find_dups(root, hashes)

        to_remove = []
        for val in hashes.values():
            if len(val) > 1 and all([len(n.children) for n in val]):
                to_remove.extend(val)
                # print(val)
        root.remove(set(to_remove))

        paths = root.get_all_paths()

        return paths


def test():
    paths = [["a"], ["c"], ["d"], ["a", "b"], ["c", "b"], ["d", "a"]]
    res = [["d"], ["d", "a"]]
    out = sorted(Solution().deleteDuplicateFolder(paths))

    assert res == out


def test2():
    paths = [
        ["a"],
        ["c"],
        ["a", "b"],
        ["c", "b"],
        ["a", "b", "x"],
        ["a", "b", "x", "y"],
        ["w"],
        ["w", "y"],
    ]
    res = [['a'], ['a', 'b'], ['c'], ['c', 'b']]
    out = sorted(Solution().deleteDuplicateFolder(paths))

    assert res == out


def test3():
    paths = [["a", "b"], ["c", "d"], ["c"], ["a"]]
    res = [['a'], ['a', 'b'], ['c'], ['c', 'd']]
    out = sorted(Solution().deleteDuplicateFolder(paths))

    assert res == out


def test4():
    paths = [
        ["c"],
        ["b"],
        ["p"],
        ["c", "a", "b", "a"],
        ["c", "a", "b"],
        ["c", "a"],
        ["b", "a"],
        ["b", "a", "a"],
        ["p", "a"],
        ["p", "a", "a"],
    ]
    res = [['c'], ['c', 'a']]
    out = sorted(Solution().deleteDuplicateFolder(paths))

    assert res == out


def test5():
    paths = [
        ["a"],
        ["a", "x"],
        ["a", "x", "y"],
        ["a", "z"],
        ["b"],
        ["b", "x"],
        ["b", "x", "y"],
        ["b", "z"],
        ["b", "w"],
    ]
    res = [['a'], ['a', 'z'], ['b'], ['b', 'w'], ['b', 'z']]
    out = sorted(Solution().deleteDuplicateFolder(paths))
    
    assert res == out
