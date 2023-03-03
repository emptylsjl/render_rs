
#include <iostream>
#include <string>
#include <utility>
#include <vector>
#include <limits>

#define STB_IMAGE_WRITE_IMPLEMENTATION
#include "stb_image_write.h"
#include "weekend.h"

#include <Eigen/Dense>
using namespace Eigen;

#include <chrono>
using namespace std::chrono;

using v2d = Vector2d;
//using v3d = Vector3d;
using v4d = Vector4d;
using rgbV3d = Vector3d;
using rgbV4d = Vector4d;

using v3d = vec3;


const double infinity = std::numeric_limits<double>::infinity();
const double pi = 3.1415926535897932385;
const auto aspect_ratio = 16.0 / 9.0;
const int W = 800;
//const int W = 40;
const int H = W/aspect_ratio;
const double sample_count = 100;
const int max_depth = 50;
int count0 = 0;
int count1 = 0;
int count2 = 0;

double process(double c) {
//    return c;
    return sqrt(c / sample_count);
}

uint8_t d2u8(double c) {
    return uint8_t(std::clamp(process(c), 0., 1.) * 255);
}

class rgba {
public:
    rgba() : c{0, 0, 0, 0} {}
    rgba(uint8_t r, uint8_t g, uint8_t b, uint8_t a) : c{r, g, b, a} {}
    rgba(v3d c) : c{d2u8(c[0]), d2u8(c[1]), d2u8(c[2]), 255 } {}
//    rgba(v3d c) : c{uint8_t(c[0] * 255), uint8_t(c[1] * 255), uint8_t(c[2] * 255), 255 } {}
    rgba(v4d c) : c{uint8_t(c[0] * 255), uint8_t(c[1] * 255), uint8_t(c[2] * 255), uint8_t(c[3] * 255) } {}

    uint8_t r() { return c[3]; }
    uint8_t g() { return c[2]; }
    uint8_t b() { return c[1]; }
    uint8_t a() { return c[0]; }
    uint8_t* rgba_slice() { return c; }
    uint32_t rgba_u32() { return *(uint32_t*)c; }

public:
    uint8_t c[4];
};

class ray {
public:
    ray() = default;
    ray(v3d origin, v3d dir) : pt(std::move(origin)), dir(std::move(dir)) {
        //        printf("b- %f %f %f\n", dir.x(), dir.y(), dir.z() );
    }

    v3d at(double t) const {return pt + t * dir;}

public:
    v3d pt;
    v3d dir;
};

struct hpt {
    v3d p;
    v3d n;
    double t;
    int hit;
};

class sphere {
    public:
        sphere() {}
        sphere(v3d cpt, double r) : cpt(cpt), r(r), type(1) {};
        sphere(v3d cpt, double r, int type) : cpt(cpt), r(r), type(type) {};
    public:
        v3d cpt;
        double r;
        int type;
};

hpt hpt_from(ray r, sphere s, double t) {
    return {r.at(t), (r.at(t)-s.cpt)/s.r, t, 1};
}

//class hray {
//public:
//    hray() {}
//    hray(ray r) : r(r), hit(0) {}
//    hray(ray r, hpt dst)  : r(r), dst(dst), hit(1) {}
//
//public:
//    ray r;
//    hpt dst;
//    int hit;
//};

class camera {
public:
    camera() {
        auto viewport_height = 2.0;
        auto viewport_width = aspect_ratio * viewport_height;
        auto focal_length = 1.0;

        origin = v3d(0, 0, 0);
        horizontal = v3d(viewport_width, 0.0, 0.0);
        vertical = v3d(0.0, viewport_height, 0.0);
        lower_left_corner = origin - horizontal/2 - vertical/2 - v3d(0, 0, focal_length);
    }

    ray get_ray(double u, double v) const {
        return ray(origin, lower_left_corner + u*horizontal + v*vertical - origin);
    }

private:
    v3d origin;
    v3d lower_left_corner;
    v3d horizontal;
    v3d vertical;

};

class world {
    public:
        world() {}
        world(sphere* s, int len) { objs.insert(objs.begin(), s, s+len); }

        void add(sphere s) { objs.push_back(s); }
        void clean() { objs.clear(); }
        uint32_t len() { return objs.size(); }

    hpt onHit(const ray& r) {

            for (const auto& s : objs) {
                v3d ps = r.pt - s.cpt;
                double a = dot(r.dir, r.dir);
                double b = dot(r.dir, ps);
                double c = dot(ps, ps) - s.r*s.r;
                double d = b*b-a*c;
                if (d > 0) {
                    double t = std::min((-b-sqrt(d))/a, (-b+sqrt(d))/a);
                    if (t > 0.001) {
                        return hpt_from(r, s, t);
                    }
                }
            }
            return {};
        }

    public:
        std::vector<sphere> objs;
};


void write_to_img(uint8_t *img, int x, int y, rgba color) {
    int pos = 4 * (x + y*W);
    *(uint32_t*)(img+pos) = color.rgba_u32();
}

v3d ray_color(world& w, const ray& r, int depth) {
    if (depth <= 0)
        return v3d(0,0,0);

    auto [p, n, t, hit] = w.onHit(r);
    if (hit) {
        v3d target = p + n + random_unit_vector();
        return 0.5 * ray_color(w, ray(p, target - p), depth-1);
    }
    auto tp = 0.5*(normalized(r.dir).y() + 1.0);
    return (1.0-tp)*v3d(1.0, 1.0, 1.0) + tp*v3d(0.5, 0.7, 1.0);
}


int main() {
    const std::string filename("D:\\cur\\render_rs\\raytrace_cpp\\book_impl\\img\\raytrace.png");
    auto *image = (uint8_t*)calloc(H * W * 4, sizeof(uint8_t));
    setbuf(stdout, 0);

    auto viewport_height = 2.0;
    auto viewport_width = aspect_ratio * viewport_height;
    auto focal_length = 1.0;
    auto origin = v3d(0, 0, 0);
    auto horizontal = v3d(viewport_width, 0, 0);
    auto vertical = v3d(0, viewport_height, 0);
    auto lower_left_corner = origin - horizontal/2 - vertical/2 - v3d(0, 0, focal_length);

    sphere aa[2] = {
        sphere(v3d(0, 0, -1), 0.5),
        sphere(v3d(0,-100.5,-1), 100),
    };
    world w;
    w.add(aa[0]);
    w.add(aa[1]);

    camera cam;

    auto st = high_resolution_clock::now();

    for (int i = 0, ii = W-1; i < W; ++i, --ii) {
        for (int j = 0, jj = H-1; j < H; ++j, --jj) {
            v3d c;
            for (int k = 0; k < sample_count; ++k) {
                auto u = (i + random_double()) / (W - 1);
                auto v = (j + random_double()) / (H - 1);
                c += ray_color(w, cam.get_ray(u, v), max_depth);
            }
            write_to_img(image, i, jj, rgba(c));
        }
        std::cout << i << " " << std::flush;
    }

    duration<float> duration = (high_resolution_clock::now() - st);
    std::cout << "\nelapse: " << duration.count() << std::endl;

    stbi_write_png(filename.c_str(), W, H, 4, image, W*4);
    printf("\n\n233\n");
}
