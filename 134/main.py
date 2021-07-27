import inspect
import json
from pathlib import Path


class Solution:
    def canCompleteCircuit(self, gas: list[int], cost: list[int]) -> int:
        l_ = len(gas)
        start, idx, cnt = 0, 0, 0
        tank = 0
        while True:
            print(start, idx, cnt, tank)
            if start >= l_:
                return -1
            if cnt >= l_:
                break
            if tank + gas[idx % l_] - cost[idx % l_] >= 0:
                tank += gas[idx % l_] - cost[idx % l_]
                idx += 1
                cnt += 1
            else:
                tank = 0
                cnt = 0
                start = idx+1
                idx = idx+1
        return start if start < len(gas) else -1

        

def main():
    files = Path(".").glob("*.json")

    for fname in files:
        input_ = json.load(open(fname))
        res = input_.pop("out")

        _, fn = list(filter(
            lambda x: not x[0].startswith("_"),
            inspect.getmembers(Solution(), predicate=inspect.ismethod)
        ))[0]
        out = fn(**input_)
        assert out == res, (out, res)


if __name__ == "__main__":
    main()

