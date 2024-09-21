from typing import List

class Solution:
    def numIdenticalPairs(self, nums: List[int]) -> int:
        ret = 0
        arr = [0] * 101

        for i in nums:
            arr[i] += 1

        for i in arr:
            ret += (i * (i - 1)) // 2

        return ret


if __name__ == "__main__":
    solution = Solution()
    print(solution.numIdenticalPairs([1,2,3,1,1,3]))
    print(solution.numIdenticalPairs([1,1,1,1]))
    print(solution.numIdenticalPairs([1,2,3]))
        
