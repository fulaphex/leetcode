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
        
    def search(self, word, idx = 0, changes = 1):
        if len(word) == idx:
            return self.end and changes == 0
        
        el = word[idx]
        
        if changes > 0:
            for nel in self.neighs:
                if el == nel:
                    continue
                if self.neighs[nel].search(word, idx+1, changes - 1):
                    return True
        
        if el not in self.neighs:
            return False
        
        return self.neighs[el].search(word, idx+1, changes)

class MagicDictionary:

    def __init__(self):
        """
        Initialize your data structure here.
        """
        self.root = Node(None)
        

    def buildDict(self, dictionary: List[str]) -> None:
        for word in dictionary:
            self.root.insert(word)
        

    def search(self, searchWord: str) -> bool:
        return self.root.search(searchWord)
    


# Your MagicDictionary object will be instantiated and called as such:
# obj = MagicDictionary()
# obj.buildDict(dictionary)
# param_2 = obj.search(searchWord)