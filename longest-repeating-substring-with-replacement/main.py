class Solution:
    def characterReplacement(self, s: str, k: int) -> int:
        if s == '':
            return 0

        max_count = 0
        current_left_idx = 0
        num_map = [0] * 26

        for idx, c in enumerate(s):
            num_map[ord(c) - 65] += 1

            expected_tolerance = idx - current_left_idx + 1 - max(num_map)

            if k >= expected_tolerance:
                max_count = max(max_count, idx - current_left_idx + 1)
            else:
                num_map[ord(s[current_left_idx]) - 65] -= 1
                current_left_idx += 1

        return max_count

if __name__ == '__main__':
    solution = Solution()
    print(solution.characterReplacement("XYYZ", 2))
    print(solution.characterReplacement("AAABABB", 1))
        
