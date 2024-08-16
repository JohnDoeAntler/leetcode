from typing import List

class Solution:
    def twoSum(self, numbers: List[int], target: int) -> List[int]:
        left_idx = 0
        right_idx = len(numbers) - 1

        while left_idx < right_idx:
            acc = numbers[left_idx] + numbers[right_idx]

            if acc > target:
                right_idx -= 1
            elif acc < target:
                left_idx += 1
            else:
                return [left_idx + 1, right_idx + 1]

        return [-1, -1]

if __name__ == '__main__':
    solution = Solution()
    print(solution.twoSum([1,2,3,4], 3))
