use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Item {
    pub id: u32,
    pub name: String,
}

pub type Db = Arc<Mutex<HashMap<u32, Item>>>;

pub fn create_item(db: &Db, item: Item) -> Result<(), String> {
    let mut db = db.lock().unwrap();
    if db.contains_key(&item.id) {
        return Err("Item with this ID already exists".to_string());
    }
    db.insert(item.id, item);
    Ok(())
}

pub fn read_items(db: &Db) -> Vec<Item> {
    let db = db.lock().unwrap();
    db.values().cloned().collect()
}

pub fn update_item(db: &Db, id: u32, name: String) -> Result<(), String> {
    let mut db = db.lock().unwrap();
    if let Some(item) = db.get_mut(&id) {
        item.name = name;
        Ok(())
    } else {
        Err("Item not found".to_string())
    }
}

pub fn delete_item(db: &Db, id: u32) -> Result<(), String> {
    let mut db = db.lock().unwrap();
    if db.remove(&id).is_some() {
        Ok(())
    } else {
        Err("Item not found".to_string())
    }
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create a new database
    fn create_test_db() -> Db {
        Arc::new(Mutex::new(HashMap::new()))
    }

    #[test]
    fn test_create_item() {
        let db = create_test_db();
        let item = Item {
            id: 1,
            name: "Test Item".to_string(),
        };

        // Test successful creation
        assert!(create_item(&db, item.clone()).is_ok());

        // Test duplicate creation
        assert!(create_item(&db, item).is_err());
    }

    #[test]
    fn test_read_items() {
        let db = create_test_db();
        let item1 = Item {
            id: 1,
            name: "Item 1".to_string(),
        };
        let item2 = Item {
            id: 2,
            name: "Item 2".to_string(),
        };

        // Add items to the database
        create_item(&db, item1.clone()).unwrap();
        create_item(&db, item2.clone()).unwrap();

        // Read items and verify
        let items = read_items(&db);
        assert_eq!(items.len(), 2);
        assert!(items.contains(&item1));
        assert!(items.contains(&item2));
    }

    #[test]
    fn test_update_item() {
        let db = create_test_db();
        let item = Item {
            id: 1,
            name: "Old Name".to_string(),
        };

        // Add an item to the database
        create_item(&db, item.clone()).unwrap();

        // Test successful update
        assert!(update_item(&db, 1, "New Name".to_string()).is_ok());

        // Verify the update
        let updated_item = db.lock().unwrap().get(&1).unwrap().clone();
        assert_eq!(updated_item.name, "New Name");

        // Test updating a non-existent item
        assert!(update_item(&db, 2, "Another Name".to_string()).is_err());
    }

    #[test]
    fn test_delete_item() {
        let db = create_test_db();
        let item = Item {
            id: 1,
            name: "Test Item".to_string(),
        };

        // Add an item to the database
        create_item(&db, item.clone()).unwrap();

        // Test successful deletion
        assert!(delete_item(&db, 1).is_ok());

        // Verify the item is deleted
        assert!(db.lock().unwrap().get(&1).is_none());

        // Test deleting a non-existent item
        assert!(delete_item(&db, 2).is_err());
    }
}
