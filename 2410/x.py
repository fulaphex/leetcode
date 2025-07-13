class Solution:
    def matchPlayersAndTrainers(self, players: list[int], trainers: list[int]) -> int:
        players = sorted(players)
        trainers = sorted(trainers)
        
        res = 0
        tidx, pidx = 0, 0
        
        while tidx < len(trainers) and pidx < len(players):
            if players[pidx] <= trainers[tidx]:
                res += 1
                pidx +=1
            tidx+=1

        return res


def test():
    players = [4,7,9]
    trainers = [8,2,5,8]
    result = 2
    assert Solution().matchPlayersAndTrainers(players=players, trainers=trainers) == result


def test2():
    players = [1,1,1]
    trainers = [10]
    result = 1
    assert Solution().matchPlayersAndTrainers(players=players, trainers=trainers) == result
