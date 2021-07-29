import inspect
import json
from pathlib import Path


class Solution:
    def wordBreak(self, s: str, wordDict: list[str]) -> list[str]:
        words = set(wordDict) or {""}
        dp = [[] for _ in range (len(s)+1)]
        dp[0] = [()]

        for idx in range(1, len(s)+1):
            for pref_idx in range(idx):
                rest = s[pref_idx:idx]
                if rest in words:
                    for split_ in dp[pref_idx]:
                        new_split_ = split_ + (rest,)
                        dp[idx].append(new_split_)

        return [" ".join(split_) for split_ in dp[-1]]


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
        assert sorted(out) == sorted(res), (out, res)


if __name__ == "__main__":
    main()

