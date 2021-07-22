class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        return sum(
            max(prices[idx+1] - prices[idx], 0)
            for idx in range(len(prices)-1)
        )
