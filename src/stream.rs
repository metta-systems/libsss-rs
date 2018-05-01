/**
 * @nosubgrouping
 *
 * User-space interface to the stream.
 *
 * This is the primary high-level class that client applications use to communicate over
 * the network via SSS. The class provides standard stream-oriented read/write functionality
 * and adds SSS-specific methods for controlling SSS streams and substreams.
 *
 * To initiate an outgoing "top-level" SSS stream to a remote host, the client application
 * creates a stream instance and then calls connect_to().
 *
 * To initiate a sub-stream from an existing stream, the application calls
 * open_substream() on the parent stream.
 *
 * To accept incoming top-level streams from other hosts the application creates
 * a sss::server instance, and that class creates stream instances for incoming
 * connections.
 *
 * To accept new incoming substreams on existing streams, the application calls
 * listen() on the parent stream, and upon arrival of a new_substream() signal
 * the application calls accept_substream() to obtain a stream object for the
 * new incoming substream.
 *
 * SSS uses service and protocol names in place of the port numbers used
 * by TCP and UDP to differentiate and select among different application
 * protocols.
 *
 * A service name represents an abstract service being provided: e.g., "Web",
 * "File", "E-mail", etc. A protocol name represents a concrete application
 * protocol to be used for communication with an abstract service: e.g.,
 * "HTTP 1.0" or "HTTP 1.1" for communication with a "Web" service; "FTP",
 * "NFS v4", or "CIFS" for communication with a "File" service; "SMTP", "POP3",
 * or "IMAP4" for communication with an "E-mail" service.
 *
 * Service names are intended to be suitable for non-technical users to see, in
 * a service manager or firewall configuration utility for example, while
 * protocol names are primarily intended for application developers.
 *
 * A server can support multiple distinct protocols on one logical service,
 * for backward compatibility or functional modularity reasons for example,
 * by registering to listen on multiple (service, protocol) name pairs.
 *
 * @see server
 */
