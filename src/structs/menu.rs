#[derive(Debug)]
pub struct Menu {
    pub id: String,
    pub name: String,
    pub items: Vec<MenuItem>,
}

#[derive(Debug)]
pub struct MenuItem {
    pub id: String,
    pub name: String,
    pub price: f32,
    pub description: String,
}
