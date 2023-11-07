fn main() {
    let mut shopify = Shopify::default();
    let mut ro = RO::default();

    // Receive some inventory into RO
    ro.make_adjustment(10);

    // Send it to shopify
    let applied: i32 = ro.sync_inventory(&mut shopify);
    assert_eq!(applied, 10);

    // Shopify agrees we have 10
    assert_eq!(shopify.available(), 10);

    // Some orders come in via Shopify
    shopify.place_order(1);
    shopify.place_order(1);

    // RO gets a couple orders from another system, or maybe some got lost or whatever
    ro.make_adjustment(-1);
    ro.make_adjustment(-1);

    // Tell shopify we have less inventory. We would previously have sent 8, but now we send -2
    let applied = ro.sync_inventory(&mut shopify);
    assert_eq!(applied, -2);

    // Shopify does the right thing, but we haven't heard about the orders yet
    assert_eq!(ro.available(), 8);
    assert_eq!(shopify.available(), 6);

    // Noww we get the Shopify orders
    ro.pull_orders(&mut shopify);

    // And at last, both systems agree we have 6
    assert_eq!(ro.available(), 6);
    assert_eq!(shopify.available(), 6);

    // Sync again, and we NoOp
    let applied = ro.sync_inventory(&mut shopify);
    assert_eq!(applied, 0);
}

struct Adjustment {
    // time: u32,
    quantity: i32,
    // status: Status,
}
struct Order {
    // time: u32,
    quantity: i32,
}

enum Status {
    Applied,
    Indeterminite,
}

#[derive(Default)]
struct RO {
    adjustments: Vec<Adjustment>,
    orders: Vec<Order>,
    inv_model: InvModel,
}

impl RO {
    fn make_adjustment(&mut self, quantity: i32) {
        self.adjustments.push(Adjustment { quantity });
    }
    fn available(&self) -> i32 {
        let mut quantity = 0;
        for adjustment in &self.adjustments {
            quantity += adjustment.quantity;
        }
        quantity
    }
    fn sync_inventory(&mut self, shopify: &mut Shopify) -> i32 {
        let delta = self.available() - self.inv_model.available();

        if delta != 0 {
            self.inv_model.make_adjustment(delta);
            shopify.make_adjustment(delta);
        }
        delta
    }
    fn pull_orders(&mut self, shopify: &Shopify) {
        for order in &shopify.orders {
            self.orders.push(Order {
                // time: order.time,
                quantity: order.quantity,
            });
            self.make_adjustment(-order.quantity);
            self.inv_model.make_adjustment(-order.quantity);
        }
    }
    fn reconcile_shopify(&mut self, shopify: &mut Shopify) {
        let delta = self.available() - shopify.available();
        if delta > 0 {
            // be conservative and wait a while to ensure to ensure no orders have come in
        }
        if delta < 0 {
            // Agressively reduce shopify inventory
        }
    }
}

#[derive(Default)]
struct InvModel {
    adjustments: Vec<Adjustment>,
}

impl InvModel {
    fn available(&self) -> i32 {
        let mut quantity = 0;
        for adjustment in &self.adjustments {
            quantity += adjustment.quantity;
        }
        quantity
    }
    fn make_adjustment(&mut self, quantity: i32) {
        self.adjustments.push(Adjustment { quantity });
    }
}

#[derive(Default)]
struct Shopify {
    adjustments: Vec<Adjustment>,
    orders: Vec<Order>,
}

impl Shopify {
    fn available(&self) -> i32 {
        let mut quantity = 0;
        for adjustment in &self.adjustments {
            quantity += adjustment.quantity;
        }
        quantity
    }
    fn make_adjustment(&mut self, quantity: i32) {
        self.adjustments.push(Adjustment { quantity });
    }
    fn place_order(&mut self, quantity: i32) {
        self.orders.push(Order { quantity });
        self.make_adjustment(-quantity)
    }
}
