class Solution:
    def longestSubarray(self, nums: List[int]) -> int:
        ones = True
        longest = 0
        last_seq = 0
        seq = 0
        for n in nums:
            ones &= n
            if n:
                seq += 1
                longest = max(longest, last_seq + seq)
            elif seq:
                last_seq = seq
                seq = 0
            elif last_seq:
                last_seq = 0
        return longest - ones