class Solution:
    def maxScore(self, s: str) -> int:
        ints = [int(e) for e in s]

        ret = 0

        left = 0
        right = sum(ints)

        for i in ints[:-1]:
            if i == 0:
                left += 1
            else:
                right -= 1

            ret = max(ret, left + right)

        return ret

if __name__ == "__main__":
    solution = Solution()
    print(solution.maxScore("011101"))
    print(solution.maxScore("00111"))
    print(solution.maxScore("1111"))
    print(solution.maxScore("00"))
        
