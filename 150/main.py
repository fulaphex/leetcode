class Solution:
    def evalRPN(self, tokens: List[str]) -> int:
        stack = []
        for el in tokens:
            if el == "+":
                x, y = stack[-2:]
                stack = stack[:-2]
                stack.append(x+y)
            elif el == "-":
                x, y = stack[-2:]
                stack = stack[:-2]
                stack.append(x-y)
            elif el == "*":
                x, y = stack[-2:]
                stack = stack[:-2]
                stack.append(x*y)
            elif el == "/":
                x, y = stack[-2:]
                stack = stack[:-2]
                stack.append(int(x/y))
            else:
                stack.append(int(el))
        return stack[0]
