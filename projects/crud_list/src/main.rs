extern crate burner;

use burner::{Server, Request, Response, RouterService};




fn main() {
        //Create the server
    let mut server = Server::new();

    // Create a route controller closure
    let controller = |_req: &Request, res: &mut Response| {
        // Set the HTTP status code to be 200 OK, default is 404
        res.status(200);
    };


    // Register controller on server to be triggered by a request to path / and method: GET
    let path = "/";
    server.get(path, Box::new(controller));

    // Start the server on port
    let port = 6789;

    // Add http response like 200 or Connected
/*
    let contents

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );
*/
    server.listen(port);
}
