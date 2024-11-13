use std::collections::HashMap;
use super::distances::{manhattan, dist};
use super::helper_structs::{Cluster, Move, AssignmentsAndClusters, SearchData};


pub fn cluster_indexing(input_data: &SearchData) -> AssignmentsAndClusters {
    println!("indexing");
    let total_vector_count = input_data.storage.len();
    let cluster_count = (total_vector_count as f32).sqrt() as usize;
    let mut clusters: Vec<Cluster> = Vec::new();
    // create clusters as sqrt of total_vector_count AKA cluster_count
    // first pass cluster-vector assignment
    for cluster_index in 0..cluster_count {
        let mut vectors: Vec<Vec<f32>> = Vec::new();
        let mut ids: Vec<String> = Vec::new();
        let mut centroid: Vec<f32> = Vec::new();
        clusters.push(Cluster {
            vectors: vectors,
            ids: ids,
            centroid: centroid,
        });
        clusters[cluster_index]
            .vectors
            .push(input_data.storage[cluster_index].clone());
        clusters[cluster_index].centroid = input_data.storage[cluster_index].clone();
    }
    for traversal_index in cluster_count..total_vector_count {
        let mut min_index: usize = 1;
        let mut min_dist: f32 = f32::MAX;
        for cluster_index in 0..cluster_count {
            let new_dist = manhattan(
                &(input_data.storage[traversal_index]),
                &(clusters[cluster_index].centroid),
            );
            if new_dist < min_dist {
                min_index = cluster_index;
                min_dist = new_dist;
            }
        }
        clusters[min_index]
            .vectors
            .push(input_data.storage[traversal_index].clone());
        clusters[min_index]
            .ids
            .push(input_data.ids[traversal_index].clone());
    }
    //multipass
    // reset centroids
    // keep track of moves (if count increases or reaches 0, stop)
    let mut full_move_counter: u32 = 0;
    let mut moves: Vec<Move> = Vec::new();
    while full_move_counter < (cluster_count as u32) / 4 {
        for cluster_index in 0..cluster_count {
            let cluster_count = clusters[cluster_index].vectors.len();
            let mut totaled_vec: Vec<f32> = Vec::new();
            let vec_dims = input_data.storage[0].len();
            for dim in 0..vec_dims {
                totaled_vec.push(0.0);
                for vec in clusters[cluster_index].vectors.clone() {
                    totaled_vec[dim] += vec[dim] / (cluster_count as f32);
                }
            }
            clusters[cluster_index] = Cluster {
                vectors: clusters[cluster_index].vectors.clone(),
                ids: clusters[cluster_index].ids.clone(),
                centroid: totaled_vec,
            };
        }

        moves = Vec::new();
        // for every vector
        for traversal_index in 0..input_data.storage.len() {
            let mut min_index: usize = 1;
            // compare against every cluster
            let mut min_dist = f32::MAX;
            for cluster_index in 0..cluster_count {
                let new_dist = manhattan(
                    &(input_data.storage[traversal_index]),
                    &(clusters[cluster_index as usize].centroid),
                );
                if new_dist < min_dist {
                    min_dist = new_dist;
                    min_index = cluster_index;
                }
            }
            // create a Move assignment
            moves.push(Move {
                new_cluster_index: min_index as usize,
                id: input_data.ids[traversal_index].clone(),
            });
        }
        let mut new_clusters: Vec<Cluster> = Vec::new();
        for _ in 0..cluster_count {
            let mut vectors: Vec<Vec<f32>> = Vec::new();
            let mut ids: Vec<String> = Vec::new();
            let mut centroid: Vec<f32> = Vec::new();
            new_clusters.push(Cluster {
                vectors: vectors,
                ids: ids,
                centroid: centroid,
            });
        }
        for move_index in 0..moves.len() {
            new_clusters[moves[move_index].new_cluster_index]
                .vectors
                .push(input_data.storage[move_index].clone());
            new_clusters[moves[move_index].new_cluster_index]
                .ids
                .push(input_data.ids[move_index].clone());
        }
        for cluster_index in 0..cluster_count {
            let cluster_count = new_clusters[cluster_index].vectors.len();
            let mut totaled_vec: Vec<f32> = Vec::new();
            let vec_dims = input_data.storage[0].len();
            for dim in 0..vec_dims {
                totaled_vec.push(0.0);
                for vec in new_clusters[cluster_index].vectors.clone() {
                    totaled_vec[dim] += vec[dim] / (cluster_count as f32);
                }
            }
            new_clusters[cluster_index] = Cluster {
                vectors: new_clusters[cluster_index].vectors.clone(),
                ids: new_clusters[cluster_index].ids.clone(),
                centroid: totaled_vec,
            };
        }
        full_move_counter += 1;
        clusters = new_clusters;
    }
    return AssignmentsAndClusters {
        moves: moves,
        clusters: clusters,
    };
}
