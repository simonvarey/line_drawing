use {FloatNum, Point, SignedNum};
use octant::Octant;
use steps::Steps;

/// An implementation of the [mid-point line drawing algorithm].
///
/// The biggest difference between this algorithm and [`Bresenham`] is that it uses floating-point points.
///
/// Example:
///
/// ```
/// extern crate line_drawing;
/// use line_drawing::Midpoint;
///
/// fn main() {
///     for (x, y) in Midpoint::<f32, i8>::new((0.2, 0.02), (2.8, 7.7)) {
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
/// [`Bresenham`]: struct.bresenham.html
pub struct MidpointEllipse</*I,*/ O> {
    x: O,
    y: O,
    a: O,
    b: O,
    //center_x: O,
    //center_y: O,
    d1: O,
    d2: O,
    quadrant: u8,
    //octant: Octant,
    //point: Point<O>,
}

impl</*I: FloatNum, */O: SignedNum> MidpointEllipse<O> {
    #[inline]
    pub fn new<I: FloatNum>(radius_x: I, radius_y: I) -> Self {

        let a = O::cast(radius_x.round());
        let b = O::cast(radius_y.round());

        let x = a; 
        let y = O::cast(0);

        let d1 = (O::cast(2) * a * a) - (O::cast(2) * a * b * b) + (b * b/O::cast(2));
        let d2 = (a * a/O::cast(2)) - (O::cast(4) * a * b * b) + (O::cast(2) * b * b);

        Self {
            x,
            y,
            a,
            b,
            //center_x,
            //center_y,
            d1,
            d2,
            quadrant: 1,
        }

        // Get the octant to use
        //let octant = Octant::new(start, end);

        // Convert the points into the octant versions
        //let start = octant.to(start);
        //let end = octant.to(end);

        // Initialise the variables

        /*let a = -(end.1 - start.1);
        let b = end.0 - start.0;
        let c = start.0 * end.1 - end.0 * start.1;

        Self {
            octant,
            a,
            b,
            point: (O::cast(start.0.round()), O::cast(start.1.round())),
            k: a * (start.0.round() + I::one()) + b * (start.1.round() + I::cast(0.5)) + c,
            end_x: O::cast(end.0.round()),
        }*/
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
                1 => (self.x /*+ self.center_x*/, self.y /*+ self.center_y*/),
                2 => (-self.x /*+ self.center_x*/, self.y /*+ self.center_y*/),
                3 => (self.x /*+ self.center_x*/, -self.y /*+ self.center_y*/),
                4 => (-self.x /*+ self.center_x*/, -self.y /*+ self.center_y*/),
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
                1 => (self.x /*+ self.center_x*/, self.y /*+ self.center_y*/),
                2 => (-self.x /*+ self.center_x*/, self.y /*+ self.center_y*/),
                3 => (self.x /*+ self.center_x*/, -self.y /*+ self.center_y*/),
                4 => (-self.x /*+ self.center_x*/, -self.y /*+ self.center_y*/),
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
    let ellipse = |a, b| MidpointEllipse::new(a, b).collect::<Vec<_>>();

    assert_eq!(
        ellipse(10.0, 20.0),
        [(1, 0), (0, 1), (-1, 0), (0, -1),]
    );

    /*assert_eq!(
        ellipse((0.0, 0.0), (6.0, 3.0)),
        [(0, 0), (1, 1), (2, 1), (3, 2), (4, 2), (5, 3), (6, 3)]
    );*/
}
