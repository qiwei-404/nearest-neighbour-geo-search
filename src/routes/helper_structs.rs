use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct JsonDistSearch {
    pub sort_by_vec: String,
    pub geoc: Vec<f32>,
    pub vector: Vec<f32>,
    pub geo_threshold: f32,
    pub vec_threshold: f32,
    pub limit_results: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NamedSearch {
    pub item_id: String,
    pub amount_of_results: String,
}


#[derive(Serialize)]
#[derive(Clone)]
pub struct Item {
    pub id: String,
    pub geo_dist: f32,
    pub dist: f32,
}

#[derive(Serialize)]
pub struct Items {
    pub items: Vec<Item>,
}

#[derive(Serialize)]
#[derive(Clone)]
pub struct Id {
    pub id: String,
}
