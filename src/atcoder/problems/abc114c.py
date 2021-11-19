n = int(input())

ans = 0


def f(n, current_num, use):
    global ans
    if current_num > n:
        return

    if use == 0b111:
        ans += 1

    f(n, 10 * current_num + 3, 0b001 | use)
    f(n, 10 * current_num + 5, 0b010 | use)
    f(n, 10 * current_num + 7, 0b100 | use)


def main():
    f(n, 0, 0)
    print(ans)


main()
