# 3 Publish/Subscribe

Show the fanout exchange.

[Publish/Subscribe](https://www.rabbitmq.com/tutorials/tutorial-three-python.html)

* BasicProperties
* delivery_mode - with_delivery_mode (generated.rs line 1396)

* docker run --name rabbitmq -p 5672:5672 -p 15672:15672 --restart=always rabbitmq:3-management
* cargo build
* terminal1: cargo run --bin producer
  * this needs to be run first to create the exchange
* terminal2: cargo run --bin consumer
* terminal3: cargo run --bin consumer
* terminal1: cargo run --bin producer warn "hi there"

* If the producer does not create the queue prior to to publiching, then all messages published to the exchange will be lost.
* TODO how is the queue automatically bound to the exchange?

work with tokio: examples/tokio.rs
