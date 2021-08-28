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
        
    def search(self, word, prefix, idx = 0):
        if len(word) == idx:
            return prefix or self.end
        el = word[idx]
        if el not in self.neighs:
            return False
        return self.neighs[el].search(word, prefix, idx+1)
        
class Trie:

    def __init__(self):
        """
        Initialize your data structure here.
        """
        self.root = Node(None)
        

    def insert(self, word: str) -> None:
        """
        Inserts a word into the trie.
        """
        self.root.insert(word)
        

    def search(self, word: str) -> bool:
        """
        Returns if the word is in the trie.
        """
        return self.root.search(word, prefix=False)

    def startsWith(self, prefix: str) -> bool:
        """
        Returns if there is any word in the trie that starts with the given prefix.
        """
        return self.root.search(prefix, prefix=True)
        


# Your Trie object will be instantiated and called as such:
# obj = Trie()
# obj.insert(word)
# param_2 = obj.search(word)
# param_3 = obj.startsWith(prefix)