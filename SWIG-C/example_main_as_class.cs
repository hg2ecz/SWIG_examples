using System;
using System.Runtime.InteropServices;

namespace CExample
{
    public class ExampleHelper
    {
        [DllImport("./c_example.so", EntryPoint="Example")]
        public static extern int Example(string str);


        [DllImport("./c_example.so", EntryPoint="Get")]
        public static extern IntPtr Get(int x);
    }

    public class Example
    {
        public static void Main(string[] args)
        {
            using (var exampleA = new ExampleWrapper("Hello C# - A"))
            {
                using (var exampleB = new ExampleWrapper("Hello C# - B"))
                {
                    Console.WriteLine(exampleA.Get());
                    Console.WriteLine(exampleA.Get());
                    Console.WriteLine(exampleA.Get());
                    Console.WriteLine(exampleA.Get());
                    Console.WriteLine(exampleB.Get());
                }
            }
        }
    }

    public class ExampleWrapper : IDisposable
    {
        private int exampleHandler;
        private bool disposed;
	
        public ExampleWrapper(string jarg)
        {
            Console.WriteLine("ST Example: " + jarg);
            this.exampleHandler = ExampleHelper.Example(jarg);
            Console.WriteLine("END Example: " + this.exampleHandler);
        }

        public string Get()
        {
            Console.WriteLine("ST Get: " + this.exampleHandler);
            var result = ExampleHelper.Get(exampleHandler);
            var str = Marshal.PtrToStringAuto(result);
            Console.WriteLine("END Get: " + str);
            return str;
        }

        # region Dispose pattern

        public void Dispose()
        {
            Console.WriteLine("ST Dispose: " + this.exampleHandler);
            Dispose(true);
        }

	private void Dispose(bool disposing)
        {
            if (!disposed)
            {
                if (!disposing)
                {
                    // Dispose managed objects
                }

                // Dispose unmanaged (native) objects
		disposed = true;
                GC.SuppressFinalize(this);
            }
        }

        #endregion
    }
}

