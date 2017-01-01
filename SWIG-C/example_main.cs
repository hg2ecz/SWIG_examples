using System;

public class Example_main {
    public static void Main(string[] args)
    {
        var a = c_example.Example("Teszt-A");
        var b = c_example.Example("Teszt-B");

        Console.WriteLine(c_example.Get(a));
        Console.WriteLine(c_example.Get(b));
        Console.WriteLine(c_example.Get(a));
        Console.WriteLine(c_example.Get(a));
        Console.WriteLine(c_example.Get(a));
        Console.WriteLine(c_example.Get(a));
        Console.WriteLine(c_example.Get(b));
    }
}
