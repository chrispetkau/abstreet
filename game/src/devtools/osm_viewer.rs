use abstutil::prettyprint_usize;
use geom::ArrowCap;
use map_model::osm;
use widgetry::{
    lctrl, Btn, Drawable, EventCtx, GeomBatch, GfxCtx, HorizontalAlignment, Key, Line, Outcome,
    Panel, State, Text, TextExt, VerticalAlignment, Widget,
};

use crate::app::App;
use crate::common::{CityPicker, Navigator};
use crate::game::{PopupMsg, Transition};
use crate::helpers::{nice_map_name, open_browser, ID};
use crate::options::OptionsPanel;
use crate::render::BIG_ARROW_THICKNESS;
use crate::sandbox::TurnExplorer;

pub struct Viewer {
    top_panel: Panel,
    fixed_object_outline: Option<Drawable>,
}

impl Viewer {
    pub fn new(ctx: &mut EventCtx, app: &mut App) -> Box<dyn State<App>> {
        app.primary.current_selection = None;

        Box::new(Viewer {
            top_panel: Panel::new(Widget::col(vec![
                Widget::row(vec![
                    Line("OpenStreetMap viewer").small_heading().draw(ctx),
                    Btn::plaintext("X")
                        .build(ctx, "close", Key::Escape)
                        .align_right(),
                ]),
                Widget::row(vec![
                    "Change map:".draw_text(ctx),
                    Btn::pop_up(ctx, Some(nice_map_name(app.primary.map.get_name()))).build(
                        ctx,
                        "change map",
                        lctrl(Key::L),
                    ),
                ]),
                Widget::row(vec![
                    Btn::svg_def("system/assets/tools/settings.svg").build(ctx, "settings", None),
                    Btn::svg_def("system/assets/tools/search.svg").build(
                        ctx,
                        "search",
                        lctrl(Key::F),
                    ),
                    Btn::plaintext("About").build_def(ctx, None),
                ]),
                Widget::horiz_separator(ctx, 0.3),
                "Zoom in and select something to begin"
                    .draw_text(ctx)
                    .named("tags"),
            ]))
            .aligned(HorizontalAlignment::Right, VerticalAlignment::Top)
            .exact_size_percent(35, 80)
            .build(ctx),
            fixed_object_outline: None,
        })
    }

    fn update_tags(&mut self, ctx: &mut EventCtx, app: &App) {
        let mut col = Vec::new();
        if self.fixed_object_outline.is_some() {
            col.push("Click something else to examine it".draw_text(ctx));
        } else {
            col.push("Click to examine".draw_text(ctx));
        }

        match app.primary.current_selection {
            Some(ID::Lane(l)) => {
                let r = app.primary.map.get_parent(l);
                col.push(
                    Btn::text_bg2(format!("Open OSM way {}", r.orig_id.osm_way_id.0)).build(
                        ctx,
                        format!("open {}", r.orig_id.osm_way_id),
                        None,
                    ),
                );

                let tags = &r.osm_tags;
                for (k, v) in tags.inner() {
                    if k.starts_with("abst:") {
                        continue;
                    }
                    if tags.contains_key(osm::INFERRED_PARKING)
                        && (k == osm::PARKING_RIGHT
                            || k == osm::PARKING_LEFT
                            || k == osm::PARKING_BOTH)
                    {
                        continue;
                    }
                    if tags.contains_key(osm::INFERRED_SIDEWALKS) && k == osm::SIDEWALK {
                        continue;
                    }
                    col.push(Widget::row(vec![
                        Btn::plaintext(k).build(
                            ctx,
                            format!("open https://wiki.openstreetmap.org/wiki/Key:{}", k),
                            None,
                        ),
                        Line(v).draw(ctx).align_right(),
                    ]));
                }
            }
            Some(ID::Intersection(i)) => {
                let i = app.primary.map.get_i(i);
                col.push(
                    Btn::text_bg2(format!("Open OSM node {}", i.orig_id.0)).build(
                        ctx,
                        format!("open {}", i.orig_id),
                        None,
                    ),
                );
            }
            Some(ID::Building(b)) => {
                let b = app.primary.map.get_b(b);
                col.push(
                    Btn::text_bg2(format!("Open OSM ID {}", b.orig_id.inner())).build(
                        ctx,
                        format!("open {}", b.orig_id),
                        None,
                    ),
                );

                let mut txt = Text::new();
                txt.add(Line(format!("Address: {}", b.address)));
                if let Some(ref names) = b.name {
                    txt.add(Line(format!(
                        "Name: {}",
                        names.get(app.opts.language.as_ref()).to_string()
                    )));
                }
                if !b.amenities.is_empty() {
                    txt.add(Line(""));
                    if b.amenities.len() == 1 {
                        txt.add(Line("1 amenity:"));
                    } else {
                        txt.add(Line(format!("{} amenities:", b.amenities.len())));
                    }
                    for (names, amenity) in &b.amenities {
                        txt.add(Line(format!(
                            "  {} ({})",
                            names.get(app.opts.language.as_ref()),
                            amenity
                        )));
                    }
                }
                col.push(txt.draw(ctx));
            }
            Some(ID::ParkingLot(pl)) => {
                let pl = app.primary.map.get_pl(pl);
                col.push(
                    Btn::text_bg2(format!("Open OSM ID {}", pl.osm_id.inner())).build(
                        ctx,
                        format!("open {}", pl.osm_id),
                        None,
                    ),
                );

                col.push(
                    format!(
                        "Estimated parking spots: {}",
                        prettyprint_usize(pl.capacity())
                    )
                    .draw_text(ctx),
                );
            }
            _ => {
                col = vec!["Zoom in and select something to begin".draw_text(ctx)];
            }
        }
        self.top_panel
            .replace(ctx, "tags", Widget::col(col).named("tags"));
    }
}

impl State<App> for Viewer {
    fn event(&mut self, ctx: &mut EventCtx, app: &mut App) -> Transition {
        ctx.canvas_movement();
        if ctx.redo_mouseover() {
            let old_id = app.primary.current_selection.clone();
            app.recalculate_current_selection(ctx);

            if self.fixed_object_outline.is_none() && old_id != app.primary.current_selection {
                self.update_tags(ctx, app);
            }
        }

        if ctx.canvas.get_cursor_in_map_space().is_some() && ctx.normal_left_click() {
            if let Some(id) = app.primary.current_selection.clone() {
                // get_obj must succeed, because we can only click static map elements.
                let outline = app
                    .primary
                    .draw_map
                    .get_obj(ctx, id, app, &mut app.primary.draw_map.agents.borrow_mut())
                    .unwrap()
                    .get_outline(&app.primary.map);
                let mut batch = GeomBatch::from(vec![(app.cs.perma_selected_object, outline)]);

                if let Some(ID::Lane(l)) = app.primary.current_selection {
                    for turn in app.primary.map.get_turns_from_lane(l) {
                        batch.push(
                            TurnExplorer::color_turn_type(turn.turn_type),
                            turn.geom
                                .make_arrow(BIG_ARROW_THICKNESS, ArrowCap::Triangle),
                        );
                    }
                }

                self.fixed_object_outline = Some(ctx.upload(batch));
            } else {
                self.fixed_object_outline = None;
            }
            self.update_tags(ctx, app);
        }

        match self.top_panel.event(ctx) {
            Outcome::Clicked(x) => match x.as_ref() {
                "close" => {
                    return Transition::Pop;
                }
                "change map" => {
                    return Transition::Push(CityPicker::new(
                        ctx,
                        app,
                        Box::new(|ctx, app| {
                            Transition::Multi(vec![
                                Transition::Pop,
                                Transition::Replace(Viewer::new(ctx, app)),
                            ])
                        }),
                    ));
                }
                "settings" => {
                    return Transition::Push(OptionsPanel::new(ctx, app));
                }
                "search" => {
                    return Transition::Push(Navigator::new(ctx, app));
                }
                "About" => {
                    return Transition::Push(PopupMsg::new(
                        ctx,
                        "About this OSM viewer",
                        vec![
                            "If you have an idea about what this viewer should do, get in touch \
                             at abstreet.org!",
                            "",
                            "Note major liberties have been taken with inferring where sidewalks \
                             and crosswalks exist.",
                            "Separate footpaths, bicycle trails, tram lines, etc are not imported \
                             yet.",
                        ],
                    ));
                }
                x => {
                    if let Some(url) = x.strip_prefix("open ") {
                        open_browser(url.to_string());
                    } else {
                        unreachable!()
                    }
                }
            },
            _ => {}
        }

        Transition::Keep
    }

    fn draw(&self, g: &mut GfxCtx, _: &App) {
        self.top_panel.draw(g);
        if let Some(ref d) = self.fixed_object_outline {
            g.redraw(d);
        }
    }
}
