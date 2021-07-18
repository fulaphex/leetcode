# char, no, no?, no*
ParsedPattern = list[tuple[chr, int, int, int]]
# char, min_occ, max_occ (-1 if inf)
ConstraintList = list[tuple[chr, int, int]]


class Solution:
    def parse_pattern(self, pattern_str: str) -> ConstraintList:
        # print(pattern_str)
        idx = 0
        pattern: ParsedPattern = []
        while idx < len(pattern_str):
            char = pattern_str[idx]
            # print(char)

            if idx == len(pattern_str) - 1:
                # last character in string
                pattern.append((char, 1, 0, 0))
                break

            if pattern_str[idx+1] == "?":
                pattern.append((char, 0, 1, 0))
                idx += 2
            elif pattern_str[idx+1] == "*":
                pattern.append((char, 0, 0, 1))
                idx += 2
            else:
                pattern.append((char, 1, 0, 0))
                idx += 1
        # print(pattern)
        # simplify pattern
        simp_pattern: ParsedPattern = pattern[:1]
        for char, x, y, z in pattern[1:]:
            if simp_pattern[-1][0] == char:
                _, ox, oy, oz = simp_pattern[-1]
                simp_pattern[-1] = char, ox+x, oy+y, oz+z
            else:
                simp_pattern.append((char, x, y, z))
        return [
            (char, x, -1 if z else x+y)
            for char, x, y, z in simp_pattern
        ]

    def isMatch(self, s: str, p: str) -> bool:
        constraint_list = self.parse_pattern(p)
        let_cnt: list[tuple[chr, int]]
        if s == "":
            let_cnt = []
        else:
            let_cnt = [(s[0], 1)]
            for c in s[1:]:
                prev_char, prev_cnt = let_cnt[-1]
                if c == prev_char:
                    let_cnt[-1] = prev_char, prev_cnt + 1
                else:
                    let_cnt.append((c, 1))

        print(let_cnt)
        # let_cnt[x], constraint_list[y]
        x, y = 0, 0
        while x < len(let_cnt) and y < len(constraint_list):
            let, let_cnt = let_cnt[x]
            constr_let, constr_min, constr_max = constraint_list[y]
            if let == constr_let:
                if let_cnt < constr_min:
                    # fewer letters than min of constraint
                    return False
                if constr_max != -1 and constr_max < let_cnt:
                    # more letters than max
                    return False
                x += 1
                y += 1
                pass
            else:
                if constr_min > 0:
                    return False
                # ? or * constraint, can skip
                y += 1
        if x < len(let_cnt):
            # pattern finished, string not finished
            return False

        while y < len(constraint_list):
            _, constr_min, _ = constraint_list[y]
            if constr_min > 0:
                return False
            y += 1

        return True



# for pat in ["aa?a*b*c?d*c?cc?"]:
#     print(Solution().parse_pattern(pat))

print(Solution().isMatch("aa", "a"))