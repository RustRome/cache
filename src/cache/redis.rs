use redis::{Client, Commands, Connection};
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json;

pub struct RedisCache {
    redis: Client,
}

impl RedisCache {
    pub fn connect<T>(url: T) -> RedisCache
    where
        T: Into<String>,
    {
        let client = Client::open(url.into().as_str()).unwrap();
        // probe redis
        client.get_connection().unwrap();
        RedisCache { redis: client }
    }

    fn get_connection(&self) -> Connection {
        self.redis.get_connection().unwrap()
    }

    pub fn get<'a, K, V>(&self, key: K) -> V
    where
        K: Into<String>,
        V: DeserializeOwned,
    {
        let val: String = self.get_connection().get(key.into()).unwrap();
        serde_json::from_str(&val).unwrap()
    }

    pub fn put<K, V>(&self, key: K, val: &V)
    where
        K: Into<String>,
        V: Serialize,
    {
        let _: () = self.get_connection()
            .set(key.into(), serde_json::to_string(val).unwrap())
            .unwrap();
    }
}
