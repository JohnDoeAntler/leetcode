from typing import Optional

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

import math

class Solution:
    def isBalanced(self, root: Optional[TreeNode]) -> bool:
        if not root:
            return True

        def helper(root):
            if not root:
                return 0

            l_height = helper(root.left)
            r_height = helper(root.right)
            
            if l_height == -1 or r_height == -1:
                return -1

            if abs(l_height - r_height) > 1:
                return -1

            return 1 + max(l_height, r_height)

        return helper(root) != -1
        
if __name__ == "__main__":
    solution = Solution()

    # node3 = TreeNode(3)
    # node9 = TreeNode(9)
    # node20 = TreeNode(20)
    # node15 = TreeNode(15)
    # node7 = TreeNode(7)

    # node3.left = node9
    # node3.right = node20
    # node20.left = node15
    # node20.right = node7

    node1 = TreeNode(1)
    node2 = TreeNode(2)
    node3 = TreeNode(3)
    node4 = TreeNode(4)
    node5 = TreeNode(5)
    node6 = TreeNode(6)
    node8 = TreeNode(8)

    node1.left = node2
    node1.right = node3

    node2.left = node4
    node2.right = node5

    node3.right = node6
    node3.right = None

    node4.left = node8

    print(solution.isBalanced(node3))
