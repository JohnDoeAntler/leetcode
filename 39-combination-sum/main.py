class Solution:
    def helper(self, candidates, target, stack, visited):
        if target == 0:
            return visited.append(stack)

        if target < 0:
            return

        for idx, i in enumerate(candidates):
            self.helper(candidates[idx:], target - i, [*stack, i], visited)

        return

    def combinationSum(self, candidates, target):
        visited = []
        self.helper(candidates, target, [], visited)
        return visited

if __name__ == '__main__':
    candidates = [2,3,5]
    target = 8
    print(Solution().combinationSum(candidates, target))
