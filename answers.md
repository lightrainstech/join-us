## About You.
### 1. Introduce yourself.

   I am a btech graduate of the year 2018 in computer science and engineering from college of engineering Adoor. I am a passionate programmer and a tech enthusiast . I am a recent gopher and MEAN stack developer . Ive been tinkering with blockchain technologies for almost a year and half . Please find my resume in this repo

### 2. Do you own a personal computer?

   Yes

### 3. Describe your development environment. (Your OS, IDE, Editor and Config manager if any)

   I am using ubuntu 16.04 and mostly use nano and visual studio code .

## Social Profile
### 1. Your StackOverflow Profile url.

[slackoverflow/alvinzach](https://stackoverflow.com/users/4623462/alvin-zachariah)

### 2. Personal website, blog or something you want us to see.

* [Github profile](https://github.com/alvinzach)
* [Resume](https://github.com/alvinzach/join-us/blob/alvin_job_application/Resume.pdf)

## The real stuff.
### 1. Which all programming languages are installed on your system.

* Node+npm+build tools 
* Golang 
* Python 2.x and gcc (preinstalled with ubuntu)

### 2. Write a function that takes a number and returns a list of its digits in an array.

```javascript
function digits(num){
    return num.toString().split("").map((digit)=>{
        return Number(digit)
    })
}
```
```javascript
console.log(digits(111))//returns [1,1,1]
```

### 3. Remove duplicates of an array and returning an array of only unique elements

```javascript
function duplicate(arr){
    for(var i=0;i<arr.length;i++){
        var buff=arr.shift()
        var check=arr.find((digit)=>{
            return digit==buff
        })
        if(!check){
            arr.push(buff)
        }

    }
    return arr
}
```
```javascript
    console.log(duplicate([9,5,6,4,5,6,7,7,8,9])) // returns [ 7, 8, 9, 4, 5, 6 ]
```

### 4. Write function that translates a text to Pig Latin and back. English is translated to Pig Latin by taking the first letter of every word, moving it to the end of the word and adding ‘ay’. “The quick brown fox” becomes “Hetay uickqay rownbay oxfay”.

```Javascript
const pigLatin=(text)=>{
    var pigArray= text.split(" ").map((element)=>{
      var buff=element.substr(0,1)
      return " "+element.substr(1,element.length)+buff+"ay"
    })
    return "".concat(...pigArray)

} 
```

```javascript
console.log(pigLatin("The quick brown fox")) //returns heTay uickqay rownbay oxfay
```

### 5. Write a function that rotates a list by `k` elements. For example [1,2,3,4,5,6] rotated by `2` becomes [3,4,5,6,1,2]. Try solving this without creating a copy of the list. How many swap or move operations do you need?

```javascript
function shifter(shift,arr){
 for(var i=0;i<shift;i++){
    var buff=arr.shift()
    arr.push(buff)
 }
 return arr
}
```

```javascript
console.log(shifter(2,[1,2,3,4,5,6])) // returns [ 3, 4, 5, 6, 1, 2 ]
```
