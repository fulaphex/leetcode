class Solution:
    def create_levels(self, root: TreeNode, level: int, level_table: dict[int, list[int]]) -> int:
        if root is None:
            return level
        l_depth = self.create_levels(root.left, level+1, level_table)
        level_table[level].append(root.val)
        r_depth = self.create_levels(root.right, level+1, level_table)
        return max(l_depth, r_depth)
        
    def zigzagLevelOrder(self, root: TreeNode) -> List[List[int]]:
        level_table = defaultdict(list)
        max_depth = self.create_levels(root, 0, level_table)
        return [
            level_table[i][::(-1 if i%2 else 1)]  # nasty python one-liner hackery
            for i in range(max_depth)
        ]
