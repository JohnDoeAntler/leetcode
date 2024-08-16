from typing import Optional, List


# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    # getting the height of the tree
    # first int is the height
    # second int is the max diameter it discovered
    def helper(self, root: Optional[TreeNode]) -> List[int]:
        if not root:
            return [0, 0]

        l = self.helper(root.left)
        r = self.helper(root.right)

        ret = [
            # the height
            1 + max(l[0], r[0]),
            # max(the diameter, the )
            max(l[0] + r[0], l[1], r[1]),
        ]

        return ret

    def diameterOfBinaryTree(self, root) -> int:
        return self.helper(root)[1]

if __name__ == '__main__':
    root = TreeNode(
        1,
        TreeNode(
            2,
            TreeNode(
                3,
                TreeNode(5),
                TreeNode(6),
            ),
            TreeNode(
                4,
                TreeNode(7),
                TreeNode(8),
            ),
        ),
        TreeNode(9),
    )

    solution = Solution()
    print(solution.diameterOfBinaryTree(root))

