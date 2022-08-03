from collections import defaultdict
import heapq

class Solution:
    def leastInterval(self, tasks: List[str], n: int) -> int:
        freqs = defaultdict(lambda: 0)
        for task in tasks:
            freqs[task] += 1
        
        taskq = list(map(lambda x: -x, freqs.values()))
        heapq.heapify(taskq)
        
        timed_out = [None for i in range(100)]
        #order = []
        cycle = 0
        while taskq or any(timed_out):
            if timed_out[cycle%100]:
                heapq.heappush(taskq, timed_out[cycle%100])
                timed_out[cycle%100] = None
            
            if taskq:
                freq = heapq.heappop(taskq)
                if freq + 1 < 0:
                    timed_out[(cycle + n + 1) % 100] = (freq + 1)
            
            cycle += 1
        
        return cycle