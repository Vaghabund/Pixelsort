#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Phase {
    Input,
    Edit,
    Crop,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DragState {
    None,
    DraggingHandle(HandlePosition),
    MovingCrop,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HandlePosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}
