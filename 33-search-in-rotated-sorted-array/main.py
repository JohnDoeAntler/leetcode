from typing import List
import math

class Solution:
    def search(self, nums: List[int], target: int) -> int:
        left_start = 0
        right_end = len(nums) - 1
        mid = math.ceil((left_start + right_end) / 2)

        while mid != right_end:
            is_left_descending = nums[left_start] > nums[mid - 1]
            is_right_descending = nums[mid] > nums[right_end]

            if is_left_descending:
                # check if the target is in right
                if nums[mid] <= target and target <= nums[right_end]:
                    left_start = mid
                else:
                    right_end = mid - 1
            elif is_right_descending:
                if nums[left_start] <= target and target <= nums[mid - 1]:
                    right_end = mid - 1
                else:
                    left_start = mid
            elif nums[left_start] <= target and target <= nums[mid - 1]:
                right_end = mid - 1
            else:
                left_start = mid
        
            mid = math.ceil((left_start + right_end) / 2)

        if nums[left_start] == target:
            return left_start
        elif nums[right_end] == target:
            return right_end
        else:
            return -1

if __name__ == "__main__":
    sol = Solution()
    print(sol.search([4,5,6,7,0,1,2], 0))
    print(sol.search([1,2,3], 2))
    print(sol.search([4,5,6,1,2,3], 1))
    print(sol.search([4,5,6,7,0,1,2], 3))
