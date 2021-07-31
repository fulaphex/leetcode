# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def sortList(self, head: ListNode) -> ListNode:
        if head is None:
            return head
        l = []
        n = head
        while n is not None:
            l.append(n)
            n = n.next
            
        l = sorted(l, key=lambda x: x.val)
        for idx in range(len(l)-1):
            l[idx].next = l[idx+1]
        l[-1].next = None
        return l[0]
 
# merge sort approach
class Solution2:
    def divide(self, head: ListNode) -> tuple[ListNode]:
        x = y = z = head
        while True:
            if z.next is None or z.next.next is None:
                break
            y = y.next
            z = z.next.next
        z = y.next
        y.next = None
        return x, z
        
    def merge(self, x: ListNode, y: ListNode) -> ListNode:
        if x is None:
            return y
        if y is None:
            return x
        
        x = self.merge(*self.divide(x))
        y = self.merge(*self.divide(y))
        
        head = og_head = ListNode(-100)
        
        while x is not None and y is not None:
            if x.val < y.val:
                head.next = x
                x = x.next
            else:
                head.next = y
                y = y.next
            head = head.next
        
        while x is not None:
            head.next = x
            x = x.next
            head = head.next
        
        while y is not None:
            head.next = y
            y = y.next
            head = head.next
            
        return og_head.next
        
    
    def sortList(self, head: ListNode) -> ListNode:
        if head is None:
            return head
        return self.merge(*self.divide(head))       
