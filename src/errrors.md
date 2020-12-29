## Handling errors in rust

1.using unwrap
- this ignores the error  for example
  
  let content=fs::read_to_string(filename).unwrap()
  <!-- in this case you are sure that the error will not occur -->
  if there is an error the program will panic cause the error has not being handled

2. Expect 
  - some errors cannot be recovered 
  so you can you use expect to panic with an error message
   - let content=fs::read_string(file_name).expect("cant read the file name)
3. Use fallback values
- we do this with a default value
- let port=env::var("PORT).unwrap_or("3000".to_string())

4. bubble up the error