use chrono::DateTime;
use chrono::prelude::Utc;
use rand::distributions::{IndependentSample, Range};
use rand;
use uuid::Uuid;

#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum DieType {
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
    D100,
    Fate,
    Other,
}

/// Determine the minimum number to roll based on the die type
fn get_die_min(die: &DieType) -> i16 {
    match die {
        &DieType::D4 => 1,
        &DieType::D6 => 1,
        &DieType::D8 => 1,
        &DieType::D10 => 1,
        &DieType::D12 => 1,
        &DieType::D20 => 1,
        &DieType::D100 => 1,
        &DieType::Fate => -1,
        &DieType::Other => 0,
    }
}

/// Determine the minimum number to roll based on the die type
fn get_die_max(die: &DieType) -> i16 {
    match die {
        &DieType::D4 => 4,
        &DieType::D6 => 6,
        &DieType::D8 => 8,
        &DieType::D10 => 10,
        &DieType::D12 => 12,
        &DieType::D20 => 20,
        &DieType::D100 => 100,
        &DieType::Fate => 1,
        &DieType::Other => 0,
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Die {
    /// Unique identifier of the die
    pub id: String,

    /// If the die was re-rolled, it will have a child
    pub child: Option<String>,

    /// The type of die (e.g. d20, d100)
    pub die: DieType,

    /// If the die is dropped in the final roll
    pub is_dropped: bool,

    /// If the die has been exploded
    pub is_exploded: bool,

    /// If the die is dropped in the final roll
    pub is_rerolled: bool,

    /// If the die is successful when we have a comparison
    pub is_successful: bool,

    /// Maximum number to roll
    pub max: i16,

    /// Minimum number to roll
    pub min: i16,

    /// Custom sides
    pub sides: Option<Vec<i16>>,

    /// Timestamp of the roll
    pub timestamp: DateTime<Utc>,

    /// The determined value of the dice roll
    pub value: i16,
}

impl Die {
    pub fn new (die: DieType) -> Die {
        Die {
            id: Uuid::new_v4().to_string(),
            child: None,
            die,
            is_dropped: false,
            is_exploded: false,
            is_rerolled: false,
            is_successful: false,
            max: get_die_max(&die),
            min: get_die_min(&die),
            sides: None,
            timestamp: Utc::now(),
            value: 0,
        }
    }

    /// Drop the die from the final roll
    pub fn drop(&mut self) {
        self.is_dropped = true
    }

    /// Mark the die as exploded.
    pub fn exploded (&mut self, die: &Die) {
        self.is_exploded = true;
        let id = &die.id;
        self.child = Some(id.to_owned());
    }

    /// Mark the die as successful to the a comparison
    pub fn success(&mut self) {
        self.is_successful = true
    }

    pub fn rerolled (&mut self, die: &Die) {
        self.is_rerolled = true;
        let id = &die.id;
        self.child = Some(id.to_owned());
    }

    /// Roll the die, generating a random number and calculating any modifiers
    pub fn roll(&mut self) -> &Die {
        // generate a random number
        match &self.sides {
            &Some(ref sides) => {
                let between = Range::new(0, sides.len());
                let mut rng = rand::thread_rng();
                let idx = between.ind_sample(&mut rng);
                let roll = sides[idx];
                self.value = roll;
                self.is_successful = true;
            },
            &None => {
                let between = Range::new(self.min, self.max);
                let mut rng = rand::thread_rng();
                let roll = between.ind_sample(&mut rng);
                self.value = roll;
                self.is_successful = true;
            }
        }
        self
    }

    pub fn set_min(&mut self, min: i16) {
        self.min = min;
    }

    pub fn set_max(&mut self, max: i16) {
        self.max = max;
    }
}

#[test]
fn it_can_create_dice() {
    // Create some random dice
    let d20 = Die::new(DieType::D20);
    assert_eq!(d20.die, DieType::D20);
    assert_eq!(d20.min, 1);
    assert_eq!(d20.max, 20);

    let d4 = Die::new(DieType::D4);
    assert_eq!(d4.die, DieType::D4);
    assert_eq!(d4.min, 1);
    assert_eq!(d4.max, 4);

    let fate = Die::new(DieType::Fate);
    assert_eq!(fate.die, DieType::Fate);
    assert_eq!(fate.min, -1);
    assert_eq!(fate.max, 1);
}

#[test]
fn it_can_set_die_min() {
    let mut custom = Die::new(DieType::Other);
    custom.set_min(-5);
    assert_eq!(custom.die, DieType::Other);
    assert_eq!(custom.min, -5);
}

#[test]
fn it_can_set_die_max() {
    let mut custom = Die::new(DieType::Other);
    custom.set_max(-50);
    assert_eq!(custom.die, DieType::Other);
    assert_eq!(custom.max, -50);
}

#[test]
fn it_can_set_die_exploded() {
    let mut custom = Die::new(DieType::D20);
    let mut child = Die::new(DieType::D20);
    custom.exploded(child);
    assert_eq!(custom.child, child.id);
    assert_eq!(custom.is_exploded, true);
}

#[test]
fn it_can_roll_die() {
    let mut die = Die::new(DieType::D20);
    die.roll();
    assert!(die.value >= 1);
    assert!(die.value <= 20);

    let mut custom = Die::new(DieType::Other);
    custom.set_max(-5);
    custom.set_min(-8);
    custom.roll();
    assert!(custom.value >= -8);
    assert!(custom.value <= -5);
}

#[test]
fn it_can_roll_custom_sides() {
    let mut die = Die::new(DieType::Other);
    die.sides = Some(vec![2, 4, 6, 8, 10]);
    die.roll();
    assert_ne!(die.value, 0);
    assert_eq!(die.value % 2, 0);
}
