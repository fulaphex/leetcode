import heapq
import json

MAX = 1e20


class Solution:
    def mostBooked(self, n: int, meetings: list[list[int]]) -> int:
        meetings = sorted(meetings)

        meetings_per_room = [0] * n
        # (meeting_end_time, room_index)
        meeting_end_times_heap = []
        free_rooms_heap = list(range(n))
        heapq.heapify(free_rooms_heap)

        for meeting in meetings:
            meeting_start, meeting_end = meeting
            meeting_duration = meeting_end - meeting_start

            while meeting_end_times_heap and meeting_end_times_heap[0][0] <= meeting_start:
                print(f"removing finished meeting, {meeting_end_times_heap[0]}")
                _, room_idx = heapq.heappop(meeting_end_times_heap)
                heapq.heappush(free_rooms_heap, room_idx)

            if free_rooms_heap:
                room_idx = heapq.heappop(free_rooms_heap)
                print(f"free room available, using {room_idx}")
                meetings_per_room[room_idx] += 1
                heapq.heappush(meeting_end_times_heap, (meeting_end, room_idx))
            else:
                room_end, room_idx = heapq.heappop(meeting_end_times_heap)
                # room_idx = -room_idx
                print(f"free room unavailable, using {room_idx}, room end time {room_end}")
                meetings_per_room[room_idx] += 1
                heapq.heappush(meeting_end_times_heap, (room_end + meeting_duration, room_idx))

            print()

        return -max([(meeting_count, -room_idx) for room_idx, meeting_count in enumerate(meetings_per_room)])[1]


def test_small():
    n = 2
    meetings = [[0, 10], [1, 5], [2, 7], [3, 4]]
    result = 0
    assert Solution().mostBooked(n=n, meetings=meetings) == result


def test_bigger():
    n = 3
    meetings = [[1, 20], [2, 10], [3, 5], [4, 9], [6, 8]]
    result = 1
    assert Solution().mostBooked(n=n, meetings=meetings) == result


def test_superbig():
    n = 10
    meetings = json.load(open("superbig.in"))
    result = 1
    assert Solution().mostBooked(n=n, meetings=meetings) == result


def test_other():
    n = 4
    meetings = [[48, 49], [22, 30], [13, 31], [31, 46], [37, 46], [32, 36], [25, 36], [49, 50], [24, 34], [6, 41]]
    result = 0
    assert Solution().mostBooked(n=n, meetings=meetings) == result
