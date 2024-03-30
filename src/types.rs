pub struct Production {
    pub symbol: String,
    pub produces: Vec<ProductionOption>,
}

pub struct ProductionOption {
    pub production: Option<String>,
    pub production1: Option<String>,
}

impl Production {
    pub fn goes_to_terminal(&self, sym: &str) -> bool {
        for prod_opt in &self.produces {
            if prod_opt.production == Some(sym.to_string())
                && prod_opt.production1 == None
            {
                return true;
            }
        }
        return false;
    }

    pub fn goes_concatted(&self, sym: &str, sym1: &str) -> bool {
        for prod_opt in &self.produces {
            if prod_opt.production == Some(sym.to_string())
                && prod_opt.production1 == Some(sym1.to_string())
            {
                return true;
            }
        }
        return false;
    }
}
