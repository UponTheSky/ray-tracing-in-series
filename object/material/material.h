#ifndef MATERIAL_H
#define MATERIAL_H

#include "utils/utils.h"
#include "common/ray.h"
#include "object/material/texture.h"

class material {
  public:
    virtual bool scatter(const ray& r_in, const hit_record& rec, color& attenuation, ray& scattered) const = 0;

    virtual color emitted(double u, double v, const point3& p) const {
      return color(0, 0, 0);
    }
};

class lambertian : public material {
  public:
    lambertian(const color& a): albedo(make_shared<solid_color>(a)) {}
    lambertian(shared_ptr<texture> a) : albedo(a) {}


    virtual bool scatter(
      const ray& r_in,
      const hit_record& rec,
      color& attenuation,
      ray& scattered
    ) const override {
      auto scattered_direction = rec.normal + random_unit_vector();
      if (scattered_direction.near_zero()) {
        scattered_direction = rec.normal;
      }

      scattered = ray(rec.p, scattered_direction, r_in.time());
      attenuation = albedo->value(rec.u, rec.v, rec.p);
      return true;
    }

  public:
    shared_ptr<texture> albedo;
};

class metal : public material {
  public:
    metal(const color& a, double f) : albedo(a), fuzz(f < 1 ? f : 1) {}

    virtual bool scatter(
     const ray& r_in, const hit_record& rec, color& attenuation, ray& scattered
    ) const override {
      vec3 reflected = reflect(unit_vector(r_in.direction()), rec.normal);
      scattered = ray(rec.p, reflected + fuzz * random_in_unit_sphere(), r_in.time());
      attenuation = albedo;
      return dot(scattered.direction(), rec.normal) > 0;
    }

  private:
    color albedo;
    double fuzz;
};

class dielectric : public material {
  public:
    dielectric(double index_of_refraction): ir(index_of_refraction) {}

    virtual bool scatter(
      const ray& r_in, const hit_record& rec, color& attenuation, ray& scattered
    ) const override {
      attenuation = color(1.0, 1.0, 1.0);
      double refraction_ratio = rec.front_face ? (1.0 / ir) : ir;

      vec3 unit_direction = unit_vector(r_in.direction());
      double cos_theta = fmin(dot(-unit_direction, rec.normal), 1.0);
      double sin_theta = sqrt(1.0 - cos_theta * cos_theta);

      vec3 direction = refraction_ratio * sin_theta > 1.0 || reflectance(cos_theta, refraction_ratio) > random_double()
        ? reflect(unit_direction, rec.normal)
        : refract(unit_direction, rec.normal, refraction_ratio);

      scattered = ray(rec.p, direction, r_in.time());
      return true;
    }

  private:
    double ir;

    static double reflectance(double cosine, double ref_idx) {
      double r0 = (1 - ref_idx) / (1 + ref_idx);
      return r0*r0 + (1 - r0*r0) * pow(1 - cosine, 5);
    }
};

class diffuse_light : public material {
  public:
    diffuse_light(shared_ptr<texture> a) : emit(a) {}
    diffuse_light(color c) : emit(make_shared<solid_color>(c)) {}

    virtual bool scatter(
      const ray& r_in, const hit_record& rec, color& attenuation, ray& scattered
    ) const override {
      return false;
    }

    virtual color emitted(double u, double v, const point3& p) const override {
      return emit->value(u, v, p);
    }

  private:
    shared_ptr<texture> emit;
};

class isotropic : public material {
  public:
    isotropic(color c) : albedo(make_shared<solid_color>(c)) {}
    isotropic(shared_ptr<texture> a) : albedo(a) {}

    virtual bool scatter(
      const ray& r_in, const hit_record& rec, color& attenuation, ray& scattered
    ) const override {
      scattered = ray(rec.p, random_in_unit_sphere(), r_in.time());
      attenuation = albedo->value(rec.u, rec.v, rec.p);
      return true;
    }

  private:
    shared_ptr<texture> albedo;
};

#endif
