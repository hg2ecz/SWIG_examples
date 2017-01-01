using System;

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
