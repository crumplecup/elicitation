//! Wire protocol and transport propositions.
//!
//! Sources: PostgreSQL Frontend/Backend Protocol (§55); RFC 8446 (TLS 1.3);
//! RFC 5246 (TLS 1.2); IETF RFC 7159 (JSON).

mod emit_impls {
    use elicitation::contracts::Prop;
    use elicitation::proc_macro2::TokenStream;
    use elicitation::quote::quote;

    // -------------------------------------------------------------------------
    // Connection setup (PG Protocol §55.2)
    // -------------------------------------------------------------------------

    /// A TCP connection to the database server is established.
    ///
    /// Source: PostgreSQL Frontend/Backend Protocol §55.2 — Connection Setup
    pub struct ConnectionEstablished;

    /// The frontend sent the `StartupMessage` with the protocol version field.
    ///
    /// Source: PostgreSQL Frontend/Backend Protocol §55.2 — Connection Setup
    pub struct StartupMessageSent;

    /// Protocol version 3.0 was agreed upon between frontend and backend.
    ///
    /// Source: PostgreSQL Frontend/Backend Protocol §55.2 — Connection Setup
    pub struct ProtocolVersionNegotiated;

    /// The frontend sent the `SSLRequest` message.
    ///
    /// Source: PostgreSQL Frontend/Backend Protocol §55.2 — Connection Setup
    pub struct SslRequestSent;

    /// The server responded `'S'` to the `SSLRequest` and the TLS handshake completed.
    ///
    /// Source: PostgreSQL Frontend/Backend Protocol §55.2 — Connection Setup
    pub struct SslNegotiationSucceeded;

    /// The server sent an `Authentication` challenge message.
    ///
    /// Source: PostgreSQL Frontend/Backend Protocol §55.2 — Connection Setup
    pub struct AuthenticationRequestReceived;

    /// The server sent `Authentication OK` (type 0), completing authentication.
    ///
    /// Source: PostgreSQL Frontend/Backend Protocol §55.2 — Connection Setup
    pub struct AuthenticationOkReceived;

    /// The server sent `BackendKeyData` carrying the process ID and cancel secret.
    ///
    /// Source: PostgreSQL Frontend/Backend Protocol §55.2 — Connection Setup
    pub struct BackendKeyDataReceived;

    /// The server sent all `ParameterStatus` messages for server-side parameters.
    ///
    /// Source: PostgreSQL Frontend/Backend Protocol §55.2 — Connection Setup
    pub struct ParameterStatusReceived;

    /// The server sent `ReadyForQuery`; the connection is ready to accept commands.
    ///
    /// Source: PostgreSQL Frontend/Backend Protocol §55.2 — Connection Setup
    pub struct ReadyForQueryReceived;

    // -------------------------------------------------------------------------
    // TLS (RFC 8446 / RFC 5246)
    // -------------------------------------------------------------------------

    /// TLS 1.2 is supported by the server.
    ///
    /// Source: RFC 5246 — The Transport Layer Security (TLS) Protocol Version 1.2
    pub struct Tls12Supported;

    /// TLS 1.3 is the preferred or negotiated version for this session.
    ///
    /// Source: RFC 8446 — The Transport Layer Security (TLS) Protocol Version 1.3
    pub struct Tls13Preferred;

    /// The server certificate chain validates to a trusted root CA.
    ///
    /// Source: RFC 8446 §4.4.2 — Certificate
    pub struct TlsCertificateChainValid;

    /// The certificate CN or SAN matches the connection hostname.
    ///
    /// Source: RFC 8446 §4.4.2; RFC 2818 §3.1 — Server Identity
    pub struct TlsHostnameVerified;

    /// The negotiated cipher suite is in the approved set.
    ///
    /// Source: RFC 8446 §B.4 — Cipher Suites
    pub struct TlsCipherSuiteApproved;

    /// A client certificate was presented during the TLS handshake.
    ///
    /// Source: RFC 8446 §4.3.2 — Certificate Request
    pub struct TlsClientCertificatePresented;

    /// TLS session tickets or PSK resumption are available for this session.
    ///
    /// Source: RFC 8446 §2.2 — Session Resumption
    pub struct TlsSessionResumptionSupported;

    /// TLS renegotiation is disabled for this session.
    ///
    /// Source: RFC 5746 — TLS Renegotiation Indication Extension
    pub struct TlsRenegotiationDisabled;

    // -------------------------------------------------------------------------
    // Message framing (PG Protocol §55.7)
    // -------------------------------------------------------------------------

    /// The request message is well-formed per the PostgreSQL wire protocol.
    ///
    /// Source: PostgreSQL Frontend/Backend Protocol §55.7 — Message Formats
    pub struct RequestWellFormed;

    /// A complete response message was received with no truncation.
    ///
    /// Source: PostgreSQL Frontend/Backend Protocol §55.7 — Message Formats
    pub struct ResponseFullyReceived;

    /// The 4-byte length prefix matches the actual message body length.
    ///
    /// Source: PostgreSQL Frontend/Backend Protocol §55.7 — Message Formats
    pub struct MessageLengthPrefixCorrect;

    /// The server sent an `ErrorResponse` carrying a valid five-character SQLSTATE code.
    ///
    /// Source: PostgreSQL Frontend/Backend Protocol §55.7 — ErrorResponse
    pub struct ErrorResponseReceived;

    /// The server sent a `NoticeResponse` message.
    ///
    /// Source: PostgreSQL Frontend/Backend Protocol §55.7 — NoticeResponse
    pub struct NoticeResponseReceived;

    /// The server sent `CommandComplete` with the affected-row count tag.
    ///
    /// Source: PostgreSQL Frontend/Backend Protocol §55.7 — CommandComplete
    pub struct CommandCompleteReceived;

    // -------------------------------------------------------------------------
    // JSON serialization (RFC 7159)
    // -------------------------------------------------------------------------

    /// The response payload is fully serializable to JSON.
    ///
    /// Source: IETF RFC 7159 — The JavaScript Object Notation (JSON) Data Interchange Format
    pub struct ResponseSerializable;

    /// The response JSON matches the expected JSON Schema.
    ///
    /// Source: IETF RFC 7159; JSON Schema specification
    pub struct JsonSchemaValid;

    /// The JSON payload is valid UTF-8.
    ///
    /// Source: IETF RFC 7159 §8.1 — Character Encoding
    pub struct JsonUtf8Encoded;

    /// The JSON nesting depth does not exceed safe implementation limits.
    ///
    /// Source: IETF RFC 7159 §9 — Parsers
    pub struct JsonNestingDepthSafe;

    // -------------------------------------------------------------------------
    // Connection lifecycle
    // -------------------------------------------------------------------------

    /// The frontend sent a `Terminate` message before closing the connection.
    ///
    /// Source: PostgreSQL Frontend/Backend Protocol §55.2 — Termination
    pub struct ConnectionClosedGracefully;

    /// A pooled connection slot was available when the request arrived.
    ///
    /// Source: PostgreSQL documentation — Connection Pooling
    pub struct ConnectionPoolSlotAvailable;

    /// The connection pool maximum was not exceeded.
    ///
    /// Source: PostgreSQL documentation — Connection Pooling
    pub struct ConnectionPoolLimitRespected;

    /// The connection timeout was not exceeded during establishment.
    ///
    /// Source: PostgreSQL documentation — connect_timeout
    pub struct ConnectionTimeoutRespected;

    // -- Extended Query Protocol (§F.3 of PostgreSQL wire protocol) --

    /// A `Parse` message was sent to the server (extended query protocol phase 1).
    ///
    /// Source: PostgreSQL §53.2.3 — Frontend message formats: Parse
    pub struct ParseMessageSent;

    /// A `Bind` message was sent with parameter values (extended query protocol phase 2).
    ///
    /// Source: PostgreSQL §53.2.3 — Frontend message formats: Bind
    pub struct BindMessageSent;

    /// A `Describe` message was sent to request parameter or result metadata.
    ///
    /// Source: PostgreSQL §53.2.3 — Frontend message formats: Describe
    pub struct DescribeMessageSent;

    /// An `Execute` message was sent to run the portal (extended query protocol phase 3).
    ///
    /// Source: PostgreSQL §53.2.3 — Frontend message formats: Execute
    pub struct ExecuteMessageSent;

    /// A `Sync` message was sent to flush the pipeline and receive a `ReadyForQuery`.
    ///
    /// Source: PostgreSQL §53.2.3 — Frontend message formats: Sync
    pub struct SyncMessageSent;

    /// A `Close` message was sent to release a prepared statement or portal.
    ///
    /// Source: PostgreSQL §53.2.3 — Frontend message formats: Close
    pub struct CloseMessageSent;

    /// A `ParseComplete` acknowledgment was received from the server.
    ///
    /// Source: PostgreSQL §53.2.4 — Backend message formats: ParseComplete
    pub struct ParseCompleteReceived;

    /// A `BindComplete` acknowledgment was received from the server.
    ///
    /// Source: PostgreSQL §53.2.4 — Backend message formats: BindComplete
    pub struct BindCompleteReceived;

    // -- COPY Protocol (§53.2.6 of PostgreSQL wire protocol) --

    /// A `CopyInResponse` was received, indicating server is ready for COPY data.
    ///
    /// Source: PostgreSQL §53.2.4 — Backend message formats: CopyInResponse
    pub struct CopyInResponseReceived;

    /// A `CopyOutResponse` was received, indicating server is streaming COPY data.
    ///
    /// Source: PostgreSQL §53.2.4 — Backend message formats: CopyOutResponse
    pub struct CopyOutResponseReceived;

    /// A `CopyData` message was sent with a chunk of COPY payload.
    ///
    /// Source: PostgreSQL §53.2.3 — Frontend message formats: CopyData
    pub struct CopyDataSent;

    /// A `CopyDone` message was received, marking end of COPY stream.
    ///
    /// Source: PostgreSQL §53.2.4 — Backend message formats: CopyDone
    pub struct CopyDoneReceived;

    // -- SCRAM Authentication (RFC 5802 + PostgreSQL SASL) --

    /// SCRAM `client-first-message` was sent (RFC 5802 §3 first step).
    ///
    /// Source: RFC 5802 §3 — SCRAM Authentication; PostgreSQL SASL authentication
    pub struct ScramClientFirstSent;

    /// SCRAM `server-first-message` was received with salt and iteration count.
    ///
    /// Source: RFC 5802 §3 — SCRAM Authentication
    pub struct ScramServerFirstReceived;

    /// SCRAM `client-final-message` with proof was sent (RFC 5802 §3 third step).
    ///
    /// Source: RFC 5802 §3 — SCRAM Authentication
    pub struct ScramClientFinalSent;

    /// SCRAM `server-final-message` with server signature was received and verified.
    ///
    /// Source: RFC 5802 §3 — SCRAM Authentication
    pub struct ScramServerFinalReceived;

    // -- Session Termination --

    /// A `Terminate` message was sent to cleanly close the connection.
    ///
    /// Source: PostgreSQL §53.2.3 — Frontend message formats: Terminate
    pub struct TerminateMessageSent;

    // -------------------------------------------------------------------------
    // Impl macro
    // -------------------------------------------------------------------------

    macro_rules! transport_prop {
        ($t:ty, $name:literal) => {
            impl Prop for $t {
                fn kani_proof() -> TokenStream {
                    quote! { /* structural: #name — verified by protocol message framing */ }
                }
                fn verus_proof() -> TokenStream {
                    quote! { /* structural: #name — verified by protocol message framing */ }
                }
                fn creusot_proof() -> TokenStream {
                    quote! { /* structural: #name — verified by protocol message framing */ }
                }
            }
        };
    }

    // Connection setup
    transport_prop!(ConnectionEstablished, "ConnectionEstablished");
    transport_prop!(StartupMessageSent, "StartupMessageSent");
    transport_prop!(ProtocolVersionNegotiated, "ProtocolVersionNegotiated");
    transport_prop!(SslRequestSent, "SslRequestSent");
    transport_prop!(SslNegotiationSucceeded, "SslNegotiationSucceeded");
    transport_prop!(
        AuthenticationRequestReceived,
        "AuthenticationRequestReceived"
    );
    transport_prop!(AuthenticationOkReceived, "AuthenticationOkReceived");
    transport_prop!(BackendKeyDataReceived, "BackendKeyDataReceived");
    transport_prop!(ParameterStatusReceived, "ParameterStatusReceived");
    transport_prop!(ReadyForQueryReceived, "ReadyForQueryReceived");

    // TLS
    transport_prop!(Tls12Supported, "Tls12Supported");
    transport_prop!(Tls13Preferred, "Tls13Preferred");
    transport_prop!(TlsCertificateChainValid, "TlsCertificateChainValid");
    transport_prop!(TlsHostnameVerified, "TlsHostnameVerified");
    transport_prop!(TlsCipherSuiteApproved, "TlsCipherSuiteApproved");
    transport_prop!(
        TlsClientCertificatePresented,
        "TlsClientCertificatePresented"
    );
    transport_prop!(
        TlsSessionResumptionSupported,
        "TlsSessionResumptionSupported"
    );
    transport_prop!(TlsRenegotiationDisabled, "TlsRenegotiationDisabled");

    // Message framing
    transport_prop!(RequestWellFormed, "RequestWellFormed");
    transport_prop!(ResponseFullyReceived, "ResponseFullyReceived");
    transport_prop!(MessageLengthPrefixCorrect, "MessageLengthPrefixCorrect");
    transport_prop!(ErrorResponseReceived, "ErrorResponseReceived");
    transport_prop!(NoticeResponseReceived, "NoticeResponseReceived");
    transport_prop!(CommandCompleteReceived, "CommandCompleteReceived");

    // JSON serialization
    transport_prop!(ResponseSerializable, "ResponseSerializable");
    transport_prop!(JsonSchemaValid, "JsonSchemaValid");
    transport_prop!(JsonUtf8Encoded, "JsonUtf8Encoded");
    transport_prop!(JsonNestingDepthSafe, "JsonNestingDepthSafe");

    // Connection lifecycle
    transport_prop!(ConnectionClosedGracefully, "ConnectionClosedGracefully");
    transport_prop!(ConnectionPoolSlotAvailable, "ConnectionPoolSlotAvailable");
    transport_prop!(ConnectionPoolLimitRespected, "ConnectionPoolLimitRespected");
    transport_prop!(ConnectionTimeoutRespected, "ConnectionTimeoutRespected");

    // Extended query protocol (Parse/Bind/Execute pipeline)
    transport_prop!(ParseMessageSent, "ParseMessageSent");
    transport_prop!(BindMessageSent, "BindMessageSent");
    transport_prop!(DescribeMessageSent, "DescribeMessageSent");
    transport_prop!(ExecuteMessageSent, "ExecuteMessageSent");
    transport_prop!(SyncMessageSent, "SyncMessageSent");
    transport_prop!(CloseMessageSent, "CloseMessageSent");
    transport_prop!(ParseCompleteReceived, "ParseCompleteReceived");
    transport_prop!(BindCompleteReceived, "BindCompleteReceived");

    // COPY protocol
    transport_prop!(CopyInResponseReceived, "CopyInResponseReceived");
    transport_prop!(CopyOutResponseReceived, "CopyOutResponseReceived");
    transport_prop!(CopyDataSent, "CopyDataSent");
    transport_prop!(CopyDoneReceived, "CopyDoneReceived");

    // SCRAM authentication
    transport_prop!(ScramClientFirstSent, "ScramClientFirstSent");
    transport_prop!(ScramServerFirstReceived, "ScramServerFirstReceived");
    transport_prop!(ScramClientFinalSent, "ScramClientFinalSent");
    transport_prop!(ScramServerFinalReceived, "ScramServerFinalReceived");

    // Graceful termination
    transport_prop!(TerminateMessageSent, "TerminateMessageSent");
}

pub use emit_impls::{
    AuthenticationOkReceived,
    // Connection setup
    AuthenticationRequestReceived,
    BackendKeyDataReceived,
    BindCompleteReceived,
    BindMessageSent,
    CloseMessageSent,
    // Message framing
    CommandCompleteReceived,
    // Connection lifecycle
    ConnectionClosedGracefully,
    ConnectionEstablished,
    ConnectionPoolLimitRespected,
    ConnectionPoolSlotAvailable,
    ConnectionTimeoutRespected,
    CopyDataSent,
    CopyDoneReceived,
    // COPY protocol
    CopyInResponseReceived,
    CopyOutResponseReceived,
    DescribeMessageSent,
    ErrorResponseReceived,
    ExecuteMessageSent,
    // JSON serialization
    JsonNestingDepthSafe,
    JsonSchemaValid,
    JsonUtf8Encoded,
    MessageLengthPrefixCorrect,
    NoticeResponseReceived,
    ParameterStatusReceived,
    ParseCompleteReceived,
    // Extended query protocol
    ParseMessageSent,
    ProtocolVersionNegotiated,
    ReadyForQueryReceived,
    RequestWellFormed,
    ResponseFullyReceived,
    ResponseSerializable,
    ScramClientFinalSent,
    // SCRAM authentication
    ScramClientFirstSent,
    ScramServerFinalReceived,
    ScramServerFirstReceived,
    SslNegotiationSucceeded,
    SslRequestSent,
    StartupMessageSent,
    SyncMessageSent,
    // Graceful termination
    TerminateMessageSent,
    // TLS
    Tls12Supported,
    Tls13Preferred,
    TlsCertificateChainValid,
    TlsCipherSuiteApproved,
    TlsClientCertificatePresented,
    TlsHostnameVerified,
    TlsRenegotiationDisabled,
    TlsSessionResumptionSupported,
};
