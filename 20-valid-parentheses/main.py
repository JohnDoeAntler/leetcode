class Solution:
    def isValid(self, s: str) -> bool:
        ret = []

        parenthese_map = [
            '()',
            '[]',
            '{}',
        ]

        for c in s:
            if c == '(' or c == '{' or c == '[':
                ret.append(c)
            elif len(ret) and ret[-1] + c in parenthese_map:
                ret.pop()
            else:
                return False

        return not len(ret)

if __name__ == '__main__':
    s = Solution()
    print(s.isValid('()'))  # True
    print(s.isValid('()[]{}'))  # True
    print(s.isValid('(]'))  # False
    print(s.isValid('([)]'))  # False
    print(s.isValid('{[]}'))  # True
