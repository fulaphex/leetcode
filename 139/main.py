import inspect
import json
from pathlib import Path


class Solution:
    def wordBreak(self, s: str, wordDict: list[str]) -> bool:
        words = set(wordDict) or {""}
        dp = [False] * (len(s)+1)
        dp[0] = True
        for idx in range(1, len(s)+1):
            for pref_idx in range(idx):
                rest = s[pref_idx:idx]
                dp[idx] = dp[idx] or (dp[pref_idx] and rest in words)
        return dp[-1]


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

