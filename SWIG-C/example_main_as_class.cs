using System;

namespace CExample
{

    public class Example
    {
        private int descr;
        public Example(string s)
        {
            this.descr = c_example.Example(s);
        }

        public string Get()
        {
            return c_example.Get(this.descr);
        }
    }

    public class Example_main
    {
        public static void Main(string[] args)
        {
            Example a = new Example("Teszt-A");
            Example b = new Example("Teszt-B");

            Console.WriteLine(a.Get());
            Console.WriteLine(b.Get());
            Console.WriteLine(a.Get());
            Console.WriteLine(a.Get());
            Console.WriteLine(a.Get());
            Console.WriteLine(a.Get());
            Console.WriteLine(b.Get());
        }
    }
}
