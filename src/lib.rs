/* yams - yet another mock server
 *
 * Copyright (C) 2017 Alex Jeffries
 *
 * See LICENSE
 *
 */

extern crate hyper;
extern crate yaml_rust;

use self::yaml_rust::YamlLoader;

pub fn load_mocks(config_yaml: &str) -> Vec<Mock> {

    let mut mocks: Vec<Mock> = Vec::new();
    let config = &YamlLoader::load_from_str(&config_yaml).unwrap()[0];

    for doc in config.as_vec().unwrap() {
        let request_method: Option<hyper::Method> = match doc["request"]["method"].as_str() {
            None => None,
            Some(text) => {
                match text.parse() {
                    Err(_) => None,
                    Ok(method) => Some(method),
                }
            }
        };

        let request_path = String::from(doc["request"]["path"]
            .as_str()
            .unwrap());

        let matcher = Matcher {
            method: request_method,
            path: request_path,
        };

        let response_status = match doc["response"]["status"].as_i64() {
            None => hyper::StatusCode::Ok,
            Some(value) => {
                match hyper::StatusCode::try_from(value as u16) {
                    Err(_) => panic!("invalid status code!"),
                    Ok(status) => status,
                }
            }
        };

        let response_body = match doc["response"]["body"].as_str() {
            None => String::from(""),
            Some(body) => String::from(body),
        };

        let headers = match doc["response"]["headers"].as_vec() {
            None => Vec::new(),
            Some(header_docs) => {
                let mut hdrs = Vec::new();

                for header_doc in header_docs {
                    let header_map = header_doc.as_hash()
                        .expect("expected a yaml object!");

                    let (fst_key, fst_val) = header_map.iter().next().unwrap();

                    let header = Header {
                        key: fst_key.as_str().unwrap().to_string(),
                        val: fst_val.as_str().unwrap().to_string(),
                    };

                    hdrs.push(header);
                }

                hdrs
            }
        };

        let response = Response {
            status: response_status,
            body: response_body,
            headers: headers,
        };

        mocks.push(Mock {
            matcher: matcher,
            response: response,
        });
    }

    mocks
}

pub struct Mock {
    matcher: Matcher,
    response: Response,
}

impl Mock {
    pub fn matcher(&self) -> &Matcher {
        &self.matcher
    }

    pub fn response(&self) -> &Response {
        &self.response
    }

    pub fn matches(&self, request: &hyper::Request) -> bool {
        let path_matches = self.matcher.path() == request.path();

        let method_matches = match *self.matcher.method() {
            None => true,
            Some(ref m) => m == request.method(),
        };

        path_matches && method_matches
    }
}

pub struct Matcher {
    method: Option<hyper::Method>,
    path: String,
}

impl Matcher {
    pub fn method(&self) -> &Option<hyper::Method> {
        &self.method
    }

    pub fn path(&self) -> &String {
        &self.path
    }
}

pub struct Response {
    status: hyper::StatusCode,
    headers: Vec<Header>,
    body: String,
}

impl Response {
    pub fn status(&self) -> hyper::StatusCode {
        self.status.clone()
    }

    // hyper::header::Headers does not impl Sync, so this is a hacky
    // workaround
    pub fn headers(&self) -> hyper::header::Headers {
        let mut headers = hyper::header::Headers::new();

        for header in &self.headers {
            headers.set_raw(header.key().clone(), header.val().clone().into_bytes());
        }

        headers
    }

    pub fn body(&self) -> String {
        self.body.clone()
    }
}

pub struct Header {
    key: String,
    val: String,
}

impl Header {
    pub fn key(&self) -> &String {
        &self.key
    }

    pub fn val(&self) -> &String {
        &self.val
    }
}

#[cfg(test)]
mod tests {
    extern crate hyper;

    // request tests

    #[test]
    fn loads_request_method_if_present() {
        let yaml_text = "
---
- request:
    method: GET
    path: /
  response:
    status: 200
";

        let expected = Some(hyper::Method::Get);
        let mocks = super::load_mocks(&yaml_text);

        assert_eq!(mocks.len(), 1);
        assert_eq!(mocks[0].matcher().method(), &expected);
    }

    #[test]
    fn loads_request_method_if_missing() {
        let yaml_text = "
---
- request:
    path: /
  response:
    status: 200
";
        let expected = None;
        let mocks = super::load_mocks(&yaml_text);

        assert_eq!(mocks.len(), 1);
        assert_eq!(mocks[0].matcher().method(), &expected);
    }

    #[test]
    fn loads_request_path_if_present() {
        let yaml_text = "
---
- request:
    path: /
  response:
    status: 200
";

        let expected = "/";
        let mocks = super::load_mocks(&yaml_text);

        assert_eq!(mocks.len(), 1);
        assert_eq!(mocks[0].matcher().path(), expected);
    }

    // response tests

    #[test]
    fn loads_response_status_if_present() {
        let yaml_text = "
---
- request:
    path: /
  response:
    status: 201
";

        let expected = hyper::StatusCode::Created;
        let mocks = super::load_mocks(&yaml_text);

        assert_eq!(mocks.len(), 1);
        assert_eq!(mocks[0].response().status(), expected);
    }

    #[test]
    fn loads_response_status_as_default_if_not_present() {
        let yaml_text = "
---
- request:
    path: /
  response:
    other: val
";

        let expected = hyper::StatusCode::Ok;
        let mocks = super::load_mocks(&yaml_text);

        assert_eq!(mocks.len(), 1);
        assert_eq!(mocks[0].response().status(), expected);
    }

    #[test]
    fn loads_response_body_if_present() {
        let yaml_text = "
---
- request:
    path: /
  response:
    body: the body
";

        let expected = "the body";
        let mocks = super::load_mocks(&yaml_text);

        assert_eq!(mocks.len(), 1);
        assert_eq!(mocks[0].response().body(), expected);
    }

    #[test]
    fn loads_response_body_as_default_if_not_present() {
        let yaml_text = "
---
- request:
    path: /
  response:
    other: val
";

        let expected = "";
        let mocks = super::load_mocks(&yaml_text);

        assert_eq!(mocks.len(), 1);
        assert_eq!(mocks[0].response().body(), expected);
    }

    #[test]
    fn loads_response_headers_as_default_if_not_present() {
        let yaml_text = "
---
- request:
    path: /
  response:
    other: val
";

        let expected = hyper::Headers::new();
        let mocks = super::load_mocks(&yaml_text);

        assert_eq!(mocks.len(), 1);
        assert_eq!(mocks[0].response().headers(), expected);
    }

    #[test]
    fn loads_response_headers_if_present() {
        let yaml_text = "
---
- request:
    path: /
  response:
    other: val
    headers:
      - header1: val1
      - header2: val2
";

        let mut expected = hyper::Headers::new();
        expected.set_raw("header1", "val1");
        expected.set_raw("header2", "val2");

        let mocks = super::load_mocks(&yaml_text);

        assert_eq!(mocks.len(), 1);
        assert_eq!(mocks[0].response().headers(), expected);
    }
}
