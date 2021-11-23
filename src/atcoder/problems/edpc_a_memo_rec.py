import sys

# 再帰の上限を増やす
sys.setrecursionlimit(10 ** 9)


def rec(i, h, dp):
    if dp[i] < 10 ** 10:
        return dp[i]

    if i > 1:
        dp[i] = min(dp[i], rec(i - 2, h, dp) + abs(h[i] - h[i - 2]))

    dp[i] = min(dp[i], rec(i - 1, h, dp) + abs(h[i] - h[i - 1]))

    return dp[i]


def main():
    n = int(input())
    h = list(map(int, input().split()))

    dp = [10 ** 10] * n
    dp[0] = 0
    rec(n - 1, h, dp)
    print(dp[n - 1])


main()
