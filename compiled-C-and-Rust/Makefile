all:
	make -f Makefile-sofile
	rm -f c_example.o
	make -f Makefile-main
	make -f Makefile-main_as_class

clean:
	make -f Makefile-main   clean
	make -f Makefile-sofile clean
	make -f Makefile-main_as_class clean
