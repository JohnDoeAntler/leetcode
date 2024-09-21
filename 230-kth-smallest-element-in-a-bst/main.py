# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def helper(self, root, visited, k):
        if not root:
            return

        if len(visited) == k:
            return

        self.helper(root.left, visited, k)
        visited.append(root.val)
        self.helper(root.right, visited, k)

    def kthSmallest(self, root, k: int):
        visited = []
        self.helper(root, visited, k)
        return visited[k - 1]

if __name__ == '__main__':
    root = TreeNode(3)
    root.left = TreeNode(1)
    root.right = TreeNode(4)
    root.left.right = TreeNode(2)
    print(Solution().kthSmallest(root, 4))
        
