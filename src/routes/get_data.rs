use std::collections::HashMap;
use std::str;
use std::string::String;
use super::cluster_indexing::cluster_indexing;
use super::helper_structs::{AssignmentsAndClusters, Cluster, SearchData};


pub fn index_data(input_data: &SearchData) -> HashMap<String, SearchData> {
    let mut assignments_and_clusters: AssignmentsAndClusters;
    assignments_and_clusters = match std::fs::read_to_string("indexes.dat") {
        Ok(value) => reassemble_assignments_and_clusters(value),
        Err(err) => cluster_indexing(&input_data),
    };

    let mut search_data: HashMap<String, SearchData> = HashMap::new();
    for cluster_index in 0..assignments_and_clusters.clusters.len() {
        let mut storage: Vec<Vec<f32>> = Vec::new();
        let mut geo: Vec<Vec<f32>> = Vec::new();
        let mut ids = assignments_and_clusters.clusters[cluster_index].ids.clone();
        search_data.insert(cluster_index.to_string(), SearchData {
            storage: storage,
            ids: ids,
            geo: geo,
            centroid: assignments_and_clusters.clusters[cluster_index].centroid.clone(),
        });
    }
    for move_index in 0..assignments_and_clusters.moves.len() {
        let search_index: &String = &assignments_and_clusters.moves[move_index].new_cluster_index.to_string();
        search_data.get_mut(search_index).unwrap().storage.push(input_data.storage[move_index].clone());
        search_data.get_mut(search_index).unwrap().ids.push(input_data.ids[move_index].clone());
        search_data.get_mut(search_index).unwrap().geo.push(input_data.geo[move_index].clone());
    }
    return search_data;
}


fn reassemble_assignments_and_clusters(content: String) -> AssignmentsAndClusters {
    let mut assignments_and_clusters: AssignmentsAndClusters;
    let mut clusters: Vec<Cluster> = Vec::new();
    
    let mut lines = content.split("\n").into_iter().map(|line| {
        let centroid_and_ids: Vec<&str> = line.split("!").collect();
        let centroid: Vec<f32> = centroid_and_ids[0].split(",").collect::<Vec<&str>>().into_iter().map(|s| s.parse::<f32>().unwrap()).collect();
        let mut ids: Vec<String> = Vec::new();
        for chunk in centroid_and_ids[1..].join("!").chars().collect::<Vec<char>>().chunks(40) {
            ids.push(chunk.iter().collect::<String>());
        }
        clusters.push(Cluster{ids:ids, centroid:centroid, vectors: Vec::<Vec<f32>>::new()})
    });
    println!("Reading from indexes.dat file.");
    return AssignmentsAndClusters{moves: Vec::new(), clusters: Vec::new()};
}


pub fn get_data(filename: String, vec_size: usize) -> SearchData {
    // This accepts a filename and returns search data
    // It (used to but it commented out) checks whether there is 2x RAM of the file available
    // If so, it uses th RAM
    // If not, it goes item by item, which is much slower (20x?)
    // let mut sys = sysinfo::System::new();
    // sys.refresh_all();
    // if (sys.total_memory() - sys.used_memory()) / 2 > std::fs::metadata(&filename).unwrap().len() {
    return load_bin_to_vec(&filename, vec_size);
    // } else {
    //     return read_filename_to_data(&filename);
    // }
}


fn load_bin_to_vec(vectors_filename: &String, vec_size: usize) -> SearchData {
    // This uses 2x the RAM of the filesize temporarily
    // to read in the data to RAM
    let bytes: Vec<u8> = std::fs::read(vectors_filename).unwrap();
    let file_length = bytes.iter().count();
    let id_size = 32;
    let row_size = id_size + vec_size * 4 + 2 * 4;
    let nb_rows: usize = file_length / row_size;
    println!("{}", nb_rows);
    let mut id: &str;
    let mut storage: Vec<f32>;
    let mut centroid: Vec<f32>;
    let mut geo: Vec<f32>;
    let mut row_start: usize;
    let float_size: usize = 4;
    let mut tmp_u8: [u8; 4];
    let mut vec_start: usize;
    let mut tmp_ids = Vec::<String>::new();
    let mut tmp_storage = Vec::<Vec<f32>>::new();
    let mut tmp_geo = Vec::<Vec<f32>>::new();
    let mut tmp_centroid = Vec::<f32>::new();
    let mut data: SearchData = SearchData {
        storage: tmp_storage,
        ids: tmp_ids,
        geo: tmp_geo,
        centroid: tmp_centroid,
    };

    for row_index in 0..(nb_rows) {
        // Read ID
        row_start = row_index * row_size;
        id = str::from_utf8(&bytes[row_start..(row_start + id_size)]).unwrap();

        // Read vector
        storage = Vec::with_capacity(vec_size);

        row_start += id_size;
        for vec_index in 0..vec_size {
            tmp_u8 = [0, 0, 0, 0];
            vec_start = vec_index * 4;
            for u_index in 0..4 {
                tmp_u8[u_index] = bytes[row_start + vec_start + u_index];
            }
            storage.push(f32::from_ne_bytes(tmp_u8));
        }

        // Read coordinates
        row_start += vec_size * float_size;
        geo = Vec::with_capacity(2);
        for vec_index in 0..2 {
            tmp_u8 = [0; 4];
            vec_start = vec_index * 4;
            for u_index in 0..4 {
                tmp_u8[u_index] = bytes[row_start + vec_start + u_index];
            }
            geo.push(f32::from_ne_bytes(tmp_u8));
        }

        data.ids.push(id.to_owned());
        data.storage.push(storage);
        data.geo.push(geo);
    }

    return data;
}
