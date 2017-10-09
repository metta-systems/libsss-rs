//
// Part of Metta OS. Check https://metta.systems for latest version.
//
// Copyright 2007 - 2017, Stanislav Karchebnyy <berkus@metta.systems>
//
// Distributed under the Boost Software License, Version 1.0.
// (See file LICENSE_1_0.txt or a copy at http://www.boost.org/LICENSE_1_0.txt)
//
use std::collections::HashMap;
use uia;

#[derive(Debug)]
pub struct ClientCoordinator;

#[derive(Debug)]
pub struct StreamResponder;

/// Host state related to stream management.
#[derive(Debug)]
struct StreamHostState {
    responder: StreamResponder,
    peers: HashMap<uia::PeerIdentity, StreamPeer>,
    listeners: HashMap<(String, String), Server>,
}

/// We store a routing::ClientCoordinator which keeps track of our
/// peer search requests. If you need different routing behavior, either extend
/// ClientCoordinator or replace RoutingHostState with your own.
#[derive(Debug)]
struct RoutingHostState {
    coordinator: ClientCoordinator,
}

/// This struct encapsulates all per-host state used by the sss protocol.
/// By centralizing this state here instead of using global/static variables,
/// the host environment can be virtualized for simulation purposes
/// and multiple sss instances can be run in one process.
#[derive(Debug)]
pub struct Host {
    base: uia::Host,
    streamState: StreamHostState,
    routingState: RoutingHostState,
}

impl Host {}
