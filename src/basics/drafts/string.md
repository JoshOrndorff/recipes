## String Storage (as Bytemap) <a name = "string" ></a>

Runtime storage is for storing the state of the business logic for which the runtime operates. If arbitrary storage must be stored in the runtime, it is better to create a bytearray(`Vec<u8>`). With that said, Substrate doesn't directly support `String`. To achieve the same functionality, it is better to store a hash to a service like IPFS to then use the hash to fetch data for the UI (*recipe coming soon!*).

Here's a workaround to store a string in the runtime using JavaScript to convert the string to hex and back. You probably shouldn't do this...

```rust, ignore
use srml_support::{StorageValue, dispatch::Result};
use rstd::prelude::*;

pub trait Trait: system::Trait {}

decl_module! {
  pub struct Module<T: Trait> for enum Call where origin: T::Origin {
    fn set_value(_origin, value: Vec<u8>) -> Result {
      <Value<T>>::put(value);
      Ok(())
    }
  }
}

decl_storage! {
  trait Store for Module<T: Trait> as RuntimeExampleStorage {
    Value: Vec<u8>;
  }
}
```

Store the string as a bytearray, which is inputted into the Polkadot UI as a hex string. These helper functions in JavaScript enable conversation from a string to hex and back, all in the browser console.

```javascript
function toHex(s) {
    var s = unescape(encodeURIComponent(s))
    var h = '0x'
    for (var i = 0; i < s.length; i++) {
        h += s.charCodeAt(i).toString(16)
    }
    return h
}

function fromHex(h) {
    var s = ''
    for (var i = 0; i < h.length; i+=2) {
        s += String.fromCharCode(parseInt(h.substr(i, 2), 16))
    }
    return decodeURIComponent(escape(s))
}
```