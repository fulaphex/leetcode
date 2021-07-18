class Solution:
    def is_mirror(self, x: TreeNode, y: TreeNode) -> bool:
        if x is None and y is None:
            return True
        if (x is None) != (y is None):
            return False
        if x.val != y.val:
            return False
        
        return self.is_mirror(x.left, y.right) and self.is_mirror(x.right, y.left)
        
    def isSymmetric(self, root: TreeNode) -> bool:
        if (root.left is None) != (root.right is None):
            return False
        if root.left is None and root.right is None:
            return True
        return self.is_mirror(root.left, root.right)
