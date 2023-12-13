/// Container for `Transaction` construction options.
#[derive(Default, Debug)]
pub struct TransactionStrParams<'a> {
    /// Path to secret key file.
    ///
    /// If `secret_key` is empty, the new transaction will not be signed and will need to be signed (e.g.
    /// via [`sign_transaction_file`](super::sign_transaction_file)) at least once in order to be made valid.
    pub secret_key: &'a str,
    /// RFC3339-like formatted timestamp. e.g. `2018-02-16T00:31:37Z`.
    ///
    /// If `timestamp` is empty, the current time will be used. Note that timestamp is UTC, not
    /// local.
    ///
    /// See [`humantime::parse_rfc3339_weak`] for more information.
    pub timestamp: &'a str,
    /// Time that the `transaction` will remain valid for.
    ///
    /// A `transaction` can only be included in a `Block` between `timestamp` and `timestamp + ttl`.
    /// Input examples: '1hr 12min', '30min 50sec', '1day'.
    ///
    /// See [`humantime::parse_duration`] for more information.
    pub ttl: &'a str,
    /// Name of the chain, to avoid the `transaction` from being accidentally or maliciously included in
    /// a different chain.
    pub chain_name: &'a str,
    /// The hex-encoded public key, account hash, or entity address of the account context under which
    /// the session code will be executed.
    ///
    /// If `initiator_addr` is empty, the initiator address will be derived from the provided
    /// `secret_key`.  It is an error for both fields to be empty.
    pub initiator_addr: &'a str,
}
