# hub-app

A smart home hub screen on harry (1920 x 1080): a navigation rail, the time and the weather, a
grid of tiles (camera, climate, lights, dimmer, fan, intercom, door, the next 24 hours) and a
news column. It is written against the EveryWare UI library (`vendor/sdv-everyware/ui-lib`)
only, as a test of the library from an app other than smarthome-app.

## The design

`data/hub.dcf` is made without Figma: `design/hub-patch.js` (the screen, `#hub-root` from Library
slots) on the EveryWare Library, then ui-lib's `design/dark-theme-patch.js`, then `fig-to-dcf.sh`.
`design/build.sh` runs the whole chain:

```bash
packages/services/display_safety/utils/hub-app/design/build.sh     # from anywhere
```

It builds the Library (ui-lib `design/build-library.sh`, `design/build/library.fig` there), this
app's `design/hub.fig` (in git: the file a designer opens, see below) and
`data/hub.dcf`; `CHECK='#hub/time #hub/...'` names what must be in the .dcf.
fig-to-dcf.sh imports with everyware's `tools/dcf` (dcf-fetch `--local`, on our fork of
DesignCompose). A new board like this one: `vendor/sdv-everyware/ui-lib/skills/new-app/SKILL.md`.

The names under `#hub-root` (`#hub/*`) are the contract with `src/ui_model.rs`: rename both or neither.


## The design in Figma, without scripts

`design/hub.fig` is this app's screen as a Figma file: what is built, checked into git, and
what a designer opens. The plain path, no agent and no tools:

1. **Open it**: Figma, File > Import, `design/hub.fig`. The screen is the frame `#hub-root`;
   the Library (the components the screen is made of) is on its Components page.
2. **Change it** as in any Figma file: move, recolour, add a layer, swap an instance's variant.
   The layer names starting with `#` are the contract with the code (`src/ui_model.rs`): keep
   them; a new element the app should drive gets a `#hub/<name>` of its own, and the app a
   field with that name.
3. **Hand it over**: share the Figma file. The developer fetches it into the app once with
   `scripts/fetch-dcf.sh data/hub.dcf <file key>` (the key is in the file's URL), and the
   board shows it.

What the scripts add on top is only the other direction: `design/build.sh` writes this `.fig`
from `design/hub-patch.js` (an agent's or a developer's drawing in code) and the Library, and
makes the `.dcf` without Figma. Both ways meet in the same `.fig` and the same `.dcf`; the
`#names` are what keeps them together.

## Build and run on the RB3

```bash
# from the tree's top: build (sdv_media_db845c, out_rb3)
source device/rb3/sdv_media_db845c/tools/env.sh; m hub_app

# the panel's dashboard bundle on 127.0.0.1:7100 serves one app at a time: stop the other one first
adb shell mkdir -p /data/local/tmp/hub
adb push out_rb3/target/product/sdv_media_db845c/system/bin/hub_app /data/local/tmp/hub/
adb push packages/services/display_safety/utils/hub-app/data/hub.dcf /data/local/tmp/hub/
adb shell "cd /data/local/tmp/hub; LD_LIBRARY_PATH=/system/lib64:/vendor/lib64:/apex/com.everyware.smarthome/lib64 \
    ./hub_app --data-path /data/local/tmp/hub --touch-size 1920x1080"
```

The fonts are compiled in (`data/*.otf`); the time comes from the board's clock, the weather
from the dashboard bundle (and the news, shown with their pictures).
