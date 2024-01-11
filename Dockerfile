FROM ubuntu:latest
RUN useradd -ms /bin/bash www
USER www
WORKDIR /home/www/blog
COPY ./target/release/htmx_tests /home/www/blog/serve
COPY ./assets /home/www/blog/assets
CMD ["./serve"]

