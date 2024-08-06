## About You.
1. Introduce yourself.

    Myself Sajil C. K., I have done a PhD in Computational Biology and Bioinformatics. I did my undergraduate (B.Sc.) and postgraduate (M.Sc.) in electronics. I have more than two years of experience working in the industry as data scientist, project associate and research assistant.

2. Do you own a personal computer?

    Yes, I have a MacBook Air.

3. Describe your development environment. (Your OS, IDE, Editor and Config manager if any)

    I usually work with Ubuntu OS other than my personal computer. My favorite IDE is Visual Studio Code. I also use free cloud services such as Kaggle or Google Colab.

## Social Profile
1. Your StackOverflow Profile url.
    
    Here is my StackOverflow profile url [https://stackoverflow.com/users/4827289/sajil-c-k](https://stackoverflow.com/users/4827289/sajil-c-k)

2. Personal website, blog or something you want us to see.
    I have a blog [https://intuitivetutorial.com/](https://intuitivetutorial.com/) where I have written more than 60+ articles on machine learning, signal processing, Python programming etc.

## The real stuff.
1. Which all programming languages are installed on your system.

    I mostly work with Python. But I am familiar with R, and Matlab as well.

2. Write a function that takes a number and returns a list of its digits in an array.

    ```python
    def num_list_of_digits(num):
        """
        Takes a number and returns a list of its digits
        """
        return [int(x) for x in str(num)]
    ```
3. Remove duplicates of an array and returning an array of only unique elements

    ```python
    def return_unique(in_list):
        """
        Remove duplicates and return unique elements
        """
        return list(set(in_list))
    ```
4. Write function that translates a text to Pig Latin and back. English is translated to Pig Latin by taking the first letter of every word, moving it to the end of the word and adding ‘ay’. “The quick brown fox” becomes “Hetay uickqay rownbay oxfay”.

    ```python
    def text_to_pig_latin(text):
        """
        Translates a text to Pig Latin
        """
        text = text.lower()
        text_new = [word[1:] + word[0] + "ay" for word in text.split(" ")]
        text_new = " ".join(text_new)
        text_new = text_new[0].upper() + text_new[1:]
        return text_new
    ```
5. Write a function that rotates a list by `k` elements. For example [1,2,3,4,5,6] rotated by `2` becomes [3,4,5,6,1,2]. Try solving this without creating a copy of the list. How many swap or move operations do you need?
    ```python
    def rotate_list_by_k(in_list, k):
        """
        Rotates a list by `k` elements
        """
        return in_list[k:] + in_list[:k]
    ```

Note: It is not mandatory that you answer all the questions. You may leave some behind and create a PR. However maximum questions will earn you maximum points. It is advised that you answer all the puzzles in a language that you are applying for.

### Notes

- [Pig Latin](https://en.wikipedia.org/wiki/Pig_Latin)
- [Fun](http://www.snowcrest.net/donnelly/piglatin.html)
- [Nice Read](https://medium.com/javascript-scene/10-interview-questions-every-javascript-developer-should-know-6fa6bdf5ad95)
