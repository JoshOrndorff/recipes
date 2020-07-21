# HTTP Fetching and JSON Parsing in Off-chain Workers

_[`pallets/offchain-demo`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/offchain-demo)_

## HTTP Fetching

In traditional web app, it is often necessary to communicate with third-party APIs to fetch data
that the app itself does not contains. But this becomes tricky in blockchain decentralized apps
because HTTP requests are indeterministic. There are uncertainty in terms of whether the HTTP
request will come back, how long it takes, and if the result stays the same when the result is being
validated by another node at a future point.

In Substrate, we solve this problem by using off-chain workers to issue HTTP requests and get the
result back.

In `pallets/offchain-demo/src/lib.rs`, we have an example of fetching information of github
organization `substrate-developer-hub` via its public API. Then we extract the `login`, `blog`, and
`public_repos` values out.

First, include the tools implemented in `sp_runtime::offchain` at the top.

```rust
use sp_runtime::{
	offchain as rt_offchain
}
```

We then issue http requests inside the `fetch_from_remote()` function.

```rust
// Initiate an external HTTP GET request. This is using high-level wrappers from `sp_runtime`.
let request = rt_offchain::http::Request::get(HTTP_REMOTE_REQUEST);
```

We should also set a timeout period so the http request does not hold indefinitely. For github API
usage, we also need to add extra HTTP header information to it. This is how we do it.

```rust
pub const FETCH_TIMEOUT_PERIOD: u64 = 3000; // in milli-seconds
pub const HTTP_HEADER_USER_AGENT: &str = "my-github-username";

// ...
// Keeping the offchain worker execution time reasonable, so limiting the call to be within 3s.
let timeout = sp_io::offchain::timestamp()
	.add(rt_offchain::Duration::from_millis(FETCH_TIMEOUT_PERIOD));

// For github API request, we also need to specify `user-agent` in http request header.
//   See: https://developer.github.com/v3/#user-agent-required
let pending = request
	.add_header("User-Agent", HTTP_HEADER_USER_AGENT)
	.deadline(timeout) // Setting the timeout time
	.send() // Sending the request out by the host
	.map_err(|_| <Error<T>>::HttpFetchingError)?; // Here we capture and return any http error.
```

HTTP requests from off-chain worker are fetched asynchronously. Here we use `try_wait()` to wait for
the result to come back, and terminate and return if any errors occured.

Then, We check for the response status code to ensure it is okay with HTTP status code equals
to 200. Any status code that is non-200 is regarded as error and return.

```rust
let response = pending.try_wait(timeout)
	.map_err(|_| <Error<T>>::HttpFetchingError)?
	.map_err(|_| <Error<T>>::HttpFetchingError)?;

if response.code != 200 {
	debug::error!("Unexpected http request status code: {}", response.code);
	return Err(<Error<T>>::HttpFetchingError);
}
```

Finally, get the response back with `response.body()` iterator. Since we are in a `no_std`
environment, we collect them back as a vector of bytes instead of a string and return.

```rust
Ok(response.body().collect::<Vec<u8>>())
```

## JSON Parsing

We usually get JSON objects back when requesting from HTTP APIs. The next task is to parse the JSON
object and fetch the required (key, value) pair out. This is demonstrated in the `fetch_n_parse`
function.

### Setup

In Rust, `serde` and `serde-json` are the popular combo-package used for JSON parsing. Due to the
project setup of compiling Substrate node with `serde` feature `std` on and cargo feature
unification limitation, we cannot simultaneously have `serde` feature `std` off (`no_std` on) when
compiling the runtime
([details described in this issue](https://github.com/rust-lang/cargo/issues/4463)). So we are going
to use a renamed `serde` crate, `alt_serde`, in our offchain-demo pallet to remedy this situation.

src: `pallets/offchain-demo/Cargo.toml`

```toml
[package]
# ...

[dependencies]
# external dependencies
# ...

alt_serde = { version = "1", default-features = false, features = ["derive"] }
# updated to `alt_serde_json` when latest version supporting feature `alloc` is released
serde_json = { version = "1", default-features = false, git = "https://github.com/Xanewok/json", branch = "no-std", features = ["alloc"] }

# ...
```

We also use a modified version of `serde_json` with the latest `alloc` feature and depending on `alt_serde`.

### Deserializing JSON string to struct

Then we use the usual `serde-derive` approach on deserializing. First we define the struct with
fields we are interested to extract out.

src: `pallets/offchain-demo/src/lib.rs`

```rust
// We use `alt_serde`, and Xanewok-modified `serde_json` so that we can compile the program
//   with serde(features `std`) and alt_serde(features `no_std`).
use alt_serde::{Deserialize, Deserializer};

// Specifying serde path as `alt_serde`
// ref: https://serde.rs/container-attrs.html#crate
#[serde(crate = "alt_serde")]
#[derive(Deserialize, Encode, Decode, Default)]
struct GithubInfo {
	// Specify our own deserializing function to convert JSON string to vector of bytes
	#[serde(deserialize_with = "de_string_to_bytes")]
	login: Vec<u8>,
	#[serde(deserialize_with = "de_string_to_bytes")]
	blog: Vec<u8>,
	public_repos: u32,
}
```

By default, `serde` deserialize JSON string to the datatype `String`. We want to write our own
deserializer to convert it to vector of bytes.

```rust
pub fn de_string_to_bytes<'de, D>(de: D) -> Result<Vec<u8>, D::Error>
where D: Deserializer<'de> {
	let s: &str = Deserialize::deserialize(de)?;
	Ok(s.as_bytes().to_vec())
}
```

Now the actual deserialization takes place in the `Self::fetch_n_parse` function.

```rust
/// Fetch from remote and deserialize the JSON to a struct
fn fetch_n_parse() -> Result<GithubInfo, Error<T>> {
	let resp_bytes = Self::fetch_from_remote()
		.map_err(|e| {
			debug::error!("fetch_from_remote error: {:?}", e);
			<Error<T>>::HttpFetchingError
		})?;

	let resp_str = str::from_utf8(&resp_bytes)
		.map_err(|_| <Error<T>>::HttpFetchingError)?;

	// Deserializing JSON to struct, thanks to `serde` and `serde_derive`
	let gh_info: GithubInfo = serde_json::from_str(&resp_str).unwrap();
	Ok(gh_info)
}
```
