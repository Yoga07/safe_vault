use super::validator::Validator;
use crate::{cmd::OutboundMsg, node::keys::NodeKeys, node::msg_decisions::ElderMsgDecisions};
use safe_nd::{AccountId, Message, MessageId, Money, NetworkCmd, TransferValidated};
use safe_transfers::{ActorEvent, TransferActor};
use ActorEvent::*;

pub(super) struct SectionFunds {
    actor: TransferActor<Validator>,
    decisions: ElderMsgDecisions,
}

impl SectionFunds {
    pub fn new(actor: TransferActor<Validator>, decisions: ElderMsgDecisions) -> Self {
        Self { actor, decisions }
    }

    pub fn initiate_reward_payout(&mut self, amount: Money, to: AccountId) -> Option<OutboundMsg> {
        match self.actor.transfer(amount, to) {
            Ok(Some(event)) => {
                self.actor.apply(TransferInitiated(event));
                self.decisions.send(Message::NetworkCmd {
                    cmd: NetworkCmd::InitiateRewardPayout(event.signed_transfer),
                    id: MessageId::new(),
                })
            }
            Ok(None) => None,
            Err(error) => None, // for now, but should give NetworkCmdError
        }
    }

    pub fn receive(&mut self, validation: TransferValidated) -> Option<OutboundMsg> {
        match self.actor.receive(validation) {
            Ok(Some(event)) => {
                self.actor.apply(TransferValidationReceived(event));
                self.decisions.send(Message::NetworkCmd {
                    cmd: NetworkCmd::FinaliseRewardPayout(event.proof?),
                    id: MessageId::new(),
                })
            }
            Ok(None) => None,
            Err(error) => None, // for now, but should give NetworkCmdError
        }
    }
}