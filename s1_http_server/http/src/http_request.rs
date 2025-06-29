use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized
}

impl From<&str> for Method {
    fn from(s: &str) -> Self {
        match s {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Uninitialized
        }
    }
}

impl From<i32> for Method {
    fn from(value: i32) -> Self {
        match value {
            1 => Method::Get,
            2 => Method::Post,
            _ => Method::Uninitialized
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized
}

impl From<&str> for Version {
    fn from(s: &str) -> Self {
        match s {
            "HTTP/1.1" => Version::V1_1,
            _ => Version::Uninitialized
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String)
}

#[derive(Debug, PartialEq)]
pub struct HttpRequest {
    pub method: Method,
    pub resource: Resource,
    pub version: Version,
    pub headers: HashMap<String, String>,
    pub req_body: String
}

// 思考:为什么这里使用了String，而不是像Method和Version那样使用&str?
// 因为此处是需要解析String并将其用于构造结构体，涉及所有权
// 而Method和Version仅仅是根据&str进行分支判断
impl From<String> for HttpRequest {
    fn from(req: String) -> Self {
        let mut method = Method::Uninitialized;
        let mut version = Version::Uninitialized;
        let mut resource = Resource::Path("".to_string());
        let mut headers = HashMap::new();
        let mut req_body = "";
        for line in req.lines() {
            if line.contains("HTTP") { // case1:request line
                (method, resource, version) = process_req_line(line);
            } else if line.contains(":") { // case2:request header
                let (key, value) = process_header_line(line);
                headers.insert(key, value);
            } else if line.len() == 0 { // case3: empty line
                
            } else {// case4:req_body
                req_body = line;
            }
        }
        HttpRequest {
            method, 
            version, 
            resource, 
            headers, 
            req_body: req_body.to_string(),
        }
    }
}

fn process_req_line(line: &str) -> (Method, Resource, Version) {
    let mut words = line.split_whitespace();
    let method = words.next().unwrap();
    let resource = words.next().unwrap();
    let version = words.next().unwrap();
    (   
        method.into(),
        Resource::Path(resource.to_string()),
        version.into()
    )
}

fn process_header_line(s: &str) -> (String, String) {
    if let Some((key, value)) = s.split_once(':') {
        (key.trim().to_string(), value.trim().to_string())
    } else {
        (String::new(), String::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_method_into() {
        let method1: Method = "GET".into();
        let method2: Method = 2.into();
        assert_eq!(method1, Method::Get);
        assert_eq!(method2, Method::Post);
    }

    #[test]
    fn test_version_into() {
        let v: Version = "HTTP/1.1".into();
        assert_eq!(v, Version::V1_1);
    }

    #[test]
    fn test_read_http() {
        let s =
            String::from("GET /greeting HTTP/1.1\r\nHost: localhost:3000\r\nUser-Agent: curl/7.71.1\r\nAccept: */*\r\n\r\n'");
        let mut expected_header: HashMap<String, String> = HashMap::new();
        expected_header.insert("Host".into(), "localhost:3000".into());
        expected_header.insert("Accept".into(), "*/*".into());
        expected_header.insert("User-Agent".into(), "curl/7.71.1".into());

        println!("{:?}", expected_header);
        let req: HttpRequest = s.into();
        assert_eq!(Method::Get, req.method);
        assert_eq!(Version::V1_1, req.version);
        assert_eq!(Resource::Path("/greeting".to_string()), req.resource);
        assert_eq!(expected_header, req.headers);
    }
}