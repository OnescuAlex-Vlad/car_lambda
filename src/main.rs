use lambda_http::{Response, Body};
use tracing_subscriber;

struct CarList {
    cars: Vec<Car>,
}

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
    panic!("")
}

fn main() {
    tracing_subscriber::fmt::init();
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
        let test_car = Car { name: String::from("test_car"), price: 100_000};
        let result = build_success_response(&test_car).await;
        let (parts, body) = result.into_parts();
        assert_eq!(200, parts.status);
        assert_eq!("application/json", parts.headers.get("content-type").unwrap());
        
    }
}
