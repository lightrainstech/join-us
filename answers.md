


1)python 3.7

2)def convert(list):
    s = [str(i) for i in list]
    res = int("".join(s))
    return(res)
list = [1, 2, 3]
print(convert(list))



3)str1=input("Enter the string:")
str1=str1.split()
str2=[]
for i in str1:
    if i not in str2:
        str2.append(i)
        str2.append(" ")
str1=" "
for i in str2:
    str1=str1+i
print("Remove Duplicates:",str1)

4)def main():
        lst = ['sh', 'gl', 'ch', 'ph', 'tr', 'br', 'fr', 'bl', 'gr', 'st', 'sl', 'cl', 'pl', 'fl']
        sentence = input('Type what you would like translated into pig-latin and press ENTER: ')
        sentence = sentence.split()
        for k in range(len(sentence)):
                i = sentence[k]
                if i[0] in ['a', 'e', 'i', 'o', 'u']:
                        sentence[k] = i+'ay'
                elif t(i) in lst:
                        sentence[k] = i[2:]+i[:2]+'ay'
                elif i.isalpha() == False:
                        sentence[k] = i
                else:
                        sentence[k] = i[1:]+i[0]+'ay'
        return ' '.join(sentence)

5)def leftRotate(arr, d, n):
    d = d % n
    g_c_d = gcd(d, n)
    for i in range(g_c_d):
        temp = arr[i]
        j = i
        while 1:
            k = j + d
            if k >= n:
                k = k - n
            if k == i:
                break
            arr[j] = arr[k]
            j = k
        arr[j] = temp
def printArray(arr, size):
    for i in range(size):
        print ("% d" % arr[i], end =" ")

def gcd(a, b):
    if b == 0:
        return a;
    else:
        return gcd(b, a % b)
arr = [1,2,3,4,5,6,7]
n = len(arr)
d = 2
leftRotate(arr, d, n)
printArray(arr, n)

