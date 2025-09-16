use macroquad::prelude::*;

/// Draw a filled rounded rectangle.
/// X, y = top-left corner. W, h = size. Radius = corner radius (clamped to half-min dimension).
pub fn draw_rounded_rect(x: f32, y: f32, w: f32, h: f32, radius: f32, color: Color) {
    // clamp radius so corners don't overlap
    let r = radius.min(w.min(h) * 0.5);

    // center rect
    if w > 2.0 * r && h > 2.0 * r {
        draw_rectangle(x + r, y + r, w - 2.0 * r, h - 2.0 * r, color);
    } else {
        // if radius consumes whole shape, fallback to a circle-like rounded rect
        draw_rectangle(x, y, w, h, color);
        return;
    }

    // side rectangles (fill the cross-areas between corner circles)
    draw_rectangle(x, y + r, r, h - 2.0 * r, color); // left
    draw_rectangle(x + w - r, y + r, r, h - 2.0 * r, color); // right
    draw_rectangle(x + r, y, w - 2.0 * r, r, color); // top
    draw_rectangle(x + r, y + h - r, w - 2.0 * r, r, color); // bottom

    // four corner circles
    draw_circle(x + r, y + r, r, color); // top-left
    draw_circle(x + w - r, y + r, r, color); // top-right
    draw_circle(x + r, y + h - r, r, color); // bottom-left
    draw_circle(x + w - r, y + h - r, r, color); // bottom-right
}

/// Draw a filled rounded rectangle with individually controllable corner radii.
///
/// r_tl = top-left radius
/// r_tr = top-right radius
/// r_bl = bottom-left radius
/// r_br = bottom-right radius
pub fn draw_rounded_rect_ex(
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    mut r_tl: f32,
    mut r_tr: f32,
    mut r_bl: f32,
    mut r_br: f32,
    color: Color,
) {
    // sanitize negative sizes
    if w <= 0.0 || h <= 0.0 {
        return;
    }

    // clamp radii to non-negative
    r_tl = r_tl.max(0.0);
    r_tr = r_tr.max(0.0);
    r_bl = r_bl.max(0.0);
    r_br = r_br.max(0.0);

    // Prevent radii overlapping horizontally / vertically.
    // Compute scale factors for each relevant pair and apply the minimum
    // to each corner so that sums fit the rectangle.
    let scale_w_top = if r_tl + r_tr > 0.0 {
        (w / (r_tl + r_tr)).min(1.0)
    } else {
        1.0
    };
    let scale_w_bottom = if r_bl + r_br > 0.0 {
        (w / (r_bl + r_br)).min(1.0)
    } else {
        1.0
    };
    let scale_h_left = if r_tl + r_bl > 0.0 {
        (h / (r_tl + r_bl)).min(1.0)
    } else {
        1.0
    };
    let scale_h_right = if r_tr + r_br > 0.0 {
        (h / (r_tr + r_br)).min(1.0)
    } else {
        1.0
    };

    // apply combined scale per corner
    r_tl *= scale_w_top.min(scale_h_left);
    r_tr *= scale_w_top.min(scale_h_right);
    r_bl *= scale_w_bottom.min(scale_h_left);
    r_br *= scale_w_bottom.min(scale_h_right);

    // If radii consume the whole shape (degenerate), just draw a rectangle
    // (this also guards against negative widths below).
    let left = r_tl.max(r_bl);
    let right = r_tr.max(r_br);
    let top = r_tl.max(r_tr);
    let bottom = r_bl.max(r_br);

    let center_w = w - left - right;
    let center_h = h - top - bottom;

    if center_w <= 0.0 || center_h <= 0.0 {
        // Radii fill the rect; fallback to a simple rectangle (or a circle-like shape).
        draw_rectangle(x, y, w, h, color);
        return;
    }

    // center rectangle
    draw_rectangle(x + left, y + top, center_w, center_h, color);

    // top and bottom rectangles (span between left/right "max" offsets)
    draw_rectangle(x + left, y, center_w, top, color);
    draw_rectangle(x + left, y + h - bottom, center_w, bottom, color);

    // left and right rectangles (span between top/bottom "max" offsets)
    draw_rectangle(x, y + top, left, center_h, color);
    draw_rectangle(x + w - right, y + top, right, center_h, color);

    // corner circles (only draw if radius > 0)
    if r_tl > 0.0 {
        draw_circle(x + r_tl, y + r_tl, r_tl, color);
    }
    if r_tr > 0.0 {
        draw_circle(x + w - r_tr, y + r_tr, r_tr, color);
    }
    if r_bl > 0.0 {
        draw_circle(x + r_bl, y + h - r_bl, r_bl, color);
    }
    if r_br > 0.0 {
        draw_circle(x + w - r_br, y + h - r_br, r_br, color);
    }
}

/// Draw an outlined (stroke) rounded rectangle by painting an outer rounded rect (stroke color)
/// then painting a smaller inner rounded rect with the `background` color to create the stroke.
/// Thickness will be clamped so inner rect still exists.
pub fn draw_rounded_rect_stroke(
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    radius: f32,
    thickness: f32,
    stroke_color: Color,
    background: Color,
) {
    // clamp radius and thickness
    let r = radius.min(w.min(h) * 0.5);
    let t = thickness.max(0.0).min(w.min(h) * 0.5); // thickness not more than half min dim

    // Draw outer (stroke) rounded rect
    draw_rounded_rect(x, y, w, h, r, stroke_color);

    // Compute inner rect (where we paint background to simulate the stroke)
    let inner_x = x + t;
    let inner_y = y + t;
    let inner_w = w - 2.0 * t;
    let inner_h = h - 2.0 * t;

    if inner_w > 0.0 && inner_h > 0.0 {
        let inner_r = (r - t).max(0.0).min(inner_w.min(inner_h) * 0.5);
        draw_rounded_rect(inner_x, inner_y, inner_w, inner_h, inner_r, background);
    }
}
