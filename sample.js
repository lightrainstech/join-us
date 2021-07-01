const rotateElements = (arr, k) => {
    // let newArray = [];
    for(let i=0; i<k; i++) {
        arr.push(arr.shift())
    }
        return arr;
}

const sampleArray = [1,2,3,4,5,6,7,8,9]
const k = 2
console.log(rotateElements(sampleArray, k))