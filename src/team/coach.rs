use crate::game::context::context::GameContext;
use crate::team::playcall::{
    OffensivePlayCall,
    OffensivePlay,
    DefensivePlayCall,
    DefensivePlay,
    PlayCaller
};

use rand::Rng;

/// # OffensiveStyle enum
///
/// An `OffensiveStyle` represents a coaching style for offense
pub enum OffensiveStyle {
    Run,
    Balanced,
    Pass
}

/// # DefensiveStyle enum
///
/// A `DefensiveStyle` represents a coaching style for defense
pub enum DefensiveStyle {
    Coverage,
    Balanced,
    Blitz
}

/// # Coach struct
///
/// A `Coach` represents a football coach
pub struct Coach {
    pub name: String,
    pub aggressiveness: usize,
    pub clock_management: usize,
    pub intelligence: usize,
    pub offensive_style: OffensiveStyle,
    pub defensive_style: DefensiveStyle
}

impl Coach {
    /// Initialize a new coach given its attributes
    pub fn new(name: &str, aggressiveness: usize, clock_management: usize, intelligence: usize, offensive_style: OffensiveStyle, defensive_style: DefensiveStyle) -> Coach {
        Coach {
            name: String::from(name),
            aggressiveness: aggressiveness,
            clock_management: clock_management,
            intelligence: intelligence,
            offensive_style: offensive_style,
            defensive_style: defensive_style
        }
    }
}

impl PlayCaller for Coach {
    /// Expected to generate the playcaller's offensive play call
    fn generate_offensive_playcall(&self, context: &GameContext, is_home: bool, mut rng: &mut impl Rng) -> Box<dyn OffensivePlayCall> {
        /*Initialize probabilities for pass/run, inside/outside, pass depth*/
        let mut p_run: f64 = 0.5_f64;
        let mut p_run_sum_count: usize = 1_usize;
        let mut p_inside: f64 = 0.5_f64;
        let mut p_inside_sum_count: usize = 1_usize;
        let mut p_long: f64 = 0.5_f64;
        let mut p_long_sum_count: usize = 1_usize;

        /*Initialize helper function for adjusting probabilities*/
        fn inc_prob(p: f64, sum_count: usize, inc: f64) -> (f64, usize) {
            let mut tmp_p: f64 = p * (sum_count as f64);
            tmp_p += inc;
            tmp_p /= (sum_count + 1_usize) as f64;
            (tmp_p, (sum_count + 1_usize))
        }

        /*Tweak probabilities based on game context & coaching style*/
        // Tweak run probability based on coaching style
        match self.offensive_style {
            OffensiveStyle::Run => {
                (p_run, p_run_sum_count) = inc_prob(p_run, p_run_sum_count, 1_f64);
            },
            OffensiveStyle::Balanced => {
                (p_run, p_run_sum_count) = inc_prob(p_run, p_run_sum_count, 0.5_f64);
            },
            OffensiveStyle::Pass => {
                (p_run, p_run_sum_count) = inc_prob(p_run, p_run_sum_count, 0_f64);
            }
        }

        // Tweak inside & depth based on aggressiveness
        (p_inside, p_inside_sum_count) = inc_prob(p_inside, p_inside_sum_count, 0.1_f64 * self.aggressiveness as f64);
        (p_long, p_long_sum_count) = inc_prob(p_long, p_long_sum_count, 0.1_f64 * self.aggressiveness as f64);

        // Tweak run probability based on winning/losing
        if is_home && context.score.home_team_score > context.score.away_team_score {
            (p_run, p_run_sum_count) = inc_prob(p_run, p_run_sum_count, 1_f64);
        } else if !is_home && context.score.home_team_score < context.score.away_team_score {
            (p_run, p_run_sum_count) = inc_prob(p_run, p_run_sum_count, 1_f64);
        } else if is_home && context.score.home_team_score < context.score.away_team_score {
            (p_run, p_run_sum_count) = inc_prob(p_run, p_run_sum_count, 0_f64);
        } else if !is_home && context.score.home_team_score > context.score.away_team_score {
            (p_run, p_run_sum_count) = inc_prob(p_run, p_run_sum_count, 0_f64);
        } else {
            (p_run, p_run_sum_count) = inc_prob(p_run, p_run_sum_count, 0.5_f64);
        }

        /*Randomly generate a play call*/
        Box::new(OffensivePlay::generate(p_run, p_inside, p_long, &mut rng))
    }

    /// Expected to generate the playcaller's defensive play call
    fn generate_defensive_playcall(&self, context: &GameContext, is_home: bool, mut rng: &mut impl Rng) -> Box<dyn DefensivePlayCall> {
        /*Initialize probabilities for blitz/coverage, man/zone, zone depth, blitz aggressiveness*/
        let mut p_blitz: f64 = 0.5_f64;
        let mut p_blitz_sum_count: usize = 1_usize;
        let mut p_all_out_blitz: f64 = 0.5_f64;
        let mut p_all_out_blitz_sum_count: usize = 1_usize;
        let mut p_zone: f64 = 0.5_f64;
        let mut p_zone_sum_count: usize = 1_usize;
        let mut p_zone_depth: f64 = 0.5_f64;
        let mut p_zone_depth_sum_count: usize = 1_usize;

        /*Initialize helper function for adjusting probabilities*/
        fn inc_prob(p: f64, sum_count: usize, inc: f64) -> (f64, usize) {
            let mut tmp_p: f64 = p * (sum_count as f64);
            tmp_p += inc;
            tmp_p /= (sum_count + 1_usize) as f64;
            (tmp_p, (sum_count + 1_usize))
        }

        /*Tweak probabilities based on game context & coaching style*/
        // Tweak run probability based on coaching style
        match self.defensive_style {
            DefensiveStyle::Blitz => {
                (p_blitz, p_blitz_sum_count) = inc_prob(p_blitz, p_blitz_sum_count, 1_f64);
            },
            DefensiveStyle::Balanced => {
                (p_blitz, p_blitz_sum_count) = inc_prob(p_blitz, p_blitz_sum_count, 0.5_f64);
            },
            DefensiveStyle::Coverage => {
                (p_blitz, p_blitz_sum_count) = inc_prob(p_blitz, p_blitz_sum_count, 0_f64);
            }
        }

        // Tweak blitz aggressiveness and zone depth based on aggressiveness
        (p_all_out_blitz, p_all_out_blitz_sum_count) = inc_prob(p_all_out_blitz, p_all_out_blitz_sum_count, 0.1_f64 * self.aggressiveness as f64);
        (p_zone_depth, p_zone_depth_sum_count) = inc_prob(p_zone_depth, p_zone_depth_sum_count, 0.1_f64 * self.aggressiveness as f64);

        // Tweak blitz probability based on winning/losing
        if is_home && context.score.home_team_score > context.score.away_team_score {
            (p_blitz, p_blitz_sum_count) = inc_prob(p_blitz, p_blitz_sum_count, 1_f64);
        } else if !is_home && context.score.home_team_score < context.score.away_team_score {
            (p_blitz, p_blitz_sum_count) = inc_prob(p_blitz, p_blitz_sum_count, 1_f64);
        } else if is_home && context.score.home_team_score < context.score.away_team_score {
            (p_blitz, p_blitz_sum_count) = inc_prob(p_blitz, p_blitz_sum_count, 0_f64);
        } else if !is_home && context.score.home_team_score > context.score.away_team_score {
            (p_blitz, p_blitz_sum_count) = inc_prob(p_blitz, p_blitz_sum_count, 0_f64);
        } else {
            (p_blitz, p_blitz_sum_count) = inc_prob(p_blitz, p_blitz_sum_count, 0.5_f64);
        }

        /*Randomly generate a play call*/
        Box::new(DefensivePlay::generate(p_blitz, p_all_out_blitz, p_zone, p_zone_depth, &mut rng))
    }
}