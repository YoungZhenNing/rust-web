use std::collections::HashMap;
use std::io::{Result, Write};

#[derive(Debug, PartialEq ,Clone)]
pub struct HttpResponse<'a> {
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str,&'a str>>,
    body: Option<String>,
}

impl <'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        Self {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            headers: None,
            body: None,
        }
    }
}

impl <'a> From<HttpResponse<'a>> for String {
    fn from(res: HttpResponse) -> String {
        format!{
            "{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
            res.get_version(),
            res.get_status_code(),
            res.get_status_text(),
            res.get_headers(),
            res.get_body().len(),
            res.get_body()
        }
    }
}


impl <'a> HttpResponse<'a> {
    pub fn new(
        status_code: &'a str,
        headers: Option<HashMap<&'a str,&'a str>>,
        body: Option<String>
    ) -> HttpResponse<'a> {
        let mut response:HttpResponse<'a> = HttpResponse::default();
        if status_code != "200" {
            response.status_code = status_code;
        };
        response.headers = match &headers {
            Some(_s) => headers,
            None => {
                let mut h = HashMap::new();
                h.insert("Content-Type","text/html");
                Some(h)
            }
        };
        response.status_text = match response.status_code {
            "200" => "OK",
            "400" => "Bad Request",
            "500" => "Internal Server Error",
            _ => "Not Found"
        };

        response.body = body;
        response
    }

    pub fn send_response(&self,write_stream: &mut impl Write) -> Result<()>{
        let res1 = self.clone();
        let response_string: String = res1.into();
        let _ = write!(write_stream,"{}",response_string);
        Ok(())
    }

    fn get_status_code(&self) -> &str{
        self.status_code
    }
    fn get_status_text(&self) -> &str{
        self.status_text
    }
    fn get_version(&self) -> &str{
        self.version
    }
    fn get_headers(&self) -> String {
        let map: HashMap<&str,&str> = self.headers.clone().unwrap();
        let mut header_string = String::from("");

        for (k,v) in map {
            header_string = format!("{}{}:{}\r\n",header_string,k,v);
        }
        header_string
    }
    pub fn get_body(&self) -> &str {
        match &self.body {
            Some(s) => s.as_str(),
            None => "",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_struct_creation_200() {
        let response_actual = HttpResponse::new(
            "200",
            None,
            Some("xxxx".into()),
        );
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type","text/html");
                Some(h)
            },
            body: Some("xxxx".into())
        };
        assert_eq!(response_actual,response_expected)
    }

    #[test]
    fn test_response_struct_creation_404() {
        let response_actual = HttpResponse::new(
            "404",
            None,
            Some("xxxx".into()),
        );
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "Not Found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type","text/html");
                Some(h)
            },
            body: Some("xxxx".into())
        };
        assert_eq!(response_actual,response_expected)
    }

    #[test]
    fn test_http_response_creation(){
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "Not Found",
            headers: {
                let mut h =  HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("xxxx".into())
        };
        let http_string: String = response_expected.into();
        let actual_string = "HTTP/1.1 404 Not Found\r\nContent-Type:text/html\r\nContent-Length: 4\r\n\r\nxxxx";
        assert_eq!(http_string,actual_string);
    }
}