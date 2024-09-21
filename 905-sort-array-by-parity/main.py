from typing import List

class Solution:
    def sortArrayByParity(self, nums: List[int]) -> List[int]:
        left_idx = 0
        right_idx = len(nums) - 1

        while left_idx < right_idx:
            # is even, does nothing
            if nums[left_idx] % 2 == 0:
                left_idx += 1
                continue

            # is odd
            # find last even
            while nums[right_idx] % 2 != 0:
                if right_idx <= left_idx:
                    return nums

                right_idx -= 1

            nums[left_idx], nums[right_idx] = nums[right_idx], nums[left_idx]

        return nums

if __name__ == '__main__':
    solution = Solution()
    print(solution.sortArrayByParity([3, 1, 2, 4]))
    print(solution.sortArrayByParity([3, 1, 2])) # 2 3 1
    print(solution.sortArrayByParity([3, 1, 2])) # 2 1 3

