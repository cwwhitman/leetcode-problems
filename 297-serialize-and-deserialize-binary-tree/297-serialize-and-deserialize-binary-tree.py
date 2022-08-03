class Codec:
    
    def __init__(self):
        self.parsed = 0

    def serialize(self, root):
        if root is None:
            return 'n'
        return f"{root.val},{self.serialize(root.left)},{self.serialize(root.right)}"

    def deserialize(self, data):
        self.data = data.split(',')
        return self.d(0)
    
    def d(self, i):
        self.parsed += 1
        if self.data[i] == 'n':
            return None
        node = TreeNode(self.data[i])
        node.left = self.d(i+1)
        node.right = self.d(self.parsed)
        return node