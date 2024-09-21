# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def helper(self, root, visited):
        if not root:
            return

        self.helper(root.left, visited)
        visited.append(root.val)
        self.helper(root.right, visited)

    def isValidBST(self, root):
        ret = []

        self.helper(root, ret)

        if all(ret[i] < ret[i + 1] for i in range(len(ret) - 1)):
            return True

        return False

        
if __name__ == '__main__':
    solution = Solution()

    root = TreeNode(
        5,
        TreeNode(
            4,
        ),
        TreeNode(
            6,
            TreeNode(
                3,
            ),
            TreeNode(
                7,
            ),
        ),
    )

    print(solution.isValidBST(root))


