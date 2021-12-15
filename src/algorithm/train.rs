use crate::models::*;
use log::*;

pub struct TrainedRMI {
    // pub model_avg_error: f64,
    // pub model_avg_l2_error: f64,
    // pub model_avg_log2_error: f64,
    // pub model_max_error: u64,
    // pub model_max_error_idx: usize,
    // pub last_layer_max_l1s: Vec<u64>,
    pub rmi: Vec<Vec<Box<dyn Model>>>,
}

fn train_model(model_name: &str, data: &ModelData) -> Box<dyn Model> {
    let model: Box<dyn Model> = match model_name {
        "linear" => Box::new(LinearModel::new(data)),
        "loglinear" => Box::new(LogLinearModel::new(data)),
        _ => panic!("Unknown model type: {}", model_name),
    };
    model
}

pub fn train_two_layer(data: &ModelData) {
    let mut train_data = data.clone();
    let num_leaf_model = 64;
    let num_row = train_data.len();
    let scale = num_leaf_model as f64 / num_row as f64;
    train_data.set_scale(scale);
    let top_model = train_model(&"linear", &train_data);
    info!(
        "Top model evaluate result: {}",
        top_model.evaluate(&train_data.get_all_x(), &train_data.get_all_y())
    );

    // info!("{:?}", train_data);
    // train_data.set_scale(1.0);
    let mid_point = num_leaf_model / 2;
    // Find split point near mid_point
    info!("Mid point of dataset: {}", mid_point);
    let pre_result = top_model.predict_list(&train_data.get_all_x());
    for i in pre_result.into_iter() {
        // info!("{}", i / scale);
    }
}

pub fn train(data: &ModelData) {
    if data.len() > 1000000 {
        println!("Train two layer");
        return;
    }
    let mut rmi: Vec<Vec<Box<dyn Model>>> = Vec::new();
    let model_list: Vec<&str> = vec!["linear"];
    // let data = ModelData::new(x_values, y_values);
    let branch_factor = 100usize;

    let mut current_model_count = 1;
    let mut current_data_size = 0;
    for (layer_idx, model_name) in model_list.into_iter().enumerate() {
        println!("{}, {}", layer_idx, model_name);
        let next_layer_size: usize = current_model_count * branch_factor as usize;
        println!("{}", next_layer_size);
        let mut models: Vec<Box<dyn Model>> = Vec::with_capacity(next_layer_size as usize);

        let model_num = data.len() / branch_factor;
        for _ in 0..model_num {
            // data partition
            let batch_data = data.get_range(current_data_size, current_data_size + branch_factor);
            // let scale = (next_layer_size as f64) / (model_num as f64);
            // batch_data.set_scale(scale);
            // println!("{:?}", batch_data);

            let model = train_model(model_name, &batch_data);
            let y_predict = model.predict_list(&batch_data.get_all_x());
            let eval_result = model.evaluate(&batch_data.get_all_x(), &batch_data.get_all_y());
            info!("Prediction {:?}", y_predict);
            info!("Evalution {:?}", eval_result);
            models.push(model);
            current_model_count += 1;
            current_data_size += branch_factor;
        }

        rmi.push(models);

        // bound check for model prediction, if prediction fall into next level model
    }
    println!("Total models: {}", current_model_count);
    // TrainedRMI {}
}
