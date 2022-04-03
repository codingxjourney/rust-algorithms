# rust-algorithms

All the algorithms are written in rust language.

![image](https://user-images.githubusercontent.com/90622203/161422154-07fa8ff9-372a-41a8-95f3-d31e12391ee8.png)


# Time Complexity
Time complexity of an algorithm quantifies the amount of time taken by an algorithm to run as a function of the length of the input.


Types of notations       
1. O-notation: It is used to denote asymptotic upper bound. For a given function g(n), we denote it by O(g(n)). Pronounced as “big-oh of g of n”. It also known as worst case time complexity as it denotes the upper bound in which algorithm terminates.
2. Ω-notation: It is used to denote asymptotic lower bound. For a given function g(n), we denote it by Ω(g(n)). Pronounced as “big-omega of g of n”. It also known as best case time complexity as it denotes the lower bound in which algorithm terminates.

# Space Complextiy
Space complexity of an algorithm quantifies the amount of time taken by a program to run as a function of length of the input. It is directly proportional to the largest memory your program acquires at any instance during run time.

## [Search Algorithms](./src/searching)

- [x] [Linear Search](./src/searching/linear_search.rs): a linear search is a algorithm for finding an element within a array. It sequentially checks each element of the array until a match is found or the whole array has been searched.                                                                                                        
     Time Complexity                         
     Best-case scenario: 1                                                   
     Worst-case scenario: n                                                  

- [x] [Binary Search](./src/searching/binary_search.rs): Binary Search is a searching algorithm used in a sorted array by repeatedly dividing the search interval in half.                
     Time Comlexity           
     Best-case scenario: 1                                                   
     Worst-case scenario: log(n)
