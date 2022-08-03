class Solution:
    def superPow(self, a: int, b: List[int]) -> int:
        a %= 1337
        b = int("".join(map(str, b)))
        
        result = 1
        last = [None for i in range(1337)]
        last[1] = 0
        mul = 0
        while b > 0:
            result *= a
            result %= 1337
            mul += 1
            
            if last[result] is not None:
                b %= (mul - last[result])
            else:
                last[result] = mul
            b -= 1
            
        
        return result