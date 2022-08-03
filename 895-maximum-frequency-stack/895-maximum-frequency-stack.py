import heapq
from collections import defaultdict

class FreqStack:

    def __init__(self):
        self.freqs = defaultdict(lambda: 0)
        self.most_common = []
        self.added = 0

    def push(self, val: int) -> None:
        self.freqs[val] += 1

        heapq.heappush(self.most_common, (-self.freqs[val], -self.added, val))
        self.added += 1

    def pop(self) -> int:
        _, _, val = heapq.heappop(self.most_common)
        self.freqs[val] -= 1
        return val
        