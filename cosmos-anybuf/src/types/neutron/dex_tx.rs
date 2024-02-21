use crate::{
    types::{neutron::dex::Params, timestamp::Timestamp},
    StargateMsg,
};
use anybuf::Anybuf;

pub struct MsgDeposit {
    pub creator: String,
    pub receiver: String,
    pub token_a: String,
    pub token_b: String,
    pub amounts_a: Vec<String>,
    pub amounts_b: Vec<String>,
    pub tick_indexes_a_to_b: Vec<i64>,
    pub fees: Vec<u64>,
    pub options: Vec<DepositOptions>,
}

impl MsgDeposit {
    pub fn new(
        creator: impl Into<String>,
        receiver: impl Into<String>,
        token_a: impl Into<String>,
        token_b: impl Into<String>,
        amounts_a: Vec<String>,
        amounts_b: Vec<String>,
        tick_indexes_a_to_b: Vec<i64>,
        fees: Vec<u64>,
        options: Vec<DepositOptions>,
    ) -> Self {
        Self {
            creator: creator.into(),
            receiver: receiver.into(),
            token_a: token_a.into(),
            token_b: token_b.into(),
            amounts_a,
            amounts_b,
            tick_indexes_a_to_b,
            fees,
            options,
        }
    }
}

impl StargateMsg for MsgDeposit {
    fn url() -> &'static str {
        "/neutron.dex.MsgDeposit"
    }

    fn to_buf(&self) -> Vec<u8> {
        let options_messages: Vec<Anybuf> =
            self.options.iter().map(DepositOptions::to_anybuf).collect();
        Anybuf::new()
            .append_string(1, &self.creator)
            .append_string(2, &self.receiver)
            .append_string(3, &self.token_a)
            .append_string(4, &self.token_b)
            .append_repeated_string(5, &self.amounts_a)
            .append_repeated_string(6, &self.amounts_b)
            .append_repeated_int64(7, &self.tick_indexes_a_to_b)
            .append_repeated_uint64(8, &self.fees)
            .append_repeated_message(9, &options_messages)
            .into_vec()
    }
}

pub struct MsgWithdrawal {
    pub creator: String,
    pub receiver: String,
    pub token_a: String,
    pub token_b: String,
    pub shares_to_remove: Vec<String>,
    pub tick_indexes_a_to_b: Vec<i64>,
    pub fees: Vec<u64>,
}

impl MsgWithdrawal {
    pub fn new(
        creator: impl Into<String>,
        receiver: impl Into<String>,
        token_a: impl Into<String>,
        token_b: impl Into<String>,
        shares_to_remove: Vec<String>,
        tick_indexes_a_to_b: Vec<i64>,
        fees: Vec<u64>,
    ) -> Self {
        Self {
            creator: creator.into(),
            receiver: receiver.into(),
            token_a: token_a.into(),
            token_b: token_b.into(),
            shares_to_remove,
            tick_indexes_a_to_b,
            fees,
        }
    }
}

impl StargateMsg for MsgWithdrawal {
    fn url() -> &'static str {
        "/neutron.dex.MsgWithdrawal"
    }

    fn to_buf(&self) -> Vec<u8> {
        Anybuf::new()
            .append_string(1, &self.creator)
            .append_string(2, &self.receiver)
            .append_string(3, &self.token_a)
            .append_string(4, &self.token_b)
            .append_repeated_string(5, &self.shares_to_remove)
            .append_repeated_int64(6, &self.tick_indexes_a_to_b)
            .append_repeated_uint64(7, &self.fees)
            .into_vec()
    }
}

pub struct MsgPlaceLimitOrder {
    pub creator: String,
    pub receiver: String,
    pub token_in: String,
    pub token_out: String,
    pub tick_index_in_to_out: i64,
    pub amount_in: String,
    pub order_type: LimitOrderType,
    pub expiration_time: Option<Timestamp>,
    pub max_amount_out: Option<String>,
}

impl MsgPlaceLimitOrder {
    pub fn new(
        creator: impl Into<String>,
        receiver: impl Into<String>,
        token_in: impl Into<String>,
        token_out: impl Into<String>,
        tick_index_in_to_out: i64,
        amount_in: impl Into<String>,
        order_type: LimitOrderType,
        expiration_time: Option<Timestamp>,
        max_amount_out: Option<impl Into<String>>,
    ) -> Self {
        Self {
            creator: creator.into(),
            receiver: receiver.into(),
            token_in: token_in.into(),
            token_out: token_out.into(),
            tick_index_in_to_out,
            amount_in: amount_in.into(),
            order_type,
            expiration_time,
            max_amount_out: max_amount_out.map(Into::into),
        }
    }
}

impl StargateMsg for MsgPlaceLimitOrder {
    fn url() -> &'static str {
        "/neutron.dex.MsgPlaceLimitOrder"
    }

    fn to_buf(&self) -> Vec<u8> {
        Anybuf::new()
            .append_string(1, &self.creator)
            .append_string(2, &self.receiver)
            .append_string(3, &self.token_in)
            .append_string(4, &self.token_out)
            .append_int64(5, self.tick_index_in_to_out)
            .append_string(6, &self.amount_in)
            .append_int32(7, self.order_type as i32)
            .append_message(
                8,
                &self
                    .expiration_time
                    .as_ref()
                    .map(Timestamp::to_anybuf)
                    .unwrap_or_default(),
            )
            .append_string(9, self.max_amount_out.as_deref().unwrap_or_default())
            .into_vec()
    }
}

pub struct MsgMultiHopSwap {
    pub creator: String,
    pub receiver: String,
    pub routes: Vec<MultiHopRoute>,
    pub amount_in: String,
    pub exit_limit_price: String,
    pub pick_best_route: bool,
}

impl MsgMultiHopSwap {
    pub fn new(
        creator: impl Into<String>,
        receiver: impl Into<String>,
        routes: Vec<MultiHopRoute>,
        amount_in: impl Into<String>,
        exit_limit_price: impl Into<String>,
        pick_best_route: bool,
    ) -> Self {
        Self {
            creator: creator.into(),
            receiver: receiver.into(),
            routes,
            amount_in: amount_in.into(),
            exit_limit_price: exit_limit_price.into(),
            pick_best_route,
        }
    }
}

impl StargateMsg for MsgMultiHopSwap {
    fn url() -> &'static str {
        "/neutron.dex.MsgMultiHopSwap"
    }

    fn to_buf(&self) -> Vec<u8> {
        let routes_anybuf: Vec<Anybuf> = self.routes.iter().map(MultiHopRoute::to_anybuf).collect();
        Anybuf::new()
            .append_string(1, &self.creator)
            .append_string(2, &self.receiver)
            .append_repeated_message(3, &routes_anybuf)
            .append_string(4, &self.amount_in)
            .append_string(5, &self.exit_limit_price)
            .append_bool(6, self.pick_best_route)
            .into_vec()
    }
}

pub struct MsgUpdateParams {
    pub authority: String,
    pub params: Params,
}

impl MsgUpdateParams {
    pub fn new(authority: impl Into<String>, params: Params) -> Self {
        Self {
            authority: authority.into(),
            params,
        }
    }
}

impl StargateMsg for MsgUpdateParams {
    fn url() -> &'static str {
        "/neutron.dex.MsgUpdateParams"
    }

    fn to_buf(&self) -> Vec<u8> {
        Anybuf::new()
            .append_string(1, &self.authority)
            .append_bytes(2, self.params.to_buf())
            .into_vec()
    }
}

pub struct DepositOptions {
    pub disable_autoswap: bool,
}

impl DepositOptions {
    pub fn new(disable_autoswap: bool) -> Self {
        Self { disable_autoswap }
    }

    pub fn to_anybuf(&self) -> Anybuf {
        Anybuf::new().append_bool(1, self.disable_autoswap)
    }
}

pub struct MultiHopRoute {
    pub hops: Vec<String>,
}

impl MultiHopRoute {
    pub fn new(hops: Vec<String>) -> Self {
        Self { hops }
    }

    pub fn to_anybuf(&self) -> Anybuf {
        Anybuf::new().append_repeated_string(1, &self.hops)
    }
}

#[derive(Copy, Clone)]
#[repr(i32)]
pub enum LimitOrderType {
    GoodTilCancelled = 0,
    FillOrKill = 1,
    ImmediateOrCancel = 2,
    JustInTime = 3,
    GoodTilTime = 4,
}
