from collections import defaultdict

class Solution:
    def groupThePeople(self, groupSizes: List[int]) -> List[List[int]]:
        completeGroups = []
        partGroups = defaultdict(lambda: [])
        
        for i, size in enumerate(groupSizes):
            partGroups[size].append(i)
            if len(partGroups[size]) == size:
                completeGroups.append(partGroups[size])
                partGroups[size] = []
        return completeGroups