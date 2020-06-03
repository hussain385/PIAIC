// extern crate reqwest;
// fn main() {
//     match reqwest::get("https://httpbin.org/get"){
//         Ok(mut res) => {
//             match res.text(){
//                 Ok(text) => println!("the response is: {}",text),
//                 Err(_) => println!("ERROR!"),
//             }
//         }
//         Err(_) => println!("ERROR!!")
//     }
// }

// use std::collections::HashMap;
// use async_std::task;

// fn main() -> Result<(),Box<dyn std::error::Error + Send +Sync>>{
//     task::block_on(async{
//         let res = reqwest::blocking::get("https://httpbin.org/ip")?
//             .json::<HashMap<String,String>>()?;
//         println!("{:#?}",res);
//         Ok(())
//     })
// }

use std::convert::Infallible;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};

async fn hello(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Hello World!")))
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let make_svc = make_service_fn(|_conn| {
        async { Ok::<_, Infallible>(service_fn(hello)) }
    });
    let addr = ([192, 168, 0, 108], 3000).into();
    let server = Server::bind(&addr).serve(make_svc);
    println!("Listening on http://{}", addr);
    server.await?;
    Ok(())
}
