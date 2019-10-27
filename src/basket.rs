use std::fmt::Display;
use std::error::Error;
use json::JsonValue;
use crate::params::Params;
use crate::common::*;

#[derive(Clone)]
pub struct Basket {
    items: Vec<i32>
}

impl<'a> Basket {
    pub fn new(params: &'a Params) -> Basket {
        Basket{
            items: vec![ 0; params.get_num_basket_categories() ],
        }
    }

    pub fn add(&mut self, category: usize, quantity: i32) {
        self.items[category] += quantity;
    }

    pub fn total_shortfall(&self, actual_basket: &Basket) -> i32 {
        let mut result = 0;
        for category in 0..self.items.len() {
            let shortfall = self.shortfall(actual_basket, category);
            if shortfall > 0 {
                result += shortfall;
            }
        }
        result
    }

    pub fn shortfall(&self, actual_basket: &Basket, category: usize) -> i32 {
        let expected = self.items[category];
        let actual = actual_basket.items[category];
        // let shortfall = expected - actual;
        return (expected - actual).abs()
        // if shortfall > 0 {
        //     shortfall
        // } else {
        //     0
        // }
    }

    pub fn try_parse(value: &JsonValue, params: &mut Params) -> Result<Self, Box<dyn Error>> {
        let value_obj = as_object(value)?;
        let mut basket = Basket::new(params);
        for item in value_obj.iter() {
            let category_id = params.get_basket_category_id(&item.0);
            let quantity = as_int(item.1)?;
            basket.add(category_id, quantity);
        };
        Ok(basket)
    }
}

pub struct BasketDisplay<'a> {
    pub basket: &'a Basket,
    pub params: &'a Params,
}

impl<'a> Display for BasketDisplay<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        for category in 0..self.basket.items.len() {
            let category_name = self.params.get_basket_category_name(category);
            let quantity = self.basket.items[category];
            writeln!(f, "{:<4} x {}", quantity, category_name)?;
        }

        Ok(())
    }
}

#[cfg(test)]
#[test]
fn try_parse() {
    let js = json::parse(r#"
{
    "tomato": 30,
    "greens": 20
}"#).expect("test is wrong");

    let mut params = Params{
        varieties: vec![],
        beds: vec![],
        baskets: vec![],
        basket_category_names: vec![ String::from("tomato"), String::from("greens") ]
    };

    let basket = Basket::try_parse(&js, &mut params).expect("Parse failed");

    let tomato_id = params.get_basket_category_id("tomato");
    let greens_id = params.get_basket_category_id("greens");

    assert_eq!(basket.items[tomato_id], 30);
    assert_eq!(basket.items[greens_id], 20);
}

