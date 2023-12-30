# RabbitMQ hello world

* docker run --name rabbitmq -p 5672:5672 -p 15672:15672 --restart=always rabbitmq:3-management
* cargo build
* terminal1: cargo run --bin consumer
* terminal1: cargo run --bin producer
