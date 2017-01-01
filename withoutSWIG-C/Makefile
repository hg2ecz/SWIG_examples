all:
	make -f Makefile-sofile
	rm -f c_example.o

	make -f Makefile-CSharp
	make -f Makefile-CSharp_as_class

clean:
	make -f Makefile-sofile clean

	make -f Makefile-CSharp clean
	make -f Makefile-CSharp_as_class clean
