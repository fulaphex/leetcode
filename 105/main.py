class Solution:
    def build_tree(
        self, 
        preorder: List[int], 
        x: int, y: int,
        inorder: List[int], 
        x2: int, y2: int,
        lookup_inorder_idx: dict[int, int]
    ) -> Optional[TreeNode]:
        
        if x == y:
            return None
        
        node = TreeNode(preorder[x])
        
        idx = lookup_inorder_idx[node.val]
        cnt = idx - x2
        
        node.left = self.build_tree(preorder, x+1, x+1+cnt, inorder, x2, idx, lookup_inorder_idx)
        
        node.right = self.build_tree(preorder, x+1+cnt, y, inorder, idx+1, y2, lookup_inorder_idx)
        
        return node
        
        
    def buildTree(self, preorder: List[int], inorder: List[int]) -> TreeNode:
        lookup_inorder_idx = {
            el: idx
            for idx, el in enumerate(inorder)
        }
        return self.build_tree(preorder, 0, len(preorder), inorder, 0, len(inorder), lookup_inorder_idx)
