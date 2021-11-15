s = input()

ans = 0
max_bit = 2 ** (len(s) - 1)  # +が入るパターンの総数
for i in range(max_bit):
    left = 0  # 足し始めのインデックス

    # +が入り得る場所に2進数を割り当て、値が1のときは+が入るものとみなして足していく
    # +が入り得る場所の数に合わせて0埋めする
    for j, val in enumerate(format(i, "b").zfill(len(s) - 1)):
        if val == "1":
            # +が入るとき、足し始めからそこまでの値を足す
            ans += int(s[left : j + 1])

            # 足し始めを次のインデックスに設定
            left = j + 1

    # left が途中なら残りを足す
    if left < len(s):
        ans += int(s[left:])

print(ans)
