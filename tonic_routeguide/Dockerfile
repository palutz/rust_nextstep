FROM rust:1.70-bullseye as builder

ARG APP_NAME=blend_api

RUN USER=root

WORKDIR /app
ADD . ./
RUN make clean && make release


FROM debian:bullseye
ARG APP=/usr/src/app


ENV APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER  \
    && mkdir -p ${APP}

# Copy the compiled binaries into the new container.
COPY --from=builder /app/target/release/${APP_NAME} ${APP}/${APP_NAME}

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./blend_api"]