CC = gcc
AR = ar

# Targets
all: app

libcontainer.a: container.o
	$(AR) rcs libcontainer.a container.o

container.o: container.c container.h
	$(CC) -c container.c -o container.o

app: application.o libcontainer.a
	$(CC) application.o -L. -lcontainer -o app

application.o: application.c container.h
	$(CC) -c application.c -o application.o

clean:
	rm -f *.o *.a app
