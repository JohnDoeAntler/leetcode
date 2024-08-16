from typing import List

class Solution:
    def permute(self, nums: List[int]) -> List[List[int]]:
        if len(nums) == 1:
            return [nums]

        left = nums[1:]
        left.reverse()
        right = []

        ret = []

        for num in nums:
            tmp = self.permute(left + right)
            for nested_perm in tmp:
                ret.append([num] + nested_perm)

            # visited.append()

            if len(left) > 0:
                left.pop()
            right.append(num)

        return ret

if __name__ == "__main__":
    solution = Solution()
    print(solution.permute([1,3,4,2]))
