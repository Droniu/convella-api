# convella-api

This is an API written in Rust which fetches *n* English nouns.

In order to start the server:
```
cargo run
```
Fetching:
```
http://127.0.0.1:8000/words/<n>
```
Where n is amount of words we want.

English nouns are taken from the following source:
https://www.desiquintans.com/nounlist
