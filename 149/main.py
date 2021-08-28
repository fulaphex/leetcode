class Solution:
    def maxPoints(self, points: List[List[int]]) -> int:
        # a*x_1 + b*y_1 = a*x_2 + b*y_2
        # a*x_1 - a*x_2 = b*y_2 - b*y_1
        # a*(x_1 - x_2) = b*(y_2 - y_1)
        # a = b*(y_2 - y_1)/(x_1 - x_2)
        res = 0
        
        for idx, (x1, y1) in enumerate(points):
            dirs: dict[float, int] = defaultdict(int)
                
            for idx2, (x2, y2) in enumerate(points):
                if idx == idx2:
                    continue
                # print(x1, y1, x2, y2)
                if x1 == x2:
                    dir_ = float("inf")
                else:
                    dir_ = (y2-y1)/(x2-x1)
                dirs[dir_] += 1
                res = max(res, dirs[dir_])
        return res+1
