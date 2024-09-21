from typing import List

class Solution:
    def isMonotonic(self, nums: List[int]) -> bool:
        if len(nums) == 1:
            return True

        is_ascending = None

        for i, e in enumerate(nums[1:]):
            if is_ascending is None:
                if nums[i] > e:
                    is_ascending = False
                if nums[i] < e:
                    is_ascending = True
            elif (is_ascending and nums[i] > e) or (not is_ascending and nums[i] < e):
                return False

        return True

if __name__ == '__main__':
    solution = Solution()
    nums = [1,2,2,3]
    nums = [6,5,4,4]
    nums = [1,3,2]
    nums = [1,1,2]
    print(solution.isMonotonic(nums))
