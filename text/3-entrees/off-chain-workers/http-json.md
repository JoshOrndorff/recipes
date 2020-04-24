# HTTP Fetching and JSON Parsing in Off-chain Workers

*[`pallets/offchain-demo`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/offchain-demo)*

## HTTP Fetching

In traditional web app, it is often necessary to communicate with third-party APIs to fetch data that the app itself does not contains. But this becomes tricky in blockchain decentralized apps because HTTP requests are indeterministic. There are uncertainty in terms of whether the HTTP request will come back, how long it takes, and if the result stays the same when the result is being validated by another node at a future point.

In Substrate, we solve this problem by using off-chain workers to issue HTTP requests and get the result back.

In `pallets/offchain-demo/src/lib.rs`, we have an example of fetching information of github organization `substrate-developer-hub` via [its public API](https://api.github.com/orgs/substrate-developer-hub). Then we extract the `login` and `blog` value out.

First, include the tools implemented in `sp_runtime::offchain` at the top.

```rust
use sp_runtime::{
	offchain as rt_offchain
}
```

We then issue http requests inside the `fetch_from_remote()` function.

```rust
// Initiate an external HTTP GET request. This is using high-level wrappers from `sp_runtime`.
let remote_url = str::from_utf8(&remote_url_bytes)
	.map_err(|_| <Error<T>>::HttpFetchingError)?;

let request = rt_offchain::http::Request::get(remote_url);
```

We should also set a timeout period so the http request does not hold indefinitely. For github API usage, we also need to add extra HTTP header information to it. This is how we do it.

```rust
// Keeping the offchain worker execution time reasonable, so limiting the call to be within 3s.
//   `sp_io` pallet offers a timestamp() to get the current timestamp from off-chain perspective.
let timeout = sp_io::offchain::timestamp().add(rt_offchain::Duration::from_millis(3000));

// For github API request, we also need to specify `user-agent` in http request header.
//   See: https://developer.github.com/v3/#user-agent-required
let pending = request
	.add_header("User-Agent", str::from_utf8(&user_agent)
		.map_err(|_| <Error<T>>::HttpFetchingError)?)
	.deadline(timeout) // Setting the timeout time
	.send() // Sending the request out by the host
	.map_err(|_| <Error<T>>::HttpFetchingError)?; // Here we capture and return any http error.
```

HTTP requests from off-chain worker are fetched asynchronously. Here we use `try_wait()` to wait for the result to come back, and terminate and return if any errors occured.

Then, We check for the response status code to ensure it is okay with HTTP status code equals to 200. Any status code that is non-200 is regarded as error and return.

```rust
let response = pending.try_wait(timeout)
	.map_err(|_| <Error<T>>::HttpFetchingError)?
	.map_err(|_| <Error<T>>::HttpFetchingError)?;

if response.code != 200 {
	debug::error!("Unexpected http request status code: {}", response.code);
	return Err(<Error<T>>::HttpFetchingError);
}
```

Finally, get the response back with `response.body()` iterator. Since we are in a `no_std` environment, we collect them back as a vector of bytes instead of a string and return.

```rust
Ok(response.body().collect::<Vec<u8>>())
```

## JSON Parsing

We usually get JSON objects back when doing HTTP API requests. The next task is to parse the JSON object and fetch the required (key, value) pair out. This is demonstrated in the `parse_for_value` function.

In Rust, `serde` and `serde-json` are the popular combo-package used for JSON parsing. But they only work in `std` environment, so we need to use something else. We have a neat community-written tool to parse JSON in a `no-std` environment, [`simple_json2`](https://github.com/jimmychu0807/simple-json2).

First, include the library at the top

```rust
use simple_json2::{ json::{ JsonObject, JsonValue }, parse_json };
```

Then pass the whole JSON string to get a `JsonValue` out for further parsing.

```rust
let json: JsonValue = parse_json(&json_str)
```

`JsonValue` is actually an enum object consists of the following:

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
	Object(JsonObject),
	Array(Vec<JsonValue>),
	String(Vec<char>),
	Number(NumberValue),
	Boolean(bool),
	Null,
}
```

There are a few convenient methods to extract these inner values out from the enum, namely `get_object()`, `get_array()`, `get_string()`, etc. Details can be seen [in the code](https://github.com/jimmychu0807/simple-json2/blob/master/src/json.rs#L214-L280).

To extract out the value of a particular key in the JSON, we first get the `JsonObject` out from `JsonValue`.

```rust
let json_obj: &JsonObject = json.get_object()
	.map_err(|_| <Error<T>>::JsonParsingError)?;
```

Then we look for the (`key`, `value`) pair that has the key matched with the one we are looking for.

```rust
// We iterate through the key and retrieve the (key, value) pair that match the `key`
//   parameter.
// `key_val.0` contains the key and `key_val.1` contains the value.
let key_val = json_obj
	.iter()
	.find(|(k, _)| *k == key.chars().collect::<Vec<char>>())
	.ok_or(<Error<T>>::JsonParsingError)?;
```

Finally, we return the second item in the tuple, which is the value we want, in bytes form.

```rust
// We assume the value is a string, so we use `get_bytes()` to collect them back.
//   In a real app, you may need to catch the error and further process it if the value is not
//   a string.
key_val.1.get_bytes()
	.map_err(|_| <Error<T>>::JsonParsingError)
```
