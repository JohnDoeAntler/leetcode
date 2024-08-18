from typing import List

class Solution:
    def maxDistance(self, arrays: List[List[int]]) -> int:
        normalized = []

        for arr in arrays:
            if len(arr) >= 2:
                normalized.append([min(*arr), max(*arr)])
            else:
                normalized.append([arr[0], arr[0]])

        min_arrays = normalized[::]
        max_arrays = normalized[::]

        min_arrays.sort(key=lambda a: a[0])
        max_arrays.sort(key=lambda a: a[1])

        if min_arrays[0] == max_arrays[-1]:
            return max(
                max_arrays[-2][1] - min_arrays[0][0],
                max_arrays[-1][1] - min_arrays[1][0],
            )
        else:
            return max_arrays[-1][1] - min_arrays[0][0]

if __name__ == '__main__':
    arrays = [[1,2,3],[4,5],[1,2,3]]
    arrays = [[1],[1]]
    solution = Solution()
    print(solution.maxDistance(arrays))
        
