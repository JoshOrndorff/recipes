# Using Futures
> **WRITE THIS AS THE PULL REQUEST I'LL MAKE ASAP TO THE RESPECTIVE PARTS OF SUBSTRATE**

With the imminent formalization of the async/await syntax in Rust, 

> introduce this concept using the Wakers2 Post language (look at my notes as well)

## Improvements that can be made to the SRML

A future is essentially a proxy to an eventual response. We can chain combinators at the end of a future to manipulate the eventual response. In practice, this code pattern represents an optimization of our usage of resources.

### List of Places in the SRML This Can be Used

**TODO**
* make a pull request for one of these with a valid implementation by the end of the week
    * then add more...