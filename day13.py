import functools

@functools.cache
def is_pallindrome(string: str) -> bool:
    half = int(len(string) / 2)
    l = len(string) - 1
    for i in range(half):
        if string[i] != string[l-i]:
            return False
    return True

def get_first(line: str):
    v_pos = set({})
    l = len(line)
    for i in range(len(line)):
        l_sub = line[i:]
        r_sub = line[:l - i]
        if len(l_sub) > 1 and is_pallindrome(l_sub):
            s = i + (len(l_sub) / 2) - 1
            print(l_sub, ' ', s)
            v_pos.add(s)
        if len(r_sub) > 1 and is_pallindrome(r_sub):
            s = len(r_sub) / 2
            print(r_sub, ' ', s)
            v_pos.add(s)
    return v_pos

def check_block(block) -> int:
    l = len(block)
    for i in range(l):
        l_sub = block[i:]
        r_sub = block[:l - i]
        if len(l_sub) > 1 and is_pallindrome(l_sub):
            s = i + (len(l_sub) / 2) - 1
            print(l_sub, ' ', s)
            return int(s)
        if len(r_sub) > 1 and is_pallindrome(r_sub):
            s = len(r_sub) / 2
            print(r_sub, ' ', s)
            return int(s)
    return 0

def part1(blocks):
    total = 0
    for block in blocks:
        l = [0,1,2,3,4,5,6,7,8,9]
        for line in block:
            a = get_first(line)
            n = []
            for x in l:
                if x in a:
                    n.append(x)
            l = n
        if len(l):
            total += int(l[0]) + 1
        else:
            hashes = []
            for line in block:
                hashes.append(hash(line))
            r = check_block(hashes)
            total += (r + 1) * 100

filename = 'test.txt'

with open(filename, 'r') as file:
    lines = []
    blocks = []
    for line in file.readlines():
        if line.strip() == '':
            blocks.append(lines)
            lines = []
        else:
            lines.append(line.strip())
    if len(lines) > 0:
        blocks.append(lines)
    part1(blocks)
