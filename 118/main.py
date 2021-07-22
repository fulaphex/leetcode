class Solution:
    def generate(self, numRows: int) -> List[List[int]]:
        rows = [[1]]
        for i in range(numRows-1):
            prev_row = rows[-1]
            row = [1]
            for idx in range(len(prev_row)-1):
                row.append(prev_row[idx] + prev_row[idx+1])
            row.append(1)
            rows.append(row)
        return rows
