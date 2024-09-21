class Solution:
    def largestGoodInteger(self, num: str) -> str:
        ret = ""

        for i, e in enumerate(num[:-2]):
            if e == num[i + 1] and e == num[i + 2]:
                ret = max(ret, num[i:i+3])

        return ret
        
if __name__ == "__main__":
    solution = Solution()
    print(solution.largestGoodInteger("6777133339"))
    print(solution.largestGoodInteger("2300019"))
    print(solution.largestGoodInteger("42352338"))
    
