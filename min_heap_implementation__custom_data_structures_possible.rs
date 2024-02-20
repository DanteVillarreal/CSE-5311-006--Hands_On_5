//Dante Villarreal. ID: 1001580411

//HOW TO RUN CODE
//This is coded in Rust. This is the best programming language.  
//if you put the code into Rust playground, a Cargo.toml file is not necessary
//if you want to run it on your own computer, in your Cargo.toml file do:
//    [dependencies]
//    ordered-float = "2.7.0"
//else if you have never used Rust
//go to the Rust playground and paste the code to run it:
//    https://play.rust-lang.org/?version=stable&mode=debug&edition=2021


//REQUIREMENTS:
/*
Implement a min heap data structure. For the parent and left/right functions use bit manipulation operators.
    The MinHeap struct is the implementation of the min heap data structure.
    The parent of a node at index i is calculated as (i - 1) >> 1 in the push method.
    The left child of a node at index i is calculated as (i << 1) + 1 in the heapify method.
    The right child of a node at index i is calculated as (i << 1) + 2 in the heapify method.
The ability to initially build the heap (build_min_heap)
    The build_min_heap method is used to initially build the heap from an array of elements.
The ability to heapify
    The heapify method “bubbles down” the element at the given index to its correct position in the heap.
The ability to get and remove (“pop”) the root node from the heap (and of course re-heapify everything)
    The pop method removes the root node from the heap and re-heapifies the remaining elements.
The heap should be generic to the type of data (can store floats, int, custom datastructure)
    The MinHeap struct is generic over the type T which must implement the Ord and Clone traits. 
    This allows it to store any type of data that can be ordered and cloned, including floats, ints, and custom data structures.
Show example(s) of your heap working. Please demonstrate ALL the functionality you implemented.
  The main function demonstrates how to use the MinHeap type with integers, floats, and Person instances. 
  The println! statements print the smallest element in each heap.

*/




//  Rust is better than Python because it catches bugs before they catch you
//    Also the speed, duh.
//  Rust is better than C because have you ever been writing in C and
//    you're like when can I actually do something? Why isn't there a universal
//    function for this like in javascript. Rust is a low level language
//    but makes it easy to use code libraries like in javascript.
//    Which includes why it is better than javascript.



use std::cmp::Ordering;
use ordered_float::OrderedFloat;

// Define a custom data type `Person` with a `name` and an `age`
#[derive(Eq, PartialEq, Clone)]
struct Person {
    name: String,
    age: u32,
}

// Define a new struct `PersonByAge` that wraps `Person`
// This struct will be used when we want to order people by age
#[derive(Eq, PartialEq, Clone)]
struct PersonByAge(Person);

// Implement the `Ord` trait for `PersonByAge`
// This implementation orders people by their age
impl Ord for PersonByAge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.age.cmp(&other.0.age)
    }
}

// Implement the `PartialOrd` trait for `PersonByAge`
// The `PartialOrd` trait is a prerequisite for the `Ord` trait.
// This is because `Ord` requires a total ordering, which means that any two 
// values can be compared to each other. However, not all types have a total 
// ordering.
// For example, floating point numbers do not have a total ordering because `NaN`
// is not less than, greater than, or equal to any other float, including itself.
// Therefore, Rust separates the concept of a total ordering (`Ord`) from a 
// partial ordering (`PartialOrd`).
// In this case, since we know that ages (represented as `u32`) do have a 
// total ordering, we can simply delegate to the `cmp` method when implementing
// `partial_cmp`.
impl PartialOrd for PersonByAge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Define a new struct `PersonByName` that wraps `Person`
// This struct will be used when we want to order people by name
#[derive(Eq, PartialEq, Clone)]
struct PersonByName(Person);

// Implement the `Ord` trait for `PersonByName`
// This implementation orders people by their name
impl Ord for PersonByName {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.name.cmp(&other.0.name)
    }
}

// Implement the `PartialOrd` trait for `PersonByName`
// This is necessary because the `Ord` trait depends on it
impl PartialOrd for PersonByName {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct MinHeap<T: Ord + Clone> {
    heap: Vec<T>,
}

impl<T: Ord + Clone> MinHeap<T> {
    pub fn new() -> Self {
        MinHeap {
            heap: Vec::new(),
        }
    }

    pub fn push(&mut self, item: T) {
        self.heap.push(item);
        let mut i = self.heap.len() - 1;
        while i > 0 {
            let parent = (i - 1) >> 1;
            if self.heap[i] < self.heap[parent] {
                self.heap.swap(i, parent);
            }
            i = parent;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        let len = self.heap.len();
        if len == 0 {
            return None;
        }
        self.heap.swap(0, len - 1);
        let popped = self.heap.pop();
        let mut i = 0;
        while i < self.heap.len() {
            let left = (i << 1) + 1;
            let right = (i << 1) + 2;
            let mut smallest = i;
            if left < self.heap.len() && self.heap[left] < self.heap[smallest] {
                smallest = left;
            }
            if right < self.heap.len() && self.heap[right] < self.heap[smallest] {
                smallest = right;
            }
            if smallest == i {
                break;
            }
            self.heap.swap(i, smallest);
            i = smallest;
        }
        popped
    }

    pub fn heapify(&mut self, i: usize) {
        let left = (i << 1) + 1;
        let right = (i << 1) + 2;
        let mut smallest = i;
        if left < self.heap.len() && self.heap[left] < self.heap[smallest] {
            smallest = left;
        }
        if right < self.heap.len() && self.heap[right] < self.heap[smallest] {
            smallest = right;
        }
        if smallest != i {
            self.heap.swap(i, smallest);
            self.heapify(smallest);
        }
    }

    pub fn build_min_heap(&mut self, arr: &[T]) {
        self.heap = arr.to_vec();
        if self.heap.len() > 1 {
            for i in (0..self.heap.len() / 2).rev() {
                self.heapify(i);
            }
        }
    }
}

fn main() {
    // Example with integers
    let mut int_heap = MinHeap::new();
    int_heap.build_min_heap(&[3, 2, 1, 15, 5, 4, 45, 6, 7, 8, 9]);
    match int_heap.pop() {
        Some(value) => println!("Min value in integer heap: {}", value),
        None => println!("Integer heap is empty"),
    }

    // Example with floats
    let mut float_heap = MinHeap::new();
    float_heap.build_min_heap(&[
        OrderedFloat(3.0), 
        OrderedFloat(2.0), 
        OrderedFloat(1.0), 
        OrderedFloat(5.0), 
        OrderedFloat(4.0)
    ]);
    match float_heap.pop() {
        Some(value) => println!("Min value in float heap: {}", value),
        None => println!("Float heap is empty"),
    }

    // Example with custom data type ordered by age
    let mut person_heap = MinHeap::new();
    person_heap.build_min_heap(&[
        PersonByAge(Person { name: String::from("Alice"), age: 30 }),
        PersonByAge(Person { name: String::from("Bob"), age: 20 }),
        PersonByAge(Person { name: String::from("Charlie"), age: 50 }),
    ]);
    match person_heap.pop() {
        Some(person) => println!("Youngest person: {} (age {})", person.0.name,
        person.0.age),
        None => println!("Person heap is empty"),
    }

    // Example with custom data type ordered by name
    let mut person_heap_by_name = MinHeap::new();
    person_heap_by_name.build_min_heap(&[
        PersonByName(Person { name: String::from("Alice"), age: 30 }),
        PersonByName(Person { name: String::from("Bob"), age: 20 }),
        PersonByName(Person { name: String::from("Charlie"), age: 50 }),
    ]);
    match person_heap_by_name.pop() {
        Some(person) => {
            println!("First person in alphabetical order: {} (age {})",
            person.0.name, person.0.age)
        },
        None => println!("Person heap is empty"),
    }
}
