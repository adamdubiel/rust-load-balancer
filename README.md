rust-load-balancer
==================

This is a simple load balancer written in Rust to get to know new language.

## Features

This load balancer always returns consistent results for same configuration,
i.e. unless you change available groups or their weights, you can assume each
load-balancer node will return same group for given user id.

## Configuration

Configuration is read from `config.toml` file that should reside in same dir
as binary. Format:

```
group_name_a: weight_a (unsigned int)
group_name_b: weight_b
....
```

Weight specifies what is a chance of putting user into that group. For example:

```
A: 8
B: 2
```

means 80% of users will be assigned to group A, while only 20% will get group B.

## API

```
curl http://load-balancer/groups/<user-id>
```

## Running

Use `cargo` to run project:

```
cargo run
curl localhost:3000/groups/hello
```

## License

Apache v2
