use actix_web::{Responder, web, post};
use glidesort;
mod helper_structs;
mod distances;
pub mod get_data;
use std::collections::HashMap;
use actix_web::Error;


struct KeyDist {
    distance: f32,
    key: String,
}


#[post("/get_by_id")]
pub async fn get_by_id(app_state: web::Data<HashMap<String, get_data::SearchData>>, item: web::Json<helper_structs::NamedSearch>) -> impl Responder {
    let amount_of_results: usize = item.amount_of_results.parse().unwrap();
    let mut result: Vec::<f32> = Vec::<f32>::new();
    let mut found = false;
    let mut results: Vec<helper_structs::Item> = Vec::<helper_structs::Item>::new();

    // get a single item
    for key in app_state.keys() {
        for index in 0..app_state[&key.to_owned()].ids.len() {
            if &app_state[&key.to_owned()].ids[index] == &item.item_id {
                result = app_state[&key.to_owned()].storage[index].clone();
                found = true;
                break;
            }
        }
        if found {
            break;
        }
    }

    // use that item on an ANN search
    let mut key_list: Vec::<KeyDist> = Vec::<KeyDist>::new();
    for key in app_state.keys() {
        key_list.push(KeyDist{
            distance: distances::dist(&result, &app_state[key].centroid),
            key: key.to_owned()
        });
    }
    key_list.sort_by(|a, b| b.distance.partial_cmp(&a.distance).unwrap());

    let top_5_percent = key_list.len() / 20;
    let mut fl_dist;
    for counter in 0..top_5_percent {
        let key = key_list[counter].key.clone();
        for index in 0..app_state[&key].storage.len() {
            fl_dist = distances::dist(&result, &app_state[&key].storage[index]);
            results.push(helper_structs::Item{
                id: app_state[&key].ids[index].clone(),
                geo_dist: 0.0,
                dist: fl_dist,
            });
        }
    }

    // Sorting based on smallest vector distance
    glidesort::sort_by(&mut results, |a, b| {
        a.dist.partial_cmp(&b.dist).unwrap()
    });

    let output: Vec<helper_structs::Id> = results.iter().map(|item| helper_structs::Id{id: item.id.clone()}).collect();
    Ok::<web::Json<Vec<helper_structs::Id>>, Error>(web::Json(output[..amount_of_results].to_vec()))

}


#[post("/search")]
pub async fn search(app_state: web::Data<HashMap<String, get_data::SearchData>>, item: web::Json<helper_structs::JsonDistSearch>) -> impl Responder {
    // Calculate distances
    let mut results: Vec<helper_structs::Item> = Vec::<helper_structs::Item>::new();
    let mut fl_dist: f32;
    let mut haver_dist: f32;

    for key in app_state.keys() {
        for index in 0..app_state[&key.to_owned()].storage.len() {
            fl_dist = distances::dist(&item.vector, &app_state[&key.to_owned()].storage[index]);
            haver_dist = distances::haversine(&item.geoc, &app_state[&key.to_owned()].geo[index]);
            if fl_dist < item.vec_threshold && haver_dist < item.geo_threshold {
                results.push(helper_structs::Item{
                    id: app_state[&key.to_owned()].ids[index].clone(),
                    geo_dist: haver_dist,
                    dist: fl_dist.sqrt(),
                });
            }
        }
    }

    // Sorting based on smallest distance
    if &item.sort_by_vec == "1" {
        // results.sort_by(|a, b| a.dist.partial_cmp(&b.dist).unwrap());
        glidesort::sort_by(&mut results, |a, b| {
            a.dist.partial_cmp(&b.dist).unwrap()
        });
    } else {
        // results.sort_by(|a, b| a.geo_dist.partial_cmp(&b.geo_dist).unwrap());
        glidesort::sort_by(&mut results, |a, b| {
            a.geo_dist.partial_cmp(&b.geo_dist).unwrap()
        });
    }

    let results = helper_structs::Items{items: results.to_vec()[..item.limit_results].to_vec()};

    Ok::<web::Json<helper_structs::Items>, Error>(web::Json(results))
}


#[post("/search_ann")]
pub async fn search_ann(app_state: web::Data<HashMap<String, get_data::SearchData>>, item: web::Json<helper_structs::JsonDistSearch>) -> impl Responder {
    // Calculate distances
    let mut results: Vec<helper_structs::Item> = Vec::<helper_structs::Item>::new();
    let mut fl_dist: f32;
    let mut haver_dist: f32;

    let mut key_list: Vec::<KeyDist> = Vec::<KeyDist>::new();
    for key in app_state.keys() {
        key_list.push(KeyDist{
            distance: distances::dist(&item.vector, &app_state[key].centroid),
            key: key.to_owned()
        });
    }
    key_list.sort_by(|a, b| b.distance.partial_cmp(&a.distance).unwrap());
    let top_5_percent = key_list.len() / 20;
    for counter in 0..top_5_percent {
        let key = key_list[counter].key.clone();
        for index in 0..app_state[&key].storage.len() {
            fl_dist = distances::dist(&item.vector, &app_state[&key].storage[index]);
            haver_dist = distances::haversine(&item.geoc, &app_state[&key].geo[index]);
            if fl_dist < item.vec_threshold  && haver_dist < item.geo_threshold {
                results.push(helper_structs::Item{
                    id: app_state[&key].ids[index].clone(),
                    geo_dist: haver_dist,
                    dist: fl_dist.sqrt(),
                });
            }
        }
    }

    // Sorting based on smallest distance
    if &item.sort_by_vec == "1" {
        // results.sort_by(|a, b| a.dist.partial_cmp(&b.dist).unwrap());
        glidesort::sort_by(&mut results, |a, b| {
            a.dist.partial_cmp(&b.dist).unwrap()
        });
    } else {
        // results.sort_by(|a, b| a.geo_dist.partial_cmp(&b.geo_dist).unwrap());
        glidesort::sort_by(&mut results, |a, b| {
            a.geo_dist.partial_cmp(&b.geo_dist).unwrap()
        });
    }

    let results = helper_structs::Items{items: results.to_vec()[..item.limit_results].to_vec()};

    Ok::<web::Json<helper_structs::Items>, Error>(web::Json(results))
}
    
