#ifndef CATCH_CONFIG_MAIN
#define CATCH_CONFIG_MAIN
#endif

#include <catch/catch.hpp>

#include <vector>
#include <string>
#include <iostream>

#include "sandwich.hpp"

using std::string;
using std::vector;

// A helper method for you to visualize what the contents of a vector are. You
// are welcome to modify this method or create additional helper methods.
//
// Example usage:
//  vector<string> my_vec;
//  my_vec.push_back("hello");
//  my_vec.push_back("world"):
//  PrintVector(my_vec);
// Prints:
//  [hello, world]
void PrintVector(const vector<string>& vec) {
    std::cout << "[";
    bool first = true;
    for (const auto& it : vec) {
        if (!first) {
            std::cout << ", ";
        } else {
            first = false;
        }

        std::cout << it;
    }

    std::cout << "]" << std::endl;
}

TEST_CASE("Sandwich::AddTopping test", "[AddTopping]") {
    // your tests for Sandwich::AddTopping here
}

TEST_CASE("Sandwich::RemoveTopping test", "[RemoveTopping]") {
    // your tests for Sandwich::RemoveTopping here
}

TEST_CASE("Sandwich::AddDressing test", "[AddDressing]") {
    // your tests for Sandwich::AddDressing here
}
