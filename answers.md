

//code compailed and run in nodejs
//most problems are solved without using JavaScript enrich predefined functions

// 1: to print all digits of an array

    function getDigitsArray(number){

        let digitArray=[];
        while(number!=0){
            digitArray.push(number%10);
            number=Math.floor(number/10)
        }
        return  digitArray;
    }

   function reverseArray(array){
        temp=[];
        for(let i=array.length-1;i>=0;i--){
            temp.push(array[i]);
        }
        return temp;
    }

   console.log('digits in given number')
   console.log(reverseArray(getDigitsArray([123])));

 ------------------------------------------------------------------------------------------   
2: Remove duplicates from array

    function removeDuplicates(array){
        temp=[];
        for(let i=0;i<array.length;i++){
            if(array[i]!=null){
                temp.push(array[i]);
                for(let j=i+1;j<array.length;j++){
                    if(array[i]===array[j]){
                        //console.log(i+''+j);
                        array[j]=null;
                    }
                }
            }
           
        }
        return temp;
    }

    console.log('After removing duplicates from given array:')
    console.log(removeDuplicates([1,5,8,9,8]));

-------------------------------------------------------------------------------------------

4: Rotate array k times 

    function shiftArray(array,k){
       
        let j=0;
        while(j<k){
            let i=0;
            let temp=array[0];
            while(i<array.length){
                array[i]=array[i+1];
                i++;
            }
            array[array.length-1]=temp;
            j++;
        }
 

        return array;
     
    }

     console.log( 'rotate array by 2 position :);
     console.log( +shiftArray([1,2,3,4,5,6],2));

-----------------------------------------------------------------------------------------

3: convert sentence to pigLatin

    function converttoPigLatin(sentence){
        var newString='';
         let start=0;
        for(let i=0;i<=sentence.length;i++){
          
            if(sentence[i]===' ' || i===sentence.length){
             
                let firstLetter=sentence[start];
                while(start<i-1){
                   
                    newString+=sentence[start+1];
                    if(start===0) newString=newString[0].toUpperCase();
                    start++;
                }
                newString+=firstLetter.toLowerCase()+'ay ';
                start=i+1;
            
            }
        }
        

        return newString;
        
    }

       console.log('corresponding pigLatin sentence : ');
       console.log(converttoPigLatin(new String('The quick brown fox')));
