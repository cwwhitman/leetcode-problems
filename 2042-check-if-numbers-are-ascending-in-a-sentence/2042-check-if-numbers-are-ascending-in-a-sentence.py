class Solution:
    def areNumbersAscending(self, s: str) -> bool:
        nums = [int(n) for n in s.split() if n.isdecimal()]
        return all(nums[i-1] < nums[i] for i in range(1, len(nums)))