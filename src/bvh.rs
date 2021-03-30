use std::sync::Arc;

use crate::hittable::{Hittable, Object};

pub struct BVHNode {
    left: Arc<Object>,
    right: Arc<Object>,
    
}
