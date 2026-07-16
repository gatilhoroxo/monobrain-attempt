
n = int(input())
palavra = str(input())

pos = palavra.find("lv")

if pos != -1:
    print("0\n")
else:
    pos = palavra.find("l")
    if pos != -1:
        print("1\n")
    else:
        pos = palavra.find("v")
        if pos != -1:
            print("1\n")
        else:
            print("2\n")
