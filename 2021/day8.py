from collections import Counter

def fun(l):
    return [k for k, v in Counter(list(''.join(l))).items() if v == 2]

with open("day8.txt") as f:
    a = f.read()
ball = [[j[0].split(), j[1].split()] for j in [i.split(" | ") for i in a.splitlines()]]

s = 0
for b in ball:
    t = [''.join(sorted(i)) for i in b[0]]

    one = [i for i in t if len(i) == 2][0]
    four = [i for i in t if len(i) == 4][0]
    seven = [i for i in t if len(i) == 3][0]
    eight = [i for i in t if len(i) == 7][0]

    sixs = [i for i in t if len(i) == 6]
    fifs = [i for i in t if len(i) == 5]
    e_segm = [j for j in fun(sixs) if j not in four][0]
    f_segm = [f for f in one if all([f in s for s in sixs])][0]
    c_segm = [i for i in one if i != f_segm][0]
    d_segm = [f for f in four if all([f in s for s in fifs])][0]
    b_segm = [i for i in four if i not in [d_segm, c_segm, f_segm]][0]

    zero = [i for i in sixs if c_segm in i and e_segm in i][0]
    two = [i for i in fifs if c_segm in i and e_segm in i][0]
    three = [i for i in fifs if c_segm in i and f_segm in i][0]
    five = [i for i in fifs if b_segm in i and f_segm in i][0]
    six = [i for i in sixs if d_segm in i and e_segm in i][0]
    nine = [i for i in sixs if c_segm in i and d_segm in i][0]

    trans = {zero: 0, one: 1, two: 2, three: 3, four: 4, five: 5, six: 6, seven: 7, eight: 8, nine: 9}
    t1 = [''.join(sorted(i)) for i in b[1]]
    s += trans[t1[0]] * 1000 + trans[t1[1]] * 100 + trans[t1[2]] * 10 + trans[t1[3]]

print(s)
