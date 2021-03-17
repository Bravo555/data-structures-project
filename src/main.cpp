#include <chrono>
#include <random>
#include <iostream>
#include <sstream>
#include <cassert>
#include <iomanip>
#include <queue>

#include "linked_list.cpp"
#include "stack.cpp"


std::string to_hr_size(int size) {
    std::ostringstream os;
    if(size < 1024) {
        os << size << "B";
    }
    else if(size < 1024 * 1024) {
        os << size / 1024 << "kiB";
    }
    else if(size < 1024 * 1024 * 1024) {
        os << size / 1024 / 1024 << "MiB";
    }
    else if(size < 1024L * 1024 * 1024 * 1024) {
        os << size / 1024 / 1024 / 1024 << "GiB";
    }
    return os.str();
}

void array_test(int n) {
    int *arr = new int[n];

    std::mt19937 mt;
    std::uniform_int_distribution<> dist(-10000, 10000);

    std::cout << "filling array of size " << to_hr_size(sizeof(int) * n) << std::endl;
    auto start = std::chrono::high_resolution_clock::now();

    for(int i = 0; i < n; ++i) {
        arr[i] = dist(mt);
    }

    auto end = std::chrono::high_resolution_clock::now();
    auto time = std::chrono::duration_cast<std::chrono::microseconds>(end - start);
    std::cout << "filled the array in: " << time.count() / 1000 << "ms" << std::endl;
    std::cout << std::chrono::duration_cast<std::chrono::nanoseconds>(end - start).count() / n << "ns / elem" << std::endl;
    auto seconds = (float)time.count() / (1000 * 1000);
    std::cout << to_hr_size((float)(sizeof(int) * n) / seconds) << "/s" << std::endl;
}

void linked_list_test(int n) {
    linked_list list;

    std::mt19937 mt;
    std::uniform_int_distribution<> dist(-10000, 10000);

    std::cout << "filling linked list of size " << to_hr_size(sizeof(int) * n) << std::endl;
    auto start = std::chrono::high_resolution_clock::now();

    for(int i = 0; i < n/2; ++i) {
        list.push_front(dist(mt));
        list.push_back(dist(mt));
    }

    auto end = std::chrono::high_resolution_clock::now();
    auto time = std::chrono::duration_cast<std::chrono::microseconds>(end - start);
    std::cout << "filled the linked list in: " << time.count() / 1000 << "ms" << std::endl;
    std::cout << std::chrono::duration_cast<std::chrono::nanoseconds>(end - start).count() / n << "ns / elem" << std::endl;
    auto seconds = (float)time.count() / (1000 * 1000);
    std::cout << to_hr_size((float)(sizeof(int) * n) / seconds) << "/s" << std::endl;
}

void stack_test() {
    stack s;

    s.push(1);
    s.push(2);
    s.push(3);

    std::cout << s.pop() << std::endl;
    std::cout << s.pop() << std::endl;
    std::cout << s.pop() << std::endl;
}

void queue_test(int n) {
    std::queue<int> queue;

    std::mt19937 mt;
    std::uniform_int_distribution<> dist(-10000, 10000);

    std::cout << "filling queue of size " << to_hr_size(sizeof(int) * n) << std::endl;
    auto start = std::chrono::high_resolution_clock::now();

    for(int i = 0; i < n; ++i) {
        queue.push(dist(mt));
    }

    auto end = std::chrono::high_resolution_clock::now();
    auto time = std::chrono::duration_cast<std::chrono::microseconds>(end - start);
    std::cout << "filled the linked list in: " << time.count() / 1000 << "ms" << std::endl;
    std::cout << std::chrono::duration_cast<std::chrono::nanoseconds>(end - start).count() / n << "ns / elem" << std::endl;
    auto seconds = (float)time.count() / (1000 * 1000);
    std::cout << to_hr_size((float)(sizeof(int) * n) / seconds) << "/s" << std::endl;
}

int main() {
    int n = 1024 * 1024 * 16;

    array_test(n);
    linked_list_test(n);
    // stack_test();
    queue_test(n);

    return 0;
}
