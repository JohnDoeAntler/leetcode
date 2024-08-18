from typing import List

class Solution:
    def jump(self, nums: List[int]) -> int:
        ret = 0

        left_idx = 0
        right_idx = 1

        while right_idx < len(nums):
            old_right_idx = right_idx

            for i in range(left_idx, right_idx):
                right_idx = max(right_idx, i + nums[i] + 1)

            left_idx = old_right_idx
            ret += 1
    
        return ret

if __name__ == "__main__":
    solution = Solution()
    print(solution.jump([2,3,1,1,4]))
    print(solution.jump([2,3,0,1,4]))
    print(solution.jump([0]))
    print(solution.jump([1,1]))
        
