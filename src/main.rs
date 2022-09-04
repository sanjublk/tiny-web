mod lib;


fn index(req: lib::Request) -> lib::Response {
    lib::Response { body: "<h1>welcome to home page</h1>".to_owned(), status_code: 200, content_type: "text/html".to_owned() }
}

fn main() {

    let mut web = lib::TinyWeb::new();
    web.add_route("", index);
    web.run("1234");

}

