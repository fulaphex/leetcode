class Solution:
    def canFinish(self, numCourses: int, prerequisites: List[List[int]]) -> bool:
        neighs: dict[int, list[int]] = defaultdict(list)
        cnt: dict[int, int] = defaultdict(int)
            
        for x, y in prerequisites:
            neighs[y].append(x)
            cnt[x] += 1
        
        zeros = {idx for idx in range(numCourses) if cnt[idx] == 0}
        
        order = []
        while zeros:
            el = zeros.pop()
            order.append(el)
            
            for neigh in neighs[el]:
                cnt[neigh] -= 1
                if cnt[neigh] == 0:
                    zeros.add(neigh)
        
        return len(order) == numCourses        