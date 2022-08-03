class Solution:
    def canIWin(self, maxChoosableInteger: int, desiredTotal: int) -> bool:
        if (maxChoosableInteger*(maxChoosableInteger+1))//2 < desiredTotal:
            return False
        
        decided = {}
        def can_win(taken, goal):
            if taken in decided:
                return decided[taken]
            
            bit_mask = 1 << (maxChoosableInteger-1)
            goal -= maxChoosableInteger
            while bit_mask:
                if not (taken & bit_mask):
                    if goal <= 0:
                        decided[taken] = True
                        return True
                    result = can_win(taken + bit_mask, goal)
                    if not result:
                        decided[taken] = True
                        return True
                bit_mask >>= 1
                goal += 1
            
            decided[taken] = False
            return False
        return can_win(0, desiredTotal)