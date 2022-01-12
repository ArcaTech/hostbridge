struct TestStruct {
	int a;
	unsigned int b;
};

void init_stuff();
void hello(char *name);
void *get_handle_from_struct(struct TestStruct s);
struct TestStruct get_struct_from_handle(void *handle);
void gomain();