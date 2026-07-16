use std::io;
use url::Url;
use clap::Parser;
use std::error::Error;
use std::net::TcpStream;
use std::io::{Read, Write};

//TODO: [] Implement Extensibility for HTTPS

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    url: String,
}

enum Scheme {
    Http,
    Https,
}
struct RequestTarget {
    // the struct is like a class in OOP
    scheme: Scheme,
    host: String,
    port: u16,
    path: String,
}

impl RequestTarget {
    // implement the struct
    fn address(&self) -> String {
        //  methods like on a class in OOP
        format!("{}:{}", self.host, self.port)
    }
    fn request_bytes(&self) -> Vec<u8> {
        // Vector taking in unsigned 8-bit integers
        format!(
            "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
            self.path, self.host
        )
            .into_bytes()
    }
}

fn parse_target(url_str: &str) -> Result<RequestTarget, std::io::Error> {
    let url = Url::parse(url_str)
        .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))?;

    // map the string to my enum
    let scheme = match url.scheme() {
        "http" => Scheme::Http,
        "https" => Scheme::Https,
        _ => {
            return Err(std::io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid scheme"
            ));
        }
    };

    let host = url
        .host_str()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Missing host"))?
        .to_string();

    let port = url
        .port_or_known_default()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Missing port"))?;

    let path = url.path().to_string();

    Ok(RequestTarget {
        scheme,
        host,
        port,
        path,
    })
}

fn parse_response<'buf>(
    response: &'buf [u8],
    headers: &mut [httparse::Header<'buf>],
) -> Result<httparse::Status<usize>, httparse::Error> {
    let mut resp = httparse::Response::new(headers);
    resp.parse(response)
}

fn send_request<Stream: Write>(stream: &mut Stream, request: &[u8]) -> std::io::Result<()> {
    stream.write_all(request)
}

fn read_response<Stream: Read>(stream: &mut Stream) -> std::io::Result<Vec<u8>> {
    let mut response = Vec::new(); // growing heap buffer
    let mut chunk = [0u8; 512]; // temp stack buffer, reused each read

    loop {
        let n = stream.read(&mut chunk)?;

        if n == 0 {
            break;
        }
        response.extend_from_slice(&chunk[..n]);
    }

    Ok(response)
}

fn print_response(response: &[u8]) {
    //NOTE: No `Result` used, because `from_utf8_lossy` cannot fail
    let text = String::from_utf8_lossy(response);
    println!("{}", text);
}

trait ReadWrite: Read + Write {}
impl<T: Read + Write> ReadWrite for T {}
fn connect(target: &RequestTarget) -> Result<Box<dyn ReadWrite>, io::Error> {
    match target.scheme {
        Scheme::Http => {
            let address = target.address();
            let stream = TcpStream::connect(address)?;
            let boxed_stream = Box::new(stream);
            Ok(boxed_stream)
        }

        Scheme::Https => {
            //TODO: Implement the Https scheme
            todo!()
        }
    }
}

fn fetch_response<Stream: Read + Write>(
    stream: &mut Stream,
    request: &[u8],
) -> std::io::Result<Vec<u8>> {
    send_request(stream, request)?;
    read_response(stream)
}

fn print_parse_status(response: &[u8]) -> Result<(), httparse::Error> {
    let mut headers = [httparse::EMPTY_HEADER; 16]; // fixed-size array, room for up to 16 headers
    let status = parse_response(response, &mut headers)?;
    println!("Status: {:?}", status);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let agrs = Args::parse();
    let target = parse_target(&agrs.url)?;

    // target url
    let address = target.address();

    println!("TCP Parser: Attempting to connect to {}...\n", address);

    // passes the entire struct `RequestTarget`
    let mut stream = match connect(&target) {
        Ok(s) => {
            println!("Connected!");
            s // returns the stream from match
        }
        Err(e) => {
            eprintln!("Connection failed: {}", e);
            return Ok(());
        }
    };

    // store the response
    let response = fetch_response(&mut stream, &target.request_bytes())?;
    print_response(&response);
    print_parse_status(&response)?;

    Ok(())
}
