// Copyright 2019 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under The General Public License (GPL), version 3.
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied. Please review the Licences for the specific language governing
// permissions and limitations relating to use of the SAFE Network Software.

mod auth;
mod client;
mod client_msg_analysis;
mod payment;
mod transfers;

use self::{
    auth::{Auth, AuthKeysDb},
    client::ClientGateway,
    client_msg_analysis::ClientMsgAnalysis,
    payment::Payments,
    transfers::{replica_manager::ReplicaManager, Transfers},
};
use crate::{
    node::msg_wrapping::ElderMsgWrapping,
    node::node_ops::{AuthDuty, GroupDecision, KeySectionDuty, NodeOperation},
    node::section_querying::SectionQuerying,
    node::state_db::NodeInfo,
    Result,
};
use log::trace;
use rand::{CryptoRng, Rng};
use routing::{Node as Routing, RoutingError};
use safe_nd::{ElderDuties, MsgEnvelope, PublicId};
use std::{cell::RefCell, rc::Rc};

pub struct KeySection<R: CryptoRng + Rng> {
    auth: Auth,
    gateway: ClientGateway<R>,
    payments: Payments,
    transfers: Transfers,
    msg_analysis: ClientMsgAnalysis,
    routing: Rc<RefCell<Routing>>,
}

impl<R: CryptoRng + Rng> KeySection<R> {
    pub fn new(info: NodeInfo, routing: Rc<RefCell<Routing>>, rng: R) -> Result<Self> {
        let section_querying = SectionQuerying::new(routing.clone());
        let wrapping = ElderMsgWrapping::new(info.keys.clone(), ElderDuties::Gateway);

        // Auth
        let auth_keys_db = AuthKeysDb::new(info.root_dir.clone(), info.init_mode)?;
        let auth = Auth::new(info.keys.clone(), auth_keys_db, wrapping);

        // ClientGateway
        let gateway = ClientGateway::new(info.clone(), section_querying, rng)?;

        // (AT2 Replicas)
        let replica_manager = Self::replica_manager(routing.clone())?;

        // Payments
        let payments = Payments::new(info.keys.clone(), replica_manager.clone());

        // Transfers
        let transfers = Transfers::new(info.keys.clone(), replica_manager);

        let msg_analysis = ClientMsgAnalysis::new(routing.clone());

        Ok(Self {
            auth,
            gateway,
            payments,
            transfers,
            msg_analysis,
            routing,
        })
    }

    pub fn process(&mut self, duty: KeySectionDuty) -> Option<NodeOperation> {
        use KeySectionDuty::*;
        match duty {
            EvaluateClientMsg { public_id, msg } => self.evaluate(public_id, &msg),
            ProcessGroupDecision(decision) => self.process_group_decision(decision),
            RunAsAuth(duty) => self.auth.process(duty),
            RunAsGateway(duty) => self.gateway.process(&duty),
            RunAsPayment(duty) => self.payments.process(&duty),
            RunAsTransfers(duty) => self.transfers.process(&duty),
        }
    }

    fn evaluate(&mut self, public_id: PublicId, msg: &MsgEnvelope) -> Option<NodeOperation> {
        if let Some(error) = self.auth.verify_client_signature(msg) {
            return Some(error.into());
        };
        if let Some(error) = self.auth.authorise_app(&public_id, &msg) {
            return Some(error.into());
        }
        self.msg_analysis.evaluate(msg)
    }

    /// Basically.. when Gateway nodes have voted and agreed,
    /// that this is a valid client request to handle locally,
    /// they'll process it locally.
    fn process_group_decision(&mut self, decision: GroupDecision) -> Option<NodeOperation> {
        use GroupDecision::*;
        trace!("KeySection: Group decided on {:?}", decision);
        match decision {
            Process {
                cmd,
                msg_id,
                origin,
            } => Some(
                AuthDuty::Process {
                    cmd,
                    msg_id,
                    origin,
                }
                .into(),
            ),
        }
    }

    // Update our replica with the latest keys
    pub fn elders_changed(&mut self) -> Option<NodeOperation> {
        let pub_key_set = self.routing.borrow().public_key_set().ok()?.clone();
        let sec_key_share = self.routing.borrow().secret_key_share().ok()?.clone();
        let proof_chain = self.routing.borrow().our_history()?.clone();
        let our_index = self.routing.borrow().our_index().ok()?;
        self.transfers.update_replica_on_churn(
            pub_key_set,
            sec_key_share,
            our_index,
            proof_chain,
        )?;
        None
    }

    fn replica_manager(routing: Rc<RefCell<Routing>>) -> Result<Rc<RefCell<ReplicaManager>>> {
        let node = routing.borrow();
        let public_key_set = node.public_key_set()?;
        let secret_key_share = node.secret_key_share()?;
        let key_index = node.our_index()?;
        let proof_chain = node.our_history().ok_or(RoutingError::InvalidState)?;
        let replica_manager = ReplicaManager::new(
            secret_key_share,
            key_index,
            public_key_set,
            vec![],
            proof_chain.clone(),
        )?;
        Ok(Rc::new(RefCell::new(replica_manager)))
    }
}

// /// Process a msg from a client.
// fn evaluate_client_msg(&mut self, msg: &MsgEnvelope) -> Option<NodeOperation> {
//     match &msg.message {
//         Message::Cmd {
//             cmd: Cmd::Auth(_), ..
//         } => wrap(self.auth.initiate(msg)),
//         Message::Query {
//             query: Query::Auth(_),
//             ..
//         } => wrap(self.auth.list_keys_and_version(msg)),
//         Message::Query {
//             query: Query::Transfer(query),
//             ..
//         } => self.transfers.process_query(query, msg.id(), msg.origin.address())
//         | Message::Cmd {
//             cmd: Cmd::Transfer(cmd),
//             ..
//         } => self.transfers.process_cmd(cmd, msg.id(), msg.origin.address()),
//         Message::Cmd {
//             cmd: Cmd::Data { cmd, .. },
//             ..
//         } => self.payment.process(cmd),
//         Message::Query {
//             query: Query::Data(data_query),
//             ..
//         } => forward,
//         _ => None, // error..!
//     }
// }

// fn process_transfer(&self, msg: &MsgEnvelope) -> Option<NodeOperation> {
//     let duty = match &msg.message {
//         Message::Query {
//             query: Query::Transfer(query),
//             ..
//         } => TransferDuty::ProcessQuery {
//             query: query.clone().into(),
//             msg_id: msg.id(),
//             origin: msg.origin.address(),
//         },
//         Message::Cmd {
//             cmd: Cmd::Transfer(cmd),
//             ..
//         } => TransferDuty::ProcessCmd {
//             cmd: cmd.clone().into(),
//             msg_id: msg.id(),
//             origin: msg.origin.address(),
//         },
//         _ => return None, // error..!
//     };
//     Some(NodeOperation::RunAsTransfers(duty))
// }