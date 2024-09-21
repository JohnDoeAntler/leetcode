fn main() {
    println!("Hello, world!");
}

// --

struct Codec {}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Codec { }
    }
	
    // Encodes a URL to a shortened URL.
    fn encode(&self, longURL: String) -> String {
        longURL
    }
	
    // Decodes a shortened URL to its original URL.
    fn decode(&self, shortURL: String) -> String {
        shortURL
    }
}
