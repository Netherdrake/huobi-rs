#![allow(dead_code)]
#![allow(unused_variables)]

use serde::de::{self, Unexpected, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt::{self, Display};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct APIResponse<R> {
    pub status: String,
    pub data: R,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct APIErrorResponse<R> {
    pub status: Option<String>,
    #[serde(rename = "err-code")]
    pub err_code: Option<String>,
    #[serde(rename = "err-msg")]
    pub err_msg: Option<String>,
    pub data: Option<R>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Balance {
    pub id: u32,
    #[serde(rename = "type")]
    pub account_type: String,
    pub state: String,
    pub list: Vec<Asset>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Asset {
    pub currency: String,
    #[serde(rename = "type")]
    pub trade_type: String,
    #[serde(deserialize_with = "string_as_f64")]
    pub balance: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    pub state: String,

    #[serde(rename = "id")]
    pub account_id: u32,

    #[serde(rename = "type")]
    pub account_type: String,

    #[serde(rename = "subtype")]
    pub account_subtype: String,
}

pub type Currency = Vec<String>;
pub type Timestamp = u64;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pair {
    #[serde(rename = "base-currency")]
    pub base_currency: String, // "eth", "btc"

    #[serde(rename = "quote-currency")]
    pub quote_currency: String, // "eth", "btc"

    #[serde(rename = "price-precision")]
    pub price_precision: u32,

    #[serde(rename = "amount-precision")]
    pub amount_precision: u32,

    #[serde(rename = "symbol-partition")]
    pub symbol_partition: String,

    #[serde(rename = "symbol")] // "edubtc", "linkusdt"
    pub symbol: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Kline {
    pub id: u32,
    pub amount: f64,
    pub count: u32,
    pub open: f64,
    pub close: f64,
    pub low: f64,
    pub high: f64,
    pub vol: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ticker {
    pub amount: f64,
    pub count: u32,
    pub open: f64,
    pub close: f64,
    pub low: f64,
    pub high: f64,
    pub vol: f64,
    pub symbol: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderReceipt {
    #[serde(rename = "data")]
    #[serde(deserialize_with = "string_as_u64")]
    pub order_id: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Order {
    pub id: u64,
    pub symbol: String,
    pub source: String,
    pub state: String,

    #[serde(rename = "account-id")]
    pub account_id: u64,

    #[serde(deserialize_with = "string_as_f64")]
    pub amount: f64,

    #[serde(deserialize_with = "string_as_f64")]
    pub price: f64,

    #[serde(rename = "created-at")]
    pub created_at: u64,

    #[serde(rename = "type")]
    pub order_type: String,

    #[serde(rename = "filled-amount")]
    #[serde(deserialize_with = "string_as_f64")]
    pub filled_amount: f64,

    #[serde(rename = "filled-cash-amount")]
    #[serde(deserialize_with = "string_as_f64")]
    pub filled_cash_amount: f64,

    #[serde(rename = "filled-fees")]
    #[serde(deserialize_with = "string_as_f64")]
    pub filled_fees: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewOrder {
    pub id: u64,
    pub symbol: String,
    pub source: String,
    pub state: String,

    #[serde(rename = "account-id")]
    pub account_id: u64,

    #[serde(deserialize_with = "string_as_f64")]
    pub amount: f64,

    #[serde(deserialize_with = "string_as_f64")]
    pub price: f64,

    #[serde(rename = "created-at")]
    pub created_at: u64,

    #[serde(rename = "type")]
    pub order_type: String,

    #[serde(rename = "field-amount")]
    #[serde(deserialize_with = "string_as_f64")]
    pub field_amount: f64,

    #[serde(rename = "field-cash-amount")]
    #[serde(deserialize_with = "string_as_f64")]
    pub field_cash_amount: f64,

    #[serde(rename = "field-fees")]
    #[serde(deserialize_with = "string_as_f64")]
    pub field_fees: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContractInfo {
    pub status: String,
    pub data: Vec<Symbol>,
    pub ts: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Symbol {
    pub symbol: String,
    pub contract_code: String,
    pub contract_size: f64,
    pub price_tick: f64,
    pub settlement_date: String,
    pub create_date: String,
    pub contract_status: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountInfo {
    pub status: Option<String>,
    pub data: Vec<Account>,
    pub ts: u64,
    pub op: Option<String>,
    pub topic: Option<String>,
    pub event: Option<String>,
}

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct Account {
//     pub symbol: String,
//     pub contract_code: String,
//     pub margin_balance: f64,
//     pub margin_static: f64,
//     pub margin_position: f64,
//     pub margin_frozen: f64,
//     pub margin_available: f64,
//     pub profit_real: f64,
//     pub profit_unreal: f64,
//     pub risk_rate: Option<f64>,
//     pub liquidation_price: Option<f64>,
//     pub withdraw_available: f64,
//     pub lever_rate: f64,
//     pub adjust_factor: f64,
//     pub ts: Option<u64>,
//     pub event: Option<u64>,
// }


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PositionInfo {
    pub status: String,
    pub data: Vec<Position>,
    pub ts: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Position {
    pub symbol: String,
    pub contract_code: String,
    pub volume: f64,
    pub available: f64,
    pub frozen: f64,
    pub cost_open: f64,
    pub cost_hold: f64,
    pub profit_unreal: f64,
    pub profit_rate: f64,
    pub profit: f64,
    pub position_margin: f64,
    pub lever_rate: u32,
    pub direction: String,
    pub last_price: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GOrderInfo {
    pub status: String,
    pub data: Vec<OrderItem>,
    pub ts: u64,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderItem {
    pub symbol: String,
    pub contract_code: String,
    pub volume: f64,
    pub price: f64,
    pub order_price_type: String,
    pub direction: String,
    pub offset: String,
    pub lever_rate: u32,
    pub order_id: u64,
    pub order_id_str: String,
    pub client_order_id: u64,
    pub created_at: u64,
    pub trade_volume: u32,
    pub trade_turnover: f64,
    pub fee: f64,
    pub fee_asset: String,
    pub trade_avg_price: Option<f64>,
    pub margin_frozen: f64,
    pub profit: f64,
    pub status: u32,
    pub order_type: u32,
    pub order_source: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderDetailInfo {
    pub status: String,
    pub data: OrderDetail,
    pub ts: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderDetail {
    pub symbol: String,
    pub contract_code: String,
    pub lever_rate: u32,
    pub direction: String,
    pub offset: String,
    pub volume: f64,
    pub price: f64,
    pub created_at: u64,
    pub canceled_at: u64,
    pub order_source: String,
    pub order_price_type: String,
    pub margin_frozen: f64,
    pub profit: f64,
    pub total_page: u32,
    pub current_page: u32,
    pub total_size: u32,
    pub instrument_price: f64,
    pub final_interest: f64,
    pub adjust_value: f64,
    pub fee: f64,
    pub fee_asset: String,
    pub liquidation_type: String,
    pub trades: Vec<TradeItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TradeItem {
    pub trade_id: u64,
    pub id: String,
    pub trade_price: f64,
    pub trade_volume: f64,
    pub trade_fee: f64,
    pub fee_asset: String,
    pub role: String,
    pub created_at: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderInfo {
    pub status: String,
    pub data: Order,
    pub ts: u64,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderBook {
    pub ch: String,
    pub status: Option<String>,
    pub tick: Tick,
    pub ts: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tick {
    pub bids: Vec<Bids>,
    pub asks: Vec<Asks>,
    pub mrid: Option<u64>,
    pub id: Option<u32>,
    pub ts: Option<u64>,
    pub version: Option<u64>,
    pub ch: Option<String>,
    pub event: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bids {
    pub price: f64,
    pub qty: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Asks {
    pub price: f64,
    pub qty: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Klines {
    pub ch: String,
    pub data: Option<Vec<Kline>>,
    pub tick: Option<Kline>,
    pub status: Option<String>,
    pub ts: u64,
}

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct Kline {
//     #[serde(rename = "id")]
//     pub timestamp: u64,
//     #[serde(rename = "vol")]
//     pub volume: f64,
//     pub count: f64,
//     pub open: f64,
//     pub close: f64,
//     pub low: f64,
//     pub high: f64,
//     pub amount: f64,
//     pub mrid: Option<u64>,
// }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Trade {
    pub ch: String,
    pub ts: u64,
    pub tick: TradeDetail,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TradeDetail {
    pub id: u64,
    pub ts: u64,
    pub data: Vec<TradeDetailItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TradeDetailItem {
    pub amount: u32,
    pub ts: u64,
    pub id: u64,
    pub price: f64,
    pub direction: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiTradeStatus {
    pub status: String,
    pub ts: u64,
    pub data: ApiTradeStatusItem,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiTradeStatusItem {
    pub is_disable: u32,
    pub order_price_types: String,
    pub disable_reason: String,
    pub disable_interval: u64,
    pub recovery_time: u64,
    #[serde(rename = "COR")]
    pub cor: COR,
    #[serde(rename = "TDN")]
    pub tdn: TDN,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct COR {
    pub orders_threshold: u64,
    pub orders: u64,
    pub invalid_cancel_orders: u64,
    pub cancel_ratio_threshold: f64,
    pub cancel_ratio: f64,
    pub is_trigger: u32,
    pub is_active: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TDN {
    pub disables_threshold: u64,
    pub disables: u64,
    pub is_trigger: u32,
    pub is_active: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderSubs {
    pub op: String,
    pub topic: String,
    pub ts: u64,
    pub symbol: String,
    pub contract_code: String,
    pub volume: u32,
    pub price: f64,
    pub order_price_type: String,
    pub direction: String,
    pub offset: String,
    pub status: u32,
    pub lever_rate: u32,
    pub order_id: u64,
    pub order_id_str: String,
    pub client_order_id: Option<u64>,
    pub order_source: String,
    pub order_type: u32,
    pub created_at: u64,
    pub trade_volume: u32,
    pub trade_turnover: f64,
    pub fee: f64,
    pub trade_avg_price: f64,
    pub margin_frozen: f64,
    pub profit: f64,
    pub liquidation_type: String,
    pub trade: Vec<TradeSubItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TradeSubItem {
    pub trade_id: u64,
    pub id: String,
    pub trade_volume: u32,
    pub trade_price: f64,
    pub trade_fee: f64,
    pub fee_asset: String,
    pub trade_turnover: f64,
    pub created_at: u64,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PositionSubs
{
    pub op: String,
    pub topic: String,
    pub ts: u64,
    pub event: String,
    pub data: Vec<Position>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LiquidationSubs{
    pub op: String,
    pub topic: String,
    pub ts: u64,
    pub data: Vec<LiquidationItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LiquidationItem {
    pub symbol: String,
    pub contract_code: String,
    pub direction: String,
    pub offset: String,
    pub volume: f64,
    pub price: f64,
    pub created_at: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FundingRateSubs {
    pub op: String,
    pub topic: String,
    pub ts: u64,
    pub data: Vec<FundingRateItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FundingRateItem {
    pub symbol: String,
    pub contract_code: String,
    pub fee_asset: String,
    pub funding_time: String,
    pub funding_rate: String,
    pub estimated_rate: Option<String>,
    pub settlement_time: Option<String>,
    pub next_funding_time: Option<String>,
    pub realized_rate: Option<String>,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IndexInfo {
    pub status: String,
    pub data: Vec<IndexInfoItem>,
    pub ts: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IndexInfoItem {
    pub contract_code: String,
    pub index_price: f64,
    pub index_ts: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PriceLimit {
    pub status: String,
    pub data: Vec<PriceLimitItem>,
    pub ts: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PriceLimitItem {
    pub symbol: String,
    pub high_limit: f64,
    pub low_limit: f64,
    pub contract_code: String,
}

#[derive(Debug,Serialize, Deserialize, Clone)]
pub struct OpenInterest {
    pub status: String,
    pub data: Vec<OpenInterestItem>,
    pub ts: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenInterestItem {
    pub symbol: String,
    pub volume: f64,
    pub amount: f64,
    pub contract_code: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MergedInfo {
    pub ch: String,
    pub status: String,
    pub tick: MergedInfoItem,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MergedInfoItem {
    pub id: u64,
    pub vol: String,
    pub count: u32,
    pub open: String,
    pub close: String,
    pub low: String,
    pub high: String,
    pub amount: String,
    pub ask: Vec<f64>,
    pub bid: Vec<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HistoryTrade {
    pub ch: String,
    pub status: String,
    pub ts: u64,
    pub data: Vec<HistoryTradeItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HistoryTradeItem {
    pub data: Vec<TradeDetailItem>,
    pub id: u64,
    pub ts: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RiskInfo {
    pub status: String,
    pub ts: u64,
    pub data: Vec<RiskInfoItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RiskInfoItem {
    pub contract_code: String,
    pub insurance_fund: f64,
    pub estimated_clawback: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InsuranceFund {
    pub status: String,
    pub ts: u64,
    pub data: InsuranceFundItem,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InsuranceFundItem {
    pub symbol: String,
    pub contract_code: String,
    pub tick: Vec<InsuranceFundTick>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InsuranceFundTick {
    pub insurance_fund: f64,
    pub ts: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AdjustFactor {
    pub status: String,
    pub ts: u64,
    pub data: Vec<AdjustFactorItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AdjustFactorItem {
    pub symbol: String,
    pub contract_code: String,
    pub list: Vec<AdjustFactorDetailList>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AdjustFactorDetailList {
    pub lever_rate: u32,
    pub ladders: Vec<AdjustFactorDetail>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AdjustFactorDetail {
    pub min_size: u32,
    pub max_size: Option<u32>,
    pub ladder: u32,
    pub adjust_factor: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HisOpenInterest {
    pub status: String,
    pub ts: u64,
    pub data: HisOpenInterestItem,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HisOpenInterestItem {
    pub symbol: String,
    pub contract_code: String,
    pub tick: Vec<HisOpenInterestTick>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HisOpenInterestTick {
    pub volume: f64,
    pub amount_type: u32,
    pub ts: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EliteAccountRatio {
    pub status: String,
    pub ts: u64,
    pub data: EliteAccountRatioItem,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EliteAccountRatioItem {
    pub symbol: String,
    pub contract_code: String,
    pub list: Vec<EliteAccountRatioList>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EliteAccountRatioList {
    pub buy_ratio: f64,
    pub sell_ratio: f64,
    pub locked_ratio: f64,
    pub ts: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ElitePositionRatio {
    pub status: String,
    pub ts: u64,
    pub data: ElitePositionRatioItem,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ElitePositionRatioItem {
    pub symbol: String,
    pub contract_code: String,
    pub list: Vec<ElitePositionRatioList>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ElitePositionRatioList {
    pub buy_ratio: f64,
    pub sell_ratio: f64,
    pub ts: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiState {
    pub status: String,
    pub ts: u64,
    pub data: Vec<ApiStateItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiStateItem {
    pub symbol: String,
    pub contract_code: String,
    pub open: u32,
    pub close: u32,
    pub cancel: u32,
    pub transfer_in: u32,
    pub transfer_out: u32,
    pub master_transfer_sub: u32,
    pub sub_transfer_master: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FundingRate {
    pub status: String,
    pub ts: u64,
    pub data: FundingRateItem,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HisFundingRate {
    pub status: String,
    pub ts: u64,
    pub data: HisFundingRateList,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HisFundingRateList {
    pub data: Vec<FundingRateItem>,
    pub total_page: u32,
    pub current_page: u32,
    pub total_size: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LiquidationOrdersInfo {
    pub status: String,
    pub data: LiquidationOrders,
    pub ts: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LiquidationOrders {
    pub orders: Vec<LiquidationItem>,
    pub total_page: u32,
    pub current_page: u32,
    pub total_size: u32,
}

//
// serialization methods
//

fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(de::Error::custom)
}


fn string_as_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(U64Visitor)
}

struct U64Visitor;
impl<'de> Visitor<'de> for U64Visitor {
    type Value = u64;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string representation of a f64")
    }
    fn visit_str<E>(self, value: &str) -> Result<u64, E>
    where
        E: de::Error,
    {
        if let Ok(integer) = value.parse::<i32>() {
            Ok(integer as u64)
        } else {
            value.parse::<u64>().map_err(|err| {
                E::invalid_value(Unexpected::Str(value), &"a string representation of a u64")
            })
        }
    }
}


fn string_as_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(F64Visitor)
}

struct F64Visitor;
impl<'de> Visitor<'de> for F64Visitor {
    type Value = f64;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string representation of a f64")
    }
    fn visit_str<E>(self, value: &str) -> Result<f64, E>
    where
        E: de::Error,
    {
        if let Ok(integer) = value.parse::<i32>() {
            Ok(integer as f64)
        } else {
            value.parse::<f64>().map_err(|err| {
                E::invalid_value(Unexpected::Str(value), &"a string representation of a f64")
            })
        }
    }
}

