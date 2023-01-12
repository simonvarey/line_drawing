use {FloatNum, Point, SignedNum};
use steps::Steps;

/// An implementation of the [mid-point ellipse drawing algorithm].
///
/// Example:
///
/// ```
/// extern crate line_drawing;
/// use line_drawing::MidpointEllipse;
///
/// fn main() {
///     for (x, y) in MidpointEllipse::<f32, i8>::new((0.2, 0.02), (2.8, 7.7)) {
///         print!("({}, {}), ", x, y);
///     }
/// }
/// ```
///
/// ```text
/// (0, 0), (1, 1), (1, 2), (1, 3), (2, 4), (2, 5), (2, 6), (3, 7), (3, 8),
/// ```
///
/// [mid-point line drawing algorithm]: http://www.mat.univie.ac.at/~kriegl/Skripten/CG/node25.html
pub struct MidpointEllipse<O> {
    x: O,
    y: O,
    a: O,
    b: O,
    center_x: O,
    center_y: O,
    d1: O,
    d2: O,
    quadrant: u8,
}

impl<O: SignedNum> MidpointEllipse<O> {
    #[inline]
    pub fn new<I: FloatNum>(center_x: I, center_y: I, radius_x: I, radius_y: I) -> Self {

        let a = O::cast(radius_x.round());
        let b = O::cast(radius_y.round());

        Self {
            x: a,
            y: O::cast(0),
            a,
            b,
            center_x: O::cast(center_x.round()),
            center_y: O::cast(center_y.round()),
            d1: (O::cast(2) * a * a) - (O::cast(2) * a * b * b) + (b * b/O::cast(2)),
            d2: (a * a/O::cast(2)) - (O::cast(4) * a * b * b) + (O::cast(2) * b * b),
            quadrant: 1,
        }
    }

    #[inline]
    pub fn steps(self) -> Steps<Point<O>, Self> {
        Steps::new(self)
    }
}

impl<O: SignedNum> Iterator for MidpointEllipse<O> {
    type Item = Point<O>;
   
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // Region 1
        if self.d2 < O::cast(0) {
            let point = match self.quadrant {
                1 => (self.x + self.center_x, self.y + self.center_y),
                2 => (-self.x + self.center_x, self.y + self.center_y),
                3 => (self.x + self.center_x, -self.y + self.center_y),
                4 => (-self.x + self.center_x, -self.y + self.center_y),
                _ => unreachable!(),
            };
  
            // Update the variables after each set of quadrants
            if self.quadrant == 4 {
          
                //Checking and updating value of decision parameter based on algorithm
                if self.d1 < O::cast(0) {
                    self.y += O::cast(1);
                    self.d1 += (O::cast(4) * self.a * self.a * self.y) + (O::cast(2) * self.a * self.a);
                    self.d2 += O::cast(4) * self.a * self.a * self.y;
                } else {
                    self.x -= O::cast(1);
                    self.y += O::cast(1);
                    self.d1 -= (O::cast(4) * self.b * self.b * self.x) + (O::cast(4) * self.a * self.a * self.y) + (O::cast(2) * self.a * self.a);
                    self.d2 -= (O::cast(4) * self.b * self.b * self.x) + (O::cast(4) * self.a * self.a * self.y) + (O::cast(2) * self.b * self.b);
                }
            }
  
            self.quadrant = self.quadrant % 4 + 1;
  
            Some(point)
        } else if self.x >= O::cast(0) {
  
            let point = match self.quadrant {
                1 => (self.x + self.center_x, self.y + self.center_y),
                2 => (-self.x + self.center_x, self.y + self.center_y),
                3 => (self.x + self.center_x, -self.y + self.center_y),
                4 => (-self.x + self.center_x, -self.y + self.center_y),
                _ => unreachable!(),
            };
    
            // Update the variables after each set of quadrants
            if self.quadrant == 4 {
            
                //Checking and updating value of decision parameter based on algorithm
                if self.d2 < O::cast(0) {
                    self.x -= O::cast(1);
                    self.y += O::cast(1);
                    self.d2 -= (O::cast(4) * self.b * self.b * self.x) + (O::cast(4) * self.a * self.a * self.y) + (O::cast(2) * self.b * self.b);
                } else {
                    self.x -= O::cast(1);
                    self.d2 -= (O::cast(4) * self.b * self.b * self.x) + (O::cast(2) * self.b * self.b);
                }
            }   
    
            self.quadrant = self.quadrant % 4 + 1;
    
            Some(point)
        } else {
            None
        }

    /*#[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.point.0 <= self.end_x {
            let point = self.octant.from(self.point);

            // Take an N step
            if self.k <= I::zero() {
                self.k += self.b;
                self.point.1 += O::one();
            }

            // Take an E step
            self.k += self.a;
            self.point.0 += O::one();

            Some(point)
        } else {
            None
        }*/
    }
}

#[test]
fn tests() {
    let ellipse = |a, b, c, d| 
        MidpointEllipse::<i32>::new(a, b, c, d).collect::<Vec<_>>();

    assert_eq!(
        ellipse(50.0, 50.0, 10.0, 15.0),
        [(50, 65), (50, 65), (50, 35), (50, 35), (51, 65), (49, 65), (51, 35), (49, 35), (52, 65), (48, 65), (52, 35), (48, 35), (53, 64), (47, 64), (53, 36), (47, 36), (54, 64), (46, 64), (54, 36), (46, 36), (55, 63), (45, 63), (55, 37), (45, 37), (56, 62), (44, 62), (56, 38), (44, 38), (57, 61), (43, 61), (57, 39), (43, 39), (57, 60), (43, 60), (57, 40), (43, 40), (58, 59), (42, 59), (58, 41), 
        (42, 41), (58, 58), (42, 58), (58, 42), (42, 42), (59, 57), (41, 57), (59, 43), (41, 43), (59, 56), (41, 56), (59, 44), (41, 44), (59, 55), (41, 55), (59, 45), (41, 45), (60, 54), (40, 54), (60, 46), (40, 46), (60, 53), (40, 53), (60, 47), (40, 47), (60, 52), (40, 52), (60, 48), (40, 48), (60, 51), (40, 51), (60, 49), (40, 49), (60, 50), (40, 50), (60, 50), (40, 50),]
    );

    /*assert_eq!(
        ellipse((0.0, 0.0), (6.0, 3.0)),
        [(0, 0), (1, 1), (2, 1), (3, 2), (4, 2), (5, 3), (6, 3)]
    );*/
}
