class Solution:
    def minOperations(self, s: str) -> int:
        r1 = 0
        r2 = 0

        for i, c in enumerate(s):
            if i % 2:
                if c == '0':
                    r1 += 1
                else:
                    r2 += 1
            else:
                if c == '1':
                    r1 += 1
                else:
                    r2 += 1

        return min(r1, r2)

if __name__ == "__main__":
    solution = Solution()
    print(solution.minOperations("0100"))
    print(solution.minOperations("10"))
    print(solution.minOperations("1111"))
    print(solution.minOperations("010101010010101010")) # 9 characters need to flipped
    print(solution.minOperations("010101010101010100")) # 1 characters need to flipped
    print(solution.minOperations("10010100"))
        
