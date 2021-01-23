use super::*;
//use crate::error::*;
//use crate::models::*;
use serde_json::from_str;

impl Client {

    // Get Orderbook
    pub fn get_orderbook<S: Into<String>>(&self, symbol: S, orderbook_type: S) -> APIResult<OrderBook>
    {
        let symbol: String = symbol.into().to_lowercase();
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("type".into(), orderbook_type.into());

        let data: String = self.get("/market/depth", &parameters)?;

        let info: OrderBook = from_str(data.as_str())?;

        Ok(info)
    }

    // Get Kline
    pub fn get_klines<S1, S2, S3, S4, S5>(&self, symbol: S1, interval: S2, limit: S3, start_time: S4, end_time: S5) -> APIResult<Klines>
    where S1: Into<String>, S2: Into<String>, S3: Into<Option<u32>>, S4: Into<Option<u64>>, S5: Into<Option<u64>>
    {
        let symbol: String = symbol.into().to_lowercase();
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("period".into(), interval.into());

        if let Some(lt) = limit.into() { parameters.insert("size".into(), format!{"{}", lt});}
        if let Some(st) = start_time.into() { parameters.insert("from".into(), format!("{}", st));}
        if let Some(et) = end_time.into() { parameters.insert("to".into(), format!("{}", et));}

        let data: String = self.get("/market/history/kline", &parameters)?;
        let klines: Klines = from_str(data.as_str())?;

        Ok(klines)
    }

    // Get Merged data
    pub fn get_market_merged<S1>(&self, symbol: S1) -> APIResult<MergedInfo>
    where S1: Into<String>
    {
        let symbol: String = symbol.into().to_lowercase();
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("symbol".into(), symbol.into());

        let  data: String = self.get("/market/detail/merged", &parameters)?;

        let info: MergedInfo = from_str(data.as_str())?;

        Ok(info)
    }




}
