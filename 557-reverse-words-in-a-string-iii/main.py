from typing import List

class Solution:
    def reverse_word(self, s: List[str], start_idx: int, end_idx: int):
        left_idx = start_idx
        right_idx = end_idx - 1

        while left_idx < right_idx:
            s[left_idx], s[right_idx] = s[right_idx], s[left_idx]
            left_idx += 1
            right_idx -= 1

    def reverseWords(self, s: str) -> str:
        l = list(s)
        string = l + [' ']
        start_idx = 0

        for end_idx, char in enumerate(string):
            if char == ' ':
                print(start_idx, end_idx)
                self.reverse_word(string, start_idx, end_idx)
                start_idx = end_idx + 1

        return ''.join(string[:-1])

if __name__ == '__main__':
    solution = Solution()
    print(solution.reverseWords("Let's  take  LeetCode  contest "))
