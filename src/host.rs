/**
 * This class encapsulates all per-host state used by the sss protocol.
 * By centralizing this state here instead of using global/static variables,
 * the host environment can be virtualized for simulation purposes
 * and multiple sss instances can be run in one process.
 *
 * It is the client's responsibility to ensure that a host object
 * is not destroyed while any sss objects still refer to it.
 *
 * Example: it is customary to create a shared_ptr to host.
 * @snippet doc/snippets.cpp Creating a host
 */
