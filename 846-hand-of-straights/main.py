from typing import List

class Solution:
    def isNStraightHand(self, hand: List[int], groupSize: int) -> bool:

        hand.sort()
        map = dict()

        for i in hand:
            if i in map:
                map[i] += 1
            else:
                map[i] = 1

        for i in range(len(hand)):
            current_hand = hand[i]

            if map[current_hand] == 0:
                continue

            map[current_hand] -= 1
            
            for j in range(groupSize - 1):
                compare = current_hand + j + 1
                
                if compare not in map or map[compare] == 0:
                    return False

                map[compare] -= 1

        return True

if __name__ == "__main__":
    solution = Solution()
    print(solution.isNStraightHand([1,2,3,6,2,3,4,7,8], 3))
    print(solution.isNStraightHand([1,2,3,4,5], 4))
        
