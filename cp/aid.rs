use std;
import std::io::println;
import float::{fmin,fmax,sqrt,acos,asin,abs,atan2,cos,sin};

import cp::num::num;
import cp::num::INFINITY;
import cp::num::PI;
import cp::vec2;
import cp::vec2::vec2;
import cp::vec2::Vec2;

fn moment_for_circle(m : num, r1 : num, r2 : num, offset : vec2 ) -> num {
   (m * (0.5f*(r1*r1+r2*r2))) + offset.lengthsq()
}

fn area_for_circle(r1 : num, r2 : num) -> num  {  
  (PI*abs(r1*r1 - r2*r2))
}


fn moment_for_segment(m : num, a : vec2,  b : vec2 ) -> num {
  let length = b.sub(a).length();
  let offset = a.add(b).div(2.0);    
  m * (((length*length) / 12.0) + offset.lengthsq())
}

fn area_for_segment(a : vec2, b : vec2, r : num) -> num  {
  r*(PI*r + 2.0*a.dist(b))
}




/*
cpFloat
cpMomentForPoly(cpFloat m, const int numVerts, const cpVect *verts, cpVect offset)
{
  cpFloat sum1 = 0.0f;
  cpFloat sum2 = 0.0f;
  for(int i=0; i<numVerts; i++){
    cpVect v1 = cpvadd(verts[i], offset);
    cpVect v2 = cpvadd(verts[(i+1)%numVerts], offset);

    cpFloat a = cpvcross(v2, v1);
    cpFloat b = cpvdot(v1, v1) + cpvdot(v1, v2) + cpvdot(v2, v2);

    sum1 += a*b;
    sum2 += a;
  }

  return (m*sum1)/(6.0f*sum2);
}

cpFloat
cpAreaForPoly(const int numVerts, const cpVect *verts)
{
  cpFloat area = 0.0f;
  for(int i=0; i<numVerts; i++){
    area += cpvcross(verts[i], verts[(i+1)%numVerts]);
  }

  return -area/2.0f;
}

cpVect
cpCentroidForPoly(const int numVerts, const cpVect *verts)
{
  cpFloat sum = 0.0f;
  cpVect vsum = cpvzero;

  for(int i=0; i<numVerts; i++){
    cpVect v1 = verts[i];
    cpVect v2 = verts[(i+1)%numVerts];
    cpFloat cross = cpvcross(v1, v2);

    sum += cross;
    vsum = cpvadd(vsum, cpvmult(cpvadd(v1, v2), cross));
  }

  return cpvmult(vsum, 1.0f/(3.0f*sum));
}

void
cpRecenterPoly(const int numVerts, cpVect *verts){
  cpVect centroid = cpCentroidForPoly(numVerts, verts);

  for(int i=0; i<numVerts; i++){
    verts[i] = cpvsub(verts[i], centroid);
  }
}

cpFloat
cpMomentForBox(cpFloat m, cpFloat width, cpFloat height)
{
  return m*(width*width + height*height)/12.0f;
}

cpFloat
cpMomentForBox2(cpFloat m, cpBB box)
{
  cpFloat width = box.r - box.l;
  cpFloat height = box.t - box.b;
  cpVect offset = cpvmult(cpv(box.l + box.r, box.b + box.t), 0.5f);

  return cpMomentForBox(m, width, height) + m*cpvlengthsq(offset);
}
*/