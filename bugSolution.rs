fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // Solution 1: Consume the iterator
    let mut iter = vec.iter();
    println!("First element: {:?}", iter.next());
    println!("Second element: {:?}", iter.next());
    drop(iter); // consume the iterator to release the borrow

    vec.push(4);
    println!("Vec after push: {:?}", vec);

    // Solution 2: Create a new vector
    let mut vec2 = vec.clone();
    let mut iter2 = vec2.iter();
    println!("First element (vec2): {:?}", iter2.next());
    println!("Second element (vec2): {:?}", iter2.next());
    vec.push(5); // This is now allowed
    println!("Vec after push: {:?}", vec);
    println!("Vec2 after iteration: {:?}", vec2);
    
    //Solution 3: Use a for loop
    let mut vec3 = vec.clone();
    for (i, x) in vec3.iter().enumerate() {
        if i == 0 {
            println!("First element (vec3): {:?}", x);
        } else if i == 1 {
            println!("Second element (vec3): {:?}", x);
        }
    }
    vec.push(6); // This is now allowed
    println!("Vec after push: {:?}", vec);
    println!("Vec3 after iteration: {:?}", vec3);
} 