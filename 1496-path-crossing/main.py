class Solution:
    def isPathCrossing(self, path: str) -> bool:
        x = 0
        y = 0

        s = set()
        s.add((0, 0))

        for e in path:
            if e == 'N':
                y -= 1
            elif e == 'E':
                x += 1
            elif e == 'S':
                y += 1
            elif e == 'W':
                x -= 1

            if (x, y) in s:
                return True

            s.add((x, y))

        return False

if __name__ == "__main__":
    solution = Solution()
    print(solution.isPathCrossing("NES"))
    print(solution.isPathCrossing("NESWW"))
    print(solution.isPathCrossing("NNSWWEWSSESSWENNW"))

