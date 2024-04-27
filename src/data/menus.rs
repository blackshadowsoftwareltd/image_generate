use crate::structs::menu::{Category, MenuCategory, Product};

pub fn menu_category() -> MenuCategory {
    let mut menus = Vec::new();
    let menu1 = Category {
        id: Some("1".to_string()),
        name: Some("Breakfast".to_string()),
        products: Some(vec![
            Product {
                id: Some("1".to_string()),
                name: Some("Eggs".to_string()),
                price: Some(865376.99),
            },
            Product {
                id: Some("2".to_string()),
                name: Some("Pancakes".to_string()),
                price: Some(46218.99),
            },
            Product {
                id: Some("3".to_string()),
                name: Some("Oatmeal".to_string()),
                price: Some(7614.99),
            },
        ]),
    };
    let menu2 = Category {
        id: Some("2".to_string()),
        name: Some("Lunch".to_string()),
        products: Some(vec![
            Product {
                id: Some("4".to_string()),
                name: Some("Burger".to_string()),
                price: Some(790.99),
            },
            Product {
                id: Some("5".to_string()),
                name: Some("Salad".to_string()),
                price: Some(98.99),
            },
            Product {
                id: Some("6".to_string()),
                name: Some("Soup".to_string()),
                price: Some(6.99),
            },
        ]),
    };
    menus.push(menu1);
    menus.push(menu2);

    MenuCategory {
        background: Some("assets/pxfuel.jpg".to_string()),
        width: Some(320.33),
        height: Some(660.33),
        currency: Some("$".to_string()),
        categories: Some(menus),
    }
}
