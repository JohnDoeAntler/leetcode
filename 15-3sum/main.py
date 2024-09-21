from typing import List

class Solution:
    def threeSum(self, nums: List[int]) -> List[List[int]]:
        ret = []
        nums.sort()

        for target_idx, target in enumerate(nums[:-2]):
            # prevent the duplicated target
            if target_idx > 0 and target == nums[target_idx - 1]:
                continue

            left_idx = target_idx + 1
            right_idx = len(nums) - 1

            while left_idx < right_idx:
                if nums[left_idx] + nums[right_idx] == -target:
                    ret.append([target, nums[left_idx], nums[right_idx]])

                    while right_idx > 0 and nums[right_idx] == nums[right_idx - 1]:
                        right_idx -= 1

                if nums[left_idx] + nums[right_idx] < -target:
                    left_idx += 1
                else:
                    right_idx -= 1

        return ret

if __name__ == '__main__':
    nums = [-1, 0, 1, 2, -1, -4]
    nums = [0,1,1]
    nums = [-5,-4,-3,-2,1,0,1,2,3,4,5]
    nums = [1,1,1,1,1,1,1,-1,-1,0,1,1]

    solution = Solution()
    print(solution.threeSum(nums))
