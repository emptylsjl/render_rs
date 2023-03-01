
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

using v2d = Vector2d;
//using v3d = Vector3d;
using v4d = Vector4d;
using rgbV3d = Vector3d;
using rgbV4d = Vector4d;

using v3d = vec3;


const double infinity = std::numeric_limits<double>::infinity();
const double pi = 3.1415926535897932385;
const auto aspect_ratio = 16.0 / 9.0;
int count0 = 0;
int count1 = 0;
int count2 = 0;

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

uint8_t d2u8(double c) {
    return uint8_t(std::clamp(c, 0., 1.) * 255);
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

struct spt {
    v3d p;
    v3d n;
    double t;
};

struct hitSpt {
    spt pts[2];
    int min;
    int count;
};

struct hit_record {
    v3d p;
};

class sphere {
    public:
        sphere() {}
        sphere(v3d cpt, double r) : cpt(std::move(cpt)), r(r) {};
    public:
        v3d cpt;
        double r;
};

class world {
    public:
        world() {}
        world(sphere* s, int len) { objs.insert(objs.begin(), s, s+len); }

        void add(sphere s) { objs.push_back(s); }
        void clean() { objs.clear(); }
        uint32_t len() { return objs.size(); }

        hitSpt onHit(const ray& r) {

            double mode = 0;

            for (const auto& s : objs) {
                v3d ps = r.pt - s.cpt;
                double a = dot(r.dir, r.dir);
                double b = dot(r.dir, ps) *(mode+1) ;
                double c = dot(ps, ps) - s.r*s.r;
                double d = b*b-a*c*(1+mode*3);
                if (d > 0) {
    //                double t0 = (-b-sqrt(d))/(mode+1)*a;
    //                double t1 = (-b+sqrt(d))/(mode+1)*a;
                    double t0 = (-b-sqrt(d))/a;
                    double t1 = (-b+sqrt(d))/a;
                    if (t0 > 0 && t1 > 0) {
                        spt a0 = {r.at(t0), (r.at(t0)-s.cpt)/s.r, t0};
                        spt a1 = {r.at(t1), (r.at(t1)-s.cpt)/s.r, t1};
                        return { {a0, a1}, a0.t>a1.t, (a1.t!=a0.t)+1 };
                    }
                }
            }
            return { {}, {}, 0};
        }

    public:
        std::vector<sphere> objs;
};

class camera {
    public:
        camera() {
            auto viewport_height = 2.0;
            auto viewport_width = aspect_ratio * viewport_height;
            auto focal_length = 1.0;

            origin = point3(0, 0, 0);
            horizontal = vec3(viewport_width, 0.0, 0.0);
            vertical = vec3(0.0, viewport_height, 0.0);
            lower_left_corner = origin - horizontal/2 - vertical/2 - vec3(0, 0, focal_length);
        }

        ray get_ray(double u, double v) const {
            return ray(origin, lower_left_corner + u*horizontal + v*vertical - origin);
        }

    private:
        point3 origin;
        point3 lower_left_corner;
        vec3 horizontal;
        vec3 vertical;

};

//const int W = 800;
const int W = 40;
const int H = W/aspect_ratio;

void write_to_img(uint8_t *img, int x, int y, rgba color) {
    int pos = 4 * (x + y*W);
    *(uint32_t*)(img+pos) = color.rgba_u32();
}

v3d ray_color(world& w, ray& r) {
    auto [pts, min, count] = w.onHit(r);
    if (count > 0) {
//        return ((r.at(pts[min].t) - v3d(0, 0, -1)).normalized() + v3d(1, 1, 1)) * 0.5;
        return (pts[min].n + v3d(1, 1, 1)) * 0.5;
    }
    auto t = 0.5*(normalized(r.dir).y() + 1.0);
    return (1.0-t)*v3d(1.0, 1.0, 1.0) + t*v3d(0.5, 0.7, 1.0);
}


int main() {
    const std::string filename("D:\\cur\\render_rs\\raytrace_cpp\\img\\raytrace.png");
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

    for (int i = 0, ii = W-1; i < W; ++i, --ii) {
        for (int j = 0, jj = H-1; j < H; ++j, --jj) {
            auto u = double(i) / W;
            auto v = double(j) / H;
            ray r(origin, lower_left_corner + u*horizontal + v*vertical);
            v3d c = ray_color(w, r);
            write_to_img(image, i, jj, rgba(c));
        }
    }
    stbi_write_png(filename.c_str(), W, H, 4, image, W*4);
    printf("\n\n233\n");
}
