using System;
using System.Runtime.InteropServices;

namespace CExample
{
    public class Example_main
    {
        [DllImport("./c_example.so", EntryPoint="Example")]
        public static extern int Example(string str);

        [DllImport("./c_example.so", EntryPoint="Get")]
        public static extern IntPtr Get(int x);

        public static void Main(string[] args)
        {
            int a = Example("Teszt-A");
            int b = Example("Teszt-B");

            Console.WriteLine(Marshal.PtrToStringAuto(Get(a)));
            Console.WriteLine(Marshal.PtrToStringAuto(Get(b)));
            Console.WriteLine(Marshal.PtrToStringAuto(Get(a)));
            Console.WriteLine(Marshal.PtrToStringAuto(Get(a)));
            Console.WriteLine(Marshal.PtrToStringAuto(Get(a)));
            Console.WriteLine(Marshal.PtrToStringAuto(Get(a)));
            Console.WriteLine(Marshal.PtrToStringAuto(Get(b)));
        }
    }
}
