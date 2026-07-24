## About You.

1. Introduce yourself.

I'm a Full Stack Developer proficient at developing modern, scalable, and optimized web applications via React, Next.js, and Node.js. I thrive on transforming complex issues into simple, elegant, and intuitive solutions.
In front-end development, I leverage my expertise in React and Next.js to create intuitive, high-performing, and accessible web application interfaces. In back-end development, I apply my knowledge of Node.js to architect and develop powerful API systems, databases, and scalable server architectures.
I appreciate working within the entire development cycle of web applications – from ideation and architectural planning to optimization and deployment. In addition, I am enthusiastic about writing high-quality and scalable code.
I'm always open to joining exciting projects and initiatives where I can apply my skills.

2. Do you own a personal computer?

YES

3. Describe your development environment. (Your OS, IDE, Editor and Config manager if any)

I use linux and use vs code for coding

## Social Profile

1. Your StackOverflow Profile url.
   I don't have a stackoverflow profile but here's my github- https://github.com/Nitin05karunakaran
2. Personal website, blog or something you want us to see.
   portfolio-https://portfolio-nitin-karunakaran.vercel.app/
   This website was made entirely by me for my company- https://www.tutorcomp.com/

## The real stuff.

1. Which all programming languages are installed on your system.

React.js,next.js and node.js

2. Write a function that takes a number and returns a list of its digits in an array.

function digitsInArr(n) {
const digits = [];

for (const digit of String(n)) {
digits.push(Number(digit));
}

return digits;
}

3. Remove duplicates of an array and returning an array of only unique elements

function uniqueNums(arr) {
let obj = {};

for (let num of arr) {
obj[num] = true;
}

return Object.keys(obj).map(Number);
}

4. Write function that translates a text to Pig Latin and back. English is translated to Pig Latin by taking the first letter of every word, moving it to the end of the word and adding ‘ay’. “The quick brown fox” becomes “Hetay uickqay rownbay oxfay”.

function toPigLatin(text) {
return text
.split(" ")
.map(word => word.slice(1) + word[0] + "ay")
.join(" ");
}
