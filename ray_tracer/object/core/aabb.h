#ifndef AABB_H
#define AABB_H

#include "../../utils/utils.h"
#include "../../common/vec3.h"

class aabb {
  public:
    aabb() {}
    aabb(const point3& a, const point3& b) : minimum{a}, maximum{b} {}

    point3 min() const { return minimum; }
    point3 max() const { return maximum; }

    inline bool hit(const ray& r, double t_min, double t_max) const {
      for (int a = 0; a < 3; a++) {
        auto invD = 1.0 / r.direction()[a];
        auto t0 = (min()[a] - r.origin()[a]) * invD;
        auto t1 = (max()[a] - r.origin()[a]) * invD;
        if (invD < 0.0f) {
          std::swap(t0, t1);
        }

        t_min = fmax(t0, t_min);
        t_max = fmin(t1, t_max);
        if (t_max <= t_min) {
          return false;
        }
      }
      return true;
    }

  private:
    point3 minimum, maximum;
};

aabb surrounding_box(const aabb& box0, const aabb& box1) {
  point3 small(
    fmin(box0.min().x(), box1.min().x()),
    fmin(box0.min().y(), box1.min().y()),
    fmin(box0.min().z(), box1.min().z())
  );
  point3 big(
    fmax(box0.max().x(), box1.max().x()),
    fmax(box0.max().y(), box1.max().y()),
    fmax(box0.max().z(), box1.max().z())
  );

  return aabb(small, big);
}

#endif
