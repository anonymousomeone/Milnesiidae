pub struct Timeman {}

impl Timeman {
    pub fn get_time(time_left: u32, movestogo: u16) -> u32 {
        let time = time_left / Into::<u32>::into(movestogo);

        time
    }
}