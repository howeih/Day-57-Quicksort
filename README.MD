Day 57: Quicksort
<br>
Quicksort (sometimes called partition-exchange sort) is an efficient sorting algorithm, serving as a systematic method for placing the elements of a random access file or an array in order. Developed by British computer scientist Tony Hoare in 1959 and published in 1961, it is still a commonly used algorithm for sorting. When implemented well, it can be about two or three times faster than its main competitors, merge sort and heapsort.
<br>
Quicksort is a comparison sort, meaning that it can sort items of any type for which a "less-than" relation (formally, a total order) is defined. In efficient implementations it is not a stable sort, meaning that the relative order of equal sort items is not preserved. Quicksort can operate in-place on an array, requiring small additional amounts of memory to perform the sorting. It is very similar to selection sort, except that it does not always choose worst-case partition.
<br>
Mathematical analysis of quicksort shows that, on average, the algorithm takes O(n log n) comparisons to sort n items. In the worst case, it makes O(n2) comparisons, though this behavior is rare.
<br>
run:
```

fn main() {
    let mut rng = rand::thread_rng();
    let mut data = Vec::<i32>::new();
    for _ in 0..100 {
        data.push(rng.gen_range(0, 1000));
    }
    let high = data.len() as i32 - 1;
    println!("before sort: {:?}", data);
    quick_sort(&mut data, 0, high);
    println!("after sort: {:?}", data);
}


```

result:
```

before sort: [366, 162, 494, 111, 73, 5, 478, 539, 949, 320, 791, 634, 923, 205, 981, 170, 824, 619, 543, 432, 553, 163, 600, 592, 798, 201, 258, 170, 30, 466, 513, 883, 986, 533, 997, 667, 427, 128, 383, 904, 393, 630, 83, 13, 767, 259, 157, 784, 869, 230, 26, 171, 100, 245, 886, 481, 594, 531, 675, 107, 637, 789, 924, 731, 661, 62, 529, 112, 357, 451, 746, 191, 811, 846, 88, 217, 569, 311, 248, 288, 354, 156, 392, 301, 879, 407, 350, 458, 595, 784, 856, 667, 991, 150, 201, 797, 990, 718, 642, 321]
after sort: [5, 13, 26, 30, 62, 73, 83, 88, 100, 107, 111, 112, 128, 150, 156, 157, 162, 163, 170, 170, 171, 191, 201, 201, 205, 217, 230, 245, 248, 258, 259, 288, 301, 311, 320, 321, 350, 354, 357, 366, 383, 392, 393, 407, 427, 432, 451, 458, 466, 478, 481, 494, 513, 529, 531, 533, 539, 543, 553, 569, 592, 594, 595, 600, 619, 630, 634, 637, 642, 661, 667, 667, 675, 718, 731, 746, 767, 784, 784, 789, 791, 797, 798, 811, 824, 846, 856, 869, 879, 883, 886, 904, 923, 924, 949, 981, 986, 990, 991, 997]


```