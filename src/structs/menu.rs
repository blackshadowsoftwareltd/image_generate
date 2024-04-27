#[derive(Debug, PartialEq)]
pub struct MenuCategory {
    pub background: Option<String>,
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub currency: Option<String>,
    pub categories: Option<Vec<Category>>,
}

#[derive(Debug, PartialEq)]
pub struct Category {
    pub id: Option<String>,
    pub name: Option<String>,
    pub products: Option<Vec<Product>>,
}

#[derive(Debug, PartialEq)]
pub struct Product {
    pub id: Option<String>,
    pub name: Option<String>,
    pub price: Option<f64>,
}
