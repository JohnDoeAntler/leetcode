# Definition for singly-linked list.
class ListNode:
    def __init__(self, x):
        self.val = x
        self.next: ListNode | None = None

class Solution:
    def hasCycle(self, head):
        slow = head
        fast = head

        # if fast none => false
        # if slow none => impossible but false
        # if fast.next none => false
        # if fast is slow

        while fast and slow and fast and fast.next:
            slow = slow.next
            fast = fast.next.next

            if fast is slow:
                return True
        else:
            return False

if __name__ == '__main__':
    head = ListNode(3)
    head.next = ListNode(2)
    # head.next.next = ListNode(0)
    # head.next.next.next = ListNode(-4)
    # head.next.next.next.next = head.next

    s = Solution()
    print(s.hasCycle(head))
        
