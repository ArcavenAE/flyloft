//! The distance axis (design: docs/design/distance-axis.md, ticket aae-orc-p4t1.1).
//!
//! Red first: these tests use the API the design calls for, so they fail to
//! compile until `Distance`, `NewBatten`, `BattenError` and the getters exist.

use chrono::{TimeZone, Utc};
use flyloft_core::batten::{BattenError, NewBatten};
use flyloft_core::catalog::{CachePolicy, CatalogPointer};
use flyloft_core::provenance::ExtractionMethod;
use flyloft_core::{
    Batten, BattenContent, BattenId, CatalogId, Confidence, Contributor, Cue, CueId, Distance,
    LineSetId, Position, Provenance,
};

fn who() -> Contributor {
    Contributor::System { id: "test".into() }
}

fn at() -> chrono::DateTime<Utc> {
    Utc.with_ymd_and_hms(2026, 10, 25, 12, 0, 0).unwrap()
}

fn held(text: &str) -> BattenContent {
    BattenContent::Held { text: text.into() }
}

fn cataloged() -> BattenContent {
    BattenContent::Cataloged(CatalogPointer {
        catalog: CatalogId("gh".into()),
        external_id: "issue-1".into(),
        url: None,
        snippet: None,
        cache_policy: CachePolicy::default(),
        last_fetched: None,
        adapter_metadata: serde_json::Value::Null,
    })
}

fn spec(distance: Distance, projects: Vec<BattenId>, content: BattenContent) -> NewBatten {
    let line_set = LineSetId("ls_test".into());
    let position = Position {
        ordinal: 0,
        path: vec![],
        char_range: None,
    };
    NewBatten {
        id: BattenId::from_held(&line_set, &position, "text"),
        line_set,
        content,
        parent: None,
        position,
        provenance: Provenance {
            rigged_at: at(),
            rigged_by: who(),
            extraction: ExtractionMethod::Manual,
            upstream_ref: None,
        },
        confidence: Confidence::Frontier,
        spiked: false,
        dogged: false,
        annotations: vec![],
        entities: vec![],
        promoted_to: None,
        rigged_at: at(),
        last_modified: at(),
        distance,
        projects,
    }
}

fn prop_id() -> BattenId {
    BattenId("bat_prop".into())
}

fn prop() -> Batten {
    Batten::new(spec(Distance::Prop, vec![], held("a prop"))).unwrap()
}

fn backdrop() -> Batten {
    Batten::new(spec(
        Distance::Backdrop,
        vec![prop_id()],
        held("a backdrop"),
    ))
    .unwrap()
}

#[test]
fn distance_defaults_to_prop() {
    assert_eq!(Distance::default(), Distance::Prop);
}

#[test]
fn distance_serializes_snake_case() {
    assert_eq!(
        serde_norway::to_string(&Distance::Backdrop).unwrap().trim(),
        "backdrop"
    );
    assert_eq!(
        serde_norway::to_string(&Distance::Scenery).unwrap().trim(),
        "scenery"
    );
    assert_eq!(
        serde_norway::to_string(&Distance::Prop).unwrap().trim(),
        "prop"
    );
}

#[test]
fn a_prop_with_projects_is_refused() {
    let err = Batten::new(spec(Distance::Prop, vec![prop_id()], held("x"))).unwrap_err();
    assert!(matches!(err, BattenError::PropProjects), "{err:?}");
}

#[test]
fn a_backdrop_with_no_projects_is_refused() {
    let err = Batten::new(spec(Distance::Backdrop, vec![], held("x"))).unwrap_err();
    assert!(
        matches!(err, BattenError::NoProjects(Distance::Backdrop)),
        "{err:?}"
    );
}

#[test]
fn scenery_with_no_projects_is_refused() {
    let err = Batten::new(spec(Distance::Scenery, vec![], held("x"))).unwrap_err();
    assert!(
        matches!(err, BattenError::NoProjects(Distance::Scenery)),
        "{err:?}"
    );
}

#[test]
fn a_backdrop_with_cataloged_content_is_refused() {
    let err = Batten::new(spec(Distance::Backdrop, vec![prop_id()], cataloged())).unwrap_err();
    assert!(
        matches!(err, BattenError::NotHeld(Distance::Backdrop)),
        "{err:?}"
    );
}

#[test]
fn scenery_with_cataloged_content_is_refused() {
    let err = Batten::new(spec(Distance::Scenery, vec![prop_id()], cataloged())).unwrap_err();
    assert!(
        matches!(err, BattenError::NotHeld(Distance::Scenery)),
        "{err:?}"
    );
}

#[test]
fn a_prop_may_be_cataloged() {
    let b = Batten::new(spec(Distance::Prop, vec![], cataloged())).unwrap();
    assert!(b.content().is_cataloged());
}

#[test]
fn getters_report_what_was_built() {
    let b = backdrop();
    assert_eq!(b.distance(), Distance::Backdrop);
    assert_eq!(b.projects(), &[prop_id()]);
    assert!(b.content().is_held());
    assert_eq!(prop().distance(), Distance::Prop);
    assert!(prop().projects().is_empty());
}

#[test]
fn a_backdrop_round_trips_with_its_projects() {
    let b = backdrop();
    let yaml = serde_norway::to_string(&b).unwrap();
    assert!(yaml.contains("distance: backdrop"), "{yaml}");
    let back: Batten = serde_norway::from_str(&yaml).unwrap();
    assert_eq!(back.distance(), Distance::Backdrop);
    assert_eq!(back.projects(), &[prop_id()]);
    assert_eq!(serde_norway::to_string(&back).unwrap(), yaml);
}

fn without_keys(b: &Batten, keys: &[&str]) -> serde_norway::Value {
    let mut v = serde_norway::to_value(b).unwrap();
    let map = v.as_mapping_mut().unwrap();
    for k in keys {
        map.remove(*k);
    }
    v
}

#[test]
fn a_batten_written_before_the_field_reads_as_a_prop() {
    let old = without_keys(&prop(), &["distance", "projects"]);
    let b: Batten = serde_norway::from_value(old).unwrap();
    assert_eq!(b.distance(), Distance::Prop);
    assert!(b.projects().is_empty());
}

#[test]
fn loading_a_backdrop_with_no_projects_is_an_error() {
    let v = without_keys(&backdrop(), &["projects"]);
    let err = serde_norway::from_value::<Batten>(v).unwrap_err();
    assert!(err.to_string().contains("projects"), "{err}");
}

#[test]
fn loading_a_prop_with_projects_is_an_error() {
    let mut v = serde_norway::to_value(prop()).unwrap();
    v.as_mapping_mut().unwrap().insert(
        "projects".into(),
        serde_norway::to_value(vec![prop_id()]).unwrap(),
    );
    let err = serde_norway::from_value::<Batten>(v).unwrap_err();
    assert!(err.to_string().contains("projects"), "{err}");
}

#[test]
fn loading_a_backdrop_with_cataloged_content_is_an_error() {
    let mut v = serde_norway::to_value(backdrop()).unwrap();
    v.as_mapping_mut().unwrap().insert(
        "content".into(),
        serde_norway::to_value(cataloged()).unwrap(),
    );
    let err = serde_norway::from_value::<Batten>(v).unwrap_err();
    assert!(err.to_string().contains("held"), "{err}");
}

fn cue(distance: Option<Distance>) -> Cue {
    Cue {
        id: CueId("cue_test".into()),
        query: "q".into(),
        rewritten: None,
        rewrite_strategy: None,
        requester: who(),
        flown: vec![],
        cited: vec![],
        at: at(),
        latency_ms: None,
        distance,
    }
}

#[test]
fn a_cue_records_the_distance_asked_for() {
    let yaml = serde_norway::to_string(&cue(Some(Distance::Backdrop))).unwrap();
    let back: Cue = serde_norway::from_str(&yaml).unwrap();
    assert_eq!(back.distance, Some(Distance::Backdrop));
}

#[test]
fn a_cue_written_before_the_field_reads_as_none() {
    let mut v = serde_norway::to_value(cue(None)).unwrap();
    v.as_mapping_mut().unwrap().remove("distance");
    let back: Cue = serde_norway::from_value(v).unwrap();
    assert_eq!(back.distance, None);
}
