class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        if s == '':
            return 0

        max_count = 1
        current_left_idx = 0

        num_map = [0] * 128
        num_map[ord(s[0])] += 1

        for idx, c in enumerate(s[1:]):
            num_map[ord(c)] += 1

            # duplicates
            if all(e < 2 for e in num_map):
                max_count = max(max_count,  idx - current_left_idx + 1 + 1);
            else: 
                num_map[ord(s[current_left_idx])] -= 1
                current_left_idx += 1
 
        return max_count
        
        # [; 26] => { a: number; b: number, ... }
        
if __name__ == '__main__':
    solution = Solution()
    print(solution.lengthOfLongestSubstring("zxyyyzzabcdfg"))
