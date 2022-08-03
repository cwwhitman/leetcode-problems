import re
from collections import defaultdict

class Solution:
    def evaluate(self, s: str, knowledge: List[List[str]]) -> str:
        knowledge = defaultdict(lambda: '?', knowledge)
        return re.sub(r"\((\w+)\)", lambda m: knowledge[m.group(1)], s)
            
        