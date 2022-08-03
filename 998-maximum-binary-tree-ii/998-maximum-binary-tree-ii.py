class Solution:
    def insertIntoMaxTree(self, root: Optional[TreeNode], val: int) -> Optional[TreeNode]:
        head = TreeNode(-1, None, root)
        node = head
        while node.right and node.right.val > val:
            node = node.right
        node.right = TreeNode(val, node.right, None)
        return head.right