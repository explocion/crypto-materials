FROM rust:latest

RUN apt-get update
RUN apt-get upgrade -y
RUN apt-get install -y gcc libgmp3-dev llvm make git curl neovim zsh

RUN useradd -ms /bin/zsh debian
USER debian

SHELL ["zsh", "-c"]
RUN zsh <(curl -s https://raw.githubusercontent.com/zap-zsh/zap/master/install.zsh)

RUN rustup default stable && rustup component add rust-analyzer
RUN mkdir /home/debian/crypto

WORKDIR /home/debian
CMD ["zsh"]
