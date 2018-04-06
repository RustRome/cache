extern crate cache;
#[macro_use]
extern crate serde_derive;

use cache::RedisCache;

fn cache() -> RedisCache {
    let redis = RedisCache::connect("redis://127.0.0.1/");
    // redis.clear();
    std::thread::sleep_ms(1000);
    redis
}
fn person() -> Person {
    Person {
        name: String::from("Enrico"),
    }
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Person {
    name: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct PersonFail {
    name: String,
}
#[test]
fn test_connection() {
    let _cache = cache();
}

#[test]
fn test_put_get() {
    let cache = cache();

    cache.put("1", &3);

    let val: i32 = cache.get("1");

    assert_eq!(val, 3);
}

#[test]
fn test_simple_put_get_fail() {
    let cache = cache();

    cache.put("1", &3);

    let val: String = cache.get("1");

    assert_eq!(val, "fail");
}

#[test]
fn test_not_found() {
    let cache = cache();

    let val: String = cache.get("2");

    assert_eq!(val, "fail");
}

#[test]
fn test_complex_put_get() {
    let cache = cache();

    cache.put("3", &person());

    let p: Person = cache.get("3");

    assert_eq!(p, person());
}
#[test]
fn test_complex_put_get_fail() {
    let cache = cache();

    cache.put("3", &person());

    let p: PersonFail = cache.get("3");
}
