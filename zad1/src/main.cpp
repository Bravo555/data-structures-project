#include <chrono>
#include <random>
#include <iostream>
#include <sstream>
#include <cassert>
#include <iomanip>
#include <queue>
#include <stack>
#include <functional>

#include "vector.cpp"
#include "linked_list.cpp"
#include "stack.cpp"

std::mt19937 mt;
std::uniform_int_distribution<> dist(-10000, 10000);

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

void benchmark(std::string text, int n, std::function<void()> f) {
    std::cout << text << " - size " << to_hr_size(sizeof(int) * n) << std::endl;

    auto start = std::chrono::high_resolution_clock::now();
    f();
    auto end = std::chrono::high_resolution_clock::now();
    auto time = std::chrono::duration_cast<std::chrono::microseconds>(end - start);

    std::cout << "finished in: " << time.count() / 1000 << "ms" << std::endl;
    std::cout << std::chrono::duration_cast<std::chrono::nanoseconds>(end - start).count() / n << "ns / elem" << std::endl;
    auto seconds = (double)time.count() / (1000 * 1000);
    std::cout << "seconds: " << seconds << std::endl;
    std::cout << "stuff: " << (double)(sizeof(int) * n) << std::endl;
    std::cout << to_hr_size((double)(sizeof(int) * n) / seconds) << "/s\n" << std::endl;
}

int main() {
    int n = 1024 * 1024 * 64;

    benchmark("creating array", n, [n]() {
        vector vec;
        for(int i = 0; i < n; ++i) {
            vec.push(dist(mt));
        }
    });

    // TODO fix bad human readable size representation for this test case
    vector v;
    std::uniform_int_distribution<size_t> random_index(0, n);
    for(int i = 0; i < n - 1; ++i) {
        v.push(dist(mt));
    }
    size_t idx = n-1;
    v.insert(idx, 69420);
    std::cout << "value placed at: " << idx << std::endl;

    benchmark("searching array", n, [&v]() {
        size_t idx2 = v.find(69420);
        std::cout << "value found at: " << idx2 << "\n";
    });

    benchmark("creating linked list", n, [n]() {
        linked_list list;
        for(int i = 0; i < n; ++i) {
            list.push_back(dist(mt));
        }
    });

    benchmark("creating stack", n, [n]() {
        std::stack<int> stack;
        for(int i = 0; i < n; ++i) {
            stack.push(dist(mt));
        }
    });

    benchmark("creating queue", n, [n]() {
        std::queue<int> queue;
        for(int i = 0; i < n; ++i) {
            queue.push(dist(mt));
        }
    });

    return 0;
}
