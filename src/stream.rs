//
// Part of Metta OS. Check https://metta.systems for latest version.
//
// Copyright 2007 - 2017, Stanislav Karchebnyy <berkus@metta.systems>
//
// Distributed under the Boost Software License, Version 1.0.
// (See file LICENSE_1_0.txt or a copy at http://www.boost.org/LICENSE_1_0.txt)
//
use std::collections::{HashMap, HashSet};
use host::Host;
use std::rc::Rc;
use uia;

pub struct AbstractStream;
pub struct BaseStream;

/// Flag bits used as arguments to the listen() method, indicating when and
/// how to accept incoming substreams.
pub enum ListenMode {
    /// Reject incoming substreams.
    Reject,
    /// Accept subs up to receive buffer size.
    BufferLimit,
    /// Accept substreams of any size.
    Unlimited,
    /// Flag: Substreams inherit this listen mode.
    Inherit,
}

/// Flag bits used as operands to the shutdown() method, indicating how and
/// in which direction(s) to shutdown a stream.
pub enum ShutdownMode {
    /// Read (incoming data) direction.
    Read = 1,
    /// Write (outgoing data) direction.
    Write = 2,
    /// Both directions (Read|Write).
    Close = 3,
    ///< Forceful reset.
    Reset = 4,
}

pub struct UniqueStreamID;
pub struct RoutingCoordination;

/// Private helper class to keep information about a peer we are establishing connection with.
/// Contains cryptographic identifier as well as a set of possible endpoint addresses.
/// Keeps track of established sessions.
/// Communicates with resolver service to find reachable peer endpoints.
///
/// uia::peer
/// stream_peer
///  - list of active streams
///      - both connected to channels
///      - and not bound anywhere (waiting for channel or idle)
/// routing_peer
///  - routing state
///      - peer IP lookups
///      - DHT state related to this peer's online status
///
/// For the purposes of channel management, it might be necessary to split this class into
/// uia::channel_peer that would handle channels and DHT state and sss::stream_peer that would
/// add stream management.
///
/// Perhaps, routing coordination must also be extracter into routing::routing_peer class.
struct StreamPeer {
    /// All existing streams involving this peer.
    all_streams: HashSet<BaseStream>,
    /// All streams that have USIDs, registered by their USIDs.
    usid_streams: HashMap<UniqueStreamID, Rc<BaseStream>>,
    coord: RoutingCoordination,
}

impl StreamPeer {
    fn no_lookups_possible(self) -> bool {
        self.coord.lookups.isEmpty() && key_exchanges_initiated.isEmpty()
    }
}


/// @nosubgrouping
///
/// User-space interface to the stream.
///
/// This is the primary high-level class that client applications use to communicate over
/// the network via SSS. The class provides standard stream-oriented read/write functionality
/// and adds SSS-specific methods for controlling SSS streams and substreams.
///
/// To initiate an outgoing "top-level" SSS stream to a remote host, the client application
/// creates a stream instance and then calls connect_to().
///
/// To initiate a sub-stream from an existing stream, the application calls
/// open_substream() on the parent stream.
///
/// To accept incoming top-level streams from other hosts the application creates
/// a sss::server instance, and that class creates stream instances for incoming
/// connections.
///
/// To accept new incoming substreams on existing streams, the application calls
/// listen() on the parent stream, and upon arrival of a new_substream() signal
/// the application calls accept_substream() to obtain a stream object for the
/// new incoming substream.
///
/// SSS uses service and protocol names in place of the port numbers used
/// by TCP and UDP to differentiate and select among different application
/// protocols.
///
/// A service name represents an abstract service being provided: e.g., "Web",
/// "File", "E-mail", etc. A protocol name represents a concrete application
/// protocol to be used for communication with an abstract service: e.g.,
/// "HTTP 1.0" or "HTTP 1.1" for communication with a "Web" service; "FTP",
/// "NFS v4", or "CIFS" for communication with a "File" service; "SMTP", "POP3",
/// or "IMAP4" for communication with an "E-mail" service.
///
/// Service names are intended to be suitable for non-technical users to see, in
/// a service manager or firewall configuration utility for example, while
/// protocol names are primarily intended for application developers.
///
/// A server can support multiple distinct protocols on one logical service,
/// for backward compatibility or functional modularity reasons for example,
/// by registering to listen on multiple (service, protocol) name pairs.
///
/// @see Server
pub struct Stream {
    /// Per-host SSS state
    host: Rc<Host>,
    /// Internal stream control object @todo needed still?
    stream: Rc<AbstractStream>,
}

impl Stream {
    /// Create a new stream instance.
    /// The stream must be connected to a remote host via connect_to() before it can be
    /// used for communication.
    /// @param host the Host instance containing hostwide SSS state.
    pub fn create(host: Rc<Host>) {
        Stream { host, stream }
    }

    /* @name Connection-related services. */

    /*
     * Connect to a given service and protocol on a remote host.
     * The stream logically goes into the "connected" state immediately (i.e., is_connected()
     * returns true), and the application may start writing to the stream immediately, but actual
     * network connectivity may not be established until later or not at all.
     * Either during or some time after the connect_to() call, SSS emits the on_link_up() signal
     * to indicate active connectivity, or on_link_down() to indicate connectivity could not be
     * established. An on_link_down() signal is not necessarily fatal, however: unless
     * the application disconnects or deletes the stream object, SSS will continue attempting
     * to establish connectivity and emit on_link_up() if and when it eventually succeeds.
     *
     * If the stream is already connected when connect_to() is called, SSS immediately re-binds
     * the stream object to the new target but closes the old stream gracefully in the background.
     * Similarly, SSS closes the stream gracefully in the background if the application just
     * deletes a connected Stream object. To close a stream forcefully without retaining internal
     * state, the application may explicitly call shutdown(reset) before re-connecting or
     * deleting the stream object.
     *
     * @param  destination The endpoint identifier (EID) of the desired remote host to connect to.
     *      The destination may be either a cryptographic EID or a non-cryptographic legacy address
     *      as defined by the identity class. The destination may also be empty, indicating that
     *      the destination's identity is unknown; in this case the caller must provide a location
     *      hint via the destination_endpoint_hint argument.
     * @param  service The service name to connect to on the remote host.
     * @param  protocol The application protocol name to connect to.
     * @param  destination_endpoint_hint An optional location hint for SSS to use in attempting
     *      to contact the host. If the @a destination parameter is a cryptographic EID, which
     *      is inherently location-independent, SSS may need a location hint to find the remote
     *      host if this host and the remote host are unable to locate each other using routing
     *      service. This parameter is not needed if the destination is a non-cryptographic
     *      legacy address.
     * @return true if successful, false if an error occurred.
     * @see sss::identity
     */
    pub fn connect_to(
        self,
        destination: uia::PeerIdentity,
        service: String,
        protocol: String,
        destination_endpoint_hint: uia::comm::Endpoint,
    ) -> bool {
        true
    }

    /*
     * Disconnect the stream from its current peer.
     *
     * This method immediately returns the stream to the unconnected state:
     * is_connected() subsequently returns false.
     *
     * If the stream has not already been shutdown, however, SSS gracefully closes the stream
     * in the background as if with shutdown(close).
     *
     * @see shutdown()
     */
    pub fn disconnect(self) {}

    /*
     * Returns true if the stream is logically connected and usable for data read/write operations.
     * The return value from this method changes only as a result of the application's calls to
     * connect_to() and disconnect(). Logical connectivity does not imply that the network link
     * is live: the underlying link may go up or down repeatedly during the logical lifetime
     * of the stream.
     */
    pub fn is_connected(self) -> bool {
        true
    }

    /*
     * Provide a new or additional peer address/location hint.
     * May be called at any time, e.g., if the target host has migrated,
     * to give SST a hint at where it might find the target
     * in order to re-establish connectivity.
     */
    pub fn connect_at(self, ep: uia::comm::Endpoint) {}

    /* @name Byte-oriented data transfer. Reading data. */

    /*
     * Determine the number of bytes currently available to be read via read_data().
     * Note that calling read_data() with a buffer this large may not read all
     * the available data if there are message/record markers present in the read stream.
     * @return Number of bytes available.
     */
    pub fn bytes_available(self) -> usize {
        0
    }

    /*
     * Returns true if at least one byte is available for reading.
     */
    pub fn has_bytes_available(self) -> bool {
        self.bytes_available() > 0
    }
}
/*
 * Read up to max_size bytes of data from the stream.
 * This method only returns data that has already been received and is waiting to be read:
 * it never blocks waiting for new data to arrive on the network. A single read_data() call
 * never crosses a record boundary: if it encounters a record marker in the incoming
 * byte stream, it returns only the bytes up to that record marker and leaves any further data
 * for subsequent read_data() calls.
 *
 * @param data the buffer into which to read. This parameter may be nullptr, in which case
 *      the data read is simply discarded.
 * @param max_size the maximum number of bytes to read.
 * @return the number of bytes read, or -1 if an error occurred.
 *      Returns zero if there is no error condition
 *      but no bytes are immediately available for reading.
 */
// ssize_t read_data(char* data, ssize_t max_size);

/*
 * Read up to max_size bytes of data into a byte_array.
 * @overload
 */
// byte_array read_data(ssize_t max_size = 1 << 30);

/*
 * Return number of complete records currently available for reading.
 */
// int pending_records() const;

/*
 * Return true if at least one complete record is currently available for reading.
 */
// inline bool has_pending_records() const { return pending_records() > 0; }

/*
 * Return size of the first available record.
 *
 * XXX This function may need to be removed from the API, since the size of a large record
 * will be unknown until the entire record has already come in, which it may not if receiver
 * congestion control is working.
 */
// ssize_t pending_record_size() const;

/*
 * Read a complete record all at once.
 * Reads up to the next record marker (or end of stream). If no record marker has arrived
 * yet, just returns without reading anything. If the next record to be read is larger than
 * max_size, this method simply discards the record data beyond max_size.
 * @param data the buffer into which to read the record.
 * @param max_size the maximum size of the record to read;
 *      any bytes in the record beyond this are discarded.
 * @return the size of the record read, or -1 if an error occurred.
 *      Returns zero if there is no error condition but no complete record is
 *      available for reading.
 */
// ssize_t read_record(char* data, ssize_t max_size);

/*
 * Read a complete record into a new byte_array.
 * @param max_size the maximum size of the record to read;
 *      any bytes in the record beyond this are discarded.
 * @return the record received, or an empty byte_array if an error occurred
 *      or there are no records to receive.
 * @overload
 */
// byte_array read_record(ssize_t max_size = 1 << 30);

/*
 * Returns true if all data has been read from the stream and the remote host has closed
 * its end: no more data will ever be available for reading on this stream.
 */
// bool at_end() const; // @fixme QIODevice relic

/* @name Byte-oriented data transfer. Writing data. */

/*
 * Write data bytes to a stream.
 * If not all the supplied data can be transmitted immediately, it is queued locally until
 * ready to transmit.
 * @param data the buffer containing the bytes to write.
 * @param size the number of bytes to write.
 * @return the number of bytes written (same as the size parameter), or -1 if an error occurred.
 */
// ssize_t write_data(const char* data, ssize_t size);

/*
 * Write a record to a stream.
 * Writes the data in the supplied buffer followed by a record/record marker. If some data has
 * already been written via write_data(), then that data logically forms the "head" of the
 * record and the data presented to write_record() forms the "tail". Thus, a large record can
 * be written incrementally by calling write_data() any number of times followed by a call to
 * write_record() to finish the record. A record marker is written at the current position
 * even if this method is called with no data (size = 0).
 * @param data the buffer containing the record to write.
 * @param size the number of bytes of data to write.
 * @return the number of bytes written (same as the size parameter), or -1 if an error occurred.
 */
// ssize_t write_record(const char* data, ssize_t size);

/*
 * Write a record to a stream.
 * @param rec a byte_array containing the record to write.
 * @overload
 */
// inline ssize_t write_record(const byte_array& rec)
// {
//     return write_record(rec.data(), rec.size());
// }

/* @name Datagram protocol. Send and receive unordered, unreliable datagrams on this stream. */

// enum class datagram_type
// {
//     non_reliable = 0,
//     reliable     = 1
// };

/*
 * Send and receive unordered datagrams on this stream.
 * Reliability is optional.
 */
// ssize_t read_datagram(char* data, ssize_t max_size);
// byte_array read_datagram(ssize_t max_size = 1 << 30);
// ssize_t write_datagram(const char* data, ssize_t size, datagram_type is_reliable);
// inline ssize_t write_datagram(const byte_array& dgm, datagram_type is_reliable)
// {
//     return write_datagram(dgm.data(), dgm.size(), is_reliable);
// }

/*
 * Check for pending datagrams.
 */
// bool has_pending_datagrams() const;
// ssize_t pending_datagram_size() const;

/* @name Substreams management. */

/*
 * Initiate a new substream as a child of this stream.
 * This method completes without synchronizing with the remote host, and the client application
 * can use the new substream immediately to send data to the remote host via the new substream.
 * If the remote host is not yet ready to accept the new substream, SSS queues the new
 * substream and any data written to it locally until the remote host is ready to accept
 * the new substream.
 *
 * @return A stream object representing the new substream.
 */
// stream_ptr open_substream();

/*
 * Listen for incoming substreams on this stream.
 */
// void listen(listen_mode mode);

/*
 * Returns true if this stream is set to accept incoming substreams.
 */
// bool is_listening() const;

/*
 * Accept a waiting incoming substream.
 *
 * @return nullptr if no incoming substreams are waiting.
 */
// stream_ptr accept_substream();

/* @name Stream control. */

/*
 * Returns the endpoint identifier of the local host as used in connecting the current stream.
 * Only valid if the stream is connected.
 */
// uia::peer_identity local_host_id() const;

/*
 * Returns the endpoint identifier of the remote host to which this stream is connected.
 */
// uia::peer_identity remote_host_id() const;

/*
 * Returns true if the stream is logically connected and network connectivity is currently
 * available. SSS emits on_link_up() and on_link_down() signals when the underlying link
 * connectivity state changes.
 */
// bool is_link_up() const;

/*
 * Set the stream's transmit priority level. When the application has multiple streams with
 * data ready to transmit to the same remote host, SSS uses the respective streams' priority
 * levels to determine which data to transmit first. SSS gives strict preference to streams
 * with higher priority over streams with lower priority, but it divides available transmit
 * bandwidth evenly among streams with the same priority level. All streams have a default
 * priority of zero on creation.
 * @param priority the new priority level for the stream.
 */
// void set_priority(int priority);

/*
 * Returns the stream's current priority level.
 */
// int current_priority() const;

/*
 * Control the receive buffer size for this stream.
 */
// void set_receive_buffer_size(ssize_t size);
/*
 * Control the initial receive buffer size for new child streams.
 */
// void set_child_receive_buffer_size(ssize_t size);

/*
 * Begin graceful or forceful shutdown of the stream.
 * If this internal stream control object is lingering - i.e., if its 'owner_'
 * back-pointer is null - then it should self-destruct once the shutdown is complete.
 *
 * To close the stream gracefully in either or both directions, specify read, write,
 * or read|write for the @a mode argument. Closing the stream in the write direction
 * writes the end-of-stream marker to our end of the stream, indicating to our peer
 * that no more data will arrive from us. Closing the stream in the read direction
 * discards any data waiting to be read or subsequently received from the peer.
 * Specify a mode of @a reset to shutdown the stream immediately;
 * written data that is still queued or in transit may be lost.
 *
 * @param mode which part of the stream to close:
 *      either read, write, close (read|write), or reset.
 */
// void shutdown(shutdown_mode mode);

/*
 * Gracefully close the stream for both reading and writing. Still-buffered written data
 * continues to be sent, but any further data received from the other side is dropped.
 */
// inline void close() { shutdown(shutdown_mode::close); }

/*
 * Give the stream layer a location hint for a specific EID, which may or may not be the EID
 * of the host to which this stream is currently connected (if any). The stream layer will
 * use this hint in any current or subsequent attempts to connect to the specified EID.
 */
// bool add_location_hint(uia::peer_identity const& eid, uia::comm::endpoint const& hint);

/*
 * Set an error condition on this stream and emit the error_notify signal.
 * @param error Textual description of the error.
 */
// void set_error(std::string const& error);

// std::string error() const { return error_string_; }

/*
 * Dump the state of this stream, for debugging purposes.
 */
// void dump();

/* @name Signals. */

// using bytes_written_signal = boost::signals2::signal<void(ssize_t)>;
/*
 * Emitted when some locally buffered data gets flushed
 * after being delivered to the receiver and acknowledged.
 */
// bytes_written_signal on_bytes_written;

// using ready_signal = boost::signals2::signal<void(void)>;
// ready_signal on_ready_read;

/*
 * Emitted when a record marker arrives in the incoming byte stream ready to be read. This
 * signal indicates that a complete record may be read at once. If the client wishes to
 * delay the reading of any data in the record or record until this signal arrives, to avoid
 * the potential for deadlock the client must ensure that the stream's maximum receive window
 * is large enough to accommodate any record or record that might arrive - or else monitor
 * the on_receive_blocked() signal and dynamically expand the receive window as necessary.
 */
// ready_signal on_ready_read_record;
/*
 * Emitted when a queued incoming substream may be read as a datagram. This occurs once the
 * substream's entire data content arrives and the remote peer closes its end while the
 * substream is queued, so that the entire content may be read at once via read_datagram().
 * (XXX or perhaps when an entire first record arrives?)
 * Note that if the client wishes to read datagrams using this signal, the client must ensure
 * that the parent's maximum receive window is large enough to hold any incoming datagram
 * that might arrive, or else monitor the parent stream's on_receive_blocked() signal and grow
 * the receive window to accommodate large datagrams.
 */
// ready_signal on_ready_read_datagram;
/*
 * Emitted when our transmit buffer contains only in-flight data
 * and we could transmit more immediately if the app supplies more.
 */
// ready_signal on_ready_write;
/*
 * Emitted when incoming data has filled our receive window. When this situation occurs,
 * the client must read some queued data or else increase the maximum receive window before
 * SSS will accept further incoming data from the peer. Every single byte of the receive window
 * might not be utilized when the receive process becomes blocked in this way, because SSS
 * does not fragment packets just to "top up" a nearly empty receive window: the effective
 * limit may be as low as half the specified maximum window size.
 */
// ready_signal on_receive_blocked;

// using link_status_signal = boost::signals2::signal<void(void)>;
/*
 * Emitted when the stream establishes live connectivity
 * upon first connecting, or after being down or stalled.
 */
// link_status_signal on_link_up;
/*
 * Emitted when connectivity on the stream has temporarily stalled. SSS emits the
 * on_link_stalled() signal at the first sign of trouble: this provides an early warning
 * that the link may have failed, but it may also just represent an ephemeral network glitch.
 * The application may wish to use this signal to indicate the network status to the user.
 */
// link_status_signal on_link_stalled;
/*
 * Emitted when link connectivity for the stream has been lost. SSS may emit this signal either
 * due to a timeout or due to detection of a link- or network-level "hard" failure.
 * The link may come back up sometime later, however, in which case SSS emits on_link_up()
 * and stream connectivity resumes.
 *
 * If the application desires TCP-like behavior where a connection timeout causes permanent
 * stream failure, the application may simply destroy the stream upon receiving the
 * on_link_down() signal.
 */
// link_status_signal on_link_down;

// using link_status_changed_signal = boost::signals2::signal<void(uia::comm::socket::status)>;
/*
 * Emitted when this stream observes a change in link status.
 */
// link_status_changed_signal on_link_status_changed;

// using substream_notify_signal = boost::signals2::signal<void(void)>;
/*
 * Emitted when we receive an incoming substream while listening.
 * In response the client should call accept_substream() in a loop
 * to accept all queued incoming substreams,
 * until accept_substream() returns nullptr.
 */
// substream_notify_signal on_new_substream;

// using error_signal = boost::signals2::signal<void(const std::string&)>;
// using reset_signal = boost::signals2::signal<void(void)>;
/*
 * Emitted when an error condition is detected on the stream.
 * Link stalls or failures are not considered error conditions.
 */
// error_signal on_error_notify;
/*
 * Emitted when the stream is reset by either endpoint.
 */
// reset_signal on_reset_notify;
