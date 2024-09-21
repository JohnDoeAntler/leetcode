from typing import List

class Solution:
    def numOfSubarrays(self, arr: List[int], k: int, threshold: int) -> int:
        ret = 0
        acc = 0

        for i in range(k - 1):
            acc += arr[i]

        for i, e in enumerate(arr[k - 1:]):
            acc += e

            if acc >= threshold * k:
                ret += 1

            acc -= arr[i]

        return ret

if __name__ == '__main__':
    solution = Solution()
    print(solution.numOfSubarrays([2,2,2,2,5,5,5,8], 3, 4))
    print(solution.numOfSubarrays([11,13,17,23,29,31,7,5,2,3], 3, 5))
        

