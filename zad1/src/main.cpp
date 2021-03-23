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
#include <fstream>

#include "container.cpp"

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

std::chrono::milliseconds benchmark(std::string text, size_t n, std::function<void()> f) {
    std::cout << text << std::endl;

    auto start = std::chrono::high_resolution_clock::now();
    for(size_t i = 0; i < n; i++) {
        f();
    }
    auto end = std::chrono::high_resolution_clock::now();
    auto time = std::chrono::duration_cast<std::chrono::milliseconds>(end - start);

    std::cout << "finished in: " << time.count() << "ms" << std::endl;
    // std::cout << std::chrono::duration_cast<std::chrono::nanoseconds>(end - start).count() / n << "ns / elem" << std::endl;
    // auto seconds = (double)time.count() / (1000 * 1000);
    // std::cout << "stuff: " << (double)(sizeof(int) * n) << std::endl;
    // std::cout << to_hr_size((double)(sizeof(int) * n) / seconds) << "/s\n" << std::endl;

    return time;
}

void stack_insert(std::stack<int>& stack, size_t idx, int val) {
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

void queue_insert(std::queue<int>& queue, size_t idx, int val) {
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

struct task {
    std::string type;
    std::string operation;
    size_t instance_size;
    std::vector<size_t> iterations;

    task(std::string line) {
        std::istringstream ss(line);

        ss >> type >> operation >> instance_size;
        int n;
        while(ss >> n) {
            iterations.push_back(n);
        }
    }
};

int main() {
    // read config
    std::ifstream file;
    file.open("config.ini");

    std::string filename;
    std::getline(file, filename);

    std::vector<task> tasks;
    std::string line;
    while(std::getline(file, line)) {
        if(line.empty()) {
            continue;
        }
        tasks.emplace_back<task>(line);
    }
    std::cout << "filename: " << filename << std::endl;

    std::mt19937 mt;
    std::uniform_int_distribution<> dist(-10000, 10000);

    for(auto task: tasks) {
        std::string name = task.type + " " + task.operation;
        std::cout << name << ", instance:" << task.instance_size << ", iterations: ";
        for(auto iteration : task.iterations) {
            std::cout << iteration << " ";
        }
        std::cout << std::endl;

        std::uniform_int_distribution<size_t> random_index(0, task.instance_size);

        // get data type
        Container *container;
        if(task.type == "array") {
            container = new Array();
        }
        else if(task.type == "list") {
            container = new List();
        }
        else if(task.type == "stack") {
            container = new Stack();
        }
        else if(task.type == "queue") {
            container = new Queue();
        }
        else {
            assert(false);
        }

        // if op type create, we benchmark filling up to instance size
        if(task.operation == "create") {
            for(auto iteration: task.iterations) {
                benchmark(name, iteration, [&]() {
                    for(size_t i = 0; i < task.instance_size; ++i) {
                        container->append(dist(mt));
                    }
                });
            }
            continue;
        }

        // else we initialize outside of benchmark block, and provide a filled instance
        for(size_t i = 0; i < task.instance_size; ++i) {
            container->append(dist(mt));
        }

        if(task.operation == "add") {
            for(auto iteration: task.iterations) {
                benchmark(name, iteration, [&]() {
                    container->append(dist(mt));
                });
            }
        }
        else if(task.operation == "insert") {
            for(auto iteration: task.iterations) {
                benchmark(name, iteration, [&]() {
                    container->insert(random_index(mt), dist(mt));
                });
            }
        }
        else if(task.operation == "search") {
            for(auto iteration: task.iterations) {
                benchmark(name, iteration, [&]() {
                    container->find(dist(mt));
                });
            }
        }
        else if(task.operation == "remove") {
            for(auto iteration: task.iterations) {
                benchmark(name, iteration, [&]() {
                    container->remove(random_index(mt));
                });
            }
        }
    }

    return 0;


    //     // oh dear god help me
    //     benchmark("creating array", 10, [&]() {
    //         vector vec;
    //         for(size_t i = 0; i < n; ++i) {
    //             vec.push(dist(mt));
    //         }
    //     });

    //     {
    //         benchmark("adding to array", 10, [&]() {
    //             vec.insert(random_index(mt), dist(mt));
    //         });
    //     }

    //     {
    //         // TODO fix bad human readable size representation for this test case
    //         vector vec2(vec);
    //         size_t idx = n-1;
    //         vec2.insert(idx, 69420);

    //         benchmark("searching array", 10, [&]() {
    //             size_t idx2 = vec2.find(69420);
    //             assert(idx == idx2);
    //         });
    //     }

    //     vector vec3(vec);
    //     int deletions = 10;
    //     benchmark("deleting from array", deletions, [&]() {
    //         for(int i = 0; i < deletions; ++i) {
    //             // not sure how to handle decreasing list length and maintain random data uniformity, we can:
    //             // a) ignore it and hope we don't generate OOB index (we use fixed seed, so if it works once, it's guaranteed to work all the time
    //             // b) clamp the output of the distribution, thus making output equal to `len - 1` slightly more likely to appear since values in range [`len`; n) all map to `len - 1` and possibly affecting the results (but only slightly)
    //             // c) create a new distribution for each iteration, which seems like overkill and needlessly complicates the testcase
    //             // for now i'll go with a)
    //             vec3.remove(random_index(mt));
    //         }
    //     });

    //     // LINKED LIST
    //     std::list<int> list;

    //     // TODO memory leaks in linked list impl
    //     benchmark("creating linked list", 10, [&]() {
    //         for(size_t i = 0; i < n; ++i) {
    //             list.push_back(dist(mt));
    //         }
    //     });

    //     int new_elems = 100;
    //     {
    //         std::list<int> list2(list);
    //         benchmark("adding to linked list", new_elems, [&]() {
    //             // NOTE: the nature of inserting to linked list is almost always non-trivial (search for an element, add something to it's right, remove something on it's left, do something else, etc...), so just a loop inserting at a random index is not exactly a useful benchmarking method; still, that's exactly what we are doing here.
    //             // unsuprisingly, C++ stdlib is designed with this in mind, so it models it by requiring us to use an interator.
    //             for(int i = 0; i < new_elems; ++i) {
    //                 auto it = list2.begin();
    //                 std::advance(it, random_index(mt));
    //                 list2.insert(it, dist(mt));
    //             }
    //         });
    //     }

    //     {
    //         std::list<int> list3(list);
    //         size_t idx = random_index(mt);
    //         int val = 69420;
    //         auto it = list3.begin();
    //         std::advance(it, idx);
    //         list3.insert(it, val);
    //         benchmark("finding in linked list", new_elems, [&]() {
    //             auto found = std::find(list3.begin(), list3.end(), val);
    //             assert(std::distance(list3.begin(), found) == idx);
    //         });
    //     }

    //     {
    //         int ops = 1000;
    //         std::list<int> list4(list);
    //         benchmark("removing from linked list", new_elems, [&]() {
    //             for(int i = 0; i < ops; ++i) {
    //                 auto it = list4.begin();
    //                 std::advance(it, random_index(mt));
    //                 list4.erase(it);
    //             }
    //         });
    //     }

    //     // STACK
    //     std::stack<int> stack;

    //     benchmark("creating stack", n, [&]() {
    //         for(size_t i = 0; i < n; ++i) {
    //             stack.push(dist(mt));
    //         }
    //     });

    //     {
    //         std::stack<int> stack2(stack);
    //         benchmark("adding to stack", new_elems, [&]() {
    //             for(int i = 0; i < new_elems; ++i) {
    //                 stack2.push(dist(mt));
    //             }
    //         });
    //     }

    //     {
    //         std::stack<int> stack(stack);
    //         int ops = 100;
    //         size_t idx = random_index(mt);
    //         int val = dist(mt);
    //         benchmark("inserting into a stack", ops, [&]() {
    //             for(int op = 0; op < ops; ++op) {
    //                 stack_insert(stack, idx, val);
    //             }
    //         });
    //     }


    //     // removing from stack doesn't make a lot of sense, we choose the stack because we only ever want to remove from the
    //     // top, but whatever
    //     {
    //         // insert impl
    //         std::stack<int> stack3(stack);
    //         size_t idx = random_index(mt);
    //         int val = 69420;

    //         stack_insert(stack3, idx, val);

    //         // custom search implementation
    //         benchmark("finding in stack", new_elems, [&]() {
    //             std::stack<int> temp;
    //             size_t i = 0;
    //             while(!stack3.empty()) {
    //                 int elem = stack3.top();
    //                 if(elem == val) {
    //                     break;
    //                 }
    //                 temp.push(elem);
    //                 stack3.pop();
    //                 ++i;
    //             }
    //             while(!temp.empty()) {
    //                 stack3.push(temp.top());
    //                 temp.pop();
    //             }

    //             assert(i == idx);
    //         });
    //     }

    //     {
    //         int ops = 1000;
    //         std::stack<int> stack4(stack);
    //         benchmark("removing from stack", new_elems, [&]() {
    //             for(int op = 0; op < ops; ++op) {
    //                 size_t idx = random_index(mt);
    //                 std::stack<int> temp;
    //                 for(size_t i = 0; i < idx; ++i) {
    //                     temp.push(stack4.top());
    //                     stack4.pop();
    //                 }
    //                 stack4.pop();
    //                 while(!temp.empty()) {
    //                     stack4.push(temp.top());
    //                     temp.pop();
    //                 }
    //             }
    //         });
    //     }

    //     // QUEUE
    //     std::queue<int> queue;
    //     benchmark("creating queue", n, [&]() {
    //         for(size_t i = 0; i < n; ++i) {
    //             queue.push(dist(mt));
    //         }
    //     });
        
    //     {
    //         std::queue<int> queue2(queue);
    //         benchmark("adding to queue", new_elems, [&]() {
    //             for(int i = 0; i < new_elems; ++i) {
    //                 queue.push(dist(mt));
    //             }
    //         });
    //     }

    //     {
    //         int ops = 100;
    //         std::queue<int> queue(queue);
    //         benchmark("inserting into a queue", ops, [&]() {
    //             for(int op = 0; op < ops; ++op) {
    //                 size_t idx = random_index(mt);
    //                 int val = dist(mt);
    //                 queue_insert(queue, idx, val);
    //             }
    //         });
    //     }

    //     {
    //         std::queue<int> queue3(queue);
    //         size_t idx = random_index(mt);
    //         int val = 69420;

    //         queue_insert(queue, idx, val);
    //         benchmark("finding in queue", new_elems, [&]() {
    //             std::queue<int> temp;
    //             size_t i = 0;
    //             while(!queue.empty()) {
    //                 int elem = queue.front();
    //                 if(elem == val) {
    //                     break;
    //                 }
    //                 temp.push(elem);
    //                 queue.pop();
    //                 ++i;
    //             }
    //             while(!queue.empty()) {
    //                 temp.push(queue.front());
    //                 queue.pop();
    //             }
    //             queue.swap(temp);
    //             assert(i == idx);
    //         });
    //     }

    //     {
    //         int ops = 100;
    //         std::queue<int> queue4(queue);
    //         benchmark("removing from queue", new_elems, [&]() {
    //             for(int i = 0; i < ops; ++i) {
    //                 size_t idx = random_index(mt);
    //                 std::queue<int> temp;
    //                 for(size_t i = 0; i < idx; ++i) {
    //                     temp.push(queue4.front());
    //                     queue4.pop();
    //                 }
    //                 queue4.pop();
    //                 while(!queue4.empty()) {
    //                     temp.push(queue4.front());
    //                     queue4.pop();
    //                 }
    //                 queue4.swap(temp);
    //             }
    //         });
    //     }
    // }

    // return 0;
}
