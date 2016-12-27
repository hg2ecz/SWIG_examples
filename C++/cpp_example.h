class Example {
    public:
	Example(const char *s);
	char *Get();
    private:
	char s[200];
	int counter=0;
};
