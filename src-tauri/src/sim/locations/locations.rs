use std::collections::HashMap;
use std::hash::{Hash, Hasher}; // Required for LocationId to be a HashMap key

// --- 1. LocationId Newtype ---
// Provides type safety for location identifiers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocationId(String);

impl Hash for LocationId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state); // Delegate hashing to the inner String
    }
}

// Convenient constructor for LocationId from &'static str
impl From<&'static str> for LocationId {
    fn from(s: &'static str) -> Self {
        LocationId(s.to_string())
    }
}

// --- 2. LocationType Enum ---
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LocationType {
    Skewed,
    Versatile,
    Hub,
}

// --- 3. Location Struct ---
// Stores metadata about each location.
#[derive(Debug, Clone)]
pub struct Location {
    pub id: LocationId,
    pub skew_name: &'static str,
    pub location_name: &'static str,
    pub hint: &'static str,
    pub icon: &'static str,
    pub loc_type: LocationType,
}

// --- 4. CommuteGraph Struct ---
// Encapsulates the adjacency list and provides methods for graph operations.
#[derive(Debug, Clone)]
pub struct CommuteGraph {
    connections: HashMap<LocationId, HashMap<LocationId, u32>>,
}
impl Default for CommuteGraph {
    fn default() -> Self {
        let  mut graph = Self::new();
        graph.initialize_default_connections();
        graph
    }
}

impl CommuteGraph {
    /// Creates a new, empty `CommuteGraph`.
    pub fn new() -> Self {
        CommuteGraph {
            connections: HashMap::new(),
        }
    }

    /// Adds a unidirectional connection (edge) from `from` to `to` with a given `cost`.
    ///
    /// # Arguments
    /// * `from` - The ID of the starting location.
    /// * `to` - The ID of the destination location.
    /// * `cost` - The cost of traversing this connection.
    pub fn add_unidirectional(&mut self, from: LocationId, to: LocationId, cost: u32) {
        self.connections.entry(from).or_default().insert(to, cost);
    }

    /// Adds a bidirectional connection (edges in both directions) between `loc1` and `loc2`
    /// with the same `cost`.
    ///
    /// # Arguments
    /// * `loc1` - The ID of the first location.
    /// * `loc2` - The ID of the second location.
    /// * `cost` - The cost of traversing the connection in either direction.
    pub fn add_bidirectional(&mut self, loc1: LocationId, loc2: LocationId, cost: u32) {
        self.add_unidirectional(loc1.clone(), loc2.clone(), cost);
        self.add_unidirectional(loc2, loc1, cost);
    }

    /// Retrieves the direct commute cost between two locations.
    ///
    /// # Arguments
    /// * `from` - The ID of the starting location.
    /// * `to` - The ID of the destination location.
    ///
    /// # Returns
    /// An `Option<u32>` containing the cost if a direct connection exists, otherwise `None`.
    pub fn get_cost(&self, from: &LocationId, to: &LocationId) -> Option<u32> {
        self.connections
            .get(from) // Get the inner HashMap for 'from' location
            .and_then(|neighbors| neighbors.get(to).copied()) // Get cost to 'to' location and copy it
    }

    /// Retrieves a reference to the `HashMap` of neighbors and their costs for a given location.
    ///
    /// # Arguments
    /// * `location_id` - The ID of the location whose neighbors are to be retrieved.
    ///
    /// # Returns
    /// An `Option<&HashMap<LocationId, u32>>` containing the map of neighbors and their costs
    /// if the location exists in the graph, otherwise `None`.
    pub fn get_neighbors_with_costs(&self, location_id: &LocationId) -> Option<&HashMap<LocationId, u32>> {
        self.connections.get(location_id)
    }

    /// Initializes the `CommuteGraph` with the default game connections:
    /// - Circular graph for skewed locations (cost 2).
    /// - Connections between skewed locations and Versatile Plaza (cost 3).
    /// - Uni-directional connections *into* Tech Valley from all others (cost 1).
    /// - Uni-directional connections *from* Tech Valley to all others (cost 1).
    pub fn initialize_default_connections(&mut self) {
        // --- Circular Graph (Skewed Locations) - Cost 2 (Bidirectional) ---
        self.add_bidirectional("precision_spire".into(), "discipline_stack".into(), 2);
        self.add_bidirectional("discipline_stack".into(), "architecture_foundry".into(), 2);
        self.add_bidirectional("architecture_foundry".into(), "endurance_basin".into(), 2);
        self.add_bidirectional("endurance_basin".into(), "collaboration_core".into(), 2);
        self.add_bidirectional("collaboration_core".into(), "idea_hub".into(), 2);
        self.add_bidirectional("idea_hub".into(), "precision_spire".into(), 2); // Completes the circle

        // --- Skewed Locations <-> Versatile Plaza - Cost 3 (Bidirectional) ---
        self.add_bidirectional("precision_spire".into(), "versatile_plaza".into(), 3);
        self.add_bidirectional("discipline_stack".into(), "versatile_plaza".into(), 3);
        self.add_bidirectional("architecture_foundry".into(), "versatile_plaza".into(), 3);
        self.add_bidirectional("endurance_basin".into(), "versatile_plaza".into(), 3);
        self.add_bidirectional("collaboration_core".into(), "versatile_plaza".into(), 3);
        self.add_bidirectional("idea_hub".into(), "versatile_plaza".into(), 3);

        // --- Connections *into* Tech Valley from all other locations - Cost 1 (Uni-directional) ---
        // Represents talent commuting FROM the other location INTO Tech Valley.
        self.add_unidirectional("precision_spire".into(), "tech_valley".into(), 1);
        self.add_unidirectional("discipline_stack".into(), "tech_valley".into(), 1);
        self.add_unidirectional("architecture_foundry".into(), "tech_valley".into(), 1);
        self.add_unidirectional("endurance_basin".into(), "tech_valley".into(), 1);
        self.add_unidirectional("collaboration_core".into(), "tech_valley".into(), 1);
        self.add_unidirectional("idea_hub".into(), "tech_valley".into(), 1);
        self.add_unidirectional("versatile_plaza".into(), "tech_valley".into(), 1);


    }

    pub fn successors_fn(&self) -> impl Fn(&LocationId) -> Vec<(LocationId, u32)> + '_ {
        move |id: &LocationId| {
            self.get_neighbors_with_costs(id)
                .map(|neighbors| {
                    neighbors
                        .iter()
                        .map(|(neighbor_id, &cost)| (neighbor_id.clone(), cost))
                        .collect()
                })
                .unwrap_or_default()
        }
    }
}

// --- Hardcoded Instances of Locations (Remains the same as before, no changes needed) ---
pub fn get_game_locations() -> Vec<Location> {
    vec![
        Location {
            id: LocationId::from("precision_spire"),
            skew_name: "Precision Core",
            location_name: "Precision Spire",
            hint: "🧠 Detail-obsessed coders",
            icon: "fa-bullseye",
            loc_type: LocationType::Skewed,
        },
        Location {
            id: LocationId::from("discipline_stack"),
            skew_name: "Discipline Stack",
            location_name: "Logic Grid",
            hint: "🧱 Rule-following builders",
            icon: "fa-check-square",
            loc_type: LocationType::Skewed,
        },
        Location {
            id: LocationId::from("architecture_foundry"),
            skew_name: "Systems Instinct",
            location_name: "Architecture Foundry",
            hint: "🛠 Backend / architectural thinkers",
            icon: "fa-cogs",
            loc_type: LocationType::Skewed,
        },
        Location {
            id: LocationId::from("endurance_basin"),
            skew_name: "Resilience Tank",
            location_name: "Endurance Basin",
            hint: "🔋 Tough, low-drama workers",
            icon: "fa-fist-raised",
            loc_type: LocationType::Skewed,
        },
        Location {
            id: LocationId::from("collaboration_core"),
            skew_name: "Empathy Drive",
            location_name: "Collaboration Core",
            hint: "💬 Talkers, feelers, facilitators",
            icon: "fa-handshake",
            loc_type: LocationType::Skewed,
        },
        Location {
            id: LocationId::from("idea_hub"),
            skew_name: "Creative Bloom",
            location_name: "Idea Hub",
            hint: "🎨 Chaotic creativity",
            icon: "fa-lightbulb",
            loc_type: LocationType::Skewed,
        },
        Location {
            id: LocationId::from("versatile_plaza"),
            skew_name: "Balanced Floor",
            location_name: "Versatile Plaza",
            hint: "⚖️ Dependable generalists",
            icon: "fa-balance-scale",
            loc_type: LocationType::Versatile,
        },
        Location {
            id: LocationId::from("tech_valley"),
            skew_name: "Tech Hub",
            location_name: "Tech Valley",
            hint: "💰 Elite talent, high cost",
            icon: "fa-atom",
            loc_type: LocationType::Hub,
        },
    ]
}
#[cfg(test)]
mod tests {
    use super::*;

    // --- LocationId Tests ---
    #[test]
    fn location_id_creation_and_equality() {
        let id1 = LocationId::from("test_location");
        let id2 = LocationId("test_location".to_string());
        let id3 = LocationId::from("another_location");

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn location_id_hashing() {
        let mut map = HashMap::new();
        let id1 = LocationId::from("key_location");
        map.insert(id1.clone(), "value");

        assert_eq!(map.get(&id1), Some(&"value"));
    }

    // --- CommuteGraph Tests ---
    #[test]
    fn new_commute_graph_is_empty() {
        let graph = CommuteGraph::new();
        assert!(graph.connections.is_empty());
    }

    #[test]
    fn add_unidirectional_connection() {
        let mut graph = CommuteGraph::new();
        let loc_a = LocationId::from("loc_a");
        let loc_b = LocationId::from("loc_b");

        graph.add_unidirectional(loc_a.clone(), loc_b.clone(), 10);

        assert_eq!(graph.get_cost(&loc_a, &loc_b), Some(10));
        assert_eq!(graph.get_cost(&loc_b, &loc_a), None); // Should not exist in reverse
    }

    #[test]
    fn add_bidirectional_connection() {
        let mut graph = CommuteGraph::new();
        let loc_a = LocationId::from("loc_a");
        let loc_b = LocationId::from("loc_b");

        graph.add_bidirectional(loc_a.clone(), loc_b.clone(), 5);

        assert_eq!(graph.get_cost(&loc_a, &loc_b), Some(5));
        assert_eq!(graph.get_cost(&loc_b, &loc_a), Some(5));
    }

    #[test]
    fn get_cost_existing_and_non_existing() {
        let mut graph = CommuteGraph::new();
        let loc_a = LocationId::from("loc_a");
        let loc_b = LocationId::from("loc_b");
        let loc_c = LocationId::from("loc_c");

        graph.add_unidirectional(loc_a.clone(), loc_b.clone(), 7);

        assert_eq!(graph.get_cost(&loc_a, &loc_b), Some(7));
        assert_eq!(graph.get_cost(&loc_a, &loc_c), None);
        assert_eq!(graph.get_cost(&loc_b, &loc_a), None);
    }

    #[test]
    fn get_neighbors_with_costs_test() {
        let mut graph = CommuteGraph::new();
        let loc_a = LocationId::from("loc_a");
        let loc_b = LocationId::from("loc_b");
        let loc_c = LocationId::from("loc_c");
        let loc_d = LocationId::from("loc_d"); // No connections from loc_a

        graph.add_unidirectional(loc_a.clone(), loc_b.clone(), 1);
        graph.add_unidirectional(loc_a.clone(), loc_c.clone(), 2);

        let neighbors_a = graph.get_neighbors_with_costs(&loc_a);
        assert!(neighbors_a.is_some());
        let neighbors_a_map = neighbors_a.unwrap();
        assert_eq!(neighbors_a_map.len(), 2);
        assert_eq!(neighbors_a_map.get(&loc_b), Some(&1));
        assert_eq!(neighbors_a_map.get(&loc_c), Some(&2));

        let neighbors_b = graph.get_neighbors_with_costs(&loc_b); // loc_b was only a destination, not a source
        assert!(neighbors_b.is_none());

        let neighbors_d = graph.get_neighbors_with_costs(&loc_d);
        assert!(neighbors_d.is_none());
    }

    #[test]
    fn initialize_default_connections_creates_expected_graph() {
        let mut graph = CommuteGraph::new();
        graph.initialize_default_connections();

        let precision_spire: LocationId = "precision_spire".into();
        let discipline_stack: LocationId = "discipline_stack".into();
        let versatile_plaza: LocationId = "versatile_plaza".into();
        let tech_valley: LocationId = "tech_valley".into();
        let architecture_foundry: LocationId = "architecture_foundry".into();


        // Test a few circular connections
        assert_eq!(graph.get_cost(&precision_spire, &discipline_stack), Some(2));
        assert_eq!(graph.get_cost(&discipline_stack, &precision_spire), Some(2));

        // Test a few skewed to versatile connections
        assert_eq!(graph.get_cost(&precision_spire, &versatile_plaza), Some(3));
        assert_eq!(graph.get_cost(&versatile_plaza, &precision_spire), Some(3));

        // Test connections into Tech Valley
        assert_eq!(graph.get_cost(&precision_spire, &tech_valley), Some(1));
        assert_eq!(graph.get_cost(&versatile_plaza, &tech_valley), Some(1));

        // Test that connections FROM Tech Valley are NOT automatically created by initialize_default_connections
        // (as per current implementation, only INTO Tech Valley is defined there)
        // If you later add bidirectional connections to/from Tech Valley in initialize_default_connections,
        // this part of the test would need to be updated.
        assert_eq!(graph.get_cost(&tech_valley, &precision_spire), None);
        assert_eq!(graph.get_cost(&tech_valley, &versatile_plaza), None);

        // Test a non-existent direct link in the circular graph that should not be there
        assert_eq!(graph.get_cost(&precision_spire, &architecture_foundry), None);
    }

    #[test]
    fn default_commute_graph_initializes_connections() {
        let graph = CommuteGraph::default();

        let precision_spire: LocationId = "precision_spire".into();
        let tech_valley: LocationId = "tech_valley".into();

        // Check a sample connection to ensure initialization happened
        assert_eq!(graph.get_cost(&precision_spire, &tech_valley), Some(1));
    }

    #[test]
    fn successors_fn_works_correctly() {
        let mut graph = CommuteGraph::new();
        let loc_a = LocationId::from("loc_a");
        let loc_b = LocationId::from("loc_b");
        let loc_c = LocationId::from("loc_c");
        let loc_d = LocationId::from("loc_d"); // No outgoing connections

        graph.add_unidirectional(loc_a.clone(), loc_b.clone(), 10);
        graph.add_unidirectional(loc_a.clone(), loc_c.clone(), 20);

        let successors = graph.successors_fn();

        let mut succ_a = successors(&loc_a);
        succ_a.sort_by_key(|k| k.0.0.clone()); // Sort for consistent comparison

        let expected_succ_a = vec![(loc_b.clone(), 10), (loc_c.clone(), 20)];
        let mut sorted_expected_succ_a = expected_succ_a.clone();
        sorted_expected_succ_a.sort_by_key(|k| k.0.0.clone());


        assert_eq!(succ_a.len(), 2);
        assert!(succ_a.contains(&(loc_b.clone(), 10)));
        assert!(succ_a.contains(&(loc_c.clone(), 20)));


        let succ_b = successors(&loc_b);
        assert!(succ_b.is_empty());

        let succ_d = successors(&loc_d); // loc_d is not in the graph's keys
        assert!(succ_d.is_empty());
    }

    #[test]
    fn get_game_locations_returns_all_locations() {
        let locations = get_game_locations();
        assert_eq!(locations.len(), 8); // Based on the hardcoded locations

        // Spot check a few
        assert!(locations.iter().any(|loc| loc.id == LocationId::from("precision_spire")));
        assert!(locations.iter().any(|loc| loc.id == LocationId::from("versatile_plaza")));
        assert!(locations.iter().any(|loc| loc.id == LocationId::from("tech_valley")));
    }
}
