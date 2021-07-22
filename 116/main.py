"""
# Definition for a Node.
class Node:
    def __init__(self, val: int = 0, left: 'Node' = None, right: 'Node' = None, next: 'Node' = None):
        self.val = val
        self.left = left
        self.right = right
        self.next = next
"""

class Solution:
    def create_levels(self, root: 'Node', level: int, level_table: dict[int, list['Node']]) -> int:
        if root is None:
            return level
        l_depth = self.create_levels(root.left, level+1, level_table)
        level_table[level].append(root)
        r_depth = self.create_levels(root.right, level+1, level_table)
        return max(l_depth, r_depth)
        
        
    def connect(self, root: 'Node') -> 'Node':
        level_table = defaultdict(list)
        max_depth = self.create_levels(root, 0, level_table)
        for level in level_table.values():
            for idx in range(len(level)-1):
                level[idx].next = level[idx+1]
        return root
