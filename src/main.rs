use lambda_http::{run, service_fn, Body, Error, IntoResponse, Request, RequestExt, Response};
use serde::Serialize;
use serde_json::json;

struct CarList {
    cars: Vec<Car>,
}

#[derive(Serialize)]
struct Car {
    name: String,
    price: i32,
}

impl CarList {
    fn new() -> CarList {
        CarList {
            cars: vec![
                Car {
                    name: String::from("porsche"),
                    price: 120_000,
                },
                Car {
                    name: String::from("ferrari"),
                    price: 340_000,
                },
                Car {
                    name: String::from("mazda"),
                    price: 32_000,
                },
            ],
        }
    }
}

fn get_car_from_name<'a>(car_name: &'a str, car_list: &'a CarList) -> Option<&'a Car> {
    let mut iter = car_list.cars.iter();
    iter.find(|car| car.name == car_name)
}

async fn build_success_response(car: &Car) -> Response<Body> {
    json!(car).into_response().await
}

async fn build_failure_response(error_message: &str) -> Response<Body> {
    Response::builder()
        .status(400)
        .header("content-type", "application/json")
        .body(Body::from(json!({"error": error_message}).to_string()))
        .expect("could not build the error response")
}

fn process_event<'a>(car_name: Option<&'a str>, car_list: &'a CarList) -> Result<&'a Car, &'a str> {
    match car_name {
        Some(car_name) => match get_car_from_name(car_name, car_list) {
            Some(car) => Ok(car),
            _ => Err("No car found for the given name"),
        },
        _ => Err("No car name provided"),
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    let all_cars = &CarList::new();
    let handler_func = |event: Request| async move {
        let response = match process_event(event.path_parameters().first("car_name"), all_cars) {
            Ok(car) => build_success_response(car).await,
            Err(error_message) => build_failure_response(error_message).await,
        };
        Result::<Response<Body>, Error>::Ok(response)
    };
    run(service_fn(handler_func)).await?;
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn new_car_list_test() {
        let all_cars = CarList::new();
        assert_eq!(3, all_cars.cars.len());
        let porsche = get_car_from_name("porsche", &all_cars);
        assert_eq!(120_000, porsche.unwrap().price);
        let ferrari = get_car_from_name("ferrari", &all_cars);
        assert_eq!(340_000, ferrari.unwrap().price);
        let mazda = get_car_from_name("mazda", &all_cars);
        assert_eq!(32_000, mazda.unwrap().price);
    }

    #[tokio::test]
    async fn build_success_response_test() {
        let test_car = Car {
            name: String::from("test_car"),
            price: 100_000,
        };
        let result = build_success_response(&test_car).await;
        let (parts, body) = result.into_parts();
        assert_eq!(200, parts.status);
        assert_eq!(
            "application/json",
            parts.headers.get("content-type").unwrap()
        );
        assert_eq!(
            "{\"name\":\"test_car\",\"price\":100000}",
            String::from_utf8(body.to_ascii_lowercase()).unwrap()
        )
    }

    #[tokio::test]
    async fn build_failure_response_test() {
        let result = build_failure_response("test error message").await;
        let (parts, body) = result.into_parts();
        assert_eq!(400, parts.status);
        assert_eq!(
            "application/json",
            parts.headers.get("content-type").unwrap()
        );
        assert_eq!(
            "{\"error\":\"test error message\"}",
            String::from_utf8(body.to_ascii_lowercase()).unwrap()
        )
    }

    #[test]
    fn process_event_valid_car_test() {
        let car_list = CarList::new();
        let res = process_event(Some("porsche"), &car_list);
        assert!(res.is_ok())
    }

    #[test]
    fn process_event_invalid_car_test() {
        let car_list = CarList::new();
        let res = process_event(Some("pagani"), &car_list);
        assert!(matches!(res, Err("No car found for the given name")));
    }

    #[test]
    fn process_event_no_car_test() {
        let car_list = CarList::new();
        let res = process_event(None, &car_list);
        assert!(matches!(res, Err("No car name provided")));
    }
}
