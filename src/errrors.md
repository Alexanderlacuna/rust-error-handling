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