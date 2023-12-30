# 5 Topics

Show the fanout exchange.

[Topics](https://www.rabbitmq.com/tutorials/tutorial-five-python.html)

* BasicProperties
* delivery_mode - with_delivery_mode (generated.rs line 1396)

* docker run --name rabbitmq -p 5672:5672 -p 15672:15672 --restart=always rabbitmq:3-management
* cargo build
* terminal1: cargo run --bin producer
  * this needs to be run first to create the exchange
* terminal2: cargo run --bin consumer "#"
* terminal3: cargo run --bin consumer "kern.*"
* terminal4: cargo run --bin consumer "*.critical"
* terminal5: cargo run --bin consumer "kern.*" "*.critical"
* terminal1: cargo run --bin producer "kern.critical" "A critical kernel error"
* terminal1: cargo run --bin producer cron.warn "Warning from cron"
* terminal1: cargo run --bin producer kern.info "kernel information"
* terminal1: cargo run --bin producer cron.critical "A critical message from cron"

* If the producer does not create the queue prior to to publiching, then all messages published to the exchange will be lost.
* TODO how is the queue automatically bound to the exchange?

work with tokio: examples/tokio.rs
