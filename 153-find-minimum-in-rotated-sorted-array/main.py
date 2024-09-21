from typing import List
import math

class Solution:
    def findMin(self, nums: List[int]) -> int:
        if len(nums) == 1:
            return nums[0]

        center_idx = len(nums) // 2
        left_start = 0
        right_end = len(nums) - 1

        while center_idx != right_end:
            is_left_ascending = nums[left_start] <= nums[center_idx - 1]
            is_right_ascending = nums[center_idx] <= nums[right_end]

            if not is_left_ascending:
                right_end = center_idx - 1
            elif not is_right_ascending:
                left_start = center_idx
            elif nums[left_start] < nums[center_idx]:
                right_end = center_idx - 1
            else:
                left_start = center_idx

            center_idx = math.ceil((left_start + right_end) / 2)
        
        return min(nums[left_start], nums[right_end])


if __name__ == '__main__':
    solution = Solution()
    print(solution.findMin([3,4,5,6,1,2]))
    print(solution.findMin([4,5,6,1,2,3]))
    print(solution.findMin([1,2,3]))
