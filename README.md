# browser-reporting-api-observability-ingester

Status work in progress 🚧

Service that listens for browser reporting-api reports (eg
content-security-policy voilation reports, crash reports, deprecation reports)
and ingests them into observability systems.

Endpoint to ingest content-security-policy violation reports, crash reports and
deprecation reports into observability systems by producing metric counters and
structured JSON logged to std-out.

There is a ready to use docker image. However to best monitor your website, we
encourage you to fork and customize the metric labels and JSON output with
details relevant to monitoring your site.

## Project structure

### Site with violations

```
projects/site-with-violations/
```

A collection of pages that will trigger reports in order to explore how browsers
send reports in practice and to verify the working of this project.

## Configuration

For configuration specific to your website fork the project.

Default configuration

```
METRICS_LABELS=ua_is_app,ua_is_mobile,url_origin
LOG_LABELS=url,ip_addr,ua
IP_ADDR_FROM=socket
INGESTOR_URL_ORIGIN=http
```

`IP_ADDR_FROM` can be used to determine how the client IP address is derived.
`socket` takes directly from the connection- use this if clients connect
directly to the ingestor. `x-forwarded-for` uses the
[x-forwarded-for header](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/X-Forwarded-For).
`forwarded` uses the
[forwarded header](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Forwarded).

`INGESTOR_URL_ORIGIN` can be used to determine the origin of the URL the client
used to connect to the ingestor. It is useful to set this up if you're running
the same instance of the ingestor on multiple domains and wish to tell them
apart. `http` uses the
[host header](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Host)
and assumes `http` as the scheme (`http` is the only scheme directly supported
by this project). `x-forwarded` uses the headers
[x-forwarded-proto](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/X-Forwarded-Proto)
and
[x-forwarded-host](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/X-Forwarded-Host).
`forwarded` uses the
[forwarded header](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Forwarded).

⚠️ When using `x-forwarded-for`, `x-forwarded` (`host` and `proto`) or
`forwarded` options the first header value is used. It is your responsibility to
configure your outermost proxy to remove any pre-existing instances of these
headers to prevent rogue users spoofing values. Failure to do so may result in
the incorrect value being recorded or excess cardinality in the metric counters.
