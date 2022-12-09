
struct City {
    name: String,
    population: i64,
    country: String,
}

fn sort_cities(cities: &mut Vec<City>) {
    cities.sort_by_key(|city| -city.population);
}

// Sort by any of several different statistics.
// fn sort_by_statistic(cities: &mut Vec<City>, stat: Statistic) {
//     cities.sort_by_key(|city| -city.get_statistic(stat));
// }

struct Statistic{}

impl City {
    fn get_statistic(self: &Self, stat: &Statistic)-> i64{0}
}

use std::thread;

fn start_sorting_thread(mut cities: Vec<City>, stat: Statistic)
    -> thread::JoinHandle<Vec<City>> 
{
    let mut key_fn =move |city: &City| -> i64 { city.get_statistic(&stat) };

    thread::spawn(move || {
        cities.sort_by_key(key_fn);
        cities
    })
}

fn main() {
    println!("Hello, world!");
}

fn count_selected_cities<F>(cities: &Vec<City>, test_fn: F) -> usize
    where F: Fn(&City) -> bool
{
    let mut count = 0;
    for city in cities {
        if test_fn(city) {
            count += 1
        }
    }
    count
}

// fn(&City) -> bool // fn type (functions only)
// Fn(&City) -> bool // Fn trait (both functions and closures)

// count_selected_cities(
//     &my_cities,
//     has_monster_attacks
// );

// count_selected_cities(
//     &my_cities,
//     |city| city.monster_attack_risk > limit
// );

// fn call_twice<F>(mut closure: F) where F: FnMut() {
//     closure();
//     closure();
// }

// let mut i = 0;
// call_twice(|| i += 1);
// assert_eq!(i, 2);

// Callbacks

// App::new()
//     .route("/", web::get().to(get_index))
//     .route("/gcd", web::post().to(post_gcd))


// App::new()
//     .route("/", web::get().to(|| {
//         HttpResponse::Ok()
//             .content_type("text/html")
//             .body("<title>GCD Calculator</title>...")
//     }))
//     .route("/gcd", web::post().to(|form: web::Form<GcdParameters>| {
//         HttpResponse::Ok()
//             .content_type("text/html")
//             .body(format!("The GCD of {} and {} is {}.",
//                             form.n, form.m, gcd(form.n, form.m)))
//     }))

struct Request {
    method: String,
    url: String,
    headers: HashMap<String, String>,
    body: Vec<u8>
}

struct Response {
    code: u32,
    headers: HashMap<String, String>,
    body: Vec<u8>
}

type BoxedCallback = Box<dyn Fn(&Request) -> Response>;

struct BasicRouter {
    routes: HashMap<String, BoxedCallback>
}

impl BasicRouter {
    // Create an empty router.
    fn new() -> BasicRouter<C> {
        BasicRouter { routes: HashMap::new() }
    }

    // Add a route to the router
    fn add_route(&mut self, url: &str, callback: C)
        where C: Fn(&Request) -> Response + 'static
    {
        self.routes.insert(url.to_string(), Box::new(callback));
    }

    fn handle_request(&self, request: &Request) -> Response {
        match self.routes.get(&request.url) {
            None => not_found_response(),
            Some(callback) => callback(request),
        }
    }
}