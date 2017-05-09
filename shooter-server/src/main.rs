use std::net::UdpSocket;

fn main() {

    let socket = UdpSocket::bind("127.0.0.1:12345").unwrap();

    let mut buf = [0;256];
    let (n_bytes, src_adr) = socket.recv_from(&mut buf).unwrap();

    let string = unsafe { String::from_utf8_unchecked(buf.to_vec()) };
    println!("Received string: {}, from {:?}", string, src_adr);

}
