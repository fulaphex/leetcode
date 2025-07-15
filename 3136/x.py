class Solution:
    def isValid(self, word: str) -> bool:
        if len(word) < 3:
            return False
        vowels = set("aeiouAEIOU")
        letters = {chr(ord("a") + offset) for offset in range(26)} | {
            chr(ord("A") + offset) for offset in range(26)
        }

        consonants = letters - vowels
        chars = set(word)
        digits = set("0123456789")
        if not (vowels & chars):
            return False
        if not (consonants & chars):
            return False
        if chars - letters - digits:
            return False
        return True


def test():
    word = "234Adas"
    assert Solution().isValid(word)


def test2():
    word = "b3"
    assert not Solution().isValid(word)


def test3():
    word = "a3$e"
    assert not Solution().isValid(word)
