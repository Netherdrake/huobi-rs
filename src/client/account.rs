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

    pub fn get_open_orders(&self, symbol: &str) -> APIResult<Vec<Order>> {
        let symbol: String = String::from(symbol).to_lowercase();
        let mut params: BTreeMap<String, String> = BTreeMap::new();
        params.insert("account-id".to_string(), self.account_id.unwrap().to_string());
        params.insert("symbol".to_string(), symbol.to_string());
        params.insert("size".to_string(), "100".to_string());
        let data = self.get_signed("/v1/order/openOrders", params)?;
        let response: APIResponse<Vec<Order>> = from_str(data.as_str())?;
        Ok(response.data)
    }

    pub fn construct_order_type(
        order_side: &str,
        order_type: &str,
        execution_type: &str,
    ) -> String {
        let mut classification: &str = "";
        if order_type == "MARKET" {
            classification = "market";
        }
        if order_type == "LIMIT" {
            if execution_type == "IOC" {
                classification = "ioc";
            } else {
                classification = "limit";
            }
        }
        if order_type == "LIMIT_MAKER" {
            classification = "limit-maker";
        }
        format!("{}-{}", order_side.to_lowercase(), classification)
    }

    pub fn place_order(
        &self,
        symbol: &str,
        qty: f64,
        price: f64,
        order_side: &str,
        order_type: &str,
        execution_type: &str,
    ) -> APIResult<OrderReceipt>
    {
        let symbol: String = String::from(symbol).to_lowercase();
        let type_ = &Client::construct_order_type(order_side, order_type, execution_type);

        let params: BTreeMap<String, String> = BTreeMap::new();
        let mut body: BTreeMap<String, String> = BTreeMap::new();

        let symbol: String = symbol.to_string().to_lowercase();
        body.insert("account-id".to_string(), self.account_id.unwrap().to_string());
        body.insert("symbol".into(), symbol);
        body.insert("type".into(), type_.into());
        body.insert("amount".into(), qty.to_string());
        if order_type != "MAKER" {
            body.insert("price".into(), price.to_string());
        }
        body.insert("source".into(), "api".into());

        // if let Some(client_id) = client_order_id.into() {
        //     body.insert("client_order_id".into(), format!("{}", client_id));
        // }

        let data = self.post_signed("/v1/order/orders/place", params, &body)?;
        let receipt: OrderReceipt = from_str(data.as_str())?;

        Ok(receipt)
    }

    // pub fn place_order(
    //     &self,
    //     symbol: &str,
    //     qty: f64,
    //     price: f64,
    //     order_side: &str,
    //     order_type: &str,
    //     execution_type: &str,
    // ) -> APIResult<NewOrder>
    //     // where S: Into<String> + PartialEq
    // {
    //     let symbol: String = String::from(symbol).to_lowercase();
    //     let type_ = &Client::construct_order_type(order_side, order_type, execution_type);
    //
    //     let params: BTreeMap<String, String> = BTreeMap::new();
    //     let mut body: BTreeMap<String, String> = BTreeMap::new();
    //
    //     let symbol: String = symbol.to_string().to_lowercase();
    //     body.insert("account-id".to_string(), self.account_id.unwrap().to_string());
    //     body.insert("symbol".into(), symbol);
    //     body.insert("type".into(), type_.into());
    //     body.insert("amount".into(), qty.to_string());
    //     if order_type != "MAKER" {
    //         body.insert("price".into(), price.to_string());
    //     }
    //     body.insert("source".into(), "api".into());
    //
    //     // if let Some(client_id) = client_order_id.into() {
    //     //     body.insert("client_order_id".into(), format!("{}", client_id));
    //     // }
    //
    //     let data = self.post_signed("/v1/order/orders/place", params, &body)?;
    //     let receipt: OrderReceipt = from_str(data.as_str())?;
    //     let order: NewOrder = self.get_order(receipt.order_id.to_string())?;
    //
    //     Ok(order)
    // }

    pub fn get_order<S: Into<String>>(&self, order_id: S) -> APIResult<NewOrder> {
        let uri = format!("/v1/order/orders/{}", order_id.into());
        let params: BTreeMap<String, String> = BTreeMap::new();
        let ret = self.get_signed(&uri, params)?;
        let order: APIResponse<NewOrder> = from_str(&ret)?;
        Ok(order.data)
    }

    pub fn cancel_order(
        &self,
        order_id: &str,
    ) -> APIResult<OrderReceipt>
        // where S: Into<String> + PartialEq
    {
        let body:BTreeMap<String, String> = BTreeMap::new();
        let params: BTreeMap<String, String> = BTreeMap::new();

        let data = self.post_signed(
            &format!("/v1/order/orders/{}/submitcancel", order_id),
            params,
            &body)?;
        let order: OrderReceipt = from_str(data.as_str())?;

        Ok(order)
    }

    pub fn cancel_all(&self, symbol: &str) -> APIResult<bool> {
        let symbol: String = String::from(symbol).to_lowercase();
        let uri = "/v1/order/orders/batchCancelOpenOrders";
        let params: BTreeMap<String, String> = BTreeMap::new();
        let mut body: BTreeMap<String, String> = BTreeMap::new();
        body.insert("account-id".into(), self.account_id.unwrap().to_string());
        body.insert("symbol".into(), symbol.to_string().to_lowercase());
        let _ret = self.post_signed(uri, params, &body)?;
        Ok(true)
    }


}
