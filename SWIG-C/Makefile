all:
	make -f Makefile-LUAjit
	make -f Makefile-Pypy
	make -f Makefile-CSharp
	make -f Makefile-CSharp_as_class

clean:
	make -f Makefile-LUAjit clean
	make -f Makefile-Pypy   clean; rm -rf __pycache__
	make -f Makefile-CSharp clean
	make -f Makefile-CSharp_as_class clean
