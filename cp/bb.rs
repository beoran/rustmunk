use std;
import std::io::println;
import float::{fmin,fmax,sqrt,acos,asin,abs,atan2,cos,sin};

import cp::num::num;
import cp::num::INFINITY;
import cp::vec2;
import cp::vec2::vec2;


// A twodimensional axis aligned bounds box.
type bb = { l : num, b : num, r : num, t : num };

// Constructor for the bounds box. 
fn new(lnew : num, bnew : num, rnew : num, tnew : num) -> bb {
  ret { l : lnew, b : bnew, r : rnew, t : tnew };
}

// Constructs a bb for a circle with the given position and radius.
fn newforcircle(p : vec2, r : num) -> bb {
  new(p.x - r, p.y - r, p.x + r, p.y + r)
}


iface BB {
  
}

impl BB for bb {
  // Returns true if b intersects with self
  fn intersects(b : bb) -> bool {
    self.l <= b.r && b.l <= self.r && self.b <= b.t && b.b <= self.t
  }
  
  // Returns true if b lies completely within self
  fn contains_bb(b : bb) -> bool {
    self.l <= b.l && self.r >= b.r && self.b <= b.b && self.t >= b.t
  }

  // Returns true if self contains vec2 v.
  fn contains_vec2(v : vec2) -> bool {
    self.l <= v.x && self.r >= v.x && self.b <= v.y && self.t >= v.y
  }

  // Returns a bounding box that holds both bounding boxes
  fn merge(b : bb) -> bb {
    new(fmin(self.l, b.l), fmin(self.b, b.b),
        fmax(self.r, b.r), fmax(self.t, b.t))
  }

  // Returns a bounding box that holds self and vec2 v
  fn expand(v : vec2) -> bb {
    new(fmin(self.l, v.x), fmin(self.b, v.y),
        fmax(self.r, v.x), fmax(self.t, v.y))
  }

  // Return the width of the bounds box
  fn width() -> num { self.r - self.l }
  
  // Return the height of the bounds box
  fn height() -> num { self.t - self.b }
  
  // Returns the area of the bounds box
  fn area() -> num  { self.width() * self.height() }

  // Returns the area of self merged with the box b
  fn mergedarea(b : bb) -> num  {
    (fmax(self.r, b.r) - fmin(self.l, b.l)) *
    (fmax(self.t, b.t) - fmin(self.b, b.b))
  }

// Returns the fraction along the segment query the cpBB is hit.
// Returns INFINITY if it doesn't hit.
  fn segmentquery (a : vec2, b : vec2) -> num {
    let idx   = 1.0f/(b.x - a.x);
    let tx1   = if self.l == a.x {-INFINITY } else { (self.l - a.x)*idx };
    let tx2   = if self.r == a.x { INFINITY } else { (self.r - a.x)*idx };
    let txmin = fmin(tx1, tx2);
    let txmax = fmax(tx1, tx2);
    let idy   = 1.0f/(b.y - a.y);
    let ty1   = if self.b == a.y { -INFINITY } else { (self.b - a.y)*idy };
    let ty2   = if self.t == a.y {  INFINITY } else { (self.t - a.y)*idy };
    let tymin = fmin(ty1, ty2);
    let tymax = fmax(ty1, ty2);
    if tymin <= txmax && txmin <= tymax {
      let min = fmax(txmin, tymin);
      let max = fmin(txmax, tymax);
      if 0.0 <= max && min <= 1.0  { ret fmax(min, 0.0); }
    }
    ret INFINITY;     
  }

  // Return true if the bounding box intersects the line segment with ends
  // a and b
  fn intersects_segment(a : vec2, b : vec2) -> bool {
    self.segmentquery(a, b) != INFINITY
  }

  // Clamps v to this bounds box
  fn clamp_vec2(v : vec2) -> vec2 {
    let x = fmin(fmax(self.l, v.x), self.r);
    let y = fmin(fmax(self.b, v.y), self.t);
    ret vec2::new(x, y);
  }

  // wraps this vector to the bounds box
  fn wrap_vec2(v : vec2) -> vec2 {
    let ix    = abs(self.r - self.l);
    let modx  = (v.x - self.l) % ix;
    let x     =  if modx > 0.0f { modx } else { modx + ix };
    let iy    = abs(self.t - self.b);
    let mody  = (v.y - self.b) % iy;
    let y     = if mody > 0.0f { mody } else { mody + iy };
    ret vec2::new(x + self.l, y + self.b);
  }    
}

#[test]
mod tests {
  
  #[test]
  fn test_segmentquery() {
    let b1 = new(0.0, 0.0, 10.0, 5.0);
    let v1 = vec2::new(3.0, 3.0);
    let v2 = vec2::new(4.0, 4.0);
    let v3 = vec2::new(13.0, 13.0);
    let v4 = vec2::new(14.0, 14.0);    
    assert(b1.intersects_segment(v1, v2));
    assert(!b1.intersects_segment(v3, v4));
  }
  
  #[test]
  fn test_modulus() {
    let f1 = 12.0;
    let f2 = 5.0;
    let m1 = f1 % f2;
    assert(m1 == 2.0);    
  }
}



