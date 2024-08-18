"""
# Definition for a Node.
"""

class Node:
    def __init__(self, val = 0, neighbors = None):
        self.val = val
        self.neighbors = neighbors if neighbors is not None else []

from typing import Optional

class Solution:
    def cloneGraph(self, node: Optional['Node']) -> Optional['Node']:
        def helper(node: Optional['Node'], m: dict[Node, Node]) -> Optional['Node']:
            if not node:
                return

            if node in m:
                return m[node]

            new_node = Node(node.val)
            m[node] = new_node

            for i in node.neighbors:
                m[node].neighbors.append(helper(i, m))

            return new_node

        m = dict()
        return helper(node, m)

if __name__ == '__main__':
    node1 = Node(1)
    node2 = Node(2)
    node3 = Node(3)
    node4 = Node(4)

    node1.neighbors = [node2, node4]
    node2.neighbors = [node1, node3]
    node3.neighbors = [node2, node4]
    node4.neighbors = [node1, node3]

    solution = Solution()

    print(solution.cloneGraph(node1))
