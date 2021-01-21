use super::*;
use serde_json::from_str;

impl Client {

    pub fn accounts(&self) -> APIResult<Vec<Account>> {
        let params: BTreeMap<String, String> = BTreeMap::new();
        let data = self.get_signed("/v1/account/accounts", params)?;
        let response: APIResponse<Vec<Account>> = from_str(data.as_str())?;
        Ok(response.data)
    }

    pub fn balances(&self) -> APIResult<Balance> {
        let params: BTreeMap<String, String> = BTreeMap::new();
        let data = self.get_signed(
            &format!("/v1/account/accounts/{}/balance", self.account_id.unwrap()), params)?;
        let response: APIResponse<Balance> = from_str(data.as_str())?;
        Ok(response.data)
    }

    pub fn open_orders(&self, symbol: &str) -> APIResult<Vec<Order>> {
        let mut params: BTreeMap<String, String> = BTreeMap::new();
        params.insert("account-id".to_string(), self.account_id.unwrap().to_string());
        params.insert("symbol".to_string(), symbol.to_string());
        params.insert("side".to_string(), "both".to_string());
        params.insert("size".to_string(), "100".to_string());
        let data = self.get_signed("/v1/order/openOrders", params)?;
        let response: APIResponse<Vec<Order>> = from_str(data.as_str())?;
        Ok(response.data)
    }

    // place order
    pub fn place_order<S1, S2, S3, S5, S6, S8>(&self, contract_code: S1, client_order_id: S2, price: S3, volume: u32,
        direction: S5, offset: S6, lever_rate: u32, order_price_type: S8) -> APIResult<OrderInfo>
        where S1: Into<String>, S2: Into<Option<u32>>, S3: Into<Option<f64>>, S5: Into<String>, S6: Into<String>,
        S8: Into<String>
    {
        let mut body:BTreeMap<String, String> = BTreeMap::new();
        let params: BTreeMap<String, String> = BTreeMap::new();

        body.insert("contract_code".into(), contract_code.into());
        body.insert("volume".into(), format!("{}", volume));
        body.insert("direction".into(), direction.into());
        body.insert("offset".into(), offset.into());
        body.insert("lever_rate".into(), lever_rate.to_string());
        body.insert("order_price_type".into(), order_price_type.into());

        if let Some(client_id) = client_order_id.into() { body.insert("client_order_id".into(), format!("{}", client_id)); }
        if let Some(p) = price.into() { body.insert("price".into(), format!("{}", p)); }

        println!("body: {:?}", body.clone());

        let data = self.post_signed("/v1/swap_order", params, &body)?;

        let order: OrderInfo = from_str(data.as_str())?;

        Ok(order)

    }

    // place batch order
    pub fn place_orders(&self, orders_data: BatchOrderRequest) -> APIResult<BatchOrder>
    {
        let params: BTreeMap<String, String> = BTreeMap::new();

        let data = self.post_signed("/v1/swap_batchorder", params, &orders_data)?;

        let order: BatchOrder = from_str(data.as_str())?;

        Ok(order)
    } 

    // cancel orders
    pub fn cancel_orders<S1, S2, S3>(&self, order_id: S1, client_order_id: S2, contract_code: S3) -> APIResult<OrderCancelInfo>
        where S1: Into<Option<String>>, S2: Into<Option<String>>, S3: Into<String>
    {
        let params: BTreeMap<String, String> = BTreeMap::new();
        let mut body: BTreeMap<String, String> = BTreeMap::new();

        body.insert("contract_code".into(), contract_code.into());

        if let Some(oid) = order_id.into() {body.insert("order_id".into(), format!("{}", oid)); }
        if let Some(cid) = client_order_id.into() { body.insert("client_order_id".into(), cid); }

        let data = self.post_signed("/v1/swap_cancel", params, &body)?;

        let cancel: OrderCancelInfo = from_str(data.as_str())?;

        Ok(cancel)
    }

    // cancel all orders
    pub fn cancel_all(&self, contract_code: String) -> APIResult<OrderCancelInfo> {
        let params: BTreeMap<String, String> = BTreeMap::new();
        let mut body: BTreeMap<String, String> = BTreeMap::new();

        body.insert("contract_code".to_string(), contract_code);

        let data = self.post_signed("/v1/swap_cancelall", params, &body)?;

        let cancel_all: OrderCancelInfo = from_str(data.as_str())?;

        Ok(cancel_all)
    }

    // get order info
    pub fn get_order_info<S1, S2, S3>(&self, order_id: S1, client_order_id: S2, contract_code: S3) -> APIResult<GOrderInfo>
        where S1: Into<Option<String>>, S2: Into<Option<String>>, S3: Into<String>
    {
        let params: BTreeMap<String, String> = BTreeMap::new();
        let mut body: BTreeMap<String, String> = BTreeMap::new();

        body.insert("contract_code".into(), contract_code.into());
        if let Some(oid) = order_id.into() { body.insert("order_id".into(), format!("{}", oid));}
        if let Some(cid) = client_order_id.into() { body.insert("client_order_id".into(), format!("{}", cid));}

        let data = self.post_signed("/v1/swap_order_info", params, &body)?;

        let order_info: GOrderInfo = from_str(data.as_str())?;

        Ok(order_info)
    }

    // get order detail information
    pub fn get_order_detail<S1, S2, S3, S4>(&self, contract_code: S1, order_id: u64, created_at: S2, order_type: u32,
        page_index: S3, page_size: S4) -> APIResult<OrderDetailInfo> 
        where S1: Into<String>, S2: Into<Option<u64>>, S3: Into<Option<u32>>, S4: Into<Option<u32>>
    {
        let params: BTreeMap<String, String> = BTreeMap::new();
        let mut body: BTreeMap<String, String> = BTreeMap::new();

        body.insert("contract_code".into(), contract_code.into());
        body.insert("order_id".into(), format!("{}", order_id));
        body.insert("order_type".to_string(), format!("{}", order_type));
        if let Some(ct) = created_at.into() { body.insert("created_at".into(),format!("{}", ct));}
        if let Some(offset) = page_index.into() { body.insert("page_index".into(), format!("{}", offset));}
        if let Some(limit) = page_size.into() { body.insert("page_size".into(), format!("{}", limit));}

        let data = self.post_signed("/v1/swap_order_detail", params, &body)?;

        let order_detail: OrderDetailInfo = from_str(data.as_str())?;

        Ok(order_detail)

    }
    // get open orders
    pub fn get_open_orders<S1, S2, S3>(&self, contract_code: S1, page_index: S2, page_size: S3) -> APIResult<OpenOrders>
        where S1: Into<String>, S2: Into<Option<u32>>, S3: Into<Option<u32>>
    {
        let params: BTreeMap<String, String> = BTreeMap::new();
        let mut body: BTreeMap<String, String> = BTreeMap::new();

        body.insert("contract_code".into(), contract_code.into());
        if let Some(offset) = page_index.into() { body.insert("page_index".into(), format!("{}", offset));}
        if let Some(limit) = page_size.into() { body.insert("page_size".into(), format!("{}", limit));}

        let data = self.post_signed("/v1/swap_openorders", params, &body)?;

        let open_orders: OpenOrders = from_str(data.as_str())?;

        Ok(open_orders)

    }

    // get history orders
    pub fn get_his_orders<S1, S2, S3>(&self, contract_code: S1, trade_type: u32, r_type: u32, status: String, create_date: u32, page_index: S2, page_size: S3)
        -> APIResult<HisOrders>
        where S1: Into<String>, S2: Into<Option<u32>>, S3: Into<Option<u32>>
    {
        let params: BTreeMap<String, String> = BTreeMap::new();
        let mut body: BTreeMap<String, String> = BTreeMap::new();

        body.insert("contract_code".into(), contract_code.into());
        body.insert("trade_type".into(), format!("{}", trade_type));
        body.insert("type".into(), format!("{}", r_type));
        body.insert("status".into(), format!("{}", status));
        body.insert("create_date".into(), format!("{}", create_date));
        if let Some(offset) = page_index.into() { body.insert("page_index".into(), format!("{}", offset));}
        if let Some(limit) = page_size.into() { body.insert("page_size".into(), format!("{}", limit));}

        let data = self.post_signed("/v1/swap_hisorders", params, &body)?;

        let his_orders: HisOrders = from_str(data.as_str())?;

        Ok(his_orders)

    }

    // get match results
    pub fn get_match_results<S1, S2, S3>(&self, contract_code: S1, trade_type: u32, days: u32, page_index: S2, page_size: S3)
        -> APIResult<MatchResults>
        where S1: Into<String>, S2: Into<Option<u32>>, S3: Into<Option<u32>>
    {
        let params: BTreeMap<String, String> = BTreeMap::new();
        let mut body: BTreeMap<String, String> = BTreeMap::new();

        body.insert("contract_code".into(), contract_code.into());
        body.insert("trade_type".into(), format!("{}", trade_type));
        body.insert("create_date".into(), format!("{}", days));
        if let Some(offset) = page_index.into() { body.insert("page_index".into(), format!("{}", offset));}
        if let Some(limit) = page_size.into() { body.insert("page_size".into(), format!("{}", limit));}

        let data = self.post_signed("/v1/swap_matchresults", params, &body)?;

        let match_results: MatchResults = from_str(data.as_str())?;

        Ok(match_results)
    }



    pub fn spot_account_transfer<S1, S2, S3>(&self, from: S1, to: S2, currency: S3, amount: f64) -> APIResult<AccountTransferResult>
        where S1: Into<String>, S2: Into<String>, S3: Into<String>
    {
        let params: BTreeMap<String, String> = BTreeMap::new();
        let mut body: BTreeMap<String, String> = BTreeMap::new();

        body.insert("from".into(), from.into());
        body.insert("to".into(), to.into());
        body.insert("currency".into(), currency.into());
        body.insert("amount".into(), format!("{}", amount));

        let data = self.post_signed("/v2/account/transfer", params, &body)?;

        let account_transfer: AccountTransferResult = from_str(data.as_str())?;

        Ok(account_transfer)
    }


}
