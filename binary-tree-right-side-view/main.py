from typing import Optional, List

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def rightSideView(self, root: Optional[TreeNode]) -> List[int]:
        ret = []

        def helper(root, level):
            if not root:
                return

            if level == len(ret):
                ret.append([])

            ret[level].append(root.val)

            helper(root.left, level + 1)
            helper(root.right, level + 1)

        helper(root, 0)

        return [e[-1] for e in ret]

if __name__ == '__main__':
    solution = Solution()

    root = TreeNode(1)
    root.left = TreeNode(2)
    root.right = TreeNode(3)
    root.left.right = TreeNode(5)
    root.right.right = TreeNode(4)

    print(solution.rightSideView(root))
