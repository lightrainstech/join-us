## About You.

1. Introduce yourself.

   Hi, This is Nithya Rajan from Tirunelveli, Tamil Nadu. I am a front end developer having 3 years of experience. Specialized in JavaScript and Angular. Lead the Front End Team for a project for the past 1 year. Currently working in Global Software Solutions, Tirunelveli, Tamil Nadu. I love writing blog posts and contribute to open source as well.

2. Do you own a personal computer?

   Yes, a laptop. Not that much powerful processor, it has only 2gb ram.

3. Describe your development environment. (Your OS, IDE, Editor and Config manager if any)

   Windows, VS Code

## Social Profile

1. Your StackOverflow Profile url. 

    - [StackOverflow Profile](https://stackoverflow.com/users/5013729/bear-nithi) 
    - [StackOverflow Developer Story](https://stackoverflow.com/users/story/5013729)
    - [NPM](https://www.npmjs.com/~bearnithi)

2. Personal website, blog or something you want us to see. 
    -[Bear Nithi Blog](http://bearnithi.com)

## The real stuff.

1. Which all programming languages are installed on your system.
   Node JS, Angular, TypeScript

2. Write a function that takes a number and returns a list of its digits in an array.

   ```javascript
   function printNum(number) {
     for (let i = 0; i <= number; i++) {
       console.log(i);
     }
   }
   ```

3. Remove duplicates of an array and returning an array of only unique elements

   ```javascript
   // es6 way
   function removeDuplicates(arr) {
     return [...new Set(arr)];
   }

   function removeDuplicates(arr) {
     const filteredArr = [];
     for (value of arr) {
       if (filteredArr.indexOf(value) === -1) {
         filteredArr.push(value);
       }
     }
     return filteredArr;
   }
   ```

4. Write function that translates a text to Pig Latin and back. English is translated to Pig Latin by taking the first letter of every word, moving it to the end of the word and adding ‘ay’. “The quick brown fox” becomes “Hetay uickqay rownbay oxfay”.

    ```javascript
    function translateToPigLatin(sentence) {
        return sentence
            .split(" ").map((word, i) => {
                const firstLetter = word[0].toLowerCase();
                const remainingLetters =
                    i === 0 ? word.slice(1)[0].toUpperCase() : word.slice(1);
                return `${remainingLetters}${firstLetter}ay`;
            }).join(" ");
    }
    ```

5. Write a function that rotates a list by `k` elements. For example [1,2,3,4,5,6] rotated by `2` becomes [3,4,5,6,1,2]. Try solving this without creating a copy of the list. How many swap or move operations do you need?

    ```javascript
    function rotateArrElements(arr,number) {
        return arr.concat(arr.splice(0,number))
    }
    ```


### Notes

- bearnithi@gmail.com
- 8248802608