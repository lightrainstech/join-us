// Function to convert text into pigLatin
const toPigLatin = (text) => {
    const wordsArray = text.split(" ")
    const pigLatinArray = wordsArray.map(element => {
        let firstLetter = element[0];
        let newElement = element.slice(1)
        newElement = newElement + firstLetter + 'ay'
        return newElement
    })
    return pigLatinArray.join(" ")
}

const toText = (text) => {
    const pigLatinArray = text.split(" ")
    const wordsArray = pigLatinArray.map(element => {
        let firstLetter = element[element.length-3];
        let newElement = element.slice(0, element.length-3)
        newElement = firstLetter + newElement
        return newElement
    })
    return wordsArray.join(" ")
}


console.log(toPigLatin("Hii Dibin Jose"))
console.log(toText("iiHay ibinDay oseJay"))