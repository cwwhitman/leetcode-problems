class Solution:
    def minEatingSpeed(self, piles: List[int], h: int) -> int:
        lo, hi = 0, max(piles)
        def check_speed(speed):
            return sum(-(-pile // speed) for pile in piles) <= h
        while lo + 1 < hi:
            mid_speed = (lo + hi)//2
            if check_speed(mid_speed):
                hi = mid_speed
            else:
                lo = mid_speed
        return hi