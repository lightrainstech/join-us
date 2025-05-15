## About You.

1. Introduce yourself.

This is Hafilue c, I am a Frontend Developer with over 2 years of focused expertise in building responsive user interfaces
with React.js and Redux. My work has resulted in significant improvements, such as a 40% reduction in
page load times and a 25% increase in user satisfaction. I thrive in team environments, effectively
collaborating with designers and backend developers to deliver high-quality applications and user
experiences

2. Do you own a personal computer?

yes

3. Describe your development environment. (Your OS, IDE, Editor and Config manager if any)

windows 10/zorin
vscode

## Social Profile

1. Your StackOverflow Profile url.

nil

2. Personal website, blog or something you want us to see.

github : https://github.com/Hafilu?tab=repositories
netlify : https://app.netlify.com/teams/hafilu/projects

## The real stuff.

1. Which all programming languages are installed on your system.

JavaScript, c, python

2. Write a function that takes a number and returns a list of its digits in an array.

// Converts number to an array of digits
const numberToDigits = (num) => {
if (typeof num !== "number" || isNaN(num)) return [];
return num.toString().split("").map(Number);
};

console.log(numberToDigits(10));

3. Remove duplicates of an array and returning an array of only unique elements

// Removes duplicates from an array
const uniqueArray = (arr) => {
return Array.isArray(arr) ? [...new Set(arr)] : [];
};

console.log(uniqueArray([1, 1, 1, 2, 2, 3]));

4. Write function that translates a text to Pig Latin and back. English is translated to Pig Latin by taking the first letter of every word, moving it to the end of the word and adding ‘ay’. “The quick brown fox” becomes “Hetay uickqay rownbay oxfay”.

// Converts a sentence to Pig Latin
const toPigLatin = (str) => {
return str
.split(" ")
.map((word) => {
if (word.length === 0) return word;
return word.slice(1) + word[0].toLowerCase() + "ay";
})
.join(" ");
};

console.log(toPigLatin("The quick brown fox"));

5. Write a function that rotates a list by `k` elements. For example [1,2,3,4,5,6] rotated by `2` becomes [3,4,5,6,1,2]. Try solving this without creating a copy of the list. How many swap or move operations do you need?

// Rotates array k times to the left
const rotateArr = (arr, k) => {
if (!Array.isArray(arr) || arr.length === 0) return arr;
const count = k % arr.length;
return [...arr.slice(count), ...arr.slice(0, count)];
};

console.log(rotateArr([1, 2, 3, 4, 5, 6], 7));
