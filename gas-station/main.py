from typing import List

class Solution:
    def canCompleteCircuit(self, gas: List[int], cost: List[int]) -> int:
        tmp = -1
        idx = 0
        acc = 0

        for i in range(len(gas)):
            current = gas[i] - cost[i]
            acc += current

            if tmp < 0:
                tmp = current
                idx = i
            else:
                tmp += current

        if acc < 0:
            return -1

        return idx


if __name__ == '__main__':
    solution = Solution()
    print(solution.canCompleteCircuit([1,2,3,4,5], [3,4,5,1,2]) == 3)
    # print(solution.canCompleteCircuit([2,3,4], [3,4,3]))
    print(solution.canCompleteCircuit([5,1,2,3,4], [4,4,1,5,1]) == 4)
    print(solution.canCompleteCircuit([3,1,1], [1,2,2]) == 0)
    print(solution.canCompleteCircuit([1,2,3,4,5,5,70], [2,3,4,3,9,6,2]) == 6)


