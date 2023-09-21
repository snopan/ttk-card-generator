pub fn reisize_maintain_ratio(
    old_width: f32,
    old_height: f32,
    new_width: f32,
    new_height: f32,
) -> (f32, f32) {
    let current_ratio = old_width / old_height;
    let required_ratio = new_width / new_height;

    // It means that the current width is longer than the new width
    // so we keep use the new height the same and modify current width
    if current_ratio > required_ratio {
        return (new_height * current_ratio, new_height);
    }

    // Current height is longer than new height so we do the opposite
    (new_width, new_width / current_ratio)
}
