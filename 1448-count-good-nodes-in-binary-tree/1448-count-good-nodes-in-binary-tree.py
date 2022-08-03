class Solution:
    def goodNodes(self, root: TreeNode) -> int:
        def count_good(root, hi):
            if root is None:
                return 0
            if root.val >= hi:
                return 1 + count_good(root.left, root.val) + count_good(root.right, root.val)
            else:
                return count_good(root.left, hi) + count_good(root.right, hi)
        return count_good(root, -10**4)