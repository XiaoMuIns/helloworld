fn main() {
    println!("Hello, world!");
//    变量绑定
    let x = 5;
    println!("{}", x);

    let x = 100;
    let y = 200;
    println!("{}+{} = {}",x, y, x + y);

    let client = Client::new();

    let res = client.get("http://example.domain").send().unwrap();
    assert_eq!(res.status, hyper::Ok);

}
