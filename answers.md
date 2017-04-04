About You
1: I’m Ranjith Jayaram, working with Polus software for the past 1.8 years, as a software developer. I used to work mostly  with javascript frameworks, like Angular and Node Js. 
Recently i was working with an enterprise application for Broad Institute of MIT and Harvard, named SIA which made use of Angular Js, Node (sails Js), Mongo Db and shell scripts. I was the major contributor in that project. Along with that i used to work mostly on a couple of maintenance projects , which consisted of the same technology stack. All these applications are deployed on Linux environment with docker container technology. I also has a background of C#.NET. 
	I had my graduation from Anna university, where i hold an engineering degree in Computer Science. I hails from Palakkad, Kerala.
	Although I love my current role, I feel I’m now ready for a more challenging assignment and this position really excites me especially the way you are scheduling the interview.

2: Yes, I do own a personal computer but i am not using it for development .

3: My development environment
Operating System  - WInodws 10 , 7.1.1503
IDE- Eclipse, Visual Studion
Editors- Notepad ++, Sublime Text, Brackets.

Social Profile
1: http://stackoverflow.com/users/5474383/ranjith-j

The real stuff.


1: Programming Languages installed in my system:

Node Js
.NET framework
Java - 1.8
Python -2.7


2:

var a=prompt("Enter Number");
function returnDigits(a)
{	
	var input = a.toString();
	var arr  = [];
	console.log(input);
	if(!Number.isInteger(input))
	{
		if(isNaN(input))
		{
		alert("Not a Number.. Please enter a valid number");
		return;
		}
		else
		{
		Input =  input.replace(/\D/g, '');
		}
	}
	var arr = input.split('').map(Number);
	return arr;
}


3:

function duplicates(arr) 
{
	var len = arr.length;
	var result = [];
	var obj = {};
	for (var i=0; i<len; i++)
	{
		obj[arr[i]]=0;
	}
	for (i in obj) 
	{
		result.push(i);
	}
	return result;
}
var arr = [5,5,7,4,5,4,5,4,5,454];

4:

function returnPiglatin(str)
{
var strArr=str.split(" ");
var len =strArr.length;
var result="";
for(var i=0;i<len; i++)
{
	var res = strArr[i].substr(1) + strArr[i].substr(0, 1)+"ay ";
	result=result.concat(res);
}
return result;
}


5:
var num = prompt("Enter Number");
function rotationElements(num )
{
	var array = [3,1,2,5,8,9,10];
	for(var i = 0;  i < num;  i++)
	{
		var first  =  array.splice(0,1);
		array.push( first[0] ); 
	}
	return array;
}








