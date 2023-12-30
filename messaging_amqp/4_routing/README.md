# 4 Routing

Show the fanout exchange.

[Routing](https://www.rabbitmq.com/tutorials/tutorial-four-python.html)

* BasicProperties
* delivery_mode - with_delivery_mode (generated.rs line 1396)

* docker run --name rabbitmq -p 5672:5672 -p 15672:15672 --restart=always rabbitmq:3-management
* cargo build
* terminal1: cargo run --bin producer
  * this needs to be run first to create the exchange
* terminal2: cargo run --bin consumer error
* terminal3: cargo run --bin consumer info warning error
* terminal1: cargo run --bin producer error "first error"
* terminal1: cargo run --bin producer info "info is good"
* terminal1: cargo run --bin producer warning "warnings should probably be looked at"
* terminal1: cargo run --bin producer error "last error"

* If the producer does not create the queue prior to to publiching, then all messages published to the exchange will be lost.
* TODO how is the queue automatically bound to the exchange?

work with tokio: examples/tokio.rs
