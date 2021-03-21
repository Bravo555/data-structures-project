#include <chrono>
#include <random>
#include <iostream>
#include <sstream>
#include <cassert>
#include <iomanip>
#include <queue>
#include <stack>
#include <functional>
#include <list>
#include <iterator>
#include <algorithm>

#include "vector.cpp"
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

void benchmark(std::string text, size_t n, std::function<void()> f) {
    std::cout << text << " - size " << to_hr_size(sizeof(int) * n) << std::endl;

    auto start = std::chrono::high_resolution_clock::now();
    f();
    auto end = std::chrono::high_resolution_clock::now();
    auto time = std::chrono::duration_cast<std::chrono::microseconds>(end - start);

    std::cout << "finished in: " << time.count() / 1000 << "ms" << std::endl;
    std::cout << std::chrono::duration_cast<std::chrono::nanoseconds>(end - start).count() / n << "ns / elem" << std::endl;
    auto seconds = (double)time.count() / (1000 * 1000);
    std::cout << "stuff: " << (double)(sizeof(int) * n) << std::endl;
    std::cout << to_hr_size((double)(sizeof(int) * n) / seconds) << "/s\n" << std::endl;
}

int main() {
    size_t n = 1024 * 1024 * 64;
    std::mt19937 mt;
    std::uniform_int_distribution<> dist(-10000, 10000);
    std::uniform_int_distribution<size_t> random_index(0, n);

    vector vec;
    for(size_t i = 0; i < n; i++) {
        vec.push(dist(mt));
    }

    benchmark("creating array", n, [&]() {
        vector vec;
        for(size_t i = 0; i < n; ++i) {
            vec.push(dist(mt));
        }
    });

    {
        int new_elements = 10;
        benchmark("adding to array", new_elements, [&]() {
            for(int i = 0; i < new_elements; i++) {
                vec.insert(random_index(mt), dist(mt));
            }
        });
    }

    {
        // TODO fix bad human readable size representation for this test case
        vector vec2(vec);
        size_t idx = n-1;
        vec2.insert(idx, 69420);

        benchmark("searching array", n, [&]() {
            size_t idx2 = vec2.find(69420);
            assert(idx == idx2);
        });
    }

    vector vec3(vec);
    int deletions = 10000;
    benchmark("deleting from array", deletions, [&]() {
        for(int i = 0; i < deletions; ++i) {
            // not sure how to handle decreasing list length and maintain random data uniformity, we can:
            // a) ignore it and hope we don't generate OOB index (we use fixed seed, so if it works once, it's guaranteed to work all the time
            // b) clamp the output of the distribution, thus making output equal to `len - 1` slightly more likely to appear since values in range [`len`; n) all map to `len - 1` and possibly affecting the results (but only slightly)
            // c) create a new distribution for each iteration, which seems like overkill and needlessly complicates the testcase
            // for now i'll go with a)
            vec3.remove(random_index(mt));
        }
    });

    // LINKED LIST
    std::list<int> list;

    // TODO memory leaks in linked list impl
    benchmark("creating linked list", n, [&]() {
        for(size_t i = 0; i < n; ++i) {
            list.push_back(dist(mt));
        }
    });

    int new_elems = 100;
    {
        std::list<int> list2(list);
        benchmark("adding to linked list", new_elems, [&]() {
            // NOTE: the nature of inserting to linked list is almost always non-trivial (search for an element, add something to it's right, remove something on it's left, do something else, etc...), so just a loop inserting at a random index is not exactly a useful benchmarking method; still, that's exactly what we are doing here.
            // unsuprisingly, C++ stdlib is designed with this in mind, so it models it by requiring us to use an interator.
            for(int i = 0; i < new_elems; ++i) {
                auto it = list2.begin();
                std::advance(it, random_index(mt));
                list2.insert(it, dist(mt));
            }
        });
    }

    // TODO add repetitions
    {
        std::list<int> list3(list);
        size_t idx = random_index(mt);
        int val = 69420;
        auto it = list3.begin();
        std::advance(it, idx);
        list3.insert(it, val);
        benchmark("finding in linked list", new_elems, [&]() {
            auto found = std::find(list3.begin(), list3.end(), val);
            assert(std::distance(list3.begin(), found) == idx);
        });
    }

    {
        int ops = 100;
        std::list<int> list4(list);
        benchmark("removing from linked list", new_elems, [&]() {
            for(int i = 0; i < ops; ++i) {
                auto it = list4.begin();
                std::advance(it, random_index(mt));
                list4.erase(it);
            }
        });
    }

    // STACK
    std::stack<int> stack;

    // TODO memory leaks in linked stack impl
    benchmark("creating stack", n, [&]() {
        for(size_t i = 0; i < n; ++i) {
            stack.push(dist(mt));
        }
    });

    {
        std::stack<int> stack2(stack);
        benchmark("adding to stack", new_elems, [&]() {
            for(int i = 0; i < new_elems; ++i) {
                stack2.push(dist(mt));
            }
        });
    }


    // removing from stack doesn't make a lot of sense, we choose the stack because we only ever want to remove from the
    // top, but whatever
    // TODO: fix search and delete
    // {
    //     std::stack<int> stack3(stack);
    //     size_t idx = random_index(mt);
    //     int val = 69420;
    //     auto it = stack3.begin();
    //     std::advance(it, idx);
    //     stack3.insert(it, val);
    //     benchmark("finding in stack", new_elems, [&]() {
    //         auto found = std::find(stack3.begin(), stack3.end(), val);
    //         assert(std::distance(stack3.begin(), found) == idx);
    //     });
    // }

    // {
    //     int ops = 100;
    //     std::stack<int> stack4(stack);
    //     benchmark("removing from stack", new_elems, [&]() {
    //         for(int i = 0; i < ops; ++i) {
    //             auto it = stack4.begin();
    //             std::advance(it, random_index(mt));
    //             stack4.erase(it);
    //         }
    //     });
    // }

    // QUEUE

    std::queue<int> queue;
    benchmark("creating queue", n, [&]() {
        for(size_t i = 0; i < n; ++i) {
            queue.push(dist(mt));
        }
    });
    
    {
        std::queue<int> queue2(queue);
        benchmark("adding to queue", new_elems, [&]() {
            for(int i = 0; i < new_elems; ++i) {
                queue.push(dist(mt));
            }
        });
    }

    // queue also kinda doesn't work in a sense that searching and removing elements doesnt make a lot of sense
    // TODO: fiiiiiiiiiiiiiiix
    // {
    //     std::queue<int> queue3(queue);
    //     size_t idx = random_index(mt);
    //     int val = 69420;
    //     auto it = queue3.begin();
    //     std::advance(it, idx);
    //     queue3.insert(it, val);
    //     benchmark("finding in queue", new_elems, [&]() {
    //         auto found = std::find(queue3.begin(), queue3.end(), val);
    //         assert(std::distance(queue3.begin(), found) == idx);
    //     });
    // }

    // {
    //     int ops = 100;
    //     std::queue<int> queue4(queue);
    //     benchmark("removing from queue", new_elems, [&]() {
    //         for(int i = 0; i < ops; ++i) {
    //             auto it = queue4.begin();
    //             std::advance(it, random_index(mt));
    //             queue4.erase(it);
    //         }
    //     });
    // }
    return 0;
}
