from typing import List

class Solution:
    def findMaximumScore(self, nums: List[int]) -> int:
        ret = 0
        current_idx = 0

        for i, e in enumerate(nums):
            if i == 0:
                continue

            if e > nums[current_idx] or i == len(nums) - 1:
                ret += (i - current_idx) * nums[current_idx]
                current_idx = i

        return ret

if __name__ == '__main__':
    solution = Solution()
    print(solution.findMaximumScore([1,3,1,5]))
    print(solution.findMaximumScore([4,3,1,3,2]))
