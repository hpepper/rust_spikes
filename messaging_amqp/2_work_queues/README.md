# 2 work queues - fanout exchange

Show the fanout exchange.

[Work Queues](https://www.rabbitmq.com/tutorials/tutorial-two-python.html)

Messages being produced are distributed among the consumers, round-robin style.
Only one consumer will receive the message being received.

* BasicProperties
* delivery_mode - with_delivery_mode (generated.rs line 1396)

* docker run --name rabbitmq -p 5672:5672 -p 15672:15672 --restart=always rabbitmq:3-management
* cargo build
* terminal1: cargo run --bin consumer
* terminal2: cargo run --bin consumer
* terminal3: cut and paste the command list below

```text
cargo run --bin producer .........
cargo run --bin producer .........
cargo run --bin producer .........
cargo run --bin producer ..
cargo run --bin producer ..
cargo run --bin producer ..
cargo run --bin producer ..
cargo run --bin producer ..
cargo run --bin producer end...
cargo run --bin producer end..
```

You should see the texts get distributed among the two terminals 1 and 2.
