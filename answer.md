About Me
1. My name is Arun. I am a BCA Graduate with 2 Years experiance in IT Support.Seeking an entry level position in development area as a trainee developer.
2. Yes, I own.
3. OS - Windows,IDE - Netbeans, Editor - Adobe Dreameweaver 

Social Profile
1. Linkedin Profile -  https://in.linkedin.com/in/arun-satheesh-panicker-7a5485124

The Real Stuff
1. Programming Languages - Java,PHP,C,C++,HTML,JSP,
2. function for accept a number and return digit

class Digit
{
  statc void main(String arg[])
  {
    int digit = 21;
    int a =0;
    System.out.println("Digits:");
    while(digit>0)
    {
      a = digit%10;
      System.out.println(a);
      digit = digit/10;
    }
  }
}

3.function that rotates a list by k elements
class Digit
{
    public static void main(String arg[])
    {
        int a[] = {1,2,3,4,5,6};
        int temp=0,k;

        k = 2; //no of elements
        for(int i = 0;i<a.length;i++)
        {
            System.out.print(a[i]+" ");//print elements before rotation

        }
        for(int i = 0;i<k;i++)
        {
            temp = a[0];
            for(int j =0;j<a.length-1;j++ )//rotation
            {
                a[j] = a[j+1];
            }
            a[a.length-1] = temp;
            temp = 0;
        }
        System.out.println();

        for(int i = 0;i<a.length;i++)
        {
            System.out.print(a[i]+" ");//print elements after k  rotation
        }
    }
}

