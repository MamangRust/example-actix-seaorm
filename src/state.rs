use sea_orm::DatabaseConnection;

use crate::{config::{Hashing, JwtConfig}, utils::DependenciesInject};

#[derive(Clone)]
pub struct AppState {
    pub di_container: DependenciesInject,
    pub jwt_config: JwtConfig,
}

impl AppState {
    pub fn new(pool: DatabaseConnection) -> Self {
        let jwt_config = JwtConfig::new();
        let hashing = Hashing::new();

        let di_container = DependenciesInject::new(pool, hashing, jwt_config.clone());
        
        Self { di_container, jwt_config }
    }

}