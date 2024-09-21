from typing import List

class Solution:
    def helper(self, nums):
        if not len(nums):
            return []

        first, rest = nums[0], nums[1:]

        ret = self.helper(rest)

        for f in range(len(ret)):
            ret.append([first, *ret[f]])

        ret.append([first])

        return ret


    def subsets(self, nums: List[int]) -> List[List[int]]:
        return self.helper(nums) + [[]]


if __name__ == '__main__':
    solution = Solution()

    print(solution.subsets([1, 2, 3]))
        

# [2, 3] => [] [[2] [3] [2, 3]] ([1] [1,2], [1,3], [1,2,3])
