a, b, c = map(int, input().split())

size = max(a, b, c)

s_a = "a" * size
s_b = "b" * size
s_c = "c" * size

dp_a_b = [[0 for i in range(size + 1)]] * (size + 1)

for i in range(1, size + 1):
    for j in range(1, size + 1):
        if s_a[i - 1] == s_b[j - 1]:
            dp_a_b[i][j] = dp_a_b[i - 1][j - 1] + 1
        else:
            dp_a_b[i][j] = min(dp_a_b[i - 1][j], dp_a_b[i][j - 1], dp_a_b[i - 1][j - 1]) + 1

print(dp_a_b)
