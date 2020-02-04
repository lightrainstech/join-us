About ME

I am RUPESH KUMAR living in Kuttipuzha, Kunnukara, completed my 12th from Stmarys senior secondary school,faridabad.
I have completed my graduation in BTech ECE from Sree Nararyana Guru INstitute of Science and technology, North Parvur during period of 2014-2018.
I was working in Qriotechnolgies as a software developer for past 9 months.
I was mainly working in NODE JS to provide necessary maintenance and upgrade to an IOT based HOME AUTOMATION system.
During my working period i have also interacted with aws products(dynamo DB,rds,IOT core,Lamda,apiGAteway), opencv and bitbucket.
I have completed a FULL Stack Developer course from ICT Academy and Associate Blockchain course from IIITMK.
I have good fluency in English, Malayalam and Hindi.
I have a keen interest in PRIVATE BLOCKCHAIN system and AI.


Yes i do own a personel computer with having os Windows 10 OS. i use Visual Studio Code and jyupiter notebook development.
I have node js, python installed.



Solution to the problems

/*

Author:Rupesh Kumar

*/


//declaring variables
var number = 1563434;
var text_array = ["r","o","s","",'',"t","e","r","n","e","t"];
var input = "there the chicken";
var rotatingElement=["R","U","P","e","s","h"];
var newArray = [];
var array = [];
var no_of_rotation = 2;

//Function to convert number to list
function number_to_list(numberToString){
    var number_to_string = numberToString.toString()
    for (i=0;i<number_to_string.length;i++) {
        array.push(parseInt(number_to_string[i]));
    }
    return array;
}


//Returns unique elements of an array without empty spaces 
function uniquElements(Unique){
    var newArray = [];
    // console.dir(Unique)
    for (i=0;i<Unique.length;i++){
        if(newArray.indexOf(Unique[i])===-1 && Unique[i] !=='' && Unique[i] !==""){
            newArray.push(Unique[i]);
        }else{
            //do nothing
        }
    }
    return newArray;
}

//Converts english text to pig Latin
function pig_latin(input){
    var array_split_elements = input.split(" ");
    for (i=0;i<array_split_elements.length;i++){
        // console.log(array_split_elements[i]+"==>");
        var each_elements =  array_split_elements[i];
        var each_elements_arayed = each_elements.split('');
        // console.log(y[0]);
        var first = each_elements_arayed[0];
        for (j=0;j<each_elements.length-1;j++){
            each_elements_arayed[j] = each_elements_arayed[j+1];
        }
        each_elements_arayed.pop();
        var str= each_elements_arayed.join('');
        str = str + first + "ay";
        newArray.push(str);
        
    }
    var out = newArray.join(' ');
    return out
}

//Function for rotation for k=2(no of rotation = 2)
function rotation(text_input,no_of_rotation){
    for (k=0;k<no_of_rotation;k++){
        var last = text_input[0];
        for(i=0;i<text_input.length;i++){
            if(i==text_input.length-1){
                text_input[i] = last
            }else{
                text_input[i]=text_input[i+1]
            }
        }
    }
    return text_input;
}

//Here functions are called
var array_returned = number_to_list(number);
var unique_elements_Returned = uniquElements(text_array);
var piglatin = pig_latin(input);
var rotated_Out = rotation(rotatingElement, no_of_rotation);

//Here results are printed
console.log("\n"+"Original Number");
console.log(number);
console.log('\n')
console.log("\n"+"Converted Number to list");
console.dir(array_returned);
console.log('\n')


console.log("Input List");
console.dir(text_array);
console.log('\n')
console.log("Sorted unique elements list");
console.dir(unique_elements_Returned);
console.log('\n')


console.log("Input text");
console.dir(input);
console.log("Pig latin text");
console.dir(piglatin);


console.log("Input");
console.dir(text_array);
console.log('\n')
console.log("After given rotation");
console.dir(rotated_Out);
console.log('\n')

/*END of code*/
