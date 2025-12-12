#!/usr/bin/python3
from typing import Dict, List
from functools import cache

def readfile(filename: str) -> Dict[str, List[str]]:
    res = {}
    with open(filename) as f:
        for line in f.readlines():
            k,v = line.strip().split(":")
            res[k] = v.strip().split(" ")
    return res

def count(key: str, g: Dict[str, List[str]]) -> int:
    res = 0
    if key not in g:
        return 0
    for v in g[key]:
        if v == "out":
            res += 1
        else:
            res += count(v, g)
    return res

@cache
def count2(key: str, dac: bool, fft: bool) -> int:
    res = 0
    if key not in g:
        return 0
    for v in g[key]:
        if v == "out":
            if dac and fft:
                res += 1
        else:
            res += count2(v, dac or v == "dac", fft or v == "fft")
    return res


g = readfile("test.txt")
print(count("you", g))
#g = readfile("test2.txt")
#print(count2("svr", False, False))
g = readfile("input.txt")
print(count("you", g))
print(count2("svr", False, False))
