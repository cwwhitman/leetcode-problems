class Solution:
    def minimumNumbers(self, num: int, k: int) -> int:
        if not num:
            return 0
        elif not k:
            if num % 10:
                return -1
            else:
                return 1
        count = 1
        num -= k
        while num % 10 and num > 0:
            num -= k
            count += 1
        if num < 0:
            return -1
        else:
            return count