struct TestStruct {
	int a;
	unsigned int b;
};

void init_stuff();
void hello(char *name);
void test_pass_struct(struct TestStruct s);
void *test_handle(struct TestStruct s);
struct TestStruct get_struct_from_handle(void *handle);
void gomain();