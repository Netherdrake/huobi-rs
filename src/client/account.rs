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
        let mut params: BTreeMap<String, String> = BTreeMap::new();
        params.insert("account-id".to_string(), self.account_id.unwrap().to_string());
        params.insert("symbol".to_string(), symbol.to_string());
        params.insert("size".to_string(), "100".to_string());
        let data = self.get_signed("/v1/order/openOrders", params)?;
        let response: APIResponse<Vec<Order>> = from_str(data.as_str())?;
        Ok(response.data)
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
        // where S: Into<String> + PartialEq
    {

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
        let tmp = format!("{}-{}", order_side.to_lowercase(), classification);
        classification = &tmp;

        let mut body:BTreeMap<String, String> = BTreeMap::new();
        let mut params: BTreeMap<String, String> = BTreeMap::new();

        let symbol: String = symbol.to_string().to_lowercase();
        body.insert("account-id".to_string(), self.account_id.unwrap().to_string());
        body.insert("symbol".into(), symbol);
        body.insert("type".into(), classification.into());
        body.insert("amount".into(), qty.to_string());
        if order_type != "MAKER" {
            body.insert("price".into(), price.to_string());
        }
        body.insert("source".into(), "api".into());

        // if let Some(client_id) = client_order_id.into() {
        //     body.insert("client_order_id".into(), format!("{}", client_id));
        // }

        let data = self.post_signed("/v1/order/orders/place", params, &body)?;
        let order: OrderReceipt = from_str(data.as_str())?;

        Ok(order)
    }

    pub fn cancel_order(
        &self,
        order_id: &str,
    ) -> APIResult<OrderReceipt>
        // where S: Into<String> + PartialEq
    {
        let mut body:BTreeMap<String, String> = BTreeMap::new();
        let mut params: BTreeMap<String, String> = BTreeMap::new();

        // body.insert("account-id".to_string(), self.account_id.unwrap().to_string());
        // body.insert("order-id".into(), order_id.into());

        let data = self.post_signed(
            &format!("/v1/order/orders/{}/submitcancel", order_id),
            params,
            &body)?;
        let order: OrderReceipt = from_str(data.as_str())?;

        Ok(order)
    }




}
