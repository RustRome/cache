extern crate cache;

use cache::RedisCache;

fn cache() -> RedisCache {
    RedisCache::connect("redis://127.0.0.1/")
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
fn test_put_get_fail() {
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
