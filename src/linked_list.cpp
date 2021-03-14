#include <memory>

struct linked_list_node {
    int value;
    std::unique_ptr<linked_list_node> next;

    linked_list_node(int value) : value(value) {}
};

struct linked_list {
    std::unique_ptr<linked_list_node> head;

    linked_list() {
        head = nullptr;
    }

    void append(int val) {
        if(head == nullptr) {
            head.reset(new linked_list_node(val));
            return;
        }

        auto curr = head.get();
        while(curr->next != nullptr) {
            curr = curr->next.get();
        }
        curr->next.reset(new linked_list_node(val));
    }
};

