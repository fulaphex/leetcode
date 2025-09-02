import random


class Solution:

    def __init__(self, radius: float, x_center: float, y_center: float):
        self.radius = radius
        self.x = x_center
        self.y = y_center
        

    def randPoint(self) -> list[float]:
        while True:
            x = random.uniform(-self.radius, self.radius)
            y = random.uniform(-self.radius, self.radius)
            if (x*x + y*y) <= (self.radius*self.radius):
                return [self.x + x, self.y + y]