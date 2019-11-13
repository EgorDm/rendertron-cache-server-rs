# Rendertron Cache Server
Cache server for between the rendertron cache server and the client. 
Caches rendertron urls using file cache. No documents are preserved in memory.

## Setup (Docker)
* (Optional) Update env.docker variables to match your preferred configuration. (Like cache path)
* Update the docker compose file. Comment / uncomment startup of the rendertron server
* Run `docker compose up`

## Usage
Route should be formatted as follows: `/render/your url`
* GET - Retrieves and caches the page (if cached the cache will be used)
* PUT - Invalidates cache for given url and retrieves it's content
* DELETE - Invalidates cache for given url and all it's sub paths

## Benchmarks Results
 ```
Running 30s test @ http://172.18.16.200:5000/render/http://www.google.com/
  12 threads and 400 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    33.19ms   15.87ms 229.50ms   65.32%
    Req/Sec     0.99k    84.90     3.73k    78.97%
  356787 requests in 30.06s, 30.96GB read
Requests/sec:  11867.67
Transfer/sec:      1.03GB
```