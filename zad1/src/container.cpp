#include <stdlib.h>
#include <list>
#include <stack>
#include <queue>
#include <algorithm>

#include "vector.cpp"

class Container {

    public:
    void fill(size_t n) {
        for(size_t i = 0; i < n; ++i) {
            append(i);
        }
    }

    virtual void append(int val) = 0;
    virtual void insert(size_t idx, int val) = 0;
    virtual size_t find(int val) = 0;
    virtual void remove(size_t idx) = 0;
};

class Array: public Container {
    vector v;

public:
    void append(int val) {
        v.push(val);
    }

    void insert(size_t idx, int val) {
        v.insert(idx, val);
    }

    size_t find(int val) {
        return v.find(val);
    }

    void remove(size_t idx) {
        std::cout << "now deleting: " << idx << std::endl;
        v.remove(idx);
    }
};

class List: public Container {
    std::list<int> list;

public:
    void append(int val) {
        list.push_back(val);
    }

    void insert(size_t idx, int val) {
        auto it = list.begin();
        std::advance(it, idx);
        list.insert(it, val);
    }

    size_t find(int val) {
        auto it = std::find(list.begin(), list.end(), val);
        return std::distance(list.begin(), it);
    }

    void remove(size_t idx) {
        auto it = list.begin();
        std::advance(it, idx);
        list.erase(it);
    }
};

class Stack: public Container {
    std::stack<int> stack;

public:
    void append(int val) {
        stack.push(val);
    }

    void insert(size_t idx, int val) {
        std::stack<int> temp;
        for(size_t i = 0; i < idx; ++i) {
            temp.push(stack.top());
            stack.pop();
        }
        stack.push(val);
        while(!temp.empty()) {
            stack.push(temp.top());
            temp.pop();
        }
    }

    size_t find(int val) {
        std::stack<int> temp;
        size_t i = 0;
        while(!stack.empty()) {
            int elem = stack.top();
            if(elem == val) {
                break;
            }
            temp.push(elem);
            stack.pop();
            ++i;
        }
        while(!temp.empty()) {
            stack.push(temp.top());
            temp.pop();
        }
        return i;
    }

    void remove(size_t idx) {
        std::stack<int> temp;
        for(size_t i = 0; i < idx; ++i) {
            temp.push(stack.top());
            stack.pop();
        }
        stack.pop();
        while(!temp.empty()) {
            stack.push(temp.top());
            temp.pop();
        }
    }
};

class Queue: public Container {
    std::queue<int> queue;

public:
    void append(int val) {
        queue.push(val);
    }

    void insert(size_t idx, int val) {
        std::queue<int> temp;
        for(size_t i = 0; i < idx; ++i) {
            temp.push(queue.front());
            queue.pop();
        }
        temp.push(val);
        while(!queue.empty()) {
            temp.push(queue.front());
            queue.pop();
        }
        queue.swap(temp);
    }

    size_t find(int val) {
        std::queue<int> temp;
        size_t i = 0;
        while(!queue.empty()) {
            int elem = queue.front();
            if(elem == val) {
                break;
            }
            temp.push(elem);
            queue.pop();
            ++i;
        }
        while(!queue.empty()) {
            temp.push(queue.front());
            queue.pop();
        }
        queue.swap(temp);
        return i;
    }

    void remove(size_t idx) {
        std::queue<int> temp;
        for(size_t i = 0; i < idx; ++i) {
            temp.push(queue.front());
            queue.pop();
        }
        queue.pop();
        while(!queue.empty()) {
            temp.push(queue.front());
            queue.pop();
        }
        queue.swap(temp);
    }
};