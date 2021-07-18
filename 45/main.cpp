#include<vector>
#include<iostream>
using namespace std;
int MAX_INT = 1<<30;

class Solution {
public:
    int jump(const vector<int>& nums) {
        auto dp = vector<int>(nums.size(), MAX_INT);
        dp[0] = 0;
        for(size_t i = 0; i < dp.size(); i++) {
            for(int j = 0; j <= nums[i]; j++) {
                if(i+j >= dp.size()) {
                    break;
                }
                dp[i+j] = min(dp[i+j], dp[i]+1);
            }
        }
        return dp[dp.size()-1];
    }
};

int main() {
    auto sol = Solution();
    cout << sol.jump({2, 3, 1, 1, 4});
    return 0;
}
