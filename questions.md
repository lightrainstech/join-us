## About You
1. Introduce yourself.
2. Describe your development environment. (Your OS, IDE, editor, AI tools and config manager, if any)

## Social Profile
1. Your GitHub profile URL, and links to any Open Source contributions you're proud of.
2. Personal website, blog, StackOverflow profile or anything else you want us to see.

## The real stuff

For every coding question below:

- **Write tests.** Cover the edge cases, not just the example given.
- **Explain your reasoning** in a few sentences: why your solution works, which edge cases you considered, and its time and space complexity.
- **If you used AI**, note what it got wrong or what you changed. See "Using AI tools" in the README.

1. Which programming languages are installed on your system?
2. Write a function that takes a number and returns a list of its digits in an array. State what your function does with negative numbers and non-integers.
3. Write a function that removes duplicates from an array and returns an array of only the unique elements, keeping the original order.
4. Write functions that translate text to Pig Latin and back.
   - **To Pig Latin:** for every word, move the first letter to the end of the word and add `ay`. If the original word started with a capital letter, the translated word should too.
   - **Back to English:** for every word, remove the trailing `ay`, move the last letter back to the front and restore the capitalisation.
   - You may assume the input contains only letters and single spaces.
   - Example: “The quick brown fox” becomes “Hetay uickqay rownbay oxfay”, and translating that back gives “The quick brown fox” again.
5. Write a function that rotates a list by `k` elements. For example [1,2,3,4,5,6] rotated by `2` becomes [3,4,5,6,1,2]. Try solving this without creating a copy of the list. How many swap or move operations do you need? Give an exact count in terms of `n` and `k`, not just the big-O.

## Spot the bugs

The function below is supposed to return the `n` most frequent words in a text, most frequent first. It has several bugs.

```javascript
function topWords(text, n) {
  const counts = {};
  for (const word of text.split(' ')) {
    counts[word] = (counts[word] || 0) + 1;
  }
  return Object.keys(counts)
    .sort((a, b) => counts[a] - counts[b])
    .slice(0, n);
}
```

1. List each bug you find, with an input that triggers it and the wrong output it produces.
2. Write a fixed version (in any language) with tests that would have caught each bug.

## Build something small

Turn your Pig Latin translator into a small command-line tool **or** HTTP endpoint, your choice. Include short instructions for running it, and a few sentences on the design decisions and trade-offs you made.

Note: It is not mandatory that you answer all the questions. You may leave some out and still create a PR. However, more answers will earn you more points. We advise answering the puzzles in the language of the role you're applying for.

### Notes

- [Pig Latin](https://en.wikipedia.org/wiki/Pig_Latin)
- [Fun](http://www.snowcrest.net/donnelly/piglatin.html)
- [Nice Read](https://medium.com/javascript-scene/10-interview-questions-every-javascript-developer-should-know-6fa6bdf5ad95)
