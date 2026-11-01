use ultralytics_inference::{YOLOModel, InferenceConfig};

fn main() {
    println!("Hello, world!");

    // Automatically loads and routes to your CUDA GPU
    let mut model = YOLOModel::load("yolo11n.onnx").unwrap(); 

    let mut config = InferenceConfig::default();
    config.conf = 0.5; // Bounding box threshold
    config.stream = true; // Optimization for security video streams

    // Ingests an RTSP security camera stream or file path directly
    model.predict_config("rtsp://your_camera_ip/stream", config); 
}
