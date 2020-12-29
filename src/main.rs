
use std::collections::HashMap;

use reqwest::header::Date;



// use reqwest;
fn main() {
    println!("Hello, world!");

    let results=get_current_date_2();
    if let Ok(val)= results{
        println!("the results are {:?}",val);

    }
    else{
        println!("no value has been found");

    }

    let results_1=match multiple_errors(){
        Ok(val)=>val,
        Err(err)=>panic!("error occurred")
    };

    // us



}


fn get_current_date()->Result<String,reqwest::Error>{
    let url= "https://postman-echo.com/time/object";
    let results=reqwest::get(url);
    let mut response =match results {
        Ok(res)=>res,
        Err(err)=>return Err(err)
    };
    let body=response.json::<HashMap<String,i32>>();
    let json =match body {
        Ok(json)=>json,
        Err(err)=>return Err(err)
    };
    let date=json["years"].to_string();
    Ok(date)

}

fn get_current_date_2()->Result<String,reqwest::Error>{
    let url= "https://postman-echo.com/time/object";
    let res=reqwest::get(url)?.json::<HashMap<String, i32>>()?;
    
    let date=res["years"].to_string();
    Ok(date)
}
// define enum results
// use std::error::Error;
enum Results<T,E> {
    Ok(T),
    Err(E)
}


fn get_results()->Results<String,String>{
    Results::Ok("sdsdf".to_string())
}

fn multiple_errors()->Result<i32,Box<dyn std::error::Error>>{
    let num =" 4";
    let results=num.trim().parse::<i32>()?;
    Ok(results)
}

fn get_date_3()->Result<String,Box<dyn std::error::Error>>{

    let url= "https://postman-echo.com/time/object";

    let mut response=reqwest::get(url).map_err(|_|MyCustomError::HttpError)?.json::<HashMap<String,i32>>().map_err(|_|MyCustomError::HttpError)?;

    let data=response["years"].to_string();

    Ok(data)
    
}


// creating custom errors
#[derive(Debug)]
pub enum MyCustomError {
    HttpError,
    ParseError,
}

impl std::error::Error for MyCustomError {}

use std::fmt;
impl fmt::Display for MyCustomError {
fn fmt(&self,f:&  mut fmt::Formatter)->fmt::Result{
    match self {
        MyCustomError::HttpError=>write!(f,"Http error"),
        MyCustomError::ParseError=>write!(f,"ParseError")
    }
}
}