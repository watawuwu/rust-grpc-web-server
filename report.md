```txt:stepancheg/grpc-rust(v0.6.1)
 ❯❯ ghz \
    --connections 4 \
    --concurrency 100 \
    --proto proto/helloworld.proto \
    --insecure \
    --call helloworld.Greeter.SayHello \
    --data '{"name":"watawuwu"}' \
    --total 100000 \
    localhost:50051

Summary:
  Count:        100000
  Total:        5.41 s
  Slowest:      15.42 ms
  Average:      5.35 ms
  Requests/sec: 18475.74

Response time histogram:
  0.434 [1]     |
  1.932 [80]    |
  3.430 [5673]  |∎∎∎∎∎
  4.928 [31716] |∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
  6.427 [42737] |∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
  7.925 [16876] |∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
  9.423 [2016]  |∎∎
  10.921 [645]  |∎
  12.419 [180]  |
  13.917 [58]   |
  15.416 [18]   |

Latency distribution:
  10%% in 3.77 ms
  25%% in 4.52 ms
  50%% in 5.18 ms
  75%% in 6.05 ms
  90%% in 7.14 ms
  95%% in 7.54 ms
  99%% in 9.30 ms

Status code distribution:
  [OK]   100000 responses
```

```txt:stepancheg/grpc-rust(latest)
 ❯❯ ghz \
    --connections 4 \
    --concurrency 100 \
    --proto proto/helloworld.proto \
    --insecure \
    --call helloworld.Greeter.SayHello \
    --data '{"name":"watawuwu"}' \
    --total 100000 \
    localhost:50051

Summary:
  Count:        100000
  Total:        2.20 s
  Slowest:      17.07 ms
  Fastest:      0.14 ms
  Average:      2.15 ms
  Requests/sec: 45352.70

Response time histogram:
  0.144 [1]     |
  1.836 [30698] |∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
  3.528 [66974] |∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
  5.220 [2118]  |∎
  6.913 [102]   |
  8.605 [55]    |
  10.297 [13]   |
  11.989 [21]   |
  13.681 [9]    |
  15.373 [7]    |
  17.066 [2]    |

Latency distribution:
  10%% in 1.54 ms
  25%% in 1.77 ms
  50%% in 2.05 ms
  75%% in 2.48 ms
  90%% in 2.85 ms
  95%% in 3.17 ms
  99%% in 3.93 ms

Status code distribution:
  [OK]   100000 responses
```

```txt:tikv/grpc-rs(v0.4.6)
 ❯❯ ghz \
    --connections 4 \
    --concurrency 40 \
    --proto proto/helloworld.proto \
    --insecure \
    --call helloworld.Greeter.SayHello \
    --data '{"name":"watawuwu"}' \
    --total 100000 \
    localhost:50051

Summary:
  Count:        100000
  Total:        3.23 s
  Slowest:      5.91 ms
  Fastest:      0.10 ms
  Average:      1.25 ms
  Requests/sec: 30950.58

Response time histogram:
  0.102 [1]     |
  0.683 [1324]  |∎
  1.263 [59906] |∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
  1.844 [33348] |∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
  2.425 [4805]  |∎∎∎
  3.005 [576]   |
  3.586 [25]    |
  4.167 [5]     |
  4.747 [3]     |
  5.328 [4]     |
  5.908 [3]     |

Latency distribution:
  10%% in 0.97 ms
  25%% in 1.07 ms
  50%% in 1.20 ms
  75%% in 1.35 ms
  90%% in 1.60 ms
  95%% in 1.87 ms
  99%% in 2.32 ms

Status code distribution:
  [OK]   100000 responses
```

- tikv/grpc-rs は concurrency=50 以上だと以下のエラーが発生する
```tikv/grpc-rs
E1130 16:39:16.347007000 123145531621376 chttp2_transport.cc:813]      Memory exhausted, rejecting the stream.
```

```txt:hyperium/tonic(v0.1.0-alpha.6)
 ❯❯ ghz \
    --connections 4 \
    --concurrency 100 \
    --proto proto/helloworld.proto \
    --insecure \
    --call helloworld.Greeter.SayHello \
    --data '{"name":"watawuwu"}' \
    --total 100000 \
    localhost:50051

Summary:
  Count:        100000
  Total:        1.93 s
  Slowest:      11.68 ms
  Fastest:      0.17 ms
  Average:      1.81 ms
  Requests/sec: 51898.82

Response time histogram:
  0.165 [1]     |
  1.317 [24434] |∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
  2.468 [60321] |∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
  3.619 [13187] |∎∎∎∎∎∎∎∎∎
  4.770 [1582]  |∎
  5.921 [324]   |
  7.073 [112]   |
  8.224 [31]    |
  9.375 [1]     |
  10.526 [0]    |
  11.678 [7]    |

Latency distribution:
  10%% in 1.05 ms
  25%% in 1.33 ms
  50%% in 1.68 ms
  75%% in 2.15 ms
  90%% in 2.72 ms
  95%% in 3.12 ms
  99%% in 4.12 ms

Status code distribution:
  [OK]   100000 responses
```

