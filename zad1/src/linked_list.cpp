#include <memory>

struct linked_list_node {
    int value;
    std::shared_ptr<linked_list_node> next;
    std::shared_ptr<linked_list_node> prev;

    linked_list_node(int value) : value(value) {}
};

struct linked_list {
    std::shared_ptr<linked_list_node> head;
    std::shared_ptr<linked_list_node> tail;

    linked_list() {
        head = nullptr;
        tail = nullptr;
    }

    void push_back(int val) {
        if(tail == nullptr) {
            head = std::make_shared<linked_list_node>(val);
            tail = head;
            return;
        }

        auto newnode = std::make_shared<linked_list_node>(val);
        newnode->prev = tail;
        tail->next = newnode;
        tail = newnode;
    }

    void push_front(int val) {
        if(head == nullptr) {
            head = std::make_shared<linked_list_node>(val);
            tail = head;
            return;
        }

        auto newnode = std::make_shared<linked_list_node>(val);
        newnode->next = head;
        head->prev = newnode;
        head = newnode;
    }
};

