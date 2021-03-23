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

size_t benchmark(std::string text, size_t n, std::function<void()> f) {
    std::cout << text;

    auto start = std::chrono::high_resolution_clock::now();
    for(size_t i = 0; i < n; i++) {
        f();
    }
    auto end = std::chrono::high_resolution_clock::now();
    auto time = std::chrono::duration_cast<std::chrono::microseconds>(end - start);

    std::cout << "\t\tfinished in: " << time.count() << "us" << std::endl;
    return time.count();
}

struct task {
    std::string type;
    std::string operation;
    size_t iterations;
    std::vector<size_t> instance_sizes;

    task(std::string line) {
        std::istringstream ss(line);

        ss >> type >> operation >> iterations;
        int n;
        while(ss >> n) {
            instance_sizes.push_back(n);
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
    std::ofstream out_file(filename);
    out_file << "struktura,operacja,iteracje,instancja,mikrosekundy" << std::endl;

    std::mt19937 mt;
    std::uniform_int_distribution<> dist(-10000, 10000);

    for(auto task: tasks) {
        std::string name = task.type + " " + task.operation;
        std::cout << name << ", iterations:" << task.iterations << ", instances: ";
        for(auto iteration : task.instance_sizes) {
            std::cout << iteration << " ";
        }
        std::cout << std::endl;


        // get data type
        std::unique_ptr<Container> container;
        if(task.type == "array") {
            container = std::make_unique<Array>();
        }
        else if(task.type == "list") {
            container = std::make_unique<List>();
        }
        else if(task.type == "stack") {
            container = std::make_unique<Stack>();
        }
        else if(task.type == "queue") {
            container = std::make_unique<Queue>();
        }
        else {
            assert(false);
        }


        size_t micros;

        // if op type create, we benchmark filling up to instance size
        if(task.operation == "create") {
            for(auto instance: task.instance_sizes) {
                std::uniform_int_distribution<size_t> random_index(0, instance);
                micros = benchmark(name, task.iterations, [&]() {
                    for(size_t i = 0; i < instance; ++i) {
                        container->append(dist(mt));
                    }
                });
                out_file << task.type << "," << task.operation << "," << instance << "," << task.iterations << ","
                    << micros << std::endl;
            }
            continue;
        }


        // else we initialize outside of benchmark block, and provide a filled instance
        size_t iterations = task.iterations;
        for(auto instance : task.instance_sizes) {
            std::uniform_int_distribution<size_t> random_index(0, instance - task.iterations);
            for(size_t i = 0; i < instance; ++i) {
                container->append(dist(mt));
            }
            if(task.operation == "add") {
                    micros = benchmark(name, iterations, [&]() {
                        container->append(dist(mt));
                    });

                    out_file << task.type << "," << task.operation << "," << instance << "," << iterations << ","
                        << micros << std::endl;
            }
            else if(task.operation == "insert") {
                    micros = benchmark(name, iterations, [&]() {
                        container->insert(random_index(mt), dist(mt));
                    });
                    out_file << task.type << "," << task.operation << "," << instance << "," << iterations << ","
                        << micros << std::endl;
            }
            else if(task.operation == "search") {
                    micros = benchmark(name, iterations, [&]() {
                        container->find(dist(mt));
                    });
                    out_file << task.type << "," << task.operation << "," << instance << "," << iterations << ","
                        << micros << std::endl;
            }
            else if(task.operation == "remove") {
                    micros = benchmark(name, iterations, [&]() {
                        container->remove(random_index(mt));
                    });
                    out_file << task.type << "," << task.operation << "," << instance << "," << iterations << ","
                        << micros << std::endl;
            }
        }
        mt.seed();
    }
    return 0;
}
