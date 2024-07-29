#[derive(Clone, Debug)]
pub struct Beverage {
    pub name: String,
    pub flavor: BeverageType,
    pub size_ml: u32,
    pub abv: f32,
}

#[derive(Clone, Debug)]
pub enum BeverageType {
    Beer,
    Wine,
    Soda,
    Water,
    Liquor,
    Slime,
}
