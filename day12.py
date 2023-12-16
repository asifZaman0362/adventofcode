import functools

memo = {}

def take(text: str, n: int) -> int:
    counter = 0
    first_broken = -1
    for (i, c) in enumerate(text):
        if first_broken >= 0 and i - first_broken >= n:
            return -1
        if c == '?' or c == '#':
            if c == '#':
                first_broken = i if first_broken == -1 else first_broken
            counter += 1
            if counter == n:
                if i < len(text) - 1:
                    if (text[i + 1] == '?' or text[i + 1] == '.'):
                        return i
                    else:
                        counter -= 1
                else:
                    return i
        else:
            counter = 0
    return -1

@functools.cache
def solve(text: str, nums):
    #global memo
    #your memo sucks, use functools.cache instead kthxbye
    nums = list(nums)
    total = sum(nums)
    orig = text
    #if memo.get((text, total)):
    #    return memo[(text, total)]
    if len(nums) == 0:
        c = 1 if '#' not in text else 0
        memo[(text, total)] = c
        return c
    possibilites = 0
    start = next((i for i, char in enumerate(text) if char != '.'), None)
    if start == None:
        memo[(text, total)] = 0
        return 0
    while len(text) >= total:
        if start > 0:
            if text[start - 1] == '#':
                break
        text = text[start:]
        rem = take(text, nums[0])
        if rem == -1:
            break
        else:
            possibilites += solve(text[rem + 2:], tuple(nums[1:]))
        start = (rem + 1) - nums[0] + 1
    memo[(orig, total)] = possibilites
    return possibilites

def part2(text, num):
    memo.clear()
    text = '?'.join([text] * 5)
    x = []
    for i in ([num] * 5):
        x.extend(i)
    return solve(text, tuple(x))
    
def process(data):
    memo.clear()
    [text, num] = data.split(' ')
    nums = []
    sum = 0
    for s in num.split(','):
        nums.append(int(s))
    #print(text, ' ', solve(text, list))
    x = solve(text, tuple(nums))
    y = part2(text, nums)
    return x, y

with open('input.txt', 'r') as input:
    data = input.readline()
    total = 0
    total_p2 = 0
    i = 0
    while data != '':
        if len(data) > 0:
            (p1, p2) = process(data)
            i += 1
            total += p1
            total_p2 += p2
            print(total_p2)
        data = input.readline().strip()
    print(total, ' ', total_p2)
