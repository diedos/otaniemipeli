use crate::beverage::Beverage;

pub struct Place {
    pub name: String,
    pub beverages: Vec<Beverage>,
    pub refills: Vec<Beverage>,
}
