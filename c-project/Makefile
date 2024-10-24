
all: main
	valgrind ./main
	#Or use "valgrind --leak-check=full --show-leak-kinds=all --track-origins=yes ./main"

main: main.c ../target/release/libfiatluz.a ../target/release/fiatluz.h
	gcc -o main main.c -L../target/release/ -I../target/release/ -lfiatluz -lpthread -ldl -g -O0
# The "-g -O0" ensures debug options to simplify the life for valgrind

clean:
	rm -f main
