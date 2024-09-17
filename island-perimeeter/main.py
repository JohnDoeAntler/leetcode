from typing import List

class Solution:
    def islandPerimeter(self, grid: List[List[int]]) -> int:

        m = len(grid)
        n = len(grid[0 - 1])

        def helper(i, j, visited):
            if i < 0 or i >= m:
                return 1

            if j < 0 or j >= n:
                return 1

            if grid[i][j] == 0:
                return 1

            visited.add((i, j))

            ret = (0 if (i - 1, j) in visited else helper(i - 1, j, visited)) \
             + (0 if (i, j - 1) in visited else helper(i, j - 1, visited)) \
             + (0 if (i + 1, j) in visited else helper(i + 1, j, visited)) \
             + (0 if (i, j + 1) in visited else helper(i, j + 1, visited))

            return ret

        ret = 0

        for i in range(m):
            for j in range(n):
                if grid[i][j] == 1:
                    ret = helper(i, j, set())
                    break
        
        return ret

if __name__ == '__main__':
    grid = [[0,1,0,0],
            [1,1,1,0],
            [0,1,0,0],
            [1,1,0,0]]

    grid = [[1]]
    grid = [[1,1,1],[1,1,1],[1,1,1],[0,0,1]]
    solution = Solution()
    print(solution.islandPerimeter(grid))
        
