use crate::common::{Projection, aggregate};

pub trait MultiProjection: Projection {
    type Key: aggregate::Key;
}
