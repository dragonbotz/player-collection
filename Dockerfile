# Player collection dockerfile
#
# Authors: Lahcène Belhadi <lahcene.belhadi@gmail.com>
FROM ubuntu:22.04

WORKDIR app
COPY target/release/player-collection .

CMD ["./player-collection"]

