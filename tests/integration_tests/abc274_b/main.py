h, w = map(int, input().split())
a = [0] * w
for i in range(h):
    c = input()
    for j in range(w):
        if c[j] == "#":
            a[j] += 1
for i in a:
    print(i, end=" ")
print()
