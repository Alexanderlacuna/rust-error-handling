## Handling errors in rust

- In rust there are no exception as seen in other language,errors are handled using the Result enum type  which has got two variants:
     - Ok which means a success
     - Which means an error was raised


``` rust
  enum Results <T,E>{
      Ok(T),
      Err(E)
  }
```

- errors can either be recovarable not recovarable


1.using unwrap
- this ignores the error  for example
  ``` rust 

  <!-- reading content of a file -->
  let content=fs:read_to_string(filename).unwrap()

  ``` 
    - in this case we are unwrapping the results we 
    case the succeeds content will get the content value of the read operatiog
    else if there is an error the program panics at run time
  <!-- in this case you are sure that the error will not occur -->
  if there is an error the program will panic cause the error has not being handled

2. Expect
- works similar to unwrap but in this case we provide a custom error

``` rust

let content =fs:read_to_string(filename).expect("Error while reading file")

```

3.using fallback

``` rust
 let content= fs::read_to_string(filename).unwrap("no item".to_string())

```
4. Bubbling the error to the calling function
  - let take  an example where we have a function that reads a a file and return the content we could bubble 
  the error to the calling function incase in occurs.We  use  the ? operator

  ``` rust

  fn read_file_content(filename:&str)->Result<String,Box<std::error:Error>>{

      let results=fs::read_to_string(filename)?;
      Ok(results)

  }

  <!-- calling the function-->

  fn main() {

      let results=read_file_content("file_1.txt")

      <!-- matching the results -->
      match results {

          Ok(string_value)=>{
              println!("the string value is {:?}",string_value)
          }

        Err(_)=>{
            println!("error occurred reading the file")
        }
      }
  }

  ```


### Creating custom errors

- if you need to implement your own custom types
you need  to implement the Error trait just like any other trait
-also your error type must aslo implement the Debug and and Display::fmt trait

#### *example shown below*

``` rust
#[derive(Debug)]   //implements the Debug trait
enum MyError {
  Error1,
  Error2
}

// implementing the error trait own type MyError
impl std::Error for MyError {};


impl fmt::Display for MyError {
  fn fmt(&self,f:& mut fmt::Formatter)->fmt::Result{
    match self {
      MyError::Error1=>write(f,"Error 1 occurred "),
      MyError::Error2=>write(f,"Error 2 occurred")
    }
  }
}



```
