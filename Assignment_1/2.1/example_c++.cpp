#include <iostream>

int main() {
    // Dynamically allocate an array on the heap.
    int* arr = new int[10];

    for (int i = 0; i < 10; ++i) {
        arr[i] = i * 10;
    }

    std::cout << "Array values: ";
    for (int i = 0; i < 10; ++i) {
        std::cout << arr[i] << " ";
    }
    std::cout << std::endl;

    // Manually free the allocated memory.
    // This is very important to do, otherwise you will have a memory leak.
    delete[] arr;
    return 0;
}