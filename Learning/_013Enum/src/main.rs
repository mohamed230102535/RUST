// fn main() {

//     enum IpAddrKind{
//         V4,
//         V6
//     }


//     let four= IpAddrKind::V4;
//     let six = IpAddrKind::V6;

//     fn route(ip_kind: IpAddrKind) {
//         println!("ip_kind: {:?}", ip_kind);
//     }

//     route(IpAddrKind::V4);
//     route(IpAddrKind::V6);


//     //using struct with data that is in enum

//     struct IpAddr{
//         kind: IpAddrKind,
//         address: String,
    
//     }

//     let homes = IpAddr{
//         kind: IpAddrKind::V4,
//         address: String::from("10.0.0.2"),
//     };

//     let loopback = IpAddr{
//         kind: IpAddrKind::V4,
//         address: String::from("127.0.0.1"),
//     };

//     //using enum with data

//     enum IpAddr2{
//         V4(u8, u8, u8, u8),
//         V6(String),
//     }
//     //V4(String),
//     // let home = IpAddr2::V4(String::from("10.0.0.2"));
//     // let loopback = IpAddr2::V4(String::from("127.0.0.1"));

//     let home = IpAddr2::V4(10,0,0,2);
//     let loopback = IpAddr2::V4(String::from(127,0,0,1));
// }


fn main(){
    let req_get = Request::GET{
        url: String::from("https://www.google.com"),
    };
    let req_post =Request::POST{
        url: String::from("https://www.google.com"),
        body: String::from("Hello Google"),
    };
    let req_delete = Request::DELETE{
        url: String::from("https://www.google.com"),
    };

    req(req_get);
    req(req_post);  
    req(req_delete);

}

enum Request{
    GET {url: String},
    POST {url: String, body: String},
    DELETE {url: String},
}
fn req(req: Request){
   match req{

    Request::GET{url} =>{
        println!("GET request to: {}", url);
    }

    Request::POST{url, body} =>{
        println!("POST request to: {} with body: {}", url, body);
    }

    Request::DELETE{url} =>{
        println!("DELETE request to: {}", url);
    }
   };
}