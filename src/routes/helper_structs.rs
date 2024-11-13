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

#[derive(Serialize, Clone)]
pub struct Item {
    pub id: String,
    pub geo_dist: f32,
    pub dist: f32,
}

#[derive(Serialize)]
pub struct Items {
    pub items: Vec<Item>,
}

#[derive(Serialize, Clone)]
pub struct Id {
    pub id: String,
}

pub struct Cluster {
    pub vectors: Vec<Vec<f32>>,
    pub ids: Vec<String>,
    pub centroid: Vec<f32>,
}

pub struct Move {
    pub id: String,
    pub new_cluster_index: usize,
}

pub struct AssignmentsAndClusters {
    pub clusters: Vec<Cluster>,
    pub moves: Vec<Move>,
}

pub struct KeyDist {
    pub distance: f32,
    pub key: String,
}

pub struct SearchData {
    pub storage: Vec<Vec<f32>>,
    pub ids: Vec<String>,
    pub geo: Vec<Vec<f32>>,
    pub centroid: Vec<f32>,
}
