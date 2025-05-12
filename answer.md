About You

1. Introduce yourself.
I'm Hiran Raj, a dedicated Software Engineer from Chengannur, Alappuzha. I specialize in building and deploying web applications with a strong focus on backend development. I'm a BCA graduate who is passionate about creating clean, maintainable code and continuously expanding my technical knowledge to stay at the forefront of software development innovation.

2. Do you own a personal computer?
Yes, I own a personal computer that I use for both work and personal projects.

3. Describe your development environment. (Your OS, IDE, Editor and Config manager if any)
My development environment consists of:

Hardware: HP Pavilion
OS: Windows 10 as my primary operating system
IDE/Editor:

Visual Studio Code as my primary editor with extensions for:

JavaScript/TypeScript development
React/Redux
Tailwind CSS IntelliSense
ESLint for code quality
Prettier for code formatting
Git integration


Version Control: Git with GitHub for repositories
Configuration Management:

Docker for containerization and consistent development environments
nvm for Node.js version management
AWS CLI for cloud resource management


Package Managers: npm
API Testing: Postman for API testing and documentation
Database Tools: MongoDB Compass, pgAdmin for database management


Social Profile
1. Your StackOverflow Profile url.
https://stackoverflow.com/users/30493273/hiran-raj
2. Personal website, blog or something you want us to see.

GitHub Profile: https://github.com/Hiran-2001
LinkedIn: https://www.linkedin.com/in/hiran-raj/
Resume: https://docs.google.com/document/d/10lMr9inuePstTwHrW9buhj3SjoJ4owhGQlWfX9l04O4/edit?tab=t.0


The real stuff.
1. Which all programming languages are installed on your system.
The following programming languages are installed on my system:

JavaScript/Node.js (v20.10.0)
TypeScript (v5.0.4)
Python (v3.12.4)
Java (OpenJDK 19.0.1)
Go (go1.24.2)


2. Write a function that takes a number and returns a list of its digits in an array ?

javascript
 * Takes a number and returns an array of its digits

const getDigits = (num) => {
  const absNum = Math.abs(num); // Handle negative numbers
  return absNum.toString().split('').map(digit => parseInt(digit));
};

// Test cases
console.log(getDigits(5236));  // [5, 2, 3, 6]
console.log(getDigits(0));    // [0]
console.log(getDigits(-5236)); // [5, 2, 3, 6]


3. Remove duplicates of an array and returning an array of only unique elements ?

javascript
 * Takes an Array with duplicate numbers and removes duplicates from the array

const getUniqueElements = (arr) => {
  return [...new Set(arr)];
};

//Test cases
console.log(getUniqueElements([1, 2, 2, 3, 4, 4, 5])); //Output [1, 2, 3, 4, 5]
console.log(getUniqueElements(['a', 'b', 'a', 'c', 'b'])) // Output // ['a', 'b', 'c'];


4. Write function that translates a text to Pig Latin and back. English is translated to Pig Latin by taking the first letter of every word, moving it to the end of the word and adding ‘ay’. “The quick brown fox” becomes “Hetay uickqay rownbay oxfay”.

javascript

 function toPigLatin(text) {
    const words = text.split(' ');
    const pigLatinWords = [];

    for (let i = 0; i < words.length; i++) {
        const word = words[i];

        if (word.length <= 1) {
            pigLatinWords.push(word);
            continue;
        }

        const isCapitalized = word[0] === word[0].toUpperCase();

        const pigLatinWord = word.slice(1) + word[0] + 'ay';

        if (isCapitalized) {
            pigLatinWords.push(
                pigLatinWord[0].toUpperCase() +
                pigLatinWord.slice(1).toLowerCase()
            );
        } else {
            pigLatinWords.push(pigLatinWord.toLowerCase());
        }
    }

    return pigLatinWords.join(' ');
}

function toEnglish(text) {
    const words = text.split(' ');
    
    const englishWords = [];
    
    for (let i = 0; i < words.length; i++) {
        const word = words[i];
        
        if (word.length <= 3) {
            englishWords.push(word);
            continue;
        }
        
        if (!word.toLowerCase().endsWith('ay')) {
            englishWords.push(word);
            continue;
        }
        
        const isCapitalized = word[0] === word[0].toUpperCase();
        
        const englishWord = word[word.length - 3] + 
            word.slice(0, word.length - 3);
        
        if (isCapitalized) {
            englishWords.push(
                englishWord[0].toUpperCase() + 
                englishWord.slice(1).toLowerCase()
            );
        } else {
            englishWords.push(englishWord.toLowerCase());
        }
    }
    
    return englishWords.join(' ');
}

console.log(toPigLatin("The quick brown fox")); // Output: "Hetay uickqay rownbay oxfay"

console.log(toEnglish("Hetay uickqay rownbay oxfay")); // Output: "The quick brown fox"

