// slate.js — Slate Native Browser Rendering Engine v1.0
// Reads Slate JSON AST and renders documents natively in any browser.
// Zero HTML templates. Pure DOM construction from structured JSON data.

var Slate = (function () {
  'use strict';

  // Grayscale palette
  var C = {
    text:    '#24292f',
    muted:   '#57606a',
    subtle:  '#8c959f',
    border:  '#d0d7de',
    bg:      '#f6f8fa',
    white:   '#ffffff',
    red:     '#cf222e',
    dark:    '#1f2328'
  };

  var FONT = '-apple-system, BlinkMacSystemFont, "Segoe UI", Helvetica, Arial, sans-serif';
  var MONO = 'Consolas, "Liberation Mono", Menlo, Courier, monospace';

  // Mock data for charts
  var MOCK_DATA = {
    monthly_revenue: [
      { label: 'Jan', value: 4200 },
      { label: 'Feb', value: 4800 },
      { label: 'Mar', value: 6200 },
      { label: 'Apr', value: 5800 },
      { label: 'May', value: 7100 },
      { label: 'Jun', value: 8900 }
    ],
    user_growth: [
      { label: '1/1', value: 120 },
      { label: '2/1', value: 180 },
      { label: '3/1', value: 340 },
      { label: '4/1', value: 520 },
      { label: '5/1', value: 850 },
      { label: '6/1', value: 1240 }
    ]
  };

  var DEFAULT_DATA = [
    { label: 'A', value: 30 },
    { label: 'B', value: 70 },
    { label: 'C', value: 45 },
    { label: 'D', value: 85 }
  ];

  var GRAY_PALETTE = [C.text, C.muted, C.subtle, C.border, C.bg];

  function el(tagName, styles, parent) {
    var node = document.createElement(tagName);
    if (styles) {
      for (var k in styles) {
        if (styles.hasOwnProperty(k)) {
          node.style[k] = styles[k];
        }
      }
    }
    if (parent) parent.appendChild(node);
    return node;
  }

  function txt(parent, text) {
    parent.appendChild(document.createTextNode(text));
  }

  function cfg(node, key) {
    if (node.config && node.config[key] !== undefined) return node.config[key];
    return null;
  }

  function renderNode(node, parent) {
    if (!node || !node.tag) return;
    var tag = node.tag;

    switch (tag) {
      case 'title':
        var h1 = el('div', {
          fontSize: '2.2em', fontWeight: '600', fontFamily: FONT,
          color: C.text, borderBottom: '1px solid ' + C.border,
          paddingBottom: '0.3em', marginTop: '24px', marginBottom: '16px'
        }, parent);
        txt(h1, node.text || '');
        break;

      case 'subtitle':
        var sub = el('div', {
          fontSize: '16px', color: C.muted, fontFamily: FONT,
          marginTop: '-8px', marginBottom: '24px'
        }, parent);
        txt(sub, node.text || '');
        break;

      case 'heading':
        var h3 = el('div', {
          fontSize: '1.3em', fontWeight: '600', fontFamily: FONT,
          color: C.text, marginTop: '24px', marginBottom: '16px'
        }, parent);
        txt(h3, node.text || '');
        break;

      case 'text':
        renderText(node, parent);
        break;

      case 'divider':
        el('div', {
          height: '0.25em', backgroundColor: C.border,
          margin: '24px 0', border: '0'
        }, parent);
        break;

      case 'spacer':
        var height = cfg(node, 'height') || 20;
        el('div', { height: height + 'px' }, parent);
        break;

      case 'section':
      case 'page':
      case 'form':
        var sec = el('div', { marginBottom: '16px' }, parent);
        renderChildren(node, sec);
        break;

      case 'grid':
        var grid = el('div', {
          display: 'grid',
          gridTemplateColumns: 'repeat(auto-fit, minmax(300px, 1fr))',
          gap: '24px', marginBottom: '16px'
        }, parent);
        renderChildren(node, grid);
        break;

      case 'row':
        var row = el('div', {
          display: 'flex', flexWrap: 'wrap',
          gap: '16px', marginBottom: '16px'
        }, parent);
        renderChildren(node, row);
        break;

      case 'card':
        var card = el('div', {
          backgroundColor: C.white, border: '1px solid ' + C.border,
          borderLeft: '4px solid ' + C.text, padding: '24px',
          marginBottom: '16px', boxSizing: 'border-box'
        }, parent);
        renderChildren(node, card);
        break;

      case 'stat':
        renderStat(node, parent);
        break;

      case 'email':
      case 'input':
      case 'password':
        renderField(node, parent);
        break;

      case 'select':
        renderSelect(node, parent);
        break;

      case 'button':
      case 'submit':
        renderButton(node, parent);
        break;

      case 'bar-chart':
      case 'line-chart':
      case 'pie-chart':
      case 'donut-chart':
        renderChart(node, parent);
        break;

      case 'flowchart':
      case 'flow':
        renderFlowchart(node, parent);
        break;

      case 'item':
        // items are consumed by their parent (select)
        break;

      default:
        break;
    }
  }

  function renderChildren(node, parent) {
    if (node.children) {
      for (var i = 0; i < node.children.length; i++) {
        renderNode(node.children[i], parent);
      }
    }
  }

  function renderText(node, parent) {
    var t = node.text || '';

    if (t.indexOf('- ') === 0 || t.indexOf('* ') === 0) {
      var li = el('div', {
        fontFamily: FONT, fontSize: '15px', color: C.text,
        paddingLeft: '1.5em', marginBottom: '4px', lineHeight: '1.5',
        textIndent: '-1em'
      }, parent);
      txt(li, '\u2022 ' + t.substring(2));
    } else if (t.indexOf('[x] ') === 0) {
      var cb = el('div', {
        fontFamily: FONT, fontSize: '15px', color: C.text,
        display: 'flex', alignItems: 'center', gap: '8px', marginBottom: '8px'
      }, parent);
      var box1 = el('span', { fontSize: '16px', lineHeight: '1' }, cb);
      txt(box1, '\u2611');
      var lbl1 = el('span', {}, cb);
      txt(lbl1, t.substring(4));
    } else if (t.indexOf('[ ] ') === 0) {
      var cb2 = el('div', {
        fontFamily: FONT, fontSize: '15px', color: C.text,
        display: 'flex', alignItems: 'center', gap: '8px', marginBottom: '8px'
      }, parent);
      var box2 = el('span', { fontSize: '16px', lineHeight: '1' }, cb2);
      txt(box2, '\u2610');
      var lbl2 = el('span', {}, cb2);
      txt(lbl2, t.substring(4));
    } else if (t.indexOf('> ') === 0) {
      var bq = el('div', {
        fontFamily: FONT, fontSize: '15px', color: C.muted,
        borderLeft: '0.25em solid ' + C.border,
        padding: '8px 16px', margin: '0 0 16px 0',
        backgroundColor: C.bg
      }, parent);
      txt(bq, t.substring(2));
    } else {
      var p = el('div', {
        fontFamily: FONT, fontSize: '15px', color: C.text,
        marginBottom: '16px', lineHeight: '1.5'
      }, parent);
      txt(p, t);
    }
  }

  function renderStat(node, parent) {
    var wrap = el('div', {
      border: '1px solid ' + C.border, padding: '16px',
      marginBottom: '12px', backgroundColor: C.white
    }, parent);
    var title = el('div', {
      fontSize: '10px', fontWeight: '600', color: C.muted,
      textTransform: 'uppercase', marginBottom: '4px',
      letterSpacing: '0.5px', fontFamily: FONT
    }, wrap);
    txt(title, node.id || '');
    var valRow = el('div', {
      fontSize: '20px', fontWeight: '700', color: C.text, fontFamily: FONT
    }, wrap);
    txt(valRow, cfg(node, 'value') || '0');
    var delta = cfg(node, 'delta');
    if (delta) {
      var ds = el('span', {
        fontSize: '11px', fontWeight: '600', color: C.muted,
        marginLeft: '8px', fontFamily: FONT
      }, valRow);
      txt(ds, '' + delta);
    }
  }

  function renderField(node, parent) {
    var wrap = el('div', {
      padding: '6px 0', fontSize: '14px', lineHeight: '1.5',
      color: C.text, marginBottom: '8px', fontFamily: FONT
    }, parent);
    var lbl = el('span', { fontWeight: '600' }, wrap);
    txt(lbl, node.id || '');
    if (node.required) {
      var star = el('span', { color: C.red, marginLeft: '2px', fontWeight: 'bold' }, wrap);
      txt(star, '*');
    }
    txt(wrap, ': ');
    var ph = cfg(node, 'placeholder') || '';
    var val = el('span', { color: C.muted, fontStyle: 'italic' }, wrap);
    txt(val, ph);
    if (node.typeAssertion) {
      var ta = el('span', { color: C.subtle, fontSize: '12px', marginLeft: '4px', fontFamily: MONO }, wrap);
      txt(ta, ':: ' + node.typeAssertion);
    }
  }

  function renderSelect(node, parent) {
    var wrap = el('div', {
      padding: '6px 0', fontSize: '14px', lineHeight: '1.5',
      color: C.text, marginBottom: '8px', fontFamily: FONT
    }, parent);
    var lbl = el('span', { fontWeight: '600' }, wrap);
    txt(lbl, node.id || '');
    if (node.required) {
      var star = el('span', { color: C.red, marginLeft: '2px', fontWeight: 'bold' }, wrap);
      txt(star, '*');
    }
    txt(wrap, ': ');

    var options = [];
    if (node.children) {
      for (var i = 0; i < node.children.length; i++) {
        if (node.children[i].tag === 'item' && node.children[i].text) {
          options.push(node.children[i].text);
        }
      }
    }
    var first = options.length > 0 ? options[0] : 'Select option';
    var fv = el('span', { color: C.text }, wrap);
    txt(fv, first);
    if (options.length > 0) {
      var opts = el('span', { color: C.muted, fontSize: '13px', marginLeft: '8px' }, wrap);
      txt(opts, '(options: ' + options.join(', ') + ')');
    }
  }

  function renderButton(node, parent) {
    var btn = el('div', {
      display: 'inline-block', padding: '4px 12px',
      border: '1px solid ' + C.border, backgroundColor: C.bg,
      color: C.text, fontWeight: '600', fontSize: '13px',
      fontFamily: FONT, marginTop: '8px', marginBottom: '8px',
      cursor: 'default', userSelect: 'none'
    }, parent);
    txt(btn, '[' + (node.text || node.id || '') + ']');
  }

  // --- Charts ---

  function getDataset(node) {
    var ref = cfg(node, 'data') || '';
    if (MOCK_DATA[ref]) return MOCK_DATA[ref];
    return DEFAULT_DATA;
  }

  function renderChart(node, parent) {
    var container = el('div', {
      border: '1px solid ' + C.border, padding: '16px',
      marginBottom: '16px', backgroundColor: C.white
    }, parent);
    var titleEl = el('div', {
      fontSize: '10px', fontWeight: '600', color: C.muted,
      textTransform: 'uppercase', marginBottom: '12px',
      letterSpacing: '0.5px', fontFamily: FONT
    }, container);
    txt(titleEl, node.id || '');

    var data = getDataset(node);
    var tag = node.tag;

    if (tag === 'bar-chart') renderBarChart(data, container);
    else if (tag === 'line-chart') renderLineChart(data, container);
    else if (tag === 'pie-chart') renderPieChart(data, container, false);
    else if (tag === 'donut-chart') renderPieChart(data, container, true);
  }

  function renderBarChart(data, parent) {
    var maxVal = 1;
    for (var i = 0; i < data.length; i++) {
      if (data[i].value > maxVal) maxVal = data[i].value;
    }

    var wrap = el('div', {
      display: 'flex', height: '180px', padding: '16px 16px 8px 16px',
      boxSizing: 'border-box', fontFamily: FONT
    }, parent);

    // Y axis
    var yAxis = el('div', {
      display: 'flex', flexDirection: 'column', justifyContent: 'space-between',
      alignItems: 'flex-end', paddingRight: '8px', fontSize: '10px',
      color: C.muted, height: '130px', minWidth: '35px'
    }, wrap);
    for (var g = 0; g <= 4; g++) {
      var yv = el('div', {}, yAxis);
      txt(yv, '' + Math.round(maxVal - (maxVal * g / 4)));
    }

    // Bars
    var bars = el('div', {
      flexGrow: '1', display: 'flex', justifyContent: 'space-around',
      alignItems: 'flex-end', height: '130px',
      borderLeft: '1px solid ' + C.border,
      borderBottom: '1px solid ' + C.border,
      paddingLeft: '8px'
    }, wrap);

    for (var b = 0; b < data.length; b++) {
      var pct = (data[b].value / maxVal) * 100;
      var col = el('div', {
        display: 'flex', flexDirection: 'column', alignItems: 'center',
        flexGrow: '1', height: '100%', justifyContent: 'flex-end'
      }, bars);
      var barWrap = el('div', {
        width: '100%', height: '100%', display: 'flex',
        alignItems: 'flex-end', justifyContent: 'center'
      }, col);
      el('div', {
        width: '60%', maxWidth: '28px', height: pct + '%',
        backgroundColor: C.subtle, border: '1px solid ' + C.muted
      }, barWrap);
      var lbl = el('div', {
        fontSize: '10px', color: C.muted, marginTop: '8px', whiteSpace: 'nowrap'
      }, col);
      txt(lbl, data[b].label);
    }
  }

  function renderLineChart(data, parent) {
    var maxVal = 1;
    for (var i = 0; i < data.length; i++) {
      if (data[i].value > maxVal) maxVal = data[i].value;
    }

    var wrap = el('div', {
      display: 'flex', height: '180px', padding: '16px 16px 8px 16px',
      boxSizing: 'border-box', fontFamily: FONT
    }, parent);

    // Y axis
    var yAxis = el('div', {
      display: 'flex', flexDirection: 'column', justifyContent: 'space-between',
      alignItems: 'flex-end', paddingRight: '8px', fontSize: '10px',
      color: C.muted, height: '120px', minWidth: '35px'
    }, wrap);
    for (var g = 0; g <= 4; g++) {
      var yv = el('div', {}, yAxis);
      txt(yv, '' + Math.round(maxVal - (maxVal * g / 4)));
    }

    // Plot area
    var plotArea = el('div', {
      flexGrow: '1', position: 'relative', height: '120px',
      borderLeft: '1px solid ' + C.border,
      borderBottom: '1px solid ' + C.border
    }, wrap);

    var ns = 'http://www.w3.org/2000/svg';
    var svg = document.createElementNS(ns, 'svg');
    svg.setAttribute('viewBox', '0 0 700 120');
    svg.setAttribute('preserveAspectRatio', 'none');
    svg.style.display = 'block';
    svg.style.width = '100%';
    svg.style.height = '100%';
    plotArea.appendChild(svg);

    var step = 700 / Math.max(data.length - 1, 1);
    var d = '';
    for (var p = 0; p < data.length; p++) {
      var px = p * step;
      var py = 120 - (data[p].value / maxVal) * 120;
      d += (p === 0 ? 'M ' : ' L ') + px.toFixed(1) + ' ' + py.toFixed(1);
    }

    var path = document.createElementNS(ns, 'path');
    path.setAttribute('d', d);
    path.setAttribute('fill', 'none');
    path.setAttribute('stroke', C.text);
    path.setAttribute('stroke-width', '2.5');
    svg.appendChild(path);

    // X axis labels
    var xAxis = el('div', {
      display: 'flex', justifyContent: 'space-between',
      marginTop: '8px', fontSize: '10px', color: C.muted,
      paddingLeft: '4px', paddingRight: '4px'
    }, plotArea);
    for (var x = 0; x < data.length; x++) {
      var xl = el('div', {}, xAxis);
      txt(xl, data[x].label);
    }
  }

  function renderPieChart(data, parent, isDonut) {
    var total = 0;
    for (var i = 0; i < data.length; i++) total += data[i].value;

    var wrap = el('div', {
      display: 'flex', alignItems: 'center', height: '180px',
      padding: '16px', boxSizing: 'border-box', fontFamily: FONT
    }, parent);

    // SVG pie
    var gfx = el('div', { width: '120px', height: '120px', flexShrink: '0' }, wrap);
    var ns = 'http://www.w3.org/2000/svg';
    var svg = document.createElementNS(ns, 'svg');
    svg.setAttribute('viewBox', '0 0 120 120');
    svg.style.display = 'block';
    svg.style.width = '100%';
    svg.style.height = '100%';
    gfx.appendChild(svg);

    var angle = -90;
    for (var s = 0; s < data.length; s++) {
      var slice = (data[s].value / total) * 360;
      var end = angle + slice;
      var r1 = angle * Math.PI / 180;
      var r2 = end * Math.PI / 180;
      var x1 = 60 + 50 * Math.cos(r1);
      var y1 = 60 + 50 * Math.sin(r1);
      var x2 = 60 + 50 * Math.cos(r2);
      var y2 = 60 + 50 * Math.sin(r2);
      var lg = slice > 180 ? 1 : 0;
      var color = GRAY_PALETTE[s % GRAY_PALETTE.length];

      var p = document.createElementNS(ns, 'path');
      p.setAttribute('d', 'M 60 60 L ' + x1.toFixed(2) + ' ' + y1.toFixed(2) +
        ' A 50 50 0 ' + lg + ' 1 ' + x2.toFixed(2) + ' ' + y2.toFixed(2) + ' Z');
      p.setAttribute('fill', color);
      p.setAttribute('stroke', C.white);
      p.setAttribute('stroke-width', '1');
      svg.appendChild(p);
      angle = end;
    }

    if (isDonut) {
      var circ = document.createElementNS(ns, 'circle');
      circ.setAttribute('cx', '60');
      circ.setAttribute('cy', '60');
      circ.setAttribute('r', '30');
      circ.setAttribute('fill', C.white);
      circ.setAttribute('stroke', C.border);
      circ.setAttribute('stroke-width', '1');
      svg.appendChild(circ);

      var t1 = document.createElementNS(ns, 'text');
      t1.setAttribute('x', '60');
      t1.setAttribute('y', '58');
      t1.setAttribute('fill', C.text);
      t1.setAttribute('font-family', MONO);
      t1.setAttribute('font-size', '9');
      t1.setAttribute('font-weight', 'bold');
      t1.setAttribute('text-anchor', 'middle');
      t1.textContent = 'Total';
      svg.appendChild(t1);

      var t2 = document.createElementNS(ns, 'text');
      t2.setAttribute('x', '60');
      t2.setAttribute('y', '70');
      t2.setAttribute('fill', C.muted);
      t2.setAttribute('font-family', MONO);
      t2.setAttribute('font-size', '8');
      t2.setAttribute('text-anchor', 'middle');
      t2.textContent = '' + Math.round(total);
      svg.appendChild(t2);
    }

    // Legend
    var legend = el('div', {
      flexGrow: '1', marginLeft: '24px', display: 'flex',
      flexDirection: 'column', gap: '8px'
    }, wrap);
    for (var l = 0; l < data.length; l++) {
      var pct = ((data[l].value / total) * 100).toFixed(1);
      var item = el('div', {
        display: 'flex', alignItems: 'center', fontSize: '12px', color: C.text
      }, legend);
      el('div', {
        width: '12px', height: '12px', marginRight: '8px',
        backgroundColor: GRAY_PALETTE[l % GRAY_PALETTE.length],
        border: '1px solid ' + C.border, display: 'inline-block'
      }, item);
      var lt = el('span', { whiteSpace: 'nowrap' }, item);
      txt(lt, data[l].label + ' (' + pct + '%)');
    }
  }

  // --- Flowchart ---

  function renderFlowchart(node, parent) {
    var container = el('div', {
      border: '1px solid ' + C.border, padding: '16px',
      marginBottom: '16px', backgroundColor: C.white
    }, parent);
    var titleEl = el('div', {
      fontSize: '10px', fontWeight: '600', color: C.muted,
      textTransform: 'uppercase', marginBottom: '12px',
      letterSpacing: '0.5px', fontFamily: FONT
    }, container);
    txt(titleEl, node.id || '');

    if (!node.children) return;

    var nodesMap = {};
    var edges = [];

    for (var i = 0; i < node.children.length; i++) {
      var ch = node.children[i];
      if (ch.tag === 'node' && ch.id) {
        var shape = (ch.config && ch.config.shape) ? ch.config.shape : 'rect';
        nodesMap[ch.id] = { id: ch.id, shape: shape, label: ch.text || ch.id };
      } else if (ch.tag === 'relationship') {
        var label = (ch.config && ch.config.label) ? ch.config.label : '';
        edges.push({ source: ch.source, target: ch.target, label: label });
      }
    }

    // Auto-create missing nodes from edges
    for (var e = 0; e < edges.length; e++) {
      if (!nodesMap[edges[e].source]) {
        nodesMap[edges[e].source] = { id: edges[e].source, shape: 'rect', label: edges[e].source };
      }
      if (!nodesMap[edges[e].target]) {
        nodesMap[edges[e].target] = { id: edges[e].target, shape: 'rect', label: edges[e].target };
      }
    }

    // Topological ranking (vertical layout)
    var ranks = {};
    for (var nid in nodesMap) ranks[nid] = 0;

    var changed = true;
    var iter = 0;
    var nodeIds = Object.keys(nodesMap);
    while (changed && iter < nodeIds.length) {
      changed = false;
      iter++;
      for (var ei = 0; ei < edges.length; ei++) {
        var uR = ranks[edges[ei].source] || 0;
        var vR = ranks[edges[ei].target] || 0;
        if (vR < uR + 1) {
          ranks[edges[ei].target] = uR + 1;
          changed = true;
        }
      }
    }

    var rankGroups = {};
    var maxRank = 0;
    for (var nk in ranks) {
      var r = ranks[nk];
      if (r > maxRank) maxRank = r;
      if (!rankGroups[r]) rankGroups[r] = [];
      rankGroups[r].push(nk);
    }
    for (var rk in rankGroups) rankGroups[rk].sort();

    var width = 700;
    var rowH = 80;
    var padX = 40;
    var padY = 40;
    var height = padY * 2 + maxRank * rowH;

    var coords = {};
    for (var rr in rankGroups) {
      var group = rankGroups[rr];
      var count = group.length;
      for (var gi = 0; gi < count; gi++) {
        var y = padY + parseInt(rr) * rowH;
        var x = padX + (gi + 1) * (width - 2 * padX) / (count + 1);
        coords[group[gi]] = { x: x, y: y };
      }
    }

    // Draw as SVG
    var ns = 'http://www.w3.org/2000/svg';
    var svg = document.createElementNS(ns, 'svg');
    svg.setAttribute('viewBox', '0 0 ' + width + ' ' + height.toFixed(1));
    svg.style.display = 'block';
    svg.style.width = '100%';
    svg.style.height = 'auto';
    container.appendChild(svg);

    // Background
    var bg = document.createElementNS(ns, 'rect');
    bg.setAttribute('x', '0'); bg.setAttribute('y', '0');
    bg.setAttribute('width', '' + width); bg.setAttribute('height', '' + height.toFixed(1));
    bg.setAttribute('fill', C.white); bg.setAttribute('stroke', C.border);
    bg.setAttribute('stroke-width', '1'); bg.setAttribute('rx', '0');
    svg.appendChild(bg);

    // Arrow marker
    var defs = document.createElementNS(ns, 'defs');
    var marker = document.createElementNS(ns, 'marker');
    marker.setAttribute('id', 'arrowhead');
    marker.setAttribute('viewBox', '0 0 10 10');
    marker.setAttribute('refX', '6'); marker.setAttribute('refY', '5');
    marker.setAttribute('markerWidth', '5'); marker.setAttribute('markerHeight', '5');
    marker.setAttribute('orient', 'auto-start-reverse');
    var arrowPath = document.createElementNS(ns, 'path');
    arrowPath.setAttribute('d', 'M 0 0 L 10 5 L 0 10 z');
    arrowPath.setAttribute('fill', C.subtle);
    marker.appendChild(arrowPath);
    defs.appendChild(marker);
    svg.appendChild(defs);

    // Draw edges
    for (var de = 0; de < edges.length; de++) {
      var edge = edges[de];
      var sc = coords[edge.source];
      var tc = coords[edge.target];
      if (!sc || !tc) continue;

      var dx = tc.x - sc.x;
      var dy = tc.y - sc.y;
      var len = Math.sqrt(dx * dx + dy * dy);
      if (len === 0) continue;

      var sm = Math.abs(dx) > Math.abs(dy) ? 38 : 20;
      var tm = Math.abs(dx) > Math.abs(dy) ? 38 : 20;

      var ax1 = sc.x + dx * sm / len;
      var ay1 = sc.y + dy * sm / len;
      var ax2 = tc.x - dx * tm / len;
      var ay2 = tc.y - dy * tm / len;

      var line = document.createElementNS(ns, 'path');
      line.setAttribute('d', 'M ' + ax1.toFixed(1) + ' ' + ay1.toFixed(1) +
        ' L ' + ax2.toFixed(1) + ' ' + ay2.toFixed(1));
      line.setAttribute('fill', 'none');
      line.setAttribute('stroke', C.subtle);
      line.setAttribute('stroke-width', '1.5');
      line.setAttribute('marker-end', 'url(#arrowhead)');
      svg.appendChild(line);

      if (edge.label) {
        var lx = (ax1 + ax2) / 2 + 8;
        var ly = (ay1 + ay2) / 2;
        var lt = document.createElementNS(ns, 'text');
        lt.setAttribute('x', lx.toFixed(1));
        lt.setAttribute('y', ly.toFixed(1));
        lt.setAttribute('fill', C.muted);
        lt.setAttribute('font-family', MONO);
        lt.setAttribute('font-size', '9');
        lt.setAttribute('text-anchor', 'start');
        lt.setAttribute('alignment-baseline', 'middle');
        lt.textContent = edge.label;
        svg.appendChild(lt);
      }
    }

    // Draw nodes
    for (var dn in coords) {
      var nd = nodesMap[dn];
      var c = coords[dn];
      if (!nd) continue;

      var g = document.createElementNS(ns, 'g');
      g.setAttribute('transform', 'translate(' + c.x.toFixed(1) + ', ' + c.y.toFixed(1) + ')');

      var rect = document.createElementNS(ns, 'rect');
      rect.setAttribute('x', '-35'); rect.setAttribute('y', '-16');
      rect.setAttribute('width', '70'); rect.setAttribute('height', '32');
      rect.setAttribute('fill', C.white); rect.setAttribute('rx', '0');

      if (nd.shape === 'circle') {
        rect.setAttribute('stroke', C.text);
        rect.setAttribute('stroke-width', '2.5');
      } else if (nd.shape === 'diamond') {
        rect.setAttribute('stroke', C.text);
        rect.setAttribute('stroke-width', '1.5');
        rect.setAttribute('stroke-dasharray', '3,3');
      } else {
        rect.setAttribute('stroke', C.text);
        rect.setAttribute('stroke-width', '1');
      }
      g.appendChild(rect);

      var label = document.createElementNS(ns, 'text');
      label.setAttribute('x', '0'); label.setAttribute('y', '3.5');
      label.setAttribute('fill', C.text);
      label.setAttribute('font-family', MONO);
      label.setAttribute('font-size', '9');
      label.setAttribute('font-weight', 'bold');
      label.setAttribute('text-anchor', 'middle');
      label.textContent = nd.label;
      g.appendChild(label);

      svg.appendChild(g);
    }
  }

  // --- Public API ---

  function render(json, targetElement) {
    if (typeof json === 'string') {
      json = JSON.parse(json);
    }
    targetElement.innerHTML = '';

    // Apply base styles to target
    targetElement.style.fontFamily = FONT;
    targetElement.style.fontSize = '15px';
    targetElement.style.lineHeight = '1.5';
    targetElement.style.color = C.text;
    targetElement.style.backgroundColor = C.white;
    targetElement.style.margin = '0';
    targetElement.style.padding = '40px';
    targetElement.style.maxWidth = '850px';
    targetElement.style.marginLeft = 'auto';
    targetElement.style.marginRight = 'auto';
    targetElement.style.boxSizing = 'border-box';

    if (json.nodes) {
      for (var i = 0; i < json.nodes.length; i++) {
        renderNode(json.nodes[i], targetElement);
      }
    }
  }

  function renderFromUrl(url, targetElement) {
    var xhr = new XMLHttpRequest();
    xhr.open('GET', url, true);
    xhr.onreadystatechange = function () {
      if (xhr.readyState === 4 && xhr.status === 200) {
        render(xhr.responseText, targetElement);
      }
    };
    xhr.send();
  }

  return {
    render: render,
    renderFromUrl: renderFromUrl
  };
})();
