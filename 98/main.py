class Solution:
    def is_valid(self, root: Optional[TreeNode]) -> tuple[bool, Optional[int], Optional[int]]:
        if root is None:
            return True, None, None
        ret, min_left, max_left = self.is_valid(root.left)
        if not ret:
            return False, None, None
        if max_left and max_left >= root.val:
            return False, None, None
        
        ret, min_right, max_right = self.is_valid(root.right)
        if not ret:
            return False, None, None
        if min_right and min_right <= root.val:
            return False, None, None
        
        return True, min_left or root.val, max_right or root.val
        
    def isValidBST(self, root: Optional[TreeNode]) -> bool:
        return self.is_valid(root)[0]
