Project name: Parallel Sorting Algorithms

Proposal: I will implement 3 algorithms: Parallel sample sort, Parallel Radix sort, and Parallel Quick sort. As we know, these three algorithms might be slow for a large scale of input. Therefore, I will parallelize them in order to minimize the running time and sort more data, amounts that can’t be sorted serially. 

Radix sort → Radix sort is one of the fastest sorting algorithms for short keys and is the only sorting algorithm in this report which is not comparison-based. Its sequential variation first splits the elements being sorted (numbers, words, dates, ...) into d r-bit digits. The elements are then sorted from least to the most significant digit. 

Quick sort → Quicksort splits the input array into 2 buckets (containing the elements smaller than the pivot and those bigger than the pivot). These two buckets can be independently sorted in parallel and there is nothing to be done to join them together. 

Sample sort →  Samplesort is a sorting algorithm that is a divide and conquers algorithm. Conventional divide and conquer sorting algorithms partition the array into sub-intervals or buckets. The buckets are then sorted individually and then concatenated together. 

To parallelize these sorting, I will use Rust language and rayon and crossbeam library to implement the code.
