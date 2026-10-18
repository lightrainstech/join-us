# Answers

## About You

### 1. Introduce yourself.
Hi, I'm Deepika P.S, a MERN Stack Developer based in Trivandrum, Kerala. I moved into software development from a civil engineering background (B.Tech, LBS Institute of Technology for Women,CGPA-8.63) and taught myself full stack development. Since June 2024 I've been working at Brototype, where I built and deployed two full-stack applications end to end:

- **GRAVITY**: a consultation booking platform built with Next.js, Node.js, Express, MongoDB and TypeScript. It follows Clean Architecture and SOLID principles, with Socket.IO real-time chat and Stripe payments.
- **Echo-Emporium**: an MVC e-commerce platform with Razorpay payments, Google login, coupons and order management.

Both are deployed on AWS EC2 with Nginx. I enjoy clean architecture, secure API design, and I'm open to relocating to Bangalore.

### 2. Do you own a personal computer?
Yes. (8GB RAM,Windows 11)

### 3. Describe your development environment.
- **OS:** Windows 11
- **Editor / IDE:** VS Code (also Cursor AI)
- **Tools:** Git and GitHub, Postman, Figma
- **Config manager:** None

## Social Profile
Linkedin:🔗https://www.linkedin.com/in/deepika-p-s-2306199a/
GitHub:🔗 https://github.com/Deepik-a

### 2. Personal website, blog or something you want us to see
- Portfolio: https://portfolio-ruby-one-46.vercel.app/
- GitHub: https://github.com/Deepik-a
- LeetCode:https://leetcode.com/u/De_123epika/
Projects:-
- Echo-Emporium :https://ecommerce-echoemporium.onrender.com/
- GRAVITY:https://www.gravityconstruction.co.in/


## The real stuff

1. Which programming languages are installed on your system?
- JavaScript (Node.js)
- TypeScript
- Java
- C
 2. Function that takes a number and returns its digits in an array

function getDigits(num) {
  return num
    .toString()
    .split('')
    .filter((x) => !isNaN(x))
    .map(Number)
}
console.log(getDigits(12345))
console.log(getDigits(-907))


### 3. Remove duplicates from an array

function unique(arr) {
  return [...new Set(arr)];
}

console.log(unique([1, 2, 2, 3, 1, 4]))

### 4. Pig Latin translator (and back)

function toPigLatin(text) {
  return text
    .split(' ')
    .map((word) => word.slice(1) + word[0] + 'ay')
    .join(' ');
}

function fromPigLatin(text) {
  return text
    .split(' ')
    .map((word) => {
      const stripped = word.slice(0, -2); 
      return stripped[stripped.length - 1] + stripped.slice(0, -1);
    })
    .join(' ');
}

const pig = toPigLatin('The quick brown fox');
console.log(pig);                
console.log(fromPigLatin(pig));  


### 5. Rotate a list by `k` elements without copying

function reverse(arr, start, end) {
  while (start < end) {
    [arr[start], arr[end]] = [arr[end], arr[start]];
    start++;
    end--;
  }
}

function rotate(arr, k) {
  const n = arr.length;
  if (n === 0) return arr;
  k = ((k % n) + n) % n; 
  reverse(arr, 0, k - 1);
  reverse(arr, k, n - 1);
  reverse(arr, 0, n - 1);
  return arr;
}

console.log(rotate([1, 2, 3, 4, 5, 6], 2)); 

