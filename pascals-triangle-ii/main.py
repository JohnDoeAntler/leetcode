from typing import List
from math import factorial

class Solution:
    def getRow(self, rowIndex: int) -> List[int]:
        def ncr(n: int, r: int):
            return factorial(n) // (factorial(r) * factorial(n - r))

        return [ncr(rowIndex, e) for e in range(rowIndex + 1)]

if __name__ == '__main__':
    rowIndex = 5
    solution = Solution()
    print(solution.getRow(rowIndex))
        
