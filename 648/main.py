class Node:
    def __init__(self, val: Optional[str], end = False):
        self.val = val
        self.neighs = {}
        self.end = end
    
    
    def insert(self, word: str, idx = 0):
        if len(word) == idx:
            self.end = True
            return
        
        el = word[idx]
        
        if el not in self.neighs:
            self.neighs[el] = Node(el)
            
        self.neighs[el].insert(word, idx+1)
        
    def search(self, word, idx = 0):
        if self.end:
            return word[:idx]
        
        if idx == len(word):
            return word
        
        el = word[idx]
        
        if el not in self.neighs:
            return word
        
        return self.neighs[el].search(word, idx+1)

class Solution:
    def replaceWords(self, dictionary: List[str], sentence: str) -> str:
        root = Node(None)
        for word in dictionary:
            root.insert(word)
            
        return " ".join(
            root.search(word)
            for word in sentence.split(" ")
        )