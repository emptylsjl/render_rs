
#include <iostream>
#include <string>
#include <utility>
#include <vector>
#include <limits>
#include <memory>
#include <thread>


#define STB_IMAGE_WRITE_IMPLEMENTATION
#include "stb_image_write.h"

#include <chrono>
using namespace std::chrono;


const double inf = std::numeric_limits<double>::infinity();
const double pi = 3.1415926535897932385;
const auto aspect_ratio = 16.0 / 9.0;
const int W = 800;
//const int W = 40;
const int H = static_cast<int>(W / aspect_ratio);
const double sample_count = 200;
const int max_depth = 50;



inline double random_double() {
    // Returns a random real in [0,1).
    return rand() / (RAND_MAX + 1.0);
}

inline double random_double(double min, double max) {
    // Returns a random real in [min,max).
    return min + (max-min)*random_double();
}

inline double clamp(double x, double min, double max) {
    if (x < min) return min;
    if (x > max) return max;
    return x;
}

class vec3 {
public:
    vec3() : e{0,0,0} {}
    vec3(double e0, double e1, double e2) : e{e0, e1, e2} {}

    double x() const { return e[0]; }
    double y() const { return e[1]; }
    double z() const { return e[2]; }

    vec3 operator-() const { return vec3(-e[0], -e[1], -e[2]); }
    double operator[](int i) const { return e[i]; }
    double& operator[](int i) { return e[i]; }

    vec3& operator+=(const vec3 &v) {
        e[0] += v.e[0];
        e[1] += v.e[1];
        e[2] += v.e[2];
        return *this;
    }

    vec3& operator*=(const double t) {
        e[0] *= t;
        e[1] *= t;
        e[2] *= t;
        return *this;
    }

    vec3& operator/=(const double t) {
        return *this *= 1/t;
    }

    double length() const {
        return sqrt(length_squared());
    }

    double length_squared() const {
        return e[0]*e[0] + e[1]*e[1] + e[2]*e[2];
    }

    inline static vec3 random() {
        return vec3(random_double(), random_double(), random_double());
    }

    inline static vec3 random(double min, double max) {
        return vec3(random_double(min,max), random_double(min,max), random_double(min,max));
    }

    bool near_zero() const {
        // Return true if the vector is close to zero in all dimensions.
        const auto s = 1e-8;
        return (fabs(e[0]) < s) && (fabs(e[1]) < s) && (fabs(e[2]) < s);
    }

public:
    double e[3];
};

inline std::ostream& operator<<(std::ostream &out, const vec3 &v) {
    return out << v.e[0] << ' ' << v.e[1] << ' ' << v.e[2];
}

inline vec3 operator+(const vec3 &u, const vec3 &v) {
    return vec3(u.e[0] + v.e[0], u.e[1] + v.e[1], u.e[2] + v.e[2]);
}

inline vec3 operator-(const vec3 &u, const vec3 &v) {
    return vec3(u.e[0] - v.e[0], u.e[1] - v.e[1], u.e[2] - v.e[2]);
}

inline vec3 operator*(const vec3 &u, const vec3 &v) {
    return vec3(u.e[0] * v.e[0], u.e[1] * v.e[1], u.e[2] * v.e[2]);
}

inline vec3 operator*(double t, const vec3 &v) {
    return vec3(t*v.e[0], t*v.e[1], t*v.e[2]);
}

inline vec3 operator*(const vec3 &v, double t) {
    return t * v;
}

inline vec3 operator/(vec3 v, double t) {
    return (1/t) * v;
}

inline double dot(const vec3 &u, const vec3 &v) {
    return u.e[0] * v.e[0]
           + u.e[1] * v.e[1]
           + u.e[2] * v.e[2];
}

inline vec3 cross(const vec3 &u, const vec3 &v) {
    return vec3(u.e[1] * v.e[2] - u.e[2] * v.e[1],
                u.e[2] * v.e[0] - u.e[0] * v.e[2],
                u.e[0] * v.e[1] - u.e[1] * v.e[0]);
}

inline vec3 unit_vector(vec3 v) {
    return v / v.length();
}

inline vec3 normalized(vec3 v) {
    return v / v.length();
}

inline vec3 random_in_unit_disk() {
    while (true) {
        auto p = vec3(random_double(-1,1), random_double(-1,1), 0);
        if (p.length_squared() >= 1) continue;
        return p;
    }
}

inline vec3 random_in_unit_sphere() {
    while (true) {
        auto p = vec3::random(-1,1);
        if (p.length_squared() >= 1) continue;
        return p;
    }
}

inline vec3 random_unit_vector() {
    return normalized(random_in_unit_sphere());
//    vec3 a = vec3::random(-1, 1);
//    return a/a.sum();
}

inline vec3 random_unit_vector(vec3& normal) {
    vec3 in_unit_sphere = random_in_unit_sphere();
    if (dot(in_unit_sphere, normal) > 0.0) // In the same hemisphere as the normal
        return in_unit_sphere;
    else
        return -in_unit_sphere;
}

inline vec3 random_in_hemisphere(const vec3& normal) {
    vec3 in_unit_sphere = random_in_unit_sphere();
    if (dot(in_unit_sphere, normal) > 0.0) // In the same hemisphere as the normal
        return in_unit_sphere;
    else
        return -in_unit_sphere;
}

inline vec3 reflect(const vec3& v, const vec3& n) {
    return v - 2*dot(v,n)*n;
}

inline vec3 refract(const vec3& uv, const vec3& n, double etai_over_etat) {
    auto cos_theta = fmin(dot(-uv, n), 1.0);
    vec3 r_out_perp =  etai_over_etat * (uv + cos_theta*n);
    vec3 r_out_parallel = -sqrt(fabs(1.0 - r_out_perp.length_squared())) * n;
    return r_out_perp + r_out_parallel;
}

using pt3d = vec3;   // 3D point

class ray {
public:
    ray() {}
    ray(const pt3d& origin, const vec3& direction)
            : orig(origin), dir(direction)
    {}

    pt3d origin() const  { return orig; }
    vec3 direction() const { return dir; }

    pt3d at(double t) const {
        return orig + t*dir;
    }

public:
    pt3d orig;
    vec3 dir;
};

double process(double c) {
    return sqrt(c / sample_count);
}

uint8_t d2u8(double c) {
    return uint8_t(clamp(process(c), 0., 1.) * 255);
}

class rgba {
public:
    rgba() : c{0, 0, 0, 0} {}
    rgba(uint8_t r, uint8_t g, uint8_t b, uint8_t a) : c{r, g, b, a} {}
    rgba(vec3 c) : c{d2u8(c[0]), d2u8(c[1]), d2u8(c[2]), 255 } {}
//    rgba(vec3 c) : c{uint8_t(c[0] * 255), uint8_t(c[1] * 255), uint8_t(c[2] * 255), 255 } {}

    uint8_t r() { return c[3]; }
    uint8_t g() { return c[2]; }
    uint8_t b() { return c[1]; }
    uint8_t a() { return c[0]; }
    uint8_t* rgba_slice() { return c; }
    uint32_t rgba_u32() { return *(uint32_t*)c; }

public:
    uint8_t c[4];
};

class material;

struct hit_record {
    pt3d p;
    vec3 normal;
    double t;
    bool front_face;
    std::shared_ptr<material> mat_ptr;

    inline void set_face_normal(const ray& r, const vec3& outward_normal) {
        front_face = dot(r.direction(), outward_normal) < 0;
        normal = front_face ? outward_normal :-outward_normal;
    }
};

class hittable {
public:
    virtual bool hit(const ray& r, double t_min, double t_max, hit_record& rec) const = 0;
};

class sphere : public hittable {
public:
    sphere() {}
    sphere(pt3d cen, double r) : center(cen), radius(r) {};
    sphere(pt3d cen, double r, std::shared_ptr<material> m)
            : center(cen), radius(r), mat_ptr(m) {};

    virtual bool hit(
            const ray& r, double t_min, double t_max, hit_record& rec) const override;

public:
    pt3d center;
    double radius;
    std::shared_ptr<material> mat_ptr;
};

bool sphere::hit(const ray& r, double t_min, double t_max, hit_record& rec) const {
    vec3 oc = r.origin() - center;
    auto a = r.direction().length_squared();
    auto half_b = dot(oc, r.direction());
    auto c = oc.length_squared() - radius*radius;

    auto discriminant = half_b*half_b - a*c;
    if (discriminant < 0) return false;
    auto sqrtd = sqrt(discriminant);

    // Find the nearest root that lies in the acceptable range.
    auto root = (-half_b - sqrtd) / a;
    if (root < t_min || t_max < root) {
        root = (-half_b + sqrtd) / a;
        if (root < t_min || t_max < root)
            return false;
    }

    rec.t = root;
    rec.p = r.at(rec.t);
    rec.normal = (rec.p - center) / radius;
    rec.mat_ptr = mat_ptr;

    return true;
}


using std::shared_ptr;
using std::make_shared;

class hittable_list : public hittable {
public:
    hittable_list() {}
    hittable_list(shared_ptr<hittable> object) { add(object); }

    void clear() { objects.clear(); }
    void add(shared_ptr<hittable> object) { objects.push_back(object); }

    virtual bool hit(
            const ray& r, double t_min, double t_max, hit_record& rec) const override;

public:
    std::vector<shared_ptr<hittable>> objects;
};

bool hittable_list::hit(const ray& r, double t_min, double t_max, hit_record& rec) const {
    hit_record temp_rec;
    bool hit_anything = false;
    auto closest_so_far = t_max;

    for (const auto& object : objects) {
        if (object->hit(r, t_min, closest_so_far, temp_rec)) {
            hit_anything = true;
            closest_so_far = temp_rec.t;
            rec = temp_rec;
        }
    }

    return hit_anything;
}

class material {
public:
    virtual bool scatter(
            const ray& r_in, const hit_record& rec, vec3& attenuation, ray& scattered
    ) const = 0;
};


class lambertian : public material {
public:
    lambertian(const vec3& a) : albedo(a) {}

    bool scatter(
            const ray& r_in, const hit_record& rec, vec3& attenuation, ray& scattered
    ) const override {
        auto scatter_direction = rec.normal + random_unit_vector();

        // Catch degenerate scatter direction
        if (scatter_direction.near_zero())
            scatter_direction = rec.normal;

        scattered = ray(rec.p, scatter_direction);
        attenuation = albedo;
        return true;
    }

public:
    vec3 albedo;
};


class metal : public material {
public:
    metal(const vec3& a, double f) : albedo(a), fuzz(f < 1 ? f : 1) {}
    metal(const vec3& a) : albedo(a) {}

    virtual bool scatter(
            const ray& r_in, const hit_record& rec, vec3& attenuation, ray& scattered
    ) const override {
        vec3 reflected = reflect(unit_vector(r_in.direction()), rec.normal);
        scattered = ray(rec.p, reflected);
//        scattered = ray(rec.p, reflected + fuzz*random_in_unit_sphere());
        attenuation = albedo;
        return (dot(scattered.direction(), rec.normal) > 0);
    }

public:
    vec3 albedo;
    double fuzz;
};


class dielectric : public material {
public:
    dielectric(double index_of_refraction) : ir(index_of_refraction) {}

    virtual bool scatter(
            const ray& r_in, const hit_record& rec, vec3& attenuation, ray& scattered
    ) const override {
        attenuation = vec3(1.0, 1.0, 1.0);
        double refraction_ratio = rec.front_face ? (1.0/ir) : ir;

        vec3 unit_direction = unit_vector(r_in.direction());
        double cos_theta = fmin(dot(-unit_direction, rec.normal), 1.0);
        double sin_theta = sqrt(1.0 - cos_theta*cos_theta);

        bool cannot_refract = refraction_ratio * sin_theta > 1.0;
        vec3 direction;

        if (cannot_refract || reflectance(cos_theta, refraction_ratio) > random_double())
            direction = reflect(unit_direction, rec.normal);
        else
            direction = refract(unit_direction, rec.normal, refraction_ratio);

        scattered = ray(rec.p, direction);
        return true;
    }

public:
    double ir; // Index of Refraction

private:
    static double reflectance(double cosine, double ref_idx) {
        // Use Schlick's approximation for reflectance.
        auto r0 = (1-ref_idx) / (1+ref_idx);
        r0 = r0*r0;
        return r0 + (1-r0)*pow((1 - cosine),5);
    }
};

class camera {
public:
    camera() {
        auto aspect_ratio = 16.0 / 9.0;
        auto viewport_height = 2.0;
        auto viewport_width = aspect_ratio * viewport_height;
        auto focal_length = 1.0;

        origin = pt3d(0, 0, 0);
        horizontal = vec3(viewport_width, 0.0, 0.0);
        vertical = vec3(0.0, viewport_height, 0.0);
        lower_left_corner = origin - horizontal/2 - vertical/2 - vec3(0, 0, focal_length);
    }

    ray get_ray(double u, double v) const {
        return ray(origin, lower_left_corner + u*horizontal + v*vertical - origin);
    }

private:
    pt3d origin;
    pt3d lower_left_corner;
    vec3 horizontal;
    vec3 vertical;
};

void write_to_img(uint8_t *img, int x, int y, rgba color) {
    int pos = 4 * (x + y*W);
    *(uint32_t*)(img+pos) = color.rgba_u32();
}

vec3 ray_color(const ray& r, const hittable& world, int depth) {

    if (depth <= 0)
        return vec3(0,0,0);

    hit_record rec;
    if (world.hit(r, 0.001, inf, rec)) {
        ray scattered;
        vec3 attenuation;
        if (rec.mat_ptr->scatter(r, rec, attenuation, scattered))
            return attenuation * ray_color(scattered, world, depth-1);
        return vec3(0,0,0);
    }

    vec3 unit_direction = unit_vector(r.direction());
    auto t = 0.5*(unit_direction.y() + 1.0);
    return (1.0-t)*vec3(1.0, 1.0, 1.0) + t*vec3(0.5, 0.7, 1.0);
}

bool sum(int *l, int len) {
    int a = 0;
    for (int i = 0; i < len; ++i)
        a += l[i];
    printf("\r%.02f %%", double(a)*100/W);
    std::flush(std::cout);
    return a < W;
}

int main() {

    // Image

    const std::string filename("D:/cur/render_rs/raytrace_cpp/book_impl/img/raytrace_sample.png");
    auto *image = (uint8_t*)calloc(H * W * 4, sizeof(uint8_t));

    // World
    hittable_list world;

    auto material_ground = make_shared<lambertian>(vec3(0.8, 0.8, 0.0));
    auto material_center = make_shared<lambertian>(vec3(0.7, 0.3, 0.3));
    auto material_left   = make_shared<metal>(vec3(0.8, 0.8, 0.8));
    auto material_right  = make_shared<metal>(vec3(0.2, 0.6, 0.2));

    world.add(make_shared<sphere>(vec3( 0.0, -100.5, -1.0), 100.0, material_ground));
    world.add(make_shared<sphere>(vec3( 0.0,    0.0, -1.0),   0.5, material_center));
    world.add(make_shared<sphere>(vec3(-1.0,    0.0, -1.0),   0.5, material_left));
    world.add(make_shared<sphere>(vec3( 1.0,    0.0, -1.0),   0.5, material_right));
    //
    // Camera
    camera cam;

    const int ct = 8;
    int rg = W / ct;
    int stage[ct] = {};
    std::thread th[ct] = {};

    auto f = [&](int pos, int st, int ed) {
        for (int i = st, ii = ed - 1; i < ed; ++i, --ii) {
            for (int j = 0, jj = H - 1; j < H; ++j, --jj) {
                vec3 c;
                for (int k = 0; k < sample_count; ++k) {
                    auto u = (i + random_double()) / (W-1);
                    auto v = (j + random_double()) / (H-1);
                    c += ray_color(cam.get_ray(u, v), world, max_depth);
                }
                write_to_img(image, i, jj, rgba(c));
            }
            stage[pos] = i-st+1;
        }
    };

    auto st = high_resolution_clock::now();

    for (int i = 0; i < ct; ++i)
        th[i] = std::thread(f, i, i*rg, i*rg+rg);
    while (sum(stage, ct))
        std::this_thread::sleep_for(milliseconds(100));
    for (auto & t : th)
        t.join();


//    for (int i = 0, ii = W - 1; i < W; ++i, --ii) {
//        for (int j = 0, jj = H - 1; j < H; ++j, --jj) {
//            vec3 c;
//            for (int k = 0; k < sample_count; ++k) {
//                auto u = (i + random_double()) / (W-1);
//                auto v = (j + random_double()) / (H-1);
//                c += ray_color(cam.get_ray(u, v), world, max_depth);
//            }
//            write_to_img(image, i, jj, rgba(c));
//        }
//        printf("%d ", i);
//    }



    duration<float> duration = (high_resolution_clock::now() - st);
    std::cout << "\nelapse: " << duration.count() << std::endl;

    stbi_write_png(filename.c_str(), W, H, 4, image, W * 4);
    std::cerr << "\nDone.\n";
}
