# Owasu

### What is this ? 

#### My personal project to integrating from alertmanager and kumato seatalk

#### There are some metrics (prefix --> /api/v1)
- /alert
- /kuma

### Response Body

#### Success
```yaml
{
    "apiVersion": "v1",
    "success": true
}
```

### Example owasu config

```yaml
seatalk:
  chat_id: ""
server:
  port: 8080
  buffer: 10
  concurrency_limit: 100
  rate_limit: 100
  limiter_timeout: 10s
  timeout: 10s
```


### How to run

#### Manual
```shell
$ owasu --config="<PATH_TO_OWASU_CONFIG>"
```

#### Using prebuilt docker image

```shell
$ docker run -it -p8080:8080 -v "<PATH_TO_OWASU_CONFIG>":/app/config/owasu.yaml -v -d lordchou/owasu:v0.1.0 \
  ./owasu --config="<PATH_TO_OWASU_CONFIG>
```