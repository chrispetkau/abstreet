//! Integration tests

use std::fs::File;
use std::io::Write;

use abstutil::{MapName, Timer};
use geom::Duration;
use map_model::Map;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    test_map_importer()?;
    check_proposals()?;
    smoke_test()?;
    Ok(())
}

/// Test the map pipeline by importing simple, handcrafted .osm files, then emitting goldenfiles
/// that summarize part of the generated map. Keep the goldenfiles under version control to notice
/// when they change. The goldenfiles (and changes to them) themselves aren't easy to understand,
/// but the test maps are.
fn test_map_importer() -> Result<(), std::io::Error> {
    // TODO It's kind of a hack to reference the crate's directory relative to the data dir.
    for path in abstutil::list_dir(abstutil::path("../tests/input")) {
        let map = import_map(path);
        // Enable to debug the result wih the normal GUI
        if false {
            map.save();
        }
        println!("Producing goldenfiles for {}", map.get_name().describe());
        dump_turn_goldenfile(&map)?;
    }
    Ok(())
}

/// Run the contents of a .osm through the full map importer with default options.
fn import_map(path: String) -> Map {
    let mut timer = abstutil::Timer::new("convert synthetic map");
    let raw = convert_osm::convert(
        convert_osm::Options {
            name: MapName::new("oneshot", &abstutil::basename(&path)),
            osm_input: path,
            clip: None,
            map_config: map_model::MapConfig {
                driving_side: map_model::DrivingSide::Right,
                bikes_can_use_bus_lanes: true,
                inferred_sidewalks: true,
            },
            onstreet_parking: convert_osm::OnstreetParking::JustOSM,
            public_offstreet_parking: convert_osm::PublicOffstreetParking::None,
            private_offstreet_parking: convert_osm::PrivateOffstreetParking::FixedPerBldg(0),
            elevation: None,
            include_railroads: true,
        },
        &mut timer,
    );
    let map = Map::create_from_raw(raw, true, true, &mut timer);
    map
}

/// Verify what turns are generated by writing (from lane, to lane, turn type).
fn dump_turn_goldenfile(map: &Map) -> Result<(), std::io::Error> {
    let path = abstutil::path(format!("../tests/goldenfiles/{}.txt", map.get_name().map));
    let mut f = File::create(path)?;
    for (_, t) in map.all_turns() {
        writeln!(f, "{} is a {:?}", t.id, t.turn_type)?;
    }
    Ok(())
}

/// Simulate an hour on every map.
fn smoke_test() -> Result<(), std::io::Error> {
    let mut timer = Timer::new("run a smoke-test for all maps");
    for name in MapName::list_all_maps() {
        let map = map_model::Map::new(name.path(), &mut timer);
        let scenario = if map.get_city_name() == "seattle" {
            abstutil::read_binary(abstutil::path_scenario(&name, "weekday"), &mut timer)
        } else {
            let mut rng = sim::SimFlags::for_test("smoke_test").make_rng();
            sim::ScenarioGenerator::proletariat_robot(&map, &mut rng, &mut timer)
        };

        let mut opts = sim::SimOptions::new("smoke_test");
        opts.alerts = sim::AlertHandler::Silence;
        let mut sim = sim::Sim::new(&map, opts, &mut timer);
        // Bit of an abuse of this, but just need to fix the rng seed.
        let mut rng = sim::SimFlags::for_test("smoke_test").make_rng();
        scenario.instantiate(&mut sim, &map, &mut rng, &mut timer);
        sim.timed_step(&map, Duration::hours(1), &mut None, &mut timer);

        if (name.city == "seattle"
            && vec!["downtown", "lakeslice", "montlake", "udistrict"].contains(&name.map.as_str()))
            || name == MapName::new("krakow", "center")
        {
            dump_route_goldenfile(&map)?;
        }
    }
    Ok(())
}

/// Describe all public transit routes and keep under version control to spot diffs easily.
fn dump_route_goldenfile(map: &map_model::Map) -> Result<(), std::io::Error> {
    let path = abstutil::path(format!(
        "route_goldenfiles/{}.txt",
        map.get_name().as_filename()
    ));
    let mut f = File::create(path)?;
    for br in map.all_bus_routes() {
        writeln!(
            f,
            "{} from {} to {:?}",
            br.osm_rel_id, br.start, br.end_border
        )?;
        for bs in &br.stops {
            let bs = map.get_bs(*bs);
            writeln!(
                f,
                "  {}: {} driving, {} sidewalk",
                bs.name, bs.driving_pos, bs.sidewalk_pos
            )?;
        }
    }
    Ok(())
}

/// Verify all edits under version control can be correctly apply to their map.
fn check_proposals() -> Result<(), String> {
    let mut timer = Timer::new("check all proposals");
    for name in abstutil::list_all_objects(abstutil::path("system/proposals")) {
        match abstutil::maybe_read_json::<map_model::PermanentMapEdits>(
            abstutil::path(format!("system/proposals/{}.json", name)),
            &mut timer,
        ) {
            Ok(perma) => {
                let map = map_model::Map::new(perma.map_name.path(), &mut timer);
                if let Err(err) = map_model::PermanentMapEdits::from_permanent(perma, &map) {
                    return Err(format!("{} is out-of-date: {}", name, err));
                }
            }
            Err(err) => {
                return Err(format!("{} JSON is broken: {}", name, err));
            }
        }
    }
    Ok(())
}
