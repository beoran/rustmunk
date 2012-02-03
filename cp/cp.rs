use std;
import std::io::println;
import float::{fmin,fmax,sqrt,acos,asin,abs,atan2,cos,sin};

import cp::num::num;
import cp::num::INFINITY;
import cp::vec2;
import cp::vec2::vec2;

fn moment_for_circle(m : num, r1 : num, r2 : num, offset : vec2 ) {
   (m * (0.5f*(r1*r1+r2*r2))) + offset.lengthsq()
}


/*
cpFloat
cpMomentForCircle(cpFloat m, cpFloat r1, cpFloat r2, cpVect offset)
{
  return m*(0.5f*(r1*r1 + r2*r2) + cpvlengthsq(offset));
}

cpFloat
cpAreaForCircle(cpFloat r1, cpFloat r2)
{
  return (cpFloat)M_PI*cpfabs(r1*r1 - r2*r2);
}

cpFloat
cpMomentForSegment(cpFloat m, cpVect a, cpVect b)
{
  cpFloat length = cpvlength(cpvsub(b, a));
  cpVect offset = cpvmult(cpvadd(a, b), 1.0f/2.0f);

  return m*(length*length/12.0f + cpvlengthsq(offset));
}

cpFloat
cpAreaForSegment(cpVect a, cpVect b, cpFloat r)
{
  return r*((cpFloat)M_PI*r + 2.0f*cpvdist(a, b));
}

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