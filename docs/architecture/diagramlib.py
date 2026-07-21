"""
Tiny vector-diagram toolkit built on pycairo.

Every diagram is described once as a `draw(ctx)` function that draws in logical
coordinates. `render()` runs that function against a cairo SVGSurface (true
vector output) and against a scaled ImageSurface (crisp hi-DPI PNG), so the SVG
and PNG are always pixel-for-pixel the same picture.

No external binaries (graphviz / mermaid / chromium) required.
"""

import cairo

FONT = "DejaVu Sans"

# ---- palette -------------------------------------------------------------
INK = "#0f172a"      # near-black text
MUTED = "#64748b"    # secondary text
LINE = "#94a3b8"     # default connectors
WHITE = "#ffffff"
BG = "#ffffff"
PANEL = "#f8fafc"
PANEL_LINE = "#e2e8f0"

# category colours: (stroke, fill)
CLIENT = ("#334155", "#e2e8f0")
INGRESS = ("#2563eb", "#dbeafe")
DECODE = ("#0891b2", "#cffafe")
CACHE = ("#d97706", "#fef3c7")
ROUTE = ("#7c3aed", "#ede9fe")
RESOLVE = ("#059669", "#d1fae5")
EGRESS = ("#e11d48", "#ffe4e6")
SIDE = ("#475569", "#f1f5f9")
STUB = ("#b45309", "#fff7ed")   # not-yet-implemented
OK = ("#16a34a", "#dcfce7")
STORE = ("#0369a1", "#e0f2fe")

GREEN = "#16a34a"
RED = "#dc2626"
AMBER = "#d97706"
PURPLE = "#7c3aed"
BLUE = "#2563eb"


def hexrgb(h):
    h = h.lstrip("#")
    return tuple(int(h[i:i + 2], 16) / 255 for i in (0, 2, 4))


def src(c, color):
    c.set_source_rgb(*hexrgb(color))


# ---- primitives ----------------------------------------------------------

def rrect(c, x, y, w, h, r=12):
    import math
    c.new_sub_path()
    c.arc(x + w - r, y + r, r, -math.pi / 2, 0)
    c.arc(x + w - r, y + h - r, r, 0, math.pi / 2)
    c.arc(x + r, y + h - r, r, math.pi / 2, math.pi)
    c.arc(x + r, y + r, r, math.pi, 1.5 * math.pi)
    c.close_path()


def text(c, x, y, s, size=13, bold=False, italic=False, color=INK, align="left"):
    c.select_font_face(
        FONT,
        cairo.FONT_SLANT_ITALIC if italic else cairo.FONT_SLANT_NORMAL,
        cairo.FONT_WEIGHT_BOLD if bold else cairo.FONT_WEIGHT_NORMAL,
    )
    c.set_font_size(size)
    ext = c.text_extents(s)
    if align == "center":
        x -= ext.width / 2 + ext.x_bearing
    elif align == "right":
        x -= ext.width + ext.x_bearing
    src(c, color)
    c.move_to(x, y)
    c.show_text(s)
    return ext.width


def textw(c, s, size=13, bold=False):
    c.select_font_face(FONT, cairo.FONT_SLANT_NORMAL,
                       cairo.FONT_WEIGHT_BOLD if bold else cairo.FONT_WEIGHT_NORMAL)
    c.set_font_size(size)
    return c.text_extents(s).width


def anchors(x, y, w, h):
    return {
        "x": x, "y": y, "w": w, "h": h,
        "l": (x, y + h / 2), "r": (x + w, y + h / 2),
        "t": (x + w / 2, y), "b": (x + w / 2, y + h),
        "c": (x + w / 2, y + h / 2),
        "tl": (x, y), "tr": (x + w, y), "bl": (x, y + h), "br": (x + w, y + h),
    }


def box(c, x, y, w, h, title, lines=None, cat=SIDE, r=12,
        title_size=15, line_size=11, title_color=None, badge=None):
    stroke, fill = cat
    # soft shadow
    rrect(c, x + 2, y + 3, w, h, r)
    c.set_source_rgba(0.05, 0.09, 0.16, 0.06)
    c.fill()
    rrect(c, x, y, w, h, r)
    src(c, fill)
    c.fill_preserve()
    src(c, stroke)
    c.set_line_width(2.0)
    c.stroke()
    ty = y + (h / 2 - (len(lines) * (line_size + 3)) / 2 + 5 if lines else h / 2 + title_size / 2 - 3)
    if not lines:
        ty = y + h / 2 + title_size / 2 - 3
    text(c, x + w / 2, ty, title, size=title_size, bold=True,
         color=title_color or stroke, align="center")
    if lines:
        yy = ty + title_size - 2
        for ln in lines:
            text(c, x + w / 2, yy + line_size, ln, size=line_size,
                 color=MUTED, align="center")
            yy += line_size + 3
    if badge:
        bw = textw(c, badge, 9.5, True) + 12
        rrect(c, x + w - bw - 8, y - 9, bw, 18, 9)
        src(c, stroke); c.fill()
        text(c, x + w - bw / 2 - 8, y + 3, badge, size=9.5, bold=True,
             color=WHITE, align="center")
    return anchors(x, y, w, h)


def arrowhead(c, x, y, angle, size=9, color=LINE):
    import math
    src(c, color)
    c.move_to(x, y)
    c.line_to(x - size * math.cos(angle - 0.42), y - size * math.sin(angle - 0.42))
    c.line_to(x - size * math.cos(angle + 0.42), y - size * math.sin(angle + 0.42))
    c.close_path()
    c.fill()


def edge(c, p1, p2, color=LINE, width=2.2, dashed=False, label=None,
         label_t=0.5, label_dy=-7, head=True, label_bg=BG, label_size=11):
    import math
    x1, y1 = p1
    x2, y2 = p2
    src(c, color)
    c.set_line_width(width)
    if dashed:
        c.set_dash([6, 5])
    c.move_to(x1, y1)
    c.line_to(x2, y2)
    c.stroke()
    c.set_dash([])
    if head:
        arrowhead(c, x2, y2, math.atan2(y2 - y1, x2 - x1), color=color)
    if label:
        lx = x1 + (x2 - x1) * label_t
        ly = y1 + (y2 - y1) * label_t + label_dy
        _label(c, lx, ly, label, color, label_bg, label_size)


def _label(c, cx, cy, s, color, bg, size=11):
    w = textw(c, s, size, True)
    if bg:
        rrect(c, cx - w / 2 - 6, cy - size / 2 - 5, w + 12, size + 9, 7)
        src(c, bg); c.fill_preserve()
        src(c, "#e2e8f0"); c.set_line_width(1); c.stroke()
    text(c, cx, cy + size / 2 - 1, s, size=size, bold=True, color=color, align="center")


def poly(c, pts, color=LINE, width=2.2, dashed=False, head=True,
         label=None, label_idx=0, label_dy=-7, label_bg=BG):
    import math
    src(c, color)
    c.set_line_width(width)
    if dashed:
        c.set_dash([6, 5])
    c.move_to(*pts[0])
    for p in pts[1:]:
        c.line_to(*p)
    c.stroke()
    c.set_dash([])
    if head:
        (xa, ya), (xb, yb) = pts[-2], pts[-1]
        arrowhead(c, xb, yb, math.atan2(yb - ya, xb - xa), color=color)
    if label:
        (xa, ya), (xb, yb) = pts[label_idx], pts[label_idx + 1]
        _label(c, (xa + xb) / 2, (ya + yb) / 2 + label_dy, label, color, label_bg)


def panel(c, x, y, w, h, title=None, r=14):
    rrect(c, x, y, w, h, r)
    src(c, PANEL); c.fill_preserve()
    src(c, PANEL_LINE); c.set_line_width(1.5); c.stroke()
    if title:
        text(c, x + 16, y + 22, title, size=13, bold=True, color=MUTED)


def cylinder(c, x, y, w, h, title, cat=STORE, sub=None):
    import math
    stroke, fill = cat
    ry = 9
    src(c, fill)
    c.move_to(x, y + ry)
    c.line_to(x, y + h - ry)
    c.save(); c.translate(x + w / 2, y + h - ry); c.scale(w / 2, ry)
    c.arc(0, 0, 1, 0, math.pi); c.restore()
    c.line_to(x + w, y + ry)
    c.save(); c.translate(x + w / 2, y + ry); c.scale(w / 2, ry)
    c.arc(0, 0, 1, math.pi, 2 * math.pi); c.restore()
    c.close_path()
    c.fill()
    src(c, stroke); c.set_line_width(2)
    c.move_to(x, y + ry); c.line_to(x, y + h - ry)
    c.save(); c.translate(x + w / 2, y + h - ry); c.scale(w / 2, ry)
    c.arc(0, 0, 1, 0, math.pi); c.restore(); c.stroke()
    c.move_to(x + w, y + ry); c.line_to(x + w, y + h - ry); c.stroke()
    c.save(); c.translate(x + w / 2, y + ry); c.scale(w / 2, ry)
    c.arc(0, 0, 1, 0, 2 * math.pi); c.restore(); c.stroke()
    text(c, x + w / 2, y + h / 2 + 3, title, size=12.5, bold=True,
         color=stroke, align="center")
    if sub:
        text(c, x + w / 2, y + h / 2 + 18, sub, size=10, color=MUTED, align="center")
    return anchors(x, y, w, h)


def title_block(c, W, title, subtitle):
    text(c, 40, 46, title, size=25, bold=True, color=INK)
    text(c, 40, 70, subtitle, size=13.5, color=MUTED)
    src(c, PANEL_LINE); c.set_line_width(1.5)
    c.move_to(40, 84); c.line_to(W - 40, 84); c.stroke()


def legend(c, x, y, items, cols=1, gap=20):
    """items: list of (label, color) where color is (stroke,fill) or hex line."""
    cx, cy = x, y
    for i, (label, color) in enumerate(items):
        if isinstance(color, tuple):
            stroke, fill = color
            rrect(c, cx, cy - 10, 16, 14, 4)
            src(c, fill); c.fill_preserve()
            src(c, stroke); c.set_line_width(1.6); c.stroke()
        else:
            src(c, color); c.set_line_width(3)
            c.move_to(cx, cy - 3); c.line_to(cx + 16, cy - 3); c.stroke()
        text(c, cx + 24, cy + 1, label, size=11, color=INK)
        cy += gap


def footer(c, W, H, text_str):
    text(c, W - 40, H - 16, text_str, size=10, italic=True, color=MUTED, align="right")


# ---- render both formats from one draw pass ------------------------------

def render(name, W, H, draw, outdir, scale=2.0):
    import os
    svg_path = os.path.join(outdir, name + ".svg")
    s = cairo.SVGSurface(svg_path, W, H)
    c = cairo.Context(s)
    src(c, BG); c.paint()
    draw(c)
    s.finish()

    img = cairo.ImageSurface(cairo.FORMAT_ARGB32, int(W * scale), int(H * scale))
    c2 = cairo.Context(img)
    c2.scale(scale, scale)
    src(c2, BG); c2.paint()
    draw(c2)
    png_path = os.path.join(outdir, name + ".png")
    img.write_to_png(png_path)
    print(f"  {name}.svg + {name}.png  ({W}x{H})")
