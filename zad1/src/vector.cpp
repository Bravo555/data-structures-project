#include <cstdlib>
#include <iostream>

class vector
{
private:
    size_t len;
    size_t capacity;
    int *ptr;

    void realloc() {
        capacity = 2 * capacity;
        int *newbuf = new int[capacity];
        for(size_t i = 0; i < len; ++i) {
            newbuf[i] = ptr[i];
        }
        delete[] ptr;
        ptr = newbuf;
    }

public:
    vector() {
        len = 0;
        capacity = 10;
        ptr = new int[capacity];
    }

    size_t length() {
        return len;
    }

    void push(int val) {
        if(len == capacity) {
            realloc();
        }
        ptr[len++] = val;
    }

    void insert(size_t pos, int val) {
        if(pos > len) {
            // error; can only extend by pushing
            return;
        }

        if(len == capacity) {
            realloc();
        }

        for(size_t i = len++; i > pos; --i) {
            ptr[i-1] = ptr[i];
        }
        ptr[pos] = val;
    }

    void remove(size_t pos) {
        for(size_t i = pos; i < --len; i++) {
            ptr[pos] = ptr[pos+1];
        }
    }

    size_t find(int val) {
        for(size_t i = 0; i < len; ++i) {
            if(ptr[i] == val) {
                return i;
            }
        }
        return len;
    }
};