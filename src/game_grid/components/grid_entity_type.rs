#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum GridEntityType {
    Castle,
    Tree,
    Axe,
    Hero,
    Mountain,
    Water, // Standard,
           // Blocked,
           // RouteHead,
           // RoutePoint,
           // Mineral,
}
