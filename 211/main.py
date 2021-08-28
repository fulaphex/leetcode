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
        if len(word) == idx:
            return self.end
        el = word[idx]
        if el == ".":
            for el in self.neighs:
                if self.neighs[el].search(word, idx+1):
                    return True
            return False
        if el not in self.neighs:
            return False
        return self.neighs[el].search(word, idx+1)

class WordDictionary:

    def __init__(self):
        """
        Initialize your data structure here.
        """
        self.root = Node(None)
        

    def addWord(self, word: str) -> None:
        self.root.insert(word)

    def search(self, word: str) -> bool:
        return self.root.search(word)


# Your WordDictionary object will be instantiated and called as such:
# obj = WordDictionary()
# obj.addWord(word)
# param_2 = obj.search(word)