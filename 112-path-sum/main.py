from typing import Optional

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

def array_to_tree(arr):
    if not len(arr):
        return None

    nodes = [TreeNode(e) for e in arr]

    for i, e in enumerate(nodes):
        left_idx = i * 2 + 1
        if left_idx >= len(nodes):
            break
        e.left = nodes[left_idx]

        right_idx = left_idx + 1
        if right_idx >= len(nodes):
            break
        e.right = nodes[right_idx]

    return nodes[0]

class Solution:
    def hasPathSum(self, root: Optional[TreeNode], targetSum: int) -> bool:
        if root is None:
            return False

        to_be = targetSum - root.val

        if to_be == 0 and root.left is None and root.right is None:
            return True

        if self.hasPathSum(root.left, to_be):
            return True

        if self.hasPathSum(root.right, to_be):
            return True

        return False

if __name__ == "__main__":
    solution = Solution()
    
    print(solution.hasPathSum(array_to_tree([5,4,8,11,None,13,4,7,2,None,None,None,1]), 22))
    print(solution.hasPathSum(array_to_tree([1]), 5))
        
