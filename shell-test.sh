curl http://localhost:8080/tx -XPOST -H 'Content-Type: application/json' -d '{"id":1, "ip":"127.0.0.1"}'

#wrk
wrk -t10 -c10 -s post.lua http://127.0.0.1:8080/tx

#apache benchmark
ab -n 10 -c 10 -p post-data.json -T 'application/json' http://127.0.0.1:8080/tx

#k6
k6 run script.js