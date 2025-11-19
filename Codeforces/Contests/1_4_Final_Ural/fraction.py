s, c = input().split()

s_len = len(s)
c_len = len(c)

l_a_1 = (s_len + c_len) // 2
l_b_1 = s_len - l_a_1

l_a_2 = (s_len + c_len - 1) // 2
l_b_2 = s_len - l_a_2


a1 = int(s[:l_a_1])
b1 = int(s[l_a_1:])
a2 = int(s[:l_a_2])
b2 = int(s[l_a_2:])

c = int(c)

if b1 * c == a1:
    print(a1, b1)
else:
    print(a2, b2)
