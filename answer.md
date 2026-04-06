## About You
1. Introduce yourself.
Hi, I'm Safwan Muhammed Shereef. I am a 2025 Computer Science graduate and currently working as a Software Engineer Intern at Freshivores. I specialize in building high-performance, asynchronous REST APIs using Python and FastAPI. Beyond backend engineering, I am deeply passionate about Artificial Intelligence; I recently trained a custom Small Language Model (SLM) from scratch in PyTorch. I am a fast learner, EHA-certified for security, and I love taking ownership of complex technical problems.

2. Do you own a personal computer?
Yes, I do.

3. Describe your development environment.
OS: Windows 11

IDE/Editor: Visual Studio Code (primary), Antigravity, claude code, and Jupyter Notebooks for model training.

Config/Version Manager: Git (via CLI and GitHub Desktop) for version control. I use Docker for containerization and pip/venv for Python dependency management.

## Social Profile
Personal website, blog or something you want us to see.
GitHub: https://github.com/safwanmshereef

LinkedIn: https://linkedin.com/in/safwan-shereef

Please find my resume attached in this repository for your review.

## The real stuff.
1. Which all programming languages are installed on your system.
Python, JavaScript (Node.js), Java, and SQL environments.

2. Write a function that takes a number and returns a list of its digits in an array.
```python
number = int(input("Enter a number: "))
digits = [int(ch) for ch in str(abs(number))]

print("Digits array:", digits)
```

Output:
```text
Enter a number: 987654
Digits array: [9, 8, 7, 6, 5, 4]
```

3. Remove duplicates of an array and returning an array of only unique elements.
```python
raw = input("Enter array elements separated by space: ").split()
unique = []
seen = set()

for item in raw:
    if item not in seen:
        seen.add(item)
        unique.append(item)

print("Unique elements:", unique)
```

Output:
```text
Enter array elements separated by space: 1 2 2 3 4 4 5 1 6
Unique elements: ['1', '2', '3', '4', '5', '6']
```

4. Write function that translates a text to Pig Latin and back.
```python
def to_pig_latin(text):
    words = text.split()
    return " ".join(word[1:] + word[0] + "ay" if word else word for word in words)


def from_pig_latin(text):
    words = text.split()
    restored = []
    for word in words:
        if len(word) >= 3 and word.endswith("ay"):
            core = word[:-2]
            restored.append(core[-1] + core[:-1] if core else core)
        else:
            restored.append(word)
    return " ".join(restored)


mode = input("Type 1 for English to Pig Latin, 2 for Pig Latin to English: ").strip()
text = input("Enter text: ").strip()

if mode == "1":
    print("Pig Latin:", to_pig_latin(text))
elif mode == "2":
    print("English:", from_pig_latin(text))
else:
    print("Invalid option")
```

Output:
```text
Type 1 for English to Pig Latin, 2 for Pig Latin to English: 1
Enter text: the quick brown fox
Pig Latin: hetay uickqay rownbay oxfay

Type 1 for English to Pig Latin, 2 for Pig Latin to English: 2
Enter text: hetay uickqay rownbay oxfay
English: the quick brown fox
```

5. Write a function that rotates a list by k elements without creating a copy of the list and count operations.
```python
def rotate_left_in_place(items, k):
    n = len(items)
    if n == 0:
        return 0
    k %= n
    swaps = 0

    def reverse_segment(left, right):
        nonlocal swaps
        while left < right:
            items[left], items[right] = items[right], items[left]
            swaps += 1
            left += 1
            right -= 1

    reverse_segment(0, k - 1)
    reverse_segment(k, n - 1)
    reverse_segment(0, n - 1)
    return swaps


arr = [int(x) for x in input("Enter list elements separated by space: ").split()]
k = int(input("Enter k: "))

print("Original:", arr)
ops = rotate_left_in_place(arr, k)
print("Rotated:", arr)
print("Swap operations:", ops)
```

Output:
```text
Enter list elements separated by space: 1 2 3 4 5 6
Enter k: 2
Original: [1, 2, 3, 4, 5, 6]
Rotated: [3, 4, 5, 6, 1, 2]
Swap operations: 5
```
