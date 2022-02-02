typedef void (*rust_callback)(int32_t);
void register_callback(rust_callback);

static inline void RegisterCallback(rust_callback callback) {
	printf("C: RegisterCallback\n");
	register_callback(callback);
}

void start_loop();