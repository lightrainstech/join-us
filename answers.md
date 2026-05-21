## About You.
1. Introduce yourself.

My name is Pranav Raj KPV, and I am from Kannur, Kerala. I am currently working as a freelance developer and also as part of Helixon as a Full Stack Developer.

I completed my graduation in BSc Mathematics and have 1 year and 3 months of experience as a Junior Software Developer at A and D Info Solution. I also completed a full stack development bootcamp at Brototype.

I have hands-on experience with technologies including React, Next.js, Node.js, Express.js, NestJS, MongoDB, MySQL, TypeScript, and JavaScript.

In addition, I have completed two major personal projects:

  * An e-commerce based food delivery application
  * An ERP solution project for construction management

These projects helped me strengthen my skills in full stack development and real-world application building.

2. Do you own a personal computer?

Yes, I own a personal laptop and use it for development work and learning.

3. Describe your development environment. (Your OS, IDE, Editor and Config manager if any)

I use Windows as my operating system and Visual Studio Code (VS Code) as my primary IDE/editor for development. I use Git and GitHub for version control and project management.


## Social Profile

2. Personal website, blog or something you want us to see.

Portfolio: https://pranav-portfolio-5p8k.vercel.app
GitHub: https://github.com/pranavkpv
LinkedIn: https://www.linkedin.com/in/pranav-raj-k-p-v-2147b1270/


## The real stuff.

1. Which all programming languages are installed on your system.

I have JavaScript, TypeScript, Node.js, Next.js, MongoDB, and MySQL installed on my system for full stack development.

2. Write a function that takes a number and returns a list of its digits in an array.

function getDigits(num) {
    return num.toString().split('').map(Number);
}

3. Remove duplicates of an array and returning an array of only unique elements

function removeDuplicates(arr) {
    return [...new Set(arr)];
}

console.log(removeDuplicates([1, 2, 2, 3, 4, 4, 5]));
// Output: [1, 2, 3, 4, 5]

4. Write function that translates a text to Pig Latin and back. English is translated to Pig Latin by taking the first letter of every word, moving it to the end of the word and adding ‘ay’. “The quick brown fox” becomes “Hetay uickqay rownbay oxfay”.

function toPigLatin(text) {
    return text
        .split(' ')
        .map(word => word.slice(1) + word[0] + 'ay')
        .join(' ');
}

function fromPigLatin(text) {
    return text
        .split(' ')
        .map(word => {
            let original = word.slice(0, -2); // remove 'ay'
            return original.slice(-1) + original.slice(0, -1);
        })
        .join(' ');
}

// Example
const english = "The quick brown fox";

const pigLatin = toPigLatin(english);
console.log(pigLatin);
// Hetay uickqay rownbay oxfay

const normalText = fromPigLatin(pigLatin);
console.log(normalText);
// The quick brown fox

5. Write a function that rotates a list by `k` elements. For example [1,2,3,4,5,6] rotated by `2` becomes [3,4,5,6,1,2]. Try solving this without creating a copy of the list. How many swap or move operations do you need?

function rotateArray(arr, k) {
    const n = arr.length;
    k = k % n;

    // Reverse helper
    function reverse(start, end) {
        while (start < end) {
            [arr[start], arr[end]] = [arr[end], arr[start]];
            start++;
            end--;
        }
    }

    // Reverse first k elements
    reverse(0, k - 1);

    // Reverse remaining elements
    reverse(k, n - 1);

    // Reverse whole array
    reverse(0, n - 1);

    return arr;
}

// Example
console.log(rotateArray([1, 2, 3, 4, 5, 6], 2));
// Output: [3, 4, 5, 6, 1, 2]