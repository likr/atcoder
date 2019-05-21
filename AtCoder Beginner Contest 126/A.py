# coding: utf-8


def main():
    n, k = map(int, input().split())
    s = input()
    k = k - 1
    print(s[:k] + s[k].lower() + s[k + 1:])

if __name__ == '__main__':
    main()
