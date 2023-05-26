pub fn flip_vertically(frame_buffer: &Vec<u8>, width: usize, height: usize) -> Vec<u8> {
    let mut flipped_frame_buffer = frame_buffer.clone();
    for y in 0..height / 2 {
        for x in 0..width {
            let top_index = (y * width + x) * 3;
            let bottom_index = ((height - y - 1) * width + x) * 3;
            flipped_frame_buffer.swap(top_index, bottom_index);
            flipped_frame_buffer.swap(top_index + 1, bottom_index + 1);
            flipped_frame_buffer.swap(top_index + 2, bottom_index + 2);
        }
    }
    flipped_frame_buffer
}
