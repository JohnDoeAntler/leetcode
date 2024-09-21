from typing import List

class Solution:
    def makeEqual(self, words: List[str]) -> bool:
        m = [0] * 26

        for w in words:
            for c in w:
                m[ord(c) - 97] += 1

        return all(e % len(words) == 0 for e in m)

if __name__ == "__main__":
    solution = Solution()
    print(solution.makeEqual(["abc", "aabc", "bc"]))
    print(solution.makeEqual(['ab', 'a']))
        
