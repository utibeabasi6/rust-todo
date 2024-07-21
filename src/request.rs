#[derive(Debug)]
#[allow(dead_code)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub version: String,
    pub headers: Vec<Header>,
    pub body: String
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Header {
    pub key: String,
    pub value: String
}

impl Request {
    pub fn new(method: String, headers: Vec<Header>, path: String, version: String, body: String) -> Self {
        Request { method, headers, path, version, body }
    }

    pub fn build_request_body(request_array: Vec<String>) -> Self {
        let request_body = request_array[request_array.len() -1].clone();
        let request_metadata: Vec<String> = request_array[0]
            .split(" ")
            .filter_map(|l| match l.is_empty() {
                true => None,
                false => Some(l.to_string()),
            })
            .collect();
        let headers_array: Vec<Header> = request_array[1..request_array.len() - 2]
            .iter()
            .map(|h| h.split_once(":"))
            .filter_map(|h| match h {
                Some(h) => Some(Header {
                    key: h.0.trim().to_string(),
                    value: h.1.trim().to_string(),
                }),
                None => {
                    println!("Failed to process header {:?}", h);
                    None
                }
            })
            .collect();
    
        Self::new(request_metadata[0][..].to_string(), headers_array, request_metadata[1][..].to_string(), request_metadata[2][..].to_string(), request_body[..].to_string())
    }
}