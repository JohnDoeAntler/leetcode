from typing import List

class Solution:
    def arrayStringsAreEqual(self, word1: List[str], word2: List[str]) -> bool:
        w1y, w1x = 0, 0
        w2y, w2x = 0, 0
        w1_completed = False
        w2_completed = False

        while True:
            if w1_completed != w2_completed:
                return False

            if w1_completed and w2_completed:
                return True

            if word1[w1y][w1x] != word2[w2y][w2x]:
                return False

            if w1x < len(word1[w1y]) - 1:
                w1x += 1
            elif w1y < len(word1) - 1:
                w1y += 1
                w1x = 0
            else:
                w1_completed = True

            if w2x < len(word2[w2y]) - 1:
                w2x += 1
            elif w2y < len(word2) - 1:
                w2y += 1
                w2x = 0
            else:
                w2_completed = True

if __name__ == '__main__':
    solution = Solution()
    print(solution.arrayStringsAreEqual(["ab", "c"], ["a", "bc"]))
    print(solution.arrayStringsAreEqual(["a", "cb"], ["ab", "c"]))
    print(solution.arrayStringsAreEqual(["abc", "d", "defg"], ["abcddefg"]))
