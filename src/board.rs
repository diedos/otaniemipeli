use crate::beverage::Beverage;

pub struct Place {
    pub name: String,
    pub beverages: Option<Vec<Beverage>>,
    pub refilled: bool, // TODO: change to a vec of beverages to allow varying refill amounts (i.e. old kiltis)
}
