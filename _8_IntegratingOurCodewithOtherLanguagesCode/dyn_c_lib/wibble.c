//gcc -shared -o libwibble.so -fPIC wibble.c

int wibble_it(int n, int m) {
    return m - 2* n;
}