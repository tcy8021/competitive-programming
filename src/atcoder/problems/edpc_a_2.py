# 配る遷移形式
n = int(input())
h = list(map(int, input().split()))

INF = 1e10

dp = [INF] * n
dp[0] = 0
for i in range(n - 1):
    if i < n - 2:
        dp[i + 2] = min(dp[i + 2], dp[i] + abs(h[i + 2] - h[i]))

    dp[i + 1] = min(dp[i + 1], dp[i] + abs(h[i + 1] - h[i]))

print(dp[n - 1])
