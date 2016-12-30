class Example {
    public:
	Example(const char *s);
//	virtual char *Get();
//	virtual ~Example();
	char *Get();
	~Example();
    private:
	char s[200];
	int counter=0;
};
