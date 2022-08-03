class Solution:
    def verticalTraversal(self, root: Optional[TreeNode]) -> List[List[int]]:
        non_neg_col = []
        neg_col = []
        
        row = [(0, root)]
        while row:
            next_row = []
            nnc_updates = [[] for i in range(len(non_neg_col))]
            nc_updates = [[] for i in range(len(neg_col))]
            for col, node in row:
                if node is None:
                    continue
                
                if col >= 0:
                    if col == len(non_neg_col):
                        nnc_updates.append([node.val])
                        non_neg_col.append([])
                    else:
                        nnc_updates[col].append(node.val)
                else:
                    if -(1 + col) == len(neg_col):
                        nc_updates.append([node.val])
                        neg_col.append([])
                    else:
                        nc_updates[-(1 + col)].append(node.val)
                next_row.append((col-1, node.left))
                next_row.append((col+1, node.right))
            
            for col, up in zip(non_neg_col, nnc_updates):
                col.extend(sorted(up))
            
            for col, up in zip(neg_col, nc_updates):
                col.extend(sorted(up))
            
            row = next_row
        
        neg_col.reverse()
        neg_col.extend(non_neg_col)
        return neg_col
                