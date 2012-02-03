use std;
import std::io::println;
import float::{fmin,fmax,sqrt,acos,asin,abs,atan2,cos,sin};

import cp::num::num;


/* A mathematical vector with 2 dimensions.
 */
type vec2 = { x : num, y : num };

/* Constructor for vectors.
 */
fn new(xnew : num, ynew : num) -> vec2 {
  ret { x : xnew, y : ynew };
}

/* Constructr for a vector with the given angle (in radians). */
fn forangle(a : num) -> vec2 { new(cos(a), sin(a)) }

/* Mathematical vector interface.
 */

iface Vec2 {
  fn add(v: vec2) -> vec2;
  fn sub(v: vec2) -> vec2;
}

/* Delta to use to check if floating points are near another. */
const NEAR_DELTA  : float = 0.001;

/* Zero constructor for vec2. */
fn zero() -> vec2 { new(0.0, 0.0) }

/* Implementation of vector interface for vec2,
 * to get some OOP-style goodness
 */
impl of Vec2 for vec2 {
  // vector math 
  fn add(v : vec2)      -> vec2 { new(self.x + v.x, self.y + v.y)    }
  fn sub(v : vec2)      -> vec2 { new(self.x - v.x, self.y - v.y)    }
  fn equal(v : vec2)    -> bool { (self.x == v.x) && (self.y == v.y) }
  fn neg()              -> vec2 { new(-self.x, -self.y)              }
  fn mult(n : num)      -> vec2 { new(self.x * n, self.y * n)        }
  fn div(n : num)       -> vec2 { new(self.x / n, self.y / n)        }
  fn dot(v: vec2)       -> num  { self.x*v.x + self.y*v.y            }
  fn cross(v: vec2)     -> num  { self.x*v.y - self.y*v.x            }
  fn perp()             -> vec2 { new(-self.y,  self.x)              }
  fn rperp()            -> vec2 { new( self.y, -self.x)              }
  fn project(v : vec2)  -> vec2 { 
    v.mult((self.dot(v))/(v.dot(v)))       
  }
  fn rotate(v : vec2) -> vec2  { 
    new(self.x*v.x - self.y*v.y, self.x*v.y + self.y*v.x)
  }
  fn unrotate(v : vec2) -> vec2 { 
    new(self.x*v.x + self.y*v.y, self.x*v.y - self.y*v.x)
  }

  fn lengthsq()            -> num  { self.dot(self)                         }
  fn length()              -> num  { sqrt(self.dot(self))                   }
  fn lerp(v: vec2, t : num)-> vec2 { self.mult(1.0-t).add(v.mult(t))        }
  fn iszero()              -> bool { (self.x == 0.0) && (self.y == 0.0)     }
  fn normalize()           -> vec2 { self.div(self.length())                }
  fn normalize_safe()      -> vec2 {
    if self.iszero() { zero() } else { self.normalize()                     }
  }
  
  fn clamp(len : num)     -> vec2  { 
    if self.lengthsq() > (len*len) { self.normalize().mult(len) } else { self }
  }

  // Linearly interpolate between self towards v by distance d.
  fn lerpconst(v : vec2, d : num) -> vec2      {
    self.add( v.sub(self).clamp(d) )
  }
  
  // Returns the distance between self and v.
  fn dist(v : vec2) -> num         {  self.sub(v).length()                    }
  

  // Returns the squared distance between self and v.
  // Faster than dist() when you only need to compare distances.
  fn distsq(v : vec2) -> num       {  self.sub(v).lengthsq()                  }
  
  // Returns true if the distance between self and v is less than d.
  fn near(v : vec2, d : num) -> bool { self.distsq(v) < (d*d)                 }

  fn slerp(v : vec2, t : num) -> vec2 {
    let omega = acos(self.dot(v));
    if omega != 0.0 {
      let denom = 1.0 / sin(omega);      
      let sfactor = sin((1.0 - t) * omega) * denom;
      let vfactor = sin(t * omega) * denom;
      self.mult(sfactor).add(v.mult(vfactor))
    } else {
      self
    }
  }

  fn slerpconst(v : vec2, a : num) -> vec2 {
    let angle = acos(self.dot(v));
    self.slerp(v, fmin(a,angle) / angle)
  }

  // Converts the vector to the angle of it's direction
  fn toangle() -> num              { atan2(self.x, self.y)                    }
  
  // Converts vector to string represetation. 
  fn to_str()  -> str              { #fmt("( % .3f, % .3f)", self.x, self.y)  }  

  // Check if the vector is near a point
  fn xnear(x : num)       -> bool  { abs(self.x - x) < NEAR_DELTA             }
  fn ynear(y : num)       -> bool  { abs(self.y - y) < NEAR_DELTA             }
  fn xynear(x: num, y : num)  -> bool  {
    self.xnear(x) && self.ynear(y)
  }
}



#[test]
mod tests {
  
  #[test]
  fn test_vec2() {
    let self = new(1.0, 2.0);
    let v = new(4.0, 7.0);
    let v3 = self.add(v);
    let v4 = v.sub(self);
    let v5 = new(0.0, 3.0);
    assert(self.x == 1.0);
    assert(self.y == 2.0);
    assert(v3.x == 5.0);
    assert(v3.y == 9.0);
    assert(v3.xnear(5.0));
    assert(v3.ynear(9.0));
    assert(v4.x == 3.0);
    assert(v4.y == 5.0);
    assert(v5.length() == 3.0);
    assert(v.to_str() == "(  4,  7)");
  }

}





