import functools
# from rich.pretty import pprint

MAX = 1e30


def get_bitmasks(length: int) -> list[bool]:
    if length == 0:
        return [[]]
    return [[True] + suff for suff in get_bitmasks(length - 1)] + [[False] + suff for suff in get_bitmasks(length - 1)]


def debug_arr(n: int, **kwargs: dict[str, int]):
    arr = list(range(n))
    for k, v in kwargs.items():
        arr[v] = k
    return arr


class Solution:
    @functools.cache
    def earliest(self, n: int, first: int, second: int) -> int:
        # print()
        # print(f"computing for {n = }, {first = }, {second = }")
        # arr = list(range(n))
        # arr[first] = "first"
        # arr[second] = "second"
        # pprint(arr)

        # if first > second:
        #     return self.earliest(n, second, first)

        before_first = first
        after_second = n - second - 1
        # print(f"{before_first = }, {after_second = }")
        if before_first == after_second:
            return 1

        # if before_first > after_second:
        #     return self.earliest(n, after_second, n - before_first - 1)

        bitmasks = get_bitmasks(n // 2)
        if n % 2:  # odd
            full_bitmasks = [bitmask + [True] + [not val for val in bitmask[::-1]] for bitmask in bitmasks]
        else:
            full_bitmasks = [bitmask + [not val for val in bitmask[::-1]] for bitmask in bitmasks]
        full_bitmasks = [mask for mask in full_bitmasks if mask[first] and mask[second]]
        res = MAX
        # pprint(full_bitmasks)
        for mask in full_bitmasks:
            new_n = (n + 1) // 2
            acc = 0
            for idx, v in enumerate(mask):
                if idx == first:
                    new_first = acc
                if idx == second:
                    new_second = acc
                acc += int(v)
            # pprint(mask)
            # pprint(arr)
            # pprint([x for idx, x in enumerate(arr) if mask[idx]])
            # print(new_first, new_second)
            res = min(res, 1 + self.earliest(new_n, new_first, new_second))
        return res

    @functools.cache
    def earliest_fast(self, n: int, first: int, second: int):
        # print()
        # print(f"computing for {n = }, {first = }, {second = }")
        # # arr = list(range(n))
        # # arr[first] = "first"
        # # arr[second] = "second"
        # arr = debug_arr(n, first=first, second=second)
        # pprint(arr)

        if first > second:
            return self.earliest_fast(n, second, first)

        before_first = first
        after_second = n - second - 1
        # print(f"{before_first = }, {after_second = }")
        if before_first == after_second:
            return 1

        if before_first > after_second:
            return self.earliest_fast(n, after_second, n - before_first - 1)

        new_n = (n + 1) // 2
        res = MAX
        if second >= new_n:
            mirrored_second = after_second
            # arr = debug_arr(n, first=first, second=second, mirrored_second=mirrored_second)
            # pprint(arr)
            between_first_and_mirrored = mirrored_second - first - 1
            for x in range(before_first + 1):
                for y in range(between_first_and_mirrored + 1):
                    # print(f"{(x, y) = }")
                    new_first = first - x
                    new_second = (
                        second
                        - (
                            (second - mirrored_second - 1) // 2
                        )  # between mirrored_second and second half the players are guaranteed to be eliminated
                        - 1  # mirrored_second is eliminated
                        - x  # same players that are eliminated before first
                        - y  # players that are eliminated between first and mirrored_second
                    )
                    res = min(res, 1 + self.earliest_fast(new_n, new_first, new_second))
            return res
            ...
        else:
            # arr = debug_arr(n, first=first, second=second)
            # pprint(arr)
            between_first_and_second = second - first - 1
            for x in range(before_first + 1):
                for y in range(between_first_and_second + 1):
                    # print(f"{(x, y) = }")
                    new_first = first - x
                    new_second = (
                        second
                        - x  # same players that are eliminated before first
                        - y  # players that are eliminated between first and mirrored_second
                    )
                    res = min(res, 1 + self.earliest_fast(new_n, new_first, new_second))
            return res

    @functools.cache
    def latest_fast(self, n: int, first: int, second: int):
        # print()
        # print(f"computing for {n = }, {first = }, {second = }")
        # # arr = list(range(n))
        # # arr[first] = "first"
        # # arr[second] = "second"
        # arr = debug_arr(n, first=first, second=second)
        # pprint(arr)

        if first > second:
            return self.latest_fast(n, second, first)

        before_first = first
        after_second = n - second - 1
        # print(f"{before_first = }, {after_second = }")
        if before_first == after_second:
            return 1

        if before_first > after_second:
            return self.latest_fast(n, after_second, n - before_first - 1)

        new_n = (n + 1) // 2
        res = -1
        if second >= new_n:
            mirrored_second = after_second
            # arr = debug_arr(n, first=first, second=second, mirrored_second=mirrored_second)
            # pprint(arr)
            between_first_and_mirrored = mirrored_second - first - 1
            for x in range(before_first + 1):
                for y in range(between_first_and_mirrored + 1):
                    # print(f"{(x, y) = }")
                    new_first = first - x
                    new_second = (
                        second
                        - (
                            (second - mirrored_second - 1) // 2
                        )  # between mirrored_second and second half the players are guaranteed to be eliminated
                        - 1  # mirrored_second is eliminated
                        - x  # same players that are eliminated before first
                        - y  # players that are eliminated between first and mirrored_second
                    )
                    res = max(res, 1 + self.latest_fast(new_n, new_first, new_second))
        else:
            # arr = debug_arr(n, first=first, second=second)
            # pprint(arr)
            between_first_and_second = second - first - 1
            for x in range(before_first + 1):
                for y in range(between_first_and_second + 1):
                    # print(f"{(x, y) = }")
                    new_first = first - x
                    new_second = (
                        second
                        - x  # same players that are eliminated before first
                        - y  # players that are eliminated between first and mirrored_second
                    )
                    res = max(res, 1 + self.latest_fast(new_n, new_first, new_second))
        return res

    @functools.cache
    def latest(self, n: int, first: int, second: int) -> int:
        # print()
        # print(f"computing for {n = }, {first = }, {second = }")
        # arr = list(range(n))
        # arr[first] = "first"
        # arr[second] = "second"
        # pprint(arr)

        before_first = first
        after_second = n - second - 1
        # print(f"{before_first = }, {after_second = }")
        if before_first == after_second:
            return 1

        bitmasks = get_bitmasks(n // 2)
        if n % 2:  # odd
            full_bitmasks = [bitmask + [True] + [not val for val in bitmask[::-1]] for bitmask in bitmasks]
        else:
            full_bitmasks = [bitmask + [not val for val in bitmask[::-1]] for bitmask in bitmasks]
        full_bitmasks = [mask for mask in full_bitmasks if mask[first] and mask[second]]
        res = -1
        # pprint(full_bitmasks)
        for mask in full_bitmasks:
            new_n = (n + 1) // 2
            acc = 0
            for idx, v in enumerate(mask):
                if idx == first:
                    new_first = acc
                if idx == second:
                    new_second = acc
                acc += int(v)

            # pprint(mask)
            # pprint(arr)
            # pprint([x for idx, x in enumerate(arr) if mask[idx]])
            # print(new_first, new_second)

            res = max(res, 1 + self.latest(new_n, new_first, new_second))

        return res

    def earliestAndLatest(self, n: int, firstPlayer: int, secondPlayer: int) -> list[int]:
        earliest = self.earliest(n, firstPlayer - 1, secondPlayer - 1)
        latest = self.latest(n, firstPlayer - 1, secondPlayer - 1)
        return earliest, latest


def test_get_bitmasks():
    assert get_bitmasks(3) == [
        [True, True, True],
        [True, True, False],
        [True, False, True],
        [True, False, False],
        [False, True, True],
        [False, True, False],
        [False, False, True],
        [False, False, False],
    ]


def test():
    n, first, second = 11, 2, 4
    res = 3, 4
    assert Solution().earliest(n, first - 1, second - 1) == res[0]
    assert Solution().earliest_fast(n, first - 1, second - 1) == res[0]
    assert Solution().latest(n, first - 1, second - 1) == res[1]
    assert Solution().latest_fast(n, first - 1, second - 1) == res[1]
    assert Solution().earliestAndLatest(n, first, second) == res


def test2():
    n, first, second = 5, 1, 5
    res = 1, 1
    assert Solution().earliest(n, first - 1, second - 1) == res[0]
    assert Solution().earliest_fast(n, first - 1, second - 1) == res[0]
    assert Solution().latest(n, first - 1, second - 1) == res[1]
    assert Solution().latest_fast(n, first - 1, second - 1) == res[1]
    assert Solution().earliestAndLatest(n, first, second) == res


def test3():
    n, first, second = 9, 4, 9
    res = 2, 4
    assert Solution().earliest(n, first - 1, second - 1) == res[0]
    assert Solution().earliest_fast(n, first - 1, second - 1) == res[0]
    assert Solution().latest(n, first - 1, second - 1) == res[1]
    assert Solution().latest_fast(n, first - 1, second - 1) == res[1]
    assert Solution().earliestAndLatest(n, first, second) == res
