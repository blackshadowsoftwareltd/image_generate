use crate::structs::menu::{Menu, MenuItem};

pub fn menu_list() -> Vec<Menu> {
    let mut menus = Vec::new();
    let menu1 = Menu {
        id: "1".to_string(),
        name: "Breakfast".to_string(),
        items: vec![
            MenuItem {
                id: "1".to_string(),
                name: "Eggs".to_string(),
                price: 5.99,
                description: "Two eggs any style".to_string(),
            },
            MenuItem {
                id: "2".to_string(),
                name: "Pancakes".to_string(),
                price: 6.99,
                description: "Three pancakes with syrup".to_string(),
            },
            MenuItem {
                id: "3".to_string(),
                name: "Oatmeal".to_string(),
                price: 4.99,
                description: "Bowl of oatmeal".to_string(),
            },
        ],
    };
    let menu2 = Menu {
        id: "2".to_string(),
        name: "Lunch".to_string(),
        items: vec![
            MenuItem {
                id: "4".to_string(),
                name: "Burger".to_string(),
                price: 7.99,
                description: "Cheeseburger with fries".to_string(),
            },
            MenuItem {
                id: "5".to_string(),
                name: "Salad".to_string(),
                price: 8.99,
                description: "Garden salad with dressing".to_string(),
            },
            MenuItem {
                id: "6".to_string(),
                name: "Soup".to_string(),
                price: 6.99,
                description: "Bowl of soup".to_string(),
            },
        ],
    };
    menus.push(menu1);
    menus.push(menu2);
    menus
}
