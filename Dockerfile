FROM ubuntu:20.04

RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends tzdata
RUN apt-get update && \
apt-get install -y --no-install-recommends \
clang \
curl \
make \
pkg-config \
software-properties-common \
unzip \
wget \
sudo

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN echo "deb http://security.ubuntu.com/ubuntu xenial-security main" >> /etc/apt/sources.list
RUN apt-get install --only-upgrade libstdc++6 -y && add-apt-repository ppa:ubuntu-toolchain-r/test
RUN apt-get update && apt-get install -y --no-install-recommends \
libssl-dev \
libssl1.0.0 \
zlib1g-dev \
build-essential \
gpg-agent

RUN chown -R root:root /home
WORKDIR /home/
COPY . /home/
RUN /root/.cargo/bin/cargo build

RUN echo "deb http://apt.postgresql.org/pub/repos/apt $(lsb_release -cs)-pgdg main" > /etc/apt/sources.list.d/pgdg.list
RUN wget --quiet -O - https://www.postgresql.org/media/keys/ACCC4CF8.asc | apt-key add -
RUN apt-get update && apt-get install -y postgresql-client postgresql-14 \
&& rm -rf /var/lib/apt/lists/*


