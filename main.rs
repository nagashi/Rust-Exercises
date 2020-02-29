fn main() {
  for i in 0..10 {
    if i % 2 == 0 {
      println!("even {}",i );
    } else {
        println!("odd {}",i );
      }
  }

  {
    let mut sum = 0.0;
    for i in 0..10 {
      sum += i as f64;
    }
    println!("==========\nsum is {}\n", sum);
  }

  {
    /* Values can also be passed by reference. A reference is created by & and dereferenced by *. */
    fn by_ref(x: &i32) -> i32 {
      *x + 1
    }
    
    let i = 10;
    let res1 = by_ref(&i);
    let res2 = by_ref(&41);
    println!("{} {}\n", res1, res2);
  }

  {
    // What if you want a function to modify one of its arguments? Enter mutable references:
    fn modifies(x: &mut f64) {
      *x = 1.0;
    }

      let mut res = 0.0;
      modifies(&mut res);
      println!("res is {}\n", res);
  }

  {
    /* All statically-typed languages have arrays, which are values packed nose to tail in memory.
       Arrays are indexed from zero: */
    let arr = [10, 20, 30, 40];
    let first = arr[0];
    println!("first {}", first);

    for i in 0..4 {
      println!("[{}] = {}", i, arr[i]);
    }
    println!("length {}", arr.len());
  }

  {
    /* Note two important things here - how to write a slice's type, and that you 
       have to use & to pass it to the function. */
    // read as: slice of i32

    /* Ignore the code of sum for a while, and look at &[i32]. The relationship between Rust arrays and
       slices is similar to that between C arrays and pointers, except for two important differences - Rust
       slices keep track of their size (and will panic if you try to access outside that size) and you have to
       explicitly say that you want to pass an array as a slice using the & operator.

       A C programmer pronounces & as 'address of'; a Rust programmer pronounces it 'borrow'. This is
       going to be the key word when learning Rust. Borrowing is the name given to a common pattern in
       programming; whenever you pass something by reference (as nearly always happens in dynamic 
       languages) or pass a pointer in C. Anything borrowed remains owned by the original owner. */

    fn sum(values: &[i32]) -> i32 {
      let mut res = 0;
      for i in 0..values.len() {
        res += values[i]
      }
      res
    }

    let arr = [10, 20, 30, 40];
    // look at that &
    let res = sum(&arr);
    println!("========\nsum {}\n", res);
  }

  {
    /* Slicing and Dicing
       You cannot print out an array in the usual way with {} but you can do a debug print with {:?}.
    */
    let ints = [1, 2, 3];
    let floats = [1.1, 2.1, 3.1];
    let strings = ["hello", "world"];
    let ints_ints = [[1, 2], [10, 20]];
    println!("Debugging Arrays\n----------------");
    println!("ints {:?}", ints);
    println!("floats {:?}", floats);
    println!("strings {:?}", strings);
    println!("ints_ints {:?}\n", ints_ints);
  }

  {
    // There is a slice method get which does not panic. But what does it return?
    let ints = [1,2,3,4,5];
    let slice = &ints;
    let first = slice.get(0);
    let last  = slice.get(5);
    let lastly = slice.get(slice.len() - 1);
    let last_err = *slice.get(5).unwrap_or(&-1);

    println!("slice {:?}", slice);
    println!("first {:?}", first);
    println!("last {:?}", last);
    println!("last_err {}", last_err);
    println!("lastly {:?}\n", lastly);
  }

}

