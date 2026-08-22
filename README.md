# browser-reporting-api-observability-ingester

Status work in progress 🚧

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
METRICS_LABELS=ua_is_app,ua_is_mobile,document_url_origin
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

`METRICS_LABELS` and `LOG_LABELS` are both comma separated lists of values to
include. They both use the same syntax, and have the same available options.
However some values won't be sensible to use in `METRICS_LABELS` as they will
result in exess cardinality. `METRICS_LABELS` will be the list of labels to use
in the prometheus counters. `LOG_LABELS` will be the list of values to include
in the structured JSON logs.

### Labels values

#### URL

Options here are a matrix of urls, and url properties. Several different URLs
are available

1. `reporting_url`: the url of the endpoint that this service is hosted on, and
   the client connects from.
2. `document_url`: the url of the document that the report was generated from.
3. `source_file_url`: the url of the file that caused the infringement.

The following
[properties of the URL object](https://developer.mozilla.org/en-US/docs/Web/API/URL)
are supported: `hostname`, `href`, `origin`, `pathname` and `search`.

For example:

`reporting_url_origin`: the protocol and host and port (if non standard) used by
the client to connect to the reporting endpoint

`document_url_href`: the whole of the url of the document content that the
browser was on when the report was generated.
