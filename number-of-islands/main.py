from typing import List

class Solution:
    def numIslands(self, grid: List[List[str]]) -> int:
        # find the first island first
        ret = 0
        def convert(i, j):
            if i < 0 or i >= len(grid):
                return

            if j < 0 or j >= len(grid[i]):
                return

            if grid[i][j] == '0':
                return

            grid[i][j] = '0'

            convert(i - 1, j)
            convert(i, j - 1)
            convert(i + 1, j)
            convert(i, j + 1)

        for i in range(len(grid)):
            for j in range(len(grid[i])):
                if grid[i][j] == '1':
                    # convert the island to 0
                    convert(i, j)
                    ret += 1

        return ret


if __name__ == '__main__':
    # grid = [["1","1","1","1","0"],["1","1","0","1","0"],["1","1","0","0","0"],["0","0","0","0","0"]]
    grid = [
        ["1","1","0","0","0"],
        ["1","1","0","0","0"],
        ["0","0","1","0","0"],
        ["0","0","0","1","1"]
    ]
    solution = Solution()
    print(solution.numIslands(grid))
