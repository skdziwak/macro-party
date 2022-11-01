use crate::low_level_handler::EventHandler;

mod low_level_handler;

struct Handler {

}

impl EventHandler for Handler {
    fn key_pressed(&self, code: low_level_handler::VkCode) -> bool {
        println!("Pressed {0}", code);
        return true;
    }

    fn key_released(&self, code: low_level_handler::VkCode) -> bool {
        println!("Released {0}", code);
        return false;
    }
}


fn main() {
    let handler = Handler{

    };
    let handler_box: Box<dyn EventHandler> = Box::from(handler);
    low_level_handler::run(handler_box);
}
