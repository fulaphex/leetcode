from __future__ import annotations
from dataclasses import dataclass
from typing import Optional


@dataclass
class Node:
    val: int
    next: Optional[Node] = None
    prev: Optional[Node] = None


class LinkedList:
    def __init__(self, capacity) -> None:
        self.capacity = capacity
        self.length = 0
        self.node_by_key = {}
        self.head = None
        self.tail = None

    def optional_remove_last(self) -> Optional[int]:
        # removes the last element if the list if too long and returns the removed element
        if self.length > self.capacity:
            rem_val = self.tail.val
            self.tail = self.tail.next
            self.tail.prev = None
            self.length -= 1
            assert self.length == self.capacity
            self.node_by_key.pop(rem_val)
            return rem_val

        return None

    def push_front(self, val: int) -> Optional[int]:
        if self.capacity == 0:
            # add and remove at the same time
            return val
        # adds the element to the front of the list and optionally returns the element that was removed from the list
        if val in self.node_by_key:
            if self.head.val == val:
                # element is already first, not doing anything
                return
            # find the element
            node = self.node_by_key[val]
            prev_node, next_node = node.prev, node.next

            # remove it from list and fix links from neighbours
            if prev_node is not None:
                prev_node.next = next_node
            else:
                # prev is none, so the element is the tail
                # need to set the new tail, next_node
                self.tail = next_node

            if next_node is not None:
                next_node.prev = prev_node

            # fix the old head and new head pointers
            self.head.next = node
            node.prev = self.head
            node.next = None
            # put it in the front
            self.head = node
            # length isn't changed, so no need to remove anything
            return
        else:
            if self.head is None:
                # list is empty, so prev and next pointers are empty
                self.node_by_key[val] = self.tail = self.head = Node(val)
                self.length += 1
                # return self.optional_remove_last()
                return
            else:
                # creating with the prev pointer, next is empty because it will be the new head
                node = Node(val, prev=self.head)
                # adding to lookup table
                self.node_by_key[val] = node
                # setting the next pointer
                self.head.next = node
                # setting the new head
                self.head = node
                self.length += 1
                return self.optional_remove_last()


class LRUCache:
    def __init__(self, capacity: int) -> None:
        self.linked_list = LinkedList(capacity)
        self.lookup: dict[int, int] = {}

    def get(self, key: int) -> int:
        if key in self.lookup:
            rem = self.linked_list.push_front(key)
            if rem is not None:
                self.lookup.pop(rem)
            return self.lookup[key]
        return -1

    def put(self, key: int, value: int) -> None:
        self.lookup[key] = value
        rem = self.linked_list.push_front(key)
        if rem is not None:
            self.lookup.pop(rem)

# Your LRUCache object will be instantiated and called as such:
# obj = LRUCache(capacity)
# param_1 = obj.get(key)
# obj.put(key,value)