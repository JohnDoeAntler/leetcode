from typing import List

class Solution:
    def canJump(self, nums: List[int]) -> bool:
        range = 0

        for idx, num in enumerate(nums):
            if idx > range:
                return False

            range = max(range, idx + num)

        return True

if __name__ == '__main__':
    nums = [2, 3, 1, 1, 4]
    nums = [3, 2, 1, 0, 4]
    solution = Solution()
    print(solution.canJump(nums))
