from typing import List

class Solution:
    def maxAreaOfIsland(self, grid: List[List[int]]) -> int:
        ret = 0

        def helper(i, j):
            if i < 0 or i >= len(grid):
                return 0

            if j < 0 or j >= len(grid[i]):
                return 0

            if grid[i][j] != 1:
                return 0

            grid[i][j] = 0

            return 1 + helper(i - 1, j) + helper(i + 1, j) + helper(i, j - 1) + helper(i, j + 1)

        for i in range(len(grid)):
            for j in range(len(grid[i])):
                if grid[i][j] == 1:
                    ret = max(ret, helper(i, j))

        return ret

if __name__ == "__main__":
    solution = Solution()
    print(solution.maxAreaOfIsland([
        [0,0,1,0,0,0,0,1,0,0,0,0,0],
        [0,0,0,0,0,0,0,1,1,1,0,0,0],
        [0,1,1,0,1,0,0,0,0,0,0,0,0],
        [0,1,0,0,1,1,0,0,1,0,1,0,0],
        [0,1,0,0,1,1,0,0,1,1,1,0,0],
        [0,0,0,0,0,0,0,0,0,0,1,0,0],
        [0,0,0,0,0,0,0,1,1,1,0,0,0],
        [0,0,0,0,0,0,0,1,1,0,0,0,0],
    ]))
