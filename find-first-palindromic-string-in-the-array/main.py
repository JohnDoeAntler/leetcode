from typing import List

class Solution:
    def isPalindrome(self, word: str) -> bool:
        i = 0
        j = len(word) - 1

        while i < j:
            if word[i] != word[j]:
                return False
            i += 1
            j -= 1

        return True

    def firstPalindrome(self, words: List[str]) -> str:
        for w in words:
            if self.isPalindrome(w):
                return w

        return ""

if __name__ == "__main__":
    solution = Solution()
    print(solution.firstPalindrome(["abc","car","ada","racecar","cool"]));
    print(solution.firstPalindrome(["notapalindrome","racecar"]));
    print(solution.firstPalindrome(["def","ghi"]));

