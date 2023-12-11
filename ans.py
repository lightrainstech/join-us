# 2. Write a function that takes a number and returns a list of its digits in an array.
def num_to_list(num):
    ans = []

    while num > 0:
        rem = num % 10
        num //= 10
        ans.insert(0, rem)

    if not ans:
        ans.append(0)

    return ans


# 3. Remove duplicates of an array and returning an array of only unique elements
# Approach 1: Time Complexity (O(nLog(n)))
def sort_and_remove_duplicates(arr):
    arr.sort()
    ans = []
    
    if arr:
        el = arr[0]
        ans.append(el)

        for num in arr[1:]:
            if num != el:
                ans.append(num)
                el = num

    return ans



# Approach 2: Time Complexity (O(n))
def remove_duplicate(arr):
    ans = []
    seen_set = set()

    for el in arr:
        if el not in seen_set:
            seen_set.add(el)
            ans.append(el)

    return ans



# 4. Write function that translates 
# a text to Pig Latin and back. English is translated to Pig Latin by 
# taking the first letter of every word, moving it to the end of the word 
# and adding ‘ay’. “The quick brown fox” becomes “Hetay uickqay rownbay oxfay”.
def translate_to_pig_latin(s):
    if len(s) == 0:
        return "ay"
    
    ans = ""
    arr = s.split(" ")

    for st in arr:
        i = 0
        ch = st[i].lower()
        i += 1
        temp = st[i:] + ch + "ay "
        ans += temp

    return ans.capitalize()

def translate_to_text(s):
    if len(s) == 2:
        return ""
    
    arr = s.split(" ")
    ans = ""

    for st in arr:
        n = len(st)
        temp = st[-3].lower() + st[:-3].lower() + " "
        ans += temp

    return ans.capitalize()


# 5. Write a function that rotates a list by `k` elements. For example [1,2,3,4,5,6]
# rotated by `2` becomes [3,4,5,6,1,2]. Try solving this without creating a
# copy of the list. How many swap or move operations do you need?

def rotate(arr, k):
    n = len(arr)
    k %= n
    rotate_range(arr, 0, k-1)
    rotate_range(arr, k, n-1)
    rotate_range(arr, 0, n-1)
    return arr

def rotate_range(arr, s, e):
    while s <= e:
        temp = arr[s]
        arr[s] = arr[e]
        arr[e] = temp
        s += 1
        e -= 1

