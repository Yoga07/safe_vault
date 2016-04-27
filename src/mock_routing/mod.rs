// Copyright 2015 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under (1) the MaidSafe.net Commercial License,
// version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
// licence you accepted on initial access to the Software (the "Licences").
//
// By contributing code to the SAFE Network Software, or to this project generally, you agree to be
// bound by the terms of the MaidSafe Contributor Agreement, version 1.0.  This, along with the
// Licenses can be found in the root directory of this project at LICENSE, COPYING and CONTRIBUTOR.
//
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.
//
// Please review the Licences for the specific language governing permissions and limitations
// relating to use of the SAFE Network Software.

#![cfg(test)]

#![allow(unused)]

mod mock_routing_impl;

pub use self::mock_routing_impl::{Event, NodeInfo};

use self::mock_routing_impl::MockRoutingNodeImpl;
use kademlia_routing_table::RoutingTable;
use rand::random;
use routing::{Authority, Data, DataIdentifier, ImmutableData, InterfaceError, MessageId,
              RequestContent, RequestMessage, ResponseContent, ResponseMessage, RoutingError};
use sodiumoxide::crypto::sign::PublicKey;
use std::sync::{Arc, Mutex, mpsc};
use xor_name::XorName;

pub struct MockRoutingNode {
    pimpl: Arc<Mutex<MockRoutingNodeImpl>>,
}

impl MockRoutingNode {
    pub fn new(event_sender: mpsc::Sender<Event>,
               _use_data_cache: bool)
               -> Result<MockRoutingNode, RoutingError> {
        Ok(MockRoutingNode { pimpl: Arc::new(Mutex::new(MockRoutingNodeImpl::new(event_sender))) })
    }

    pub fn get_client_receiver(&self) -> mpsc::Receiver<Event> {
        unwrap_result!(self.pimpl.lock()).get_client_receiver()
    }

    // -----------  the following methods are for testing purpose only   ------------- //
    pub fn client_get(&self,
                      client_address: XorName,
                      client_pub_key: PublicKey,
                      data_identifier: DataIdentifier) {
        unwrap_result!(self.pimpl.lock())
            .client_get(Self::client_authority(client_address, client_pub_key),
                        data_identifier)
    }

    pub fn client_put(&self, client_address: XorName, client_pub_key: PublicKey, data: Data) {
        unwrap_result!(self.pimpl.lock())
            .client_put(Self::client_authority(client_address, client_pub_key), data)
    }

    pub fn client_post(&self, client_address: XorName, client_pub_key: PublicKey, data: Data) {
        unwrap_result!(self.pimpl.lock())
            .client_post(Self::client_authority(client_address, client_pub_key), data)
    }

    pub fn client_delete(&self, client_address: XorName, client_pub_key: PublicKey, data: Data) {
        unwrap_result!(self.pimpl.lock())
            .client_delete(Self::client_authority(client_address, client_pub_key), data)
    }

    pub fn node_added_event(&self, node_added: XorName) {
        unwrap_result!(self.pimpl.lock()).node_added_event(node_added)
    }

    pub fn node_lost_event(&self, node_lost: XorName) {
        unwrap_result!(self.pimpl.lock()).node_lost_event(node_lost)
    }

    pub fn get_requests_given(&self) -> Vec<RequestMessage> {
        unwrap_result!(self.pimpl.lock()).get_requests_given()
    }

    pub fn put_requests_given(&self) -> Vec<RequestMessage> {
        unwrap_result!(self.pimpl.lock()).put_requests_given()
    }

    pub fn post_requests_given(&self) -> Vec<RequestMessage> {
        unwrap_result!(self.pimpl.lock()).post_requests_given()
    }

    pub fn delete_requests_given(&self) -> Vec<RequestMessage> {
        unwrap_result!(self.pimpl.lock()).delete_requests_given()
    }

    pub fn get_successes_given(&self) -> Vec<ResponseMessage> {
        unwrap_result!(self.pimpl.lock()).get_successes_given()
    }

    pub fn get_failures_given(&self) -> Vec<ResponseMessage> {
        unwrap_result!(self.pimpl.lock()).get_failures_given()
    }

    pub fn put_successes_given(&self) -> Vec<ResponseMessage> {
        unwrap_result!(self.pimpl.lock()).put_successes_given()
    }

    pub fn put_failures_given(&self) -> Vec<ResponseMessage> {
        unwrap_result!(self.pimpl.lock()).put_failures_given()
    }

    pub fn post_successes_given(&self) -> Vec<ResponseMessage> {
        unwrap_result!(self.pimpl.lock()).post_successes_given()
    }

    pub fn post_failures_given(&self) -> Vec<ResponseMessage> {
        unwrap_result!(self.pimpl.lock()).post_failures_given()
    }

    pub fn delete_successes_given(&self) -> Vec<ResponseMessage> {
        unwrap_result!(self.pimpl.lock()).delete_successes_given()
    }

    pub fn delete_failures_given(&self) -> Vec<ResponseMessage> {
        unwrap_result!(self.pimpl.lock()).delete_failures_given()
    }

    pub fn refresh_requests_given(&self) -> Vec<RequestMessage> {
        unwrap_result!(self.pimpl.lock()).refresh_requests_given()
    }

    pub fn get_routing_table(&self) -> RoutingTable<NodeInfo> {
        unwrap_result!(self.pimpl.lock()).get_routing_table()
    }

    // -----------  the following methods are expected to be API functions   ------------- //
    pub fn send_get_request(&self,
                            src: Authority,
                            dst: Authority,
                            data_identifier: DataIdentifier,
                            id: MessageId)
                            -> Result<(), InterfaceError> {
        unwrap_result!(self.pimpl.lock()).send_get_request(src, dst, data_identifier, id)
    }

    pub fn send_put_request(&self,
                            src: Authority,
                            dst: Authority,
                            data: Data,
                            id: MessageId)
                            -> Result<(), InterfaceError> {
        unwrap_result!(self.pimpl.lock()).send_put_request(src, dst, data, id)
    }

    pub fn send_post_request(&self,
                             src: Authority,
                             dst: Authority,
                             data: Data,
                             id: MessageId)
                             -> Result<(), InterfaceError> {
        unwrap_result!(self.pimpl.lock()).send_post_request(src, dst, data, id)
    }

    pub fn send_delete_request(&self,
                               src: Authority,
                               dst: Authority,
                               data: Data,
                               id: MessageId)
                               -> Result<(), InterfaceError> {
        unwrap_result!(self.pimpl.lock()).send_delete_request(src, dst, data, id)
    }

    pub fn send_get_success(&self,
                            src: Authority,
                            dst: Authority,
                            data: Data,
                            id: MessageId)
                            -> Result<(), InterfaceError> {
        unwrap_result!(self.pimpl.lock()).send_get_success(src, dst, data, id)
    }

    pub fn send_get_failure(&self,
                            src: Authority,
                            dst: Authority,
                            request: RequestMessage,
                            external_error_indicator: Vec<u8>,
                            id: MessageId)
                            -> Result<(), InterfaceError> {
        unwrap_result!(self.pimpl.lock())
            .send_get_failure(src, dst, request, external_error_indicator, id)
    }

    pub fn send_put_success(&self,
                            src: Authority,
                            dst: Authority,
                            data_identifier: DataIdentifier,
                            id: MessageId)
                            -> Result<(), InterfaceError> {
        unwrap_result!(self.pimpl.lock()).send_put_success(src, dst, data_identifier, id)
    }

    pub fn send_put_failure(&self,
                            src: Authority,
                            dst: Authority,
                            request: RequestMessage,
                            external_error_indicator: Vec<u8>,
                            id: MessageId)
                            -> Result<(), InterfaceError> {
        unwrap_result!(self.pimpl.lock())
            .send_put_failure(src, dst, request, external_error_indicator, id)
    }

    pub fn send_post_success(&self,
                             src: Authority,
                             dst: Authority,
                             data_identifier: DataIdentifier,
                             id: MessageId)
                             -> Result<(), InterfaceError> {
        unwrap_result!(self.pimpl.lock()).send_post_success(src, dst, data_identifier, id)
    }

    pub fn send_post_failure(&self,
                             src: Authority,
                             dst: Authority,
                             request: RequestMessage,
                             external_error_indicator: Vec<u8>,
                             id: MessageId)
                             -> Result<(), InterfaceError> {
        unwrap_result!(self.pimpl.lock())
            .send_post_failure(src, dst, request, external_error_indicator, id)
    }

    pub fn send_delete_success(&self,
                               src: Authority,
                               dst: Authority,
                               data_identifier: DataIdentifier,
                               id: MessageId)
                               -> Result<(), InterfaceError> {
        unwrap_result!(self.pimpl.lock()).send_delete_success(src, dst, data_identifier, id)
    }

    pub fn send_delete_failure(&self,
                               src: Authority,
                               dst: Authority,
                               request: RequestMessage,
                               external_error_indicator: Vec<u8>,
                               id: MessageId)
                               -> Result<(), InterfaceError> {
        unwrap_result!(self.pimpl.lock())
            .send_delete_failure(src, dst, request, external_error_indicator, id)
    }

    pub fn send_refresh_request(&self,
                                src: Authority,
                                dst: Authority,
                                content: Vec<u8>,
                                message_id: MessageId)
                                -> Result<(), InterfaceError> {
        unwrap_result!(self.pimpl.lock()).send_refresh_request(src, dst, content, message_id)
    }

    pub fn close_group(&self, name: XorName) -> Result<Option<Vec<XorName>>, InterfaceError> {
        unwrap_result!(self.pimpl.lock()).close_group(name)
    }

    pub fn name(&self) -> Result<XorName, InterfaceError> {
        unwrap_result!(self.pimpl.lock()).name()
    }

    pub fn quorum_size(&self) -> Result<usize, InterfaceError> {
        unwrap_result!(self.pimpl.lock()).quorum_size()
    }

    fn client_authority(client_address: XorName, client_pub_key: PublicKey) -> Authority {
        Authority::Client {
            client_key: client_pub_key,
            peer_id: random(),
            proxy_node_name: client_address,
        }
    }
}