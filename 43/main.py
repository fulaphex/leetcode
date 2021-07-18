import json
from collections import defaultdict
from pathlib import Path


class Solution:
    def multiply(self, num1: str, num2: str) -> str:
        if num1 == "0" or num2 == "0":
            return "0"
        digs: dict[int, list[int]] = defaultdict(list)  # (offset -> list[int])
        for pos1, dig1 in enumerate(num1[::-1]):
            for pos2, dig2 in enumerate(num2[::-1]):
                digs[pos1+pos2].append(int(dig1) * int(dig2))

        res: list[str] = []
        idx = 0
        while True:
            arr = digs[idx]
            if len(arr) == 0:
                break
            num = sum(arr)
            res.append(str(num % 10))
            if num >= 10:
                digs[idx+1].append(num // 10)
            idx += 1
        return "".join(res[::-1])


def main():
    files = Path(".").glob("*.json")
    for fname in files:
        input_ = json.load(open(fname))
        num1, num2 = input_["num1"], input_["num2"]
        out = Solution().multiply(num1, num2)
        res = input_["res"]
        # print(out)
        assert out == res, (out, res)


if __name__ == "__main__":
    main()
