class Solution:
    def sortedArrayToBST(self, nums: List[int]) -> TreeNode:
        if not nums:
            return None
        idx = len(nums) // 2
        return TreeNode(
            val = nums[idx], left = self.sortedArrayToBST(nums[:idx]), right = self.sortedArrayToBST(nums[idx+1:])
        )
