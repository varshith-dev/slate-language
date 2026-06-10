use crate::ast::{AstNode, AstNodeKind, Config, Value};
use std::collections::HashMap;

pub struct Compiler {
    #[allow(dead_code)]
    live_reload: bool,
}

impl Compiler {
    pub fn new(live_reload: bool) -> Self {
        Self { live_reload }
    }

    pub fn compile(&self, nodes: &[AstNode]) -> String {
        let mut y = 40.0;
        let x = 50.0;
        let width = 900.0;
        let mut svg_content = String::new();

        for node in nodes {
            let (node_svg, node_h) = self.render_node_to_svg(node, x, y, width);
            svg_content.push_str(&node_svg);
            y += node_h;
        }

        let total_height = y + 40.0;
        self.wrap_in_svg_template(&svg_content, total_height)
    }

    fn render_node_to_svg(&self, node: &AstNode, x: f64, y: f64, width: f64) -> (String, f64) {
        match &node.kind {
            AstNodeKind::Element {
                tag,
                id,
                config,
                children,
                raw_text,
                is_required,
                type_assertion,
                ..
            } => {
                let id_str = id.as_deref().unwrap_or("");
                match tag.as_str() {
                    "section" | "page" | "form" => {
                        let mut content = String::new();
                        let mut curr_y = 0.0;
                        for child in children {
                            let (child_svg, child_h) = self.render_node_to_svg(child, x, y + curr_y, width);
                            content.push_str(&child_svg);
                            curr_y += child_h;
                        }
                        (content, curr_y)
                    }
                    "grid" => {
                        let cols = config.get_number_property("cols").unwrap_or(2.0) as usize;
                        let gap = config.get_number_property("gap").unwrap_or(24.0);
                        
                        if cols == 2 && children.len() >= 2 {
                            let col_width = (width - gap) / 2.0;
                            let col1_x = x;
                            let col2_x = x + col_width + gap;
                            
                            let mut col1_svg = String::new();
                            let mut col2_svg = String::new();
                            let mut col1_h = 0.0;
                            let mut col2_h = 0.0;
                            
                            for (idx, child) in children.iter().enumerate() {
                                if idx % 2 == 0 {
                                    let (child_svg, child_h) = self.render_node_to_svg(child, col1_x, y + col1_h, col_width);
                                    col1_svg.push_str(&child_svg);
                                    col1_h += child_h;
                                } else {
                                    let (child_svg, child_h) = self.render_node_to_svg(child, col2_x, y + col2_h, col_width);
                                    col2_svg.push_str(&child_svg);
                                    col2_h += child_h;
                                }
                            }
                            let max_h = col1_h.max(col2_h);
                            let midline_x = x + col_width + (gap / 2.0);
                            let midline_svg = format!(
                                r##"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="#CBD5E1" stroke-width="1" stroke-dasharray="4 4" />"##,
                                midline_x, y, midline_x, y + max_h
                            );
                            let combined_svg = format!("{}{}{}", col1_svg, col2_svg, midline_svg);
                            (combined_svg, max_h + 20.0)
                        } else {
                            // Fallback to vertical stack
                            let mut content = String::new();
                            let mut curr_y = 0.0;
                            for child in children {
                                let (child_svg, child_h) = self.render_node_to_svg(child, x, y + curr_y, width);
                                content.push_str(&child_svg);
                                curr_y += child_h;
                            }
                            (content, curr_y)
                        }
                    }
                    "row" => {
                        let count = children.len();
                        if count == 0 {
                            return ("".to_string(), 0.0);
                        }
                        let gap = 16.0;
                        let child_width = (width - (count - 1) as f64 * gap) / count as f64;
                        let mut content = String::new();
                        let mut max_h = 0.0;
                        for (idx, child) in children.iter().enumerate() {
                            let child_x = x + idx as f64 * (child_width + gap);
                            let (child_svg, child_h) = self.render_node_to_svg(child, child_x, y, child_width);
                            content.push_str(&child_svg);
                            if child_h > max_h {
                                max_h = child_h;
                            }
                        }
                        (content, max_h)
                    }
                    "card" => {
                        let mut content = String::new();
                        let mut curr_y = 24.0;
                        for child in children {
                            let (child_svg, child_h) = self.render_node_to_svg(child, x + 24.0, y + curr_y, width - 48.0);
                            content.push_str(&child_svg);
                            curr_y += child_h;
                        }
                        let card_height = curr_y + 12.0;
                        let svg = format!(
                            r##"<g>
                                 <rect x="{}" y="{}" width="{}" height="{}" fill="#F8FAFC" stroke="#E2E8F0" stroke-width="1" rx="6" />
                                 <rect x="{}" y="{}" width="4" height="{}" fill="#6366F1" rx="2" />
                                 {}
                               </g>
                            "##,
                            x, y, width, card_height,
                            x, y, card_height,
                            content
                        );
                        (svg, card_height + 24.0)
                    }
                    "title" => {
                        let text = raw_text.as_deref().unwrap_or("");
                        let display_text = if text.is_empty() {
                            config.get_first_positional_string().unwrap_or_default()
                        } else {
                            text.to_string()
                        };
                        let svg = format!(
                            r##"<foreignObject x="{}" y="{}" width="{}" height="55">
                                 <div xmlns="http://www.w3.org/1999/xhtml" style="font-family: 'Outfit', sans-serif; font-size: 32px; font-weight: 700; color: #0F172A; padding-bottom: 8px; border-bottom: 2px solid #E2E8F0; margin: 0; white-space: nowrap; overflow: hidden; text-overflow: ellipsis;">
                                   {}
                                 </div>
                               </foreignObject>"##,
                            x, y, width, display_text
                        );
                        (svg, 65.0)
                    }
                    "subtitle" => {
                        let text = raw_text.as_deref().unwrap_or("");
                        let display_text = if text.is_empty() {
                            config.get_first_positional_string().unwrap_or_default()
                        } else {
                            text.to_string()
                        };
                        let svg = format!(
                            r##"<foreignObject x="{}" y="{}" width="{}" height="35">
                                 <div xmlns="http://www.w3.org/1999/xhtml" style="font-family: 'Inter', sans-serif; font-size: 16px; color: #64748B; margin: 0; line-height: 1.4; overflow: hidden; text-overflow: ellipsis;">
                                   {}
                                 </div>
                               </foreignObject>"##,
                            x, y, width, display_text
                        );
                        (svg, 35.0)
                    }
                    "heading" => {
                        let text = raw_text.as_deref().unwrap_or("");
                        let display_text = if text.is_empty() {
                            config.get_first_positional_string().unwrap_or_default()
                        } else {
                            text.to_string()
                        };
                        let svg = format!(
                            r##"<foreignObject x="{}" y="{}" width="{}" height="45">
                                 <div xmlns="http://www.w3.org/1999/xhtml" style="font-family: 'Outfit', sans-serif; font-size: 20px; font-weight: 600; color: #0F172A; padding-bottom: 4px; border-bottom: 1px solid #E2E8F0; margin: 0; white-space: nowrap; overflow: hidden; text-overflow: ellipsis;">
                                   {}
                                 </div>
                               </foreignObject>"##,
                            x, y, width, display_text
                        );
                        (svg, 50.0)
                    }
                    "text" => {
                        let text = raw_text.as_deref().unwrap_or("");
                        let display_text = if text.is_empty() {
                            config.get_first_positional_string().unwrap_or_default()
                        } else {
                            text.to_string()
                        };

                        let (html_content, calculated_h) = if display_text.starts_with("- ") || display_text.starts_with("* ") {
                            let clean_text = display_text[2..].to_string();
                            let lines = (clean_text.len() as f64 / 80.0).ceil().max(1.0);
                            let h = lines * 22.0 + 8.0;
                            (format!(
                                r##"<ul style="margin: 0; padding-left: 20px; font-family: 'Inter', sans-serif; font-size: 14px; color: #334155;">
                                     <li style="line-height: 1.6;">{}</li>
                                   </ul>"##,
                                clean_text
                            ), h)
                        } else if display_text.starts_with("[x] ") {
                            let clean_text = display_text[4..].to_string();
                            let lines = (clean_text.len() as f64 / 80.0).ceil().max(1.0);
                            let h = lines * 22.0 + 8.0;
                            (format!(
                                r##"<div style="font-family: 'Inter', sans-serif; font-size: 14px; color: #334155; display: flex; align-items: flex-start; gap: 8px; margin: 0; line-height: 1.6;">
                                     <span style="color: #6366F1; font-weight: bold; font-size: 16px; line-height: 1;">✓</span>
                                     <span>{}</span>
                                   </div>"##,
                                clean_text
                            ), h)
                        } else if display_text.starts_with("[ ] ") {
                            let clean_text = display_text[4..].to_string();
                            let lines = (clean_text.len() as f64 / 80.0).ceil().max(1.0);
                            let h = lines * 22.0 + 8.0;
                            (format!(
                                r##"<div style="font-family: 'Inter', sans-serif; font-size: 14px; color: #64748B; display: flex; align-items: flex-start; gap: 8px; margin: 0; line-height: 1.6;">
                                     <span style="border: 1px solid #94A3B8; border-radius: 3px; width: 12px; height: 12px; display: inline-block; margin-top: 4px;"></span>
                                     <span>{}</span>
                                   </div>"##,
                                clean_text
                            ), h)
                        } else if display_text.starts_with("> ") {
                            let clean_text = display_text[2..].to_string();
                            let lines = (clean_text.len() as f64 / 80.0).ceil().max(1.0);
                            let h = lines * 24.0 + 16.0;
                            (format!(
                                r##"<blockquote style="margin: 0; padding: 8px 16px; background-color: #F8FAFC; border-left: 4px solid #64748B; border-radius: 0 4px 4px 0; font-family: 'Inter', sans-serif; font-style: italic; font-size: 14px; color: #475569; line-height: 1.6;">
                                     {}
                                   </blockquote>"##,
                                clean_text
                            ), h)
                        } else {
                            let lines = (display_text.len() as f64 / 80.0).ceil().max(1.0);
                            let h = lines * 22.0 + 8.0;
                            (format!(
                                r##"<p style="margin: 0; font-family: 'Inter', sans-serif; font-size: 14px; color: #334155; line-height: 1.6;">{}</p>"##,
                                display_text
                            ), h)
                        };

                        let svg = format!(
                            r##"<foreignObject x="{}" y="{}" width="{}" height="{}">
                                 {}
                               </foreignObject>"##,
                            x, y, width, calculated_h, html_content
                        );
                        (svg, calculated_h + 8.0)
                    }
                    "divider" => {
                        let svg = format!(
                            r##"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="#E2E8F0" stroke-width="1.5" />"##,
                            x, y + 10.0, x + width, y + 10.0
                        );
                        (svg, 20.0)
                    }
                    "spacer" => {
                        let height = config.get_number_property("height").unwrap_or(20.0);
                        ("".to_string(), height)
                    }
                    "stat" => {
                        let val = config.get_string_property("value").unwrap_or_else(|| "0".to_string());
                        let delta = config.get_string_property("delta").unwrap_or_default();
                        let is_pos = !delta.starts_with('-');
                        let delta_span = if delta.is_empty() {
                            "".to_string()
                        } else {
                            format!(
                                r##"<tspan dx="8" font-size="10" font-weight="bold" class="{}">{}</tspan>"##,
                                if is_pos { "stat-delta-pos" } else { "stat-delta-neg" },
                                delta
                            )
                        };
                        let svg = format!(
                            r##"<g>
                                 <rect x="{}" y="{}" width="{}" height="70" fill="#FFFFFF" stroke="#E2E8F0" stroke-width="1" rx="8" />
                                 <text x="{}" y="{}" class="stat-title">{}</text>
                                 <text x="{}" y="{}" class="stat-value">{}{}</text>
                               </g>
                            "##,
                            x, y, width,
                            x + 16.0, y + 24.0, id_str,
                            x + 16.0, y + 54.0, val, delta_span
                        );
                        (svg, 84.0)
                    }
                    "button" => {
                        let label = raw_text.as_deref().unwrap_or("Button");
                        let btn_w = 140.0;
                        let btn_h = 36.0;
                        let svg = format!(
                            r##"<g cursor="pointer">
                                 <rect x="{}" y="{}" width="{}" height="{}" fill="none" stroke="#4F46E5" stroke-width="1" rx="6" stroke-dasharray="2 2" />
                                 <rect x="{}" y="{}" width="{}" height="{}" fill="#4F46E5" rx="6" />
                                 <text x="{}" y="{}" class="button-text" text-anchor="middle">{}</text>
                               </g>
                            "##,
                            x + 2.0, y + 2.0, btn_w, btn_h,
                            x, y, btn_w, btn_h,
                            x + btn_w / 2.0, y + 22.0, label
                        );
                        (svg, 48.0)
                    }
                    "email" | "input" | "password" => {
                        let required_star = if *is_required { r##"<tspan fill="#EF4444">*</tspan>"## } else { "" };
                        let placeholder = config.get_string_property("placeholder").unwrap_or_default();
                        let type_tag = type_assertion.as_ref().map(|t| format!(
                            r##"<text x="{}" y="{}" fill="#64748B" font-family="sans-serif" font-size="11" font-weight="normal" text-anchor="end">:: {}</text>"##,
                            x + width - 12.0, y + 46.0, t
                        )).unwrap_or_default();

                        let svg = format!(
                            r##"<g>
                                 <text x="{}" y="{}" class="label-text">{}{}</text>
                                 <rect x="{}" y="{}" width="{}" height="36" class="input-box" />
                                 <text x="{}" y="{}" fill="#94A3B8" font-family="sans-serif" font-size="13">{}</text>
                                 {}
                               </g>
                            "##,
                            x, y + 14.0, id_str, required_star,
                            x, y + 24.0, width,
                            x + 12.0, y + 46.0, placeholder,
                            type_tag
                        );
                        (svg, 74.0)
                    }
                    "select" => {
                        let first_item = children.first().and_then(|c| {
                            if let AstNodeKind::Element { raw_text, .. } = &c.kind {
                                raw_text.clone()
                            } else {
                                None
                            }
                        }).unwrap_or_else(|| "Select option".to_string());

                        let svg = format!(
                            r##"<g>
                                 <text x="{}" y="{}" class="label-text">{}</text>
                                 <rect x="{}" y="{}" width="{}" height="36" class="input-box" />
                                 <text x="{}" y="{}" fill="#0F172A" font-family="sans-serif" font-size="13">{}</text>
                                 <path d="M {} {} L {} {} L {} {} Z" fill="#64748B" />
                               </g>
                            "##,
                            x, y + 14.0, id_str,
                            x, y + 24.0, width,
                            x + 12.0, y + 46.0, first_item,
                            x + width - 24.0, y + 38.0,
                            x + width - 18.0, y + 38.0,
                            x + width - 21.0, y + 44.0
                        );
                        (svg, 74.0)
                    }
                    "submit" => {
                        let label = raw_text.as_deref().unwrap_or("Submit");
                        let btn_w = 140.0;
                        let btn_h = 36.0;
                        let svg = format!(
                            r##"<g cursor="pointer">
                                 <rect x="{}" y="{}" width="{}" height="{}" fill="none" stroke="#4F46E5" stroke-width="1" rx="6" stroke-dasharray="2 2" />
                                 <rect x="{}" y="{}" width="{}" height="{}" fill="#4F46E5" rx="6" />
                                 <text x="{}" y="{}" class="button-text" text-anchor="middle">{}</text>
                               </g>
                            "##,
                            x + 2.0, y + 2.0, btn_w, btn_h,
                            x, y, btn_w, btn_h,
                            x + btn_w / 2.0, y + 22.0, label
                        );
                        (svg, 48.0)
                    }
                    "bar-chart" | "line-chart" | "pie-chart" | "donut-chart" => {
                        let data_ref = config.get_string_property("data").unwrap_or_default();
                        let color = config.get_string_property("color").unwrap_or_else(|| "#4F46E5".to_string());
                        let dataset = self.get_mock_data(&data_ref);
                        
                        let chart_svg = self.render_svg_chart_inner(tag, id_str, &dataset, &color, width, 180.0);
                        let svg = format!(
                            r##"<svg x="{}" y="{}" width="{}" height="220" viewBox="0 0 {} 220">
                                 {}
                               </svg>
                            "##,
                            x, y, width, width, chart_svg
                        );
                        (svg, 230.0)
                    }
                    "flowchart" | "flow" => {
                        let flow_svg = self.render_flowchart_inner(id_str, children, width, 220.0);
                        let svg = format!(
                            r##"<svg x="{}" y="{}" width="{}" height="250" viewBox="0 0 {} 250">
                                 {}
                               </svg>
                            "##,
                            x, y, width, width, flow_svg
                        );
                        (svg, 260.0)
                    }
                    _ => ("".to_string(), 0.0)
                }
            }
            AstNodeKind::Relationship { .. } => ("".to_string(), 0.0),
            AstNodeKind::Comment(_) => ("".to_string(), 0.0),
            AstNodeKind::Section { .. } => ("".to_string(), 0.0),
        }
    }

    fn get_mock_data(&self, data_ref: &str) -> Vec<(String, f64)> {
        match data_ref {
            "monthly_revenue" => vec![
                ("Jan".to_string(), 4200.0),
                ("Feb".to_string(), 4800.0),
                ("Mar".to_string(), 6200.0),
                ("Apr".to_string(), 5800.0),
                ("May".to_string(), 7100.0),
                ("Jun".to_string(), 8900.0),
            ],
            "user_growth" => vec![
                ("1/1".to_string(), 120.0),
                ("2/1".to_string(), 180.0),
                ("3/1".to_string(), 340.0),
                ("4/1".to_string(), 520.0),
                ("5/1".to_string(), 850.0),
                ("6/1".to_string(), 1240.0),
            ],
            _ => vec![
                ("A".to_string(), 30.0),
                ("B".to_string(), 70.0),
                ("C".to_string(), 45.0),
                ("D".to_string(), 85.0),
            ],
        }
    }

    fn render_svg_chart_inner(&self, chart_type: &str, id: &str, dataset: &[(String, f64)], color: &str, width: f64, height: f64) -> String {
        let padding = 40.0;
        let max_val = dataset.iter().map(|(_, v)| *v).fold(0.0, f64::max).max(1.0);
        
        let mut svg = format!(
            r##"<rect x="0" y="0" width="{}" height="{}" fill="#FFFFFF" stroke="#E2E8F0" stroke-width="1" rx="12" />
               <text x="20" y="24" fill="#64748B" font-family="sans-serif" font-size="11" font-weight="bold" letter-spacing="0.5">{}</text>
            "##,
            width, height, id.to_uppercase()
        );

        let grid_count = 4;
        let chart_h = height - 2.0 * padding;
        let chart_w = width - 2.0 * padding;
        
        for i in 0..=grid_count {
            let y = padding + chart_h * (i as f64) / (grid_count as f64);
            let val = max_val - (max_val * (i as f64) / (grid_count as f64));
            svg.push_str(&format!(
                r##"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="#F1F5F9" stroke-width="1" stroke-dasharray="4" />
                   <text x="{}" y="{}" fill="#64748B" font-family="sans-serif" font-size="9" text-anchor="end" alignment-baseline="middle">{:.0}</text>
                "##,
                padding, y, width - padding, y,
                padding - 8.0, y, val
            ));
        }

        match chart_type {
            "bar-chart" => {
                let count = dataset.len();
                let spacing = 12.0;
                let total_spacing = spacing * (count as f64 + 1.0);
                let bar_width = (chart_w - total_spacing) / (count as f64);

                for (idx, (label, val)) in dataset.iter().enumerate() {
                    let bar_x = padding + spacing + idx as f64 * (bar_width + spacing);
                    let bar_h = (val / max_val) * chart_h;
                    let bar_y = height - padding - bar_h;

                    svg.push_str(&format!(
                        r##"<rect x="{}" y="{}" width="{}" height="{}" fill="{}" rx="4" />
                           <text x="{}" y="{}" fill="#64748B" font-family="sans-serif" font-size="9" text-anchor="middle">{}</text>
                        "##,
                        bar_x, bar_y, bar_width, bar_h, color,
                        bar_x + bar_width / 2.0, height - padding + 14.0, label
                    ));
                }
            }
            "line-chart" => {
                let count = dataset.len();
                let step = chart_w / ((count - 1).max(1) as f64);
                let mut points = Vec::new();
                for (idx, (label, val)) in dataset.iter().enumerate() {
                    let x = padding + idx as f64 * step;
                    let y = height - padding - (val / max_val) * chart_h;
                    points.push((x, y, label, val));
                }

                let mut path_d = String::new();
                for (idx, (x, y, _, _)) in points.iter().enumerate() {
                    if idx == 0 {
                        path_d.push_str(&format!("M {} {}", x, y));
                    } else {
                        path_d.push_str(&format!(" L {} {}", x, y));
                    }
                }

                if !points.is_empty() {
                    svg.push_str(&format!(
                        r##"<path d="{}" fill="none" stroke="{}" stroke-width="3" stroke-linecap="round" />"##,
                        path_d, color
                    ));

                    for (x, y, label, _) in points {
                        svg.push_str(&format!(
                            r##"<circle cx="{}" cy="{}" r="4" fill="#FFFFFF" stroke="{}" stroke-width="2" />
                               <text x="{}" y="{}" fill="#64748B" font-family="sans-serif" font-size="9" text-anchor="middle">{}</text>
                            "##,
                            x, y, color,
                            x, height - padding + 14.0, label
                        ));
                    }
                }
            }
            "pie-chart" | "donut-chart" => {
                let center_x = width / 2.0;
                let center_y = height / 2.0;
                let radius = 55.0;
                let total: f64 = dataset.iter().map(|(_, v)| *v).sum();
                let colors = vec!["#3B82F6", "#10B981", "#F59E0B", "#EF4444", "#8B5CF6", "#EC4899"];
                let mut current_angle = -90.0;

                for (idx, (_, val)) in dataset.iter().enumerate() {
                    let slice_angle = (val / total) * 360.0;
                    let end_angle = current_angle + slice_angle;

                    let r_start = current_angle.to_radians();
                    let r_end = end_angle.to_radians();

                    let x1 = center_x + radius * r_start.cos();
                    let y1 = center_y + radius * r_start.sin();
                    let x2 = center_x + radius * r_end.cos();
                    let y2 = center_y + radius * r_end.sin();

                    let large_arc = if slice_angle > 180.0 { 1 } else { 0 };
                    let slice_color = colors[idx % colors.len()];

                    svg.push_str(&format!(
                        r##"<path d="M {} {} L {} {} A {} {} 0 {} 1 {} {} Z" fill="{}" stroke="#FFFFFF" stroke-width="1.5" />"##,
                        center_x, center_y, x1, y1, radius, radius, large_arc, x2, y2, slice_color
                    ));
                    current_angle = end_angle;
                }

                if chart_type == "donut-chart" {
                    svg.push_str(&format!(
                        r##"<circle cx="{}" cy="{}" r="32" fill="#FFFFFF" stroke="#E2E8F0" stroke-width="1" />
                           <text x="{}" y="{}" fill="#0F172A" font-family="sans-serif" font-size="10" font-weight="bold" text-anchor="middle" alignment-baseline="middle">Total</text>
                           <text x="{}" y="{}" fill="#64748B" font-family="sans-serif" font-size="8" text-anchor="middle" alignment-baseline="middle">{:.0}</text>
                        "##,
                        center_x, center_y,
                        center_x, center_y - 4.0,
                        center_x, center_y + 6.0, total
                    ));
                }
            }
            _ => {}
        }
        svg
    }

    fn render_flowchart_inner(&self, id: &str, children: &[AstNode], width: f64, height: f64) -> String {
        struct FlowNode {
            id: String,
            shape: String,
            label: String,
        }
        struct FlowEdge {
            source: String,
            target: String,
            label: String,
        }

        let mut nodes_map = HashMap::new();
        let mut edges = Vec::new();

        for child in children {
            match &child.kind {
                AstNodeKind::Element { tag, id, config, raw_text, .. } => {
                    if tag == "node" {
                        if let Some(node_id) = id {
                            let shape = config.get_string_property("shape").unwrap_or_else(|| "rect".to_string());
                            let label = raw_text.clone().unwrap_or_else(|| {
                                config.get_first_positional_string().unwrap_or_else(|| node_id.clone())
                            });
                            nodes_map.insert(node_id.clone(), FlowNode {
                                id: node_id.clone(),
                                shape,
                                label,
                            });
                        }
                    }
                }
                AstNodeKind::Relationship { source, target, config } => {
                    let label = config.get_string_property("label").unwrap_or_default();
                    edges.push(FlowEdge {
                        source: source.clone(),
                        target: target.clone(),
                        label,
                    });
                }
                _ => {}
            }
        }

        for edge in &edges {
            if !nodes_map.contains_key(&edge.source) {
                nodes_map.insert(edge.source.clone(), FlowNode {
                    id: edge.source.clone(),
                    shape: "rect".to_string(),
                    label: edge.source.clone(),
                });
            }
            if !nodes_map.contains_key(&edge.target) {
                nodes_map.insert(edge.target.clone(), FlowNode {
                    id: edge.target.clone(),
                    shape: "rect".to_string(),
                    label: edge.target.clone(),
                });
            }
        }

        let mut ranks = HashMap::new();
        for node_id in nodes_map.keys() {
            ranks.insert(node_id.clone(), 0);
        }

        let mut changed = true;
        let mut iterations = 0;
        let node_count = nodes_map.len();
        while changed && iterations < node_count {
            changed = false;
            iterations += 1;
            for edge in &edges {
                let u_rank = *ranks.get(&edge.source).unwrap_or(&0);
                let v_rank = *ranks.get(&edge.target).unwrap_or(&0);
                if v_rank < u_rank + 1 {
                    ranks.insert(edge.target.clone(), u_rank + 1);
                    changed = true;
                }
            }
        }

        let mut rank_groups: HashMap<usize, Vec<String>> = HashMap::new();
        for (node_id, rank) in &ranks {
            rank_groups.entry(*rank).or_default().push(node_id.clone());
        }

        let padding_x = 40.0;
        let padding_y = 35.0;
        let max_rank = rank_groups.keys().max().copied().unwrap_or(0);
        let col_width = if max_rank == 0 {
            width - 2.0 * padding_x
        } else {
            (width - 2.0 * padding_x) / max_rank as f64
        };

        let mut node_coords = HashMap::new();
        for (rank, mut nodes_at_rank) in rank_groups {
            nodes_at_rank.sort();
            let count = nodes_at_rank.len();
            for (idx, node_id) in nodes_at_rank.into_iter().enumerate() {
                let x = padding_x + (rank as f64) * col_width;
                let y = padding_y + ((idx + 1) as f64) * (height - 2.0 * padding_y) / ((count + 1) as f64);
                node_coords.insert(node_id, (x, y));
            }
        }

        let mut svg = format!(
            r##"<rect x="0" y="0" width="{}" height="{}" fill="#FFFFFF" stroke="#E2E8F0" stroke-width="1" rx="12" />
               <text x="20" y="24" fill="#64748B" font-family="sans-serif" font-size="11" font-weight="bold" letter-spacing="0.5">{}</text>
               <defs>
                 <marker id="arrow" viewBox="0 0 10 10" refX="6" refY="5" markerWidth="5" markerHeight="5" orient="auto-start-reverse">
                   <path d="M 0 0 L 10 5 L 0 10 z" fill="#64748B" />
                 </marker>
               </defs>
            "##,
            width, height, id.to_uppercase()
        );

        for edge in &edges {
            if let (Some(start_coord), Some(end_coord)) = (node_coords.get(&edge.source), node_coords.get(&edge.target)) {
                let (x1, y1) = *start_coord;
                let (x2, y2) = *end_coord;

                let dx = x2 - x1;
                let dy = y2 - y1;
                let len = (dx*dx + dy*dy).sqrt();

                let start_shape = nodes_map.get(&edge.source).map(|n| n.shape.as_str()).unwrap_or("rect");
                let end_shape = nodes_map.get(&edge.target).map(|n| n.shape.as_str()).unwrap_or("rect");

                let source_margin = match start_shape {
                    "circle" => 20.0,
                    "diamond" => 25.0,
                    _ => { if dx.abs() > dy.abs() { 35.0 } else { 17.5 } }
                };

                let target_margin = match end_shape {
                    "circle" => 25.0,
                    "diamond" => 29.0,
                    _ => { if dx.abs() > dy.abs() { 40.0 } else { 22.0 } }
                };

                let (adj_x1, adj_y1) = if len > 0.0 { (x1 + dx * source_margin / len, y1 + dy * source_margin / len) } else { (x1, y1) };
                let (adj_x2, adj_y2) = if len > 0.0 { (x2 - dx * target_margin / len, y2 - dy * target_margin / len) } else { (x2, y2) };

                let mid_x = (adj_x1 + adj_x2) / 2.0;
                let path_d = if dy.abs() > 10.0 {
                    format!("M {} {} C {} {}, {} {}, {} {}", adj_x1, adj_y1, mid_x, adj_y1, mid_x, adj_y2, adj_x2, adj_y2)
                } else {
                    format!("M {} {} L {} {}", adj_x1, adj_y1, adj_x2, adj_y2)
                };

                svg.push_str(&format!(
                    r##"<path d="{}" fill="none" stroke="#64748B" stroke-width="1.5" marker-end="url(#arrow)" />"##,
                    path_d
                ));

                if !edge.label.is_empty() {
                    let label_x = (x1 + x2) / 2.0;
                    let label_y = (y1 + y2) / 2.0 - 6.0;
                    svg.push_str(&format!(
                        r##"<text x="{}" y="{}" fill="#475569" font-family="sans-serif" font-size="9" text-anchor="middle">{}</text>"##,
                        label_x, label_y, edge.label
                    ));
                }
            }
        }

        for (node_id, (x, y)) in node_coords {
            if let Some(node) = nodes_map.get(&node_id) {
                let rect_w = 70.0;
                let rect_h = 32.0;
                svg.push_str(&format!(r##"<g transform="translate({}, {})">"##, x, y));

                let text_color;
                match node.shape.as_str() {
                    "circle" => {
                        text_color = "#312E81";
                        svg.push_str(r##"<circle cx="0" cy="0" r="18" fill="#EEF2FF" stroke="#6366F1" stroke-width="1.5" />"##);
                    }
                    "diamond" => {
                        text_color = "#831843";
                        svg.push_str(r##"<polygon points="0,-18 22,0 0,18 -22,0" fill="#FDF2F8" stroke="#DB2777" stroke-width="1.5" />"##);
                    }
                    _ => {
                        text_color = "#115E59";
                        svg.push_str(&format!(
                            r##"<rect x="-{}" y="-{}" width="{}" height="{}" fill="#F0FDFA" stroke="#0D9488" stroke-width="1.5" rx="6" />"##,
                            rect_w / 2.0, rect_h / 2.0, rect_w, rect_h
                        ));
                    }
                }

                svg.push_str(&format!(
                    r##"<text x="0" y="3.5" fill="{}" font-family="sans-serif" font-size="9" font-weight="bold" text-anchor="middle">{}</text>
                       </g>
                    "##,
                    text_color, node.label
                ));
            }
        }
        svg
    }

    fn wrap_in_svg_template(&self, content: &str, height: f64) -> String {
        format!(
            r##"<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="{}" viewBox="0 0 1000 {}" style="background-color: #FFFFFF;">
  <style>
    .slate-title {{ font-family: 'Outfit', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; font-size: 32px; font-weight: 700; fill: #0F172A; }}
    .slate-subtitle {{ font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; font-size: 16px; fill: #64748B; }}
    .slate-heading {{ font-family: 'Outfit', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; font-size: 18px; font-weight: 600; fill: #0F172A; }}
    .slate-text {{ font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; font-size: 14px; fill: #334155; }}
    .stat-title {{ font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; font-size: 11px; fill: #64748B; font-weight: 600; text-transform: uppercase; }}
    .stat-value {{ font-family: 'Outfit', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; font-size: 20px; fill: #0F172A; font-weight: 700; }}
    .stat-delta-pos {{ fill: #03543F; font-weight: bold; }}
    .stat-delta-neg {{ fill: #9B1C1C; font-weight: bold; }}
    .label-text {{ font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; font-size: 12px; fill: #64748B; font-weight: 600; }}
    .input-box {{ fill: #FFFFFF; stroke: #D1D5DB; stroke-width: 1; rx: 6px; }}
    .button-rect {{ fill: #4F46E5; rx: 6px; }}
    .button-text {{ font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; font-size: 13px; fill: #FFFFFF; font-weight: 600; }}
    .divider-line {{ stroke: #E2E8F0; stroke-width: 1; }}
  </style>
  {}
</svg>"##,
            height, height, content
        )
    }
}
