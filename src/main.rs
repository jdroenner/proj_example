use geo::{coords_iter::CoordsIter, Point, Rect};
use proj::{Proj, ProjError};

/// A spatial reference consists of an authority and a code
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SpatialReference {
    authority: u32,
    code: u32,
}

impl std::fmt::Display for SpatialReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.authority, self.code)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Default)]
pub struct Coordinate2D {
    pub x: f64,
    pub y: f64,
}

impl From<Coordinate2D> for geo::Coordinate<f64> {
    fn from(coordinate: Coordinate2D) -> geo::Coordinate<f64> {
        geo::Coordinate::from((coordinate.x, coordinate.y))
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
/// The bounding box of a geometry.
/// Note: may degenerate to a point!
pub struct BoundingBox2D {
    lower_left_coordinate: Coordinate2D,
    upper_right_coordinate: Coordinate2D,
}

impl From<BoundingBox2D> for geo::Rect<f64> {
    fn from(bbox: BoundingBox2D) -> geo::Rect<f64> {
        geo::Rect::new(bbox.lower_left_coordinate, bbox.upper_right_coordinate)
    }
}

pub trait Reproject<Out = Self> {
    fn reproject(
        &self,
        source_spatial_ref: SpatialReference,
        target_spatial_ref: SpatialReference,
    ) -> Result<Out, ()>;
}

impl Reproject for BoundingBox2D {
    fn reproject(
        &self,
        source_spatial_ref: SpatialReference,
        target_spatial_ref: SpatialReference,
    ) -> Result<Self, ()> {
        dbg!(source_spatial_ref, target_spatial_ref);
        let proj = Proj::new_known_crs(
            &source_spatial_ref.to_string(),
            &target_spatial_ref.to_string(),
            None,
        )
        .expect("handle ProjError");

        let rect: Rect<f64> = (*self).into();
        let res: std::result::Result<Vec<Point<f64>>, ProjError> = rect
            .coords_iter()
            .map(|c| Ok(Point::from(proj.convert(c)?)))
            .collect();

        // then build new rect from the points here...
        Ok(self.clone())
    }
}

fn main() {
    println!("Hello, world!");
}
