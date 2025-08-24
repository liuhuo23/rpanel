use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::{HttpRequest, HttpResponse, Responder};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(bound = "T: Serialize + DeserializeOwned")]
pub struct Response<T: Serialize + DeserializeOwned> {
    pub data: Option<T>,
    pub msg: String,
    pub code: u16,
}

impl<T: Serialize + DeserializeOwned> Response<T> {
    pub fn new(data: Option<T>, msg: String, code: u16) -> Self {
        Response {
            data: data,
            msg,
            code,
        }
    }
}

impl<T: Serialize + DeserializeOwned> Responder for Response<T> {
    type Body = BoxBody;
    fn respond_to(self, _: &HttpRequest) -> HttpResponse {
        let status = match self.code {
            0 => StatusCode::OK,
            404 => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        HttpResponse::build(status).json(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Serialize, Deserialize, Debug)]
    struct TestData {
        field1: String,
        field2: i32,
    }

    #[test]
    fn test_response_serialization() {
        let response = Response {
            data: Some(TestData {
                field1: "value1".into(),
                field2: 42,
            }),
            msg: "Success".into(),
            code: 200,
        };

        let json = serde_json::to_string(&response).unwrap();
        let deserialized: Response<TestData> = serde_json::from_str(&json).unwrap();

        let response_data = response.data.unwrap();
        let deserialized_data = deserialized.data.unwrap();

        assert_eq!(response_data.field1, deserialized_data.field1);
        assert_eq!(response_data.field2, deserialized_data.field2);
        assert_eq!(response.msg, deserialized.msg);
        assert_eq!(response.code, deserialized.code);
    }
}
