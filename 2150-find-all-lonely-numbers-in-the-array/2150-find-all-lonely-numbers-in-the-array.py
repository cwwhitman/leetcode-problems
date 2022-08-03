class Solution:
    def findLonely(self, nums: List[int]) -> List[int]:
        is_lonely = {}
        for num in nums:
            if num not in is_lonely:
                is_lonely[num] = True
            else:
                is_lonely[num] = False
            is_lonely[num-1] = False
            is_lonely[num+1] = False
        
        return [num for num, lonely in is_lonely.items() if lonely]