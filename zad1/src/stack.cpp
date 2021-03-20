#include <memory>
#include <iostream>

struct stack_frame {
    int value;
    std::unique_ptr<stack_frame> next;

    stack_frame(int val) : value(val) {}

};

struct stack {
    std::unique_ptr<stack_frame> top;

    stack() {
        top = nullptr;
    }

    void push(int val) {
        auto *new_frame = new stack_frame(val);
        new_frame->next.reset(top.release());
        top.reset(new_frame);

    }

    int pop() {
        auto frame = top.release();
        top.reset(frame->next.release());
        return frame->value;
    }

    int peek() {
        return top->value;
    }
};

