from typing import List

class Solution:
    def maxArea(self, heights: List[int]) -> int:
        ret = 0

        l, r = 0, len(heights) - 1

        while l != r:
            width = r - l
            height = min(heights[l], heights[r])
            ret = max(ret, width * height)

            if heights[l] > heights[r]:
                r -= 1
            else:
                l += 1

        return ret

if __name__ == "__main__":
    solution = Solution()
    print(solution.maxArea([1,7,2,5,4,7,3,6]))

