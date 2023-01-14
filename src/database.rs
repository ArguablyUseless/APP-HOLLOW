use std::{collections::BTreeMap};

use rocket::{fairing::Result};
use surrealdb::{Session, Datastore, sql::Value, Error};

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                Static Database (safe reference)                                //
////////////////////////////////////////////////////////////////////////////////////////////////////

static mut DATABASE: Option<Database> = None;

const CONFIG_NAMESPACE: &str = "test";
const CONFIG_DATABASE: &str = "test";
const CONFIG_DATASTORE: &str = "memory";

/// Init the database with the defined parameters.
/// Throw a warning if the database has already been initialized.
pub async fn init_database() {
    unsafe {
        if DATABASE.is_some() {
            eprintln!("[Warning]: The database has already been initialized!");
            return;
        }
        
        let db = Database::new(
            CONFIG_NAMESPACE,
            CONFIG_DATABASE,
            CONFIG_DATASTORE
        ).await;

        DATABASE = Some(db);
    }
}

/// Return a static (safe) reference to the Database
/// Throw an error if the database has not been initialized!
pub fn get_database() -> &'static Database {
    unsafe {
        if let Some(db) = &DATABASE {
            db
        } else {
            eprintln!("[Error]: The database has not been initialized, consider called init_database()!");
            std::process::exit(1);
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                            Database                                            //
////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Database {
    session: Session,
    datastore: Datastore
}

impl Database {

    /// Creates a new Database with the specified parameters.
    async fn new(namespace: &str, database: &str, datastore: &str) -> Self {
        Self {
            session: Session::for_kv().with_ns(namespace).with_db(database),
            datastore: Datastore::new(&datastore).await.unwrap()
        }
    }

    /// Execute a query with the specified statement and the variables
    /// If there are no variables, [`vars`] can be set to [`None`]
    pub async fn query(&self, statement: &str, vars: Option<BTreeMap<String, Value>>) -> Result<Vec<Value>, Error> {
        let responses = self.datastore.execute(statement, &self.session, vars, false).await?;

        let mut results = Vec::new();
        for response in responses {
            results.push(response.result?);
        }

        Ok(results)
    }

}