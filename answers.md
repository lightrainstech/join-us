# Join Lightrains — Answers

## About You

### 1. Introduce yourself.

I am Subin Sasi, a Front-End Developer with 4+ years of professional experience in web development. I primarily work with JavaScript, React.js, Next.js, HTML, CSS, Tailwind CSS, and WordPress.
I have experience building responsive websites and web applications, developing reusable frontend components, integrating REST APIs, and working with CMS platforms such as WordPress and Payload CMS. I also have experience with PHP and database-related web development.
I enjoy solving problems, learning new technologies, and building clean, responsive, and user-friendly interfaces.

### 2. Do you own a personal computer?

Yes, I own a personal computer that I use for development and other technical work.

### 3. Describe your development environment. (Your OS, IDE, Editor and Config manager if any)

- **Operating System:** Windows 11
- **IDE / Editor:** Visual Studio Code
- **Terminal:** Windows PowerShell
- **Version Control:** Git and GitHub
- **Package Managers:** npm
- **Frameworks / Libraries:** React.js, Next.js, Tailwind CSS, and other project-specific dependencies

I use Visual Studio Code as my primary editor and Git for version control. I manage JavaScript project dependencies using npm and use PowerShell for running development commands.

## Social Profile

### 1. Your StackOverflow Profile url.

I do not currently have an active Stack Overflow profile.

### 2. Personal website, blog or something you want us to see.

- **Portfolio:** https://sub1n.netlify.app/works
- **LinkedIn:** https://www.linkedin.com/in/subin-sasi

## The real stuff

### 1. Which all programming languages are installed on your system.

My primary development languages and runtimes include:

- JavaScript
- PHP
- Python
- HTML
- CSS
- SQL

I primarily use JavaScript for frontend development, PHP for WordPress and backend-related work, and SQL when working with databases.

### 2. Write a function that takes a number and returns a list of its digits in an array.

```javascript
function getDigits(number) {
  return Math.abs(number)
    .toString()
    .split('')
    .map(Number);
}

console.log(getDigits(12345)); // [1, 2, 3, 4, 5]
console.log(getDigits(-908));  // [9, 0, 8]
```


### 3. Remove duplicates of an array and returning an array of only unique elements

```javascript
function removeDuplicates(array) {
  return [...new Set(array)];
}

console.log(removeDuplicates([1, 2, 2, 3, 4, 4, 5]));
// [1, 2, 3, 4, 5]

console.log(removeDuplicates(['a', 'b', 'a', 'c', 'b']));
// ['a', 'b', 'c']
```


### 4. Write function that translates a text to Pig Latin and back.

The given rule moves the first letter of each word to the end and adds `ay`.

```javascript
function toPigLatin(text) {
  return text
    .split(/\s+/)
    .map(word => {
      if (!word) return word;
      return word.slice(1) + word[0] + 'ay';
    })
    .join(' ');
}

function fromPigLatin(text) {
  return text
    .split(/\s+/)
    .map(word => {
      if (!word || word.length < 3) return word;

      const withoutAy = word.slice(0, -2);

      return withoutAy.slice(-1) + withoutAy.slice(0, -1);
    })
    .join(' ');
}

const originalText = "The quick brown fox";

const pigLatinText = toPigLatin(originalText);
console.log(pigLatinText);
// Hetay uickqay rownbay oxfay

console.log(fromPigLatin(pigLatinText));
// The quick brown fox
```


### 5. Write a function that rotates a list by k elements. For example [1,2,3,4,5,6] rotated by 2 becomes [3,4,5,6,1,2]. Try solving this without creating a copy of the list. How many swap or move operations do you need?

```javascript
function rotateLeft(array, k) {
  const n = array.length;

  if (n === 0) return array;

  k = k % n;

  if (k === 0) return array;

  function reverse(start, end) {
    while (start < end) {
      [array[start], array[end]] = [array[end], array[start]];
      start++;
      end--;
    }
  }

  // Reverse the first k elements
  reverse(0, k - 1);

  // Reverse the remaining elements
  reverse(k, n - 1);

  // Reverse the entire array
  reverse(0, n - 1);

  return array;
}

const numbers = [1, 2, 3, 4, 5, 6];

console.log(rotateLeft(numbers, 2));
// [3, 4, 5, 6, 1, 2]
```

For the example [1,2,3,4,5,6] rotated by 2, the algorithm performs 6
swap operations in total: 1 swap for the first reversal, 2 swaps for
the second reversal, and 3 swaps for the final reversal.

In general, the number of swaps is:
floor(k / 2) + floor((n - k) / 2) + floor(n / 2),
where k is reduced modulo n.
