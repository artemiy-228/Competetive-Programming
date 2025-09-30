l = 0
r = 1023

while l < r:
    mid = (r + l) // 2
    print(f"? {l} {r}", flush=True)
    count = int(input())
    if count == 2:
        r = mid
    elif count == 0:
        l = r + 1
        r = r * 2
    else:
        l1 = l
        r1 = mid
        while l1 < r1:
            mid1 = (l1 + r1) // 2
            print(f"? {l1} {r1}", flush=True)
            count = int(input())
            if count == 0:
                l1 = r1 + 1
                r1 = 2 * r1
            else:
                r1 = mid1

        l2 = mid + 1
        r2 = r
        while l2 < r2:
            mid2 = (l2 + r2) // 2
            print(f"? {l2} {r2}", flush=True)
            count = int(input())
            if count == 0:
                l2 = r2 + 1
                r2 = 2 * r2
            else:
                r2 = mid2

        print(f"! {l1} {r2 + 1}")
        exit(0)
