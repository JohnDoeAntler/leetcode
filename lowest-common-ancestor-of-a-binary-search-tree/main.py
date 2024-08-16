# Definition for a binary tree node.
class TreeNode:
    def __init__(self, x):
        self.val = x
        self.left: TreeNode | None = None
        self.right: TreeNode | None = None

class Solution:
    def lowestCommonAncestor(self, root, p, q):
        if min(p.val, q.val) <= root.val and root.val <= max(p.val, q.val):
            return root
        elif max(p.val, q.val) < root.val:
            return self.lowestCommonAncestor(root.left, p, q)
        else:
            return self.lowestCommonAncestor(root.right, p, q)

def test1():
    n_0 = TreeNode(0)
    n_1 = TreeNode(1)
    n_2 = TreeNode(2)
    n_3 = TreeNode(3)
    n_4 = TreeNode(4)
    n_5 = TreeNode(5)
    n_6 = TreeNode(6)
    n_7 = TreeNode(7)
    n_8 = TreeNode(8)
    n_9 = TreeNode(9)

    n_6.left = n_2
    n_6.right = n_8
    n_2.left = n_0
    n_2.right = n_4
    n_8.left = n_7
    n_8.right = n_9
    n_4.left = n_3
    n_4.right = n_5

    solution = Solution()
    print(solution.lowestCommonAncestor(n_6, 2, 8).val)

def test2():
    n_0 = TreeNode(0)
    n_1 = TreeNode(1)
    n_2 = TreeNode(2)
    n_3 = TreeNode(3)
    n_4 = TreeNode(4)
    n_5 = TreeNode(5)
    n_6 = TreeNode(6)
    n_7 = TreeNode(7)
    n_8 = TreeNode(8)
    n_9 = TreeNode(9)

    n_6.left = n_2
    n_6.right = n_8
    n_2.left = n_0
    n_2.right = n_4
    n_8.left = n_7
    n_8.right = n_9
    n_4.left = n_3
    n_4.right = n_5

    solution = Solution()
    print(solution.lowestCommonAncestor(n_6, 2, 4).val)

if __name__ == '__main__':
    test1()
    test2()
        
