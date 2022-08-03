class Solution:
    def flatten(self, root: Optional[TreeNode]) -> None:
        if not root:
            return
        
        root.left, root.right = root.right, root.left
        self.flatten(root.right)
        if not root.right:
            self.end = root
        end = self.end
        self.flatten(root.left)
        end.right = root.left
        root.left = None