use crate::game::context::context::GameContext;

use std::fmt;
use std::fmt::Display;
use rand::Rng;
use rand::distributions::{Distribution, Bernoulli};
use statrs::distribution::Beta;

/// # OffensivePlayCallDisplay trait
///
/// An `OffensivePlayCallDisplay` implementation represents an offensive
/// play call which also implements std::fmt::Display
pub trait OffensivePlayCallDisplay: OffensivePlayCall + Display {}
impl<OffensivePlay: OffensivePlayCall + Display> OffensivePlayCallDisplay for OffensivePlay {}

/// # DefensivePlayCallDisplay trait
///
/// A `DefensivePlayCallDisplay` implementation represents an offensive
/// play call which also implements std::fmt::Display
pub trait DefensivePlayCallDisplay: DefensivePlayCall + Display {}
impl<DefensivePlay: DefensivePlayCall + Display> DefensivePlayCallDisplay for DefensivePlay {}

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

    /// Get the level of blocking for this play as a usize from 1 to 10
    pub fn get_blocking(&self) -> usize {
        // Match the play and return the blocking for the play
        match self {
            OffensivePlay::InsideRun => 8_usize,
            OffensivePlay::OutsideRun => 7_usize,
            OffensivePlay::ShortPass => 6_usize,
            OffensivePlay::MediumPass => 5_usize,
            OffensivePlay::LongPass => 4_usize,
        }
    }

    /// Get the level of short field aim for this play as a usize from 1 to 10
    pub fn get_short_aim(&self) -> usize {
        // Match the play and return the short field aim for the play
        match self {
            OffensivePlay::InsideRun => 9_usize,
            OffensivePlay::OutsideRun => 8_usize,
            OffensivePlay::ShortPass => 9_usize,
            OffensivePlay::MediumPass => 5_usize,
            OffensivePlay::LongPass => 3_usize,
        }
    }

    /// Get the level of medium field aim for this play as a usize from 1 to 10
    pub fn get_medium_aim(&self) -> usize {
        // Match the play and return the medium field aim for the play
        match self {
            OffensivePlay::InsideRun => 3_usize,
            OffensivePlay::OutsideRun => 4_usize,
            OffensivePlay::ShortPass => 5_usize,
            OffensivePlay::MediumPass => 9_usize,
            OffensivePlay::LongPass => 5_usize,
        }
    }

    /// Get the level of long field aim for this play as a usize from 1 to 10
    pub fn get_long_aim(&self) -> usize {
        // Match the play and return the long field aim for the play
        match self {
            OffensivePlay::InsideRun => 1_usize,
            OffensivePlay::OutsideRun => 2_usize,
            OffensivePlay::ShortPass => 3_usize,
            OffensivePlay::MediumPass => 5_usize,
            OffensivePlay::LongPass => 9_usize,
        }
    }

    /// Get the level of inside aim for this play as a usize from 1 to 10
    pub fn get_inside_aim(&self) -> usize {
        // Match the play and return the inside aim for the play
        match self {
            OffensivePlay::InsideRun => 9_usize,
            OffensivePlay::OutsideRun => 3_usize,
            OffensivePlay::ShortPass => 7_usize,
            OffensivePlay::MediumPass => 5_usize,
            OffensivePlay::LongPass => 3_usize,
        }
    }

    /// Get the level of outside aim for this play as a usize from 1 to 10
    pub fn get_outside_aim(&self) -> usize {
        // Match the play and return the outside aim for the play
        match self {
            OffensivePlay::InsideRun => 3_usize,
            OffensivePlay::OutsideRun => 9_usize,
            OffensivePlay::ShortPass => 3_usize,
            OffensivePlay::MediumPass => 5_usize,
            OffensivePlay::LongPass => 7_usize,
        }
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

impl fmt::Display for OffensivePlay {
    /// Format an `OffensivePlay` as a string
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let play_str = match self {
            OffensivePlay::InsideRun => "Inside Run",
            OffensivePlay::OutsideRun => "Outside Run",
            OffensivePlay::ShortPass => "Short Pass",
            OffensivePlay::MediumPass => "Medium Pass",
            OffensivePlay::LongPass => "Long Pass",
        };
        f.write_str(play_str)
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

    /// Get the level of blitzing for this play as a usize from 1 to 10
    pub fn get_blitzing(&self) -> usize {
        // Match the play and return the blitzing for the play
        match self {
            DefensivePlay::LightBlitz => 7_usize,
            DefensivePlay::AllOutBlitz => 9_usize,
            DefensivePlay::ManCoverage => 5_usize,
            DefensivePlay::ShortZone => 5_usize,
            DefensivePlay::MediumZone => 3_usize,
            DefensivePlay::LongZone => 1_usize
        }
    }

    /// Get the level of short field coverage for this play as a usize from 1 to 10
    pub fn get_short_coverage(&self) -> usize {
        // Match the play and return the short field coverage for the play
        match self {
            DefensivePlay::LightBlitz => 4_usize,
            DefensivePlay::AllOutBlitz => 3_usize,
            DefensivePlay::ManCoverage => 6_usize,
            DefensivePlay::ShortZone => 9_usize,
            DefensivePlay::MediumZone => 6_usize,
            DefensivePlay::LongZone => 2_usize
        }
    }

    /// Get the level of medium field coverage for this play as a usize from 1 to 10
    pub fn get_medium_coverage(&self) -> usize {
        // Match the play and return the medium field coverage for the play
        match self {
            DefensivePlay::LightBlitz => 3_usize,
            DefensivePlay::AllOutBlitz => 2_usize,
            DefensivePlay::ManCoverage => 6_usize,
            DefensivePlay::ShortZone => 6_usize,
            DefensivePlay::MediumZone => 9_usize,
            DefensivePlay::LongZone => 5_usize
        }
    }

    /// Get the level of long field coverage for this play as a usize from 1 to 10
    pub fn get_long_coverage(&self) -> usize {
        // Match the play and return the long field coverage for the play
        match self {
            DefensivePlay::LightBlitz => 2_usize,
            DefensivePlay::AllOutBlitz => 1_usize,
            DefensivePlay::ManCoverage => 5_usize,
            DefensivePlay::ShortZone => 2_usize,
            DefensivePlay::MediumZone => 5_usize,
            DefensivePlay::LongZone => 9_usize
        }
    }

    /// Get the level of inside coverage for this play as a usize from 1 to 10
    pub fn get_inside_coverage(&self) -> usize {
        // Match the play and return the inside coverage for this play
        match self {
            DefensivePlay::LightBlitz => 8_usize,
            DefensivePlay::AllOutBlitz => 8_usize,
            DefensivePlay::ManCoverage => 3_usize,
            DefensivePlay::ShortZone => 7_usize,
            DefensivePlay::MediumZone => 5_usize,
            DefensivePlay::LongZone => 3_usize
        }
    }

    /// Get the level of outside coverage for this play as a usize from 1 to 10
    pub fn get_outside_coverage(&self) -> usize {
        // Match the play and return the inside coverage for this play
        match self {
            DefensivePlay::LightBlitz => 3_usize,
            DefensivePlay::AllOutBlitz => 2_usize,
            DefensivePlay::ManCoverage => 6_usize,
            DefensivePlay::ShortZone => 5_usize,
            DefensivePlay::MediumZone => 7_usize,
            DefensivePlay::LongZone => 9_usize
        }
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

impl fmt::Display for DefensivePlay {
    /// Format a `DefensivePlay` as a string
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let play_str = match self {
            DefensivePlay::LightBlitz => "Light Blitz",
            DefensivePlay::AllOutBlitz => "All Out Blitz",
            DefensivePlay::ManCoverage => "Man Coverage",
            DefensivePlay::ShortZone => "Short Zone",
            DefensivePlay::MediumZone => "Medium Zone",
            DefensivePlay::LongZone => "Long Zone"
        };
        f.write_str(play_str)
    }
}

/// # PlayCaller trait
///
/// A `PlayCaller` implementation represents someone who calls plays for
/// a football team
pub trait PlayCaller {
    /// Expected to generate the playcaller's offensive play call
    fn generate_offensive_playcall(&self, context: &GameContext, is_home: bool, rng: &mut impl Rng) -> Box<dyn OffensivePlayCallDisplay>;

    /// Expected to generate the playcaller's defensive play call
    fn generate_defensive_playcall(&self, context: &GameContext, is_home: bool, rng: &mut impl Rng) -> Box<dyn DefensivePlayCallDisplay>;
}

/// # PlayComp struct
///
/// A `PlayComp` struct is an empty struct with implementations for
/// functionality for comparing play calls
pub struct PlayComp;
impl PlayComp {
    /// Initialize a PlayComp struct (empty struct)
    pub fn new() -> PlayComp {
        PlayComp{}
    }

    /// Determine the base likelihood of defensive line penetration given
    /// two play calls.  Returns an f64 ranging from 0 to 1, where 0
    /// indicates very unlikely and 1 indicates very likely.
    pub fn compare_penetration(&self, offensive_play_call: &Box<dyn OffensivePlayCallDisplay>, defensive_play_call: &Box<dyn DefensivePlayCallDisplay>) -> f64 {
        // Convert the offensive & defensive calls into plays
        let offensive_play: OffensivePlay = OffensivePlay::new(
            offensive_play_call.is_run(),
            offensive_play_call.is_inside_run(),
            offensive_play_call.get_pass_depth()
        );
        let defensive_play: DefensivePlay = DefensivePlay::new(
            defensive_play_call.is_blitz(),
            defensive_play_call.is_all_out_blitz(),
            defensive_play_call.is_zone_coverage(),
            defensive_play_call.get_zone_depth()
        );

        // Get the blitzing levels for each play
        let offensive_blocking: usize = offensive_play.get_blocking();
        let defensive_blitzing: usize = defensive_play.get_blitzing();

        // Compare the blitzing levels for each play & return
        let blitzing_spread: isize = (offensive_blocking as isize) - (defensive_blitzing as isize) + 9_isize;
        let blitzing_spread_normalized: f64 = (blitzing_spread as f64) / 18.0_f64;
        blitzing_spread_normalized
    }

    /// Determine the compatibility of the plays were the offense to target
    /// the short field.  Returns an f64 ranging from 0 to 1, where 0 indicates
    /// unlikely offensive success while 1 indicates likely offensive success.
    pub fn compare_short_compatibility(&self, offensive_play_call: &Box<dyn OffensivePlayCallDisplay>, defensive_play_call: &Box<dyn DefensivePlayCallDisplay>) -> f64 {
        // Convert the offensive & defensive calls into plays
        let offensive_play: OffensivePlay = OffensivePlay::new(
            offensive_play_call.is_run(),
            offensive_play_call.is_inside_run(),
            offensive_play_call.get_pass_depth()
        );
        let defensive_play: DefensivePlay = DefensivePlay::new(
            defensive_play_call.is_blitz(),
            defensive_play_call.is_all_out_blitz(),
            defensive_play_call.is_zone_coverage(),
            defensive_play_call.get_zone_depth()
        );

        // Get the short field levels for each play
        let offensive_short_aim: usize = offensive_play.get_short_aim();
        let defensive_short_cov: usize = defensive_play.get_short_coverage();

        // Compare the short coverage levels for each play & return
        let short_spread: isize = (offensive_short_aim as isize) - (defensive_short_cov as isize) + 9_isize;
        let short_spread_normalized: f64 = (short_spread as f64) / 18.0_f64;
        short_spread_normalized
    }

    /// Determine the compatibility of the plays were the offense to target
    /// the medium field.  Returns an f64 ranging from 0 to 1, where 0 indicates
    /// unlikely offensive success while 1 indicates likely offensive success.
    pub fn compare_medium_compatibility(&self, offensive_play_call: &Box<dyn OffensivePlayCallDisplay>, defensive_play_call: &Box<dyn DefensivePlayCallDisplay>) -> f64 {
        // Convert the offensive & defensive calls into plays
        let offensive_play: OffensivePlay = OffensivePlay::new(
            offensive_play_call.is_run(),
            offensive_play_call.is_inside_run(),
            offensive_play_call.get_pass_depth()
        );
        let defensive_play: DefensivePlay = DefensivePlay::new(
            defensive_play_call.is_blitz(),
            defensive_play_call.is_all_out_blitz(),
            defensive_play_call.is_zone_coverage(),
            defensive_play_call.get_zone_depth()
        );

        // Get the medium field levels for each play
        let offensive_med_aim: usize = offensive_play.get_medium_aim();
        let defensive_med_cov: usize = defensive_play.get_medium_coverage();

        // Compare the medium coverage levels for each play & return
        let med_spread: isize = (offensive_med_aim as isize) - (defensive_med_cov as isize) + 9_isize;
        let med_spread_normalized: f64 = (med_spread as f64) / 18.0_f64;
        med_spread_normalized
    }

    /// Determine the compatibility of the plays were the offense to target
    /// the long field.  Returns an f64 ranging from 0 to 1, where 0 indicates
    /// unlikely offensive success while 1 indicates likely offensive success.
    pub fn compare_long_compatibility(&self, offensive_play_call: &Box<dyn OffensivePlayCallDisplay>, defensive_play_call: &Box<dyn DefensivePlayCallDisplay>) -> f64 {
        // Convert the offensive & defensive calls into plays
        let offensive_play: OffensivePlay = OffensivePlay::new(
            offensive_play_call.is_run(),
            offensive_play_call.is_inside_run(),
            offensive_play_call.get_pass_depth()
        );
        let defensive_play: DefensivePlay = DefensivePlay::new(
            defensive_play_call.is_blitz(),
            defensive_play_call.is_all_out_blitz(),
            defensive_play_call.is_zone_coverage(),
            defensive_play_call.get_zone_depth()
        );

        // Get the long field levels for each play
        let offensive_long_aim: usize = offensive_play.get_long_aim();
        let defensive_long_cov: usize = defensive_play.get_long_coverage();

        // Compare the long coverage levels for each play & return
        let long_spread: isize = (offensive_long_aim as isize) - (defensive_long_cov as isize) + 9_isize;
        let long_spread_normalized: f64 = (long_spread as f64) / 18.0_f64;
        long_spread_normalized
    }

    /// Determine the compatibility of the plays were the offense to target
    /// the inside.  Returns an f64 ranging from 0 to 1, where 0 indicates
    /// unlikely offensive success while 1 indicates likely offensive success.
    pub fn compare_inside_compatibility(&self, offensive_play_call: &Box<dyn OffensivePlayCallDisplay>, defensive_play_call: &Box<dyn DefensivePlayCallDisplay>) -> f64 {
        // Convert the offensive & defensive calls into plays
        let offensive_play: OffensivePlay = OffensivePlay::new(
            offensive_play_call.is_run(),
            offensive_play_call.is_inside_run(),
            offensive_play_call.get_pass_depth()
        );
        let defensive_play: DefensivePlay = DefensivePlay::new(
            defensive_play_call.is_blitz(),
            defensive_play_call.is_all_out_blitz(),
            defensive_play_call.is_zone_coverage(),
            defensive_play_call.get_zone_depth()
        );

        // Get the inside field levels for each play
        let offensive_in_aim: usize = offensive_play.get_inside_aim();
        let defensive_in_cov: usize = defensive_play.get_inside_coverage();

        // Compare the inside coverage levels for each play & return
        let in_spread: isize = (offensive_in_aim as isize) - (defensive_in_cov as isize) + 9_isize;
        let in_spread_normalized: f64 = (in_spread as f64) / 18.0_f64;
        in_spread_normalized
    }

    /// Determine the compatibility of the plays were the offense to target
    /// the outside.  Returns an f64 ranging from 0 to 1, where 0 indicates
    /// unlikely offensive success while 1 indicates likely offensive success.
    pub fn compare_outside_compatibility(&self, offensive_play_call: &Box<dyn OffensivePlayCallDisplay>, defensive_play_call: &Box<dyn DefensivePlayCallDisplay>) -> f64 {
        // Convert the offensive & defensive calls into plays
        let offensive_play: OffensivePlay = OffensivePlay::new(
            offensive_play_call.is_run(),
            offensive_play_call.is_inside_run(),
            offensive_play_call.get_pass_depth()
        );
        let defensive_play: DefensivePlay = DefensivePlay::new(
            defensive_play_call.is_blitz(),
            defensive_play_call.is_all_out_blitz(),
            defensive_play_call.is_zone_coverage(),
            defensive_play_call.get_zone_depth()
        );

        // Get the outside field levels for each play
        let offensive_out_aim: usize = offensive_play.get_outside_aim();
        let defensive_out_cov: usize = defensive_play.get_outside_coverage();

        // Compare the outside coverage levels for each play & return
        let out_spread: isize = (offensive_out_aim as isize) - (defensive_out_cov as isize) + 9_isize;
        let out_spread_normalized: f64 = (out_spread as f64) / 18.0_f64;
        out_spread_normalized
    }
}