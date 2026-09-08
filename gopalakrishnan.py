"""
Lightrains Technologies - Assessment
Name: Gopalakrishnan
Position Applied: Python Developer
"""


# ============================================================
# ABOUT ME
# ============================================================

ABOUT_ME = """
I am Gopalakrishnan, a Python Backend Developer with around
3 years of experience in software development.

I have experience working with Python, Django, Flask, FastAPI,
REST APIs, MySQL, PostgreSQL, Redis, Docker, AWS, Linux, and Git.

I enjoy building backend applications, designing APIs,
solving problems, and learning new technologies.
"""

PERSONAL_COMPUTER = "Yes, I own a personal computer."

DEVELOPMENT_ENVIRONMENT = """
Operating System: Windows
IDE/Editor: Visual Studio Code
Programming Language: Python, SQL, JavaScripts
Version Control: Git and GitHub
Databases: MySQL and PostgreSQL
API Testing: Postman
"""

STACKOVERFLOW_PROFILE = "N/A"
PERSONAL_WEBSITE_OR_BLOG = "N/A"


# ============================================================
# PROGRAMMING LANGUAGES
# ============================================================

PROGRAMMING_LANGUAGES = [
    "Python 3.13"
]


# ============================================================
# QUESTION 2
# Takes a number and returns its digits as a list
# ============================================================

def number_to_digits(number):
    """
    Convert a number into a list of its digits.

    Example:
    12345 -> [1, 2, 3, 4, 5]
    """

    return [int(digit) for digit in str(abs(number))]


# ============================================================
# QUESTION 3
# Remove duplicates and return unique elements
# ============================================================

def remove_duplicates(items):
    """
    Remove duplicate elements while preserving the original order.
    """

    unique_items = []

    for item in items:
        if item not in unique_items:
            unique_items.append(item)

    return unique_items


# ============================================================
# QUESTION 4
# English to Pig Latin and Pig Latin back to English
# ============================================================

def to_pig_latin(text):
    """
    Convert English text to Pig Latin.

    Rule:
    Move the first letter to the end and add 'ay'.

    Example:
    The quick brown fox
    -> heTay uickqay rownbay oxfay
    """

    words = text.split()
    result = []

    for word in words:
        if word:
            pig_latin_word = word[1:] + word[0] + "ay"
            result.append(pig_latin_word)

    return " ".join(result)


def from_pig_latin(text):
    """
    Convert Pig Latin text back to English.
    """

    words = text.split()
    result = []

    for word in words:
        if len(word) >= 3 and word.endswith("ay"):
            original_word = word[-3] + word[:-3]
            result.append(original_word)
        else:
            result.append(word)

    return " ".join(result)


# ============================================================
# QUESTION 5
# Rotate a list in-place by k elements
# ============================================================

def reverse_list(arr, start, end):
    """
    Reverse a portion of the list in-place.
    """

    swaps = 0

    while start < end:
        arr[start], arr[end] = arr[end], arr[start]
        start += 1
        end -= 1
        swaps += 1

    return swaps


def rotate_list(arr, k):
    """
    Rotate the list left by k elements without creating
    another copy of the list.

    Example:
    [1, 2, 3, 4, 5, 6], k=2
    -> [3, 4, 5, 6, 1, 2]

    Uses the reversal algorithm.
    """

    n = len(arr)

    if n == 0:
        return 0

    k = k % n

    if k == 0:
        return 0

    # Reverse first k elements
    swaps = reverse_list(arr, 0, k - 1)

    # Reverse remaining elements
    swaps += reverse_list(arr, k, n - 1)

    # Reverse the complete list
    swaps += reverse_list(arr, 0, n - 1)

    return swaps


# ============================================================
# TESTING
# ============================================================

if __name__ == "__main__":

    print("=" * 50)
    print("QUESTION 2 - NUMBER TO DIGITS")
    print("=" * 50)

    print(number_to_digits(12345))

    print("\n" + "=" * 50)
    print("QUESTION 3 - REMOVE DUPLICATES")
    print("=" * 50)

    numbers = [1, 2, 2, 3, 4, 4, 5]
    print(remove_duplicates(numbers))

    print("\n" + "=" * 50)
    print("QUESTION 4 - PIG LATIN")
    print("=" * 50)

    text = "The quick brown fox"

    pig_latin = to_pig_latin(text)

    print("Original:", text)
    print("Pig Latin:", pig_latin)
    print("Back to English:", from_pig_latin(pig_latin))

    print("\n" + "=" * 50)
    print("QUESTION 5 - ROTATE LIST")
    print("=" * 50)

    numbers = [1, 2, 3, 4, 5, 6]

    print("Original List:", numbers)

    operations = rotate_list(numbers, 2)

    print("Rotated List:", numbers)
    print("Number of swap operations:", operations)