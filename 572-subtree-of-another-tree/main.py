# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def isSameTree(self, tree1, tree2):
        if tree1 is None and tree2 is None:
            return True

        if tree1 is None or tree2 is None:
            return False

        if tree1.val == tree2.val:
            return self.isSameTree(tree1.left, tree2.left) and self.isSameTree(tree1.right, tree2.right)

        return False

    def isSubtree(self, root, subRoot):
        if not root and not subRoot:
            return True

        if not root or not subRoot:
            return False

        if root.val == subRoot.val and self.isSameTree(root, subRoot):
            return True

        return self.isSubtree(root.left, subRoot) or self.isSubtree(root.right, subRoot)
        
if __name__ == '__main__':
    solution = Solution()
    root = TreeNode(
        3,
                TreeNode(
                    1,
                    None,
                    None,
                ),
        TreeNode(
            2,
            None,
            None,
        ),
    )
    sub = TreeNode(
        3,
        TreeNode(
            1,
            None,
            None,
        ),
        TreeNode(
            2,
            None,
            None,
        ),
    )

    print(solution.isSubtree(root, sub))

