use cpuio::Port;

pub fn check_release(last_code: u8, current_code: u8) -> bool {
    return (last_code != current_code) &&(((8 << 4) | last_code) == current_code)
}
