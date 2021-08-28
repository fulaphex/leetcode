class MinStack:

    def __init__(self):
        """
        initialize your data structure here.
        """
        self._stack = []
        self.st = []

    def push(self, val: int) -> None:
        self._stack.append(val)
        if len(self.st) == 0 or val <= self.st[-1]:
            self.st.append(val)
        

    def pop(self) -> None:
        if self._stack.pop() == self.st[-1]:
            self.st.pop()

    def top(self) -> int:
        return self._stack[-1]

    def getMin(self) -> int:
        return self.st[-1]
        


# Your MinStack object will be instantiated and called as such:
# obj = MinStack()
# obj.push(val)
# obj.pop()
# param_3 = obj.top()
# param_4 = obj.getMin()
