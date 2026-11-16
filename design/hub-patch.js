// hub-patch.js: the screen of hub-app, a smart home hub (1920 x 1080): a navigation rail on
// the left, the time and the weather on top, a grid of tiles (camera, climate, lights, dimmer,
// fan, intercom, door, the next 24 hours) and a news column on the right. Every control is a
// slot of the Library (components-patch.js); the app (packages/services/display_safety/utils/
// hub-app) drives the #hub/* names. Applied with everyware's roundtrip.js to the EveryWare Library .fig, before ui-lib's
// dark-theme-patch.js (the colours here are the Library's light palette, the theme maps them):
//
//   design/build.sh     # the Library, this patch, the dark theme, data/hub.dcf
//
// The root is a top-level frame of the Library's Page 1, next to #ui-gallery; nothing of the Library
// changes. Numbers are pixels of the 1920 x 1080 screen.

// roundtrip.js hands over fig-helpers.js (the node authoring) as the second argument
module.exports = function (msg, { key, C, FONT, authoring }) {
  const A = authoring(msg, 'hub');
  const { nodes, kids, frame, rect, text, instance, shadow, solid, log } = A;
  // the app's own pictures of the weather, a set next to the Library's (weather-pictures.js)
  require('./weather-pictures.js')(A, C, '#hub-weather');

  // the Library's .fig (ui-lib design/build-library.sh): the screen goes on its Page 1
  const page = nodes.find(n => n.type === 'CANVAS' && n.name === 'Page 1');
  if (!page) throw new Error('no Page 1: build on the EveryWare Library .fig');
  if (nodes.some(n => n.name === '#hub-root')) throw new Error('#hub-root exists already');

  // a variant of a Library set by its name
  const variant = (set, name) => {
    const s = nodes.find(n => n.isStateGroup && n.name === set);
    const v = s && kids(key(s.guid)).find(n => n.type === 'SYMBOL' && n.name === name);
    if (!v) throw new Error(`no variant ${set} / ${name}`);
    return v;
  };
  const deep = (n, name) => { for (const c of kids(key(n.guid))) { if (c.name === name) return c; const d = deep(c, name); if (d) return d; } return null; };
  // a slot: a frame named for the app, fixed auto layout of the variant's size, a preview
  // instance of the variant inside (the app's clone replaces it)
  const slot = (parent, name, x, y, v, overrides = []) => {
    const f = frame(parent, name, x, y, v.size.x, v.size.y, {
      bordersTakeSpace: false, stackMode: 'HORIZONTAL', stackPrimaryAlignItems: 'CENTER', stackCounterAlignItems: 'CENTER',
      stackPrimarySizing: 'FIXED', stackCounterSizing: 'FIXED', stackLayoutVersion: 1,
    });
    instance(f, `${name}/preview`, 0, 0, v, null, overrides);
    return f;
  };
  const label = (v, part, chars) => {
    const t = deep(v, part); if (!t) throw new Error(`no ${part} in ${v.name}`);
    return [{ guidPath: { guids: [t.guid] }, textData: { characters: chars, lines: t.textData.lines } }];
  };
  const card = (name, x, y, w, h, extra) => frame(root, name, x, y, w, h, Object.assign({
    fillPaints: [solid(C.white)], cornerRadius: 28, frameMaskDisabled: false, effects: shadow(4, 16, 0.06),
  }, extra || {}));
  const title = (parent, s, y = 28) => text(parent, 'Title', 28, y, parent.size.x - 56, 30, s, 24, FONT.semibold, C.ink);
  const sub = (parent, name, s, y = 64) => text(parent, name, 28, y, parent.size.x - 56, 22, s, 16, FONT.regular, C.ink2);

  // ---- the root
  const root = frame(page, '#hub-root', -2400, 0, 1920, 1080, { fillPaints: [solid(C.page)], frameMaskDisabled: false });

  // ---- the rail: the house, the scenes, Wi-Fi; settings at the bottom
  const rail = frame(root, 'Rail', 0, 0, 112, 1080, { fillPaints: [solid(C.surface)] });
  const navButton = icon => variant('#icon-button', `kind=tonal, state=default, icon=${icon}, size=regular`);
  slot(rail, '#hub/nav/home', 20, 40, navButton('house'));
  slot(rail, '#hub/nav/scenes', 20, 160, navButton('sliders-horizontal'));
  slot(rail, '#hub/nav/wifi', 20, 256, navButton('wifi'));
  slot(rail, '#hub/nav/settings', 20, 968, navButton('settings-2'));

  // ---- the top row: time, date, the weather now
  text(root, '#hub/time', 152, 28, 360, 100, '08:30', 80, FONT.regular, C.ink);
  text(root, '#hub/date', 156, 130, 360, 24, 'SUN, 8 NOV', 18, FONT.semibold, C.ink2);
  slot(root, '#hub/weather/icon', 560, 54, variant('#hub-weather', 'kind=partly'));
  text(root, '#hub/weather/temp', 612, 40, 130, 60, '18°', 48, FONT.regular, C.ink);
  text(root, '#hub/weather/condition', 760, 44, 560, 30, 'Partly cloudy', 24, FONT.regular, C.ink);
  text(root, '#hub/weather/place', 760, 80, 560, 22, 'Berlin', 16, FONT.regular, C.ink2);

  // ---- row 1: camera, climate
  const X0 = 152, W = 1204, GAP = 24, COL = (W - 2 * GAP) / 3;
  const cam = card('Camera', X0, 176, 760, 320, { fillPaints: [solid([0.16, 0.17, 0.16])] });
  rect(cam, 'Live pill', 24, 24, 80, 34, C.accent, 17);
  text(cam, 'Live', 24, 31, 80, 20, '● LIVE', 14, FONT.semibold, C.white, 'CENTER');
  text(cam, 'Camera title', 28, 236, 600, 40, 'Front yard', 32, FONT.regular, C.white);
  text(cam, 'Camera status', 28, 276, 600, 22, 'Camera · No motion detected', 16, FONT.regular, C.muted);

  const climate = card('Climate', X0 + 760 + GAP, 176, W - 760 - GAP, 320);
  title(climate, 'Climate');
  sub(climate, 'Mode', 'Auto · indoors');
  slot(climate, '#hub/climate/minus', 28, 140, navButton('minus'));
  text(climate, '#hub/climate/setpoint', 100, 124, climate.size.x - 200, 92, '22.5°', 72, FONT.regular, C.ink, 'CENTER');
  slot(climate, '#hub/climate/plus', climate.size.x - 100, 140, navButton('plus'));
  text(climate, 'Target', 28, 248, climate.size.x - 56, 22, 'Target temperature', 16, FONT.regular, C.ink2, 'CENTER');

  // ---- row 2: lights, dimmer, fan
  const tile = (name, i, y, h) => card(name, X0 + i * (COL + GAP), y, COL, h);
  const lights = tile('Lights', 0, 520, 260);
  instance(lights, 'Symbol', 28, 28, variant('#symbol', 'icon=house'));
  title(lights, 'Living room', 96);
  sub(lights, '#hub/lights/state', 'Lights on', 132);
  slot(lights, '#hub/lights', COL - 28 - 68, 196, variant('#toggle', 'state=default'));

  const dimmer = tile('Dimmer', 1, 520, 260);
  instance(dimmer, 'Symbol', 28, 28, variant('#symbol', 'icon=bed-double'));
  title(dimmer, 'Bedroom', 96);
  sub(dimmer, '#hub/dimmer/value', 'Lights 65%', 132);
  slot(dimmer, '#hub/dimmer', 28, 176, variant('#slider', 'state=default'));

  const fan = tile('Fan', 2, 520, 260);
  instance(fan, 'Symbol', 28, 28, variant('#symbol', 'icon=armchair'));
  title(fan, 'Office fan', 96);
  sub(fan, '#hub/fan/state', 'On · Low', 132);
  slot(fan, '#hub/fan', 28, 176, variant('#segmented', 'segments=3, state=default'));

  // ---- row 3: intercom, door, the next 24 hours
  const intercom = tile('Intercom', 0, 804, 236);
  title(intercom, 'Intercom');
  sub(intercom, 'Rooms', 'All rooms');
  const talk = variant('#button', 'kind=primary, state=default, size=compact');
  slot(intercom, '#hub/talk', 28, 152, talk, label(talk, '#button/label', 'Talk'));

  const door = tile('Door', 1, 804, 236);
  title(door, 'Front door');
  sub(door, '#hub/door/state', 'Locked');
  slot(door, '#hub/door', COL - 28 - 72, 140, navButton('chevron-right'));

  const day = tile('Next hours', 2, 804, 236);
  title(day, 'Next 24 hours');
  slot(day, '#hub/weather/chart', 28, 92, variant('#bar-chart', 'bars=24'));
  text(day, '#hub/weather/t1', 28, 160, 60, 16, '13:00', 13, FONT.regular, C.ink2);
  text(day, '#hub/weather/t2', 28 + 152 - 30, 160, 60, 16, '01:00', 13, FONT.regular, C.ink2, 'CENTER');
  text(day, '#hub/weather/t3', 28 + 304 - 60, 160, 60, 16, '12:00', 13, FONT.regular, C.ink2, 'RIGHT');

  // ---- the news column: a list of #list-item (image, three lines, column) and its scrollbar
  const NX = 1380, LIST_H = 744, PITCH = 128 + 8;
  text(root, 'News heading', NX, 40, 490, 30, 'News', 24, FONT.semibold, C.ink);
  const list = frame(root, '#hub/news', NX, 96, 490, LIST_H, { frameMaskDisabled: false });
  const row = variant('#list-item', 'state=default, kind=image, lines=three, size=column');
  for (let i = 0; i * PITCH < LIST_H; i++) instance(list, `#hub/news/preview-${i + 1}`, 0, i * PITCH, row);
  A.scrollbarSlot(root, '#hub/news/scrollbar', NX + 490 + 8, 96, LIST_H);

  log(`#hub-root: ${A.count()} nodes`);
  return msg;
};
