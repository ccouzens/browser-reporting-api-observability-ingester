use serde::{Deserialize, Serialize};

/**
The type of a network error (or `"ok"` if the request succeeded).

- <https://w3c.github.io/network-error-logging/#predefined-network-error-types>
- <https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/Network_Error_Logging#error_reports>
- <https://chromium.googlesource.com/chromium/src/+/989bfb21678bad41c86612a54df922726246ecd7/net/network_error_logging/network_error_logging_service.cc>
*/
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum NetworkErrorType {
    /**
    The request was aborted before completion (`"abandoned"`).
    */
    #[serde(rename = "abandoned")]
    Abandoned,
    /**
    The server IP address changed between policy receipt and error generation
    (`"dns.address_changed"`).
    */
    #[serde(rename = "dns.address_changed")]
    DnsAddressChanged,
    /**
    Request to the DNS server failed (`"dns.failed"`).
    */
    #[serde(rename = "dns.failed")]
    DnsFailed,
    /**
    The user's DNS server was unable to resolve an IP address
    (`"dns.name_not_resolved"`).
    */
    #[serde(rename = "dns.name_not_resolved")]
    DnsNameNotResolved,
    /**
    Malformed DNS response (`"dns.protocol"`).
    */
    #[serde(rename = "dns.protocol")]
    DnsProtocol,
    /**
    DNS server returned an error such as SERVFAIL or REFUSED (`"dns.server"`).
    */
    #[serde(rename = "dns.server")]
    DnsServer,
    /**
    DNS resolution timed out (`"dns.timed_out"`).
    */
    #[serde(rename = "dns.timed_out")]
    DnsTimedOut,
    /**
    The user's DNS server is unreachable (`"dns.unreachable"`).
    */
    #[serde(rename = "dns.unreachable")]
    DnsUnreachable,
    /**
    HTTP/2 ping failed (`"h2.ping_failed"`).
    */
    #[serde(rename = "h2.ping_failed")]
    H2PingFailed,
    /**
    HTTP/2 protocol error (`"h2.protocol.error"`).
    */
    #[serde(rename = "h2.protocol.error")]
    H2ProtocolError,
    /**
    HTTP/3 protocol error (`"h3.protocol.error"`).
    */
    #[serde(rename = "h3.protocol.error")]
    H3ProtocolError,
    /**
    The user agent received a 4xx or 5xx HTTP response (`"http.error"`).
    */
    #[serde(rename = "http.error")]
    HttpError,
    /**
    The connection failed due to an HTTP protocol error not covered by other
    types (`"http.failed"`).
    */
    #[serde(rename = "http.failed")]
    HttpFailed,
    /**
    The connection was aborted due to an HTTP protocol error
    (`"http.protocol.error"`).
    */
    #[serde(rename = "http.protocol.error")]
    HttpProtocolError,
    /**
    The requested range was not satisfiable (`"http.request.range_not_satisfiable"`).
    */
    #[serde(rename = "http.request.range_not_satisfiable")]
    HttpRequestRangeNotSatisfiable,
    /**
    Response headers contained multiple `Content-Disposition` headers
    (`"http.response.headers.multiple_content_disposition"`).
    */
    #[serde(rename = "http.response.headers.multiple_content_disposition")]
    HttpResponseHeadersMultipleContentDisposition,
    /**
    Response headers contained multiple `Content-Length` headers
    (`"http.response.headers.multiple_content_length"`).
    */
    #[serde(rename = "http.response.headers.multiple_content_length")]
    HttpResponseHeadersMultipleContentLength,
    /**
    Response headers were truncated (`"http.response.headers.truncated"`).
    */
    #[serde(rename = "http.response.headers.truncated")]
    HttpResponseHeadersTruncated,
    /**
    Response could not be processed (`"http.response.invalid"`).
    */
    #[serde(rename = "http.response.invalid")]
    HttpResponseInvalid,
    /**
    Response content decoding failed (`"http.response.invalid.content_encoding"`).
    */
    #[serde(rename = "http.response.invalid.content_encoding")]
    HttpResponseInvalidContentEncoding,
    /**
    Response `Content-Length` did not match the body length
    (`"http.response.invalid.content_length_mismatch"`).
    */
    #[serde(rename = "http.response.invalid.content_length_mismatch")]
    HttpResponseInvalidContentLengthMismatch,
    /**
    Response was empty (`"http.response.invalid.empty"`).
    */
    #[serde(rename = "http.response.invalid.empty")]
    HttpResponseInvalidEmpty,
    /**
    Response had incomplete chunked encoding
    (`"http.response.invalid.incomplete_chunked_encoding"`).
    */
    #[serde(rename = "http.response.invalid.incomplete_chunked_encoding")]
    HttpResponseInvalidIncompleteChunkedEncoding,
    /**
    Response had invalid chunked encoding
    (`"http.response.invalid.invalid_chunked_encoding"`).
    */
    #[serde(rename = "http.response.invalid.invalid_chunked_encoding")]
    HttpResponseInvalidInvalidChunkedEncoding,
    /**
    The request was aborted due to a redirect loop
    (`"http.response.redirect_loop"`).
    */
    #[serde(rename = "http.response.redirect_loop")]
    HttpResponseRedirectLoop,
    /**
    The request succeeded (`"ok"`).
    */
    #[serde(rename = "ok")]
    Ok,
    /**
    The TCP connection was aborted (`"tcp.aborted"`).
    */
    #[serde(rename = "tcp.aborted")]
    TcpAborted,
    /**
    The IP address is invalid (`"tcp.address_invalid"`).
    */
    #[serde(rename = "tcp.address_invalid")]
    TcpAddressInvalid,
    /**
    The IP address is unreachable (`"tcp.address_unreachable"`).
    */
    #[serde(rename = "tcp.address_unreachable")]
    TcpAddressUnreachable,
    /**
    The TCP connection was closed by the server (`"tcp.closed"`).
    */
    #[serde(rename = "tcp.closed")]
    TcpClosed,
    /**
    The TCP connection failed (`"tcp.failed"`).
    */
    #[serde(rename = "tcp.failed")]
    TcpFailed,
    /**
    The TCP connection was refused by the server (`"tcp.refused"`).
    */
    #[serde(rename = "tcp.refused")]
    TcpRefused,
    /**
    The TCP connection was reset (`"tcp.reset"`).
    */
    #[serde(rename = "tcp.reset")]
    TcpReset,
    /**
    The TCP connection timed out (`"tcp.timed_out"`).
    */
    #[serde(rename = "tcp.timed_out")]
    TcpTimedOut,
    /**
    Bad TLS client authentication certificate (`"tls.bad_client_auth_cert"`).
    */
    #[serde(rename = "tls.bad_client_auth_cert")]
    TlsBadClientAuthCert,
    /**
    TLS certificate authority is invalid (`"tls.cert.authority_invalid"`).
    */
    #[serde(rename = "tls.cert.authority_invalid")]
    TlsCertAuthorityInvalid,
    /**
    TLS certificate date is invalid (`"tls.cert.date_invalid"`).
    */
    #[serde(rename = "tls.cert.date_invalid")]
    TlsCertDateInvalid,
    /**
    TLS certificate is invalid (`"tls.cert.invalid"`).
    */
    #[serde(rename = "tls.cert.invalid")]
    TlsCertInvalid,
    /**
    TLS certificate common name is invalid (`"tls.cert.name_invalid"`).
    */
    #[serde(rename = "tls.cert.name_invalid")]
    TlsCertNameInvalid,
    /**
    TLS certificate pinned key is not in the cert chain
    (`"tls.cert.pinned_key_not_in_cert_chain"`).
    */
    #[serde(rename = "tls.cert.pinned_key_not_in_cert_chain")]
    TlsCertPinnedKeyNotInCertChain,
    /**
    TLS certificate has been revoked (`"tls.cert.revoked"`).
    */
    #[serde(rename = "tls.cert.revoked")]
    TlsCertRevoked,
    /**
    TLS connection failed (`"tls.failed"`).
    */
    #[serde(rename = "tls.failed")]
    TlsFailed,
    /**
    TLS protocol error (`"tls.protocol.error"`).
    */
    #[serde(rename = "tls.protocol.error")]
    TlsProtocolError,
    /**
    TLS unrecognized name alert (`"tls.unrecognized_name_alert"`).
    */
    #[serde(rename = "tls.unrecognized_name_alert")]
    TlsUnrecognizedNameAlert,
    /**
    TLS version or cipher mismatch (`"tls.version_or_cipher_mismatch"`).
    */
    #[serde(rename = "tls.version_or_cipher_mismatch")]
    TlsVersionOrCipherMismatch,
    /**
    Unknown network error (`"unknown"`).
    */
    #[serde(rename = "unknown")]
    Unknown,
    /**
    Any other network error type string not explicitly enumerated above.
    */
    #[serde(untagged)]
    Other(String),
}
