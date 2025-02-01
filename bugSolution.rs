fn main() {
    let vec = vec![1, 2, 3];

    // Using a for loop to iterate and access elements safely
    for (index, element) in vec.iter().enumerate() {
        println!("Element at index {}: {}", index, element);
    }

    //Alternative solution using cloning
    let vec2 = vec![1,2,3];
    let mut iter = vec2.iter();
    println!("First element: {}", iter.next().unwrap());
    println!("Second element: {}", iter.next().unwrap());
    println!("Third element: {}", iter.next().unwrap());
} 