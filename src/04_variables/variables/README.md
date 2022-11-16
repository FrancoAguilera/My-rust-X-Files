# Programming concepts


## Variables and inmutability
**let** variables are inmutable by default, unless the use of the "mut" keyword. 
Also the tyṕe can be infered by the data type assigned (optional)

```rust
  let mut x = 10;
  println!("X value is {}", x);

  x = 11; // without the "mut" above this will throw and error
  println!("X mutated to {}", x);

  let y: i32 = 69;
  println!("Y value is {}", y);
```

**const** are inmutable, and the type must be always specified (_as a common practice the const name is uppercase and "snake case"_)
```rust
  const X_VALUE: i32 = 100_000;
  println!("X_VALUE const is: {}", X_VALUE);
```

## Shadowing
**Shadowing** allows you to create a new variable using an existing name
```rust
  let x: i32 = 10;
  println!("X value is {}", x);

  let x: &str = "foo bar baz"; // same variable name as above
  println!("X value is {}", x);
```
In this example the first variable get shadowed by the second on, this give us 2 advantages
1) preserve inmutability
2) we can change types

## Data types

### Scalar types 
_They represent **ONE** value:_

#### Integers
___

No decimal point numbers (i32 default type)

| Length | Signed +/-| Unsigned +|
| --- | --- | --- |
| 8-bit	| i8	| u8 |
| 16-bit	| i16	| u16 |
| 32-bit	 | i32	| u32 |
| 64-bit	| i64	| u64 |
| 128-bit	| i128	| u128 |
| arch	| isize	| usize |

Integer oveflow
```rust
  let f: u8 = 255; // max value a u8 can hold

  // this will oveload the max value, and it will go back to the minimum value, so 256 will become 0, 257 will be 1, and so on
  let t: u8 = 256; 
```


Floating point values
___
Numbers with decimal point (f64 default type)

Booleans
___
true / false values
```rust
  let t: bool = true;
  let f: bool = false;
```


Characters
___
They represent an unicode charcater, written in single quotes
```rust
  let mut c: char = 'z';
  c = 'a';
```

### Compound types
_They represent a **GROUP** value_
Tuple type
___

A fixed size array of related data that can have different types
```rust
  let tup: (&str, f64) = ("this value", 70.5);

  // this can be destructured 
  let (value, number_count) = tup;
  println!("{}, and {}", value, number_count);

  // or use the dot notation
  let string_value: &str = tup.0;
  println!("{}", string_value);
```

Array type
___
Unlike a tuple, every element of an array must have the same type. Unlike arrays in some other languages, arrays in Rust have a fixed length.
```rust
  let a = [1, 2, 3, 4, 5];
  let months = [
    "one", 
    "Two"
  ];

  // You write an array’s type using square brackets with the type of each 
  // element, a semicolon, and then the number of elements in the array, like so:
  let error_codes: [i32; 3] = [101, 200, 404];
  let not_found: i32 = error_codes[2];

  // array can also be initialized this way 
  let byte = [0; 4]; // [0, 0, 0, 0]
```
## Functions
___
You declare a function with the **fn** keyword, which allows you to declare new functions.

Rust code uses snake case as the conventional style for function and variable names, in which all letters are lowercase and underscores separate words. Here’s a program that contains an example function definition:
```rust
  fn main() {
      println!("Hello, world!");
      another_function(11, 22);
  }

  fn another_function(x: i32, y: i32) {
      println!("X value is {}", x);
      println!("Y value is {}", y);
  }
```

Statements and Expressions
___
In Rust you can this a piece of code as an **statement** or and **expression**
* Statements are instructions that perform some action and do not return a value.
* Expressions return a value

```rust
  fn main () {
      let sum_value = add(11, 22);
      println!("Add value is: {}", sum_value);
  }

  fn add (x: i32, y: i32) -> i32 { // need to specify the return type
      println!("X value is {}", x);
      println!("Y value is {}", y);

      let _sum: i32 = x + y;

      // return value can be made in several ways:
      // return sum;
      // sum
      // x + y

      x + y
  }
```

## Control flow


```rust
  let num: f64 = 10.2;

  // if statements
  if num > 5.0 {
    println!("This first statement is true");
  } else if num > 10.0 {
    println!("This second statement is false");
  } else {
    println!("Condition was false");
  }

  // conditions MUST be a boolean
  let condition: bool = true;
  let number: i8 = if condition { 10 } else { 20 };
  println!("Number value is: {}", number);


  // loops
  loop {
    println!("Will execute until break statement");
    break;
  }

  let mut counter: i8 = 0;
  let result: i8 = loop {
      counter += 1;
      if counter == 15 {
          println!("Break statement");
          break counter; // this will return counter value
      }
  };

  println!("Result value is: {}", result);

  let mut counter: i8 = 3;
  while counter != 0 {
      println!("{}", counter);
      counter -= 1;
  }

  println!("Lift off!");


  let numbers_array: [i8; 4] = [10, 20, 30, 40];
  for element in numbers_array.iter() {
      println!("The value is {}", element);
  }

  // for loop in a range
  for number in 1..4 {
      println!("The value is {}", number);
  }
```








