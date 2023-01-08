## About You.

1. Introduce yourself.

Hi there, my name is Senzel and I am a recent graduate in Mechatronics Engineering. I have always been fascinated by the power of technology to revolutionize the way we live and work, and have developed a particular interest in the field of web3 and blockchain technology.

To further my knowledge in these areas, I have taken a number of online courses such as LearnWeb3DAO and buildspace, as well as attending the Solidity Bootcamp. I have also had the opportunity to work on projects using protocols like TheGraph, Ceramic, and Chainlink, which has helped me to gain practical experience in the field. In addition, I regularly follow developments in the industry through protocol sharing, local ETH communities, discords, and twitter to stay up to date on the latest developments.

2. Do you own a personal computer?

   Yes

3. Describe your development environment. (Your OS, IDE, Editor and Config manager if any)

Windows 11, Vscode

## Social Profile

1. Your StackOverflow Profile url.

https://stackoverflow.com/users/20533774/senzel

2. Personal website, blog or something you want us to see.

[Personal portfolio website](https://0xsenzel.netlify.app/)

## The real stuff.

1. Which all programming languages are installed on your system.

Javascript, Solidity, python

2. Write a function that takes a number and returns a list of its digits in an array.

```
function getDigits(num) {
  let digits = [];
  while (num > 0) {
    digits.unshift(num % 10);
    num = Math.floor(num / 10);
  }
  return digits;
}
console.log(getDigits(1019))

output: [1, 0, 1, 9]
```

3. Remove duplicates of an array and returning an array of only unique elements

```
function removeDuplicates(arr) {
  let uniqueElements = [];
  for (let i = 0; i < arr.length; i++) {
    if (!uniqueElements.includes(arr[i])) {
      uniqueElements.push(arr[i]);
    }
  }
  return uniqueElements;
}

const number = [1, 2, 2, 3, 4, 4, 4, 5, 6, 6, 6, 6, 7, 7, 8];
console.log(removeDuplicates(number));

output: [1,2,3,4,5,6,7,8]
```

4. Write function that translates a text to Pig Latin and back. English is translated to Pig Latin by taking the first letter of every word, moving it to the end of the word and adding ‘ay’. “The quick brown fox” becomes “Hetay uickqay rownbay oxfay”.

```
function EnglishToPigLatin(text) {
  // Split the text into an array of words
  const words = text.split(" ");

  const pigLatinWords = words.map((word) => {
    // Extract the first letter and move it to the end
    const firstLetter = word[0];
    const restOfWord = word.slice(1);
    const pigLatinWord = restOfWord + firstLetter + "ay";
    return pigLatinWord;
  });

  // Join the array of Pig Latin words back into a single string
  const pigLatinText = pigLatinWords.join(" ");
  return pigLatinText;
}
console.log(EnglishToPigLatin("The quick brown fox"));

function PigLatinToEnglish(text) {
  // Split the text into an array of words
  const words = text.split(" ");

  // Translate each word to English
  const englishWords = words.map((word) => {
    // Extract the last letter and move it to the beginning
    const lastLetter = word[word.length - 3];
    const restOfWord = word.slice(0, word.length - 3);
    const englishWord = lastLetter + restOfWord;
    return englishWord;
  });

  // Join the array of English words back into a single string
  const englishText = englishWords.join(" ");
  return englishText;
}
console.log(PigLatinToEnglish("Hetay uickqay rownbay oxfay"));

output:
heTay uickqay rownbay oxfay
tHe quick brown fox
```

5. Write a function that rotates a list by `k` elements. For example [1,2,3,4,5,6] rotated by `2` becomes [3,4,5,6,1,2]. Try solving this without creating a copy of the list. How many swap or move operations do you need?

```
function rotateList(list, k) {
  for (let i = 0; i < k; i++) {
    const firstElement = list.shift();
    list.push(firstElement);
  }
  return list;
}

const List = [1, 2, 3, 4, 5, 6];
console.log(rotateList(List, 2));

output: [3, 4, 5, 6, 1, 2]
```

Note: It is not mandatory that you answer all the questions. You may leave some behind and create a PR. However maximum questions will earn you maximum points. It is advised that you answer all the puzzles in a language that you are applying for.

### Notes

- [Pig Latin](https://en.wikipedia.org/wiki/Pig_Latin)
- [Fun](http://www.snowcrest.net/donnelly/piglatin.html)
- [Nice Read](https://medium.com/javascript-scene/10-interview-questions-every-javascript-developer-should-know-6fa6bdf5ad95)
