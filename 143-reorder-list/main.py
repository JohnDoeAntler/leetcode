from typing import Optional
from collections import deque

class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

def print_the_list(l):
    current = l

    while current:
        print(current.val, end=', ')
        current = current.next

    return


class Solution:
    def reorderList(self, head) -> None:
        if not head:
            return

        queue = deque()
        current = head

        while current:
            queue.append(current)
            current = current.next

        flag = True
        current = queue.popleft()

        while len(queue):
            if flag:
                tmp = queue.pop()
                current.next = tmp
                current = tmp
                current.next = None
                flag = False
            else:
                tmp = queue.popleft()
                current.next = tmp
                current = tmp
                current.next = None
                flag = True

if __name__ == '__main__':
    solution = Solution()
    l = ListNode(
        1,
        ListNode(
            2,
            ListNode(
                3,
                ListNode(
                    4,
                    ListNode(
                        5,
                        None
                    )
                )
            )
        )
    )

    solution.reorderList(l)
    print_the_list(l)


