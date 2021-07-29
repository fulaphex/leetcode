class Solution:
    def hasCycle(self, head: ListNode) -> bool:
        if head is None or head.next is None or head.next.next is None:
            return False
        
        slow, fast = head.next, head.next.next
        while fast != slow:
            if fast.next is None or fast.next.next is None:
                return False
            slow = slow.next
            fast = fast.next.next
        return True
