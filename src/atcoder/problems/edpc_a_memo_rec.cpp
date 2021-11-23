#include <iostream>
#include <vector>
using namespace std;

long long rec(int i, vector<long long> &dp, vector<int> &h) {
  if (dp[i] != 1e10) {
    return dp[i];
  }

  if (i > 1) {
    dp[i] = min(dp[i], rec(i - 2, dp, h) + abs(h[i] - h[i - 2]));
  }

  dp[i] = min(dp[i], rec(i - 1, dp, h) + abs(h[i] - h[i - 1]));

  return dp[i];
}

int main() {
  int n;
  cin >> n;

  vector<int> h(n);
  for (int i = 0; i < n; i++) {
    cin >> h[i];
  }

  vector<long long> dp(n, 1e10);
  dp[0] = 0;
  rec(n - 1, dp, h);
  cout << dp[n - 1] << endl;

  for (int i = 0; i < 100000; i++) {
    cout << 1 << " ";
  }
  cout << endl;

  return 0;
}