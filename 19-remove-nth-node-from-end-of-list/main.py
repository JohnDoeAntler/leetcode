
# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

def print_the_list(l):
    current = l

    while current:
        print(current.val, end=', ')
        current = current.next

    print()
    return

class Solution:
    def removeNthFromEnd(self, head, n: int):
        count = 0

        if not head:
            return

        current = head

        while current:
            count += 1
            current = current.next

        if count == n:
            return head.next

        if n > count:
            return head

        # 5 - 2 => 3, -1 required
        target_idx = count - n - 1
        current = head

        for _ in range(target_idx):
            current = current.next

        if current.next:
            current.next = current.next.next
        else:
            current.next = None

        return head
        

if __name__ == '__main__':

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

    solution = Solution()
    print_the_list(solution.removeNthFromEnd(l, 2))

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

    
    print_the_list(solution.removeNthFromEnd(l, 1))


    l = ListNode(
        1,
        None
    )

    solution.removeNthFromEnd(l, 1)
    print_the_list(l)

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

    
    print_the_list(solution.removeNthFromEnd(l, 5))
    
