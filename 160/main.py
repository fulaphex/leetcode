# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def getIntersectionNode(self, headA: ListNode, headB: ListNode) -> ListNode:
        l1, l2 = 0, 0
        x, y = headA, headB
        
        while x is not None:
            x = x.next
            l1 += 1
        
        while y is not None:
            y = y.next
            l2 += 1
        
        if l2 > l1:
            headA, headB = headB, headA
            l1, l2 = l2, l1
            
        while l1 > l2:
            headA = headA.next
            l1 -= 1
        
        while headA is not None and headA != headB:
            headA, headB = headA.next, headB.next
        
        return headA
