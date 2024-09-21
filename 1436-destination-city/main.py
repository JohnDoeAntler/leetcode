from typing import List

class Solution:
    def destCity(self, paths: List[List[str]]) -> str:
        s = set()

        for path in paths:
            s.add(path[1])

        for path in paths:
            if path[0] in s:
                s.remove(path[0])

        return next(iter(s))
        
if __name__ == "__main__":
    solution = Solution()
    print(solution.destCity([["London","New York"],["New York","Lima"],["Lima","Sao Paulo"]]))
    print(solution.destCity([["B","C"],["D","B"],["C","A"]]))

