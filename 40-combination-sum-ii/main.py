from typing import List

class Solution:
    def helper(self, candidates, target, stack, visited):
        if target == 0:
            visited.append(stack)
            return

        if target < 0:
            return

        if len(candidates) == 0:
            return

        right = candidates[1:]
        right.reverse()

        for idx, i in enumerate(candidates):
            if idx > 0 and candidates[idx - 1] == i:
                if len(right) != 0:
                    right.pop()
                continue

            self.helper(right, target - i, stack + [i], visited)
            if len(right) != 0:
                right.pop()

    def combinationSum2(self, candidates: List[int], target: int) -> List[List[int]]:
        stack = []
        visited = []
        candidates.sort()
        self.helper(candidates, target, stack, visited)
        return visited
        
if __name__ == "__main__":
    solution = Solution()
    print(solution.combinationSum2([3,1,3,5,1,1], 8))
