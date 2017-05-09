use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:12346").unwrap();

    socket.connect("127.0.0.1:12345");

    let to_send = b"foobar yo";
    socket.send(to_send);
}
