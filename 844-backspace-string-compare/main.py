class Solution:
    def backspaceCompare(self, s: str, t: str) -> bool:
        s_idx = len(s) - 1
        t_idx = len(t) - 1

        while s_idx >= 0 or t_idx >= 0:
            s_sum = 0
            t_sum = 0

            while s_idx >= 0:
                if s[s_idx] == '#':
                    s_idx -= 1
                    s_sum += 1
                elif s_sum > 0:
                    s_idx -= 1
                    s_sum -= 1
                else:
                    break
            
            while t_idx >= 0:
                if t[t_idx] == '#':
                    t_idx -= 1
                    t_sum += 1
                elif t_sum > 0:
                    t_idx -= 1
                    t_sum -= 1
                else:
                    break

            if (s_idx >= 0) != (t_idx >= 0):
                return False

            if s_idx >= 0 and t_idx >= 0 and s[s_idx] != t[t_idx]:
                return False

            s_idx -= 1
            t_idx -= 1

        return True
        
if __name__ == '__main__':
    solution = Solution()
    print(solution.backspaceCompare("bbbextm", "bbb#extm"))


