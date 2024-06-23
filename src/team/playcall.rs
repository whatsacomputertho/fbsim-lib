use crate::game::context::context::GameContext;

use rand::Rng;
use rand::distributions::{Distribution, Bernoulli};
use statrs::distribution::Beta;

/// # PlayDepth enum
///
/// The PlayDepth enum represents the depth of a pass or pass defense play
pub enum PlayDepth {
    Short,
    Medium,
    Long
}

impl PlayDepth {
    /// Initialize a new play depth given a floating point number
    pub fn new(seed: f64) -> PlayDepth {
        if seed < 0.33_f64 {
            return PlayDepth::Short;
        } else if seed < 0.67_f64 {
            return PlayDepth::Medium;
        } else {
            return PlayDepth::Long;
        }
    }
}

/// # OffensivePlayCall trait
///
/// A `OffensivePlayCall` implementation represents an offensive play call
pub trait OffensivePlayCall {
    /// Expected to determine if the play call is a run or a pass
    fn is_run(&self) -> bool;

    /// Expected to determine if the run play is inside or outside
    fn is_inside_run(&self) -> bool;

    /// Expected to return the depth of the pass play
    fn get_pass_depth(&self) -> PlayDepth;
}

/// # DefensivePlayCall trait
///
/// A `DefensivePlayCall` implementation represents a defensive play call
pub trait DefensivePlayCall {
    /// Expected to determine if the play call is a blitz or coverage
    fn is_blitz(&self) -> bool;

    /// Expected to determine if the coverage is man or zone
    fn is_zone_coverage(&self) -> bool;

    /// Expected to return the depth of the zone coverage
    fn get_zone_depth(&self) -> PlayDepth;

    /// Expected to determine if the blitz is all-out or partial
    fn is_all_out_blitz(&self) -> bool;
}

/// # OffensivePlay enum
///
/// The OffensivePlay enum represents a play called by the offense
pub enum OffensivePlay {
    InsideRun,
    OutsideRun,
    ShortPass,
    MediumPass,
    LongPass
}

impl OffensivePlay {
    /// Initializes an offensive play call given whether it is a run,
    /// whether it is inside, and whether it is long
    pub fn new(is_run: bool, is_inside_run: bool, pass_depth: PlayDepth) -> OffensivePlay {
        if is_run {
            if is_inside_run {
                return OffensivePlay::InsideRun;
            } else {
                return OffensivePlay::OutsideRun;
            }
        } else {
            return match pass_depth {
                PlayDepth::Short => OffensivePlay::ShortPass,
                PlayDepth::Medium => OffensivePlay::MediumPass,
                PlayDepth::Long => OffensivePlay::LongPass
            };
        }
    }

    /// Generates a new offensive play given a run/pass probability,
    /// an inside/outside probability, and a depth probability
    pub fn generate(p_run: f64, p_inside: f64, p_depth: f64, mut rng: &mut impl Rng) -> OffensivePlay {
        // Generate whether a run play was called
        let dst_run: Bernoulli = Bernoulli::new(p_run).unwrap();
        let is_run: bool = dst_run.sample(&mut rng);

        // Generate a run direction for if a run was called
        let dst_inside: Bernoulli = Bernoulli::new(p_depth).unwrap();
        let is_inside_run: bool = dst_inside.sample(&mut rng);

        // Generate a pass depth for if a pass was called
        let dst_depth = Beta::new(p_depth, 5.0_f64).unwrap();
        let depth_seed = dst_depth.sample(&mut rng);
        let depth: PlayDepth = PlayDepth::new(depth_seed);

        // Initialize and return the offensive play
        OffensivePlay::new(is_run, is_inside_run, depth)
    }
}

impl OffensivePlayCall for OffensivePlay {
    /// Determines if the play call is a run or a pass
    fn is_run(&self) -> bool {
        match self {
            OffensivePlay::InsideRun => true,
            OffensivePlay::OutsideRun => true,
            _ => false,
        }
    }

    /// Determines if the run play is inside or outside
    fn is_inside_run(&self) -> bool {
        match self {
            OffensivePlay::InsideRun => true,
            _ => false,
        }
    }

    /// Returns the depth of the pass play
    fn get_pass_depth(&self) -> PlayDepth {
        match self {
            OffensivePlay::MediumPass => PlayDepth::Medium,
            OffensivePlay::LongPass => PlayDepth::Long,
            _ => PlayDepth::Short,
        }
    }
}

/// # DefensivePlay enum
///
/// The `DefensivePlay` enum represents a play called by the defense
pub enum DefensivePlay {
    LightBlitz,
    AllOutBlitz,
    ManCoverage,
    ShortZone,
    MediumZone,
    LongZone
}

impl DefensivePlay {
    /// Initializes an offensive play call given whether it is a run,
    /// whether it is inside, and whether it is long
    pub fn new(is_blitz: bool, is_all_out_blitz: bool, is_zone: bool, zone_depth: PlayDepth) -> DefensivePlay {
        if is_blitz {
            if is_all_out_blitz {
                return DefensivePlay::AllOutBlitz;
            } else {
                return DefensivePlay::LightBlitz;
            }
        } else {
            if is_zone {
                return match zone_depth {
                    PlayDepth::Short => DefensivePlay::ShortZone,
                    PlayDepth::Medium => DefensivePlay::MediumZone,
                    PlayDepth::Long => DefensivePlay::LongZone
                };
            } else {
                return DefensivePlay::ManCoverage;
            }
        }
    }

    /// Generates a new offensive play given a run/pass probability,
    /// an inside/outside probability, and a depth probability
    pub fn generate(p_blitz: f64, p_all_out_blitz: f64, p_zone: f64, p_zone_depth: f64, mut rng: &mut impl Rng) -> DefensivePlay {
        // Generate whether a blitz was called
        let dst_blitz: Bernoulli = Bernoulli::new(p_blitz).unwrap();
        let is_blitz: bool = dst_blitz.sample(&mut rng);

        // Generate whether the blitz is light or all-out
        let dst_all_out: Bernoulli = Bernoulli::new(p_all_out_blitz).unwrap();
        let is_all_out_blitz: bool = dst_all_out.sample(&mut rng);

        // Generate whether zone coverage was called
        let dst_zone: Bernoulli = Bernoulli::new(p_zone).unwrap();
        let is_zone: bool = dst_zone.sample(&mut rng);

        // Generate a zone depth for if zone coverage was called
        let dst_depth = Beta::new(p_zone_depth, 5.0_f64).unwrap();
        let depth_seed = dst_depth.sample(&mut rng);
        let depth: PlayDepth = PlayDepth::new(depth_seed);

        // Initialize and return the defensive play
        DefensivePlay::new(is_blitz, is_all_out_blitz, is_zone, depth)
    }
}

impl DefensivePlayCall for DefensivePlay {
    /// Determines if the play call is a blitz or coverage
    fn is_blitz(&self) -> bool {
        match self {
            DefensivePlay::LightBlitz => true,
            DefensivePlay::AllOutBlitz => true,
            _ => false,
        }
    }

    /// Determines if the coverage is man or zone
    fn is_zone_coverage(&self) -> bool {
        match self {
            DefensivePlay::ManCoverage => false,
            _ => true,
        }
    }

    /// Expected to return the depth of the zone coverage
    fn get_zone_depth(&self) -> PlayDepth {
        match self {
            DefensivePlay::MediumZone => PlayDepth::Medium,
            DefensivePlay::LongZone => PlayDepth::Long,
            _ => PlayDepth::Short,
        }
    }

    /// Expected to determine if the blitz is all-out or partial
    fn is_all_out_blitz(&self) -> bool {
        match self {
            DefensivePlay::AllOutBlitz => true,
            _ => false,
        }
    }
}

/// # PlayCaller trait
///
/// A `PlayCaller` implementation represents someone who calls plays for
/// a football team
pub trait PlayCaller {
    /// Expected to generate the playcaller's offensive play call
    fn generate_offensive_playcall(&self, context: &GameContext, is_home: bool, rng: &mut impl Rng) -> Box<dyn OffensivePlayCall>;

    /// Expected to generate the playcaller's defensive play call
    fn generate_defensive_playcall(&self, context: &GameContext, is_home: bool, rng: &mut impl Rng) -> Box<dyn DefensivePlayCall>;
}