use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum GridEntityType {
    Castle,
    Tree,
    Axe,
    Hero,
    Mountain,
    Water,
    LumberMill,
    Bridge,
    Mineral,
    // Standard,
    // Blocked,
    // RouteHead,
    // RoutePoint,
}
