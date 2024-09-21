from typing import List

class Solution:
    def countCharacters(self, words: List[str], chars: str) -> int:
        ret = 0

        # get the dictionary of the chars
        m = { chr(e + 97): 0 for e in range(26) }

        for e in chars:
            m[e] += 1

        # foreach word, determine whether they could be composed via the chars
        for word in words:
            tmp = m.copy()

            for c in word:
                tmp[c] -= 1

                if tmp[c] < 0:
                    break
            else:
                ret += len(word)

        return ret

if __name__ == "__main__":
    solution = Solution()
    print(solution.countCharacters(
        ["cat","bt","hat","tree"],
        "atach"
    ))
    print(solution.countCharacters(
        ["hello","world","leetcode"],
        "welldonehoneyr",
    ))
