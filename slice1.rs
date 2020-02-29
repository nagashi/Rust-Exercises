fn main() {
    let ints = [1, 2, 3, 4, 5];
    let slice = &ints;

    for s in slice.windows(2) {
        println!("window {:?}", s);
        /* window [1, 2]
           window [2, 3]
           window [3, 4]
           window [4, 5]
        */
    }
    println!("\n");
    {
      for s in slice.chunks(2) {
          println!("chunks {:?}", s);
          /* chunks [1, 2]
             chunks [3, 4]
             chunks [5]
          */
      }
      println!("\n");
    }
}