# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:

    def helper(self, root, height, visited):
        if not root:
            return

        if len(visited) <= height:
            visited.append([])

        visited[height].append(root.val)

        self.helper(root.left, height + 1, visited)
        self.helper(root.right, height + 1, visited)

    def levelOrder(self, root):
        ret = []

        self.helper(root, 0, ret)

        return ret
        
if __name__ == '__main__':
    solution = Solution()

    n = TreeNode(
        3,
        TreeNode(
            9
        ),
        TreeNode(
            20,
            TreeNode(
                15
            ),
            TreeNode(
                7
            )
        )
    )

    print(solution.levelOrder(n))

