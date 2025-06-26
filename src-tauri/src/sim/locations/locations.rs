use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

// --- 1. LocationId Newtype ---
// Provides type safety for location identifiers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocationId(String);

impl Hash for LocationId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

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

// --- 4. LocationGraph Struct ---
// Encapsulates the adjacency list and provides methods for graph operations.
#[derive(Debug, Clone)]
pub struct LocationGraph {
    connections: HashMap<LocationId, HashMap<LocationId, u32>>,
    pub locations: HashMap<LocationId, Location>,
}

impl LocationGraph {
    /// Creates a new, empty `LocationGraph`.
    pub fn new() -> Self {
        LocationGraph {
            connections: HashMap::new(),
            locations: HashMap::new(),
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

    /// Retrieves the direct cost between two locations.
    ///
    /// # Arguments
    /// * `from` - The ID of the starting location.
    /// * `to` - The ID of the destination location.
    ///
    /// # Returns
    /// An `Option<u32>` containing the cost if a direct connection exists, otherwise `None`.
    pub fn get_cost(&self, from: &LocationId, to: &LocationId) -> Option<u32> {
        self.connections
            .get(from)
            .and_then(|neighbors| neighbors.get(to).copied())
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

    /// Retrieves a reference to a `Location` by its ID.
    pub fn get_location(&self, id: &LocationId) -> Option<&Location> {
        self.locations.get(id)
    }

    /// Initializes the `LocationGraph` with all default locations and connections.
    pub fn initialize_default_connections_and_locations(&mut self) {
        // --- Populate Locations first ---
        let raw_locations = vec![
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
        ];

        for loc in raw_locations {
            self.locations.insert(loc.id.clone(), loc);
        }

        // --- Populate Connections ---
        // Circular Graph (Skewed Locations) - Cost 2 (Bidirectional)
        self.add_bidirectional("precision_spire".into(), "discipline_stack".into(), 2);
        self.add_bidirectional("discipline_stack".into(), "architecture_foundry".into(), 2);
        self.add_bidirectional("architecture_foundry".into(), "endurance_basin".into(), 2);
        self.add_bidirectional("endurance_basin".into(), "collaboration_core".into(), 2);
        self.add_bidirectional("collaboration_core".into(), "idea_hub".into(), 2);
        self.add_bidirectional("idea_hub".into(), "precision_spire".into(), 2); // Completes the circle

        // Skewed Locations <-> Versatile Plaza - Cost 3 (Bidirectional)
        self.add_bidirectional("precision_spire".into(), "versatile_plaza".into(), 3);
        self.add_bidirectional("discipline_stack".into(), "versatile_plaza".into(), 3);
        self.add_bidirectional("architecture_foundry".into(), "versatile_plaza".into(), 3);
        self.add_bidirectional("endurance_basin".into(), "versatile_plaza".into(), 3);
        self.add_bidirectional("collaboration_core".into(), "versatile_plaza".into(), 3);
        self.add_bidirectional("idea_hub".into(), "versatile_plaza".into(), 3);

        // Connections *into* Tech Valley from all other locations - Cost 1 (Uni-directional)
        self.add_unidirectional("precision_spire".into(), "tech_valley".into(), 1);
        self.add_unidirectional("discipline_stack".into(), "tech_valley".into(), 1);
        self.add_unidirectional("architecture_foundry".into(), "tech_valley".into(), 1);
        self.add_unidirectional("endurance_basin".into(), "tech_valley".into(), 1);
        self.add_unidirectional("collaboration_core".into(), "tech_valley".into(), 1);
        self.add_unidirectional("idea_hub".into(), "tech_valley".into(), 1);
        self.add_unidirectional("versatile_plaza".into(), "tech_valley".into(), 1);
    }
}

// --- Global Static Declaration ---

/// Globally available LocationGraph, containing both location data and connection costs.
/// Initialized once when first accessed.
pub static LOCATION_GRAPH: Lazy<LocationGraph> = Lazy::new(|| {
    let mut graph = LocationGraph::new();
    graph.initialize_default_connections_and_locations();
    graph
});

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    // Ensure HashMap is in scope for tests

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

    // --- LocationGraph Tests ---
    #[test]
    fn new_location_graph_is_empty() {
        let graph = LocationGraph::new();
        assert!(graph.connections.is_empty());
        assert!(graph.locations.is_empty());
    }

    #[test]
    fn add_unidirectional_connection() {
        let mut graph = LocationGraph::new();
        let loc_a = LocationId::from("loc_a");
        let loc_b = LocationId::from("loc_b");

        graph.add_unidirectional(loc_a.clone(), loc_b.clone(), 10);

        assert_eq!(graph.get_cost(&loc_a, &loc_b), Some(10));
        assert_eq!(graph.get_cost(&loc_b, &loc_a), None); // Should not exist in reverse
    }

    #[test]
    fn add_bidirectional_connection() {
        let mut graph = LocationGraph::new();
        let loc_a = LocationId::from("loc_a");
        let loc_b = LocationId::from("loc_b");

        graph.add_bidirectional(loc_a.clone(), loc_b.clone(), 5);

        assert_eq!(graph.get_cost(&loc_a, &loc_b), Some(5));
        assert_eq!(graph.get_cost(&loc_b, &loc_a), Some(5));
    }

    #[test]
    fn get_cost_existing_and_non_existing() {
        let mut graph = LocationGraph::new();
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
        let mut graph = LocationGraph::new();
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

        let neighbors_b = graph.get_neighbors_with_costs(&loc_b);
        // loc_b was only added as a destination, not a source of connections.
        // Therefore, it should not have an entry in the primary `connections` map.
        assert!(neighbors_b.is_none(), "loc_b should have no outgoing connections defined, so get_neighbors_with_costs should return None");

        let neighbors_d = graph.get_neighbors_with_costs(&loc_d);
        assert!(neighbors_d.is_none(), "loc_d was never added as a 'from' node, so get_neighbors_with_costs should return None");
    }

    #[test]
    fn get_location_test() {
        let mut graph = LocationGraph::new();
        let loc_id_a = LocationId::from("loc_a");
        let loc_a_data = Location {
            id: loc_id_a.clone(),
            skew_name: "SkewA",
            location_name: "Location A",
            hint: "Hint A",
            icon: "icon-a",
            loc_type: LocationType::Skewed,
        };
        graph.locations.insert(loc_id_a.clone(), loc_a_data.clone());

        let retrieved_loc = graph.get_location(&loc_id_a);
        assert!(retrieved_loc.is_some());
        assert_eq!(retrieved_loc.unwrap().id, loc_id_a);
        assert_eq!(retrieved_loc.unwrap().location_name, "Location A");

        let loc_id_b = LocationId::from("loc_b");
        assert!(graph.get_location(&loc_id_b).is_none());
    }

    #[test]
    fn initialize_default_connections_and_locations_creates_expected_graph() {
        let mut graph = LocationGraph::new();
        graph.initialize_default_connections_and_locations();

        // --- Test Locations ---
        assert_eq!(graph.locations.len(), 8); // Check total number of locations

        let precision_spire_id: LocationId = "precision_spire".into();
        let versatile_plaza_id: LocationId = "versatile_plaza".into();
        let tech_valley_id: LocationId = "tech_valley".into();

        // Spot check a few locations and their data
        let precision_spire_loc = graph.get_location(&precision_spire_id);
        assert!(precision_spire_loc.is_some());
        let precision_spire_loc = precision_spire_loc.unwrap();
        assert_eq!(precision_spire_loc.id, precision_spire_id);
        assert_eq!(precision_spire_loc.skew_name, "Precision Core");
        assert_eq!(precision_spire_loc.location_name, "Precision Spire");
        assert_eq!(precision_spire_loc.loc_type, LocationType::Skewed);

        let versatile_plaza_loc = graph.get_location(&versatile_plaza_id);
        assert!(versatile_plaza_loc.is_some());
        assert_eq!(versatile_plaza_loc.unwrap().loc_type, LocationType::Versatile);

        let tech_valley_loc = graph.get_location(&tech_valley_id);
        assert!(tech_valley_loc.is_some());
        assert_eq!(tech_valley_loc.unwrap().loc_type, LocationType::Hub);


        // --- Test Connections ---
        let discipline_stack_id: LocationId = "discipline_stack".into();
        let architecture_foundry_id: LocationId = "architecture_foundry".into();

        // Test a few circular connections (bidirectional)
        assert_eq!(graph.get_cost(&precision_spire_id, &discipline_stack_id), Some(2));
        assert_eq!(graph.get_cost(&discipline_stack_id, &precision_spire_id), Some(2));

        // Test a few skewed to versatile connections (bidirectional)
        assert_eq!(graph.get_cost(&precision_spire_id, &versatile_plaza_id), Some(3));
        assert_eq!(graph.get_cost(&versatile_plaza_id, &precision_spire_id), Some(3));

        // Test connections into Tech Valley (unidirectional)
        assert_eq!(graph.get_cost(&precision_spire_id, &tech_valley_id), Some(1));
        assert_eq!(graph.get_cost(&versatile_plaza_id, &tech_valley_id), Some(1));

        // Test that connections FROM Tech Valley are NOT automatically created
        assert_eq!(graph.get_cost(&tech_valley_id, &precision_spire_id), None);
        assert_eq!(graph.get_cost(&tech_valley_id, &versatile_plaza_id), None);

        // Test a non-existent direct link in the circular graph that should not be there
        assert_eq!(graph.get_cost(&precision_spire_id, &architecture_foundry_id), None);
    }

    #[test]
    fn global_static_location_graph_is_initialized() {
        // Access the global static graph
        let graph = &*LOCATION_GRAPH; // Dereference Lazy to get LocationGraph

        // Perform similar checks as initialize_default_connections_and_locations
        // to ensure it was initialized correctly.
        assert_eq!(graph.locations.len(), 8);

        let precision_spire_id: LocationId = "precision_spire".into();
        let tech_valley_id: LocationId = "tech_valley".into();
        let versatile_plaza_id: LocationId = "versatile_plaza".into();


        let precision_spire_loc = graph.get_location(&precision_spire_id);
        assert!(precision_spire_loc.is_some());
        assert_eq!(precision_spire_loc.unwrap().location_name, "Precision Spire");

        assert_eq!(graph.get_cost(&precision_spire_id, &tech_valley_id), Some(1));
        assert_eq!(graph.get_cost(&versatile_plaza_id, &tech_valley_id), Some(1));
        assert_eq!(graph.get_cost(&tech_valley_id, &precision_spire_id), None);
    }
}