class Solution:
    def copyRandomList(self, head: 'Node') -> 'Node':
        old_to_new = {None: None}
        node = head
        while node != None:
            new_node = Node(node.val)
            old_to_new[node] = new_node
            node = node.next
        
        node = head
        while node != None:
            new_node = old_to_new[node]
            new_node.next = old_to_new[node.next]
            new_node.random = old_to_new[node.random]
            node = node.next
            
        return old_to_new[head]
