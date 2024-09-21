from typing import List
from collections import deque

class Solution:
    def orangesRotting(self, grid: List[List[int]]) -> int:
        queue = deque()
        ret = 0
        count = 0

        # find the rotting oranges on start
        for i in range(len(grid)):
            for j in range(len(grid[i])):
                if grid[i][j] == 2:
                    queue.append((i - 1, j))
                    queue.append((i + 1, j))
                    queue.append((i, j - 1))
                    queue.append((i, j + 1))

                if grid[i][j] == 1:
                    count += 1

        while len(queue):
            arr = []

            l = len(queue)
            for _ in range(l):
                (i, j) = queue.popleft()

                if i < 0 or i >= len(grid):
                    continue

                if j < 0 or j >= len(grid[i]):
                    continue

                if grid[i][j] == 0 or grid[i][j] == 2:
                    continue

                count -= 1
                grid[i][j] = 2

                arr.append((i - 1, j))
                arr.append((i + 1, j))
                arr.append((i, j - 1))
                arr.append((i, j + 1))

            if len(arr):
                queue.extend(arr)

                ret += 1

        if count == 0:
            return ret

        return -1
        

if __name__ == "__main__":
    solution = Solution()
    print(solution.orangesRotting([[2,1,1],[1,1,0],[0,1,1]])) # 4
    print(solution.orangesRotting([[2,1,1],[0,1,1],[1,0,1]]))
    print(solution.orangesRotting([[0,2]])) # 0
