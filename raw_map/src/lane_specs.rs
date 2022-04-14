use osm2lanes::road::Designated;
use osm2lanes::tag::TagsWrite;

use abstutil::Tags;
use geom::Distance;

use crate::{Direction, DrivingSide, LaneSpec, LaneType, MapConfig};

pub fn get_lane_specs_ltr(orig_tags: &Tags, cfg: &MapConfig) -> Vec<LaneSpec> {
    let mut tags = osm2lanes::tag::Tags::default();
    for (k, v) in orig_tags.inner() {
        // Workaround common incorrect tagging
        if k == "sidewalk" && v == "none" {
            tags.checked_insert("sidewalk", "no").unwrap();
        } else {
            tags.checked_insert(k.to_string(), v).unwrap();
        }
    }
    let locale = osm2lanes::locale::Config::new()
        .driving_side(match cfg.driving_side {
            DrivingSide::Right => osm2lanes::locale::DrivingSide::Right,
            DrivingSide::Left => osm2lanes::locale::DrivingSide::Left,
        })
        .build();
    let mut config = osm2lanes::transform::TagsToLanesConfig::default();
    config.error_on_warnings = false;
    config.include_separators = true;

    match osm2lanes::transform::tags_to_lanes(&tags, &locale, &config) {
        Ok(output) => {
            let mut result = output
                .road
                .lanes
                .into_iter()
                .map(|lane| transform(lane, &locale))
                .flatten()
                .collect::<Vec<_>>();

            // Fix direction on outer lanes
            for (idx, lane) in result.iter_mut().enumerate() {
                if lane.lt == LaneType::Sidewalk || lane.lt == LaneType::Shoulder {
                    if idx == 0 {
                        lane.dir = if cfg.driving_side == DrivingSide::Right {
                            Direction::Back
                        } else {
                            Direction::Fwd
                        };
                    } else {
                        // Assume last
                        lane.dir = if cfg.driving_side == DrivingSide::Right {
                            Direction::Fwd
                        } else {
                            Direction::Back
                        };
                    }
                }
            }

            result
        }
        Err(err) => {
            error!("Broke on {:?}: {}", orig_tags, err);
            vec![LaneSpec {
                lt: LaneType::Driving,
                dir: Direction::Fwd,
                width: Distance::meters(1.0),
            }]
        }
    }
}

fn transform(lane: osm2lanes::road::Lane, locale: &osm2lanes::locale::Locale) -> Option<LaneSpec> {
    use osm2lanes::road::Lane;

    let mut lt;
    let dir;
    match lane {
        Lane::Travel {
            direction,
            designated,
            ..
        } => {
            lt = match designated {
                Designated::Foot => LaneType::Sidewalk,
                Designated::Motor => LaneType::Driving,
                Designated::Bicycle => LaneType::Biking,
                Designated::Bus => LaneType::Bus,
            };
            match direction {
                Some(direction) => match direction {
                    osm2lanes::road::Direction::Forward => {
                        dir = Direction::Fwd;
                    }
                    osm2lanes::road::Direction::Backward => {
                        dir = Direction::Back;
                    }
                    osm2lanes::road::Direction::Both => {
                        assert_eq!(designated, Designated::Motor);
                        lt = LaneType::SharedLeftTurn;
                        dir = Direction::Fwd;
                    }
                },
                // Fix later
                None => {
                    dir = Direction::Fwd;
                }
            };
        }
        Lane::Shoulder { .. } => {
            lt = LaneType::Shoulder;
            // Fix later
            dir = Direction::Fwd;
        }
        Lane::Separator { .. } => {
            // TODO Barriers
            return None;
        }
        Lane::Parking {
            direction,
            designated: Designated::Motor,
            ..
        } => {
            lt = LaneType::Parking;
            dir = match direction {
                osm2lanes::road::Direction::Forward => Direction::Fwd,
                osm2lanes::road::Direction::Backward => Direction::Back,
                osm2lanes::road::Direction::Both => todo!("dir = both for parking"),
            }
        }
        _ => todo!("handle {:?}", lane),
    }
    let width = Distance::meters(lane.width(locale).val());
    Some(LaneSpec { lt, dir, width })
}
